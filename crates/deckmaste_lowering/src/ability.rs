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

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_spell_ability() {
        assert_lowers(deckmaste_authoring::SpellAbility {
            ability_word: None,
            effect: minimal_one_shot_effect(),
        });
    }

    #[test]
    fn lowers_activated_ability() {
        assert_lowers(deckmaste_authoring::ActivatedAbility {
            ability_word: None,
            cost: minimal_cost(),
            from: None,
            window: None,
            condition: None,
            limits: [].into(),
            effect: minimal_one_shot_effect(),
        });
    }

    #[test]
    fn lowers_use_limit_once_per_turn() {
        assert_lowers(deckmaste_authoring::UseLimit::OncePerTurn);
        assert_matches!(
            deckmaste_authoring::UseLimit::OncePerTurn.lower(),
            deckmaste_core::UseLimit::OncePerTurn
        );
    }

    #[test]
    fn lowers_use_limit_once_per_game() {
        assert_lowers(deckmaste_authoring::UseLimit::OncePerGame);
        assert_matches!(
            deckmaste_authoring::UseLimit::OncePerGame.lower(),
            deckmaste_core::UseLimit::OncePerGame
        );
    }

    #[test]
    fn lowers_use_limit_loyalty_once_per_turn() {
        assert_lowers(deckmaste_authoring::UseLimit::LoyaltyOncePerTurn);
        assert_matches!(
            deckmaste_authoring::UseLimit::LoyaltyOncePerTurn.lower(),
            deckmaste_core::UseLimit::LoyaltyOncePerTurn
        );
    }

    #[test]
    fn lowers_triggered_ability() {
        assert_lowers(deckmaste_authoring::TriggeredAbility {
            ability_word: None,
            event: minimal_event_filter(),
            from: None,
            condition: None,
            limits: [].into(),
            where_x: None,
            effect: minimal_one_shot_effect(),
        });
    }

    #[test]
    fn lowers_choose_spec() {
        assert_lowers(deckmaste_authoring::ChooseSpec {
            count: minimal_quantity(),
            up_to: false,
            repeats: false,
            chooser: minimal_reference(),
            rider: None,
        });
    }

    #[test]
    fn lowers_modal_cost_rider_entwine() {
        assert_lowers(deckmaste_authoring::ModalCostRider::Entwine(minimal_cost()));
        assert_matches!(
            deckmaste_authoring::ModalCostRider::Entwine(minimal_cost()).lower(),
            deckmaste_core::ModalCostRider::Entwine(..)
        );
    }

    #[test]
    fn lowers_modal_cost_rider_escalate() {
        assert_lowers(deckmaste_authoring::ModalCostRider::Escalate(minimal_cost()));
        assert_matches!(
            deckmaste_authoring::ModalCostRider::Escalate(minimal_cost()).lower(),
            deckmaste_core::ModalCostRider::Escalate(..)
        );
    }

    #[test]
    fn lowers_mode() {
        assert_lowers(deckmaste_authoring::Mode {
            effect: minimal_one_shot_effect(),
            cost: None,
        });
    }

    #[test]
    fn lowers_ability_static() {
        assert_lowers_debug(deckmaste_authoring::Ability::Static(std::sync::Arc::new(
            minimal_static_effect(),
        )));
        assert_matches!(
            deckmaste_authoring::Ability::Static(std::sync::Arc::new(minimal_static_effect()))
                .lower(),
            deckmaste_core::Ability::Static(..)
        );
    }

    #[test]
    fn lowers_ability_activated() {
        assert_lowers_debug(deckmaste_authoring::Ability::Activated(
            std::sync::Arc::new(minimal_activated_ability()),
        ));
        assert_matches!(
            deckmaste_authoring::Ability::Activated(std::sync::Arc::new(
                minimal_activated_ability()
            ))
            .lower(),
            deckmaste_core::Ability::Activated(..)
        );
    }

    #[test]
    fn lowers_ability_triggered() {
        assert_lowers_debug(deckmaste_authoring::Ability::Triggered(
            std::sync::Arc::new(minimal_triggered_ability()),
        ));
        assert_matches!(
            deckmaste_authoring::Ability::Triggered(std::sync::Arc::new(
                minimal_triggered_ability()
            ))
            .lower(),
            deckmaste_core::Ability::Triggered(..)
        );
    }

    #[test]
    fn lowers_ability_spell() {
        assert_lowers_debug(deckmaste_authoring::Ability::Spell(std::sync::Arc::new(
            minimal_spell_ability(),
        )));
        assert_matches!(
            deckmaste_authoring::Ability::Spell(std::sync::Arc::new(minimal_spell_ability()))
                .lower(),
            deckmaste_core::Ability::Spell(..)
        );
    }

    #[test]
    fn lowers_ability_keyword() {
        assert_lowers_debug(deckmaste_authoring::Ability::Keyword(
            minimal_keyword_ability(),
        ));
        assert_matches!(
            deckmaste_authoring::Ability::Keyword(minimal_keyword_ability()).lower(),
            deckmaste_core::Ability::Keyword(..)
        );
    }

    #[test]
    fn lowers_ability_innate() {
        assert_lowers_debug(deckmaste_authoring::Ability::Innate(std::sync::Arc::new(
            minimal_ability(),
        )));
        assert_matches!(
            deckmaste_authoring::Ability::Innate(std::sync::Arc::new(minimal_ability())).lower(),
            deckmaste_core::Ability::Innate(..)
        );
    }

    #[test]
    fn lowers_ability_expanded() {
        assert_lowers_debug(deckmaste_authoring::Ability::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_ability()),
            },
        ));
        assert_matches!(
            deckmaste_authoring::Ability::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_ability())
            })
            .lower(),
            deckmaste_core::Ability::Expanded(..)
        );
    }
}
