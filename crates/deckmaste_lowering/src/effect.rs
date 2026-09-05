//! `effect` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

use deckmaste_core::Instruction;

fn one(instructions: Vec<Instruction>) -> Instruction {
    match instructions.as_slice() {
        [instruction] => instruction.clone(),
        _ => Instruction::Sequentially(instructions.into()),
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
    fn lowers_one_shot_effect_sequentially() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Sequentially([].into()).lower(),
            deckmaste_core::Instruction::Sequentially(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_simultaneously() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Simultaneously([].into()).lower(),
            deckmaste_core::Instruction::Simultaneously(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_continuously() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Continuously(minimal_continuously()).lower(),
            deckmaste_core::Instruction::Continuously(deckmaste_core::Continuously {
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
            deckmaste_core::Instruction::Until(
                deckmaste_core::Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn),
                _
            )
        );
    }

    #[test]
    fn lowers_one_shot_effect_separate_piles() {
        assert_matches!(
            in_spell_region(|| {
                deckmaste_semantics::OneShotEffect::SeparatePiles(minimal_separate_piles()).lower()
            }),
            deckmaste_core::Instruction::SeparatePiles(deckmaste_core::SeparatePiles {
                dests: _,
                group: deckmaste_core::Selection::SelectAll(_),
                by: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                then: None
            })
        );
    }

    #[test]
    fn lowers_one_shot_effect_delayed() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Delayed(std::sync::Arc::new(
                minimal_triggered_ability()
            ))
            .lower(),
            deckmaste_core::Instruction::Delayed(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_reflexive() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Reflexive(std::sync::Arc::new(
                minimal_triggered_ability()
            ))
            .lower(),
            deckmaste_core::Instruction::Reflexive(_)
        );
    }

    #[test]
    fn lowers_one_shot_effect_modal() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::Modal(minimal_modal()).lower(),
            deckmaste_core::Instruction::Modal(deckmaste_core::Modal {
                choose: deckmaste_core::ChooseSpec {
                    count: deckmaste_core::Quantity::Range(None, None),
                    up_to: false,
                    repeats: false,
                    chooser: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                    rider: None
                },
                modes: _
            })
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
                    chooser: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                    rider: None
                },
                modes: _
            }
        );
    }

    #[test]
    fn lowers_separate_piles() {
        assert_matches!(
            in_spell_region(|| {
                deckmaste_semantics::SeparatePiles {
                    group: minimal_selection(),
                    into: [].into(),
                    by: minimal_reference(),
                    note: None,
                    then: None,
                }
                .lower()
            }),
            deckmaste_core::SeparatePiles {
                dests: _,
                group: deckmaste_core::Selection::SelectAll(_),
                by: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                then: None
            }
        );
    }

    #[test]
    fn lowers_pile_source_labels() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::SeparatePiles(deckmaste_semantics::SeparatePiles {
                group: minimal_selection(),
                into: ["left".into(), "right".into()].into(),
                by: minimal_reference(),
                note: None,
                then: Some(std::sync::Arc::new(
                    deckmaste_semantics::OneShotEffect::ChoosePile(
                        deckmaste_semantics::ChoosePile {
                            from: deckmaste_semantics::PileSource::Labels(
                                ["left".into(), "right".into()].into(),
                            ),
                            by: minimal_reference(),
                            random: false,
                            then: std::sync::Arc::new(minimal_one_shot_effect()),
                        },
                    ),
                )),
            })
            .lower()
        });
        let deckmaste_core::Instruction::SeparatePiles(separate) = lowered else {
            panic!("expected SeparatePiles");
        };
        assert_eq!(
            separate.dests.as_ref(),
            [deckmaste_core::DefId(3), deckmaste_core::DefId(4)]
        );
        let Some(then) = separate.then else {
            panic!("expected nested choice");
        };
        let deckmaste_core::Instruction::ChoosePile(choice) = then.as_ref() else {
            panic!("expected ChoosePile");
        };
        assert_eq!(
            choice.from.as_ref(),
            [deckmaste_core::RefId(3), deckmaste_core::RefId(4)]
        );
        assert_eq!(choice.dest, deckmaste_core::DefId(5));
    }

    #[test]
    fn lowers_pile_source_noted() {
        let error = crate::lower_for_test(|| {
            in_spell_region(|| {
                deckmaste_semantics::ChoosePile {
                    from: deckmaste_semantics::PileSource::Noted {
                        note: "X".into(),
                        of: minimal_reference(),
                    },
                    by: minimal_reference(),
                    random: false,
                    then: std::sync::Arc::new(minimal_one_shot_effect()),
                }
                .lower()
            })
        })
        .expect_err("an unimplemented noted pile source is refused");
        assert_eq!(&*error.card, "Lowering Test");
        assert!(
            error
                .message
                .contains("noted pile sets have no register spelling yet"),
            "the returned diagnostic carries the refusal, got {:?}",
            error.message
        );
    }

    // ---- Instruction lowering, restored from the discourse landing ----
    //
    // A `Spell` region declares source(0), controller(1), announced X(2); the
    // first instruction definition is therefore register 3.

    /// One authored `Act` becomes the instruction pair its magnitude needs:
    /// the amount is pinned at its evaluation moment and the verb reads that
    /// register ([CR#608.2h]).
    #[test]
    fn lowers_one_shot_effect_act() {
        let lowered =
            in_spell_region(|| deckmaste_semantics::OneShotEffect::Act(minimal_action()).lower());
        assert!(is_minimal_lowered_effect(&lowered));
    }

    /// Runtime-produced amounts are declarations in the same region as their
    /// later anaphoric read. The flip writes register 3; the following life
    /// change snapshots that register into its own amount definition.
    #[test]
    fn a_coin_tally_defines_the_register_that_many_reads() {
        let lowered = sem::SpellAbility {
            ability_word: None,
            effect: sem::OneShotEffect::Sequentially(
                [
                    sem::OneShotEffect::Act(sem::Action::FlipCoins(
                        sem::Reference::You,
                        sem::Count::Literal(3),
                        false,
                    )),
                    sem::OneShotEffect::Act(sem::Action::ChangeLife(
                        sem::Reference::You,
                        sem::LifeOp::Up(sem::Count::ThatMany),
                    )),
                ]
                .into(),
            ),
        }
        .lower();

        let [
            deckmaste_core::Instruction::Act {
                dest: Some(tally),
                action: deckmaste_core::Action::FlipCoins(..),
            },
            deckmaste_core::Instruction::Let(deckmaste_core::Let {
                expr: deckmaste_core::Expr::Number(deckmaste_core::Count::Reg(read)),
                ..
            }),
            deckmaste_core::Instruction::Act {
                action: deckmaste_core::Action::ChangeLife(..),
                ..
            },
        ] = lowered.effect.body.as_ref()
        else {
            panic!("flip then life gain lowers through a numeric register")
        };
        assert_eq!(*tally, deckmaste_core::DefId(3));
        assert_eq!(*read, (*tally).into());
        deckmaste_core::validate(&lowered.effect).expect("runtime magnitude is well typed");
    }

    #[test]
    fn dice_and_discard_tallies_lower_as_number_producers() {
        let actions = [
            sem::Action::RollDice(sem::Reference::You, sem::Count::Literal(2), 6),
            sem::Action::discard(sem::Reference::You, sem::Count::Literal(2), true),
        ];
        for action in actions {
            let lowered = sem::SpellAbility {
                ability_word: None,
                effect: sem::OneShotEffect::Act(action),
            }
            .lower();
            assert!(
                matches!(
                    lowered.effect.body.as_ref(),
                    [deckmaste_core::Instruction::Act {
                        dest: Some(deckmaste_core::DefId(3)),
                        ..
                    }]
                ),
                "runtime action did not lower as one number producer: {:?}",
                lowered.effect.body
            );
            deckmaste_core::validate(&lowered.effect)
                .expect("runtime magnitude destination is a number definition");
        }
    }

    #[test]
    fn lowers_one_shot_effect_batch() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::Batch(
                minimal_count(),
                std::sync::Arc::new(minimal_one_shot_effect()),
            )
            .lower()
        });
        let deckmaste_core::Instruction::Batch(count, body) = &lowered else {
            panic!("a semantic Batch lowers to a core Batch");
        };
        assert_eq!(
            *count,
            deckmaste_core::Count::Literal(0),
            "the batch preserves its count"
        );
        assert!(is_minimal_lowered_effect(body));
    }

    #[test]
    fn lowers_one_shot_effect_repeat() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::Repeat(
                minimal_count(),
                std::sync::Arc::new(minimal_one_shot_effect()),
            )
            .lower()
        });
        let deckmaste_core::Instruction::Repeat(count, body) = &lowered else {
            panic!("a semantic Repeat lowers to a core Repeat");
        };
        assert_eq!(*count, deckmaste_core::Count::Literal(0));
        assert!(is_minimal_lowered_effect(body));
    }

    /// A macro invocation's provenance does not cross `lower`: the expansion
    /// wrapper erases to its value.
    #[test]
    fn lowers_one_shot_effect_expanded() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_one_shot_effect()),
            })
            .lower()
        });
        assert!(is_minimal_lowered_effect(&lowered));
    }

    #[test]
    fn lowers_one_shot_effect_if() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::OneShotEffect::If(minimal_if()).lower()),
            deckmaste_core::Instruction::If(deckmaste_core::If {
                condition: deckmaste_core::Condition::Compare(
                    deckmaste_core::Count::Literal(0),
                    deckmaste_core::Cmp::Eq,
                    deckmaste_core::Count::Literal(0)
                ),
                then: _,
                otherwise: None
            })
        );
    }

    #[test]
    fn lowers_if() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::If {
                condition: minimal_condition(),
                then: std::sync::Arc::new(minimal_one_shot_effect()),
                otherwise: None
            }
            .lower()),
            deckmaste_core::If {
                condition: deckmaste_core::Condition::Compare(
                    deckmaste_core::Count::Literal(0),
                    deckmaste_core::Cmp::Eq,
                    deckmaste_core::Count::Literal(0)
                ),
                then: _,
                otherwise: None
            }
        );
    }

    #[test]
    fn lowers_one_shot_effect_may() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::OneShotEffect::May(minimal_may()).lower()),
            deckmaste_core::Instruction::May(deckmaste_core::May {
                who: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                effect: _,
                if_did: None,
                if_not: None
            })
        );
    }

    #[test]
    fn lowers_may() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::May {
                who: minimal_reference(),
                effect: std::sync::Arc::new(minimal_one_shot_effect()),
                if_did: None,
                if_not: None
            }
            .lower()),
            deckmaste_core::May {
                who: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                effect: _,
                if_did: None,
                if_not: None
            }
        );
    }

    /// `Targeted` is an ANNOUNCEMENT declaration, not an instruction
    /// ([CR#601.2b,601.2c]): its slots become the ability's declared targets
    /// and the node itself erases to its body. Re-spelled from
    /// `lowers_targeted` / `lowers_one_shot_effect_targeted` — core deletes
    /// `OneShotEffect::Targeted`.
    #[test]
    fn lowers_one_shot_effect_targeted() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::Targeted(minimal_targeted()).lower()
        });
        assert!(is_minimal_lowered_effect(&lowered));
    }

    #[test]
    fn lowers_targeted() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::Targeted {
                targets: [].into(),
                effect: std::sync::Arc::new(minimal_one_shot_effect()),
            }
            .lower()
        });
        assert!(is_minimal_lowered_effect(&lowered));
    }

    /// An announced target reaches the ability's declaration, and the region
    /// takes it as a parameter — never as an effect node ([CR#601.2c], ADR
    /// law 9).
    #[test]
    fn a_target_slot_becomes_an_announced_region_parameter() {
        let lowered = deckmaste_semantics::SpellAbility {
            ability_word: None,
            effect: deckmaste_semantics::OneShotEffect::Targeted(deckmaste_semantics::Targeted {
                targets: [deckmaste_semantics::TargetSpec::Target(
                    minimal_quantity(),
                    deckmaste_semantics::Predicate::Any,
                )]
                .into(),
                effect: std::sync::Arc::new(minimal_one_shot_effect()),
            }),
        }
        .lower();
        assert_eq!(
            lowered.targets.len(),
            1,
            "the slot reaches the announcement"
        );
        assert_eq!(
            lowered
                .effect
                .reference_for(&deckmaste_core::Provenance::AnnouncedTarget(0)),
            Some(deckmaste_core::RefId(2)),
            "the region declares the announced slot as a parameter"
        );
    }

    /// `Label` is erased at lowering: the labelled body's instructions survive
    /// unchanged and the name becomes an alias for the register that body
    /// defined ([CR#607.2a]). Re-spelled from `lowers_label` /
    /// `lowers_one_shot_effect_label` — core deletes the `Label` node.
    #[test]
    fn lowers_label_to_its_body_and_binds_the_name_to_its_register() {
        let name: deckmaste_semantics::Ident = "X".into();
        let (_, (lowered, bound)) =
            crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
                let lowered = deckmaste_semantics::OneShotEffect::Label(minimal_label()).lower();
                let bound = crate::region::named(&"X".into());
                (lowered, bound)
            });
        let _ = name;
        assert!(is_minimal_lowered_effect(&lowered));
        assert_eq!(
            bound,
            Some(deckmaste_core::RefId(3)),
            "the label names the register its body defined"
        );
    }

    /// `Noting` is the same erasure under a memory key ([CR#607.2a]).
    /// Re-spelled from `lowers_noting` / `lowers_one_shot_effect_noting`.
    #[test]
    fn lowers_noting_to_its_body_and_binds_the_key_to_its_register() {
        let (_, (lowered, bound)) =
            crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
                let lowered = deckmaste_semantics::OneShotEffect::Noting(minimal_noting()).lower();
                let bound = crate::region::named(&"X".into());
                (lowered, bound)
            });
        assert!(is_minimal_lowered_effect(&lowered));
        assert_eq!(
            bound,
            Some(deckmaste_core::RefId(3)),
            "the note key names the register its body defined"
        );
    }

    /// A `With` binder becomes a DEFINING instruction and its body flattens
    /// into the enclosing block; nothing survives to re-resolve at runtime.
    /// Re-spelled from `lowers_with` / `lowers_one_shot_effect_with` — core
    /// deletes the `With` node and the `Binder` enum.
    #[test]
    fn lowers_with_to_a_defining_instruction_then_its_flattened_body() {
        let lowered =
            in_spell_region(|| deckmaste_semantics::OneShotEffect::With(minimal_with()).lower());
        let deckmaste_core::Instruction::Sequentially(parts) = &lowered else {
            panic!("a binder plus its body is a sequence of instructions");
        };
        assert_matches!(
            parts.as_ref(),
            [
                deckmaste_core::Instruction::Let(deckmaste_core::Let {
                    dest: deckmaste_core::DefId(3),
                    expr: deckmaste_core::Expr::Object(deckmaste_core::Reference::Reg(
                        deckmaste_core::RefId(0)
                    )),
                }),
                deckmaste_core::Instruction::Let(_),
                deckmaste_core::Instruction::Act { dest: None, .. },
            ],
            "the binder defines register 3, then the body's own instructions follow"
        );
    }

    /// `Each` is a region: the body declares its loop element as a parameter
    /// and iterates the register the binder defined. Re-spelled from
    /// `lowers_each` / `lowers_one_shot_effect_each`.
    #[test]
    fn lowers_each_to_a_loop_region_over_the_binders_register() {
        let lowered =
            in_spell_region(|| deckmaste_semantics::OneShotEffect::Each(minimal_each()).lower());
        let deckmaste_core::Instruction::Sequentially(parts) = &lowered else {
            panic!("a binder plus its loop is a sequence of instructions");
        };
        let [
            deckmaste_core::Instruction::Let(deckmaste_core::Let { dest, .. }),
            deckmaste_core::Instruction::Each(each),
        ] = parts.as_ref()
        else {
            panic!("the binder's definition precedes the loop that reads it");
        };
        assert_eq!(
            each.over,
            deckmaste_core::Selection::Reg((*dest).into()),
            "the loop iterates the register the binder defined"
        );
        assert_eq!(
            each.body.params[0],
            deckmaste_core::Param {
                def: deckmaste_core::DefId(0),
                kind: deckmaste_core::Kind::Entity,
                provenance: deckmaste_core::Provenance::LoopElement,
            },
            "the body declares its element as parameter zero"
        );
    }

    /// `Distribute` is a region with two engine-supplied parameters — the recipient
    /// and its allotted share ([CR#601.2d]). Re-spelled from
    /// `lowers_distribute` / `lowers_one_shot_effect_distribute`.
    #[test]
    fn lowers_distribute_to_a_region_with_element_and_allotment_parameters() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::Distribute(minimal_distribute()).lower()
        });
        let deckmaste_core::Instruction::Sequentially(parts) = &lowered else {
            panic!("a binder plus its distribution is a sequence of instructions");
        };
        let [
            deckmaste_core::Instruction::Let(deckmaste_core::Let { dest, .. }),
            deckmaste_core::Instruction::Distribute(distribute),
        ] = parts.as_ref()
        else {
            panic!("the binder's definition precedes the distribution that reads it");
        };
        assert_eq!(distribute.amount, deckmaste_core::Count::Literal(0));
        assert_eq!(
            distribute.over,
            deckmaste_core::Selection::Reg((*dest).into())
        );
        assert_eq!(
            distribute.body.params[0].provenance,
            deckmaste_core::Provenance::LoopElement
        );
        assert_eq!(
            distribute.body.params[1],
            deckmaste_core::Param {
                def: deckmaste_core::DefId(1),
                kind: deckmaste_core::Kind::Number,
                provenance: deckmaste_core::Provenance::Allotment,
            },
            "the share is the body's second declared parameter"
        );
    }

    /// A dig-until supplies its found card and passed-over prefix as
    /// DEFINITIONS ([CR#702.85]), and its match test is a candidate region.
    /// Re-spelled from `lowers_reveal_until` /
    /// `lowers_one_shot_effect_reveal_until`.
    #[test]
    fn lowers_reveal_until_to_definitions_plus_a_candidate_region() {
        let lowered = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::RevealUntil(minimal_reveal_until()).lower()
        });
        let deckmaste_core::Instruction::RevealUntil(reveal) = &lowered else {
            panic!("a semantic RevealUntil lowers to a core RevealUntil");
        };
        assert_eq!(
            reveal.whose,
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
        );
        assert_eq!(reveal.found, deckmaste_core::DefId(3));
        assert_eq!(reveal.passed, deckmaste_core::DefId(4));
        assert_eq!(
            reveal.matches.params[0].provenance,
            deckmaste_core::Provenance::Candidate(deckmaste_core::Domain::Object),
            "the match test is a per-candidate region over the Object domain"
        );
        assert_eq!(
            reveal.matches.body,
            deckmaste_core::Predicate::Class(deckmaste_core::ObjectClass::AbilityOnStack)
        );
    }

    /// A pile choice writes the chosen pile to a definition the nested body
    /// reads ([CR#700.3b]). Re-spelled from `lowers_choose_pile` /
    /// `lowers_one_shot_effect_choose_pile` — `ChoosePile` gained `dest`.
    #[test]
    fn lowers_one_shot_effect_choose_pile() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::OneShotEffect::ChoosePile(
                minimal_choose_pile()
            )
            .lower()),
            deckmaste_core::Instruction::ChoosePile(deckmaste_core::ChoosePile {
                dest: deckmaste_core::DefId(3),
                from: _,
                by: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                random: false,
                then: _
            })
        );
    }

    #[test]
    fn lowers_choose_pile() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::ChoosePile {
                from: minimal_pile_source(),
                by: minimal_reference(),
                random: false,
                then: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            deckmaste_core::ChoosePile {
                dest: deckmaste_core::DefId(3),
                from: _,
                by: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                random: false,
                then: _
            }
        );
    }

    /// [CR#118.8]: an additional cost is announced and paid with the spell's
    /// mana cost, so the declaration is HOISTED onto the ability rather than
    /// surviving as an instruction. Re-spelled from `lowers_additional_cost` /
    /// `lowers_one_shot_effect_additional_cost` — core deletes the node.
    #[test]
    fn lowers_additional_cost_onto_the_abilitys_announcement() {
        let lowered = deckmaste_semantics::SpellAbility {
            ability_word: None,
            effect: deckmaste_semantics::OneShotEffect::AdditionalCost(
                deckmaste_semantics::AdditionalCost {
                    pay: deckmaste_semantics::Cost(
                        [deckmaste_semantics::CostComponent::do_action(
                            deckmaste_semantics::Action::Sacrifice(
                                deckmaste_semantics::Reference::You,
                                deckmaste_semantics::Reference::This,
                            ),
                        )]
                        .into(),
                    ),
                    body: std::sync::Arc::new(minimal_one_shot_effect()),
                },
            ),
        }
        .lower();
        assert_eq!(
            lowered.cost.0.len(),
            1,
            "the declared additional cost reaches the ability's announcement"
        );
        assert!(is_minimal_lowered_effect(&super::one(
            lowered.effect.body.to_vec()
        )));
    }

    /// A nested additional cost has no announcement to be paid at
    /// ([CR#118.8a]); a payment made while a spell resolves is `May`
    /// ([CR#118.12]).
    #[test]
    #[should_panic(expected = "nested AdditionalCost")]
    fn a_nested_additional_cost_is_a_lowering_error() {
        let _ = in_spell_region(|| {
            deckmaste_semantics::OneShotEffect::May(deckmaste_semantics::May {
                who: minimal_reference(),
                effect: std::sync::Arc::new(deckmaste_semantics::OneShotEffect::AdditionalCost(
                    minimal_additional_cost(),
                )),
                if_did: None,
                if_not: None,
            })
            .lower()
        });
    }

    // ---- The named fixtures the discourse stage's ticket required ----
    //
    // A `Spell` region with one announced target declares source(0),
    // controller(1), the target(2) and announced X(3), so its first
    // instruction definition is register 4.

    use deckmaste_semantics as sem;

    /// The first instruction definition of a one-target spell region.
    const FIRST_TARGETED_DEF: deckmaste_core::DefId = deckmaste_core::DefId(4);
    /// The announced target of a one-target spell region.
    const TARGET_0: deckmaste_core::RefId = deckmaste_core::RefId(2);

    /// One target slot admitting any creature.
    fn one_creature_target() -> sem::TargetSpec {
        sem::TargetSpec::Target(
            sem::Quantity::one(),
            sem::Predicate::r#type(sem::Type::Creature),
        )
    }

    /// Lower `effect` as a spell ability with one announced creature target.
    fn lower_targeted_spell(effect: sem::OneShotEffect) -> deckmaste_core::SpellAbility {
        sem::SpellAbility {
            ability_word: None,
            effect: sem::OneShotEffect::Targeted(sem::Targeted {
                targets: [one_creature_target()].into(),
                effect: std::sync::Arc::new(effect),
            }),
        }
        .lower()
    }

    fn move_to(what: sem::Reference, zone: sem::Zone) -> sem::OneShotEffect {
        sem::OneShotEffect::Act(sem::Action::Move(
            what,
            sem::Destination::Zone(zone),
            [].into(),
            None,
        ))
    }

    /// FIXTURE — exile and return. "Exile target creature, then return it to
    /// the battlefield under its owner's control" ([CR#400.7,400.7j]): the
    /// exile is a PRODUCING instruction whose destination the return reads, so
    /// the second clause names the new object rather than re-reading the
    /// announced slot (which is now a departed target).
    #[test]
    fn exile_and_return_reads_the_exiles_product_definition() {
        let lowered = lower_targeted_spell(sem::OneShotEffect::Sequentially(
            [
                move_to(sem::Reference::Target(0), sem::Zone::Exile),
                move_to(
                    sem::Reference::That(sem::Sort::Card),
                    sem::Zone::Battlefield,
                ),
            ]
            .into(),
        ));
        let [
            deckmaste_core::Instruction::Act {
                dest: Some(product),
                action: exile,
            },
            deckmaste_core::Instruction::Act {
                dest: Some(_),
                action: ret,
            },
        ] = lowered.effect.body.as_ref()
        else {
            panic!(
                "two producing move instructions, got {:?}",
                lowered.effect.body
            );
        };
        assert_eq!(
            *product, FIRST_TARGETED_DEF,
            "the exile defines the region's first instruction product"
        );
        assert_eq!(
            *exile,
            deckmaste_core::Action::Move(
                deckmaste_core::Reference::Reg(TARGET_0),
                deckmaste_core::Destination::Zone(deckmaste_core::Zone::Exile),
                [].into(),
                None,
            ),
            "the exile reads the announced slot"
        );
        assert_eq!(
            *ret,
            deckmaste_core::Action::Move(
                deckmaste_core::Reference::Reg((*product).into()),
                deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield),
                [].into(),
                None,
            ),
            "the return reads the exile's product, not the announced slot"
        );
    }

    /// FIXTURE — the insertion negative. Inserting a producing clause between a
    /// binder and its mention RE-RESOLVES the mention at lowering; it never
    /// silently keeps the old register. With two equally compatible products in
    /// scope the resolver refuses (R2) rather than picking one, so the change
    /// is loud at compile time instead of a wrong object at resolution.
    #[test]
    fn a_clause_inserted_between_binder_and_mention_re_resolves() {
        // Sanity: without the insertion the mention resolves — see
        // `exile_and_return_reads_the_exiles_product_definition`.
        let error = crate::lower_for_test(|| {
            lower_targeted_spell(sem::OneShotEffect::Sequentially(
                [
                    move_to(sem::Reference::Target(0), sem::Zone::Exile),
                    // The inserted producing clause: it defines a second card-sorted
                    // product between the binder and its mention.
                    move_to(sem::Reference::This, sem::Zone::Graveyard),
                    move_to(
                        sem::Reference::That(sem::Sort::Card),
                        sem::Zone::Battlefield,
                    ),
                ]
                .into(),
            ))
        })
        .expect_err("two compatible products are ambiguous");
        assert_eq!(&*error.card, "Lowering Test");
        assert!(
            error.message.contains("ambiguous discourse anaphor"),
            "the returned diagnostic carries the refusal, got {:?}",
            error.message
        );
    }

    /// FIXTURE — a multi-sentence anaphora card. "Create a Treasure token.
    /// Sacrifice it." — the second sentence's anaphor reads the register the
    /// first sentence's product defined ([CR#400.7j]), across the sentence
    /// boundary and with no name to carry it.
    #[test]
    fn a_multi_sentence_anaphor_reads_the_earlier_sentences_product() {
        let lowered = sem::SpellAbility {
            ability_word: None,
            effect: sem::OneShotEffect::Sequentially(
                [
                    sem::OneShotEffect::Act(sem::Action::Create {
                        agent: sem::Reference::You,
                        count: sem::Count::Literal(1),
                        token: sem::TokenSpec::Named(sem::TokenName::from("Treasure")),
                        riders: [].into(),
                    }),
                    sem::OneShotEffect::Act(sem::Action::Sacrifice(
                        sem::Reference::You,
                        sem::Reference::That(sem::Sort::Token),
                    )),
                ]
                .into(),
            ),
        }
        .lower();
        let [
            deckmaste_core::Instruction::Act {
                dest: Some(token),
                action: deckmaste_core::Action::Create { .. },
            },
            deckmaste_core::Instruction::Act {
                dest: _,
                action: deckmaste_core::Action::Sacrifice(_, sacrificed),
            },
        ] = lowered.effect.body.as_ref()
        else {
            panic!(
                "a creating instruction then a sacrifice, got {:?}",
                lowered.effect.body
            );
        };
        assert_eq!(
            *sacrificed,
            deckmaste_core::Reference::Reg((*token).into()),
            "the second sentence sacrifices the register the first defined"
        );
    }

    /// Two damage clauses, then a life gain reading one of them.
    fn two_magnitudes_then(gain: sem::Count, label: Option<&str>) -> sem::SpellAbility {
        let damage = |amount| {
            sem::OneShotEffect::Act(sem::Action::DealDamage(
                sem::Reference::This,
                sem::Count::Literal(amount),
                sem::Reference::Opponent,
            ))
        };
        let first = match label {
            Some(name) => sem::OneShotEffect::Label(sem::Label {
                r#as: name.into(),
                effect: std::sync::Arc::new(damage(2)),
            }),
            None => damage(2),
        };
        sem::SpellAbility {
            ability_word: None,
            effect: sem::OneShotEffect::Sequentially(
                [
                    first,
                    damage(5),
                    sem::OneShotEffect::Act(sem::Action::ChangeLife(
                        sem::Reference::You,
                        sem::LifeOp::Up(gain),
                    )),
                ]
                .into(),
            ),
        }
    }

    /// Every numeric definition a lowered spell's block pins, in order.
    fn numeric_pins(ability: &deckmaste_core::SpellAbility) -> Vec<deckmaste_core::DefId> {
        ability
            .effect
            .body
            .iter()
            .filter_map(|instruction| match instruction {
                deckmaste_core::Instruction::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Number(_),
                }) => Some(*dest),
                _ => None,
            })
            .collect()
    }

    /// FIXTURE — a two-magnitude card, half one. Each clause pins its own
    /// magnitude at its own evaluation moment ([CR#608.2h]), so the two amounts
    /// occupy two registers rather than one shared slot the second clause would
    /// overwrite. That alone makes the wrong-magnitude bug unrepresentable.
    #[test]
    fn a_two_magnitude_card_pins_each_magnitude_separately() {
        use sem::Count as SemValue;

        let lowered = two_magnitudes_then(SemValue::Noted("first".into()), Some("first")).lower();
        let pins = numeric_pins(&lowered);
        assert_eq!(
            pins.len(),
            3,
            "two damage amounts and the life gain each get their own definition"
        );
        assert_eq!(
            pins.iter().collect::<std::collections::BTreeSet<_>>().len(),
            3,
            "three DISTINCT registers — no shared magnitude slot"
        );
    }

    /// FIXTURE — a two-magnitude card, half two. A bare "that much" after TWO
    /// magnitudes pinned by the SAME region is refused at lowering (R2): the
    /// resolver never guesses which one the card meant, so the ambiguity is a
    /// compile-time error rather than a wrong amount at resolution.
    ///
    /// The rules carry no proximity tiebreak to appeal to. [CR#608.2c] is the
    /// only rule governing how a later clause reads an earlier one, and it
    /// refuses a positional scan outright — "Don't just apply effects step by
    /// step without thinking in these cases—read the whole text and apply the
    /// rules of English to the text." What the CR does supply is linkage,
    /// [CR#607.1]: the second ability "refers only to actions that were taken
    /// or objects or players that were affected by the first, and not by any
    /// other ability" — identity, never position. Naming the magnitude is
    /// therefore the escape, exercised by
    /// `a_named_magnitude_reads_the_clause_that_bound_it` below; scope is the
    /// other, exercised by
    /// `that_much_in_a_loop_body_reads_the_bodys_own_magnitude`.
    #[test]
    fn a_bare_that_much_after_two_magnitudes_is_refused() {
        let error =
            crate::lower_for_test(|| two_magnitudes_then(sem::Count::ThatMuch, None).lower())
                .expect_err("two compatible magnitudes are ambiguous");
        assert_eq!(&*error.card, "Lowering Test");
        assert!(
            error.message.contains("ambiguous discourse anaphor"),
            "the returned diagnostic carries the refusal, got {:?}",
            error.message
        );
    }

    /// FIXTURE — a two-magnitude card, half four: the loop body's own
    /// magnitude. A region is applied once per entry and determines its
    /// information then ([CR#608.2h]), and each affected player is processed
    /// individually ([CR#608.2f]), so the per-element amount a loop body pins
    /// is a DIFFERENT value from the enclosing region's — not a competing
    /// antecedent at the same discourse level. The outer magnitude survives
    /// inside the body only as a declared capture (ADR law 7), so "that much"
    /// in the body reads the body's own amount instead of being refused.
    ///
    /// This is the second clause of the absorbed
    /// `engine-that-much-frame-scoped` ticket ("an `Each` over players
    /// followed by 'that much' reads the per-element amount"); before the
    /// discourse tier it panicked as R2.
    #[test]
    fn that_much_in_a_loop_body_reads_the_bodys_own_magnitude() {
        let lowered = sem::SpellAbility {
            ability_word: None,
            effect: sem::OneShotEffect::Sequentially(
                [
                    // The enclosing region pins a magnitude of its own.
                    sem::OneShotEffect::Act(sem::Action::DealDamage(
                        sem::Reference::This,
                        sem::Count::Literal(2),
                        sem::Reference::Opponent,
                    )),
                    sem::OneShotEffect::Each(sem::Each {
                        binder: sem::Binder::Existing(sem::Selection::SelectAll(
                            sem::Predicate::Kind(sem::ObjectKind::Player),
                        )),
                        effect: std::sync::Arc::new(sem::OneShotEffect::Sequentially(
                            [
                                sem::OneShotEffect::Act(sem::Action::ChangeLife(
                                    sem::Reference::That(sem::Sort::Player),
                                    sem::LifeOp::Down(sem::Count::Literal(3)),
                                )),
                                sem::OneShotEffect::Act(sem::Action::ChangeLife(
                                    sem::Reference::You,
                                    sem::LifeOp::Up(sem::Count::ThatMuch),
                                )),
                            ]
                            .into(),
                        )),
                    }),
                ]
                .into(),
            ),
        }
        .lower();
        let Some(deckmaste_core::Instruction::Each(each)) = lowered
            .effect
            .body
            .iter()
            .find(|instruction| matches!(instruction, deckmaste_core::Instruction::Each(_)))
        else {
            panic!("the loop lowers to an Each, got {:?}", lowered.effect.body);
        };
        // The body pins its own loss amount, then gains exactly that register.
        let pins: Vec<deckmaste_core::DefId> = each
            .body
            .body
            .iter()
            .filter_map(|instruction| match instruction {
                deckmaste_core::Instruction::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Number(_),
                }) => Some(*dest),
                _ => None,
            })
            .collect();
        assert_eq!(pins.len(), 2, "the loss and the gain each pin an amount");
        let gain = each
            .body
            .body
            .iter()
            .find_map(|instruction| match instruction {
                deckmaste_core::Instruction::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Number(count),
                }) if *dest == pins[1] => Some(count),
                _ => None,
            })
            .expect("the gain pins its amount");
        assert_eq!(
            *gain,
            deckmaste_core::Count::Reg(pins[0].into()),
            "\"that much\" reads the loop body's own per-element amount, not the \
             captured magnitude the enclosing region pinned"
        );
    }

    /// FIXTURE — a two-magnitude card, half three. A card that NAMES the
    /// magnitude it means reads exactly that register ([CR#607.2a]) — the
    /// escape the refusal above leaves open. The life gain pins its own amount
    /// at its own moment ([CR#608.2h]), and the expression it pins is the
    /// named clause's register.
    #[test]
    fn a_named_magnitude_reads_the_clause_that_bound_it() {
        use sem::Count as SemValue;

        let lowered = two_magnitudes_then(SemValue::Noted("first".into()), Some("first")).lower();
        let pins = numeric_pins(&lowered);
        let pinned: Vec<&deckmaste_core::Count> = lowered
            .effect
            .body
            .iter()
            .filter_map(|instruction| match instruction {
                deckmaste_core::Instruction::Let(deckmaste_core::Let {
                    expr: deckmaste_core::Expr::Number(count),
                    ..
                }) => Some(count),
                _ => None,
            })
            .collect();
        assert_eq!(pinned.len(), 3);
        assert_eq!(
            *pinned[2],
            deckmaste_core::Count::Reg(pins[0].into()),
            "the life gain pins the FIRST clause's register, not the nearer second one"
        );
    }
}

