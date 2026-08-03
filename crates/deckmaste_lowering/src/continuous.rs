//! `continuous` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Duration {
    type Target = deckmaste_core::Duration;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FixedUntil(f0) => deckmaste_core::Duration::FixedUntil(f0.lower()),
            Self::UntilEvent(f0) => deckmaste_core::Duration::UntilEvent(f0.lower()),
            Self::ForAsLongAs(f0) => deckmaste_core::Duration::ForAsLongAs(f0.lower()),
            Self::ForThisEvent => deckmaste_core::Duration::ForThisEvent,
            Self::EndOfGame => deckmaste_core::Duration::EndOfGame,
        }
    }
}

impl Lower for deckmaste_authoring::NumericOp {
    type Target = deckmaste_core::NumericOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => deckmaste_core::NumericOp::Set(f0.lower()),
            Self::Up(f0) => deckmaste_core::NumericOp::Up(f0.lower()),
            Self::Down(f0) => deckmaste_core::NumericOp::Down(f0.lower()),
        }
    }
}

impl<T: Lower + Clone> Lower for deckmaste_authoring::CollectionOp<T> {
    type Target = deckmaste_core::CollectionOp<<T as Lower>::Target>;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => deckmaste_core::CollectionOp::Set(f0.lower()),
            Self::Add(f0) => deckmaste_core::CollectionOp::Add(f0.lower()),
            Self::Remove(f0) => deckmaste_core::CollectionOp::Remove(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Modification {
    type Target = deckmaste_core::Modification;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Power(f0) => deckmaste_core::Modification::Power(f0.lower()),
            Self::Toughness(f0) => deckmaste_core::Modification::Toughness(f0.lower()),
            Self::SwitchPowerToughness => deckmaste_core::Modification::SwitchPowerToughness,
            Self::Colors(f0) => deckmaste_core::Modification::Colors(f0.lower()),
            Self::CardTypes(f0) => deckmaste_core::Modification::CardTypes(f0.lower()),
            Self::Subtypes(f0) => deckmaste_core::Modification::Subtypes(f0.lower()),
            Self::Supertypes(f0) => deckmaste_core::Modification::Supertypes(f0.lower()),
            Self::GainAbility(f0) => deckmaste_core::Modification::GainAbility(f0.lower()),
            Self::LoseAbility(f0) => deckmaste_core::Modification::LoseAbility(f0.lower()),
            Self::LoseAllAbilities => deckmaste_core::Modification::LoseAllAbilities,
            Self::CantHaveAbility(f0) => deckmaste_core::Modification::CantHaveAbility(f0.lower()),
            Self::SetController(f0) => deckmaste_core::Modification::SetController(f0.lower()),
            Self::SetText(f0) => deckmaste_core::Modification::SetText(f0.lower()),
            Self::AllCreatureTypes => deckmaste_core::Modification::AllCreatureTypes,
            Self::BaseLoyalty(f0) => deckmaste_core::Modification::BaseLoyalty(f0.lower()),
            Self::BaseDefense(f0) => deckmaste_core::Modification::BaseDefense(f0.lower()),
            Self::BecomeBasicLandType(f0) => {
                deckmaste_core::Modification::BecomeBasicLandType(f0.lower())
            }
            Self::Several(f0) => deckmaste_core::Modification::Several(f0.lower()),
            Self::Expanded(f0) => deckmaste_core::Modification::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::CostChange {
    type Target = deckmaste_core::CostChange;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Increase(f0) => deckmaste_core::CostChange::Increase(f0.lower()),
            Self::Reduce(f0) => deckmaste_core::CostChange::Reduce(f0.lower()),
            Self::Additional { components } => deckmaste_core::CostChange::Additional {
                components: components.lower(),
            },
            Self::Scaled { change, times } => deckmaste_core::CostChange::Scaled {
                change: change.lower(),
                times: times.lower(),
            },
        }
    }
}

impl Lower for deckmaste_authoring::StaticEffect {
    type Target = deckmaste_core::StaticEffect;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Modify(f0, f1) => deckmaste_core::StaticEffect::Modify(f0.lower(), f1.lower()),
            Self::BecomesCopy(f0, f1) => {
                deckmaste_core::StaticEffect::BecomesCopy(f0.lower(), f1.lower())
            }
            Self::Each(f0, f1) => deckmaste_core::StaticEffect::Each(f0.lower(), f1.lower()),
            Self::Conditionally(f0, f1) => {
                deckmaste_core::StaticEffect::Conditionally(f0.lower(), f1.lower())
            }
            Self::Deontic(f0) => deckmaste_core::StaticEffect::Deontic(f0.lower()),
            Self::CostModifier { of, change } => deckmaste_core::StaticEffect::CostModifier {
                of: of.lower(),
                change: change.lower(),
            },
            Self::CostOption(f0) => deckmaste_core::StaticEffect::CostOption(f0.lower()),
            Self::TriggerMultiplier {
                cause,
                extra,
                affected,
            } => deckmaste_core::StaticEffect::TriggerMultiplier {
                cause: cause.lower(),
                extra: extra.lower(),
                affected: affected.lower(),
            },
            Self::ModifyPlayer(f0, f1) => {
                deckmaste_core::StaticEffect::ModifyPlayer(f0.lower(), f1.lower())
            }
            Self::Replacement(f0) => deckmaste_core::StaticEffect::Replacement(f0.lower()),
            Self::Prevention(f0) => deckmaste_core::StaticEffect::Prevention(f0.lower()),
            Self::CantPrevent { from, to } => deckmaste_core::StaticEffect::CantPrevent {
                from: from.lower(),
                to: to.lower(),
            },
            Self::SpendAsThough { mana_from, as_ } => deckmaste_core::StaticEffect::SpendAsThough {
                mana_from: mana_from.lower(),
                as_: as_.lower(),
            },
            Self::AsThough(f0) => deckmaste_core::StaticEffect::AsThough(f0.lower()),
            Self::Sba { when, then } => deckmaste_core::StaticEffect::Sba {
                when: when.lower(),
                then: then.lower(),
            },
            Self::OutcomeGate { who, gate } => deckmaste_core::StaticEffect::OutcomeGate {
                who: who.lower(),
                gate: gate.lower(),
            },
            Self::CantHappen(f0) => deckmaste_core::StaticEffect::CantHappen(f0.lower()),
            Self::ReplaceRoll {
                query,
                extra,
                ignore,
            } => deckmaste_core::StaticEffect::ReplaceRoll {
                query: query.lower(),
                extra: extra.lower(),
                ignore: ignore.lower(),
            },
            Self::PayPips(f0, f1) => deckmaste_core::StaticEffect::PayPips(f0.lower(), f1.lower()),
            Self::Expanded(f0) => deckmaste_core::StaticEffect::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::OutcomeGateKind {
    type Target = deckmaste_core::OutcomeGateKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::CantLose => deckmaste_core::OutcomeGateKind::CantLose,
            Self::CantWin => deckmaste_core::OutcomeGateKind::CantWin,
        }
    }
}

