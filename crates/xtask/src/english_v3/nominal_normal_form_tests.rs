use super::*;
use crate::english_v3::tests::lexicon;
use deckmaste_english_v3::grammar::{Category, Grammar, Value};
use std::collections::BTreeSet;

fn readings(text: &str, category: Category) -> Vec<Reading> {
    let grammar = Grammar::with_lexicon(lexicon());
    let input = lexicon().analyze(text);
    let forest = deckmaste_english_v3::parse(&grammar, lexicon(), &input, &category).unwrap();
    let values: BTreeSet<_> = forest
        .readings(&grammar)
        .map(|value| value.unwrap())
        .collect();
    values
        .into_iter()
        .map(|v| {
            let Value::Reading(r) = v else { panic!("Reading root") };
            assert_eq!(r.realize_with(&grammar, lexicon()).unwrap(), text);
            r
        })
        .collect()
}

macro_rules! case {
    ($name:ident, $text:literal, $category:ident, $trees:literal, $normal:literal) => {
        #[test]
        fn $name() {
            let values = readings($text, Category::$category);
            assert_eq!(values.len(), $trees);
            let keys: BTreeSet<_> = values.iter().map(|r| fingerprint(r).unwrap()).collect();
            assert_eq!(keys.len(), $normal);
        }
    };
}
case!(judgment_1, "red creatures you control", NounPhrase, 2, 1);
case!(judgment_2, "red creatures with flying", NounPhrase, 2, 1);
case!(
    judgment_3,
    "red artifact creatures you control",
    NounPhrase,
    3,
    1
);
case!(
    judgment_4,
    "red nonlegendary artifact creatures you control",
    NounPhrase,
    4,
    1
);
case!(judgment_5, "tapped creatures you control", NounPhrase, 2, 1);
case!(
    judgment_6,
    "other red creatures you control",
    NounPhrase,
    3,
    2
);
case!(
    judgment_7,
    "only red creatures you control",
    NounPhrase,
    2,
    1
);
case!(
    judgment_8,
    "another red creature you control",
    NounPhrase,
    2,
    1
);
case!(
    judgment_16,
    "You may draw a card and discard a card.",
    Sentence,
    3,
    3
);
case!(
    judgment_17,
    "You draw a card and gain 1 life.",
    Sentence,
    2,
    2
);
case!(
    judgment_18,
    "You draw a card and you gain 1 life.",
    Sentence,
    1,
    1
);
case!(
    judgment_19,
    "Destroy target creature with flying.",
    Sentence,
    2,
    2
);
case!(
    judgment_20,
    "Destroy all creatures and planeswalkers.",
    Sentence,
    2,
    2
);
case!(
    judgment_21,
    "red creatures and planeswalkers",
    NounPhrase,
    3,
    3
);
case!(judgment_22, "Target creature is tapped.", Sentence, 2, 2);
case!(
    judgment_23,
    "Search your library for a card.",
    Sentence,
    3,
    3
);
case!(
    judgment_24,
    "This creature can't be blocked.",
    Sentence,
    2,
    2
);
case!(
    judgment_25,
    "This creature can't be blocked by Walls.",
    Sentence,
    7,
    7
);
case!(judgment_26, "the card you cast", NounPhrase, 2, 2);
case!(
    judgment_27,
    "the card you put into your hand",
    NounPhrase,
    2,
    2
);
case!(
    judgment_28,
    "Artifact spells you cast have convoke.",
    Sentence,
    4,
    2
);
case!(
    judgment_29,
    "Spells you cast cost {1} less to cast.",
    Sentence,
    4,
    4
);
case!(
    judgment_30,
    "This creature gets +1/+1 for each other creature you control.",
    Sentence,
    2,
    2
);
case!(
    judgment_31,
    "Creatures block creatures with flying.",
    Sentence,
    2,
    2
);
case!(judgment_34, "Tap or untap target creature.", Sentence, 1, 1);
case!(judgment_35, "Venture into the dungeon.", Sentence, 1, 1);
case!(judgment_36, "the creature that attacks", NounPhrase, 1, 1);
case!(judgment_37, "the creature you attack", NounPhrase, 1, 1);
case!(
    judgment_38,
    "Target creature gains \"Destroy target creature with flying. Draw a card.\" until end of turn.",
    Sentence,
    2,
    2
);
case!(
    judgment_40,
    "the card you are putting into your hand",
    NounPhrase,
    0,
    0
);

