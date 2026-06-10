//! Mechanic probes: counters proving the matchup's machinery actually fired,
//! accumulated from the event stream. Asserted across a seed batch, never
//! per game.

use deckmaste_core::Zone;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameState;
use deckmaste_engine::ObjectId;

#[derive(Debug, Clone, Copy, Default)]
pub struct Probes {
    pub lands_played: u32,
    pub spells_cast: u32,
    pub attacks_declared: u32,
    pub blocks_declared: u32,
    /// Combat/creature damage (source was on the battlefield when observed).
    pub creature_damage_to_players: u32,
    /// Spell damage (source not on the battlefield — it resolved and left).
    pub spell_damage_to_creatures: u32,
    pub spell_damage_to_players: u32,
    /// A nonland permanent tapped (wave 0: Llanowar Elves making mana).
    pub nonland_taps: u32,
    pub battlefield_to_graveyard: u32,
}

impl Probes {
    /// Folds one progress batch in. `state` is the post-batch state: objects
    /// referenced by events may already have moved on, so membership checks
    /// guard every def-style lookup.
    pub fn observe(&mut self, state: &GameState, events: &[GameEvent], proxies: [ObjectId; 2]) {
        for ev in events {
            match ev {
                GameEvent::LandPlayed { .. } => self.lands_played += 1,
                GameEvent::SpellCast(_) => self.spells_cast += 1,
                GameEvent::Attacking(_) => self.attacks_declared += 1,
                GameEvent::Blocked { .. } => self.blocks_declared += 1,
                GameEvent::Tapped(o)
                    if state.zones.battlefield.contains(o)
                        && !crate::observe::face(state.def(*o))
                            .types
                            .contains(&deckmaste_core::Type::Land) =>
                {
                    self.nonland_taps += 1;
                }
                GameEvent::DamageDealt { source, target, .. } => {
                    let from_battlefield = state.zones.battlefield.contains(source);
                    let to_player = proxies.contains(target);
                    match (from_battlefield, to_player) {
                        (true, true) => self.creature_damage_to_players += 1,
                        (false, true) => self.spell_damage_to_players += 1,
                        (false, false) => self.spell_damage_to_creatures += 1,
                        (true, false) => {} // creature-on-creature combat
                    }
                }
                GameEvent::ZoneChanged {
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard,
                    ..
                } => {
                    self.battlefield_to_graveyard += 1;
                }
                _ => {}
            }
        }
    }

    pub fn merge(&mut self, other: &Probes) {
        self.lands_played += other.lands_played;
        self.spells_cast += other.spells_cast;
        self.attacks_declared += other.attacks_declared;
        self.blocks_declared += other.blocks_declared;
        self.creature_damage_to_players += other.creature_damage_to_players;
        self.spell_damage_to_creatures += other.spell_damage_to_creatures;
        self.spell_damage_to_players += other.spell_damage_to_players;
        self.nonland_taps += other.nonland_taps;
        self.battlefield_to_graveyard += other.battlefield_to_graveyard;
    }
}
