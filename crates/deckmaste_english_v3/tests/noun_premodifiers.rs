use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_english_v3::grammar::{Category, Grammar, Reading, Word};
use deckmaste_english_v3::parse;
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Lexicon, Number, SurfaceCase, WordForm,
};

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

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
    Reading::Noun {
        form: 0,
        head: word(owner, number),
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

fn readings(text: &str, category: Category) -> BTreeSet<Reading> {
    let grammar = Grammar::default();
    let analyzed = LEXICON.analyze(text);
    let forest = parse(&grammar, &LEXICON, &analyzed, &category).unwrap();
    grammar
        .readings(&forest)
        .map(|reading| {
            let reading = reading.unwrap();
            assert_eq!(reading.realize(&LEXICON).unwrap(), text);
            reading
        })
        .collect()
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
        modified(
            "lexeme:creature_subtype/vampire",
            noun("lexeme:type/creature", Number::Singular),
        ),
    );
    // Mishra's Factory: "Target Assembly-Worker creature gets +1/+1 until end of turn."
    exact(
        "Assembly-Worker creature",
        Category::Nominal,
        modified(
            "lexeme:creature_subtype/assemblyWorker",
            noun("lexeme:type/creature", Number::Singular),
        ),
    );
    // Time Lord Regeneration: "... reveal a Time Lord creature card."
    let expected = modified(
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
    // Sanguine Indulgence: "Return up to two target creature cards from your graveyard to your hand."
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
    // Anowon, the Ruin Sage: "... each player sacrifices a non-Vampire creature ..."
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
            modified(owner, noun(head, Number::Singular)),
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
            head: Box::new(modified(
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
    // Ogre Battlecaster: "... you may cast target instant or sorcery card from your graveyard ..."
    let binary = Reading::NounPremodifierCoordination {
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
    let shared = Reading::NounPremodifiedNominal {
        form: 0,
        modifier: Box::new(binary),
        head: Box::new(noun("lexeme:CommonNoun/Card", Number::Singular)),
    };
    assert_eq!(shared.realize(&LEXICON).unwrap(), "instant or sorcery card");
    let ordinary = Reading::NominalCoordination {
        form: 0,
        left: Box::new(noun("lexeme:type/instant", Number::Singular)),
        coordinator: coordinator("vocab:Coordinator/Or"),
        right: Box::new(modified(
            "lexeme:type/sorcery",
            noun("lexeme:CommonNoun/Card", Number::Singular),
        )),
    };
    assert_eq!(
        readings("instant or sorcery card", Category::Nominal),
        BTreeSet::from([shared, ordinary])
    );
    // Custodi Squire: "... each player votes for an artifact, creature, or enchantment card in your graveyard."
    let serial = Reading::SerialNounPremodifier {
        form: 0,
        left: Box::new(modifier("lexeme:type/artifact")),
        rest: Box::new(Reading::NounPremodifierSeriesEnd {
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
