use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Countability;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;

constructions! {
    pub mod grammar {
        feature dependency { Open, Closed }
        table additive_person(person, person) -> person {
            (First, First) => First,
            (First, Second) => First,
            (First, Third) => First,
            (Second, First) => First,
            (Second, Second) => Second,
            (Second, Third) => Second,
            (Third, First) => First,
            (Third, Second) => Second,
            (Third, Third) => Third,
        }
        table closed_pair(dependency, dependency) -> dependency {
            (Closed, Closed) => Closed,
        }
        category Nominal(number, person, dependency);
        category Clause();
        construction Noun: Nominal {
            form [head: lexical(Noun)];
            export number = head.number;
            export person = Third;
            export dependency = Closed;
        }
        construction Pair: Nominal {
            form [left: Nominal, " and ", right: Nominal];
            export number = Plural;
            export person = additive_person(left.person, right.person);
            export dependency = closed_pair(left.dependency, right.dependency);
        }
        construction Pronoun: Nominal {
            form [head: lexical(Pronoun)];
            export number = head.number;
            export person = head.person;
            export dependency = Closed;
        }
        construction Open: Nominal {
            form [head: lexical(Adjective)];
            export number = Singular;
            export person = Third;
            export dependency = Open;
        }
        construction Clause: Clause {
            form [subject: Nominal, " ", predicate: lexical(Verb)];
            require subject.dependency = Closed;
            require predicate.finiteness = Finite;
            agree subject.person = predicate.person;
            agree subject.number = predicate.number;
        }
    }
}

#[test]
fn mixed_person_tables_preserve_the_correlated_inputs_and_independent_values() {
    use deckmaste_lexical::Category as LexicalCategory;
    use deckmaste_lexical::FeatureBundle;
    use deckmaste_lexical::Person;
    use grammar::Category;
    use grammar::Grammar;
    use grammar::Reading;
    use grammar::Word;
    let persons = [
        ("alpha", Person::First),
        ("beta", Person::Second),
        ("gamma", Person::Third),
    ];
    let declarations = persons.map(|(id, person)| {
        let mut lexeme = Lexeme::invariant(
            id,
            id,
            LexicalCategory::Pronoun,
            Source {
                kind: SourceKind::Core,
                path: "synthetic".into(),
                owner: id.into(),
            },
        );
        lexeme.forms[0].features = FeatureBundle {
            person: Some(person),
            number: Some(Number::Singular),
            ..FeatureBundle::default()
        };
        lexeme
    });
    let mut declarations = declarations.to_vec();
    declarations.push(Lexeme::verb(
        "verb",
        "act",
        Source {
            kind: SourceKind::Core,
            path: "synthetic".into(),
            owner: "verb".into(),
        },
    ));
    let lexicon = Lexicon::new(declarations).unwrap();
    let grammar = Grammar::default();
    let independent = |id: &str| Reading::Pronoun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(lexicon.values().find(|v| v.lexeme == id).unwrap().clone()),
            frame: None,
            countability: None,
        },
    };
    for (left, left_person) in persons {
        for (right, right_person) in persons {
            // Independent linguistic oracle: the lowest person rank present
            // controls agreement; additive Number is plural.
            let person = left_person.min(right_person);
            let predicate = lexicon
                .values()
                .find(|v| {
                    v.lexeme == "verb"
                        && v.features.person == Some(person)
                        && v.features.number == Some(Number::Plural)
                        && v.features.tense == Some(deckmaste_lexical::Tense::Present)
                })
                .unwrap()
                .clone();
            let expected = Reading::Clause {
                form: 0,
                subject: Box::new(Reading::Pair {
                    form: 0,
                    left: Box::new(independent(left)),
                    right: Box::new(independent(right)),
                }),
                predicate: Word {
                    value: LexicalReading::Word(predicate),
                    frame: None,
                    countability: None,
                },
            };
            let text = format!("{left} and {right} act");
            assert_eq!(expected.realize(&lexicon).unwrap(), text);
            let forest = parse(
                &grammar,
                &lexicon,
                &lexicon.analyze(&text),
                &Category::Clause,
            )
            .unwrap();
            assert_eq!(
                grammar
                    .readings(&forest)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap(),
                [expected]
            );
        }
    }
}

