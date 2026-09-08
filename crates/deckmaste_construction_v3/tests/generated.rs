use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::Grammar as _;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Category;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

constructions! {
    pub mod fixture {
        category Atom();
        category Phrase();
        category List();
        category Empty();
        category Subject(number, person);
        category Clause();
        category Head(frame);
        category VerbPhrase();
        category Dependency(extraction);
        category Closed();
        feature extraction { Open, Closed }
        frame Object = "(kind: \"transitive\", items: [Argument((relation: Complement, category: \"Atom\"))])";

        construction Atom: Atom {
            form [word: lexical(Adjective)];
            form [word: lexical(Adjective)];
        }
        construction Phrase: Phrase {
            form [left: Atom, " ", right: optional(Atom)];
            form [right: optional(Atom), "\n", left: Atom];
        }
        construction List: List { form [items: repeat(Atom, ", ")]; }
        construction Empty: Empty { form []; form [""]; }
        construction Late: Empty { form [inner: List]; }
        construction Subject: Subject {
            form [word: lexical(Pronoun)];
            export number = word.number;
            export person = word.person;
        }
        construction Clause: Clause {
            form [subject: Subject, " ", predicate: Subject];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
        }
        construction Head: Head {
            form [word: lexical(Verb)];
            require word.form = Plain;
            export frame = word.frame;
        }
        construction VerbPhrase: VerbPhrase {
            form [head: Head, " ", complement: Atom];
            require head.frame = Object;
        }
        construction Open: Dependency {
            form [word: lexical(Adverb)];
            export extraction = word.extraction;
        }
        construction Forward: Dependency {
            form ["(", child: Dependency, ")"];
            export extraction = child.extraction;
        }
        construction Closed: Closed {
            form [child: Dependency];
            require child.extraction = Closed;
        }
    }
}

fn source(id: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "synthetic".into(),
        owner: id.into(),
    }
}
fn invariant(id: &str, surface: &str, category: Category) -> Lexeme {
    Lexeme::invariant(id, surface, category, source(id))
}
fn word(id: &str, form: WordForm, features: FeatureBundle) -> fixture::Word {
    fixture::Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame: None,
        countability: None,
    }
}
fn atom(id: &str) -> fixture::Reading {
    fixture::Reading::Atom {
        form: 0,
        word: word(id, WordForm::Invariant, FeatureBundle::default()),
    }
}

fn census(
    lexicon: &Lexicon,
    text: &str,
    category: fixture::Category,
) -> BTreeSet<fixture::Reading> {
    let grammar = fixture::Grammar::default();
    let input = lexicon.analyze(text);
    let forest = parse(&grammar, lexicon, &input, &category).unwrap();
    let mut materialized = forest.readings(&grammar);
    assert_eq!(materialized.metrics().builds, 0);
    let mut readings = BTreeSet::new();
    for value in materialized.by_ref() {
        let fixture::Value::Reading(reading) = value.unwrap() else {
            panic!("a requested public Category must produce a Reading")
        };
        assert_eq!(reading.category(), category);
        assert_eq!(reading.realize(lexicon).unwrap(), text);
        assert!(readings.insert(reading));
    }
    assert_eq!(materialized.metrics().internal_failures, 0);
    assert_eq!(materialized.metrics().cyclic_derivations, 0);
    println!(
        "{category:?} {text:?}: {:?}; {:?}",
        forest.metrics(),
        materialized.metrics()
    );
    readings
}

#[test]
fn alternative_optional_and_repeated_forms_satisfy_both_laws() {
    let lexicon = Lexicon::new([
        invariant("a", "α", Category::Adjective),
        invariant("b", "β", Category::Adjective),
    ])
    .unwrap();
    let mut independent = vec![];
    for left in [atom("a"), atom("b")] {
        for right in [None, Some(Box::new(atom("a"))), Some(Box::new(atom("b")))] {
            for form in [0, 1] {
                independent.push(fixture::Reading::Phrase {
                    form,
                    left: Box::new(left.clone()),
                    right: right.clone(),
                });
            }
        }
    }
    for length in 0..=4 {
        for bits in 0..(1 << length) {
            independent.push(fixture::Reading::List {
                form: 0,
                items: (0..length)
                    .map(|i| atom(if bits & (1 << i) == 0 { "a" } else { "b" }))
                    .collect(),
            });
        }
    }
    for value in independent {
        let text = value.realize(&lexicon).unwrap();
        assert_eq!(
            census(&lexicon, &text, value.category()),
            BTreeSet::from([value])
        );
    }
    assert!(census(&lexicon, "α  β", fixture::Category::Phrase).is_empty());
    assert!(census(&lexicon, "α, ", fixture::Category::List).is_empty());
}

