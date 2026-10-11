use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{FeatureBundle, LexicalReading, LexicalValue, SurfaceCase, WordForm};

use crate::common;

fn word(owner: &str, capitalization: SurfaceCase) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization,
        }),
        frame: None,
        countability: None,
    }
}

fn named_keyword(name: &str) -> Reading {
    Reading::NameKeyword {
        form: 0,
        head: word("lexeme:keyword_ability/partnerWith", SurfaceCase::Initial),
        marker: word("vocab:Preposition/With", SurfaceCase::Declared),
        name: Box::new(Reading::CatalogName {
            form: 0,
            head: word(
                &format!("catalog:card-names.txt/{name}"),
                SurfaceCase::Declared,
            ),
        }),
    }
}

fn document(keyword: Reading) -> Reading {
    Reading::Document {
        form: 0,
        first: Box::new(Reading::KeywordLine {
            form: 0,
            first: Box::new(keyword),
            rest: vec![],
        }),
        rest: vec![],
    }
}

#[test]
fn printed_partner_lines_keep_the_name_parameter_and_both_roundtrip_laws() {
    for (card, name) in [
        ("Amy Pond", "Rory Williams"),
        ("Pir, Imaginative Rascal", "Toothy, Imaginary Friend"),
        ("Blaring Recruiter", "Blaring Captain"),
        ("Khorvath Brightflame", "Sylvia Brightspear"),
        ("Rowan Kenrith", "Will Kenrith"),
    ] {
        let text = format!("Partner with {name}");
        let expected = document(named_keyword(name));
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings_on_card(card, &text, Category::Document),
            BTreeSet::from([expected]),
            "{card}",
        );
    }
}

#[test]
fn name_payload_uses_the_catalog_class_including_names_outside_partner_pairs() {
    // PW5 licenses the catalog class, rather than the attested partner-name pairs.
    for name in ["Black Lotus", "Aang and Katara", "Chandra, Pyromaster"] {
        let expected = named_keyword(name);
        expected.admit(common::lexicon()).unwrap();
        let text = expected.realize(common::lexicon()).unwrap();
        assert_eq!(
            common::readings(&text, Category::KeywordPhrase),
            BTreeSet::from([expected]),
        );
    }
}

#[test]
fn name_keyword_rejects_quality_payloads_bare_heads_and_unagreed_markers() {
    let mut wrong_head = named_keyword("Rory Williams");
    let Reading::NameKeyword { head, .. } = &mut wrong_head else { unreachable!() };
    *head = word("lexeme:keyword_ability/partner", SurfaceCase::Initial);
    assert!(wrong_head.admit(common::lexicon()).is_err());

    let mut wrong_marker = named_keyword("Rory Williams");
    let Reading::NameKeyword { marker, .. } = &mut wrong_marker else {
        unreachable!()
    };
    *marker = word("vocab:Preposition/From", SurfaceCase::Declared);
    assert!(wrong_marker.admit(common::lexicon()).is_err());

    let quality = Reading::QualityKeyword {
        form: 0,
        head: word("lexeme:keyword_ability/partnerWith", SurfaceCase::Initial),
        quality: Box::new(Reading::KeywordQualityPreposition {
            form: 0,
            head: word("vocab:Preposition/With", SurfaceCase::Declared),
            quality: Box::new(Reading::NounPhraseKeywordQuality {
                form: 0,
                phrase: Box::new(Reading::ProperNameNounPhrase {
                    form: 0,
                    head: Box::new(Reading::CatalogName {
                        form: 0,
                        head: word(
                            "catalog:card-names.txt/Rory Williams",
                            SurfaceCase::Declared,
                        ),
                    }),
                }),
            }),
        }),
    };
    assert!(quality.admit(common::lexicon()).is_err());
    for text in [
        "Partner with artifacts",
        "Partner from Rory Williams",
        "Partner with",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn bare_partner_and_the_dashed_variant_keep_their_existing_lexical_owners() {
    for (text, owner) in [
        ("Partner", "lexeme:keyword_ability/partner"),
        (
            "Partner—Survivors",
            "lexeme:keyword_ability/partnerSurvivors",
        ),
    ] {
        let expected = document(Reading::BareKeyword {
            form: 0,
            head: word(owner, SurfaceCase::Initial),
        });
        assert_eq!(
            common::readings(text, Category::Document),
            BTreeSet::from([expected])
        );
    }
}
