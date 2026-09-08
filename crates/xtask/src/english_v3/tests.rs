use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::OnceLock;

use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Lexicon;
use serde_json::json;

use crate::english_v3::EnglishV3Args;
use crate::english_v3::analyze_cards;
use crate::english_v3::report::Census;
use crate::english_v3::report::Enumeration;
use crate::english_v3::report::Measurements;
use crate::english_v3::report::Report;
use crate::english_v3::validation::Issue;
use crate::english_v3::validation::Tracing;
use crate::english_v3::validation::validate;
use crate::english_v3::write_report;

fn lexicon() -> &'static Lexicon {
    static LEXICON: OnceLock<Lexicon> = OnceLock::new();
    LEXICON.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        Lexicon::new(
            deckmaste_lexical_source::load_workspace(&root)
                .unwrap()
                .lexemes,
        )
        .unwrap()
    })
}

fn args(limit: Option<usize>) -> EnglishV3Args {
    EnglishV3Args {
        data: "unused.json".into(),
        output: "unused-report.json".into(),
        reading_limit: limit.map(|n| NonZeroUsize::new(n).unwrap()),
        samples_per_face: 2,
        workers: NonZeroUsize::new(2).unwrap(),
    }
}

fn card(text: Option<&str>, legality: &str) -> serde_json::Value {
    json!({"name":"Fixture", "types":[], "supertypes":[], "subtypes":[],
        "layout":"normal", "legalities":{"vintage":legality}, "text":text})
}

#[test]
fn raw_face_census_preserves_support_identity_and_unknown_byte_ranges() {
    let bytes = serde_json::to_vec(&json!({"data": {
        "A": [card(Some("Draw cards."), "Legal"), card(Some("Draw cards."), "Restricted")],
        "B": [card(Some("Creatures attacks."), "Legal")],
        "C": [card(Some("Draw cards. (éphantom)"), "Legal")],
        "D": [card(None, "Legal"), card(Some(""), "Legal")],
        "Excluded": [card(Some("quuxblorf."), "NotLegal")]
    }}))
    .unwrap();
    let cards = AtomicCards::parse(&bytes).unwrap();
    let faces = analyze_cards(&cards, lexicon(), &Grammar::default(), &args(None)).unwrap();
    assert_eq!(
        faces
            .iter()
            .map(|f| (f.group_name.as_str(), f.face_index))
            .collect::<Vec<_>>(),
        [("A", 0), ("A", 1), ("B", 0), ("C", 0), ("D", 0), ("D", 1)]
    );
    assert_ne!(faces[0].id, faces[1].id);
    assert_eq!(faces[0].raw_text_sha256, faces[1].raw_text_sha256);
    assert_eq!(
        faces.iter().map(|f| f.census).collect::<Vec<_>>(),
        [
            Census::One,
            Census::One,
            Census::No,
            Census::No,
            Census::One,
            Census::One
        ]
    );
    assert!(faces[2].unknown_words.is_empty());
    assert_eq!(faces[3].raw_text.as_deref(), Some("Draw cards. (éphantom)"));
    let unknown = &faces[3].unknown_words;
    assert_eq!(unknown.len(), 1);
    assert_eq!(
        (&*unknown[0].text, unknown[0].start, unknown[0].end),
        ("éphantom", 13, 22)
    );
    assert_eq!(faces[4].raw_text, None);
    assert_eq!(faces[5].raw_text.as_deref(), Some(""));
    assert_ne!(faces[4].id, faces[5].id);
    for face in faces {
        assert_eq!(face.enumeration, Enumeration::Complete);
        assert_eq!(face.exact_readings, Some(face.checked_readings));
        assert!(face.issues.is_empty(), "{:?}", face.issues);
        assert_eq!(face.chart.len(), 10);
        assert_eq!(face.readings.len(), 7);
    }
}

