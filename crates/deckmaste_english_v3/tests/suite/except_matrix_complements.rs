use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;

use crate::common;

fn passive_witness(text: &str, complement: &str) {
    witness(text, complement, Category::PrepositionPhrase);
    for value in common::readings(text, Category::Document) {
        let mut attachments = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::MatrixComplementPredicate { head, modifier, .. } = node {
                    attachments.push((
                        head.realize(common::lexicon()).unwrap(),
                        modifier.realize(common::lexicon()).unwrap(),
                    ));
                }
            })
            .unwrap();
        assert_eq!(attachments, vec![("blocked".into(), complement.into())]);
        value.admit(common::lexicon()).unwrap();
    }
}

fn witness(text: &str, complement: &str, category: Category) {
    common::assert_constituents(
        text,
        Category::Document,
        &[
            (Category::PrepositionPhrase, complement),
            (category, complement.strip_prefix("except ").unwrap()),
        ],
    );
    for value in common::readings(text, Category::Document) {
        value.admit(common::lexicon()).unwrap();
        value
            .visit(&mut |node| {
                if let Reading::PostmodifiedNominal { modifier, .. } = node {
                    assert!(
                        !modifier
                            .realize(common::lexicon())
                            .unwrap()
                            .starts_with("except ")
                    );
                }
                if let Reading::LicensedMatrixComplementPredicate { head, modifier, .. } = node {
                    assert_eq!(modifier.realize(common::lexicon()).unwrap(), complement);
                    let host = head.realize(common::lexicon()).unwrap();
                    assert!(
                        host.starts_with("Create ") || host.starts_with("sacrifices "),
                        "wrong matrix host {host:?}"
                    );
                }
            })
            .unwrap();
    }
}

#[test]
fn evil_eye_of_orms_by_gore() {
    passive_witness(
        "This creature can't be blocked except by Walls.",
        "except by Walls",
    );
}
#[test]
fn shifting_sliver() {
    passive_witness(
        "Slivers can't be blocked except by Slivers.",
        "except by Slivers",
    );
}
#[test]
fn seeker() {
    passive_witness(
        "Enchanted creature can't be blocked except by artifact creatures and/or white creatures.",
        "except by artifact creatures and/or white creatures",
    );
}
#[test]
fn quantum_misalignment() {
    witness(
        "Create a token that's a copy of target creature you control, except it isn't legendary.",
        "except it isn't legendary",
        Category::FiniteClause,
    );
}
#[test]
fn keldon_firebombers() {
    witness(
        "When this creature enters, each player sacrifices all lands they control except for three.",
        "except for three",
        Category::PrepositionPhrase,
    );
}

#[test]
fn shifting_sliver_control_retains_its_complete_reading() {
    let values = common::readings("Slivers can't be blocked by Slivers.", Category::Document);
    assert_eq!(values.len(), 7);
    for value in values {
        value.admit(common::lexicon()).unwrap();
    }
}

#[test]
fn independently_constructed_passive_exception_roundtrips() {
    let head = common::readings("blocked", Category::SecondaryVerbPhrase)
        .into_iter()
        .find(|value| matches!(value, Reading::PassivePredicate { .. }))
        .unwrap();
    let complement = common::readings("by Walls", Category::PrepositionPhrase)
        .into_iter()
        .next()
        .unwrap();
    let governor = deckmaste_english_v3::grammar::Word {
        value: deckmaste_lexical::LexicalReading::Word(deckmaste_lexical::LexicalValue {
            lexeme: "vocab:Preposition/Except".into(),
            form: deckmaste_lexical::WordForm::Invariant,
            features: deckmaste_lexical::FeatureBundle::default(),
            variant: 0,
            capitalization: deckmaste_lexical::SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    };
    let expected = Reading::MatrixComplementPredicate {
        form: 0,
        head: Box::new(head),
        modifier: Box::new(Reading::MatrixComplementPreposition {
            form: 0,
            head: governor,
            complement: Box::new(complement),
        }),
    };
    expected.admit(common::lexicon()).unwrap();
    assert_eq!(
        expected.realize(common::lexicon()).unwrap(),
        "blocked except by Walls"
    );
    assert!(
        common::readings("blocked except by Walls", Category::SecondaryVerbPhrase)
            .contains(&expected)
    );
}

#[test]
fn copy_description_requires_its_declared_host_permission() {
    let mut entries: Vec<_> = common::lexicon().lexemes().values().cloned().collect();
    let host = entries
        .iter_mut()
        .find(|entry| entry.id.as_str() == "lexeme:keyword_action/create")
        .unwrap();
    host.properties
        .features
        .insert("MatrixComplementLicence".into(), "None".into());
    let unlicensed = deckmaste_lexical::Lexicon::new(entries).unwrap();
    assert!(common::readings_with_lexicon(&unlicensed,
        "Create a token that's a copy of target creature you control, except it isn't legendary.",
        Category::Document).is_empty());
}
