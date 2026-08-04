//! Deterministic code emission. Input is [`crate::validate::ValidatedGroup`]
//! by construction — the validator is the only way to obtain one. Iteration
//! is over declaration-order `Vec`s exclusively; determinism is structural,
//! not an afterthought.

use proc_macro2::TokenStream;
use quote::quote;

use crate::model::AstShape;
use crate::model::Constraint;
use crate::model::ConstructionDeclaration;
use crate::model::ElementDeclaration;
use crate::model::FieldBinding;
use crate::model::FieldKind;
use crate::model::GroupDeclaration;
use crate::model::Predicate;
use crate::validate::ValidatedGroup;

#[must_use]
pub fn emit_group(validated: &ValidatedGroup<'_>) -> TokenStream {
    let group = validated.group();
    let module = quote::format_ident!("__constructions_{}", group.name.value);
    let serde_reached = serde_reached_elements(group);
    let elements: Vec<TokenStream> = group
        .elements
        .iter()
        .map(|element| {
            let serde = serde_reached.contains(&element.name.value);
            element_item(group, element, serde)
        })
        .collect();
    let constructions: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|c| own_construction(group, c))
        .collect();
    let bind_constructions: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|c| bind_construction(group, c))
        .collect();
    let deserialize_impls: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|construction| deserialize_impl(group, construction))
        .collect();
    let declaration = declaration_static(group);
    let reexports = reexports(group);
    let free_witness_types: Vec<TokenStream> = group
        .constructions
        .iter()
        .flat_map(|c| c.witnesses.iter())
        .filter_map(|w| match &w.class {
            crate::model::WitnessClass::Free { ty } => Some(parse_type(&ty.value)),
            _ => None,
        })
        .collect();
    let witness_assertions = if free_witness_types.is_empty() {
        quote! {}
    } else {
        quote! {
            #[allow(dead_code, reason = "the trait bound on assert_payload is the check; nothing calls this")]
            fn __assert_free_witness_payloads() {
                fn assert_payload<T: ::deckmaste_features::SurfaceWitnessPayload>() {}
                #(assert_payload::<#free_witness_types>();)*
            }
        }
    };
    quote! {
        mod #module {
            #![allow(
                clippy::nonminimal_bool,
                reason = "the `if !(…)` wrapper negates arbitrary author predicates \
                          uniformly, including `matches!` and nested `&&`/`||` trees; \
                          a De Morgan pass would add a whole rewriting layer for zero \
                          semantic gain"
            )]
            #![allow(
                clippy::must_use_candidate,
                reason = "accessors are generated per declared field, not chosen"
            )]
            #![allow(
                clippy::missing_errors_doc,
                reason = "try_new's error conditions are the declaration's `require` \
                          clauses, which the author wrote and can read"
            )]
            use super::*;
            #(#elements)*
            #(#constructions)*
            #(#bind_constructions)*
            #witness_assertions
            #(#deserialize_impls)*
            #declaration
        }
        #(#reexports)*
    }
}

/// BFS over `seq` fields reached from at least one serde-opt-in own
/// construction. Membership-only: element **emission** order stays
/// declaration order in `emit_group`, unaffected by `sort_unstable()` below.
fn serde_reached_elements(group: &GroupDeclaration) -> Vec<String> {
    let mut reached: Vec<String> = Vec::new();
    let mut queue: Vec<&str> = Vec::new();
    for construction in &group.constructions {
        if !construction.deserialize {
            continue;
        }
        for binding in construction.ast.fields() {
            enqueue_sequences(&binding.kind, &mut queue);
        }
    }
    while let Some(name) = queue.pop() {
        if reached.iter().any(|r| r == name) {
            continue;
        }
        reached.push(name.to_owned());
        if let Some(element) = group.elements.iter().find(|e| e.name.value == name) {
            for binding in &element.fields {
                enqueue_sequences(&binding.kind, &mut queue);
            }
        }
    }
    reached.sort_unstable(); // deterministic regardless of discovery order
    reached
}

fn enqueue_sequences<'g>(kind: &'g FieldKind, queue: &mut Vec<&'g str>) {
    match kind {
        FieldKind::Sequence { element } => queue.push(element.value.as_str()),
        FieldKind::Optional { inner } => enqueue_sequences(inner, queue),
        FieldKind::Subtree { .. } | FieldKind::Scalar { .. } => {}
    }
}

