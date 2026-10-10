use std::collections::BTreeSet;
use std::io::Write as _;
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::RealizationContext;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Lexicon;
use serde_json::json;

use crate::english_v3::EnglishV3Args;
use crate::english_v3::SourceField;
use crate::english_v3::analyze_cards;
use crate::english_v3::report::Census;
use crate::english_v3::report::Enumeration;
use crate::english_v3::report::Measurements;
use crate::english_v3::report::Report;
use crate::english_v3::validation::Issue;
use crate::english_v3::validation::TracedValue;
use crate::english_v3::validation::Tracing;
use crate::english_v3::validation::validate;
use crate::english_v3::validation::validate_in_context;
use crate::english_v3::write_report;
use crate::raw_corpus::CorpusSelectionArgs;
use crate::raw_corpus::SelectedCorpus;
use crate::raw_corpus::SelectionRequest;
use crate::raw_corpus::load_selected;

pub(super) fn lexicon() -> &'static Lexicon {
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

pub(super) fn args(limit: Option<usize>) -> EnglishV3Args {
    EnglishV3Args {
        command: None,
        field: SourceField::Text,
        data: "unused.json".into(),
        selection: CorpusSelectionArgs::default(),
        output: Some("unused-report.json".into()),
        incremental_from: None,
        reading_limit: limit.map(|n| NonZeroUsize::new(n).unwrap()),
        samples_per_face: 2,
        workers: NonZeroUsize::new(2).unwrap(),
    }
}

fn card(text: Option<&str>, legality: &str) -> serde_json::Value {
    json!({"name":"Fixture", "types":[], "supertypes":[], "subtypes":[],
        "layout":"normal", "legalities":{"vintage":legality}, "text":text})
}

pub(super) fn selected_corpus(legacy_fixture: &[u8]) -> SelectedCorpus {
    let value: serde_json::Value = serde_json::from_slice(legacy_fixture).unwrap();
    let mut records = Vec::new();
    for (group, faces) in value["data"].as_object().unwrap() {
        let faces = faces.as_array().unwrap();
        let first = &faces[0];
        let vintage = match first["legalities"]["vintage"].as_str() {
            Some("Legal") => "legal",
            Some("Restricted") => "restricted",
            _ => "not_legal",
        };
        let mut record = json!({
            "object": "card",
            "id": format!("printing-{group}"),
            "oracle_id": format!("oracle-{group}"),
            "name": group,
            "layout": first["layout"],
            "legalities": {"vintage": vintage},
            "color_identity": []
        });
        if faces.len() == 1 {
            copy_source_fields(&mut record, first);
        } else {
            record["card_faces"] = serde_json::Value::Array(
                faces
                    .iter()
                    .enumerate()
                    .map(|(index, face)| {
                        let mut projected = json!({
                            "name": face["faceName"]
                                .as_str()
                                .map_or_else(|| format!("{group} {index}"), str::to_owned)
                        });
                        copy_source_fields(&mut projected, face);
                        projected
                    })
                    .collect(),
            );
        }
        records.push(serde_json::to_string(&record).unwrap());
    }
    let mut input = tempfile::NamedTempFile::new().unwrap();
    writeln!(input, "{}", records.join("\n")).unwrap();
    load_selected(
        input.path(),
        &SelectionRequest {
            all: true,
            ..SelectionRequest::default()
        },
        None,
    )
    .unwrap()
}

fn copy_source_fields(target: &mut serde_json::Value, source: &serde_json::Value) {
    for (from, to) in [
        ("manaCost", "mana_cost"),
        ("manaValue", "cmc"),
        ("type", "type_line"),
        ("text", "oracle_text"),
        ("power", "power"),
        ("toughness", "toughness"),
        ("loyalty", "loyalty"),
        ("defense", "defense"),
        ("colors", "colors"),
    ] {
        if let Some(value) = source.get(from) {
            target[to] = value.clone();
        }
    }
}

