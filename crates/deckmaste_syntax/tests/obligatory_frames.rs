//! An `obligatory` Valence phrase keeps its bare form to heads that declare
//! no Subcategorization Frame or an Intransitive one: another head realizes
//! its frame.

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
        path: "tests/obligatory_frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut lexemes = vec![
        Lexeme::verb("v:see", "see", core()),
        Lexeme::invariant("p:of", "of", Category::Preposition, core()),
    ];
    for noun in ["turn", "end", "top"] {
        lexemes.push(Lexeme::noun(
            format!("n:{noun}"),
            noun,
            vec![Countability::Count],
            core(),
        ));
    }
    Lexicon::new(lexemes).unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:see": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "n:end": [(family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))])],
        "n:top": [
            (family: NounComplement, slots: [(relation: Complement, fill: PP(["p:of"]))]),
            (family: Intransitive, slots: []),
        ],
    },
    prepositions: {
        "p:of": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

fn grammar(obligatory: bool) -> String {
    format!(
        r#"(
    root: "Sentence",
    features: {{
        "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"],
    }},
    categories: {{
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "NounPhrase": (),
        "Nominal": (),
        "PrepositionPhrase": (),
    }},
    slot_categories: {{ NP: "NounPhrase", PP: "PrepositionPhrase" }},
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain")]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Cat("head", "Nominal")]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase"),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
        Valence(head: Noun, phrase: "Nominal", obligatory: {obligatory}),
    ],
)"#
    )
}

/// Every Reading of `text`, each checked for Admission and round-trip law 1.
fn readings(obligatory: bool, text: &str) -> Vec<Reading> {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(&grammar(obligatory)).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let readings = grammar.parse(&lexicon, text).readings();
    for reading in &readings {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()));
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    readings
}

#[test]
fn a_frameless_head_forms_the_bare_phrase() {
    assert_eq!(readings(true, "See turn.").len(), 1);
}

#[test]
fn a_head_with_a_frame_realizes_it() {
    assert_eq!(readings(true, "See end of turn.").len(), 1);
    assert_eq!(readings(true, "See end.").len(), 0);
}

#[test]
fn a_frame_is_optional_without_obligatory() {
    assert_eq!(readings(false, "See end.").len(), 1);
}

#[test]
fn an_intransitive_frame_makes_the_complement_optional() {
    assert_eq!(readings(true, "See top.").len(), 1);
    assert_eq!(readings(true, "See top of turn.").len(), 1);
    // The bare phrase is the Intransitive frame's only realization.
    assert_eq!(readings(false, "See top.").len(), 1);
}
