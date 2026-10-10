use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common::{LEXICON, readings};

fn noun(owner: &str, countability: bool) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Singular,
            features: FeatureBundle {
                number: Some(Number::Singular),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: Some(countability),
    }
}

fn negative_premodifier() -> Reading {
    Reading::NounPremodifier {
        form: 0,
        head: noun("lexeme:turn_part/combat/non", true),
    }
}

fn witness(text: &str) {
    let values = readings(text, Category::Sentence);
    assert!(!values.is_empty(), "no Reading for {text}");
    // W9-A: independent ambiguity in the positive counterpart is retained.
    assert_eq!(
        values.len(),
        readings(&text.replace("noncombat", "combat"), Category::Sentence).len()
    );
    for value in values {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::NounPremodifiedNominal { modifier, head, .. } = node
                    && **modifier == negative_premodifier()
                {
                    let mut owners = Vec::new();
                    head.visit_words(&mut |word| {
                        if let LexicalReading::Word(word) = &word.value {
                            owners.push(word.lexeme);
                        }
                    })
                    .unwrap();
                    assert!(owners.contains(&"lexeme:CommonNoun/Damage".into()));
                    modifiers.push((**modifier).clone());
                }
            })
            .unwrap();
        assert_eq!(modifiers, vec![negative_premodifier()]);
    }
}

#[test]
fn chandras_spitfire() {
    witness(
        "Whenever an opponent is dealt noncombat damage, this creature gets +3/+0 until end of turn.",
    );
}

#[test]
fn niv_mizzet_visionary() {
    witness(
        "Whenever a source you control deals noncombat damage to an opponent, you draw that many cards.",
    );
}

#[test]
fn mark_of_asylum() {
    witness("Prevent all noncombat damage that would be dealt to creatures you control.");
}

#[test]
fn wildfire_elemental() {
    witness(
        "Whenever an opponent is dealt noncombat damage, creatures you control get +1/+0 until end of turn.",
    );
}

#[test]
fn chandras_pyreling() {
    witness(
        "Whenever a source you control deals noncombat damage to an opponent, this creature gets +1/+0 and gains double strike until end of turn.",
    );
}

#[test]
fn negative_noun_retains_plugin_provenance_and_declared_properties() {
    let base = &LEXICON.lexemes()["lexeme:turn_part/combat"];
    let negative = &LEXICON.lexemes()["lexeme:turn_part/combat/non"];
    assert_eq!(base.category, deckmaste_lexical::Category::Noun);
    assert_eq!(negative.category, base.category);
    assert_eq!(base.source.kind, deckmaste_lexical::SourceKind::Plugin);
    assert_eq!(negative.source, base.source);
    assert_eq!(negative.properties, base.properties);
    assert_eq!(negative.capitalization, base.capitalization);
    assert_eq!(negative.binding, base.binding);
    let mut forms = base.forms.clone();
    for slot in &mut forms {
        if let Some(surfaces) = &mut slot.surfaces {
            for surface in surfaces {
                *surface = format!("non{surface}");
            }
        }
    }
    assert_eq!(negative.forms, forms);
    assert_eq!(negative.lemma, format!("non{}", base.lemma));
}

#[test]
fn independently_constructed_negative_premodifier_roundtrips_with_mass_head() {
    // Constituent attested on every ticket witness, including Mark of Asylum.
    let expected = Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(negative_premodifier()),
        head: Box::new(Reading::Noun {
            form: 0,
            head: noun("lexeme:CommonNoun/Damage", false),
        }),
    };
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), "noncombat damage");
    assert_eq!(
        readings("noncombat damage", Category::Nominal),
        BTreeSet::from([expected])
    );
    assert!(readings("non-combat damage", Category::Nominal).is_empty());
    assert!(readings("non combat damage", Category::Nominal).is_empty());
    assert!(readings("noncombat", Category::AdjectivePhrase).is_empty());
}
