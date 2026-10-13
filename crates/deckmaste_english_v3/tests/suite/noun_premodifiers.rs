use std::collections::BTreeSet;

use common::LEXICON;
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

use crate::common;

fn word(owner: &str, number: Number) -> Word {
    Word {
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
    }
}

fn noun(owner: &str, number: Number) -> Reading {
    Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Noun {
                form: 0,
                head: word(owner, number),
            }),
        }),
    }
}

fn modifier(owner: &str) -> Reading {
    Reading::NounPremodifier {
        form: 0,
        head: word(owner, Number::Singular),
    }
}

fn modified(owner: &str, head: Reading) -> Reading {
    Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(modifier(owner)),
        head: Box::new(head),
    }
}

fn barrier_modified(owner: &str, head: Reading) -> Reading {
    crate::common::nominal(Reading::BarrierNounPremodifiedNominal {
        form: 0,
        modifier: Box::new(modifier(owner)),
        head: Box::new(head),
    })
}

fn exact(text: &str, category: Category, expected: Reading) {
    expected.admit(&LEXICON).unwrap();
    assert_eq!(expected.realize(&LEXICON).unwrap(), text);
    assert_eq!(readings(text, category), BTreeSet::from([expected]));
}

#[test]
fn independently_declared_noun_modifier_bases_keep_noun_identity() {
    // Skyfisher Spider: "... each creature card in your graveyard."
    // Kalitas, Bloodchief of Ghet: "... create a black Vampire creature token."
    // Mishra's Factory: "Target Assembly-Worker creature gets +1/+1 ..."
    // Time Lord Regeneration: "... reveal a Time Lord creature card."
    for (owner, text) in [
        ("lexeme:type/creature", "creature"),
        ("lexeme:creature_subtype/vampire", "Vampire"),
        ("lexeme:creature_subtype/assemblyWorker", "Assembly-Worker"),
        ("lexeme:creature_subtype/timeLord", "Time Lord"),
    ] {
        assert_eq!(
            LEXICON.lexemes()[owner].category,
            deckmaste_lexical::Category::Noun
        );
        exact(text, Category::NounPremodifier, modifier(owner));
    }
}

#[test]
fn authentic_noun_modification_and_stacking_have_complete_independent_values() {
    // Skyfisher Spider: "... each creature card in your graveyard."
    exact(
        "creature card",
        Category::Nominal,
        modified(
            "lexeme:type/creature",
            noun("lexeme:CommonNoun/Card", Number::Singular),
        ),
    );
    // Kalitas, Bloodchief of Ghet: "... create a black Vampire creature token."
    exact(
        "Vampire creature",
        Category::Nominal,
        barrier_modified(
            "lexeme:creature_subtype/vampire",
            noun("lexeme:type/creature", Number::Singular),
        ),
    );
    // Mishra's Factory: "Target Assembly-Worker creature gets +1/+1 until end
    // of turn."
    exact(
        "Assembly-Worker creature",
        Category::Nominal,
        barrier_modified(
            "lexeme:creature_subtype/assemblyWorker",
            noun("lexeme:type/creature", Number::Singular),
        ),
    );
    // Time Lord Regeneration: "... reveal a Time Lord creature card."
    let expected = barrier_modified(
        "lexeme:creature_subtype/timeLord",
        modified(
            "lexeme:type/creature",
            noun("lexeme:CommonNoun/Card", Number::Singular),
        ),
    );
    exact(
        "Time Lord creature card",
        Category::Nominal,
        expected.clone(),
    );
    let mut leaves = vec![];
    expected
        .visit_words(&mut |word| leaves.push(word.clone()))
        .unwrap();
    assert_eq!(
        leaves,
        vec![
            word("lexeme:creature_subtype/timeLord", Number::Singular),
            word("lexeme:type/creature", Number::Singular),
            word("lexeme:CommonNoun/Card", Number::Singular),
        ]
    );
}

#[test]
fn authentic_plural_heads_do_not_require_plural_noun_modifiers() {
    // Sanguine Indulgence: "Return up to two target creature cards from your
    // graveyard to your hand."
    exact(
        "creature cards",
        Category::Nominal,
        modified(
            "lexeme:type/creature",
            noun("lexeme:CommonNoun/Card", Number::Plural),
        ),
    );
    for text in [
        "creatures card",
        "creatures cards",
        "creature in your graveyard card",
        "creature target card",
    ] {
        assert!(readings(text, Category::Nominal).is_empty(), "{text}");
    }
    let wrong = Reading::NounPremodifier {
        form: 0,
        head: word("lexeme:type/creature", Number::Plural),
    };
    assert!(wrong.admit(&LEXICON).is_err());
    assert!(wrong.realize(&LEXICON).is_err());
}

#[test]
fn authentic_negative_type_nouns_use_the_same_modifier_primitive() {
    // Skyfisher Spider: "When you do, destroy target nonland permanent."
    // Go for the Throat: "Destroy target nonartifact creature."
    // Anowon, the Ruin Sage: "... each player sacrifices a non-Vampire creature
    // ..."
    for (owner, text, head) in [
        (
            "lexeme:type/land/non",
            "nonland permanent",
            "lexeme:CommonNoun/Permanent",
        ),
        (
            "lexeme:type/artifact/non",
            "nonartifact creature",
            "lexeme:type/creature",
        ),
        (
            "lexeme:creature_subtype/vampire/non",
            "non-Vampire creature",
            "lexeme:type/creature",
        ),
    ] {
        exact(
            text,
            Category::Nominal,
            barrier_modified(owner, noun(head, Number::Singular)),
        );
    }
    let target = Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: "vocab:TargetingMarker/Target".into(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    };
    exact(
        "target nonland permanent",
        Category::NounPhrase,
        Reading::TargetNounPhrase {
            form: 0,
            marker: target,
            head: Box::new(barrier_modified(
                "lexeme:type/land/non",
                noun("lexeme:CommonNoun/Permanent", Number::Singular),
            )),
        },
    );
}

