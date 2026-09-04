//! `condition` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Cmp {
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

impl Lower for deckmaste_semantics::Condition {
    type Target = deckmaste_core::Condition;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Compare(f0, f1, f2) => {
                deckmaste_core::Condition::Compare(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Exists(f0) => deckmaste_core::Condition::Exists(f0.lower()),
            // [CR#120.1,120.3,702.2c]: `Matches(Source, F)` is not a read of
            // one referent — it asks whether the object under evaluation was
            // dealt damage by a source matching F, existentially over its
            // marks' deal-time abilities. Core names that relation, so the
            // whole condition lowers as a unit onto the same `DealtDamageBy`
            // the explicit spelling below produces.
            Self::Matches(deckmaste_semantics::Reference::Source, filter) => {
                deckmaste_core::Condition::DealtDamageBy(
                    deckmaste_core::Reference::Reg(
                        crate::region::source().unwrap_or(deckmaste_core::RefId(0)),
                    ),
                    filter.lower(),
                )
            }
            Self::Matches(reference, filter) => {
                deckmaste_core::Condition::Matches(reference.lower(), filter.lower())
            }
            Self::DealtDamageBy(reference, filter) => {
                deckmaste_core::Condition::DealtDamageBy(reference.lower(), filter.lower())
            }
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
    fn lowers_cmp_eq() {
        assert_matches!(
            deckmaste_semantics::Cmp::Eq.lower(),
            deckmaste_core::Cmp::Eq
        );
    }

    #[test]
    fn lowers_cmp_at_least() {
        assert_matches!(
            deckmaste_semantics::Cmp::AtLeast.lower(),
            deckmaste_core::Cmp::AtLeast
        );
    }

    #[test]
    fn lowers_cmp_at_most() {
        assert_matches!(
            deckmaste_semantics::Cmp::AtMost.lower(),
            deckmaste_core::Cmp::AtMost
        );
    }

    #[test]
    fn lowers_cmp_greater() {
        assert_matches!(
            deckmaste_semantics::Cmp::Greater.lower(),
            deckmaste_core::Cmp::Greater
        );
    }

    #[test]
    fn lowers_cmp_less() {
        assert_matches!(
            deckmaste_semantics::Cmp::Less.lower(),
            deckmaste_core::Cmp::Less
        );
    }

    #[test]
    fn lowers_condition_compare() {
        assert_matches!(
            deckmaste_semantics::Condition::Compare(
                minimal_count(),
                minimal_cmp(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Condition::Compare(
                deckmaste_core::Count::Literal(0),
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_condition_exists() {
        assert_matches!(
            deckmaste_semantics::Condition::Exists(minimal_predicate()).lower(),
            deckmaste_core::Condition::Exists(deckmaste_core::Predicate::Class(
                deckmaste_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_condition_matches() {
        assert_matches!(
            deckmaste_semantics::Condition::Matches(minimal_reference(), minimal_predicate())
                .lower(),
            deckmaste_core::Condition::Matches(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            )
        );
    }

    #[test]
    fn lowers_condition_dealt_damage_by() {
        assert_eq!(
            deckmaste_semantics::Condition::DealtDamageBy(
                deckmaste_semantics::Reference::This,
                minimal_predicate(),
            )
            .lower(),
            deckmaste_core::Condition::DealtDamageBy(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
            )
        );
    }

    #[test]
    fn lowers_legacy_matches_source_as_dealt_damage_by() {
        assert_eq!(
            deckmaste_semantics::Condition::Matches(
                deckmaste_semantics::Reference::Source,
                minimal_predicate(),
            )
            .lower(),
            deckmaste_core::Condition::DealtDamageBy(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
            )
        );
    }

    #[test]
    fn lowers_condition_legally_attached() {
        assert_matches!(
            deckmaste_semantics::Condition::LegallyAttached(minimal_reference()).lower(),
            deckmaste_core::Condition::LegallyAttached(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_condition_happened() {
        assert_matches!(
            deckmaste_semantics::Condition::Happened {
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
            deckmaste_semantics::Condition::Crossed {
                value: minimal_count(),
                thresholds: [].into()
            }
            .lower(),
            deckmaste_core::Condition::Crossed {
                value: deckmaste_core::Count::Literal(0),
                thresholds: _
            }
        );
    }

    #[test]
    fn lowers_condition_paid_cost() {
        assert_matches!(
            deckmaste_semantics::Condition::PaidCost(minimal_cost_tag()).lower(),
            deckmaste_core::Condition::PaidCost(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_condition_cast_with() {
        assert_matches!(
            deckmaste_semantics::Condition::CastWith(minimal_cost_tag()).lower(),
            deckmaste_core::Condition::CastWith(deckmaste_core::CostTag(_))
        );
    }

    #[test]
    fn lowers_condition_your_turn() {
        assert_matches!(
            deckmaste_semantics::Condition::YourTurn.lower(),
            deckmaste_core::Condition::YourTurn
        );
    }

    #[test]
    fn lowers_condition_turn_of() {
        assert_matches!(
            deckmaste_semantics::Condition::TurnOf(minimal_predicate()).lower(),
            deckmaste_core::Condition::TurnOf(deckmaste_core::Predicate::Class(
                deckmaste_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_condition_during_phase() {
        assert_matches!(
            deckmaste_semantics::Condition::DuringPhase(minimal_phase_step()).lower(),
            deckmaste_core::Condition::DuringPhase(deckmaste_core::PhaseStep::Beginning(
                deckmaste_core::BeginningStep::Untap
            ))
        );
    }

    #[test]
    fn lowers_condition_and() {
        assert_matches!(
            deckmaste_semantics::Condition::And([].into()).lower(),
            deckmaste_core::Condition::And(_)
        );
    }

    #[test]
    fn lowers_condition_or() {
        assert_matches!(
            deckmaste_semantics::Condition::Or([].into()).lower(),
            deckmaste_core::Condition::Or(_)
        );
    }

    #[test]
    fn lowers_condition_not() {
        assert_matches!(
            deckmaste_semantics::Condition::Not(std::sync::Arc::new(minimal_condition())).lower(),
            deckmaste_core::Condition::Not(_)
        );
    }

    #[test]
    fn lowers_condition_expanded() {
        assert_matches!(
            deckmaste_semantics::Condition::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_condition())
            })
            .lower(),
            deckmaste_core::Condition::Compare(
                deckmaste_core::Count::Literal(0),
                deckmaste_core::Cmp::Eq,
                deckmaste_core::Count::Literal(0)
            )
        );
    }
}
