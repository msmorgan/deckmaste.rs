//! A frame restricted to active voice derives no passive of any kind: *cost*
//! and *have* are transitive but do not passivize (CGEL, Ch. 16 §10.1.2,
//! p. 1432, [17]). The same frame without the restriction keeps its verbal,
//! prepositional and adjectival passives, and its participle premodifies.

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
    let source = |kind| Source {
        kind,
        path: "tests/active_only_frames.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let core = || source(SourceKind::Core);
    let mut cost = Lexeme::verb("v:cost", "cost", core());
    for slot in &mut cost.forms {
        if matches!(
            slot.form,
            deckmaste_lexical::WordForm::Preterite | deckmaste_lexical::WordForm::PastParticiple
        ) {
            slot.surfaces = Some(vec!["cost".to_owned()]);
        }
    }
    Lexicon::new([
        Lexeme::verb("v:destroy", "destroy", core()),
        cost,
        Lexeme::verb("v:look", "look", core()),
        Lexeme::verb("v:gaze", "gaze", core()),
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        Lexeme::invariant("p:by", "by", Category::Preposition, core()),
        Lexeme::invariant("p:at", "at", Category::Preposition, core()),
    ])
    .unwrap()
}

/// *destroy* and *look at* passivize; *cost* and *gaze at* are declared
/// active-only.
const FRAMES: &str = r#"(
    frames: {
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
        "v:cost": [(family: Monotransitive, slots: [(relation: Object, fill: NP)], active_only: true)],
        "v:look": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:at"]))])],
        "v:gaze": [(family: Prepositional, slots: [(relation: Complement, fill: PP(["p:at"]))], active_only: true)],
    },
    prepositions: {
        "p:by": [(complement: NP, hosts: ComplementOnly)],
        "p:at": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

const GRAMMAR: &str = r#"(
    root: "Sentence",
    features: {
        "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Voice": ["Active", "Passive"],
        "Inflection": ["Invariant", "IndependentGenitive", "Singular", "Plural", "Plain", "Present", "Preterite", "Irrealis", "GerundParticiple", "PastParticiple"],
    },
    categories: {
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm", "Voice"], gap: true),
        "AdjectivePhrase": (features: ["Voice"]),
        "NounPhrase": (),
        "Nominal": (),
        "Participle": (),
        "PrepositionPhrase": (gap: true),
    },
    closed_words: { "By": "p:by" },
    slot_categories: {
        NP: "NounPhrase",
        PP: "PrepositionPhrase",
    },
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain"), Require("clause.Voice", "Active")]),
        (name: "Passive", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('!')],
            equations: [Require("clause.Voice", "Passive")]),
        (name: "Adjectival", lhs: "Sentence", rhs: [Cat("head", "AdjectivePhrase"), Punct('?')]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Cat("head", "Nominal")]),
        (name: "Fragment", lhs: "Sentence", rhs: [Cat("head", "NounPhrase"), Punct(';')]),
    ],
    schemas: [
        Premodification(head: Verb, modifier: "Participle", host: "Nominal", forms: [PastParticiple]),
        Valence(head: Verb, phrase: "VerbPhrase", active: [Require("$.Voice", "Active")], passive: Some("By"),
            verbal_passive: [Require("$.Voice", "Passive"), Require("$.VerbForm", "PastParticiple")],
            prepositional_passive: [Require("$.Voice", "Passive"), Require("$.VerbForm", "PastParticiple")],
            adjectival: Some("AdjectivePhrase"),
            adjectival_passive: [Require("head.VerbForm", "PastParticiple"), Require("$.Voice", "Passive")]),
        Valence(head: Preposition, phrase: "PrepositionPhrase"),
        Valence(head: Noun, phrase: "Nominal"),
    ],
)"#;

fn count(text: &str) -> usize {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(GRAMMAR).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    grammar.parse(&lexicon, text).readings().len()
}

#[test]
fn an_active_only_frame_keeps_its_active() {
    assert_eq!(count("Destroy Zombies."), 1);
    assert_eq!(count("Cost Zombies."), 1);
    assert_eq!(count("Look at Zombies."), 1);
    assert_eq!(count("Gaze at Zombies."), 1);
}

#[test]
fn an_active_only_frame_has_no_verbal_passive() {
    assert_eq!(count("Destroyed!"), 1);
    assert_eq!(count("Destroyed by Zombies!"), 1);
    assert_eq!(count("Cost!"), 0);
    assert_eq!(count("Cost by Zombies!"), 0);
}

#[test]
fn an_active_only_frame_has_no_prepositional_passive() {
    assert_eq!(count("Looked at!"), 1);
    assert_eq!(count("Gazed at!"), 0);
}

#[test]
fn an_active_only_frame_has_no_adjectival_passive() {
    assert_eq!(count("Destroyed?"), 1);
    assert_eq!(count("Cost?"), 0);
}

#[test]
fn an_active_only_verb_has_no_premodifying_participle() {
    assert_eq!(count("Destroyed Zombies;"), 1);
    assert_eq!(count("Cost Zombies;"), 0);
}
