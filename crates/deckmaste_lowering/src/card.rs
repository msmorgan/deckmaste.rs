//! `card` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::CardFace {
    type Target = deckmaste_card::CardFace;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_card::CardFace {
            name: self.name.lower(),
            mana_cost: self.mana_cost.lower(),
            color_indicator: self.color_indicator.lower(),
            supertypes: self.supertypes.lower(),
            types: self.types.lower(),
            subtypes: self.subtypes.lower(),
            abilities: self.abilities.lower(),
            power: self.power.lower(),
            toughness: self.toughness.lower(),
            loyalty: self.loyalty.lower(),
            defense: self.defense.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::FaceLayout {
    type Target = deckmaste_card::FaceLayout;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Transforming => deckmaste_card::FaceLayout::Transforming,
            Self::ModalDfc => deckmaste_card::FaceLayout::ModalDfc,
            Self::Split => deckmaste_card::FaceLayout::Split,
            Self::Adventure => deckmaste_card::FaceLayout::Adventure,
            Self::Flip => deckmaste_card::FaceLayout::Flip,
        }
    }
}

impl Lower for deckmaste_authoring::Card {
    type Target = deckmaste_card::Card;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Normal(f0) => deckmaste_card::Card::Normal(f0.lower()),
            Self::TwoFaced {
                layout,
                front,
                back,
            } => deckmaste_card::Card::TwoFaced {
                layout: layout.lower(),
                front: front.lower(),
                back: back.lower(),
            },
        }
    }
}
