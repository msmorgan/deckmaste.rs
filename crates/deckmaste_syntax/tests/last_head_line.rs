//! `State::head_final` means "the last head-line child so far ends in its
//! head", so a `first`/coordinator/`last` Production that does not declare a
//! `coordinator` still lets the chart record whether its first conjunct ends
//! in its head. Its last conjunct, which ends in a dependent the first lacks,
//! then has the Attachment Class the tree computes: the forest's Ranking Key
//! equals the tree's (`docs/decisions/english-v4.md` §4.3).

use std::collections::BTreeSet;

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
        path: "tests/last_head_line.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
        Lexeme::invariant("d:this", "this", Category::Determinative, core()),
        Lexeme::invariant("c:or", "or", Category::Coordinator, core()),
    ])
    .unwrap()
}

/// `Alternative` names `first`, `coordinator` and `last` as an authored
/// Production does (*power 3 or less*), but declares no `coordinator`. A
/// `Long` group ends in a dependent; a `Plain` one ends in its head.
fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    let text = r#"(
        root: "Sentence",
        categories: { "Sentence": (), "Group": (), "Temporal": () },
        closed_words: { "This": "d:this", "Or": "c:or" },
        productions: [
            (name: "Statement", lhs: "Sentence", rhs: [Cat("head", "Group"), Punct('.')]),
            (name: "Plain", lhs: "Group", rhs: [Lex("head", Noun)]),
            (name: "Long", lhs: "Group", rhs: [Lex("head", Noun), Cat("modifier", "Temporal")]),
            (name: "This", lhs: "Temporal", rhs: [Word("determiner", "This"), Lex("head", Noun)]),
            (name: "Alternative", lhs: "Group", rhs: [Cat("first", "Group"), Word("coordinator", "Or"), Cat("last", "Group")]),
        ],
    )"#;
    CompiledGrammar::bind(
        &Grammar::from_ron_str(text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
    .unwrap()
}

#[test]
fn a_last_conjunct_ending_in_a_dependent_has_the_same_forest_and_tree_key() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let parse = grammar.parse(&lexicon, "Card or card this card.");
    let ranked = parse.best_ranked(usize::MAX);
    assert_eq!(ranked.len(), 1, "{ranked:#?}");
    let (key, reading) = &ranked[0];
    // The first conjunct ends in its head and the last in a dependent: the
    // dependent belongs to the whole coordination, so the Class is counted.
    assert_eq!(key.attachment_class, 1);
    assert_eq!(
        grammar.ranking_key(reading, &lexicon, &BTreeSet::new()),
        Ok(*key)
    );
}

#[test]
fn a_last_conjunct_ending_in_its_head_has_the_same_forest_and_tree_key() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let parse = grammar.parse(&lexicon, "Card this card or card.");
    let ranked = parse.best_ranked(usize::MAX);
    assert_eq!(ranked.len(), 1, "{ranked:#?}");
    let (key, reading) = &ranked[0];
    assert_eq!(key.attachment_class, 0);
    assert_eq!(
        grammar.ranking_key(reading, &lexicon, &BTreeSet::new()),
        Ok(*key)
    );
}
