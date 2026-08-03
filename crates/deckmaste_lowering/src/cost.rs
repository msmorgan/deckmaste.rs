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
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_cost_component_mana() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::Mana(
            deckmaste_authoring::ManaCost::from(
                std::sync::Arc::<[deckmaste_authoring::ManaSymbol]>::from([]),
            ),
        ));
        assert_matches!(
            deckmaste_authoring::CostComponent::Mana(deckmaste_authoring::ManaCost::from(
                std::sync::Arc::<[deckmaste_authoring::ManaSymbol]>::from([])
            ))
            .lower(),
            deckmaste_core::CostComponent::Mana(..)
        );
    }

    #[test]
    fn lowers_cost_component_mana_cost_of() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::ManaCostOf(
            minimal_reference(),
        ));
        assert_matches!(
            deckmaste_authoring::CostComponent::ManaCostOf(minimal_reference()).lower(),
            deckmaste_core::CostComponent::ManaCostOf(..)
        );
    }

    #[test]
    fn lowers_cost_component_tap() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::Tap);
        assert_matches!(
            deckmaste_authoring::CostComponent::Tap.lower(),
            deckmaste_core::CostComponent::Tap
        );
    }

    #[test]
    fn lowers_cost_component_untap() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::Untap);
        assert_matches!(
            deckmaste_authoring::CostComponent::Untap.lower(),
            deckmaste_core::CostComponent::Untap
        );
    }

    #[test]
    fn lowers_cost_component_do() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::Do(std::sync::Arc::new(
            minimal_action(),
        )));
        assert_matches!(
            deckmaste_authoring::CostComponent::Do(std::sync::Arc::new(minimal_action())).lower(),
            deckmaste_core::CostComponent::Do(..)
        );
    }

    #[test]
    fn lowers_cost_component_cost() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::Cost(minimal_cost()));
        assert_matches!(
            deckmaste_authoring::CostComponent::Cost(minimal_cost()).lower(),
            deckmaste_core::CostComponent::Cost(..)
        );
    }

    #[test]
    fn lowers_cost_component_tap_total() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::TapTotal {
            stat: minimal_stat(),
            cmp: minimal_cmp(),
            count: minimal_count(),
            filter: std::sync::Arc::new(minimal_predicate()),
        });
        assert_matches!(
            deckmaste_authoring::CostComponent::TapTotal {
                stat: minimal_stat(),
                cmp: minimal_cmp(),
                count: minimal_count(),
                filter: std::sync::Arc::new(minimal_predicate())
            }
            .lower(),
            deckmaste_core::CostComponent::TapTotal { .. }
        );
    }

    #[test]
    fn lowers_cost_component_with() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::With {
            binder: std::sync::Arc::new(minimal_binder()),
            body: minimal_cost(),
        });
        assert_matches!(
            deckmaste_authoring::CostComponent::With {
                binder: std::sync::Arc::new(minimal_binder()),
                body: minimal_cost()
            }
            .lower(),
            deckmaste_core::CostComponent::With { .. }
        );
    }

    #[test]
    fn lowers_cost_component_expanded() {
        assert_lowers_debug(deckmaste_authoring::CostComponent::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_cost_component()),
            },
        ));
        assert_matches!(
            deckmaste_authoring::CostComponent::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_cost_component())
            })
            .lower(),
            deckmaste_core::CostComponent::Expanded(..)
        );
    }

    #[test]
    fn lowers_cost() {
        assert_lowers(deckmaste_authoring::Cost([].into()));
    }

    #[test]
    fn lowers_cost_tag() {
        assert_lowers(deckmaste_authoring::CostTag("X".into()));
    }

    #[test]
    fn lowers_optional_cost() {
        assert_lowers(deckmaste_authoring::OptionalCost {
            components: [].into(),
            tag: minimal_cost_tag(),
            repeatable: false,
        });
    }

    #[test]
    fn lowers_total_cost() {
        assert_lowers_debug(deckmaste_authoring::TotalCost {
            base: [].into(),
            trace: [].into(),
            locked: false,
        });
    }
}
