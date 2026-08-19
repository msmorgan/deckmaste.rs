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
use crate::model::TerminalBindingKind;
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
            quote! { pub(crate) const #rules_constant: &[Rule<Category, LexicalTerminal, RuleId>] = &[#(#rows),*]; },
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
        .enumerate()
        .map(|(index, atom)| emit_position(plan, construction, index, atom))
        .collect::<syn::Result<Vec<_>>>()?;
    if let Some(root) = plan.parse_root(construction.category()) {
        let punctuation = syn::LitStr::new(root.punctuation(), Span::call_site());
        let stable_id = syn::LitStr::new(
            &format!("root:{}/punctuation", root.category()),
            Span::call_site(),
        );
        rhs.push(lexical_terminal(
            &quote! { Lexical::Literal(#punctuation) },
            &quote! {
                LexicalOwnerTemplate::Static {
                    kind: LexicalProvenanceKind::FormLiteral,
                    stable_id: #stable_id,
                }
            },
        ));
        rhs.push(lexical_terminal(
            &quote! { Lexical::EndOfInput },
            &quote! { LexicalOwnerTemplate::None },
        ));
    }
    Ok(quote! { Rule { id: RuleId::#rule_id, lhs: Category::#lhs, rhs: &[#(#rhs),*] } })
}

fn emit_position(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
    atom_index: usize,
    atom: &AtomPlan,
) -> syn::Result<TokenStream> {
    match atom {
        AtomPlan::Literal(literal) => {
            let literal = syn::LitStr::new(literal, Span::call_site());
            let stable_id = syn::LitStr::new(
                &format!(
                    "form:{}/{}/{}",
                    construction.construction_id(),
                    construction.form(),
                    atom_index
                ),
                Span::call_site(),
            );
            Ok(lexical_terminal(
                &quote! { Lexical::Literal(#literal) },
                &quote! {
                    LexicalOwnerTemplate::Static {
                        kind: LexicalProvenanceKind::FormLiteral,
                        stable_id: #stable_id,
                    }
                },
            ))
        }
        AtomPlan::Category { category, .. } => {
            let category = ident(category);
            Ok(quote! { N(Category::#category) })
        }
        AtomPlan::Lex { terminal, .. } | AtomPlan::Identity { terminal, .. } => {
            let lexical = lexical_variant(plan, terminal)?;
            let owner = owner_template(plan, terminal)?;
            Ok(lexical_terminal(&lexical, &owner))
        }
        AtomPlan::Noun { terminal, .. } => {
            let lexical = lexical_variant(plan, terminal)?;
            let number = noun_number(plan, construction)?;
            let owner = owner_template(plan, terminal)?;
            Ok(lexical_terminal(&quote! { #lexical(#number) }, &owner))
        }
        AtomPlan::VerbFixed {
            terminal, variant, ..
        } => {
            let terminal = ident(terminal);
            let variant = ident(variant);
            let declaration = syn::LitStr::new(&terminal.to_string(), Span::call_site());
            let member = syn::LitStr::new(&variant.to_string(), Span::call_site());
            Ok(lexical_terminal(
                &quote! { Lexical::Verb(#terminal::#variant) },
                &quote! {
                    LexicalOwnerTemplate::Lexeme {
                        declaration: #declaration,
                        member: #member,
                    }
                },
            ))
        }
        AtomPlan::OpenDeclaration(open) => {
            let kind = crate::emit::declaration_kind(open.kind());
            let name = syn::LitStr::new(open.name(), Span::call_site());
            let position = crate::emit::grammar_position(open.position());
            let feature = open_verb_feature(plan, construction)?;
            Ok(lexical_terminal(
                &quote! {
                    Lexical::Declaration(DeclarationMatcher {
                        kind: #kind,
                        name: #name,
                        position: #position,
                        feature: #feature,
                    })
                },
                &quote! { LexicalOwnerTemplate::Declaration { kind: #kind, name: #name } },
            ))
        }
    }
}

pub(crate) fn open_verb_feature(
    plan: &SemanticPlan,
    construction: &ConstructionPlan,
) -> syn::Result<TokenStream> {
    let target = FeaturePlace::Role {
        field: syn::Ident::new("verb", construction.origin_span()),
        feature: Feature::Agreement,
    };
    match plan.feature_resolution(construction.construction_id(), &target) {
        Some(crate::feature::FeatureResolution::Known(value)) => match value {
            FeatureValue::Bare => Ok(quote! {
                FeatureConstraint::Exact(::macro_ron::v2::SurfaceFeature::Bare)
            }),
            FeatureValue::ThirdPersonSingular => Ok(quote! {
                FeatureConstraint::Exact(::macro_ron::v2::SurfaceFeature::ThirdPersonSingular)
            }),
            FeatureValue::Singular | FeatureValue::Plural => {
                Err(internal("open verb agreement has a number value"))
            }
        },
        Some(
            crate::feature::FeatureResolution::External
            | crate::feature::FeatureResolution::Runtime,
        ) => Ok(quote! { FeatureConstraint::Any }),
        None => Err(internal("open verb has no sealed agreement resolution")),
    }
}

fn lexical_terminal(matcher: &TokenStream, owner: &TokenStream) -> TokenStream {
    quote! {
        L(LexicalTerminal { matcher: #matcher, owner: #owner })
    }
}

fn owner_template(plan: &SemanticPlan, terminal: &str) -> syn::Result<TokenStream> {
    let declaration = syn::LitStr::new(terminal, Span::call_site());
    match plan.atom_terminal(terminal)? {
        AtomTerminal::Vocab(_) => Ok(quote! {
            LexicalOwnerTemplate::Vocab { declaration: #declaration }
        }),
        AtomTerminal::Binding(binding) => {
            let (kind, prefix) = match binding.kind() {
                TerminalBindingKind::Codec => (quote! { LexicalProvenanceKind::Codec }, "codec"),
                TerminalBindingKind::Identity => {
                    (quote! { LexicalProvenanceKind::Identity }, "identity")
                }
            };
            let stable_id = syn::LitStr::new(&format!("{prefix}:{terminal}"), Span::call_site());
            Ok(quote! {
                LexicalOwnerTemplate::Static { kind: #kind, stable_id: #stable_id }
            })
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
            if binding.codec_atom() == Some(crate::model::CodecAtomClass::Noun) {
                return Ok(quote! { Lexical::Noun });
            }
            let path = binding
                .lexical_variant()
                .ok_or_else(|| internal("atom-capable terminal binding has no lexical variant"))?;
            Ok(quote! { #path })
        }
    }
}

fn noun_number(plan: &SemanticPlan, construction: &ConstructionPlan) -> syn::Result<TokenStream> {
    let equation = plan
        .feature_equations(construction.construction_id())
        .iter()
        .find(|equation| equation.target() == &FeaturePlace::Construction(Feature::Number))
        .ok_or_else(|| internal("noun atom has no validated construction number"))?;
    match equation.value() {
        FeatureExpr::Constant(value) => match value.value() {
            FeatureValue::Singular => Ok(quote! { FeatureConstraint::Exact(Number::Singular) }),
            FeatureValue::Plural => Ok(quote! { FeatureConstraint::Exact(Number::Plural) }),
            FeatureValue::Bare | FeatureValue::ThirdPersonSingular => {
                Err(internal("noun number has an agreement value"))
            }
        },
        FeatureExpr::MatchVocab { .. } | FeatureExpr::FromRole { .. } => {
            Ok(quote! { FeatureConstraint::Any })
        }
    }
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
            rules.contains("Lexical :: Noun (FeatureConstraint :: Any)"),
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
                .matches("Lexical :: Noun (FeatureConstraint :: Any)")
                .count(),
            2,
            "each noun scanner is independently unconstrained until build: {rules}"
        );
    }

    fn expected_synthetic_projection_rules() -> syn::Item {
        syn::parse_quote! {
            pub(crate) const RULES: &[Rule<Category, LexicalTerminal, RuleId>] = &[
                Rule {
                    id: RuleId::ExprLeaf,
                    lhs: Category::Expr,
                    rhs: &[
                        L(LexicalTerminal {
                            matcher: Lexical::Mode,
                            owner: LexicalOwnerTemplate::Vocab { declaration: "Mode" },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Noun(FeatureConstraint::Any),
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Resource",
                            },
                        }),
                    ],
                },
                Rule {
                    id: RuleId::ExprNested,
                    lhs: Category::Expr,
                    rhs: &[
                        L(LexicalTerminal {
                            matcher: Lexical::Literal("nest"),
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::FormLiteral,
                                stable_id: "form:nested/nested/0",
                            },
                        }),
                        N(Category::Expr),
                        L(LexicalTerminal {
                            matcher: Lexical::Marker,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Marker",
                            },
                        }),
                    ],
                },
                Rule {
                    id: RuleId::PredicateAction,
                    lhs: Category::Predicate,
                    rhs: &[L(LexicalTerminal {
                        matcher: Lexical::Verb(ActionStem::Activate),
                        owner: LexicalOwnerTemplate::Lexeme {
                            declaration: "ActionStem",
                            member: "Activate",
                        },
                    })],
                },
                Rule {
                    id: RuleId::PredicateIdle,
                    lhs: Category::Predicate,
                    rhs: &[L(LexicalTerminal {
                        matcher: Lexical::Literal("idle"),
                        owner: LexicalOwnerTemplate::Static {
                            kind: LexicalProvenanceKind::FormLiteral,
                            stable_id: "form:idle/idle/0",
                        },
                    })],
                },
                Rule {
                    id: RuleId::TagSolo,
                    lhs: Category::Tag,
                    rhs: &[L(LexicalTerminal {
                        matcher: Lexical::Mode,
                        owner: LexicalOwnerTemplate::Vocab { declaration: "Mode" },
                    })],
                },
                Rule {
                    id: RuleId::DocumentDocument,
                    lhs: Category::Document,
                    rhs: &[
                        N(Category::Expr),
                        N(Category::Predicate),
                        L(LexicalTerminal {
                            matcher: Lexical::Handle,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Identity,
                                stable_id: "identity:Handle",
                            },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Pair,
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::Codec,
                                stable_id: "codec:Pair",
                            },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::Literal("!"),
                            owner: LexicalOwnerTemplate::Static {
                                kind: LexicalProvenanceKind::FormLiteral,
                                stable_id: "root:Document/punctuation",
                            },
                        }),
                        L(LexicalTerminal {
                            matcher: Lexical::EndOfInput,
                            owner: LexicalOwnerTemplate::None,
                        }),
                    ],
                },
            ];
        }
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

        let normalize = |item: syn::Item| {
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: Vec::new(),
                items: vec![item],
            })
        };
        assert_eq!(
            normalize(actual[4].clone()),
            normalize(expected_synthetic_projection_rules())
        );

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
        assert!(rules.contains("FeatureConstraint :: Exact (Number :: Singular)"));
        assert!(rules.contains("Lexical :: Literal (\"!\")"));
        assert!(rules.contains("Lexical :: EndOfInput"));
    }
}
