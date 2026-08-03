//! `action` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

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
            Self::BecomeDay => deckmaste_core::Action::BecomeDay,
            Self::BecomeNight => deckmaste_core::Action::BecomeNight,
            Self::TheRingTempts(f0) => deckmaste_core::Action::TheRingTempts(f0.lower()),
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
            Self::VentureIntoDungeon(f0) => deckmaste_core::Action::VentureIntoDungeon(f0.lower()),
            Self::Tap(f0) => deckmaste_core::Action::Tap(f0.lower()),
            Self::Untap(f0) => deckmaste_core::Action::Untap(f0.lower()),
            Self::GetEmblem(f0, f1) => deckmaste_core::Action::GetEmblem(f0.lower(), f1.lower()),
            Self::GetDesignation(f0, f1) => {
                deckmaste_core::Action::GetDesignation(f0.lower(), f1.lower())
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
            Self::Expanded(f0) => deckmaste_core::Action::Expanded(f0.lower()),
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
