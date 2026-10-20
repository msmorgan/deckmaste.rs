//! A Valence schema's `except` Closed Words head none of its phrases: the
//! grammar builds them only into Productions of their own, as the Targeting
//! Marker is a determiner of a targeted nominal and no general determinative.

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
        path: "tests/valence_exceptions.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::invariant("d:target", "target", Category::Determinative, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
    ])
    .unwrap()
}

fn grammar(
    lexicon: &Lexicon,
    except: &str,
) -> Result<CompiledGrammar, deckmaste_syntax::LoadError> {
    let text = format!(
        r#"(
        root: "Phrase",
        categories: {{
            "Phrase": (),
            "DeterminerPhrase": (),
            "Marker": (category: Some("DeterminerPhrase")),
        }},
        closed_words: {{ "Target": "d:target", "Or": "c:or" }},
        productions: [
            (name: "Determined", lhs: "Phrase", rhs: [Cat("determiner", "DeterminerPhrase"), Lex("head", Noun)]),
            (name: "Marker", lhs: "Marker", rhs: [Word("head", "Target")]),
            (name: "Marked", lhs: "Phrase", rhs: [Cat("determiner", "Marker"), Lex("head", Noun)]),
        ],
        schemas: [
            Valence(head: Determinative, phrase: "DeterminerPhrase", except: [{except}]),
        ],
    )"#
    );
    CompiledGrammar::bind(
        &Grammar::from_ron_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
}

fn productions(reading: &Reading) -> Vec<String> {
    match reading {
        Reading::Node(node) => std::iter::once(node.production.to_string())
            .chain(node.children.iter().flat_map(productions))
            .collect(),
        Reading::Leaf(_) => Vec::new(),
    }
}

#[test]
fn an_excepted_word_heads_only_its_own_production() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, r#""Target""#).unwrap();
    let the = grammar.parse(&lexicon, "the card").readings();
    assert_eq!(the.len(), 1, "{the:#?}");
    assert!(productions(&the[0]).contains(&"Determined".to_owned()));
    let target = grammar.parse(&lexicon, "target card").readings();
    assert_eq!(target.len(), 1, "{target:#?}");
    let used = productions(&target[0]);
    assert!(used.contains(&"Marked".to_owned()), "{used:?}");
    assert!(!used.contains(&"Determined".to_owned()), "{used:?}");
    for (text, reading) in [("the card", &the[0]), ("target card", &target[0])] {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
}

#[test]
fn without_the_exception_the_word_is_also_a_general_determinative() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, "").unwrap();
    assert_eq!(grammar.parse(&lexicon, "target card").readings().len(), 2);
}

#[test]
fn an_exception_must_be_a_closed_word_of_the_head_category() {
    let lexicon = lexicon();
    assert!(grammar(&lexicon, r#""Or""#).is_err());
    assert!(grammar(&lexicon, r#""Missing""#).is_err());
}
