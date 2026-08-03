//! `event` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::PhaseKind {
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

impl Lower for deckmaste_authoring::PhaseStep {
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

impl Lower for deckmaste_authoring::BeginningStep {
    type Target = deckmaste_core::BeginningStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Untap => deckmaste_core::BeginningStep::Untap,
            Self::Upkeep => deckmaste_core::BeginningStep::Upkeep,
            Self::Draw => deckmaste_core::BeginningStep::Draw,
        }
    }
}

impl Lower for deckmaste_authoring::CombatStep {
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

impl Lower for deckmaste_authoring::EndingStep {
    type Target = deckmaste_core::EndingStep;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::End => deckmaste_core::EndingStep::End,
            Self::Cleanup => deckmaste_core::EndingStep::Cleanup,
        }
    }
}

impl Lower for deckmaste_authoring::WhoseTurn {
    type Target = deckmaste_core::WhoseTurn;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Your => deckmaste_core::WhoseTurn::Your,
            Self::EachPlayers => deckmaste_core::WhoseTurn::EachPlayers,
            Self::AnOpponents => deckmaste_core::WhoseTurn::AnOpponents,
        }
    }
}

impl Lower for deckmaste_authoring::StateChange {
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

impl Lower for deckmaste_authoring::Agency {
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

impl Lower for deckmaste_authoring::VerbName {
    type Target = deckmaste_core::VerbName;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::VerbName(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::CausePattern {
    type Target = deckmaste_core::CausePattern;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CausePattern {
            verb: self.verb.lower(),
            agency: self.agency.lower(),
            agent: self.agent.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Cause {
    type Target = deckmaste_core::Cause;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Cause(f0) => deckmaste_core::Cause::Cause(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::EventFilter {
    type Target = deckmaste_core::EventFilter;
    #[expect(
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
            Self::DesignationChanged { name, of } => {
                deckmaste_core::EventFilter::DesignationChanged {
                    name: name.lower(),
                    of: of.lower(),
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
            Self::BecameDay => deckmaste_core::EventFilter::BecameDay,
            Self::BecameNight => deckmaste_core::EventFilter::BecameNight,
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
            Self::Expanded(f0) => deckmaste_core::EventFilter::Expanded(f0.lower()),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_phase_kind_beginning() {
        assert_lowers(deckmaste_authoring::PhaseKind::Beginning);
    }

    #[test]
    fn lowers_phase_kind_precombat_main() {
        assert_lowers(deckmaste_authoring::PhaseKind::PrecombatMain);
    }

    #[test]
    fn lowers_phase_kind_combat() {
        assert_lowers(deckmaste_authoring::PhaseKind::Combat);
    }

    #[test]
    fn lowers_phase_kind_postcombat_main() {
        assert_lowers(deckmaste_authoring::PhaseKind::PostcombatMain);
    }

    #[test]
    fn lowers_phase_kind_ending() {
        assert_lowers(deckmaste_authoring::PhaseKind::Ending);
    }

    #[test]
    fn lowers_phase_step_beginning() {
        assert_lowers(deckmaste_authoring::PhaseStep::Beginning(
            minimal_beginning_step(),
        ));
    }

    #[test]
    fn lowers_phase_step_precombat_main() {
        assert_lowers(deckmaste_authoring::PhaseStep::PrecombatMain);
    }

    #[test]
    fn lowers_phase_step_combat() {
        assert_lowers(deckmaste_authoring::PhaseStep::Combat(minimal_combat_step()));
    }

    #[test]
    fn lowers_phase_step_postcombat_main() {
        assert_lowers(deckmaste_authoring::PhaseStep::PostcombatMain);
    }

    #[test]
    fn lowers_phase_step_ending() {
        assert_lowers(deckmaste_authoring::PhaseStep::Ending(minimal_ending_step()));
    }

    #[test]
    fn lowers_beginning_step_untap() {
        assert_lowers(deckmaste_authoring::BeginningStep::Untap);
    }

    #[test]
    fn lowers_beginning_step_upkeep() {
        assert_lowers(deckmaste_authoring::BeginningStep::Upkeep);
    }

    #[test]
    fn lowers_beginning_step_draw() {
        assert_lowers(deckmaste_authoring::BeginningStep::Draw);
    }

    #[test]
    fn lowers_combat_step_beginning_of_combat() {
        assert_lowers(deckmaste_authoring::CombatStep::BeginningOfCombat);
    }

    #[test]
    fn lowers_combat_step_declare_attackers() {
        assert_lowers(deckmaste_authoring::CombatStep::DeclareAttackers);
    }

    #[test]
    fn lowers_combat_step_declare_blockers() {
        assert_lowers(deckmaste_authoring::CombatStep::DeclareBlockers);
    }

    #[test]
    fn lowers_combat_step_first_combat_damage() {
        assert_lowers(deckmaste_authoring::CombatStep::FirstCombatDamage);
    }

    #[test]
    fn lowers_combat_step_combat_damage() {
        assert_lowers(deckmaste_authoring::CombatStep::CombatDamage);
    }

    #[test]
    fn lowers_combat_step_end_of_combat() {
        assert_lowers(deckmaste_authoring::CombatStep::EndOfCombat);
    }

    #[test]
    fn lowers_ending_step_end() {
        assert_lowers(deckmaste_authoring::EndingStep::End);
    }

    #[test]
    fn lowers_ending_step_cleanup() {
        assert_lowers(deckmaste_authoring::EndingStep::Cleanup);
    }

    #[test]
    fn lowers_whose_turn_your() {
        assert_lowers(deckmaste_authoring::WhoseTurn::Your);
    }

    #[test]
    fn lowers_whose_turn_each_players() {
        assert_lowers(deckmaste_authoring::WhoseTurn::EachPlayers);
    }

    #[test]
    fn lowers_whose_turn_an_opponents() {
        assert_lowers(deckmaste_authoring::WhoseTurn::AnOpponents);
    }

    #[test]
    fn lowers_state_change_tapped() {
        assert_lowers(deckmaste_authoring::StateChange::Tapped);
    }

    #[test]
    fn lowers_state_change_untapped() {
        assert_lowers(deckmaste_authoring::StateChange::Untapped);
    }

    #[test]
    fn lowers_state_change_phased() {
        assert_lowers(deckmaste_authoring::StateChange::Phased(minimal_phasing()));
    }

    #[test]
    fn lowers_state_change_turned_face() {
        assert_lowers(deckmaste_authoring::StateChange::TurnedFace(minimal_face()));
    }

    #[test]
    fn lowers_state_change_transformed() {
        assert_lowers(deckmaste_authoring::StateChange::Transformed);
    }

    #[test]
    fn lowers_agency_cost_payment() {
        assert_lowers(deckmaste_authoring::Agency::CostPayment);
    }

    #[test]
    fn lowers_agency_attack_declaration() {
        assert_lowers(deckmaste_authoring::Agency::AttackDeclaration);
    }

    #[test]
    fn lowers_agency_effect_instruction() {
        assert_lowers(deckmaste_authoring::Agency::EffectInstruction);
    }

    #[test]
    fn lowers_agency_turn_based_action() {
        assert_lowers(deckmaste_authoring::Agency::TurnBasedAction);
    }

    #[test]
    fn lowers_agency_state_based_action() {
        assert_lowers(deckmaste_authoring::Agency::StateBasedAction);
    }

    #[test]
    fn lowers_agency_mana_ability_resolution() {
        assert_lowers(deckmaste_authoring::Agency::ManaAbilityResolution);
    }

    #[test]
    fn lowers_agency_special_action() {
        assert_lowers(deckmaste_authoring::Agency::SpecialAction);
    }

    #[test]
    fn lowers_verb_name() {
        assert_lowers_debug(deckmaste_authoring::VerbName("X".into()));
    }

    #[test]
    fn lowers_cause_pattern() {
        assert_lowers(deckmaste_authoring::CausePattern {
            verb: None,
            agency: None,
            agent: None,
        });
    }

    #[test]
    fn lowers_cause_cause() {
        assert_lowers(deckmaste_authoring::Cause::Cause(minimal_cause_pattern()));
    }

    #[test]
    fn lowers_event_filter_zone_change() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::ZoneChange {
            what: minimal_predicate(),
            from: None,
            to: None,
            cause: None,
        });
    }

    #[test]
    fn lowers_event_filter_damage() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Damage {
            source: minimal_predicate(),
            to: minimal_predicate(),
            combat: None,
            amount: None,
        });
    }

    #[test]
    fn lowers_event_filter_life_gained() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::LifeGained {
            who: minimal_predicate(),
            amount: None,
        });
    }

    #[test]
    fn lowers_event_filter_life_lost() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::LifeLost {
            who: minimal_predicate(),
            amount: None,
        });
    }

