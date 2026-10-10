use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

fn word(
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

// Constituents of Time Stop, Upwelling, coin-flip instructions, life-total
// instructions, storage lands, Ponder, Ancestral Knowledge, Cult of the
// Waxing Moon, and surveil triggers. Pin roles rather than complete units.
type Host = Option<(&'static str, usize, bool)>;
type Row = (
    &'static str,
    Category,
    &'static str,
    Host,
    &'static [(Category, &'static str)],
);
const ROLE_ROWS: &[Row] = &[
    (
        "End the turn.",
        Category::Document,
        "core-verb:End",
        Some(("core-verb:End", 1, false)),
        &[(Category::AccusativePhrase, "the turn")],
    ),
    (
        "steps end",
        Category::FiniteClause,
        "core-verb:End",
        Some(("core-verb:End", 0, false)),
        &[(Category::FinitePredicate, "end")],
    ),
    (
        "lose the flip",
        Category::SecondaryVerbPhrase,
        "lexeme:CommonNoun/Flip",
        Some(("core-verb:Lose", 0, false)),
        &[(Category::AccusativePhrase, "the flip")],
    ),
    (
        "your life total",
        Category::NounPhrase,
        "lexeme:CommonNoun/LifeTotal",
        None,
        &[(Category::Nominal, "life total")],
    ),
    (
        "a mana ability",
        Category::NounPhrase,
        "lexeme:CommonNoun/ManaAbility/compound-noun",
        None,
        &[(Category::Nominal, "mana ability")],
    ),
    (
        "removed from this land",
        Category::SecondaryVerbPhrase,
        "core-verb:Remove",
        Some(("core-verb:Remove", 1, true)),
        &[(Category::SourceComplement, "from this land")],
    ),
    (
        "put them back",
        Category::SecondaryVerbPhrase,
        "vocab:Preposition/Back",
        Some(("core-verb:Put", 1, false)),
        &[(Category::LocativeComplement, "back")],
    ),
    (
        "put them back in any order",
        Category::SecondaryVerbPhrase,
        "vocab:Preposition/Back",
        Some(("core-verb:Put", 4, false)),
        &[
            (Category::LocativeComplement, "back"),
            (Category::MannerComplement, "any order"),
        ],
    ),
    (
        "put the rest back on top of your library in any order",
        Category::SecondaryVerbPhrase,
        "vocab:Preposition/Back",
        Some(("core-verb:Put", 6, false)),
        &[
            (Category::LocativeComplement, "on top of your library"),
            (Category::MannerComplement, "any order"),
        ],
    ),
    (
        "It was put there.",
        Category::Document,
        "vocab:Preposition/There",
        None,
        &[(Category::LocativeComplement, "there")],
    ),
    (
        "this game",
        Category::NominalAdjunctPhrase,
        "lexeme:CommonNoun/Game",
        None,
        &[(Category::NominalAdjunctPhrase, "this game")],
    ),
    (
        "any time",
        Category::NominalAdjunctPhrase,
        "vocab:Determinative/Any",
        None,
        &[(Category::NominalAdjunctPhrase, "any time")],
    ),
    (
        "transforms",
        Category::FinitePredicate,
        "lexeme:keyword_action/transform",
        Some(("lexeme:keyword_action/transform", 1, false)),
        &[(Category::FinitePredicate, "transforms")],
    ),
    (
        "transforms into a non-Human creature",
        Category::FinitePredicate,
        "lexeme:keyword_action/transform",
        Some(("lexeme:keyword_action/transform", 2, false)),
        &[(Category::AccusativePhrase, "a non-Human creature")],
    ),
    (
        "surveil",
        Category::SecondaryVerbPhrase,
        "lexeme:keyword_action/surveil",
        Some(("lexeme:keyword_action/surveil", 1, false)),
        &[(Category::SecondaryVerbPhrase, "surveil")],
    ),
];

