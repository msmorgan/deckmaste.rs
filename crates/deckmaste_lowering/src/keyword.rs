//! `keyword` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::KeywordRef {
    type Target = deckmaste_core::KeywordRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::KeywordRef(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::ParamShape {
    type Target = deckmaste_core::ParamShape;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::None => deckmaste_core::ParamShape::None,
            Self::Counted => deckmaste_core::ParamShape::Counted,
            Self::Costed => deckmaste_core::ParamShape::Costed,
            Self::CountedCost => deckmaste_core::ParamShape::CountedCost,
            Self::Predicated => deckmaste_core::ParamShape::Predicated,
            Self::PredicatedCosted => deckmaste_core::ParamShape::PredicatedCosted,
            Self::Named => deckmaste_core::ParamShape::Named,
        }
    }
}

impl Lower for deckmaste_authoring::KeywordDecl {
    type Target = deckmaste_core::KeywordDecl;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::KeywordDecl {
            name: self.name.lower(),
            shape: self.shape.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::KeywordAbility {
    type Target = deckmaste_core::KeywordAbility;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::FirstStrike => deckmaste_core::KeywordAbility::FirstStrike,
            Self::DoubleStrike => deckmaste_core::KeywordAbility::DoubleStrike,
            Self::Deathtouch => deckmaste_core::KeywordAbility::Deathtouch,
            Self::Trample => deckmaste_core::KeywordAbility::Trample,
            Self::Vigilance => deckmaste_core::KeywordAbility::Vigilance,
            Self::Composite { name, abilities } => deckmaste_core::KeywordAbility::Composite {
                name: name.lower(),
                abilities: abilities.lower(),
            },
            Self::Expanded(f0) => deckmaste_core::KeywordAbility::Expanded(f0.lower()),
        }
    }
}
