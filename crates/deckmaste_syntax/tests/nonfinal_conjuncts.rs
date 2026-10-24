//! An Adjunct schema declared `nonfinal` builds a phrase that is only a
//! conjunct of a coordination other than the last: a comma-set Supplement to
//! a Verb Phrase conjunct before the next (*searches …, where X is …, puts
//! them …*). After the last conjunct the Supplement is the Clause's, so each
//! string keeps one Supplement site (CGEL, Ch. 15 §5.1, p. 1351).

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
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
        path: "tests/nonfinal_conjuncts.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("pro:it", "it", Category::Pronoun, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
        Lexeme::invariant("p:unlike", "unlike", Category::Preposition, core()),
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        Lexeme::verb("v:draw", "draw", core()),
        Lexeme::verb("v:discard", "discard", core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    prepositions: {
        "p:unlike": [(complement: NP, hosts: Supplement)],
    },
)"#;

const GRAMMAR: &str = r#"(
    root: "Sentence",
    categories: {
        "Sentence": (),
        "Clause": (),
        "VerbPhrase": (),
        "NounPhrase": (),
        "PrepositionPhrase": (),
    },
    closed_words: { "And": "c:and" },
    slot_categories: { NP: "NounPhrase" },
    productions: [
        (name: "Declarative", lhs: "Sentence", rhs: [Cat("clause", "Clause"), Punct('.')]),
        (name: "SubjectPredicate", lhs: "Clause", rhs: [Lex("subject", Pronoun), Cat("head", "VerbPhrase")]),
        (name: "Verb", lhs: "VerbPhrase", rhs: [Lex("head", Verb)]),
        (name: "Noun", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
    ],
    schemas: [
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
        Coordination(category: "VerbPhrase", coordinators: "And", delimiter: Some(',')),
        Adjunct(host: "Clause", adjunct: "PrepositionPhrase", side: After, host_kind: Supplement, delimiter: Some(',')),
        Adjunct(host: "VerbPhrase", adjunct: "PrepositionPhrase", side: After, host_kind: Supplement, delimiter: Some(','), nonfinal: true),
    ],
)"#;

const VERB_PHRASE_SUPPLEMENT: &str = "Adjunct:VerbPhrase+PrepositionPhrase/After/Supplement/,/nonfinal";
const CLAUSE_SUPPLEMENT: &str = "Adjunct:Clause+PrepositionPhrase/After/Supplement/,";

fn uses(reading: &Reading, production: &str) -> bool {
    reading.node().is_some_and(|node| {
        node.production.as_str() == production || node.children.iter().any(|c| uses(c, production))
    })
}

/// The one Reading of `text`.
fn reading(text: &str) -> Reading {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let parse = grammar.parse(&lexicon, text);
    assert_eq!(parse.verify_realization(&lexicon), Ok(()), "{text}");
    let mut readings = parse.readings();
    assert_eq!(readings.len(), 1, "{text}: {readings:#?}");
    let reading = readings.remove(0);
    assert_eq!(grammar.admit(&reading, &lexicon), Ok(()), "{text}");
    assert_eq!(grammar.realize(&reading, &lexicon).unwrap(), text);
    reading
}

#[test]
fn a_nonfinal_conjunct_takes_the_supplement() {
    for text in [
        "it draws, unlike Zombie, and discards.",
        "it draws, unlike Zombie, discards, and draws.",
        "it draws, discards, unlike Zombie, and draws.",
    ] {
        let reading = reading(text);
        assert!(uses(&reading, VERB_PHRASE_SUPPLEMENT), "{text}: {reading:#?}");
        assert!(!uses(&reading, CLAUSE_SUPPLEMENT), "{text}: {reading:#?}");
    }
}

#[test]
fn after_the_last_conjunct_the_supplement_is_the_clauses() {
    for text in [
        "it draws, unlike Zombie.",
        "it draws, and discards, unlike Zombie.",
        "it draws, discards, and draws, unlike Zombie.",
    ] {
        let reading = reading(text);
        assert!(uses(&reading, CLAUSE_SUPPLEMENT), "{text}: {reading:#?}");
        assert!(!uses(&reading, VERB_PHRASE_SUPPLEMENT), "{text}: {reading:#?}");
    }
}
