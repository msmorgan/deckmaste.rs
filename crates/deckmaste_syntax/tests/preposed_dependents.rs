//! The second of two dependents before their head violates an Attachment
//! Class (`docs/decisions/english-v4.md` §4.3): where the dependent before it
//! can hold it, that Reading ranks first, and the stacked Reading stays a
//! costlier alternative.

use std::collections::BTreeSet;

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
        path: "tests/preposed_dependents.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
        Lexeme::invariant("a:red", "red", Category::Adjective, core()),
        Lexeme::invariant("d:this", "this", Category::Determinative, core()),
    ])
    .unwrap()
}

/// A preposed group and a temporal set off by one comma; a noun's group can
/// take the temporal inside it (`Long`), an adjective's cannot.
fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    let text = r#"(
        root: "Sentence",
        categories: { "Sentence": (), "Group": (), "Temporal": (), "Clause": () },
        closed_words: { "This": "d:this" },
        productions: [
            (name: "Statement", lhs: "Sentence", rhs: [Cat("head", "Clause"), Punct('.')]),
            (name: "Plain", lhs: "Clause", rhs: [Lex("head", Noun)]),
            (name: "Noun", lhs: "Group", rhs: [Lex("head", Noun)]),
            (name: "Adjective", lhs: "Group", rhs: [Lex("head", Adjective)]),
            (name: "Long", lhs: "Group", rhs: [Lex("head", Noun), Cat("modifier", "Temporal")]),
            (name: "This", lhs: "Temporal", rhs: [Word("determiner", "This"), Lex("head", Noun)]),
            (name: "Preposed", lhs: "Clause", rhs: [Cat("adjunct", "Group"), Punct(','), Cat("head", "Clause")]),
            (name: "PreposedPair", lhs: "Clause", rhs: [Cat("adjunct", "Group"), Cat("adjunct", "Temporal"), Punct(','), Cat("head", "Clause")]),
        ],
    )"#;
    CompiledGrammar::bind(
        &Grammar::from_ron_str(text).unwrap(),
        lexicon,
        &FrameCatalog::default(),
    )
    .unwrap()
}

fn uses(reading: &Reading, production: &str) -> bool {
    reading.node().is_some_and(|node| {
        node.production.as_str() == production || node.children.iter().any(|c| uses(c, production))
    })
}

#[test]
fn a_dependent_the_one_before_it_can_hold_ranks_first_inside_it() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let parse = grammar.parse(&lexicon, "Card this card, card.");
    // Both analyses are Readings.
    let ranked = parse.best_ranked(usize::MAX);
    assert_eq!(ranked.len(), 2, "{ranked:#?}");
    // The noun's group holds the temporal, and ranks first.
    let (best, reading) = &ranked[0];
    assert!(uses(reading, "Long") && !uses(reading, "PreposedPair"));
    assert_eq!(best.attachment_class, 0);
    assert_eq!(parse.cheapest_tier().map(|(_, tied)| tied), Some(1));
    // The stacked temporal violates the Attachment Class, on the tree and in
    // the forest alike.
    let (stacked, reading) = &ranked[1];
    assert!(uses(reading, "PreposedPair"));
    assert_eq!(stacked.attachment_class, 1);
    assert_eq!(
        grammar.ranking_key(reading, &lexicon, &BTreeSet::new()),
        Ok(*stacked)
    );
    assert_eq!(parse.admit(reading, &lexicon), Ok(()));
}

#[test]
fn a_stacked_dependent_with_no_other_site_is_the_reading() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    // The adjective's group cannot hold the temporal: the pair is the only
    // analysis.
    let found = grammar.parse(&lexicon, "Red this card, card.").readings();
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(uses(&found[0], "PreposedPair"));
    assert_eq!(grammar.admit(&found[0], &lexicon), Ok(()));
}
