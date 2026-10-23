//! An `unlike` Coordination schema coordinates conjuncts of unlike kinds in
//! one function (CGEL, Ch. 15 §3.2, pp. 1326–1329). Its conjuncts need only
//! not all be of one kind: in a serial or polysyndetic coordination, the
//! first and last may be alike when a middle one differs (*indestructible,
//! "…," and equip {0}*), and each chain still has one tree. An equation over
//! `initial` holds of the initial conjunct alone, where one over `first`
//! holds of every conjunct but the last.

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
        path: "tests/unlike_serials.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:land", "land", vec![Countability::Count], core()),
        Lexeme::noun("n:mana", "mana", vec![Countability::Mass], core()),
    ])
    .unwrap()
}

fn grammar(lexicon: &Lexicon, equations: &str) -> CompiledGrammar {
    let text = format!(
        r#"(
        root: "Phrase",
        features: {{ "Determination": ["Bare", "Overt"] }},
        categories: {{
            "Phrase": (),
            "NounPhrase": (features: ["Determination"]),
        }},
        closed_words: {{ "The": "d:the", "And": "c:and" }},
        productions: [
            (name: "Bare", lhs: "NounPhrase", rhs: [Lex("head", Noun)], equations: [Require("$.Determination", "Bare")]),
            (name: "Determined", lhs: "NounPhrase", rhs: [Word("determiner", "The"), Lex("head", Noun)], equations: [Require("$.Determination", "Overt")]),
            (name: "Plain", lhs: "Phrase", rhs: [Cat("head", "NounPhrase")]),
        ],
        schemas: [
            Coordination(category: "NounPhrase", coordinators: "And", unshared: ["Determination"], unlike: Some("Determination"), serial: Some(true), delimiter: Some(','), equations: [{equations}]),
        ],
    )"#
    );
    let grammar = Grammar::from_ron_str(&text).unwrap();
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).unwrap()
}

/// The Readings of `text`, each checked for Admission and round-trip law 1.
fn readings(text: &str) -> Vec<Reading> {
    readings_with("", text)
}

fn readings_with(equations: &str, text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, equations);
    let readings = grammar.parse(&lexicon, text).readings();
    for reading in &readings {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings
}

#[test]
fn unlike_conjuncts_are_not_all_alike() {
    for text in [
        // The first and last unlike, as before.
        "card, land, and the mana",
        "the card, land, and mana",
        // The first and last alike, a middle one unlike.
        "card, the land, and mana",
        "the card, land, and the mana",
        "card, land, the card, and mana",
        // Polysyndetic.
        "card, and the land, and mana",
    ] {
        let readings = readings(text);
        assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
    }
}

#[test]
fn like_conjuncts_are_no_unlike_coordination() {
    for text in [
        "card and land",
        "card, land, and mana",
        "the card, the land, and the mana",
        "card, and land, and mana",
    ] {
        assert_eq!(readings(text).len(), 0, "{text}");
    }
}

#[test]
fn an_initial_equation_holds_of_the_initial_conjunct_alone() {
    let initial = r#"Require("initial.Determination", "Overt")"#;
    for text in [
        "the card and land",
        "the card, land, and mana",
        "the card, the land, and mana",
        "the card, land, and the mana",
        "the card, and land, and mana",
    ] {
        assert_eq!(readings_with(initial, text).len(), 1, "{text}");
    }
    for text in [
        "card and the land",
        "card, the land, and mana",
        "card, land, and the mana",
        "card, and the land, and mana",
    ] {
        assert_eq!(readings_with(initial, text).len(), 0, "{text}");
    }
}
