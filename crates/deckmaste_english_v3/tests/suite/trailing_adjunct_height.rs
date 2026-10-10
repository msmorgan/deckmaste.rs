use std::collections::BTreeSet;

use crate::common;
use deckmaste_english_v3::grammar::{Category, Reading};

fn coordinated(clause: &Reading) -> bool {
    match clause {
        Reading::ClauseCoordination { .. }
        | Reading::SerialCoordination {
            category: Category::Clause,
            ..
        }
        | Reading::EitherCoordination {
            category: Category::Clause,
            ..
        }
        | Reading::EitherSerialCoordination {
            category: Category::Clause,
            ..
        } => true,
        Reading::ClausalPreposition { clause, .. } => coordinated(clause),
        _ => false,
    }
}

fn clause_complement(pp: &Reading) -> bool {
    match pp {
        Reading::ClauseComplementPreposition { .. }
        | Reading::IrrealisClauseComplementPreposition { .. }
        | Reading::CoordinatedClauseComplementPreposition { .. } => true,
        Reading::FocusedPhrase {
            category: Category::PrepositionPhrase,
            head,
            ..
        } => clause_complement(head),
        Reading::Coordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::EitherCoordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::BothCoordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::NeitherCoordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::CoordinationSeriesEnd {
            category: Category::PrepositionPhraseSeries,
            left,
            right,
            ..
        }
        | Reading::CorrelativeSeriesEnd {
            category: Category::CorrelativePrepositionPhraseSeries,
            left,
            right,
            ..
        } => clause_complement(left) || clause_complement(right),
        Reading::SerialCoordination {
            category: Category::PrepositionPhrase,
            left,
            rest,
            ..
        }
        | Reading::EitherSerialCoordination {
            category: Category::PrepositionPhrase,
            left,
            rest,
            ..
        }
        | Reading::NeitherSerialCoordination {
            category: Category::PrepositionPhrase,
            left,
            rest,
            ..
        }
        | Reading::CoordinationSeriesContinuation {
            category: Category::PrepositionPhraseSeries,
            left,
            rest,
            ..
        }
        | Reading::CorrelativeSeriesContinuation {
            category: Category::CorrelativePrepositionPhraseSeries,
            left,
            rest,
            ..
        } => clause_complement(left) || clause_complement(rest),
        _ => false,
    }
}

fn connective(pp: &Reading) -> bool {
    match pp {
        Reading::IntransitivePreposition { head, .. } => {
            let deckmaste_lexical::LexicalReading::Word(value) = &head.value else {
                return false;
            };
            common::lexicon().lexemes()[&value.lexeme]
                .properties
                .features
                .get("ConnectiveAdjunctUse")
                .is_some_and(|value| value == "Yes")
        }
        Reading::FocusedPhrase {
            category: Category::PrepositionPhrase,
            head,
            ..
        } => connective(head),
        Reading::Coordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::EitherCoordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::BothCoordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::NeitherCoordination {
            category: Category::PrepositionPhrase,
            left,
            right,
            ..
        }
        | Reading::CoordinationSeriesEnd {
            category: Category::PrepositionPhraseSeries,
            left,
            right,
            ..
        }
        | Reading::CorrelativeSeriesEnd {
            category: Category::CorrelativePrepositionPhraseSeries,
            left,
            right,
            ..
        } => connective(left) && connective(right),
        Reading::SerialCoordination {
            category: Category::PrepositionPhrase,
            left,
            rest,
            ..
        }
        | Reading::EitherSerialCoordination {
            category: Category::PrepositionPhrase,
            left,
            rest,
            ..
        }
        | Reading::NeitherSerialCoordination {
            category: Category::PrepositionPhrase,
            left,
            rest,
            ..
        }
        | Reading::CoordinationSeriesContinuation {
            category: Category::PrepositionPhraseSeries,
            left,
            rest,
            ..
        }
        | Reading::CorrelativeSeriesContinuation {
            category: Category::CorrelativePrepositionPhraseSeries,
            left,
            rest,
            ..
        } => connective(left) && connective(rest),
        _ => false,
    }
}

