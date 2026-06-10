use macro_ron::Ident;
use serde::{Deserialize, Serialize};

use crate::{Expand, Property};

// [CR#300.1]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum Type {
    Artifact,
    Battle,
    // Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Kindred,
    Land,
    // Phenomenon,
    // Plane,
    Planeswalker,
    // Scheme,
    Sorcery,
    // Vanguard,
}

// [CR#205.4a]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

/// A subtype: its name, the card types it can appear on ([CR#205.3]), and
/// what it confers on its bearers — how [CR#305.6] gives basic lands their
/// mana abilities, as plugin data rather than an engine special case.
/// Embedded in the value: a macro-expanded card describes the entirety of
/// its behavior.
///
/// Subtypes are open-ended data, declared by plugins (usually as macro
/// definitions produced by meta-macros like
/// `LandType(name: "Forest", template: "Forest")`) rather than baked in as
/// Rust variants.
/// Plain serde on both sides; card files reference declared subtypes by bare
/// name (`Forest`), which the macro-aware reader expands to the full
/// declaration before this type ever sees it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
pub struct Subtype {
    pub name: Ident,
    pub types: Vec<Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub confers: Vec<Property>,
}
