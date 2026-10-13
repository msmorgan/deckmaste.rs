use deckmaste_construction_v3::constructions;
use deckmaste_english_v3::parse;
use deckmaste_lexical::{
    Category, Lexeme, LexicalReading, LexicalValue, Lexicon, Source, SourceKind, SurfaceCase,
    WordForm,
};
use std::collections::BTreeSet;

constructions! {
    pub mod fixture {
        closed_word Marker: Subordinator = "old:marker";
        lexical_class Markers: Subordinator | Preposition;
        category Direct();
        category Policy();
        category Member();
        category Optional();
        policy Identity<x> { require x is Marker; }
        construction Direct: Direct { form [head: lexical_word(Marker)]; }
        construction Policy: Policy { form [head: lexical(Subordinator)]; use Identity(head); }
        construction Member: Member {
            form [head: lexical(Subordinator)];
            form [head: lexical(Preposition)];
            require head in Markers;
        }
        construction Optional: Optional {
            form ["(", head: optional(lexical(Subordinator), " "), ")"];
            require head is Marker;
        }
    }
}

fn marker_lexeme() -> Lexeme {
    Lexeme::invariant(
        "old:marker",
        "to",
        Category::Subordinator,
        Source {
            kind: SourceKind::Core,
            path: "fixture.ron".into(),
            owner: "old:marker".into(),
        },
    )
}

fn lexicon() -> Lexicon {
    let mut words = vec![marker_lexeme()];
    let mut other = Lexeme::invariant(
        "plugin:other",
        "to",
        Category::Subordinator,
        Source {
            kind: SourceKind::Plugin,
            path: "synthetic".into(),
            owner: "plugin:other".into(),
        },
    );
    other.properties.classes.insert("Markers".into());
    words.push(other.clone());
    other.id = "plugin:preposition".into();
    other.category = Category::Preposition;
    words.push(other);
    Lexicon::new(words).unwrap()
}

fn readings(
    lexicon: &Lexicon,
    text: &str,
    category: fixture::Category,
) -> BTreeSet<fixture::Reading> {
    let grammar = fixture::Grammar::default();
    let forest = parse(&grammar, lexicon, &lexicon.analyze(text), &category).unwrap();
    grammar.readings(&forest).map(Result::unwrap).collect()
}

fn word(id: &str) -> fixture::Word {
    fixture::Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form: WordForm::Invariant,
            features: deckmaste_lexical::FeatureBundle::default(),
            capitalization: SurfaceCase::Declared,
            variant: 0,
        }),
        frame: None,
        countability: None,
    }
}

#[test]
fn identity_checks_parse_admit_and_realize_without_accepting_homographs() {
    let lexicon = lexicon();
    for (category, expected, wrong) in [
        (
            fixture::Category::Direct,
            fixture::Reading::Direct {
                form: 0,
                head: word("old:marker"),
            },
            fixture::Reading::Direct {
                form: 0,
                head: word("plugin:other"),
            },
        ),
        (
            fixture::Category::Policy,
            fixture::Reading::Policy {
                form: 0,
                head: word("old:marker"),
            },
            fixture::Reading::Policy {
                form: 0,
                head: word("plugin:other"),
            },
        ),
    ] {
        assert_eq!(
            readings(&lexicon, "to", category),
            BTreeSet::from([expected.clone()])
        );
        assert_eq!(expected.realize(&lexicon).unwrap(), "to");
        assert!(wrong.admit(&lexicon).is_err());
        assert!(wrong.realize(&lexicon).is_err());
    }
}

#[test]
fn data_membership_intersects_categories_and_keeps_form_indices() {
    let lexicon = lexicon();
    let expected = BTreeSet::from([
        fixture::Reading::Member {
            form: 0,
            head: word("plugin:other"),
        },
        fixture::Reading::Member {
            form: 1,
            head: word("plugin:preposition"),
        },
    ]);
    assert_eq!(
        readings(&lexicon, "to", fixture::Category::Member),
        expected
    );
    for value in expected {
        assert_eq!(value.realize(&lexicon).unwrap(), "to");
    }
}

#[test]
fn optional_identity_checks_absence_and_only_the_present_declared_word() {
    let lexicon = lexicon();
    let absent = fixture::Reading::Optional {
        form: 0,
        head: None,
    };
    let present = fixture::Reading::Optional {
        form: 0,
        head: Some(word("old:marker")),
    };
    assert_eq!(
        readings(&lexicon, "()", fixture::Category::Optional),
        BTreeSet::from([absent.clone()])
    );
    assert_eq!(
        readings(&lexicon, "(to )", fixture::Category::Optional),
        BTreeSet::from([present.clone()])
    );
    assert_eq!(absent.realize(&lexicon).unwrap(), "()");
    assert_eq!(present.realize(&lexicon).unwrap(), "(to )");
}

#[test]
fn word_selector_preserves_capitalization_and_surface_variants() {
    let mut words = vec![marker_lexeme()];
    words[0].forms[0].surfaces = Some(vec!["to".into(), "unto".into()]);
    let lexicon = Lexicon::new(words).unwrap();
    let mut head = word("old:marker");
    let LexicalReading::Word(value) = &mut head.value else { panic!() };
    value.variant = 1;
    value.capitalization = SurfaceCase::Initial;
    let expected = fixture::Reading::Direct { form: 0, head };
    assert_eq!(
        readings(&lexicon, "Unto", fixture::Category::Direct),
        BTreeSet::from([expected.clone()])
    );
    assert_eq!(expected.realize(&lexicon).unwrap(), "Unto");
}