case!(
    unlisted_nonred_is_a_barrier,
    "nonred creatures you control",
    NounPhrase,
    2,
    2
);
case!(
    unlisted_enchanted_is_a_barrier,
    "enchanted creatures you control",
    NounPhrase,
    2,
    2
);
case!(
    unlisted_subtype_is_a_barrier,
    "Goblin creatures you control",
    NounPhrase,
    2,
    2
);

#[test]
fn lexical_payload_form_variant_frame_and_quotation_are_retained() {
    let reading = readings("red creatures you control", Category::NounPhrase).remove(0);
    let text = format!("{reading:?}");
    let original = encoded(&text).unwrap();
    for (from, to) in [
        ("vocab:ColorWord/Red", "vocab:ColorWord/Blue"),
        ("form: Plural", "form: Singular"),
        ("variant: 0", "variant: 1"),
        ("frame: Some(0)", "frame: Some(1)"),
    ] {
        assert!(text.contains(from), "{from}");
        assert_ne!(
            original,
            encoded(&text.replacen(from, to, 1)).unwrap(),
            "{from}"
        );
    }
    assert_ne!(
        original,
        encoded(&format!("QuotedText {{ form: 0, text: {text} }}")).unwrap()
    );
}

#[test]
fn malformed_or_changed_known_shapes_fail_closed() {
    for text in [
        "PremodifiedNominal { head: Noun }",
        "Noun { head:",
        "Noun {} trailing",
        "Noun { head: A, head: B }",
    ] {
        assert!(encoded(text).is_err(), "{text}");
    }
    assert_ne!(
        encoded("Unknown { form: 0, head: A }").unwrap(),
        encoded("Unknown { form: 1, head: A }").unwrap()
    );
    assert_ne!(
        encoded("Unknown { head: A, modifier: B }").unwrap(),
        encoded("Unknown { head: B, modifier: A }").unwrap()
    );
}

#[test]
fn strings_are_atomic_and_repeated_occurrences_are_not_sets() {
    let text =
        r#"Unknown { text: "a\"b: PremodifiedNominal { head: X }", words: [Word(A), Word(A)] }"#;
    assert_ne!(
        encoded(text).unwrap(),
        encoded(&text.replace("Word(A), Word(A)", "Word(A)")).unwrap()
    );
    assert_ne!(
        encoded("Unknown { words: [Word(A), Word(B)] }").unwrap(),
        encoded("Unknown { words: [Word(B), Word(A)] }").unwrap()
    );
}

#[test]
fn recursive_nominal_inventory_requires_review_when_grammar_changes() {
    use proc_macro2::TokenTree;
    fn has_nominal_head(tokens: proc_macro2::TokenStream) -> bool {
        let tokens: Vec<_> = tokens.into_iter().collect();
        tokens.windows(3).any(|w| {
            w[0].to_string() == "head"
                && w[1].to_string() == ":"
                && matches!(&w[2], TokenTree::Ident(i) if i == "Nominal")
        }) || tokens.iter().any(|t| match t {
            TokenTree::Group(g) => has_nominal_head(g.stream()),
            _ => false,
        })
    }
    let file = syn::parse_file(include_str!(
        "../../../deckmaste_english_v3/src/declarations.rs"
    ))
    .unwrap();
    let declaration = file
        .items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Macro(m) if m.mac.path.is_ident("constructions") => Some(m.mac.tokens),
            _ => None,
        })
        .unwrap();
    let body = declaration
        .into_iter()
        .find_map(|t| match t {
            TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        })
        .unwrap();
    let tokens: Vec<_> = body.into_iter().collect();
    let mut recursive = BTreeSet::new();
    for window in tokens.windows(5) {
        if window[0].to_string() == "construction"
            && window[3].to_string() == "Nominal"
            && let TokenTree::Group(body) = &window[4]
            && has_nominal_head(body.stream())
        {
            recursive.insert(window[1].to_string());
        }
    }
    let expected: BTreeSet<_> = [
        "CoordinatedPremodifiedNominal",
        "NounPremodifiedNominal",
        "PremodifiedNominal",
        "SuperlativePremodifiedNominal",
        "CardinalPremodifiedNominal",
        "CardinalMeasuredNominal",
        "ParticipialPremodifier",
        "NamedNominal",
        "TargetedNominal",
        "PostmodifiedNominal",
        "ParticipialPostmodifiedNominal",
        "SubjectRelativeNominal",
        "ObjectRelativeNominal",
        "SlashModifiedNominal",
        "CorrelativePostpositiveNominal",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(
        recursive, expected,
        "classify every new recursive nominal use; unlisted uses stay barriers"
    );
}