fn element_item(
    group: &GroupDeclaration,
    element: &ElementDeclaration,
    serde: bool,
) -> TokenStream {
    let Some(bind_path) = &element.bind_path else {
        return owned_element_struct(group, element, serde);
    };
    let target = parse_type(&bind_path.value);
    if element.fields.is_empty() {
        return quote! {
            const _: fn(&#target) = |_| {};
        };
    }
    let parts = quote::format_ident!("__parts_{}", element.name.value);
    let field_names: Vec<proc_macro2::Ident> = element
        .fields
        .iter()
        .map(|binding| quote::format_ident!("{}", binding.field.value))
        .collect();
    let field_types: Vec<TokenStream> = element
        .fields
        .iter()
        .map(|binding| field_type(group, &binding.kind))
        .collect();
    quote! {
        fn #parts(value: &#target) -> (#(&#field_types),*) {
            let #target { #(#field_names),* } = value;
            (#(#field_names),*)
        }
        const _: fn(&#target) -> (#(&#field_types),*) = #parts;
    }
}

fn owned_element_struct(
    group: &GroupDeclaration,
    element: &ElementDeclaration,
    serde: bool,
) -> TokenStream {
    let name = pascal_ident(&element.name.value);
    let fields: Vec<TokenStream> = element
        .fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let ty = field_type(group, &binding.kind);
            quote! { pub #field: #ty, }
        })
        .collect();
    // Reached (transitively, via `seq`) by at least one serde-opt-in own
    // construction: the element also needs a structural `Deserialize`.
    let serde_derive = if serde {
        quote! { #[derive(serde::Deserialize)] }
    } else {
        quote! {}
    };
    quote! {
        // Elements carry no declaration invariants, so their fields stay
        // public; the owning construction validates the sequence whole.
        #[derive(Debug, PartialEq, Eq)]
        #serde_derive
        pub struct #name {
            #(#fields)*
        }
    }
}

fn own_construction(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Option<TokenStream> {
    let AstShape::Own { name, fields } = &construction.ast else {
        return None; // bind mode: handled separately by `bind_construction`
    };
    let ty = quote::format_ident!("{}", name.value);
    let id = construction.id.value.as_str();
    let field_decls: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty, }
        })
        .collect();
    let params: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty }
        })
        .collect();
    let checks: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| match constraint {
            Constraint::Require(predicate) => {
                Some(require_check(id, group, fields, &predicate.value))
            }
            Constraint::DeriveFeature { .. } => None, // feature derivation is chart/render-side
        })
        .collect();
    let field_names: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|b| quote::format_ident!("{}", b.field.value))
        .collect();
    let accessors: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! {
                pub fn #field(&self) -> &#field_ty {
                    &self.#field
                }
            }
        })
        .collect();
    Some(quote! {
        #[derive(Debug, PartialEq, Eq)]
        pub struct #ty {
            #(#field_decls)*
        }
        impl #ty {
            pub fn try_new(#(#params),*) -> Result<Self, ::deckmaste_construction_compiler::runtime::DeclarationViolation> {
                #(#checks)*
                Ok(Self { #(#field_names),* })
            }
            #(#accessors)*
        }
    })
}

/// Bind mode's checked door onto an unmigrated target type (CF-7): a
/// `require`-enforcing builder plus a full-pattern destructurer, both
/// re-exported. No struct, no `try_new`, no serde — EC005 already bars
/// bind+deserialize at validation time, so there is no door to route through
/// a `Deserialize` impl here.
fn bind_construction(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Option<TokenStream> {
    let AstShape::Bind { path, fields } = &construction.ast else {
        return None; // own mode: handled separately by `own_construction`
    };
    let target = parse_type(&path.value);
    let id = construction.id.value.as_str();
    let build_fn = quote::format_ident!("build_{}", construction.id.value);
    let parts_fn = quote::format_ident!("parts_{}", construction.id.value);
    let params: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty }
        })
        .collect();
    let checks: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| match constraint {
            Constraint::Require(predicate) => {
                Some(require_check(id, group, fields, &predicate.value))
            }
            Constraint::DeriveFeature { .. } => None, // feature derivation is chart/render-side
        })
        .collect();
    let field_names: Vec<proc_macro2::Ident> = fields
        .iter()
        .map(|b| quote::format_ident!("{}", b.field.value))
        .collect();
    let field_types: Vec<TokenStream> = fields.iter().map(|b| field_type(group, &b.kind)).collect();
    Some(quote! {
        // Bind mode: the target type stays public and unmigrated; these are
        // the checked door and the drift gate. The struct literal and the
        // full (no `..`) pattern each name every declared field, so a
        // declaration/type mismatch in either direction is a compile error.
        pub fn #build_fn(#(#params),*) -> Result<#target, ::deckmaste_construction_compiler::runtime::DeclarationViolation> {
            #(#checks)*
            Ok(#target { #(#field_names),* })
        }
        pub fn #parts_fn(value: &#target) -> (#(&#field_types),*) {
            let #target { #(#field_names),* } = value;
            (#(#field_names),*)
        }
    })
}