impl Lower for deckmaste_authoring::IgnoreRule {
    type Target = deckmaste_core::IgnoreRule;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::IgnoreLowest => deckmaste_core::IgnoreRule::IgnoreLowest,
            Self::IgnoreChosen(f0) => deckmaste_core::IgnoreRule::IgnoreChosen(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::PipClass {
    type Target = deckmaste_core::PipClass;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Generic => deckmaste_core::PipClass::Generic,
            Self::Colored(f0) => deckmaste_core::PipClass::Colored(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::PayAct {
    type Target = deckmaste_core::PayAct;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::TapToPay(f0) => deckmaste_core::PayAct::TapToPay(f0.lower()),
            Self::ExileToPay(f0) => deckmaste_core::PayAct::ExileToPay(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::PlayerAttr {
    type Target = deckmaste_core::PlayerAttr;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Life => deckmaste_core::PlayerAttr::Life,
            Self::HandSize => deckmaste_core::PlayerAttr::HandSize,
            Self::HandSizeLimit => deckmaste_core::PlayerAttr::HandSizeLimit,
            Self::LandPlaysPerTurn => deckmaste_core::PlayerAttr::LandPlaysPerTurn,
        }
    }
}

impl Lower for deckmaste_authoring::PlayerMod {
    type Target = deckmaste_core::PlayerMod;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SetTo(f0, f1) => deckmaste_core::PlayerMod::SetTo(f0.lower(), f1.lower()),
            Self::Raise(f0, f1) => deckmaste_core::PlayerMod::Raise(f0.lower(), f1.lower()),
            Self::Lower(f0, f1) => deckmaste_core::PlayerMod::Lower(f0.lower(), f1.lower()),
            Self::NoMax(f0) => deckmaste_core::PlayerMod::NoMax(f0.lower()),
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
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_duration_fixed_until() {
        assert_lowers(deckmaste_authoring::Duration::FixedUntil(
            minimal_turn_marker(),
        ));
        assert_matches!(
            deckmaste_authoring::Duration::FixedUntil(minimal_turn_marker()).lower(),
            deckmaste_core::Duration::FixedUntil(..)
        );
    }

    #[test]
    fn lowers_duration_until_event() {
        assert_lowers(deckmaste_authoring::Duration::UntilEvent(
            minimal_event_filter(),
        ));
        assert_matches!(
            deckmaste_authoring::Duration::UntilEvent(minimal_event_filter()).lower(),
            deckmaste_core::Duration::UntilEvent(..)
        );
    }

    #[test]
    fn lowers_duration_for_as_long_as() {
        assert_lowers(deckmaste_authoring::Duration::ForAsLongAs(
            minimal_condition(),
        ));
        assert_matches!(
            deckmaste_authoring::Duration::ForAsLongAs(minimal_condition()).lower(),
            deckmaste_core::Duration::ForAsLongAs(..)
        );
    }

    #[test]
    fn lowers_duration_for_this_event() {
        assert_lowers(deckmaste_authoring::Duration::ForThisEvent);
        assert_matches!(
            deckmaste_authoring::Duration::ForThisEvent.lower(),
            deckmaste_core::Duration::ForThisEvent
        );
    }

    #[test]
    fn lowers_duration_end_of_game() {
        assert_lowers(deckmaste_authoring::Duration::EndOfGame);
        assert_matches!(
            deckmaste_authoring::Duration::EndOfGame.lower(),
            deckmaste_core::Duration::EndOfGame
        );
    }

    #[test]
    fn lowers_numeric_op_set() {
        assert_lowers(deckmaste_authoring::NumericOp::Set(minimal_stat_value()));
        assert_matches!(
            deckmaste_authoring::NumericOp::Set(minimal_stat_value()).lower(),
            deckmaste_core::NumericOp::Set(..)
        );
    }

    #[test]
    fn lowers_numeric_op_up() {
        assert_lowers(deckmaste_authoring::NumericOp::Up(minimal_count()));
        assert_matches!(
            deckmaste_authoring::NumericOp::Up(minimal_count()).lower(),
            deckmaste_core::NumericOp::Up(..)
        );
    }

    #[test]
    fn lowers_numeric_op_down() {
        assert_lowers(deckmaste_authoring::NumericOp::Down(minimal_count()));
        assert_matches!(
            deckmaste_authoring::NumericOp::Down(minimal_count()).lower(),
            deckmaste_core::NumericOp::Down(..)
        );
    }

    #[test]
    fn lowers_collection_op_set() {
        assert_lowers(deckmaste_authoring::CollectionOp::<
            deckmaste_authoring::Color,
        >::Set([].into()));
        assert_matches!(
            deckmaste_authoring::CollectionOp::<deckmaste_authoring::Color>::Set([].into()).lower(),
            deckmaste_core::CollectionOp::Set(..)
        );
    }

    #[test]
    fn lowers_collection_op_add() {
        assert_lowers(deckmaste_authoring::CollectionOp::<
            deckmaste_authoring::Color,
        >::Add(minimal_color()));
        assert_matches!(
            deckmaste_authoring::CollectionOp::<deckmaste_authoring::Color>::Add(minimal_color())
                .lower(),
            deckmaste_core::CollectionOp::Add(..)
        );
    }

    #[test]
    fn lowers_collection_op_remove() {
        assert_lowers(deckmaste_authoring::CollectionOp::<
            deckmaste_authoring::Color,
        >::Remove(minimal_color()));
        assert_matches!(deckmaste_authoring::CollectionOp::<deckmaste_authoring::Color>::Remove(minimal_color()).lower(), deckmaste_core::CollectionOp::Remove(..));
    }

    #[test]
    fn lowers_modification_power() {
        assert_lowers_debug(deckmaste_authoring::Modification::Power(
            minimal_numeric_op(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::Power(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::Power(..)
        );
    }

    #[test]
    fn lowers_modification_toughness() {
        assert_lowers_debug(deckmaste_authoring::Modification::Toughness(
            minimal_numeric_op(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::Toughness(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::Toughness(..)
        );
    }

    #[test]
    fn lowers_modification_switch_power_toughness() {
        assert_lowers_debug(deckmaste_authoring::Modification::SwitchPowerToughness);
        assert_matches!(
            deckmaste_authoring::Modification::SwitchPowerToughness.lower(),
            deckmaste_core::Modification::SwitchPowerToughness
        );
    }

    #[test]
    fn lowers_modification_colors() {
        assert_lowers_debug(deckmaste_authoring::Modification::Colors(
            minimal_collection_op::<deckmaste_authoring::Color>(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::Colors(minimal_collection_op::<
                deckmaste_authoring::Color,
            >())
            .lower(),
            deckmaste_core::Modification::Colors(..)
        );
    }

    #[test]
    fn lowers_modification_card_types() {
        assert_lowers_debug(deckmaste_authoring::Modification::CardTypes(
            minimal_collection_op::<deckmaste_authoring::Ident>(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::CardTypes(minimal_collection_op::<
                deckmaste_authoring::Ident,
            >())
            .lower(),
            deckmaste_core::Modification::CardTypes(..)
        );
    }

    #[test]
    fn lowers_modification_subtypes() {
        assert_lowers_debug(deckmaste_authoring::Modification::Subtypes(
            minimal_collection_op::<deckmaste_authoring::SubtypeRef>(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::Subtypes(minimal_collection_op::<
                deckmaste_authoring::SubtypeRef,
            >())
            .lower(),
            deckmaste_core::Modification::Subtypes(..)
        );
    }

    #[test]
    fn lowers_modification_supertypes() {
        assert_lowers_debug(deckmaste_authoring::Modification::Supertypes(
            minimal_collection_op::<deckmaste_authoring::Supertype>(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::Supertypes(minimal_collection_op::<
                deckmaste_authoring::Supertype,
            >())
            .lower(),
            deckmaste_core::Modification::Supertypes(..)
        );
    }

    #[test]
    fn lowers_modification_gain_ability() {
        assert_lowers_debug(deckmaste_authoring::Modification::GainAbility(
            std::sync::Arc::new(minimal_ability()),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::GainAbility(std::sync::Arc::new(minimal_ability()))
                .lower(),
            deckmaste_core::Modification::GainAbility(..)
        );
    }

    #[test]
    fn lowers_modification_lose_ability() {
        assert_lowers_debug(deckmaste_authoring::Modification::LoseAbility("X".into()));
        assert_matches!(
            deckmaste_authoring::Modification::LoseAbility("X".into()).lower(),
            deckmaste_core::Modification::LoseAbility(..)
        );
    }

    #[test]
    fn lowers_modification_lose_all_abilities() {
        assert_lowers_debug(deckmaste_authoring::Modification::LoseAllAbilities);
        assert_matches!(
            deckmaste_authoring::Modification::LoseAllAbilities.lower(),
            deckmaste_core::Modification::LoseAllAbilities
        );
    }

    #[test]
    fn lowers_modification_cant_have_ability() {
        assert_lowers_debug(deckmaste_authoring::Modification::CantHaveAbility(
            "X".into(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::CantHaveAbility("X".into()).lower(),
            deckmaste_core::Modification::CantHaveAbility(..)
        );
    }

    #[test]
    fn lowers_modification_set_controller() {
        assert_lowers_debug(deckmaste_authoring::Modification::SetController(
            minimal_reference(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::SetController(minimal_reference()).lower(),
            deckmaste_core::Modification::SetController(..)
        );
    }

    #[test]
    fn lowers_modification_set_text() {
        assert_lowers_debug(deckmaste_authoring::Modification::SetText(String::new()));
        assert_matches!(
            deckmaste_authoring::Modification::SetText(String::new()).lower(),
            deckmaste_core::Modification::SetText(..)
        );
    }

    #[test]
    fn lowers_modification_all_creature_types() {
        assert_lowers_debug(deckmaste_authoring::Modification::AllCreatureTypes);
        assert_matches!(
            deckmaste_authoring::Modification::AllCreatureTypes.lower(),
            deckmaste_core::Modification::AllCreatureTypes
        );
    }

    #[test]
    fn lowers_modification_base_loyalty() {
        assert_lowers_debug(deckmaste_authoring::Modification::BaseLoyalty(
            minimal_numeric_op(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::BaseLoyalty(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::BaseLoyalty(..)
        );
    }

    #[test]
    fn lowers_modification_base_defense() {
        assert_lowers_debug(deckmaste_authoring::Modification::BaseDefense(
            minimal_numeric_op(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::BaseDefense(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::BaseDefense(..)
        );
    }

    #[test]
    fn lowers_modification_become_basic_land_type() {
        assert_lowers_debug(deckmaste_authoring::Modification::BecomeBasicLandType(
            [].into(),
        ));
        assert_matches!(
            deckmaste_authoring::Modification::BecomeBasicLandType([].into()).lower(),
            deckmaste_core::Modification::BecomeBasicLandType(..)
        );
    }

    #[test]
    fn lowers_modification_several() {
        assert_lowers_debug(deckmaste_authoring::Modification::Several([].into()));
        assert_matches!(
            deckmaste_authoring::Modification::Several([].into()).lower(),
            deckmaste_core::Modification::Several(..)
        );
    }

    #[test]
    fn lowers_modification_expanded() {
        assert_lowers_debug(deckmaste_authoring::Modification::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_modification()),
            },
        ));
        assert_matches!(
            deckmaste_authoring::Modification::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_modification())
            })
            .lower(),
            deckmaste_core::Modification::Expanded(..)
        );
    }

    #[test]
    fn lowers_cost_change_increase() {
        assert_lowers(deckmaste_authoring::CostChange::Increase([].into()));
        assert_matches!(
            deckmaste_authoring::CostChange::Increase([].into()).lower(),
            deckmaste_core::CostChange::Increase(..)
        );
    }

    #[test]
    fn lowers_cost_change_reduce() {
        assert_lowers(deckmaste_authoring::CostChange::Reduce([].into()));
        assert_matches!(
            deckmaste_authoring::CostChange::Reduce([].into()).lower(),
            deckmaste_core::CostChange::Reduce(..)
        );
    }

    #[test]
    fn lowers_cost_change_additional() {
        assert_lowers(deckmaste_authoring::CostChange::Additional {
            components: [].into(),
        });
        assert_matches!(
            deckmaste_authoring::CostChange::Additional {
                components: [].into()
            }
            .lower(),
            deckmaste_core::CostChange::Additional { .. }
        );
    }

    #[test]
    fn lowers_cost_change_scaled() {
        assert_lowers(deckmaste_authoring::CostChange::Scaled {
            change: std::sync::Arc::new(minimal_cost_change()),
            times: minimal_count(),
        });
        assert_matches!(
            deckmaste_authoring::CostChange::Scaled {
                change: std::sync::Arc::new(minimal_cost_change()),
                times: minimal_count()
            }
            .lower(),
            deckmaste_core::CostChange::Scaled { .. }
        );
    }

    #[test]
    fn lowers_static_effect_modify() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Modify(
            minimal_reference(),
            minimal_modification(),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::Modify(minimal_reference(), minimal_modification())
                .lower(),
            deckmaste_core::StaticEffect::Modify(..)
        );
    }

    #[test]
    fn lowers_static_effect_becomes_copy() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::BecomesCopy(
            minimal_reference(),
            minimal_copy_spec(),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::BecomesCopy(
                minimal_reference(),
                minimal_copy_spec()
            )
            .lower(),
            deckmaste_core::StaticEffect::BecomesCopy(..)
        );
    }

    #[test]
    fn lowers_static_effect_each() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Each(
            minimal_selection(),
            std::sync::Arc::new(minimal_static_effect()),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::Each(
                minimal_selection(),
                std::sync::Arc::new(minimal_static_effect())
            )
            .lower(),
            deckmaste_core::StaticEffect::Each(..)
        );
    }

    #[test]
    fn lowers_static_effect_conditionally() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Conditionally(
            minimal_condition(),
            std::sync::Arc::new(minimal_static_effect()),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::Conditionally(
                minimal_condition(),
                std::sync::Arc::new(minimal_static_effect())
            )
            .lower(),
            deckmaste_core::StaticEffect::Conditionally(..)
        );
    }

    #[test]
    fn lowers_static_effect_deontic() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Deontic(minimal_deontic()));
        assert_matches!(
            deckmaste_authoring::StaticEffect::Deontic(minimal_deontic()).lower(),
            deckmaste_core::StaticEffect::Deontic(..)
        );
    }

    #[test]
    fn lowers_static_effect_cost_modifier() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::CostModifier {
            of: minimal_predicate(),
            change: minimal_cost_change(),
        });
        assert_matches!(
            deckmaste_authoring::StaticEffect::CostModifier {
                of: minimal_predicate(),
                change: minimal_cost_change()
            }
            .lower(),
            deckmaste_core::StaticEffect::CostModifier { .. }
        );
    }

    #[test]
    fn lowers_static_effect_cost_option() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::CostOption(
            minimal_optional_cost(),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::CostOption(minimal_optional_cost()).lower(),
            deckmaste_core::StaticEffect::CostOption(..)
        );
    }

    #[test]
    fn lowers_static_effect_trigger_multiplier() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::TriggerMultiplier {
            cause: minimal_event_filter(),
            extra: minimal_count(),
            affected: minimal_predicate(),
        });
        assert_matches!(
            deckmaste_authoring::StaticEffect::TriggerMultiplier {
                cause: minimal_event_filter(),
                extra: minimal_count(),
                affected: minimal_predicate()
            }
            .lower(),
            deckmaste_core::StaticEffect::TriggerMultiplier { .. }
        );
    }

    #[test]
    fn lowers_static_effect_modify_player() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::ModifyPlayer(
            minimal_reference(),
            minimal_player_mod(),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::ModifyPlayer(
                minimal_reference(),
                minimal_player_mod()
            )
            .lower(),
            deckmaste_core::StaticEffect::ModifyPlayer(..)
        );
    }

    #[test]
    fn lowers_static_effect_replacement() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Replacement(
            std::sync::Arc::new(minimal_replacement()),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::Replacement(std::sync::Arc::new(
                minimal_replacement()
            ))
            .lower(),
            deckmaste_core::StaticEffect::Replacement(..)
        );
    }

    #[test]
    fn lowers_static_effect_prevention() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Prevention(
            std::sync::Arc::new(minimal_prevention()),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::Prevention(
                std::sync::Arc::new(minimal_prevention())
            )
            .lower(),
            deckmaste_core::StaticEffect::Prevention(..)
        );
    }

    #[test]
    fn lowers_static_effect_cant_prevent() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::CantPrevent {
            from: minimal_predicate(),
            to: minimal_predicate(),
        });
        assert_matches!(
            deckmaste_authoring::StaticEffect::CantPrevent {
                from: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::StaticEffect::CantPrevent { .. }
        );
    }

    #[test]
    fn lowers_static_effect_spend_as_though() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::SpendAsThough {
            mana_from: minimal_predicate(),
            as_: minimal_symbol_pred(),
        });
        assert_matches!(
            deckmaste_authoring::StaticEffect::SpendAsThough {
                mana_from: minimal_predicate(),
                as_: minimal_symbol_pred()
            }
            .lower(),
            deckmaste_core::StaticEffect::SpendAsThough { .. }
        );
    }

    #[test]
    fn lowers_static_effect_as_though() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::AsThough(
            minimal_as_though(),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::AsThough(minimal_as_though()).lower(),
            deckmaste_core::StaticEffect::AsThough(..)
        );
    }

    #[test]
    fn lowers_static_effect_sba() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Sba {
            when: std::sync::Arc::new(minimal_condition()),
            then: std::sync::Arc::new(minimal_one_shot_effect()),
        });
        assert_matches!(
            deckmaste_authoring::StaticEffect::Sba {
                when: std::sync::Arc::new(minimal_condition()),
                then: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::StaticEffect::Sba { .. }
        );
    }

    #[test]
    fn lowers_static_effect_outcome_gate() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::OutcomeGate {
            who: minimal_predicate(),
            gate: minimal_outcome_gate_kind(),
        });
        assert_matches!(
            deckmaste_authoring::StaticEffect::OutcomeGate {
                who: minimal_predicate(),
                gate: minimal_outcome_gate_kind()
            }
            .lower(),
            deckmaste_core::StaticEffect::OutcomeGate { .. }
        );
    }

    #[test]
    fn lowers_static_effect_cant_happen() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::CantHappen(
            minimal_event_filter(),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::CantHappen(minimal_event_filter()).lower(),
            deckmaste_core::StaticEffect::CantHappen(..)
        );
    }

    #[test]
    fn lowers_static_effect_replace_roll() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::ReplaceRoll {
            query: minimal_event_filter(),
            extra: minimal_count(),
            ignore: minimal_ignore_rule(),
        });
        assert_matches!(
            deckmaste_authoring::StaticEffect::ReplaceRoll {
                query: minimal_event_filter(),
                extra: minimal_count(),
                ignore: minimal_ignore_rule()
            }
            .lower(),
            deckmaste_core::StaticEffect::ReplaceRoll { .. }
        );
    }

    #[test]
    fn lowers_static_effect_pay_pips() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::PayPips(
            minimal_pip_class(),
            minimal_pay_act(),
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::PayPips(minimal_pip_class(), minimal_pay_act())
                .lower(),
            deckmaste_core::StaticEffect::PayPips(..)
        );
    }

    #[test]
    fn lowers_static_effect_expanded() {
        assert_lowers_debug(deckmaste_authoring::StaticEffect::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_static_effect()),
            },
        ));
        assert_matches!(
            deckmaste_authoring::StaticEffect::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_static_effect())
            })
            .lower(),
            deckmaste_core::StaticEffect::Expanded(..)
        );
    }

    #[test]
    fn lowers_outcome_gate_kind_cant_lose() {
        assert_lowers(deckmaste_authoring::OutcomeGateKind::CantLose);
        assert_matches!(
            deckmaste_authoring::OutcomeGateKind::CantLose.lower(),
            deckmaste_core::OutcomeGateKind::CantLose
        );
    }

    #[test]
    fn lowers_outcome_gate_kind_cant_win() {
        assert_lowers(deckmaste_authoring::OutcomeGateKind::CantWin);
        assert_matches!(
            deckmaste_authoring::OutcomeGateKind::CantWin.lower(),
            deckmaste_core::OutcomeGateKind::CantWin
        );
    }

    #[test]
    fn lowers_ignore_rule_ignore_lowest() {
        assert_lowers(deckmaste_authoring::IgnoreRule::IgnoreLowest);
        assert_matches!(
            deckmaste_authoring::IgnoreRule::IgnoreLowest.lower(),
            deckmaste_core::IgnoreRule::IgnoreLowest
        );
    }

    #[test]
    fn lowers_ignore_rule_ignore_chosen() {
        assert_lowers(deckmaste_authoring::IgnoreRule::IgnoreChosen(0));
        assert_matches!(
            deckmaste_authoring::IgnoreRule::IgnoreChosen(0).lower(),
            deckmaste_core::IgnoreRule::IgnoreChosen(..)
        );
    }

    #[test]
    fn lowers_pip_class_generic() {
        assert_lowers(deckmaste_authoring::PipClass::Generic);
        assert_matches!(
            deckmaste_authoring::PipClass::Generic.lower(),
            deckmaste_core::PipClass::Generic
        );
    }

    #[test]
    fn lowers_pip_class_colored() {
        assert_lowers(deckmaste_authoring::PipClass::Colored(minimal_color()));
        assert_matches!(
            deckmaste_authoring::PipClass::Colored(minimal_color()).lower(),
            deckmaste_core::PipClass::Colored(..)
        );
    }

    #[test]
    fn lowers_pay_act_tap_to_pay() {
        assert_lowers(deckmaste_authoring::PayAct::TapToPay(minimal_predicate()));
        assert_matches!(
            deckmaste_authoring::PayAct::TapToPay(minimal_predicate()).lower(),
            deckmaste_core::PayAct::TapToPay(..)
        );
    }

    #[test]
    fn lowers_pay_act_exile_to_pay() {
        assert_lowers(deckmaste_authoring::PayAct::ExileToPay(minimal_predicate()));
        assert_matches!(
            deckmaste_authoring::PayAct::ExileToPay(minimal_predicate()).lower(),
            deckmaste_core::PayAct::ExileToPay(..)
        );
    }

    #[test]
    fn lowers_player_attr_life() {
        assert_lowers(deckmaste_authoring::PlayerAttr::Life);
        assert_matches!(
            deckmaste_authoring::PlayerAttr::Life.lower(),
            deckmaste_core::PlayerAttr::Life
        );
    }

    #[test]
    fn lowers_player_attr_hand_size() {
        assert_lowers(deckmaste_authoring::PlayerAttr::HandSize);
        assert_matches!(
            deckmaste_authoring::PlayerAttr::HandSize.lower(),
            deckmaste_core::PlayerAttr::HandSize
        );
    }

    #[test]
    fn lowers_player_attr_hand_size_limit() {
        assert_lowers(deckmaste_authoring::PlayerAttr::HandSizeLimit);
        assert_matches!(
            deckmaste_authoring::PlayerAttr::HandSizeLimit.lower(),
            deckmaste_core::PlayerAttr::HandSizeLimit
        );
    }

    #[test]
    fn lowers_player_attr_land_plays_per_turn() {
        assert_lowers(deckmaste_authoring::PlayerAttr::LandPlaysPerTurn);
        assert_matches!(
            deckmaste_authoring::PlayerAttr::LandPlaysPerTurn.lower(),
            deckmaste_core::PlayerAttr::LandPlaysPerTurn
        );
    }

    #[test]
    fn lowers_player_mod_set_to() {
        assert_lowers(deckmaste_authoring::PlayerMod::SetTo(
            minimal_player_attr(),
            minimal_count(),
        ));
        assert_matches!(
            deckmaste_authoring::PlayerMod::SetTo(minimal_player_attr(), minimal_count()).lower(),
            deckmaste_core::PlayerMod::SetTo(..)
        );
    }

    #[test]
    fn lowers_player_mod_raise() {
        assert_lowers(deckmaste_authoring::PlayerMod::Raise(
            minimal_player_attr(),
            minimal_count(),
        ));
        assert_matches!(
            deckmaste_authoring::PlayerMod::Raise(minimal_player_attr(), minimal_count()).lower(),
            deckmaste_core::PlayerMod::Raise(..)
        );
    }

    #[test]
    fn lowers_player_mod_lower() {
        assert_lowers(deckmaste_authoring::PlayerMod::Lower(
            minimal_player_attr(),
            minimal_count(),
        ));
        assert_matches!(
            deckmaste_authoring::PlayerMod::Lower(minimal_player_attr(), minimal_count()).lower(),
            deckmaste_core::PlayerMod::Lower(..)
        );
    }

    #[test]
    fn lowers_player_mod_no_max() {
        assert_lowers(deckmaste_authoring::PlayerMod::NoMax(minimal_player_attr()));
        assert_matches!(
            deckmaste_authoring::PlayerMod::NoMax(minimal_player_attr()).lower(),
            deckmaste_core::PlayerMod::NoMax(..)
        );
    }
}
