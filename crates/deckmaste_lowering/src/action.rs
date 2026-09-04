//! `action` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

#![allow(
    clippy::items_after_test_module,
    reason = "generated lowering tests remain adjacent to the constructor families they cover"
)]

use crate::Lower;

impl Lower for deckmaste_semantics::Anchor {
    type Target = deckmaste_core::Anchor;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FromTop(offset) => deckmaste_core::Anchor::FromTop(offset.lower()),
            Self::FromBottom(offset) => deckmaste_core::Anchor::FromBottom(offset.lower()),
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
    fn lowers_anchor_from_top() {
        assert_matches!(
            deckmaste_semantics::Anchor::FromTop(minimal_count()).lower(),
            deckmaste_core::Anchor::FromTop(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_anchor_from_bottom() {
        assert_matches!(
            deckmaste_semantics::Anchor::FromBottom(minimal_count()).lower(),
            deckmaste_core::Anchor::FromBottom(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_destination_zone() {
        assert_matches!(
            deckmaste_semantics::Destination::Zone(minimal_zone()).lower(),
            deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_destination_library() {
        assert_matches!(
            deckmaste_semantics::Destination::Library(minimal_anchor()).lower(),
            deckmaste_core::Destination::Library(deckmaste_core::Anchor::FromTop(
                deckmaste_core::Count::Literal(0)
            ))
        );
    }

    #[test]
    fn lowers_enter_rider_tapped() {
        assert_matches!(
            deckmaste_semantics::EnterRider::Tapped.lower(),
            deckmaste_core::EnterRider::Tapped
        );
    }

    #[test]
    fn lowers_enter_rider_face_down() {
        assert_matches!(
            deckmaste_semantics::EnterRider::FaceDown.lower(),
            deckmaste_core::EnterRider::FaceDown
        );
    }

    #[test]
    fn lowers_enter_rider_under_control_of() {
        assert_matches!(
            deckmaste_semantics::EnterRider::UnderControlOf(minimal_reference()).lower(),
            deckmaste_core::EnterRider::UnderControlOf(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_enter_rider_under_owners_control() {
        assert_matches!(
            deckmaste_semantics::EnterRider::UnderOwnersControl.lower(),
            deckmaste_core::EnterRider::UnderOwnersControl
        );
    }

    #[test]
    fn lowers_enter_rider_attacking() {
        assert_matches!(
            deckmaste_semantics::EnterRider::Attacking(None).lower(),
            deckmaste_core::EnterRider::Attacking(None)
        );
    }

    #[test]
    fn lowers_enter_rider_with_counters() {
        assert_matches!(
            deckmaste_semantics::EnterRider::WithCounters(minimal_counter_ref(), minimal_count())
                .lower(),
            deckmaste_core::EnterRider::WithCounters(
                deckmaste_core::CounterRef(_),
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_enter_rider_as_copy() {
        assert_matches!(
            deckmaste_semantics::EnterRider::AsCopy(minimal_copy_spec()).lower(),
            deckmaste_core::EnterRider::AsCopy(deckmaste_core::CopySpec {
                source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::Reg(
                    deckmaste_core::RefId(0)
                )),
                exceptions: _
            })
        );
    }

    #[test]
    fn lowers_arrangement_chosen_order() {
        assert_matches!(
            deckmaste_semantics::Arrangement::ChosenOrder(minimal_reference()).lower(),
            deckmaste_core::Arrangement::ChosenOrder(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_arrangement_any_order() {
        assert_matches!(
            deckmaste_semantics::Arrangement::AnyOrder.lower(),
            deckmaste_core::Arrangement::AnyOrder
        );
    }

    #[test]
    fn lowers_arrangement_same_order() {
        assert_matches!(
            deckmaste_semantics::Arrangement::SameOrder.lower(),
            deckmaste_core::Arrangement::SameOrder
        );
    }

    #[test]
    fn lowers_arrangement_random_order() {
        assert_matches!(
            deckmaste_semantics::Arrangement::RandomOrder.lower(),
            deckmaste_core::Arrangement::RandomOrder
        );
    }

    #[test]
    fn lowers_action_deal_damage() {
        assert_matches!(
            deckmaste_semantics::Action::DealDamage(
                minimal_reference(),
                minimal_count(),
                minimal_reference()
            )
            .lower(),
            deckmaste_core::Action::DealDamage(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Count::Literal(0),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_counter() {
        assert_matches!(
            deckmaste_semantics::Action::Counter(minimal_reference()).lower(),
            deckmaste_core::Action::Counter(deckmaste_core::Reference::Reg(deckmaste_core::RefId(
                0
            )))
        );
    }

    #[test]
    fn lowers_action_transform() {
        assert_matches!(
            deckmaste_semantics::Action::Transform(minimal_reference()).lower(),
            deckmaste_core::Action::Transform(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_action_cease() {
        assert_matches!(
            deckmaste_semantics::Action::Cease(minimal_reference()).lower(),
            deckmaste_core::Action::Cease(deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_attach() {
        assert_matches!(
            deckmaste_semantics::Action::Attach {
                what: minimal_reference(),
                to: minimal_reference()
            }
            .lower(),
            deckmaste_core::Action::Attach {
                what: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                to: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_action_unattach() {
        assert_matches!(
            deckmaste_semantics::Action::Unattach(minimal_reference()).lower(),
            deckmaste_core::Action::Unattach(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_action_move() {
        assert_matches!(
            deckmaste_semantics::Action::Move(
                minimal_reference(),
                minimal_destination(),
                [].into(),
                None
            )
            .lower(),
            deckmaste_core::Action::Move(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield),
                _,
                None
            )
        );
    }

    #[test]
    fn lowers_action_move_group() {
        assert_matches!(
            deckmaste_semantics::Action::MoveGroup {
                group: minimal_selection(),
                arrangement: minimal_arrangement(),
                to: minimal_destination(),
                riders: [].into()
            }
            .lower(),
            deckmaste_core::Action::MoveGroup {
                group: deckmaste_core::Selection::SelectAll(_),
                arrangement: deckmaste_core::Arrangement::ChosenOrder(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
                ),
                to: deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield),
                riders: _
            }
        );
    }

    #[test]
    fn lowers_action_gain_control() {
        assert_matches!(
            deckmaste_semantics::Action::GainControl(minimal_reference(), minimal_reference())
                .lower(),
            deckmaste_core::Action::GainControl(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_extra_phase() {
        assert_matches!(
            deckmaste_semantics::Action::ExtraPhase(minimal_phase_kind(), minimal_reference())
                .lower(),
            deckmaste_core::Action::ExtraPhase(
                deckmaste_core::PhaseKind::Beginning,
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_move_counters() {
        assert_matches!(
            deckmaste_semantics::Action::MoveCounters(
                minimal_counter_spec(),
                minimal_reference(),
                minimal_reference()
            )
            .lower(),
            deckmaste_core::Action::MoveCounters(
                deckmaste_core::CounterSpec::Named(
                    deckmaste_core::CounterRef(_),
                    deckmaste_core::Count::Literal(0)
                ),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_change_life() {
        assert_matches!(
            deckmaste_semantics::Action::ChangeLife(minimal_reference(), minimal_life_op()).lower(),
            deckmaste_core::Action::ChangeLife(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::LifeOp::Set(deckmaste_core::Count::Literal(0))
            )
        );
    }

    #[test]
    fn lowers_action_add_mana() {
        assert_matches!(
            deckmaste_semantics::Action::AddMana(
                minimal_reference(),
                minimal_count(),
                minimal_mana_production()
            )
            .lower(),
            deckmaste_core::Action::AddMana(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Count::Literal(0),
                deckmaste_core::ManaProduction::WithRiders {
                    mana: deckmaste_core::ManaSpec::AnyColor,
                    riders: _
                }
            )
        );
    }

    #[test]
    fn lowers_action_create() {
        assert_matches!(
            deckmaste_semantics::Action::Create {
                agent: minimal_reference(),
                count: minimal_count(),
                token: minimal_token_spec(),
                riders: [].into()
            }
            .lower(),
            deckmaste_core::Action::Create {
                agent: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                count: deckmaste_core::Count::Literal(0),
                token: deckmaste_core::TokenSpec::Token(_),
                riders: _
            }
        );
    }

    #[test]
    fn lowers_action_sacrifice() {
        assert_matches!(
            deckmaste_semantics::Action::Sacrifice(minimal_reference(), minimal_reference())
                .lower(),
            deckmaste_core::Action::Sacrifice(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_action_draw_card() {
        assert_matches!(
            deckmaste_semantics::Action::DrawCard(minimal_reference()).lower(),
            deckmaste_core::Action::DrawCard(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_action_tap() {
        assert_matches!(
            deckmaste_semantics::Action::Tap(minimal_reference()).lower(),
            deckmaste_core::Action::Tap(deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_untap() {
        assert_matches!(
            deckmaste_semantics::Action::Untap(minimal_reference()).lower(),
            deckmaste_core::Action::Untap(deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)))
        );
    }

    #[test]
    fn lowers_action_get_emblem() {
        assert_matches!(
            deckmaste_semantics::Action::GetEmblem(minimal_reference(), [].into()).lower(),
            deckmaste_core::Action::GetEmblem(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                _
            )
        );
    }

    #[test]
    fn lowers_action_get_designation() {
        assert_matches!(
            deckmaste_semantics::Action::GetDesignation(minimal_reference(), "X".into()).lower(),
            deckmaste_core::Action::GetDesignation(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                _
            )
        );
    }

    #[test]
    fn lowers_action_set_game_designation() {
        assert_matches!(
            deckmaste_semantics::Action::SetGameDesignation("X".into(), "Y".into()).lower(),
            deckmaste_core::Action::SetGameDesignation(name, value)
                if name.as_ref() == "X" && value.as_ref() == "Y"
        );
    }

    #[test]
    fn lowers_action_choose_value() {
        assert_matches!(
            deckmaste_semantics::Action::ChooseValue(
                minimal_reference(),
                minimal_chosen_value_kind(),
                "X".into()
            )
            .lower(),
            deckmaste_core::Action::ChooseValue(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::ChosenValueKind::Color,
                _
            )
        );
    }

    #[test]
    fn lowers_action_copy_spell() {
        assert_matches!(
            deckmaste_semantics::Action::CopySpell {
                controller: minimal_reference(),
                spec: minimal_copy_spec(),
                retarget: minimal_copy_retarget()
            }
            .lower(),
            deckmaste_core::Action::CopySpell {
                controller: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                spec: deckmaste_core::CopySpec {
                    source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::Reg(
                        deckmaste_core::RefId(0)
                    )),
                    exceptions: _
                },
                retarget: deckmaste_core::CopyRetarget::AsIs
            }
        );
    }

    #[test]
    fn lowers_action_cast_copy() {
        assert_matches!(
            deckmaste_semantics::Action::CastCopy(minimal_reference(), minimal_copy_spec()).lower(),
            deckmaste_core::Action::CastCopy(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::CopySpec {
                    source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::Reg(
                        deckmaste_core::RefId(0)
                    )),
                    exceptions: _
                }
            )
        );
    }

    #[test]
    fn lowers_action_cast() {
        assert_matches!(
            deckmaste_semantics::Action::Cast(minimal_reference(), minimal_reference(), None)
                .lower(),
            deckmaste_core::Action::Cast(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                None
            )
        );
    }

    #[test]
    fn lowers_action_retarget() {
        assert_matches!(
            deckmaste_semantics::Action::Retarget {
                mode: minimal_retarget_mode(),
                of: minimal_reference(),
                by: minimal_reference()
            }
            .lower(),
            deckmaste_core::Action::Retarget {
                mode: deckmaste_core::RetargetMode::ChangeAll,
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                by: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_action_flip_coins() {
        assert_matches!(
            deckmaste_semantics::Action::FlipCoins(minimal_reference(), minimal_count(), false)
                .lower(),
            deckmaste_core::Action::FlipCoins(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Count::Literal(0),
                false
            )
        );
    }

    #[test]
    fn lowers_action_roll_dice() {
        assert_matches!(
            deckmaste_semantics::Action::RollDice(minimal_reference(), minimal_count(), 0).lower(),
            deckmaste_core::Action::RollDice(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Count::Literal(0),
                0
            )
        );
    }

    #[test]
    fn lowers_action_roll_planar_die() {
        assert_matches!(
            deckmaste_semantics::Action::RollPlanarDie(minimal_reference()).lower(),
            deckmaste_core::Action::RollPlanarDie(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_action_put_counters() {
        assert_matches!(
            deckmaste_semantics::Action::PutCounters(
                minimal_reference(),
                minimal_counter_ref(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Action::PutCounters(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::CounterRef(_),
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_action_remove_counters() {
        assert_matches!(
            deckmaste_semantics::Action::RemoveCounters(
                minimal_reference(),
                minimal_counter_ref(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Action::RemoveCounters(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::CounterRef(_),
                deckmaste_core::Count::Literal(0)
            )
        );
    }

    #[test]
    fn lowers_action_win_game() {
        assert_matches!(
            deckmaste_semantics::Action::WinGame(minimal_reference()).lower(),
            deckmaste_core::Action::WinGame(deckmaste_core::Reference::Reg(deckmaste_core::RefId(
                0
            )))
        );
    }

    #[test]
    fn lowers_action_lose_game() {
        assert_matches!(
            deckmaste_semantics::Action::LoseGame(minimal_reference()).lower(),
            deckmaste_core::Action::LoseGame(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_action_restart_game() {
        assert_matches!(
            deckmaste_semantics::Action::RestartGame.lower(),
            deckmaste_core::Action::RestartGame
        );
    }

    #[test]
    fn lowers_action_shuffle() {
        assert_matches!(
            deckmaste_semantics::Action::Shuffle(minimal_selection()).lower(),
            deckmaste_core::Action::Shuffle(deckmaste_core::Selection::SelectAll(_))
        );
    }

    #[test]
    fn lowers_action_reveal() {
        assert_matches!(
            deckmaste_semantics::Action::Reveal {
                what: minimal_reference(),
                to: None
            }
            .lower(),
            deckmaste_core::Action::Reveal {
                what: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                to: None
            }
        );
    }

    #[test]
    fn lowers_action_remove_damage() {
        assert_matches!(
            deckmaste_semantics::Action::RemoveDamage(minimal_reference()).lower(),
            deckmaste_core::Action::RemoveDamage(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_action_pay() {
        assert_matches!(
            deckmaste_semantics::Action::Pay(minimal_cost()).lower(),
            deckmaste_core::Action::Pay(deckmaste_core::Cost(_))
        );
    }

    #[test]
    fn lowers_action_expanded() {
        assert_matches!(
            deckmaste_semantics::Action::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_action())
            })
            .lower(),
            deckmaste_core::Action::DealDamage(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Count::Literal(0),
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_life_op_set() {
        assert_matches!(
            deckmaste_semantics::LifeOp::Set(minimal_count()).lower(),
            deckmaste_core::LifeOp::Set(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_life_op_up() {
        assert_matches!(
            deckmaste_semantics::LifeOp::Up(minimal_count()).lower(),
            deckmaste_core::LifeOp::Up(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_life_op_down() {
        assert_matches!(
            deckmaste_semantics::LifeOp::Down(minimal_count()).lower(),
            deckmaste_core::LifeOp::Down(deckmaste_core::Count::Literal(0))
        );
    }

    #[test]
    fn lowers_retarget_mode_change_all() {
        assert_matches!(
            deckmaste_semantics::RetargetMode::ChangeAll.lower(),
            deckmaste_core::RetargetMode::ChangeAll
        );
    }

    #[test]
    fn lowers_retarget_mode_change_one() {
        assert_matches!(
            deckmaste_semantics::RetargetMode::ChangeOne.lower(),
            deckmaste_core::RetargetMode::ChangeOne
        );
    }

    #[test]
    fn lowers_retarget_mode_change_any() {
        assert_matches!(
            deckmaste_semantics::RetargetMode::ChangeAny.lower(),
            deckmaste_core::RetargetMode::ChangeAny
        );
    }

    #[test]
    fn lowers_retarget_mode_choose_new() {
        assert_matches!(
            deckmaste_semantics::RetargetMode::ChooseNew.lower(),
            deckmaste_core::RetargetMode::ChooseNew
        );
    }

    #[test]
    fn lowers_copy_retarget_as_is() {
        assert_matches!(
            deckmaste_semantics::CopyRetarget::AsIs.lower(),
            deckmaste_core::CopyRetarget::AsIs
        );
    }

    #[test]
    fn lowers_copy_retarget_may_choose_new() {
        assert_matches!(
            deckmaste_semantics::CopyRetarget::MayChooseNew.lower(),
            deckmaste_core::CopyRetarget::MayChooseNew
        );
    }

    #[test]
    fn lowers_copy_retarget_targets_that() {
        assert_matches!(
            deckmaste_semantics::CopyRetarget::TargetsThat(minimal_reference()).lower(),
            deckmaste_core::CopyRetarget::TargetsThat(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_action_composite() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::Action::Composite {
                name: minimal_verb_name(),
                body: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            deckmaste_core::Action::Composite {
                name: deckmaste_core::VerbName(_),
                body: _
            }
        );
    }

    #[test]
    fn lowers_action_create_replacement() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::Action::CreateReplacement {
                replacement: std::sync::Arc::new(minimal_replacement()),
                duration: minimal_duration(),
                one_shot: false
            }
            .lower()),
            deckmaste_core::Action::CreateReplacement {
                subject: deckmaste_core::Reference::Reg(_),
                replacement: _,
                duration: deckmaste_core::Duration::FixedUntil(
                    deckmaste_core::TurnMarker::EndOfTurn
                ),
                one_shot: false
            }
        );
    }
}

impl Lower for deckmaste_semantics::Destination {
    type Target = deckmaste_core::Destination;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Zone(zone) => deckmaste_core::Destination::Zone(zone.lower()),
            Self::Library(anchor) => deckmaste_core::Destination::Library(anchor.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::EnterRider {
    type Target = deckmaste_core::EnterRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => deckmaste_core::EnterRider::Tapped,
            Self::FaceDown => deckmaste_core::EnterRider::FaceDown,
            Self::UnderControlOf(who) => deckmaste_core::EnterRider::UnderControlOf(who.lower()),
            Self::UnderOwnersControl => deckmaste_core::EnterRider::UnderOwnersControl,
            Self::Attacking(who) => deckmaste_core::EnterRider::Attacking(who.lower()),
            Self::WithCounters(kind, count) => {
                deckmaste_core::EnterRider::WithCounters(kind.lower(), count.lower())
            }
            Self::AsCopy(spec) => deckmaste_core::EnterRider::AsCopy(spec.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::Arrangement {
    type Target = deckmaste_core::Arrangement;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ChosenOrder(by) => deckmaste_core::Arrangement::ChosenOrder(by.lower()),
            Self::AnyOrder => deckmaste_core::Arrangement::AnyOrder,
            Self::SameOrder => deckmaste_core::Arrangement::SameOrder,
            Self::RandomOrder => deckmaste_core::Arrangement::RandomOrder,
        }
    }
}

impl Lower for deckmaste_semantics::Action {
    type Target = deckmaste_core::Action;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::DealDamage(source, amount, target) => {
                deckmaste_core::Action::DealDamage(source.lower(), amount.lower(), target.lower())
            }
            Self::Counter(what) => deckmaste_core::Action::Counter(what.lower()),
            Self::Transform(what) => deckmaste_core::Action::Transform(what.lower()),
            Self::Cease(what) => deckmaste_core::Action::Cease(what.lower()),
            Self::Attach { what, to } => deckmaste_core::Action::Attach {
                what: what.lower(),
                to: to.lower(),
            },
            Self::Unattach(what) => deckmaste_core::Action::Unattach(what.lower()),
            Self::Move(what, to, riders, from) => {
                deckmaste_core::Action::Move(what.lower(), to.lower(), riders.lower(), from.lower())
            }
            Self::MoveGroup {
                group,
                arrangement,
                to,
                riders,
            } => deckmaste_core::Action::MoveGroup {
                group: group.lower(),
                arrangement: arrangement.lower(),
                to: to.lower(),
                riders: riders.lower(),
            },
            Self::GainControl(what, to) => {
                deckmaste_core::Action::GainControl(what.lower(), to.lower())
            }
            Self::ExtraPhase(phase, who) => {
                deckmaste_core::Action::ExtraPhase(phase.lower(), who.lower())
            }
            Self::MoveCounters(spec, from, to) => {
                deckmaste_core::Action::MoveCounters(spec.lower(), from.lower(), to.lower())
            }
            Self::CreateReplacement {
                replacement,
                duration,
                one_shot,
            } => deckmaste_core::Action::CreateReplacement {
                // [CR#614.1]: a replacement effect acts like a shield around
                // WHATEVER IT AFFECTS. English names the affected permanent by
                // discourse ("the next time IT would be destroyed"), so the
                // shield's subject is the region's nearest permanent-sorted
                // antecedent, resolved HERE (ADR law 12) and declared as a
                // register the engine reads by index. R1/R2 refuse an
                // ambiguous one at compile time; a shield with no antecedent
                // at all falls back to the source parameter, which is what a
                // self-regenerating permanent means.
                subject: deckmaste_core::Reference::Reg(
                    crate::region::that(deckmaste_semantics::Sort::Permanent)
                        .or_else(crate::region::it)
                        .unwrap_or_else(|| {
                            crate::region::source().expect(
                                "a replacement shield is created inside a region with a source",
                            )
                        }),
                ),
                replacement: replacement.lower(),
                duration: duration.lower(),
                one_shot: one_shot.lower(),
            },
            Self::Composite { name, body } => deckmaste_core::Action::Composite {
                name: name.lower(),
                body: body.lower(),
            },
            Self::ChangeLife(patient, operation) => {
                deckmaste_core::Action::ChangeLife(patient.lower(), operation.lower())
            }
            Self::AddMana(recipient, count, production) => deckmaste_core::Action::AddMana(
                recipient.lower(),
                count.lower(),
                production.lower(),
            ),
            Self::Create {
                agent,
                count,
                token,
                riders,
            } => deckmaste_core::Action::Create {
                agent: agent.lower(),
                count: count.lower(),
                token: token.lower(),
                riders: riders.lower(),
            },
            Self::Sacrifice(agent, what) => {
                deckmaste_core::Action::Sacrifice(agent.lower(), what.lower())
            }
            Self::DrawCard(agent) => deckmaste_core::Action::DrawCard(agent.lower()),
            Self::Tap(what) => deckmaste_core::Action::Tap(what.lower()),
            Self::Untap(what) => deckmaste_core::Action::Untap(what.lower()),
            Self::GetEmblem(recipient, abilities) => {
                deckmaste_core::Action::GetEmblem(recipient.lower(), abilities.lower())
            }
            Self::GetDesignation(recipient, name) => {
                deckmaste_core::Action::GetDesignation(recipient.lower(), name.lower())
            }
            Self::SetGameDesignation(name, value) => {
                deckmaste_core::Action::SetGameDesignation(name.lower(), value.lower())
            }
            Self::ChooseValue(who, kind, note) => {
                deckmaste_core::Action::ChooseValue(who.lower(), kind.lower(), note.lower())
            }
            Self::CopySpell {
                controller,
                spec,
                retarget,
            } => deckmaste_core::Action::CopySpell {
                controller: controller.lower(),
                spec: spec.lower(),
                retarget: retarget.lower(),
            },
            Self::CastCopy(agent, spec) => {
                deckmaste_core::Action::CastCopy(agent.lower(), spec.lower())
            }
            Self::Cast(agent, what, cost) => {
                deckmaste_core::Action::Cast(agent.lower(), what.lower(), cost.lower())
            }
            Self::Retarget { mode, of, by } => deckmaste_core::Action::Retarget {
                mode: mode.lower(),
                of: of.lower(),
                by: by.lower(),
            },
            Self::FlipCoins(agent, count, called) => {
                deckmaste_core::Action::FlipCoins(agent.lower(), count.lower(), called.lower())
            }
            Self::RollDice(agent, count, sides) => {
                deckmaste_core::Action::RollDice(agent.lower(), count.lower(), sides.lower())
            }
            Self::RollPlanarDie(agent) => deckmaste_core::Action::RollPlanarDie(agent.lower()),
            Self::PutCounters(what, kind, count) => {
                deckmaste_core::Action::PutCounters(what.lower(), kind.lower(), count.lower())
            }
            Self::RemoveCounters(what, kind, count) => {
                deckmaste_core::Action::RemoveCounters(what.lower(), kind.lower(), count.lower())
            }
            Self::WinGame(who) => deckmaste_core::Action::WinGame(who.lower()),
            Self::LoseGame(who) => deckmaste_core::Action::LoseGame(who.lower()),
            Self::RestartGame => deckmaste_core::Action::RestartGame,
            Self::Shuffle(what) => deckmaste_core::Action::Shuffle(what.lower()),
            Self::Reveal { what, to } => deckmaste_core::Action::Reveal {
                what: what.lower(),
                to: to.lower(),
            },
            Self::RemoveDamage(what) => deckmaste_core::Action::RemoveDamage(what.lower()),
            Self::Pay(cost) => deckmaste_core::Action::Pay(cost.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(expansion) => *expansion.value.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::LifeOp {
    type Target = deckmaste_core::LifeOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(value) => deckmaste_core::LifeOp::Set(value.lower()),
            Self::Up(amount) => deckmaste_core::LifeOp::Up(amount.lower()),
            Self::Down(amount) => deckmaste_core::LifeOp::Down(amount.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::RetargetMode {
    type Target = deckmaste_core::RetargetMode;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ChangeAll => deckmaste_core::RetargetMode::ChangeAll,
            Self::ChangeOne => deckmaste_core::RetargetMode::ChangeOne,
            Self::ChangeAny => deckmaste_core::RetargetMode::ChangeAny,
            Self::ChooseNew => deckmaste_core::RetargetMode::ChooseNew,
        }
    }
}

impl Lower for deckmaste_semantics::CopyRetarget {
    type Target = deckmaste_core::CopyRetarget;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AsIs => deckmaste_core::CopyRetarget::AsIs,
            Self::MayChooseNew => deckmaste_core::CopyRetarget::MayChooseNew,
            Self::TargetsThat(what) => deckmaste_core::CopyRetarget::TargetsThat(what.lower()),
        }
    }
}