/// Serde-opt-in own constructions only; `None` for opt-out. The `AstShape::Own`
/// match below also returns `None` for a `Bind`-mode construction, but
/// EC005 (`DeserializeRequiresOwn`) already rejects `deserialize: true` on
/// bind mode at validation time — for a `ValidatedGroup`, `construction.ast`
/// is never `Bind` here once `construction.deserialize` is true. That arm is
/// defensive dead code, not a live deferral to a later milestone.
fn deserialize_impl(
    group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Option<TokenStream> {
    if !construction.deserialize {
        return None;
    }
    let AstShape::Own { name, fields } = &construction.ast else { return None };
    let ty = quote::format_ident!("{}", name.value);
    let raw_fields: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(group, &binding.kind);
            quote! { #field: #field_ty, }
        })
        .collect();
    let args: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            quote! { raw.#field }
        })
        .collect();
    Some(quote! {
        // Opt-in validating deserialization: a private raw mirror is
        // deserialized structurally, then handed to the same validator as
        // public construction. There is no other deserialize path.
        impl<'de> serde::Deserialize<'de> for #ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(serde::Deserialize)]
                struct Raw {
                    #(#raw_fields)*
                }
                let raw = Raw::deserialize(deserializer)?;
                #ty::try_new(#(#args),*).map_err(|violation| {
                    serde::de::Error::custom(format!(
                        "{}: {}",
                        violation.construction, violation.requirement
                    ))
                })
            }
        }
    })
}