fn check_height(reading: &Reading) {
    reading
        .visit(&mut |node| match node {
            Reading::ClausalPreposition { clause, .. } => assert!(
                coordinated(clause),
                "Adjunct above a simple Clause or trigger: {node:?}"
            ),
            Reading::PrepositionPredicate { head, modifier, .. } => {
                if let Reading::PrepositionPredicate { modifier: preceding, .. } = &**head {
                    assert!(!clause_complement(preceding) || connective(modifier),
                        "Nonconnective Adjunct outside the preceding finite Clause Complement: {node:?}");
                }
            }
            Reading::NominalAdjunctPredicate { head, .. } => {
                if let Reading::PrepositionPredicate { modifier, .. } = &**head {
                    assert!(
                        !clause_complement(modifier),
                        "Adjunct outside the preceding Clause Complement: {node:?}"
                    );
                }
            }
            _ => {}
        })
        .unwrap();
}

#[test]
fn trailing_adjuncts_keep_their_predicate_scope() {
    for (card, text) in [
        (
            "Goliath Daydreamer",
            "Whenever this creature attacks, you may cast a spell from among cards you own in exile with dream counters on them without paying its mana cost.",
        ),
        (
            "Goliath Daydreamer",
            "Whenever you cast an instant or sorcery spell from your hand, exile that card with a dream counter on it instead of putting it into your graveyard as it resolves.",
        ),
        (
            "Belladonna Took",
            "Whenever a token you control enters, you gain 1 life if this is the first time this ability has resolved this turn.",
        ),
        (
            "Vizier of Deferment",
            "When this creature enters, you may exile target creature if it attacked or blocked this turn.",
        ),
        (
            "Veil of Summer",
            "Draw a card if an opponent has cast a blue or black spell this turn.",
        ),
        (
            "Wyluli Wolf",
            "Target creature gets +1/+1 until end of turn.",
        ),
        (
            "Suffocation",
            "Cast this spell only if you were dealt damage this turn by a red instant or sorcery spell.",
        ),
        (
            "Scarlet Witch, Chaotic Avenger",
            "Then you may cast a Hero or noncreature spell from among cards exiled with Scarlet Witch without paying its mana cost.",
        ),
    ] {
        let readings = common::readings_on_card(card, text, Category::Sentence);
        assert!(!readings.is_empty(), "missing Reading for {card}");
        for reading in &readings {
            check_height(reading);
        }
    }
}

#[test]
fn preposed_and_relative_controls_keep_their_readings() {
    for (card, text) in [
        (
            "Suffocation",
            "Suffocation deals 4 damage to the controller of the last red instant or sorcery spell that dealt damage to you this turn.",
        ),
        (
            "Teething Wurmlet",
            "If this is the first time this ability has resolved this turn, put a +1/+1 counter on this creature.",
        ),
    ] {
        let readings = common::readings_on_card(card, text, Category::Sentence);
        assert!(!readings.is_empty(), "missing Reading for {card}");
        for reading in &readings {
            check_height(reading);
        }
    }
}

fn attach_to_coordination(clause: Reading, dependent: Reading) -> Option<Reading> {
    match clause {
        Reading::InitialPreposition {
            form,
            dependent: initial,
            clause,
        } => attach_to_coordination(*clause, dependent).map(|clause| Reading::InitialPreposition {
            form,
            dependent: initial,
            clause: Box::new(clause),
        }),
        clause if coordinated(&clause) => Some(Reading::ClausalPreposition {
            form: 0,
            clause: Box::new(clause),
            dependent: Box::new(dependent),
        }),
        _ => None,
    }
}

fn modal_head(predicate: &Reading) -> bool {
    match predicate {
        Reading::BareAuxiliaryPredicate { .. } => true,
        Reading::PrepositionPredicate { head, .. }
        | Reading::NominalAdjunctPredicate { head, .. } => modal_head(head),
        _ => false,
    }
}

fn sentence(clause: Reading) -> Reading {
    Reading::Sentence {
        form: 0,
        clause: Box::new(clause),
    }
}

fn exact_values(text: &str, expected: &BTreeSet<Reading>, actual: &BTreeSet<Reading>) {
    assert!(!expected.is_empty());
    for reading in expected {
        common::ENVIRONMENT.admit(reading).unwrap();
        assert_eq!(
            common::ENVIRONMENT
                .grammar()
                .realize(reading, common::lexicon())
                .unwrap(),
            text
        );
        check_height(reading);
    }
    assert_eq!(actual, expected);
}

