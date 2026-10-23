//! A `Quoted` child is set in the quotation marks of its nesting depth,
//! double then single (`docs/oracle-style-guide.md` §3, Quotation marks and
//! apostrophes). The period or comma after a closing mark is written inside
//! it, and the quotation's own final period is not written at all (§3,
//! Periods): the punctuation is derived on realization, never stored, so
//! both round-trip laws hold.

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
        path: "tests/quotations.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("pro:it", "it", Category::Pronoun, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::verb("v:gain", "gain", core()),
        Lexeme::verb("v:draw", "draw", core()),
    ])
    .unwrap()
}

fn grammar(lexicon: &Lexicon, quotation: &str) -> Result<CompiledGrammar, String> {
    let text = format!(
        r#"(
        root: "Sentence",
        categories: {{ "Sentence": (), "Clause": (), "Quotation": () }},
        closed_words: {{ "And": "c:and" }},
        productions: [
            (name: "Declarative", lhs: "Sentence", rhs: [Cat("clause", "Clause"), Punct('.')]),
            (name: "Intransitive", lhs: "Clause", rhs: [Lex("subject", Pronoun), Lex("head", Verb)]),
            (name: "Granting", lhs: "Clause", rhs: [Lex("subject", Pronoun), Lex("head", Verb), Cat("object", "Quotation")]),
            (name: "Quotation", lhs: "Quotation", rhs: [{quotation}]),
        ],
        schemas: [Coordination(category: "Quotation", coordinators: "And")],
    )"#
    );
    let grammar = Grammar::from_ron_str(&text).map_err(|e| format!("{e:?}"))?;
    CompiledGrammar::bind(&grammar, lexicon, &FrameCatalog::default()).map_err(|e| format!("{e:?}"))
}

#[test]
fn quotations_round_trip_with_derived_punctuation() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, r#"Quoted("head", "Sentence")"#).unwrap();
    for text in [
        r#"it gains "it draws.""#,
        r#"it gains "it gains 'it draws.'""#,
        r#"it gains "it gains 'it gains "it draws."'""#,
        r#"it gains "it draws" and "it draws.""#,
        r#"it gains "it draws," "it draws," and "it draws.""#,
        r#"it gains "it gains 'it draws' and 'it draws'" and "it draws.""#,
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
fn quotation_marks_and_their_punctuation_are_oracle_s() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, r#"Quoted("head", "Sentence")"#).unwrap();
    for text in [
        // The outermost marks are double, the next single.
        "it gains 'it draws.'",
        r#"it gains "it gains "it draws."""#,
        // A period or comma is written inside the closing mark, once.
        r#"it gains "it draws"."#,
        r#"it gains "it draws..""#,
        r#"it gains "it draws", "it draws", and "it draws"."#,
    ] {
        assert!(
            grammar.parse(&lexicon, text).readings().is_empty(),
            "{text}"
        );
    }
}

#[test]
fn quotation_marks_are_declared_only_by_quoting() {
    let lexicon = lexicon();
    for symbol in [r#"Punct('"')"#, r"Mark('\'', Bound)"] {
        assert!(grammar(&lexicon, symbol).is_err(), "{symbol}");
    }
}
