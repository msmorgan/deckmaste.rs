use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::FrameValue;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::LexicalValue;
use deckmaste_lexical::Number;
use deckmaste_lexical::SurfaceCase;
use deckmaste_lexical::WordForm;

use crate::common;

pub(super) fn selected_copy(text: &str, copies: &[&str], depictive: bool) {
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty(), "{text}");
    let mut observed_copies = BTreeSet::new();
    for value in values {
        let mut selected = 0;
        value.visit(&mut |node| {
            let complement = match node {
                Reading::SelectedPredicate { head, complements, .. }
                    if matches!(&head.value, LexicalReading::Word(value) if value.lexeme.as_ref() == "core-verb:Enter") => {
                    assert!(!depictive);
                    let [FrameValue::Argument(complement)] = complements.as_slice() else {
                        panic!("Enter must select exactly its predicative Complement: {node:?}");
                    };
                    complement.as_ref()
                }
                Reading::DepictiveSelectedPredicativePredicate { modifier, complement, .. } => {
                    assert!(depictive);
                    assert_eq!(modifier.realize(common::lexicon()).unwrap(), "tapped");
                    complement.as_ref()
                }
                _ => return,
            };
            let phrase = complement;
            let Reading::PredicativeComplementPreposition { complement, .. } = phrase else {
                panic!("expected nominally complemented PP: {phrase:?}");
            };
            observed_copies.insert(phrase.realize(common::lexicon()).unwrap());
            assert_eq!(complement.category(), Category::PredicativeComplement);
            selected += 1;
        }).unwrap();
        assert_eq!(selected, 1, "{text}");
        value.admit(common::lexicon()).unwrap();
    }
    assert_eq!(
        observed_copies,
        copies
            .iter()
            .map(|copy| (*copy).to_owned())
            .collect::<BTreeSet<_>>()
    );
}

#[test]
fn essence_of_the_wild() {
    selected_copy(
        "Creatures you control enter as a copy of this creature.",
        &["as a copy of this creature"],
        false,
    );
}
#[test]
fn mirror_image() {
    selected_copy(
        "You may have this creature enter as a copy of a creature you control.",
        &["as a copy of a creature you control"],
        false,
    );
}
#[test]
fn vesuva() {
    selected_copy(
        "You may have this land enter tapped as a copy of any land on the battlefield.",
        &[
            "as a copy of any land",
            "as a copy of any land on the battlefield",
        ],
        true,
    );
}
#[test]
fn body_double() {
    selected_copy(
        "You may have this creature enter as a copy of any creature card in a graveyard.",
        &[
            "as a copy of any creature card",
            "as a copy of any creature card in a graveyard",
        ],
        false,
    );
}
#[test]
fn masterwork_of_ingenuity() {
    selected_copy(
        "You may have this Equipment enter as a copy of any Equipment on the battlefield.",
        &[
            "as a copy of any Equipment",
            "as a copy of any Equipment on the battlefield",
        ],
        false,
    );
}

fn word(
    id: &str,
    form: WordForm,
    number: Option<Number>,
    frame: Option<usize>,
    countability: Option<bool>,
) -> Word {
    Word {
        value: LexicalReading::Word(LexicalValue {
            lexeme: id.into(),
            form,
            features: FeatureBundle {
                number,
                ..Default::default()
            },
            variant: 0,
            capitalization: SurfaceCase::Declared,
        }),
        frame,
        countability,
    }
}
fn invariant(id: &str) -> Word {
    word(id, WordForm::Invariant, None, None, None)
}
fn accusative(head: Reading) -> Reading {
    Reading::CasePhrase {
        category: Category::AccusativePhrase,
        form: 0,
        head: Box::new(head),
    }
}

