//! State-based actions ([CR#704]). The skeleton checks two: a player at zero
//! or less life loses ([CR#704.5a]), and a player who drew from an empty
//! library loses ([CR#704.5c]). Task 6 adds [CR#704.5g]: a creature with lethal
//! marked damage is destroyed.

use deckmaste_core::{Type, Zone};

use crate::event::{GameEvent, LossReason};
use crate::state::GameState;

/// One sweep ([CR#704.3]): the `PlayerLost` and `ZoneWillChange`
/// (battlefield→graveyard) events this check would perform. The caller emits
/// them and re-checks until a sweep comes back empty. The LKI snapshot for a
/// destroy is captured later, at the will-change apply (the object is still
/// live then), not here.
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

    // [CR#704.5g,704.5h]: a creature with lethal marked damage, or struck by
    // any damage from a deathtouch source, is destroyed. We collect the ids
    // to destroy into a `BTreeSet` so that a creature triggering both checks
    // (e.g. it has lethal damage AND was struck by deathtouch) emits only
    // one `ZoneWillChange` event.
    let view = state.layers();
    let mut to_destroy = std::collections::BTreeSet::new();
    for &id in &state.zones.battlefield {
        let obj = state.objects.obj(id);
        let c = view.get(id);
        if !c.card_types.contains(&Type::Creature) {
            continue;
        }
        if let Some(toughness) = c.toughness {
            // Both destroy SBAs require toughness > 0 ([CR#704.5g], [CR#704.5h]);
            // toughness is an Int (i32) and a creature with toughness ≤ 0 is
            // handled by other SBAs (not yet wired). Guard once for both and to
            // avoid the cast underflow.
            if toughness > 0 {
                // [CR#704.5g]: lethal marked damage (damage >= toughness).
                #[expect(clippy::cast_sign_loss)]
                if obj.damage >= toughness as deckmaste_core::Uint {
                    to_destroy.insert(id);
                }
                // [CR#704.5h]: dealt any damage by a deathtouch source.
                if obj.struck_by_deathtouch {
                    to_destroy.insert(id);
                }
            }
        }
    }
    for id in to_destroy {
        // Destroy. The LKI snapshot is captured at the will-change apply
        // while the object is still live ([CR#400.7]).
        actions.push(GameEvent::ZoneWillChange {
            object: id,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
        });
    }

    actions
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::{Card, Filter, Type, Zone};

    use crate::agenda::WorkItem;
    use crate::event::{GameEvent, Occurrence};
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::{GameConfig, GameOutcome, GameState, PlayerConfig, StartingPlayer};
    use crate::step::StepOutcome;
    use crate::{matches as obj_matches, sba};

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn testing() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/testing"),
        )
        .unwrap()
    }

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

    /// A two-player game; player 0's deck is Vanilla Creature.
    /// Returns the state plus a creature object forced onto the battlefield.
    fn bear_on_field() -> (GameState, crate::object::ObjectId) {
        let bears = Arc::new(testing().card("Vanilla Creature").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
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
        // Force a Vanilla Creature from player 0's hand onto the battlefield.
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
            .expect("a Vanilla Creature in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    #[test]
    fn lethal_damage_destroys_a_creature_in_the_sba_sweep() {
        let (mut state, bear) = bear_on_field();

        // Vanilla Creature has toughness 2; set lethal damage. The sweep emits
        // the destroy as a battlefield→graveyard ZoneWillChange (no snapshot —
        // captured later, at the will-change apply).
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::ZoneWillChange {
                    object,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard,
                    enters: None,
                    position: None,
                } if *object == bear
            )),
            "sweep should include a battlefield→graveyard ZoneWillChange for Vanilla Creature at lethal damage"
        );

        // Sublethal: damage = 1 < toughness 2.
        state.objects.obj_mut(bear).damage = 1;
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .all(|e| !matches!(e, GameEvent::ZoneWillChange { .. })),
            "sweep should NOT include a destroy for Vanilla Creature at sublethal damage"
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

    /// [CR#104.4a,704.3]: two players at ≤0 life in the same sweep → Draw, not
    /// a Win for whoever was checked first.
    #[test]
    fn simultaneous_double_loss_is_a_draw() {
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        state.players[0].life = 0;
        state.players[1].life = 0;
        state.schedule_front(vec![WorkItem::CheckSbas]);
        loop {
            if let StepOutcome::GameOver(o) = state.step() {
                assert_eq!(o, GameOutcome::Draw);
                return;
            }
        }
    }

    /// [CR#400.7]: when a creature is destroyed, the old `ObjectId` is removed
    /// from the store entirely, and a fresh `ObjectId` is minted in the owner's
    /// graveyard. The `LkiSnapshot` rides the event.
    #[test]
    fn destroy_remints_old_id_gone_new_in_graveyard() {
        let (mut state, bear) = bear_on_field();
        // Vanilla Creature has toughness 2; set lethal damage.
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(
            state.objects.get(bear).is_none(),
            "old battlefield id must be gone from the object store"
        );
        assert!(
            !state.zones.battlefield.contains(&bear),
            "old id must not remain on the battlefield"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "owner's graveyard must contain exactly one object"
        );
        let new = state.zones.graveyards[0][0];
        assert_ne!(new, bear, "graveyard object must have a fresh ObjectId");
    }
}
