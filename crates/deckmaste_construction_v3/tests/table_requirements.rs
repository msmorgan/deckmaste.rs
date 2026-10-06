use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Category;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;

constructions! {
    pub mod fixture {
        feature Verdict { Yes, No }
        table selected(number, person) -> Verdict {
            (Singular, First) => Yes,
            (Plural, Third) => Yes,
            (Singular, Third) => No,
        }
        table crossed(number, person) -> Verdict {
            (Singular, Third) => Yes,
            (Plural, First) => Yes,
        }
        category Atom(number, person);
        category Allowed();
        category Crossed();
        category SurfaceAllowed();
        construction Atom: Atom {
            cost 2;
            form [word: lexical(Pronoun)];
            export number = word.number;
            export person = word.person;
        }
        policy Selected<Other> {
            require selected(Other.number, Other.person) = Yes;
        }
        policy Crossed<Other> {
            require crossed(Other.number, Other.person) = Yes;
        }
        schema Checked {
            cost 7;
            form [item: node];
            form ["[", item: node, "]"];
        }
        instance Checked<Result, Guard>: [
            (Allowed, Selected),
            (Crossed, Crossed),
        ] {
            bind item = Atom;
            use Guard(item);
        }
        schema SurfaceChecked {
            form [item: node] require item.number = Singular;
            form ["[", item: node, "]"] require selected(item.number, item.person) = Yes
                require item.number = Plural;
        }
        instance SurfaceChecked: SurfaceAllowed {
            bind item = Atom;
        }
    }
}

#[test]
fn surface_requirements_select_the_same_independent_values_in_both_directions() {
    let lexicon = lexicon();
    let grammar = fixture::Grammar::default();
    for (form, text, owner) in [(0, "x", "first"), (1, "[x]", "third")] {
        let fixture::Reading::Checked { item, .. } =
            independent(&lexicon, owner, 0, fixture::Category::Allowed)
        else {
            unreachable!()
        };
        let expected = fixture::Reading::SurfaceChecked {
            category: fixture::Category::SurfaceAllowed,
            form,
            item,
        };
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(text),
            &fixture::Category::SurfaceAllowed,
        )
        .unwrap();
        let actual: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert_eq!(actual, [expected.clone()].into());
        expected.admit(&lexicon).unwrap();
        assert_eq!(expected.realize(&lexicon).unwrap(), text);
        let mut wrong_form = expected;
        let fixture::Reading::SurfaceChecked { form: selected, .. } = &mut wrong_form else {
            unreachable!()
        };
        *selected = 1 - form;
        assert!(wrong_form.admit(&lexicon).is_err());
        assert!(wrong_form.realize(&lexicon).is_err());
    }
}

fn lexicon() -> Lexicon {
    let declarations = [
        ("first", "x", Some(Number::Singular), Some(Person::First)),
        ("third", "x", Some(Number::Plural), Some(Person::Third)),
        ("no", "y", Some(Number::Singular), Some(Person::Third)),
        (
            "missing_tuple",
            "z",
            Some(Number::Plural),
            Some(Person::First),
        ),
        ("missing_feature", "w", Some(Number::Singular), None),
    ]
    .map(|(id, surface, number, person)| {
        let mut lexeme = Lexeme::invariant(
            id,
            surface,
            Category::Pronoun,
            Source {
                kind: SourceKind::Core,
                path: "synthetic".into(),
                owner: id.into(),
            },
        );
        lexeme.forms[0].features = FeatureBundle {
            number,
            person,
            ..FeatureBundle::default()
        };
        lexeme
    });
    Lexicon::new(declarations.to_vec()).unwrap()
}

fn independent(
    lexicon: &Lexicon,
    id: &str,
    form: usize,
    category: fixture::Category,
) -> fixture::Reading {
    fixture::Reading::Checked {
        category,
        form,
        item: Box::new(fixture::Reading::Atom {
            form: 0,
            word: fixture::Word {
                value: LexicalReading::Word(
                    lexicon
                        .values()
                        .find(|word| word.lexeme == id)
                        .unwrap()
                        .clone(),
                ),
                frame: None,
                countability: None,
            },
        }),
    }
}

#[test]
fn relational_guards_preserve_independent_values_surfaces_traversal_and_cost() {
    let lexicon = lexicon();
    let grammar = fixture::Grammar::default();
    for (form, text) in [(0, "x"), (1, "[x]")] {
        let expected: BTreeSet<_> = ["first", "third"]
            .map(|id| independent(&lexicon, id, form, fixture::Category::Allowed))
            .into_iter()
            .collect();
        let forest = parse(
            &grammar,
            &lexicon,
            &lexicon.analyze(text),
            &fixture::Category::Allowed,
        )
        .unwrap();
        let actual: BTreeSet<_> = grammar.readings(&forest).map(Result::unwrap).collect();
        assert_eq!(actual, expected);
        for reading in actual {
            assert_eq!(reading.realize(&lexicon).unwrap(), text);
            assert!(reading.admit(&lexicon).is_ok());
            assert_eq!(reading.total_cost().unwrap(), 9);
            let mut visited = Vec::new();
            reading
                .visit(&mut |node| visited.push(node.clone()))
                .unwrap();
            assert_eq!(visited.len(), 2);
            assert_eq!(visited[0], reading);
            let mut words = Vec::new();
            reading
                .visit_words(&mut |word| words.push(word.clone()))
                .unwrap();
            let fixture::Reading::Checked { item, .. } = &reading else { unreachable!() };
            let fixture::Reading::Atom { word, .. } = item.as_ref() else { unreachable!() };
            assert_eq!(words, vec![word.clone()]);
        }
    }
}

#[test]
fn guards_reject_false_missing_and_uncorrelated_combinations_in_both_directions() {
    let lexicon = lexicon();
    let grammar = fixture::Grammar::default();
    for (id, text, category) in [
        ("no", "y", fixture::Category::Allowed),
        ("missing_tuple", "z", fixture::Category::Allowed),
        ("missing_feature", "w", fixture::Category::Allowed),
        ("first", "x", fixture::Category::Crossed),
        ("third", "x", fixture::Category::Crossed),
    ] {
        let forest = parse(&grammar, &lexicon, &lexicon.analyze(text), &category).unwrap();
        assert!(forest.roots().is_empty(), "{text} {category:?}");
        let value = independent(&lexicon, id, 0, category);
        assert!(value.admit(&lexicon).is_err());
        assert!(value.realize(&lexicon).is_err());
    }
}