fn field_type(group: &GroupDeclaration, kind: &FieldKind) -> TokenStream {
    match kind {
        FieldKind::Subtree { category, boxed } => {
            let ty = parse_type(&category.value);
            if *boxed {
                quote! { Box<#ty> }
            } else {
                quote! { #ty }
            }
        }
        FieldKind::Scalar { codec } => {
            let ty = parse_type(&codec.value);
            quote! { #ty }
        }
        FieldKind::Sequence { element } => {
            let declaration = group
                .elements
                .iter()
                .find(|declaration| declaration.name.value == element.value);
            let ty = declaration
                .and_then(|declaration| declaration.bind_path.as_ref())
                .map_or_else(
                    || {
                        // EC003 covers top-level construction fields. Preserve
                        // the prior PascalCase fallback for an undeclared nested
                        // element sequence rather than changing that diagnostic
                        // boundary as a side effect of bound-element resolution.
                        let owned = pascal_ident(&element.value);
                        quote! { #owned }
                    },
                    |path| parse_type(&path.value),
                );
            quote! { Vec<#ty> }
        }
        FieldKind::Optional { inner } => {
            let ty = field_type(group, inner);
            quote! { Option<#ty> }
        }
    }
}

fn require_check(
    id: &str,
    group: &GroupDeclaration,
    fields: &[FieldBinding],
    predicate: &Predicate,
) -> TokenStream {
    let condition = predicate_tokens(group, fields, predicate);
    let requirement = render_predicate(predicate);
    quote! {
        if !(#condition) {
            return Err(::deckmaste_construction_compiler::runtime::DeclarationViolation {
                construction: #id,
                requirement: #requirement,
            });
        }
    }
}

/// EC032 guarantees every path here is either a single segment naming a
/// direct field, or exactly `<sequence>.last.<element field>` (and only onto
/// a scalar/optional-scalar element field), and EC015 guarantees kind
/// agreement (`In` targets a scalar, `len()` targets a sequence) — so this
/// mapping is total for validated input.
fn predicate_tokens(
    group: &GroupDeclaration,
    fields: &[FieldBinding],
    predicate: &Predicate,
) -> TokenStream {
    match predicate {
        Predicate::LenAtLeast { path, min } => {
            // EC032: `len()` paths stay single-segment.
            let field = path_ident(path);
            // Unsuffixed: the generated comparison is against usize (.len()),
            // and an unsuffixed literal adopts that type with no cast.
            let min = proc_macro2::Literal::u32_unsuffixed(*min);
            quote! { #field.len() >= #min }
        }
        Predicate::LenIs { path, len } => {
            let field = path_ident(path);
            let len = proc_macro2::Literal::u32_unsuffixed(*len);
            quote! { #field.len() == #len }
        }
        Predicate::In { path, .. } | Predicate::IsSome { path } | Predicate::IsNone { path } => {
            match path.segments.as_slice() {
                [only] => {
                    let accessor = quote::format_ident!("{}", only.value);
                    single_field_check(&quote! { #accessor }, fields, &only.value, predicate)
                }
                [seq, _last, elem_field] => {
                    let seq_field = quote::format_ident!("{}", seq.value);
                    let element = element_of_sequence_field(group, fields, &seq.value);
                    let field_ident = quote::format_ident!("{}", elem_field.value);
                    let inner = single_field_check(
                        &quote! { member.#field_ident },
                        &element.fields,
                        &elem_field.value,
                        predicate,
                    );
                    // Vacuous on an empty sequence; when the element field
                    // itself is optional-scalar, `single_field_check` nests
                    // Task 1's `None | Some(...)` reading INSIDE this
                    // closure, so the two vacuous layers compose correctly.
                    quote! { #seq_field.last().is_none_or(|member| #inner) }
                }
                _ => unreachable!(
                    "validated: EC032 admits only a single-field path or exactly seq.last.field"
                ),
            }
        }
        Predicate::All(children) => {
            let parts: Vec<TokenStream> = children
                .iter()
                .map(|c| predicate_tokens(group, fields, c))
                .collect();
            quote! { (#(#parts)&&*) }
        }
        Predicate::Any(children) => {
            let parts: Vec<TokenStream> = children
                .iter()
                .map(|c| predicate_tokens(group, fields, c))
                .collect();
            quote! { (#(#parts)||*) }
        }
    }
}

/// The single-field check body shared between a direct field access
/// (`accessor` = the field itself) and the `.last()` closure's per-element
/// access (`accessor` = `member.field`) — today's `In`/`IsSome`/`IsNone`
/// emission, reused verbatim so the `.last()` closure's vacuous-on-empty
/// layer and Task 1's vacuous-on-absent optional-`In` reading nest
/// correctly by construction rather than by a second hand-written copy.
/// `fields`/`field_name` locate the SAME field `accessor` reads: top-level
/// ast fields for a direct path, an element's fields for the `.last` case.
fn single_field_check(
    accessor: &TokenStream,
    fields: &[FieldBinding],
    field_name: &str,
    predicate: &Predicate,
) -> TokenStream {
    match predicate {
        Predicate::In { allowed, .. } => {
            let codec = codec_of(fields, field_name);
            let variants: Vec<TokenStream> = allowed
                .iter()
                .map(|variant| {
                    let v = quote::format_ident!("{}", variant);
                    quote! { #codec::#v }
                })
                .collect();
            if is_optional_scalar(fields, field_name) {
                // Reading B: absence is vacuously admitted — `f in [...]`
                // on an optional field does not by itself demand presence.
                quote! { matches!(#accessor, None | Some(#(#variants)|*)) }
            } else {
                quote! { matches!(#accessor, #(#variants)|*) }
            }
        }
        Predicate::IsSome { .. } => quote! { #accessor.is_some() },
        Predicate::IsNone { .. } => quote! { #accessor.is_none() },
        Predicate::LenAtLeast { .. }
        | Predicate::LenIs { .. }
        | Predicate::All(_)
        | Predicate::Any(_) => {
            unreachable!("single_field_check is only called for In/IsSome/IsNone predicates")
        }
    }
}

/// The element declaration a sequence field's `.last` addresses. `fields`
/// is the field list `seq_field_name` is looked up in (the construction's
/// own ast fields — `.last` paths only ever open off a top-level sequence).
fn element_of_sequence_field<'g>(
    group: &'g GroupDeclaration,
    fields: &[FieldBinding],
    seq_field_name: &str,
) -> &'g ElementDeclaration {
    let binding = fields
        .iter()
        .find(|b| b.field.value == seq_field_name)
        .expect("validated: EC032 admits a `.last` path only when its sequence field resolves");
    let FieldKind::Sequence { element } = &binding.kind else {
        unreachable!("validated: resolve_path only opens `.last` on a Sequence-kind field")
    };
    group
        .elements
        .iter()
        .find(|e| e.name.value == element.value)
        .expect("validated: EC003 rejects a sequence naming an undeclared element")
}

fn render_predicate(predicate: &Predicate) -> String {
    match predicate {
        Predicate::LenAtLeast { path, min } => format!("{}.len() >= {min}", path.dotted()),
        Predicate::LenIs { path, len } => format!("{}.len() == {len}", path.dotted()),
        Predicate::In { path, allowed } => format!("{} in [{}]", path.dotted(), allowed.join(", ")),
        Predicate::IsSome { path } => format!("{}.is_some()", path.dotted()),
        Predicate::IsNone { path } => format!("{}.is_none()", path.dotted()),
        Predicate::All(children) => {
            let parts: Vec<String> = children.iter().map(render_predicate).collect();
            format!("all({})", parts.join(", "))
        }
        Predicate::Any(children) => {
            let parts: Vec<String> = children.iter().map(render_predicate).collect();
            format!("any({})", parts.join(", "))
        }
    }
}

/// Only ever called on a path EC032 guarantees is a single segment
/// (`LenAtLeast`/`LenIs` — `.last` deep paths are `In`/`IsSome`/`IsNone`
/// only, handled separately in `predicate_tokens`).
fn path_ident(path: &crate::model::FieldPath) -> proc_macro2::Ident {
    quote::format_ident!("{}", path.segments[0].value)
}

fn codec_of(fields: &[FieldBinding], field_name: &str) -> TokenStream {
    let binding = fields
        .iter()
        .find(|b| b.field.value == field_name)
        .expect("validated: EC010 rejects a require path naming a nonexistent field");
    let codec = match &binding.kind {
        FieldKind::Scalar { codec } => codec,
        FieldKind::Optional { inner } => match &**inner {
            FieldKind::Scalar { codec } => codec,
            _ => unreachable!(
                "validated: EC015 guarantees an In-predicate path resolves to a scalar kind"
            ),
        },
        FieldKind::Subtree { .. } | FieldKind::Sequence { .. } => {
            unreachable!(
                "validated: EC015 guarantees an In-predicate path resolves to a scalar kind"
            )
        }
    };
    parse_type(&codec.value)
}

/// Whether an `In` predicate's target field (as `codec_of` locates it) is
/// `Optional { Scalar }` — the case whose `matches!` gains a `None |` arm.
fn is_optional_scalar(fields: &[FieldBinding], field_name: &str) -> bool {
    let binding = fields
        .iter()
        .find(|b| b.field.value == field_name)
        .expect("validated: EC010 rejects a require path naming a nonexistent field");
    matches!(&binding.kind, FieldKind::Optional { inner } if matches!(**inner, FieldKind::Scalar { .. }))
}

fn parse_type(name: &str) -> TokenStream {
    let ty: syn::Type = syn::parse_str(name)
        .unwrap_or_else(|_| panic!("declared type `{name}` is not a parseable Rust type"));
    quote! { #ty }
}

fn pascal_ident(snake: &str) -> proc_macro2::Ident {
    quote::format_ident!("{}", crate::model::pascal_case(snake))
}

fn declaration_ident(group: &GroupDeclaration) -> proc_macro2::Ident {
    quote::format_ident!("{}_DECLARATION", group.name.value.to_uppercase())
}

fn declaration_static(group: &GroupDeclaration) -> TokenStream {
    let upper = declaration_ident(group);
    let name = group.name.value.as_str();
    let elements: Vec<TokenStream> = group
        .elements
        .iter()
        .map(|e| {
            let n = e.name.value.as_str();
            quote! { #n }
        })
        .collect();
    let element_data: Vec<TokenStream> = group.elements.iter().map(element_row).collect();
    let constructions: Vec<TokenStream> =
        group.constructions.iter().map(construction_row).collect();
    quote! {
        pub static #upper: ::deckmaste_construction_compiler::runtime::GroupData =
            ::deckmaste_construction_compiler::runtime::GroupData {
                name: #name,
                elements: &[#(#elements),*],
                element_data: &[#(#element_data),*],
                constructions: &[#(#constructions),*],
            };
    }
}

fn element_row(element: &ElementDeclaration) -> TokenStream {
    let name = element.name.value.as_str();
    let bind_path = element.bind_path.as_ref().map_or_else(
        || quote! { None },
        |path| {
            let path = path.value.as_str();
            quote! { Some(#path) }
        },
    );
    let fields: Vec<TokenStream> = element.fields.iter().map(field_row).collect();
    quote! {
        ::deckmaste_construction_compiler::runtime::ElementData {
            name: #name,
            bind_path: #bind_path,
            fields: &[#(#fields),*],
        }
    }
}

fn construction_row(construction: &ConstructionDeclaration) -> TokenStream {
    let id = construction.id.value.as_str();
    let category = construction.category.value.as_str();
    let internal = construction.internal;
    let deserialize = construction.deserialize;
    let selection_unique = matches!(
        construction.selection,
        crate::model::SelectionPromise::Unique
    );
    let (own_type, bind_path) = match &construction.ast {
        AstShape::Own { name, .. } => {
            let n = name.value.as_str();
            (quote! { Some(#n) }, quote! { None })
        }
        AstShape::Bind { path, .. } => {
            let p = path.value.as_str();
            (quote! { None }, quote! { Some(#p) })
        }
    };
    // `dominance` holds every edge this construction is named in, winner or
    // loser (see `edges_leaving_the_group_are_not_cycle_checked_here` in
    // validate.rs, whose edge lists exactly that shape). `dominates` means
    // "ids this construction beats" — only edges where it is the winner.
    let dominates: Vec<TokenStream> = construction
        .dominance
        .iter()
        .filter(|edge| edge.winner.value == construction.id.value)
        .map(|edge| {
            let loser = edge.loser.value.as_str();
            quote! { #loser }
        })
        .collect();
    let fields: Vec<TokenStream> = construction.ast.fields().iter().map(field_row).collect();
    let witnesses: Vec<TokenStream> = construction
        .witnesses
        .iter()
        .map(|witness| {
            let name = witness.name.value.as_str();
            let class = match &witness.class {
                crate::model::WitnessClass::Stored { path } => {
                    let dotted = path.dotted();
                    quote! { ::deckmaste_construction_compiler::runtime::WitnessClassData::Stored { path: #dotted } }
                }
                crate::model::WitnessClass::Derived { combinator, .. } => {
                    let combinator = combinator.value.as_str();
                    quote! { ::deckmaste_construction_compiler::runtime::WitnessClassData::Derived { combinator: #combinator } }
                }
                crate::model::WitnessClass::Free { ty } => {
                    let ty = ty.value.as_str();
                    quote! { ::deckmaste_construction_compiler::runtime::WitnessClassData::Free { ty: #ty } }
                }
            };
            quote! {
                ::deckmaste_construction_compiler::runtime::WitnessData { name: #name, class: #class }
            }
        })
        .collect();
    let forms: Vec<TokenStream> = construction
        .forms
        .iter()
        .map(|form| {
            let name = form.name.value.as_str();
            let ordinal = form.ordinal.value;
            let guarded = form.guard.is_some();
            let atoms: Vec<TokenStream> = form
                .surface
                .iter()
                .map(|atom| match atom {
                    crate::model::SurfaceAtom::Literal(s) => {
                        let s = s.value.as_str();
                        quote! { ::deckmaste_construction_compiler::runtime::AtomData::Literal(#s) }
                    }
                    crate::model::SurfaceAtom::Hole(path) => {
                        let dotted = path.dotted();
                        quote! { ::deckmaste_construction_compiler::runtime::AtomData::Hole(#dotted) }
                    }
                    crate::model::SurfaceAtom::Lexeme(path) => {
                        let dotted = path.dotted();
                        quote! { ::deckmaste_construction_compiler::runtime::AtomData::Lexeme(#dotted) }
                    }
                })
                .collect();
            quote! {
                ::deckmaste_construction_compiler::runtime::FormData {
                    name: #name,
                    ordinal: #ordinal,
                    guarded: #guarded,
                    atoms: &[#(#atoms),*],
                }
            }
        })
        .collect();
    quote! {
        ::deckmaste_construction_compiler::runtime::ConstructionData {
            id: #id,
            category: #category,
            internal: #internal,
            own_type: #own_type,
            bind_path: #bind_path,
            fields: &[#(#fields),*],
            witnesses: &[#(#witnesses),*],
            deserialize: #deserialize,
            selection_unique: #selection_unique,
            dominates: &[#(#dominates),*],
            forms: &[#(#forms),*],
        }
    }
}

fn field_row(binding: &FieldBinding) -> TokenStream {
    let name = binding.field.value.as_str();
    let kind = field_kind_row(&binding.kind);
    quote! {
        ::deckmaste_construction_compiler::runtime::FieldData { name: #name, kind: #kind }
    }
}

fn field_kind_row(kind: &FieldKind) -> TokenStream {
    match kind {
        FieldKind::Subtree { category, boxed } => {
            let category = category.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Subtree { category: #category, boxed: #boxed } }
        }
        FieldKind::Scalar { codec } => {
            let codec = codec.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Scalar { codec: #codec } }
        }
        FieldKind::Sequence { element } => {
            let element = element.value.as_str();
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Sequence { element: #element } }
        }
        FieldKind::Optional { inner } => {
            let inner = field_kind_row(inner);
            quote! { ::deckmaste_construction_compiler::runtime::FieldKindData::Optional { inner: &#inner } }
        }
    }
}

fn reexports(group: &GroupDeclaration) -> Vec<TokenStream> {
    let module = quote::format_ident!("__constructions_{}", group.name.value);
    // `DeclarationViolation` is not re-exported: it lives once in
    // `runtime.rs` (not minted per group), so there is no per-module item to
    // export — every generated door already names it by absolute path.
    let mut items: Vec<proc_macro2::Ident> = Vec::new();
    for element in &group.elements {
        if element.bind_path.is_none() {
            items.push(pascal_ident(&element.name.value));
        }
    }
    for construction in &group.constructions {
        match &construction.ast {
            AstShape::Own { name, .. } => items.push(quote::format_ident!("{}", name.value)),
            AstShape::Bind { .. } => {
                items.push(quote::format_ident!("build_{}", construction.id.value));
                items.push(quote::format_ident!("parts_{}", construction.id.value));
            }
        }
    }
    items.push(declaration_ident(group));
    items
        .iter()
        .map(|item| quote! { pub use #module::#item; })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::validate;

    #[test]
    fn own_mode_emission_parses_and_seals() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let tokens = emit_group(&validated);
        let file: syn::File = syn::parse2(tokens).expect("emission is a valid Rust file");
        let rendered = prettyplease::unparse(&file);
        assert!(
            rendered.contains("mod __constructions_"),
            "sealed module present"
        );
        assert!(
            rendered.contains("pub fn try_new("),
            "validated constructor present"
        );
        assert!(
            rendered.contains("DeclarationViolation"),
            "error class present"
        );
        assert!(
            !rendered.contains("pub conjunction"),
            "construction fields stay private"
        );
    }

    #[test]
    fn in_predicate_emits_a_closed_match() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("matches!(conjunction, Conjunction::And | Conjunction::Or)"),
            "closed-domain check, no wildcard: {rendered}"
        );
        assert!(
            rendered.contains("conjunction in [And, Or]"),
            "human-readable requirement string"
        );
    }

    #[test]
    fn last_path_in_emits_is_none_or_with_nested_optional_reading() {
        // `.last()`'s vacuous-on-empty layer must nest OUTSIDE Task 1's
        // vacuous-on-absent optional-`In` reading: `rest.last().is_none_or(
        // |member| matches!(member.comma, None | Some(Comma::Present)))`.
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("comma".to_owned()),
                kind: crate::model::FieldKind::Optional {
                    inner: Box::new(crate::model::FieldKind::Scalar {
                        codec: crate::model::Spanned::call_site("Comma".to_owned()),
                    }),
                },
            }],
        });
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("rest".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("rest"),
            ));
        group.constructions[0]
            .constraints
            .push(crate::model::Constraint::Require(
                crate::model::Spanned::call_site(crate::model::Predicate::In {
                    path: crate::model::FieldPath::call_site("rest.last.comma"),
                    allowed: vec!["Present".to_owned()],
                }),
            ));
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        // prettyplease line-wraps the chained call, so match the pieces
        // rather than one contiguous string: `.last()` (vacuous on empty)
        // wrapping a closure whose body is Task 1's `None | Some(...)`
        // optional-`In` reading — the two vacuous layers nesting correctly.
        assert!(
            rendered.contains(".last()"),
            "`.last()` present: {rendered}"
        );
        assert!(
            rendered.contains(".is_none_or(|member| {"),
            "`.is_none_or` closure present: {rendered}"
        );
        assert!(
            rendered.contains("matches!(member.comma, None | Some(Comma::Present))"),
            "nested optional-In reading inside the closure: {rendered}"
        );
        assert!(
            rendered.contains("rest.last.comma in [Present]"),
            "human-readable requirement string"
        );
    }

    #[test]
    fn bind_mode_emits_builder_and_destructurer() {
        let group = crate::validate::fixtures::minimal_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("pub fn build_"),
            "bind mode emits a checked builder: {rendered}"
        );
        assert!(
            rendered.contains("pub fn parts_"),
            "bind mode emits a destructurer: {rendered}"
        );
        assert!(
            !rendered.contains("fn try_new("),
            "bind mode still emits no sealed constructor"
        );
        assert!(
            !rendered.contains("impl<'de> serde::Deserialize"),
            "bind mode still has no serde door"
        );
        assert!(
            !rendered.contains("struct NounPhraseCoordination")
                && !rendered.contains("struct MinimalNode"),
            "bind mode emits no owned construction struct: {rendered}"
        );
    }

    #[test]
    fn opt_in_construction_gains_validating_deserialize() {
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.constructions[0].deserialize = true;
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(rendered.contains("impl<'de> serde::Deserialize<'de> for MinimalNode"));
        assert!(rendered.contains("struct Raw"), "private mirror present");
        assert!(
            rendered.contains("MinimalNode::try_new(raw.conjunction)"),
            "routes through the validator"
        );
        // The opt-out sibling property: absence.
        let plain = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&plain).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            !rendered.contains("Deserialize"),
            "opt-out means no impl at all"
        );
    }

    #[test]
    fn elements_reached_by_an_opt_in_construction_derive_deserialize() {
        // Synthetic: opt-in construction with a seq field. Not compiled
        // against serde here (no vocabulary codec is Deserialize); the
        // derive's presence is the structural pin until a real family
        // exercises the combination (Coverage item 9).
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.elements.push(crate::model::ElementDeclaration {
            name: crate::model::Spanned::call_site("m".to_owned()),
            bind_path: None,
            fields: vec![crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("tag".to_owned()),
                kind: crate::model::FieldKind::Subtree {
                    category: crate::model::Spanned::call_site("FixturePhrase".to_owned()),
                    boxed: false,
                },
            }],
        });
        if let crate::model::AstShape::Own { fields, .. } = &mut group.constructions[0].ast {
            fields.push(crate::model::FieldBinding {
                field: crate::model::Spanned::call_site("members".to_owned()),
                kind: crate::model::FieldKind::Sequence {
                    element: crate::model::Spanned::call_site("m".to_owned()),
                },
            });
        }
        group.constructions[0].forms[0]
            .surface
            .push(crate::model::SurfaceAtom::Hole(
                crate::model::FieldPath::call_site("members"),
            ));
        group.constructions[0].deserialize = true;
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains(
                "    #[derive(Debug, PartialEq, Eq)]\n    #[derive(serde::Deserialize)]\n    pub struct M"
            ),
            "reached element derives: {rendered}"
        );
    }

    #[test]
    fn declaration_data_table_is_emitted() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(rendered.contains("pub static NOUN_COORDINATION_DECLARATION"));
        assert!(rendered.contains("::deckmaste_construction_compiler::runtime::GroupData"));
    }

    #[test]
    fn field_and_witness_rows_are_emitted() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("fields: &["),
            "declaration data carries field rows: {rendered}"
        );
        assert!(
            rendered.contains("name: \"conjunction\""),
            "conjunction field row present: {rendered}"
        );
        assert!(
            rendered.contains("FieldKindData::Scalar { codec : \"Conjunction\" }")
                || rendered.contains("codec: \"Conjunction\""),
            "conjunction's kind is a Conjunction-codec scalar row: {rendered}"
        );
    }

    #[test]
    fn free_witness_group_emits_payload_assertion() {
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.constructions[0]
            .witnesses
            .push(crate::model::WitnessDeclaration {
                name: crate::model::Spanned::call_site("gap".to_owned()),
                class: crate::model::WitnessClass::Free {
                    ty: crate::model::Spanned::call_site("Comma".to_owned()),
                },
            });
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            rendered.contains("fn __assert_free_witness_payloads"),
            "a free witness renders the payload assertion fn: {rendered}"
        );
        assert!(
            rendered.contains("assert_payload :: < Comma > ()")
                || rendered.contains("assert_payload::<Comma>()"),
            "the assertion instantiates for the free witness's type: {rendered}"
        );
    }

    #[test]
    fn no_free_witness_means_no_payload_assertion() {
        let group = crate::validate::fixtures::minimal_own_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            !rendered.contains("__assert_free_witness_payloads"),
            "no free witness means no payload assertion fn: {rendered}"
        );
    }

    #[test]
    fn dominates_row_excludes_edges_where_this_construction_is_the_loser() {
        // This construction (`noun_phrase_coordination`) is the *loser* of
        // this edge, not the winner — an in-IR shape `validate()` permits
        // for external dominance (see
        // `edges_leaving_the_group_are_not_cycle_checked_here` in
        // validate.rs). Its own `dominates` row must not claim its own id.
        let mut group = crate::validate::fixtures::minimal_own_group();
        group.constructions[0]
            .dominance
            .push(crate::model::DominanceEdge {
                winner: crate::model::Spanned::call_site("noun_phrase_nominal".to_owned()),
                loser: crate::model::Spanned::call_site("noun_phrase_coordination".to_owned()),
            });
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        assert!(
            !rendered.contains("dominates: &[\"noun_phrase_coordination\"]"),
            "construction must not claim to dominate itself via an edge where it is the loser: {rendered}"
        );
    }
}
