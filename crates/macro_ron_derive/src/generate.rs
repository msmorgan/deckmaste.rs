//! Code generation for the two public derive entry points:
//!
//! * **`supports_macros`** (`#[derive(SupportsMacros)]`): emits four impls
//!   inside one `const _: () = { … }` block — `SupportsMacros` (the variant
//!   lists, kind facts, and tag dispatch), `Serialize` (invocation write-back
//!   and bare embeds), `Deserialize` (the macro-layer-aware entry), and
//!   `Expand` (recursive `Expanded`-stripping) — preceded by one private owned
//!   helper struct per struct variant (see [`gen_struct_helpers`]).
//!
//! * **`expand_only`** (`#[derive(Expand)]`): emits a single bare `impl
//!   ::macro_ron::Expand` with no helper structs and no surrounding `const _`
//!   block — for plain grammar types (`SupportsMacros` doesn't apply): every
//!   field recurses via `expand_all`, no serde involvement.
//!
//! Generated code names items only via absolute `::macro_ron::…` /
//! `::serde::…` / `::core::…` paths (`extern crate self as macro_ron` makes
//! the former resolve inside `macro_ron`'s own tests).

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Attribute;
use syn::Data;
use syn::DeriveInput;
use syn::Error;
use syn::Ident;
use syn::Result;
use syn::Type;

use crate::input::Field;
use crate::input::Input;
use crate::input::Marker;
use crate::input::Shape;
use crate::input::Variant;
use crate::input::{self};

pub fn supports_macros(input: &Input) -> Result<TokenStream> {
    let helpers = gen_struct_helpers(input);
    let support = gen_support(input)?;
    let serialize = gen_serialize(input);
    let deserialize = gen_deserialize(input);
    let expand = gen_expand_enum(input);
    Ok(quote! {
        const _: () = {
            #helpers
            #support
            #serialize
            #deserialize
            #expand
        };
    })
}

/// Whether the variant keeps its name in `OWN_VARIANTS`: a `flatten`
/// variant's name is erased entirely (the payload's names surface instead),
/// and a *newtype* embed is name-erased (always written bare). A *tuple*
/// embed keeps its tag; `expanded` and `literal` variants are ordinary.
fn is_named(v: &Variant) -> bool {
    match v.marker {
        Some(Marker::Flatten) => false,
        Some(Marker::Embed) => matches!(v.shape, Shape::Tuple(_)),
        _ => true,
    }
}

/// Sees through one level of `Box<…>` when naming a flatten/embed payload
/// type: a payload field typed `Box<Pick>` uses `<Pick as SupportsMacros>`
/// for the static lookups, and the constructor re-boxes the payload.
fn peeled(ty: &Type) -> (&Type, bool) {
    if let Type::Path(p) = ty
        && let Some(seg) = p.path.segments.last()
        && seg.ident == "Box"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && args.args.len() == 1
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        return (inner, true);
    }
    (ty, false)
}

/// The `flatten` variants' payload types, peeled, in declaration order.
fn flatten_payloads(input: &Input) -> Vec<&Type> {
    input
        .variants
        .iter()
        .filter(|v| v.marker == Some(Marker::Flatten))
        .map(|v| match &v.shape {
            Shape::Newtype(f) => peeled(&f.ty).0,
            _ => unreachable!("flatten is a newtype (validated in parse())"),
        })
        .collect()
}

/// The payload variant names every `flatten` compartment must NOT lift, as
/// string literals (`#[macro_ron(flatten, exclude(A, B))]`). Empty for a type
/// with no `exclude(...)`.
fn flatten_excludes(input: &Input) -> Vec<String> {
    input
        .variants
        .iter()
        .flat_map(|v| v.flatten_exclude.iter())
        .map(ToString::to_string)
        .collect()
}

