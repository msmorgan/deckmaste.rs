//! The Segment walk over a Packed Forest: a parse's top-level stretches and
//! their Derivations, over small hand grammars.

use deckmaste_english_v4::segments::NotSegmentable;
use deckmaste_english_v4::segments::SegmentFrame;
use deckmaste_english_v4::segments::Segmentation;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::LoadError;

fn word(id: &str, lemma: &str, category: Category) -> Lexeme {
    Lexeme::invariant(
        id,
        lemma,
        category,
        Source {
            kind: SourceKind::Core,
            path: "tests/segments.rs".to_owned(),
            owner: "test".to_owned(),
        },
    )
}

/// `cast` is a noun and an adjective, so each `Word` over it has two
/// Derivations; `card` is a noun only.
fn lexicon() -> Lexicon {
    Lexicon::new([
        word("n:cast", "cast", Category::Noun),
        word("a:cast", "cast", Category::Adjective),
        word("n:card", "card", Category::Noun),
    ])
    .unwrap()
}

fn bind(grammar: &str) -> CompiledGrammar {
    CompiledGrammar::bind(
        &Grammar::from_ron_str(grammar).unwrap(),
        &lexicon(),
        &FrameCatalog::default(),
    )
    .unwrap()
}

const OUTER: &[&str] = &["Document", "Sentences"];
const EDGES: &[char] = &['.', ',', ':'];

/// A sentence sequence, and a sentence that holds a bracketed one of its own.
const SEQUENCE: &str = r#"(
    root: "Document",
    categories: { "Document": (), "Sentences": (), "Sentence": (), "Word": () },
    productions: [
        (name: "Text", lhs: "Document", rhs: [Cat("head", "Sentences")]),
        (name: "Single", lhs: "Sentences", rhs: [Cat("head", "Sentence")]),
        (name: "Sequence", lhs: "Sentences", rhs: [Cat("first", "Sentence"), Cat("rest", "Sentences")]),
        (name: "Plain", lhs: "Sentence", rhs: [Cat("word", "Word"), Punct('.')]),
        (name: "Nested", lhs: "Sentence", rhs: [Cat("word", "Word"), Mark('[', Prefix), Cat("inner", "Sentences"), Punct(']'), Punct('.')]),
        (name: "Noun", lhs: "Word", rhs: [Lex("word", Noun)]),
        (name: "Adjective", lhs: "Word", rhs: [Lex("word", Adjective)]),
    ],
)"#;

fn segments(grammar: &CompiledGrammar, text: &str) -> (u128, Segmentation) {
    let parse = grammar.parse(&lexicon(), text);
    let frame = SegmentFrame::resolve(grammar, OUTER, EDGES).unwrap();
    (
        parse.derivation_count(),
        deckmaste_english_v4::segments::segments(&parse, &frame),
    )
}

fn counts(segmentation: &Segmentation) -> Vec<(&str, u128)> {
    match segmentation {
        Segmentation::Segments(segments) => segments
            .iter()
            .map(|s| (s.text.as_str(), s.derivations))
            .collect(),
        Segmentation::NotSegmentable(why) => panic!("not segmentable: {why:?}"),
    }
}

#[test]
fn sentences_of_a_sequence_are_segments_whose_counts_multiply() {
    let grammar = bind(SEQUENCE);
    let (derivations, segmentation) = segments(&grammar, "Cast. Card. Cast.");
    assert_eq!(derivations, 4);
    assert_eq!(
        counts(&segmentation),
        [("Cast", 2), ("Card", 1), ("Cast", 2)]
    );
    assert_eq!(segmentation.surplus(derivations), 2);
    let (derivations, segmentation) = segments(&grammar, "Card.");
    assert_eq!((derivations, segmentation.surplus(derivations)), (1, 0));
}

#[test]
fn a_nested_sequence_multiplies_into_its_segment() {
    let grammar = bind(SEQUENCE);
    let (derivations, segmentation) = segments(&grammar, "Card [cast. cast.].");
    assert_eq!(derivations, 4);
    assert_eq!(counts(&segmentation), [("Card [cast. cast.]", 4)]);
    assert_eq!(segmentation.surplus(derivations), 3);
}

#[test]
fn a_parse_without_a_derivation_has_no_segments() {
    let grammar = bind(SEQUENCE);
    let (derivations, segmentation) = segments(&grammar, "Card card.");
    assert_eq!(derivations, 0);
    assert_eq!(segmentation, Segmentation::Segments(Vec::new()));
    assert_eq!(segmentation.surplus(derivations), 0);
}

/// Two outer analyses: two words as two segments, or as one non-outer pair.
#[test]
fn segment_boundaries_that_differ_between_derivations_are_reported() {
    let grammar = bind(
        r#"(
        root: "Document",
        categories: { "Document": (), "Sentences": (), "Pair": (), "Word": () },
        productions: [
            (name: "Words", lhs: "Document", rhs: [Cat("first", "Word"), Cat("second", "Word"), Punct('.')]),
            (name: "Paired", lhs: "Document", rhs: [Cat("head", "Pair"), Punct('.')]),
            (name: "Pair", lhs: "Pair", rhs: [Cat("first", "Word"), Cat("second", "Word")]),
            (name: "Noun", lhs: "Word", rhs: [Lex("word", Noun)]),
        ],
    )"#,
    );
    let (derivations, segmentation) = segments(&grammar, "Card card.");
    assert_eq!(derivations, 2);
    assert_eq!(
        segmentation,
        Segmentation::NotSegmentable(NotSegmentable::Segmentations)
    );
    assert_eq!(segmentation.surplus(derivations), 1);
}

/// One segmentation under two outer Productions: the segment's count is 1,
/// the unit's Derivations 2.
#[test]
fn several_outer_structures_over_one_segmentation_are_reported() {
    let grammar = bind(
        r#"(
        root: "Document",
        categories: { "Document": (), "Sentences": (), "Word": () },
        productions: [
            (name: "Direct", lhs: "Document", rhs: [Cat("head", "Word"), Punct('.')]),
            (name: "Listed", lhs: "Document", rhs: [Cat("head", "Sentences")]),
            (name: "Single", lhs: "Sentences", rhs: [Cat("head", "Word"), Punct('.')]),
            (name: "Noun", lhs: "Word", rhs: [Lex("word", Noun)]),
        ],
    )"#,
    );
    let (derivations, segmentation) = segments(&grammar, "Card.");
    assert_eq!(derivations, 2);
    assert_eq!(
        segmentation,
        Segmentation::NotSegmentable(NotSegmentable::NonProduct)
    );
}

#[test]
fn an_outer_category_must_be_declared() {
    let grammar = bind(SEQUENCE);
    assert!(matches!(
        SegmentFrame::resolve(&grammar, &["Document", "Paragraphs"], EDGES),
        Err(LoadError::Invalid(message)) if message.contains("Paragraphs")
    ));
}
