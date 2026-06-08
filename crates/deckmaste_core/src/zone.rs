use serde::{Deserialize, Serialize};

/// A game zone ([CR#400.1]). Vintage-legal scope: no ante.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Zone {
    Battlefield,
    Command,
    Exile,
    Graveyard,
    Hand,
    Library,
    Stack,
}