#[test]
fn a_derived_card_type_id_does_not_extend_the_reviewed_inventory() {
    let values = readings("artifact creatures you control", Category::NounPhrase);
    assert_eq!(values.len(), 2);
    let mut ordinary = BTreeSet::new();
    let mut derived = BTreeSet::new();
    for value in values {
        let debug = format!("{value:?}");
        assert!(debug.contains("lexeme:type/artifact"));
        ordinary.insert(encoded(&debug).unwrap());
        derived.insert(
            encoded(&debug.replace("lexeme:type/artifact", "lexeme:type/artifact/non")).unwrap(),
        );
    }
    assert_eq!(ordinary.len(), 1);
    assert_eq!(derived.len(), 2);
}

#[test]
fn normal_form_version_identifies_the_exact_reviewed_source() {
    // Append a version when this source changes; never replace an old entry.
    let versions = [
        (
            2,
            "9cacc5055c558101180b903645271e91874613e1ee4b8d29585755bb8c685e3e",
        ),
        (
            3,
            "2b48ff9165ddecd677360be67aefe948248701b03b6db81a1bf7dfb7c8ba6273",
        ),
    ];
    assert_eq!(versions.last().unwrap().0, VERSION);
    assert_eq!(
        versions.last().unwrap().1,
        source_sha256(),
        "classification or normalization changed: bump VERSION and append its source hash"
    );
}

#[test]
fn perilous_predicament_census_records_two_distinct_normal_forms() {
    use crate::english_v3::report::{Measurements, Report};
    use crate::english_v3::tests::{args, selected_corpus};
    use crate::english_v3::{analyze_cards, write_report};
    use serde_json::json;

    let text =
        "Each opponent sacrifices an artifact creature and a nonartifact creature of their choice.";
    let corpus = selected_corpus(
        &serde_json::to_vec(&json!({"data": {
            "Perilous Predicament": [{"name": "Perilous Predicament", "layout": "normal",
                "legalities": {"vintage": "Legal"}, "text": text}]
        }}))
        .unwrap(),
    );
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/english-v3");
    std::fs::create_dir_all(&root).unwrap();
    let directory = tempfile::tempdir_in(root).unwrap();
    let mut args = args(None);
    let path = directory.path().join("baseline.json");
    args.output = Some(path.clone());
    let grammar = Grammar::with_lexicon(lexicon());
    let faces = analyze_cards(&corpus.faces, lexicon(), &grammar, &args).unwrap();
    let report = Report::new(
        &args,
        &corpus,
        "test".into(),
        vec![],
        faces,
        Measurements {
            setup_wall_ns: 0,
            corpus_wall_ns: 0,
            host_load: None,
        },
    );
    write_report(&args, &report, &mut Vec::new()).unwrap();
    let baseline: serde_json::Value =
        serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();
    assert_eq!(baseline["nominal_normal_form_version"], VERSION);
    assert_eq!(
        baseline["nominal_normal_form_source_sha256"],
        source_sha256()
    );
    let face = &baseline["faces"][0];
    assert_eq!(face["name"], "Perilous Predicament");
    assert_eq!(face["lines"].as_array().unwrap().len(), 1);
    let recorded: BTreeSet<String> =
        serde_json::from_value(face["lines"][0]["nominal_normal_forms"].clone()).unwrap();
    let computed: BTreeSet<_> = readings(text, Category::Sentence)
        .iter()
        .map(|r| fingerprint(r).unwrap())
        .collect();
    assert_eq!(computed.len(), 2);
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded, computed);
}

#[test]
fn lexical_modifier_order_agrees_with_the_independent_reviewed_inventory() {
    use deckmaste_lexical::Category as LexicalCategory;
    fn commuting_word(id: &str, noun: bool) -> bool {
        let name = if noun { "NounPremodifier" } else { "Adjective" };
        let debug = format!(
            "{name} {{ form: 0, head: Word {{ value: Word(Word {{ lexeme: Ident({id:?}) }}) }} }}"
        );
        Parser { input: &debug }.tree().unwrap().commuting_pre(noun)
    }
    let mut admitted = BTreeSet::new();
    for (id, lexeme) in lexicon().lexemes() {
        let reviewed = match lexeme.category {
            LexicalCategory::Adjective => commuting_word(id.as_str(), false),
            LexicalCategory::Noun => commuting_word(id.as_str(), true),
            _ => false,
        };
        let declared = lexeme
            .properties
            .features
            .get("NominalModifierOrder")
            .map_or("Barrier", String::as_str);
        assert_eq!(
            declared,
            if reviewed { "Commuting" } else { "Barrier" },
            "{id}"
        );
        if reviewed {
            admitted.insert(id.to_string());
        }
    }
    assert!(admitted.contains("vocab:ColorWord/Red"));
    assert!(admitted.contains("vocab:PredicativeAdjective/Legendary/non"));
    assert!(admitted.contains("lexeme:keyword_action/tap/adjective"));
    assert!(admitted.contains("lexeme:type/artifact"));
    assert!(!admitted.contains("vocab:ColorWord/Red/non"));
    assert!(!admitted.contains("lexeme:type/artifact/non"));
}

