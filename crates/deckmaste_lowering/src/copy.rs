//! `copy` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::CopySpec {
    type Target = deckmaste_core::CopySpec;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::CopySpec {
            source: self.source.lower(),
            exceptions: self.exceptions.lower(),
        }
    }
}

impl Lower for deckmaste_semantics::CopySource {
    type Target = deckmaste_core::CopySource;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object(f0) => deckmaste_core::CopySource::Object(f0.lower()),
            Self::SelfCard => deckmaste_core::CopySource::SelfCard,
        }
    }
}

impl Lower for deckmaste_semantics::CopyException {
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

impl Lower for deckmaste_semantics::CopiableValues {
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
    use crate::minimal::*;

    #[test]
    fn lowers_copy_spec() {
        assert_matches!(
            deckmaste_semantics::CopySpec {
                source: minimal_copy_source(),
                exceptions: Vec::new()
            }
            .lower(),
            deckmaste_core::CopySpec {
                source: deckmaste_core::CopySource::Object(deckmaste_core::Reference::This),
                exceptions: _
            }
        );
    }

    #[test]
    fn lowers_copy_source_object() {
        assert_matches!(
            deckmaste_semantics::CopySource::Object(minimal_reference()).lower(),
            deckmaste_core::CopySource::Object(deckmaste_core::Reference::This)
        );
    }

    #[test]
    fn lowers_copy_source_self_card() {
        assert_matches!(
            deckmaste_semantics::CopySource::SelfCard.lower(),
            deckmaste_core::CopySource::SelfCard
        );
    }

    #[test]
    fn lowers_copy_exception_modify() {
        assert_matches!(
            deckmaste_semantics::CopyException::Modify(minimal_modification()).lower(),
            deckmaste_core::CopyException::Modify(deckmaste_core::Modification::Power(
                deckmaste_core::NumericOp::Set(deckmaste_core::StatValue::DefinedByAbility)
            ))
        );
    }

    #[test]
    fn lowers_copy_exception_retain() {
        assert_matches!(
            deckmaste_semantics::CopyException::Retain(minimal_characteristic()).lower(),
            deckmaste_core::CopyException::Retain(deckmaste_core::Characteristic::Colors)
        );
    }

    #[test]
    fn lowers_copy_exception_additional_effect() {
        assert_matches!(
            deckmaste_semantics::CopyException::AdditionalEffect(minimal_enter_rider()).lower(),
            deckmaste_core::CopyException::AdditionalEffect(deckmaste_core::EnterRider::Tapped)
        );
    }

    #[test]
    fn lowers_copiable_values() {
        assert_matches!(
            deckmaste_semantics::CopiableValues {
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
            deckmaste_core::CopiableValues {
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
}