#[test]
fn face_census_preserves_raw_text_and_strips_reminders_before_parsing() {
    // Reach Through Mists supplies the positive Oracle sentence; the added
    // reminder is a synthetic normalization probe, not a grammar witness.
    let bytes = serde_json::to_vec(&json!({"data": {
        "A": [card(Some("Draw a card."), "Legal"), card(Some("Draw a card."), "Restricted")],
        "B": [card(Some("Creatures attacks."), "Legal")],
        "C": [card(Some("Draw a card. (éphantom)"), "Legal")],
        "D": [card(None, "Legal"), card(Some(""), "Legal")],
        "Excluded": [card(Some("quuxblorf."), "NotLegal")]
    }}))
    .unwrap();
    let corpus = selected_corpus(&bytes);
    let faces = analyze_cards(
        &corpus.faces,
        lexicon(),
        &Grammar::with_lexicon(lexicon()),
        &args(None),
    )
    .unwrap();
    assert_eq!(
        faces
            .iter()
            .map(|f| (f.group_name.as_str(), f.face_index))
            .collect::<Vec<_>>(),
        [
            ("A", Some(0)),
            ("A", Some(1)),
            ("B", None),
            ("C", None),
            ("D", Some(0)),
            ("D", Some(1))
        ]
    );
    assert_ne!(faces[0].id, faces[1].id);
    assert_eq!(faces[0].raw_text_sha256, faces[1].raw_text_sha256);
    assert_eq!(
        faces.iter().map(|f| f.census).collect::<Vec<_>>(),
        [
            Census::One,
            Census::One,
            Census::No,
            Census::One,
            Census::One,
            Census::One
        ]
    );
    assert!(faces[2].unknown_words.is_empty());
    assert_eq!(
        faces[3].raw_text.as_deref(),
        Some("Draw a card. (éphantom)")
    );
    assert_eq!(faces[3].analyzed_source, "Draw a card.");
    assert_eq!(faces[3].source_sha256, faces[3].raw_text_sha256);
    assert_eq!(faces[3].analyzed_source_sha256, faces[0].source_sha256);
    assert!(faces[3].unknown_words.is_empty());
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
    // Authentic complete Oracle texts: Deep Sight is ambiguous in the current
    // grammar; Reach Through Mists and Revitalize each have one Reading.
    let bytes = serde_json::to_vec(&json!({"data": {
        "Deep Sight": [card(Some("You draw a card and gain 1 life."), "Legal")],
        "Reach Through Mists": [card(Some("Draw a card."), "Legal")],
        "Revitalize": [card(Some("You gain 3 life.\nDraw a card."), "Legal")]
    }}))
    .unwrap();
    let corpus = selected_corpus(&bytes);
    let grammar = Grammar::with_lexicon(lexicon());
    let one = analyze_cards(&corpus.faces, lexicon(), &grammar, &args(Some(1))).unwrap();
    for (face, expected) in one.iter().zip([1, 1, 2]) {
        assert_eq!(face.checked_readings, expected);
        assert_eq!(face.exact_readings, None);
        assert_eq!(face.enumeration, Enumeration::Limited);
        assert_eq!(face.census, Census::Undetermined);
        assert_eq!(face.readings["requests"], expected);
    }
    let two = analyze_cards(&corpus.faces, lexicon(), &grammar, &args(Some(2))).unwrap();
    assert_eq!(two[0].checked_readings, 2);
    assert_eq!(two[0].census, Census::Multiple);
    assert_eq!(two[0].enumeration, Enumeration::Limited);
    assert_eq!(two[0].exact_readings, None);
    for (face, expected) in two[1..].iter().zip([1, 2]) {
        assert_eq!(face.census, Census::One);
        assert_eq!(face.exact_readings, Some(expected));
    }
    let all = analyze_cards(&corpus.faces, lexicon(), &grammar, &args(None)).unwrap();
    assert_eq!(all[0].census, Census::Multiple);
    assert_eq!(all[0].enumeration, Enumeration::Complete);
    assert_eq!(all[0].exact_readings, Some(all[0].checked_readings));
    for (face, expected) in all[1..].iter().zip([1, 2]) {
        assert_eq!(face.census, Census::One);
        assert_eq!(face.enumeration, Enumeration::Complete);
        assert_eq!(face.exact_readings, Some(expected));
    }
    assert!(all.iter().all(|face| face.issues.is_empty()));
}

#[test]
fn worker_count_does_not_change_chart_results() {
    // Reach Through Mists and the first sentence of Revitalize.
    let bytes = serde_json::to_vec(&json!({"data": {
        "A": [card(Some("Draw a card."), "Legal")],
        "B": [card(Some("You gain 3 life."), "Restricted")]
    }}))
    .unwrap();
    let corpus = selected_corpus(&bytes);
    let grammar = Grammar::with_lexicon(lexicon());
    let mut one_args = args(None);
    one_args.workers = NonZeroUsize::new(1).unwrap();
    let mut many_args = args(None);
    many_args.workers = NonZeroUsize::new(3).unwrap();

    let mut one =
        serde_json::to_value(analyze_cards(&corpus.faces, lexicon(), &grammar, &one_args).unwrap())
            .unwrap();
    let mut many = serde_json::to_value(
        analyze_cards(&corpus.faces, lexicon(), &grammar, &many_args).unwrap(),
    )
    .unwrap();
    for faces in [&mut one, &mut many] {
        for face in faces.as_array_mut().unwrap() {
            let face = face.as_object_mut().unwrap();
            face.remove("lexical_wall_ns");
            face.remove("chart_wall_ns");
            face.remove("validation_wall_ns");
            face.remove("thread_cpu_ns");
        }
    }
    assert_eq!(one, many);
}

#[test]
fn validation_compares_exact_surface_and_both_traversal_identities() {
    let grammar = Grammar::with_lexicon(lexicon());
    let input = lexicon().analyze("Draw a card.");
    let forest = parse(&grammar, lexicon(), &input, &Category::Document).unwrap();
    let traced = forest
        .readings(Tracing::new(&grammar))
        .next()
        .unwrap()
        .unwrap();
    assert!(
        validate(
            &traced,
            "Draw a card.",
            lexicon(),
            Category::Document,
            &Grammar::with_lexicon(lexicon())
        )
        .is_ok()
    );
    assert_eq!(
        validate(
            &traced,
            "draw a card.",
            lexicon(),
            Category::Document,
            &Grammar::with_lexicon(lexicon())
        ),
        Err(Issue::Roundtrip {
            realized: "Draw a card.".into()
        })
    );
    let mut wrong_nodes = traced.clone();
    std::sync::Arc::make_mut(&mut wrong_nodes.nodes).swap(0, 1);
    assert_eq!(
        validate(
            &wrong_nodes,
            "Draw a card.",
            lexicon(),
            Category::Document,
            &Grammar::with_lexicon(lexicon())
        ),
        Err(Issue::ConstructionTraversal)
    );
    let mut wrong_words = traced.clone();
    std::sync::Arc::make_mut(&mut wrong_words.words).swap(0, 1);
    assert_eq!(
        validate(
            &wrong_words,
            "Draw a card.",
            lexicon(),
            Category::Document,
            &Grammar::with_lexicon(lexicon())
        ),
        Err(Issue::LexicalTraversal)
    );
}

