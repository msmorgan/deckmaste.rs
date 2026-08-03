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
    fn lowers_cost_component_mana() {
        assert_matches!(
            deckmaste_authoring::CostComponent::Mana(deckmaste_authoring::ManaCost::from(
                std::sync::Arc::<[deckmaste_authoring::ManaSymbol]>::from([])
            ))
            .lower(),
            deckmaste_core::CostComponent::Mana(_)
        );
    }

    #[test]
    fn lowers_cost_component_mana_cost_of() {
        assert_matches!(
            deckmaste_authoring::CostComponent::ManaCostOf(minimal_reference()).lower(),
            deckmaste_core::CostComponent::ManaCostOf(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_cost_component_tap() {
        assert_matches!(
            deckmaste_authoring::CostComponent::Tap.lower(),
            deckmaste_core::CostComponent::Tap
        );
    }

    #[test]
    fn lowers_cost_component_untap() {
        assert_matches!(
            deckmaste_authoring::CostComponent::Untap.lower(),
            deckmaste_core::CostComponent::Untap
        );
    }

    #[test]
    fn lowers_cost_component_do() {
        assert_matches!(
            deckmaste_authoring::CostComponent::Do(std::sync::Arc::new(minimal_action())).lower(),
            deckmaste_core::CostComponent::Do(_)
        );
    }

    #[test]
    fn lowers_cost_component_cost() {
        assert_matches!(
            deckmaste_authoring::CostComponent::Cost(minimal_cost()).lower(),
            deckmaste_core::CostComponent::Cost(deckmaste_core::Cost(_))
        );
    }

    #[test]
    fn lowers_cost_component_tap_total() {
        assert_matches!(
            deckmaste_authoring::CostComponent::TapTotal {
                stat: minimal_stat(),
                cmp: minimal_cmp(),
                count: minimal_count(),
                filter: std::sync::Arc::new(minimal_predicate())
            }
            .lower(),
            deckmaste_core::CostComponent::TapTotal {
                stat: deckmaste_core::Stat::Power,
                cmp: deckmaste_core::Cmp::Eq,
                count: deckmaste_core::Count::X,
                filter: _
            }
        );
    }

    #[test]
    fn lowers_cost_component_with() {
        assert_matches!(
            deckmaste_authoring::CostComponent::With {
                binder: std::sync::Arc::new(minimal_binder()),
                body: minimal_cost()
            }
            .lower(),
            deckmaste_core::CostComponent::With {
                binder: _,
                body: deckmaste_core::Cost(_)
            }
        );
    }

    #[test]
    fn lowers_cost_component_expanded() {
        assert_matches!(
            deckmaste_authoring::CostComponent::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_cost_component())
            })
            .lower(),
            deckmaste_core::CostComponent::Expanded(_)
        );
    }

    #[test]
    fn lowers_cost() {
        assert_matches!(
            deckmaste_authoring::Cost([].into()).lower(),
            deckmaste_core::Cost(_)
        );
    }

    #[test]
    fn lowers_cost_tag() {
        assert_matches!(
            deckmaste_authoring::CostTag("X".into()).lower(),
            deckmaste_core::CostTag(_)
        );
    }

    #[test]
    fn lowers_optional_cost() {
        assert_matches!(
            deckmaste_authoring::OptionalCost {
                components: [].into(),
                tag: minimal_cost_tag(),
                repeatable: false
            }
            .lower(),
            deckmaste_core::OptionalCost {
                components: _,
                tag: deckmaste_core::CostTag(_),
                repeatable: false
            }
        );
    }

    #[test]
    fn lowers_total_cost() {
        assert_matches!(
            deckmaste_authoring::TotalCost {
                base: [].into(),
                trace: [].into(),
                locked: false
            }
            .lower(),
            deckmaste_core::TotalCost {
                base: _,
                trace: _,
                locked: false
            }
        );
    }
}
