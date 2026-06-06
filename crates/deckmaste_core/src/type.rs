use serde::{Deserialize, Serialize};

// CR 300.1
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
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
