//! `deontic` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::AlternativeCost {
    type Target = deckmaste_core::AlternativeCost;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Free => deckmaste_core::AlternativeCost::Free,
            Self::Components(f0) => deckmaste_core::AlternativeCost::Components(
                crate::cost::lower_scoped_cost_components(&f0),
            ),
        }
    }
}

impl Lower for deckmaste_semantics::CostPredicate {
    type Target = deckmaste_core::CostPredicate;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::IncludesTapSymbol => deckmaste_core::CostPredicate::IncludesTapSymbol,
        }
    }
}

impl Lower for deckmaste_semantics::AsThough {
    type Target = deckmaste_core::AsThough;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Counterfactual { premise, then } => deckmaste_core::AsThough::Counterfactual {
                premise: premise.lower(),
                then: then.lower(),
            },
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::CountBound {
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

impl Lower for deckmaste_semantics::DeedAgent {
    type Target = deckmaste_core::DeedAgent;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::DeedAgent {
            stack_object: self.stack_object.lower(),
            source: self.source.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::DeonticAction {
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
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Deontic {
    type Target = deckmaste_core::Deontic;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::May(f0) => deckmaste_core::Deontic::May(f0.lower()),
            Self::Cant(f0) => deckmaste_core::Deontic::Cant(f0.lower()),
            Self::Must(f0) => deckmaste_core::Deontic::Must(f0.lower()),
            Self::Gate(f0, f1) => deckmaste_core::Deontic::Gate(
                f0.lower(),
                crate::cost::lower_scoped_cost_components(&f1),
            ),
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
    fn lowers_alternative_cost_free() {
        assert_matches!(
            deckmaste_semantics::AlternativeCost::Free.lower(),
            deckmaste_core::AlternativeCost::Free
        );
    }

    #[test]
    fn lowers_alternative_cost_components() {
        assert_matches!(
            deckmaste_semantics::AlternativeCost::Components([].into()).lower(),
            deckmaste_core::AlternativeCost::Components(_)
        );
    }

    #[test]
    fn lowers_cost_predicate_includes_tap_symbol() {
        assert_matches!(
            deckmaste_semantics::CostPredicate::IncludesTapSymbol.lower(),
            deckmaste_core::CostPredicate::IncludesTapSymbol
        );
    }

    #[test]
    fn lowers_as_though_counterfactual() {
        assert_matches!(
            deckmaste_semantics::AsThough::Counterfactual {
                premise: minimal_predicate(),
                then: std::sync::Arc::new(minimal_deontic())
            }
            .lower(),
            deckmaste_core::AsThough::Counterfactual {
                premise: deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                ),
                then: _
            }
        );
    }

    #[test]
    fn lowers_as_though_expanded() {
        assert_matches!(
            deckmaste_semantics::AsThough::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_as_though())
            })
            .lower(),
            deckmaste_core::AsThough::Counterfactual {
                premise: deckmaste_core::Predicate::Class(
                    deckmaste_core::ObjectClass::AbilityOnStack
                ),
                then: _
            }
        );
    }

    #[test]
    fn lowers_count_bound_eq() {
        assert_matches!(
            deckmaste_semantics::CountBound::Eq(minimal_count()).lower(),
            deckmaste_core::CountBound::Eq(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_at_least() {
        assert_matches!(
            deckmaste_semantics::CountBound::AtLeast(minimal_count()).lower(),
            deckmaste_core::CountBound::AtLeast(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_at_most() {
        assert_matches!(
            deckmaste_semantics::CountBound::AtMost(minimal_count()).lower(),
            deckmaste_core::CountBound::AtMost(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_greater() {
        assert_matches!(
            deckmaste_semantics::CountBound::Greater(minimal_count()).lower(),
            deckmaste_core::CountBound::Greater(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_count_bound_less() {
        assert_matches!(
            deckmaste_semantics::CountBound::Less(minimal_count()).lower(),
            deckmaste_core::CountBound::Less(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_deed_agent() {
        assert_matches!(
            deckmaste_semantics::DeedAgent {
                stack_object: None,
                source: None
            }
            .lower(),
            deckmaste_core::DeedAgent {
                stack_object: None,
                source: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_attack() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Attack {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_block() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Block {
                by: minimal_predicate(),
                on: minimal_predicate(),
                count: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Block {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                count: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_target() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Target {
                by: minimal_deed_agent(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Target {
                by: deckmaste_core::DeedAgent {
                    stack_object: None,
                    source: None
                },
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_attach() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Attach {
                what: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Attach {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                to: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_cast() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Cast {
                what: minimal_predicate(),
                by: minimal_predicate(),
                from: None,
                window: None,
                cost: None,
                tag: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Cast {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None,
                window: None,
                cost: None,
                tag: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_play() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Play {
                what: minimal_predicate(),
                by: minimal_predicate(),
                from: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Play {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                from: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_activate() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Activate {
                what: minimal_predicate(),
                by: minimal_predicate(),
                cost: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Activate {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                cost: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_regenerate() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Regenerate {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Regenerate {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_counter() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Counter {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Counter {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_untap() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Untap {
                what: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Untap {
                what: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_expanded() {
        assert_matches!(
            deckmaste_semantics::DeonticAction::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic_action())
            })
            .lower(),
            deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            }
        );
    }

    #[test]
    fn lowers_deontic_may() {
        assert_matches!(
            deckmaste_semantics::Deontic::May(minimal_deontic_action()).lower(),
            deckmaste_core::Deontic::May(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            })
        );
    }

    #[test]
    fn lowers_deontic_cant() {
        assert_matches!(
            deckmaste_semantics::Deontic::Cant(minimal_deontic_action()).lower(),
            deckmaste_core::Deontic::Cant(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            })
        );
    }

    #[test]
    fn lowers_deontic_must() {
        assert_matches!(
            deckmaste_semantics::Deontic::Must(minimal_deontic_action()).lower(),
            deckmaste_core::Deontic::Must(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            })
        );
    }

    #[test]
    fn lowers_deontic_gate() {
        assert_matches!(
            deckmaste_semantics::Deontic::Gate(minimal_deontic_action(), [].into()).lower(),
            deckmaste_core::Deontic::Gate(
                deckmaste_core::DeonticAction::Attack {
                    by: deckmaste_core::Predicate::Class(
                        deckmaste_core::ObjectClass::AbilityOnStack
                    ),
                    on: deckmaste_core::Predicate::Class(
                        deckmaste_core::ObjectClass::AbilityOnStack
                    )
                },
                _
            )
        );
    }

    #[test]
    fn lowers_deontic_expanded() {
        assert_matches!(
            deckmaste_semantics::Deontic::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic())
            })
            .lower(),
            deckmaste_core::Deontic::May(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack),
                on: deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
            })
        );
    }
}
