use std::collections::BTreeSet;

use deckmaste_lexical::*;

fn source(owner: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "tests/declared-lexicon.ron".to_owned(),
        owner: owner.to_owned(),
    }
}

fn invariant(id: &str, surface: &str, category: Category) -> Lexeme {
    Lexeme::invariant(id, surface, category, source(id))
}

fn multiword(id: &str, surface: &str, category: Category) -> Lexeme {
    let mut lexeme = invariant(id, surface, category);
    lexeme.surface_structure = SurfaceStructure::Multiword;
    lexeme
}

fn noun(id: &str, lemma: &str) -> Lexeme {
    Lexeme::noun(id, lemma, vec![Countability::Count], source(id))
}

fn verb(id: &str, lemma: &str) -> Lexeme {
    Lexeme::verb(id, lemma, source(id))
}

fn nominal(number: Number) -> FeatureBundle {
    FeatureBundle {
        number: Some(number),
        ..FeatureBundle::default()
    }
}

fn finite(person: Person, number: Number, tense: Tense) -> FeatureBundle {
    FeatureBundle {
        number: Some(number),
        person: Some(person),
        tense: Some(tense),
        finiteness: Some(Finiteness::Finite),
        case: None,
    }
}

fn nonfinite() -> FeatureBundle {
    FeatureBundle {
        finiteness: Some(Finiteness::Nonfinite),
        ..FeatureBundle::default()
    }
}

fn word(id: &str, form: WordForm, features: FeatureBundle) -> LexicalReading {
    LexicalReading::Word(LexicalValue {
        lexeme: id.to_owned(),
        form,
        features,
        variant: 0,
        capitalization: SurfaceCase::Declared,
    })
}

fn numeral(value: i32, notation: Numeral) -> LexicalReading {
    LexicalReading::Numeral {
        value,
        notation,
        capitalization: SurfaceCase::Declared,
    }
}

fn initial(reading: LexicalReading) -> LexicalReading {
    match reading {
        LexicalReading::Word(value) => LexicalReading::Word(LexicalValue {
            capitalization: SurfaceCase::Initial,
            ..value
        }),
        LexicalReading::Numeral {
            value, notation, ..
        } => LexicalReading::Numeral {
            value,
            notation,
            capitalization: SurfaceCase::Initial,
        },
    }
}

fn at(text: &AnalyzedText, start: usize, end: usize) -> BTreeSet<LexicalReading> {
    text.matches
        .iter()
        .filter(|m| m.start == start && m.end == end)
        .map(|m| m.reading.clone())
        .collect()
}

fn complete(lexicon: &Lexicon, text: &str) -> BTreeSet<LexicalReading> {
    at(&lexicon.analyze(text), 0, text.chars().count())
}

fn override_form(lexeme: &mut Lexeme, form: WordForm, surfaces: &[&str]) {
    for declaration in &mut lexeme.forms {
        if declaration.form == form {
            declaration.surfaces = Some(surfaces.iter().map(|s| (*s).to_owned()).collect());
        }
    }
}

#[test]
fn counters_preserves_noun_and_correlated_finite_verb() {
    let lexicon =
        Lexicon::new([noun("n:counter", "counter"), verb("v:counter", "counter")]).unwrap();
    assert_eq!(
        complete(&lexicon, "counters"),
        BTreeSet::from([
            word("n:counter", WordForm::Plural, nominal(Number::Plural)),
            word(
                "v:counter",
                WordForm::Present,
                finite(Person::Third, Number::Singular, Tense::Present)
            ),
        ])
    );
}

