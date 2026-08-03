//! `deontic` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

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
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the authored
            // spelling (spec §12). Prose recovers the authored term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
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
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the authored
            // spelling (spec §12). Prose recovers the authored term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
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
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the authored
            // spelling (spec §12). Prose recovers the authored term through the
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
            deckmaste_authoring::AlternativeCost::Free.lower(),
            deckmaste_core::AlternativeCost::Free
        );
    }

    #[test]
    fn lowers_alternative_cost_components() {
        assert_matches!(
            deckmaste_authoring::AlternativeCost::Components([].into()).lower(),
            deckmaste_core::AlternativeCost::Components(_)
        );
    }

    #[test]
    fn lowers_cost_predicate_includes_tap_symbol() {
        assert_matches!(
            deckmaste_authoring::CostPredicate::IncludesTapSymbol.lower(),
            deckmaste_core::CostPredicate::IncludesTapSymbol
        );
    }

    #[test]
    fn lowers_as_though_counterfactual() {
        assert_matches!(
            deckmaste_authoring::AsThough::Counterfactual {
                premise: minimal_predicate(),
                then: std::sync::Arc::new(minimal_deontic())
            }
            .lower(),
            deckmaste_core::AsThough::Counterfactual {
                premise: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                then: _
            }
        );
    }

    #[test]
    fn lowers_as_though_expanded() {
        assert_matches!(
            deckmaste_authoring::AsThough::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_as_though())
            })
            .lower(),
            deckmaste_core::AsThough::Counterfactual {
                premise: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                then: _
            }
        );
    }

    #[test]
    fn lowers_count_bound_eq() {
        assert_matches!(
            deckmaste_authoring::CountBound::Eq(minimal_count()).lower(),
            deckmaste_core::CountBound::Eq(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_count_bound_at_least() {
        assert_matches!(
            deckmaste_authoring::CountBound::AtLeast(minimal_count()).lower(),
            deckmaste_core::CountBound::AtLeast(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_count_bound_at_most() {
        assert_matches!(
            deckmaste_authoring::CountBound::AtMost(minimal_count()).lower(),
            deckmaste_core::CountBound::AtMost(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_count_bound_greater() {
        assert_matches!(
            deckmaste_authoring::CountBound::Greater(minimal_count()).lower(),
            deckmaste_core::CountBound::Greater(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_count_bound_less() {
        assert_matches!(
            deckmaste_authoring::CountBound::Less(minimal_count()).lower(),
            deckmaste_core::CountBound::Less(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_deed_agent() {
        assert_matches!(
            deckmaste_authoring::DeedAgent {
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
            deckmaste_authoring::DeonticAction::Attack {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_block() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Block {
                by: minimal_predicate(),
                on: minimal_predicate(),
                count: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Block {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                count: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_target() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Target {
                by: minimal_deed_agent(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Target {
                by: deckmaste_core::DeedAgent {
                    stack_object: None,
                    source: None
                },
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_attach() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Attach {
                what: minimal_predicate(),
                to: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Attach {
                what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                to: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_cast() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Cast {
                what: minimal_predicate(),
                by: minimal_predicate(),
                from: None,
                window: None,
                cost: None,
                tag: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Cast {
                what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
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
            deckmaste_authoring::DeonticAction::Play {
                what: minimal_predicate(),
                by: minimal_predicate(),
                from: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Play {
                what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                from: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_activate() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Activate {
                what: minimal_predicate(),
                by: minimal_predicate(),
                cost: None
            }
            .lower(),
            deckmaste_core::DeonticAction::Activate {
                what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                cost: None
            }
        );
    }

    #[test]
    fn lowers_deontic_action_regenerate() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Regenerate {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Regenerate {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_counter() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Counter {
                by: minimal_predicate(),
                on: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Counter {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_untap() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Untap {
                what: minimal_predicate()
            }
            .lower(),
            deckmaste_core::DeonticAction::Untap {
                what: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_deontic_action_expanded() {
        assert_matches!(
            deckmaste_authoring::DeonticAction::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic_action())
            })
            .lower(),
            deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            }
        );
    }

    #[test]
    fn lowers_deontic_may() {
        assert_matches!(
            deckmaste_authoring::Deontic::May(minimal_deontic_action()).lower(),
            deckmaste_core::Deontic::May(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            })
        );
    }

    #[test]
    fn lowers_deontic_cant() {
        assert_matches!(
            deckmaste_authoring::Deontic::Cant(minimal_deontic_action()).lower(),
            deckmaste_core::Deontic::Cant(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            })
        );
    }

    #[test]
    fn lowers_deontic_must() {
        assert_matches!(
            deckmaste_authoring::Deontic::Must(minimal_deontic_action()).lower(),
            deckmaste_core::Deontic::Must(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            })
        );
    }

    #[test]
    fn lowers_deontic_gate() {
        assert_matches!(
            deckmaste_authoring::Deontic::Gate(minimal_deontic_action(), [].into()).lower(),
            deckmaste_core::Deontic::Gate(
                deckmaste_core::DeonticAction::Attack {
                    by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                    on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
                },
                _
            )
        );
    }

    #[test]
    fn lowers_deontic_expanded() {
        assert_matches!(
            deckmaste_authoring::Deontic::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic())
            })
            .lower(),
            deckmaste_core::Deontic::May(deckmaste_core::DeonticAction::Attack {
                by: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                on: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            })
        );
    }
}
