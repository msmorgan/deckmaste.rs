//! Engine core: loading, feature agreement, packing, Admission and both
//! round-trip laws, over a small hand-written grammar.

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;
use deckmaste_syntax::LoadError;
use deckmaste_syntax::Reading;

fn source(kind: SourceKind) -> Source {
    Source {
        kind,
        path: "tests/engine.rs".to_owned(),
        owner: "test".to_owned(),
    }
}

fn lexicon() -> Lexicon {
    let core = || source(SourceKind::Core);
    let mut lexemes = vec![
        Lexeme::verb("v:see", "see", core()),
        Lexeme::verb("v:destroy", "destroy", source(SourceKind::Catalog)),
        Lexeme::invariant("d:the", "the", Category::Determinative, core()),
        Lexeme::invariant("d:this", "this", Category::Determinative, core()),
        Lexeme::invariant("d:these", "these", Category::Determinative, core()),
        Lexeme::invariant("p:on", "on", Category::Preposition, core()),
        Lexeme::invariant("p:with", "with", Category::Preposition, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
    ];
    for noun in ["man", "hill", "telescope", "card"] {
        lexemes.push(Lexeme::noun(
            format!("n:{noun}"),
            noun,
            vec![Countability::Count],
            core(),
        ));
    }
    Lexicon::new(lexemes).unwrap()
}

/// Imperatives with PP attachment to the verb phrase or to any nominal.
const GRAMMAR: &str = r#"(
    root: "Sentence",
    features: { "Number": ["Singular", "Plural"], "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "NounPhrase": (features: ["Number"]),
        "Nominal": (features: ["Number"]),
        "PrepositionPhrase": (),
    },
    closed_words: { "The": "d:the", "This": "d:this", "These": "d:these" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain")]),
        (name: "Transitive", lhs: "VerbPhrase", rhs: [Lex("head", Verb), Cat("object", "NounPhrase")]),
        (name: "PredicateAdjunct", lhs: "VerbPhrase", rhs: [Cat("head", "VerbPhrase"), Cat("adjunct", "PrepositionPhrase")], cost: 2),
        (name: "Determined", lhs: "NounPhrase", rhs: [Word("det", "The"), Cat("head", "Nominal")]),
        (name: "DeterminedSingular", lhs: "NounPhrase", rhs: [Word("det", "This"), Cat("head", "Nominal")],
            equations: [Require("head.Number", "Singular")]),
        (name: "DeterminedPlural", lhs: "NounPhrase", rhs: [Word("det", "These"), Cat("head", "Nominal")],
            equations: [Require("head.Number", "Plural")]),
        (name: "Noun", lhs: "Nominal", rhs: [Lex("head", Noun)]),
        (name: "Postmodifier", lhs: "Nominal", rhs: [Cat("head", "Nominal"), Cat("modifier", "PrepositionPhrase")]),
        (name: "PrepositionPhrase", lhs: "PrepositionPhrase", rhs: [Lex("head", Preposition), Cat("object", "NounPhrase")]),
    ],
)"#;

fn bind(text: &str, lexicon: &Lexicon) -> Result<CompiledGrammar, LoadError> {
    CompiledGrammar::bind(
        &Grammar::from_ron_str(text)?,
        lexicon,
        &FrameCatalog::default(),
    )
}

/// Parses and checks round-trip law 1 and Admission on every Reading.
fn readings(grammar: &CompiledGrammar, lexicon: &Lexicon, text: &str) -> Vec<Reading> {
    let parse = grammar.parse(lexicon, text);
    let readings = parse.readings();
    assert_eq!(
        u128::try_from(readings.len()).unwrap(),
        parse.derivation_count(),
        "every derivation of {text:?} is a distinct Reading"
    );
    for reading in &readings {
        assert_eq!(grammar.admit(reading, lexicon), Ok(()), "{reading:#?}");
        assert_eq!(grammar.realize(reading, lexicon).unwrap(), text);
    }
    readings
}

fn load_error(text: &str) -> String {
    match bind(text, &lexicon()) {
        Err(LoadError::Invalid(message)) => message,
        other => panic!("expected an invalid grammar, got {other:?}"),
    }
}

