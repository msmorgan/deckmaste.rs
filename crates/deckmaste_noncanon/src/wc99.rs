//! Historical deck fixtures: Mark Le Pine's Sped Red and Matt Linde's
//! Mono-Green Stompy from Worlds 1999, plus Sigurd Eskeland's Worlds 2000
//! RDW2K. The primary matchup is the two WC99 decks.

use crate::deck::DeckSpec;

/// Sigurd Eskeland's mono-red Standard deck from Worlds 2000.
pub const RDW2K: DeckSpec = DeckSpec {
    name: "RDW2K (Sigurd Eskeland, Worlds 2000)",
    basics: ("Mountain", 16),
    rest: &[
        ("Goblin Patrol", 4),
        ("Kris Mage", 1),
        ("Goblin Cadets", 4),
        ("Viashino Cutthroat", 2),
        ("Hammer of Bogardan", 4),
        ("Pillage", 3),
        ("Arc Lightning", 4),
        ("Lightning Blast", 2),
        ("Rhystic Lightning", 3),
        ("Shock", 4),
        ("Parch", 2),
        ("Seal of Fire", 4),
        ("Ghitu Encampment", 3),
        ("Rishadan Port", 4),
    ],
};

pub const SPED_RED: DeckSpec = DeckSpec {
    name: "Sped Red (Mark Le Pine, WC99)",
    basics: ("Mountain", 16),
    rest: &[
        ("Avalanche Riders", 4),
        ("Cursed Scroll", 4),
        ("Jackal Pup", 4),
        ("Mogg Fanatic", 4),
        ("Pillage", 4),
        ("Shock", 4),
        ("Stone Rain", 4),
        ("Wasteland", 4),
        ("Fireslinger", 3),
        ("Hammer of Bogardan", 3),
        ("Ancient Tomb", 2),
        ("Arc Lightning", 2),
        ("Ghitu Encampment", 2),
    ],
};

pub const STOMPY: DeckSpec = DeckSpec {
    name: "Mono-Green Stompy (Matt Linde, WC99)",
    basics: ("Forest", 14),
    rest: &[
        ("Albino Troll", 4),
        ("Cursed Scroll", 4),
        ("Elvish Lyrist", 4),
        ("Gaea's Cradle", 4),
        ("Giant Growth", 4),
        ("Llanowar Elves", 4),
        ("Pouncing Jaguar", 4),
        ("Rancor", 4),
        ("River Boa", 4),
        ("Treetop Village", 4),
        ("Wild Dogs", 4),
        ("Uktabi Orangutan", 2),
    ],
};
