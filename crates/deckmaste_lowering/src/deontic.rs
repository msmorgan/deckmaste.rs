//! `deontic` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::AlternativeCost {
    type Target = deckmaste_core::AlternativeCost;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Free => deckmaste_core::AlternativeCost::Free,
            Self::Components(f0) => deckmaste_core::AlternativeCost::Components(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::CostPredicate {
    type Target = deckmaste_core::CostPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::IncludesTapSymbol => deckmaste_core::CostPredicate::IncludesTapSymbol,
        }
    }
}

impl Lower for deckmaste_authoring::AsThough {
    type Target = deckmaste_core::AsThough;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Counterfactual { premise, then } => deckmaste_core::AsThough::Counterfactual {
                premise: premise.lower(),
                then: then.lower(),
            },
            Self::Expanded(f0) => deckmaste_core::AsThough::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::CountBound {
    type Target = deckmaste_core::CountBound;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Eq(f0) => deckmaste_core::CountBound::Eq(f0.lower()),
            Self::AtLeast(f0) => deckmaste_core::CountBound::AtLeast(f0.lower()),
            Self::AtMost(f0) => deckmaste_core::CountBound::AtMost(f0.lower()),
            Self::Greater(f0) => deckmaste_core::CountBound::Greater(f0.lower()),
            Self::Less(f0) => deckmaste_core::CountBound::Less(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::DeedAgent {
    type Target = deckmaste_core::DeedAgent;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::DeedAgent {
            stack_object: self.stack_object.lower(),
            source: self.source.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::DeonticAction {
    type Target = deckmaste_core::DeonticAction;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Attack { by, on } => deckmaste_core::DeonticAction::Attack {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Block { by, on, count } => deckmaste_core::DeonticAction::Block {
                by: by.lower(),
                on: on.lower(),
                count: count.lower(),
            },
            Self::Target { by, on } => deckmaste_core::DeonticAction::Target {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Attach { what, to } => deckmaste_core::DeonticAction::Attach {
                what: what.lower(),
                to: to.lower(),
            },
            Self::Cast {
                what,
                by,
                from,
                window,
                cost,
                tag,
            } => deckmaste_core::DeonticAction::Cast {
                what: what.lower(),
                by: by.lower(),
                from: from.lower(),
                window: window.lower(),
                cost: cost.lower(),
                tag: tag.lower(),
            },
            Self::Play { what, by, from } => deckmaste_core::DeonticAction::Play {
                what: what.lower(),
                by: by.lower(),
                from: from.lower(),
            },
            Self::Activate { what, by, cost } => deckmaste_core::DeonticAction::Activate {
                what: what.lower(),
                by: by.lower(),
                cost: cost.lower(),
            },
            Self::Regenerate { by, on } => deckmaste_core::DeonticAction::Regenerate {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Counter { by, on } => deckmaste_core::DeonticAction::Counter {
                by: by.lower(),
                on: on.lower(),
            },
            Self::Untap { what } => deckmaste_core::DeonticAction::Untap { what: what.lower() },
            Self::Expanded(f0) => deckmaste_core::DeonticAction::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Deontic {
    type Target = deckmaste_core::Deontic;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::May(f0) => deckmaste_core::Deontic::May(f0.lower()),
            Self::Cant(f0) => deckmaste_core::Deontic::Cant(f0.lower()),
            Self::Must(f0) => deckmaste_core::Deontic::Must(f0.lower()),
            Self::Gate(f0, f1) => deckmaste_core::Deontic::Gate(f0.lower(), f1.lower()),
            Self::Expanded(f0) => deckmaste_core::Deontic::Expanded(f0.lower()),
        }
    }
}
