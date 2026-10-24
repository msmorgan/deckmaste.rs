//! Complement Clusters: a verb heads a coordination of parallel sequences of
//! its Frame Slots, each optionally ending in an Adjunct (right
//! nonce-constituent coordination, CGEL, Ch. 15 §4.3, pp. 1341–1343).

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
        path: "tests/complement_clusters.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:give", "give", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
        Lexeme::noun("n:player", "player", vec![Countability::Count], core()),
        Lexeme::invariant("p:to", "to", Category::Preposition, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:give": [
            (family: Monotransitive, slots: [(relation: Object, fill: NP)]),
            (family: TransitiveWithPP, slots: [
                (relation: Object, fill: NP),
                (relation: Complement, fill: PP(["p:to"])),
            ]),
        ],
    },
    prepositions: {
        "p:to": [(complement: NP, hosts: ComplementOnly)],
        "p:in": [(complement: NP, hosts: Predicate)],
    },
)"#;

fn grammar(lexicon: &Lexicon) -> CompiledGrammar {
    let source = r#"(
    root: "Sentence",
    features: { "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "NounPhrase": (),
        "PrepositionPhrase": (),
        "Adjunct": (),
        "Cluster": (),
        "Clusters": (category: Some("Cluster")),
    },
    lexical_classes: { "Coordinator": ["c:and"] },
    slot_categories: { NP: "NounPhrase", PP: "PrepositionPhrase" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("head", "VerbPhrase"), Punct('.')], equations: [Require("head.VerbForm", "Plain")]),
        (name: "Noun", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
        (name: "PredicateAdjunct", lhs: "Adjunct", rhs: [Hosted("filler", "PrepositionPhrase", Predicate)]),
    ],
    schemas: [
        Valence(head: Verb, phrase: "VerbPhrase",
            clusters: Some((cluster: "Cluster", coordination: "Clusters", adjunct: Some("Adjunct")))),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
        Adjunct(host: "VerbPhrase", adjunct: "PrepositionPhrase", side: After, host_kind: Predicate, cost: 1),
        Coordination(category: "Clusters", conjunct: Some("Cluster"), coordinators: "Coordinator"),
    ],
)"#;
    CompiledGrammar::bind(
        &Grammar::from_ron_str(source).unwrap(),
        lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
}

fn count(text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let found = grammar.parse(&lexicon, text).readings();
    for reading in &found {
        assert_eq!(grammar.admit(reading, &lexicon), Ok(()), "{text}");
        assert_eq!(grammar.realize(reading, &lexicon).unwrap(), text);
    }
    found.len()
}

#[test]
fn clusters_of_two_slots_coordinate() {
    assert_eq!(count("Give card to player and card to player."), 1);
}

#[test]
fn a_single_slot_clusters_only_with_its_adjunct() {
    assert_eq!(count("Give card in hand and card in hand."), 1);
    assert_eq!(count("Give card and card."), 0);
}

#[test]
fn any_number_of_clusters_coordinate() {
    assert_eq!(
        count("Give card in hand, card in hand, and card in hand."),
        1
    );
}

#[test]
fn clusters_are_parallel_in_shape() {
    // An Object and an Adjunct do not coordinate with an Object and a
    // selected Complement (CGEL, Ch. 15 §4.3, p. 1341, [17ii]).
    assert_eq!(count("Give card to player and card in hand."), 0);
    // A Cluster of more than one Slot may end in an Adjunct its parallel
    // Cluster lacks.
    assert_eq!(count("Give card to player in hand and card to player."), 1);
}

#[test]
fn each_dependent_in_a_cluster_bears_its_function_in_the_verbs_phrase() {
    let lexicon = lexicon();
    let grammar = grammar(&lexicon);
    let text = "Give card to player in hand and card to player.";
    let found = grammar.parse(&lexicon, text).readings();
    assert_eq!(found.len(), 1);
    let (realized, constituents) = grammar.constituents(&found[0], &lexicon).unwrap();
    let verb = constituents
        .iter()
        .position(|c| c.category.as_str() == "VerbPhrase")
        .unwrap();
    let in_hand = constituents
        .iter()
        .find(|c| c.category.as_str() == "Adjunct" && &realized[c.span.clone()] == "in hand")
        .unwrap();
    assert_eq!(in_hand.also, [(dpsi::Ident::new("adjunct"), verb)]);
}

