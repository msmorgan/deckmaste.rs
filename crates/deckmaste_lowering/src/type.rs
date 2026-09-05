//! `type` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Type {
    type Target = deckmaste_core::Type;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Artifact => deckmaste_core::Type::Artifact,
            Self::Battle => deckmaste_core::Type::Battle,
            Self::Creature => deckmaste_core::Type::Creature,
            Self::Dungeon => deckmaste_core::Type::Dungeon,
            Self::Enchantment => deckmaste_core::Type::Enchantment,
            Self::Instant => deckmaste_core::Type::Instant,
            Self::Kindred => deckmaste_core::Type::Kindred,
            Self::Land => deckmaste_core::Type::Land,
            Self::Planeswalker => deckmaste_core::Type::Planeswalker,
            Self::Sorcery => deckmaste_core::Type::Sorcery,
        }
    }
}

impl Lower for deckmaste_semantics::Supertype {
    type Target = deckmaste_core::Supertype;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::Basic => deckmaste_core::Supertype::Basic,
            Self::Legendary => deckmaste_core::Supertype::Legendary,
            Self::Ongoing => deckmaste_core::Supertype::Ongoing,
            Self::Snow => deckmaste_core::Supertype::Snow,
            Self::World => deckmaste_core::Supertype::World,
        }
    }
}

impl Lower for deckmaste_semantics::Subtype {
    type Target = deckmaste_core::Subtype;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Subtype {
            name: self.name.lower(),
            types: self.types.lower(),
            confers: crate::property::conferrals(self.confers.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::TypeDef {
    type Target = deckmaste_core::TypeDef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TypeDef {
            name: self.name.lower(),
            permanent_type: self.permanent.lower(),
            confers: crate::property::conferrals(self.confers.lower()),
        }
    }
}

impl Lower for deckmaste_semantics::TypeRef {
    type Target = deckmaste_core::TypeRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TypeRef(self.0.lower())
    }
}

impl Lower for deckmaste_semantics::SubtypeRef {
    type Target = deckmaste_core::SubtypeRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::SubtypeRef(self.0.lower())
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
    fn lowers_type_artifact() {
        assert_matches!(
            deckmaste_semantics::Type::Artifact.lower(),
            deckmaste_core::Type::Artifact
        );
    }

    #[test]
    fn lowers_type_battle() {
        assert_matches!(
            deckmaste_semantics::Type::Battle.lower(),
            deckmaste_core::Type::Battle
        );
    }

    #[test]
    fn lowers_type_creature() {
        assert_matches!(
            deckmaste_semantics::Type::Creature.lower(),
            deckmaste_core::Type::Creature
        );
    }

    #[test]
    fn lowers_type_dungeon() {
        assert_matches!(
            deckmaste_semantics::Type::Dungeon.lower(),
            deckmaste_core::Type::Dungeon
        );
    }

    #[test]
    fn lowers_type_enchantment() {
        assert_matches!(
            deckmaste_semantics::Type::Enchantment.lower(),
            deckmaste_core::Type::Enchantment
        );
    }

    #[test]
    fn lowers_type_instant() {
        assert_matches!(
            deckmaste_semantics::Type::Instant.lower(),
            deckmaste_core::Type::Instant
        );
    }

    #[test]
    fn lowers_type_kindred() {
        assert_matches!(
            deckmaste_semantics::Type::Kindred.lower(),
            deckmaste_core::Type::Kindred
        );
    }

    #[test]
    fn lowers_type_land() {
        assert_matches!(
            deckmaste_semantics::Type::Land.lower(),
            deckmaste_core::Type::Land
        );
    }

    #[test]
    fn lowers_type_planeswalker() {
        assert_matches!(
            deckmaste_semantics::Type::Planeswalker.lower(),
            deckmaste_core::Type::Planeswalker
        );
    }

    #[test]
    fn lowers_type_sorcery() {
        assert_matches!(
            deckmaste_semantics::Type::Sorcery.lower(),
            deckmaste_core::Type::Sorcery
        );
    }

    #[test]
    fn lowers_supertype_basic() {
        assert_matches!(
            deckmaste_semantics::Supertype::Basic.lower(),
            deckmaste_core::Supertype::Basic
        );
    }

    #[test]
    fn lowers_supertype_legendary() {
        assert_matches!(
            deckmaste_semantics::Supertype::Legendary.lower(),
            deckmaste_core::Supertype::Legendary
        );
    }

    #[test]
    fn lowers_supertype_ongoing() {
        assert_matches!(
            deckmaste_semantics::Supertype::Ongoing.lower(),
            deckmaste_core::Supertype::Ongoing
        );
    }

    #[test]
    fn lowers_supertype_snow() {
        assert_matches!(
            deckmaste_semantics::Supertype::Snow.lower(),
            deckmaste_core::Supertype::Snow
        );
    }

    #[test]
    fn lowers_supertype_world() {
        assert_matches!(
            deckmaste_semantics::Supertype::World.lower(),
            deckmaste_core::Supertype::World
        );
    }

    #[test]
    fn lowers_subtype() {
        assert_matches!(
            deckmaste_semantics::Subtype {
                name: "X".into(),
                types: [].into(),
                confers: [].into()
            }
            .lower(),
            deckmaste_core::Subtype {
                name: _,
                types: _,
                confers: _
            }
        );
    }

    #[test]
    fn lowers_type_def() {
        assert_matches!(
            deckmaste_semantics::TypeDef {
                name: "X".into(),
                permanent: false,
                confers: [].into()
            }
            .lower(),
            deckmaste_core::TypeDef {
                name: _,
                permanent_type: false,
                confers: _
            }
        );
    }

    #[test]
    fn lowers_type_ref() {
        assert_matches!(
            deckmaste_semantics::TypeRef(std::sync::Arc::new(minimal_type_def())).lower(),
            deckmaste_core::TypeRef(_)
        );
    }

    #[test]
    fn lowers_subtype_ref() {
        assert_matches!(
            deckmaste_semantics::SubtypeRef(std::sync::Arc::new(minimal_subtype())).lower(),
            deckmaste_core::SubtypeRef(_)
        );
    }
}
