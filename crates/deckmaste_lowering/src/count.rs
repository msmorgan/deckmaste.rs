//! `count` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Stat {
    type Target = deckmaste_core::Stat;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Power => deckmaste_core::Stat::Power,
            Self::Toughness => deckmaste_core::Stat::Toughness,
            Self::ManaValue => deckmaste_core::Stat::ManaValue,
            Self::Loyalty => deckmaste_core::Stat::Loyalty,
            Self::Defense => deckmaste_core::Stat::Defense,
        }
    }
}

impl Lower for deckmaste_authoring::RoundMode {
    type Target = deckmaste_core::RoundMode;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::RoundUp => deckmaste_core::RoundMode::RoundUp,
            Self::RoundDown => deckmaste_core::RoundMode::RoundDown,
        }
    }
}

impl Lower for deckmaste_authoring::Characteristic {
    type Target = deckmaste_core::Characteristic;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Colors => deckmaste_core::Characteristic::Colors,
            Self::Types => deckmaste_core::Characteristic::Types,
            Self::Subtypes => deckmaste_core::Characteristic::Subtypes,
            Self::BasicLandTypes => deckmaste_core::Characteristic::BasicLandTypes,
            Self::Supertypes => deckmaste_core::Characteristic::Supertypes,
            Self::Power => deckmaste_core::Characteristic::Power,
            Self::Toughness => deckmaste_core::Characteristic::Toughness,
            Self::Defense => deckmaste_core::Characteristic::Defense,
            Self::ManaCost => deckmaste_core::Characteristic::ManaCost,
            Self::Name => deckmaste_core::Characteristic::Name,
        }
    }
}

impl Lower for deckmaste_authoring::Countable {
    type Target = deckmaste_core::Countable;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Objects(f0) => deckmaste_core::Countable::Objects(f0.lower()),
            Self::Players(f0) => deckmaste_core::Countable::Players(f0.lower()),
            Self::ManaSymbols(f0, f1) => {
                deckmaste_core::Countable::ManaSymbols(f0.lower(), f1.lower())
            }
            Self::Singleton(f0) => deckmaste_core::Countable::Singleton(f0.lower()),
            Self::ManaSpentMatching(f0, f1) => {
                deckmaste_core::Countable::ManaSpentMatching(f0.lower(), f1.lower())
            }
        }
    }
}

