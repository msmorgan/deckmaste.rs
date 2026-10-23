//! An `elliptical` Production ends in an ellipsis site (a stranded auxiliary,
//! CGEL Ch. 17 §7.1, p. 1519). Before any punctuation, a Constituent that ends
//! in one is followed only by a coordinator or by the head it is a dependent
//! of: a dependent right after the site would be the omitted Complement. Nor
//! does a head spelled like a verb in the omitted Complement's form, which
//! would be that Complement.

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
        path: "tests/elliptical.rs".to_owned(),
        owner: "test".to_owned(),
    };
    // *block*, plain and present alike.
    let block = Lexeme::verb("v:block", "block", core());
    Lexicon::new([
        block,
        Lexeme::invariant("n:card", "card", Category::Noun, core()),
        Lexeme::invariant("w:can", "can", Category::Adverb, core()),
        Lexeme::invariant("w:draws", "draws", Category::Adverb, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    prepositions: {
        "p:in": [(complement: NP, hosts: NominalAndPredicate)],
    },
)"#;

/// The grammar, its stranded *can* declared `elliptical` with these word
/// forms for its omitted Complement (RON: `None` or `Some([...])`).
fn grammar(lexicon: &Lexicon, elliptical: &str) -> CompiledGrammar {
    let text = format!(
        r#"(
        root: "Sentence",
        categories: {{
            "Sentence": (),
            "Clause": (),
            "Predicate": (),
            "NounPhrase": (),
            "PrepositionPhrase": (),
        }},
        closed_words: {{ "Can": "w:can", "Draws": "w:draws", "Block": "v:block" }},
        slot_categories: {{ NP: "NounPhrase", PP: "PrepositionPhrase" }},
        productions: [
            (name: "Statement", lhs: "Sentence", rhs: [Cat("clause", "Clause"), Punct('.')]),
            (name: "SubjectPredicate", lhs: "Clause", rhs: [Cat("subject", "NounPhrase"), Cat("head", "Predicate")]),
            (name: "Noun", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
            // A reduced relative: *card [that] can*.
            (name: "Postmodified", lhs: "NounPhrase", rhs: [Cat("head", "NounPhrase"), Cat("modifier", "Predicate")]),
            (name: "Stranded", lhs: "Predicate", rhs: [Word("head", "Can")], elliptical: {elliptical}),
            (name: "Lexical", lhs: "Predicate", rhs: [Word("head", "Draws")]),
            (name: "Verbal", lhs: "Predicate", rhs: [Word("head", "Block")]),
        ],
        schemas: [
            Valence(head: Preposition, phrase: "PrepositionPhrase"),
            Adjunct(host: "Clause", adjunct: "PrepositionPhrase", side: After, host_kind: Predicate, cost: 1),
            Adjunct(host: "Clause", adjunct: "PrepositionPhrase", side: After, host_kind: Predicate, delimiter: Some(','), cost: 1),
        ],
    )"#
    );
    CompiledGrammar::bind(
        &Grammar::from_ron_str(&text).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
}

fn count(grammar: &CompiledGrammar, lexicon: &Lexicon, text: &str) -> usize {
    let found = grammar.parse(lexicon, text).readings();
    for reading in &found {
        assert_eq!(grammar.admit(reading, lexicon), Ok(()), "{text}");
    }
    found.len()
}

#[test]
fn an_ellipsis_site_ends_its_phrase() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, "Some([Plain])");
    assert_eq!(count(&grammar, &lexicon, "Card can."), 1);
    // No dependent comes right after the site, at any height.
    assert_eq!(count(&grammar, &lexicon, "Card can in card."), 0);
    // An ordinary predicate still takes the dependent.
    assert_eq!(count(&grammar, &lexicon, "Card draws in card."), 1);
    // Without the declaration, the dependent attaches above the site.
    let open = self::grammar(&lexicon, "None");
    assert_eq!(count(&open, &lexicon, "Card can in card."), 1);
}

#[test]
fn punctuation_ends_the_reach_of_an_ellipsis_site() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, "Some([Plain])");
    assert_eq!(count(&grammar, &lexicon, "Card can, in card."), 1);
}

#[test]
fn the_head_a_phrase_ending_in_an_ellipsis_site_depends_on_follows_it() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, "Some([Plain])");
    // *card [that] can* is the Subject of *draws*.
    let found = grammar.parse(&lexicon, "Card can draws.").readings();
    assert_eq!(found.len(), 1, "{found:#?}");
    let clause = found[0].node().unwrap().children[0].node().unwrap();
    assert_eq!(clause.production.as_str(), "SubjectPredicate");
    let subject = clause.children[0].node().unwrap();
    assert_eq!(subject.production.as_str(), "Postmodified");
    // The Subject so ended takes no dependent of its own after the site.
    assert_eq!(count(&grammar, &lexicon, "Card can in card draws."), 0);
}

#[test]
fn a_head_that_could_be_the_omitted_complement_never_follows_the_site() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon, "Some([Plain])");
    // Plain *block* would be *can*'s Complement: no ellipsis is there.
    assert_eq!(count(&grammar, &lexicon, "Card can block."), 0);
    // A site whose Complement is a past participle keeps every Reading the
    // undeclared site has (one per paradigm cell spelled *block*).
    let participial = self::grammar(&lexicon, "Some([PastParticiple])");
    let open = self::grammar(&lexicon, "None");
    let all = count(&open, &lexicon, "Card can block.");
    assert!(all > 0);
    assert_eq!(count(&participial, &lexicon, "Card can block."), all);
    // Nor does *draws*, which is no plain form.
    assert_eq!(count(&grammar, &lexicon, "Card can draws."), 1);
}

#[test]
fn an_ellipsis_site_names_its_omitted_complement_forms() {
    let lexicon = lexicon();
    let text = r#"(
        root: "Predicate",
        categories: { "Predicate": () },
        closed_words: { "Can": "w:can" },
        productions: [
            (name: "Stranded", lhs: "Predicate", rhs: [Word("head", "Can")], elliptical: Some([])),
        ],
    )"#;
    let error = CompiledGrammar::bind(
        &Grammar::from_ron_str(text).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("word forms"), "{error}");
}
