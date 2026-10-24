//! Clusters of Adjuncts alone: a head with no Complement heads a
//! coordination of parallel sequences of two Preposition Phrases, a later
//! one beginning with a remnant, its preposition elided (CGEL, Ch. 15 §4.3,
//! p. 1343, note 65: *We'll be in Paris for a week and Bonn for three days*).

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
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
        path: "tests/adjunct_clusters.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:go", "go", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:player", "player", vec![Countability::Count], core()),
        Lexeme::invariant("p:with", "with", Category::Preposition, core()),
        Lexeme::invariant("p:for", "for", Category::Preposition, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:go": [(family: Intransitive, slots: [])],
    },
    prepositions: {
        "p:with": [(complement: NP, hosts: Predicate)],
        "p:for": [(complement: NP, hosts: Predicate)],
        "p:in": [(complement: NP, hosts: Predicate)],
    },
)"#;

fn count(text: &str) -> usize {
    let source = r#"(
    root: "Sentence",
    features: { "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "NounPhrase": (),
        "PrepositionPhrase": (),
        "PrepositionRemnant": (),
        "Adjunct": (),
        "Cluster": (),
        "Clusters": (category: Some("Cluster")),
        "AdjunctCluster": (),
        "AdjunctClusters": (category: Some("AdjunctCluster")),
    },
    lexical_classes: { "Coordinator": ["c:and"] },
    slot_categories: { NP: "NounPhrase" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("head", "VerbPhrase"), Punct('.')], equations: [Require("head.VerbForm", "Plain")]),
        (name: "Noun", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
        (name: "PredicateAdjunct", lhs: "Adjunct", rhs: [Hosted("filler", "PrepositionPhrase", Predicate)]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase",
            clusters: Some((cluster: "Cluster", coordination: "Clusters", adjunct: Some("Adjunct"),
                adjunct_clusters: Some((cluster: "AdjunctCluster", coordination: "AdjunctClusters", preposition: "PrepositionPhrase", remnant: "PrepositionRemnant"))))),
        Valence(head: Preposition, phrase: "PrepositionPhrase", remnants: [(NP, "PrepositionRemnant")]),
        Adjunct(host: "VerbPhrase", adjunct: "PrepositionPhrase", side: After, host_kind: Predicate, cost: 1),
        Coordination(category: "AdjunctClusters", conjunct: Some("AdjunctCluster"), coordinators: "Coordinator", parallel: true),
    ],
)"#;
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(source).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    let found = grammar.parse(&lexicon, text).readings();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn a_later_cluster_may_elide_the_initial_clusters_preposition() {
    // *enters with a counter for each red creature and a counter for each
    // green creature*.
    assert_eq!(count("Go with card for player and card for player."), 1);
    assert_eq!(
        count("Go with card for player and with card for player."),
        1
    );
}

#[test]
fn adjunct_clusters_are_parallel_in_their_prepositions() {
    // The elided preposition is the initial Cluster's first one, and each
    // Cluster ends in a Preposition Phrase of the initial Cluster's last.
    assert_eq!(count("Go with card for player and in card for player."), 0);
    assert_eq!(count("Go with card for player and card in player."), 0);
}

#[test]
fn a_remnant_never_begins_the_coordination() {
    assert_eq!(count("Go card for player and with card for player."), 0);
}

#[test]
fn a_single_adjunct_is_no_cluster() {
    assert_eq!(count("Go with card and card."), 0);
}
