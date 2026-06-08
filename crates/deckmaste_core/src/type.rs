use serde::{Deserialize, Serialize};

// [CR#300.1]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}