#[test]
fn pp_chain_packs_catalan_many_trees_in_a_small_forest() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let text = "See the man on the hill with the telescope in the card.";
    let parse = grammar.parse(&lexicon, text);
    // V NP PP PP PP: the fifth Catalan number of attachment trees, each one an
    // analysis this toy grammar endorses.
    assert_eq!(parse.derivation_count(), 14);
    assert_eq!(readings(&grammar, &lexicon, text).len(), 14);
    let metrics = parse.forest.metrics();
    println!("{text} {metrics:?}");
    assert!(metrics.completed_nodes < 40, "{metrics:?}");
}

/// A Production beginning with a word or a mark is predicted only where that
/// terminal can begin: declaring one that nothing here starts adds no chart
/// item and loses no Reading, and one the text does start still parses.
#[test]
fn terminal_initial_productions_are_predicted_only_where_they_scan() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let bracketed = bind(
        &GRAMMAR.replace(
            "    ],\n)",
            "        (name: \"Bracketed\", lhs: \"NounPhrase\", rhs: [Mark('(', Prefix), Cat(\"head\", \"NounPhrase\"), Punct(')')]),\n    ],\n)",
        ),
        &lexicon,
    )
    .unwrap();
    let text = "See the man on the hill with the telescope.";
    assert_eq!(
        readings(&bracketed, &lexicon, text),
        readings(&grammar, &lexicon, text)
    );
    assert_eq!(
        bracketed.parse(&lexicon, text).forest.metrics().chart_items,
        grammar.parse(&lexicon, text).forest.metrics().chart_items
    );
    assert_eq!(
        readings(&bracketed, &lexicon, "See (the man) on (the hill).").len(),
        1
    );
}

#[test]
fn cheaper_attachment_ranks_first() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let readings = readings(&grammar, &lexicon, "See the man with the telescope.");
    assert_eq!(readings.len(), 2);
    let first = readings[0].node().unwrap();
    let verb_phrase = first.children[0].node().unwrap();
    assert_eq!(
        verb_phrase.production, "Transitive",
        "nominal attachment first"
    );
    assert!(grammar.cost(&readings[0]) < grammar.cost(&readings[1]));
}

#[test]
fn feature_requirements_reject_disagreement() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    assert_eq!(readings(&grammar, &lexicon, "See this card.").len(), 1);
    assert_eq!(readings(&grammar, &lexicon, "See these cards.").len(), 1);
    assert_eq!(
        readings(&grammar, &lexicon, "See this cards."),
        [] as [deckmaste_syntax::Reading; 0]
    );
    assert_eq!(
        readings(&grammar, &lexicon, "See these card."),
        [] as [deckmaste_syntax::Reading; 0]
    );
    // Only the plain form is imperative.
    assert_eq!(
        readings(&grammar, &lexicon, "Sees the card."),
        [] as [deckmaste_syntax::Reading; 0]
    );
}

#[test]
fn feature_exclusion_is_shared_by_parsing_and_admission() {
    let lexicon = lexicon();
    let source = GRAMMAR.replace(
        "Require(\"clause.VerbForm\", \"Plain\")",
        "Exclude(\"clause.VerbForm\", \"Finite\")",
    );
    let grammar = bind(&source, &lexicon).unwrap();
    assert_ne!(
        readings(&grammar, &lexicon, "See the card."),
        [] as [Reading; 0]
    );
    assert_ne!(
        readings(&grammar, &lexicon, "Seeing the card."),
        [] as [Reading; 0]
    );
    assert_eq!(
        readings(&grammar, &lexicon, "Sees the card."),
        [] as [Reading; 0]
    );
    let unrestricted = bind(
        &source.replace("Exclude(\"clause.VerbForm\", \"Finite\")", ""),
        &lexicon,
    )
    .unwrap();
    let finite = unrestricted
        .parse(&lexicon, "Sees the card.")
        .best_readings(1)
        .pop()
        .unwrap();
    assert!(grammar.admit(&finite, &lexicon).is_err());
}

#[test]
fn unknown_words_are_vocabulary_gaps() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let parse = grammar.parse(&lexicon, "See the frobnicator.");
    assert_eq!(parse.vocabulary_gaps, vec![8..19]);
    assert_eq!(parse.readings(), [] as [deckmaste_syntax::Reading; 0]);
}

