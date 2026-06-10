use serde::{Deserialize, Serialize};

use crate::Expand;

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
