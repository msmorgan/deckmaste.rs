//! `designation` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::DesignationScope {
    type Target = deckmaste_core::DesignationScope;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Object => deckmaste_core::DesignationScope::Object,
            Self::Player => deckmaste_core::DesignationScope::Player,
            Self::Game => deckmaste_core::DesignationScope::Game,
        }
    }
}

impl Lower for deckmaste_semantics::DesignationShape {
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

impl Lower for deckmaste_semantics::DesignationUniqueness {
    type Target = deckmaste_core::DesignationUniqueness;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::None => deckmaste_core::DesignationUniqueness::None,
            Self::PerPlayer => deckmaste_core::DesignationUniqueness::PerPlayer,
            Self::PerGame => deckmaste_core::DesignationUniqueness::PerGame,
        }
    }
}

impl Lower for deckmaste_semantics::DesignationPersistence {
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

impl Lower for deckmaste_semantics::DesignationDef {
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

impl Lower for deckmaste_semantics::CoreDeed {
    type Target = deckmaste_core::CoreDeed;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Attack => deckmaste_core::CoreDeed::Attack,
            Self::Block => deckmaste_core::CoreDeed::Block,
            Self::Target => deckmaste_core::CoreDeed::Target,
            Self::Copy => deckmaste_core::CoreDeed::Copy,
            Self::Draw => deckmaste_core::CoreDeed::Draw,
            Self::GainLife => deckmaste_core::CoreDeed::GainLife,
            Self::LoseGame => deckmaste_core::CoreDeed::LoseGame,
            Self::WinGame => deckmaste_core::CoreDeed::WinGame,
            Self::Spend => deckmaste_core::CoreDeed::Spend,
            Self::Trigger => deckmaste_core::CoreDeed::Trigger,
            Self::Put => deckmaste_core::CoreDeed::Put,
            Self::Return => deckmaste_core::CoreDeed::Return,
            Self::GainControl => deckmaste_core::CoreDeed::GainControl,
            Self::Unlock => deckmaste_core::CoreDeed::Unlock,
            Self::FullyUnlock => deckmaste_core::CoreDeed::FullyUnlock,
        }
    }
}

impl Lower for deckmaste_semantics::DesignationDecl {
    type Target = deckmaste_core::DesignationDecl;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::DesignationDecl {
            name: self.name.lower(),
            definition: self.definition.lower(),
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
    fn lowers_designation_scope_object() {
        assert_matches!(
            deckmaste_semantics::DesignationScope::Object.lower(),
            deckmaste_core::DesignationScope::Object
        );
    }

    #[test]
    fn lowers_designation_scope_player() {
        assert_matches!(
            deckmaste_semantics::DesignationScope::Player.lower(),
            deckmaste_core::DesignationScope::Player
        );
    }

    #[test]
    fn lowers_designation_scope_game() {
        assert_matches!(
            deckmaste_semantics::DesignationScope::Game.lower(),
            deckmaste_core::DesignationScope::Game
        );
    }

    #[test]
    fn lowers_designation_shape_flag() {
        assert_matches!(
            deckmaste_semantics::DesignationShape::Flag.lower(),
            deckmaste_core::DesignationShape::Flag
        );
    }

    #[test]
    fn lowers_designation_shape_number() {
        assert_matches!(
            deckmaste_semantics::DesignationShape::Number.lower(),
            deckmaste_core::DesignationShape::Number
        );
    }

    #[test]
    fn lowers_designation_shape_enum() {
        assert_matches!(
            deckmaste_semantics::DesignationShape::Enum([].into()).lower(),
            deckmaste_core::DesignationShape::Enum(_)
        );
    }

    #[test]
    fn lowers_designation_shape_relation() {
        assert_matches!(
            deckmaste_semantics::DesignationShape::Relation.lower(),
            deckmaste_core::DesignationShape::Relation
        );
    }

    #[test]
    fn lowers_designation_uniqueness_none() {
        assert_matches!(
            deckmaste_semantics::DesignationUniqueness::None.lower(),
            deckmaste_core::DesignationUniqueness::None
        );
    }

    #[test]
    fn lowers_designation_uniqueness_per_player() {
        assert_matches!(
            deckmaste_semantics::DesignationUniqueness::PerPlayer.lower(),
            deckmaste_core::DesignationUniqueness::PerPlayer
        );
    }

    #[test]
    fn lowers_designation_uniqueness_per_game() {
        assert_matches!(
            deckmaste_semantics::DesignationUniqueness::PerGame.lower(),
            deckmaste_core::DesignationUniqueness::PerGame
        );
    }

    #[test]
    fn lowers_designation_persistence_object_lifetime() {
        assert_matches!(
            deckmaste_semantics::DesignationPersistence::ObjectLifetime.lower(),
            deckmaste_core::DesignationPersistence::ObjectLifetime
        );
    }

    #[test]
    fn lowers_designation_persistence_until_end_of_turn() {
        assert_matches!(
            deckmaste_semantics::DesignationPersistence::UntilEndOfTurn.lower(),
            deckmaste_core::DesignationPersistence::UntilEndOfTurn
        );
    }

    #[test]
    fn lowers_designation_persistence_effect_supplied() {
        assert_matches!(
            deckmaste_semantics::DesignationPersistence::EffectSupplied.lower(),
            deckmaste_core::DesignationPersistence::EffectSupplied
        );
    }

    #[test]
    fn lowers_designation_persistence_permanently() {
        assert_matches!(
            deckmaste_semantics::DesignationPersistence::Permanently.lower(),
            deckmaste_core::DesignationPersistence::Permanently
        );
    }

    #[test]
    fn lowers_designation_def_stored() {
        assert_matches!(
            deckmaste_semantics::DesignationDef::Stored {
                scope: minimal_designation_scope(),
                shape: minimal_designation_shape(),
                uniqueness: minimal_designation_uniqueness(),
                persistence: minimal_designation_persistence(),
                payload: [].into()
            }
            .lower(),
            deckmaste_core::DesignationDef::Stored {
                scope: deckmaste_core::DesignationScope::Object,
                shape: deckmaste_core::DesignationShape::Flag,
                uniqueness: deckmaste_core::DesignationUniqueness::None,
                persistence: deckmaste_core::DesignationPersistence::ObjectLifetime,
                payload: _
            }
        );
    }

    #[test]
    fn lowers_designation_def_derived() {
        assert_matches!(
            deckmaste_semantics::DesignationDef::Derived(minimal_predicate()).lower(),
            deckmaste_core::DesignationDef::Derived(deckmaste_core::Predicate::Class(
                deckmaste_core::ObjectClass::AbilityOnStack
            ))
        );
    }

    #[test]
    fn lowers_designation_def_derived_if() {
        assert_matches!(
            deckmaste_semantics::DesignationDef::DerivedIf(
                std::sync::Arc::new(minimal_condition())
            )
            .lower(),
            deckmaste_core::DesignationDef::DerivedIf(_)
        );
    }

    #[test]
    fn lowers_designation_decl() {
        assert_matches!(
            deckmaste_semantics::DesignationDecl {
                name: "X".into(),
                definition: minimal_designation_def()
            }
            .lower(),
            deckmaste_core::DesignationDecl {
                name: _,
                definition: deckmaste_core::DesignationDef::Stored {
                    scope: deckmaste_core::DesignationScope::Object,
                    shape: deckmaste_core::DesignationShape::Flag,
                    uniqueness: deckmaste_core::DesignationUniqueness::None,
                    persistence: deckmaste_core::DesignationPersistence::ObjectLifetime,
                    payload: _
                }
            }
        );
    }
}
