use std::collections::BTreeSet;

use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;

constructions! {
    pub mod fixture {
        feature Class { None, Special } default None;
        feature Required { Yes, No }
        category Atom(Class);
        category Ordinary();
        category Special();
        category Explicit();
        category Numbered();
        construction Atom: Atom {
            form [word: lexical(Noun)];
            export Class = word.Class;
        }
        policy Ordinary {
            require child.Class = None;
        }
        policy Special {
            require child.Class = Special;
        }
        schema Checked {
            form [child: Atom];
        }
        instance Checked<Result, Guard>: [
            (Ordinary, Ordinary),
            (Special, Special),
        ] {
            use Guard;
        }
        construction Explicit: Explicit {
            form [word: lexical(Noun)];
            require word.Required = Yes;
        }
        construction Numbered: Numbered {
            form [word: lexical(Noun)];
            require word.number = Singular;
            require word.Class = None;
        }
    }
}

fn lexicon() -> Lexicon {
    Lexicon::new(
        [
            ("absent", None),
            ("ordinary", Some("None")),
            ("special", Some("Special")),
            ("invalid", Some("Unknown")),
        ]
        .map(|(id, class)| {
            let mut lexeme = Lexeme::invariant(
                id,
                "x",
                Category::Noun,
                Source {
                    kind: SourceKind::Core,
                    path: "synthetic".into(),
                    owner: id.into(),
                },
            );
            if let Some(class) = class {
                lexeme
                    .properties
                    .features
                    .insert("Class".into(), class.into());
            }
            if id == "ordinary" {
                lexeme
                    .properties
                    .features
                    .insert("Required".into(), "Yes".into());
            }
            lexeme
        })
        .to_vec(),
    )
    .unwrap()
}

fn word(lexicon: &Lexicon, id: &str) -> fixture::Word {
    fixture::Word {
        value: LexicalReading::Word(
            lexicon
                .values()
                .find(|value| value.lexeme == id)
                .unwrap()
                .clone(),
        ),
        frame: None,
        countability: None,
    }
}

fn atom(lexicon: &Lexicon, id: &str) -> fixture::Reading {
    fixture::Reading::Atom {
        form: 0,
        word: word(lexicon, id),
    }
}

fn readings(lexicon: &Lexicon, category: fixture::Category) -> BTreeSet<fixture::Reading> {
    let grammar = fixture::Grammar::default();
    let forest = parse(&grammar, lexicon, &lexicon.analyze("x"), &category).unwrap();
    grammar.readings(&forest).map(Result::unwrap).collect()
}

#[test]
fn absent_custom_properties_default_without_losing_explicit_values_or_identities() {
    let lexicon = lexicon();
    for (category, ids) in [
        (
            fixture::Category::Atom,
            vec!["absent", "ordinary", "special"],
        ),
        (fixture::Category::Ordinary, vec!["absent", "ordinary"]),
        (fixture::Category::Special, vec!["special"]),
    ] {
        let expected: BTreeSet<_> = ids
            .into_iter()
            .map(|id| {
                let child = atom(&lexicon, id);
                if category == fixture::Category::Atom {
                    child
                } else {
                    fixture::Reading::Checked {
                        category,
                        form: 0,
                        child: Box::new(child),
                    }
                }
            })
            .collect();
        assert_eq!(readings(&lexicon, category), expected);
        for value in expected {
            value.admit(&lexicon).unwrap();
            assert_eq!(value.realize(&lexicon).unwrap(), "x");
            assert!(readings(&lexicon, category).contains(&value));
        }
    }
}

#[test]
fn defaults_do_not_mask_invalid_values_or_fill_other_custom_or_builtin_features() {
    let lexicon = lexicon();
    let mut invalid = vec![atom(&lexicon, "invalid")];
    for id in ["absent", "ordinary", "special", "invalid"] {
        invalid.push(fixture::Reading::Numbered {
            form: 0,
            word: word(&lexicon, id),
        });
        if id != "ordinary" {
            invalid.push(fixture::Reading::Explicit {
                form: 0,
                word: word(&lexicon, id),
            });
        }
    }
    for value in invalid {
        assert!(value.admit(&lexicon).is_err());
        assert!(value.realize(&lexicon).is_err());
    }
    assert!(readings(&lexicon, fixture::Category::Numbered).is_empty());
    let explicit = fixture::Reading::Explicit {
        form: 0,
        word: word(&lexicon, "ordinary"),
    };
    assert_eq!(
        readings(&lexicon, fixture::Category::Explicit),
        BTreeSet::from([explicit.clone()])
    );
    assert_eq!(explicit.realize(&lexicon).unwrap(), "x");
}
