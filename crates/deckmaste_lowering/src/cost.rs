//! `cost` — semantics grammar to engine AST.
//!
//! A semantic cost is a list of payment demands; a core cost is a BLOCK of
//! cost instructions run at announcement in the ability's own activation
//! ([CR#601.2b,601.2h]). Lowering is what turns one into the other: a
//! semantic `With` binder becomes the decision instruction that writes the
//! payment subject's register, and the verbs that spend it follow flat in the
//! same block, reading that register. The binding stays live afterwards, so
//! the ability body can read the paid product ("the sacrificed creature") as
//! a def rather than re-deriving it.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use std::sync::Arc;

use crate::Lower;
use crate::effect::BoundValue;

/// Lower a semantic cost list into a core cost block. The block's definitions
/// continue the ability region's sequence ([CR#601.2b]: the announcement runs
/// in the ability's own activation), but its ANAPHORA are scoped to the block —
/// a payment subject is read by the verbs that spend it, not by the effect
/// clause that follows. Returns the block plus the LAST payment subject it
/// bound: the paid product an ability body names ("the sacrificed creature",
/// [CR#118.8]), which the caller wires to the region's event-object channel.
pub(crate) fn lower_cost_block(
    components: &[deckmaste_semantics::CostComponent],
) -> (Arc<[deckmaste_core::CostComponent]>, Option<BoundValue>) {
    if crate::region::is_active() {
        crate::region::scoped_anaphora(|| lower_cost_block_inner(components))
    } else {
        lower_cost_block_inner(components)
    }
}

fn lower_cost_block_inner(
    components: &[deckmaste_semantics::CostComponent],
) -> (Arc<[deckmaste_core::CostComponent]>, Option<BoundValue>) {
    let mut out = Vec::new();
    let mut paid = None;
    for component in components {
        if let Some(bound) = lower_cost_component(component.clone(), &mut out) {
            paid = Some(bound);
        }
    }
    (out.into(), paid)
}

/// Lower a cost list whose bindings must NOT escape — a deontic gate, an
/// alternative or optional cost row, a modal cost rider. These are announced
/// against another announcement's register file, so nothing downstream may
/// read their products.
pub(crate) fn lower_scoped_cost_components(
    components: &[deckmaste_semantics::CostComponent],
) -> Arc<[deckmaste_core::CostComponent]> {
    if crate::region::is_active() {
        crate::region::scoped_antecedents(|| lower_cost_block(components).0)
    } else {
        lower_cost_block(components).0
    }
}

/// Lower one semantic component, appending the instructions it becomes.
/// Returns the payment subject it bound, if it bound one.
fn lower_cost_component(
    component: deckmaste_semantics::CostComponent,
    out: &mut Vec<deckmaste_core::CostComponent>,
) -> Option<BoundValue> {
    use deckmaste_semantics::CostComponent as S;
    match component {
        S::Mana(f0) => {
            out.push(deckmaste_core::CostComponent::Mana(f0.lower()));
            None
        }
        S::ManaCostOf(f0) => {
            out.push(deckmaste_core::CostComponent::ManaCostOf(f0.lower()));
            None
        }
        S::Tap => {
            out.push(deckmaste_core::CostComponent::Tap);
            None
        }
        S::Untap => {
            out.push(deckmaste_core::CostComponent::Untap);
            None
        }
        S::Do(action) => lower_action_cost(&action, out),
        S::Cost(nested) => {
            // A macro list-splice ([CR#702.29a] cycling): the nested block's
            // instructions are this block's instructions, so splice rather
            // than nest — one definition sequence, one announcement.
            let (inner, bound) = lower_cost_block_inner(&nested.0);
            out.extend(inner.iter().cloned());
            bound
        }
        S::TapTotal {
            stat,
            cmp,
            count,
            filter,
        } => {
            out.push(deckmaste_core::CostComponent::TapTotal {
                stat: stat.lower(),
                cmp: cmp.lower(),
                count: count.lower(),
                filter: filter.lower(),
            });
            None
        }
        S::With { binder, body } => {
            // [CR#601.2b]: the choice is its OWN instruction, made before the
            // verbs that spend it, writing a register they read.
            let (instructions, bound) = crate::effect::lower_binder(Arc::unwrap_or_clone(binder));
            out.extend(instructions.into_iter().map(cost_instruction));
            crate::region::push_antecedent(
                bound.reference,
                bound.kind,
                bound.cardinality,
                bound.sort,
                crate::region::Site::Frame,
            );
            let (inner, inner_bound) = lower_cost_block_inner(&body.0);
            out.extend(inner.iter().cloned());
            Some(inner_bound.unwrap_or(bound))
        }
        // Invocation provenance does not cross `lower`: the core grammar is
        // a compiled artifact and carries no record of the semantic
        // spelling (spec §12). Prose recovers the semantic term through the
        // provenance index instead. This is the divergence ledger's first
        // non-identity arm family.
        S::Expanded(f0) => lower_cost_component(*f0.value, out),
    }
}

