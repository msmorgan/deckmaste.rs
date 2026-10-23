//! What a derived phrase carries beyond its frame comes from the grammar's
//! declarations, never from feature or value names in the engine
//! (`docs/decisions/english-v4.md` §4.4, principle 6): a Valence schema's voice and
//! passive equations and its `complement_kind`, a Premodification's per-form
//! equations, the grammar's `slot_equations` and `high_attachment`. A
//! derivation whose declaration is missing is a load error, never a silent
//! default.

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

fn lexicon() -> Lexicon {
    let core = || Source {
        kind: SourceKind::Core,
        path: "tests/declared_derivations.rs".to_owned(),
        owner: "test".to_owned(),
    };
    let mut he = Lexeme::noun("n:he", "he", vec![Countability::Count], core());
    he.properties
        .features
        .insert("Case".into(), "Nominative".into());
    Lexicon::new([
        Lexeme::verb("v:destroy", "destroy", core()),
        Lexeme::noun("n:Zombie", "Zombie", vec![Countability::Count], core()),
        he,
        Lexeme::invariant("p:by", "by", Category::Preposition, core()),
    ])
    .unwrap()
}

const FRAMES: &str = r#"(
    frames: {
        "v:destroy": [(family: Monotransitive, slots: [(relation: Object, fill: NP)])],
    },
    prepositions: {
        "p:by": [(complement: NP, hosts: ComplementOnly)],
    },
)"#;

/// The verb Valence schema's declarations, the grammar-level ones and a
/// Premodification's are spliced in.
fn grammar(valence: &str, grammar_level: &str, premodification: &str) -> String {
    format!(
        r#"(
    root: "Sentence",
    features: {{
        "VerbForm": ["Plain", "Finite", "GerundParticiple", "PastParticiple"],
        "Voice": ["Active", "Passive"],
        "Case": ["Nominative", "Accusative", "Genitive"],
        "ComplementKind": ["NP", "Intransitive"],
        "Inflection": ["Invariant", "IndependentGenitive", "Singular", "Plural", "Plain", "Present", "Preterite", "Irrealis", "GerundParticiple", "PastParticiple"],
    }},
    categories: {{
        "Sentence": (),
        "VerbPhrase": (features: ["VerbForm", "Voice"], gap: true),
        "AdjectivePhrase": (features: ["Voice"]),
        "NounPhrase": (features: ["Case"]),
        "Nominal": (features: ["Case"]),
        "Participle": (features: ["Inflection"]),
        "PrepositionPhrase": (features: ["ComplementKind"], gap: true),
    }},
    closed_words: {{ "By": "p:by" }},
    slot_categories: {{
        NP: "NounPhrase",
        PP: "PrepositionPhrase",
    }},
    {grammar_level}
    productions: [
        (name: "Imperative", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('.')],
            equations: [Require("clause.VerbForm", "Plain")]),
        (name: "Passive", lhs: "Sentence", rhs: [Cat("clause", "VerbPhrase"), Punct('!')],
            equations: [Require("clause.Voice", "Passive")]),
        (name: "Adjectival", lhs: "Sentence", rhs: [Cat("head", "AdjectivePhrase"), Punct('?')]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Cat("head", "Nominal")]),
        (name: "Fragment", lhs: "Sentence", rhs: [Cat("head", "NounPhrase"), Punct(';')]),
    ],
    schemas: [
        {premodification}
        Valence(head: Verb, phrase: "VerbPhrase", {valence}),
        Valence(head: Preposition, phrase: "PrepositionPhrase", complement_kind: Some("ComplementKind")),
        Valence(head: Noun, phrase: "Nominal"),
    ],
)"#
    )
}

const VOICES: &str = r#"active: [Require("$.Voice", "Active")], passive: Some("By"),
    verbal_passive: [Require("$.Voice", "Passive"), Require("$.VerbForm", "PastParticiple")],
    prepositional_passive: [Require("$.Voice", "Passive"), Require("$.VerbForm", "PastParticiple")],
    adjectival: Some("AdjectivePhrase"),
    adjectival_passive: [Require("head.VerbForm", "PastParticiple"), Require("$.Voice", "Passive")]"#;

