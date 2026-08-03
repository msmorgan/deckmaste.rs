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

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_card_face() {
        assert_lowers(deckmaste_authoring::CardFace {
            name: "x".into(),
            mana_cost: deckmaste_authoring::ManaCost::from(std::sync::Arc::<
                [deckmaste_authoring::ManaSymbol],
            >::from([])),
            color_indicator: Vec::new(),
            supertypes: Vec::new(),
            types: Vec::new(),
            subtypes: Vec::new(),
            abilities: Vec::new(),
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
        });
    }

    #[test]
    fn lowers_face_layout_transforming() {
        assert_lowers(deckmaste_authoring::FaceLayout::Transforming);
    }

    #[test]
    fn lowers_face_layout_modal_dfc() {
        assert_lowers(deckmaste_authoring::FaceLayout::ModalDfc);
    }

    #[test]
    fn lowers_face_layout_split() {
        assert_lowers(deckmaste_authoring::FaceLayout::Split);
    }

    #[test]
    fn lowers_face_layout_adventure() {
        assert_lowers(deckmaste_authoring::FaceLayout::Adventure);
    }

    #[test]
    fn lowers_face_layout_flip() {
        assert_lowers(deckmaste_authoring::FaceLayout::Flip);
    }

    #[test]
    fn lowers_card_normal() {
        assert_lowers(deckmaste_authoring::Card::Normal(minimal_card_face()));
    }

    #[test]
    fn lowers_card_two_faced() {
        assert_lowers(deckmaste_authoring::Card::TwoFaced {
            layout: minimal_face_layout(),
            front: minimal_card_face(),
            back: minimal_card_face(),
        });
    }
}