/// One lowered instruction as the cost instruction it becomes. A cost block
/// admits exactly the instruction kinds a payment can perform: a payment-time
/// decision and a paying action ([CR#601.2b,601.2h]).
fn cost_instruction(instruction: deckmaste_core::Instr) -> deckmaste_core::CostComponent {
    use deckmaste_core::CostComponent;
    use deckmaste_core::Instr;
    match instruction {
        Instr::Choose(choice) => CostComponent::Choose(choice),
        Instr::Search(search) => CostComponent::Search(search),
        Instr::Let(binding) => CostComponent::Let(binding),
        Instr::Act { dest, action } => CostComponent::Act {
            dest,
            action: runnable_action(action),
        },
        other => unreachable!(
            "a cost binder lowers to a decision or a paying action ([CR#601.2b]), got {other:?}"
        ),
    }
}

fn lower_action_cost(
    action: &Arc<deckmaste_semantics::Action>,
    out: &mut Vec<deckmaste_core::CostComponent>,
) -> Option<BoundValue> {
    use deckmaste_semantics::Action;
    use deckmaste_semantics::OneShotEffect;

    let Action::Composite { name, body } = action.as_ref() else {
        out.push(deckmaste_core::CostComponent::Act {
            dest: None,
            action: runnable_action(action.as_ref().clone().lower()),
        });
        return None;
    };
    let OneShotEffect::With(with) = body.as_ref() else {
        out.push(deckmaste_core::CostComponent::Act {
            dest: None,
            action: runnable_action(action.as_ref().clone().lower()),
        });
        return None;
    };
    // "Discard a card:" — the composite's own chooser is lifted out of the
    // verb into a preceding cost instruction ([CR#701.9,601.2b]).
    let (instructions, bound) = crate::effect::lower_binder(with.binder.clone());
    out.extend(instructions.into_iter().map(cost_instruction));
    crate::region::push_antecedent(
        bound.reference,
        bound.kind,
        bound.cardinality,
        bound.sort,
        crate::region::Site::Frame,
    );
    let lowered = deckmaste_core::Action::Composite {
        name: (*name).lower(),
        body: with.body.clone().lower(),
    };
    out.push(deckmaste_core::CostComponent::Act {
        dest: None,
        action: runnable_action(lowered),
    });
    Some(bound)
}

fn runnable_action(action: deckmaste_core::Action) -> deckmaste_core::RunnableCostAction {
    deckmaste_core::RunnableCostAction::try_new(action)
        .expect("validated semantic cost actions must be eligible and have every choice lifted")
}

