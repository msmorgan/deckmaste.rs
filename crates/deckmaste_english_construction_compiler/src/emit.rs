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

pub fn emit_group(validated: &ValidatedGroup<'_>) -> TokenStream {
    let group = validated.group();
    let module = quote::format_ident!("__constructicon_{}", group.name.value);
    let violation = violation_struct();
    let serde_reached = serde_reached_elements(group);
    let elements: Vec<TokenStream> = group
        .elements
        .iter()
        .map(|element| {
            let serde = serde_reached
                .iter()
                .any(|reached| *reached == element.name.value);
            element_struct(element, serde)
        })
        .collect();
    let constructions: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(|c| own_construction(group, c))
        .collect();
    let deserialize_impls: Vec<TokenStream> = group
        .constructions
        .iter()
        .filter_map(deserialize_impl)
        .collect();
    let declaration = declaration_static(group);
    let reexports = reexports(group);
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
            #violation
            #(#elements)*
            #(#constructions)*
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
    return reached;

    fn enqueue_sequences<'g>(kind: &'g FieldKind, queue: &mut Vec<&'g str>) {
        match kind {
            FieldKind::Sequence { element } => queue.push(element.value.as_str()),
            FieldKind::Optional { inner } => enqueue_sequences(inner, queue),
            FieldKind::Subtree { .. } | FieldKind::Scalar { .. } => {}
        }
    }
}

fn violation_struct() -> TokenStream {
    quote! {
        /// A `require` clause an ingress value failed. One error class for
        /// every generated door: `try_new` and validating deserialization.
        #[derive(Debug, PartialEq, Eq)]
        pub struct DeclarationViolation {
            pub construction: &'static str,
            pub requirement: &'static str,
        }
    }
}

fn element_struct(element: &ElementDeclaration, serde: bool) -> TokenStream {
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

/// Serde-opt-in own constructions only; `None` for opt-out and for bind
/// mode (whose own-struct emission, and so its deserialize impl, land with
/// the chart adapter).
fn deserialize_impl(construction: &ConstructionDeclaration) -> Option<TokenStream> {
    if !construction.deserialize {
        return None;
    }
    let AstShape::Own { name, fields } = &construction.ast else { return None };
    let ty = quote::format_ident!("{}", name.value);
    let raw_fields: Vec<TokenStream> = fields
        .iter()
        .map(|binding| {
            let field = quote::format_ident!("{}", binding.field.value);
            let field_ty = field_type(&binding.kind);
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
        FieldKind::Subtree { .. } | FieldKind::Sequence { .. } => {
            unreachable!(
                "validated: EC015 guarantees an In-predicate path resolves to a scalar kind"
            )
        }
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
    let constructions: Vec<TokenStream> =
        group.constructions.iter().map(construction_row).collect();
    quote! {
        pub static #upper: ::deckmaste_english_construction_compiler::runtime::GroupData =
            ::deckmaste_english_construction_compiler::runtime::GroupData {
                name: #name,
                elements: &[#(#elements),*],
                constructions: &[#(#constructions),*],
            };
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
                        quote! { ::deckmaste_english_construction_compiler::runtime::AtomData::Literal(#s) }
                    }
                    crate::model::SurfaceAtom::Hole(path) => {
                        let dotted = path.dotted();
                        quote! { ::deckmaste_english_construction_compiler::runtime::AtomData::Hole(#dotted) }
                    }
                    crate::model::SurfaceAtom::Lexeme(path) => {
                        let dotted = path.dotted();
                        quote! { ::deckmaste_english_construction_compiler::runtime::AtomData::Lexeme(#dotted) }
                    }
                })
                .collect();
            quote! {
                ::deckmaste_english_construction_compiler::runtime::FormData {
                    name: #name,
                    ordinal: #ordinal,
                    guarded: #guarded,
                    atoms: &[#(#atoms),*],
                }
            }
        })
        .collect();
    quote! {
        ::deckmaste_english_construction_compiler::runtime::ConstructionData {
            id: #id,
            category: #category,
            internal: #internal,
            own_type: #own_type,
            bind_path: #bind_path,
            deserialize: #deserialize,
            selection_unique: #selection_unique,
            dominates: &[#(#dominates),*],
            forms: &[#(#forms),*],
        }
    }
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
        assert!(rendered.contains("::deckmaste_english_construction_compiler::runtime::GroupData"));
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
