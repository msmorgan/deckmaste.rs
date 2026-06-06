use serde::{Deserialize, Serialize};

use crate::mana::ManaSymbol;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Symbol {
    Tap,       // {T}
    Untap,     // {Q}
    Phyrexian, // Rage Extractor is the only card with {H}.
    Energy,    // {E}
    Pawprint,  // {P}
    Ticket,    // Two vintage-legal cards with {TK}.
    Mana(ManaSymbol),
}