#[test]
fn a_failed_validation_is_written_before_the_command_returns_an_error() {
    let bytes =
        serde_json::to_vec(&json!({"data":{"A":[card(Some("Draw a card."), "Legal")]}})).unwrap();
    let corpus = selected_corpus(&bytes);
    let mut args = args(None);
    let directory = tempfile::tempdir().unwrap();
    args.output = Some(directory.path().join("report.json"));
    let mut faces = analyze_cards(
        &corpus.faces,
        lexicon(),
        &Grammar::with_lexicon(lexicon()),
        &args,
    )
    .unwrap();
    faces[0].issues.push(Issue::ConstructionTraversal);
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
    let mut output = Vec::new();
    assert!(write_report(&args, &report, &mut output).is_err());
    let saved: serde_json::Value =
        serde_json::from_slice(&std::fs::read(args.output.as_ref().unwrap()).unwrap()).unwrap();
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

#[test]
fn type_line_census_uses_its_own_source_and_root_without_relabeling_rules_text() {
    // Memnarch supplies the authentic positive type line.
    let mut valid = card(Some("Unknownword."), "Legal");
    valid["type"] = json!("Legendary Artifact Creature — Wizard");
    let mut invalid = card(Some("Draw a card."), "Legal");
    invalid["type"] = json!("Creature Legendary");
    let bytes = serde_json::to_vec(&json!({"data": {
        "A": [valid], "B": [invalid], "C": [card(None, "Legal")]
    }}))
    .unwrap();
    let corpus = selected_corpus(&bytes);
    let grammar = Grammar::with_lexicon(lexicon());
    let mut args = args(None);
    let text = analyze_cards(&corpus.faces, lexicon(), &grammar, &args).unwrap();
    args.field = SourceField::TypeLine;
    let faces = analyze_cards(&corpus.faces, lexicon(), &grammar, &args).unwrap();
    assert_eq!(
        faces.iter().map(|face| face.census).collect::<Vec<_>>(),
        [Census::One, Census::No, Census::No]
    );
    assert_eq!(faces[0].raw_text.as_deref(), Some("Unknownword."));
    assert_eq!(
        faces[0].type_line.as_deref(),
        Some("Legendary Artifact Creature — Wizard")
    );
    assert_eq!(faces[0].id, text[0].id);
    assert_eq!(faces[0].raw_text_sha256, text[0].raw_text_sha256);
    assert_eq!(
        faces[0].source_sha256,
        crate::raw_corpus::digest("Legendary Artifact Creature — Wizard".as_bytes())
    );
    assert!(
        faces
            .iter()
            .all(|face| face.issues.is_empty() && face.unknown_words.is_empty())
    );
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
    let saved = serde_json::to_value(report).unwrap();
    assert_eq!(saved["field"], "type_line");
    assert_eq!(
        saved["totals"]["source_bytes"],
        "Legendary Artifact Creature — Wizard".len() + "Creature Legendary".len()
    );
    assert_eq!(saved["totals"]["supported_faces_without_source"], 1);
    let input = lexicon().analyze("Instant");
    let forest = parse(&grammar, lexicon(), &input, &Category::TypeLine).unwrap();
    let traced = forest
        .readings(Tracing::new(&grammar))
        .next()
        .unwrap()
        .unwrap();
    assert!(
        validate(
            &traced,
            "Instant",
            lexicon(),
            Category::TypeLine,
            &Grammar::with_lexicon(lexicon())
        )
        .is_ok()
    );
    assert_eq!(
        validate(
            &traced,
            "Instant",
            lexicon(),
            Category::Document,
            &Grammar::with_lexicon(lexicon())
        ),
        Err(Issue::RootCategory)
    );
}

#[test]
fn unknown_word_offsets_and_roundtrips_use_the_stripped_source() {
    let bytes = serde_json::to_vec(&json!({"data": {
        "A": [card(Some("(ignored éphantom) Draw a card. éphantom."), "Legal")]
    }}))
    .unwrap();
    let corpus = selected_corpus(&bytes);
    let faces = analyze_cards(
        &corpus.faces,
        lexicon(),
        &Grammar::with_lexicon(lexicon()),
        &args(None),
    )
    .unwrap();
    let face = &faces[0];
    assert_eq!(face.analyzed_source, " Draw a card. éphantom.");
    let unknown = &face.unknown_words;
    assert_eq!(unknown.len(), 1);
    assert_eq!(
        (&*unknown[0].text, unknown[0].start, unknown[0].end),
        ("éphantom", 14, 23)
    );
    assert_eq!(
        &face.analyzed_source[unknown[0].start..unknown[0].end],
        "éphantom"
    );
    assert_eq!(face.issues, [] as [crate::english_v3::validation::Issue; 0]);
}

#[test]
fn cheapest_frequency_sample_preserves_all_readings_in_census() {
    let bytes = serde_json::to_vec(&json!({"data": {
        "Jadelight Spelunker": [card(Some("When this creature enters, it explores X times."), "Legal")]
    }})).unwrap();
    let corpus = selected_corpus(&bytes);
    let mut options = args(None);
    options.samples_per_face = 1;
    let faces = analyze_cards(
        &corpus.faces,
        lexicon(),
        &Grammar::with_lexicon(lexicon()),
        &options,
    )
    .unwrap();
    let face = &faces[0];
    assert!(face.issues.is_empty(), "{:?}", face.issues);
    assert_eq!(face.census, Census::Multiple);
    assert_eq!(face.exact_readings, Some(2));
    let serialized = serde_json::to_value(face).unwrap();
    assert_eq!(serialized["samples"].as_array().unwrap().len(), 1);
    let tree = serialized["samples"][0]["tree"].as_str().unwrap();
    let frequency = tree.split_once("FrequencyPredicate {").unwrap().1;
    assert!(
        frequency
            .lines()
            .take(4)
            .any(|line| line.trim() == "category: FinitePredicate,")
    );
    assert_eq!(serialized["samples"][0]["total_cost"], 19);
}

fn trace_card_draw(grammar: &Grammar) -> TracedValue {
    let input = lexicon().analyze("Draw a card.");
    let forest = parse(grammar, lexicon(), &input, &Category::Document).unwrap();
    let mut values = forest.readings(Tracing::new(grammar));
    values.next().unwrap().unwrap()
}

#[test]
fn census_validation_reports_a_one_byte_surface_mismatch() {
    let grammar = Grammar::with_lexicon(lexicon());
    let traced = trace_card_draw(&grammar);
    let source = "draw a card.";
    let context = RealizationContext::new(&grammar, lexicon(), source);
    assert_eq!(source.len(), "Draw a card.".len());
    assert_eq!(
        source
            .bytes()
            .zip("Draw a card.".bytes())
            .filter(|(actual, expected)| actual != expected)
            .count(),
        1
    );
    let result = validate_in_context(&traced, source, Category::Document, &context);
    assert_eq!(
        result,
        Err(Issue::Roundtrip {
            realized: "Draw a card.".into()
        })
    );
    assert_eq!(
        result,
        validate(&traced, source, lexicon(), Category::Document, &grammar)
    );
}

#[test]
fn census_validation_reports_a_wrong_lexical_identity_at_one_leaf() {
    let grammar = Grammar::with_lexicon(lexicon());
    let traced = trace_card_draw(&grammar);
    let source = "Draw a card.";
    let context = RealizationContext::new(&grammar, lexicon(), source);
    let expected = validate(&traced, source, lexicon(), Category::Document, &grammar).unwrap();
    assert_eq!(
        validate_in_context(&traced, source, Category::Document, &context),
        Ok(expected)
    );

    let mut wrong_words = traced.clone();
    // Change one independent lexical trace leaf, preserving the Reading and
    // construction trace so the fault reaches the word-traversal check.
    let wrong_leaf = Arc::make_mut(&mut wrong_words.words).last_mut().unwrap();
    wrong_leaf.countability = Some(false);
    assert_eq!(wrong_words.value, traced.value);
    assert_eq!(wrong_words.nodes, traced.nodes);
    assert_ne!(wrong_words.words.last(), traced.words.last());
    let unchanged_leaves = traced.words.len() - 1;
    assert_eq!(
        wrong_words.words[..unchanged_leaves],
        traced.words[..unchanged_leaves]
    );
    let result = validate_in_context(&wrong_words, source, Category::Document, &context);
    assert_eq!(result, Err(Issue::LexicalTraversal));
    assert_eq!(
        result,
        validate(
            &wrong_words,
            source,
            lexicon(),
            Category::Document,
            &grammar
        )
    );
}

#[test]
fn census_validation_reports_a_duplicate_reading() {
    let grammar = Grammar::with_lexicon(lexicon());
    let traced = trace_card_draw(&grammar);
    let source = "Draw a card.";
    let context = RealizationContext::new(&grammar, lexicon(), source);
    let mut checked = BTreeSet::new();
    // Duplicate reporting belongs to the census caller, after validation.
    // Use analyze_face's exact fingerprint/full-value counting key.
    let mut count = |value: &TracedValue| -> Result<(), Issue> {
        validate_in_context(value, source, Category::Document, &context)?;
        if !checked.insert((value.identity().to_owned(), Arc::clone(&value.value))) {
            return Err(Issue::DuplicateReading);
        }
        Ok(())
    };
    assert_eq!(count(&traced), Ok(()));
    assert_eq!(count(&traced.clone()), Err(Issue::DuplicateReading));
    assert_eq!(checked.len(), 1);

    let reading = validate(&traced, source, lexicon(), Category::Document, &grammar)
        .unwrap()
        .clone();
    let mut old_checked = BTreeSet::new();
    assert!(old_checked.insert(reading.clone()));
    assert!(!old_checked.insert(reading));
    assert_eq!(checked.len(), old_checked.len());
}

#[test]
fn census_validation_rejects_cached_reading_for_different_source_text() {
    let grammar = Grammar::with_lexicon(lexicon());
    let traced = trace_card_draw(&grammar);
    let source = "Draw a card.";
    let context = RealizationContext::new(&grammar, lexicon(), source);
    let expected = validate(&traced, source, lexicon(), Category::Document, &grammar).unwrap();
    assert_eq!(
        validate_in_context(&traced, source, Category::Document, &context),
        Ok(expected)
    );

    let cached = traced.clone();
    assert!(Arc::ptr_eq(&cached.value, &traced.value));
    let different_source = "Draw two cards.";
    // Keep the warmed context: a fresh context would not exercise cached
    // declaration admission from the first successful validation.
    let result = validate_in_context(&cached, different_source, Category::Document, &context);
    assert_eq!(
        result,
        Err(Issue::Roundtrip {
            realized: source.into()
        })
    );
    assert_eq!(
        result,
        validate(
            &cached,
            different_source,
            lexicon(),
            Category::Document,
            &grammar
        )
    );
}

#[test]
fn cached_materializations_preserve_complete_values_and_independent_traces() {
    use std::collections::BTreeSet;
    use std::sync::Arc;

    use deckmaste_english_v3::grammar::RealizationContext;

    use crate::english_v3::validation::validate_in_context;

    let grammar = Grammar::with_lexicon(lexicon());
    for text in [
        // Seedborn Muse: three distinct PP attachments must stay correlated.
        "Untap all permanents you control during each other player's untap step.",
        // Deep Sight: predicate versus clause coordination.
        "You draw a card and gain 1 life.",
        // Revitalize: shared lexical values in separate sentence positions.
        "You gain 3 life.\nDraw a card.",
    ] {
        let input = lexicon().analyze(text);
        let forest = parse(&grammar, lexicon(), &input, &Category::Document).unwrap();
        let expected: BTreeSet<_> = forest.readings(&grammar).map(Result::unwrap).collect();
        let context = RealizationContext::new(&grammar, lexicon(), text);
        let mut values = forest.readings(Tracing::new(&grammar));
        let mut actual = BTreeSet::new();
        for value in values.by_ref() {
            let value = value.unwrap();
            assert_eq!(
                validate_in_context(&value, text, Category::Document, &context),
                validate(&value, text, lexicon(), Category::Document, &grammar),
            );
            assert!(actual.insert(Arc::clone(&value.value)));
        }
        assert_eq!(
            actual.iter().map(|value| &**value).collect::<Vec<_>>(),
            expected.iter().collect::<Vec<_>>()
        );
        assert_eq!(values.metrics().internal_failures, 0);
        let metrics = values.materializer().metrics();
        assert!(metrics.unique_builds <= metrics.build_requests);
        if expected.len() > 1 {
            assert!(metrics.unique_builds < metrics.build_requests);
        }
    }
}

#[test]
fn cached_validation_rejects_same_construction_with_different_lexical_identity() {
    use std::sync::Arc;

    use deckmaste_english_v3::grammar::RealizationContext;
    use deckmaste_english_v3::grammar::Value;

    use crate::english_v3::validation::validate_in_context;

    let grammar = Grammar::with_lexicon(lexicon());
    // Reach Through Mists supplies the sentence; changing just a lexical leaf
    // in its independent materialization trace is a validator fault probe.
    let text = "Draw a card.";
    let input = lexicon().analyze(text);
    let forest = parse(&grammar, lexicon(), &input, &Category::Document).unwrap();
    let mut traced = forest
        .readings(Tracing::new(&grammar))
        .next()
        .unwrap()
        .unwrap();
    let original_digest = traced.dedup_key();
    let expected = Arc::make_mut(&mut traced.nodes).last_mut().unwrap();
    let Value::Reading(mut node) = (**expected).clone() else { unreachable!() };
    assert_eq!(node.construction(), "Noun");
    let deckmaste_english_v3::grammar::Reading::Noun { head, .. } = &mut node else {
        unreachable!()
    };
    head.countability = Some(false);
    *expected = Arc::new(Value::Reading(node));
    assert_ne!(original_digest, traced.dedup_key());
    let context = RealizationContext::new(&grammar, lexicon(), text);
    assert_eq!(
        validate_in_context(&traced, text, Category::Document, &context),
        Err(Issue::ConstructionTraversal)
    );
}

#[test]
fn a_fingerprint_collision_keeps_distinct_complete_values() {
    use std::collections::BTreeSet;
    use std::sync::Arc;

    let grammar = Grammar::with_lexicon(lexicon());
    let mut values = Vec::new();
    // Reach Through Mists and Divination supply authentic distinct values.
    for text in ["Draw a card.", "Draw two cards."] {
        let input = lexicon().analyze(text);
        let forest = parse(&grammar, lexicon(), &input, &Category::Document).unwrap();
        values.push(
            forest
                .readings(Tracing::new(&grammar))
                .next()
                .unwrap()
                .unwrap(),
        );
    }
    let first = values.remove(0);
    let second = values.remove(0);
    let mut collision = first.clone();
    // Preserve the cached digest while injecting a different full value. This
    // simulates a collision; production materializations remain immutable.
    let _ = collision.identity();
    collision.value = Arc::clone(&second.value);
    assert_eq!(first.identity(), collision.identity());
    assert_ne!(first, collision);
    assert_eq!(BTreeSet::from([first, collision]).len(), 2);
}

#[test]
fn digest_dedup_and_cache_eviction_preserve_all_values_and_traces() {
    let grammar = Grammar::with_lexicon(lexicon());
    for text in [
        // Seedborn Muse, Deep Sight and Revitalize.
        "Untap all permanents you control during each other player's untap step.",
        "You draw a card and gain 1 life.",
        "You gain 3 life.\nDraw a card.",
    ] {
        let input = lexicon().analyze(text);
        let forest = parse(&grammar, lexicon(), &input, &Category::Document).unwrap();
        let mut expected: Vec<_> = forest
            .readings(Tracing::new(&grammar))
            .map(Result::unwrap)
            .collect();
        let context = RealizationContext::new(&grammar, lexicon(), text);
        let mut values = forest
            .readings(Tracing::new(&grammar).with_memo_cap(1))
            .with_digest_dedup(TracedValue::dedup_key);
        let mut actual: Vec<_> = values
            .by_ref()
            .map(|value| {
                let value = value.unwrap();
                validate_in_context(&value, text, Category::Document, &context).unwrap();
                value
            })
            .collect();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected, "{text}");
        assert_eq!(values.metrics().readings, expected.len());
        assert_eq!(values.metrics().internal_failures, 0);
        let metrics = values.materializer().metrics();
        assert_eq!(metrics.unique_builds, metrics.build_requests);
    }
}

