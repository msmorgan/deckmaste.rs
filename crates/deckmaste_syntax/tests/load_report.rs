//! Binding reports a feature with no value and a feature value with fewer
//! than two holders as findings (`english-lexical-analysis.md` §3,
//! `english-v4.md` §4.4 item 5), without refusing the grammar. A feature may
//! declare one, privative, value. A value's holders are counted as they are:
//! the members of a lifted class, the lexemes that declare a Frame or a
//! Parameter pattern that gives it, and once across linked features.

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_syntax::CompiledGrammar;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::Grammar;

fn source() -> Source {
    Source {
        kind: SourceKind::Core,
        path: "tests/load_report.rs".to_owned(),
        owner: "test".to_owned(),
    }
}

/// Two common nouns and one rare one; three adverbs, two of them front;
/// four prepositions; two keywords.
fn lexicon() -> Lexicon {
    let mut lexemes = vec![
        Lexeme::invariant("det:the", "the", Category::Determinative, source()),
        Lexeme::invariant("det:a", "a", Category::Determinative, source()),
        Lexeme::invariant("adv:first", "first", Category::Adverb, source()),
        Lexeme::invariant("adv:then", "then", Category::Adverb, source()),
        Lexeme::invariant("adv:only", "only", Category::Adverb, source()),
        Lexeme::invariant("prep:out", "out", Category::Preposition, source()),
        Lexeme::invariant("prep:up", "up", Category::Preposition, source()),
        Lexeme::invariant("prep:of", "of", Category::Preposition, source()),
        Lexeme::invariant("prep:with", "with", Category::Preposition, source()),
        Lexeme::invariant("keyword:ward", "ward", Category::Keyword, source()),
        Lexeme::invariant("keyword:echo", "echo", Category::Keyword, source()),
    ];
    for (noun, kind) in [("card", "Common"), ("land", "Common"), ("relic", "Rare")] {
        let mut lexeme = Lexeme::noun(
            format!("noun:{noun}"),
            noun,
            vec![Countability::Count],
            source(),
        );
        lexeme
            .properties
            .features
            .insert("Kind".into(), kind.into());
        lexemes.push(lexeme);
    }
    Lexicon::new(lexemes).unwrap()
}

/// `prep:out` and `prep:up` declare an Intransitive frame, `prep:of` alone
/// an NP one. `keyword:ward` has two patterns that each end in a Cost, and
/// `keyword:echo` one that ends in an Amount.
const FRAMES: &str = r#"(
    prepositions: {
        "prep:out": [(complement: Intransitive, hosts: ComplementOnly)],
        "prep:up": [(complement: Intransitive, hosts: ComplementOnly)],
        "prep:of": [(complement: NP, hosts: ComplementOnly)],
    },
    parameters: {
        "keyword:ward": [[Head, Feature("Final", "Cost")], [Head, Slot(NP), Feature("Final", "Cost")]],
        "keyword:echo": [[Head, Slot(NP), Feature("Final", "Amount")]],
    },
)"#;

/// `Kind` is lexical; `Form`, `Lone` and `Plural` are set by Productions;
/// `Mark` is privative. `Number` is projected from inflection. `Position` is
/// set by a Production lifting a class, `ComplementKind` by the Preposition
/// Valence, and `Final` by Parameter patterns; `ObliqueKind` is linked to
/// `ComplementKind`.
const GRAMMAR: &str = r#"(
    root: "Sentence",
    features: {
        "Number": ["Singular", "Plural"],
        "Kind": ["Common", "Rare"],
        "Form": ["Plain", "Marked"],
        "Lone": ["Only"],
        "Mark": ["Set"],
        "Position": ["Front", "End"],
        "ComplementKind": ["NP", "Intransitive"],
        "ObliqueKind": ["NP", "Intransitive"],
        "Final": ["Cost", "Amount"],
    },
    categories: {
        "Sentence": (),
        "NounPhrase": (features: ["Number", "Kind", "Form", "Lone", "Mark"]),
        "AdverbPhrase": (features: ["Position"]),
        "PrepositionPhrase": (features: ["ComplementKind"]),
        "Oblique": (features: ["ObliqueKind"]),
        "KeywordParameters": (features: ["Final"]),
    },
    closed_words: { "The": "det:the", "A": "det:a", "With": "prep:with", "Only": "adv:only" },
    lexical_classes: { "FrontAdverb": ["adv:first", "adv:then"] },
    slot_categories: { NP: "NounPhrase" },
    productions: [
        (name: "Statement", lhs: "Sentence", rhs: [Cat("subject", "NounPhrase"), Punct('.')]),
        (name: "Bare", lhs: "NounPhrase", rhs: [Lex("head", Noun)], equations: [Require("$.Form", "Plain")]),
        (name: "Definite", lhs: "NounPhrase", rhs: [Word("det", "The"), Lex("head", Noun)], equations: [Require("$.Form", "Plain"), Require("$.Mark", "Set")]),
        (name: "Indefinite", lhs: "NounPhrase", rhs: [Word("det", "A"), Lex("head", Noun)], equations: [Require("$.Form", "Marked"), Require("$.Lone", "Only"), Require("$.Mark", "Set")]),
        (name: "Front", lhs: "AdverbPhrase", rhs: [Class("head", "FrontAdverb")], equations: [Require("$.Position", "Front")]),
        (name: "End", lhs: "AdverbPhrase", rhs: [Word("head", "Only")], equations: [Require("$.Position", "End")]),
        (name: "Oblique", lhs: "Oblique", rhs: [Cat("head", "PrepositionPhrase")], equations: [Agree("$.ObliqueKind", "head.ComplementKind")]),
        (name: "Accompanied", lhs: "PrepositionPhrase", rhs: [Word("head", "With"), Cat("complement", "NounPhrase")], equations: [Require("$.ComplementKind", "NP")]),
    ],
    schemas: [
        Valence(head: Preposition, phrase: "PrepositionPhrase", complement_kind: Some("ComplementKind")),
        Parameters(head: Keyword, phrase: "KeywordParameters"),
    ],
)"#;

