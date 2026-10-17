//! A Production can require a child to be light (one lexical leaf) or heavy
//! (more than one), as a single postpositive participle differs from a
//! participial clause.

use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;

fn lexicon() -> Lexicon {
    let core = Source {
        kind: SourceKind::Core,
        path: "tests/weights.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([Lexeme::invariant("n:card", "card", Category::Noun, core)]).unwrap()
}

fn grammar(lexicon: &Lexicon, light: &str) -> Result<CompiledGrammar, deckmaste_syntax::LoadError> {
    let text = format!(
        r#"(
        root: "Sentence",
        categories: {{ "Sentence": (), "Words": () }},
        productions: [
            (name: "Word", lhs: "Words", rhs: [Lex("head", Noun)]),
            (name: "MoreWords", lhs: "Words", rhs: [Cat("head", "Words"), Lex("next", Noun)]),
            (name: "Light", lhs: "Sentence", rhs: [Cat("words", "Words"), Punct('.')], light: ["{light}"]),
            (name: "Heavy", lhs: "Sentence", rhs: [Cat("words", "Words"), Punct('!')], heavy: ["words"]),
        ],
    )"#
    );
    CompiledGrammar::bind(
        &Grammar::from_ron_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
}

#[test]
fn a_light_child_is_one_leaf_and_a_heavy_child_more() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, "words").unwrap();
    for (text, readings) in [
        ("Card.", 1),
        ("Card card.", 0),
        ("Card!", 0),
        ("Card card!", 1),
    ] {
        let found = grammar.parse(&lexicon, text).readings();
        assert_eq!(found.len(), readings, "{text}: {found:#?}");
        for reading in &found {
            assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        }
    }
}

#[test]
fn a_weighted_child_must_exist() {
    assert!(grammar(&lexicon(), "nothing").is_err());
}