#[test]
fn skyrider_patrol_keeps_every_clause_coordination_attachment() {
    let card = "Skyrider Patrol";
    let text = "When you do, put a +1/+1 counter on another target creature you control, and that creature gains flying until end of turn.";
    let base = text.strip_suffix(" until end of turn.").unwrap();
    let dependents = common::readings("until end of turn", Category::PrepositionPhrase);
    let mut expected = BTreeSet::new();
    for clause in common::readings_on_card(card, base, Category::Clause) {
        for dependent in &dependents {
            if let Some(clause) = attach_to_coordination(clause.clone(), dependent.clone()) {
                expected.insert(sentence(clause));
            }
        }
    }
    let actual = common::readings_on_card(card, text, Category::Sentence)
        .into_iter()
        .filter(|reading| {
            let mut clause_scope = false;
            reading
                .visit(&mut |node| {
                    if let Reading::ClausalPreposition { .. } = node {
                        clause_scope = true;
                    }
                })
                .unwrap();
            clause_scope
        })
        .collect();
    exact_values(text, &expected, &actual);
}

#[test]
fn nissa_keeps_every_adjunct_above_the_modal() {
    let card = "Nissa of Shadowed Boughs";
    let text =
        "You may have it become a 3/3 Elemental creature with haste and menace until end of turn.";
    let base = text.strip_suffix(" until end of turn.").unwrap();
    let mut expected = BTreeSet::new();
    for clause in common::readings_on_card(card, base, Category::Clause) {
        let Reading::Declarative {
            form: declarative_form,
            clause,
        } = clause
        else {
            panic!("expected Declarative");
        };
        let Reading::FiniteClause {
            form,
            subject,
            predicate,
        } = *clause
        else {
            panic!("expected Finite Clause");
        };
        for dependent in common::readings("until end of turn", Category::PrepositionPhrase) {
            expected.insert(sentence(Reading::Declarative {
                form: declarative_form,
                clause: Box::new(Reading::FiniteClause {
                    form,
                    subject: subject.clone(),
                    predicate: Box::new(Reading::PrepositionPredicate {
                        category: Category::FinitePredicate,
                        form: 0,
                        head: predicate.clone(),
                        modifier: Box::new(dependent),
                    }),
                }),
            }));
        }
    }
    let actual = common::readings_on_card(card, text, Category::Sentence)
        .into_iter()
        .filter(|reading| {
            let Reading::Sentence { clause, .. } = reading else {
                return false;
            };
            let Reading::Declarative { clause, .. } = &**clause else {
                return false;
            };
            let Reading::FiniteClause { predicate, .. } = &**clause else {
                return false;
            };
            let Reading::PrepositionPredicate { head, modifier, .. } = &**predicate else {
                return false;
            };
            modal_head(head)
                && common::readings("until end of turn", Category::PrepositionPhrase)
                    .contains(&**modifier)
        })
        .collect();
    exact_values(text, &expected, &actual);
}

#[test]
fn skyrider_patrol_keeps_stacked_clause_coordination_adjuncts() {
    let card = "Skyrider Patrol";
    let base = "When you do, put a +1/+1 counter on another target creature you control, and that creature gains flying";
    let text = format!("{base} until end of turn if you control a creature.");
    let duration = common::readings("until end of turn", Category::PrepositionPhrase);
    let condition = common::readings("if you control a creature", Category::PrepositionPhrase);
    let mut expected = BTreeSet::new();
    for clause in common::readings_on_card(card, base, Category::Clause) {
        for duration in &duration {
            if let Some(inner) = attach_to_coordination(clause.clone(), duration.clone()) {
                for condition in &condition {
                    expected.insert(sentence(
                        attach_to_coordination(inner.clone(), condition.clone()).unwrap(),
                    ));
                }
            }
        }
    }
    let actual = common::readings_on_card(card, &text, Category::Sentence)
        .into_iter()
        .filter(|reading| {
            let Reading::Sentence { clause, .. } = reading else {
                return false;
            };
            let clause = match &**clause {
                Reading::InitialPreposition { clause, .. } => &**clause,
                clause => clause,
            };
            let Reading::ClausalPreposition { clause, .. } = clause else {
                return false;
            };
            matches!(&**clause, Reading::ClausalPreposition { .. })
        })
        .collect();
    exact_values(&text, &expected, &actual);
}