    #[test]
    fn lowers_event_filter_drawn() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Drawn {
            who: minimal_predicate(),
            amount: None,
        });
    }

    #[test]
    fn lowers_event_filter_act() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Act {
            verb: minimal_verb_name(),
            who: minimal_predicate(),
            on: minimal_predicate(),
            cause: None,
        });
    }

    #[test]
    fn lowers_event_filter_counter_placed() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::CounterPlaced {
            kind: None,
            on: minimal_predicate(),
            amount: None,
            cause: None,
        });
    }

    #[test]
    fn lowers_event_filter_counter_removed() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::CounterRemoved {
            kind: None,
            on: minimal_predicate(),
            amount: None,
            cause: None,
        });
    }

    #[test]
    fn lowers_event_filter_cast() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Cast {
            who: minimal_predicate(),
            what: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_copied() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Copied {
            who: minimal_predicate(),
            what: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_played() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Played {
            who: minimal_predicate(),
            what: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_activated_ab() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::ActivatedAb {
            who: minimal_predicate(),
            what: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_attack_declared() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::AttackDeclared {
            by: minimal_predicate(),
            against: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_block_declared() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::BlockDeclared {
            by: minimal_predicate(),
            of: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_attached() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Attached {
            what: minimal_predicate(),
            to: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_state_became() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::StateBecame {
            of: minimal_predicate(),
            becomes: minimal_state_change(),
            cause: None,
        });
    }

    #[test]
    fn lowers_event_filter_becomes_target() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::BecomesTarget {
            what: minimal_predicate(),
            by: minimal_predicate(),
            source: None,
        });
    }

    #[test]
    fn lowers_event_filter_step_begins() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::StepBegins {
            at: minimal_phase_step(),
            whose: minimal_whose_turn(),
        });
    }

    #[test]
    fn lowers_event_filter_control_changed() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::ControlChanged {
            of: minimal_predicate(),
            to: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_designation_changed() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::DesignationChanged {
            name: "X".into(),
            of: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_token_created() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::TokenCreated {
            what: minimal_predicate(),
            by: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_shuffled() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Shuffled {
            by: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_revealed() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Revealed {
            what: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_used() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Used {
            of: minimal_reference(),
        });
    }

    #[test]
    fn lowers_event_filter_coin_flipped() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::CoinFlipped {
            by: minimal_predicate(),
            won: None,
        });
    }

    #[test]
    fn lowers_event_filter_dice_rolled() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::DiceRolled {
            by: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_tap_for_mana() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::TapForMana {
            what: minimal_predicate(),
            by: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_event_filter_roll_planar_die() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::RollPlanarDie {
            by: minimal_predicate(),
            face: None,
        });
    }

    #[test]
    fn lowers_event_filter_became_day() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::BecameDay);
    }

    #[test]
    fn lowers_event_filter_became_night() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::BecameNight);
    }

    #[test]
    fn lowers_event_filter_all_of() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::AllOf([].into()));
    }

    #[test]
    fn lowers_event_filter_one_of() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::OneOf([].into()));
    }

    #[test]
    fn lowers_event_filter_not() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Not(std::sync::Arc::new(
            minimal_event_filter(),
        )));
    }

    #[test]
    fn lowers_event_filter_one_or_more() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::OneOrMore(
            std::sync::Arc::new(minimal_event_filter()),
        ));
    }

    #[test]
    fn lowers_event_filter_nth() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Nth {
            n: 0,
            of: std::sync::Arc::new(minimal_event_filter()),
            within: minimal_lookback(),
        });
    }

    #[test]
    fn lowers_event_filter_when() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::When(
            std::sync::Arc::new(minimal_event_filter()),
            std::sync::Arc::new(minimal_condition()),
        ));
    }

    #[test]
    fn lowers_event_filter_within() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Within(
            std::sync::Arc::new(minimal_event_filter()),
            minimal_lookback(),
        ));
    }

    #[test]
    fn lowers_event_filter_before() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Before(minimal_reference()));
    }

    #[test]
    fn lowers_event_filter_expanded() {
        assert_lowers_debug(deckmaste_authoring::EventFilter::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_event_filter()),
            },
        ));
    }
}
