//! `type` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Type {
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

impl Lower for deckmaste_authoring::Supertype {
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

impl Lower for deckmaste_authoring::Subtype {
    type Target = deckmaste_core::Subtype;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::Subtype {
            name: self.name.lower(),
            types: self.types.lower(),
            confers: self.confers.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::TypeDef {
    type Target = deckmaste_core::TypeDef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TypeDef {
            name: self.name.lower(),
            permanent: self.permanent.lower(),
            confers: self.confers.lower(),
        }
    }
}

impl Lower for deckmaste_authoring::TypeRef {
    type Target = deckmaste_core::TypeRef;
    fn lower(self) -> <Self as Lower>::Target {
        deckmaste_core::TypeRef(self.0.lower())
    }
}

impl Lower for deckmaste_authoring::SubtypeRef {
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

    use crate::assert_lowers;
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_type_artifact() {
        assert_lowers(deckmaste_authoring::Type::Artifact);
    }

    #[test]
    fn lowers_type_battle() {
        assert_lowers(deckmaste_authoring::Type::Battle);
    }

    #[test]
    fn lowers_type_creature() {
        assert_lowers(deckmaste_authoring::Type::Creature);
    }

    #[test]
    fn lowers_type_dungeon() {
        assert_lowers(deckmaste_authoring::Type::Dungeon);
    }

    #[test]
    fn lowers_type_enchantment() {
        assert_lowers(deckmaste_authoring::Type::Enchantment);
    }

    #[test]
    fn lowers_type_instant() {
        assert_lowers(deckmaste_authoring::Type::Instant);
    }

    #[test]
    fn lowers_type_kindred() {
        assert_lowers(deckmaste_authoring::Type::Kindred);
    }

    #[test]
    fn lowers_type_land() {
        assert_lowers(deckmaste_authoring::Type::Land);
    }

    #[test]
    fn lowers_type_planeswalker() {
        assert_lowers(deckmaste_authoring::Type::Planeswalker);
    }

    #[test]
    fn lowers_type_sorcery() {
        assert_lowers(deckmaste_authoring::Type::Sorcery);
    }

    #[test]
    fn lowers_supertype_basic() {
        assert_lowers(deckmaste_authoring::Supertype::Basic);
    }

    #[test]
    fn lowers_supertype_legendary() {
        assert_lowers(deckmaste_authoring::Supertype::Legendary);
    }

    #[test]
    fn lowers_supertype_ongoing() {
        assert_lowers(deckmaste_authoring::Supertype::Ongoing);
    }

    #[test]
    fn lowers_supertype_snow() {
        assert_lowers(deckmaste_authoring::Supertype::Snow);
    }

    #[test]
    fn lowers_supertype_world() {
        assert_lowers(deckmaste_authoring::Supertype::World);
    }

    #[test]
    fn lowers_subtype() {
        assert_lowers(deckmaste_authoring::Subtype {
            name: "X".into(),
            types: [].into(),
            confers: [].into(),
        });
    }

    #[test]
    fn lowers_type_def() {
        assert_lowers(deckmaste_authoring::TypeDef {
            name: "X".into(),
            permanent: false,
            confers: [].into(),
        });
    }

    #[test]
    fn lowers_type_ref() {
        assert_lowers_debug(deckmaste_authoring::TypeRef(std::sync::Arc::new(
            minimal_type_def(),
        )));
    }

    #[test]
    fn lowers_subtype_ref() {
        assert_lowers_debug(deckmaste_authoring::SubtypeRef(std::sync::Arc::new(
            minimal_subtype(),
        )));
    }
}
