//! `copy` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::CopySpec {
    type Target = deckmaste_core::CopySpec;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CopySpec {
            source: self.source.lower(),
            exceptions: self.exceptions.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::CopySource {
    type Target = deckmaste_core::CopySource;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object(f0) => deckmaste_core::CopySource::Object(f0.lower()),
            Self::SelfCard => deckmaste_core::CopySource::SelfCard,
        }
    }
}

impl Lower for deckmaste_authoring::CopyException {
    type Target = deckmaste_core::CopyException;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Modify(f0) => deckmaste_core::CopyException::Modify(f0.lower()),
            Self::Retain(f0) => deckmaste_core::CopyException::Retain(f0.lower()),
            Self::AdditionalEffect(f0) => {
                deckmaste_core::CopyException::AdditionalEffect(f0.lower())
            }
        }
    }
}

impl Lower for deckmaste_authoring::CopiableValues {
    type Target = deckmaste_core::CopiableValues;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CopiableValues {
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
    fn lowers_copy_spec() {
        assert_lowers(deckmaste_authoring::CopySpec {
            source: minimal_copy_source(),
            exceptions: Vec::new(),
        });
    }

    #[test]
    fn lowers_copy_source_object() {
        assert_lowers_debug(deckmaste_authoring::CopySource::Object(minimal_reference()));
        assert_matches!(
            deckmaste_authoring::CopySource::Object(minimal_reference()).lower(),
            deckmaste_core::CopySource::Object(..)
        );
    }

    #[test]
    fn lowers_copy_source_self_card() {
        assert_lowers_debug(deckmaste_authoring::CopySource::SelfCard);
        assert_matches!(
            deckmaste_authoring::CopySource::SelfCard.lower(),
            deckmaste_core::CopySource::SelfCard
        );
    }

    #[test]
    fn lowers_copy_exception_modify() {
        assert_lowers_debug(deckmaste_authoring::CopyException::Modify(
            minimal_modification(),
        ));
        assert_matches!(
            deckmaste_authoring::CopyException::Modify(minimal_modification()).lower(),
            deckmaste_core::CopyException::Modify(..)
        );
    }

    #[test]
    fn lowers_copy_exception_retain() {
        assert_lowers_debug(deckmaste_authoring::CopyException::Retain(
            minimal_characteristic(),
        ));
        assert_matches!(
            deckmaste_authoring::CopyException::Retain(minimal_characteristic()).lower(),
            deckmaste_core::CopyException::Retain(..)
        );
    }

    #[test]
    fn lowers_copy_exception_additional_effect() {
        assert_lowers_debug(deckmaste_authoring::CopyException::AdditionalEffect(
            minimal_enter_rider(),
        ));
        assert_matches!(
            deckmaste_authoring::CopyException::AdditionalEffect(minimal_enter_rider()).lower(),
            deckmaste_core::CopyException::AdditionalEffect(..)
        );
    }

    #[test]
    fn lowers_copiable_values() {
        assert_lowers(deckmaste_authoring::CopiableValues {
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
}
