//! Deterministic code emission. Input is [`crate::validate::ValidatedGroup`]
//! by construction — the validator is the only way to obtain one. Iteration
//! is over declaration-order `Vec`s exclusively; determinism is structural,
//! not an afterthought.

use proc_macro2::TokenStream;
use quote::quote;

use crate::model::AstShape;
use crate::model::Constraint;
use crate::model::ConstructionDeclaration;
use crate::model::FieldBinding;
use crate::model::FieldKind;
use crate::model::GroupDeclaration;
use crate::model::Predicate;
use crate::validate::ValidatedGroup;

pub fn emit_group(validated: &ValidatedGroup<'_>) -> TokenStream {
    let group = validated.group();
    let module = quote::format_ident!("__constructicon_{}", group.name.value);
    let violation = violation_struct();
    let elements: Vec<TokenStream> = group.elements.iter().map(element_struct).collect();
    let constructions: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|c| own_construction(group, c))
        .collect();
    let reexports = reexports(group);
    quote! {
        mod #module {
            use super::*;
            #violation
            #(#elements)*
            #(#constructions)*
        }
        #(#reexports)*
    }
}

fn violation_struct() -> TokenStream {
    quote! {
        /// A `require` clause an ingress value failed. One error class for
        /// every generated door: try_new and validating deserialization.
        #[derive(Debug, PartialEq, Eq)]
        pub struct DeclarationViolation {
            pub construction: &'static str,
            pub requirement: &'static str,
        }
    }
}

fn element_struct(element: &crate::model::ElementDeclaration) -> TokenStream {
    let name = pascal_ident(&element.name.value);
    let fields: Vec<TokenStream> = element
        .fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let ty = field_type(&binding.kind);
            quote! { pub #field: #ty, }
        })
        .collect();
    quote! {
        // Elements carry no declaration invariants, so their fields stay
        // public; the owning construction validates the sequence whole.
        #[derive(Debug, PartialEq, Eq)]
        pub struct #name {
            #(#fields)*
        }
    }
}

fn own_construction(
    _group: &GroupDeclaration,
    construction: &ConstructionDeclaration,
) -> Option<TokenStream> {
    let AstShape::Own { name, fields } = &construction.ast else {
        return None; // bind mode: declaration-data row only until the chart adapter lands
    };
    let ty = quote::format_ident!("{}", name.value);
    let id = construction.id.value.as_str();
    let field_decls: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(&binding.kind);
            quote! { #field: #field_ty, }
        })
        .collect();
    let params: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(&binding.kind);
            quote! { #field: #field_ty }
        })
        .collect();
    let checks: Vec<TokenStream> = construction
        .constraints
        .iter()
        .filter_map(|constraint| match constraint {
            Constraint::Require(predicate) => Some(require_check(id, fields, &predicate.value)),
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
            let field_ty = field_type(&binding.kind);
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
            pub fn try_new(#(#params),*) -> Result<Self, DeclarationViolation> {
                #(#checks)*
                Ok(Self { #(#field_names),* })
            }
            #(#accessors)*
        }
    })
}

fn field_type(kind: &FieldKind) -> TokenStream {
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
            let ty = pascal_ident(&element.value);
            quote! { Vec<#ty> }
        }
        FieldKind::Optional { inner } => {
            let ty = field_type(inner);
            quote! { Option<#ty> }
        }
    }
}

