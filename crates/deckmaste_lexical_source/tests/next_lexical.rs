use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use deckmaste_lexical::Category;
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
use deckmaste_lexical::SourceKind;
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
fn assign_and_change_have_complete_regular_paradigms_and_declared_frames() {
    for (owner, plain, third, past, gerund) in [
        (
            "core-verb:Assign",
            "assign",
            "assigns",
            "assigned",
            "assigning",
        ),
        (
            "core-verb:Change",
            "change",
            "changes",
            "changed",
            "changing",
        ),
    ] {
        let lexeme = &LEXICON.lexemes()[owner];
        assert_eq!(lexeme.category, Category::Verb);
        assert_eq!(lexeme.source.kind, SourceKind::Core);
        assert_eq!(lexeme.source.owner, owner);
        assert_eq!(paradigm(owner), expected_verb(plain, third, past, gerund));
        for surface in [plain, third, past, gerund] {
            assert!(LEXICON.analyze(surface).matches.iter().any(|matched|
                matches!(&matched.reading, LexicalReading::Word(value) if value.lexeme == owner)));
        }
    }
    let object = FrameItem::Argument(FrameSlot {
        relation: Relation::Object,
        category: "NounPhrase".into(),
    });
    let transitive = Frame {
        kind: "Predicate".into(),
        items: vec![object],
    };
    assert_eq!(
        LEXICON.lexemes()["core-verb:Assign"].properties.frames,
        [transitive.clone()]
    );
    assert_eq!(
        LEXICON.lexemes()["core-verb:Change"].properties.frames,
        [
            Frame {
                kind: "Predicate".into(),
                items: vec![]
            },
            transitive,
        ]
    );
    for bogus in ["assignned", "assignning", "changeing", "changged"] {
        assert!(LEXICON.analyze(bogus).matches.is_empty(), "{bogus}");
    }
}

#[test]
fn single_and_extra_are_invariant_adjectives_with_declared_default_license() {
    for (owner, spelling) in [
        ("vocab:Adjective/Single", "single"),
        ("vocab:Adjective/Extra", "extra"),
    ] {
        let lexeme = &LEXICON.lexemes()[owner];
        assert_eq!(lexeme.category, Category::Adjective);
        assert_eq!(lexeme.source.kind, SourceKind::Core);
        assert_eq!(lexeme.source.owner, owner);
        assert_eq!(
            lexeme.properties.features,
            std::collections::BTreeMap::from([
                ("BareSingularUse".into(), "No".into()),
                (
                    "FeatureSource:BareSingularUse".into(),
                    "crates/deckmaste_lexical_source/lexicon/core.ron".into()
                ),
            ])
        );
        assert!(lexeme.properties.frames.is_empty());
        assert!(lexeme.properties.countability.is_empty());
        assert_eq!(
            paradigm(owner),
            BTreeSet::from([(
                spelling.into(),
                WordForm::Invariant,
                FeatureBundle::default()
            )])
        );
    }
}

#[test]
fn authentic_witnesses_expose_the_expected_ordinary_lexical_owners() {
    // Platinum Emperion, Doran, Swerve, and Time Walk respectively.
    // This checks lexical ownership only, not grammatical admission.
    for (text, spelling, owner) in [
        (
            "Your life total can't change.",
            "change",
            "core-verb:Change",
        ),
        (
            "Each creature assigns combat damage equal to its toughness rather than its power.",
            "assigns",
            "core-verb:Assign",
        ),
        (
            "Change the target of target spell with a single target.",
            "single",
            "vocab:Adjective/Single",
        ),
        (
            "Take an extra turn after this one.",
            "extra",
            "vocab:Adjective/Extra",
        ),
    ] {
        let start = text.find(spelling).unwrap();
        let analyzed = LEXICON.analyze(text);
        assert!(analyzed.matches.iter().any(|matched| matched.start == start && matched.end == start + spelling.len()
            && matches!(&matched.reading, LexicalReading::Word(value) if value.lexeme == owner)), "{text}");
    }
}
