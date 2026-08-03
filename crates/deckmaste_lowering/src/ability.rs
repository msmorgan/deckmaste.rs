//! `ability` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::SpellAbility {
    type Target = deckmaste_core::SpellAbility;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::SpellAbility {
            ability_word: self.ability_word.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::ActivatedAbility {
    type Target = deckmaste_core::ActivatedAbility;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::ActivatedAbility {
            ability_word: self.ability_word.lower(),
            cost: self.cost.lower(),
            from: self.from.lower(),
            window: self.window.lower(),
            condition: self.condition.lower(),
            limits: self.limits.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::UseLimit {
    type Target = deckmaste_core::UseLimit;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::OncePerTurn => deckmaste_core::UseLimit::OncePerTurn,
            Self::OncePerGame => deckmaste_core::UseLimit::OncePerGame,
            Self::LoyaltyOncePerTurn => deckmaste_core::UseLimit::LoyaltyOncePerTurn,
        }
    }
}

impl Lower for deckmaste_authoring::TriggeredAbility {
    type Target = deckmaste_core::TriggeredAbility;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TriggeredAbility {
            ability_word: self.ability_word.lower(),
            event: self.event.lower(),
            from: self.from.lower(),
            condition: self.condition.lower(),
            limits: self.limits.lower(),
            where_x: self.where_x.lower(),
            effect: self.effect.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::ChooseSpec {
    type Target = deckmaste_core::ChooseSpec;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::ChooseSpec {
            count: self.count.lower(),
            up_to: self.up_to.lower(),
            repeats: self.repeats.lower(),
            chooser: self.chooser.lower(),
            rider: self.rider.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::ModalCostRider {
    type Target = deckmaste_core::ModalCostRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Entwine(f0) => deckmaste_core::ModalCostRider::Entwine(f0.lower()),
            Self::Escalate(f0) => deckmaste_core::ModalCostRider::Escalate(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::Mode {
    type Target = deckmaste_core::Mode;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Mode {
            effect: self.effect.lower(),
            cost: self.cost.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::Ability {
    type Target = deckmaste_core::Ability;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Static(f0) => deckmaste_core::Ability::Static(f0.lower()),
            Self::Activated(f0) => deckmaste_core::Ability::Activated(f0.lower()),
            Self::Triggered(f0) => deckmaste_core::Ability::Triggered(f0.lower()),
            Self::Spell(f0) => deckmaste_core::Ability::Spell(f0.lower()),
            Self::Keyword(f0) => deckmaste_core::Ability::Keyword(f0.lower()),
            Self::Innate(f0) => deckmaste_core::Ability::Innate(f0.lower()),
            Self::Expanded(f0) => deckmaste_core::Ability::Expanded(f0.lower()),
        }
    }
}
