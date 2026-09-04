//! `property` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Property {
    type Target = deckmaste_core::Property;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Ability(f0) => ability_conferral(f0.lower()),
            Self::Continuous(f0, f1) => {
                deckmaste_core::Property::Continuous(f0.lower(), f1.lower())
            }
            Self::StateBased { condition, effect } => deckmaste_core::Property::StateBased {
                condition: condition.lower(),
                effect: effect.lower(),
            },
            Self::TurnBased { at, effect } => deckmaste_core::Property::TurnBased {
                at: at.lower(),
                effect: effect.lower(),
            },
        }
    }
}

/// Re-home an `Ability`-flavored v1 conferral onto the core flavor its
/// CONTENT calls for. A DEONTIC row conferred by a type or subtype states a
/// quality of the object — "an Equipment can be attached to a creature"
/// ([CR#301.5]), "a land can be played" ([CR#305.9]) — and [CR#113.12] is
/// explicit that stating a quality is "neither granting an ability nor setting
/// a characteristic", so it lands on the ability-free
/// [`deckmaste_core::Property::Static`]. Everything else is a real conferred
/// ability ([CR#305.6] a basic land type's mana ability, [CR#714.3a] a Saga's
/// enters-with-a-lore-counter replacement) and stays `Property::Ability`.
fn ability_conferral(ability: std::sync::Arc<deckmaste_core::Ability>) -> deckmaste_core::Property {
    if let deckmaste_core::Ability::Static(region) = ability.as_ref() {
        // A rules-defined state-based action is a GAME ACTION, not an ability
        // of any kind ([CR#704.1,704.1a]) — the Aura must-be-attached rule
        // ([CR#704.5m]). v1 spells it as an `Sba` static (lowered to
        // `ConditionallyDo`); core's home for it is the ability-free
        // `StateBased` flavor.
        if let deckmaste_core::StaticSpec::ConditionallyDo { when, then } = &region.body {
            return deckmaste_core::Property::StateBased {
                condition: std::sync::Arc::clone(when),
                effect: std::sync::Arc::clone(then),
            };
        }
        if is_deontic(&region.body) {
            return deckmaste_core::Property::Static(std::sync::Arc::clone(region));
        }
    }
    deckmaste_core::Property::Ability(ability)
}

/// Whether a static effect is a deontic row, looking through the
/// `Conditionally` gate that Creature's summoning-sickness pair ([CR#302.6])
/// wears.
fn is_deontic(spec: &deckmaste_core::StaticSpec) -> bool {
    match spec {
        deckmaste_core::StaticSpec::Deontic(_) => true,
        deckmaste_core::StaticSpec::Conditionally(_, inner) => is_deontic(inner),
        _ => false,
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
    fn lowers_property_ability() {
        assert_matches!(
            deckmaste_semantics::Property::Ability(std::sync::Arc::new(minimal_ability())).lower(),
            deckmaste_core::Property::Ability(_)
        );
    }

    /// [CR#113.12,301.5]: a DEONTIC conferral is a quality of the object, so
    /// it re-homes onto the ability-free `Property::Static` flavor rather than
    /// staying an ability.
    #[test]
    fn deontic_conferral_re_homes_to_property_static() {
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::StaticSpec;

        let rule = deckmaste_core::Ability::r#static(StaticSpec::Deontic(Deontic::May(
            DeonticAction::Attach {
                what: deckmaste_core::Predicate::Any,
                to: deckmaste_core::Predicate::Any,
            },
        )));
        assert_matches!(
            super::ability_conferral(std::sync::Arc::new(rule)),
            deckmaste_core::Property::Static(_)
        );
    }

    #[test]
    fn lowers_property_continuous() {
        assert_matches!(
            deckmaste_semantics::Property::Continuous(minimal_reference(), minimal_modification())
                .lower(),
            deckmaste_core::Property::Continuous(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Modification::Power(deckmaste_core::NumericOp::Set(
                    deckmaste_core::StatValue::DefinedByAbility
                ))
            )
        );
    }

    #[test]
    fn lowers_property_state_based() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::Property::StateBased {
                condition: std::sync::Arc::new(minimal_condition()),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            deckmaste_core::Property::StateBased {
                condition: _,
                effect: _
            }
        );
    }

    #[test]
    fn lowers_property_turn_based() {
        assert_matches!(
            in_spell_region(|| deckmaste_semantics::Property::TurnBased {
                at: minimal_phase_step(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower()),
            deckmaste_core::Property::TurnBased {
                at: deckmaste_core::PhaseStep::Beginning(deckmaste_core::BeginningStep::Untap),
                effect: _
            }
        );
    }
}
