//! Closures decided on the whole chart: a Derivation that a longer analysis of
//! the words after it rules out. A fused-head Production forms no Constituent
//! where its twin, the Production with the same right-hand side, is followed
//! by its head: that nominal is the head, not an antecedent (CGEL Ch. 17 §6.1,
//! pp. 1516–1517).

use deckmaste_lexical::Category;
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
        path: "tests/chart_closures.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
        Lexeme::invariant("d:own", "own", Category::Determinative, core()),
    ])
    .unwrap()
}

/// `own` stands in for a genitive marker: *card own* is a subject-determiner
/// (`Genitive`) or a fused subject-determiner-head (`Fused`).
fn genitive_grammar(
    lexicon: &Lexicon,
    fused: &str,
) -> Result<CompiledGrammar, deckmaste_syntax::LoadError> {
    let text = format!(
        r#"(
        root: "Sentence",
        features: {{ "Role": ["Whole", "Determiner"] }},
        categories: {{
            "Sentence": (),
            "Predicate": (),
            "Nominal": (),
            "NounPhrase": (features: ["Role"]),
        }},
        closed_words: {{ "Own": "d:own" }},
        productions: [
            (name: "Statement", lhs: "Sentence", rhs: [Cat("head", "Predicate"), Punct('.')]),
            (name: "Subject", lhs: "Predicate", rhs: [Cat("subject", "NounPhrase")], equations: [Require("subject.Role", "Whole")]),
            (name: "WithAdjunct", lhs: "Predicate", rhs: [Cat("subject", "NounPhrase"), Cat("adjunct", "NounPhrase")],
                equations: [Require("subject.Role", "Whole"), Require("adjunct.Role", "Whole")]),
            (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
            (name: "Bare", lhs: "NounPhrase", rhs: [Cat("head", "Nominal")], equations: [Require("$.Role", "Whole")]),
            (name: "Genitive", lhs: "NounPhrase", rhs: [Lex("possessor", Noun), Word("marker", "Own")], equations: [Require("$.Role", "Determiner")]),
            (name: "Fused", lhs: "NounPhrase", rhs: [Lex("possessor", Noun), Word("marker", "Own")], equations: [Require("$.Role", "Whole")]{fused}),
            (name: "Determined", lhs: "NounPhrase", rhs: [Cat("determiner", "NounPhrase"), Cat("head", "Nominal")],
                equations: [Carry("determiner.Role", "Determiner"), Require("$.Role", "Whole")]),
        ],
    )"#
    );
    CompiledGrammar::bind(
        &Grammar::from_ron_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
}

fn uses(reading: &Reading, production: &str) -> bool {
    reading.node().is_some_and(|node| {
        node.production.as_str() == production || node.children.iter().any(|c| uses(c, production))
    })
}

#[test]
fn a_nominal_right_after_a_genitive_is_its_head() {
    let lexicon = lexicon();
    let grammar = genitive_grammar(&lexicon, ", fused_head: true").unwrap();
    // With nothing after it, the genitive is a fused head.
    let found = grammar.parse(&lexicon, "Card own.").readings();
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(uses(&found[0], "Fused"));
    assert_eq!(grammar.admit(&found[0], &lexicon), Ok(()));
    // Followed by a nominal it determines, it is a subject-determiner only.
    let found = grammar.parse(&lexicon, "Card own card.").readings();
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(uses(&found[0], "Determined") && !uses(&found[0], "Fused"));
    assert_eq!(grammar.admit(&found[0], &lexicon), Ok(()));
}

#[test]
fn admission_rejects_a_fused_head_before_its_head() {
    let lexicon = lexicon();
    let open = genitive_grammar(&lexicon, "").unwrap();
    let stranded = open
        .parse(&lexicon, "Card own card.")
        .readings()
        .into_iter()
        .find(|r| uses(r, "Fused"))
        .expect("without the closure the fused head strands its nominal");
    let closed = genitive_grammar(&lexicon, ", fused_head: true").unwrap();
    assert!(closed.admit(&stranded, &lexicon).is_err());
}

#[test]
fn a_fused_head_needs_one_twin() {
    let lexicon = lexicon();
    let text = |twin: &str| {
        format!(
            r#"(
            root: "NounPhrase",
            categories: {{ "NounPhrase": (), "Nominal": () }},
            closed_words: {{ "Own": "d:own" }},
            productions: [
                (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
                (name: "Fused", lhs: "NounPhrase", rhs: [Lex("possessor", Noun), Word("marker", "Own")], fused_head: true),
                {twin}
                (name: "Determined", lhs: "NounPhrase", rhs: [Cat("determiner", "NounPhrase"), Cat("head", "Nominal")]),
            ],
        )"#
        )
    };
    let bind = |twin: &str| {
        CompiledGrammar::bind(
            &Grammar::from_ron_str(&text(twin)).unwrap(),
            &lexicon,
            &FrameCatalog::default(),
        )
    };
    assert!(bind("").is_err());
    let twin = r#"(name: "Genitive", lhs: "NounPhrase", rhs: [Lex("possessor", Noun), Word("marker", "Own")]),"#;
    assert!(bind(twin).is_ok());
}