#[test]
fn duplicate_derivations_pack_but_same_surface_identities_survive() {
    let lexicon = Lexicon::new([
        invariant("first", "light", Category::Adjective),
        invariant("second", "light", Category::Adjective),
    ])
    .unwrap();
    let grammar = fixture::Grammar::default();
    let forest = parse(
        &grammar,
        &lexicon,
        &lexicon.analyze("light"),
        &fixture::Category::Atom,
    )
    .unwrap();
    assert_eq!(forest.roots().len(), 1);
    let mut readings = forest.readings(&grammar);
    assert_eq!(readings.by_ref().count(), 2);
    assert_eq!(readings.metrics().derivations, 4);
    assert_eq!(readings.metrics().duplicates, 2);
    assert_eq!(
        census(&lexicon, "light", fixture::Category::Atom),
        BTreeSet::from([atom("first"), atom("second")])
    );
    let expected = BTreeSet::from([
        fixture::Reading::Empty { form: 0 },
        fixture::Reading::Late {
            form: 0,
            inner: Box::new(fixture::Reading::List {
                form: 0,
                items: vec![],
            }),
        },
    ]);
    assert_eq!(census(&lexicon, "", fixture::Category::Empty), expected);
}

fn agreement(number: Number, person: Person) -> FeatureBundle {
    FeatureBundle {
        number: Some(number),
        person: Some(person),
        ..FeatureBundle::default()
    }
}
fn pronoun(id: &str, surface: &str, features: &[FeatureBundle]) -> Lexeme {
    let mut lexeme = invariant(id, surface, Category::Pronoun);
    lexeme.forms = features
        .iter()
        .map(|f| FormDeclaration {
            form: WordForm::Invariant,
            features: f.clone(),
            surfaces: None,
        })
        .collect();
    lexeme
}

#[test]
fn category_interfaces_preserve_correlated_agreement_and_absence_is_not_a_wildcard() {
    let alternatives = [
        agreement(Number::Singular, Person::First),
        agreement(Number::Plural, Person::Third),
    ];
    let lexicon = Lexicon::new([
        pronoun("left", "a", &alternatives),
        pronoun("right", "b", &alternatives),
        pronoun(
            "crossed",
            "c",
            &[agreement(Number::Singular, Person::Third)],
        ),
        pronoun("missing", "d", &[FeatureBundle::default()]),
    ])
    .unwrap();
    let expected = alternatives
        .iter()
        .map(|f| {
            let subject = |id| fixture::Reading::Subject {
                form: 0,
                word: word(id, WordForm::Invariant, f.clone()),
            };
            fixture::Reading::Clause {
                form: 0,
                subject: Box::new(subject("left")),
                predicate: Box::new(subject("right")),
            }
        })
        .collect();
    assert_eq!(census(&lexicon, "a b", fixture::Category::Clause), expected);
    assert!(census(&lexicon, "a c", fixture::Category::Clause).is_empty());
    assert!(census(&lexicon, "a d", fixture::Category::Clause).is_empty());
    for value in census(&lexicon, "a b", fixture::Category::Clause) {
        assert!(
            census(
                &lexicon,
                &value.realize(&lexicon).unwrap(),
                value.category()
            )
            .contains(&value)
        );
    }
}

#[test]
fn lexical_frames_and_open_properties_reach_the_governing_construction() {
    let mut verb = Lexeme::verb("head", "act", source("head"));
    verb.properties.frames.push(deckmaste_lexical::Frame {
        kind: "transitive".into(),
        items: vec![deckmaste_lexical::FrameItem::Argument(
            deckmaste_lexical::FrameSlot {
                relation: deckmaste_lexical::Relation::Complement,
                category: "Atom".into(),
            },
        )],
    });
    let mut closed = invariant("closed", "here", Category::Adverb);
    closed
        .properties
        .features
        .insert("extraction".into(), "Closed".into());
    let mut open = invariant("open", "there", Category::Adverb);
    open.properties
        .features
        .insert("extraction".into(), "Open".into());
    let lexicon = Lexicon::new([
        verb,
        Lexeme::verb("unframed", "fail", source("unframed")),
        invariant("object", "x", Category::Adjective),
        closed,
        open,
    ])
    .unwrap();
    let mut head = word(
        "head",
        WordForm::Plain,
        FeatureBundle {
            finiteness: Some(deckmaste_lexical::Finiteness::Nonfinite),
            ..FeatureBundle::default()
        },
    );
    head.frame = Some(0);
    let independent = fixture::Reading::VerbPhrase {
        form: 0,
        head: Box::new(fixture::Reading::Head {
            form: 0,
            word: head,
        }),
        complement: Box::new(atom("object")),
    };
    assert_eq!(independent.realize(&lexicon).unwrap(), "act x");
    assert_eq!(
        census(&lexicon, "act x", fixture::Category::VerbPhrase),
        BTreeSet::from([independent])
    );
    assert!(census(&lexicon, "fail x", fixture::Category::VerbPhrase).is_empty());
    assert_eq!(
        census(&lexicon, "(((here)))", fixture::Category::Closed).len(),
        1
    );
    assert!(census(&lexicon, "(((there)))", fixture::Category::Closed).is_empty());
}

