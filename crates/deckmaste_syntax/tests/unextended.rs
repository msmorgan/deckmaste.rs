//! A Production's `unextended` child does not end in a Preposition Phrase that
//! the Production, or a sibling with the same children up to and including
//! it, selects as a `PP` Complement right after it (an existential's
//! Extension after its Displaced Subject, CGEL Ch. 16 §6.2.1, p. 1394). A
//! phrase the Complement cannot take stays a postmodifier.

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

fn grammar(
    lexicon: &Lexicon,
    extended: &str,
) -> Result<CompiledGrammar, deckmaste_syntax::LoadError> {
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
fn a_child_before_a_selected_complement_does_not_end_in_one() {
    let lexicon = lexicon();
    let closed = r#"(name: "Extended", lhs: "Predicate", rhs: [Cat("subject", "NounPhrase"), Selected("complement", "PrepositionPhrase", ["In"])], unextended: ["subject"]),"#;
    let open = grammar(&lexicon, EXTENDED).unwrap();
    let grammar = grammar(&lexicon, closed).unwrap();
    // Open, *card in card* can be the subject before the extension *in card*.
    assert_eq!(
        open.parse(&lexicon, "Card in card in card.")
            .readings()
            .len(),
        2
    );
    // Closed, the first *in* phrase begins the extension.
    let found = grammar.parse(&lexicon, "Card in card in card.").readings();
    assert_eq!(found.len(), 1, "{found:#?}");
    assert_eq!(grammar.admit(&found[0], &lexicon), Ok(()));
    let subject = found[0].node().unwrap().children[0]
        .node()
        .unwrap()
        .children[0]
        .node()
        .unwrap();
    assert_eq!(subject.production.as_str(), "Noun");
    // A phrase the extension cannot take still ends the subject.
    let found = grammar.parse(&lexicon, "Card of card in card.").readings();
    assert!(
        found.iter().any(|reading| {
            let predicate = reading.node().unwrap().children[0].node().unwrap();
            predicate.production.as_str() == "Extended"
                && predicate.children[0].node().unwrap().production.as_str() != "Noun"
        }),
        "{found:#?}"
    );
}

#[test]
fn an_unextended_child_needs_a_sibling_with_a_selected_complement() {
    assert!(grammar(&lexicon(), "").is_err());
}

#[test]
fn an_unextended_child_before_no_selected_complement_is_a_load_error() {
    let lexicon = lexicon();
    // A first child followed by a Constituent is not `unextended` unless a
    // selected `PP` Complement follows it in some sibling.
    let first = r#"(name: "Pair", lhs: "Sentence", rhs: [Cat("subject", "NounPhrase"), Cat("object", "NounPhrase"), Punct('.')], unextended: ["subject"]),"#;
    assert!(grammar(&lexicon, &format!("{EXTENDED}{first}")).is_err());
    let unknown = r#"(name: "Named", lhs: "Predicate", rhs: [Cat("subject", "NounPhrase"), Selected("complement", "PrepositionPhrase", ["In"])], unextended: ["nonesuch"]),"#;
    assert!(grammar(&lexicon, unknown).is_err());
}