#[test]
fn missing_table_rows_reject_before_materialization_and_checked_construction() {
    use grammar::Category;
    use grammar::Grammar;
    use grammar::Reading;
    use grammar::Word;
    let mut declarations: Vec<_> = lexicon().lexemes().values().cloned().collect();
    declarations.push(Lexeme::invariant(
        "open",
        "open",
        deckmaste_lexical::Category::Adjective,
        Source {
            kind: SourceKind::Core,
            path: "synthetic".into(),
            owner: "open".into(),
        },
    ));
    let lexicon = Lexicon::new(declarations).unwrap();
    let grammar = Grammar::default();
    for text in ["open and open", "creature and open", "open and creature"] {
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(text),
            &Category::Nominal,
        )
        .unwrap();
        assert!(forest.roots().is_empty(), "{text}");
        assert!(grammar.readings(&forest).next().is_none());
    }
    let open = Reading::Open {
        form: 0,
        head: Word {
            value: LexicalReading::Word(
                lexicon
                    .values()
                    .find(|v| v.lexeme == "open")
                    .unwrap()
                    .clone(),
            ),
            frame: None,
            countability: None,
        },
    };
    assert_eq!(open.realize(&lexicon).unwrap(), "open");
    let invalid = Reading::Pair {
        form: 0,
        left: Box::new(open.clone()),
        right: Box::new(open),
    };
    assert!(invalid.admit(&lexicon).is_err());
    assert!(invalid.realize(&lexicon).is_err());
}

fn lexicon() -> Lexicon {
    let source = || Source {
        kind: SourceKind::Core,
        path: "synthetic".into(),
        owner: "derived-features".into(),
    };
    Lexicon::new([
        Lexeme::noun("noun", "creature", vec![Countability::Count], source()),
        Lexeme::verb("verb", "attack", source()),
    ])
    .unwrap()
}

#[test]
fn derived_constants_constrain_parents_in_both_directions() {
    use grammar::Category;
    use grammar::Grammar;
    use grammar::Reading;
    use grammar::Word;
    let lexicon = lexicon();
    let noun = Reading::Noun {
        form: 0,
        head: Word {
            value: LexicalReading::Word(
                lexicon
                    .values()
                    .find(|v| v.lexeme == "noun" && v.features.number == Some(Number::Singular))
                    .unwrap()
                    .clone(),
            ),
            frame: None,
            countability: Some(true),
        },
    };
    let pair = Reading::Pair {
        form: 0,
        left: Box::new(noun.clone()),
        right: Box::new(noun),
    };
    let predicate = lexicon
        .values()
        .find(|v| {
            v.lexeme == "verb"
                && v.features.number == Some(Number::Plural)
                && v.features.person == Some(deckmaste_lexical::Person::Third)
                && v.features.tense == Some(deckmaste_lexical::Tense::Present)
        })
        .unwrap()
        .clone();
    let expected = Reading::Clause {
        form: 0,
        subject: Box::new(pair),
        predicate: Word {
            value: LexicalReading::Word(predicate.clone()),
            frame: None,
            countability: None,
        },
    };
    assert_eq!(
        expected.realize(&lexicon).unwrap(),
        "creature and creature attack"
    );
    let grammar = Grammar::default();
    let forest = parse(
        &grammar,
        &lexicon,
        &lexicon.analyze("creature and creature attack"),
        &Category::Clause,
    )
    .unwrap();
    let actual: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
    assert_eq!(actual, BTreeSet::from([expected.clone()]));
    for text in [
        "creature and creature attacks",
        "creature attack",
        "creatures attacks",
    ] {
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(text),
            &Category::Clause,
        )
        .unwrap();
        assert!(forest.roots().is_empty(), "{text}");
    }
    let Reading::Clause { subject, .. } = expected else { unreachable!() };
    let mut singular = predicate;
    singular.features.number = Some(Number::Singular);
    let invalid = Reading::Clause {
        form: 0,
        subject,
        predicate: Word {
            value: LexicalReading::Word(singular),
            frame: None,
            countability: None,
        },
    };
    assert!(invalid.admit(&lexicon).is_err());
    assert!(invalid.realize(&lexicon).is_err());
}