#[test]
fn cast_preserves_plain_present_preterite_and_participle_alternatives() {
    let mut cast = verb("v:cast", "cast");
    override_form(&mut cast, WordForm::Preterite, &["cast"]);
    override_form(&mut cast, WordForm::PastParticiple, &["cast"]);
    let lexicon = Lexicon::new([cast]).unwrap();
    let readings = complete(&lexicon, "cast");

    for expected in [
        word("v:cast", WordForm::Plain, nonfinite()),
        word(
            "v:cast",
            WordForm::Present,
            finite(Person::First, Number::Singular, Tense::Present),
        ),
        word(
            "v:cast",
            WordForm::Preterite,
            finite(Person::Third, Number::Plural, Tense::Past),
        ),
        word("v:cast", WordForm::PastParticiple, nonfinite()),
    ] {
        assert!(readings.contains(&expected), "missing {expected:?}");
    }
    assert!(!readings.contains(&word(
        "v:cast",
        WordForm::Present,
        finite(Person::Third, Number::Singular, Tense::Present),
    )));
    assert_eq!(readings.len(), 13);
}

#[test]
fn declarations_roundtrip_with_shared_frame_and_feature_values() {
    let mut cast = verb("v:cast", "cast");
    cast.properties.frames = vec![Frame {
        kind: "Predicate".to_owned(),
        items: vec![FrameItem::Marked {
            vocabulary: "Preposition".to_owned(),
            member: "At".to_owned(),
            slot: FrameSlot {
                relation: Relation::Object,
                category: "NounPhrase".to_owned(),
            },
        }],
    }];
    override_form(&mut cast, WordForm::Preterite, &["cast"]);
    override_form(&mut cast, WordForm::PastParticiple, &["cast"]);

    let serialized = ron::to_string(&cast).unwrap();
    let restored: Lexeme = ron::from_str(&serialized).unwrap();
    assert_eq!(restored, cast);

    let lexicon = Lexicon::new([restored]).unwrap();
    let readings = complete(&lexicon, "cast");
    assert!(readings.contains(&word(
        "v:cast",
        WordForm::Preterite,
        finite(Person::Third, Number::Plural, Tense::Past),
    )));
    assert!(readings.contains(&word("v:cast", WordForm::PastParticiple, nonfinite(),)));
}

