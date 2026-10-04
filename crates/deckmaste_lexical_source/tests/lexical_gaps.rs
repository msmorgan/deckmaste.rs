use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::Tense;
use deckmaste_lexical::WordForm;

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    Lexicon::new(
        deckmaste_lexical_source::load_workspace(&root)
            .unwrap()
            .lexemes,
    )
    .unwrap()
});

type Paradigm = BTreeSet<(String, WordForm, FeatureBundle)>;

fn paradigm(owner: &str) -> Paradigm {
    LEXICON
        .values()
        .filter(|value| value.lexeme == owner && value.capitalization == SurfaceCase::Declared)
        .map(|value| {
            (
                LEXICON
                    .realize(&LexicalReading::Word(value.clone()))
                    .unwrap(),
                value.form,
                value.features.clone(),
            )
        })
        .collect()
}

fn expected_verb(plain: &str, third: &str, past: &str, gerund: &str) -> Paradigm {
    let nonfinite = FeatureBundle {
        finiteness: Some(Finiteness::Nonfinite),
        ..FeatureBundle::default()
    };
    let mut expected = BTreeSet::from([
        (plain.into(), WordForm::Plain, FeatureBundle::default()),
        (past.into(), WordForm::PastParticiple, nonfinite.clone()),
        (gerund.into(), WordForm::GerundParticiple, nonfinite),
    ]);
    for person in [Person::First, Person::Second, Person::Third] {
        for number in [Number::Singular, Number::Plural] {
            for (form, tense, surface) in [
                (
                    WordForm::Present,
                    Tense::Present,
                    if person == Person::Third && number == Number::Singular {
                        third
                    } else {
                        plain
                    },
                ),
                (WordForm::Preterite, Tense::Past, past),
            ] {
                expected.insert((
                    surface.into(),
                    form,
                    FeatureBundle {
                        number: Some(number),
                        person: Some(person),
                        tense: Some(tense),
                        finiteness: Some(Finiteness::Finite),
                        case: None,
                    },
                ));
            }
        }
    }
    expected
}

#[test]
fn newly_declared_verbs_have_the_expected_english_paradigms() {
    for (owner, plain, third, past, gerund) in [
        (
            "core-verb:Trigger",
            "trigger",
            "triggers",
            "triggered",
            "triggering",
        ),
        (
            "core-verb:Resolve",
            "resolve",
            "resolves",
            "resolved",
            "resolving",
        ),
        (
            "core-verb:Remain",
            "remain",
            "remains",
            "remained",
            "remaining",
        ),
        (
            "core-verb:Distribute",
            "distribute",
            "distributes",
            "distributed",
            "distributing",
        ),
    ] {
        assert_eq!(LEXICON.lexemes()[owner].category, Category::Verb);
        assert_eq!(
            paradigm(owner),
            expected_verb(plain, third, past, gerund),
            "{owner}"
        );
    }
    for bogus in ["triggerred", "triggerring", "triggerd"] {
        assert!(LEXICON.analyze(bogus).matches.is_empty(), "{bogus}");
    }
}

fn argument(relation: Relation, category: &str) -> FrameItem {
    FrameItem::Argument(FrameSlot {
        relation,
        category: category.into(),
    })
}

fn predicate(items: Vec<FrameItem>) -> Frame {
    Frame {
        kind: "Predicate".into(),
        items,
    }
}

#[test]
fn verb_frames_preserve_complement_kind_and_order() {
    let object = argument(Relation::Object, "NounPhrase");
    for owner in ["core-verb:Trigger", "core-verb:Resolve"] {
        assert_eq!(
            LEXICON.lexemes()[owner].properties.frames,
            [predicate(vec![]), predicate(vec![object.clone()])]
        );
    }
    assert_eq!(
        LEXICON.lexemes()["core-verb:Remain"].properties.frames,
        [
            predicate(vec![argument(
                Relation::Complement,
                "PredicativeComplement"
            )]),
            predicate(vec![argument(Relation::Complement, "LocativeComplement")]),
        ]
    );
    assert_eq!(
        LEXICON.lexemes()["core-verb:Distribute"].properties.frames,
        [
            predicate(vec![object.clone()]),
            predicate(vec![
                object,
                FrameItem::Marked {
                    vocabulary: "Preposition".into(),
                    member: "Among".into(),
                    slot: FrameSlot {
                        relation: Relation::Complement,
                        category: "NounPhrase".into()
                    }
                }
            ]),
        ]
    );
}

#[test]
fn noun_number_and_countability_do_not_erase_time_homographs() {
    for (owner, singular, plural, countability) in [
        (
            "lexeme:CommonNoun/Effect",
            "effect",
            Some("effects"),
            vec![Countability::Count],
        ),
        (
            "lexeme:CommonNoun/Emblem",
            "emblem",
            Some("emblems"),
            vec![Countability::Count],
        ),
        (
            "lexeme:CommonNoun/Devotion",
            "devotion",
            None,
            vec![Countability::Mass],
        ),
        (
            "lexeme:CommonNoun/Time",
            "time",
            Some("times"),
            vec![Countability::Count, Countability::Mass],
        ),
    ] {
        let lexeme = &LEXICON.lexemes()[owner];
        assert_eq!(lexeme.category, Category::Noun);
        assert_eq!(lexeme.properties.countability, countability);
        let mut expected = BTreeSet::from([(
            singular.into(),
            WordForm::Singular,
            FeatureBundle {
                number: Some(Number::Singular),
                ..FeatureBundle::default()
            },
        )]);
        if let Some(plural) = plural {
            expected.insert((
                plural.into(),
                WordForm::Plural,
                FeatureBundle {
                    number: Some(Number::Plural),
                    ..FeatureBundle::default()
                },
            ));
        }
        assert_eq!(paradigm(owner), expected, "{owner}");
    }
    let time = LEXICON.analyze("time");
    let owners: BTreeSet<_> = time
        .matches
        .iter()
        .filter_map(|matched| match &matched.reading {
            LexicalReading::Word(value) => Some(value.lexeme.as_str()),
            _ => None,
        })
        .collect();
    assert!(owners.contains("lexeme:CommonNoun/Time"));
    assert!(owners.contains("lexeme:counter_kind/timeCounter"));
    assert_eq!(
        LEXICON.lexemes()["lexeme:counter_kind/timeCounter"].category,
        Category::Keyword
    );
}