fn combat_lexicon_before_landing() -> Lexicon {
    // Replay precisely the three feature assignments from the real
    // english-v3-combat-interval-noun landing. Category defaults supply None
    // before the explicit features are assigned; provenance is unchanged.
    let mut entries: Vec<_> = lexicon().lexemes().values().cloned().collect();
    for entry in &mut entries {
        if entry.id == "lexeme:turn_part/combat" {
            entry
                .properties
                .features
                .insert("NominalBareClass".into(), "None".into());
            entry
                .properties
                .features
                .insert("NominalAdjunctClass".into(), "None".into());
        }
        if entry.id == "vocab:Preposition/During" {
            entry
                .properties
                .features
                .insert("NominalBareClass".into(), "None".into());
        }
    }
    Lexicon::new(entries).unwrap()
}

fn combat_corpus() -> SelectedCorpus {
    selected_corpus(&serde_json::to_vec(&json!({"data": {
        "Dire Fleet Warmonger": [card(Some("At the beginning of combat on your turn, you may sacrifice another creature."), "Legal")],
        "Glyph of Destruction": [card(Some("Target blocking Wall you control gets +10/+0 until end of combat."), "Legal")],
        "Echo Circlet": [card(Some("Equipped creature can block an additional creature each combat."), "Legal")],
        "Basandra, Battle Seraph": [card(Some("Players can't cast spells during combat."), "Legal")],
        "Fog": [card(Some("Prevent all combat damage that would be dealt this turn."), "Legal")],
        "Deep Sight": [card(Some("You draw a card and gain 1 life."), "Legal")],
        "Reach Through Mists": [card(Some("Draw a card."), "Legal")]
    }})).unwrap())
}