#[test]
fn one_or_two_targets_retains_each_word_and_numeral_reading() {
    let lexicon = Lexicon::new([
        noun("n:one", "one"),
        invariant("d:one", "one", Category::Determinative),
        invariant("c:or", "or", Category::Coordinator),
        invariant("d:target", "target", Category::Determinative),
        noun("n:creature", "creature"),
    ])
    .unwrap();
    let plain_one = BTreeSet::from([
        word("n:one", WordForm::Singular, nominal(Number::Singular)),
        word("d:one", WordForm::Invariant, FeatureBundle::default()),
        numeral(1, Numeral::Cardinal),
    ]);
    assert_eq!(complete(&lexicon, "one"), plain_one);
    let text = lexicon.analyze("One or two target creatures");
    assert_eq!(
        at(&text, 0, 3),
        plain_one.into_iter().map(initial).collect()
    );
    assert_eq!(
        at(&text, 7, 10),
        BTreeSet::from([numeral(2, Numeral::Cardinal)])
    );
    assert_eq!(
        at(&text, 11, 17),
        BTreeSet::from([word(
            "d:target",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&text, 18, 27),
        BTreeSet::from([word(
            "n:creature",
            WordForm::Plural,
            nominal(Number::Plural)
        ),])
    );
    assert!(text.unknown_words().is_empty());
    assert!(!text.matches.iter().any(|m| m.start == 0 && m.end > 3));
}

#[test]
fn numeric_prefixes_survive_longer_numeric_phrases() {
    let lexicon = Lexicon::new([noun("n:one", "one")]).unwrap();
    let text = lexicon.analyze("one hundred");
    assert_eq!(
        at(&text, 0, 3),
        BTreeSet::from([
            word("n:one", WordForm::Singular, nominal(Number::Singular)),
            numeral(1, Numeral::Cardinal),
        ])
    );
    assert_eq!(
        at(&text, 0, 11),
        BTreeSet::from([numeral(100, Numeral::Cardinal)])
    );
}

#[test]
fn initial_variants_do_not_case_fold_exact_catalog_entries() {
    let mut catalog = invariant("name:ThreeDog", "Three Dog", Category::Catalog);
    catalog.capitalization = Capitalization::Exact;
    catalog.source = Source {
        kind: SourceKind::Catalog,
        path: "tests/card-names.txt".to_owned(),
        owner: "Three Dog".to_owned(),
    };
    let lexicon = Lexicon::new([noun("n:counter", "counter"), catalog]).unwrap();
    assert_eq!(
        complete(&lexicon, "Counters"),
        BTreeSet::from([initial(word(
            "n:counter",
            WordForm::Plural,
            nominal(Number::Plural)
        )),])
    );
    assert!(complete(&lexicon, "COUNTERS").is_empty());
    assert_eq!(
        complete(&lexicon, "Three Dog"),
        BTreeSet::from([word(
            "name:ThreeDog",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert!(complete(&lexicon, "three Dog").is_empty());
    assert!(complete(&lexicon, "Three dog").is_empty());
}

#[test]
fn default_morphology_handles_y_sibilants_e_and_ie() {
    let lexicon = Lexicon::new([
        noun("n:ability", "ability"),
        noun("n:box", "box"),
        verb("v:copy", "copy"),
        verb("v:watch", "watch"),
        verb("v:exile", "exile"),
        verb("v:die", "die"),
    ])
    .unwrap();
    for (surface, reading) in [
        (
            "abilities",
            word("n:ability", WordForm::Plural, nominal(Number::Plural)),
        ),
        (
            "boxes",
            word("n:box", WordForm::Plural, nominal(Number::Plural)),
        ),
        (
            "copies",
            word(
                "v:copy",
                WordForm::Present,
                finite(Person::Third, Number::Singular, Tense::Present),
            ),
        ),
        (
            "watches",
            word(
                "v:watch",
                WordForm::Present,
                finite(Person::Third, Number::Singular, Tense::Present),
            ),
        ),
        (
            "copying",
            word("v:copy", WordForm::GerundParticiple, nonfinite()),
        ),
        (
            "exiling",
            word("v:exile", WordForm::GerundParticiple, nonfinite()),
        ),
        (
            "dying",
            word("v:die", WordForm::GerundParticiple, nonfinite()),
        ),
    ] {
        assert_eq!(complete(&lexicon, surface), BTreeSet::from([reading]));
    }
    for (surface, id) in [("copied", "v:copy"), ("exiled", "v:exile")] {
        assert_eq!(complete(&lexicon, surface), past_readings(id, 0));
    }
    for malformed in [
        "abilitys", "boxs", "copys", "watchs", "copyed", "exileed", "exileing", "dieing",
    ] {
        assert!(complete(&lexicon, malformed).is_empty(), "{malformed}");
    }
}

fn past_readings(id: &str, variant: usize) -> BTreeSet<LexicalReading> {
    let mut readings: BTreeSet<_> = [
        (Person::First, Number::Singular),
        (Person::Second, Number::Singular),
        (Person::Third, Number::Singular),
        (Person::First, Number::Plural),
        (Person::Second, Number::Plural),
        (Person::Third, Number::Plural),
    ]
    .into_iter()
    .map(|(person, number)| word(id, WordForm::Preterite, finite(person, number, Tense::Past)))
    .collect();
    readings.insert(word(id, WordForm::PastParticiple, nonfinite()));
    readings
        .into_iter()
        .map(|reading| match reading {
            LexicalReading::Word(value) => LexicalReading::Word(LexicalValue { variant, ..value }),
            LexicalReading::Numeral { .. } => unreachable!(),
        })
        .collect()
}

#[test]
fn overrides_replace_defaults_and_keep_explicit_syncretic_variants() {
    let mut tap = verb("v:tap", "tap");
    override_form(&mut tap, WordForm::GerundParticiple, &["tapping"]);
    let mut draw = verb("v:draw", "draw");
    override_form(&mut draw, WordForm::Preterite, &["drew"]);
    override_form(&mut draw, WordForm::PastParticiple, &["drawn"]);
    let mut burn = verb("v:burn", "burn");
    override_form(&mut burn, WordForm::Preterite, &["burned", "burnt"]);
    override_form(&mut burn, WordForm::PastParticiple, &["burned", "burnt"]);
    let lexicon = Lexicon::new([tap, draw, burn]).unwrap();
    assert_eq!(
        complete(&lexicon, "tapping"),
        BTreeSet::from([word("v:tap", WordForm::GerundParticiple, nonfinite()),])
    );
    assert_eq!(
        complete(&lexicon, "drawn"),
        BTreeSet::from([word("v:draw", WordForm::PastParticiple, nonfinite()),])
    );
    let expected_drew: BTreeSet<_> = past_readings("v:draw", 0).into_iter().filter(|reading| {
        matches!(reading, LexicalReading::Word(value) if value.form == WordForm::Preterite)
    }).collect();
    assert_eq!(complete(&lexicon, "drew"), expected_drew);
    assert_eq!(complete(&lexicon, "burned"), past_readings("v:burn", 0));
    assert_eq!(complete(&lexicon, "burnt"), past_readings("v:burn", 1));
    for obsolete in ["taping", "drawed"] {
        assert!(complete(&lexicon, obsolete).is_empty(), "{obsolete}");
    }
}

#[test]
fn be_agreement_keeps_complete_bundles_instead_of_crossing_features() {
    let mut be = invariant("v:be", "be", Category::Verb);
    be.forms = [
        ("am", Person::First, Number::Singular),
        ("is", Person::Third, Number::Singular),
        ("are", Person::Second, Number::Singular),
        ("are", Person::First, Number::Plural),
        ("are", Person::Second, Number::Plural),
        ("are", Person::Third, Number::Plural),
    ]
    .into_iter()
    .map(|(surface, person, number)| FormDeclaration {
        form: WordForm::Present,
        features: finite(person, number, Tense::Present),
        surfaces: Some(vec![surface.to_owned()]),
    })
    .collect();
    let lexicon = Lexicon::new([be]).unwrap();
    assert_eq!(
        complete(&lexicon, "am"),
        BTreeSet::from([word(
            "v:be",
            WordForm::Present,
            finite(Person::First, Number::Singular, Tense::Present)
        ),])
    );
    assert_eq!(
        complete(&lexicon, "is"),
        BTreeSet::from([word(
            "v:be",
            WordForm::Present,
            finite(Person::Third, Number::Singular, Tense::Present)
        ),])
    );
    assert_eq!(
        complete(&lexicon, "are"),
        BTreeSet::from([
            word(
                "v:be",
                WordForm::Present,
                finite(Person::Second, Number::Singular, Tense::Present)
            ),
            word(
                "v:be",
                WordForm::Present,
                finite(Person::First, Number::Plural, Tense::Present)
            ),
            word(
                "v:be",
                WordForm::Present,
                finite(Person::Second, Number::Plural, Tense::Present)
            ),
            word(
                "v:be",
                WordForm::Present,
                finite(Person::Third, Number::Plural, Tense::Present)
            ),
        ])
    );
    assert!(complete(&lexicon, "bes").is_empty());
}

#[test]
fn multiword_and_bound_forms_keep_overlapping_analyses() {
    let mut swamp = invariant("type:Swamp", "Swamp", Category::Catalog);
    swamp.capitalization = Capitalization::Exact;
    swamp.binding = Binding::Prefix;
    let mut walk = invariant("bound:walk", "walk", Category::Keyword);
    walk.binding = Binding::Suffix;
    let lexicon = Lexicon::new([
        multiword("term:mana-value", "mana value", Category::Noun),
        invariant("n:mana", "mana", Category::Noun),
        noun("n:value", "value"),
        swamp,
        walk,
        invariant("keyword:Swampwalk", "Swampwalk", Category::Keyword),
    ])
    .unwrap();
    let phrase = lexicon.analyze("mana value");
    assert_eq!(
        at(&phrase, 0, 10),
        BTreeSet::from([word(
            "term:mana-value",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&phrase, 0, 4),
        BTreeSet::from([word(
            "n:mana",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&phrase, 5, 10),
        BTreeSet::from([word(
            "n:value",
            WordForm::Singular,
            nominal(Number::Singular)
        ),])
    );
    let bound = lexicon.analyze("Swampwalk");
    for (start, end, id) in [
        (0, 5, "type:Swamp"),
        (5, 9, "bound:walk"),
        (0, 9, "keyword:Swampwalk"),
    ] {
        assert_eq!(
            at(&bound, start, end),
            BTreeSet::from([word(id, WordForm::Invariant, FeatureBundle::default()),])
        );
    }
    assert!(bound.unknown_words().is_empty());
    assert!(complete(&lexicon, "mana  value").is_empty());
    assert!(!lexicon.analyze("Swampwalker").matches.iter().any(|m| {
        m.reading == word("bound:walk", WordForm::Invariant, FeatureBundle::default())
    }));
}

#[test]
fn unicode_positions_and_normalization_preserve_distinct_provenance() {
    let lexeme = invariant("name:Ae", "Æther", Category::Catalog);
    let expected_source = lexeme.source.clone();
    let lexicon = Lexicon::new([lexeme, noun("n:counter", "counter")]).unwrap();
    let text = lexicon.analyze_source("Æther\u{a0}counter", Some("Æther counter"));
    assert_eq!(text.raw, "Æther\u{a0}counter");
    assert_eq!(text.normalized.as_deref(), Some("Æther counter"));
    assert_eq!(text.tokens.iter().collect::<String>(), "Æther counter");
    assert_eq!(text.byte_range(6, 13), Some(7..14));
    assert_eq!(&text.text()[7..14], "counter");
    assert_eq!(
        at(&text, 6, 13),
        BTreeSet::from([word(
            "n:counter",
            WordForm::Singular,
            nominal(Number::Singular)
        ),])
    );
    assert_eq!(
        at(&text, 0, 5),
        BTreeSet::from([word(
            "name:Ae",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    let name = text
        .matches
        .iter()
        .find(|m| m.start == 0 && m.end == 5)
        .unwrap();
    let LexicalReading::Word(value) = &name.reading else {
        panic!("expected declared name");
    };
    assert_eq!(lexicon.lexemes()[&value.lexeme].source, expected_source);
    assert_eq!(text.byte_range(14, 15), None);
    assert_eq!(text.byte_range(8, 2), None);
    assert_eq!(lexicon.analyze("Æther counter").normalized, None);
}

#[test]
fn undeclared_words_and_parts_of_speech_are_not_guessed() {
    let lexicon = Lexicon::new([noun("n:counter", "counter")]).unwrap();
    let text = lexicon.analyze("counters countering unknown");
    assert_eq!(
        at(&text, 0, 8),
        BTreeSet::from([word("n:counter", WordForm::Plural, nominal(Number::Plural)),])
    );
    assert_eq!(text.unknown_words(), vec![9..19, 20..27]);
    assert!(!text.matches.iter().any(|m| m.start >= 9));
}

#[test]
fn independently_constructed_readings_roundtrip_through_realization() {
    let lexicon = Lexicon::new([noun("n:counter", "counter")]).unwrap();
    for (reading, expected_surface) in [
        (
            word("n:counter", WordForm::Plural, nominal(Number::Plural)),
            "counters",
        ),
        (
            initial(word(
                "n:counter",
                WordForm::Singular,
                nominal(Number::Singular),
            )),
            "Counter",
        ),
        (numeral(100, Numeral::Cardinal), "one hundred"),
        (initial(numeral(2, Numeral::Cardinal)), "Two"),
        (numeral(1_001, Numeral::Cardinal), "one thousand, one"),
        (numeral(-2, Numeral::Ordinal), "second to last"),
        (numeral(-1_000, Numeral::Arabic(true)), "-1,000"),
        (numeral(4, Numeral::Roman), "IV"),
    ] {
        assert_eq!(lexicon.realize(&reading).unwrap(), expected_surface);
        assert!(
            complete(&lexicon, expected_surface).contains(&reading),
            "{reading:?} from {expected_surface}"
        );
    }
}

#[test]
fn every_returned_reading_realizes_its_exact_source_occurrence() {
    let lexicon = Lexicon::new([
        noun("n:counter", "counter"),
        verb("v:counter", "counter"),
        multiword("term:mana-value", "mana value", Category::Noun),
    ])
    .unwrap();
    let text = lexicon.analyze("Counters: one hundred mana value; countered.");
    assert!(
        text.matches
            .iter()
            .any(|m| m.reading == numeral(100, Numeral::Cardinal))
    );
    for found in &text.matches {
        let range = text.byte_range(found.start, found.end).unwrap();
        assert_eq!(
            lexicon.realize(&found.reading).unwrap(),
            &text.text()[range]
        );
    }
}

#[test]
fn invalid_constructed_values_are_rejected() {
    let lexicon = Lexicon::new([noun("n:counter", "counter")]).unwrap();
    for reading in [
        word("n:counter", WordForm::Plural, nominal(Number::Singular)),
        word("v:counter", WordForm::Plain, nonfinite()),
        numeral(4_000, Numeral::Roman),
        initial(numeral(4, Numeral::Roman)),
    ] {
        assert!(
            matches!(
                lexicon.realize(&reading),
                Err(LexicalError::UnlicensedValue)
            ),
            "{reading:?}"
        );
    }
    assert_eq!(
        lexicon.realize(&numeral(i32::MAX, Numeral::Roman)).unwrap(),
        "infinitum"
    );
}

#[test]
fn invariant_categories_reject_inapplicable_nominal_features() {
    let features = FeatureBundle {
        number: Some(Number::Plural),
        person: Some(Person::First),
        case: Some(Case::Nominative),
        ..FeatureBundle::default()
    };
    let mut pronoun = invariant("pro:we", "we", Category::Pronoun);
    pronoun.forms[0].features = features.clone();
    let lexicon = Lexicon::new([pronoun]).unwrap();
    assert_eq!(
        complete(&lexicon, "we"),
        BTreeSet::from([word("pro:we", WordForm::Invariant, features.clone()),])
    );
    let mut preposition = invariant("p:with", "with", Category::Preposition);
    preposition.forms[0].features = features;
    assert!(matches!(Lexicon::new([preposition]),
        Err(LexicalError::Declaration { lexeme, .. }) if lexeme == "p:with"));
}

#[test]
fn combining_marks_do_not_create_false_free_word_boundaries() {
    let lexicon = Lexicon::new([invariant("n:e", "e", Category::Noun)]).unwrap();
    let text = lexicon.analyze("e\u{301}");
    assert_eq!(text.words(), vec![0..2]);
    assert_eq!(text.unknown_words(), vec![0..2]);
    assert_eq!(text.byte_range(0, 2), Some(0..3));
    assert!(text.matches.is_empty());
    assert_eq!(
        complete(&lexicon, "e"),
        BTreeSet::from([word("n:e", WordForm::Invariant, FeatureBundle::default()),])
    );
}

#[test]
fn crossing_bound_matches_do_not_hide_unsegmentable_words() {
    let bound = |id: &str, surface: &str| {
        let mut lexeme = invariant(id, surface, Category::Noun);
        lexeme.binding = Binding::Bound;
        lexeme
    };
    let crossing = Lexicon::new([bound("piece:ab", "ab"), bound("piece:bc", "bc")]).unwrap();
    let text = crossing.analyze("abc");
    assert_eq!(
        at(&text, 0, 2),
        BTreeSet::from([word(
            "piece:ab",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&text, 1, 3),
        BTreeSet::from([word(
            "piece:bc",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(text.unknown_words(), vec![0..3]);
    let adjacent = Lexicon::new([bound("piece:a", "a"), bound("piece:bc", "bc")]).unwrap();
    let text = adjacent.analyze("abc");
    assert_eq!(
        at(&text, 0, 1),
        BTreeSet::from([word(
            "piece:a",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert_eq!(
        at(&text, 1, 3),
        BTreeSet::from([word(
            "piece:bc",
            WordForm::Invariant,
            FeatureBundle::default()
        ),])
    );
    assert!(text.unknown_words().is_empty());
}

/// The ability words are a listed inventory [CR#207.2c]; flavor words are
/// listed nowhere [CR#207.2d], so an italic head no declaration spells is one
/// and the run itself is the label.
#[test]
fn an_italic_run_no_declaration_spells_is_a_flavor_word() {
    let lexicon = Lexicon::new([
        invariant(
            "lexeme:ability_word/battalion",
            "Battalion",
            Category::Keyword,
        ),
        verb("lexeme:keyword_action/attack", "attack"),
    ])
    .unwrap();

    assert_eq!(
        lexicon.analyze_italic_head("Battalion"),
        ItalicHead::AbilityWord(LexicalValue {
            lexeme: "lexeme:ability_word/battalion".to_owned(),
            form: WordForm::Invariant,
            features: FeatureBundle::default(),
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        "a declared ability word is not a flavor word",
    );

    for run in ["Kowabunga", "Blade Beam", "A Test of Your Reflexes"] {
        assert_eq!(
            lexicon.analyze_italic_head(run),
            ItalicHead::FlavorWord {
                label: run.to_owned()
            },
            "{run:?} is a flavor word whose label is the run verbatim",
        );
    }

    assert_eq!(
        lexicon.analyze_italic_head("Attack"),
        ItalicHead::FlavorWord {
            label: "Attack".to_owned()
        },
        "only a listed ability word displaces a flavor word; a declared verb does not",
    );
}

#[test]
fn ordinary_recipes_reject_separator_and_quote_boundaries() {
    for surface in [
        "two words",
        "two\nwords",
        "word\t",
        "a,b",
        "\"word\"",
        "“word”",
        "'word'",
        "word;next",
    ] {
        assert!(
            Lexicon::new([invariant("word", surface, Category::Noun)]).is_err(),
            "{surface:?}"
        );
        let mut lexeme = noun("noun", "card");
        lexeme.forms[0].surfaces = Some(vec![surface.into()]);
        assert!(Lexicon::new([lexeme]).is_err(), "override {surface:?}");
    }
    for surface in [
        "two\nwords",
        "two\twords",
        "two  words",
        "two \"words\"",
        "two, words",
    ] {
        assert!(
            Lexicon::new([multiword("phrase", surface, Category::Noun)]).is_err(),
            "{surface:?}"
        );
    }
    let lexicon = Lexicon::new([
        multiword("phrase", "two words", Category::Noun),
        invariant("catalog", "A, \"Name\"", Category::Catalog),
        invariant("symbol", "{W/U}", Category::Symbol),
        invariant("contraction", "y'all", Category::Pronoun),
    ])
    .unwrap();
    for surface in ["two words", "A, \"Name\"", "{W/U}", "y'all"] {
        assert!(!complete(&lexicon, surface).is_empty());
    }
    let mut disguised = invariant("noun", "two words", Category::Noun);
    disguised.surface_structure = SurfaceStructure::Opaque;
    assert!(Lexicon::new([disguised]).is_err());
}

#[test]
fn bound_forms_expose_adjacent_hosts_without_guessing_unknown_hosts() {
    let mut prefix = invariant("prefix:non", "non", Category::Affix);
    prefix.binding = Binding::Prefix;
    let mut suffix = invariant("suffix:walk", "walk", Category::Keyword);
    suffix.binding = Binding::Suffix;
    let lexicon = Lexicon::new([prefix, suffix, noun("noun:land", "land")]).unwrap();
    for text in ["nonland", "landwalk", "nonlandwalk", "nonnonland"] {
        let analyzed = lexicon.analyze(text);
        assert!(analyzed.unknown_words().is_empty(), "{text}");
        let start = text.find("land").unwrap();
        assert_eq!(
            at(&analyzed, start, start + 4),
            BTreeSet::from([word(
                "noun:land",
                WordForm::Singular,
                nominal(Number::Singular)
            )])
        );
    }
    for text in ["unknownland", "landunknown", "nonunknown", "unknownwalk"] {
        assert!(!lexicon.analyze(text).unknown_words().is_empty(), "{text}");
    }
    assert!(lexicon.analyze("outlandish").matches.is_empty());
}
