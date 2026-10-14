//! An intransitive preposition heads a Preposition Phrase alone, as *out* in
//! *phase out*, and a verb frame can select it.

use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::frames::FrameFamily;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/intransitive_prepositions.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:phase", "phase", core()),
        Lexeme::invariant("p:out", "out", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:phase": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:out"]))])],
    },
    prepositions: {
        "p:out": [(complement: Intransitive, hosts: ComplementOnly)],
    },
)"#;

const GRAMMAR: &str = r#"(
    root: "Sentence",
    features: { "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "PrepositionPhrase": (),
    },
    slot_categories: { PP: "PrepositionPhrase" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain")]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase"),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
    ],
)"#;

#[test]
fn a_verb_selects_an_intransitive_preposition() {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let mut readings = grammar.parse(&lexicon, "Phase out.").readings();
    assert_eq!(readings.len(), 1, "{readings:#?}");
    let reading = readings.remove(0);
    assert_eq!(grammar.admit(&reading, &lexicon), Ok(()));
    assert_eq!(grammar.realize(&reading, &lexicon).unwrap(), "Phase out.");
    let clause = reading.node().unwrap().children[0].node().unwrap();
    assert_eq!(
        clause.selected_frame.as_ref().map(|frame| frame.family),
        Some(FrameFamily::Prepositional)
    );
    assert!(
        grammar.parse(&lexicon, "Phase.").readings().is_empty(),
        "the particle is selected, not optional"
    );
}

#[test]
fn only_a_preposition_has_no_complement() {
    let noun = r#"(frames: {"n:x": [(family: NounComplement, slots: [(relation: Complement, fill: Intransitive)])]})"#;
    assert!(FrameCatalog::from_ron_str(noun).is_err());
}