impl Lower for deckmaste_semantics::Cost {
    type Target = deckmaste_core::Cost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Cost(lower_scoped_cost_components(&self.0))
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
            components: lower_scoped_cost_components(&self.components),
            tag: self.tag.lower(),
            repeatable: self.repeatable.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::TotalCost {
    type Target = deckmaste_core::TotalCost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TotalCost {
            base: lower_scoped_cost_components(&self.base),
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
        let (lowered, _) = super::lower_cost_block(&[deckmaste_semantics::CostComponent::Mana(
            deckmaste_semantics::ManaCost::from(
                std::sync::Arc::<[deckmaste_semantics::ManaSymbol]>::from([]),
            ),
        )]);
        assert_matches!(lowered[0], deckmaste_core::CostComponent::Mana(_));
    }

    #[test]
    fn lowers_cost_component_mana_cost_of() {
        let (lowered, _) =
            super::lower_cost_block(&[deckmaste_semantics::CostComponent::ManaCostOf(
                minimal_reference(),
            )]);
        assert_matches!(
            lowered[0],
            deckmaste_core::CostComponent::ManaCostOf(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_cost_component_tap() {
        let (lowered, _) = super::lower_cost_block(&[deckmaste_semantics::CostComponent::Tap]);
        assert_eq!(&*lowered, &[deckmaste_core::CostComponent::Tap]);
    }

    #[test]
    fn lowers_cost_component_untap() {
        let (lowered, _) = super::lower_cost_block(&[deckmaste_semantics::CostComponent::Untap]);
        assert_eq!(&*lowered, &[deckmaste_core::CostComponent::Untap]);
    }

    #[test]
    fn lowers_cost_component_do_to_runnable_act() {
        let action = deckmaste_semantics::Action::Sacrifice(
            deckmaste_semantics::Reference::You,
            deckmaste_semantics::Reference::This,
        );
        let (lowered, paid) = super::lower_cost_block(&[deckmaste_semantics::CostComponent::Do(
            std::sync::Arc::new(action),
        )]);
        assert_matches!(
            lowered[0],
            deckmaste_core::CostComponent::Act { dest: None, .. }
        );
        assert!(paid.is_none(), "a bound verb binds no new payment subject");
    }

    /// A nested cost list SPLICES: one announcement, one definition sequence
    /// ([CR#702.29a] cycling).
    #[test]
    fn lowers_nested_cost_flat() {
        let (lowered, _) = super::lower_cost_block(&[deckmaste_semantics::CostComponent::Cost(
            deckmaste_semantics::Cost(vec![deckmaste_semantics::CostComponent::Tap].into()),
        )]);
        assert_eq!(&*lowered, &[deckmaste_core::CostComponent::Tap]);
    }

    #[test]
    fn lowers_cost_component_tap_total() {
        let (lowered, _) =
            super::lower_cost_block(&[deckmaste_semantics::CostComponent::TapTotal {
                stat: minimal_stat(),
                cmp: minimal_cmp(),
                count: minimal_count(),
                filter: std::sync::Arc::new(minimal_predicate()),
            }]);
        assert_matches!(lowered[0], deckmaste_core::CostComponent::TapTotal { .. });
    }

    /// A cost-side `With` becomes a payment-time DECISION instruction followed
    /// by the verbs that spend its register ([CR#601.2b]) — the binder never
    /// survives as a nested cost.
    #[test]
    fn lowers_cost_component_with_to_a_decision_then_its_verbs() {
        let (_, (lowered, paid)) =
            crate::region::in_region(crate::region::RegionKind::Activated, 0, || {
                super::lower_cost_block(&[deckmaste_semantics::CostComponent::With {
                    binder: std::sync::Arc::new(deckmaste_semantics::Binder::ChooseOne {
                        filter: deckmaste_semantics::Predicate::Any,
                        by: deckmaste_semantics::Reference::You,
                    }),
                    body: deckmaste_semantics::Cost(
                        vec![deckmaste_semantics::CostComponent::Do(std::sync::Arc::new(
                            deckmaste_semantics::Action::Sacrifice(
                                deckmaste_semantics::Reference::You,
                                deckmaste_semantics::Reference::That(
                                    deckmaste_semantics::Sort::Permanent,
                                ),
                            ),
                        ))]
                        .into(),
                    ),
                }])
            });
        assert_matches!(lowered[0], deckmaste_core::CostComponent::Choose(_));
        assert_matches!(
            lowered[1],
            deckmaste_core::CostComponent::Act { dest: None, .. }
        );
        let chosen = paid
            .expect("the chooser bound the payment subject")
            .reference;
        let deckmaste_core::CostComponent::Act { action, .. } = &lowered[1] else {
            panic!("the paying verb is an Act");
        };
        assert_eq!(
            **action,
            deckmaste_core::Action::Sacrifice(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
                deckmaste_core::Reference::Reg(chosen),
            ),
            "the verb pays through the register the choice wrote"
        );
    }

    /// One payment binder, lowered as the cost instruction it becomes.
    /// Payment binders lower inside a region (their destinations continue the
    /// ability's definition sequence), so each case runs in one.
    fn lower_payment_binder(
        binder: deckmaste_semantics::Binder,
    ) -> Vec<deckmaste_core::CostComponent> {
        let (_, (lowered, _)) =
            crate::region::in_region(crate::region::RegionKind::Activated, 0, || {
                super::lower_cost_block(&[deckmaste_semantics::CostComponent::With {
                    binder: std::sync::Arc::new(binder),
                    body: deckmaste_semantics::Cost([].into()),
                }])
            });
        lowered.to_vec()
    }

    // Each payment binder's lowered image. Re-spelled from the deleted
    // `Binder -> CostBinder` cases: `CostBinder` is gone, so a binder's image
    // is the cost INSTRUCTION that writes the payment subject's register
    // ([CR#601.2b]) — same binder, same asserted shape.

    #[test]
    fn lowers_binder_the_ref() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::TheRef(minimal_reference()))
                .as_slice(),
            [deckmaste_core::CostComponent::Let(deckmaste_core::Let {
                dest: _,
                expr: deckmaste_core::Expr::Object(deckmaste_core::Reference::Reg(_)),
            })]
        );
    }

    #[test]
    fn lowers_binder_choose_one() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::ChooseOne {
                filter: minimal_predicate(),
                by: minimal_reference(),
            })
            .as_slice(),
            [deckmaste_core::CostComponent::Choose(
                deckmaste_core::Choose {
                    quantity: deckmaste_core::Quantity::Range(
                        Some(deckmaste_core::Count::Literal(1)),
                        Some(deckmaste_core::Count::Literal(1)),
                    ),
                    ..
                }
            )]
        );
    }

    #[test]
    fn lowers_binder_produce() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::Produce(std::sync::Arc::new(
                deckmaste_semantics::Action::Move(
                    deckmaste_semantics::Reference::This,
                    deckmaste_semantics::Destination::Zone(deckmaste_semantics::Zone::Exile),
                    [].into(),
                    None,
                )
            )))
            .as_slice(),
            [deckmaste_core::CostComponent::Act { dest: Some(_), .. }]
        );
    }

    #[test]
    fn lowers_binder_search_one() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::SearchOne {
                filter: minimal_predicate(),
                by: minimal_reference(),
                whose: minimal_reference(),
                from: [].into(),
                if_none: None,
            })
            .as_slice(),
            [deckmaste_core::CostComponent::Search(
                deckmaste_core::Search {
                    quantity: deckmaste_core::Quantity::Range(
                        Some(deckmaste_core::Count::Literal(1)),
                        Some(deckmaste_core::Count::Literal(1)),
                    ),
                    ..
                }
            )]
        );
    }

    #[test]
    fn lowers_binder_choose() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::Choose {
                quantity: minimal_quantity(),
                filter: minimal_predicate(),
                by: minimal_reference(),
            })
            .as_slice(),
            [deckmaste_core::CostComponent::Choose(
                deckmaste_core::Choose {
                    quantity: deckmaste_core::Quantity::Range(None, None),
                    by: deckmaste_core::Reference::Reg(_),
                    ..
                }
            )]
        );
    }

    #[test]
    fn lowers_binder_existing() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::Existing(minimal_selection()))
                .as_slice(),
            [deckmaste_core::CostComponent::Let(deckmaste_core::Let {
                dest: _,
                expr: deckmaste_core::Expr::Objects(deckmaste_core::Selection::SelectAll(_)),
            })]
        );
    }

    #[test]
    fn lowers_binder_search() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::Search {
                quantity: minimal_quantity(),
                filter: minimal_predicate(),
                by: minimal_reference(),
                whose: minimal_reference(),
                from: [].into(),
                if_none: None,
            })
            .as_slice(),
            [deckmaste_core::CostComponent::Search(
                deckmaste_core::Search {
                    quantity: deckmaste_core::Quantity::Range(None, None),
                    by: deckmaste_core::Reference::Reg(_),
                    whose: deckmaste_core::Reference::Reg(_),
                    ..
                }
            )]
        );
    }

    #[test]
    fn lowers_binder_expanded() {
        assert_matches!(
            lower_payment_binder(deckmaste_semantics::Binder::Expanded(
                macro_ron::Expansion {
                    name: "X".into(),
                    args: macro_ron::ExpansionArgs::none(),
                    template: None,
                    value: Box::new(minimal_binder()),
                }
            ))
            .as_slice(),
            [deckmaste_core::CostComponent::Let(deckmaste_core::Let {
                dest: _,
                expr: deckmaste_core::Expr::Object(deckmaste_core::Reference::Reg(_)),
            })]
        );
    }

    #[test]
    fn lowers_cost_component_expanded() {
        let (lowered, _) =
            super::lower_cost_block(&[deckmaste_semantics::CostComponent::Expanded(
                macro_ron::Expansion {
                    name: "X".into(),
                    args: macro_ron::ExpansionArgs::none(),
                    template: None,
                    value: Box::new(minimal_cost_component()),
                },
            )]);
        assert_matches!(lowered[0], deckmaste_core::CostComponent::Mana(_));
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
