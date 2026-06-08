//! State-based actions (CR 704). The skeleton checks two: a player at zero
//! or less life loses (704.5a), and a player who drew from an empty library
//! loses (704.5c). Task 6 adds CR 704.5g: a creature with lethal marked
//! damage is destroyed.

use deckmaste_core::{StatValue, Type};

use crate::event::{GameEvent, LossReason};
use crate::state::GameState;

/// One sweep (CR 704.3): the `PlayerLost` and `Destroyed` events this check
/// would perform. The caller emits them and re-checks until a sweep comes
/// back empty.
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

    // CR 704.5g: a creature with lethal marked damage is destroyed.
    for &id in &state.zones.battlefield {
        let obj = state.objects.obj(id);
        let face = crate::derive::face(state.def(id));
        if let Some(StatValue::Number(toughness)) = face.toughness
            && face.types.contains(&Type::Creature)
        {
            // Toughness is an Int (i32). Printed toughness could be
            // negative or zero (e.g. */0 token) — those are handled by
            // other SBAs (not yet wired); skip here to avoid underflow.
            if toughness > 0 {
                #[expect(clippy::cast_sign_loss)]
                if obj.damage >= toughness as deckmaste_core::Uint {
                    actions.push(GameEvent::Destroyed(id));
                }
            }
        }
    }

    actions
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::{Card, Filter, Type, Zone};

    use crate::event::GameEvent;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::{GameConfig, GameState, PlayerConfig, StartingPlayer};
    use crate::{matches as obj_matches, sba};

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

    /// A two-player game; player 0's deck is Grizzly Bears.
    /// Returns the state plus a Bears object forced onto the battlefield.
    fn bear_on_field() -> (GameState, crate::object::ObjectId) {
        let plugin = builtin();
        let bears = Arc::new(plugin.card("Grizzly Bears").unwrap());
        let forest = Arc::new(plugin.card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&bears, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        // Force a Bears from player 0's hand onto the battlefield.
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                obj_matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        Type::Creature,
                    )),
                )
            })
            .expect("a Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    #[test]
    fn lethal_damage_destroys_a_creature_in_the_sba_sweep() {
        let (mut state, bear) = bear_on_field();

        // Grizzly Bears has toughness 2; set lethal damage.
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .any(|e| matches!(e, GameEvent::Destroyed(o) if *o == bear)),
            "sweep should include Destroyed for Bears at lethal damage"
        );

        // Sublethal: damage = 1 < toughness 2.
        state.objects.obj_mut(bear).damage = 1;
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .all(|e| !matches!(e, GameEvent::Destroyed(_))),
            "sweep should NOT include Destroyed for Bears at sublethal damage"
        );
    }

    #[test]
    fn players_not_on_battlefield_do_not_trigger_704_5g() {
        let (state, _) = bear_on_field();
        let proxy = state.players[0].object;
        // Player proxy should never have source Card(...), so def() would
        // panic — the sweep guards against this by only scanning the
        // battlefield (which never contains player proxies).
        // Just confirm: the proxy's source is Player, not on battlefield.
        assert!(matches!(
            state.objects.obj(proxy).source,
            ObjectSource::Player(_)
        ));
        assert!(!state.zones.battlefield.contains(&proxy));
    }
}
