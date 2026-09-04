//! `count` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

impl Lower for deckmaste_semantics::Stat {
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
    fn lowers_stat_power() {
        assert_matches!(
            deckmaste_semantics::Stat::Power.lower(),
            deckmaste_core::Stat::Power
        );
    }

    #[test]
    fn lowers_stat_toughness() {
        assert_matches!(
            deckmaste_semantics::Stat::Toughness.lower(),
            deckmaste_core::Stat::Toughness
        );
    }

    #[test]
    fn lowers_stat_mana_value() {
        assert_matches!(
            deckmaste_semantics::Stat::ManaValue.lower(),
            deckmaste_core::Stat::ManaValue
        );
    }

    #[test]
    fn lowers_stat_loyalty() {
        assert_matches!(
            deckmaste_semantics::Stat::Loyalty.lower(),
            deckmaste_core::Stat::Loyalty
        );
    }

    #[test]
    fn lowers_stat_defense() {
        assert_matches!(
            deckmaste_semantics::Stat::Defense.lower(),
            deckmaste_core::Stat::Defense
        );
    }

    #[test]
    fn lowers_round_mode_round_up() {
        assert_matches!(
            deckmaste_semantics::RoundMode::RoundUp.lower(),
            deckmaste_core::RoundMode::RoundUp
        );
    }

    #[test]
    fn lowers_round_mode_round_down() {
        assert_matches!(
            deckmaste_semantics::RoundMode::RoundDown.lower(),
            deckmaste_core::RoundMode::RoundDown
        );
    }