#[test]
fn independently_constructed_enter_complement_roundtrips() {
    // Essence of the Wild's Predicate, in its attested plain form.
    let creature = Reading::DeterminedNounPhrase {
        form: 0,
        determiner: invariant("vocab:SingularDemonstrative/This"),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::NominalCoreProjection {
                form: 0,
                head: Box::new(Reading::Noun {
                    form: 0,
                    head: word(
                        "lexeme:type/creature",
                        WordForm::Singular,
                        Some(Number::Singular),
                        None,
                        Some(true),
                    ),
                }),
            }),
        }),
    };
    let copy = Reading::IndefiniteNounPhrase {
        form: 0,
        determiner: word(
            "vocab:Article/Indefinite",
            WordForm::Invariant,
            Some(Number::Singular),
            None,
            None,
        ),
        head: Box::new(Reading::NominalProjection {
            form: 0,
            head: Box::new(Reading::PostmodifiedNominal {
                form: 0,
                head: Box::new(Reading::NominalCoreProjection {
                    form: 0,
                    head: Box::new(Reading::Noun {
                        form: 0,
                        head: word(
                            "lexeme:CommonNoun/Copy",
                            WordForm::Singular,
                            Some(Number::Singular),
                            None,
                            Some(true),
                        ),
                    }),
                }),
                modifier: Box::new(Reading::PrepositionPhrase {
                    form: 0,
                    head: invariant("vocab:Preposition/Of"),
                    complement: Box::new(accusative(creature)),
                }),
            }),
        }),
    };
    let expected = Reading::SelectedPredicate {
        category: Category::SecondaryVerbPhrase,
        form: 0,
        head: word("core-verb:Enter", WordForm::Plain, None, Some(5), None),
        complements: vec![FrameValue::Argument(Box::new(
            Reading::PredicativeComplementPreposition {
                form: 0,
                head: invariant("vocab:Preposition/As"),
                complement: Box::new(Reading::NominalComplement {
                    form: 0,
                    phrase: Box::new(accusative(copy)),
                }),
            },
        ))],
    };
    expected.admit(common::lexicon()).unwrap();
    let text = "enter as a copy of this creature";
    assert_eq!(expected.realize(common::lexicon()).unwrap(), text);
    assert_eq!(
        common::readings(text, Category::SecondaryVerbPhrase),
        BTreeSet::from([expected])
    );
}

#[test]
fn existing_enter_frames_and_adjunct_exclusion_survive() {
    for text in [
        "Creatures you control enter tapped.",
        "You may have this creature enter tapped.",
    ] {
        assert_eq!(common::readings(text, Category::Document).len(), 1);
    }
    assert_eq!(
        common::readings("Activate as a sorcery.", Category::Document).len(),
        1
    );
    let text = "Creatures you control die as a copy of this creature.";
    assert!(
        common::readings(text, Category::Document).is_empty(),
        "{text}"
    );
}

#[test]
fn geths_summons_keeps_finite_as_as_an_adjunct() {
    let text = "Return up to one target creature card from your graveyard to the battlefield.\nCorrupted — For each opponent who has three or more poison counters as you cast this spell, put up to one target creature card from that player's graveyard onto the battlefield under your control.";
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty());
    for value in values {
        let mut selected_predicatives = 0;
        let mut finite_as = false;
        value
            .visit(&mut |node| {
                if let Reading::ClauseComplementPreposition { head, .. } = node
                    && let LexicalReading::Word(word) = &head.value
                    && word.lexeme.as_str() == "vocab:Preposition/As"
                {
                    finite_as = true;
                }
                if let Reading::PredicativePreposition { .. } = node {
                    selected_predicatives += 1;
                }
            })
            .unwrap();
        assert_eq!(selected_predicatives, 0);
        assert!(
            finite_as,
            "as you cast this spell remains a finite as-clause"
        );
    }
}

#[test]
fn master_biomancer_keeps_licensed_unlike_complement_coordination() {
    let text = "Each other creature you control enters with a number of additional +1/+1 counters on it equal to this creature's power and as a Mutant in addition to its other types.";
    let values = common::readings(text, Category::Document);
    assert!(!values.is_empty());
    let mut selected_coordination = false;
    for value in values {
        value.visit(&mut |node| {
            let Reading::SelectedPredicate { head, complements, .. } = node else { return; };
            if !matches!(&head.value, LexicalReading::Word(word) if word.lexeme.as_str() == "core-verb:Enter") { return; }
            let [FrameValue::Argument(complement)] = complements.as_slice() else { return; };
            let Reading::Coordination { category: Category::PrepositionPhrase, left, right, .. } = complement.as_ref() else { return; };
            selected_coordination |= left.realize(common::lexicon()).unwrap().starts_with("with a number of")
                && right.realize(common::lexicon()).unwrap().starts_with("as a Mutant");
        }).unwrap();
        value.admit(common::lexicon()).unwrap();
        assert_eq!(value.realize(common::lexicon()).unwrap(), text);
    }
    assert!(
        selected_coordination,
        "Enter must admit the coordinated with/as complement"
    );
}

#[test]
fn adjunct_only_coordinate_cannot_fill_the_predicative_complement() {
    for text in [
        "This creature enters during your turn and as a Mutant.",
        "This creature enters as a Mutant and during your turn.",
    ] {
        assert!(
            common::readings(text, Category::Document).is_empty(),
            "{text}"
        );
    }
}
