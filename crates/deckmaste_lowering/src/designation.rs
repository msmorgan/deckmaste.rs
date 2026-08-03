//! `designation` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::DesignationScope {
    type Target = deckmaste_core::DesignationScope;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object => deckmaste_core::DesignationScope::Object,
            Self::Player => deckmaste_core::DesignationScope::Player,
            Self::Game => deckmaste_core::DesignationScope::Game,
        }
    }
}

impl Lower for deckmaste_authoring::DesignationShape {
    type Target = deckmaste_core::DesignationShape;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Flag => deckmaste_core::DesignationShape::Flag,
            Self::Number => deckmaste_core::DesignationShape::Number,
            Self::Enum(f0) => deckmaste_core::DesignationShape::Enum(f0.lower()),
            Self::Relation => deckmaste_core::DesignationShape::Relation,
        }
    }
}

impl Lower for deckmaste_authoring::DesignationUniqueness {
    type Target = deckmaste_core::DesignationUniqueness;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::None => deckmaste_core::DesignationUniqueness::None,
            Self::PerPlayer => deckmaste_core::DesignationUniqueness::PerPlayer,
            Self::PerGame => deckmaste_core::DesignationUniqueness::PerGame,
        }
    }
}

impl Lower for deckmaste_authoring::DesignationPersistence {
    type Target = deckmaste_core::DesignationPersistence;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::ObjectLifetime => deckmaste_core::DesignationPersistence::ObjectLifetime,
            Self::UntilEndOfTurn => deckmaste_core::DesignationPersistence::UntilEndOfTurn,
            Self::EffectSupplied => deckmaste_core::DesignationPersistence::EffectSupplied,
            Self::Permanently => deckmaste_core::DesignationPersistence::Permanently,
        }
    }
}

impl Lower for deckmaste_authoring::DesignationDef {
    type Target = deckmaste_core::DesignationDef;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Stored {
                scope,
                shape,
                uniqueness,
                persistence,
                payload,
            } => deckmaste_core::DesignationDef::Stored {
                scope: scope.lower(),
                shape: shape.lower(),
                uniqueness: uniqueness.lower(),
                persistence: persistence.lower(),
                payload: payload.lower(),
            },
            Self::Derived(f0) => deckmaste_core::DesignationDef::Derived(f0.lower()),
            Self::DerivedIf(f0) => deckmaste_core::DesignationDef::DerivedIf(f0.lower()),
        }
    }
}

impl Lower for deckmaste_authoring::DesignationDecl {
    type Target = deckmaste_core::DesignationDecl;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::DesignationDecl {
            name: self.name.lower(),
            definition: self.definition.lower(),
        }
    }
}
