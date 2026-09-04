//! `event` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::PhaseKind {
    type Target = deckmaste_core::PhaseKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Beginning => deckmaste_core::PhaseKind::Beginning,
            Self::PrecombatMain => deckmaste_core::PhaseKind::PrecombatMain,
            Self::Combat => deckmaste_core::PhaseKind::Combat,
            Self::PostcombatMain => deckmaste_core::PhaseKind::PostcombatMain,
            Self::Ending => deckmaste_core::PhaseKind::Ending,
        }
    }
}

impl Lower for deckmaste_semantics::PhaseStep {
    type Target = deckmaste_core::PhaseStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Beginning(f0) => deckmaste_core::PhaseStep::Beginning(f0.lower()),
            Self::PrecombatMain => deckmaste_core::PhaseStep::PrecombatMain,
            Self::Combat(f0) => deckmaste_core::PhaseStep::Combat(f0.lower()),
            Self::PostcombatMain => deckmaste_core::PhaseStep::PostcombatMain,
            Self::Ending(f0) => deckmaste_core::PhaseStep::Ending(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::BeginningStep {
    type Target = deckmaste_core::BeginningStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Untap => deckmaste_core::BeginningStep::Untap,
            Self::Upkeep => deckmaste_core::BeginningStep::Upkeep,
            Self::Draw => deckmaste_core::BeginningStep::Draw,
        }
    }
}

impl Lower for deckmaste_semantics::CombatStep {
    type Target = deckmaste_core::CombatStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::BeginningOfCombat => deckmaste_core::CombatStep::BeginningOfCombat,
            Self::DeclareAttackers => deckmaste_core::CombatStep::DeclareAttackers,
            Self::DeclareBlockers => deckmaste_core::CombatStep::DeclareBlockers,
            Self::FirstCombatDamage => deckmaste_core::CombatStep::FirstCombatDamage,
            Self::CombatDamage => deckmaste_core::CombatStep::CombatDamage,
            Self::EndOfCombat => deckmaste_core::CombatStep::EndOfCombat,
        }
    }
}

impl Lower for deckmaste_semantics::EndingStep {
    type Target = deckmaste_core::EndingStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::End => deckmaste_core::EndingStep::End,
            Self::Cleanup => deckmaste_core::EndingStep::Cleanup,
        }
    }
}

impl Lower for deckmaste_semantics::WhoseTurn {
    type Target = deckmaste_core::WhoseTurn;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Your => deckmaste_core::WhoseTurn::Your,
            Self::EachPlayers => deckmaste_core::WhoseTurn::EachPlayers,
            Self::AnOpponents => deckmaste_core::WhoseTurn::AnOpponents,
        }
    }
}

impl Lower for deckmaste_semantics::StateChange {
    type Target = deckmaste_core::StateChange;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => deckmaste_core::StateChange::Tapped,
            Self::Untapped => deckmaste_core::StateChange::Untapped,
            Self::Phased(f0) => deckmaste_core::StateChange::Phased(f0.lower()),
            Self::TurnedFace(f0) => deckmaste_core::StateChange::TurnedFace(f0.lower()),
            Self::Transformed => deckmaste_core::StateChange::Transformed,
        }
    }
}

impl Lower for deckmaste_semantics::Agency {
    type Target = deckmaste_core::Agency;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::CostPayment => deckmaste_core::Agency::CostPayment,
            Self::AttackDeclaration => deckmaste_core::Agency::AttackDeclaration,
            Self::EffectInstruction => deckmaste_core::Agency::EffectInstruction,
            Self::TurnBasedAction => deckmaste_core::Agency::TurnBasedAction,
            Self::StateBasedAction => deckmaste_core::Agency::StateBasedAction,
            Self::ManaAbilityResolution => deckmaste_core::Agency::ManaAbilityResolution,
            Self::SpecialAction => deckmaste_core::Agency::SpecialAction,
        }
    }
}

impl Lower for deckmaste_semantics::VerbName {
    type Target = deckmaste_core::VerbName;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::VerbName(self.0.lower())
    }
}

