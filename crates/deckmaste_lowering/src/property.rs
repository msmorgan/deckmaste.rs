//! `property` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

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
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_property_ability() {
        assert_lowers(deckmaste_authoring::Property::Ability(std::sync::Arc::new(
            minimal_ability(),
        )));
        assert_matches!(
            deckmaste_authoring::Property::Ability(std::sync::Arc::new(minimal_ability())).lower(),
            deckmaste_core::Property::Ability(..)
        );
    }

    #[test]
    fn lowers_property_continuous() {
        assert_lowers(deckmaste_authoring::Property::Continuous(
            minimal_reference(),
            minimal_modification(),
        ));
        assert_matches!(
            deckmaste_authoring::Property::Continuous(minimal_reference(), minimal_modification())
                .lower(),
            deckmaste_core::Property::Continuous(..)
        );
    }

    #[test]
    fn lowers_property_state_based() {
        assert_lowers(deckmaste_authoring::Property::StateBased {
            condition: std::sync::Arc::new(minimal_condition()),
            effect: std::sync::Arc::new(minimal_one_shot_effect()),
        });
        assert_matches!(
            deckmaste_authoring::Property::StateBased {
                condition: std::sync::Arc::new(minimal_condition()),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Property::StateBased { .. }
        );
    }

    #[test]
    fn lowers_property_turn_based() {
        assert_lowers(deckmaste_authoring::Property::TurnBased {
            at: minimal_phase_step(),
            effect: std::sync::Arc::new(minimal_one_shot_effect()),
        });
        assert_matches!(
            deckmaste_authoring::Property::TurnBased {
                at: minimal_phase_step(),
                effect: std::sync::Arc::new(minimal_one_shot_effect())
            }
            .lower(),
            deckmaste_core::Property::TurnBased { .. }
        );
    }
}