#[test]
fn declarations_fill_attested_roles_and_preserve_both_roundtrip_laws() {
    for &(text, category, owner, host, roles) in ROLE_ROWS {
        let values = common::readings(text, category);
        assert!(!values.is_empty(), "{text}");
        for value in values {
            let mut owners = vec![];
            value
                .visit_words(&mut |word| {
                    if let LexicalReading::Word(lexical) = &word.value
                        && lexical.lexeme.as_str() == owner
                    {
                        owners.push(lexical.lexeme);
                    }
                })
                .unwrap();
            assert_eq!(
                owners,
                vec![deckmaste_lexical::LexemeId::new(owner)],
                "{text}"
            );
            for &(role, surface) in roles {
                let expected = common::readings(surface, role);
                let mut actual = vec![];
                value
                    .visit(&mut |node| {
                        if node.category() == role
                            && node.realize(common::lexicon()).unwrap() == surface
                        {
                            actual.push(node.clone());
                        }
                    })
                    .unwrap();
                assert!(!actual.is_empty(), "{text}: missing {role:?} {surface:?}");
                for actual in actual {
                    assert!(expected.contains(&actual), "{text}: {actual:?}");
                }
            }
            if let Some((owner, frame, passive)) = host {
                let mut actual = vec![];
                value
                    .visit(&mut |node| {
                        let (head, is_passive) = match node {
                            Reading::SelectedPredicate { head, .. } => (head, false),
                            Reading::SelectedFramePassive { head, .. } => (head, true),
                            _ => return,
                        };
                        if let LexicalReading::Word(lexical) = &head.value
                            && lexical.lexeme.as_str() == owner
                        {
                            actual.push((head.frame, is_passive));
                        }
                    })
                    .unwrap();
                assert_eq!(actual, vec![(Some(frame), passive)], "{text}");
            }
        }
    }
    assert_eq!(
        common::readings("combat", Category::NounPhrase),
        BTreeSet::new()
    );
    for (owner, frame, text) in [
        ("core-verb:End", 0, "end"),
        ("lexeme:keyword_action/transform", 1, "transform"),
        ("lexeme:keyword_action/surveil", 1, "surveil"),
    ] {
        let expected = Reading::SelectedPredicate {
            category: Category::SecondaryVerbPhrase,
            form: 0,
            head: word(
                owner,
                WordForm::Plain,
                FeatureBundle::default(),
                Some(frame),
                None,
            ),
            complements: vec![],
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::SecondaryVerbPhrase),
            BTreeSet::from([expected])
        );
    }
    for (owner, text) in [
        ("lexeme:CommonNoun/Flip", "flip"),
        ("lexeme:CommonNoun/LifeTotal", "life total"),
        (
            "lexeme:CommonNoun/ManaAbility/compound-noun",
            "mana ability",
        ),
    ] {
        let expected = Reading::Noun {
            form: 0,
            head: word(
                owner,
                WordForm::Singular,
                FeatureBundle {
                    number: Some(Number::Singular),
                    ..Default::default()
                },
                None,
                Some(true),
            ),
        };
        expected.admit(common::lexicon()).unwrap();
        assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
        assert_eq!(
            common::readings(text, Category::Nominal),
            BTreeSet::from([expected])
        );
    }
    let complement = common::readings("from this land", Category::SourceComplement)
        .into_iter()
        .next()
        .unwrap();
    let passive = Reading::SelectedFramePassive {
        form: 0,
        head: word(
            "core-verb:Remove",
            WordForm::PastParticiple,
            FeatureBundle {
                finiteness: Some(Finiteness::Nonfinite),
                ..Default::default()
            },
            Some(1),
            None,
        ),
        complements: vec![FrameValue::Argument(Box::new(complement))],
    };
    passive.admit(common::lexicon()).unwrap();
    assert_eq!(
        passive.realize(common::lexicon()).unwrap(),
        "removed from this land"
    );
    assert_eq!(
        common::readings("removed from this land", Category::SecondaryVerbPhrase),
        BTreeSet::from([passive])
    );
}

#[test]
fn combat_source_deferrals_follow_the_selected_complement_route() {
    for (text, category) in [
        ("Remove it from combat.", Category::Document),
        ("removed from combat", Category::SecondaryVerbPhrase),
    ] {
        common::assert_constituents(
            text,
            category,
            &[
                (Category::SourceComplement, "from combat"),
                (Category::BareTemporalNominal, "combat"),
            ],
        );
    }
}