#[test]
fn capped_enumeration_never_claims_uniqueness_or_an_exact_total() {
    let bytes = serde_json::to_vec(&json!({"data": {
        "Ambiguous": [card(Some("You draw cards."), "Legal")],
        "Unique": [card(Some("Draw cards."), "Legal")]
    }}))
    .unwrap();
    let cards = AtomicCards::parse(&bytes).unwrap();
    let grammar = Grammar::default();
    let one = analyze_cards(&cards, lexicon(), &grammar, &args(Some(1))).unwrap();
    for face in &one {
        assert_eq!(face.checked_readings, 1);
        assert_eq!(face.exact_readings, None);
        assert_eq!(face.enumeration, Enumeration::Limited);
        assert_eq!(face.census, Census::Undetermined);
        assert_eq!(face.readings["requests"], 1);
    }
    let two = analyze_cards(&cards, lexicon(), &grammar, &args(Some(2))).unwrap();
    assert_eq!(two[0].checked_readings, 2);
    assert_eq!(two[0].census, Census::Multiple);
    assert_eq!(two[0].enumeration, Enumeration::Limited);
    assert_eq!(two[0].exact_readings, None);
    assert_eq!(two[1].census, Census::One);
    assert_eq!(two[1].exact_readings, Some(1));
    let all = analyze_cards(&cards, lexicon(), &grammar, &args(None)).unwrap();
    assert_eq!(all[0].census, Census::Multiple);
    assert_eq!(all[0].enumeration, Enumeration::Complete);
    assert_eq!(all[0].exact_readings, Some(all[0].checked_readings));
    assert!(all.iter().all(|face| face.issues.is_empty()));
}

#[test]
fn validation_compares_exact_surface_and_both_traversal_identities() {
    let grammar = Grammar::default();
    let input = lexicon().analyze("Draw cards.");
    let forest = parse(&grammar, lexicon(), &input, &Category::Document).unwrap();
    let traced = forest.readings(Tracing(&grammar)).next().unwrap().unwrap();
    assert!(validate(&traced, "Draw cards.", lexicon()).is_ok());
    assert_eq!(
        validate(&traced, "draw cards.", lexicon()),
        Err(Issue::Roundtrip {
            realized: "Draw cards.".into()
        })
    );
    let mut wrong_nodes = traced.clone();
    wrong_nodes.nodes.swap(0, 1);
    assert_eq!(
        validate(&wrong_nodes, "Draw cards.", lexicon()),
        Err(Issue::ConstructionTraversal)
    );
    let mut wrong_words = traced.clone();
    wrong_words.words.swap(0, 1);
    assert_eq!(
        validate(&wrong_words, "Draw cards.", lexicon()),
        Err(Issue::LexicalTraversal)
    );
}

#[test]
fn a_failed_validation_is_written_before_the_command_returns_an_error() {
    let bytes =
        serde_json::to_vec(&json!({"data":{"A":[card(Some("Draw cards."), "Legal")]}})).unwrap();
    let cards = AtomicCards::parse(&bytes).unwrap();
    let mut args = args(None);
    let directory = tempfile::tempdir().unwrap();
    args.output = directory.path().join("report.json");
    let mut faces = analyze_cards(&cards, lexicon(), &Grammar::default(), &args).unwrap();
    faces[0].issues.push(Issue::ConstructionTraversal);
    let report = Report::new(
        &args,
        &bytes,
        "test".into(),
        vec![],
        faces,
        Measurements {
            setup_wall_ns: 0,
            corpus_wall_ns: 0,
            host_load: None,
        },
    );
    let mut output = Vec::new();
    assert!(write_report(&args, &report, &mut output).is_err());
    let saved: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&args.output).unwrap()).unwrap();
    assert_eq!(saved["totals"]["issues"], 1);
    assert_eq!(
        saved["faces"][0]["issues"][0]["kind"],
        "construction_traversal"
    );
    assert!(
        saved["validation_scope"]
            .as_str()
            .unwrap()
            .contains("NOT checked")
    );
}
