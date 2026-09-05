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
            abilities: {
                let abilities = self.abilities.lower();
                refuse_rules_defined_sba(&abilities);
                abilities
            },
            power: self.power.lower(),
            toughness: self.toughness.lower(),
            loyalty: self.loyalty.lower(),
            defense: self.defense.lower(),
        }
    }
}

/// [CR#704.1]: a rules-defined state-based action is a game action, not an
/// ability of any kind — [CR#704.1a] routes a state-WATCHING ability to the
/// triggered bucket instead. A card-level state-checked static ([CR#604.1])
/// whose instruction removes its own object from the battlefield is therefore
/// a [CR#704.5]-shaped state-based action wearing a static ability; its home
/// is a `Property::StateBased` conferral on the type or subtype whose rule it
/// is, or an `SbaRule` ([CR#714.4] the Saga sacrifice, [CR#704.5m] the Aura
/// graveyard rule). Ascend ([CR#702.131b]) is a genuine static and passes: it
/// grants a designation and removes nothing.
fn refuse_rules_defined_sba(abilities: &[deckmaste_core::Ability]) {
    for ability in abilities {
        let deckmaste_core::Ability::Static(region) = ability else {
            continue;
        };
        let deckmaste_core::StaticSpec::ConditionallyDo { then, .. } = &region.body else {
            continue;
        };
        if removes_source(then) {
            crate::region::refuse(
                "a state-checked static that removes its own object restates a rules-defined \
                 state-based action ([CR#704.1,704.1a]); confer it on the type or subtype as \
                 `Property::StateBased`, or author it as an `SbaRule`",
            );
        }
    }
}

/// Whether an instruction removes the region's own source object: the
/// [CR#704.5] removal family — sacrifice ([CR#701.21a]), a zone change
/// ([CR#400.7]), or ceasing to exist ([CR#704.5d]).
fn removes_source(instruction: &deckmaste_core::Instruction) -> bool {
    use deckmaste_core::Action;

    let this = deckmaste_core::Reference::source_parameter();
    match instruction {
        deckmaste_core::Instruction::Act {
            action: Action::Sacrifice(_, what) | Action::Move(what, ..) | Action::Cease(what),
            ..
        } => what == &this,
        deckmaste_core::Instruction::Sequentially(body) => body.iter().any(removes_source),
        _ => false,
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

    /// A card-level state-checked static that removes its own object restates
    /// a rules-defined state-based action ([CR#704.1,704.1a,714.4]), so the
    /// card does not compile.
    #[test]
    fn a_self_removing_state_checked_static_is_refused() {
        let face = deckmaste_semantics::CardFace {
            abilities: vec![deckmaste_semantics::Ability::Static(std::sync::Arc::new(
                deckmaste_semantics::StaticEffect::Sba {
                    when: std::sync::Arc::new(minimal_condition()),
                    then: std::sync::Arc::new(deckmaste_semantics::OneShotEffect::Act(
                        deckmaste_semantics::Action::Sacrifice(
                            deckmaste_semantics::Reference::You,
                            deckmaste_semantics::Reference::This,
                        ),
                    )),
                },
            ))],
            ..minimal_card_face()
        };
        let refusal = crate::lower_card(deckmaste_semantics::Card::Normal(face))
            .expect_err("a rules-defined SBA spelled as a card static is refused");
        assert!(
            refusal.message.contains("state-based action"),
            "{}",
            refusal.message
        );
    }

    /// Ascend ([CR#702.131b]) is a genuine static ability: its instruction
    /// grants a designation and removes nothing, so the same shape compiles.
    #[test]
    fn a_state_checked_static_that_removes_nothing_compiles() {
        let face = deckmaste_semantics::CardFace {
            abilities: vec![deckmaste_semantics::Ability::Static(std::sync::Arc::new(
                deckmaste_semantics::StaticEffect::Sba {
                    when: std::sync::Arc::new(minimal_condition()),
                    then: std::sync::Arc::new(deckmaste_semantics::OneShotEffect::Act(
                        deckmaste_semantics::Action::GetDesignation(
                            deckmaste_semantics::Reference::You,
                            "CitysBlessing".into(),
                        ),
                    )),
                },
            ))],
            ..minimal_card_face()
        };
        assert!(crate::lower_card(deckmaste_semantics::Card::Normal(face)).is_ok());
    }

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
                    characteristics: deckmaste_card::Characteristics { .. }
                },
                back: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { .. }
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
                    characteristics: deckmaste_card::Characteristics { .. }
                },
                back: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { .. }
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
                    characteristics: deckmaste_card::Characteristics { .. }
                },
                right: deckmaste_card::CardFace {
                    characteristics: deckmaste_card::Characteristics { .. }
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
                    characteristics: deckmaste_card::Characteristics { .. }
                },
                alternative: deckmaste_card::Characteristics { .. }
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
                    characteristics: deckmaste_card::Characteristics { .. }
                },
                adventure: deckmaste_card::Characteristics { .. }
            }
        );
    }
}