fn warnings() -> Vec<String> {
    let grammar = CompiledGrammar::bind(
        &Grammar::from_ron_str(GRAMMAR).unwrap(),
        &lexicon(),
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap();
    grammar.report().warnings.clone()
}

fn assert_reported(warnings: &[String], expected: &str) {
    assert!(
        warnings.iter().any(|w| w == expected),
        "{expected}\n{warnings:#?}"
    );
}

fn assert_unreported(warnings: &[String], value: &str) {
    assert!(
        !warnings.iter().any(|w| w.contains(value)),
        "{value}\n{warnings:#?}"
    );
}

#[test]
fn a_privative_feature_is_reported_only_by_its_holders() {
    let warnings = warnings();
    // One value, held by two Productions: no finding.
    assert!(
        !warnings.iter().any(|w| w.contains("`Mark`")),
        "{warnings:#?}"
    );
    // One value, held by one Production: its value is reported.
    assert_reported(
        &warnings,
        "feature value `Lone` `Only` has 1 holder(s) (Indefinite); a value needs two or more",
    );
    assert!(
        !warnings.iter().any(|w| w.starts_with("feature `")),
        "{warnings:#?}"
    );
}

#[test]
fn a_value_with_one_holder_is_reported_with_its_holder() {
    let warnings = warnings();
    for expected in [
        // One lexeme carries it.
        "feature value `Kind` `Rare` has 1 holder(s) (noun:relic); a value needs two or more",
        // No lexeme carries it, and one Production sets it.
        "feature value `Form` `Marked` has 1 holder(s) (Indefinite); a value needs two or more",
    ] {
        assert_reported(&warnings, expected);
    }
    // Two lexemes, two Productions, and inflection hold the rest.
    for unreported in ["`Common`", "`Plain`", "`Singular`", "`Plural`"] {
        assert_unreported(&warnings, unreported);
    }
}

#[test]
fn a_lifted_class_value_is_held_by_the_class_members() {
    let warnings = warnings();
    // `Front` is set by one Production, lifting two adverbs.
    assert_unreported(&warnings, "`Front`");
    // `End` is set by one Production on one Closed Word.
    assert_reported(
        &warnings,
        "feature value `Position` `End` has 1 holder(s) (adv:only); a value needs two or more",
    );
}

#[test]
fn a_projected_slot_kind_is_held_by_the_lexemes_declaring_the_frame() {
    let warnings = warnings();
    // Two prepositions declare the Intransitive frame of one Valence draft.
    assert_unreported(&warnings, "`Intransitive`");
    // *of* declares the NP frame and *with* heads a Production that sets it,
    // each reported once across the linked features.
    assert_unreported(&warnings, "`NP`");
}

#[test]
fn a_parameter_kind_is_held_by_the_keywords_declaring_it() {
    let warnings = warnings();
    // Two patterns of one keyword: one holder.
    assert_reported(
        &warnings,
        "feature value `Final` `Cost` has 1 holder(s) (keyword:ward); a value needs two or more",
    );
    assert_reported(
        &warnings,
        "feature value `Final` `Amount` has 1 holder(s) (keyword:echo); a value needs two or more",
    );
}

#[test]
fn a_value_of_linked_features_is_reported_once_naming_both() {
    let grammar = GRAMMAR.replace(
        r#"(name: "Accompanied", lhs: "PrepositionPhrase", rhs: [Word("head", "With"), Cat("complement", "NounPhrase")], equations: [Require("$.ComplementKind", "NP")]),"#,
        "",
    );
    let warnings = CompiledGrammar::bind(
        &Grammar::from_ron_str(&grammar).unwrap(),
        &lexicon(),
        &FrameCatalog::from_ron_str(FRAMES).unwrap(),
    )
    .unwrap()
    .report()
    .warnings
    .clone();
    let reported: Vec<&String> = warnings.iter().filter(|w| w.contains("`NP`")).collect();
    assert_eq!(
        reported,
        [
            "feature value `ComplementKind`/`ObliqueKind` `NP` has 1 holder(s) (prep:of); a \
             value needs two or more"
        ],
        "{warnings:#?}"
    );
}
