//! The stratum profiler on representative Oracle lines: each unit's Stratum
//! is the highest Stratum among the Syntactic Families it needs.

use deckmaste_english_v4::corpus::UnitKind;
use deckmaste_english_v4::profile::Profiler;
use deckmaste_english_v4::profile::Stratum;
use deckmaste_lexical_source_v4::LexicalEnvironment;

fn profile(env: &LexicalEnvironment, kind: UnitKind, text: &str) -> (Stratum, Vec<&'static str>) {
    let profile = Profiler::new(env).profile(kind, &env.lexicon.analyze(text));
    (
        profile.stratum,
        profile.families.iter().map(|f| f.name).collect(),
    )
}

#[test]
fn each_unit_takes_the_highest_stratum_of_its_families() {
    let env = LexicalEnvironment::load().unwrap();
    for (kind, text, stratum, family) in [
        (
            UnitKind::TypeLine,
            "Legendary Creature — Elf Druid",
            0,
            "TypeLine",
        ),
        (UnitKind::Line, "Flying, haste", 0, "KeywordLine"),
        (UnitKind::Line, "Swampwalk", 0, "KeywordLine"),
        (UnitKind::Line, "Choose one —", 0, "ModalHeader"),
        (
            UnitKind::Line,
            "{1}{G}, {T}: Choose one —",
            0,
            "ModalHeader",
        ),
        (
            UnitKind::Line,
            "Sacrifice a creature: Choose one —",
            1,
            "ModalHeader",
        ),
        (UnitKind::Line, "LEVEL 1-3", 0, "LevelHeader"),
        (UnitKind::Line, "4/4", 0, "NumeralLine"),
        (UnitKind::Line, "{T}: Add {G}.", 1, "SymbolCost"),
        (
            UnitKind::Line,
            "Destroy target creature.",
            1,
            "CoreClauseOrPhrase",
        ),
        (UnitKind::Line, "• Draw a card.", 1, "ModeBullet"),
        (UnitKind::Line, "This creature can't block.", 2, "Auxiliary"),
        (
            UnitKind::Line,
            "Target creature gets +2/+2 until end of turn.",
            2,
            "PredicativeComplement",
        ),
        (
            UnitKind::Line,
            "Creatures you control have haste.",
            3,
            "ContactRelative",
        ),
        (
            UnitKind::Line,
            "Destroy target creature that attacked this turn.",
            3,
            "RelativeClause",
        ),
        (
            UnitKind::Line,
            "When this creature enters, draw a card.",
            4,
            "TriggerClause",
        ),
        (
            UnitKind::Line,
            "Landfall — Whenever a land you control enters, you gain 1 life.",
            4,
            "LabelledAbility",
        ),
        (
            UnitKind::Line,
            "This creature attacks each combat if able.",
            4,
            "PrepositionWithClause",
        ),
        (
            UnitKind::Line,
            "Put a +1/+1 counter on each of up to two target creatures.",
            5,
            "Partitive",
        ),
        (
            UnitKind::Line,
            "Draw a card and lose 1 life.",
            6,
            "Coordination",
        ),
        (
            UnitKind::Line,
            "Draw a card, then discard a card.",
            7,
            "DiscourseSequencing",
        ),
        (UnitKind::Line, "If you do, draw a card.", 7, "ProFormDo"),
        (
            UnitKind::Line,
            "Enchanted creature has \"{T}: Add {C}.\"",
            7,
            "QuotedAbility",
        ),
    ] {
        let (found, families) = profile(&env, kind, text);
        assert_eq!(found, stratum, "{text}: {families:?}");
        assert!(families.contains(&family), "{text}: {families:?}");
    }
}

#[test]
fn not_every_preposition_or_to_is_clausal() {
    let env = LexicalEnvironment::load().unwrap();
    for text in [
        // `until` with an NP, and `to` before a premodified nominal.
        "Untap target creature until end of turn.",
        "It deals 2 damage to target creature.",
        "Return it to its owner's hand.",
    ] {
        let (_, families) = profile(&env, UnitKind::Line, text);
        for wrong in ["PrepositionWithClause", "Infinitival", "Auxiliary"] {
            assert!(!families.contains(&wrong), "{text}: {families:?}");
        }
    }
}

#[test]
fn a_flavor_word_label_is_not_a_vocabulary_gap() {
    let env = LexicalEnvironment::load().unwrap();
    let (stratum, families) = profile(&env, UnitKind::Line, "Fathomless descent — Draw a card.");
    assert_eq!(stratum, 1, "{families:?}");
    assert!(families.contains(&"LabelledAbility"));
}

#[test]
fn reviewed_misplacements_stay_fixed() {
    let env = LexicalEnvironment::load().unwrap();
    for (text, stratum) in [
        // A genitive is not a contracted copula; *turn* after *of* is a noun.
        ("Return target creature to its owner's hand.", 1),
        ("Target creature gains flying until end of turn.", 1),
        // A type line profiled as a line (as in the judgment set) is Stratum 0.
        ("Artifact Creature — Golem", 0),
        ("Instant", 0),
        // A depictive participle is not a postmodifier.
        (
            "Put target creature card from a graveyard onto the battlefield tapped.",
            1,
        ),
        (
            "Rather than pay this spell's mana cost, you may sacrifice a Forest.",
            4,
        ),
        ("Put a +1/+1 counter on any number of target creatures.", 5),
        ("All creatures able to block this creature do so.", 7),
        ("III — Hall of Sorrow — Draw two cards.", 1),
        ("It's still a land.", 2),
        ("Craft with two that share a card type {6}", 3),
    ] {
        let (found, families) = profile(&env, UnitKind::Line, text);
        assert_eq!(found, stratum, "{text}: {families:?}");
    }
}
