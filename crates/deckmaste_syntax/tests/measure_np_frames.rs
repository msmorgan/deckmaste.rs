//! A `MeasureNP` Frame Slot: a slash measure is a Noun Phrase only where a
//! head selects one (*gets +2/+2*). Its Category is no general Noun Phrase,
//! so a verb whose frame selects only an `NP` Object never takes it.

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
use deckmaste_syntax::frames::SlotFill;

fn lexicon() -> Lexicon {
    let source = |kind| Source {
        kind,
        path: "tests/measure_np_frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:get", "get", source(SourceKind::Core)),
        Lexeme::verb("v:draw", "draw", source(SourceKind::Core)),
        Lexeme::noun(
            "n:card",
            "card",
            vec![Countability::Count],
            source(SourceKind::Core),
        ),
        Lexeme::invariant(
            "b:by",
            "by",
            Category::Preposition,
            source(SourceKind::Core),
        ),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:get": [
            (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
            (family: Monotransitive, slots: [(relation: Object, fill: MeasureNP)]),
        ],
        "v:draw": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
    prepositions: {
        "b:by": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

// A numeral stands in for a slash measure: the Slot's Category is what is
// under test, not the measure's internal structure.
const GRAMMAR: &str = r#"(
    root: "Sentence",
    features: {
        "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Voice": ["Active", "Passive"],
    },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm", "Voice"], gap: true),
        "NounPhrase": (),
        "MeasureNounPhrase": (category: Some("NounPhrase")),
        "PrepositionPhrase": (),
    },
    closed_words: { "By": "b:by" },
    slot_categories: {
        NP: "NounPhrase",
        MeasureNP: "MeasureNounPhrase",
        PP: "PrepositionPhrase",
    },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain"), Require("clause.Voice", "Active")]),
        (name: "Measure", lhs: "MeasureNounPhrase", rhs: [Lex("head", Numeral)]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase", active: [Require("$.Voice", "Active")], passive: Some("By"),
            verbal_passive: [Require("$.Voice", "Passive"), Require("$.VerbForm", "PastParticiple")],
            prepositional_passive: [Require("$.Voice", "Passive"), Require("$.VerbForm", "PastParticiple")]),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
    ],
)"#;

fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    CompiledGrammar::bind(
        &Grammar::from_ron_str(GRAMMAR).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
}

/// The one Reading of `text`, checked for Admission and round-trip law 1.
fn only_reading(text: &str) -> Reading {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let mut readings = grammar.parse(&lexicon, text).readings();
    assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
    let reading = readings.remove(0);
    assert_eq!(grammar.admit(&reading, &lexicon), Ok(()));
    assert_eq!(grammar.realize(&reading, &lexicon).unwrap(), text);
    reading
}

/// The Object's fill in the verb phrase's Selected Frame.
fn object_fill(reading: &Reading) -> SlotFill {
    let phrase = reading.node().unwrap().children[0].node().unwrap();
    phrase.selected_frame.as_ref().unwrap().slots[0]
        .fill
        .clone()
}

#[test]
fn a_selecting_verb_takes_a_measure_noun_phrase_as_its_object() {
    let reading = only_reading("Get 2.");
    assert_eq!(object_fill(&reading), SlotFill::MeasureNP);
    let phrase = reading.node().unwrap().children[0].node().unwrap();
    // A Noun Phrase, built only by the measure's own Production.
    let object = phrase.children[1].node().unwrap();
    assert_eq!(object.category, "NounPhrase");
    assert_eq!(object.production, "Measure");
}

#[test]
fn the_selecting_verb_keeps_its_general_object() {
    assert_eq!(object_fill(&only_reading("Get card.")), SlotFill::NP);
}

#[test]
fn a_measure_noun_phrase_is_no_general_object() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    assert_eq!(
        grammar.parse(&lexicon, "Draw 2.").readings(),
        [] as [Reading; 0]
    );
    assert_eq!(grammar.parse(&lexicon, "Draw card.").readings().len(), 1);
}

#[test]
fn a_measure_noun_phrase_is_a_monotransitive_object_only() {
    for (frame, ok) in [
        (
            r"(family: Monotransitive, slots: [(relation: Object, fill: MeasureNP)])",
            true,
        ),
        (
            r"(family: Monotransitive, slots: [(relation: Complement, fill: MeasureNP)])",
            false,
        ),
        (
            r"(family: Ditransitive, slots: [(relation: IndirectObject, fill: NP), (relation: Object, fill: MeasureNP)])",
            false,
        ),
    ] {
        let catalog = format!(r#"(frames: {{"v:get": [{frame}]}})"#);
        assert_eq!(FrameCatalog::from_ron_str(&catalog).is_ok(), ok, "{frame}");
    }
}
