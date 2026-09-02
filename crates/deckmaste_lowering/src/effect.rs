//! `effect` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::OneShotEffect {
    type Target = deckmaste_core::OneShotEffect;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Act(f0) => deckmaste_core::OneShotEffect::Act(f0.lower()),
            Self::Sequentially(f0) => deckmaste_core::OneShotEffect::Sequentially(f0.lower()),
            Self::Simultaneously(f0) => deckmaste_core::OneShotEffect::Simultaneously(f0.lower()),
            Self::Continuously(f0) => deckmaste_core::OneShotEffect::Continuously(f0.lower()),
            Self::Until(f0, f1) => deckmaste_core::OneShotEffect::Until(f0.lower(), f1.lower()),
            Self::Label(f0) => deckmaste_core::OneShotEffect::Label(f0.lower()),
            Self::SeparatePiles(f0) => deckmaste_core::OneShotEffect::SeparatePiles(f0.lower()),
            Self::ChoosePile(f0) => deckmaste_core::OneShotEffect::ChoosePile(f0.lower()),
            Self::May(f0) => deckmaste_core::OneShotEffect::May(f0.lower()),
            Self::If(f0) => deckmaste_core::OneShotEffect::If(f0.lower()),
            Self::AdditionalCost(f0) => deckmaste_core::OneShotEffect::AdditionalCost(f0.lower()),
            Self::Each(f0) => deckmaste_core::OneShotEffect::Each(f0.lower()),
            Self::With(f0) => deckmaste_core::OneShotEffect::With(f0.lower()),
            Self::Distribute(f0) => deckmaste_core::OneShotEffect::Distribute(f0.lower()),
            Self::Noting(f0) => deckmaste_core::OneShotEffect::Noting(f0.lower()),
            Self::Delayed(f0) => deckmaste_core::OneShotEffect::Delayed(f0.lower()),
            Self::Reflexive(f0) => deckmaste_core::OneShotEffect::Reflexive(f0.lower()),
            Self::Modal(f0) => deckmaste_core::OneShotEffect::Modal(f0.lower()),
            Self::Targeted(f0) => f0.lower(),
            Self::Repeat(f0, f1) => deckmaste_core::OneShotEffect::Repeat(f0.lower(), f1.lower()),
            Self::Batch(f0, f1) => deckmaste_core::OneShotEffect::Batch(f0.lower(), f1.lower()),
            Self::RevealUntil(f0) => deckmaste_core::OneShotEffect::RevealUntil(f0.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Continuously {
    type Target = deckmaste_core::Continuously;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Continuously {
            effect: self.effect.lower(),
            duration: self.duration.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Targeted {
    type Target = deckmaste_core::OneShotEffect;
    fn lower(self) -> <Self as Lower>::Target {
        std::sync::Arc::unwrap_or_clone(self.effect).lower()
    }
}

impl Lower for deckmaste_semantics::May {
    type Target = deckmaste_core::May;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::May {
            who: self.who.lower(),
            effect: self.effect.lower(),
            if_did: self.if_did.lower(),
            if_not: self.if_not.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::If {
    type Target = deckmaste_core::If;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::If {
            condition: self.condition.lower(),
            then: self.then.lower(),
            otherwise: self.otherwise.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Noting {
    type Target = deckmaste_core::Noting;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Noting {
            key: self.key.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::AdditionalCost {
    type Target = deckmaste_core::AdditionalCost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::AdditionalCost {
            pay: self.pay.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Each {
    type Target = deckmaste_core::Each;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Each {
            binder: self.binder.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::With {
    type Target = deckmaste_core::With;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::With {
            binder: self.binder.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Distribute {
    type Target = deckmaste_core::Distribute;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Distribute {
            amount: self.amount.lower(),
            binder: self.binder.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::RevealUntil {
    type Target = deckmaste_core::RevealUntil;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::RevealUntil {
            whose: self.whose.lower(),
            matches: self.matches.lower(),
            body: self.body.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Modal {
    type Target = deckmaste_core::Modal;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Modal {
            choose: self.choose.lower(),
            modes: self.modes.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::Label {
    type Target = deckmaste_core::Label;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Label {
            r#as: self.r#as.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::SeparatePiles {
    type Target = deckmaste_core::SeparatePiles;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::SeparatePiles {
            group: self.group.lower(),
            into: self.into.lower(),
            by: self.by.lower(),
            note: self.note.lower(),
            then: self.then.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::ChoosePile {
    type Target = deckmaste_core::ChoosePile;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::ChoosePile {
            from: self.from.lower(),
            by: self.by.lower(),
            random: self.random.lower(),
            then: self.then.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::PileSource {
    type Target = deckmaste_core::PileSource;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Labels(f0) => deckmaste_core::PileSource::Labels(f0.lower()),
            Self::Noted { note, of } => deckmaste_core::PileSource::Noted {
                note: note.lower(),
                of: of.lower(),
            },
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
    fn lowers_one_shot_effect_act() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Act(minimal_action()).lower(),
            deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                deckmaste_core::Reference::This,
                deckmaste_core::Count::X,
                deckmaste_core::Reference::This
            ))
        );
    }

    #[test]
    fn lowers_one_shot_effect_sequentially() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Sequentially([].into()).lower(),
            deckmaste_core::OneShotEffect::Sequentially(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_simultaneously() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Simultaneously([].into()).lower(),
            deckmaste_core::OneShotEffect::Simultaneously(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_continuously() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Continuously(minimal_continuously()).lower(),
            deckmaste_core::OneShotEffect::Continuously(deckmaste_core::Continuously {
                effect: _,
                duration: deckmaste_core::Duration::FixedUntil(
                    deckmaste_core::TurnMarker::EndOfTurn
                )
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_until() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Until(minimal_duration(), [].into()).lower(),
            deckmaste_core::OneShotEffect::Until(
                deckmaste_core::Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
                _
            )
        );
    }

    #[test]
    fn lowers_one_shot_effect_label() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Label(minimal_label()).lower(),
            deckmaste_core::OneShotEffect::Label(deckmaste_core::Label { r#as: _, effect: _ })
        );
    }

    #[test]
    fn lowers_one_shot_effect_separate_piles() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::SeparatePiles(minimal_separate_piles()).lower(),
            deckmaste_core::OneShotEffect::SeparatePiles(deckmaste_core::SeparatePiles {
                group: deckmaste_core::Selection::SelectAll(deckmaste_core::Predicate::Kind(
                    deckmaste_core::ObjectKind::Ability
                )),
                into: _,
                by: deckmaste_core::Reference::This,
                note: None,
                then: None
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_choose_pile() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::ChoosePile(minimal_choose_pile()).lower(),
            deckmaste_core::OneShotEffect::ChoosePile(deckmaste_core::ChoosePile {
                from: deckmaste_core::PileSource::Labels(_),
                by: deckmaste_core::Reference::This,
                random: false,
                then: _
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_may() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::May(minimal_may()).lower(),
            deckmaste_core::OneShotEffect::May(deckmaste_core::May {
                who: deckmaste_core::Reference::This,
                effect: _,
                if_did: None,
                if_not: None
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_if() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::If(minimal_if()).lower(),
            deckmaste_core::OneShotEffect::If(deckmaste_core::If {
                condition: deckmaste_core::Condition::Compare(
                    deckmaste_core::Count::X,
                    deckmaste_core::Cmp::Eq,
                    deckmaste_core::Count::X
                ),
                then: _,
                otherwise: None
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_additional_cost() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::AdditionalCost(minimal_additional_cost()).lower(),
            deckmaste_core::OneShotEffect::AdditionalCost(deckmaste_core::AdditionalCost {
                pay: deckmaste_core::Cost(_),
                body: _
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_each() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Each(minimal_each()).lower(),
            deckmaste_core::OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This),
                effect: _
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_with() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::With(minimal_with()).lower(),
            deckmaste_core::OneShotEffect::With(deckmaste_core::With {
                binder: deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This),
                body: _
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_distribute() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Distribute(minimal_distribute()).lower(),
            deckmaste_core::OneShotEffect::Distribute(deckmaste_core::Distribute {
                amount: deckmaste_core::Count::X,
                binder: deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This),
                body: _
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_noting() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Noting(minimal_noting()).lower(),
            deckmaste_core::OneShotEffect::Noting(deckmaste_core::Noting { key: _, effect: _ })
        );
    }

    #[test]
    fn lowers_one_shot_effect_delayed() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Delayed(std::sync::Arc::new(
                minimal_triggered_ability()
            ))
            .lower(),
            deckmaste_core::OneShotEffect::Delayed(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_reflexive() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Reflexive(std::sync::Arc::new(
                minimal_triggered_ability()
            ))
            .lower(),
            deckmaste_core::OneShotEffect::Reflexive(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_modal() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Modal(minimal_modal()).lower(),
            deckmaste_core::OneShotEffect::Modal(deckmaste_core::Modal {
                choose: deckmaste_core::ChooseSpec {
                    count: deckmaste_core::Quantity::Range(None, None),
                    up_to: false,
                    repeats: false,
                    chooser: deckmaste_core::Reference::This,
                    rider: None
                },
                modes: _
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_targeted() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Targeted(minimal_targeted()).lower(),
            deckmaste_core::OneShotEffect::Act(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_repeat() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Repeat(
                minimal_count(),
                std::sync::Arc::new(minimal_one_shot_effect())
            )
            .lower(),
            deckmaste_core::OneShotEffect::Repeat(deckmaste_core::Count::X, _)
        );
    }

    #[test]
    fn lowers_one_shot_effect_batch() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Batch(
                minimal_count(),
                std::sync::Arc::new(minimal_one_shot_effect())
            )
            .lower(),
            deckmaste_core::OneShotEffect::Batch(deckmaste_core::Count::X, _)
        );
    }

    #[test]
    fn lowers_one_shot_effect_reveal_until() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::RevealUntil(minimal_reveal_until()).lower(),
            deckmaste_core::OneShotEffect::RevealUntil(deckmaste_core::RevealUntil {
                whose: deckmaste_core::Reference::This,
                matches: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                body: _
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_expanded() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_one_shot_effect())
            })
            .lower(),
            deckmaste_core::OneShotEffect::Act(deckmaste_core::Action::DealDamage(
                deckmaste_core::Reference::This,
                deckmaste_core::Count::X,
                deckmaste_core::Reference::This
            ))
        );
    }

    #[test]
    fn lowers_continuously() {
        assert_matches!(
            deckmaste_semantics::Continuously {
                effect: std::sync::Arc::new(minimal_static_effect()),
                duration: minimal_duration()
            }
            .lower(),
            deckmaste_core::Continuously {
                effect: _,
                duration: deckmaste_core::Duration::FixedUntil(
                    deckmaste_core::TurnMarker::EndOfTurn
                )
            }
        );
    }

    #[test]
    fn lowers_targeted() {
        assert_matches!(
            deckmaste_semantics::Targeted {
                targets: [].into(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::OneShotEffect::Act(_)
        );
    }

    #[test]
    fn lowers_may() {
        assert_matches!(
            deckmaste_semantics::May {
                who: minimal_reference(),
                effect: std::sync::Arc::new(minimal_one_shot_effect()),
                if_did: None,
                if_not: None
            }
            .lower(),
            deckmaste_core::May {
                who: deckmaste_core::Reference::This,
                effect: _,
                if_did: None,
                if_not: None
            }
        );
    }

    #[test]
    fn lowers_if() {
        assert_matches!(
            deckmaste_semantics::If {
                condition: minimal_condition(),
                then: std::sync::Arc::new(minimal_one_shot_effect()),
                otherwise: None
            }
            .lower(),
            deckmaste_core::If {
                condition: deckmaste_core::Condition::Compare(
                    deckmaste_core::Count::X,
                    deckmaste_core::Cmp::Eq,
                    deckmaste_core::Count::X
                ),
                then: _,
                otherwise: None
            }
        );
    }

    #[test]
    fn lowers_noting() {
        assert_matches!(
            deckmaste_semantics::Noting {
                key: "X".into(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Noting { key: _, effect: _ }
        );
    }

    #[test]
    fn lowers_additional_cost() {
        assert_matches!(
            deckmaste_semantics::AdditionalCost {
                pay: minimal_cost(),
                body: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::AdditionalCost {
                pay: deckmaste_core::Cost(_),
                body: _
            }
        );
    }

    #[test]
    fn lowers_each() {
        assert_matches!(
            deckmaste_semantics::Each {
                binder: minimal_binder(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Each {
                binder: deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This),
                effect: _
            }
        );
    }

    #[test]
    fn lowers_with() {
        assert_matches!(
            deckmaste_semantics::With {
                binder: minimal_binder(),
                body: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::With {
                binder: deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This),
                body: _
            }
        );
    }

    #[test]
    fn lowers_distribute() {
        assert_matches!(
            deckmaste_semantics::Distribute {
                amount: minimal_count(),
                binder: minimal_binder(),
                body: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Distribute {
                amount: deckmaste_core::Count::X,
                binder: deckmaste_core::Binder::TheRef(deckmaste_core::Reference::This),
                body: _
            }
        );
    }

    #[test]
    fn lowers_reveal_until() {
        assert_matches!(
            deckmaste_semantics::RevealUntil {
                whose: minimal_reference(),
                matches: minimal_predicate(),
                body: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::RevealUntil {
                whose: deckmaste_core::Reference::This,
                matches: deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability),
                body: _
            }
        );
    }

    #[test]
    fn lowers_modal() {
        assert_matches!(
            deckmaste_semantics::Modal {
                choose: minimal_choose_spec(),
                modes: [].into()
            }
            .lower(),
            deckmaste_core::Modal {
                choose: deckmaste_core::ChooseSpec {
                    count: deckmaste_core::Quantity::Range(None, None),
                    up_to: false,
                    repeats: false,
                    chooser: deckmaste_core::Reference::This,
                    rider: None
                },
                modes: _
            }
        );
    }

    #[test]
    fn lowers_label() {
        assert_matches!(
            deckmaste_semantics::Label {
                r#as: "X".into(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Label { r#as: _, effect: _ }
        );
    }

    #[test]
    fn lowers_separate_piles() {
        assert_matches!(
            deckmaste_semantics::SeparatePiles {
                group: minimal_selection(),
                into: [].into(),
                by: minimal_reference(),
                note: None,
                then: None
            }
            .lower(),
            deckmaste_core::SeparatePiles {
                group: deckmaste_core::Selection::SelectAll(deckmaste_core::Predicate::Kind(
                    deckmaste_core::ObjectKind::Ability
                )),
                into: _,
                by: deckmaste_core::Reference::This,
                note: None,
                then: None
            }
        );
    }

    #[test]
    fn lowers_choose_pile() {
        assert_matches!(
            deckmaste_semantics::ChoosePile {
                from: minimal_pile_source(),
                by: minimal_reference(),
                random: false,
                then: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::ChoosePile {
                from: deckmaste_core::PileSource::Labels(_),
                by: deckmaste_core::Reference::This,
                random: false,
                then: _
            }
        );
    }

    #[test]
    fn lowers_pile_source_labels() {
        assert_matches!(
            deckmaste_semantics::PileSource::Labels([].into()).lower(),
            deckmaste_core::PileSource::Labels(_)
        );
    }

    #[test]
    fn lowers_pile_source_noted() {
        assert_matches!(
            deckmaste_semantics::PileSource::Noted {
                note: "X".into(),
                of: minimal_reference()
            }
            .lower(),
            deckmaste_core::PileSource::Noted {
                note: _,
                of: deckmaste_core::Reference::This
            }
        );
    }
}
