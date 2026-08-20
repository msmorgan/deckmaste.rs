use std::collections::HashMap;
use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::key as identifier_key;
use crate::identifier::local_name;
use crate::identifier::spelling_key;
use crate::semantic::InvariantPlan;
use crate::semantic::PredicateMemberPlan;
use crate::semantic::PredicateSubjectPlan;

pub(crate) mod ast;
pub(crate) mod build;
pub(crate) mod render;
pub(crate) mod rules;
pub(crate) mod runtime;
pub(crate) mod scanner;
pub(crate) mod terminal;
pub(crate) mod visit;

const RUST_SOURCE_MARGIN: usize = 100;

pub(super) fn emit_invariant_expression(
    invariant: &InvariantPlan,
    subjects: &HashMap<String, TokenStream>,
) -> syn::Result<TokenStream> {
    if invariant.alternatives().is_empty() {
        return Err(internal_invariant_expression(
            "sealed predicate has no alternatives",
        ));
    }
    let alternatives = invariant
        .alternatives()
        .iter()
        .map(|alternative| {
            let atoms = alternative
                .atoms()
                .iter()
                .map(|atom| {
                    let subject = atom.subject();
                    let key = subject.semantic_key();
                    let expression = subjects.get(&key).ok_or_else(|| {
                        internal_invariant_expression(&format!(
                            "missing typed subject expression `{key}`"
                        ))
                    })?;
                    emit_predicate_atom(subject, atom.allowed(), expression)
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(if atoms.is_empty() {
                quote! { true }
            } else {
                quote! { #(#atoms)&&* }
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(quote! { #(#alternatives)||* })
}

fn emit_predicate_atom(
    subject: &PredicateSubjectPlan,
    allowed: &[PredicateMemberPlan],
    expression: &TokenStream,
) -> syn::Result<TokenStream> {
    if allowed.is_empty() {
        return Err(internal_invariant_expression(
            "sealed predicate atom has no allowed members",
        ));
    }
    match subject {
        PredicateSubjectPlan::CategoryRole { category, .. } => {
            let category = local_ident(category);
            let variants = allowed
                .iter()
                .map(|member| match member {
                    PredicateMemberPlan::Variant(variant) => Ok(variant),
                    PredicateMemberPlan::Feature(_) => Err(internal_invariant_expression(
                        "category subject has a feature member",
                    )),
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! { matches!(#expression, #(#category::#variants(_))|*) })
        }
        PredicateSubjectPlan::VocabRole { terminal, .. } => {
            let terminal = local_ident(terminal);
            let variants = allowed
                .iter()
                .map(|member| match member {
                    PredicateMemberPlan::Variant(variant) => Ok(variant),
                    PredicateMemberPlan::Feature(_) => Err(internal_invariant_expression(
                        "vocabulary subject has a feature member",
                    )),
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! { matches!(#expression, #(#terminal::#variants)|*) })
        }
        PredicateSubjectPlan::RoleFeature { feature, .. }
        | PredicateSubjectPlan::ConstructionFeature(feature) => {
            let feature_type = match feature {
                crate::feature::Feature::Agreement => local_ident("Agreement"),
                crate::feature::Feature::Number => local_ident("Number"),
            };
            let members = allowed
                .iter()
                .map(|member| {
                    let PredicateMemberPlan::Feature(member) = member else {
                        return Err(internal_invariant_expression(
                            "feature subject has a variant member",
                        ));
                    };
                    if !feature.domain().contains(member.value()) {
                        return Err(internal_invariant_expression(
                            "feature subject member is outside its sealed domain",
                        ));
                    }
                    Ok(local_ident(member.value().key()))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! { matches!(#expression, #(#feature_type::#members)|*) })
        }
    }
}

fn internal_invariant_expression(detail: &str) -> syn::Error {
    syn::Error::new(
        proc_macro2::Span::call_site(),
        format!("internal invariant expression emitter: {detail}"),
    )
}

#[derive(Clone, Default)]
pub(super) struct LocalAllocator {
    used: HashSet<String>,
}

impl LocalAllocator {
    pub(super) fn reserve(&mut self, name: impl AsRef<str>) {
        self.used.insert(spelling_key(name.as_ref()));
    }

    pub(super) fn reserve_ident(&mut self, name: &syn::Ident) {
        self.used.insert(identifier_key(name));
    }

    pub(super) fn allocate(&mut self, preferred: &str) -> syn::Ident {
        let preferred = local_name(preferred);
        if self.used.insert(preferred.clone()) {
            return local_ident(&preferred);
        }
        for suffix in 2.. {
            let candidate = format!("{preferred}_{suffix}");
            if self.used.insert(candidate.clone()) {
                return local_ident(&candidate);
            }
        }
        unreachable!("the local binder suffix space is unbounded")
    }

    pub(super) fn allocate_ident(&mut self, preferred: &syn::Ident) -> syn::Ident {
        self.allocate(&identifier_key(preferred))
    }
}

fn local_ident(name: &str) -> syn::Ident {
    crate::identifier::emitted_ident(name, proc_macro2::Span::call_site())
}

pub(super) fn call_match_arm(
    pattern: &TokenStream,
    call: &TokenStream,
    indentation: usize,
) -> TokenStream {
    let candidate = quote! { #pattern => #call, };
    if indentation + compact_rust_width(&candidate) + 1 >= RUST_SOURCE_MARGIN {
        quote! { #pattern => { #call; } }
    } else {
        quote! { #pattern => #call }
    }
}

fn compact_rust_width(tokens: &TokenStream) -> usize {
    tokens
        .to_string()
        .replace(" :: ", "::")
        .replace(" . ", ".")
        .replace(" (", "(")
        .replace("[ ", "[")
        .replace("{ ", "{")
        .replace(" )", ")")
        .replace(" ]", "]")
        .replace(" }", "}")
        .replace(" ,", ",")
        .replace(" ;", ";")
        .chars()
        .count()
}

pub(super) fn declaration_kind(kind: macro_ron::v2::DeclarationKind) -> TokenStream {
    use macro_ron::v2::DeclarationKind;

    match kind {
        DeclarationKind::KeywordAction => {
            quote! { ::macro_ron::v2::DeclarationKind::KeywordAction }
        }
        DeclarationKind::KeywordAbility => {
            quote! { ::macro_ron::v2::DeclarationKind::KeywordAbility }
        }
        DeclarationKind::Type => quote! { ::macro_ron::v2::DeclarationKind::Type },
        DeclarationKind::CounterKind => quote! { ::macro_ron::v2::DeclarationKind::CounterKind },
        DeclarationKind::Designation => quote! { ::macro_ron::v2::DeclarationKind::Designation },
        DeclarationKind::Subtype(_) => unreachable!("open_verb validation excludes subtype kinds"),
    }
}

pub(super) fn grammar_position(position: macro_ron::v2::GrammarPosition) -> TokenStream {
    use macro_ron::v2::GrammarPosition;

    match position {
        GrammarPosition::Verb => quote! { ::macro_ron::v2::GrammarPosition::Verb },
        GrammarPosition::Noun => quote! { ::macro_ron::v2::GrammarPosition::Noun },
        GrammarPosition::FixedTerm => quote! { ::macro_ron::v2::GrammarPosition::FixedTerm },
        GrammarPosition::FixedClause => quote! { ::macro_ron::v2::GrammarPosition::FixedClause },
        GrammarPosition::FixedKeyword => quote! { ::macro_ron::v2::GrammarPosition::FixedKeyword },
    }
}

pub(super) fn closed_lexeme_owner_id(
    declaration: &str,
    member: &str,
    feature: macro_ron::v2::SurfaceFeature,
) -> syn::LitStr {
    let feature = match feature {
        macro_ron::v2::SurfaceFeature::Bare => "bare",
        macro_ron::v2::SurfaceFeature::ThirdPersonSingular => "third_person_singular",
        macro_ron::v2::SurfaceFeature::Singular => "singular",
        macro_ron::v2::SurfaceFeature::Plural => "plural",
        macro_ron::v2::SurfaceFeature::Fixed => {
            unreachable!("closed lexemes use only Agreement or Number features")
        }
    };
    syn::LitStr::new(
        &format!("lexeme:{declaration}/{member}/{feature}"),
        proc_macro2::Span::call_site(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn local_allocator_keys_raw_names_semantically_and_legalizes_keywords() {
        let mut allocator = super::LocalAllocator::default();
        allocator.reserve("context");

        let cases = [
            ("r#payload", "payload"),
            ("payload", "payload_2"),
            ("r#context", "context_2"),
            ("where", "where_value"),
            ("r#where", "where_value_2"),
            ("r#self", "self_value"),
            ("Self", "self_value_2"),
            ("super", "super_value"),
            ("crate", "crate_value"),
            (
                "not an ident",
                "_generated_local_6e_6f_74_20_61_6e_20_69_64_65_6e_74",
            ),
        ];
        for (preferred, expected) in cases {
            assert_eq!(allocator.allocate(preferred), expected, "{preferred}");
        }
    }
}
