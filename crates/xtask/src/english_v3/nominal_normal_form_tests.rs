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
case!(judgment_1, "red creatures you control", NounPhrase, 1, 1);
case!(judgment_2, "red creatures with flying", NounPhrase, 1, 1);
case!(
    judgment_3,
    "red artifact creatures you control",
    NounPhrase,
    1,
    1
);
case!(
    judgment_4,
    "red nonlegendary artifact creatures you control",
    NounPhrase,
    1,
    1
);
case!(judgment_5, "tapped creatures you control", NounPhrase, 1, 1);

case!(
    judgment_7,
    "only red creatures you control",
    NounPhrase,
    1,
    1
);
case!(
    judgment_8,
    "another red creature you control",
    NounPhrase,
    1,
    1
);

case!(
    judgment_18,
    "You draw a card and you gain 1 life.",
    Sentence,
    1,
    1
);

case!(judgment_22, "Target creature is tapped.", Sentence, 2, 2);

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
    2,
    2
);

case!(judgment_34, "Tap or untap target creature.", Sentence, 1, 1);
case!(judgment_35, "Venture into the dungeon.", Sentence, 1, 1);
case!(judgment_36, "the creature that attacks", NounPhrase, 1, 1);
case!(judgment_37, "the creature you attack", NounPhrase, 1, 1);

case!(
    reviewed_nonred_commutes_with_relatives,
    "nonred creatures you control",
    NounPhrase,
    1,
    1
);
case!(
    reviewed_enchanted_commutes_with_relatives,
    "enchanted creatures you control",
    NounPhrase,
    1,
    1
);
case!(
    reviewed_subtype_commutes_with_relatives,
    "Goblin creatures you control",
    NounPhrase,
    1,
    1
);

