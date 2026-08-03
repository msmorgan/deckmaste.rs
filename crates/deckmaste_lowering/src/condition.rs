//! `condition` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

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
