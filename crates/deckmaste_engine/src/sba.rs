//! State-based actions ([CR#704]). Player losses: zero or less life
//! ([CR#704.5a]), drew from an empty library ([CR#704.5b]), ten or more
//! poison counters ([CR#704.5c]). Creatures with lethal marked damage are
//! destroyed ([CR#704.5g]); tokens stranded off the battlefield cease to
//! exist ([CR#704.5d]).

use deckmaste_core::Type;
use deckmaste_core::Zone;

use crate::event::GameEvent;
use crate::event::LossReason;
use crate::state::GameState;

/// One sweep ([CR#704.3]): the `PlayerLost`, `WillDestroy` (the replaceable
/// destruction intent), and `TokenCeased` events this check would perform. The
/// caller emits them and re-checks until a sweep comes back empty. A destroy's
/// LKI snapshot is captured later, at the will-change apply the `WillDestroy`
/// resolves into (the object is still live then), not here.
#[must_use]
pub fn sweep(state: &GameState) -> Vec<GameEvent> {
    let mut actions = Vec::new();
    let view = state.layers();

    // P0.W6 presence guard: an `OutcomeGate` row in the derived view must
    // suppress matching outcomes at each check — U5 semantics: precedence,
    // not consumption ([CR#101.2,704.3]); concession pierces it
    // ([CR#104.3a]). An unevaluated gate must not let a loss through (or a
    // win past "can't win") silently.
    if crate::legal::statics_present(state, &view, |s| {
        matches!(s, deckmaste_core::StaticEffect::OutcomeGate { .. })
    }) {
        todo!("P0.W6: outcome gates (suppress-per-check, [CR#101.1])");
    }

    let poison: deckmaste_core::Ident = "Poison".into();
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
        } else if state
            .objects
            .obj(player.object)
            .counters
            .get(&poison)
            .is_some_and(|&n| n >= 10)
        {
            // [CR#704.5c]: player counters live on the player's PROXY
            // object ([CR#122.1] — counters go on objects and players; one
            // storage, never a parallel map). Live but dormant: nothing
            // places counters yet (the PutCounters apply arm is a P0.W3
            // seam). Two-Headed Giant swaps in the fifteen-counter TEAM
            // check ([CR#704.6b]) — variant-gated, not built.
            actions.push(GameEvent::PlayerLost {
                player: player.id,
                reason: LossReason::Poison,
            });
        }
    }

    // [CR#704.5g,704.5h]: a creature with lethal marked damage, or struck by
    // any damage from a deathtouch source, is destroyed. We collect the ids
    // to destroy into a `BTreeSet` so that a creature triggering both checks
    // (e.g. it has lethal damage AND was struck by deathtouch) emits only
    // one `ZoneWillChange` event.
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
        // Destroy through the replaceable `WillDestroy` intent so indestructible
        // / regeneration can intercede ([CR#702.12b]): its apply checks the
        // object's destruction-replacement statics and either spares it or
        // commits the battlefield→graveyard move (capturing LKI then,
        // [CR#400.7]). The cause names the verb — lethal-damage destruction is
        // one of "destroyed"'s exactly two causes ([CR#701.8b]), so the named
        // view can narrow on it.
        actions.push(GameEvent::WillDestroy {
            object: id,
            cause: Some(crate::event::Cause::destroy(
                deckmaste_core::Agency::StateBasedAction,
                None,
            )),
        });
    }

    // [CR#704.5d,111.7]: a token in a zone other than the battlefield ceases
    // to exist. The move that stranded it already fired its zone-leave
    // triggers; this sweep just cleans up ([CR#111.7]'s note). Stack objects
    // are exempt: an activated/triggered ability minted from a token source
    // rides the token's `CardId` but is an ability, not the token. (The
    // [CR#111.8] stay-put rule — a token that left the battlefield can't
    // change zones again — is an unwired seam; the window between the move
    // and this sweep is currently unobservable.)
    for obj in state.objects.iter() {
        if matches!(
            obj.zone,
            Some(Zone::Graveyard | Zone::Exile | Zone::Hand | Zone::Library)
        ) && obj.card_id().is_some_and(|c| state.cards.get(c).is_token)
        {
            actions.push(GameEvent::TokenCeased(obj.id));
        }
    }

    actions
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::Card;
    use deckmaste_core::Filter;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::agenda::WorkItem;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
    use crate::matches as obj_matches;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::sba;
    use crate::state::GameConfig;
    use crate::state::GameOutcome;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::step::StepOutcome;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    fn deck(card: &Arc<Card>, n: usize) -> Vec<Arc<Card>> { vec![Arc::clone(card); n] }

    /// A two-player game; player 0's deck is Grizzly Bears.
    /// Returns the state plus a creature object forced onto the battlefield.
    fn bear_on_field() -> (GameState, crate::object::ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
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
        // Force a Grizzly Bears from player 0's hand onto the battlefield.
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
            .expect("a Grizzly Bears in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// Player 0's deck = Darksteel Myr (indestructible 0/1), one on the field.
    fn myr_on_field() -> (GameState, crate::object::ObjectId) {
        let myr = Arc::new(canon().card("Darksteel Myr").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&myr, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let m = *state.zones.hands[0]
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
            .expect("a Darksteel Myr in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != m);
        state.objects.obj_mut(m).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(m);
        (state, m)
    }

    /// [CR#704.5g,702.12b]: an indestructible creature with lethal damage is
    /// NOT destroyed by the SBA — the sweep emits a `WillDestroy`, and its
    /// apply finds the destruction-replacement static and replaces the destroy
    /// to nothing. The Myr stays on the battlefield.
    #[test]
    fn indestructible_survives_lethal_damage() {
        let (mut state, myr) = myr_on_field();
        state.objects.obj_mut(myr).damage = 1; // toughness 1 → lethal
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step(); // WillDestroy applies → replaced to nothing
        assert!(
            state.objects.get(myr).is_some(),
            "indestructible survives lethal damage"
        );
        assert!(state.zones.battlefield.contains(&myr));
        assert!(state.zones.graveyards[0].is_empty(), "not destroyed");
    }

    #[test]
    fn lethal_damage_destroys_a_creature_in_the_sba_sweep() {
        let (mut state, bear) = bear_on_field();

        // Grizzly Bears has toughness 2; set lethal damage. The sweep emits
        // the destroy as a replaceable `WillDestroy` intent (its apply commits
        // the battlefield→graveyard move when nothing replaces it), cause-tagged
        // as the SBA destruction verb ([CR#701.8b]).
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(
                e,
                GameEvent::WillDestroy {
                    object,
                    cause: Some(c),
                } if *object == bear
                    && c.verb == deckmaste_core::Ident::from("Destroy")
                    && c.agency == deckmaste_core::Agency::StateBasedAction
            )),
            "sweep should include a WillDestroy for Grizzly Bears at lethal damage"
        );

        // Sublethal: damage = 1 < toughness 2.
        state.objects.obj_mut(bear).damage = 1;
        let actions = sba::sweep(&state);
        assert!(
            actions
                .iter()
                .all(|e| !matches!(e, GameEvent::WillDestroy { .. })),
            "sweep should NOT include a destroy for Grizzly Bears at sublethal damage"
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

    /// [CR#704.5d,111.7]: a token put into a graveyard is removed from the
    /// game by the next SBA sweep — the graveyard empties and the object is
    /// gone from the store, with no `ZoneChanged` fact (ceasing to exist is
    /// not a move). A token still on the battlefield never ceases.
    #[test]
    fn dead_token_ceases_to_exist() {
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::Effect;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::Reference;
        use deckmaste_core::Token;

        let (mut state, src) = bear_on_field();
        let frame = crate::stack::Frame {
            source: src,
            controller: PlayerId(0),
            targets: vec![],
            bindings: None,
            chosen: None,
            x: None,
        };
        let token = Token {
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![],
            power: None,
            toughness: None,
        };
        state.run_effect(
            Effect::Act(Action::By(
                Reference::You,
                PlayerAction::Create(Count::Literal(1), token.into()),
            )),
            &frame,
        );
        let _ = state.step(); // TokenCreated applies
        let _ = state.step(); // its ZoneChanged fact
        let &token_obj = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the token on the battlefield");

        // On the battlefield the token is exempt.
        assert!(
            sba::sweep(&state)
                .iter()
                .all(|e| !matches!(e, GameEvent::TokenCeased(_))),
            "a battlefield token must not cease"
        );

        // Put it into the graveyard (the generic move: remint + LKI).
        state.schedule_front(vec![WorkItem::Emit(Occurrence::single(
            GameEvent::ZoneWillChange {
                object: token_obj,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard,
                enters: None,
                position: None,
                face: None,
                cause: None,
            },
        ))]);
        let _ = state.step(); // the move applies
        let _ = state.step(); // its ZoneChanged fact
        let dead = state.zones.graveyards[0][0];

        // The sweep emits exactly one TokenCeased for the reminted object.
        let actions = sba::sweep(&state);
        assert_eq!(actions, vec![GameEvent::TokenCeased(dead)]);

        // Applying it removes the object outright — graveyard empty, id gone.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(state.zones.graveyards[0].is_empty(), "[CR#704.5d]");
        assert!(
            state.objects.get(dead).is_none(),
            "the ceased token's id must be gone from the store"
        );
    }

    /// [CR#400.7]: when a creature is destroyed, the old `ObjectId` is removed
    /// from the store entirely, and a fresh `ObjectId` is minted in the owner's
    /// graveyard. The `LkiSnapshot` rides the event.
    #[test]
    fn destroy_remints_old_id_gone_new_in_graveyard() {
        let (mut state, bear) = bear_on_field();
        // Grizzly Bears has toughness 2; set lethal damage.
        state.objects.obj_mut(bear).damage = 2;
        let actions = sba::sweep(&state);
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        // WillDestroy applies (nothing replaces it) → ZoneWillChange remints.
        let _ = state.step();
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
