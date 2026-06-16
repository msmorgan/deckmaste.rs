use serde::Deserialize;
use serde::Serialize;

use crate::Condition;
use crate::Filter;
use crate::Ident;
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
    Enum(Vec<Ident>),
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
    Permanent,
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
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        payload: Vec<StaticEffect>,
    },
    /// A designation derived by re-evaluating a filter (e.g. `Modified`).
    Derived(Filter),
    /// A designation derived by re-evaluating a condition. Boxed: `Condition`
    /// dominates this enum's size; boxing keeps `DesignationDef` small
    /// (`clippy::large_enum_variant`).
    DerivedIf(Box<Condition>),
}

/// A designation declaration (§6, taxonomy §8): an open `Ident`
/// vocabulary carrying a definition. Declaration-file type (like `MacroDef`);
/// references to designations elsewhere use a bare `Ident`. No loader wiring
/// yet.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DesignationDecl {
    pub name: Ident,
    pub definition: DesignationDef,
}