#[test]
fn readings_digests_are_stable_and_change_only_on_affected_analyses() {
    let corpus = combat_corpus();
    let args = args(None);
    let grammar = Grammar::with_lexicon(lexicon());
    let first = analyze_cards(&corpus.faces, lexicon(), &grammar, &args).unwrap();
    let second = analyze_cards(&corpus.faces, lexicon(), &grammar, &args).unwrap();
    let digest_rows = |faces: &[crate::english_v3::report::FaceReport]| {
        faces
            .iter()
            .map(|face| (face.id.clone(), face.readings_sha256.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(digest_rows(&first), digest_rows(&second));
    let before_lexicon = combat_lexicon_before_landing();
    let before = analyze_cards(
        &corpus.faces,
        &before_lexicon,
        &Grammar::with_lexicon(&before_lexicon),
        &args,
    )
    .unwrap();
    let changed: Vec<_> = before
        .iter()
        .zip(&first)
        .filter(|(before, after)| before.readings_sha256 != after.readings_sha256)
        .map(|(_, after)| after.name.as_str())
        .collect();
    assert_eq!(
        changed,
        [
            "Basandra, Battle Seraph",
            "Dire Fleet Warmonger",
            "Echo Circlet",
            "Glyph of Destruction"
        ]
    );
    assert!(
        first
            .iter()
            .chain(&before)
            .all(|face| face.issues.is_empty() && face.readings_sha256.is_some())
    );
}

fn comparable_faces(faces: &[crate::english_v3::report::FaceReport]) -> serde_json::Value {
    let mut json = serde_json::to_value(faces).unwrap();
    for face in json.as_array_mut().unwrap() {
        for key in [
            "lexical_wall_ns",
            "chart_wall_ns",
            "validation_wall_ns",
            "thread_cpu_ns",
        ] {
            face.as_object_mut().unwrap().remove(key);
        }
    }
    json
}

fn incremental_snapshot(lexicon: &Lexicon, core: &str) -> super::incremental::Snapshot {
    super::incremental::Snapshot {
        sources: std::collections::BTreeMap::from([
            (
                "crates/deckmaste_english_v3/src/declarations.rs".into(),
                "same grammar".into(),
            ),
            (
                "crates/deckmaste_lexical_source/lexicon/core.ron".into(),
                core.into(),
            ),
        ]),
        lexemes: super::incremental::Snapshot::lexemes(lexicon).unwrap(),
    }
}

fn incremental_base(
    corpus: &SelectedCorpus,
    lexicon: &Lexicon,
    args: &EnglishV3Args,
    snapshot: super::incremental::Snapshot,
) -> super::incremental::Base {
    let faces = analyze_cards(
        &corpus.faces,
        lexicon,
        &Grammar::with_lexicon(lexicon),
        args,
    )
    .unwrap();
    super::incremental::Base {
        schema_version: super::report::SCHEMA_VERSION,
        readings_digest_version: super::report::READINGS_DIGEST_VERSION,
        field: args.field,
        input_sha256: corpus.snapshot_sha256.clone(),
        support_filter: crate::raw_corpus::SUPPORT_FILTER.into(),
        reading_limit: args.reading_limit.map(NonZeroUsize::get),
        samples_per_face: args.samples_per_face,
        incremental_snapshot: snapshot,
        faces,
    }
}

#[test]
fn incremental_combat_landing_equals_full_census_in_both_directions() {
    let corpus = combat_corpus();
    let before_lexicon = combat_lexicon_before_landing();
    let before_snapshot = incremental_snapshot(&before_lexicon, "before combat landing");
    let after_snapshot = incremental_snapshot(lexicon(), "after combat landing");
    let mut args = args(None);
    args.incremental_from = Some("target/english-v3/base.json".into());
    for (old, current, old_snapshot, current_snapshot) in [
        (
            &before_lexicon,
            lexicon(),
            &before_snapshot,
            &after_snapshot,
        ),
        (
            lexicon(),
            &before_lexicon,
            &after_snapshot,
            &before_snapshot,
        ),
    ] {
        let base = incremental_base(&corpus, old, &args, old_snapshot.clone());
        let grammar = Grammar::with_lexicon(current);
        let (incremental, mode) = super::incremental::analyze(
            &corpus,
            current,
            &grammar,
            &args,
            current_snapshot,
            Some(base),
        )
        .unwrap();
        let full = analyze_cards(&corpus.faces, current, &grammar, &args).unwrap();
        assert_eq!(comparable_faces(&incremental), comparable_faces(&full));
        assert!(mode.used);
        assert_eq!(
            mode.affected_lexemes,
            ["lexeme:turn_part/combat", "vocab:Preposition/During"]
        );
        assert_eq!(mode.rechecked_faces.len(), 5);
        let reused: Vec<_> = incremental
            .iter()
            .filter(|face| mode.reused_faces.contains(&face.id))
            .map(|face| face.name.as_str())
            .collect();
        assert_eq!(reused, ["Deep Sight", "Reach Through Mists"]);
    }
}

#[test]
fn grammar_policy_and_incomplete_base_changes_disable_incremental_reuse() {
    let corpus = combat_corpus();
    let snapshot = incremental_snapshot(lexicon(), "same lexicon");
    let mut args = args(None);
    args.incremental_from = Some("target/english-v3/base.json".into());
    let grammar = Grammar::with_lexicon(lexicon());
    let expected = analyze_cards(&corpus.faces, lexicon(), &grammar, &args).unwrap();
    for scenario in [
        "grammar", "input", "samples", "limited", "missing", "invalid",
    ] {
        let mut base = incremental_base(&corpus, lexicon(), &args, snapshot.clone());
        match scenario {
            "grammar" => {
                base.incremental_snapshot.sources.insert(
                    "crates/deckmaste_english_v3/src/declarations.rs".into(),
                    "old grammar".into(),
                );
            }
            "input" => base.input_sha256 = "different snapshot".into(),
            "samples" => base.samples_per_face += 1,
            "limited" => base.faces[0].enumeration = Enumeration::Limited,
            "missing" => {
                base.faces.pop();
            }
            "invalid" => base.faces[0].issues.push(Issue::ConstructionTraversal),
            _ => unreachable!(),
        }
        let (actual, mode) =
            super::incremental::analyze(&corpus, lexicon(), &grammar, &args, &snapshot, Some(base))
                .unwrap();
        assert!(!mode.used, "{scenario}");
        assert!(mode.fallback_reason.is_some(), "{scenario}");
        assert!(mode.reused_faces.is_empty(), "{scenario}");
        assert_eq!(
            comparable_faces(&actual),
            comparable_faces(&expected),
            "{scenario}"
        );
    }
}

#[test]
fn base_json_roundtrip_and_old_schema_fallback_preserve_the_face_digests() {
    let corpus = combat_corpus();
    let mut args = args(None);
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/english-v3/census-efficiency/tests");
    std::fs::create_dir_all(&root).unwrap();
    let directory = tempfile::tempdir_in(root).unwrap();
    args.output = Some(directory.path().join("base.json"));
    let grammar = Grammar::with_lexicon(lexicon());
    let faces = analyze_cards(&corpus.faces, lexicon(), &grammar, &args).unwrap();
    let expected = comparable_faces(&faces);
    let snapshot = incremental_snapshot(lexicon(), "same lexicon");
    let mut report = Report::new(
        &args,
        &corpus,
        "inventory".into(),
        vec![],
        faces,
        Measurements {
            setup_wall_ns: 0,
            corpus_wall_ns: 0,
            host_load: None,
        },
    );
    report.incremental_snapshot = Some(snapshot.clone());
    write_report(&args, &report, &mut Vec::new()).unwrap();
    args.incremental_from = args.output.clone();
    let base = super::incremental::Base::load(args.output.as_ref().unwrap()).unwrap();
    let (copied, mode) =
        super::incremental::analyze(&corpus, lexicon(), &grammar, &args, &snapshot, base).unwrap();
    assert!(mode.used);
    assert_eq!(mode.reused_faces.len(), corpus.faces.len());
    assert_eq!(mode.rechecked_faces, Vec::<String>::new());
    assert_eq!(comparable_faces(&copied), expected);
    std::fs::write(args.output.as_ref().unwrap(), "{\"schema_version\":7}").unwrap();
    let old = super::incremental::Base::load(args.output.as_ref().unwrap()).unwrap();
    let (full, mode) =
        super::incremental::analyze(&corpus, lexicon(), &grammar, &args, &snapshot, old).unwrap();
    assert!(!mode.used);
    assert!(mode.fallback_reason.is_some());
    assert_eq!(comparable_faces(&full), expected);
}

#[test]
fn removed_irregular_forms_recheck_faces_that_have_no_remaining_form() {
    // Barbarian Class's first sentence, without its reminder text.
    let text = "If you would roll one or more dice, instead roll that many dice plus one and ignore the lowest roll.";
    let corpus = selected_corpus(
        &serde_json::to_vec(&json!({"data": {
            "Barbarian Class": [card(Some(text), "Legal")],
            "Reach Through Mists": [card(Some("Draw a card."), "Legal")]
        }}))
        .unwrap(),
    );
    let mut entries: Vec<_> = lexicon().lexemes().values().cloned().collect();
    entries
        .iter_mut()
        .find(|entry| entry.id == "lexeme:CommonNoun/Die")
        .unwrap()
        .forms
        .retain(|slot| slot.form != deckmaste_lexical::WordForm::Plural);
    let current = Lexicon::new(entries).unwrap();
    let old_snapshot = incremental_snapshot(lexicon(), "before irregular form removal");
    let snapshot = incremental_snapshot(&current, "after irregular form removal");
    assert!(
        snapshot.lexemes["lexeme:CommonNoun/Die"]
            .surfaces
            .iter()
            .all(|surface| !text.to_lowercase().contains(&surface.to_lowercase()))
    );
    let mut args = args(None);
    args.incremental_from = Some("target/english-v3/base.json".into());
    let base = incremental_base(&corpus, lexicon(), &args, old_snapshot);
    let grammar = Grammar::with_lexicon(&current);
    let (actual, mode) =
        super::incremental::analyze(&corpus, &current, &grammar, &args, &snapshot, Some(base))
            .unwrap();
    let full = analyze_cards(&corpus.faces, &current, &grammar, &args).unwrap();
    assert_eq!(comparable_faces(&actual), comparable_faces(&full));
    assert!(mode.used);
    assert_eq!(mode.rechecked_faces, [corpus.faces[0].identity.clone()]);
    assert_eq!(mode.reused_faces, [corpus.faces[1].identity.clone()]);
    assert!(
        actual[0]
            .unknown_words
            .iter()
            .any(|word| word.text == "dice")
    );
}

#[test]
fn incremental_form_matching_uses_parser_input_after_reminder_removal() {
    // Disdainful Stroke, with an inserted reminder as a normalization probe.
    // The parser input remains its authentic complete Oracle sentence.
    let text = "Counter target spell with mana (reminder) value 4 or greater.";
    let corpus = selected_corpus(
        &serde_json::to_vec(&json!({"data": {
            "Disdainful Stroke": [card(Some(text), "Legal")],
            "Reach Through Mists": [card(Some("Draw a card."), "Legal")]
        }}))
        .unwrap(),
    );
    let entries = lexicon()
        .lexemes()
        .values()
        .filter(|entry| entry.id != "lexeme:CommonNoun/ManaValue")
        .cloned();
    let current = Lexicon::new(entries).unwrap();
    let old_snapshot = incremental_snapshot(lexicon(), "before multiword form removal");
    let snapshot = incremental_snapshot(&current, "after multiword form removal");
    let mut args = args(None);
    args.incremental_from = Some("target/english-v3/base.json".into());
    let base = incremental_base(&corpus, lexicon(), &args, old_snapshot);
    assert_eq!(
        base.faces[0].analyzed_source,
        "Counter target spell with mana value 4 or greater."
    );
    let grammar = Grammar::with_lexicon(&current);
    let (actual, mode) =
        super::incremental::analyze(&corpus, &current, &grammar, &args, &snapshot, Some(base))
            .unwrap();
    let full = analyze_cards(&corpus.faces, &current, &grammar, &args).unwrap();
    assert_eq!(comparable_faces(&actual), comparable_faces(&full));
    assert!(mode.used);
    assert_eq!(mode.rechecked_faces, [corpus.faces[0].identity.clone()]);
    assert_eq!(mode.reused_faces, [corpus.faces[1].identity.clone()]);
}

#[test]
fn census_segments_revitalize_and_realizes_each_line_in_the_whole_context() {
    let bytes = serde_json::to_vec(&json!({"data": {
        "Revitalize": [card(Some("You gain 3 life.\nDraw a card."), "Legal")]
    }}))
    .unwrap();
    let corpus = selected_corpus(&bytes);
    let grammar = Grammar::with_lexicon(lexicon());
    let faces = analyze_cards(&corpus.faces, lexicon(), &grammar, &args(None)).unwrap();
    let face = &faces[0];
    assert_eq!(face.checked_readings, 2);
    assert_eq!(face.exact_readings, Some(2));
    assert_eq!(face.census, Census::One);
    assert_eq!(
        face.lines
            .iter()
            .map(|line| (line.span, line.checked_readings))
            .collect::<Vec<_>>(),
        [((0, 16), 1), ((17, 29), 1)]
    );
    assert_eq!(
        face.lines[0].text_sha256,
        crate::raw_corpus::digest(b"You gain 3 life.")
    );
    assert!(face.issues.is_empty(), "{:?}", face.issues);
    assert!(!face.constructions.contains_key("Document"));
}
