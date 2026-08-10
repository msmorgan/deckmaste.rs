//! `cost` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::CostComponent {
    type Target = deckmaste_core::CostComponent;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Mana(f0) => deckmaste_core::CostComponent::Mana(f0.lower()),
            Self::ManaCostOf(f0) => deckmaste_core::CostComponent::ManaCostOf(f0.lower()),
            Self::Tap => deckmaste_core::CostComponent::Tap,
            Self::Untap => deckmaste_core::CostComponent::Untap,
            Self::Do(action) => lower_action_cost(action),
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
            Self::With { binder, body } => deckmaste_core::CostComponent::ChooseAndPay {
                binder: binder.lower(),
                body: body.lower(),
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

fn lower_action_cost(
    action: std::sync::Arc<deckmaste_semantics::Action>,
) -> deckmaste_core::CostComponent {
    use deckmaste_semantics::Action;
    use deckmaste_semantics::OneShotEffect;

    match action.as_ref() {
        Action::Composite { name, body } => {
            let OneShotEffect::With(with) = body.as_ref() else {
                return deckmaste_core::CostComponent::Act(action.lower());
            };
            let lowered_action = deckmaste_core::Action::Composite {
                name: (*name).lower(),
                body: with.body.clone().lower(),
            };
            deckmaste_core::CostComponent::ChooseAndPay {
                binder: std::sync::Arc::new(with.binder.clone().lower()),
                body: deckmaste_core::Cost(
                    vec![deckmaste_core::CostComponent::Act(std::sync::Arc::new(
                        lowered_action,
                    ))]
                    .into(),
                ),
            }
        }
        _ => deckmaste_core::CostComponent::Act(action.lower()),
    }
}

impl Lower for deckmaste_semantics::Cost {
    type Target = deckmaste_core::Cost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Cost(self.0.lower())
    }
}

impl Lower for deckmaste_semantics::CostTag {
    type Target = deckmaste_core::CostTag;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CostTag(self.0.lower())
    }
}

impl Lower for deckmaste_semantics::OptionalCost {
    type Target = deckmaste_core::OptionalCost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::OptionalCost {
            components: self.components.lower(),
            tag: self.tag.lower(),
            repeatable: self.repeatable.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::TotalCost {
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
            deckmaste_semantics::CostComponent::Mana(deckmaste_semantics::ManaCost::from(
                std::sync::Arc::<[deckmaste_semantics::ManaSymbol]>::from([])
            ))
            .lower(),
            deckmaste_core::CostComponent::Mana(_)
        );
    }

    #[test]
    fn lowers_cost_component_mana_cost_of() {
        assert_matches!(
            deckmaste_semantics::CostComponent::ManaCostOf(minimal_reference()).lower(),
            deckmaste_core::CostComponent::ManaCostOf(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_cost_component_tap() {
        assert_matches!(
            deckmaste_semantics::CostComponent::Tap.lower(),
            deckmaste_core::CostComponent::Tap
        );
    }

    #[test]
    fn lowers_cost_component_untap() {
        assert_matches!(
            deckmaste_semantics::CostComponent::Untap.lower(),
            deckmaste_core::CostComponent::Untap
        );
    }

    #[test]
    fn lowers_cost_component_do_to_runnable_act() {
        assert_matches!(
            deckmaste_semantics::CostComponent::Do(std::sync::Arc::new(minimal_action())).lower(),
            deckmaste_core::CostComponent::Act(_)
        );
    }

    #[test]
    fn lowers_cost_component_cost() {
        assert_matches!(
            deckmaste_semantics::CostComponent::Cost(minimal_cost()).lower(),
            deckmaste_core::CostComponent::Cost(deckmaste_core::Cost(_))
        );
    }

    #[test]
    fn lowers_cost_component_tap_total() {
        assert_matches!(
            deckmaste_semantics::CostComponent::TapTotal {
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
    fn lowers_cost_component_with_to_choose_and_pay() {
        assert_matches!(
            deckmaste_semantics::CostComponent::With {
                binder: std::sync::Arc::new(minimal_binder()),
                body: minimal_cost()
            }
            .lower(),
            deckmaste_core::CostComponent::ChooseAndPay {
                binder: _,
                body: deckmaste_core::Cost(_)
            }
        );
    }

    #[test]
    fn lifts_action_embedded_choice_into_cost_position() {
        use deckmaste_semantics::Action;
        use deckmaste_semantics::OneShotEffect;
        use deckmaste_semantics::With;

        let semantic =
            deckmaste_semantics::CostComponent::Do(std::sync::Arc::new(Action::Composite {
                name: "Discard".into(),
                body: std::sync::Arc::new(OneShotEffect::With(With {
                    binder: minimal_binder(),
                    body: std::sync::Arc::new(minimal_one_shot_effect()),
                })),
            }));

        let deckmaste_core::CostComponent::ChooseAndPay { body, .. } = semantic.lower() else {
            panic!("action-embedded choice must lower into cost position");
        };
        let [deckmaste_core::CostComponent::Act(action)] = body.0.as_ref() else {
            panic!("lifted choice body must be a runnable action cost");
        };
        let deckmaste_core::Action::Composite { body, .. } = action.as_ref() else {
            panic!("lifting must retain the authored keyword-action boundary");
        };
        assert!(
            !matches!(body.as_ref(), deckmaste_core::OneShotEffect::With(_)),
            "the runnable action must not retain the lifted choice"
        );
    }

    #[test]
    fn lowers_cost_component_expanded() {
        assert_matches!(
            deckmaste_semantics::CostComponent::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_cost_component())
            })
            .lower(),
            deckmaste_core::CostComponent::Mana(_)
        );
    }

    #[test]
    fn lowers_cost() {
        assert_matches!(
            deckmaste_semantics::Cost([].into()).lower(),
            deckmaste_core::Cost(_)
        );
    }

    #[test]
    fn lowers_cost_tag() {
        assert_matches!(
            deckmaste_semantics::CostTag("X".into()).lower(),
            deckmaste_core::CostTag(_)
        );
    }

    #[test]
    fn lowers_optional_cost() {
        assert_matches!(
            deckmaste_semantics::OptionalCost {
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
            deckmaste_semantics::TotalCost {
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
