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
        features: BTreeMap::new(),
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
