//! `action` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::Anchor {
    type Target = deckmaste_core::Anchor;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FromTop(f0) => deckmaste_core::Anchor::FromTop(f0.lower()),
            Self::FromBottom(f0) => deckmaste_core::Anchor::FromBottom(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Destination {
    type Target = deckmaste_core::Destination;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Zone(f0) => deckmaste_core::Destination::Zone(f0.lower()),
            Self::Library(f0) => deckmaste_core::Destination::Library(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::EnterRider {
    type Target = deckmaste_core::EnterRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Tapped => deckmaste_core::EnterRider::Tapped,
            Self::FaceDown => deckmaste_core::EnterRider::FaceDown,
            Self::UnderControlOf(f0) => deckmaste_core::EnterRider::UnderControlOf(f0.lower()),
            Self::UnderOwnersControl => deckmaste_core::EnterRider::UnderOwnersControl,
            Self::Attacking(f0) => deckmaste_core::EnterRider::Attacking(f0.lower()),
            Self::WithCounters(f0, f1) => {
                deckmaste_core::EnterRider::WithCounters(f0.lower(), f1.lower())
            }
            Self::AsCopy(f0) => deckmaste_core::EnterRider::AsCopy(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Arrangement {
    type Target = deckmaste_core::Arrangement;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ChosenOrder(f0) => deckmaste_core::Arrangement::ChosenOrder(f0.lower()),
            Self::AnyOrder => deckmaste_core::Arrangement::AnyOrder,
            Self::SameOrder => deckmaste_core::Arrangement::SameOrder,
            Self::RandomOrder => deckmaste_core::Arrangement::RandomOrder,
        }
    }
}

impl Lower for deckmaste_authoring::Action {
    type Target = deckmaste_core::Action;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::DealDamage(f0, f1, f2) => {
                deckmaste_core::Action::DealDamage(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Counter(f0) => deckmaste_core::Action::Counter(f0.lower()),
            Self::Transform(f0) => deckmaste_core::Action::Transform(f0.lower()),
            Self::Cease(f0) => deckmaste_core::Action::Cease(f0.lower()),
            Self::Attach { what, to } => deckmaste_core::Action::Attach {
                what: what.lower(),
                to: to.lower(),
            },
            Self::Unattach(f0) => deckmaste_core::Action::Unattach(f0.lower()),
            Self::Move(f0, f1, f2, f3) => {
                deckmaste_core::Action::Move(f0.lower(), f1.lower(), f2.lower(), f3.lower())
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
            Self::GainControl(f0, f1) => {
                deckmaste_core::Action::GainControl(f0.lower(), f1.lower())
            }
            Self::ExtraPhase(f0, f1) => deckmaste_core::Action::ExtraPhase(f0.lower(), f1.lower()),
            Self::MoveCounters(f0, f1, f2) => {
                deckmaste_core::Action::MoveCounters(f0.lower(), f1.lower(), f2.lower())
            }
            Self::CreateReplacement {
                replacement,
                duration,
                one_shot,
            } => deckmaste_core::Action::CreateReplacement {
                replacement: replacement.lower(),
                duration: duration.lower(),
                one_shot: one_shot.lower(),
            },
            Self::Composite { name, body } => deckmaste_core::Action::Composite {
                name: name.lower(),
                body: body.lower(),
            },
            Self::ChangeLife(f0, f1) => deckmaste_core::Action::ChangeLife(f0.lower(), f1.lower()),
            Self::AddMana(f0, f1, f2) => {
                deckmaste_core::Action::AddMana(f0.lower(), f1.lower(), f2.lower())
            }
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
            Self::Sacrifice(f0, f1) => deckmaste_core::Action::Sacrifice(f0.lower(), f1.lower()),
            Self::DrawCard(f0) => deckmaste_core::Action::DrawCard(f0.lower()),
            Self::Tap(f0) => deckmaste_core::Action::Tap(f0.lower()),
            Self::Untap(f0) => deckmaste_core::Action::Untap(f0.lower()),
            Self::GetEmblem(f0, f1) => deckmaste_core::Action::GetEmblem(f0.lower(), f1.lower()),
            Self::GetDesignation(f0, f1) => {
                deckmaste_core::Action::GetDesignation(f0.lower(), f1.lower())
            }
            Self::SetGameDesignation(f0, f1) => {
                deckmaste_core::Action::SetGameDesignation(f0.lower(), f1.lower())
            }
            Self::ChooseValue(f0, f1, f2) => {
                deckmaste_core::Action::ChooseValue(f0.lower(), f1.lower(), f2.lower())
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
            Self::CastCopy(f0, f1) => deckmaste_core::Action::CastCopy(f0.lower(), f1.lower()),
            Self::Cast(f0, f1, f2) => {
                deckmaste_core::Action::Cast(f0.lower(), f1.lower(), f2.lower())
            }
            Self::Retarget { mode, of, by } => deckmaste_core::Action::Retarget {
                mode: mode.lower(),
                of: of.lower(),
                by: by.lower(),
            },
            Self::FlipCoins(f0, f1, f2) => {
                deckmaste_core::Action::FlipCoins(f0.lower(), f1.lower(), f2.lower())
            }
            Self::RollDice(f0, f1, f2) => {
                deckmaste_core::Action::RollDice(f0.lower(), f1.lower(), f2.lower())
            }
            Self::RollPlanarDie(f0) => deckmaste_core::Action::RollPlanarDie(f0.lower()),
            Self::PutCounters(f0, f1, f2) => {
                deckmaste_core::Action::PutCounters(f0.lower(), f1.lower(), f2.lower())
            }
            Self::RemoveCounters(f0, f1, f2) => {
                deckmaste_core::Action::RemoveCounters(f0.lower(), f1.lower(), f2.lower())
            }
            Self::WinGame(f0) => deckmaste_core::Action::WinGame(f0.lower()),
            Self::LoseGame(f0) => deckmaste_core::Action::LoseGame(f0.lower()),
            Self::RestartGame => deckmaste_core::Action::RestartGame,
            Self::Shuffle(f0) => deckmaste_core::Action::Shuffle(f0.lower()),
            Self::Reveal { what, to } => deckmaste_core::Action::Reveal {
                what: what.lower(),
                to: to.lower(),
            },
            Self::RemoveDamage(f0) => deckmaste_core::Action::RemoveDamage(f0.lower()),
            Self::Pay(f0) => deckmaste_core::Action::Pay(f0.lower()),
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the authored
            // spelling (spec §12). Prose recovers the authored term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::LifeOp {
    type Target = deckmaste_core::LifeOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => deckmaste_core::LifeOp::Set(f0.lower()),
            Self::Up(f0) => deckmaste_core::LifeOp::Up(f0.lower()),
            Self::Down(f0) => deckmaste_core::LifeOp::Down(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::RetargetMode {
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

impl Lower for deckmaste_authoring::CopyRetarget {
    type Target = deckmaste_core::CopyRetarget;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AsIs => deckmaste_core::CopyRetarget::AsIs,
            Self::MayChooseNew => deckmaste_core::CopyRetarget::MayChooseNew,
            Self::TargetsThat(f0) => deckmaste_core::CopyRetarget::TargetsThat(f0.lower()),
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
            deckmaste_authoring::Anchor::FromTop(minimal_count()).lower(),
            deckmaste_core::Anchor::FromTop(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_anchor_from_bottom() {
        assert_matches!(
            deckmaste_authoring::Anchor::FromBottom(minimal_count()).lower(),
            deckmaste_core::Anchor::FromBottom(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_destination_zone() {
        assert_matches!(
            deckmaste_authoring::Destination::Zone(minimal_zone()).lower(),
            deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield)
        );
    }

    #[test]
    fn lowers_destination_library() {
        assert_matches!(
            deckmaste_authoring::Destination::Library(minimal_anchor()).lower(),
            deckmaste_core::Destination::Library(deckmaste_core::Anchor::FromTop(
                deckmaste_core::Count::X
            ))
        );
    }

    #[test]
    fn lowers_enter_rider_tapped() {
        assert_matches!(
            deckmaste_authoring::EnterRider::Tapped.lower(),
            deckmaste_core::EnterRider::Tapped
        );
    }

    #[test]
    fn lowers_enter_rider_face_down() {
        assert_matches!(
            deckmaste_authoring::EnterRider::FaceDown.lower(),
            deckmaste_core::EnterRider::FaceDown
        );
    }

    #[test]
    fn lowers_enter_rider_under_control_of() {
        assert_matches!(
            deckmaste_authoring::EnterRider::UnderControlOf(minimal_reference()).lower(),
            deckmaste_core::EnterRider::UnderControlOf(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_enter_rider_under_owners_control() {
        assert_matches!(
            deckmaste_authoring::EnterRider::UnderOwnersControl.lower(),
            deckmaste_core::EnterRider::UnderOwnersControl
        );
    }

    #[test]
    fn lowers_enter_rider_attacking() {
        assert_matches!(
            deckmaste_authoring::EnterRider::Attacking(None).lower(),
            deckmaste_core::EnterRider::Attacking(None)
        );
    }

    #[test]
    fn lowers_enter_rider_with_counters() {
        assert_matches!(
            deckmaste_authoring::EnterRider::WithCounters(minimal_counter_ref(), minimal_count())
                .lower(),
            deckmaste_core::EnterRider::WithCounters(
                deckmaste_core::CounterRef(_),
                deckmaste_core::Count::X
            )
        );
    }

    #[test]
    fn lowers_enter_rider_as_copy() {
        assert_matches!(
            deckmaste_authoring::EnterRider::AsCopy(minimal_copy_spec()).lower(),
            deckmaste_core::EnterRider::AsCopy(deckmaste_core::CopySpec {
                source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::This),
                exceptions: _
            })
        );
    }

    #[test]
    fn lowers_arrangement_chosen_order() {
        assert_matches!(
            deckmaste_authoring::Arrangement::ChosenOrder(minimal_reference()).lower(),
            deckmaste_core::Arrangement::ChosenOrder(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_arrangement_any_order() {
        assert_matches!(
            deckmaste_authoring::Arrangement::AnyOrder.lower(),
            deckmaste_core::Arrangement::AnyOrder
        );
    }

    #[test]
    fn lowers_arrangement_same_order() {
        assert_matches!(
            deckmaste_authoring::Arrangement::SameOrder.lower(),
            deckmaste_core::Arrangement::SameOrder
        );
    }

    #[test]
    fn lowers_arrangement_random_order() {
        assert_matches!(
            deckmaste_authoring::Arrangement::RandomOrder.lower(),
            deckmaste_core::Arrangement::RandomOrder
        );
    }

    #[test]
    fn lowers_action_deal_damage() {
        assert_matches!(
            deckmaste_authoring::Action::DealDamage(
                minimal_reference(),
                minimal_count(),
                minimal_reference()
            )
            .lower(),
            deckmaste_core::Action::DealDamage(
                deckmaste_core::Reference::This,
                deckmaste_core::Count::X,
                deckmaste_core::Reference::This
            )
        );
    }

    #[test]
    fn lowers_action_counter() {
        assert_matches!(
            deckmaste_authoring::Action::Counter(minimal_reference()).lower(),
            deckmaste_core::Action::Counter(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_transform() {
        assert_matches!(
            deckmaste_authoring::Action::Transform(minimal_reference()).lower(),
            deckmaste_core::Action::Transform(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_cease() {
        assert_matches!(
            deckmaste_authoring::Action::Cease(minimal_reference()).lower(),
            deckmaste_core::Action::Cease(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_attach() {
        assert_matches!(
            deckmaste_authoring::Action::Attach {
                what: minimal_reference(),
                to: minimal_reference()
            }
            .lower(),
            deckmaste_core::Action::Attach {
                what: deckmaste_core::Reference::This,
                to: deckmaste_core::Reference::This
            }
        );
    }

    #[test]
    fn lowers_action_unattach() {
        assert_matches!(
            deckmaste_authoring::Action::Unattach(minimal_reference()).lower(),
            deckmaste_core::Action::Unattach(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_move() {
        assert_matches!(
            deckmaste_authoring::Action::Move(
                minimal_reference(),
                minimal_destination(),
                [].into(),
                None
            )
            .lower(),
            deckmaste_core::Action::Move(
                deckmaste_core::Reference::This,
                deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield),
                _,
                None
            )
        );
    }

    #[test]
    fn lowers_action_move_group() {
        assert_matches!(
            deckmaste_authoring::Action::MoveGroup {
                group: minimal_selection(),
                arrangement: minimal_arrangement(),
                to: minimal_destination(),
                riders: [].into()
            }
            .lower(),
            deckmaste_core::Action::MoveGroup {
                group: deckmaste_core::Selection::SelectAll(deckmaste_core::Predicate::Kind(
                    deckmaste_core::ObjectKind::Ability
                )),
                arrangement: deckmaste_core::Arrangement::ChosenOrder(
                    deckmaste_core::Reference::This
                ),
                to: deckmaste_core::Destination::Zone(deckmaste_core::Zone::Battlefield),
                riders: _
            }
        );
    }

    #[test]
    fn lowers_action_gain_control() {
        assert_matches!(
            deckmaste_authoring::Action::GainControl(minimal_reference(), minimal_reference())
                .lower(),
            deckmaste_core::Action::GainControl(
                deckmaste_core::Reference::This,
                deckmaste_core::Reference::This
            )
        );
    }

    #[test]
    fn lowers_action_extra_phase() {
        assert_matches!(
            deckmaste_authoring::Action::ExtraPhase(minimal_phase_kind(), minimal_reference())
                .lower(),
            deckmaste_core::Action::ExtraPhase(
                deckmaste_core::PhaseKind::Beginning,
                deckmaste_core::Reference::This
            )
        );
    }

    #[test]
    fn lowers_action_move_counters() {
        assert_matches!(
            deckmaste_authoring::Action::MoveCounters(
                minimal_counter_spec(),
                minimal_reference(),
                minimal_reference()
            )
            .lower(),
            deckmaste_core::Action::MoveCounters(
                deckmaste_core::CounterSpec::Named(
                    deckmaste_core::CounterRef(_),
                    deckmaste_core::Count::X
                ),
                deckmaste_core::Reference::This,
                deckmaste_core::Reference::This
            )
        );
    }

    #[test]
    fn lowers_action_create_replacement() {
        assert_matches!(
            deckmaste_authoring::Action::CreateReplacement {
                replacement: std::sync::Arc::new(minimal_replacement()),
                duration: minimal_duration(),
                one_shot: false
            }
            .lower(),
            deckmaste_core::Action::CreateReplacement {
                replacement: _,
                duration: deckmaste_core::Duration::FixedUntil(
                    deckmaste_core::TurnMarker::EndOfTurn
                ),
                one_shot: false
            }
        );
    }

    #[test]
    fn lowers_action_composite() {
        assert_matches!(
            deckmaste_authoring::Action::Composite {
                name: minimal_verb_name(),
                body: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Action::Composite {
                name: deckmaste_core::VerbName(_),
                body: _
            }
        );
    }

    #[test]
    fn lowers_action_change_life() {
        assert_matches!(
            deckmaste_authoring::Action::ChangeLife(minimal_reference(), minimal_life_op()).lower(),
            deckmaste_core::Action::ChangeLife(
                deckmaste_core::Reference::This,
                deckmaste_core::LifeOp::Set(deckmaste_core::Count::X)
            )
        );
    }

    #[test]
    fn lowers_action_add_mana() {
        assert_matches!(
            deckmaste_authoring::Action::AddMana(
                minimal_reference(),
                minimal_count(),
                minimal_mana_production()
            )
            .lower(),
            deckmaste_core::Action::AddMana(
                deckmaste_core::Reference::This,
                deckmaste_core::Count::X,
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
            deckmaste_authoring::Action::Create {
                agent: minimal_reference(),
                count: minimal_count(),
                token: minimal_token_spec(),
                riders: [].into()
            }
            .lower(),
            deckmaste_core::Action::Create {
                agent: deckmaste_core::Reference::This,
                count: deckmaste_core::Count::X,
                token: deckmaste_core::TokenSpec::Token(_),
                riders: _
            }
        );
    }

    #[test]
    fn lowers_action_sacrifice() {
        assert_matches!(
            deckmaste_authoring::Action::Sacrifice(minimal_reference(), minimal_reference())
                .lower(),
            deckmaste_core::Action::Sacrifice(
                deckmaste_core::Reference::This,
                deckmaste_core::Reference::This
            )
        );
    }

    #[test]
    fn lowers_action_draw_card() {
        assert_matches!(
            deckmaste_authoring::Action::DrawCard(minimal_reference()).lower(),
            deckmaste_core::Action::DrawCard(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_tap() {
        assert_matches!(
            deckmaste_authoring::Action::Tap(minimal_reference()).lower(),
            deckmaste_core::Action::Tap(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_untap() {
        assert_matches!(
            deckmaste_authoring::Action::Untap(minimal_reference()).lower(),
            deckmaste_core::Action::Untap(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_get_emblem() {
        assert_matches!(
            deckmaste_authoring::Action::GetEmblem(minimal_reference(), [].into()).lower(),
            deckmaste_core::Action::GetEmblem(deckmaste_core::Reference::This, _)
        );
    }

    #[test]
    fn lowers_action_get_designation() {
        assert_matches!(
            deckmaste_authoring::Action::GetDesignation(minimal_reference(), "X".into()).lower(),
            deckmaste_core::Action::GetDesignation(deckmaste_core::Reference::This, _)
        );
    }

    #[test]
    fn lowers_action_set_game_designation() {
        assert_matches!(
            deckmaste_authoring::Action::SetGameDesignation("X".into(), "Y".into()).lower(),
            deckmaste_core::Action::SetGameDesignation(name, value)
                if name.as_ref() == "X" && value.as_ref() == "Y"
        );
    }

    #[test]
    fn lowers_action_choose_value() {
        assert_matches!(
            deckmaste_authoring::Action::ChooseValue(
                minimal_reference(),
                minimal_chosen_value_kind(),
                "X".into()
            )
            .lower(),
            deckmaste_core::Action::ChooseValue(
                deckmaste_core::Reference::This,
                deckmaste_core::ChosenValueKind::Color,
                _
            )
        );
    }

    #[test]
    fn lowers_action_copy_spell() {
        assert_matches!(
            deckmaste_authoring::Action::CopySpell {
                controller: minimal_reference(),
                spec: minimal_copy_spec(),
                retarget: minimal_copy_retarget()
            }
            .lower(),
            deckmaste_core::Action::CopySpell {
                controller: deckmaste_core::Reference::This,
                spec: deckmaste_core::CopySpec {
                    source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::This),
                    exceptions: _
                },
                retarget: deckmaste_core::CopyRetarget::AsIs
            }
        );
    }

    #[test]
    fn lowers_action_cast_copy() {
        assert_matches!(
            deckmaste_authoring::Action::CastCopy(minimal_reference(), minimal_copy_spec()).lower(),
            deckmaste_core::Action::CastCopy(
                deckmaste_core::Reference::This,
                deckmaste_core::CopySpec {
                    source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::This),
                    exceptions: _
                }
            )
        );
    }

    #[test]
    fn lowers_action_cast() {
        assert_matches!(
            deckmaste_authoring::Action::Cast(minimal_reference(), minimal_reference(), None)
                .lower(),
            deckmaste_core::Action::Cast(
                deckmaste_core::Reference::This,
                deckmaste_core::Reference::This,
                None
            )
        );
    }

    #[test]
    fn lowers_action_retarget() {
        assert_matches!(
            deckmaste_authoring::Action::Retarget {
                mode: minimal_retarget_mode(),
                of: minimal_reference(),
                by: minimal_reference()
            }
            .lower(),
            deckmaste_core::Action::Retarget {
                mode: deckmaste_core::RetargetMode::ChangeAll,
                of: deckmaste_core::Reference::This,
                by: deckmaste_core::Reference::This
            }
        );
    }

    #[test]
    fn lowers_action_flip_coins() {
        assert_matches!(
            deckmaste_authoring::Action::FlipCoins(minimal_reference(), minimal_count(), false)
                .lower(),
            deckmaste_core::Action::FlipCoins(
                deckmaste_core::Reference::This,
                deckmaste_core::Count::X,
                false
            )
        );
    }

    #[test]
    fn lowers_action_roll_dice() {
        assert_matches!(
            deckmaste_authoring::Action::RollDice(minimal_reference(), minimal_count(), 0).lower(),
            deckmaste_core::Action::RollDice(
                deckmaste_core::Reference::This,
                deckmaste_core::Count::X,
                0
            )
        );
    }

    #[test]
    fn lowers_action_roll_planar_die() {
        assert_matches!(
            deckmaste_authoring::Action::RollPlanarDie(minimal_reference()).lower(),
            deckmaste_core::Action::RollPlanarDie(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_put_counters() {
        assert_matches!(
            deckmaste_authoring::Action::PutCounters(
                minimal_reference(),
                minimal_counter_ref(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Action::PutCounters(
                deckmaste_core::Reference::This,
                deckmaste_core::CounterRef(_),
                deckmaste_core::Count::X
            )
        );
    }

    #[test]
    fn lowers_action_remove_counters() {
        assert_matches!(
            deckmaste_authoring::Action::RemoveCounters(
                minimal_reference(),
                minimal_counter_ref(),
                minimal_count()
            )
            .lower(),
            deckmaste_core::Action::RemoveCounters(
                deckmaste_core::Reference::This,
                deckmaste_core::CounterRef(_),
                deckmaste_core::Count::X
            )
        );
    }

    #[test]
    fn lowers_action_win_game() {
        assert_matches!(
            deckmaste_authoring::Action::WinGame(minimal_reference()).lower(),
            deckmaste_core::Action::WinGame(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_lose_game() {
        assert_matches!(
            deckmaste_authoring::Action::LoseGame(minimal_reference()).lower(),
            deckmaste_core::Action::LoseGame(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_restart_game() {
        assert_matches!(
            deckmaste_authoring::Action::RestartGame.lower(),
            deckmaste_core::Action::RestartGame
        );
    }

    #[test]
    fn lowers_action_shuffle() {
        assert_matches!(
            deckmaste_authoring::Action::Shuffle(minimal_selection()).lower(),
            deckmaste_core::Action::Shuffle(deckmaste_core::Selection::SelectAll(
                deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            ))
        );
    }

    #[test]
    fn lowers_action_reveal() {
        assert_matches!(
            deckmaste_authoring::Action::Reveal {
                what: minimal_reference(),
                to: None
            }
            .lower(),
            deckmaste_core::Action::Reveal {
                what: deckmaste_core::Reference::This,
                to: None
            }
        );
    }

    #[test]
    fn lowers_action_remove_damage() {
        assert_matches!(
            deckmaste_authoring::Action::RemoveDamage(minimal_reference()).lower(),
            deckmaste_core::Action::RemoveDamage(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_action_pay() {
        assert_matches!(
            deckmaste_authoring::Action::Pay(minimal_cost()).lower(),
            deckmaste_core::Action::Pay(deckmaste_core::Cost(_))
        );
    }

    #[test]
    fn lowers_action_expanded() {
        assert_matches!(
            deckmaste_authoring::Action::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_action())
            })
            .lower(),
            deckmaste_core::Action::DealDamage(
                deckmaste_core::Reference::This,
                deckmaste_core::Count::X,
                deckmaste_core::Reference::This
            )
        );
    }

    #[test]
    fn lowers_life_op_set() {
        assert_matches!(
            deckmaste_authoring::LifeOp::Set(minimal_count()).lower(),
            deckmaste_core::LifeOp::Set(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_life_op_up() {
        assert_matches!(
            deckmaste_authoring::LifeOp::Up(minimal_count()).lower(),
            deckmaste_core::LifeOp::Up(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_life_op_down() {
        assert_matches!(
            deckmaste_authoring::LifeOp::Down(minimal_count()).lower(),
            deckmaste_core::LifeOp::Down(deckmaste_core::Count::X)
        );
    }

    #[test]
    fn lowers_retarget_mode_change_all() {
        assert_matches!(
            deckmaste_authoring::RetargetMode::ChangeAll.lower(),
            deckmaste_core::RetargetMode::ChangeAll
        );
    }

    #[test]
    fn lowers_retarget_mode_change_one() {
        assert_matches!(
            deckmaste_authoring::RetargetMode::ChangeOne.lower(),
            deckmaste_core::RetargetMode::ChangeOne
        );
    }

    #[test]
    fn lowers_retarget_mode_change_any() {
        assert_matches!(
            deckmaste_authoring::RetargetMode::ChangeAny.lower(),
            deckmaste_core::RetargetMode::ChangeAny
        );
    }

    #[test]
    fn lowers_retarget_mode_choose_new() {
        assert_matches!(
            deckmaste_authoring::RetargetMode::ChooseNew.lower(),
            deckmaste_core::RetargetMode::ChooseNew
        );
    }

    #[test]
    fn lowers_copy_retarget_as_is() {
        assert_matches!(
            deckmaste_authoring::CopyRetarget::AsIs.lower(),
            deckmaste_core::CopyRetarget::AsIs
        );
    }

    #[test]
    fn lowers_copy_retarget_may_choose_new() {
        assert_matches!(
            deckmaste_authoring::CopyRetarget::MayChooseNew.lower(),
            deckmaste_core::CopyRetarget::MayChooseNew
        );
    }

    #[test]
    fn lowers_copy_retarget_targets_that() {
        assert_matches!(
            deckmaste_authoring::CopyRetarget::TargetsThat(minimal_reference()).lower(),
            deckmaste_core::CopyRetarget::TargetsThat(deckmaste_core::Reference::This)
        );
    }
}