fn remnant_lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/complement_clusters.rs".to_owned(),
        owner: "test".to_owned(),
    };
    Lexicon::new([
        Lexeme::verb("v:gain", "gain", core()),
        Lexeme::noun("n:card", "card", vec![Countability::Count], core()),
        Lexeme::noun("n:hand", "hand", vec![Countability::Count], core()),
        Lexeme::invariant("k:protection", "protection", Category::Keyword, core()),
        Lexeme::invariant("k:hexproof", "hexproof", Category::Keyword, core()),
        Lexeme::invariant("p:from", "from", Category::Preposition, core()),
        Lexeme::invariant("p:in", "in", Category::Preposition, core()),
        Lexeme::invariant("c:and", "and", Category::Coordinator, core()),
    ])
    .unwrap()
}

const REMNANT_FRAMES: &str = r#"(
    frames: {
        "v:gain": [(family: Monotransitive, slots: [(relation: Object, fill: KeywordPhrase)])],
    },
    prepositions: {
        "p:in": [(complement: NP, hosts: Predicate)],
    },
    parameters: {
        "k:protection": [[Head, Word("p:from"), Slot(NP)]],
        "k:hexproof": [[Head, Word("p:from"), Slot(NP)]],
    },
)"#;

fn remnant_count(text: &str) -> usize {
    let source = r#"(
    root: "Sentence",
    features: { "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"] },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm"]),
        "NounPhrase": (),
        "KeywordParameters": (),
        "KeywordPhrase": (),
        "ParameterRemnant": (),
        "PrepositionPhrase": (),
        "Adjunct": (),
        "Cluster": (),
        "Clusters": (category: Some("Cluster")),
    },
    lexical_classes: { "Coordinator": ["c:and"] },
    slot_categories: { NP: "NounPhrase", KeywordPhrase: "KeywordPhrase" },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("head", "VerbPhrase"), Punct('.')], equations: [Require("head.VerbForm", "Plain")]),
        (name: "Noun", lhs: "NounPhrase", rhs: [Lex("head", Noun)]),
        (name: "Keyword", lhs: "KeywordPhrase", rhs: [Cat("head", "KeywordParameters")]),
        (name: "PredicateAdjunct", lhs: "Adjunct", rhs: [Hosted("filler", "PrepositionPhrase", Predicate)]),
    ],
    schemas: [
        Parameters(head: Keyword, phrase: "KeywordParameters", remnant: Some("ParameterRemnant")),
        Valence(head: Verb, phrase: "VerbPhrase",
            clusters: Some((cluster: "Cluster", coordination: "Clusters", adjunct: Some("Adjunct"),
                remnants: [(KeywordPhrase, "ParameterRemnant")]))),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
        Adjunct(host: "VerbPhrase", adjunct: "PrepositionPhrase", side: After, host_kind: Predicate, cost: 1),
        Coordination(category: "Clusters", conjunct: Some("Cluster"), coordinators: "Coordinator"),
    ],
)"#;
    let lexicon = remnant_lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(source).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(REMNANT_FRAMES).unwrap(),
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
fn a_later_cluster_may_elide_the_head_of_the_first_clusters_object() {
    // *gain protection from white if ..., [from blue] if ...* (CGEL, Ch. 15
    // §4.3, p. 1343, note 65). The remnant elides *protection*, the initial
    // Cluster's Object head, and not *hexproof*, which takes the same
    // parameters: one Reading.
    assert_eq!(
        remnant_count("Gain protection from card in hand and from card in hand."),
        1
    );
    assert_eq!(
        remnant_count("Gain hexproof from card in hand, from card in hand, and from card in hand."),
        1
    );
}

#[test]
fn the_initial_cluster_is_no_remnant() {
    assert_eq!(
        remnant_count("Gain from card in hand and protection from card in hand."),
        0
    );
}
