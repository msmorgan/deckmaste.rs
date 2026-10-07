mod common;

use common::assert_constituents;
use common::lexicon;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

#[test]
fn tamanoa_degree_quantifier() {
    assert_constituents(
        "Whenever a noncreature source you control deals damage, you gain that much life.",
        Category::Document,
        &[
            (Category::QuantitativeDeterminer, "that much"),
            (Category::NounPhrase, "that much life"),
        ],
    );
}

#[test]
fn mourning_thrull_degree_quantifier() {
    assert_constituents(
        "Whenever this creature deals damage, you gain that much life.",
        Category::Document,
        &[
            (Category::QuantitativeDeterminer, "that much"),
            (Category::NounPhrase, "that much life"),
        ],
    );
}

#[test]
fn crosstown_courier_degree_quantifier() {
    assert_constituents(
        "Whenever this creature deals combat damage to a player, that player mills that many cards.",
        Category::Document,
        &[
            (Category::QuantitativeDeterminer, "that many"),
            (Category::NounPhrase, "that many cards"),
        ],
    );
}

#[test]
fn guilty_conscience_degree_quantifier() {
    assert_constituents(
        "Whenever enchanted creature deals damage, this Aura deals that much damage to that creature.",
        Category::Document,
        &[
            (Category::QuantitativeDeterminer, "that much"),
            (Category::NounPhrase, "that much damage"),
        ],
    );
}

#[test]
fn firedrinker_satyr_degree_quantifier() {
    assert_constituents(
        "Whenever this creature is dealt damage, it deals that much damage to you.",
        Category::Document,
        &[
            (Category::QuantitativeDeterminer, "that much"),
            (Category::NounPhrase, "that much damage"),
        ],
    );
}

#[test]
fn degree_quantifiers_keep_head_countability_and_one_constituency() {
    // Attested constituents of Crosstown Courier, Tamanoa and Guilty
    // Conscience.
    for text in ["that many cards", "that much life", "that much damage"] {
        assert_eq!(readings(text, Category::NounPhrase).len(), 1, "{text}");
    }
    for text in ["that many", "that much"] {
        assert_eq!(
            readings(text, Category::QuantitativeDeterminer).len(),
            1,
            "{text}"
        );
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
    for text in [
        "that many card",
        "that many life",
        "that much cards",
        "that much creature",
        "that all cards",
        "those much life",
    ] {
        assert!(readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}

fn word(owner: &str) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn degree(head: &str, modifier: &str) -> Reading {
    Reading::DegreeDeterminativePhrase {
        form: 0,
        modifier: word(modifier),
        head: word(head),
    }
}

fn quantified(quantity: Reading, owner: &str, number: Number, count: bool) -> Reading {
    let mut head = word(owner);
    let LexicalReading::Word(noun) = &mut head.value else { unreachable!() };
    noun.form = if number == Number::Singular { WordForm::Singular } else { WordForm::Plural };
    noun.features.number = Some(number);
    head.countability = Some(count);
    Reading::QuantifiedNounPhrase {
        form: 0,
        quantity: Box::new(quantity),
        head: Box::new(Reading::Noun { form: 0, head }),
    }
}

#[test]
fn independent_degree_phrases_preserve_both_roundtrip_laws_and_traversal_identity() {
    // The NP constituents in the five named Oracle witnesses above.
    for (head, noun, number, count, text) in [
        (
            "vocab:Determinative/Many",
            "lexeme:CommonNoun/Card",
            Number::Plural,
            true,
            "that many cards",
        ),
        (
            "vocab:Determinative/Much",
            "lexeme:CommonNoun/Life",
            Number::Singular,
            false,
            "that much life",
        ),
        (
            "vocab:Determinative/Much",
            "lexeme:CommonNoun/Damage",
            Number::Singular,
            false,
            "that much damage",
        ),
    ] {
        let quantity = degree(head, "vocab:SingularDemonstrative/That");
        let value = quantified(quantity.clone(), noun, number, count);
        for (value, surface, category) in [
            (
                &quantity,
                text.rsplit_once(' ').unwrap().0,
                Category::QuantitativeDeterminer,
            ),
            (&value, text, Category::NounPhrase),
        ] {
            value.admit(lexicon()).unwrap();
            assert_eq!(value.realize(lexicon()).unwrap(), surface);
            let parsed = readings(surface, category);
            assert_eq!(parsed, std::collections::BTreeSet::from([value.clone()]));
            let observed = parsed.get(value).unwrap();
            let mut expected_nodes = Vec::new();
            let mut actual_nodes = Vec::new();
            value
                .visit(&mut |node| expected_nodes.push(node.clone()))
                .unwrap();
            observed
                .visit(&mut |node| actual_nodes.push(node.clone()))
                .unwrap();
            assert_eq!(actual_nodes, expected_nodes);
            let mut expected_words = Vec::new();
            let mut actual_words = Vec::new();
            value
                .visit_words(&mut |word| expected_words.push(word.clone()))
                .unwrap();
            observed
                .visit_words(&mut |word| actual_words.push(word.clone()))
                .unwrap();
            assert_eq!(actual_words, expected_words);
            let Reading::DegreeDeterminativePhrase { modifier, head, .. } = &quantity else {
                unreachable!()
            };
            assert_eq!(&expected_words[..2], &[modifier.clone(), head.clone()]);
        }
    }
}

#[test]
fn independent_degree_values_reject_unlicensed_roles_and_wrong_noun_heads() {
    let many = degree(
        "vocab:Determinative/Many",
        "vocab:SingularDemonstrative/That",
    );
    let much = degree(
        "vocab:Determinative/Much",
        "vocab:SingularDemonstrative/That",
    );
    for invalid in [
        degree(
            "vocab:FloatedQuantifier/All",
            "vocab:SingularDemonstrative/That",
        ),
        degree("vocab:Determinative/Many", "vocab:Determinative/Those"),
        quantified(
            many.clone(),
            "lexeme:CommonNoun/Card",
            Number::Singular,
            true,
        ),
        quantified(many, "lexeme:CommonNoun/Life", Number::Singular, false),
        quantified(much, "lexeme:CommonNoun/Card", Number::Plural, true),
    ] {
        assert!(invalid.admit(lexicon()).is_err(), "{invalid:?}");
        assert!(invalid.realize(lexicon()).is_err(), "{invalid:?}");
    }
}