pub(crate) fn lower_block(effect: deckmaste_semantics::OneShotEffect) -> deckmaste_core::Block {
    deckmaste_core::Block(lower_instructions(effect).into())
}

fn lower_arc(
    effect: std::sync::Arc<deckmaste_semantics::OneShotEffect>,
) -> std::sync::Arc<Instruction> {
    std::sync::Arc::new(one(lower_instructions(std::sync::Arc::unwrap_or_clone(
        effect,
    ))))
}

fn lower_optional_arc(
    effect: Option<std::sync::Arc<deckmaste_semantics::OneShotEffect>>,
) -> Option<std::sync::Arc<Instruction>> {
    effect.map(lower_arc)
}

fn each_over_they(
    effect: &deckmaste_semantics::OneShotEffect,
) -> Option<&deckmaste_semantics::Each> {
    match effect {
        deckmaste_semantics::OneShotEffect::Each(each)
            if matches!(
                each.binder,
                deckmaste_semantics::Binder::Existing(deckmaste_semantics::Selection::They)
            ) =>
        {
            Some(each)
        }
        deckmaste_semantics::OneShotEffect::Expanded(expanded) => each_over_they(&expanded.value),
        _ => None,
    }
}

fn lower_existing_each(
    over: deckmaste_core::Selection,
    effect: std::sync::Arc<deckmaste_semantics::OneShotEffect>,
) -> Instruction {
    let (params, body) = crate::region::in_child(
        [(
            deckmaste_core::Kind::Entity,
            deckmaste_core::Provenance::LoopElement,
        )],
        || {
            crate::region::push_antecedent(
                deckmaste_core::RefId(0),
                deckmaste_core::Kind::Entity,
                crate::region::Cardinality::One,
                None,
                crate::region::Site::Loop,
            );
            lower_block(std::sync::Arc::unwrap_or_clone(effect))
        },
    );
    Instruction::Each(deckmaste_core::Each {
        over,
        body: deckmaste_core::Region::new(params, body),
    })
}

