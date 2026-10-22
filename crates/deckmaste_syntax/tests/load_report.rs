//! Binding reports a feature with one value and a feature value with one
//! holder as findings (`english-lexical-analysis.md` §3, `english-v4.md`
//! §4.4 item 5), without refusing the grammar.

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;

fn source() -> Source {
    Source {
        kind: SourceKind::Core,
        path: "tests/load_report.rs".to_owned(),
        owner: "test".to_owned(),
    }
}

/// Two common nouns and one rare one.
fn lexicon() -> Lexicon {
    let mut lexemes = vec![
        Lexeme::invariant("det:the", "the", Category::Determinative, source()),
        Lexeme::invariant("det:a", "a", Category::Determinative, source()),
    ];
    for (noun, kind) in [("card", "Common"), ("land", "Common"), ("relic", "Rare")] {
        let mut lexeme = Lexeme::noun(
            format!("noun:{noun}"),
            noun,
            vec![Countability::Count],
            source(),
        );
        lexeme
            .properties
            .features
            .insert("Kind".into(), kind.into());
        lexemes.push(lexeme);
    }
    Lexicon::new(lexemes).unwrap()
}

/// `Kind` is lexical; `Form` and `Lone` are set by Productions. `Number` is
/// projected from inflection.
const GRAMMAR: &str = r#"(
    root: "Sentence",
    features: {
        "Number": ["Singular", "Plural"],
        "Kind": ["Common", "Rare"],
        "Form": ["Plain", "Marked"],
        "Lone": ["Only"],
    },
    categories: {
        "Sentence": (),
        "NounPhrase": (features: ["Number", "Kind", "Form", "Lone"]),
    },
    closed_words: { "The": "det:the", "A": "det:a" },
    productions: [
        (name: "Statement", lhs: "Sentence", rhs: [Cat("subject", "NounPhrase"), Punct('.')]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Lex("head", Noun)], equations: [Require("$.Form", "Plain")]),
        (name: "Definite", lhs: "NounPhrase", rhs: [Word("det", "The"), Lex("head", Noun)], equations: [Require("$.Form", "Plain")]),
        (name: "Indefinite", lhs: "NounPhrase", rhs: [Word("det", "A"), Lex("head", Noun)], equations: [Require("$.Form", "Marked"), Require("$.Lone", "Only")]),
    ],
)"#;

fn warnings() -> Vec<String> {
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(GRAMMAR).unwrap(),
        &lexicon(),
        &FrameCatalog::default(),
    )
    .unwrap();
    grammar.report().warnings.clone()
}

#[test]
fn a_feature_with_one_value_is_reported() {
    let warnings = warnings();
    assert!(
        warnings.contains(&"feature `Lone` has 1 value(s); a feature needs two or more".to_owned()),
        "{warnings:#?}"
    );
    assert!(
        !warnings.iter().any(|w| w.starts_with("feature `Kind`")
            || w.starts_with("feature `Form`")
            || w.starts_with("feature `Number`")),
        "{warnings:#?}"
    );
}

#[test]
fn a_value_with_one_holder_is_reported_with_its_holder() {
    let warnings = warnings();
    for expected in [
        // One lexeme carries it.
        "feature value `Kind` `Rare` has 1 holder(s) (noun:relic); a value needs two or more",
        // No lexeme carries it, and one Production sets it.
        "feature value `Form` `Marked` has 1 holder(s) (Indefinite); a value needs two or more",
        "feature value `Lone` `Only` has 1 holder(s) (Indefinite); a value needs two or more",
    ] {
        assert!(
            warnings.contains(&expected.to_owned()),
            "{expected}\n{warnings:#?}"
        );
    }
    // Two lexemes, two Productions, and inflection hold the rest.
    for unreported in ["`Common`", "`Plain`", "`Singular`", "`Plural`"] {
        assert!(
            !warnings.iter().any(|w| w.contains(unreported)),
            "{unreported}\n{warnings:#?}"
        );
    }
}
