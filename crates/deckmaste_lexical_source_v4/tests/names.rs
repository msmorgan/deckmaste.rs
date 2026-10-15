//! Card names [CR#201.1] and their Shortened Names [CR#201.5c], which are in
//! scope only on their own card.

use std::collections::BTreeSet;

use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_lexical_source_v4::Sources;
use deckmaste_lexical_source_v4::shortened_names;

fn vocabulary() -> Lexicon {
    Lexicon::new(Sources::load().unwrap().vocabulary().unwrap().0).unwrap()
}

#[test]
fn shortened_names_are_derived_from_the_full_name() {
    let vocabulary = vocabulary();
    for (name, expected) in [
        ("Gisela, Blade of Goldnight", &["Gisela"][..]),
        ("Wolverine, Best There Is", &["Wolverine"]),
        ("Gandalf the Grey", &["Gandalf"]),
        ("Daxos of Meletis", &["Daxos"]),
        ("Edgar Markov", &["Edgar"]),
        // A planeswalker type is the character's own name.
        ("Jace, the Mind Sculptor", &["Jace"]),
        ("Karn Liberated", &["Karn"]),
        // Any other declared word is not a name for the card.
        ("Sliver Queen", &[]),
        ("Goblin Guide", &[]),
        ("Shang-Chi and the Ten Rings", &["Shang-Chi"]),
        ("King Darien XLVIII", &["King Darien"]),
        ("Counterspell", &[]),
    ] {
        assert_eq!(shortened_names(name, &vocabulary), expected, "{name}");
    }
}

#[test]
fn a_shortened_name_is_in_scope_only_on_its_own_card() {
    let environment = LexicalEnvironment::load().unwrap();
    let gisela = LexemeId::new("name:Gisela, Blade of Goldnight");
    let names = |scope: &BTreeSet<LexemeId>| -> BTreeSet<LexemeId> {
        environment
            .lexicon
            .analyze_source_scoped("Gisela", None, scope)
            .matches
            .into_iter()
            .filter_map(|found| match found.reading {
                LexicalReading::Word(value) => Some(value.lexeme),
                _ => None,
            })
            .collect()
    };
    assert!(names(&[gisela].into()).contains(&gisela));
    assert!(!names(&BTreeSet::new()).contains(&gisela));
    // Full names are in scope everywhere.
    let full = environment.lexicon.analyze("Gisela, Blade of Goldnight");
    assert!(full.matches.iter().any(|found| matches!(
        &found.reading,
        LexicalReading::Word(value) if value.lexeme == gisela
    )));
}

/// Whether `text` analyzes, whole, as `lexeme` with `scope`'s cards in scope.
fn spells(environment: &LexicalEnvironment, text: &str, lexeme: &str, scope: &[&str]) -> bool {
    let scope: BTreeSet<LexemeId> = scope
        .iter()
        .map(|card| LexemeId::new(&format!("name:{card}")))
        .collect();
    let analyzed = environment
        .lexicon
        .analyze_source_scoped(text, None, &scope);
    let whole = analyzed.tokens.len();
    analyzed.matches.iter().any(|found| {
        found.start == 0
            && found.end == whole
            && matches!(&found.reading, LexicalReading::Word(value) if value.lexeme == lexeme)
    })
}

#[test]
fn an_introduced_name_is_in_scope_only_on_the_cards_that_use_it() {
    let environment = LexicalEnvironment::load().unwrap();
    for (name, card) in [
        ("Marit Lage", "Dark Depths"),
        ("Marit Lage", "Marit Lage's Slumber"),
        ("Icingdeath, Frost Tongue", "Icingdeath, Frost Tyrant"),
        ("Tuktuk the Returned", "Tuktuk the Explorer"),
        ("Moon", "Princess Yue"),
    ] {
        let id = format!("introduced_name:{name}");
        assert!(spells(&environment, name, &id, &[card]), "{name} on {card}");
        assert!(!spells(&environment, name, &id, &[]), "{name} elsewhere");
        assert!(
            !spells(&environment, name, &id, &["Vampire Hexmage"]),
            "{name} on another card"
        );
    }
    // Its spelling is exact.
    assert!(!spells(
        &environment,
        "moon",
        "introduced_name:Moon",
        &["Princess Yue"]
    ));
}

#[test]
fn every_introducing_card_admits_its_introduced_names() {
    let environment = LexicalEnvironment::load().unwrap();
    let sources = Sources::load().unwrap();
    for introduced in &sources.introduced_names {
        let id = format!("introduced_name:{}", introduced.name);
        for card in &introduced.cards {
            assert!(
                spells(&environment, &introduced.name, &id, &[card]),
                "{} on {card}",
                introduced.name
            );
        }
    }
}