fn destination_sort(destination: &deckmaste_core::Destination) -> deckmaste_semantics::Sort {
    match destination {
        deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield) => {
            deckmaste_semantics::Sort::Permanent
        }
        deckmaste_core::Destination::Zone(deckmaste_core::Zone::Stack) => {
            deckmaste_semantics::Sort::Spell
        }
        _ => deckmaste_semantics::Sort::Card,
    }
}

fn lower_action(action: deckmaste_semantics::Action) -> Vec<Instruction> {
    if let deckmaste_semantics::Action::ChooseValue(who, domain, note) = action {
        let by = who.lower();
        let domain = domain.lower();
        let kind = match domain {
            deckmaste_core::ChosenValueKind::Number => deckmaste_core::Kind::Number,
            deckmaste_core::ChosenValueKind::CardName | deckmaste_core::ChosenValueKind::Color => {
                deckmaste_core::Kind::Symbol
            }
        };
        let dest = crate::region::define(kind);
        // [CR#607.2d]: "the chosen [value]" read by another ability on the card
        // is a linked read, so the choice publishes a memory cell too.
        let published = crate::region::cell_write(&note, kind);
        crate::region::bind_named(note, dest.into());
        let mut instructions = vec![Instruction::ChooseValue(deckmaste_core::ChooseValue {
            dest,
            by,
            domain,
        })];
        if published {
            instructions.push(Instruction::Remember(deckmaste_core::Remember {
                cell: note.lower(),
                kind,
                value: dest.into(),
            }));
        }
        return instructions;
    }

    let action = action.lower();
    match &action {
        deckmaste_core::Action::Move(_, destination, _, _) => {
            let dest = crate::region::define(deckmaste_core::Kind::Entity);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Entity,
                crate::region::Cardinality::One,
                Some(destination_sort(destination)),
                crate::region::Site::Product,
            );
            vec![Instruction::producing(dest, action)]
        }
        deckmaste_core::Action::MoveGroup { to, .. } => {
            let dest = crate::region::define(deckmaste_core::Kind::Entities);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Entities,
                crate::region::Cardinality::Many,
                Some(destination_sort(to)),
                crate::region::Site::Product,
            );
            vec![Instruction::producing(dest, action)]
        }
        deckmaste_core::Action::Create { .. } => {
            let dest = crate::region::define(deckmaste_core::Kind::Entities);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Entities,
                crate::region::Cardinality::Many,
                Some(deckmaste_semantics::Sort::Token),
                crate::region::Site::Product,
            );
            vec![Instruction::producing(dest, action)]
        }
        deckmaste_core::Action::DrawCard(_) => {
            let dest = crate::region::define(deckmaste_core::Kind::Entity);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Entity,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                crate::region::Site::Product,
            );
            let amount = crate::region::define(deckmaste_core::Kind::Number);
            crate::region::push_antecedent(
                amount.into(),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Product,
            );
            vec![
                Instruction::producing(dest, action),
                Instruction::Let(deckmaste_core::Let {
                    dest: amount,
                    expr: deckmaste_core::Expr::Number(deckmaste_core::Count::Literal(1)),
                }),
            ]
        }
        deckmaste_core::Action::DealDamage(source, amount, target) => {
            let dest = crate::region::define(deckmaste_core::Kind::Number);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Product,
            );
            vec![
                Instruction::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Number(amount.clone()),
                }),
                Instruction::act(deckmaste_core::Action::DealDamage(
                    source.clone(),
                    deckmaste_core::Count::Reg(dest.into()),
                    target.clone(),
                )),
            ]
        }
        deckmaste_core::Action::ChangeLife(patient, op) => {
            use deckmaste_core::LifeOp;
            let (amount, rebuild): (_, fn(deckmaste_core::Count) -> LifeOp) = match op {
                LifeOp::Up(amount) => (amount, LifeOp::Up),
                LifeOp::Down(amount) => (amount, LifeOp::Down),
                LifeOp::Set(_) => return vec![Instruction::act(action)],
            };
            let dest = crate::region::define(deckmaste_core::Kind::Number);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Product,
            );
            vec![
                Instruction::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Number(amount.clone()),
                }),
                Instruction::act(deckmaste_core::Action::ChangeLife(
                    patient.clone(),
                    rebuild(deckmaste_core::Count::Reg(dest.into())),
                )),
            ]
        }
        _ if action.produces_runtime_magnitude() => {
            let dest = crate::region::define(deckmaste_core::Kind::Number);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Number,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Amount),
                crate::region::Site::Product,
            );
            vec![Instruction::producing(dest, action)]
        }
        _ => vec![Instruction::act(action)],
    }
}