#[test]
fn punctuation_decides_but_is_not_stored() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    assert_eq!(
        readings(&grammar, &lexicon, "See the card"),
        [] as [deckmaste_syntax::Reading; 0]
    );
    let reading = &readings(&grammar, &lexicon, "See the card.")[0];
    let sentence = reading.node().unwrap();
    assert_eq!(sentence.children.len(), 1, "no punctuation child");
}

#[test]
fn hand_built_readings_reparse() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let text = "See the man on the hill with the telescope in the card.";
    for (index, built) in readings(&grammar, &lexicon, text)
        .into_iter()
        .enumerate()
        .step_by(4)
    {
        // Rebuild the Reading from its serialized form rather than from the
        // parse, then check law 2: it is admitted and is a Reading of the
        // parse of its realization.
        let rebuilt: Reading = ron::from_str(&ron::to_string(&built).unwrap()).unwrap();
        assert_eq!(grammar.admit(&rebuilt, &lexicon), Ok(()), "tree {index}");
        let surface = grammar.realize(&rebuilt, &lexicon).unwrap();
        assert!(readings(&grammar, &lexicon, &surface).contains(&rebuilt));
    }
}

#[test]
fn admission_rejects_broken_readings() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let reading = readings(&grammar, &lexicon, "See this card.").remove(0);
    let text = ron::to_string(&reading).unwrap();
    let plural: Reading = ron::from_str(
        &text
            .replace(
                "lexeme:\"n:card\",form:Singular",
                "lexeme:\"n:card\",form:Plural",
            )
            .replace("number:Some(Singular)", "number:Some(Plural)"),
    )
    .unwrap();
    assert_ne!(plural, reading, "the edit changed the noun");
    assert!(grammar.admit(&plural, &lexicon).is_err());
    let unknown: Reading =
        ron::from_str(&text.replace("\"Transitive\"", "\"Ditransitive\"")).unwrap();
    assert!(grammar.admit(&unknown, &lexicon).is_err());
}

#[test]
fn load_errors() {
    let swap = |from: &str, to: &str| {
        assert!(GRAMMAR.contains(from), "{from}");
        GRAMMAR.replacen(from, to, 1)
    };
    assert!(
        load_error(&swap(
            "Cat(\"object\", \"NounPhrase\")",
            "Cat(\"object\", \"Noun\")"
        ))
        .contains("unknown category")
    );
    assert!(
        load_error(&swap(
            "Require(\"clause.VerbForm\", \"Plain\")",
            "Require(\"clause.Voice\", \"Plain\")"
        ))
        .contains("unknown feature")
    );
    assert!(
        load_error(&swap(
            "Require(\"clause.VerbForm\", \"Plain\")",
            "Require(\"clause.VerbForm\", \"Imperative\")"
        ))
        .contains("unknown value")
    );
    assert!(
        load_error(&swap("Word(\"det\", \"This\")", "Word(\"det\", \"That\")"))
            .contains("unknown closed word")
    );
    assert!(
        load_error(&swap("\"This\": \"d:this\"", "\"This\": \"d:that\""))
            .contains("unknown lexeme")
    );
    assert!(load_error(&swap("rhs: [Lex(\"head\", Noun)]", "rhs: []")).contains("empty"));
    for suffix in ["Use", "Licence", "License", "Present", "Restriction"] {
        let policing = swap(
            "\"Number\": [",
            &format!("\"Attachment{suffix}\": [\"Yes\"], \"Number\": ["),
        );
        assert!(load_error(&policing).contains("policing"), "{suffix}");
    }
    // The CR-catalog ban: a keyword action is catalog vocabulary.
    assert!(
        load_error(&swap("\"This\": \"d:this\"", "\"This\": \"v:destroy\"")).contains("Catalog")
    );
    // An open-class core word is not a closed word either.
    assert!(
        load_error(&swap("\"This\": \"d:this\"", "\"This\": \"n:card\"")).contains("closed-class")
    );
    match FrameCatalog::from_ron_str(
        "(frames: { \"v:see\": [(family: Monotransitive, slots: [])] })",
    ) {
        Err(LoadError::Invalid(message)) => assert!(message.contains("shape")),
        other => panic!("{other:?}"),
    }
}

#[test]
fn best_readings_are_the_cheapest_prefix() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let parse = grammar.parse(
        &lexicon,
        "See the man on the hill with the telescope in the card.",
    );
    let all = parse.readings();
    for k in [1, 3, 14, 20] {
        assert_eq!(parse.best_readings(k), all[..k.min(all.len())]);
    }
}

