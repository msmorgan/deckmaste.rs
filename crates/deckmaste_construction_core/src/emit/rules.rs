use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::ValidatedDeclarations;
use crate::feature::Feature;
use crate::feature::FeatureExpr;
use crate::feature::FeaturePlace;
use crate::feature::FeatureValue;
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
            Declaration::Vocab(vocab) if vocab.name == path_name(terminal) => {
                let name = &vocab.name;
                return Ok(quote! { Lexical::#name });
            }
            Declaration::Codec(binding) | Declaration::Identity(binding)
                if binding.name == path_name(terminal) =>
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
        .feature_equations(&construction.name.to_string())
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
        FeatureExpr::MatchVocab { .. } => "Either",
        FeatureExpr::FromRole { .. } => {
            return Err(internal(
                "role-derived noun number cannot project one parser request",
            ));
        }
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
            DeclarationKey::new(DeclarationKind::Construction, construction.name.to_string())
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
        .find(|field| field.name == *role)
        .ok_or_else(|| internal("resolved form role is absent"))
}

fn terminal_path(field: &crate::Field) -> syn::Result<&syn::Path> {
    match &field.kind {
        FieldKind::Lex(path) | FieldKind::Identity(path) => Ok(path),
        FieldKind::Category(_) => Err(internal("terminal atom resolves to category field")),
    }
}

fn path_name(path: &syn::Path) -> String {
    path.segments
        .last()
        .map_or_else(String::new, |segment| segment.ident.to_string())
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
    fn full_golden_rules_are_the_five_exact_structural_items() {
        let validated = crate::validate_declarations(
            crate::parse_declarations(crate::validate::tests::full_golden_tokens()).unwrap(),
        )
        .unwrap();
        let generated = super::emit(&validated).unwrap();
        let actual = generated
            .iter()
            .map(|item| syn::parse2::<syn::Item>(item.tokens.clone()).unwrap())
            .collect::<Vec<_>>();
        let expected = expected_items();
        let construction_origins = expected_construction_origins();
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
                .iter()
                .copied()
                .chain([(crate::DeclarationKind::Root, "Ability")])
                .collect::<Vec<_>>(),
        );
        for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
            let actual = prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![actual.clone()],
            });
            let expected = prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![expected.clone()],
            });
            assert_eq!(
                &actual, &expected,
                "rule item {index}\nactual:\n{actual}\nexpected:\n{expected}",
            );
        }
        assert_eq!(actual.len(), 5);
        assert_eq!(
            actual
                .iter()
                .map(|item| syn::parse2::<syn::File>(quote!(#item)).unwrap().items.len())
                .collect::<Vec<_>>(),
            [1, 1, 1, 1, 1]
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

    fn expected_items() -> Vec<syn::Item> {
        let construction_variants = quote! {
            AbilitySpell, AbilityTriggered, SentenceImperative, SentenceDeclarative,
            SentenceWithWhere, ClauseEvent, ClauseWhere, NounPhrasePronoun,
            NounPhraseCommon, NounPhraseDemonstrative, NounPhraseTarget,
            NounPhraseSelfReference, NounPhraseCount, VerbPhraseDestroy,
            VerbPhraseConnive, VerbPhraseDealDamage, VerbPhraseGainLife,
            AmountNumber, AmountVariable
        };
        let items = vec![
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Category { Ability, Sentence, Clause, NounPhrase, VerbPhrase, Amount }
            },
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum Construction { #construction_variants }
            },
            quote! {
                #[repr(usize)]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
                pub(crate) enum RuleId { #construction_variants }
            },
            quote! {
                impl RuleId {
                    #[cfg(test)]
                    pub(crate) const COUNT: usize = 19;
                    pub(crate) const fn construction(self) -> Construction {
                        match self {
                            Self::AbilitySpell => Construction::AbilitySpell,
                            Self::AbilityTriggered => Construction::AbilityTriggered,
                            Self::SentenceImperative => Construction::SentenceImperative,
                            Self::SentenceDeclarative => Construction::SentenceDeclarative,
                            Self::SentenceWithWhere => Construction::SentenceWithWhere,
                            Self::ClauseEvent => Construction::ClauseEvent,
                            Self::ClauseWhere => Construction::ClauseWhere,
                            Self::NounPhrasePronoun => Construction::NounPhrasePronoun,
                            Self::NounPhraseCommon => Construction::NounPhraseCommon,
                            Self::NounPhraseDemonstrative => Construction::NounPhraseDemonstrative,
                            Self::NounPhraseTarget => Construction::NounPhraseTarget,
                            Self::NounPhraseSelfReference => Construction::NounPhraseSelfReference,
                            Self::NounPhraseCount => Construction::NounPhraseCount,
                            Self::VerbPhraseDestroy => Construction::VerbPhraseDestroy,
                            Self::VerbPhraseConnive => Construction::VerbPhraseConnive,
                            Self::VerbPhraseDealDamage => Construction::VerbPhraseDealDamage,
                            Self::VerbPhraseGainLife => Construction::VerbPhraseGainLife,
                            Self::AmountNumber => Construction::AmountNumber,
                            Self::AmountVariable => Construction::AmountVariable,
                        }
                    }
                    pub(crate) const fn index(self) -> usize { self as usize }
                }
            },
            expected_rules(),
        ];
        items
            .into_iter()
            .map(|tokens| syn::parse2(tokens).unwrap())
            .collect()
    }

    fn expected_rules() -> proc_macro2::TokenStream {
        quote! {
            pub(crate) const RULES: &[Rule<Category, Lexical, RuleId>] = &[
                Rule { id: RuleId::AbilitySpell, lhs: Category::Ability, rhs: &[N(Category::Sentence), L(Lexical::Literal(".")), L(Lexical::EndOfInput)] },
                Rule { id: RuleId::AbilityTriggered, lhs: Category::Ability, rhs: &[L(Lexical::TriggerWord), N(Category::Clause), L(Lexical::Literal(",")), N(Category::Sentence), L(Lexical::Literal(".")), L(Lexical::EndOfInput)] },
                Rule { id: RuleId::SentenceImperative, lhs: Category::Sentence, rhs: &[N(Category::VerbPhrase)] },
                Rule { id: RuleId::SentenceDeclarative, lhs: Category::Sentence, rhs: &[N(Category::NounPhrase), N(Category::VerbPhrase)] },
                Rule { id: RuleId::SentenceWithWhere, lhs: Category::Sentence, rhs: &[N(Category::Sentence), L(Lexical::Literal(",")), N(Category::Clause)] },
                Rule { id: RuleId::ClauseEvent, lhs: Category::Clause, rhs: &[N(Category::NounPhrase), N(Category::VerbPhrase)] },
                Rule { id: RuleId::ClauseWhere, lhs: Category::Clause, rhs: &[L(Lexical::Literal("where")), L(Lexical::Variable), L(Lexical::Verb(VerbLexeme::Be)), L(Lexical::Literal("the")), L(Lexical::Literal("number")), L(Lexical::Literal("of")), N(Category::NounPhrase)] },
                Rule { id: RuleId::NounPhrasePronoun, lhs: Category::NounPhrase, rhs: &[L(Lexical::Pronoun)] },
                Rule { id: RuleId::NounPhraseCommon, lhs: Category::NounPhrase, rhs: &[L(Lexical::Article), L(Lexical::Noun(NounNumber::Singular))] },
                Rule { id: RuleId::NounPhraseDemonstrative, lhs: Category::NounPhrase, rhs: &[L(Lexical::Demonstrative), L(Lexical::Noun(NounNumber::Either))] },
                Rule { id: RuleId::NounPhraseTarget, lhs: Category::NounPhrase, rhs: &[L(Lexical::Literal("target")), L(Lexical::Noun(NounNumber::Singular))] },
                Rule { id: RuleId::NounPhraseSelfReference, lhs: Category::NounPhrase, rhs: &[L(Lexical::SelfReference)] },
                Rule { id: RuleId::NounPhraseCount, lhs: Category::NounPhrase, rhs: &[L(Lexical::Noun(NounNumber::Plural)), L(Lexical::Pronoun), L(Lexical::Verb(VerbLexeme::Control)), L(Lexical::Literal("with")), L(Lexical::Literal("power")), L(Lexical::SignedNumber), L(Lexical::Literal("or")), L(Lexical::Literal("less"))] },
                Rule { id: RuleId::VerbPhraseDestroy, lhs: Category::VerbPhrase, rhs: &[L(Lexical::Verb(VerbLexeme::Destroy)), N(Category::NounPhrase)] },
                Rule { id: RuleId::VerbPhraseConnive, lhs: Category::VerbPhrase, rhs: &[L(Lexical::Verb(VerbLexeme::Connive))] },
                Rule { id: RuleId::VerbPhraseDealDamage, lhs: Category::VerbPhrase, rhs: &[L(Lexical::Verb(VerbLexeme::Deal)), N(Category::Amount), L(Lexical::Literal("damage")), L(Lexical::Literal("to")), N(Category::NounPhrase)] },
                Rule { id: RuleId::VerbPhraseGainLife, lhs: Category::VerbPhrase, rhs: &[L(Lexical::Verb(VerbLexeme::Gain)), N(Category::Amount), L(Lexical::Literal("life"))] },
                Rule { id: RuleId::AmountNumber, lhs: Category::Amount, rhs: &[L(Lexical::SignedNumber)] },
                Rule { id: RuleId::AmountVariable, lhs: Category::Amount, rhs: &[L(Lexical::Variable)] },
            ];
        }
    }

    fn expected_construction_origins() -> Vec<(crate::DeclarationKind, &'static str)> {
        [
            "spell",
            "triggered",
            "imperative",
            "declarative",
            "with_where",
            "event",
            "where",
            "pronoun",
            "common",
            "demonstrative",
            "target",
            "self_reference",
            "count",
            "destroy",
            "connive",
            "deal_damage",
            "gain_life",
            "number",
            "variable",
        ]
        .into_iter()
        .map(|name| (crate::DeclarationKind::Construction, name))
        .collect()
    }
}
