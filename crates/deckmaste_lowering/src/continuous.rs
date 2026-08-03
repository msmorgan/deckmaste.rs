//! `continuous` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Duration {
    type Target = deckmaste_core::Duration;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FixedUntil(f0) => deckmaste_core::Duration::FixedUntil(f0.lower()),
            Self::UntilEvent(f0) => deckmaste_core::Duration::UntilEvent(f0.lower()),
            Self::ForAsLongAs(f0) => deckmaste_core::Duration::ForAsLongAs(f0.lower()),
            Self::ForThisEvent => deckmaste_core::Duration::ForThisEvent,
            Self::EndOfGame => deckmaste_core::Duration::EndOfGame,
        }
    }
}

impl Lower for deckmaste_authoring::NumericOp {
    type Target = deckmaste_core::NumericOp;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => deckmaste_core::NumericOp::Set(f0.lower()),
            Self::Up(f0) => deckmaste_core::NumericOp::Up(f0.lower()),
            Self::Down(f0) => deckmaste_core::NumericOp::Down(f0.lower()),
        }
    }
}

impl<T: Lower + Clone> Lower for deckmaste_authoring::CollectionOp<T> {
    type Target = deckmaste_core::CollectionOp<<T as Lower>::Target>;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Set(f0) => deckmaste_core::CollectionOp::Set(f0.lower()),
            Self::Add(f0) => deckmaste_core::CollectionOp::Add(f0.lower()),
            Self::Remove(f0) => deckmaste_core::CollectionOp::Remove(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Modification {
    type Target = deckmaste_core::Modification;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Power(f0) => deckmaste_core::Modification::Power(f0.lower()),
            Self::Toughness(f0) => deckmaste_core::Modification::Toughness(f0.lower()),
            Self::SwitchPowerToughness => deckmaste_core::Modification::SwitchPowerToughness,
            Self::Colors(f0) => deckmaste_core::Modification::Colors(f0.lower()),
            Self::CardTypes(f0) => deckmaste_core::Modification::CardTypes(f0.lower()),
            Self::Subtypes(f0) => deckmaste_core::Modification::Subtypes(f0.lower()),
            Self::Supertypes(f0) => deckmaste_core::Modification::Supertypes(f0.lower()),
            Self::GainAbility(f0) => deckmaste_core::Modification::GainAbility(f0.lower()),
            Self::LoseAbility(f0) => deckmaste_core::Modification::LoseAbility(f0.lower()),
            Self::LoseAllAbilities => deckmaste_core::Modification::LoseAllAbilities,
            Self::CantHaveAbility(f0) => deckmaste_core::Modification::CantHaveAbility(f0.lower()),
            Self::SetController(f0) => deckmaste_core::Modification::SetController(f0.lower()),
            Self::SetText(f0) => deckmaste_core::Modification::SetText(f0.lower()),
            Self::AllCreatureTypes => deckmaste_core::Modification::AllCreatureTypes,
            Self::BaseLoyalty(f0) => deckmaste_core::Modification::BaseLoyalty(f0.lower()),
            Self::BaseDefense(f0) => deckmaste_core::Modification::BaseDefense(f0.lower()),
            Self::BecomeBasicLandType(f0) => {
                deckmaste_core::Modification::BecomeBasicLandType(f0.lower())
            }
            Self::Several(f0) => deckmaste_core::Modification::Several(f0.lower()),
            Self::Expanded(f0) => deckmaste_core::Modification::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::CostChange {
    type Target = deckmaste_core::CostChange;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Increase(f0) => deckmaste_core::CostChange::Increase(f0.lower()),
            Self::Reduce(f0) => deckmaste_core::CostChange::Reduce(f0.lower()),
            Self::Additional { components } => deckmaste_core::CostChange::Additional {
                components: components.lower(),
            },
            Self::Scaled { change, times } => deckmaste_core::CostChange::Scaled {
                change: change.lower(),
                times: times.lower(),
            },
        }
    }
}

impl Lower for deckmaste_authoring::StaticEffect {
    type Target = deckmaste_core::StaticEffect;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Modify(f0, f1) => deckmaste_core::StaticEffect::Modify(f0.lower(), f1.lower()),
            Self::BecomesCopy(f0, f1) => {
                deckmaste_core::StaticEffect::BecomesCopy(f0.lower(), f1.lower())
            }
            Self::Each(f0, f1) => deckmaste_core::StaticEffect::Each(f0.lower(), f1.lower()),
            Self::Conditionally(f0, f1) => {
                deckmaste_core::StaticEffect::Conditionally(f0.lower(), f1.lower())
            }
            Self::Deontic(f0) => deckmaste_core::StaticEffect::Deontic(f0.lower()),
            Self::CostModifier { of, change } => deckmaste_core::StaticEffect::CostModifier {
                of: of.lower(),
                change: change.lower(),
            },
            Self::CostOption(f0) => deckmaste_core::StaticEffect::CostOption(f0.lower()),
            Self::TriggerMultiplier {
                cause,
                extra,
                affected,
            } => deckmaste_core::StaticEffect::TriggerMultiplier {
                cause: cause.lower(),
                extra: extra.lower(),
                affected: affected.lower(),
            },
            Self::ModifyPlayer(f0, f1) => {
                deckmaste_core::StaticEffect::ModifyPlayer(f0.lower(), f1.lower())
            }
            Self::Replacement(f0) => deckmaste_core::StaticEffect::Replacement(f0.lower()),
            Self::Prevention(f0) => deckmaste_core::StaticEffect::Prevention(f0.lower()),
            Self::CantPrevent { from, to } => deckmaste_core::StaticEffect::CantPrevent {
                from: from.lower(),
                to: to.lower(),
            },
            Self::SpendAsThough { mana_from, as_ } => deckmaste_core::StaticEffect::SpendAsThough {
                mana_from: mana_from.lower(),
                as_: as_.lower(),
            },
            Self::AsThough(f0) => deckmaste_core::StaticEffect::AsThough(f0.lower()),
            Self::Sba { when, then } => deckmaste_core::StaticEffect::Sba {
                when: when.lower(),
                then: then.lower(),
            },
            Self::OutcomeGate { who, gate } => deckmaste_core::StaticEffect::OutcomeGate {
                who: who.lower(),
                gate: gate.lower(),
            },
            Self::CantHappen(f0) => deckmaste_core::StaticEffect::CantHappen(f0.lower()),
            Self::ReplaceRoll {
                query,
                extra,
                ignore,
            } => deckmaste_core::StaticEffect::ReplaceRoll {
                query: query.lower(),
                extra: extra.lower(),
                ignore: ignore.lower(),
            },
            Self::PayPips(f0, f1) => deckmaste_core::StaticEffect::PayPips(f0.lower(), f1.lower()),
            Self::Expanded(f0) => deckmaste_core::StaticEffect::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::OutcomeGateKind {
    type Target = deckmaste_core::OutcomeGateKind;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::CantLose => deckmaste_core::OutcomeGateKind::CantLose,
            Self::CantWin => deckmaste_core::OutcomeGateKind::CantWin,
        }
    }
}

