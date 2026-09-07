use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Ident;
use crate::Predicate;
use crate::continuous::StaticEffect;

/// Where a designation attaches (taxonomy §8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DesignationScope {
    Object,
    Player,
    Game,
}

/// The data shape a stored designation carries (taxonomy §8).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DesignationShape {
    /// Present-or-absent (e.g. monstrous).
    Flag,
    /// A number (e.g. a count).
    Number,
    /// One of a fixed set of named values.
    Enum(Arc<[Ident]>),
    /// A relation to another object.
    Relation,
}

/// How unique a designation is across its scope (taxonomy §8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DesignationUniqueness {
    None,
    PerPlayer,
    PerGame,
}

/// How long a designation persists by default (taxonomy §8). Object lifetime
/// is the free default via [CR#400.7].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DesignationPersistence {
    ObjectLifetime,
    UntilEndOfTurn,
    EffectSupplied,
    /// Persists without an intrinsic expiry condition.
    Permanently,
}

/// A designation's definition: stored (with metadata) or derived from a
/// predicate. Granting a derived designation is a load error (invariant §7).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DesignationDef {
    /// A stored designation, with its full metadata (taxonomy §8).
    Stored {
        scope: DesignationScope,
        shape: DesignationShape,
        uniqueness: DesignationUniqueness,
        persistence: DesignationPersistence,
        /// Abilities the designation confers (e.g. suspected's menace).
        #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
        payload: Arc<[StaticEffect]>,
    },
    /// A designation derived by re-evaluating a filter (e.g. `Modified`).
    Derived(Predicate),
    /// A designation derived by re-evaluating a condition. Boxed: `Condition`
    /// dominates this enum's size; boxing keeps `DesignationDef` small
    /// (`clippy::large_enum_variant`).
    DerivedIf(Arc<Condition>),
}

/// A deed defined by the core rules, independently of the open keyword-action
/// registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CoreDeed {
    Attack,
    Block,
    Target,
    Copy,
    Draw,
    GainLife,
    LoseGame,
    WinGame,
    Spend,
    Trigger,
    Put,
    Return,
    GainControl,
    Unlock,
    FullyUnlock,
}

/// A designation declaration (§6, taxonomy §8): an open `Ident`
/// vocabulary carrying a definition. Declaration-file type (like `MacroDef`);
/// references to designations elsewhere use a bare `Ident`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DesignationDecl {
    pub name: Ident,
    pub definition: DesignationDef,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permanent_duration_is_spelled_adverbially() {
        let persistence: DesignationPersistence =
            crate::ron::options().from_str("Permanently").unwrap();
        assert_eq!(persistence, DesignationPersistence::Permanently);
        assert_eq!(
            crate::ron::options().to_string(&persistence).unwrap(),
            "Permanently"
        );
        let err = crate::ron::options()
            .from_str::<DesignationPersistence>("Permanent")
            .unwrap_err();
        let err = err.to_string();
        assert!(err.contains("Permanent"));
    }
}
