use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;

use crate::common::LEXICON;
use crate::common::assert_constituents;
use crate::common::readings;

fn witness(text: &str) {
    let surface = if text.starts_with("Nontoken") { "Nontoken" } else { "nontoken" };
    assert_constituents(
        text,
        Category::Sentence,
        &[(Category::NounPremodifier, surface)],
    );
    for reading in readings(text, Category::Sentence) {
        let mut owners = Vec::new();
        reading
            .visit_words(&mut |word| {
                if let deckmaste_lexical::LexicalReading::Word(value) = &word.value {
                    owners.push(value.lexeme);
                }
            })
            .unwrap();
        assert!(owners.contains(&"lexeme:CommonNoun/Token/non".into()));
        assert!(!owners.contains(&"vocab:NegativePrefix/Non".into()));
    }
}

#[test]
fn hour_of_reckoning() {
    witness("Destroy all nontoken creatures.");
}
#[test]
fn swift_spiral() {
    witness("Exile target nontoken creature.");
}
#[test]
fn rhythm_of_the_wild() {
    witness("Nontoken creatures you control have riot.");
}
#[test]
fn isilu_carrier_of_twilight() {
    witness("Each other nontoken creature you control has persist.");
}
#[test]
fn thraben_watcher() {
    witness("Other nontoken creatures you control get +1/+1 and have vigilance.");
}

#[test]
fn independently_constructed_negative_noun_keeps_its_identity_and_plural_head() {
    use deckmaste_english_v3::grammar::Word;
    use deckmaste_lexical::FeatureBundle;
    use deckmaste_lexical::LexicalReading;
    use deckmaste_lexical::LexicalValue;
    use deckmaste_lexical::Number;
    use deckmaste_lexical::SurfaceCase;
    use deckmaste_lexical::WordForm;
    let word = |owner: &str, number| Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: if number == Number::Singular { WordForm::Singular } else { WordForm::Plural },
            features: FeatureBundle {
                number: Some(number),
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: Some(true),
    };
    let expected = Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::BarrierNounPremodifiedNominal {
                form: 0,
                modifier: Box::new(Reading::NounPremodifier {
                    form: 0,
                    head: word("lexeme:CommonNoun/Token/non", Number::Singular),
                }),
                head: Box::new(Reading::NominalProjection {
                    form: 0,
                    head: Box::new(Reading::NominalCoreProjection {
                        form: 0,
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word("lexeme:type/creature", Number::Plural),
                        }),
                    }),
                }),
            }),
        }),
    };
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), "nontoken creatures");
    assert_eq!(
        readings("nontoken creatures", Category::Nominal),
        std::collections::BTreeSet::from([expected])
    );
    assert!(readings("token", Category::NounPremodifier).is_empty());
    assert!(readings("nontokens", Category::NounPremodifier).is_empty());
    assert!(readings("non token creatures", Category::Nominal).is_empty());
}