#[test]
fn layer_adapter_preserves_old_records_and_barrier_scope() {
    let legacy =
        "PremodifiedNominal { form: 0, modifier: Other, head: Noun { form: 0, head: Creature } }";
    let layered = "NominalProjection { form: 0, head: NominalCoreProjection { form: 0, head: BarrierPremodifiedNominal { form: 0, modifier: Other, head: NominalProjection { form: 0, head: NominalCoreProjection { form: 0, head: Noun { form: 0, head: Creature } } } } } }";
    assert_eq!(encoded(legacy).unwrap(), encoded(layered).unwrap());
    assert_ne!(
        encoded(layered).unwrap(),
        encoded(&layered.replace("Creature", "Artifact")).unwrap()
    );
    for (name, fields) in [
        ("Coordination", "left: A, coordinator: And, right: B"),
        ("SerialCoordination", "left: A, rest: B"),
        (
            "DiscontinuousComparativeNominal",
            "governor: More, head: A, marker: Than, complement: B",
        ),
        ("PostpositiveNominal", "head: A, modifier: B"),
    ] {
        let old = format!("{name} {{ form: 0, category: Nominal, {fields} }}");
        let new = old.replace("category: Nominal,", "category: NominalCore,");
        assert_eq!(encoded(&old).unwrap(), encoded(&new).unwrap());
        assert_ne!(
            encoded(&old).unwrap(),
            encoded(&old.replace("category: Nominal,", "category: NounPhrase,")).unwrap()
        );
        assert!(encoded(&new.replace("form: 0", "form: 1")).is_err());
        assert!(encoded(&new.replace("form: 0,", "form: 0, extra: X,")).is_err());
    }
    for name in [
        "NominalProjection",
        "NominalCoreProjection",
        "BarrierPremodifiedNominal",
        "BarrierNounPremodifiedNominal",
    ] {
        assert!(encoded(&format!("{name} {{ form: 1, head: A }}")).is_err());
        assert!(encoded(&format!("{name} {{ form: 0, head: A, extra: B }}")).is_err());
    }
    assert_ne!(
        encoded("Unreviewed { form: 0, category: NominalCore, head: A }").unwrap(),
        encoded("Unreviewed { form: 0, category: Nominal, head: A }").unwrap()
    );
}

#[test]
fn layer_lifts_do_not_interrupt_a_commuting_head_chain() {
    fn lift_nouns(tree: Tree) -> Tree {
        match tree {
            Tree::Record(name, fields) => {
                let record = Tree::Record(
                    name.clone(),
                    fields
                        .into_iter()
                        .map(|(n, v)| (n, lift_nouns(v)))
                        .collect(),
                );
                if name == "Noun" {
                    Tree::Record(
                        "NominalProjection".into(),
                        vec![
                            ("form".into(), Tree::Atom("0".into())),
                            (
                                "head".into(),
                                Tree::Record(
                                    "NominalCoreProjection".into(),
                                    vec![
                                        ("form".into(), Tree::Atom("0".into())),
                                        ("head".into(), record),
                                    ],
                                ),
                            ),
                        ],
                    )
                } else {
                    record
                }
            }
            Tree::Call(n, vs) => Tree::Call(n, vs.into_iter().map(lift_nouns).collect()),
            Tree::List(vs) => Tree::List(vs.into_iter().map(lift_nouns).collect()),
            t => t,
        }
    }
    for reading in readings("red creatures you control", Category::NounPhrase) {
        let debug = format!("{reading:?}");
        let old = Parser { input: &debug }.tree().unwrap();
        let new = lift_nouns(old.clone());
        assert_ne!(old, new);
        assert_eq!(
            old.normalize().unwrap(),
            new.adapt_layers().unwrap().normalize().unwrap()
        );
    }
}