impl Lower for deckmaste_authoring::IgnoreRule {
    type Target = deckmaste_core::IgnoreRule;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::IgnoreLowest => deckmaste_core::IgnoreRule::IgnoreLowest,
            Self::IgnoreChosen(f0) => deckmaste_core::IgnoreRule::IgnoreChosen(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::PipClass {
    type Target = deckmaste_core::PipClass;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Generic => deckmaste_core::PipClass::Generic,
            Self::Colored(f0) => deckmaste_core::PipClass::Colored(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::PayAct {
    type Target = deckmaste_core::PayAct;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::TapToPay(f0) => deckmaste_core::PayAct::TapToPay(f0.lower()),
            Self::ExileToPay(f0) => deckmaste_core::PayAct::ExileToPay(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::PlayerAttr {
    type Target = deckmaste_core::PlayerAttr;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Life => deckmaste_core::PlayerAttr::Life,
            Self::HandSize => deckmaste_core::PlayerAttr::HandSize,
            Self::HandSizeLimit => deckmaste_core::PlayerAttr::HandSizeLimit,
            Self::LandPlaysPerTurn => deckmaste_core::PlayerAttr::LandPlaysPerTurn,
        }
    }
}

impl Lower for deckmaste_authoring::PlayerMod {
    type Target = deckmaste_core::PlayerMod;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SetTo(f0, f1) => deckmaste_core::PlayerMod::SetTo(f0.lower(), f1.lower()),
            Self::Raise(f0, f1) => deckmaste_core::PlayerMod::Raise(f0.lower(), f1.lower()),
            Self::Lower(f0, f1) => deckmaste_core::PlayerMod::Lower(f0.lower(), f1.lower()),
            Self::NoMax(f0) => deckmaste_core::PlayerMod::NoMax(f0.lower()),
        }
    }
}
