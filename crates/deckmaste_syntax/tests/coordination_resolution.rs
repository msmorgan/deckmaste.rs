//! A Coordination schema's `resolve` features: every conjunct but the last
//! shares the value, and the coordination takes the common value with the
//! last conjunct's, or the schema's value when they differ, so a coordination
//! of bare Noun Phrases is no determined one (*all cards and lands* has no
//! predeterminer analysis). An `ExcludeTogether` equation over `first` and
//! `last` rejects a combination of the last conjunct's values with those of
//! the one before it (*the card and mana*), and so, with a resolved feature,
//! of every conjunct before it.

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::Reading;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/coordination_resolution.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::invariant("d:all", "all", Category::Determinative, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:land", "land", vec![Countability::Count], core()),
        Lexeme::noun("n:mana", "mana", vec![Countability::Mass], core()),
    ])
    .unwrap()
}

fn grammar(lexicon: &Lexicon, coordination: &str) -> Result<CompiledGrammar, String> {
    let text = format!(
        r#"(
        root: "Phrase",
        features: {{
            "Determination": ["Bare", "Overt"],
            "Countability": ["Count", "Mass"],
        }},
        categories: {{
            "Phrase": (),
            "NounPhrase": (features: ["Determination", "Countability"]),
        }},
        closed_words: {{ "The": "d:the", "All": "d:all", "And": "c:and" }},
        productions: [
            (name: "Bare", lhs: "NounPhrase", rhs: [Lex("head", Noun)], equations: [Require("$.Determination", "Bare")]),
            (name: "Determined", lhs: "NounPhrase", rhs: [Word("determiner", "The"), Lex("head", Noun)], equations: [Require("$.Determination", "Overt")]),
            (name: "Plain", lhs: "Phrase", rhs: [Cat("head", "NounPhrase")]),
            (name: "Predetermined", lhs: "Phrase", rhs: [Word("predeterminer", "All"), Cat("head", "NounPhrase")], equations: [Require("head.Determination", "Overt")]),
        ],
        schemas: [
            {coordination},
        ],
    )"#
    );
    let grammar = Grammar::from_ron_str(&text).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).map_err(|e| format!("{e:?}"))
}

const RESOLVED: &str = r#"Coordination(category: "NounPhrase", coordinators: "And", resolve: [("Determination", "Bare")], proximate: ["Countability"], equations: [ExcludeTogether([("first.Determination", "Overt"), ("last.Determination", "Bare"), ("last.Countability", "Mass")])])"#;

const UNSHARED: &str = r#"Coordination(category: "NounPhrase", coordinators: "And", unshared: ["Determination", "Countability"])"#;

fn productions(reading: &Reading) -> Vec<String> {
    match reading {
        Reading::Node(node) => std::iter::once(node.production.to_string())
            .chain(node.children.iter().flat_map(productions))
            .collect(),
        Reading::Leaf(_) => Vec::new(),
    }
}

/// The Readings of `text`, each checked for Admission and round-trip law 1.
fn readings(coordination: &str, text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, coordination).unwrap();
    let readings = grammar.parse(&lexicon, text).readings();
    for reading in &readings {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings
}

fn predetermined(coordination: &str, text: &str) -> bool {
    readings(coordination, text)
        .iter()
        .any(|r| productions(r).contains(&"Predetermined".to_owned()))
}

#[test]
fn a_coordination_is_determined_only_when_every_conjunct_is() {
    assert!(predetermined(RESOLVED, "all the card and the land"));
    assert!(!predetermined(RESOLVED, "all card and land"));
    assert!(!predetermined(RESOLVED, "all card and the land"));
    assert!(!predetermined(RESOLVED, "all the card, the land, and land"));
    assert!(predetermined(RESOLVED, "all the card, the land, and the mana"));
}

#[test]
fn an_unshared_feature_is_left_unvalued_and_admits_any_requirement() {
    assert!(predetermined(UNSHARED, "all card and land"));
}

#[test]
fn a_combination_is_excluded_only_when_every_condition_holds() {
    for text in [
        "the card and the mana",
        "the card and land",
        "mana and the card",
        "card and mana",
    ] {
        assert_eq!(readings(RESOLVED, text).len(), 1, "{text}");
    }
    assert_eq!(readings(RESOLVED, "the card and mana").len(), 0);
    assert_eq!(readings(UNSHARED, "the card and mana").len(), 1);
}

#[test]
fn every_conjunct_but_the_last_shares_a_resolved_feature() {
    for text in [
        "card, land, and the mana",
        "the card, the land, and land",
        "the card, the land, and the mana",
        "card, mana, and mana",
    ] {
        assert_eq!(readings(RESOLVED, text).len(), 1, "{text}");
    }
    for text in ["the card, land, and mana", "card, the land, and the mana"] {
        assert_eq!(readings(RESOLVED, text).len(), 0, "{text}");
    }
    // The conjunct before the last shares the others' value.
    assert_eq!(readings(RESOLVED, "the card, the land, and mana").len(), 0);
}

#[test]
fn malformed_resolutions_and_joint_exclusions_are_rejected() {
    let lexicon = lexicon();
    for bad in [
        r#"Coordination(category: "NounPhrase", coordinators: "And", unshared: ["Determination"], resolve: [("Determination", "Bare")])"#,
        r#"Coordination(category: "NounPhrase", coordinators: "And", resolve: [("Determination", "Some")])"#,
        r#"Coordination(category: "NounPhrase", coordinators: "And", equations: [ExcludeTogether([("first.Determination", "Overt"), ("$.Countability", "Mass")])])"#,
        r#"Coordination(category: "NounPhrase", coordinators: "And", equations: [ExcludeTogether([("first.Determination", "Overt")])])"#,
    ] {
        assert!(grammar(&lexicon, bad).is_err(), "{bad}");
    }
}
