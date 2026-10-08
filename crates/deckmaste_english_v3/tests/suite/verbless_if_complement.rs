use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading, Word};
use std::collections::BTreeSet;

fn witness(text: &str) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "missing Reading of {text:?}");
    eprintln!("{text:?}: {} Readings", values.len());
    for value in values {
        value.admit(common::lexicon()).unwrap();
        let mut complements = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::VerblessClauseComplementPreposition { .. } = node {
                    complements.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(
            complements,
            vec![conditional()],
            "wrong Complement in {value:?}"
        );
    }
    common::assert_constituents(
        text,
        Category::Document,
        &[
            (Category::PrepositionPhrase, "if able"),
            (Category::VerblessClause, "able"),
        ],
    );
}

#[test]
fn ares_active_finite_predicate() {
    witness("Ares attacks each combat if able.");
}

#[test]
fn jurin_passive_modal_predicate() {
    witness("Jurin must be blocked if able.");
}

#[test]
fn tangle_angler_activated_ability() {
    witness("{G}: Target creature blocks this creature this turn if able.");
}

#[test]
fn alluring_siren_selected_object() {
    witness("{T}: Target creature an opponent controls attacks you this turn if able.");
}

#[test]
fn hustle_coordinated_predicates() {
    witness("Target creature attacks or blocks this turn if able.");
}

fn word(id: &str) -> Word {
    Word {
        value: deckmaste_lexical::LexicalReading::Word(deckmaste_lexical::LexicalValue {
            lexeme: id.into(),
            form: deckmaste_lexical::WordForm::Invariant,
            features: deckmaste_lexical::FeatureBundle::default(),
            variant: 0,
            capitalization: deckmaste_lexical::SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}

fn conditional() -> Reading {
    let mut predicate = word("vocab:Adjective/Able");
    predicate.frame = Some(0);
    Reading::VerblessClauseComplementPreposition {
        form: 0,
        head: word("vocab:Preposition/If"),
        complement: Box::new(Reading::VerblessClause { form: 0, predicate }),
    }
}

#[test]
fn independently_constructed_complement_keeps_identity_and_leaf_order() {
    let expected = conditional();
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(expected.realize(common::lexicon()).unwrap(), "if able");
    assert_eq!(
        common::readings("if able", Category::PrepositionPhrase),
        BTreeSet::from([expected.clone()])
    );
    let mut words = Vec::new();
    expected
        .visit_words(&mut |word| words.push(word.clone()))
        .unwrap();
    assert_eq!(
        words,
        vec![word("vocab:Preposition/If"), {
            let mut predicate = word("vocab:Adjective/Able");
            predicate.frame = Some(0);
            predicate
        }]
    );
}

#[test]
fn both_lexical_licences_and_the_adjunct_function_are_required() {
    for (id, feature, value) in [
        ("vocab:Preposition/If", "VerblessClauseComplement", "No"),
        ("vocab:Adjective/Able", "VerblessPredicateUse", "No"),
        (
            "vocab:Preposition/If",
            "PrepositionFunctionLicence",
            "Modifier",
        ),
    ] {
        let mut lexemes: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
        lexemes
            .iter_mut()
            .find(|lexeme| lexeme.id.as_str() == id)
            .unwrap()
            .properties
            .features
            .insert(feature.into(), value.into());
        let lexicon = deckmaste_lexical::Lexicon::new(lexemes).unwrap();
        assert!(
            conditional().admit(&lexicon).is_err(),
            "unchecked {feature}"
        );
        assert!(
            common::readings_with_lexicon(&lexicon, "if able", Category::PrepositionPhrase)
                .is_empty(),
            "unchecked {feature}"
        );
    }
}

#[test]
fn verbless_complement_does_not_supply_nominal_or_generic_adjective_use() {
    assert!(common::readings("Target creature if able", Category::NounPhrase).is_empty());
    assert!(common::readings("able", Category::AdjectivePhrase).is_empty());
    assert!(common::readings("if able", Category::LocativeComplement).is_empty());
    assert!(common::readings("when able", Category::PrepositionPhrase).is_empty());
    assert!(common::readings("if possible", Category::PrepositionPhrase).is_empty());
    let clause = common::readings("Ares attacks each combat", Category::Clause)
        .into_iter()
        .next()
        .unwrap();
    let preposed = Reading::InitialPreposition {
        form: 0,
        dependent: Box::new(conditional()),
        clause: Box::new(clause),
    };
    assert!(preposed.admit(common::lexicon()).is_err());
}