pub(crate) struct BoundValue {
    pub(crate) reference: deckmaste_core::RefId,
    pub(crate) kind: deckmaste_core::Kind,
    pub(crate) cardinality: crate::region::Cardinality,
    pub(crate) sort: Option<deckmaste_semantics::Sort>,
}

fn predicate_sort(predicate: &deckmaste_semantics::Predicate) -> Option<deckmaste_semantics::Sort> {
    use deckmaste_semantics::CharacteristicPredicate;
    use deckmaste_semantics::ObjectKind;
    use deckmaste_semantics::Predicate;
    use deckmaste_semantics::Sort;
    use deckmaste_semantics::Type;

    match predicate {
        Predicate::Kind(kind) => match kind {
            ObjectKind::Player => Some(Sort::Player),
            ObjectKind::Spell => Some(Sort::Spell),
            ObjectKind::Ability => Some(Sort::StackObject),
            ObjectKind::Card | ObjectKind::CardCopy => Some(Sort::Card),
            ObjectKind::Token => Some(Sort::Token),
            ObjectKind::Emblem => None,
        },
        Predicate::Characteristic(CharacteristicPredicate::Type(kind)) => {
            let kind = match kind.as_str() {
                "Artifact" => Type::Artifact,
                "Battle" => Type::Battle,
                "Creature" => Type::Creature,
                "Dungeon" => Type::Dungeon,
                "Enchantment" => Type::Enchantment,
                "Instant" => Type::Instant,
                "Kindred" => Type::Kindred,
                "Land" => Type::Land,
                "Planeswalker" => Type::Planeswalker,
                "Sorcery" => Type::Sorcery,
                _ => return None,
            };
            Some(Sort::OfType(kind))
        }
        Predicate::And(parts) => {
            parts
                .iter()
                .filter_map(predicate_sort)
                .min_by_key(|sort| match sort {
                    Sort::Token => 0,
                    Sort::OfType(_) => 1,
                    Sort::Spell | Sort::StackObject | Sort::Player | Sort::Card => 2,
                    Sort::Permanent | Sort::Amount | Sort::Pile => 3,
                })
        }
        Predicate::Expanded(expanded) => predicate_sort(&expanded.value),
        _ => None,
    }
}