#[test]
fn lexical_payload_form_variant_frame_and_quotation_are_retained() {
    let text = format!(
        "Noun {{ form: 0, head: {} }}",
        synthetic_word("vocab:ColorWord/Red")
    );
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
                && matches!(&w[2], TokenTree::Ident(i) if matches!(i.to_string().as_str(), "Nominal" | "NominalCore" | "NominalPostmodification"))
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
            && matches!(
                window[3].to_string().as_str(),
                "Nominal" | "NominalCore" | "NominalPostmodification"
            )
            && let TokenTree::Group(body) = &window[4]
            && has_nominal_head(body.stream())
        {
            recursive.insert(window[1].to_string());
        }
    }
    let expected: BTreeSet<_> = [
        "NominalProjection",
        "NominalCoreProjection",
        "BarrierPremodifiedNominal",
        "BarrierNounPremodifiedNominal",
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
        (
            4,
            "3f29a1d713f733a12e3af7668a9a9056fbf32128220ef70c0156d32ff7618f5f",
        ),
        (
            5,
            "d0a4880a635ed7c552ee37123f25a43eee4967b9aab4eaa324ad668f4412716f",
        ),
        (
            6,
            "be40c1d4c7838ea990789ad86b418f26b94a281838355e6ab264d5def59b2af3",
        ),
        (
            7,
            "f0e9f5b0fd35c6a69dd681eabe8ae32c53ae54700c715070e215e218b9c6a32c",
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
fn perilous_predicament_census_records_the_committed_normal_forms() {
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
    assert!(!computed.is_empty());
    assert_eq!(recorded, computed);
}

#[test]
fn lexical_modifier_order_agrees_with_the_independent_reviewed_inventory() {
    use deckmaste_lexical::Category as LexicalCategory;
    fn commuting_word(id: &str, noun: bool) -> bool {
        let name = if noun {
            "NounPremodifier"
        } else if id == "vocab:AttributiveAdjective/Other" {
            "IntransitiveAdjective"
        } else {
            "Adjective"
        };
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
    assert!(admitted.contains("vocab:AttributiveAdjective/Other"));
    assert!(!admitted.contains("vocab:Determinative/Another"));
    assert!(!admitted.contains("lexeme:CommonNoun/Other"));
    assert!(admitted.contains("vocab:PredicativeAdjective/Legendary/non"));
    assert!(admitted.contains("lexeme:keyword_action/tap/adjective"));
    for word in ["enchant", "equip"] {
        assert!(admitted.contains(&format!("lexeme:keyword_ability/{word}/adjective")));
        assert!(!admitted.contains(&format!("lexeme:keyword_ability/{word}")));
    }
    for id in [
        "lexeme:keyword_action/untap/adjective",
        "lexeme:keyword_ability/fortify/adjective",
        "lexeme:keyword_ability/kicker/adjective",
    ] {
        assert!(!admitted.contains(id));
    }
    for subtype in ["goblin", "elf", "soldier", "wizard"] {
        assert!(admitted.contains(&format!("lexeme:creature_subtype/{subtype}")));
        assert!(!admitted.contains(&format!("lexeme:creature_subtype/{subtype}/non")));
        assert!(!admitted.contains(&format!(
            "lexeme:creature_subtype/{subtype}/bound-keyword-stem"
        )));
    }
    assert!(admitted.contains("lexeme:type/artifact"));
    for colour in ["White", "Blue", "Black", "Red", "Green"] {
        assert!(admitted.contains(&format!("vocab:ColorWord/{colour}/non")));
    }
    assert!(!admitted.contains("lexeme:type/creature/non"));
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
        let old = Parser { input: &debug }
            .tree()
            .unwrap()
            .adapt_layers()
            .unwrap();
        let new = lift_nouns(old.clone());
        assert_ne!(old, new);
        assert_eq!(
            old.normalize().unwrap(),
            new.adapt_layers().unwrap().normalize().unwrap()
        );
    }
}

#[test]
fn negative_colours_remove_only_the_simple_modifier_order_twin() {
    for colour in ["white", "blue", "black", "red", "green"] {
        let text = format!("non{colour} creatures you control");
        let values = readings(&text, Category::NounPhrase);
        assert_eq!(values.len(), 1, "{text}");
        assert_eq!(
            values
                .iter()
                .map(|r| fingerprint(r).unwrap())
                .collect::<BTreeSet<_>>()
                .len(),
            1
        );
    }
}

#[test]
fn reviewed_bare_state_adjectives_remove_the_modifier_order_twin() {
    for word in ["enchanted", "equipped"] {
        let text = format!("{word} creatures you control");
        let values = readings(&text, Category::NounPhrase);
        assert_eq!(values.len(), 1, "{text}");
        assert_eq!(
            values
                .iter()
                .map(|r| fingerprint(r).unwrap())
                .collect::<BTreeSet<_>>()
                .len(),
            1
        );
    }
}

#[test]
fn reviewed_state_ids_do_not_license_non_simple_modifier_shapes() {
    for id in [
        "lexeme:keyword_ability/enchant/adjective",
        "lexeme:keyword_ability/equip/adjective",
    ] {
        let word = format!("Word {{ value: Word(Word {{ lexeme: Ident({id:?}) }}) }}");
        let simple = format!("Adjective {{ form: 0, head: {word} }}");
        assert!(
            Parser { input: &simple }
                .tree()
                .unwrap()
                .commuting_pre(false)
        );
        for shape in [
            format!("Adjective {{ form: 1, head: {word} }}"),
            format!("Adjective {{ form: 0, head: {word}, complement: Phrase }}"),
            format!("Adjective {{ form: 0, head: {word}, agent: Phrase }}"),
            format!("SelectedPredicate {{ form: 0, head: {word} }}"),
            format!("Coordination {{ form: 0, left: {simple}, right: {simple} }}"),
        ] {
            assert!(
                !Parser { input: &shape }
                    .tree()
                    .unwrap()
                    .commuting_pre(false),
                "{shape}"
            );
        }
    }
}

#[test]
fn simple_creature_subtypes_remove_the_modifier_order_twin() {
    for word in ["Goblin", "Elf", "Soldier", "Wizard"] {
        let text = format!("{word} creatures you control");
        let values = readings(&text, Category::NounPhrase);
        assert_eq!(values.len(), 1, "{text}");
        assert_eq!(
            values
                .iter()
                .map(|r| fingerprint(r).unwrap())
                .collect::<BTreeSet<_>>()
                .len(),
            1,
            "{text}"
        );
    }
}

#[test]
fn subtype_inventory_does_not_license_quoted_coordinated_or_derived_modifiers() {
    let id = "lexeme:creature_subtype/goblin";
    let word = format!("Word {{ value: Word(Word {{ lexeme: Ident({id:?}) }}) }}");
    let simple = format!("NounPremodifier {{ form: 0, head: {word} }}");
    assert!(
        Parser { input: &simple }
            .tree()
            .unwrap()
            .commuting_pre(true)
    );
    for shape in [
        simple.replace("form: 0", "form: 1"),
        simple.replace(id, &format!("{id}/non")),
        simple.replace(id, &format!("{id}/bound-keyword-stem")),
        format!("NounPremodifier {{ form: 0, head: Quoted {{ head: {word} }} }}"),
        format!("QuotedNominal {{ form: 0, head: {word} }}"),
        format!("Coordination {{ form: 0, left: {simple}, right: {simple} }}"),
    ] {
        assert!(
            !Parser { input: &shape }.tree().unwrap().commuting_pre(true),
            "{shape}"
        );
    }
}

fn synthetic_word(id: &str) -> String {
    format!(
        "Word {{ value: Word(Word {{ lexeme: Ident({id:?}), form: Plural, variant: 0 }}), frame: Some(0) }}"
    )
}

fn synthetic_premodifier(id: &str, noun: bool) -> String {
    let category = if noun { "NounPremodifier" } else { "Adjective" };
    format!("{category} {{ form: 0, head: {} }}", synthetic_word(id))
}

fn synthetic_pre(modifier: &str, head: &str) -> String {
    format!("PremodifiedNominal {{ form: 0, modifier: {modifier}, head: {head} }}")
}

fn synthetic_post(head: &str, modifier: &str) -> String {
    format!("PostmodifiedNominal {{ form: 0, head: {head}, modifier: {modifier} }}")
}

#[test]
fn synthetic_trees_preserve_attachment_lexeme_form_gap_and_barrier_scope() {
    // These are inputs to the tree normaliser, not claims that a parser should
    // admit either member of a pair.
    let noun = format!("Noun {{ form: 0, head: {} }}", synthetic_word("head-a"));
    let dependent_on_left = format!(
        "Coordination {{ form: 0, category: Nominal, left: {}, coordinator: And, right: {noun} }}",
        synthetic_post(&noun, "Dependent")
    );
    let dependent_on_right = format!(
        "Coordination {{ form: 0, category: Nominal, left: {noun}, coordinator: And, right: {} }}",
        synthetic_post(&noun, "Dependent")
    );
    assert_ne!(
        encoded(&dependent_on_left).unwrap(),
        encoded(&dependent_on_right).unwrap()
    );
    let other_noun = noun.replace("head-a", "head-b");
    let red = synthetic_premodifier("vocab:ColorWord/Red", false);
    let red_noun = synthetic_pre(&red, &noun);
    let red_other_noun = synthetic_pre(&red, &other_noun);
    let coordinate = |left: &str, right: &str| {
        format!(
            "Coordination {{ form: 0, category: Nominal, left: {left}, coordinator: And, right: {right} }}"
        )
    };
    let post_on_head_a = coordinate(&synthetic_post(&red_noun, "Dependent"), &red_other_noun);
    let post_on_head_b = coordinate(&red_noun, &synthetic_post(&red_other_noun, "Dependent"));
    assert_eq!(
        encoded(&post_on_head_a).unwrap(),
        encoded(&coordinate(
            &synthetic_pre(&red, &synthetic_post(&noun, "Dependent")),
            &red_other_noun,
        ))
        .unwrap()
    );
    assert_eq!(
        encoded(&post_on_head_b).unwrap(),
        encoded(&coordinate(
            &red_noun,
            &synthetic_pre(&red, &synthetic_post(&other_noun, "Dependent")),
        ))
        .unwrap()
    );
    assert_ne!(
        encoded(&post_on_head_a).unwrap(),
        encoded(&post_on_head_b).unwrap()
    );
    for replacement in [
        noun.replace("head-a", "head-b"),
        noun.replace("form: Plural", "form: Singular"),
        noun.replace("variant: 0", "variant: 1"),
        noun.replace("frame: Some(0)", "frame: Some(1)"),
    ] {
        assert_ne!(encoded(&noun).unwrap(), encoded(&replacement).unwrap());
    }
    let object_gap = format!(
        "ObjectRelativeNominal {{ form: 0, head: {noun}, relative: ZeroObjectRelativeClause {{ form: 0, subject: Subject, predicate: FiniteObjectGap {{ form: 0, head: Predicate }} }} }}"
    );
    let adjunct_gap = format!(
        "ObjectRelativeNominal {{ form: 0, head: {noun}, relative: AdverbialGapRelativeClause {{ form: 0, clause: FiniteClause {{ form: 0, subject: Subject, predicate: Predicate }} }} }}"
    );
    assert_ne!(
        encoded(&object_gap).unwrap(),
        encoded(&adjunct_gap).unwrap()
    );
    let cardinal = |head: &str| {
        format!("CardinalPremodifiedNominal {{ form: 0, quantity: Three, head: {head} }}")
    };
    assert_ne!(
        encoded(&cardinal(&synthetic_post(&noun, "Restriction"))).unwrap(),
        encoded(&synthetic_post(&cardinal(&noun), "Restriction")).unwrap()
    );
}

#[test]
fn synthetic_order_twin_removal_preserves_the_set_but_distinct_tree_loss_does_not() {
    let modifier = synthetic_premodifier("vocab:ColorWord/Red", false);
    let noun = "Noun { form: 0, head: HeadA }";
    let a = synthetic_pre(&modifier, &synthetic_post(noun, "Restriction"));
    let b = synthetic_post(&synthetic_pre(&modifier, noun), "Restriction");
    let keys = |trees: &[String]| {
        trees
            .iter()
            .map(|s| encoded(s).unwrap())
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        keys(&[a.clone(), b.clone()]),
        keys(std::slice::from_ref(&a))
    );
    let different_head = a.replace("HeadA", "HeadB");
    assert_ne!(
        keys(&[a.clone(), different_head]),
        keys(std::slice::from_ref(&a))
    );
    let inside = format!("CardinalPremodifiedNominal {{ form: 0, quantity: Three, head: {b} }}");
    let outside = synthetic_post(
        &format!(
            "CardinalPremodifiedNominal {{ form: 0, quantity: Three, head: {} }}",
            synthetic_pre(&modifier, noun)
        ),
        "Restriction",
    );
    assert_ne!(keys(&[inside.clone(), outside]), keys(&[inside]));
}

#[test]
fn unreviewed_modifier_classification_is_tested_without_parser_count_claims() {
    for (id, noun) in [
        ("lexeme:type/artifact/non", true),
        ("lexeme:type/creature/non", true),
        ("lexeme:creature_subtype/goblin/non", true),
        ("lexeme:keyword_action/untap/adjective", false),
        ("lexeme:keyword_ability/fortify/adjective", false),
        ("lexeme:keyword_ability/kicker/adjective", false),
    ] {
        let modifier = synthetic_premodifier(id, noun);
        assert!(
            !Parser { input: &modifier }
                .tree()
                .unwrap()
                .commuting_pre(noun),
            "{id}"
        );
    }
    for constructor in ["Coordination", "SelectedPredicate", "QuotedNominal"] {
        let tree = format!(
            "{constructor} {{ form: 0, head: {} }}",
            synthetic_word("lexeme:creature_subtype/goblin")
        );
        assert!(!Parser { input: &tree }.tree().unwrap().commuting_pre(true));
        assert!(!Parser { input: &tree }.tree().unwrap().commuting_pre(false));
    }
}

#[test]
fn a_derived_card_type_id_does_not_extend_the_reviewed_inventory() {
    for (text, id) in [
        (
            "nonartifact creatures you control",
            "lexeme:type/artifact/non",
        ),
        (
            "noncreature creatures you control",
            "lexeme:type/creature/non",
        ),
    ] {
        let values = readings(text, Category::NounPhrase);
        assert_ne!(values, Vec::<Reading>::new());
        for value in values {
            assert!(format!("{value:?}").contains(id));
        }
        let modifier = synthetic_premodifier(id, true);
        assert!(
            !Parser { input: &modifier }
                .tree()
                .unwrap()
                .commuting_pre(true)
        );
    }
}

case!(
    reviewed_other_commutes_with_relatives,
    "other red creatures you control",
    NounPhrase,
    1,
    1
);
case!(
    judgment_30,
    "This creature gets +1/+1 for each other creature you control.",
    Sentence,
    1,
    1
);
case!(
    each_does_not_prevent_attributive_other_from_commuting,
    "each other red creature you control",
    NounPhrase,
    1,
    1
);

#[test]
fn other_classification_excludes_non_simple_uses_and_distinct_lexemes() {
    let id = "vocab:AttributiveAdjective/Other";
    assert!(
        Parser {
            input: &synthetic_premodifier(id, false)
                .replace("Adjective {", "IntransitiveAdjective {")
        }
        .tree()
        .unwrap()
        .commuting_pre(false)
    );
    let wrong_shape = format!(
        "IntransitiveAdjective {{ form: 0, head: {}, complement: Than }}",
        synthetic_word(id)
    );
    assert!(
        !Parser {
            input: &wrong_shape
        }
        .tree()
        .unwrap()
        .commuting_pre(false)
    );
    for constructor in ["ComplementedAdjective", "Coordination", "Adjective"] {
        let text = format!("{constructor} {{ form: 0, head: {} }}", synthetic_word(id));
        assert!(
            !Parser { input: &text }.tree().unwrap().commuting_pre(false),
            "{constructor}"
        );
    }
    for id in [
        "vocab:Determinative/Another",
        "lexeme:CommonNoun/Other",
        "vocab:AttributiveAdjective/Same",
    ] {
        let text = format!(
            "IntransitiveAdjective {{ form: 0, head: {} }}",
            synthetic_word(id)
        );
        assert!(
            !Parser { input: &text }.tree().unwrap().commuting_pre(false),
            "{id}"
        );
    }
}

#[test]
fn judgment_24_keeps_the_adjectival_state_and_verbal_passive() {
    use deckmaste_lexical::LexicalReading;
    let values = readings("This creature can't be blocked.", Category::Sentence);
    assert_eq!(values.len(), 2);
    let mut kinds = BTreeSet::new();
    for reading in &values {
        let mut state = false;
        let mut passive = false;
        reading.visit(&mut |node| match node {
            Reading::IntransitiveAdjective { head, .. }
                if matches!(&head.value, LexicalReading::Word(word) if word.lexeme.as_str() == "core-verb:Block/adjective") => state = true,
            Reading::PassivePredicate { .. } => passive = true,
            _ => {}
        }).unwrap();
        kinds.insert((state, passive));
    }
    assert_eq!(kinds, BTreeSet::from([(true, false), (false, true)]));
    assert_eq!(
        values
            .iter()
            .map(|r| fingerprint(r).unwrap())
            .collect::<BTreeSet<_>>()
            .len(),
        2
    );
}
