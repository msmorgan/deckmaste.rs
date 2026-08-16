use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::ValidatedDeclarations;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
use crate::identifier::key as identifier_key;
use crate::identifier::path_key;
use crate::model::Declaration;
use crate::model::FieldKind;
use crate::model::FormAtom;
use crate::model::VerbOperand;
use crate::plan::DeclarationKey;
use crate::plan::DeclarationKind;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;

pub(crate) fn emit(validated: &ValidatedDeclarations) -> syn::Result<Vec<GeneratedItem>> {
    let constructions = constructions(validated);
    let origins = construction_origins(&constructions);
    let categories = category_names(&constructions);
    let rule_ids = validated
        .contributions()
        .constructions()
        .iter()
        .map(|record| ident(record.rule_id()))
        .collect::<Vec<_>>();
    if constructions.len() != rule_ids.len() {
        return Err(internal("validated rule inventory is inconsistent"));
    }
    let count = syn::LitInt::new(&constructions.len().to_string(), Span::call_site());
    let construction_matches = rule_ids.iter().map(|rule_id| {
        quote! { Self::#rule_id => Construction::#rule_id }
    });
    let rows = constructions
        .iter()
        .zip(&rule_ids)
        .map(|(construction, rule_id)| emit_rule(validated, construction, rule_id))
        .collect::<syn::Result<Vec<_>>>()?;

    let mut rule_origins = origins.clone();
    rule_origins.extend(
        validated
            .raw()
            .declarations
            .iter()
            .filter_map(|declaration| {
                let Declaration::Root(root) = declaration else { return None };
                root.eoi
                    .then(|| DeclarationKey::new(DeclarationKind::Root, path_name(&root.category)))
            }),
    );
    Ok(vec![
        GeneratedItem::new(
            ItemKey::named_type("Category"),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Category { #(#categories),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::named_type("Construction"),
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Construction { #(#rule_ids),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::named_type("RuleId"),
            quote! {
                #[repr(usize)]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum RuleId { #(#rule_ids),* }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Impl {
                trait_name: None,
                self_ty: "RuleId".to_owned(),
            },
            quote! {
                impl RuleId {
                    #[cfg(test)]
                    pub(crate) const COUNT: usize = #count;
                    pub(crate) const fn construction(self) -> Construction {
                        match self { #(#construction_matches,)* }
                    }
                    pub(crate) const fn index(self) -> usize { self as usize }
                }
            },
            origins.clone(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Constant,
                name: "RULES".to_owned(),
            },
            quote! { pub(crate) const RULES: &[Rule<Category, Lexical, RuleId>] = &[#(#rows),*]; },
            rule_origins,
        ),
    ])
}

fn emit_rule(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    rule_id: &syn::Ident,
) -> syn::Result<TokenStream> {
    let lhs = &construction.category;
    let mut rhs = construction
        .form
        .atoms
        .iter()
        .map(|atom| emit_position(validated, construction, atom))
        .collect::<syn::Result<Vec<_>>>()?;
    if let Some(root) = parse_root(validated, lhs) {
        let punctuation = &root.punctuation;
        rhs.push(quote! { L(Lexical::Literal(#punctuation)) });
        rhs.push(quote! { L(Lexical::EndOfInput) });
    }
    Ok(quote! { Rule { id: RuleId::#rule_id, lhs: Category::#lhs, rhs: &[#(#rhs),*] } })
}

fn emit_position(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
    atom: &FormAtom,
) -> syn::Result<TokenStream> {
    match atom {
        FormAtom::Literal(literal) => Ok(quote! { L(Lexical::Literal(#literal)) }),
        FormAtom::Role(role) => {
            let FieldKind::Category(category) = &field(construction, role)?.kind else {
                return Err(internal("validated category role has the wrong field kind"));
            };
            Ok(quote! { N(Category::#category) })
        }
        FormAtom::Lex(role) | FormAtom::Identity(role) => {
            let terminal = terminal_path(field(construction, role)?)?;
            let lexical = lexical_variant(validated, terminal)?;
            Ok(quote! { L(#lexical) })
        }
        FormAtom::Noun(role) => {
            let terminal = terminal_path(field(construction, role)?)?;
            let lexical = lexical_variant(validated, terminal)?;
            let number = noun_number(validated, construction)?;
            Ok(quote! { L(#lexical(NounNumber::#number)) })
        }
        FormAtom::Verb(VerbOperand::Fixed(path)) => Ok(quote! { L(Lexical::Verb(#path)) }),
        FormAtom::Verb(VerbOperand::Projected(_)) => Err(internal(
            "projected verb rules require a fixed lexical variant",
        )),
    }
}

fn lexical_variant(
    validated: &ValidatedDeclarations,
    terminal: &syn::Path,
) -> syn::Result<TokenStream> {
    for declaration in &validated.raw().declarations {
        match declaration {
            Declaration::Vocab(vocab) if identifier_key(&vocab.name) == path_name(terminal) => {
                let name = &vocab.name;
                return Ok(quote! { Lexical::#name });
            }
            Declaration::Codec(binding) | Declaration::Identity(binding)
                if identifier_key(&binding.name) == path_name(terminal) =>
            {
                let path = binding.lexical_variant.as_ref().ok_or_else(|| {
                    internal("atom-capable terminal binding has no lexical variant")
                })?;
                return Ok(quote! { #path });
            }
            _ => {}
        }
    }
    Err(internal("resolved terminal has no lexical projection"))
}

fn noun_number(
    validated: &ValidatedDeclarations,
    construction: &crate::Construction,
) -> syn::Result<syn::Ident> {
    let equation = validated
        .feature_equations(&identifier_key(&construction.name))
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

fn constructions(validated: &ValidatedDeclarations) -> Vec<&crate::Construction> {
    validated
        .raw()
        .declarations
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::Construction(construction) => Some(construction),
            _ => None,
        })
        .collect()
}

fn construction_origins(constructions: &[&crate::Construction]) -> Vec<DeclarationKey> {
    constructions
        .iter()
        .map(|construction| {
            DeclarationKey::new(
                DeclarationKind::Construction,
                identifier_key(&construction.name),
            )
        })
        .collect()
}

fn category_names(constructions: &[&crate::Construction]) -> Vec<syn::Path> {
    let mut names = Vec::<syn::Path>::new();
    for construction in constructions {
        if !names
            .iter()
            .any(|name| path_name(name) == path_name(&construction.category))
        {
            names.push(construction.category.clone());
        }
    }
    names
}

fn parse_root<'a>(
    validated: &'a ValidatedDeclarations,
    category: &syn::Path,
) -> Option<&'a crate::Root> {
    validated
        .raw()
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Declaration::Root(root)
                if root.eoi && path_name(&root.category) == path_name(category) =>
            {
                Some(root)
            }
            _ => None,
        })
}

fn field<'a>(
    construction: &'a crate::Construction,
    role: &syn::Ident,
) -> syn::Result<&'a crate::Field> {
    construction
        .element
        .fields
        .iter()
        .find(|field| identifier_key(&field.name) == identifier_key(role))
        .ok_or_else(|| internal("resolved form role is absent"))
}

fn terminal_path(field: &crate::Field) -> syn::Result<&syn::Path> {
    match &field.kind {
        FieldKind::Lex(path) | FieldKind::Identity(path) => Ok(path),
        FieldKind::Category(_) => Err(internal("terminal atom resolves to category field")),
    }
}

fn path_name(path: &syn::Path) -> String {
    path_key(path)
}

fn ident(name: &str) -> syn::Ident {
    format_ident!("{name}")
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
        let generated = super::emit(&validated).expect("role-derived noun rules lower");
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
        let generated = super::emit(&validated).expect("two dynamic nouns lower to rules");
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
        let generated = super::emit(&validated).unwrap();
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
        let items = super::emit(&validated).unwrap();
        let rules = items.last().unwrap().tokens.to_string();
        assert!(rules.contains("NounNumber :: Singular"));
        assert!(rules.contains("Lexical :: Literal (\"!\")"));
        assert!(rules.contains("Lexical :: EndOfInput"));
    }
}
