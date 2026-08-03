//! `condition` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::Cmp {
    type Target = deckmaste_core::Cmp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Eq => deckmaste_core::Cmp::Eq,
            Self::AtLeast => deckmaste_core::Cmp::AtLeast,
            Self::AtMost => deckmaste_core::Cmp::AtMost,
            Self::Greater => deckmaste_core::Cmp::Greater,
            Self::Less => deckmaste_core::Cmp::Less,
        }
    }
}

impl Lower for deckmaste_authoring::Condition {
    type Target = deckmaste_core::Condition;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Compare(f0, f1, f2) => {
                deckmaste_core::Condition::Compare(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Exists(f0) => deckmaste_core::Condition::Exists(f0.lower()),
            Self::Matches(f0, f1) => deckmaste_core::Condition::Matches(f0.lower(), f1.lower()),
            Self::LegallyAttached(f0) => deckmaste_core::Condition::LegallyAttached(f0.lower()),
            Self::Happened { event, within } => deckmaste_core::Condition::Happened {
                event: event.lower(),
                within: within.lower(),
            },
            Self::Crossed { value, thresholds } => deckmaste_core::Condition::Crossed {
                value: value.lower(),
                thresholds: thresholds.lower(),
            },
            Self::PaidCost(f0) => deckmaste_core::Condition::PaidCost(f0.lower()),
            Self::CastWith(f0) => deckmaste_core::Condition::CastWith(f0.lower()),
            Self::YourTurn => deckmaste_core::Condition::YourTurn,
            Self::TurnOf(f0) => deckmaste_core::Condition::TurnOf(f0.lower()),
            Self::DuringPhase(f0) => deckmaste_core::Condition::DuringPhase(f0.lower()),
            Self::And(f0) => deckmaste_core::Condition::And(f0.lower()),
            Self::Or(f0) => deckmaste_core::Condition::Or(f0.lower()),
            Self::Not(f0) => deckmaste_core::Condition::Not(f0.lower()),
            Self::Expanded(f0) => deckmaste_core::Condition::Expanded(f0.lower()),
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
    fn lowers_cmp_eq() {
        assert_matches!(
            deckmaste_authoring::Cmp::Eq.lower(),
            deckmaste_core::Cmp::Eq
        );
    }

    #[test]
    fn lowers_cmp_at_least() {
        assert_matches!(
            deckmaste_authoring::Cmp::AtLeast.lower(),
            deckmaste_core::Cmp::AtLeast
        );
    }

    #[test]
    fn lowers_cmp_at_most() {
        assert_matches!(
            deckmaste_authoring::Cmp::AtMost.lower(),
            deckmaste_core::Cmp::AtMost
        );
    }

    #[test]
    fn lowers_cmp_greater() {
        assert_matches!(
            deckmaste_authoring::Cmp::Greater.lower(),
            deckmaste_core::Cmp::Greater
        );
    }

    #[test]
    fn lowers_cmp_less() {
        assert_matches!(
            deckmaste_authoring::Cmp::Less.lower(),
            deckmaste_core::Cmp::Less
        );
    }

    #[test]
    fn lowers_condition_compare() {
        assert_matches!(
            deckmaste_authoring::Condition::Compare(
                minimal_count(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Condition::Compare(
                deckmaste_core::Count::X,
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::X
            )
        );
    }

    #[test]
    fn lowers_condition_exists() {
        assert_matches!(
            deckmaste_authoring::Condition::Exists(minimal_predicate()).lower(),
            deckmaste_core::Condition::Exists(deckmaste_core::Predicate::Kind(
                deckmaste_core::ObjectKind::Ability
            ))
        );
    }

    #[test]
    fn lowers_condition_matches() {
        assert_matches!(
            deckmaste_authoring::Condition::Matches(minimal_reference(), minimal_predicate())
                .lower(),
            deckmaste_core::Condition::Matches(
                deckmaste_core::Reference::This,
                deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            )
        );
    }

    #[test]
    fn lowers_condition_legally_attached() {
        assert_matches!(
            deckmaste_authoring::Condition::LegallyAttached(minimal_reference()).lower(),
            deckmaste_core::Condition::LegallyAttached(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_condition_happened() {
        assert_matches!(
            deckmaste_authoring::Condition::Happened {
                event: std::sync::Arc::new(minimal_event_filter()),
                within: minimal_lookback()
            }
            .lower(),
            deckmaste_core::Condition::Happened {
                event: _,
                within: deckmaste_core::Lookback::ThisTurn
            }
        );
    }

    #[test]
    fn lowers_condition_crossed() {
        assert_matches!(
            deckmaste_authoring::Condition::Crossed {
                value: minimal_count(),
                thresholds: [].into()
            }
            .lower(),
            deckmaste_core::Condition::Crossed {
                value: deckmaste_core::Count::X,
                thresholds: _
            }
        );
    }

    #[test]
    fn lowers_condition_paid_cost() {
        assert_matches!(
            deckmaste_authoring::Condition::PaidCost(minimal_cost_tag()).lower(),
            deckmaste_core::Condition::PaidCost(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_condition_cast_with() {
        assert_matches!(
            deckmaste_authoring::Condition::CastWith(minimal_cost_tag()).lower(),
            deckmaste_core::Condition::CastWith(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_condition_your_turn() {
        assert_matches!(
            deckmaste_authoring::Condition::YourTurn.lower(),
            deckmaste_core::Condition::YourTurn
        );
    }

    #[test]
    fn lowers_condition_turn_of() {
        assert_matches!(
            deckmaste_authoring::Condition::TurnOf(minimal_predicate()).lower(),
            deckmaste_core::Condition::TurnOf(deckmaste_core::Predicate::Kind(
                deckmaste_core::ObjectKind::Ability
            ))
        );
    }

    #[test]
    fn lowers_condition_during_phase() {
        assert_matches!(
            deckmaste_authoring::Condition::DuringPhase(minimal_phase_step()).lower(),
            deckmaste_core::Condition::DuringPhase(deckmaste_core::PhaseStep::Beginning(
                deckmaste_core::BeginningStep::Untap
            ))
        );
    }

    #[test]
    fn lowers_condition_and() {
        assert_matches!(
            deckmaste_authoring::Condition::And([].into()).lower(),
            deckmaste_core::Condition::And(_)
        );
    }

    #[test]
    fn lowers_condition_or() {
        assert_matches!(
            deckmaste_authoring::Condition::Or([].into()).lower(),
            deckmaste_core::Condition::Or(_)
        );
    }

    #[test]
    fn lowers_condition_not() {
        assert_matches!(
            deckmaste_authoring::Condition::Not(std::sync::Arc::new(minimal_condition())).lower(),
            deckmaste_core::Condition::Not(_)
        );
    }

    #[test]
    fn lowers_condition_expanded() {
        assert_matches!(
            deckmaste_authoring::Condition::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_condition())
            })
            .lower(),
            deckmaste_core::Condition::Expanded(_)
        );
    }
}
