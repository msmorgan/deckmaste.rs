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
