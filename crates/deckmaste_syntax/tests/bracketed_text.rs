//! A Bracketed Constituent: an unselected dependent (`adjunct`, `modifier`) may be set
//! in square brackets, the text a cleave spell removes [CR#702.148a]
//! (`docs/oracle-style-guide.md` §3, Bullets, vertical bars, and square
//! brackets). The engine derives the bracketed twin of every Production with
//! such a child, so the text reads with and without its brackets, both
//! round-trip laws hold, and a selected Complement is never Bracketed.

use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/bracketed_text.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("pro:it", "it", Category::Pronoun, core()),
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
        Lexeme::invariant("adj:red", "red", Category::Adjective, core()),
        Lexeme::verb("v:draw", "draw", core()),
    ])
    .unwrap()
}

fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    let text = r#"(
        root: "Sentence",
        categories: { "Sentence": (), "Clause": (), "Nominal": (), "Attribute": () },
        productions: [
            (name: "Declarative", lhs: "Sentence", rhs: [Cat("clause", "Clause"), Punct('.')]),
            (name: "Transitive", lhs: "Clause", rhs: [Lex("subject", Pronoun), Lex("head", Verb), Cat("object", "Nominal")]),
            (name: "Bare", lhs: "Nominal", rhs: [Lex("head", Noun)]),
            (name: "Premodified", lhs: "Nominal", rhs: [Cat("modifier", "Attribute"), Cat("head", "Nominal")]),
            (name: "Attribute", lhs: "Attribute", rhs: [Lex("head", Adjective)]),
        ],
        schemas: [],
    )"#;
    let grammar = Grammar::from_ron_str(text).unwrap();
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).unwrap()
}

#[test]
fn bracketed_text_reads_with_and_without_its_brackets() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    for text in [
        "it draws red card.",
        "it draws [red] card.",
        "it draws [red] [red] card.",
        "it draws red [red] card.",
    ] {
        let parse = grammar.parse(&lexicon, text);
        let readings = parse.readings();
        assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
        assert_eq!(parse.verify_realization(&lexicon), Ok(()), "{text}");
        assert_eq!(grammar.admit(&readings[0], &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(&readings[0], &lexicon).unwrap(), text);
        assert!(parse.contains(&readings[0]), "{text}");
    }
}

#[test]
fn only_an_unselected_dependent_is_bracketed() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    for text in [
        // A selected Complement and a head leave no whole phrase behind.
        "it draws [card].",
        "it draws red [card].",
        // Brackets pair around one dependent, once.
        "it draws [[red]] card.",
        "it draws [red card.",
        "it draws red] card.",
    ] {
        assert!(
            grammar.parse(&lexicon, text).readings().is_empty(),
            "{text}"
        );
    }
}
