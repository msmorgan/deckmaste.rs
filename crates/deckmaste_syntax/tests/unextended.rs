//! A Production's `unextended` last child does not end in a Preposition Phrase
//! that a sibling Production, the same but for a trailing selected `PP`
//! Complement, would take as that Complement (an existential's locative
//! extension, CGEL Ch. 16 §6.2.1, p. 1394). A phrase the sibling cannot take
//! stays a postmodifier.

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
        path: "tests/unextended.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("p:of", "of", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    prepositions: {
        "p:in": [(complement: NP, hosts: Nominal)],
        "p:of": [(complement: NP, hosts: Nominal)],
    },
)"#;

fn grammar(lexicon: &Lexicon, extended: &str) -> Result<CompiledGrammar, deckmaste_syntax::LoadError> {
    let text = format!(
        r#"(
        root: "Sentence",
        categories: {{
            "Sentence": (),
            "Predicate": (),
            "NounPhrase": (),
            "PrepositionPhrase": (),
        }},
        closed_words: {{ "In": "p:in" }},
        slot_categories: {{ NP: "NounPhrase", PP: "PrepositionPhrase" }},
        productions: [
            (name: "Statement", lhs: "Sentence", rhs: [Cat("head", "Predicate"), Punct('.')]),
            (name: "Noun", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
            (name: "Bare", lhs: "Predicate", rhs: [Cat("subject", "NounPhrase")], unextended: ["subject"]),
            {extended}
        ],
        schemas: [
            Valence(head: Preposition, phrase: "PrepositionPhrase"),
            Adjunct(host: "NounPhrase", adjunct: "PrepositionPhrase", side: After, host_kind: Nominal, cost: 1),
        ],
    )"#
    );
    CompiledGrammar::bind(
        &Grammar::from_ron_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
}

const EXTENDED: &str = r#"(name: "Extended", lhs: "Predicate", rhs: [Cat("subject", "NounPhrase"), Selected("complement", "PrepositionPhrase", ["In"])]),"#;

#[test]
fn a_phrase_the_sibling_selects_is_its_complement_only() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, EXTENDED).unwrap();
    for text in ["Card in card.", "Card of card."] {
        let found = grammar.parse(&lexicon, text).readings();
        assert_eq!(found.len(), 1, "{text}: {found:#?}");
        assert_eq!(grammar.admit(&found[0], &lexicon), Ok(()), "{text}");
    }
    // *in card* is the extension, never the postmodifier of the bare subject.
    let found = grammar.parse(&lexicon, "Card in card.").readings();
    let predicate = found[0].node().unwrap().children[0].node().unwrap();
    assert_eq!(predicate.production.as_str(), "Extended");
    // *of card* is no extension: it postmodifies the bare subject.
    let found = grammar.parse(&lexicon, "Card of card.").readings();
    let predicate = found[0].node().unwrap().children[0].node().unwrap();
    assert_eq!(predicate.production.as_str(), "Bare");
}

#[test]
fn an_unextended_child_needs_a_sibling_with_a_selected_complement() {
    assert!(grammar(&lexicon(), "").is_err());
}
