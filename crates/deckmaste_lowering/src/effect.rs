//! `effect` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

type Instr = deckmaste_core::OneShotEffect;

fn one(instructions: Vec<Instr>) -> Instr {
    match instructions.as_slice() {
        [instruction] => instruction.clone(),
        _ => Instr::Sequentially(instructions.into()),
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
    fn lowers_one_shot_effect_separate_piles() {
        assert_matches!(
            deckmaste_semantics::OneShotEffect::SeparatePiles(minimal_separate_piles()).lower(),
            deckmaste_core::OneShotEffect::SeparatePiles(deckmaste_core::SeparatePiles {
                group: deckmaste_core::Selection::SelectAll(_),
                into: _,
                by: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                note: None,
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
            deckmaste_semantics::SeparatePiles {
                group: minimal_selection(),
                into: [].into(),
                by: minimal_reference(),
                note: None,
                then: None
            }
            .lower(),
            deckmaste_core::SeparatePiles {
                group: deckmaste_core::Selection::SelectAll(_),
                into: _,
                by: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                note: None,
                then: None
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
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }
}

pub(crate) fn lower_block(effect: deckmaste_semantics::OneShotEffect) -> deckmaste_core::Block {
    deckmaste_core::Block(lower_instructions(effect).into())
}

fn lower_arc(effect: std::sync::Arc<deckmaste_semantics::OneShotEffect>) -> std::sync::Arc<Instr> {
    std::sync::Arc::new(one(lower_instructions(std::sync::Arc::unwrap_or_clone(
        effect,
    ))))
}

fn lower_optional_arc(
    effect: Option<std::sync::Arc<deckmaste_semantics::OneShotEffect>>,
) -> Option<std::sync::Arc<Instr>> {
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
) -> Instr {
    let (params, body) = crate::region::in_child(
        [(
            deckmaste_core::Kind::Object,
            deckmaste_core::Provenance::LoopElement,
        )],
        || {
            crate::region::push_antecedent(
                deckmaste_core::RefId(0),
                deckmaste_core::Kind::Object,
                crate::region::Cardinality::One,
                None,
                crate::region::Site::Loop,
            );
            lower_block(std::sync::Arc::unwrap_or_clone(effect))
        },
    );
    Instr::Each(deckmaste_core::Each {
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

fn lower_action(action: deckmaste_semantics::Action) -> Vec<Instr> {
    if let deckmaste_semantics::Action::ChooseValue(who, domain, note) = action {
        let by = who.lower();
        let domain = domain.lower();
        let dest = crate::region::define(deckmaste_core::Kind::Symbol);
        crate::region::bind_named(note, dest.into());
        return vec![Instr::ChooseValue(deckmaste_core::ChooseValue {
            dest,
            by,
            domain,
        })];
    }

    let action = action.lower();
    match &action {
        deckmaste_core::Action::Move(_, destination, _, _) => {
            let dest = crate::region::define(deckmaste_core::Kind::Object);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Object,
                crate::region::Cardinality::One,
                Some(destination_sort(destination)),
                crate::region::Site::Product,
            );
            vec![Instr::producing(dest, action)]
        }
        deckmaste_core::Action::MoveGroup { to, .. } => {
            let dest = crate::region::define(deckmaste_core::Kind::Objects);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                Some(destination_sort(to)),
                crate::region::Site::Product,
            );
            vec![Instr::producing(dest, action)]
        }
        deckmaste_core::Action::Create { .. } => {
            let dest = crate::region::define(deckmaste_core::Kind::Objects);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                Some(deckmaste_semantics::Sort::Token),
                crate::region::Site::Product,
            );
            vec![Instr::producing(dest, action)]
        }
        deckmaste_core::Action::DrawCard(_) => {
            let dest = crate::region::define(deckmaste_core::Kind::Object);
            crate::region::push_antecedent(
                dest.into(),
                deckmaste_core::Kind::Object,
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
                Instr::producing(dest, action),
                Instr::Let(deckmaste_core::Let {
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
                Instr::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Number(amount.clone()),
                }),
                Instr::act(deckmaste_core::Action::DealDamage(
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
                LifeOp::Set(_) => return vec![Instr::act(action)],
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
                Instr::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Number(amount.clone()),
                }),
                Instr::act(deckmaste_core::Action::ChangeLife(
                    patient.clone(),
                    rebuild(deckmaste_core::Count::Reg(dest.into())),
                )),
            ]
        }
        _ => vec![Instr::act(action)],
    }
}

struct BoundValue {
    reference: deckmaste_core::RefId,
    kind: deckmaste_core::Kind,
    cardinality: crate::region::Cardinality,
    sort: Option<deckmaste_semantics::Sort>,
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

pub(crate) fn binder_shape(
    binder: &deckmaste_semantics::Binder,
) -> (
    deckmaste_core::Kind,
    crate::region::Cardinality,
    Option<deckmaste_semantics::Sort>,
) {
    use deckmaste_semantics::Binder;
    match binder {
        Binder::TheRef(_) => (
            deckmaste_core::Kind::Object,
            crate::region::Cardinality::One,
            None,
        ),
        Binder::ChooseOne { filter, .. } => (
            deckmaste_core::Kind::Object,
            crate::region::Cardinality::One,
            predicate_sort(filter),
        ),
        Binder::Choose { .. } | Binder::Existing(_) => (
            deckmaste_core::Kind::Objects,
            crate::region::Cardinality::Many,
            None,
        ),
        Binder::Produce(action) => match action.as_ref() {
            deckmaste_semantics::Action::Move(_, to, _, _) => (
                deckmaste_core::Kind::Object,
                crate::region::Cardinality::One,
                Some(destination_sort(&to.clone().lower())),
            ),
            deckmaste_semantics::Action::MoveGroup { to, .. } => (
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                Some(destination_sort(&to.clone().lower())),
            ),
            deckmaste_semantics::Action::Create { .. } => (
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                Some(deckmaste_semantics::Sort::Token),
            ),
            _ => (
                deckmaste_core::Kind::Object,
                crate::region::Cardinality::One,
                None,
            ),
        },
        Binder::SearchOne { .. } => (
            deckmaste_core::Kind::Object,
            crate::region::Cardinality::One,
            Some(deckmaste_semantics::Sort::Card),
        ),
        Binder::Search { .. } => (
            deckmaste_core::Kind::Objects,
            crate::region::Cardinality::Many,
            Some(deckmaste_semantics::Sort::Card),
        ),
        Binder::Expanded(expanded) => binder_shape(&expanded.value),
    }
}

fn lower_binder(binder: deckmaste_semantics::Binder) -> (Vec<Instr>, BoundValue) {
    use deckmaste_semantics::Binder;
    match binder {
        Binder::TheRef(reference) => {
            let reference = reference.lower();
            let dest = crate::region::define(deckmaste_core::Kind::Object);
            (
                vec![Instr::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Object(reference),
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Object,
                    cardinality: crate::region::Cardinality::One,
                    sort: None,
                },
            )
        }
        Binder::ChooseOne { filter, by } => {
            let sort = predicate_sort(&filter);
            let by = by.lower();
            let filter = std::sync::Arc::new(crate::region::candidate_region(|| filter.lower()));
            let dest = crate::region::define(deckmaste_core::Kind::Objects);
            (
                vec![Instr::Choose(deckmaste_core::Choose {
                    dest,
                    by,
                    quantity: deckmaste_core::Quantity::one(),
                    filter,
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Objects,
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
            let filter = std::sync::Arc::new(crate::region::candidate_region(|| filter.lower()));
            let dest = crate::region::define(deckmaste_core::Kind::Objects);
            (
                vec![Instr::Choose(deckmaste_core::Choose {
                    dest,
                    by,
                    quantity,
                    filter,
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Objects,
                    cardinality: crate::region::Cardinality::Many,
                    sort: None,
                },
            )
        }
        Binder::Existing(selection) => {
            let selection = selection.lower();
            let dest = crate::region::define(deckmaste_core::Kind::Objects);
            (
                vec![Instr::Let(deckmaste_core::Let {
                    dest,
                    expr: deckmaste_core::Expr::Objects(selection),
                })],
                BoundValue {
                    reference: dest.into(),
                    kind: deckmaste_core::Kind::Objects,
                    cardinality: crate::region::Cardinality::Many,
                    sort: None,
                },
            )
        }
        Binder::Produce(action) => {
            let action = std::sync::Arc::unwrap_or_clone(action).lower();
            let (kind, cardinality, sort) = match &action {
                deckmaste_core::Action::Move(_, to, _, _) => (
                    deckmaste_core::Kind::Object,
                    crate::region::Cardinality::One,
                    Some(destination_sort(to)),
                ),
                deckmaste_core::Action::MoveGroup { to, .. } => (
                    deckmaste_core::Kind::Objects,
                    crate::region::Cardinality::Many,
                    Some(destination_sort(to)),
                ),
                deckmaste_core::Action::Create { .. } => (
                    deckmaste_core::Kind::Objects,
                    crate::region::Cardinality::Many,
                    Some(deckmaste_semantics::Sort::Token),
                ),
                _ => (
                    deckmaste_core::Kind::Object,
                    crate::region::Cardinality::One,
                    None,
                ),
            };
            let dest = crate::region::define(kind);
            (
                vec![Instr::producing(dest, action)],
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
) -> (Vec<Instr>, BoundValue) {
    let by = by.lower();
    let whose = whose.lower();
    let from = from.lower();
    let filter = std::sync::Arc::new(crate::region::candidate_region(|| filter.lower()));
    let if_none = crate::region::scoped_antecedents(|| {
        if_none.map_or_else(deckmaste_core::Block::default, |effect| {
            lower_block(std::sync::Arc::unwrap_or_clone(effect))
        })
    });
    let dest = crate::region::define(deckmaste_core::Kind::Objects);
    (
        vec![Instr::Search(deckmaste_core::Search {
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
            kind: deckmaste_core::Kind::Objects,
            cardinality: crate::region::Cardinality::Many,
            sort: Some(deckmaste_semantics::Sort::Card),
        },
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "the exhaustive semantic-to-core instruction dispatch is clearer as one match"
)]
fn lower_instructions(effect: deckmaste_semantics::OneShotEffect) -> Vec<Instr> {
    use deckmaste_semantics::OneShotEffect as S;
    match effect {
        S::Act(action) => lower_action(action),
        S::Sequentially(parts) => parts.iter().cloned().flat_map(lower_instructions).collect(),
        S::Simultaneously(parts) => vec![Instr::Simultaneously(
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
        S::Continuously(value) => vec![Instr::Continuously(value.lower())],
        S::Until(duration, effects) => vec![Instr::Until(duration.lower(), effects.lower())],
        S::Label(label) => {
            let instructions = lower_instructions(std::sync::Arc::unwrap_or_clone(label.effect));
            if let Some(reference) = crate::region::newest_antecedent() {
                crate::region::bind_named(label.r#as, reference);
            }
            instructions
        }
        S::Noting(noting) => {
            let instructions = lower_instructions(std::sync::Arc::unwrap_or_clone(noting.effect));
            if let Some(reference) = crate::region::newest_antecedent() {
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
                crate::region::Site::Frame,
            );
            instructions.extend(lower_instructions(std::sync::Arc::unwrap_or_clone(
                with.body,
            )));
            crate::region::remove_antecedent(bound.reference, crate::region::Site::Frame);
            instructions
        }
        S::Each(each) => {
            if let deckmaste_semantics::Binder::Existing(selection) = &each.binder {
                let over = selection.clone().lower();
                return vec![lower_existing_each(over, each.effect)];
            }
            let (mut setup, bound) = lower_binder(each.binder);
            let over = deckmaste_core::Selection::Reg(bound.reference);
            let (params, body) = crate::region::in_child(
                [(
                    deckmaste_core::Kind::Object,
                    deckmaste_core::Provenance::LoopElement,
                )],
                || {
                    crate::region::push_antecedent(
                        deckmaste_core::RefId(0),
                        deckmaste_core::Kind::Object,
                        crate::region::Cardinality::One,
                        bound.sort,
                        crate::region::Site::Loop,
                    );
                    lower_block(std::sync::Arc::unwrap_or_clone(each.effect))
                },
            );
            setup.push(Instr::Each(deckmaste_core::Each {
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
                            deckmaste_core::Kind::Object,
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
                            deckmaste_core::Kind::Object,
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
                return vec![Instr::Distribute(deckmaste_core::Distribute {
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
                        deckmaste_core::Kind::Object,
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
                        deckmaste_core::Kind::Object,
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
            setup.push(Instr::Distribute(deckmaste_core::Distribute {
                amount,
                over,
                body: deckmaste_core::Region::new(params, body),
            }));
            setup
        }
        S::RevealUntil(reveal) => {
            let whose = reveal.whose.lower();
            let matches =
                std::sync::Arc::new(crate::region::candidate_region(|| reveal.matches.lower()));
            let found = crate::region::define(deckmaste_core::Kind::Object);
            let passed = crate::region::define(deckmaste_core::Kind::Objects);
            crate::region::push_antecedent(
                found.into(),
                deckmaste_core::Kind::Object,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                crate::region::Site::Loop,
            );
            crate::region::push_antecedent(
                passed.into(),
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                Some(deckmaste_semantics::Sort::Card),
                crate::region::Site::Frame,
            );
            let (params, body) = crate::region::in_child([], || {
                lower_block(std::sync::Arc::unwrap_or_clone(reveal.body))
            });
            vec![Instr::RevealUntil(deckmaste_core::RevealUntil {
                found,
                passed,
                whose,
                matches,
                body: deckmaste_core::Region::new(params, body),
            })]
        }
        S::SeparatePiles(value) => vec![Instr::SeparatePiles(value.lower())],
        S::ChoosePile(value) => vec![Instr::ChoosePile(value.lower())],
        S::May(value) => vec![Instr::May(value.lower())],
        S::If(value) => vec![Instr::If(value.lower())],
        S::AdditionalCost(value) => vec![Instr::AdditionalCost(value.lower())],
        S::Delayed(value) => vec![Instr::Delayed(value.lower())],
        S::Reflexive(value) => vec![Instr::Reflexive(value.lower())],
        S::Modal(value) => vec![Instr::Modal(value.lower())],
        S::Targeted(value) => lower_instructions(std::sync::Arc::unwrap_or_clone(value.effect)),
        S::Repeat(count, body) => vec![Instr::Repeat(
            count.lower(),
            crate::region::scoped_antecedents(|| lower_arc(body)),
        )],
        S::Batch(count, body) => vec![Instr::Batch(
            count.lower(),
            crate::region::scoped_antecedents(|| lower_arc(body)),
        )],
        S::Expanded(expanded) => lower_instructions(*expanded.value),
    }
}

impl Lower for deckmaste_semantics::OneShotEffect {
    type Target = deckmaste_core::OneShotEffect;
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

impl Lower for deckmaste_semantics::AdditionalCost {
    type Target = deckmaste_core::AdditionalCost;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::AdditionalCost {
            pay: self.pay.lower(),
            body: lower_arc(self.body),
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
        let from = self.from.lower();
        let by = self.by.lower();
        let dest = crate::region::define(deckmaste_core::Kind::Objects);
        let then = crate::region::with_antecedent(
            dest.into(),
            deckmaste_core::Kind::Objects,
            crate::region::Cardinality::Many,
            Some(deckmaste_semantics::Sort::Pile),
            crate::region::Site::Frame,
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
