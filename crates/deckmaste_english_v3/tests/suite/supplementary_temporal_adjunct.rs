use std::collections::BTreeSet;

use deckmaste_english_v3::grammar::{Category, Reading, Word};
use deckmaste_lexical::{LexicalReading, Number, Person, SurfaceCase, WordForm};

use crate::common::{lexicon, readings, readings_on_card};

const WITNESS: &str = "Activate only during your turn, before attackers are declared.";

fn word(owner: &str, form: WordForm, number: Option<Number>) -> Word {
    let value = lexicon()
        .values()
        .find(|value| {
            value.lexeme.as_str() == owner
                && value.form == form
                && value.variant == 0
                && number.is_none_or(|number| value.features.number == Some(number))
                && (form != WordForm::Present || value.features.person == Some(Person::Third))
        })
        .unwrap()
        .clone();
    let lexeme = &lexicon().lexemes()[&value.lexeme];
    Word {
        value: LexicalReading::Word(value),
        frame: None,
        countability: (!lexeme.properties.countability.is_empty()).then_some(true),
    }
}

fn expected() -> Reading {
    let mut activate = word("lexeme:keyword_action/activate", WordForm::Plain, None);
    activate.frame = Some(0);
    let LexicalReading::Word(value) = &mut activate.value else { unreachable!() };
    value.capitalization = SurfaceCase::Initial;
    let mut copula = word("core-verb:Be", WordForm::Present, Some(Number::Plural));
    copula.frame = Some(1);
    let mut declare = word("core-verb:Declare", WordForm::PastParticiple, None);
    declare.frame = Some(0);
    Reading::Supplementation {
        category: Category::Sentence,
        form: 0,
        anchor: Box::new(Reading::Imperative {
            form: 0,
            predicate: Box::new(Reading::BarePredicate {
                form: 0,
                head: Box::new(Reading::PrepositionPredicate {
                    category: Category::SecondaryVerbPhrase,
                    form: 0,
                    head: Box::new(Reading::SelectedPredicate {
                        category: Category::SecondaryVerbPhrase,
                        form: 0,
                        head: activate,
                        complements: vec![],
                    }),
                    modifier: Box::new(Reading::FocusedPhrase {
                        category: Category::PrepositionPhrase,
                        form: 0,
                        modifier: word("vocab:FocusAdverb/Only", WordForm::Invariant, None),
                        head: Box::new(Reading::PrepositionPhrase {
                            form: 0,
                            head: word("vocab:Preposition/During", WordForm::Invariant, None),
                            complement: Box::new(Reading::CasePhrase {
                                category: Category::AccusativePhrase,
                                form: 0,
                                head: Box::new(Reading::PossessiveNounPhrase {
                                    form: 0,
                                    possessor: word(
                                        "vocab:PossessiveDeterminerPronoun/Your",
                                        WordForm::Invariant,
                                        Some(Number::Singular),
                                    ),
                                    head: Box::new(Reading::Noun {
                                        form: 0,
                                        head: word(
                                            "lexeme:CommonNoun/Turn",
                                            WordForm::Singular,
                                            None,
                                        ),
                                    }),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        }),
        supplement: Box::new(Reading::ClauseComplementPreposition {
            form: 0,
            head: word("vocab:Preposition/Before", WordForm::Invariant, None),
            complement: Box::new(Reading::FiniteClause {
                form: 0,
                subject: Box::new(Reading::CasePhrase {
                    category: Category::NominativePhrase,
                    form: 0,
                    head: Box::new(Reading::BareNounPhrase {
                        form: 0,
                        head: Box::new(Reading::Noun {
                            form: 0,
                            head: word("lexeme:CommonNoun/Attacker", WordForm::Plural, None),
                        }),
                    }),
                }),
                predicate: Box::new(Reading::ParticipialAuxiliaryPredicate {
                    category: Category::FinitePredicate,
                    form: 0,
                    head: copula,
                    complement: Box::new(Reading::OvertComplement {
                        category: Category::ParticipialComplement,
                        form: 0,
                        predicate: Box::new(Reading::PassiveComplement {
                            form: 0,
                            head: Box::new(Reading::PassivePredicate {
                                form: 0,
                                head: declare,
                            }),
                        }),
                    }),
                }),
            }),
        }),
    }
}

#[test]
fn witnesses_supplement_the_clause_outside_the_focused_adjunct() {
    // ST1–ST3: the comma marks supplementation; only focuses during your turn.
    let expected = expected();
    for card in [
        "Talas Researcher",
        "King's Assassin",
        "Stern Marshal",
        "Lady Sun",
        "Cao Cao, Lord of Wei",
    ] {
        assert_eq!(
            readings_on_card(card, WITNESS, Category::Sentence),
            BTreeSet::from([expected.clone()]),
            "{card}",
        );
    }
}

#[test]
fn independently_constructed_supplement_preserves_both_roundtrip_laws() {
    let expected = expected();
    expected.admit(lexicon()).unwrap();
    assert_eq!(expected.realize(lexicon()).unwrap(), WITNESS);
    assert_eq!(
        readings(WITNESS, Category::Sentence),
        BTreeSet::from([expected])
    );
}

#[test]
fn controls_keep_integrated_adjuncts_and_their_focus() {
    for (text, focus) in [
        ("Activate only during your turn.", Some("during your turn")),
        (
            "Activate only before attackers are declared.",
            Some("before attackers are declared"),
        ),
        (
            "Activate only during your turn before attackers are declared.",
            Some("during your turn"),
        ),
        (
            "Cast this spell only during combat before blockers are declared.",
            Some("during combat"),
        ),
        (
            "Cast this spell only before attackers are declared.",
            Some("before attackers are declared"),
        ),
        ("Attackers are declared.", None),
    ] {
        let observed: Vec<_> = readings(text, Category::Sentence)
            .into_iter()
            .map(|reading| {
                let mut focus = Vec::new();
                let mut supplements = Vec::new();
                reading
                    .visit(&mut |node| {
                        if let Reading::FocusedPhrase { head, .. } = node {
                            focus.push(head.realize(lexicon()).unwrap());
                        }
                        if let Reading::Supplementation { supplement, .. } = node {
                            supplements.push(supplement.realize(lexicon()).unwrap());
                        }
                    })
                    .unwrap();
                (focus, supplements)
            })
            .collect();
        assert_eq!(
            observed,
            vec![(
                focus.into_iter().map(str::to_owned).collect(),
                Vec::<String>::new()
            )],
            "{text}"
        );
    }
}

#[test]
fn supplement_licence_does_not_admit_medial_or_focused_supplements() {
    // ST3–ST4: negative localization strings, not invented positive witnesses.
    let Reading::Supplementation {
        anchor, supplement, ..
    } = expected()
    else {
        unreachable!()
    };
    let mut activate = word("lexeme:keyword_action/activate", WordForm::Plain, None);
    activate.frame = Some(0);
    let medial_value = Reading::MedialSupplementation {
        form: 0,
        continuation: Box::new(Reading::Imperative {
            form: 0,
            predicate: Box::new(Reading::BarePredicate {
                form: 0,
                head: Box::new(Reading::SelectedPredicate {
                    category: Category::SecondaryVerbPhrase,
                    form: 0,
                    head: activate,
                    complements: vec![],
                }),
            }),
        }),
        anchor,
        supplement,
        coordinator: word("vocab:Coordinator/Then", WordForm::Invariant, None),
    };
    assert!(medial_value.admit(lexicon()).is_err());
    assert!(medial_value.realize(lexicon()).is_err());
    let medial = "Activate only during your turn, before attackers are declared, then draw a card.";
    for reading in readings(medial, Category::Sentence) {
        let mut supplements = Vec::new();
        reading
            .visit(&mut |node| {
                if let Reading::MedialSupplementation { supplement, .. } = node {
                    supplements.push(supplement.realize(lexicon()).unwrap());
                }
            })
            .unwrap();
        assert_eq!(supplements, Vec::<String>::new());
    }
    assert!(
        readings(
            "Activate only during your turn, only before attackers are declared.",
            Category::Sentence,
        )
        .is_empty()
    );
}
