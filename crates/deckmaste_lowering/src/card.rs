//! `card` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

// The grammar's `CardFace` is a characteristic set: `TwoFaced`'s `back` is a
// flip card's alternative characteristics [CR#710.1] or an Adventure's
// [CR#715.2] as often as it is a face, so this lowers to `Characteristics` and
// the `Card` impl below wraps the ones that really are faces.
impl Lower for deckmaste_semantics::CardFace {
    type Target = deckmaste_card::Characteristics;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_card::Characteristics {
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

impl Lower for deckmaste_semantics::Card {
    type Target = deckmaste_card::Card;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Normal(f0) => deckmaste_card::Card::Normal(f0.lower().into()),
            // The grammar's `TwoFaced { layout, front, back }` packs all four
            // non-normal forms behind one shape; `layout` is what tells them
            // apart, so lowering fans out into core's distinct forms here
            // rather than carrying that ambiguity into `deckmaste_card`.
            Self::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Transforming,
                front,
                back,
            } => deckmaste_card::Card::DoubleFaced {
                layout: deckmaste_card::DoubleFacedLayout::Transforming,
                front: front.lower().into(),
                back: back.lower().into(),
            },
            Self::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::ModalDfc,
                front,
                back,
            } => deckmaste_card::Card::DoubleFaced {
                layout: deckmaste_card::DoubleFacedLayout::ModalDfc,
                front: front.lower().into(),
                back: back.lower().into(),
            },
            // Split cards have two card faces on one side [CR#709.1].
            Self::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Split,
                front,
                back,
            } => deckmaste_card::Card::Split {
                left: front.lower().into(),
                right: back.lower().into(),
            },
            // Flip cards have one card face plus alternative characteristics
            // used according to flipped status, not a second face
            // [CR#710.1].
            Self::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Flip,
                front,
                back,
            } => deckmaste_card::Card::Flip {
                normal: front.lower().into(),
                alternative: back.lower(),
            },
            // Adventurer cards have normal characteristics plus alternative
            // Adventure characteristics, not an Adventure face [CR#715.2].
            Self::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Adventure,
                front,
                back,
            } => deckmaste_card::Card::Adventurer {
                normal: front.lower().into(),
                adventure: back.lower(),
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
    fn lowers_card_face() {
        assert_matches!(
            deckmaste_semantics::CardFace {
                name: "x".into(),
                mana_cost: deckmaste_semantics::ManaCost::from(std::sync::Arc::<
                    [deckmaste_semantics::ManaSymbol],
                >::from([])),
                color_indicator: Vec::new(),
                supertypes: Vec::new(),
                types: Vec::new(),
                subtypes: Vec::new(),
                abilities: Vec::new(),
                power: None,
                toughness: None,
                loyalty: None,
                defense: None
            }
            .lower(),
            deckmaste_card::Characteristics {
                name: _,
                mana_cost: _,
                color_indicator: _,
                supertypes: _,
                types: _,
                subtypes: _,
                abilities: _,
                power: None,
                toughness: None,
                loyalty: None,
                defense: None
            }
        );
    }

    #[test]
    fn lowers_card_normal() {
        assert_matches!(
            deckmaste_semantics::Card::Normal(minimal_card_face()).lower(),
            deckmaste_card::Card::Normal(deckmaste_card::CardFace {
                characteristics: deckmaste_card::Characteristics {
                    name: _,
                    mana_cost: _,
                    color_indicator: _,
                    supertypes: _,
                    types: _,
                    subtypes: _,
                    abilities: _,
                    power: None,
                    toughness: None,
                    loyalty: None,
                    defense: None
                }
            })
        );
    }

    #[test]
    fn lowers_card_double_faced_transforming() {
        assert_matches!(
            deckmaste_semantics::Card::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Transforming,
                front: minimal_card_face(),
                back: minimal_card_face()
            }
            .lower(),
            deckmaste_card::Card::DoubleFaced {
                layout: deckmaste_card::DoubleFacedLayout::Transforming,
                front: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                },
                back: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                }
            }
        );
    }

    #[test]
    fn lowers_card_double_faced_modal_dfc() {
        assert_matches!(
            deckmaste_semantics::Card::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::ModalDfc,
                front: minimal_card_face(),
                back: minimal_card_face()
            }
            .lower(),
            deckmaste_card::Card::DoubleFaced {
                layout: deckmaste_card::DoubleFacedLayout::ModalDfc,
                front: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                },
                back: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                }
            }
        );
    }

    #[test]
    fn lowers_card_split() {
        assert_matches!(
            deckmaste_semantics::Card::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Split,
                front: minimal_card_face(),
                back: minimal_card_face()
            }
            .lower(),
            deckmaste_card::Card::Split {
                left: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                },
                right: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                }
            }
        );
    }

    #[test]
    fn lowers_card_flip() {
        assert_matches!(
            deckmaste_semantics::Card::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Flip,
                front: minimal_card_face(),
                back: minimal_card_face()
            }
            .lower(),
            deckmaste_card::Card::Flip {
                normal: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                },
                alternative: deckmaste_card::Characteristics { name: _, .. }
            }
        );
    }

    #[test]
    fn lowers_card_adventurer() {
        assert_matches!(
            deckmaste_semantics::Card::TwoFaced {
                layout: deckmaste_semantics::FaceLayout::Adventure,
                front: minimal_card_face(),
                back: minimal_card_face()
            }
            .lower(),
            deckmaste_card::Card::Adventurer {
                normal: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { name: _, .. }
                },
                adventure: deckmaste_card::Characteristics { name: _, .. }
            }
        );
    }
}
