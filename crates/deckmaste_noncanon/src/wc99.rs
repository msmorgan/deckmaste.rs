//! The WC99 matchup: Mark Le Pine's Sped Red vs Matt Linde's Mono-Green
//! Stompy (Worlds 1999), plus the per-wave allowlists the subset builder
//! consumes. Growing an allowlist is the per-wave ritual: graduate the card
//! in plugins/noncanon, add it here, extend the probes.

use crate::deck::DeckSpec;

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

/// What's graduated AND engine-runnable today. Graduated is not enough:
/// Mogg Fanatic parses but its sacrifice can never activate — the engine's
/// `cost_summary` rejects verb costs (`Do(Sacrifice(This))`), so the card
/// would be pure burn-dilution. It joins the moment activation verb costs
/// land.
pub const SPED_RED_ALLOWLIST: &[&str] = &["Shock"];
pub const STOMPY_ALLOWLIST: &[&str] = &["Llanowar Elves"];

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// Every listed card has a frame (graduated or todo) in the plugin —
    /// guards the seeding against decklist drift.
    #[test]
    fn all_wc99_frames_are_present() {
        let cards = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/noncanon/cards");
        for spec in [&SPED_RED, &STOMPY] {
            for &(name, _) in spec.rest {
                let graduated = cards.join(format!("{name}.ron")).exists();
                let todo = cards.join(format!("{name}.ron.todo")).exists();
                assert!(graduated || todo, "{name}: no frame in plugins/noncanon");
            }
        }
    }
}
