//! `continuous` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

impl Lower for deckmaste_semantics::Duration {
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
    fn lowers_duration_fixed_until() {
        assert_matches!(
            deckmaste_semantics::Duration::FixedUntil(minimal_turn_marker()).lower(),
            deckmaste_core::Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
        );
    }

    #[test]
    fn lowers_duration_until_event() {
        assert_matches!(
            deckmaste_semantics::Duration::UntilEvent(minimal_event_filter()).lower(),
            deckmaste_core::Duration::UntilEvent(deckmaste_core::EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            })
        );
    }

    #[test]
    fn lowers_duration_for_as_long_as() {
        assert_matches!(
            deckmaste_semantics::Duration::ForAsLongAs(minimal_condition()).lower(),
            deckmaste_core::Duration::ForAsLongAs(deckmaste_core::Condition::Compare(
                deckmaste_core::Count::Literal(0),
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::Literal(0)
            ))
        );
    }

    #[test]
    fn lowers_duration_for_this_event() {
        assert_matches!(
            deckmaste_semantics::Duration::ForThisEvent.lower(),
            deckmaste_core::Duration::ForThisEvent
        );
    }

    #[test]
    fn lowers_duration_end_of_game() {
        assert_matches!(
            deckmaste_semantics::Duration::EndOfGame.lower(),
            deckmaste_core::Duration::EndOfGame
        );
    }

    #[test]
    fn lowers_numeric_op_set() {
        assert_matches!(
            deckmaste_semantics::NumericOp::Set(minimal_stat_value()).lower(),
            deckmaste_core::NumericOp::Set(deckmaste_core::StatValue::DefinedByAbility)
        );
    }

    #[test]
    fn lowers_numeric_op_up() {
        assert_matches!(
            deckmaste_semantics::NumericOp::Up(minimal_count()).lower(),
            deckmaste_core::NumericOp::Up(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_numeric_op_down() {
        assert_matches!(
            deckmaste_semantics::NumericOp::Down(minimal_count()).lower(),
            deckmaste_core::NumericOp::Down(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_collection_op_set() {
        assert_matches!(
            deckmaste_semantics::CollectionOp::<deckmaste_semantics::Color>::Set([].into()).lower(),
            deckmaste_core::CollectionOp::Set(_)
        );
    }

    #[test]
    fn lowers_collection_op_add() {
        assert_matches!(
            deckmaste_semantics::CollectionOp::<deckmaste_semantics::Color>::Add(minimal_color())
                .lower(),
            deckmaste_core::CollectionOp::Add(_)
        );
    }

    #[test]
    fn lowers_collection_op_remove() {
        assert_matches!(deckmaste_semantics::CollectionOp::<deckmaste_semantics::Color>::Remove(minimal_color()).lower(), deckmaste_core::CollectionOp::Remove(_));
    }

    #[test]
    fn lowers_modification_power() {
        assert_matches!(
            deckmaste_semantics::Modification::Power(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::Power(deckmaste_core::NumericOp::Set(
                deckmaste_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_toughness() {
        assert_matches!(
            deckmaste_semantics::Modification::Toughness(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::Toughness(deckmaste_core::NumericOp::Set(
                deckmaste_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_switch_power_toughness() {
        assert_matches!(
            deckmaste_semantics::Modification::SwitchPowerToughness.lower(),
            deckmaste_core::Modification::SwitchPowerToughness
        );
    }

    #[test]
    fn lowers_modification_colors() {
        assert_matches!(
            deckmaste_semantics::Modification::Colors(minimal_collection_op::<
                deckmaste_semantics::Color,
            >())
            .lower(),
            deckmaste_core::Modification::Colors(deckmaste_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_card_types() {
        assert_matches!(
            deckmaste_semantics::Modification::CardTypes(minimal_collection_op::<
                deckmaste_semantics::Ident,
            >())
            .lower(),
            deckmaste_core::Modification::CardTypes(deckmaste_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_subtypes() {
        assert_matches!(
            deckmaste_semantics::Modification::Subtypes(minimal_collection_op::<
                deckmaste_semantics::SubtypeRef,
            >())
            .lower(),
            deckmaste_core::Modification::Subtypes(deckmaste_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_supertypes() {
        assert_matches!(
            deckmaste_semantics::Modification::Supertypes(minimal_collection_op::<
                deckmaste_semantics::Supertype,
            >())
            .lower(),
            deckmaste_core::Modification::Supertypes(deckmaste_core::CollectionOp::Set(_))
        );
    }

    #[test]
    fn lowers_modification_gain_ability() {
        assert_matches!(
            deckmaste_semantics::Modification::GainAbility(std::sync::Arc::new(minimal_ability()))
                .lower(),
            deckmaste_core::Modification::GainAbility(_)
        );
    }

    #[test]
    fn lowers_modification_lose_ability() {
        assert_matches!(
            deckmaste_semantics::Modification::LoseAbility("X".into()).lower(),
            deckmaste_core::Modification::LoseAbility(_)
        );
    }

    #[test]
    fn lowers_modification_lose_all_abilities() {
        assert_matches!(
            deckmaste_semantics::Modification::LoseAllAbilities.lower(),
            deckmaste_core::Modification::LoseAllAbilities
        );
    }

    #[test]
    fn lowers_modification_cant_have_ability() {
        assert_matches!(
            deckmaste_semantics::Modification::CantHaveAbility("X".into()).lower(),
            deckmaste_core::Modification::CantHaveAbility(_)
        );
    }

    #[test]
    fn lowers_modification_set_controller() {
        assert_matches!(
            deckmaste_semantics::Modification::SetController(minimal_reference()).lower(),
            deckmaste_core::Modification::SetController(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_modification_set_text() {
        assert_matches!(
            deckmaste_semantics::Modification::SetText(String::new()).lower(),
            deckmaste_core::Modification::SetText(_)
        );
    }

    #[test]
    fn lowers_modification_all_creature_types() {
        assert_matches!(
            deckmaste_semantics::Modification::AllCreatureTypes.lower(),
            deckmaste_core::Modification::AllCreatureTypes
        );
    }

    #[test]
    fn lowers_modification_base_loyalty() {
        assert_matches!(
            deckmaste_semantics::Modification::BaseLoyalty(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::BaseLoyalty(deckmaste_core::NumericOp::Set(
                deckmaste_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_base_defense() {
        assert_matches!(
            deckmaste_semantics::Modification::BaseDefense(minimal_numeric_op()).lower(),
            deckmaste_core::Modification::BaseDefense(deckmaste_core::NumericOp::Set(
                deckmaste_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_modification_become_basic_land_type() {
        assert_matches!(
            deckmaste_semantics::Modification::BecomeBasicLandType([].into()).lower(),
            deckmaste_core::Modification::BecomeBasicLandType(_)
        );
    }

    #[test]
    fn lowers_modification_several() {
        assert_matches!(
            deckmaste_semantics::Modification::Several([].into()).lower(),
            deckmaste_core::Modification::Several(_)
        );
    }

    #[test]
    fn lowers_modification_expanded() {
        assert_matches!(
            deckmaste_semantics::Modification::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_modification())
            })
            .lower(),
            deckmaste_core::Modification::Power(deckmaste_core::NumericOp::Set(
                deckmaste_core::StatValue::DefinedByAbility
            ))
        );
    }

    #[test]
    fn lowers_cost_change_increase() {
        assert_matches!(
            deckmaste_semantics::CostChange::Increase([].into()).lower(),
            deckmaste_core::CostChange::Increase(_)
        );
    }

    #[test]
    fn lowers_cost_change_reduce() {
        assert_matches!(
            deckmaste_semantics::CostChange::Reduce([].into()).lower(),
            deckmaste_core::CostChange::Reduce(_)
        );
    }

    #[test]
    fn lowers_cost_change_additional() {
        assert_matches!(
            deckmaste_semantics::CostChange::Additional {
                components: [].into()
            }
            .lower(),
            deckmaste_core::CostChange::Additional { components: _ }
        );
    }

    #[test]
    fn lowers_cost_change_scaled() {
        assert_matches!(
            deckmaste_semantics::CostChange::Scaled {
                change: std::sync::Arc::new(minimal_cost_change()),
                times: minimal_count()
            }
            .lower(),
            deckmaste_core::CostChange::Scaled {
                change: _,
                times: deckmaste_core::Count::Literal(0)
            }
        );
    }

    #[test]
    fn lowers_static_effect_modify() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::Modify(minimal_reference(), minimal_modification())
                .lower(),
            deckmaste_core::StaticSpec::Modify(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Modification::Power(deckmaste_core::NumericOp::Set(
                    deckmaste_core::StatValue::DefinedByAbility
                ))
            )
        );
    }

    #[test]
    fn lowers_static_effect_becomes_copy() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::BecomesCopy(
                minimal_reference(),
                minimal_copy_spec()
            )
            .lower(),
            deckmaste_core::StaticSpec::BecomesCopy(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::CopySpec {
                    source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::Reg(
                        deckmaste_core::RefId(0)
                    )),
                    exceptions: _
                }
            )
        );
    }

    #[test]
    fn lowers_static_effect_each() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::Each(
                minimal_selection(),
                std::sync::Arc::new(minimal_static_effect())
            )
            .lower(),
            deckmaste_core::StaticSpec::Each(deckmaste_core::Selection::SelectAll(_), _)
        );
    }

    #[test]
    fn lowers_static_effect_conditionally() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::Conditionally(
                minimal_condition(),
                std::sync::Arc::new(minimal_static_effect())
            )
            .lower(),
            deckmaste_core::StaticSpec::Conditionally(
                deckmaste_core::Condition::Compare(
                    deckmaste_core::Count::Literal(0),
                    deckmaste_core::Cmp::Eq,
                    deckmaste_core::Count::Literal(0)
                ),
                _
            )
        );
    }

    #[test]
    fn lowers_static_effect_deontic() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::Deontic(minimal_deontic()).lower(),
            deckmaste_core::StaticSpec::Deontic(deckmaste_core::Deontic::May(
                deckmaste_core::DeonticAction::Attack {
                    by: deckmaste_core::Predicate::Class(
                        deckmaste_core::ObjectClass::AbilityOnStack
                    ),
                    on: deckmaste_core::Predicate::Class(
                        deckmaste_core::ObjectClass::AbilityOnStack
                    )
                }
            ))
        );
    }

    #[test]
    fn lowers_static_effect_cost_modifier() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::CostModifier {
                of: minimal_predicate(),
                change: minimal_cost_change()
            }
            .lower(),
            deckmaste_core::StaticSpec::CostModifier {
                of: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                change: deckmaste_core::CostChange::Increase(_)
            }
        );
    }

    #[test]
    fn lowers_static_effect_cost_option() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::CostOption(minimal_optional_cost()).lower(),
            deckmaste_core::StaticSpec::CostOption(deckmaste_core::OptionalCost {
                components: _,
                tag: deckmaste_core::CostTag(_),
                repeatable: false
            })
        );
    }

    #[test]
    fn lowers_static_effect_trigger_multiplier() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::TriggerMultiplier {
                cause: minimal_event_filter(),
                extra: minimal_count(),
                affected: minimal_predicate()
            }
            .lower(),
            deckmaste_core::StaticSpec::TriggerMultiplier {
                cause: deckmaste_core::EventFilter::ZoneChange {
                    what: deckmaste_core::Predicate::Class(
                        deckmaste_core::ObjectClass::AbilityOnStack
                    ),
                    from: None,
                    to: None,
                    cause: None
                },
                extra: deckmaste_core::Count::Literal(0),
                affected: deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                )
            }
        );
    }

    #[test]
    fn lowers_static_effect_modify_player() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::ModifyPlayer(
                minimal_reference(),
                minimal_player_mod()
            )
            .lower(),
            deckmaste_core::StaticSpec::ModifyPlayer(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::PlayerMod::SetTo(
                    deckmaste_core::PlayerAttr::Life,
                    deckmaste_core::Count::Literal(0)
                )
            )
        );
    }

    #[test]
    fn lowers_static_effect_prevention() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::Prevention(
                std::sync::Arc::new(minimal_prevention())
            )
            .lower(),
            deckmaste_core::StaticSpec::Prevention(_)
        );
    }

    #[test]
    fn lowers_static_effect_cant_prevent() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::CantPrevent {
                from: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::StaticSpec::CantPrevent {
                from: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_static_effect_spend_as_though() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::SpendAsThough {
                mana_from: minimal_predicate(),
                as_: minimal_symbol_pred()
            }
            .lower(),
            deckmaste_core::StaticSpec::SpendAsThough {
                mana_from: deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                ),
                as_: deckmaste_core::SymbolPred::AnyColor
            }
        );
    }

    #[test]
    fn lowers_static_effect_as_though() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::AsThough(minimal_as_though()).lower(),
            deckmaste_core::StaticSpec::AsThough(deckmaste_core::AsThough::Counterfactual {
                premise: deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                ),
                then: _
            })
        );
    }

    #[test]
    fn lowers_static_effect_outcome_gate() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::OutcomeGate {
                who: minimal_predicate(),
                gate: minimal_outcome_gate_kind()
            }
            .lower(),
            deckmaste_core::StaticSpec::OutcomeGate {
                who: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                gate: deckmaste_core::OutcomeGateKind::CantLose
            }
        );
    }

    #[test]
    fn lowers_static_effect_cant_happen() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::CantHappen(minimal_event_filter()).lower(),
            deckmaste_core::StaticSpec::CantHappen(deckmaste_core::EventFilter::ZoneChange {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                to: None,
                cause: None
            })
        );
    }

    #[test]
    fn lowers_static_effect_replace_roll() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::ReplaceRoll {
                query: minimal_event_filter(),
                extra: minimal_count(),
                ignore: minimal_ignore_rule()
            }
            .lower(),
            deckmaste_core::StaticSpec::ReplaceRoll {
                query: deckmaste_core::EventFilter::ZoneChange {
                    what: deckmaste_core::Predicate::Class(
                        deckmaste_core::ObjectClass::AbilityOnStack
                    ),
                    from: None,
                    to: None,
                    cause: None
                },
                extra: deckmaste_core::Count::Literal(0),
                ignore: deckmaste_core::IgnoreRule::IgnoreLowest
            }
        );
    }

    #[test]
    fn lowers_static_effect_pay_pips() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::PayPips(minimal_pip_class(), minimal_pay_act())
                .lower(),
            deckmaste_core::StaticSpec::PayPips(
                deckmaste_core::PipClass::Generic,
                deckmaste_core::PayAct::TapToPay(deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                ))
            )
        );
    }

    #[test]
    fn lowers_static_effect_expanded() {
        assert_matches!(
            deckmaste_semantics::StaticEffect::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_static_effect())
            })
            .lower(),
            deckmaste_core::StaticSpec::Modify(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Modification::Power(deckmaste_core::NumericOp::Set(
                    deckmaste_core::StatValue::DefinedByAbility
                ))
            )
        );
    }

    #[test]
    fn lowers_outcome_gate_kind_cant_lose() {
        assert_matches!(
            deckmaste_semantics::OutcomeGateKind::CantLose.lower(),
            deckmaste_core::OutcomeGateKind::CantLose
        );
    }

    #[test]
    fn lowers_outcome_gate_kind_cant_win() {
        assert_matches!(
            deckmaste_semantics::OutcomeGateKind::CantWin.lower(),
            deckmaste_core::OutcomeGateKind::CantWin
        );
    }

    #[test]
    fn lowers_ignore_rule_ignore_lowest() {
        assert_matches!(
            deckmaste_semantics::IgnoreRule::IgnoreLowest.lower(),
            deckmaste_core::IgnoreRule::IgnoreLowest
        );
    }

    #[test]
    fn lowers_ignore_rule_ignore_chosen() {
        assert_matches!(
            deckmaste_semantics::IgnoreRule::IgnoreChosen(0).lower(),
            deckmaste_core::IgnoreRule::IgnoreChosen(0)
        );
    }

    #[test]
    fn lowers_pip_class_generic() {
        assert_matches!(
            deckmaste_semantics::PipClass::Generic.lower(),
            deckmaste_core::PipClass::Generic
        );
    }

    #[test]
    fn lowers_pip_class_colored() {
        assert_matches!(
            deckmaste_semantics::PipClass::Colored(minimal_color()).lower(),
            deckmaste_core::PipClass::Colored(deckmaste_core::Color::White)
        );
    }

    #[test]
    fn lowers_pay_act_tap_to_pay() {
        assert_matches!(
            deckmaste_semantics::PayAct::TapToPay(minimal_predicate()).lower(),
            deckmaste_core::PayAct::TapToPay(deckmaste_core::Predicate::Class(
                deckmaste_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_pay_act_exile_to_pay() {
        assert_matches!(
            deckmaste_semantics::PayAct::ExileToPay(minimal_predicate()).lower(),
            deckmaste_core::PayAct::ExileToPay(deckmaste_core::Predicate::Class(
                deckmaste_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_player_attr_life() {
        assert_matches!(
            deckmaste_semantics::PlayerAttr::Life.lower(),
            deckmaste_core::PlayerAttr::Life
        );
    }

    #[test]
    fn lowers_player_attr_hand_size() {
        assert_matches!(
            deckmaste_semantics::PlayerAttr::HandSize.lower(),
            deckmaste_core::PlayerAttr::HandSize
        );
    }

    #[test]
    fn lowers_player_attr_hand_size_limit() {
        assert_matches!(
            deckmaste_semantics::PlayerAttr::HandSizeLimit.lower(),
            deckmaste_core::PlayerAttr::HandSizeLimit
        );
    }

    #[test]
    fn lowers_player_attr_land_plays_per_turn() {
        assert_matches!(
            deckmaste_semantics::PlayerAttr::LandPlaysPerTurn.lower(),
            deckmaste_core::PlayerAttr::LandPlaysPerTurn
        );
    }

    #[test]
    fn lowers_player_mod_set_to() {
        assert_matches!(
            deckmaste_semantics::PlayerMod::SetTo(minimal_player_attr(), minimal_count()).lower(),
            deckmaste_core::PlayerMod::SetTo(
                deckmaste_core::PlayerAttr::Life,
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_player_mod_raise() {
        assert_matches!(
            deckmaste_semantics::PlayerMod::Raise(minimal_player_attr(), minimal_count()).lower(),
            deckmaste_core::PlayerMod::Raise(
                deckmaste_core::PlayerAttr::Life,
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_player_mod_lower() {
        assert_matches!(
            deckmaste_semantics::PlayerMod::Lower(minimal_player_attr(), minimal_count()).lower(),
            deckmaste_core::PlayerMod::Lower(
                deckmaste_core::PlayerAttr::Life,
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_player_mod_no_max() {
        assert_matches!(
            deckmaste_semantics::PlayerMod::NoMax(minimal_player_attr()).lower(),
            deckmaste_core::PlayerMod::NoMax(deckmaste_core::PlayerAttr::Life)
        );
    }

    #[test]
    fn lowers_static_effect_replacement() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::StaticEffect::Replacement(
                std::sync::Arc::new(minimal_replacement())
            )
            .lower()),
            deckmaste_core::StaticSpec::Replacement(_)
        );
    }

    #[test]
    fn lowers_static_effect_sba() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::StaticEffect::Sba {
                when: std::sync::Arc::new(minimal_condition()),
                then: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            deckmaste_core::StaticSpec::Sba { when: _, then: _ }
        );
    }
}

impl Lower for deckmaste_semantics::NumericOp {
    type Target = deckmaste_core::NumericOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => deckmaste_core::NumericOp::Set(f0.lower()),
            Self::Up(f0) => deckmaste_core::NumericOp::Up(f0.lower()),
            Self::Down(f0) => deckmaste_core::NumericOp::Down(f0.lower()),
        }
    }
}

impl<T: Lower + Clone> Lower for deckmaste_semantics::CollectionOp<T> {
    type Target = deckmaste_core::CollectionOp<<T as Lower>::Target>;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => deckmaste_core::CollectionOp::Set(f0.lower()),
            Self::Add(f0) => deckmaste_core::CollectionOp::Add(f0.lower()),
            Self::Remove(f0) => deckmaste_core::CollectionOp::Remove(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::Modification {
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
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::CostChange {
    type Target = deckmaste_core::CostChange;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Increase(f0) => {
                deckmaste_core::CostChange::Increase(crate::cost::lower_scoped_cost_components(&f0))
            }
            Self::Reduce(f0) => {
                deckmaste_core::CostChange::Reduce(crate::cost::lower_scoped_cost_components(&f0))
            }
            Self::Additional { components } => deckmaste_core::CostChange::Additional {
                components: crate::cost::lower_scoped_cost_components(&components),
            },
            Self::Scaled { change, times } => deckmaste_core::CostChange::Scaled {
                change: change.lower(),
                times: times.lower(),
            },
        }
    }
}

impl Lower for deckmaste_semantics::StaticEffect {
    type Target = deckmaste_core::StaticSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Modify(f0, f1) => deckmaste_core::StaticSpec::Modify(f0.lower(), f1.lower()),
            Self::BecomesCopy(f0, f1) => {
                deckmaste_core::StaticSpec::BecomesCopy(f0.lower(), f1.lower())
            }
            Self::Each(f0, f1) => deckmaste_core::StaticSpec::Each(
                f0.lower(),
                std::sync::Arc::new(crate::region::candidate_region(|| {
                    std::sync::Arc::unwrap_or_clone(f1).lower()
                })),
            ),
            Self::Conditionally(f0, f1) => {
                deckmaste_core::StaticSpec::Conditionally(f0.lower(), f1.lower())
            }
            Self::Deontic(f0) => deckmaste_core::StaticSpec::Deontic(f0.lower()),
            Self::CostModifier { of, change } => deckmaste_core::StaticSpec::CostModifier {
                of: of.lower(),
                change: change.lower(),
            },
            Self::CostOption(f0) => deckmaste_core::StaticSpec::CostOption(f0.lower()),
            Self::TriggerMultiplier {
                cause,
                extra,
                affected,
            } => deckmaste_core::StaticSpec::TriggerMultiplier {
                cause: cause.lower(),
                extra: extra.lower(),
                affected: affected.lower(),
            },
            Self::ModifyPlayer(f0, f1) => {
                deckmaste_core::StaticSpec::ModifyPlayer(f0.lower(), f1.lower())
            }
            Self::Replacement(f0) => deckmaste_core::StaticSpec::Replacement(f0.lower()),
            Self::Prevention(f0) => deckmaste_core::StaticSpec::Prevention(f0.lower()),
            Self::CantPrevent { from, to } => deckmaste_core::StaticSpec::CantPrevent {
                from: from.lower(),
                to: to.lower(),
            },
            Self::SpendAsThough { mana_from, as_ } => deckmaste_core::StaticSpec::SpendAsThough {
                mana_from: mana_from.lower(),
                as_: as_.lower(),
            },
            Self::AsThough(f0) => deckmaste_core::StaticSpec::AsThough(f0.lower()),
            Self::Sba { when, then } => deckmaste_core::StaticSpec::Sba {
                when: when.lower(),
                then: then.lower(),
            },
            Self::OutcomeGate { who, gate } => deckmaste_core::StaticSpec::OutcomeGate {
                who: who.lower(),
                gate: gate.lower(),
            },
            Self::CantHappen(f0) => deckmaste_core::StaticSpec::CantHappen(f0.lower()),
            Self::ReplaceRoll {
                query,
                extra,
                ignore,
            } => deckmaste_core::StaticSpec::ReplaceRoll {
                query: query.lower(),
                extra: extra.lower(),
                ignore: ignore.lower(),
            },
            Self::PayPips(f0, f1) => deckmaste_core::StaticSpec::PayPips(f0.lower(), f1.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::OutcomeGateKind {
    type Target = deckmaste_core::OutcomeGateKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::CantLose => deckmaste_core::OutcomeGateKind::CantLose,
            Self::CantWin => deckmaste_core::OutcomeGateKind::CantWin,
        }
    }
}

impl Lower for deckmaste_semantics::IgnoreRule {
    type Target = deckmaste_core::IgnoreRule;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::IgnoreLowest => deckmaste_core::IgnoreRule::IgnoreLowest,
            Self::IgnoreChosen(f0) => deckmaste_core::IgnoreRule::IgnoreChosen(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::PipClass {
    type Target = deckmaste_core::PipClass;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Generic => deckmaste_core::PipClass::Generic,
            Self::Colored(f0) => deckmaste_core::PipClass::Colored(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::PayAct {
    type Target = deckmaste_core::PayAct;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::TapToPay(f0) => deckmaste_core::PayAct::TapToPay(f0.lower()),
            Self::ExileToPay(f0) => deckmaste_core::PayAct::ExileToPay(f0.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::PlayerAttr {
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

impl Lower for deckmaste_semantics::PlayerMod {
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
