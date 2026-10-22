//! Card names [CR#201.1] and their Shortened Names [CR#201.5c], which are in
//! scope only on their own card.

use std::collections::BTreeSet;

use deckmaste_lexical::LexemeId;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical_source_v4::AttestedShortenings;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_lexical_source_v4::Sources;
use deckmaste_lexical_source_v4::shortened_name;

fn vocabulary() -> Lexicon {
    Lexicon::new(Sources::load().unwrap().vocabulary().unwrap().0).unwrap()
}

#[test]
fn a_card_has_the_shortened_name_its_own_text_uses() {
    let vocabulary = vocabulary();
    let attested = AttestedShortenings::load().unwrap();
    for (name, expected) in [
        ("Gandalf the Grey", Some("Gandalf")),
        ("Daxos of Meletis", Some("Daxos")),
        ("Edgar Markov", Some("Edgar")),
        // The explicit name before a comma is the character's own name.
        ("Wolverine, Best There Is", Some("Wolverine")),
        ("Karn Liberated", Some("Karn")),
        ("Shang-Chi and the Ten Rings", Some("Shang-Chi")),
        ("King Darien XLVIII", Some("King Darien")),
        ("Halana and Alena, Partners", Some("Halana and Alena")),
        // Its text uses *Sliver* only for other Slivers.
        ("Sliver Queen", None),
        // Text that never shortens its card's name gives it no shortening.
        ("Gisela, Blade of Goldnight", None),
        ("Feast of Flesh", None),
        ("Counterspell", None),
        // *Hornet* and *Wirefly* are the tokens these cards create.
        ("Hornet Cannon", None),
        ("Wirefly Hive", None),
    ] {
        assert_eq!(
            shortened_name(name, attested.get(name), &vocabulary).as_deref(),
            expected,
            "{name}"
        );
    }
}

#[test]
fn a_shortened_name_never_spells_vocabulary_in_scope_on_its_card() {
    let vocabulary = vocabulary();
    let short = |name: &str, candidate: &str| shortened_name(name, Some(candidate), &vocabulary);
    // A planeswalker type, or the name before a comma, is the character's
    // own name even where other words spell it.
    assert_eq!(
        short("Jace, the Mind Sculptor", "Jace").as_deref(),
        Some("Jace")
    );
    assert_eq!(
        short("Wolverine, Best There Is", "Wolverine").as_deref(),
        Some("Wolverine")
    );
    assert_eq!(short("Goblin Guide", "Goblin"), None);
    // An Introduced Name is other vocabulary on the cards that introduce it,
    // and only there.
    assert_eq!(short("Hornet Cannon", "Hornet"), None);
    assert_eq!(short("Wirefly Hive", "Wirefly"), None);
    assert_eq!(short("Hornet Nest", "Hornet").as_deref(), Some("Hornet"));
    // A token name is other vocabulary even before a comma.
    assert_eq!(short("Walker, Night Stalker", "Walker"), None);
}

#[test]
fn a_shortened_name_is_in_scope_only_on_its_own_card() {
    let environment = LexicalEnvironment::load().unwrap();
    let dina = LexemeId::new("name:Dina, Soul Steeper");
    let names = |scope: &BTreeSet<LexemeId>| -> BTreeSet<LexemeId> {
        environment
            .lexicon
            .analyze_source_scoped("Dina", None, scope)
            .matches
            .into_iter()
            .filter_map(|found| match found.reading {
                LexicalReading::Word(value) => Some(value.lexeme),
                _ => None,
            })
            .collect()
    };
    assert!(names(&[dina].into()).contains(&dina));
    assert!(!names(&BTreeSet::new()).contains(&dina));
    // Full names are in scope everywhere.
    let full = environment.lexicon.analyze("Dina, Soul Steeper");
    assert!(full.matches.iter().any(|found| matches!(
        &found.reading,
        LexicalReading::Word(value) if value.lexeme == dina
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
