//! `property` — authored grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_authoring::Property {
    type Target = deckmaste_core::Property;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Ability(f0) => deckmaste_core::Property::Ability(f0.lower()),
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
            deckmaste_authoring::Property::Ability(std::sync::Arc::new(minimal_ability())).lower(),
            deckmaste_core::Property::Ability(_)
        );
    }

    #[test]
    fn lowers_property_continuous() {
        assert_matches!(
            deckmaste_authoring::Property::Continuous(minimal_reference(), minimal_modification())
                .lower(),
            deckmaste_core::Property::Continuous(
                deckmaste_core::Reference::This,
                deckmaste_core::Modification::Power(deckmaste_core::NumericOp::Set(
                    deckmaste_core::StatValue::DefinedByAbility
                ))
            )
        );
    }

    #[test]
    fn lowers_property_state_based() {
        assert_matches!(
            deckmaste_authoring::Property::StateBased {
                condition: std::sync::Arc::new(minimal_condition()),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Property::StateBased {
                condition: _,
                effect: _
            }
        );
    }

    #[test]
    fn lowers_property_turn_based() {
        assert_matches!(
            deckmaste_authoring::Property::TurnBased {
                at: minimal_phase_step(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Property::TurnBased {
                at: deckmaste_core::PhaseStep::Beginning(deckmaste_core::BeginningStep::Untap),
                effect: _
            }
        );
    }
}
