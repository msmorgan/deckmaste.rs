mod common;

use common::LEXICON;
use common::readings;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::*;

fn lexical(
    owner: &str,
    form: WordForm,
    features: FeatureBundle,
    frame: Option<usize>,
    countability: Option<bool>,
) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: owner.into(),
            form,
            features,
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability,
    }
}

fn coordinator() -> Word {
    lexical(
        "vocab:Coordinator/And",
        WordForm::Invariant,
        FeatureBundle::default(),
        None,
        None,
    )
}
fn bare_gap(owner: &str) -> Reading {
    Reading::BareObjectGap {
        form: 0,
        head: lexical(
            owner,
            WordForm::Plain,
            FeatureBundle::default(),
            Some(0),
            None,
        ),
    }
}
fn finite_gap(owner: &str) -> Reading {
    Reading::FiniteObjectGap {
        form: 0,
        head: lexical(
            owner,
            WordForm::Present,
            FeatureBundle {
                number: Some(Number::Singular),
                person: Some(Person::Third),
                tense: Some(Tense::Present),
                finiteness: Some(Finiteness::Finite),
                case: None,
            },
            Some(0),
            None,
        ),
    }
}

#[test]
fn independently_constructed_correlative_gaps_preserve_selected_transitive_frames() {
    let mut both = lexical(
        "vocab:FloatedQuantifier/Both",
        WordForm::Invariant,
        FeatureBundle::default(),
        None,
        None,
    );
    let bare = Reading::BothCoordination {
        category: Category::BareObjectGap,
        form: 0,
        marker: both.clone(),
        left: Box::new(bare_gap("core-verb:Own")),
        coordinator: coordinator(),
        right: Box::new(bare_gap("core-verb:Control")),
    };
    assert_eq!(bare.realize(&LEXICON).unwrap(), "both own and control");
    assert!(readings("both own and control", Category::BareObjectGap).contains(&bare));
    let finite = Reading::BothCoordination {
        category: Category::FiniteObjectGap,
        form: 0,
        marker: both.clone(),
        left: Box::new(finite_gap("core-verb:Own")),
        coordinator: coordinator(),
        right: Box::new(finite_gap("core-verb:Control")),
    };
    assert_eq!(finite.realize(&LEXICON).unwrap(), "both owns and controls");
    assert!(readings("both owns and controls", Category::FiniteObjectGap).contains(&finite));
    both.value = LexicalReading::Word(LexicalValue {
        lexeme: "vocab:Determinative/Neither".into(),
        form: WordForm::Invariant,
        features: FeatureBundle::default(),
        variant: 0,
        capitalization: SurfaceCase::Declared,
    });
    let neither = Reading::NeitherCoordination {
        category: Category::BareObjectGap,
        form: 0,
        marker: both,
        left: Box::new(bare_gap("core-verb:Own")),
        coordinator: lexical(
            "vocab:Coordinator/Nor",
            WordForm::Invariant,
            FeatureBundle::default(),
            None,
            None,
        ),
        right: Box::new(bare_gap("core-verb:Control")),
    };
    assert_eq!(
        neither.realize(&LEXICON).unwrap(),
        "neither own nor control"
    );
    assert!(readings("neither own nor control", Category::BareObjectGap).contains(&neither));
}

#[test]
fn correlative_gaps_enforce_pairing_oxford_lists_and_concord() {
    for text in [
        "both owns and controls",
        "either owns or controls",
        "neither owns nor controls",
        "either owns, controls, or draws",
        "neither owns, controls, nor draws",
    ] {
        assert!(
            !readings(text, Category::FiniteObjectGap).is_empty(),
            "{text}"
        );
    }
    for text in [
        "both own and control",
        "either own or control",
        "neither own nor control",
        "either own, control, or draw",
        "neither own, control, nor draw",
    ] {
        assert!(
            !readings(text, Category::BareObjectGap).is_empty(),
            "{text}"
        );
    }
    for text in [
        "both owns or controls",
        "either owns nor controls",
        "neither owns or controls",
        "both owns, controls, and draws",
        "either owns, controls or draws",
        "neither owns, controls nor draws",
        "both owns and control",
    ] {
        assert!(
            readings(text, Category::FiniteObjectGap).is_empty(),
            "{text}"
        );
    }
    for text in [
        "both own or control",
        "either own nor control",
        "neither own or control",
        "both own, control, and draw",
        "either own, control or draw",
        "neither own, control nor draw",
        "both owns and control",
    ] {
        assert!(readings(text, Category::BareObjectGap).is_empty(), "{text}");
    }
}

#[test]
fn authentic_relative_excerpts_retain_the_unpronounced_shared_object() {
    // Obelisk of Undoing and Conjured Currency respectively; these are exact
    // nominal excerpts, not claims that their full abilities are supported.
    for text in [
        "target permanent you both own and control",
        "target permanent you neither own nor control",
    ] {
        assert!(!readings(text, Category::NounPhrase).is_empty(), "{text}");
    }
}