const PREMODIFICATION: &str = r#"Premodification(head: Verb, modifier: "Participle", host: "Nominal", forms: [PastParticiple]),"#;

fn bind(text: &str) -> Result<(Lexicon, CompiledGrammar), LoadError> {
    let lexicon = lexicon();
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(text).unwrap(),
        &lexicon,
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )?;
    Ok((lexicon, grammar))
}

fn count(text: &str, input: &str) -> usize {
    let (lexicon, grammar) = bind(text).unwrap();
    grammar.parse(&lexicon, input).readings().len()
}

fn load_error(text: &str) -> String {
    bind(text).map(|_| ()).unwrap_err().to_string()
}

#[test]
fn the_declared_equations_value_each_derived_voice() {
    let text = grammar(VOICES, "", "");
    assert_eq!(count(&text, "Destroy Zombie."), 1);
    assert_eq!(count(&text, "Destroyed!"), 1);
    assert_eq!(count(&text, "Destroyed by Zombie!"), 1);
    assert_eq!(count(&text, "Destroyed?"), 1);
}

#[test]
fn a_passive_without_its_equations_is_a_load_error() {
    let error = load_error(&grammar(r#"passive: Some("By")"#, "", ""));
    assert!(error.contains("verbal_passive"), "{error}");
}

#[test]
fn an_adjectival_passive_without_its_equations_is_a_load_error() {
    let valence = VOICES.replace(
        r#"adjectival_passive: [Require("head.VerbForm", "PastParticiple"), Require("$.Voice", "Passive")]"#,
        "",
    );
    let error = load_error(&grammar(&valence, "", ""));
    assert!(error.contains("adjectival_passive"), "{error}");
}

#[test]
fn a_complement_kind_the_phrase_does_not_carry_is_a_load_error() {
    let valence = format!(r#"{VOICES}, complement_kind: Some("ComplementKind")"#);
    let error = load_error(&grammar(&valence, "", ""));
    assert!(error.contains("does not carry"), "{error}");
}

#[test]
fn slot_equations_constrain_every_frame_slot_of_their_kind() {
    let undeclared = grammar(VOICES, "", "");
    assert_eq!(count(&undeclared, "Destroy he."), 1);
    let declared = grammar(
        VOICES,
        r#"slot_equations: { NP: [Exclude("slot.Case", "Nominative")] },"#,
        "",
    );
    assert_eq!(count(&declared, "Destroy he."), 0);
    assert_eq!(count(&declared, "Destroy Zombie."), 1);
}

#[test]
fn a_high_attachment_declaration_names_declared_values() {
    let error = load_error(&grammar(
        VOICES,
        r#"high_attachment: [("PrepositionPhrase", "ComplementKind", "Distributive")],"#,
        "",
    ));
    assert!(error.contains("high attachment"), "{error}");
}

#[test]
fn premodification_equations_constrain_their_form() {
    let undeclared = grammar(VOICES, "", PREMODIFICATION);
    assert_eq!(count(&undeclared, "Destroyed Zombie;"), 1);
    let excluded = grammar(
        VOICES,
        "",
        r#"Premodification(head: Verb, modifier: "Participle", host: "Nominal", forms: [PastParticiple],
            equations: [(PastParticiple, [Exclude("head.Inflection", "PastParticiple")])]),"#,
    );
    assert_eq!(count(&excluded, "Destroyed Zombie;"), 0);
}

#[test]
fn premodification_equations_name_only_its_forms() {
    let error = load_error(&grammar(
        VOICES,
        "",
        r#"Premodification(head: Verb, modifier: "Participle", host: "Nominal", forms: [PastParticiple],
            equations: [(GerundParticiple, [])]),"#,
    ));
    assert!(error.contains("GerundParticiple"), "{error}");
}