#[test]
fn structural_and_lexical_traversals_follow_each_declared_surface() {
    let lexicon = Lexicon::new([
        invariant("a", "α", Category::Adjective),
        invariant("b", "β", Category::Adjective),
    ])
    .unwrap();
    let value = fixture::Reading::Phrase {
        form: 1,
        left: Box::new(atom("a")),
        right: Some(Box::new(atom("b"))),
    };
    let mut nodes = vec![];
    value.visit(&mut |r| nodes.push(r.clone())).unwrap();
    assert_eq!(nodes, [value.clone(), atom("b"), atom("a")]);
    let mut leaves = vec![];
    value.visit_words(&mut |w| leaves.push(w.clone())).unwrap();
    let expected = ["b", "a"].map(|id| word(id, WordForm::Invariant, FeatureBundle::default()));
    assert_eq!(leaves, expected);
    assert_eq!(value.realize(&lexicon).unwrap(), "β\nα");
}

#[test]
fn independently_invalid_values_are_rejected_before_realization() {
    let lexicon = Lexicon::new([invariant("a", "x", Category::Adjective)]).unwrap();
    for value in [
        fixture::Reading::Atom {
            form: 1,
            word: word("a", WordForm::Invariant, FeatureBundle::default()),
        },
        fixture::Reading::Phrase {
            form: 0,
            left: Box::new(fixture::Reading::List {
                form: 0,
                items: vec![],
            }),
            right: None,
        },
        fixture::Reading::Subject {
            form: 0,
            word: word("a", WordForm::Invariant, FeatureBundle::default()),
        },
    ] {
        assert!(value.realize(&lexicon).is_err());
    }
}

#[test]
fn the_public_grammar_exposes_declaration_productions_without_a_scanner() {
    let grammar = fixture::Grammar::default();
    assert!(
        grammar
            .productions()
            .iter()
            .any(|p| p.category == fixture::Category::Atom
                && matches!(
                    p.symbols.as_slice(),
                    [deckmaste_english_v3::Symbol::Lexical(Category::Adjective)]
                ))
    );
}

constructions! {
    pub mod lexical_values {
        category Noun();
        category Numeral();
        category Joined();
        construction CountNoun: Noun { form [word: lexical(Noun), "!"]; require word.countability = Count; }
        construction MassNoun: Noun { form [word: lexical(Noun), "!"]; require word.countability = Mass; require word.number = Singular; }
        construction Numeral: Numeral { form [word: lexical(Numeral), "."]; }
        construction Joined: Joined { form [left: lexical(Adjective), right: lexical(Adjective)]; }
    }
}

