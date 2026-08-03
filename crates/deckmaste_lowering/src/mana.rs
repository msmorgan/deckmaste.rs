//! `mana` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::PlanarFace {
    type Target = deckmaste_core::PlanarFace;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Blank => deckmaste_core::PlanarFace::Blank,
            Self::Chaos => deckmaste_core::PlanarFace::Chaos,
            Self::Planeswalker => deckmaste_core::PlanarFace::Planeswalker,
        }
    }
}

impl Lower for deckmaste_authoring::ManaSpec {
    type Target = deckmaste_core::ManaSpec;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AnyColor => deckmaste_core::ManaSpec::AnyColor,
            Self::OneOf(f0) => deckmaste_core::ManaSpec::OneOf(f0.lower()),
            Self::OneOfRuns(f0) => deckmaste_core::ManaSpec::OneOfRuns(f0.lower()),
            Self::AmongColorsOf(f0) => deckmaste_core::ManaSpec::AmongColorsOf(f0.lower()),
            Self::ProducedByEvent => deckmaste_core::ManaSpec::ProducedByEvent,
            Self::Specific(f0) => deckmaste_core::ManaSpec::Specific(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::SimpleManaSymbol {
    type Target = deckmaste_core::SimpleManaSymbol;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Generic(f0) => deckmaste_core::SimpleManaSymbol::Generic(f0.lower()),
            Self::Specific(f0) => deckmaste_core::SimpleManaSymbol::Specific(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::ManaSymbol {
    type Target = deckmaste_core::ManaSymbol;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Variable => deckmaste_core::ManaSymbol::Variable,
            Self::Snow => deckmaste_core::ManaSymbol::Snow,
            Self::Hybrid(f0, f1) => deckmaste_core::ManaSymbol::Hybrid(f0.lower(), f1.lower()),
            Self::Phyrexian(f0, f1) => {
                deckmaste_core::ManaSymbol::Phyrexian(f0.lower(), f1.lower())
            }
            Self::Simple(f0) => deckmaste_core::ManaSymbol::Simple(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::SymbolPred {
    type Target = deckmaste_core::SymbolPred;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::AnyColor => deckmaste_core::SymbolPred::AnyColor,
            Self::AnyType => deckmaste_core::SymbolPred::AnyType,
            Self::CountsAs(f0) => deckmaste_core::SymbolPred::CountsAs(f0.lower()),
            Self::IsGeneric => deckmaste_core::SymbolPred::IsGeneric,
            Self::And(f0) => deckmaste_core::SymbolPred::And(f0.lower()),
            Self::Or(f0) => deckmaste_core::SymbolPred::Or(f0.lower()),
            Self::Not(f0) => deckmaste_core::SymbolPred::Not(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::ManaRider {
    type Target = deckmaste_core::ManaRider;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SpendOnly(f0) => deckmaste_core::ManaRider::SpendOnly(f0.lower()),
            Self::GrantOnSpend(f0) => deckmaste_core::ManaRider::GrantOnSpend(f0.lower()),
            Self::TriggerOnSpend(f0) => deckmaste_core::ManaRider::TriggerOnSpend(f0.lower()),
            Self::Persistent(f0) => deckmaste_core::ManaRider::Persistent(f0.lower()),
            Self::Snow => deckmaste_core::ManaRider::Snow,
            Self::Expanded(f0) => deckmaste_core::ManaRider::Expanded(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::ManaProduction {
    type Target = deckmaste_core::ManaProduction;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::WithRiders { mana, riders } => deckmaste_core::ManaProduction::WithRiders {
                mana: mana.lower(),
                riders: riders.lower(),
            },
            Self::Bare(f0) => deckmaste_core::ManaProduction::Bare(f0.lower()),
        }
    }
}

/// HAND-WRITTEN, not scaffolded.
///
/// `ManaCost` is a newtype over a PRIVATE `Arc<[ManaSymbol]>` (authoring's
/// `mana.rs`), so `self.0` is not reachable from this crate and the scaffold
/// generator correctly refused to emit an arm for it. This is still an
/// identity mapping — it just travels through the public `From` conversions on
/// either side of the newtype rather than through the field.
///
/// `ParseManaError` gets no arm at all: it is the `FromStr` error type, not
/// grammar, and is unreachable from any container.
impl Lower for deckmaste_authoring::ManaCost {
    type Target = deckmaste_core::ManaCost;
    fn lower(self) -> <Self as Lower>::Target {
        let symbols: std::sync::Arc<[deckmaste_authoring::ManaSymbol]> = self.into();
        symbols.lower().into()
    }
}
