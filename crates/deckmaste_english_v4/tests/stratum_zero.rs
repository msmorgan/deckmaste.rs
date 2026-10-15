//! Independent Reading construction and document round trips for Stratum 0.

use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;
use deckmaste_lexical_source_v4::LexicalEnvironment;
use deckmaste_syntax::Gap;
use deckmaste_syntax::Reading;
use deckmaste_syntax::ReadingNode;
use std::collections::BTreeMap;

fn node(production: &str, category: &str, children: Vec<Reading>) -> Reading {
    Reading::Node(ReadingNode {
        production: production.into(),
        category: category.into(),
        features: if matches!(
            category,
            "KeywordPhrase" | "KeywordParameters" | "KeywordLineItem"
        ) {
            BTreeMap::from([("KeywordForm".into(), "Bare".into())])
        } else {
            BTreeMap::new()
        },
        gap: Gap::None,
        selected_frame: None,
        children,
    })
}

fn keyword(lemma: &str, capitalization: SurfaceCase) -> Reading {
    let leaf = Reading::Leaf(LexicalReading::Word(LexicalValue {
        lexeme: format!("keyword:{lemma}").as_str().into(),
        form: WordForm::Invariant,
        features: FeatureBundle::default(),
        variant: 0,
        capitalization,
    }));
    let parameters = node(
        &format!("Parameters:KeywordParameters/keyword:{lemma}/0"),
        "KeywordParameters",
        vec![leaf],
    );
    let mut phrase = node("CompleteKeywordPhrase", "KeywordPhrase", vec![parameters]);
    if let Reading::Node(n) = &mut phrase {
        n.features
            .insert("Parameterization".into(), "Supplied".into());
    }
    node("CompleteKeywordItem", "KeywordLineItem", vec![phrase])
}

#[test]
fn independently_constructed_keyword_lists_retain_each_separator() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let tail = node(
        "SingleKeyword",
        "KeywordList",
        vec![keyword("vigilance", SurfaceCase::Declared)],
    );
    let mut readings = Vec::new();
    for (production, surface) in [
        ("CommaKeywords", "Flying, vigilance"),
        ("SemicolonKeywords", "Flying; vigilance"),
    ] {
        let list = node(
            production,
            "KeywordList",
            vec![keyword("flying", SurfaceCase::Initial), tail.clone()],
        );
        let reading = node(
            "TextAbility",
            "Document",
            vec![node("KeywordAbilityList", "Ability", vec![list])],
        );
        grammar.admit(&reading, &env.lexicon).unwrap();
        assert_eq!(grammar.realize(&reading, &env.lexicon).unwrap(), surface);
        let parse = grammar.parse(&env.lexicon, surface);
        parse.verify_realization(&env.lexicon).unwrap();
        assert!(parse.contains(&reading));
        readings.push(reading);
    }
    assert_ne!(readings[0], readings[1]);
}

#[test]
fn document_boundaries_and_symbol_attachment_round_trip() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    for text in [
        "Choose one —\n• Destroy a creature.\n• Exile a creature.",
        "Flying\nVigilance",
        "{W}{U}: Level 2\nFlying",
    ] {
        let parse = grammar.parse(&env.lexicon, text);
        assert!(parse.derivation_count() > 0, "{text}");
        parse.verify_realization(&env.lexicon).unwrap();
        for reading in parse.best_readings(8) {
            assert_eq!(grammar.realize(&reading, &env.lexicon).unwrap(), text);
            assert!(parse.contains(&reading));
        }
    }
}

