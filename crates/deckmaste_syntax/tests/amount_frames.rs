//! Amount and Cost Frame Slots: keyword actions such as *scry 2*,
//! *amass Zombies 2* and *waterbend {3}* select a numeric Amount or a Cost, not
//! an Amount complement; amass additionally selects an Object.

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
use deckmaste_syntax::frames::FrameFamily;

fn lexicon() -> Lexicon {
    let source = |kind| Source {
        kind,
        path: "tests/amount_frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut cost = Lexeme::invariant("s:{3}", "{3}", Category::Symbol, source(SourceKind::Symbol));
    cost.capitalization = deckmaste_lexical::Capitalization::Exact;
    Lexicon::new([
        Lexeme::verb("v:scry", "scry", source(SourceKind::Catalog)),
        Lexeme::verb("v:amass", "amass", source(SourceKind::Catalog)),
        Lexeme::verb("v:waterbend", "waterbend", source(SourceKind::Catalog)),
        Lexeme::noun(
            "n:Zombie",
            "Zombie",
            vec![Countability::Count],
            source(SourceKind::Catalog),
        ),
        Lexeme::invariant(
            "b:by",
            "by",
            Category::Preposition,
            source(SourceKind::Core),
        ),
        cost,
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:scry": [(family: Amount, slots: [(relation: Complement, fill: Amount)])],
        "v:amass": [(family: TransitiveAmount, slots: [
            (relation: Object, fill: NP),
            (relation: Complement, fill: Amount),
        ])],
        "v:waterbend": [(family: Cost, slots: [(relation: Complement, fill: Cost)])],
    },
    prepositions: {
        "b:by": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

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
        "Nominal": (),
        "AmountPhrase": (),
        "CostPhrase": (),
        "PrepositionPhrase": (),
    },
    closed_words: { "By": "b:by" },
    slot_categories: {
        NP: "NounPhrase",
        PP: "PrepositionPhrase",
        Nominal: "Nominal",
        Amount: "AmountPhrase",
        Cost: "CostPhrase",
    },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain"), Require("clause.Voice", "Active")]),
        (name: "Numeral", lhs: "AmountPhrase", rhs: [Lex("head", Numeral)]),
        (name: "Symbol", lhs: "CostPhrase", rhs: [Lex("head", Symbol)]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Cat("head", "Nominal")]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase", passive: Some("By")),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
        Valence(head: Noun, phrase: "Nominal"),
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

fn family(reading: &Reading) -> Option<FrameFamily> {
    let clause = reading.node()?.children.first()?.node()?;
    clause.selected_frame.as_ref().map(|frame| frame.family)
}

#[test]
fn a_verb_selects_an_amount() {
    assert_eq!(family(&only_reading("Scry 2.")), Some(FrameFamily::Amount));
}

#[test]
fn a_verb_selects_an_object_then_an_amount() {
    assert_eq!(
        family(&only_reading("Amass Zombies 2.")),
        Some(FrameFamily::TransitiveAmount)
    );
}

#[test]
fn a_verb_selects_a_cost() {
    assert_eq!(
        family(&only_reading("Waterbend {3}.")),
        Some(FrameFamily::Cost)
    );
}

#[test]
fn an_amount_and_a_cost_are_selected_not_optional() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    for text in ["Scry.", "Waterbend 2.", "Scry {3}.", "Amass 2."] {
        assert!(
            grammar.parse(&lexicon, text).readings().is_empty(),
            "{text}"
        );
    }
}

#[test]
fn amount_and_cost_frames_have_their_family_shape() {
    for malformed in [
        r#"(frames: {"v:scry": [(family: Amount, slots: [(relation: Object, fill: Amount)])]})"#,
        r#"(frames: {"v:scry": [(family: Amount, slots: [(relation: Complement, fill: NP)])]})"#,
        r#"(frames: {"v:amass": [(family: TransitiveAmount, slots: [(relation: Complement, fill: Amount)])]})"#,
        r#"(frames: {"v:waterbend": [(family: Cost, slots: [(relation: Complement, fill: Amount)])]})"#,
    ] {
        assert!(
            FrameCatalog::from_ron_str(malformed).is_err(),
            "{malformed}"
        );
    }
}

#[test]
fn complex_and_clausal_frames_have_their_family_shape() {
    for (frame, ok) in [
        (
            r"(family: ComplexTransitive, slots: [(relation: Object, fill: NP), (relation: PredicativeComplement, fill: Pred)])",
            true,
        ),
        (
            r"(family: ComplexTransitive, slots: [(relation: Object, fill: NP)])",
            false,
        ),
        (
            r"(family: ComplexCatenative, slots: [(relation: Object, fill: NP), (relation: Complement, fill: ToInf)])",
            true,
        ),
        (
            r"(family: ComplexCatenative, slots: [(relation: Object, fill: NP), (relation: Complement, fill: Gerund)])",
            false,
        ),
        (
            r"(family: ContentClause, slots: [(relation: Complement, fill: Clause(Finite))])",
            true,
        ),
        (
            r"(family: ContentClause, slots: [(relation: Complement, fill: Clause(Verbless))])",
            false,
        ),
    ] {
        let catalog = format!(r#"(frames: {{"v:scry": [{frame}]}})"#);
        assert_eq!(FrameCatalog::from_ron_str(&catalog).is_ok(), ok, "{frame}");
    }
}
