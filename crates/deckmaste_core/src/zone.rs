use serde::Deserialize;
use serde::Serialize;

use crate::Expand;

/// A game zone ([CR#400.1]). Vintage-legal scope: no ante.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Zone {
    Battlefield,
    Command,
    Exile,
    Graveyard,
    Hand,
    Library,
    Stack,
}

impl Zone {
    /// The zone's visibility DEFAULT ([CR#400.2]): hidden vs public is a
    /// property of the zone, not of the cards in it. Library and hand are
    /// hidden "even if all the cards in one such zone happen to be
    /// revealed" — visibility statics grant sight on top of an unchanged
    /// hidden default. Face-down cards in public zones are the exception
    /// machinery ([CR#708]), not a zone property.
    #[must_use]
    pub fn is_hidden(self) -> bool {
        matches!(self, Zone::Hand | Zone::Library)
    }
}
