use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{
    FeatureBundle, LexicalReading, LexicalValue, Number, SurfaceCase, WordForm,
};

use crate::common;

fn word(owner: &str, form: WordForm, number: Option<Number>, count: Option<bool>) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features: FeatureBundle {
                number,
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: count,
    }
}

fn invariant(owner: &str) -> Word {
    word(owner, WordForm::Invariant, None, None)
}

fn nominal(owner: &str, number: Number, count: bool) -> Reading {
    Reading::Noun {
        form: 0,
        head: word(
            owner,
            if number == Number::Plural { WordForm::Plural } else { WordForm::Singular },
            Some(number),
            Some(count),
        ),
    }
}

fn superlative(owner: &str, head: Reading) -> Reading {
    Reading::SuperlativePremodifiedNominal {
        form: usize::from(owner != "vocab:Determinative/Most"),
        modifier: invariant(owner),
        head: Box::new(head),
    }
}

fn determined(head: Reading) -> Reading {
    Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant("vocab:DefiniteMarker/The"),
        head: Box::new(head),
    }
}

#[test]
fn the_article_determines_one_shared_superlative_modifier_route() {
    // No Witnesses, Tectonic Hellion and Ghazbán Ogre supply these heads;
    // the owner ruling extends the class to least and fewest on those heads.
    for (text, owner, noun, number, count) in [
        (
            "the most creatures",
            "vocab:Determinative/Most",
            "lexeme:type/creature",
            Number::Plural,
            true,
        ),
        (
            "the most cards",
            "vocab:Determinative/Most",
            "lexeme:CommonNoun/Card",
            Number::Plural,
            true,
        ),
        (
            "the most life",
            "vocab:Determinative/Most",
            "lexeme:CommonNoun/Life",
            Number::Singular,
            false,
        ),
        (
            "the fewest lands",
            "vocab:AttributiveAdjective/Fewest",
            "lexeme:type/land",
            Number::Plural,
            true,
        ),
        (
            "the least life",
            "vocab:Adjective/Least",
            "lexeme:CommonNoun/Life",
            Number::Singular,
            false,
        ),
    ] {
        let expected = determined(superlative(owner, nominal(noun, number, count)));
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::NounPhrase),
            BTreeSet::from([expected]),
            "{text}"
        );
    }
}

#[test]
fn most_keeps_its_bare_determiner_reading_and_declared_number_licence() {
    let expected = Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant("vocab:Determinative/Most"),
        head: Box::new(nominal("lexeme:type/creature", Number::Plural, true)),
    };
    assert_eq!(
        common::readings("most creatures", Category::NounPhrase),
        BTreeSet::from([expected])
    );
    // SM3: neither a nested determined NP nor an analytic adjective superlative.
    for text in [
        "the most common color",
        "the most creature",
        "the most card",
        "a most creature",
        "most most creatures",
    ] {
        assert!(
            common::readings(text, Category::NounPhrase).is_empty(),
            "{text}"
        );
    }
}

#[test]
fn card_witnesses_use_the_superlative_inside_objects_and_with_complements() {
    for (text, category, surface) in [
        (
            "Each player who controls the most creatures investigates. Then destroy all creatures.",
            Category::Document,
            "most creatures",
        ),
        (
            "At the beginning of your upkeep, if a player controls more creatures than each other player, the player who controls the most creatures gains control of this creature.",
            Category::Document,
            "most creatures",
        ),
        (
            "Whenever this creature attacks, each player who controls the most lands sacrifices two lands of their choice.",
            Category::Document,
            "most lands",
        ),
        (
            "At the beginning of your upkeep, if a player has more life than each other player, the player with the most life gains control of this creature.",
            Category::Document,
            "most life",
        ),
        (
            "the player who has the most cards in hand",
            Category::NounPhrase,
            "most cards",
        ),
        (
            "the player with the most life",
            Category::NounPhrase,
            "most life",
        ),
        (
            "the player with the most cards in hand",
            Category::NounPhrase,
            "most cards",
        ),
    ] {
        // A PP may be inside the modified Nominal, rather than outside it.
        let values = common::readings(text, category);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut found = Vec::new();
            value
                .visit(&mut |node| {
                    if let Reading::SuperlativePremodifiedNominal { modifier, .. } = node {
                        assert_eq!(modifier, &invariant("vocab:Determinative/Most"));
                        found.push(node.realize(common::lexicon()).unwrap());
                    }
                })
                .unwrap();
            assert_eq!(found.len(), 1, "one route for {text}");
            assert!(found[0] == surface || found[0] == format!("{surface} in hand"));
        }
    }
}

#[test]
fn independent_pp_attachment_ambiguity_matches_the_fewest_control() {
    for text in [
        "the player with the most cards in hand",
        "the player who has the most cards in hand",
    ] {
        let control = text.replace("most", "fewest");
        let actual = common::readings(text, Category::NounPhrase);
        let control = common::readings(&control, Category::NounPhrase);
        assert_eq!(actual.len(), control.len(), "{text}");
        let mut host_surfaces = BTreeSet::new();
        for value in actual {
            value
                .visit(&mut |node| {
                    if let Reading::PostmodifiedNominal { head, modifier, .. } = node
                        && modifier.realize(common::lexicon()).unwrap() == "in hand"
                    {
                        host_surfaces.insert(head.realize(common::lexicon()).unwrap());
                    }
                })
                .unwrap();
        }
        assert!(host_surfaces.contains("cards"));
        assert!(host_surfaces.contains("most cards"));
    }
}

#[test]
fn witch_king_retains_least_modifying_its_count_scalar_property() {
    let expected = determined(superlative(
        "vocab:Adjective/Least",
        nominal("lexeme:CommonNoun/Power", Number::Singular, true),
    ));
    assert_eq!(
        expected.realize(common::lexicon()).unwrap(),
        "the least power"
    );
    assert_eq!(
        common::readings("the least power", Category::NounPhrase),
        BTreeSet::from([expected])
    );
    let values = common::readings_on_card(
        "Witch-king, Bringer of Ruin",
        "Flying\nWhenever Witch-king attacks, defending player sacrifices a creature with the least power among creatures they control.",
        Category::Document,
    );
    assert!(!values.is_empty());
    for value in values {
        let mut modifiers = Vec::new();
        value
            .visit(&mut |node| {
                if let Reading::SuperlativePremodifiedNominal { modifier, .. } = node {
                    modifiers.push(modifier.clone());
                }
            })
            .unwrap();
        assert_eq!(modifiers, [invariant("vocab:Adjective/Least")]);
    }
}