impl Lower for deckmaste_semantics::CausePattern {
    type Target = deckmaste_core::CausePattern;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CausePattern {
            verb: self.verb.lower(),
            agency: self.agency.lower(),
            agent: self.agent.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Cause {
    type Target = deckmaste_core::Cause;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Cause(f0) => deckmaste_core::Cause::Cause(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::EventFilter {
    type Target = deckmaste_core::EventFilter;
    #[allow(
        clippy::too_many_lines,
        reason = "exhaustive one-arm-per-variant map; splitting it would hide the exhaustiveness check"
    )]
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ZoneChange {
                what,
                from,
                to,
                cause,
            } => deckmaste_core::EventFilter::ZoneChange {
                what: what.lower(),
                from: from.lower(),
                to: to.lower(),
                cause: cause.lower(),
            },
            Self::Damage {
                source,
                to,
                combat,
                amount,
            } => deckmaste_core::EventFilter::Damage {
                source: source.lower(),
                to: to.lower(),
                combat: combat.lower(),
                amount: amount.lower(),
            },
            Self::LifeGained { who, amount } => deckmaste_core::EventFilter::LifeGained {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::LifeLost { who, amount } => deckmaste_core::EventFilter::LifeLost {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::Drawn { who, amount } => deckmaste_core::EventFilter::Drawn {
                who: who.lower(),
                amount: amount.lower(),
            },
            Self::Act {
                verb,
                who,
                on,
                cause,
            } => deckmaste_core::EventFilter::Act {
                verb: verb.lower(),
                who: who.lower(),
                on: on.lower(),
                cause: cause.lower(),
            },
            Self::CounterPlaced {
                kind,
                on,
                amount,
                cause,
            } => deckmaste_core::EventFilter::CounterPlaced {
                kind: kind.lower(),
                on: on.lower(),
                amount: amount.lower(),
                cause: cause.lower(),
            },
            Self::CounterRemoved {
                kind,
                on,
                amount,
                cause,
            } => deckmaste_core::EventFilter::CounterRemoved {
                kind: kind.lower(),
                on: on.lower(),
                amount: amount.lower(),
                cause: cause.lower(),
            },
            Self::Cast { who, what } => deckmaste_core::EventFilter::Cast {
                who: who.lower(),
                what: what.lower(),
            },
            Self::Copied { who, what } => deckmaste_core::EventFilter::Copied {
                who: who.lower(),
                what: what.lower(),
            },
            Self::Played { who, what } => deckmaste_core::EventFilter::Played {
                who: who.lower(),
                what: what.lower(),
            },
            Self::ActivatedAb { who, what } => deckmaste_core::EventFilter::ActivatedAb {
                who: who.lower(),
                what: what.lower(),
            },
            Self::ManaAbilityActivated { what, by } => {
                deckmaste_core::EventFilter::ManaAbilityActivated {
                    what: what.lower(),
                    by: by.lower(),
                }
            }
            Self::ManaProduced { what, by } => deckmaste_core::EventFilter::ManaProduced {
                what: what.lower(),
                by: by.lower(),
            },
            Self::ManaAdded { what, by } => deckmaste_core::EventFilter::ManaAdded {
                what: what.lower(),
                by: by.lower(),
            },
            Self::AttackDeclared { by, against } => deckmaste_core::EventFilter::AttackDeclared {
                by: by.lower(),
                against: against.lower(),
            },
            Self::BlockDeclared { by, of } => deckmaste_core::EventFilter::BlockDeclared {
                by: by.lower(),
                of: of.lower(),
            },
            Self::Attached { what, to } => deckmaste_core::EventFilter::Attached {
                what: what.lower(),
                to: to.lower(),
            },
            Self::StateBecame { of, becomes, cause } => deckmaste_core::EventFilter::StateBecame {
                of: of.lower(),
                becomes: becomes.lower(),
                cause: cause.lower(),
            },
            Self::BecomesTarget { what, by, source } => {
                deckmaste_core::EventFilter::BecomesTarget {
                    what: what.lower(),
                    by: by.lower(),
                    source: source.lower(),
                }
            }
            Self::StepBegins { at, whose } => deckmaste_core::EventFilter::StepBegins {
                at: at.lower(),
                whose: whose.lower(),
            },
            Self::ControlChanged { of, to } => deckmaste_core::EventFilter::ControlChanged {
                of: of.lower(),
                to: to.lower(),
            },
            Self::DesignationChanged { name, of, to } => {
                deckmaste_core::EventFilter::DesignationChanged {
                    name: name.lower(),
                    of: of.lower(),
                    to: to.lower(),
                }
            }
            Self::TokenCreated { what, by } => deckmaste_core::EventFilter::TokenCreated {
                what: what.lower(),
                by: by.lower(),
            },
            Self::Shuffled { by } => deckmaste_core::EventFilter::Shuffled { by: by.lower() },
            Self::Revealed { what } => deckmaste_core::EventFilter::Revealed { what: what.lower() },
            Self::Used { of } => deckmaste_core::EventFilter::Used { of: of.lower() },
            Self::CoinFlipped { by, won } => deckmaste_core::EventFilter::CoinFlipped {
                by: by.lower(),
                won: won.lower(),
            },
            Self::DiceRolled { by } => deckmaste_core::EventFilter::DiceRolled { by: by.lower() },
            Self::TapForMana { what, by } => deckmaste_core::EventFilter::TapForMana {
                what: what.lower(),
                by: by.lower(),
            },
            Self::RollPlanarDie { by, face } => deckmaste_core::EventFilter::RollPlanarDie {
                by: by.lower(),
                face: face.lower(),
            },
            Self::AllOf(f0) => deckmaste_core::EventFilter::AllOf(f0.lower()),
            Self::OneOf(f0) => deckmaste_core::EventFilter::OneOf(f0.lower()),
            Self::Not(f0) => deckmaste_core::EventFilter::Not(f0.lower()),
            Self::OneOrMore(f0) => deckmaste_core::EventFilter::OneOrMore(f0.lower()),
            Self::Nth { n, of, within } => deckmaste_core::EventFilter::Nth {
                n: n.lower(),
                of: of.lower(),
                within: within.lower(),
            },
            Self::When(f0, f1) => deckmaste_core::EventFilter::When(f0.lower(), f1.lower()),
            Self::Within(f0, f1) => deckmaste_core::EventFilter::Within(f0.lower(), f1.lower()),
            Self::Before(f0) => deckmaste_core::EventFilter::Before(f0.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_phase_kind_beginning() {
        assert_matches!(
            deckmaste_semantics::PhaseKind::Beginning.lower(),
            deckmaste_core::PhaseKind::Beginning
        );
    }

    #[test]
    fn lowers_phase_kind_precombat_main() {
        assert_matches!(
            deckmaste_semantics::PhaseKind::PrecombatMain.lower(),
            deckmaste_core::PhaseKind::PrecombatMain
        );
    }

    #[test]
    fn lowers_phase_kind_combat() {
        assert_matches!(
            deckmaste_semantics::PhaseKind::Combat.lower(),
            deckmaste_core::PhaseKind::Combat
        );
    }

    #[test]
    fn lowers_phase_kind_postcombat_main() {
        assert_matches!(
            deckmaste_semantics::PhaseKind::PostcombatMain.lower(),
            deckmaste_core::PhaseKind::PostcombatMain
        );
    }

    #[test]
    fn lowers_phase_kind_ending() {
        assert_matches!(
            deckmaste_semantics::PhaseKind::Ending.lower(),
            deckmaste_core::PhaseKind::Ending
        );
    }

    #[test]
    fn lowers_phase_step_beginning() {
        assert_matches!(
            deckmaste_semantics::PhaseStep::Beginning(minimal_beginning_step()).lower(),
            deckmaste_core::PhaseStep::Beginning(deckmaste_core::BeginningStep::Untap)
        );
    }

    #[test]
    fn lowers_phase_step_precombat_main() {
        assert_matches!(
            deckmaste_semantics::PhaseStep::PrecombatMain.lower(),
            deckmaste_core::PhaseStep::PrecombatMain
        );
    }

    #[test]
    fn lowers_phase_step_combat() {
        assert_matches!(
            deckmaste_semantics::PhaseStep::Combat(minimal_combat_step()).lower(),
            deckmaste_core::PhaseStep::Combat(deckmaste_core::CombatStep::BeginningOfCombat)
        );
    }

    #[test]
    fn lowers_phase_step_postcombat_main() {
        assert_matches!(
            deckmaste_semantics::PhaseStep::PostcombatMain.lower(),
            deckmaste_core::PhaseStep::PostcombatMain
        );
    }

    #[test]
    fn lowers_phase_step_ending() {
        assert_matches!(
            deckmaste_semantics::PhaseStep::Ending(minimal_ending_step()).lower(),
            deckmaste_core::PhaseStep::Ending(deckmaste_core::EndingStep::End)
        );
    }

    #[test]
    fn lowers_beginning_step_untap() {
        assert_matches!(
            deckmaste_semantics::BeginningStep::Untap.lower(),
            deckmaste_core::BeginningStep::Untap
        );
    }

    #[test]
    fn lowers_beginning_step_upkeep() {
        assert_matches!(
            deckmaste_semantics::BeginningStep::Upkeep.lower(),
            deckmaste_core::BeginningStep::Upkeep
        );
    }

    #[test]
    fn lowers_beginning_step_draw() {
        assert_matches!(
            deckmaste_semantics::BeginningStep::Draw.lower(),
            deckmaste_core::BeginningStep::Draw
        );
    }

    #[test]
    fn lowers_combat_step_beginning_of_combat() {
        assert_matches!(
            deckmaste_semantics::CombatStep::BeginningOfCombat.lower(),
            deckmaste_core::CombatStep::BeginningOfCombat
        );
    }

    #[test]
    fn lowers_combat_step_declare_attackers() {
        assert_matches!(
            deckmaste_semantics::CombatStep::DeclareAttackers.lower(),
            deckmaste_core::CombatStep::DeclareAttackers
        );
    }

    #[test]
    fn lowers_combat_step_declare_blockers() {
        assert_matches!(
            deckmaste_semantics::CombatStep::DeclareBlockers.lower(),
            deckmaste_core::CombatStep::DeclareBlockers
        );
    }

    #[test]
    fn lowers_combat_step_first_combat_damage() {
        assert_matches!(
            deckmaste_semantics::CombatStep::FirstCombatDamage.lower(),
            deckmaste_core::CombatStep::FirstCombatDamage
        );
    }

    #[test]
    fn lowers_combat_step_combat_damage() {
        assert_matches!(
            deckmaste_semantics::CombatStep::CombatDamage.lower(),
            deckmaste_core::CombatStep::CombatDamage
        );
    }

    #[test]
    fn lowers_combat_step_end_of_combat() {
        assert_matches!(
            deckmaste_semantics::CombatStep::EndOfCombat.lower(),
            deckmaste_core::CombatStep::EndOfCombat
        );
    }

    #[test]
    fn lowers_ending_step_end() {
        assert_matches!(
            deckmaste_semantics::EndingStep::End.lower(),
            deckmaste_core::EndingStep::End
        );
    }

    #[test]
    fn lowers_ending_step_cleanup() {
        assert_matches!(
            deckmaste_semantics::EndingStep::Cleanup.lower(),
            deckmaste_core::EndingStep::Cleanup
        );
    }

    #[test]
    fn lowers_whose_turn_your() {
        assert_matches!(
            deckmaste_semantics::WhoseTurn::Your.lower(),
            deckmaste_core::WhoseTurn::Your
        );
    }

    #[test]
    fn lowers_whose_turn_each_players() {
        assert_matches!(
            deckmaste_semantics::WhoseTurn::EachPlayers.lower(),
            deckmaste_core::WhoseTurn::EachPlayers
        );
    }

    #[test]
    fn lowers_whose_turn_an_opponents() {
        assert_matches!(
            deckmaste_semantics::WhoseTurn::AnOpponents.lower(),
            deckmaste_core::WhoseTurn::AnOpponents
        );
    }

    #[test]
    fn lowers_state_change_tapped() {
        assert_matches!(
            deckmaste_semantics::StateChange::Tapped.lower(),
            deckmaste_core::StateChange::Tapped
        );
    }

    #[test]
    fn lowers_state_change_untapped() {
        assert_matches!(
            deckmaste_semantics::StateChange::Untapped.lower(),
            deckmaste_core::StateChange::Untapped
        );
    }

    #[test]
    fn lowers_state_change_phased() {
        assert_matches!(
            deckmaste_semantics::StateChange::Phased(minimal_phasing()).lower(),
            deckmaste_core::StateChange::Phased(deckmaste_core::Phasing::In)
        );
    }

    #[test]
    fn lowers_state_change_turned_face() {
        assert_matches!(
            deckmaste_semantics::StateChange::TurnedFace(minimal_face()).lower(),
            deckmaste_core::StateChange::TurnedFace(deckmaste_core::Face::Up)
        );
    }

    #[test]
    fn lowers_state_change_transformed() {
        assert_matches!(
            deckmaste_semantics::StateChange::Transformed.lower(),
            deckmaste_core::StateChange::Transformed
        );
    }

    #[test]
    fn lowers_agency_cost_payment() {
        assert_matches!(
            deckmaste_semantics::Agency::CostPayment.lower(),
            deckmaste_core::Agency::CostPayment
        );
    }

    #[test]
    fn lowers_agency_attack_declaration() {
        assert_matches!(
            deckmaste_semantics::Agency::AttackDeclaration.lower(),
            deckmaste_core::Agency::AttackDeclaration
        );
    }

    #[test]
    fn lowers_agency_effect_instruction() {
        assert_matches!(
            deckmaste_semantics::Agency::EffectInstruction.lower(),
            deckmaste_core::Agency::EffectInstruction
        );
    }

    #[test]
    fn lowers_agency_turn_based_action() {
        assert_matches!(
            deckmaste_semantics::Agency::TurnBasedAction.lower(),
            deckmaste_core::Agency::TurnBasedAction
        );
    }

    #[test]
    fn lowers_agency_state_based_action() {
        assert_matches!(
            deckmaste_semantics::Agency::StateBasedAction.lower(),
            deckmaste_core::Agency::StateBasedAction
        );
    }

    #[test]
    fn lowers_agency_mana_ability_resolution() {
        assert_matches!(
            deckmaste_semantics::Agency::ManaAbilityResolution.lower(),
            deckmaste_core::Agency::ManaAbilityResolution
        );
    }

    #[test]
    fn lowers_agency_special_action() {
        assert_matches!(
            deckmaste_semantics::Agency::SpecialAction.lower(),
            deckmaste_core::Agency::SpecialAction
        );
    }

    #[test]
    fn lowers_verb_name() {
        assert_matches!(
            deckmaste_semantics::VerbName("X".into()).lower(),
            deckmaste_core::VerbName(_)
        );
    }

    #[test]
    fn lowers_cause_pattern() {
        assert_matches!(
            deckmaste_semantics::CausePattern {
                verb: None,
                agency: None,
                agent: None
            }
            .lower(),
            deckmaste_core::CausePattern {
                verb: None,
                agency: None,
                agent: None
            }
        );
    }

    #[test]
    fn lowers_cause_cause() {
        assert_matches!(
            deckmaste_semantics::Cause::Cause(minimal_cause_pattern()).lower(),
            deckmaste_core::Cause::Cause(deckmaste_core::CausePattern {
                verb: None,
                agency: None,
                agent: None
            })
        );
    }

    #[test]
    fn lowers_event_filter_zone_change() {
        assert_matches!(
            deckmaste_semantics::EventFilter::ZoneChange {
                what: minimal_predicate(),
                from: None,
                to: None,
                cause: None
            }
            .lower(),
            deckmaste_core::EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_damage() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Damage {
                source: minimal_predicate(),
                to: minimal_predicate(),
                combat: None,
                amount: None
            }
            .lower(),
            deckmaste_core::EventFilter::Damage {
                source: deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                ),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                combat: None,
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_life_gained() {
        assert_matches!(
            deckmaste_semantics::EventFilter::LifeGained {
                who: minimal_predicate(),
                amount: None
            }
            .lower(),
            deckmaste_core::EventFilter::LifeGained {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_life_lost() {
        assert_matches!(
            deckmaste_semantics::EventFilter::LifeLost {
                who: minimal_predicate(),
                amount: None
            }
            .lower(),
            deckmaste_core::EventFilter::LifeLost {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_drawn() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Drawn {
                who: minimal_predicate(),
                amount: None
            }
            .lower(),
            deckmaste_core::EventFilter::Drawn {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                amount: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_act() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Act {
                verb: minimal_verb_name(),
                who: minimal_predicate(),
                on: minimal_predicate(),
                cause: None
            }
            .lower(),
            deckmaste_core::EventFilter::Act {
                verb: deckmaste_core::VerbName(_),
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_counter_placed() {
        assert_matches!(
            deckmaste_semantics::EventFilter::CounterPlaced {
                kind: None,
                on: minimal_predicate(),
                amount: None,
                cause: None
            }
            .lower(),
            deckmaste_core::EventFilter::CounterPlaced {
                kind: None,
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                amount: None,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_counter_removed() {
        assert_matches!(
            deckmaste_semantics::EventFilter::CounterRemoved {
                kind: None,
                on: minimal_predicate(),
                amount: None,
                cause: None
            }
            .lower(),
            deckmaste_core::EventFilter::CounterRemoved {
                kind: None,
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                amount: None,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_cast() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Cast {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::Cast {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_copied() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Copied {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::Copied {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_played() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Played {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::Played {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_activated_ab() {
        assert_matches!(
            deckmaste_semantics::EventFilter::ActivatedAb {
                who: minimal_predicate(),
                what: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::ActivatedAb {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_attack_declared() {
        assert_matches!(
            deckmaste_semantics::EventFilter::AttackDeclared {
                by: minimal_predicate(),
                against: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::AttackDeclared {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                against: deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                )
            }
        );
    }

    #[test]
    fn lowers_event_filter_block_declared() {
        assert_matches!(
            deckmaste_semantics::EventFilter::BlockDeclared {
                by: minimal_predicate(),
                of: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::BlockDeclared {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                of: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_attached() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Attached {
                what: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::Attached {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_state_became() {
        assert_matches!(
            deckmaste_semantics::EventFilter::StateBecame {
                of: minimal_predicate(),
                becomes: minimal_state_change(),
                cause: None
            }
            .lower(),
            deckmaste_core::EventFilter::StateBecame {
                of: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                becomes: deckmaste_core::StateChange::Tapped,
                cause: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_becomes_target() {
        assert_matches!(
            deckmaste_semantics::EventFilter::BecomesTarget {
                what: minimal_predicate(),
                by: minimal_predicate(),
                source: None
            }
            .lower(),
            deckmaste_core::EventFilter::BecomesTarget {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                source: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_step_begins() {
        assert_matches!(
            deckmaste_semantics::EventFilter::StepBegins {
                at: minimal_phase_step(),
                whose: minimal_whose_turn()
            }
            .lower(),
            deckmaste_core::EventFilter::StepBegins {
                at: deckmaste_core::PhaseStep::Beginning(deckmaste_core::BeginningStep::Untap),
                whose: deckmaste_core::WhoseTurn::Your
            }
        );
    }

    #[test]
    fn lowers_event_filter_control_changed() {
        assert_matches!(
            deckmaste_semantics::EventFilter::ControlChanged {
                of: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::ControlChanged {
                of: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_designation_changed() {
        assert_matches!(
            deckmaste_semantics::EventFilter::DesignationChanged {
                name: "X".into(),
                of: minimal_predicate(),
                to: Some("Y".into()),
            }
            .lower(),
            deckmaste_core::EventFilter::DesignationChanged {
                name,
                of: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: Some(value),
            } if name.as_ref() == "X" && value.as_ref() == "Y"
        );
    }

    #[test]
    fn lowers_event_filter_token_created() {
        assert_matches!(
            deckmaste_semantics::EventFilter::TokenCreated {
                what: minimal_predicate(),
                by: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::TokenCreated {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_shuffled() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Shuffled {
                by: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::Shuffled {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_revealed() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Revealed {
                what: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::Revealed {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_used() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Used {
                of: minimal_reference()
            }
            .lower(),
            deckmaste_core::EventFilter::Used {
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_event_filter_coin_flipped() {
        assert_matches!(
            deckmaste_semantics::EventFilter::CoinFlipped {
                by: minimal_predicate(),
                won: None
            }
            .lower(),
            deckmaste_core::EventFilter::CoinFlipped {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                won: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_dice_rolled() {
        assert_matches!(
            deckmaste_semantics::EventFilter::DiceRolled {
                by: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::DiceRolled {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_tap_for_mana() {
        assert_matches!(
            deckmaste_semantics::EventFilter::TapForMana {
                what: minimal_predicate(),
                by: minimal_predicate()
            }
            .lower(),
            deckmaste_core::EventFilter::TapForMana {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_event_filter_roll_planar_die() {
        assert_matches!(
            deckmaste_semantics::EventFilter::RollPlanarDie {
                by: minimal_predicate(),
                face: None
            }
            .lower(),
            deckmaste_core::EventFilter::RollPlanarDie {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                face: None
            }
        );
    }

    #[test]
    fn lowers_event_filter_all_of() {
        assert_matches!(
            deckmaste_semantics::EventFilter::AllOf([].into()).lower(),
            deckmaste_core::EventFilter::AllOf(_)
        );
    }

    #[test]
    fn lowers_event_filter_one_of() {
        assert_matches!(
            deckmaste_semantics::EventFilter::OneOf([].into()).lower(),
            deckmaste_core::EventFilter::OneOf(_)
        );
    }

    #[test]
    fn lowers_event_filter_not() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Not(std::sync::Arc::new(minimal_event_filter()))
                .lower(),
            deckmaste_core::EventFilter::Not(_)
        );
    }

    #[test]
    fn lowers_event_filter_one_or_more() {
        assert_matches!(
            deckmaste_semantics::EventFilter::OneOrMore(
                std::sync::Arc::new(minimal_event_filter())
            )
            .lower(),
            deckmaste_core::EventFilter::OneOrMore(_)
        );
    }

    #[test]
    fn lowers_event_filter_nth() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Nth {
                n: 0,
                of: std::sync::Arc::new(minimal_event_filter()),
                within: minimal_lookback()
            }
            .lower(),
            deckmaste_core::EventFilter::Nth {
                n: 0,
                of: _,
                within: deckmaste_core::Lookback::ThisTurn
            }
        );
    }

    #[test]
    fn lowers_event_filter_when() {
        assert_matches!(
            deckmaste_semantics::EventFilter::When(
                std::sync::Arc::new(minimal_event_filter()),
                std::sync::Arc::new(minimal_condition())
            )
            .lower(),
            deckmaste_core::EventFilter::When(_, _)
        );
    }

    #[test]
    fn lowers_event_filter_within() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Within(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            deckmaste_core::EventFilter::Within(_, deckmaste_core::Lookback::ThisTurn)
        );
    }

    #[test]
    fn lowers_event_filter_before() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Before(minimal_reference()).lower(),
            deckmaste_core::EventFilter::Before(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_event_filter_expanded() {
        assert_matches!(
            deckmaste_semantics::EventFilter::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_event_filter())
            })
            .lower(),
            deckmaste_core::EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            }
        );
    }
}