fn require_check(id: &str, fields: &[FieldBinding], predicate: &Predicate) -> TokenStream {
    let condition = predicate_tokens(fields, predicate);
    let requirement = render_predicate(predicate);
    quote! {
        if !(#condition) {
            return Err(DeclarationViolation { construction: #id, requirement: #requirement });
        }
    }
}

/// EC032 guarantees every path here is a single segment naming a direct
/// field, and EC015 guarantees kind agreement (`In` targets a scalar,
/// `len()` targets a sequence) — so this mapping is total for validated
/// input.
fn predicate_tokens(fields: &[FieldBinding], predicate: &Predicate) -> TokenStream {
    match predicate {
        Predicate::LenAtLeast { path, min } => {
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
        Predicate::In { path, allowed } => {
            let field = path_ident(path);
            let codec = codec_of(fields, path);
            let variants: Vec<TokenStream> = allowed
                .iter()
                .map(|variant| {
                    let v = quote::format_ident!("{}", variant);
                    quote! { #codec::#v }
                })
                .collect();
            quote! { matches!(#field, #(#variants)|*) }
        }
        Predicate::IsSome { path } => {
            let field = path_ident(path);
            quote! { #field.is_some() }
        }
        Predicate::IsNone { path } => {
            let field = path_ident(path);
            quote! { #field.is_none() }
        }
        Predicate::All(children) => {
            let parts: Vec<TokenStream> = children
                .iter()
                .map(|c| predicate_tokens(fields, c))
                .collect();
            quote! { (#(#parts)&&*) }
        }
        Predicate::Any(children) => {
            let parts: Vec<TokenStream> = children
                .iter()
                .map(|c| predicate_tokens(fields, c))
                .collect();
            quote! { (#(#parts)||*) }
        }
    }
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

fn path_ident(path: &crate::model::FieldPath) -> proc_macro2::Ident {
    quote::format_ident!("{}", path.segments[0].value)
}

fn codec_of(fields: &[FieldBinding], path: &crate::model::FieldPath) -> TokenStream {
    let binding = fields
        .iter()
        .find(|b| b.field.value == path.segments[0].value)
        .expect("validated: EC010 rejects a require path naming a nonexistent field");
    let codec = match &binding.kind {
        FieldKind::Scalar { codec } => codec,
        FieldKind::Optional { inner } => match &**inner {
            FieldKind::Scalar { codec } => codec,
            _ => unreachable!(
                "validated: EC015 guarantees an In-predicate path resolves to a scalar kind"
            ),
        },
        _ => unreachable!(
            "validated: EC011/EC014 guarantee an In-predicate path resolves to a scalar kind"
        ),
    };
    parse_type(&codec.value)
}

fn parse_type(name: &str) -> TokenStream {
    let ty: syn::Type = syn::parse_str(name)
        .unwrap_or_else(|_| panic!("declared type `{name}` is not a parseable Rust type"));
    quote! { #ty }
}

fn pascal_ident(snake: &str) -> proc_macro2::Ident {
    let mut out = String::new();
    for part in snake.split('_') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    quote::format_ident!("{}", out)
}

fn reexports(group: &GroupDeclaration) -> Vec<TokenStream> {
    let module = quote::format_ident!("__constructicon_{}", group.name.value);
    let mut items: Vec<proc_macro2::Ident> = vec![quote::format_ident!("DeclarationViolation")];
    for element in &group.elements {
        items.push(pascal_ident(&element.name.value));
    }
    for construction in &group.constructions {
        if let AstShape::Own { name, .. } = &construction.ast {
            items.push(quote::format_ident!("{}", name.value));
        }
    }
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
            rendered.contains("mod __constructicon_"),
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
    fn bind_mode_emits_nothing() {
        let group = crate::validate::fixtures::minimal_group();
        let validated = validate(&group).expect("fixture validates");
        let rendered = prettyplease::unparse(&syn::parse2(emit_group(&validated)).expect("parses"));
        // minimal_group's sole construction is Bind-mode; the only emitted
        // items should be the module scaffold, DeclarationViolation, and its
        // reexport — no construction-specific struct, try_new, or accessor.
        assert!(
            !rendered.contains("fn try_new("),
            "bind mode must not emit a constructor: {rendered}"
        );
        assert!(
            !rendered.contains("struct NounPhraseCoordination")
                && !rendered.contains("struct MinimalNode"),
            "bind mode must not emit a construction struct: {rendered}"
        );
    }
}