impl Lower for deckmaste_authoring::AggregateOp {
    type Target = deckmaste_core::AggregateOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SumOf => deckmaste_core::AggregateOp::SumOf,
            Self::MinOf => deckmaste_core::AggregateOp::MinOf,
            Self::MaxOf => deckmaste_core::AggregateOp::MaxOf,
            Self::AverageOf(f0) => deckmaste_core::AggregateOp::AverageOf(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Projection {
    type Target = deckmaste_core::Projection;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Projection {
            of: self.of.lower(),
            by: self.by.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Count {
    type Target = deckmaste_core::Count;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::X => deckmaste_core::Count::X,
            Self::CountOf(f0) => deckmaste_core::Count::CountOf(f0.lower()),
            Self::CountDistinct(f0, f1) => {
                deckmaste_core::Count::CountDistinct(f0.lower(), f1.lower())
            }
            Self::StatOf(f0, f1) => deckmaste_core::Count::StatOf(f0.lower(), f1.lower()),
            Self::PlayerStatOf(f0, f1) => {
                deckmaste_core::Count::PlayerStatOf(f0.lower(), f1.lower())
            }
            Self::Opponents(f0) => deckmaste_core::Count::Opponents(f0.lower()),
            Self::CounterCount(f0, f1) => {
                deckmaste_core::Count::CounterCount(f0.lower(), f1.lower())
            }
            Self::Min(f0, f1) => deckmaste_core::Count::Min(f0.lower(), f1.lower()),
            Self::Max(f0, f1) => deckmaste_core::Count::Max(f0.lower(), f1.lower()),
            Self::Plus(f0, f1) => deckmaste_core::Count::Plus(f0.lower(), f1.lower()),
            Self::Minus(f0, f1) => deckmaste_core::Count::Minus(f0.lower(), f1.lower()),
            Self::Times(f0, f1) => deckmaste_core::Count::Times(f0.lower(), f1.lower()),
            Self::Half(f0, f1) => deckmaste_core::Count::Half(f0.lower(), f1.lower()),
            Self::Divide(f0, f1, f2) => {
                deckmaste_core::Count::Divide(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Mod(f0, f1) => deckmaste_core::Count::Mod(f0.lower(), f1.lower()),
            Self::Pow(f0, f1) => deckmaste_core::Count::Pow(f0.lower(), f1.lower()),
            Self::TargetsOf(f0) => deckmaste_core::Count::TargetsOf(f0.lower()),
            Self::ThatMany => deckmaste_core::Count::ThatMany,
            Self::ThatMuch => deckmaste_core::Count::ThatMuch,
            Self::Allotment => deckmaste_core::Count::Allotment,
            Self::EventCount(f0, f1) => deckmaste_core::Count::EventCount(f0.lower(), f1.lower()),
            Self::EventSum(f0, f1) => deckmaste_core::Count::EventSum(f0.lower(), f1.lower()),
            Self::Noted(f0) => deckmaste_core::Count::Noted(f0.lower()),
            Self::TimesPaid(f0) => deckmaste_core::Count::TimesPaid(f0.lower()),
            Self::Damage(f0) => deckmaste_core::Count::Damage(f0.lower()),
            Self::ManaAvailable(f0) => deckmaste_core::Count::ManaAvailable(f0.lower()),
            Self::Aggregate(f0, f1) => deckmaste_core::Count::Aggregate(f0.lower(), f1.lower()),
            Self::Expanded(f0) => deckmaste_core::Count::Expanded(f0.lower()),
            Self::Literal(f0) => deckmaste_core::Count::Literal(f0.lower()),
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
    fn lowers_stat_power() {
        assert_lowers(deckmaste_authoring::Stat::Power);
        assert_matches!(
            deckmaste_authoring::Stat::Power.lower(),
            deckmaste_core::Stat::Power
        );
    }

    #[test]
    fn lowers_stat_toughness() {
        assert_lowers(deckmaste_authoring::Stat::Toughness);
        assert_matches!(
            deckmaste_authoring::Stat::Toughness.lower(),
            deckmaste_core::Stat::Toughness
        );
    }

    #[test]
    fn lowers_stat_mana_value() {
        assert_lowers(deckmaste_authoring::Stat::ManaValue);
        assert_matches!(
            deckmaste_authoring::Stat::ManaValue.lower(),
            deckmaste_core::Stat::ManaValue
        );
    }

    #[test]
    fn lowers_stat_loyalty() {
        assert_lowers(deckmaste_authoring::Stat::Loyalty);
        assert_matches!(
            deckmaste_authoring::Stat::Loyalty.lower(),
            deckmaste_core::Stat::Loyalty
        );
    }

    #[test]
    fn lowers_stat_defense() {
        assert_lowers(deckmaste_authoring::Stat::Defense);
        assert_matches!(
            deckmaste_authoring::Stat::Defense.lower(),
            deckmaste_core::Stat::Defense
        );
    }

    #[test]
    fn lowers_round_mode_round_up() {
        assert_lowers(deckmaste_authoring::RoundMode::RoundUp);
        assert_matches!(
            deckmaste_authoring::RoundMode::RoundUp.lower(),
            deckmaste_core::RoundMode::RoundUp
        );
    }

    #[test]
    fn lowers_round_mode_round_down() {
        assert_lowers(deckmaste_authoring::RoundMode::RoundDown);
        assert_matches!(
            deckmaste_authoring::RoundMode::RoundDown.lower(),
            deckmaste_core::RoundMode::RoundDown
        );
    }

    #[test]
    fn lowers_characteristic_colors() {
        assert_lowers(deckmaste_authoring::Characteristic::Colors);
        assert_matches!(
            deckmaste_authoring::Characteristic::Colors.lower(),
            deckmaste_core::Characteristic::Colors
        );
    }

    #[test]
    fn lowers_characteristic_types() {
        assert_lowers(deckmaste_authoring::Characteristic::Types);
        assert_matches!(
            deckmaste_authoring::Characteristic::Types.lower(),
            deckmaste_core::Characteristic::Types
        );
    }

    #[test]
    fn lowers_characteristic_subtypes() {
        assert_lowers(deckmaste_authoring::Characteristic::Subtypes);
        assert_matches!(
            deckmaste_authoring::Characteristic::Subtypes.lower(),
            deckmaste_core::Characteristic::Subtypes
        );
    }

    #[test]
    fn lowers_characteristic_basic_land_types() {
        assert_lowers(deckmaste_authoring::Characteristic::BasicLandTypes);
        assert_matches!(
            deckmaste_authoring::Characteristic::BasicLandTypes.lower(),
            deckmaste_core::Characteristic::BasicLandTypes
        );
    }

    #[test]
    fn lowers_characteristic_supertypes() {
        assert_lowers(deckmaste_authoring::Characteristic::Supertypes);
        assert_matches!(
            deckmaste_authoring::Characteristic::Supertypes.lower(),
            deckmaste_core::Characteristic::Supertypes
        );
    }

    #[test]
    fn lowers_characteristic_power() {
        assert_lowers(deckmaste_authoring::Characteristic::Power);
        assert_matches!(
            deckmaste_authoring::Characteristic::Power.lower(),
            deckmaste_core::Characteristic::Power
        );
    }

    #[test]
    fn lowers_characteristic_toughness() {
        assert_lowers(deckmaste_authoring::Characteristic::Toughness);
        assert_matches!(
            deckmaste_authoring::Characteristic::Toughness.lower(),
            deckmaste_core::Characteristic::Toughness
        );
    }

    #[test]
    fn lowers_characteristic_defense() {
        assert_lowers(deckmaste_authoring::Characteristic::Defense);
        assert_matches!(
            deckmaste_authoring::Characteristic::Defense.lower(),
            deckmaste_core::Characteristic::Defense
        );
    }

    #[test]
    fn lowers_characteristic_mana_cost() {
        assert_lowers(deckmaste_authoring::Characteristic::ManaCost);
        assert_matches!(
            deckmaste_authoring::Characteristic::ManaCost.lower(),
            deckmaste_core::Characteristic::ManaCost
        );
    }

    #[test]
    fn lowers_characteristic_name() {
        assert_lowers(deckmaste_authoring::Characteristic::Name);
        assert_matches!(
            deckmaste_authoring::Characteristic::Name.lower(),
            deckmaste_core::Characteristic::Name
        );
    }

    #[test]
    fn lowers_countable_objects() {
        assert_lowers(deckmaste_authoring::Countable::Objects(
            std::sync::Arc::new(minimal_predicate()),
        ));
        assert_matches!(
            deckmaste_authoring::Countable::Objects(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::Countable::Objects(..)
        );
    }

    #[test]
    fn lowers_countable_players() {
        assert_lowers(deckmaste_authoring::Countable::Players(
            std::sync::Arc::new(minimal_predicate()),
        ));
        assert_matches!(
            deckmaste_authoring::Countable::Players(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::Countable::Players(..)
        );
    }

    #[test]
    fn lowers_countable_mana_symbols() {
        assert_lowers(deckmaste_authoring::Countable::ManaSymbols(
            std::sync::Arc::new(minimal_reference()),
            minimal_symbol_pred(),
        ));
        assert_matches!(
            deckmaste_authoring::Countable::ManaSymbols(
                std::sync::Arc::new(minimal_reference()),
                minimal_symbol_pred()
            )
            .lower(),
            deckmaste_core::Countable::ManaSymbols(..)
        );
    }

    #[test]
    fn lowers_countable_singleton() {
        assert_lowers(deckmaste_authoring::Countable::Singleton(
            std::sync::Arc::new(minimal_reference()),
        ));
        assert_matches!(
            deckmaste_authoring::Countable::Singleton(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Countable::Singleton(..)
        );
    }

    #[test]
    fn lowers_countable_mana_spent_matching() {
        assert_lowers(deckmaste_authoring::Countable::ManaSpentMatching(
            std::sync::Arc::new(minimal_reference()),
            minimal_symbol_pred(),
        ));
        assert_matches!(
            deckmaste_authoring::Countable::ManaSpentMatching(
                std::sync::Arc::new(minimal_reference()),
                minimal_symbol_pred()
            )
            .lower(),
            deckmaste_core::Countable::ManaSpentMatching(..)
        );
    }

    #[test]
    fn lowers_aggregate_op_sum_of() {
        assert_lowers(deckmaste_authoring::AggregateOp::SumOf);
        assert_matches!(
            deckmaste_authoring::AggregateOp::SumOf.lower(),
            deckmaste_core::AggregateOp::SumOf
        );
    }

    #[test]
    fn lowers_aggregate_op_min_of() {
        assert_lowers(deckmaste_authoring::AggregateOp::MinOf);
        assert_matches!(
            deckmaste_authoring::AggregateOp::MinOf.lower(),
            deckmaste_core::AggregateOp::MinOf
        );
    }

    #[test]
    fn lowers_aggregate_op_max_of() {
        assert_lowers(deckmaste_authoring::AggregateOp::MaxOf);
        assert_matches!(
            deckmaste_authoring::AggregateOp::MaxOf.lower(),
            deckmaste_core::AggregateOp::MaxOf
        );
    }

    #[test]
    fn lowers_aggregate_op_average_of() {
        assert_lowers(deckmaste_authoring::AggregateOp::AverageOf(
            minimal_round_mode(),
        ));
        assert_matches!(
            deckmaste_authoring::AggregateOp::AverageOf(minimal_round_mode()).lower(),
            deckmaste_core::AggregateOp::AverageOf(..)
        );
    }

    #[test]
    fn lowers_projection() {
        assert_lowers(deckmaste_authoring::Projection {
            of: minimal_countable(),
            by: std::sync::Arc::new(minimal_count()),
        });
    }

    #[test]
    fn lowers_count_x() {
        assert_lowers_debug(deckmaste_authoring::Count::X);
        assert_matches!(
            deckmaste_authoring::Count::X.lower(),
            deckmaste_core::Count::X
        );
    }

    #[test]
    fn lowers_count_count_of() {
        assert_lowers_debug(deckmaste_authoring::Count::CountOf(minimal_countable()));
        assert_matches!(
            deckmaste_authoring::Count::CountOf(minimal_countable()).lower(),
            deckmaste_core::Count::CountOf(..)
        );
    }

    #[test]
    fn lowers_count_count_distinct() {
        assert_lowers_debug(deckmaste_authoring::Count::CountDistinct(
            minimal_characteristic(),
            minimal_countable(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::CountDistinct(
                minimal_characteristic(),
                minimal_countable()
            )
            .lower(),
            deckmaste_core::Count::CountDistinct(..)
        );
    }

    #[test]
    fn lowers_count_stat_of() {
        assert_lowers_debug(deckmaste_authoring::Count::StatOf(
            minimal_reference(),
            minimal_stat(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::StatOf(minimal_reference(), minimal_stat()).lower(),
            deckmaste_core::Count::StatOf(..)
        );
    }

    #[test]
    fn lowers_count_player_stat_of() {
        assert_lowers_debug(deckmaste_authoring::Count::PlayerStatOf(
            minimal_reference(),
            minimal_player_attr(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::PlayerStatOf(minimal_reference(), minimal_player_attr())
                .lower(),
            deckmaste_core::Count::PlayerStatOf(..)
        );
    }

    #[test]
    fn lowers_count_opponents() {
        assert_lowers_debug(deckmaste_authoring::Count::Opponents(minimal_reference()));
        assert_matches!(
            deckmaste_authoring::Count::Opponents(minimal_reference()).lower(),
            deckmaste_core::Count::Opponents(..)
        );
    }

    #[test]
    fn lowers_count_counter_count() {
        assert_lowers_debug(deckmaste_authoring::Count::CounterCount(
            std::sync::Arc::new(minimal_reference()),
            minimal_counter_ref(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::CounterCount(
                std::sync::Arc::new(minimal_reference()),
                minimal_counter_ref()
            )
            .lower(),
            deckmaste_core::Count::CounterCount(..)
        );
    }

    #[test]
    fn lowers_count_min() {
        assert_lowers_debug(deckmaste_authoring::Count::Min(
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Min(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Min(..)
        );
    }

    #[test]
    fn lowers_count_max() {
        assert_lowers_debug(deckmaste_authoring::Count::Max(
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Max(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Max(..)
        );
    }

    #[test]
    fn lowers_count_plus() {
        assert_lowers_debug(deckmaste_authoring::Count::Plus(
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Plus(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Plus(..)
        );
    }

    #[test]
    fn lowers_count_minus() {
        assert_lowers_debug(deckmaste_authoring::Count::Minus(
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Minus(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Minus(..)
        );
    }

    #[test]
    fn lowers_count_times() {
        assert_lowers_debug(deckmaste_authoring::Count::Times(
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Times(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Times(..)
        );
    }

    #[test]
    fn lowers_count_half() {
        assert_lowers_debug(deckmaste_authoring::Count::Half(
            minimal_round_mode(),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Half(
                minimal_round_mode(),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Half(..)
        );
    }

    #[test]
    fn lowers_count_divide() {
        assert_lowers_debug(deckmaste_authoring::Count::Divide(
            minimal_round_mode(),
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Divide(
                minimal_round_mode(),
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Divide(..)
        );
    }

    #[test]
    fn lowers_count_mod() {
        assert_lowers_debug(deckmaste_authoring::Count::Mod(
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Mod(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Mod(..)
        );
    }

    #[test]
    fn lowers_count_pow() {
        assert_lowers_debug(deckmaste_authoring::Count::Pow(
            std::sync::Arc::new(minimal_count()),
            std::sync::Arc::new(minimal_count()),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Pow(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Pow(..)
        );
    }

    #[test]
    fn lowers_count_targets_of() {
        assert_lowers_debug(deckmaste_authoring::Count::TargetsOf(minimal_reference()));
        assert_matches!(
            deckmaste_authoring::Count::TargetsOf(minimal_reference()).lower(),
            deckmaste_core::Count::TargetsOf(..)
        );
    }

    #[test]
    fn lowers_count_that_many() {
        assert_lowers_debug(deckmaste_authoring::Count::ThatMany);
        assert_matches!(
            deckmaste_authoring::Count::ThatMany.lower(),
            deckmaste_core::Count::ThatMany
        );
    }

    #[test]
    fn lowers_count_that_much() {
        assert_lowers_debug(deckmaste_authoring::Count::ThatMuch);
        assert_matches!(
            deckmaste_authoring::Count::ThatMuch.lower(),
            deckmaste_core::Count::ThatMuch
        );
    }

    #[test]
    fn lowers_count_allotment() {
        assert_lowers_debug(deckmaste_authoring::Count::Allotment);
        assert_matches!(
            deckmaste_authoring::Count::Allotment.lower(),
            deckmaste_core::Count::Allotment
        );
    }

    #[test]
    fn lowers_count_event_count() {
        assert_lowers_debug(deckmaste_authoring::Count::EventCount(
            std::sync::Arc::new(minimal_event_filter()),
            minimal_lookback(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::EventCount(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            deckmaste_core::Count::EventCount(..)
        );
    }

    #[test]
    fn lowers_count_event_sum() {
        assert_lowers_debug(deckmaste_authoring::Count::EventSum(
            std::sync::Arc::new(minimal_event_filter()),
            minimal_lookback(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::EventSum(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            deckmaste_core::Count::EventSum(..)
        );
    }

    #[test]
    fn lowers_count_noted() {
        assert_lowers_debug(deckmaste_authoring::Count::Noted("X".into()));
        assert_matches!(
            deckmaste_authoring::Count::Noted("X".into()).lower(),
            deckmaste_core::Count::Noted(..)
        );
    }

    #[test]
    fn lowers_count_times_paid() {
        assert_lowers_debug(deckmaste_authoring::Count::TimesPaid(minimal_cost_tag()));
        assert_matches!(
            deckmaste_authoring::Count::TimesPaid(minimal_cost_tag()).lower(),
            deckmaste_core::Count::TimesPaid(..)
        );
    }

    #[test]
    fn lowers_count_damage() {
        assert_lowers_debug(deckmaste_authoring::Count::Damage(minimal_reference()));
        assert_matches!(
            deckmaste_authoring::Count::Damage(minimal_reference()).lower(),
            deckmaste_core::Count::Damage(..)
        );
    }

    #[test]
    fn lowers_count_mana_available() {
        assert_lowers_debug(deckmaste_authoring::Count::ManaAvailable(
            minimal_reference(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::ManaAvailable(minimal_reference()).lower(),
            deckmaste_core::Count::ManaAvailable(..)
        );
    }

    #[test]
    fn lowers_count_aggregate() {
        assert_lowers_debug(deckmaste_authoring::Count::Aggregate(
            minimal_aggregate_op(),
            minimal_projection(),
        ));
        assert_matches!(
            deckmaste_authoring::Count::Aggregate(minimal_aggregate_op(), minimal_projection())
                .lower(),
            deckmaste_core::Count::Aggregate(..)
        );
    }

    #[test]
    fn lowers_count_expanded() {
        assert_lowers_debug(deckmaste_authoring::Count::Expanded(macro_ron::Expansion {
            name: "X".into(),
            args: macro_ron::ExpansionArgs::none(),
            template: None,
            value: Box::new(minimal_count()),
        }));
        assert_matches!(
            deckmaste_authoring::Count::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_count())
            })
            .lower(),
            deckmaste_core::Count::Expanded(..)
        );
    }

    #[test]
    fn lowers_count_literal() {
        assert_lowers_debug(deckmaste_authoring::Count::Literal(0));
        assert_matches!(
            deckmaste_authoring::Count::Literal(0).lower(),
            deckmaste_core::Count::Literal(..)
        );
    }
}