#[test]
fn best_readings_do_not_enumerate_every_attachment() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let mut text = "See the man".to_owned();
    for _ in 0..5 {
        text.push_str(" on the hill with the telescope");
    }
    text.push('.');
    let parse = grammar.parse(&lexicon, &text);
    // V NP and ten PPs: the eleventh Catalan number of trees.
    assert_eq!(parse.derivation_count(), 58_786);
    let best = parse.best_readings(2);
    assert_eq!(best.len(), 2);
    for reading in &best {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()));
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    println!("{:?}", parse.forest.metrics());
}

#[test]
fn both_laws_hold_over_the_forest_without_enumeration() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let text = "See the man on the hill with the telescope in the card.";
    let parse = grammar.parse(&lexicon, text);
    assert_eq!(parse.verify_realization(&lexicon), Ok(()));
    let readings = parse.readings();
    for reading in &readings {
        let again = grammar.parse(&lexicon, &grammar.realize(reading, &lexicon).unwrap());
        assert!(again.contains(reading), "{reading:#?}");
    }
    // A tree with one word swapped is not in the forest.
    let swapped: Reading = ron::from_str(
        &ron::to_string(&readings[0])
            .unwrap()
            .replace("\"n:hill\"", "\"n:card\""),
    )
    .unwrap();
    assert_ne!(swapped, readings[0]);
    assert!(!parse.contains(&swapped));
}

#[test]
fn law_one_faults_are_found_in_the_forest() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    // Construction boundaries reject spacing that realization cannot produce.
    for text in ["See the man  on the hill.", "See the man on the hill ."] {
        let parse = grammar.parse(&lexicon, text);
        assert_eq!(parse.derivation_count(), 0, "{text}");
    }
    // Leading text is still diagnosed by the forest's independent edge check.
    let parse = grammar.parse(&lexicon, " See the man.");
    assert!(parse.derivation_count() > 0);
    assert_eq!(
        parse.verify_realization(&lexicon),
        Err(deckmaste_syntax::RealizationFault::Edge)
    );
}

#[test]
fn constituents_carry_span_function_and_head() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let text = "See the man with the telescope.";
    let mut hosts = Vec::new();
    for reading in readings(&grammar, &lexicon, text) {
        let (realized, constituents) = grammar.constituents(&reading, &lexicon).unwrap();
        assert_eq!(realized, text);
        let pp = constituents
            .iter()
            .find(|c| c.category == "PrepositionPhrase")
            .unwrap();
        assert_eq!(&text[pp.span.clone()], "with the telescope");
        assert_eq!(
            pp.head.map(|h| h.as_str().to_owned()).as_deref(),
            Some("p:with")
        );
        let host = &constituents[pp.parent.unwrap()];
        hosts.push((
            pp.function.clone(),
            host.category.clone(),
            host.head.as_ref().map(|h| h.as_str().to_owned()),
            text[host.span.clone()].to_owned(),
        ));
    }
    hosts.sort();
    assert_eq!(
        hosts,
        [
            (
                "adjunct".to_owned(),
                "VerbPhrase".to_owned(),
                Some("v:see".to_owned()),
                "See the man with the telescope".to_owned()
            ),
            (
                "modifier".to_owned(),
                "Nominal".to_owned(),
                Some("n:man".to_owned()),
                "man with the telescope".to_owned()
            ),
        ]
    );
}

#[test]
fn a_phrase_parses_and_admits_as_its_own_category() {
    let lexicon = lexicon();
    let grammar = bind(GRAMMAR, &lexicon).unwrap();
    let analyzed = lexicon.analyze("the man on the hill");
    let parse = grammar.parse_category(&analyzed, "NounPhrase").unwrap();
    let readings = parse.readings();
    assert_eq!(readings.len(), 1);
    assert_eq!(parse.admit(&readings[0], &lexicon), Ok(()));
    assert_eq!(
        grammar.admit_category(&readings[0], &lexicon, "NounPhrase"),
        Ok(())
    );
    assert!(grammar.admit(&readings[0], &lexicon).is_err());
    assert_eq!(parse.verify_realization(&lexicon), Ok(()));
    assert!(grammar.parse_category(&analyzed, "Nonesuch").is_none());
}
