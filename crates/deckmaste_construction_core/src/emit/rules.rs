use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::identifier::RULE_CATEGORY_TYPE;
use crate::identifier::RULE_CONSTRUCTION_TYPE;
use crate::identifier::RULE_ID_CONSTRUCTION;
use crate::identifier::RULE_ID_COUNT;
use crate::identifier::RULE_ID_INDEX;
use crate::identifier::RULE_ID_TYPE;
use crate::identifier::RULES_CONSTANT;
use crate::identifier::emitted_ident;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::AtomPlan;
use crate::semantic::AtomTerminal;
use crate::semantic::ConstructionPlan;
use crate::semantic::SemanticPlan;

pub(crate) fn emit(plan: &SemanticPlan) -> syn::Result<Vec<GeneratedItem>> {
    let category_type = ident(RULE_CATEGORY_TYPE);
    let construction_type = ident(RULE_CONSTRUCTION_TYPE);
    let rule_id_type = ident(RULE_ID_TYPE);
    let rule_id_count = ident(RULE_ID_COUNT);
    let rule_id_construction = ident(RULE_ID_CONSTRUCTION);
    let rule_id_index = ident(RULE_ID_INDEX);
    let rules_constant = ident(RULES_CONSTANT);
    let constructions = plan.constructions();
    let origins = construction_origins(constructions);
    let categories = category_names(constructions);
    let rule_ids = constructions
        .iter()
        .map(|construction| ident(construction.rule_id()))
        .collect::<Vec<_>>();
    let count = syn::LitInt::new(&constructions.len().to_string(), Span::call_site());
    let construction_matches = rule_ids.iter().map(|rule_id| {
        quote! { Self::#rule_id => Construction::#rule_id }
    });
    let rows = constructions
        .iter()
        .zip(&rule_ids)
        .map(|(construction, rule_id)| emit_rule(plan, construction, rule_id))
        .collect::<syn::Result<Vec<_>>>()?;

    let mut rule_origins = origins.clone();
    rule_origins.extend(
        plan.roots()
            .iter()
            .filter(|root| root.is_parse_entry())
            .map(|root| DeclarationKey::new(DeclarationKind::Root, root.category())),
    );
    Ok(vec![
        GeneratedItem::new(
            ItemKey::named_type(RULE_CATEGORY_TYPE),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum #category_type { #(#categories),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::named_type(RULE_CONSTRUCTION_TYPE),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum #construction_type { #(#rule_ids),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::named_type(RULE_ID_TYPE),
            quote! {
                #[repr(usize)]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum #rule_id_type { #(#rule_ids),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Impl {
                trait_name: None,
                self_ty: RULE_ID_TYPE.to_owned(),
            },
            quote! {
                impl RuleId {
                    #[cfg(test)]
                    pub(crate) const #rule_id_count: usize = #count;
                    pub(crate) const fn #rule_id_construction(self) -> Construction {
                        match self { #(#construction_matches,)* }
                    }
                    pub(crate) const fn #rule_id_index(self) -> usize { self as usize }
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Constant,
                name: RULES_CONSTANT.to_owned(),
            },
            quote! { pub(crate) const #rules_constant: &[Rule<Category, Lexical, RuleId>] = &[#(#rows),*]; },
            rule_origins,
        ),
    ])
}

fn emit_rule(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    rule_id: &syn::Ident,
) -> syn::Result<TokenStream> {
    let lhs = ident(construction.category());
    let mut rhs = construction
        .atoms()
        .iter()
        .map(|atom| emit_position(plan, construction, atom))
        .collect::<syn::Result<Vec<_>>>()?;
    if let Some(root) = plan.parse_root(construction.category()) {
        let punctuation = syn::LitStr::new(root.punctuation(), Span::call_site());
        rhs.push(quote! { L(Lexical::Literal(#punctuation)) });
        rhs.push(quote! { L(Lexical::EndOfInput) });
    }
    Ok(quote! { Rule { id: RuleId::#rule_id, lhs: Category::#lhs, rhs: &[#(#rhs),*] } })
}

fn emit_position(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    atom: &AtomPlan,
) -> syn::Result<TokenStream> {
    match atom {
        AtomPlan::Literal(literal) => {
            let literal = syn::LitStr::new(literal, Span::call_site());
            Ok(quote! { L(Lexical::Literal(#literal)) })
        }
        AtomPlan::Category { category, .. } => {
            let category = ident(category);
            Ok(quote! { N(Category::#category) })
        }
        AtomPlan::Lex { terminal, .. } | AtomPlan::Identity { terminal, .. } => {
            let lexical = lexical_variant(plan, terminal)?;
            Ok(quote! { L(#lexical) })
        }
        AtomPlan::Noun { terminal, .. } => {
            let lexical = lexical_variant(plan, terminal)?;
            let number = noun_number(plan, construction)?;
            Ok(quote! { L(#lexical(NounNumber::#number)) })
        }
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            let terminal = ident(terminal);
            let variant = ident(variant);
            Ok(quote! { L(Lexical::Verb(#terminal::#variant)) })
        }
    }
}

fn lexical_variant(plan: &SemanticPlan, name: &str) -> syn::Result<TokenStream> {
    match plan.atom_terminal(name)? {
        AtomTerminal::Vocab(vocab) => {
            let name = ident(vocab.name());
            Ok(quote! { Lexical::#name })
        }
        AtomTerminal::Binding(binding) => {
            let path = binding
                .lexical_variant()
                .ok_or_else(|| internal("atom-capable terminal binding has no lexical variant"))?;
            Ok(quote! { #path })
        }
    }
}

fn noun_number(plan: &SemanticPlan, construction: &ConstructionPlan) -> syn::Result<syn::Ident> {
    let equation = plan
        .feature_equations(construction.construction_id())
        .iter()
        .find(|equation| equation.target() == &FeaturePlace::Construction(Feature::Number))
        .ok_or_else(|| internal("noun atom has no validated construction number"))?;
    let name = match equation.value() {
        FeatureExpr::Constant(value) => match value.value() {
            FeatureValue::Singular => "Singular",
            FeatureValue::Plural => "Plural",
            FeatureValue::Bare | FeatureValue::ThirdPersonSingular => {
                return Err(internal("noun number has an agreement value"));
            }
        },
        FeatureExpr::MatchVocab { .. } | FeatureExpr::FromRole { .. } => "Either",
    };
    Ok(ident(name))
}

fn construction_origins(constructions: &[ConstructionPlan]) -> Vec<DeclarationKey> {
    constructions
        .iter()
        .map(|construction| {
            DeclarationKey::new(
                DeclarationKind::Construction,
                construction.construction_id(),
            )
        })
        .collect()
}

fn category_names(constructions: &[ConstructionPlan]) -> Vec<syn::Ident> {
    let mut names = Vec::<String>::new();
    for construction in constructions {
        let name = construction.category().to_owned();
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names.into_iter().map(|name| ident(&name)).collect()
}

fn ident(name: &str) -> syn::Ident {
    emitted_ident(name, Span::call_site())
}
fn internal(message: &str) -> syn::Error {
    syn::Error::new(Span::call_site(), message)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn role_derived_noun_requests_either_number() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::role_derived_noun_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).expect("role-derived noun rules lower");
        let rules = generated.last().expect("rules item").tokens.to_string();
        assert!(
            rules.contains("Lexical :: Head (NounNumber :: Either)"),
            "role-derived noun must let the scanner return either number: {rules}"
        );
    }

    #[test]
    fn vocab_matched_number_requests_either_for_every_noun() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(
                crate::test_support::vocab_matched_number_with_two_nouns_tokens(),
            )
            .unwrap(),
        )
        .unwrap();
        let generated =
            super::emit(validated.semantic()).expect("two dynamic nouns lower to rules");
        let rules = generated.last().expect("rules item").tokens.to_string();
        assert_eq!(
            rules
                .matches("Lexical :: Head (NounNumber :: Either)")
                .count(),
            2,
            "each noun scanner is independently unconstrained until build: {rules}"
        );
    }

    #[test]
    fn synthetic_projection_rules_have_exact_ids_rows_and_ownership() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::test_support::synthetic_projection_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(validated.semantic()).unwrap();
        assert_eq!(generated.len(), 5);
        let actual = generated
            .iter()
            .map(|item| syn::parse2::<syn::Item>(item.tokens.clone()).unwrap())
            .collect::<Vec<_>>();

        let enum_variants = |item: &syn::Item| {
            let syn::Item::Enum(item) = item else {
                panic!("expected enum");
            };
            item.variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            enum_variants(&actual[0]),
            ["Expr", "Predicate", "Tag", "Document"]
        );
        let ids = [
            "ExprLeaf",
            "ExprNested",
            "PredicateAction",
            "PredicateIdle",
            "TagSolo",
            "DocumentDocument",
        ];
        assert_eq!(enum_variants(&actual[1]), ids);
        assert_eq!(enum_variants(&actual[2]), ids);

        let expected_rules: syn::Item = syn::parse_quote! {
            pub(crate) const RULES: &[Rule<Category, Lexical, RuleId>] = &[
                Rule {
                    id: RuleId::ExprLeaf,
                    lhs: Category::Expr,
                    rhs: &[
                        L(Lexical::Mode),
                        L(Lexical::Resource(NounNumber::Either)),
                    ],
                },
                Rule {
                    id: RuleId::ExprNested,
                    lhs: Category::Expr,
                    rhs: &[
                        L(Lexical::Literal("nest")),
                        N(Category::Expr),
                        L(Lexical::Marker),
                    ],
                },
                Rule {
                    id: RuleId::PredicateAction,
                    lhs: Category::Predicate,
                    rhs: &[L(Lexical::Verb(ActionStem::Activate))],
                },
                Rule {
                    id: RuleId::PredicateIdle,
                    lhs: Category::Predicate,
                    rhs: &[L(Lexical::Literal("idle"))],
                },
                Rule {
                    id: RuleId::TagSolo,
                    lhs: Category::Tag,
                    rhs: &[L(Lexical::Mode)],
                },
                Rule {
                    id: RuleId::DocumentDocument,
                    lhs: Category::Document,
                    rhs: &[
                        N(Category::Expr),
                        N(Category::Predicate),
                        L(Lexical::Handle),
                        L(Lexical::Pair),
                        L(Lexical::Literal("!")),
                        L(Lexical::EndOfInput),
                    ],
                },
            ];
        };
        let normalize = |item: syn::Item| {
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: vec![item],
            })
        };
        assert_eq!(normalize(actual[4].clone()), normalize(expected_rules));

        let construction_origins = ["leaf", "nested", "action", "idle", "solo", "document"]
            .map(|name| (crate::DeclarationKind::Construction, name));
        for item in &generated[..4] {
            assert_eq!(
                item.origins
                    .iter()
                    .map(|origin| (origin.kind(), origin.name()))
                    .collect::<Vec<_>>(),
                construction_origins,
            );
        }
        assert_eq!(
            generated[4]
                .origins
                .iter()
                .map(|origin| (origin.kind(), origin.name()))
                .collect::<Vec<_>>(),
            construction_origins
                .into_iter()
                .chain([(crate::DeclarationKind::Root, "Document")])
                .collect::<Vec<_>>(),
        );
    }
    #[test]
    fn root_eoi_and_number_equations_control_rule_projection() {
        let expansion = crate::generate(quote! {
            codec Object {
                atom = noun;
                value_type = Object;
                lexical = Lexical::Object;
                render = render_object;
                build { pattern = BuildValue::Object(object); construct = object; }
                traversal { callback = borrowed; argument = object; call visitor::visit_object(borrowed(object)); }
            }
            construction one: Phrase {
                element One { object: lex Object, }
                derive number = Values::Singular;
                form one = noun(object);
            }
            root Phrase { punctuation = "!"; eoi = true; standalone_render = true; }
            root Other { punctuation = "."; eoi = false; standalone_render = false; }
        });
        assert!(
            expansion.is_err(),
            "undeclared render-only roots remain invalid"
        );

        let validated = crate::validate_declarations(
            crate::parse_declarations(quote! {
                codec Object {
                    atom = noun;
                    value_type = Object;
                    lexical = Lexical::Object;
                    render = render_object;
                    build { pattern = BuildValue::Object(object); construct = object; }
                    traversal { callback = borrowed; argument = object; call visitor::visit_object(borrowed(object)); }
                }
                construction one: Phrase {
                    element One { object: lex Object, }
                    derive number = Values::Singular;
                    form one = noun(object);
                }
                root Phrase { punctuation = "!"; eoi = true; standalone_render = true; }
            }).unwrap(),
        ).unwrap();
        let items = super::emit(validated.semantic()).unwrap();
        let rules = items.last().unwrap().tokens.to_string();
        assert!(rules.contains("NounNumber :: Singular"));
        assert!(rules.contains("Lexical :: Literal (\"!\")"));
        assert!(rules.contains("Lexical :: EndOfInput"));
    }
}