#[test]
fn rulings_keep_every_previously_lost_face_reading() {
    for (card, text) in [
        (
            "Reckless Cohort",
            "This creature attacks each combat if able unless you control another Ally.",
        ),
        (
            "Overload",
            "Kicker {2}\nDestroy target artifact if its mana value is 2 or less. If this spell was kicked, destroy that artifact if its mana value is 5 or less instead.",
        ),
        (
            "Tifa, Martial Artist",
            "Melee\nWhenever one or more creatures you control with power 7 or greater deal combat damage to a player, untap all creatures you control. If it's the first combat phase of your turn, there is an additional combat phase after this phase.",
        ),
        (
            "Shadow of the Second Sun",
            "Enchant player\nAt the beginning of each of enchanted player's postcombat main phases, there is an additional beginning phase after this phase.",
        ),
        (
            "Spelltwine",
            "Exile target instant or sorcery card from your graveyard and target instant or sorcery card from an opponent's graveyard. Copy those cards. Cast the copies if able without paying their mana costs. Exile Spelltwine.",
        ),
        (
            "Sphinx of the Second Sun",
            "Flying\nAt the beginning of each of your postcombat main phases, there is an additional beginning phase after this phase.",
        ),
        (
            "Moraug, Fury of Akoum",
            "Each creature you control gets +1/+0 for each time it has attacked this turn.\nLandfall — Whenever a land you control enters, if it's your main phase, there's an additional combat phase after this phase. At the beginning of that combat, untap all creatures you control.",
        ),
        (
            "Cyclonus, Cybertronian Fighter",
            "Living metal\nFlying\nWhenever Cyclonus deals combat damage to a player, convert it. If you do, there is an additional beginning phase after this phase.",
        ),
        (
            "Marauding Maulhorn",
            "This creature attacks each combat if able unless you control a creature named Advocate of the Beast.",
        ),
        (
            "Raiyuu, Storm's Edge",
            "First strike\nWhenever a Samurai or Warrior you control attacks alone, untap it. If it's the first combat phase of the turn, there is an additional combat phase after this phase.",
        ),
        (
            "Grim Reaper's Sprint",
            "Morbid — This spell costs {3} less to cast if a creature died this turn.\nEnchant creature\nWhen this Aura enters, untap each creature you control. If it's your main phase, there is an additional combat phase after this phase.\nEnchanted creature gets +2/+2 and has haste.",
        ),
        (
            "Y'shtola Rhul",
            "At the beginning of your end step, exile target creature you control, then return it to the battlefield under its owner's control. Then if it's the first end step of the turn, there is an additional end step after this step.",
        ),
        (
            "Great Train Heist",
            "Spree\n+ {2}{R} — Untap all creatures you control. If it's your combat phase, there is an additional combat phase after this phase.\n+ {2} — Creatures you control get +1/+0 and gain first strike until end of turn.\n+ {R} — Choose target opponent. Whenever a creature you control deals combat damage to that player this turn, create a tapped Treasure token.",
        ),
        (
            "Prohibit",
            "Kicker {2}\nCounter target spell if its mana value is 2 or less. If this spell was kicked, counter that spell if its mana value is 4 or less instead.",
        ),
    ] {
        let values = common::readings_on_card(card, text, Category::Document);
        assert!(!values.is_empty(), "missing Reading for {card}");
        for value in &values {
            check_height(value);
        }
    }
}

#[test]
fn existential_predicate_carries_the_trailing_preposition() {
    let base = "there is an additional combat phase";
    let text = "there is an additional combat phase after this phase";
    let mut expected = BTreeSet::new();
    for clause in common::readings(base, Category::FiniteClause) {
        let Reading::FiniteClause {
            form,
            subject,
            predicate,
        } = clause
        else {
            panic!("expected finite Clause");
        };
        for modifier in common::readings("after this phase", Category::PrepositionPhrase) {
            expected.insert(Reading::FiniteClause {
                form,
                subject: subject.clone(),
                predicate: Box::new(Reading::PrepositionPredicate {
                    form: 0,
                    category: Category::ExistentialPredicate,
                    head: predicate.clone(),
                    modifier: Box::new(modifier),
                }),
            });
        }
    }
    let actual = common::readings(text, Category::FiniteClause);
    exact_values(text, &expected, &actual);
}
