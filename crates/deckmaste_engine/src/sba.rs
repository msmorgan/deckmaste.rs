//! State-based actions ([CR#704]). The skeleton checks two: a player at zero
//! or less life loses ([CR#704.5a]), and a player who drew from an empty
//! library loses ([CR#704.5c]).

use crate::event::{GameEvent, LossReason};
use crate::state::GameState;

/// One sweep ([CR#704.3]): the `PlayerLost` events this check would perform.
/// The caller emits them and re-checks until a sweep comes back empty.
#[must_use]
pub fn sweep(state: &GameState) -> Vec<GameEvent> {
    let mut actions = Vec::new();
    for player in &state.players {
        if player.lost {
            continue;
        }
        if player.life <= 0 {
            actions.push(GameEvent::PlayerLost {
                player: player.id,
                reason: LossReason::LifeZero,
            });
        } else if player.drew_from_empty {
            actions.push(GameEvent::PlayerLost {
                player: player.id,
                reason: LossReason::DrewFromEmpty,
            });
        }
    }
    actions
}
