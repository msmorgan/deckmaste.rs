//! Idiomatic coordinates: a pro-form after a declared coordinator ends a
//! coordination, standing for further conjuncts of its kind (*and so on*;
//! CGEL, Ch. 17 §7.7.3(e), p. 1540).

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceStructure;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/idiomatic_coordinates.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut so_on = Lexeme::invariant("adv:so on", "so on", Category::Adverb, core());
    so_on.surface_structure = SurfaceStructure::Multiword;
    Lexicon::new([
        Lexeme::verb("v:give", "give", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        so_on,
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:give": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
)"#;

fn count(text: &str) -> usize {
    let source = r#"(
    root: "Sentence",
    features: { "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "NounPhrase": (),
        "IdiomaticCoordinate": (),
    },
    lexical_classes: { "Coordinator": ["c:and", "c:or"], "CoordinateAdverb": ["adv:so on"] },
    closed_words: { "And": "c:and" },
    slot_categories: { NP: "NounPhrase" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("head", "VerbPhrase"), Punct('.')], equations: [Require("head.VerbForm", "Plain")]),
        (name: "Noun", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
        (name: "IdiomaticCoordinate", lhs: "IdiomaticCoordinate", rhs: [Class("head", "CoordinateAdverb")]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase"),
        Coordination(category: "NounPhrase", coordinators: "Coordinator", delimiter: Some(',')),
        // A second schema whose coordinators admit *and* derives no second
        // tree of the idiomatic coordinate.
        Coordination(category: "NounPhrase", coordinators: "Coordinator", cost: 2),
    ],
    idiomatic_coordinates: [("And", "IdiomaticCoordinate")],
)"#;
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(source).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let found = grammar.parse(&lexicon, text).readings();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn an_idiomatic_coordinate_ends_a_coordination() {
    assert_eq!(count("Give card, card, and so on."), 1);
    assert_eq!(count("Give card and so on."), 1);
}

#[test]
fn it_follows_only_its_coordinator() {
    assert_eq!(count("Give card, card, or so on."), 0);
}

#[test]
fn it_is_only_the_last_coordinate() {
    assert_eq!(count("Give so on."), 0);
    assert_eq!(count("Give so on and card."), 0);
    assert_eq!(count("Give card, so on, and card."), 0);
}