#[test]
fn retained_spelling_capitalization_count_uses_and_numerals_roundtrip_independently() {
    use deckmaste_lexical::Countability;
    use deckmaste_lexical::Numeral;
    let mut noun = Lexeme::noun(
        "cafe",
        "café",
        vec![Countability::Count, Countability::Mass],
        source("cafe"),
    );
    noun.forms[1].surfaces = Some(vec!["cafés".into(), "caféses".into()]);
    let lexicon = Lexicon::new([noun]).unwrap();
    let grammar = lexical_values::Grammar::default();
    let mut independent = vec![];
    for (form, number, variants) in [
        (WordForm::Singular, Number::Singular, 1),
        (WordForm::Plural, Number::Plural, 2),
    ] {
        for variant in 0..variants {
            for capitalization in [SurfaceCase::Declared, SurfaceCase::Initial] {
                for countability in [true, false] {
                    if !countability && number == Number::Plural {
                        continue;
                    }
                    let word = lexical_values::Word {
                        value: LexicalReading::Word(LexicalValue {
                            lexeme: "cafe".into(),
                            form,
                            features: FeatureBundle {
                                number: Some(number),
                                ..FeatureBundle::default()
                            },
                            variant,
                            capitalization,
                        }),
                        frame: None,
                        countability: Some(countability),
                    };
                    independent.push(if countability {
                        lexical_values::Reading::CountNoun { form: 0, word }
                    } else {
                        lexical_values::Reading::MassNoun { form: 0, word }
                    });
                }
            }
        }
    }
    for notation in [
        Numeral::Cardinal,
        Numeral::Ordinal,
        Numeral::Arabic(false),
        Numeral::Arabic(true),
        Numeral::Roman,
    ] {
        for value in [1, 4, 29, 1000] {
            use deckmaste_lexical::NumeralCodec;
            independent.push(lexical_values::Reading::Numeral {
                form: 0,
                word: lexical_values::Word {
                    value: LexicalReading::Numeral {
                        value,
                        notation: notation.canonical_notation(value),
                        capitalization: SurfaceCase::Declared,
                    },
                    frame: None,
                    countability: None,
                },
            });
        }
    }
    for value in &independent {
        let text = value.realize(&lexicon).unwrap();
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(&text),
            &value.category(),
        )
        .unwrap();
        let readings: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert!(readings.contains(value), "{value:?}");
        for reading in readings {
            assert_eq!(reading.realize(&lexicon).unwrap(), text);
            assert!(
                independent.contains(&reading),
                "unexpected Reading {reading:?}"
            );
        }
    }
}

#[test]
fn realization_checks_contextual_binding_without_reparsing_grammar() {
    let mut prefix = invariant("prefix", "pre", Category::Adjective);
    prefix.binding = deckmaste_lexical::Binding::Prefix;
    let lexicon = Lexicon::new([
        prefix,
        invariant("free", "pre", Category::Adjective),
        invariant("host", "fix", Category::Adjective),
    ])
    .unwrap();
    let make = |id: &str| lexical_values::Reading::Joined {
        form: 0,
        left: lexical_values::Word {
            value: word(id, WordForm::Invariant, FeatureBundle::default()).value,
            frame: None,
            countability: None,
        },
        right: lexical_values::Word {
            value: word("host", WordForm::Invariant, FeatureBundle::default()).value,
            frame: None,
            countability: None,
        },
    };
    let expected = make("prefix");
    assert_eq!(expected.realize(&lexicon).unwrap(), "prefix");
    assert!(make("free").realize(&lexicon).is_err());
    let grammar = lexical_values::Grammar::default();
    let forest = parse(
        &grammar,
        &lexicon,
        &lexicon.analyze("prefix"),
        &lexical_values::Category::Joined,
    )
    .unwrap();
    assert_eq!(
        grammar
            .readings(&forest)
            .map(Result::unwrap)
            .collect::<Vec<_>>(),
        [expected]
    );
}

#[test]
fn generated_repetition_packs_long_ambiguous_prefixes_before_materialization() {
    let lexicon = Lexicon::new([
        invariant("first", "x", Category::Adjective),
        invariant("second", "x", Category::Adjective),
    ])
    .unwrap();
    let grammar = fixture::Grammar::default();
    for length in [4, 16, 64] {
        let text = vec!["x"; length].join(", ");
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(&text),
            &fixture::Category::List,
        )
        .unwrap();
        assert!(
            forest.metrics().items <= 15 * length,
            "{:?}",
            forest.metrics()
        );
        assert!(
            forest.metrics().families <= 20 * length,
            "{:?}",
            forest.metrics()
        );
        let mut readings = forest.readings(&grammar);
        assert_eq!(readings.metrics().builds, 0);
        let first = readings.next().unwrap().unwrap();
        let second = readings.next().unwrap().unwrap();
        assert_ne!(first, second);
        assert!(readings.metrics().derivations <= 4);
        println!(
            "generated repetition {length}: {:?}; {:?}",
            forest.metrics(),
            readings.metrics()
        );
        if length == 4 {
            let expected: BTreeSet<_> = (0..16)
                .map(|bits| fixture::Reading::List {
                    form: 0,
                    items: (0..4)
                        .map(|i| atom(if bits & (1 << i) == 0 { "first" } else { "second" }))
                        .collect(),
                })
                .collect();
            assert_eq!(census(&lexicon, &text, fixture::Category::List), expected);
        }
    }
}