pub(crate) fn lower_binder(binder: deckmaste_semantics::Binder) -> (Vec<Instruction>, BoundValue) {
    use deckmaste_semantics::Binder;
    match binder {
        Binder::TheRef(reference) => {
            let reference = reference.lower();
            let dest = crate::region::define(deckmaste_core::Kind::Entity);
            (
                vec![Instruction::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Object(reference),
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Entity,
                    cardinality: crate::region::Cardinality::One,
                    sort: None,
                },
            )
        }
        Binder::ChooseOne { filter, by } => {
            let sort = predicate_sort(&filter);
            let by = by.lower();
            let filter = std::sync::Arc::new(crate::region::predicate_region(|| filter.lower()));
            let dest = crate::region::define(deckmaste_core::Kind::Entities);
            (
                vec![Instruction::Choose(deckmaste_core::Choose {
                    dest,
                    by,
                    quantity: deckmaste_core::Quantity::one(),
                    filter,
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Entities,
                    cardinality: crate::region::Cardinality::One,
                    sort,
                },
            )
        }
        Binder::Choose {
            quantity,
            filter,
            by,
        } => {
            let by = by.lower();
            let quantity = quantity.lower();
            let filter = std::sync::Arc::new(crate::region::predicate_region(|| filter.lower()));
            let dest = crate::region::define(deckmaste_core::Kind::Entities);
            (
                vec![Instruction::Choose(deckmaste_core::Choose {
                    dest,
                    by,
                    quantity,
                    filter,
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Entities,
                    cardinality: crate::region::Cardinality::Many,
                    sort: None,
                },
            )
        }
        Binder::Existing(selection) => {
            let selection = selection.lower();
            let dest = crate::region::define(deckmaste_core::Kind::Entities);
            (
                vec![Instruction::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Objects(selection),
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Entities,
                    cardinality: crate::region::Cardinality::Many,
                    sort: None,
                },
            )
        }
        Binder::Produce(action) => {
            let action = std::sync::Arc::unwrap_or_clone(action).lower();
            let (kind, cardinality, sort) = match &action {
                deckmaste_core::Action::Move(_, to, _, _) => (
                    deckmaste_core::Kind::Entity,
                    crate::region::Cardinality::One,
                    Some(destination_sort(to)),
                ),
                deckmaste_core::Action::MoveGroup { to, .. } => (
                    deckmaste_core::Kind::Entities,
                    crate::region::Cardinality::Many,
                    Some(destination_sort(to)),
                ),
                deckmaste_core::Action::Create { .. } => (
                    deckmaste_core::Kind::Entities,
                    crate::region::Cardinality::Many,
                    Some(deckmaste_semantics::Sort::Token),
                ),
                _ => (
                    deckmaste_core::Kind::Entity,
                    crate::region::Cardinality::One,
                    None,
                ),
            };
            let dest = crate::region::define(kind);
            (
                vec![Instruction::producing(dest, action)],
                BoundValue {
                    reference: dest.into(),
                    kind,
                    cardinality,
                    sort,
                },
            )
        }
        Binder::SearchOne {
            filter,
            by,
            whose,
            from,
            if_none,
        } => {
            let (instructions, mut bound) = lower_search(
                by,
                whose,
                from,
                deckmaste_core::Quantity::one(),
                filter,
                if_none,
            );
            bound.cardinality = crate::region::Cardinality::One;
            (instructions, bound)
        }
        Binder::Search {
            quantity,
            filter,
            by,
            whose,
            from,
            if_none,
        } => {
            let quantity = quantity.lower();
            lower_search(by, whose, from, quantity, filter, if_none)
        }
        Binder::Expanded(expanded) => lower_binder(*expanded.value),
    }
}

fn lower_search(
    by: deckmaste_semantics::Reference,
    whose: deckmaste_semantics::Reference,
    from: std::sync::Arc<[deckmaste_semantics::Zone]>,
    quantity: deckmaste_core::Quantity,
    filter: deckmaste_semantics::Predicate,
    if_none: Option<std::sync::Arc<deckmaste_semantics::OneShotEffect>>,
) -> (Vec<Instruction>, BoundValue) {
    let by = by.lower();
    let whose = whose.lower();
    let from = from.lower();
    let filter = std::sync::Arc::new(crate::region::predicate_region(|| filter.lower()));
    let if_none = crate::region::scoped_antecedents(|| {
        if_none.map_or_else(deckmaste_core::Block::default, |effect| {
            lower_block(std::sync::Arc::unwrap_or_clone(effect))
        })
    });
    let dest = crate::region::define(deckmaste_core::Kind::Entities);
    (
        vec![Instruction::Search(deckmaste_core::Search {
            dest,
            by,
            whose,
            from,
            quantity,
            filter,
            if_none,
        })],
        BoundValue {
            reference: dest.into(),
            kind: deckmaste_core::Kind::Entities,
            cardinality: crate::region::Cardinality::Many,
            sort: Some(deckmaste_semantics::Sort::Card),
        },
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "the exhaustive semantic-to-core instruction dispatch is clearer as one match"
)]
fn lower_instructions(effect: deckmaste_semantics::OneShotEffect) -> Vec<Instruction> {
    use deckmaste_semantics::OneShotEffect as S;
    match effect {
        S::Act(action) => lower_action(action),
        S::Sequentially(parts) => parts.iter().cloned().flat_map(lower_instructions).collect(),
        S::Simultaneously(parts) => vec![Instruction::Simultaneously(
            parts
                .iter()
                .map(|part| {
                    one(crate::region::scoped_antecedents(|| {
                        lower_instructions(part.clone())
                    }))
                })
                .collect::<Vec<_>>()
                .into(),
        )],
        S::Continuously(value) => vec![Instruction::Continuously(value.lower())],
        S::Until(duration, effects) => vec![Instruction::Until(duration.lower(), effects.lower())],
        S::Label(label) => {
            let instructions = lower_instructions(std::sync::Arc::unwrap_or_clone(label.effect));
            if let Some(reference) = crate::region::newest_antecedent() {
                crate::region::bind_named(label.r#as, reference);
            }
            instructions
        }
        // [CR#607.1,607.2a]: "note the cards exiled this way" — the value is a
        // register of this region for the SAME ability's later clauses, and,
        // when another ability on the card reads the cell, also a `Remember`
        // that publishes it as the card's linked memory (ADR law 8).
        S::Noting(noting) => {
            let mut instructions =
                lower_instructions(std::sync::Arc::unwrap_or_clone(noting.effect));
            if let Some((reference, kind)) = crate::region::newest_antecedent_typed() {
                if crate::region::cell_write(&noting.key, kind) {
                    instructions.push(Instruction::Remember(deckmaste_core::Remember {
                        cell: noting.key.lower(),
                        kind,
                        value: reference,
                    }));
                }
                crate::region::bind_named(noting.key, reference);
            }
            instructions
        }
        S::With(with) => {
            if let deckmaste_semantics::Binder::Existing(
                selection @ deckmaste_semantics::Selection::Random(..),
            ) = &with.binder
                && let Some(each) = each_over_they(&with.body)
            {
                // A random group is itself a decision, so it cannot be pinned
                // by `Let`. The at-random discard idiom immediately iterates
                // that group; fuse the semantic `With` + `Each(They)` into the
                // core iterator that owns random sampling.
                return vec![lower_existing_each(
                    selection.clone().lower(),
                    each.effect.clone(),
                )];
            }
            let (mut instructions, bound) = lower_binder(with.binder);
            crate::region::push_antecedent(
                bound.reference,
                bound.kind,
                bound.cardinality,
                bound.sort,
                crate::region::Site::ExecutionFrame,
            );
            instructions.extend(lower_instructions(std::sync::Arc::unwrap_or_clone(
                with.body,
            )));
            crate::region::remove_antecedent(bound.reference, crate::region::Site::ExecutionFrame);
            instructions
        }
        // [CR#118.8]: an additional cost is announced and paid with the
        // spell's mana cost or the ability's activation cost — there is no
        // resolution-time additional cost, so this node is a DECLARATION that
        // `peel_announcement` hoists onto the ability. Reaching the
        // instruction lowerer means it was nested under another instruction,
        // where no announcement exists to carry it.
        S::AdditionalCost(_) => unreachable!(
            "a nested AdditionalCost has no announcement to be paid at ([CR#118.8,118.8a]); \
             a payment made while a spell or ability resolves is May ([CR#118.12])"
        ),
        S::Each(each) => {
            if let deckmaste_semantics::Binder::Existing(selection) = &each.binder {
                let over = selection.clone().lower();
                return vec![lower_existing_each(over, each.effect)];
            }
            let (mut setup, bound) = lower_binder(each.binder);
            let over = deckmaste_core::Selection::Reg(bound.reference);
            let (params, body) = crate::region::in_child(
                [(
                    deckmaste_core::Kind::Entity,
                    deckmaste_core::Provenance::LoopElement,
                )],
                || {
                    crate::region::push_antecedent(
                        deckmaste_core::RefId(0),
                        deckmaste_core::Kind::Entity,
                        crate::region::Cardinality::One,
                        bound.sort,
                        crate::region::Site::Loop,
                    );
                    lower_block(std::sync::Arc::unwrap_or_clone(each.effect))
                },
            );
            setup.push(Instruction::Each(deckmaste_core::Each {
                over,
                body: deckmaste_core::Region::new(params, body),
            }));
            setup
        }
        S::Distribute(distribute) => {
            let amount = distribute.amount.lower();
            if let deckmaste_semantics::Binder::Existing(selection) = &distribute.binder {
                let over = selection.clone().lower();
                let (params, body) = crate::region::in_child(
                    [
                        (
                            deckmaste_core::Kind::Entity,
                            deckmaste_core::Provenance::LoopElement,
                        ),
                        (
                            deckmaste_core::Kind::Number,
                            deckmaste_core::Provenance::Allotment,
                        ),
                    ],
                    || {
                        crate::region::push_antecedent(
                            deckmaste_core::RefId(0),
                            deckmaste_core::Kind::Entity,
                            crate::region::Cardinality::One,
                            None,
                            crate::region::Site::Loop,
                        );
                        crate::region::push_antecedent(
                            deckmaste_core::RefId(1),
                            deckmaste_core::Kind::Number,
                            crate::region::Cardinality::One,
                            Some(deckmaste_semantics::Sort::Amount),
                            crate::region::Site::Allotment,
                        );
                        lower_block(std::sync::Arc::unwrap_or_clone(distribute.body))
                    },
                );
                return vec![Instruction::Distribute(deckmaste_core::Distribute {
                    amount,
                    over,
                    body: deckmaste_core::Region::new(params, body),
                })];
            }
            let (mut setup, bound) = lower_binder(distribute.binder);
            let over = deckmaste_core::Selection::Reg(bound.reference);
            let (params, body) = crate::region::in_child(
                [
                    (
                        deckmaste_core::Kind::Entity,
                        deckmaste_core::Provenance::LoopElement,
                    ),
                    (
                        deckmaste_core::Kind::Number,
                        deckmaste_core::Provenance::Allotment,
                    ),
                ],
                || {
                    crate::region::push_antecedent(
                        deckmaste_core::RefId(0),
                        deckmaste_core::Kind::Entity,
                        crate::region::Cardinality::One,
                        bound.sort,
                        crate::region::Site::Loop,
                    );
                    crate::region::push_antecedent(
                        deckmaste_core::RefId(1),
                        deckmaste_core::Kind::Number,
                        crate::region::Cardinality::One,
                        Some(deckmaste_semantics::Sort::Amount),
                        crate::region::Site::Allotment,
                    );
                    lower_block(std::sync::Arc::unwrap_or_clone(distribute.body))
                },
            );
            setup.push(Instruction::Distribute(deckmaste_core::Distribute {
                amount,
                over,
                body: deckmaste_core::Region::new(params, body),
            }));
            setup
        }
        S::RevealUntil(reveal) => {
            let whose = reveal.whose.lower();
            let matches =
                std::sync::Arc::new(crate::region::predicate_region(|| reveal.matches.lower()));
            let found = crate::region::define(deckmaste_core::Kind::Entity);
            let passed = crate::region::define(deckmaste_core::Kind::Entities);
            crate::region::push_antecedent(
                found.into(),
                deckmaste_core::Kind::Entity,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                crate::region::Site::Loop,
            );
            crate::region::push_antecedent(
                passed.into(),
                deckmaste_core::Kind::Entities,
                crate::region::Cardinality::Many,
                Some(deckmaste_semantics::Sort::Card),
                crate::region::Site::ExecutionFrame,
            );
            let (params, body) = crate::region::in_child([], || {
                lower_block(std::sync::Arc::unwrap_or_clone(reveal.body))
            });
            vec![Instruction::RevealUntil(deckmaste_core::RevealUntil {
                found,
                passed,
                whose,
                matches,
                body: deckmaste_core::Region::new(params, body),
            })]
        }
        S::SeparatePiles(value) => vec![Instruction::SeparatePiles(value.lower())],
        S::ChoosePile(value) => vec![Instruction::ChoosePile(value.lower())],
        S::May(value) => vec![Instruction::May(value.lower())],
        S::If(value) => vec![Instruction::If(value.lower())],
        S::Delayed(value) => vec![Instruction::Delayed(value.lower())],
        S::Reflexive(value) => vec![Instruction::Reflexive(value.lower())],
        S::Modal(value) => vec![Instruction::Modal(value.lower())],
        S::Targeted(value) => lower_instructions(std::sync::Arc::unwrap_or_clone(value.effect)),
        S::Repeat(count, body) => vec![Instruction::Repeat(
            count.lower(),
            crate::region::scoped_antecedents(|| lower_arc(body)),
        )],
        S::Batch(count, body) => vec![Instruction::Batch(
            count.lower(),
            crate::region::scoped_antecedents(|| lower_arc(body)),
        )],
        S::Expanded(expanded) => lower_instructions(*expanded.value),
    }
}

impl Lower for deckmaste_semantics::OneShotEffect {
    type Target = deckmaste_core::Instruction;
    fn lower(self) -> <Self as Lower>::Target {
        one(lower_instructions(self))
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
    type Target = deckmaste_core::Instruction;
    fn lower(self) -> <Self as Lower>::Target {
        std::sync::Arc::unwrap_or_clone(self.effect).lower()
    }
}

impl Lower for deckmaste_semantics::May {
    type Target = deckmaste_core::May;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::May {
            who: self.who.lower(),
            effect: crate::region::scoped_antecedents(|| lower_arc(self.effect)),
            if_did: crate::region::scoped_antecedents(|| lower_optional_arc(self.if_did)),
            if_not: crate::region::scoped_antecedents(|| lower_optional_arc(self.if_not)),
        }
    }
}

impl Lower for deckmaste_semantics::If {
    type Target = deckmaste_core::If;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::If {
            condition: self.condition.lower(),
            then: crate::region::scoped_antecedents(|| lower_arc(self.then)),
            otherwise: crate::region::scoped_antecedents(|| lower_optional_arc(self.otherwise)),
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

impl Lower for deckmaste_semantics::SeparatePiles {
    type Target = deckmaste_core::SeparatePiles;
    fn lower(self) -> <Self as Lower>::Target {
        if self.note.is_some() {
            crate::region::refuse(
                "noted pile sets have no register spelling yet; owner: engine-piles",
            );
        }
        let group = self.group.lower();
        let by = self.by.lower();
        let (dests, then) = crate::region::scoped_anaphora(|| {
            let dests: std::sync::Arc<[deckmaste_core::DefId]> = self
                .into
                .iter()
                .copied()
                .map(|label| {
                    let dest = crate::region::define(deckmaste_core::Kind::Pile);
                    crate::region::bind_named(label, dest.into());
                    dest
                })
                .collect::<Vec<_>>()
                .into();
            (dests, self.then.lower())
        });
        deckmaste_core::SeparatePiles {
            dests,
            group,
            by,
            then,
        }
    }
}

impl Lower for deckmaste_semantics::ChoosePile {
    type Target = deckmaste_core::ChoosePile;
    fn lower(self) -> <Self as Lower>::Target {
        let from: std::sync::Arc<[deckmaste_core::RefId]> = match self.from {
            deckmaste_semantics::PileSource::Labels(labels) => labels
                .iter()
                .map(|label| {
                    crate::region::named(label).unwrap_or_else(|| {
                        crate::region::refuse(&format!(
                            "pile label `{label}` has no dominating SeparatePiles definition"
                        ));
                        deckmaste_core::RefId(0)
                    })
                })
                .collect::<Vec<_>>()
                .into(),
            deckmaste_semantics::PileSource::Noted { .. } => {
                crate::region::refuse(
                    "noted pile sets have no register spelling yet; owner: engine-piles",
                );
                std::sync::Arc::from([])
            }
        };
        let by = self.by.lower();
        let dest = crate::region::define(deckmaste_core::Kind::Pile);
        let then = crate::region::with_antecedent(
            dest.into(),
            deckmaste_core::Kind::Pile,
            crate::region::Cardinality::Many,
            Some(deckmaste_semantics::Sort::Pile),
            crate::region::Site::ExecutionFrame,
            || self.then.lower(),
        );
        deckmaste_core::ChoosePile {
            dest,
            from,
            by,
            random: self.random.lower(),
            then,
        }
    }
}