#[test]
fn new_catalog_keyword_parameters_need_no_grammar_change() {
    use deckmaste_lexical_source_v4::schema::KeywordParameter;
    let mut sources = deckmaste_lexical_source_v4::Sources::load().unwrap();
    let mut custom = sources.keyword_abilities[0].clone();
    custom.name = "Test keyword".into();
    custom.lemma = "testkeyword".into();
    custom.parameters = vec![vec![KeywordParameter::Amount, KeywordParameter::Cost]];
    sources.keyword_abilities.push(custom);
    let names = std::fs::read_to_string(deckmaste_lexical_source_v4::card_names_path()).unwrap();
    let env = sources.environment(&names).unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let parse = grammar.parse(&env.lexicon, "Testkeyword 2 {W}{U}");
    assert!(parse.derivation_count() > 0);
    parse.verify_realization(&env.lexicon).unwrap();
    assert_eq!(
        grammar
            .parse(&env.lexicon, "Testkeyword {W}")
            .derivation_count(),
        0
    );
}

#[test]
fn malformed_catalog_parameters_fail_binding() {
    use deckmaste_syntax::frames::ParameterPart;
    let env = LexicalEnvironment::load().unwrap();
    for pattern in [
        vec![ParameterPart::Head, ParameterPart::Head],
        vec![
            ParameterPart::Head,
            ParameterPart::Word("missing:marker".into()),
        ],
        vec![ParameterPart::Head, ParameterPart::Words(Vec::new())],
    ] {
        let mut frames = env.frames.clone();
        frames
            .parameters
            .insert("keyword:flying".into(), vec![pattern]);
        assert!(deckmaste_english_v4::bind(&env.lexicon, &frames).is_err());
    }
}

#[test]
fn document_indentation_is_not_admitted_as_unrealizable_whitespace() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    for text in [
        "Flying\n Vigilance",
        "Flying\n\tVigilance",
        "Choose one —\n • Flying",
    ] {
        let parse = grammar.parse(&env.lexicon, text);
        assert_eq!(parse.derivation_count(), 0, "{text:?}");
        assert!(parse.best_readings(1).is_empty(), "{text:?}");
    }
}

#[test]
fn modal_choice_headers_have_one_analysis_and_round_trip() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    for choice in [
        "Choose one —",
        "Choose one or both —",
        "Choose one or more —",
    ] {
        let parse = grammar.parse(&env.lexicon, choice);
        assert_eq!(parse.derivation_count(), 1, "{choice}");
        parse.verify_realization(&env.lexicon).unwrap();
        let readings = parse.best_readings(1);
        assert_eq!(readings.len(), 1, "{choice}");
        grammar.admit(&readings[0], &env.lexicon).unwrap();
        let text = grammar.realize(&readings[0], &env.lexicon).unwrap();
        assert_eq!(text, choice);
    }
}

#[test]
fn chapter_modal_scope_has_one_analysis_and_costed_modes_use_corpus_markers() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    let chapter = "I, II — Choose one —\n• Destroy a creature.\n• Exile a creature.";
    let parse = grammar.parse(&env.lexicon, chapter);
    assert_eq!(
        parse.derivation_count(),
        1,
        "only the chapter scopes over the modal ability"
    );
    parse.verify_realization(&env.lexicon).unwrap();
    for text in [
        "+ {2} — Destroy a creature.",
        "{P}{P} — Destroy a creature.",
        "• Thunder — {2} — Destroy a creature.",
        "• Galian Beast — {0} — 3/2.",
    ] {
        let parse = grammar.parse(&env.lexicon, text);
        assert!(parse.derivation_count() > 0, "{text}");
        parse.verify_realization(&env.lexicon).unwrap();
    }
}

#[test]
fn activated_modal_abilities_and_mixed_costs_have_one_analysis() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = deckmaste_english_v4::bind(&env.lexicon, &env.frames).unwrap();
    for text in [
        "{1}{G}, {T}: Choose one —",
        "{1}{G}, {T}: Choose one —\n• Destroy a creature.\n• Exile a creature.",
        "{T}, Sacrifice a creature: Destroy a creature.",
        "{1}, {T}, Sacrifice a creature: Destroy a creature.",
    ] {
        let parse = grammar.parse(&env.lexicon, text);
        assert_eq!(parse.derivation_count(), 1, "{text}");
        parse.verify_realization(&env.lexicon).unwrap();
    }
}
