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

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_alternative_cost_free() {
        assert_lowers(deckmaste_authoring::AlternativeCost::Free);
    }

    #[test]
    fn lowers_alternative_cost_components() {
        assert_lowers(deckmaste_authoring::AlternativeCost::Components([].into()));
    }

    #[test]
    fn lowers_cost_predicate_includes_tap_symbol() {
        assert_lowers(deckmaste_authoring::CostPredicate::IncludesTapSymbol);
    }

    #[test]
    fn lowers_as_though_counterfactual() {
        assert_lowers_debug(deckmaste_authoring::AsThough::Counterfactual {
            premise: minimal_predicate(),
            then: std::sync::Arc::new(minimal_deontic()),
        });
    }

    #[test]
    fn lowers_as_though_expanded() {
        assert_lowers_debug(deckmaste_authoring::AsThough::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_as_though()),
            },
        ));
    }

    #[test]
    fn lowers_count_bound_eq() {
        assert_lowers(deckmaste_authoring::CountBound::Eq(minimal_count()));
    }

    #[test]
    fn lowers_count_bound_at_least() {
        assert_lowers(deckmaste_authoring::CountBound::AtLeast(minimal_count()));
    }

    #[test]
    fn lowers_count_bound_at_most() {
        assert_lowers(deckmaste_authoring::CountBound::AtMost(minimal_count()));
    }

    #[test]
    fn lowers_count_bound_greater() {
        assert_lowers(deckmaste_authoring::CountBound::Greater(minimal_count()));
    }

    #[test]
    fn lowers_count_bound_less() {
        assert_lowers(deckmaste_authoring::CountBound::Less(minimal_count()));
    }

    #[test]
    fn lowers_deed_agent() {
        assert_lowers(deckmaste_authoring::DeedAgent {
            stack_object: None,
            source: None,
        });
    }

    #[test]
    fn lowers_deontic_action_attack() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Attack {
            by: minimal_predicate(),
            on: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_deontic_action_block() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Block {
            by: minimal_predicate(),
            on: minimal_predicate(),
            count: None,
        });
    }

    #[test]
    fn lowers_deontic_action_target() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Target {
            by: minimal_deed_agent(),
            on: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_deontic_action_attach() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Attach {
            what: minimal_predicate(),
            to: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_deontic_action_cast() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Cast {
            what: minimal_predicate(),
            by: minimal_predicate(),
            from: None,
            window: None,
            cost: None,
            tag: None,
        });
    }

    #[test]
    fn lowers_deontic_action_play() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Play {
            what: minimal_predicate(),
            by: minimal_predicate(),
            from: None,
        });
    }

    #[test]
    fn lowers_deontic_action_activate() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Activate {
            what: minimal_predicate(),
            by: minimal_predicate(),
            cost: None,
        });
    }

    #[test]
    fn lowers_deontic_action_regenerate() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Regenerate {
            by: minimal_predicate(),
            on: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_deontic_action_counter() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Counter {
            by: minimal_predicate(),
            on: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_deontic_action_untap() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Untap {
            what: minimal_predicate(),
        });
    }

    #[test]
    fn lowers_deontic_action_expanded() {
        assert_lowers_debug(deckmaste_authoring::DeonticAction::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic_action()),
            },
        ));
    }

    #[test]
    fn lowers_deontic_may() {
        assert_lowers_debug(deckmaste_authoring::Deontic::May(minimal_deontic_action()));
    }

    #[test]
    fn lowers_deontic_cant() {
        assert_lowers_debug(deckmaste_authoring::Deontic::Cant(minimal_deontic_action()));
    }

    #[test]
    fn lowers_deontic_must() {
        assert_lowers_debug(deckmaste_authoring::Deontic::Must(minimal_deontic_action()));
    }

    #[test]
    fn lowers_deontic_gate() {
        assert_lowers_debug(deckmaste_authoring::Deontic::Gate(
            minimal_deontic_action(),
            [].into(),
        ));
    }

    #[test]
    fn lowers_deontic_expanded() {
        assert_lowers_debug(deckmaste_authoring::Deontic::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_deontic()),
            },
        ));
    }
}
