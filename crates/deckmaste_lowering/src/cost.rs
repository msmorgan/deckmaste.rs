//! `cost` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::CostComponent {
    type Target = deckmaste_core::CostComponent;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Mana(f0) => deckmaste_core::CostComponent::Mana(f0.lower()),
            Self::ManaCostOf(f0) => deckmaste_core::CostComponent::ManaCostOf(f0.lower()),
            Self::Tap => deckmaste_core::CostComponent::Tap,
            Self::Untap => deckmaste_core::CostComponent::Untap,
            Self::Do(f0) => deckmaste_core::CostComponent::Do(f0.lower()),
            Self::Cost(f0) => deckmaste_core::CostComponent::Cost(f0.lower()),
            Self::TapTotal {
                stat,
                cmp,
                count,
                filter,
            } => deckmaste_core::CostComponent::TapTotal {
                stat: stat.lower(),
                cmp: cmp.lower(),
                count: count.lower(),
                filter: filter.lower(),
            },
            Self::With { binder, body } => deckmaste_core::CostComponent::With {
                binder: binder.lower(),
                body: body.lower(),
            },
            Self::Expanded(f0) => deckmaste_core::CostComponent::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Cost {
    type Target = deckmaste_core::Cost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Cost(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::CostTag {
    type Target = deckmaste_core::CostTag;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CostTag(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::OptionalCost {
    type Target = deckmaste_core::OptionalCost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::OptionalCost {
            components: self.components.lower(),
            tag: self.tag.lower(),
            repeatable: self.repeatable.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::TotalCost {
    type Target = deckmaste_core::TotalCost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TotalCost {
            base: self.base.lower(),
            trace: self.trace.lower(),
            locked: self.locked.lower(),
        }
    }
}