/// The `&["A", "B"]` token for an exclude list.
fn exclude_lit(excludes: &[String]) -> TokenStream {
    quote!(&[#(#excludes),*])
}

/// The constructor mapping a deserialized payload into a newtype variant:
/// the bare path constructor `#ty::#v_ident` (a tuple-struct fn), or — when the
/// payload is `Box`ed — a closure that re-boxes it. Shared by
/// [`embed_construct`]'s newtype arm and [`fall_throughs`].
fn newtype_construct(ty: &Ident, v_ident: &Ident, boxed: bool) -> TokenStream {
    if boxed {
        quote!(|__payload| #ty::#v_ident(::std::boxed::Box::new(__payload)))
    } else {
        quote!(#ty::#v_ident)
    }
}

/// The constructor mapping a deserialized embed payload into the embed
/// variant: `T::Ref` for a newtype embed, a closure filling every defaulted
/// field with its default for a tuple embed. Boxed payloads re-box.
fn embed_construct(ty: &Ident, v: &Variant) -> TokenStream {
    let v_ident = &v.ident;
    match &v.shape {
        Shape::Newtype(f) => newtype_construct(ty, v_ident, peeled(&f.ty).1),
        Shape::Tuple(fields) => {
            let args = fields.iter().map(|f| match &f.default {
                Some(default) => quote!(#default),
                None if peeled(&f.ty).1 => quote!(::std::boxed::Box::new(__payload)),
                None => quote!(__payload),
            });
            quote!(|__payload| #ty::#v_ident(#(#args),*))
        }
        Shape::Unit | Shape::Struct(_) => {
            unreachable!("embed is a newtype or tuple (validated in parse())")
        }
    }
}

/// The private owned helper struct's ident for a struct variant:
/// `__` + type + variant (e.g. `__ClauseWhen`). The name never appears in
/// RON text (the variant reads/writes the helper as newtype content), so no
/// `#[serde(rename)]` is needed.
fn helper_ident(ty: &Ident, v_ident: &Ident) -> Ident {
    format_ident!("__{ty}{v_ident}")
}

/// One generated helper struct per struct variant, emitted before the impls.
/// serde rejects field attributes like `#[serde(default)]` on enum
/// struct-variant fields, so each struct variant lowers through a private
/// owned struct that carries the forwarded attributes, read/written as a
/// newtype variant — ron's `unwrap_variant_newtypes` keeps the text flat
/// (`When(verb: "draw")`).
fn gen_struct_helpers(input: &Input) -> TokenStream {
    let ty = &input.ident;
    let helpers = input.variants.iter().filter_map(|v| {
        let Shape::Struct(fields) = &v.shape else {
            return None;
        };
        let helper = helper_ident(ty, &v.ident);
        let fields = fields.iter().map(|f| {
            let attrs = &f.serde_attrs;
            let name = f.ident.as_ref().expect("struct fields are named");
            let f_ty = &f.ty;
            quote! { #(#attrs)* #name: #f_ty, }
        });
        Some(quote! {
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            #[allow(non_camel_case_types, reason = "generated RON-flattening helper struct named after its `Enum_Variant`")]
            struct #helper { #(#fields)* }
        })
    });
    quote!(#(#helpers)*)
}

/// The named fields of a struct variant, in declaration order.
fn field_names(fields: &[Field]) -> Vec<&Ident> {
    fields
        .iter()
        .map(|f| f.ident.as_ref().expect("struct fields are named"))
        .collect()
}

fn gen_support(input: &Input) -> Result<TokenStream> {
    let ty = &input.ident;
    let ty_name = ty.to_string();
    let own: Vec<String> = input
        .variants
        .iter()
        .filter(|v| is_named(v))
        .map(|v| v.ident.to_string())
        .collect();
    let own_sigs: Vec<TokenStream> = input
        .variants
        .iter()
        .filter(|v| is_named(v))
        .map(own_signature_entry)
        .collect();

    // The dispatch tails: one per `flatten` payload (declaration order),
    // then the embed payload last. The concrete type — not `Self` — names
    // the lists inside the const-generic block, which rejects `Self`.
    //
    // A *newtype* embed variant's OWN name (e.g. `Wrapped`) is deliberately
    // absent from every variant list — it is name-erased: parents flattening
    // this type never dispatch on it, and `from_variant` returns None for it
    // by design. Only tuple embeds keep their tag and appear in OWN_VARIANTS.
    let mut tails = flatten_payloads(input);
    if let Some(v) = input.embed() {
        tails.push(peeled(&v.embed_payload().ty).0);
    }
    let all = concat_lists(ty, &tails, &flatten_excludes(input));
    // Excludes are deliberately not mirrored here: a name that never
    // dispatches is never looked up in the signature table, so
    // ALL_SIGNATURES may carry extra (unreachable) entries for names an
    // `exclude(...)` drops from ALL_VARIANTS.
    let all_sigs = concat_signature_lists(ty, &tails);

    let kind = gen_kind(input, &ty_name);

    let arms = input
        .variants
        .iter()
        .filter(|v| is_named(v))
        .map(|v| from_variant_arm(ty, v))
        .collect::<Result<Vec<_>>>()?;
    let falls = fall_throughs(input);

    let expanded = if let Some(v) = input.expanded() {
        let v_ident = &v.ident;
        quote! {
            fn expanded(e: ::macro_ron::Expansion<Self>) -> ::core::option::Option<Self> {
                ::core::option::Option::Some(#ty::#v_ident(e))
            }
        }
    } else {
        quote! {
            fn expanded(_e: ::macro_ron::Expansion<Self>) -> ::core::option::Option<Self> {
                ::core::option::Option::None
            }
        }
    };

    Ok(quote! {
        #[automatically_derived]
        impl ::macro_ron::SupportsMacros for #ty {
            const OWN_VARIANTS: &'static [&'static str] = &[#(#own),*];
            const ALL_VARIANTS: &'static [&'static str] = #all;
            const OWN_SIGNATURES: &'static [(&'static str, ::macro_ron::VariantSignature)] =
                &[#(#own_sigs),*];
            const ALL_SIGNATURES: &'static [(&'static str, ::macro_ron::VariantSignature)] =
                #all_sigs;

            fn kind() -> ::macro_ron::Kind { #kind }

            fn from_variant<'de, __A: ::serde::de::VariantAccess<'de>>(
                ident: &str,
                access: __A,
            ) -> ::core::option::Option<::core::result::Result<Self, __A::Error>> {
                match ident {
                    #(#arms)*
                    _ => {}
                }
                #(#falls)*
                ::core::option::Option::None
            }

            #expanded
        }
    })
}

/// `<T>::OWN_VARIANTS` alone, or a `concat_variants` of it with each tail
/// type's `ALL_VARIANTS` when there are tails. With `excludes`, the tails'
/// contribution drops those names (the parent's OWN names are never filtered)
/// via `concat_variants_excluding`.
fn concat_lists(ty: &Ident, tails: &[&Type], excludes: &[String]) -> TokenStream {
    if tails.is_empty() {
        return quote!(<#ty as ::macro_ron::SupportsMacros>::OWN_VARIANTS);
    }
    if excludes.is_empty() {
        return quote! {
            &::macro_ron::concat_variants::<{
                <#ty as ::macro_ron::SupportsMacros>::OWN_VARIANTS.len()
                    #(+ <#tails as ::macro_ron::SupportsMacros>::ALL_VARIANTS.len())*
            }>(&[
                <#ty as ::macro_ron::SupportsMacros>::OWN_VARIANTS,
                #(<#tails as ::macro_ron::SupportsMacros>::ALL_VARIANTS,)*
            ])
        };
    }
    let exclude = exclude_lit(excludes);
    quote! {
        &::macro_ron::concat_variants_excluding::<{
            <#ty as ::macro_ron::SupportsMacros>::OWN_VARIANTS.len()
                + ::macro_ron::count_kept(
                    &[#(<#tails as ::macro_ron::SupportsMacros>::ALL_VARIANTS),*],
                    #exclude,
                )
        }>(
            <#ty as ::macro_ron::SupportsMacros>::OWN_VARIANTS,
            &[#(<#tails as ::macro_ron::SupportsMacros>::ALL_VARIANTS),*],
            #exclude,
        )
    }
}

/// One `(name, signature)` entry of `OWN_SIGNATURES`, for a variant kept in
/// `OWN_VARIANTS` (see [`is_named`]).
fn own_signature_entry(v: &Variant) -> TokenStream {
    let name = v.ident.to_string();
    let sig = variant_signature(&v.shape);
    quote!((#name, #sig))
}

/// The `ParamDefault` a positional field maps to: `Expr(text)` when
/// `#[macro_ron(default = ...)]` is present (rendered to source text),
/// `Required` otherwise.
fn positional_default(f: &Field) -> TokenStream {
    if let Some(expr) = &f.default {
        let text = quote!(#expr).to_string();
        quote!(::macro_ron::ParamDefault::Expr(#text))
    } else {
        quote!(::macro_ron::ParamDefault::Required)
    }
}

/// Whether any forwarded `#[serde(...)]` attribute sets `default` (bare
/// `#[serde(default)]` or `#[serde(default = "path")]`) — a struct-variant
/// field's `NamedParam::default` reads its default from here, never from
/// `Field.default` (`#[macro_ron(default = ...)]` is rejected on struct
/// variants in `input::validate`; struct fields forward `#[serde(...)]`
/// instead).
fn has_serde_default(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let Ok(metas) = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        ) else {
            return false;
        };
        metas.iter().any(|m| m.path().is_ident("default"))
    })
}

/// The `VariantSignature` a variant's shape maps to: `Unit` for a unit
/// variant, `Positional(&[…])` for a newtype (always `&[Required]`) or a
/// tuple (one `ParamDefault` per field, in order), `Named(&[…])` for a
/// struct variant — one `NamedParam` per field, `Implicit` when the field
/// forwards a `#[serde(default ...)]`, `Required` otherwise.
fn variant_signature(shape: &Shape) -> TokenStream {
    match shape {
        Shape::Unit => quote!(::macro_ron::VariantSignature::Unit),
        Shape::Newtype(_) => {
            quote!(::macro_ron::VariantSignature::Positional(&[
                ::macro_ron::ParamDefault::Required
            ]))
        }
        Shape::Tuple(fields) => {
            let entries = fields.iter().map(positional_default);
            quote!(::macro_ron::VariantSignature::Positional(&[#(#entries),*]))
        }
        Shape::Struct(fields) => {
            let params = fields.iter().map(|f| {
                let name = f
                    .ident
                    .as_ref()
                    .expect("struct fields are named")
                    .to_string();
                let default = if has_serde_default(&f.serde_attrs) {
                    quote!(::macro_ron::ParamDefault::Implicit)
                } else {
                    quote!(::macro_ron::ParamDefault::Required)
                };
                quote!(::macro_ron::NamedParam { name: #name, default: #default })
            });
            quote!(::macro_ron::VariantSignature::Named(&[#(#params),*]))
        }
    }
}

/// `<T>::OWN_SIGNATURES` alone, or a `concat_signatures` of it with each tail
/// type's `ALL_SIGNATURES` when there are tails. Mirrors [`concat_lists`]'s
/// no-excludes branch — excludes are deliberately not mirrored (see
/// `gen_support`).
fn concat_signature_lists(ty: &Ident, tails: &[&Type]) -> TokenStream {
    if tails.is_empty() {
        return quote!(<#ty as ::macro_ron::SupportsMacros>::OWN_SIGNATURES);
    }
    quote! {
        &::macro_ron::concat_signatures::<{
            <#ty as ::macro_ron::SupportsMacros>::OWN_SIGNATURES.len()
                #(+ <#tails as ::macro_ron::SupportsMacros>::ALL_SIGNATURES.len())*
        }>(&[
            <#ty as ::macro_ron::SupportsMacros>::OWN_SIGNATURES,
            #(<#tails as ::macro_ron::SupportsMacros>::ALL_SIGNATURES,)*
        ])
    }
}

/// The `Kind` builder chain: `remembers_expansion` ⇔ an `expanded` variant,
/// `embeds_untagged` ⇔ an `embed` variant, `literal_wrapper` ⇔ a `literal`
/// variant.
fn gen_kind(input: &Input, ty_name: &str) -> TokenStream {
    let mut kind = quote!(
        ::macro_ron::Kind::new(#ty_name)
            .with_variants(<Self as ::macro_ron::SupportsMacros>::ALL_VARIANTS)
            .with_own_variants(<Self as ::macro_ron::SupportsMacros>::OWN_VARIANTS)
            .with_signatures(<Self as ::macro_ron::SupportsMacros>::ALL_SIGNATURES)
    );
    if input.expanded().is_some() {
        kind = quote!(#kind.remembers_expansion());
    }
    if input.embed().is_some() {
        kind = quote!(#kind.embeds_untagged());
    }
    if let Some(v) = input.literal() {
        let wrapper = v.ident.to_string();
        kind = quote!(#kind.literal_wrapper(#wrapper));
    }
    kind
}

/// One `from_variant` match arm for an own tagged variant.
// The tuple-arity match arms bind field slots positionally as
// `first`/`second`/`third`/`fourth`, mirroring the variant's own field order.
fn from_variant_arm(ty: &Ident, v: &Variant) -> Result<TokenStream> {
    let v_ident = &v.ident;
    let name = v_ident.to_string();
    let read = match &v.shape {
        Shape::Unit => quote! {
            ::serde::de::VariantAccess::unit_variant(access).map(|()| #ty::#v_ident)
        },
        Shape::Newtype(_) => quote! {
            ::serde::de::VariantAccess::newtype_variant(access).map(#ty::#v_ident)
        },
        // A tuple embed (`By`) keeps its tag and always reads every field
        // positionally — its defaults drive only the bare-write guard, never
        // the read. Plain (non-embed) tuples may carry trailing defaults, read
        // through `PairPlusDefault`.
        Shape::Tuple(fields) => {
            let trailing_default =
                v.marker != Some(Marker::Embed) && fields.iter().any(|f| f.default.is_some());
            match (trailing_default, fields.as_slice()) {
                (false, [first, second]) => {
                    let (ta, tb) = (&first.ty, &second.ty);
                    quote! {
                        ::serde::de::VariantAccess::tuple_variant(
                            access,
                            2usize,
                            ::macro_ron::Pair::<#ta, #tb>::new(),
                        )
                        .map(|(__f0, __f1)| #ty::#v_ident(__f0, __f1))
                    }
                }
                (false, [first, second, third]) => {
                    let (ta, tb, tc) = (&first.ty, &second.ty, &third.ty);
                    quote! {
                        ::serde::de::VariantAccess::tuple_variant(
                            access,
                            3usize,
                            ::macro_ron::Triple::<#ta, #tb, #tc>::new(),
                        )
                        .map(|(__f0, __f1, __f2)| #ty::#v_ident(__f0, __f1, __f2))
                    }
                }
                // Two required fields + one trailing-defaulted field: the short
                // form `V(a, b)` fills the third with its default, the long
                // form `V(a, b, c)` supplies it (`PairPlusDefault`).
                (true, [first, second, third])
                    if first.default.is_none() && second.default.is_none() =>
                {
                    let (ta, tb, tc) = (&first.ty, &second.ty, &third.ty);
                    let default_third = third.default.as_ref().expect("trailing default present");
                    quote! {
                        ::serde::de::VariantAccess::tuple_variant(
                            access,
                            3usize,
                            ::macro_ron::PairPlusDefault::<#ta, #tb, #tc>::new(#default_third),
                        )
                        .map(|(__f0, __f1, __f2)| #ty::#v_ident(__f0, __f1, __f2))
                    }
                }
                // Two required fields + two trailing-defaulted fields: `V(a,
                // b)` fills both defaults, `V(a, b, c)` fills only the
                // fourth, `V(a, b, c, d)` supplies every field
                // (`PairPlusTwoDefaults`).
                (true, [first, second, third, fourth])
                    if first.default.is_none()
                        && second.default.is_none()
                        && third.default.is_some()
                        && fourth.default.is_some() =>
                {
                    let (ta, tb, tc, td) = (&first.ty, &second.ty, &third.ty, &fourth.ty);
                    let default_third = third.default.as_ref().expect("trailing default present");
                    let default_fourth = fourth.default.as_ref().expect("trailing default present");
                    quote! {
                        ::serde::de::VariantAccess::tuple_variant(
                            access,
                            4usize,
                            ::macro_ron::PairPlusTwoDefaults::<#ta, #tb, #tc, #td>::new(
                                #default_third,
                                #default_fourth,
                            ),
                        )
                        .map(|(__f0, __f1, __f2, __f3)| #ty::#v_ident(__f0, __f1, __f2, __f3))
                    }
                }
                // One required field + one trailing-defaulted field: the short
                // form `V(a)` fills the second with its default (reading like a
                // newtype in RON, so it needs the `SinglePlusDefault` runtime
                // visitor, not the static `Pair` layout), the long form `V(a,
                // b)` supplies it. `Cast(<ref>, [cost])`'s madness slot.
                (true, [first, second]) if first.default.is_none() && second.default.is_some() => {
                    let (ta, tb) = (&first.ty, &second.ty);
                    let default_second = second.default.as_ref().expect("trailing default present");
                    quote! {
                        ::serde::de::VariantAccess::tuple_variant(
                            access,
                            2usize,
                            ::macro_ron::SinglePlusDefault::<#ta, #tb>::new(#default_second),
                        )
                        .map(|(__f0, __f1)| #ty::#v_ident(__f0, __f1))
                    }
                }
                (_, fields) => {
                    return Err(Error::new(
                        v_ident.span(),
                        format!(
                            "tuple variants of arity {} (with this default layout) are \
                             unsupported; add a runtime visitor",
                            fields.len()
                        ),
                    ));
                }
            }
        }
        Shape::Struct(fields) => {
            let helper = helper_ident(ty, v_ident);
            let names = field_names(fields);
            quote! {
                ::serde::de::VariantAccess::newtype_variant::<#helper>(access)
                    .map(|__h| #ty::#v_ident { #(#names: __h.#names),* })
            }
        }
    };
    Ok(quote! {
        #name => return ::core::option::Option::Some(#read),
    })
}

/// The fall-through membership checks `from_variant` runs after its own
/// arms: each `flatten` payload in declaration order, then the embed
/// payload. Each check consumes `access` only on its own (diverging) path.
fn fall_throughs(input: &Input) -> Vec<TokenStream> {
    let ty = &input.ident;
    let mut falls = Vec::new();
    let flattens = input
        .variants
        .iter()
        .filter(|v| v.marker == Some(Marker::Flatten));
    for v in flattens {
        let Shape::Newtype(f) = &v.shape else {
            unreachable!("flatten is a newtype (validated in parse())");
        };
        let (inner, boxed) = peeled(&f.ty);
        let construct = newtype_construct(ty, &v.ident, boxed);
        falls.push(fall_through(inner, &construct, &v.flatten_exclude));
    }
    if let Some(v) = input.embed() {
        let inner = peeled(&v.embed_payload().ty).0;
        falls.push(fall_through(inner, &embed_construct(ty, v), &[]));
    }
    falls
}

fn fall_through(inner: &Type, construct: &TokenStream, exclude: &[Ident]) -> TokenStream {
    // An excluded payload name never dispatches here — `from_variant` returns
    // `None` for it (a bare `Library` is not a `Destination::Zone`), so the
    // caller reports it as unknown, mirroring the Idris `DestinationOk` gate.
    let guard = if exclude.is_empty() {
        quote!()
    } else {
        let names = exclude.iter().map(ToString::to_string);
        quote!(&& !&[#(#names),*].contains(&ident))
    };
    quote! {
        if <#inner as ::macro_ron::SupportsMacros>::ALL_VARIANTS.contains(&ident) #guard {
            return <#inner as ::macro_ron::SupportsMacros>::from_variant(ident, access)
                .map(|__r| __r.map(#construct));
        }
    }
}

fn gen_serialize(input: &Input) -> TokenStream {
    let ty = &input.ident;
    let ty_name = ty.to_string();
    let mut arms = Vec::new();
    for (index, v) in input.variants.iter().enumerate() {
        let v_ident = &v.ident;
        let name = v_ident.to_string();
        let index = u32::try_from(index).expect("variant index fits in u32");
        match (v.marker, &v.shape) {
            // Invocation write-back, compartment transparency, the bare-numeral
            // literal, and the name-erased newtype embed: all delegate to the
            // payload — so a literal writes `3`, never `Literal(3)`. (The read
            // side is the literal-aware reader in `deckmaste_core::ron`.)
            (Some(Marker::Expanded | Marker::Flatten | Marker::Literal), _)
            | (Some(Marker::Embed), Shape::Newtype(_)) => arms.push(quote! {
                #ty::#v_ident(__f0) => ::serde::Serialize::serialize(__f0, serializer),
            }),
            (Some(Marker::Embed), Shape::Tuple(fields)) => {
                // Bare when every defaulted field equals its default …
                let pats: Vec<Ident> = (0..fields.len()).map(|i| format_ident!("__f{i}")).collect();
                let guards = fields
                    .iter()
                    .zip(&pats)
                    .filter_map(|(f, p)| f.default.as_ref().map(|d| quote!(*#p == (#d))));
                let payload = fields
                    .iter()
                    .position(|f| f.default.is_none())
                    .map(|i| &pats[i])
                    .expect("embed tuple has one non-defaulted field (validated in parse())");
                arms.push(quote! {
                    #ty::#v_ident(#(#pats),*) if #(#guards)&&* =>
                        ::serde::Serialize::serialize(#payload, serializer),
                });
                // … and the ordinary tagged tuple arm otherwise.
                arms.push(tuple_arm(ty, &ty_name, index, v_ident, &name, fields.len()));
            }
            (_, Shape::Unit) => arms.push(quote! {
                #ty::#v_ident => ::serde::Serializer::serialize_unit_variant(
                    serializer, #ty_name, #index, #name,
                ),
            }),
            (_, Shape::Newtype(_)) => arms.push(quote! {
                #ty::#v_ident(__f0) => ::serde::Serializer::serialize_newtype_variant(
                    serializer, #ty_name, #index, #name, __f0,
                ),
            }),
            (_, Shape::Tuple(fields)) if fields.iter().all(|f| f.default.is_none()) => {
                arms.push(tuple_arm(ty, &ty_name, index, v_ident, &name, fields.len()));
            }
            // A non-embed tuple with trailing defaults: the short form omits
            // every defaulted field (when each equals its default), the full
            // form writes them all.
            (_, Shape::Tuple(fields)) => {
                let required = fields.iter().filter(|f| f.default.is_none()).count();
                let pats: Vec<Ident> = (0..fields.len()).map(|i| format_ident!("__f{i}")).collect();
                let guards = fields
                    .iter()
                    .zip(&pats)
                    .filter_map(|(f, p)| f.default.as_ref().map(|d| quote!(*#p == (#d))));
                let short = &pats[..required];
                arms.push(quote! {
                    #ty::#v_ident(#(#pats),*) if #(#guards)&&* => {
                        let mut __tv = ::serde::Serializer::serialize_tuple_variant(
                            serializer, #ty_name, #index, #name, #required,
                        )?;
                        #(::serde::ser::SerializeTupleVariant::serialize_field(&mut __tv, #short)?;)*
                        ::serde::ser::SerializeTupleVariant::end(__tv)
                    },
                });
                arms.push(tuple_arm(ty, &ty_name, index, v_ident, &name, fields.len()));
            }
            // The struct variant clones its fields into the owned helper
            // and writes it as newtype content (flat in RON via
            // `unwrap_variant_newtypes`).
            (_, Shape::Struct(fields)) => {
                let helper = helper_ident(ty, v_ident);
                let names = field_names(fields);
                arms.push(quote! {
                    #ty::#v_ident { #(#names),* } =>
                        ::serde::Serializer::serialize_newtype_variant(
                            serializer, #ty_name, #index, #name,
                            &#helper { #(#names: #names.clone()),* },
                        ),
                });
            }
        }
    }
    quote! {
        #[automatically_derived]
        impl ::serde::Serialize for #ty {
            fn serialize<__S: ::serde::Serializer>(
                &self,
                serializer: __S,
            ) -> ::core::result::Result<__S::Ok, __S::Error> {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}

/// The ordinary tagged tuple-variant serialization arm.
fn tuple_arm(
    ty: &Ident,
    ty_name: &str,
    index: u32,
    v_ident: &Ident,
    name: &str,
    arity: usize,
) -> TokenStream {
    let pats: Vec<Ident> = (0..arity).map(|i| format_ident!("__f{i}")).collect();
    quote! {
        #ty::#v_ident(#(#pats),*) => {
            let mut __tv = ::serde::Serializer::serialize_tuple_variant(
                serializer, #ty_name, #index, #name, #arity,
            )?;
            #(::serde::ser::SerializeTupleVariant::serialize_field(&mut __tv, #pats)?;)*
            ::serde::ser::SerializeTupleVariant::end(__tv)
        }
    }
}

fn gen_deserialize(input: &Input) -> TokenStream {
    let ty = &input.ident;
    let ty_name = ty.to_string();
    let expecting = format!("one of the names a `{ty_name}` position accepts");

    // What this type's own `deserialize_enum` passes: own names plus
    // flattened compartments' — NEVER the embed payload's, so the macro
    // layer's `embeds_untagged` hook routes those identifiers to
    // `visit_newtype_struct` and the embedded type's macros work here.
    let native = concat_lists(ty, &flatten_payloads(input), &flatten_excludes(input));

    let visit_newtype = input.embed().map(|v| {
        let inner = peeled(&v.embed_payload().ty).0;
        let construct = embed_construct(ty, v);
        quote! {
            fn visit_newtype_struct<__D: ::serde::Deserializer<'de>>(
                self,
                deserializer: __D,
            ) -> ::core::result::Result<Self::Value, __D::Error> {
                <#inner as ::serde::Deserialize<'de>>::deserialize(deserializer).map(#construct)
            }
        }
    });

    quote! {
        const __NATIVE: &'static [&'static str] = #native;

        struct __Visitor;

        #[automatically_derived]
        impl<'de> ::serde::de::Visitor<'de> for __Visitor {
            type Value = #ty;

            fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.write_str(#expecting)
            }

            fn visit_enum<__A: ::serde::de::EnumAccess<'de>>(
                self,
                data: __A,
            ) -> ::core::result::Result<Self::Value, __A::Error> {
                let (__ident, __access) =
                    ::serde::de::EnumAccess::variant_seed(data, ::macro_ron::IdentSeed)?;
                match <#ty as ::macro_ron::SupportsMacros>::from_variant(
                    __ident.as_str(),
                    __access,
                ) {
                    ::core::option::Option::Some(__r) => __r,
                    ::core::option::Option::None => ::core::result::Result::Err(
                        ::serde::de::Error::unknown_variant(__ident.as_str(), __NATIVE),
                    ),
                }
            }

            #visit_newtype
        }

        #[automatically_derived]
        impl<'de> ::serde::Deserialize<'de> for #ty {
            fn deserialize<__D: ::serde::Deserializer<'de>>(
                deserializer: __D,
            ) -> ::core::result::Result<Self, __D::Error> {
                ::serde::Deserializer::deserialize_enum(
                    deserializer,
                    #ty_name,
                    __NATIVE,
                    __Visitor,
                )
            }
        }
    }
}

fn gen_expand_enum(input: &Input) -> TokenStream {
    let ty = &input.ident;
    let arms = input.variants.iter().map(|v| {
        let v_ident = &v.ident;
        if v.marker == Some(Marker::Expanded) {
            // Strip the wrapper, recursing into the stored value (a body may
            // itself invoke macros).
            return quote! {
                #ty::#v_ident(__e) => ::macro_ron::Expand::expand_all(*__e.value),
            };
        }
        expand_arm(ty, v_ident, &ExpandFields::of_shape(&v.shape))
    });
    expand_impl(ty, &quote!(match self { #(#arms)* }))
}

/// A variant's (or plain struct's) field bindings for [`expand_arm`]: unit,
/// positional (newtype and tuple alike), or named — all `expand_all` needs
/// to destructure and rebuild.
enum ExpandFields<'a> {
    Unit,
    Positional(usize),
    Named(Vec<&'a Ident>),
}

impl<'a> ExpandFields<'a> {
    fn of_shape(shape: &'a Shape) -> Self {
        match shape {
            Shape::Unit => Self::Unit,
            Shape::Newtype(_) => Self::Positional(1),
            Shape::Tuple(fields) => Self::Positional(fields.len()),
            Shape::Struct(fields) => Self::Named(field_names(fields)),
        }
    }

    fn of_fields(fields: &'a syn::Fields) -> Self {
        match fields {
            syn::Fields::Unit => Self::Unit,
            syn::Fields::Unnamed(fs) => Self::Positional(fs.unnamed.len()),
            syn::Fields::Named(fs) => Self::Named(
                fs.named
                    .iter()
                    .map(|f| f.ident.as_ref().expect("named fields have idents"))
                    .collect(),
            ),
        }
    }
}

/// One `expand_all` match arm recursing into every field — shared by
/// [`gen_expand_enum`] (markerless variants) and [`expand_only`] enums.
fn expand_arm(ty: &Ident, v_ident: &Ident, fields: &ExpandFields) -> TokenStream {
    match fields {
        ExpandFields::Unit => quote!(#ty::#v_ident => #ty::#v_ident,),
        ExpandFields::Positional(arity) => {
            let pats: Vec<Ident> = (0..*arity).map(|i| format_ident!("__f{i}")).collect();
            quote! {
                #ty::#v_ident(#(#pats),*) => #ty::#v_ident(
                    #(::macro_ron::Expand::expand_all(#pats)),*
                ),
            }
        }
        ExpandFields::Named(names) => quote! {
            #ty::#v_ident { #(#names),* } => #ty::#v_ident {
                #(#names: ::macro_ron::Expand::expand_all(#names)),*
            },
        },
    }
}

/// The `impl ::macro_ron::Expand` wrapper around a built `expand_all` body.
fn expand_impl(ty: &Ident, body: &TokenStream) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl ::macro_ron::Expand for #ty {
            fn expand_all(self) -> Self {
                #body
            }
        }
    }
}

/// `#[derive(Expand)]` — recursion-only, for the plain grammar types
/// `SupportsMacros` doesn't apply to: no markers, no serde involvement,
/// every field recurses via `expand_all`.
pub fn expand_only(derive: &DeriveInput) -> Result<TokenStream> {
    input::reject_generics(derive)?;
    reject_markers(derive)?;
    let ty = &derive.ident;
    let body = match &derive.data {
        Data::Struct(data) => match ExpandFields::of_fields(&data.fields) {
            ExpandFields::Unit => quote!(self),
            ExpandFields::Positional(arity) => {
                let pats: Vec<Ident> = (0..arity).map(|i| format_ident!("__f{i}")).collect();
                quote! {
                    let Self(#(#pats),*) = self;
                    Self(#(::macro_ron::Expand::expand_all(#pats)),*)
                }
            }
            ExpandFields::Named(names) => quote! {
                let Self { #(#names),* } = self;
                Self { #(#names: ::macro_ron::Expand::expand_all(#names)),* }
            },
        },
        Data::Enum(data) => {
            let arms = data
                .variants
                .iter()
                .map(|v| expand_arm(ty, &v.ident, &ExpandFields::of_fields(&v.fields)));
            quote!(match self { #(#arms)* })
        }
        Data::Union(_) => {
            return Err(Error::new(
                derive.ident.span(),
                "Expand does not support unions",
            ));
        }
    };
    Ok(expand_impl(ty, &body))
}

/// `#[macro_ron(...)]` markers describe macro-layer behavior and belong to
/// `#[derive(SupportsMacros)]`; on a plain `Expand` type they would be
/// silently inert, so reject them anywhere on the item.
fn reject_markers(derive: &DeriveInput) -> Result<()> {
    fn field_attrs(fields: &syn::Fields) -> impl Iterator<Item = &Attribute> {
        fields.iter().flat_map(|f| f.attrs.iter())
    }
    let mut attrs: Vec<&Attribute> = derive.attrs.iter().collect();
    match &derive.data {
        Data::Struct(data) => attrs.extend(field_attrs(&data.fields)),
        Data::Enum(data) => {
            for v in &data.variants {
                attrs.extend(&v.attrs);
                attrs.extend(field_attrs(&v.fields));
            }
        }
        Data::Union(data) => attrs.extend(data.fields.named.iter().flat_map(|f| f.attrs.iter())),
    }
    match attrs.iter().find(|a| a.path().is_ident("macro_ron")) {
        Some(attr) => Err(Error::new_spanned(
            attr,
            "#[macro_ron(...)] markers belong to #[derive(SupportsMacros)]; \
             #[derive(Expand)] only recurses",
        )),
        None => Ok(()),
    }
}
