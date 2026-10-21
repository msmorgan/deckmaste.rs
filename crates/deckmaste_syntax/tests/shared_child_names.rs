//! An equation on a child name that several children share constrains each
//! of them that carries the feature, not only the first: two preposed
//! Adjuncts set off by one comma both bear `adjunct`, and a requirement on
//! `adjunct` holds of both.

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
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
        path: "tests/shared_child_names.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:land", "land", vec![Countability::Count], core()),
    ])
    .unwrap()
}

fn grammar(lexicon: &Lexicon, equations: &str) -> Result<CompiledGrammar, String> {
    let text = format!(
        r#"(
        root: "Phrase",
        features: {{ "Determination": ["Bare", "Overt"] }},
        categories: {{
            "Phrase": (),
            "NounPhrase": (features: ["Determination"]),
        }},
        closed_words: {{ "The": "d:the" }},
        productions: [
            (name: "Bare", lhs: "NounPhrase", rhs: [Lex("head", Noun)], equations: [Require("$.Determination", "Bare")]),
            (name: "Determined", lhs: "NounPhrase", rhs: [Word("determiner", "The"), Lex("head", Noun)], equations: [Require("$.Determination", "Overt")]),
            (name: "Pair", lhs: "Phrase", rhs: [Cat("adjunct", "NounPhrase"), Cat("adjunct", "NounPhrase"), Punct('.')], equations: [{equations}]),
        ],
    )"#
    );
    let grammar = Grammar::from_ron_str(&text).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).map_err(|e| format!("{e:?}"))
}

fn count(equations: &str, text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, equations).unwrap();
    let found = grammar.parse(&lexicon, text).readings();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
    }
    found.len()
}

#[test]
fn a_requirement_on_a_shared_name_holds_of_every_child() {
    let overt = r#"Require("adjunct.Determination", "Overt")"#;
    assert_eq!(count(overt, "The card the land."), 1);
    assert_eq!(count(overt, "Card the land."), 0);
    assert_eq!(count(overt, "The card land."), 0);
}

#[test]
fn an_exclusion_on_a_shared_name_holds_of_every_child() {
    let bare = r#"Exclude("adjunct.Determination", "Bare")"#;
    assert_eq!(count(bare, "The card the land."), 1);
    assert_eq!(count(bare, "Card the land."), 0);
    assert_eq!(count(bare, "The card land."), 0);
}