fn coordinator(owner: &str) -> Word {
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

#[test]
fn authentic_coordinated_noun_modifiers_share_the_head_with_oxford_serials() {
    // Ogre Battlecaster: "... you may cast target instant or sorcery card from
    // your graveyard ..."
    let binary = Reading::Coordination {
        category: Category::NounPremodifier,
        form: 0,
        left: Box::new(modifier("lexeme:type/instant")),
        coordinator: coordinator("vocab:Coordinator/Or"),
        right: Box::new(modifier("lexeme:type/sorcery")),
    };
    exact(
        "instant or sorcery",
        Category::NounPremodifier,
        binary.clone(),
    );
    let shared = crate::common::nominal(Reading::BarrierNounPremodifiedNominal {
        form: 0,
        modifier: Box::new(binary),
        head: Box::new(noun("lexeme:CommonNoun/Card", Number::Singular)),
    });

    assert_eq!(shared.realize(&LEXICON).unwrap(), "instant or sorcery card");
    let ordinary = Reading::NominalProjection {
        form: 0,
        head: Box::new(Reading::NominalCoreProjection {
            form: 0,
            head: Box::new(Reading::Coordination {
                category: Category::NominalCore,
                form: 0,
                left: Box::new(noun("lexeme:type/instant", Number::Singular)),
                coordinator: coordinator("vocab:Coordinator/Or"),
                right: Box::new(modified(
                    "lexeme:type/sorcery",
                    noun("lexeme:CommonNoun/Card", Number::Singular),
                )),
            }),
        }),
    };
    assert_eq!(
        readings("instant or sorcery card", Category::Nominal),
        BTreeSet::from([shared, ordinary])
    );
    // Custodi Squire: "... each player votes for an artifact, creature, or
    // enchantment card in your graveyard."
    let serial = Reading::SerialCoordination {
        category: Category::NounPremodifier,
        form: 0,
        left: Box::new(modifier("lexeme:type/artifact")),
        rest: Box::new(Reading::CoordinationSeriesEnd {
            category: Category::NounPremodifierSeries,
            form: 0,
            left: Box::new(modifier("lexeme:type/creature")),
            coordinator: coordinator("vocab:Coordinator/Or"),
            right: Box::new(modifier("lexeme:type/enchantment")),
        }),
    };
    exact(
        "artifact, creature, or enchantment",
        Category::NounPremodifier,
        serial,
    );
    for text in [
        "artifact, creature or enchantment",
        "artifact, or creature",
        "instant nor sorcery",
    ] {
        assert!(
            readings(text, Category::NounPremodifier).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn permanent_premodifiers_keep_independently_constructed_noun_identity() {
    // Cosmic Rebirth, Primevals' Glorious Rebirth, and Defiler of Vigor.
    exact(
        "permanent",
        Category::NounPremodifier,
        modifier("lexeme:CommonNoun/Permanent"),
    );
    for (text, owner, number) in [
        ("permanent card", "lexeme:CommonNoun/Card", Number::Singular),
        ("permanent cards", "lexeme:CommonNoun/Card", Number::Plural),
        (
            "permanent spell",
            "lexeme:CommonNoun/Spell",
            Number::Singular,
        ),
    ] {
        exact(
            text,
            Category::Nominal,
            barrier_modified("lexeme:CommonNoun/Permanent", noun(owner, number)),
        );
    }
    // Skyshroud Poacher and Illuna, Apex of Wishes: stacked noun Premodifiers.
    for (text, owner) in [
        ("Elf permanent card", "lexeme:creature_subtype/elf"),
        ("nonland permanent card", "lexeme:type/land/non"),
    ] {
        exact(
            text,
            Category::Nominal,
            barrier_modified(
                owner,
                barrier_modified(
                    "lexeme:CommonNoun/Permanent",
                    noun("lexeme:CommonNoun/Card", Number::Singular),
                ),
            ),
        );
    }
}

#[test]
fn permanent_head_use_is_unchanged_and_plural_modifiers_are_rejected() {
    // Vindicate: "Destroy target permanent."
    exact(
        "permanent",
        Category::Nominal,
        noun("lexeme:CommonNoun/Permanent", Number::Singular),
    );
    let values = readings("target permanent", Category::NounPhrase);
    assert_eq!(values.len(), 1);
    for value in values {
        let mut heads = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::Noun { .. } = node {
                    heads.push(node.clone());
                }
            })
            .unwrap();
        assert_eq!(
            heads,
            vec![crate::common::into_nominal_core(noun("lexeme:CommonNoun/Permanent", Number::Singular))]
        );
    }
    for text in ["permanents card", "permanents cards", "permanents spell"] {
        assert!(readings(text, Category::Nominal).is_empty(), "{text}");
    }
    let plural = Reading::NounPremodifier {
        form: 0,
        head: word("lexeme:CommonNoun/Permanent", Number::Plural),
    };
    assert!(plural.admit(&LEXICON).is_err());
    assert!(plural.realize(&LEXICON).is_err());
}