    #[test]
    fn lowers_characteristic_colors() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Colors.lower(),
            deckmaste_core::Characteristic::Colors
        );
    }

    #[test]
    fn lowers_characteristic_types() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Types.lower(),
            deckmaste_core::Characteristic::Types
        );
    }

    #[test]
    fn lowers_characteristic_subtypes() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Subtypes.lower(),
            deckmaste_core::Characteristic::Subtypes
        );
    }

    #[test]
    fn lowers_characteristic_basic_land_types() {
        assert_matches!(
            deckmaste_semantics::Characteristic::BasicLandTypes.lower(),
            deckmaste_core::Characteristic::BasicLandTypes
        );
    }

    #[test]
    fn lowers_characteristic_supertypes() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Supertypes.lower(),
            deckmaste_core::Characteristic::Supertypes
        );
    }

    #[test]
    fn lowers_characteristic_power() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Power.lower(),
            deckmaste_core::Characteristic::Power
        );
    }

    #[test]
    fn lowers_characteristic_toughness() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Toughness.lower(),
            deckmaste_core::Characteristic::Toughness
        );
    }

    #[test]
    fn lowers_characteristic_defense() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Defense.lower(),
            deckmaste_core::Characteristic::Defense
        );
    }

    #[test]
    fn lowers_characteristic_mana_cost() {
        assert_matches!(
            deckmaste_semantics::Characteristic::ManaCost.lower(),
            deckmaste_core::Characteristic::ManaCost
        );
    }

    #[test]
    fn lowers_characteristic_name() {
        assert_matches!(
            deckmaste_semantics::Characteristic::Name.lower(),
            deckmaste_core::Characteristic::Name
        );
    }

    #[test]
    fn lowers_countable_objects() {
        assert_matches!(
            deckmaste_semantics::Countable::Objects(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::Countable::Objects(_)
        );
    }

    #[test]
    fn lowers_countable_players() {
        assert_matches!(
            deckmaste_semantics::Countable::Players(std::sync::Arc::new(minimal_predicate()))
                .lower(),
            deckmaste_core::Countable::Players(_)
        );
    }

    #[test]
    fn lowers_countable_mana_symbols() {
        assert_matches!(
            deckmaste_semantics::Countable::ManaSymbols(
                std::sync::Arc::new(minimal_reference()),
                minimal_symbol_pred()
            )
            .lower(),
            deckmaste_core::Countable::ManaSymbols(_, deckmaste_core::SymbolPred::AnyColor)
        );
    }

    #[test]
    fn lowers_countable_singleton() {
        assert_matches!(
            deckmaste_semantics::Countable::Singleton(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Countable::Singleton(_)
        );
    }

    #[test]
    fn lowers_countable_mana_spent_matching() {
        assert_matches!(
            deckmaste_semantics::Countable::ManaSpentMatching(
                std::sync::Arc::new(minimal_reference()),
                minimal_symbol_pred()
            )
            .lower(),
            deckmaste_core::Countable::ManaSpentMatching(_, deckmaste_core::SymbolPred::AnyColor)
        );
    }

    #[test]
    fn lowers_aggregate_op_sum_of() {
        assert_matches!(
            deckmaste_semantics::AggregateOp::SumOf.lower(),
            deckmaste_core::AggregateOp::SumOf
        );
    }

    #[test]
    fn lowers_aggregate_op_min_of() {
        assert_matches!(
            deckmaste_semantics::AggregateOp::MinOf.lower(),
            deckmaste_core::AggregateOp::MinOf
        );
    }

    #[test]
    fn lowers_aggregate_op_max_of() {
        assert_matches!(
            deckmaste_semantics::AggregateOp::MaxOf.lower(),
            deckmaste_core::AggregateOp::MaxOf
        );
    }

    #[test]
    fn lowers_aggregate_op_average_of() {
        assert_matches!(
            deckmaste_semantics::AggregateOp::AverageOf(minimal_round_mode()).lower(),
            deckmaste_core::AggregateOp::AverageOf(deckmaste_core::RoundMode::RoundUp)
        );
    }

    #[test]
    fn lowers_projection() {
        assert_matches!(
            deckmaste_semantics::Projection {
                of: minimal_countable(),
                by: std::sync::Arc::new(minimal_count())
            }
            .lower(),
            deckmaste_core::Projection {
                of: deckmaste_core::Countable::Objects(_),
                by: _
            }
        );
    }

    #[test]
    fn lowers_count_x() {
        use deckmaste_semantics::Count as SemValue;

        let (params, lowered) =
            crate::region::in_region(crate::region::RegionKind::Spell, 0, || SemValue::X.lower());
        let x = params
            .iter()
            .find(|param| param.provenance == deckmaste_core::Provenance::AnnouncedX)
            .expect("spell regions declare X");
        assert_eq!(lowered, deckmaste_core::Count::Reg(x.def.into()));
    }

    #[test]
    fn lowers_count_count_of() {
        assert_matches!(
            deckmaste_semantics::Count::CountOf(minimal_countable()).lower(),
            deckmaste_core::Count::CountOf(deckmaste_core::Countable::Objects(_))
        );
    }

    #[test]
    fn lowers_count_count_distinct() {
        assert_matches!(
            deckmaste_semantics::Count::CountDistinct(
                minimal_characteristic(),
                minimal_countable()
            )
            .lower(),
            deckmaste_core::Count::CountDistinct(
                deckmaste_core::Characteristic::Colors,
                deckmaste_core::Countable::Objects(_)
            )
        );
    }

    #[test]
    fn lowers_count_stat_of() {
        assert_matches!(
            deckmaste_semantics::Count::StatOf(minimal_reference(), minimal_stat()).lower(),
            deckmaste_core::Count::StatOf(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Stat::Power
            )
        );
    }

    #[test]
    fn lowers_count_player_stat_of() {
        assert_matches!(
            deckmaste_semantics::Count::PlayerStatOf(minimal_reference(), minimal_player_attr())
                .lower(),
            deckmaste_core::Count::PlayerStatOf(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::PlayerAttr::Life
            )
        );
    }

    #[test]
    fn lowers_count_opponents() {
        assert_matches!(
            deckmaste_semantics::Count::Opponents(minimal_reference()).lower(),
            deckmaste_core::Count::Opponents(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_count_counter_count() {
        assert_matches!(
            deckmaste_semantics::Count::CounterCount(
                std::sync::Arc::new(minimal_reference()),
                minimal_counter_ref()
            )
            .lower(),
            deckmaste_core::Count::CounterCount(_, deckmaste_core::CounterRef(_))
        );
    }

    #[test]
    fn lowers_count_min() {
        assert_matches!(
            deckmaste_semantics::Count::Min(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Min(_, _)
        );
    }

    #[test]
    fn lowers_count_max() {
        assert_matches!(
            deckmaste_semantics::Count::Max(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Max(_, _)
        );
    }

    #[test]
    fn lowers_count_plus() {
        assert_matches!(
            deckmaste_semantics::Count::Plus(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Plus(_, _)
        );
    }

    #[test]
    fn lowers_count_minus() {
        assert_matches!(
            deckmaste_semantics::Count::Minus(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Minus(_, _)
        );
    }

    #[test]
    fn lowers_count_times() {
        assert_matches!(
            deckmaste_semantics::Count::Times(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Times(_, _)
        );
    }

    #[test]
    fn lowers_count_half() {
        assert_matches!(
            deckmaste_semantics::Count::Half(
                minimal_round_mode(),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Half(deckmaste_core::RoundMode::RoundUp, _)
        );
    }

    #[test]
    fn lowers_count_divide() {
        assert_matches!(
            deckmaste_semantics::Count::Divide(
                minimal_round_mode(),
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Divide(deckmaste_core::RoundMode::RoundUp, _, _)
        );
    }

    #[test]
    fn lowers_count_mod() {
        assert_matches!(
            deckmaste_semantics::Count::Mod(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Mod(_, _)
        );
    }

    #[test]
    fn lowers_count_pow() {
        assert_matches!(
            deckmaste_semantics::Count::Pow(
                std::sync::Arc::new(minimal_count()),
                std::sync::Arc::new(minimal_count())
            )
            .lower(),
            deckmaste_core::Count::Pow(_, _)
        );
    }

    #[test]
    fn lowers_count_targets_of() {
        assert_matches!(
            deckmaste_semantics::Count::TargetsOf(minimal_reference()).lower(),
            deckmaste_core::Count::TargetsOf(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_count_event_count() {
        assert_matches!(
            deckmaste_semantics::Count::EventCount(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            deckmaste_core::Count::EventCount(_, deckmaste_core::Lookback::ThisTurn)
        );
    }

    #[test]
    fn lowers_count_event_sum() {
        assert_matches!(
            deckmaste_semantics::Count::EventSum(
                std::sync::Arc::new(minimal_event_filter()),
                minimal_lookback()
            )
            .lower(),
            deckmaste_core::Count::EventSum(_, deckmaste_core::Lookback::ThisTurn)
        );
    }

    #[test]
    fn lowers_count_noted() {
        use deckmaste_semantics::Count as SemValue;

        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            crate::region::bind_named("X".into(), deckmaste_core::RefId(0));
            SemValue::Noted("X".into()).lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Count::Reg(deckmaste_core::RefId(0))
        );
    }

    #[test]
    fn lowers_count_times_paid() {
        assert_matches!(
            deckmaste_semantics::Count::TimesPaid(minimal_cost_tag()).lower(),
            deckmaste_core::Count::TimesPaid(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_count_damage() {
        assert_matches!(
            deckmaste_semantics::Count::Damage(minimal_reference()).lower(),
            deckmaste_core::Count::Damage(deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_count_mana_available() {
        assert_matches!(
            deckmaste_semantics::Count::ManaAvailable(minimal_reference()).lower(),
            deckmaste_core::Count::ManaAvailable(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_count_mana_available_kind() {
        assert_matches!(
            deckmaste_semantics::Count::ManaAvailableKind(
                minimal_reference(),
                deckmaste_semantics::ColorOrColorless::Colorless,
            )
            .lower(),
            deckmaste_core::Count::ManaAvailableKind(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::ColorOrColorless::Colorless,
            )
        );
    }

    #[test]
    fn lowers_count_aggregate() {
        assert_matches!(
            deckmaste_semantics::Count::Aggregate(minimal_aggregate_op(), minimal_projection())
                .lower(),
            deckmaste_core::Count::Aggregate(
                deckmaste_core::AggregateOp::SumOf,
                deckmaste_core::Projection {
                    of: deckmaste_core::Countable::Objects(_),
                    by: _
                }
            )
        );
    }

    #[test]
    fn lowers_count_expanded() {
        assert_matches!(
            deckmaste_semantics::Count::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_count())
            })
            .lower(),
            deckmaste_core::Count::Literal(0)
        );
    }

    #[test]
    fn lowers_count_literal() {
        assert_matches!(
            deckmaste_semantics::Count::Literal(0).lower(),
            deckmaste_core::Count::Literal(0)
        );
    }

    /// "That many" names the magnitude a count pinned earlier ([CR#608.2h]).
    /// Re-spelled from `lowers_count_that_many`: `Count::ThatMany` left core
    /// with the discourse channel, so the anaphor's image is the register read
    /// of the pinned amount.
    #[test]
    fn lowers_count_that_many_to_the_pinned_amount_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let amount = crate::region::define(deckmaste_core::Kind::Number);
            crate::region::push_antecedent(
                amount.into(),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Product,
            );
            deckmaste_semantics::Count::ThatMany.lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Count::Reg(deckmaste_core::RefId(3))
        );
    }

    /// "That much" is the same register read as "that many" — one amount
    /// anaphor, one channel ([CR#608.2h]). Re-spelled from
    /// `lowers_count_that_much`.
    #[test]
    fn lowers_count_that_much_to_the_pinned_amount_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let amount = crate::region::define(deckmaste_core::Kind::Number);
            crate::region::push_antecedent(
                amount.into(),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Product,
            );
            deckmaste_semantics::Count::ThatMuch.lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Count::Reg(deckmaste_core::RefId(3))
        );
    }

    /// A distributed share reads the loop body's allotment parameter
    /// ([CR#601.2d]) rather than a `Count::Allotment` variant. Re-spelled from
    /// `lowers_count_allotment`.
    #[test]
    fn lowers_count_allotment_to_the_loop_bodys_allotment_parameter() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            crate::region::push_antecedent(
                deckmaste_core::RefId(1),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Allotment,
            );
            deckmaste_semantics::Count::Allotment.lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Count::Reg(deckmaste_core::RefId(1))
        );
    }

    /// The allotment channel is its own site: an ordinary pinned amount in
    /// scope is NOT what "that much" means inside a `Distribute` body
    /// ([CR#601.2d]), so an allotment read never falls back to it.
    #[test]
    #[should_panic(expected = "unbound distribution allotment")]
    fn allotment_does_not_fall_back_to_a_plain_amount_antecedent() {
        let _ = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let amount = crate::region::define(deckmaste_core::Kind::Number);
            crate::region::push_antecedent(
                amount.into(),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Product,
            );
            deckmaste_semantics::Count::Allotment.lower()
        });
    }
}

impl Lower for deckmaste_semantics::RoundMode {
    type Target = deckmaste_core::RoundMode;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::RoundUp => deckmaste_core::RoundMode::RoundUp,
            Self::RoundDown => deckmaste_core::RoundMode::RoundDown,
        }
    }
}

impl Lower for deckmaste_semantics::Characteristic {
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

impl Lower for deckmaste_semantics::Countable {
    type Target = deckmaste_core::Countable;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Objects(f0) => deckmaste_core::Countable::Objects(std::sync::Arc::new(
                crate::region::predicate_region(|| std::sync::Arc::unwrap_or_clone(f0).lower()),
            )),
            Self::Players(f0) => deckmaste_core::Countable::Players(std::sync::Arc::new(
                crate::region::predicate_region(|| std::sync::Arc::unwrap_or_clone(f0).lower()),
            )),
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

impl Lower for deckmaste_semantics::AggregateOp {
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

impl Lower for deckmaste_semantics::Projection {
    type Target = deckmaste_core::Projection;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Projection {
            of: self.of.lower(),
            by: std::sync::Arc::new(crate::region::candidate_region(|| {
                std::sync::Arc::unwrap_or_clone(self.by).lower()
            })),
        }
    }
}

impl Lower for deckmaste_semantics::Count {
    type Target = deckmaste_core::Count;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::X => deckmaste_core::Count::Reg(
                crate::region::x().expect("X count outside a region with an announced-X parameter"),
            ),
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
            Self::ThatMany | Self::ThatMuch => deckmaste_core::Count::Reg(
                crate::region::amount().expect("unbound amount anaphor during lowering"),
            ),
            Self::Allotment => deckmaste_core::Count::Reg(
                crate::region::allotment().expect("unbound distribution allotment during lowering"),
            ),
            Self::EventCount(f0, f1) => deckmaste_core::Count::EventCount(f0.lower(), f1.lower()),
            Self::EventSum(f0, f1) => deckmaste_core::Count::EventSum(f0.lower(), f1.lower()),
            // A region-local definition wins; then the card's declared linked
            // memory parameter ([CR#607.1], ADR law 8). There is no
            // resolution-local name-keyed fallback.
            Self::Noted(name) => crate::region::named(&name)
                .or_else(|| crate::region::cell_read(&name))
                .map_or_else(
                    || {
                        crate::region::refuse(&format!(
                            "noted number `{name}` has no region definition or declared linked cell"
                        ));
                        deckmaste_core::Count::Reg(deckmaste_core::RefId(0))
                    },
                    deckmaste_core::Count::Reg,
                ),
            Self::TimesPaid(f0) => deckmaste_core::Count::TimesPaid(f0.lower()),
            Self::Damage(f0) => deckmaste_core::Count::Damage(f0.lower()),
            Self::ManaAvailable(f0) => deckmaste_core::Count::ManaAvailable(f0.lower()),
            Self::ManaAvailableKind(f0, f1) => {
                deckmaste_core::Count::ManaAvailableKind(f0.lower(), f1.lower())
            }
            Self::Aggregate(f0, f1) => deckmaste_core::Count::Aggregate(f0.lower(), f1.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
            Self::Literal(f0) => deckmaste_core::Count::Literal(f0.lower()),
        }
    }
}
