//! State-based actions ([CR#704]). Player losses: zero or less life
//! ([CR#704.5a]), drew from an empty library ([CR#704.5b]), ten or more
//! poison counters ([CR#704.5c]). Creatures with lethal marked damage are
//! destroyed ([CR#704.5g]); tokens stranded off the battlefield cease to
//! exist ([CR#704.5d]).

use deckmaste_core::Type;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::event::GameEvent;
use crate::event::LossReason;
use crate::event::Occurrence;
use crate::object::ObjectId;
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

    // Attachment SBAs ([CR#704.5m..704.5p]) — GENERIC, no subtype branch.
    actions.extend(attachment_sbas(state, &view));

    actions
}

/// The attachment state-based actions ([CR#704.5m..704.5p]) — extracted from
/// [`sweep`] but logically part of the same [CR#704.3] check. Two passes,
/// both keyed on conferred data + the `attached_to` relation only; NEVER on
/// the Aura/Equipment/Fortification subtype:
///
/// 1. **Firing `Sba { when, then }` statics.** The Aura graveyard rule
///    ([CR#704.5m]) is `Sba(Not(LegallyAttached(Ref(This))), Move(Ref(This),
///    Graveyard))`, conferred `Innate` by the Aura subtype. For each
///    battlefield object, for each `Sba` it carries (peeling `Innate`),
///    evaluate `when` with `This` = the object; if true, run `then`'s events.
///    Objects a firing `Sba` removes this sweep are tracked so pass 2 doesn't
///    double-handle them.
/// 2. **Generic illegal-attachment cleanup.** Any object attached to an illegal
///    host (per `attachment_legal`) that no firing `Sba` removed → becomes
///    unattached and stays ([CR#704.5n] Equipment/Fortification; [CR#704.5p]
///    creature / battle / other permanent — engine-identical).
fn attachment_sbas(state: &GameState, view: &crate::layer::LayeredView) -> Vec<GameEvent> {
    let mut out = Vec::new();
    let mut removed_by_sba: std::collections::BTreeSet<ObjectId> =
        std::collections::BTreeSet::new();

    // (1) Firing `Sba` statics.
    for &id in &state.zones.battlefield {
        // A `This`-anchored frame: `condition_holds`/`action_items` resolve
        // `Ref(This)` to this object via the frame source ([CR#603.10a]).
        let frame = crate::stack::Frame {
            source: id,
            controller: state.objects.obj(id).controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
        };
        let mut rows: Vec<(deckmaste_core::Condition, deckmaste_core::Effect)> = Vec::new();
        crate::legal::for_each_static(view, id, |e| {
            if let deckmaste_core::StaticEffect::Sba { when, then } = e {
                rows.push((when.clone(), (**then).clone()));
            }
        });
        for (when, then) in rows {
            if !state.condition_holds(&when, &frame) {
                continue;
            }
            // Run `then`. For Stage 2 the only shape is `Act(<Action>)` (the
            // Aura's `Move`, the Saga generalization's `Sacrifice`); the
            // produced events are flattened out of the `Emit` work items. A
            // non-`Act` `then` (Sequence, choices) is a documented seam.
            let deckmaste_core::Effect::Act(action) = &then else {
                todo!("SBA `then` is only Act(<Action>) in Stage 2 (got {then:?})");
            };
            for item in state.action_items(action, &frame) {
                if let WorkItem::Emit(occ) = item {
                    match occ {
                        Occurrence::Single(ev) => out.push(ev),
                        Occurrence::Batch(evs) => out.extend(evs),
                    }
                }
            }
            // This object is being moved/removed by its own SBA this sweep;
            // pass 2 must not also unattach it.
            removed_by_sba.insert(id);
        }
    }

    // (2) Generic illegal-attachment cleanup ([CR#704.5n,704.5p]).
    for &id in &state.zones.battlefield {
        if removed_by_sba.contains(&id) {
            continue;
        }
        if let Some(host) = state.objects.obj(id).attached_to
            && !crate::legal::attachment_legal(state, id, host)
        {
            // Becomes unattached, stays on the battlefield. The `attached_to`
            // clear happens at the `Unattached` apply (transition-only).
            out.push(GameEvent::Unattached {
                attachment: id,
                former_host: host,
            });
        }
    }

    out
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

    /// The graduated WIZARDS corpus, loaded over its `builtin` sibling prelude
    /// (same path real wizards cards load through). Proves the Aura/Equipment/
    /// Fortification `confers:` reach a wizards card: the defs live in builtin,
    /// and the generator emits no confers-less wizards stub to shadow them.
    fn wizards() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/wizards"),
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

    /// End-to-end through the WIZARDS load path ([CR#704.5m]): a graduated
    /// wizards Aura (Angelic Gift) carries the Aura subtype's `Innate`
    /// graveyard `Sba` *via the data*, not in-Rust scaffolding. Loaded over the
    /// builtin sibling prelude, put on the battlefield UNATTACHED, the generic
    /// SBA sweep fires its battlefield→graveyard move. This is the regression
    /// the fix targets: the Aura `confers:` lives in builtin and the generator
    /// emits no confers-less wizards stub to shadow it, so a fresh wizards card
    /// inherits the attachment rule.
    #[test]
    fn wizards_aura_carries_innate_graveyard_sba() {
        let gift = Arc::new(wizards().card("Angelic Gift").unwrap());
        // Sanity: the loaded card actually carries the Aura subtype's confer.
        // (`derive::printed_of_face` is what flattens it onto the object.)
        let face = crate::derive::face(&gift);
        assert!(
            face.subtypes
                .iter()
                .any(|s| s.confers.iter().any(|p| matches!(
                    p,
                    deckmaste_core::Property::Ability(a) if a.is_innate()
                ))),
            "the wizards Aura card embeds the Innate confer; subtypes: {:?}",
            face.subtypes
        );

        let mut state = GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let card_id = state.cards.push(gift, PlayerId(0));
        let aura = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(aura);

        // Unattached → `LegallyAttached` is false → the conferred Innate SBA
        // fires, moving the Aura to its owner's graveyard.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == aura)),
            "a graduated wizards Aura's Innate graveyard SBA fires when unattached \
             ([CR#704.5m]); got {actions:?}"
        );
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

    // --- Attachment SBAs ([CR#704.5m..704.5p]) ---------------------------------

    use deckmaste_core::Ability;
    use deckmaste_core::CardFace;
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Condition;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Effect;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    }

    fn on_field(
        state: &mut GameState,
        name: &str,
        types: Vec<Type>,
        abilities: Vec<Ability>,
    ) -> crate::object::ObjectId {
        let card = Card::Normal(CardFace {
            name: name.into(),
            types,
            abilities,
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// The Aura-subtype shape (scaffolded in-Rust): `Innate(Static([Sba(Not(
    /// LegallyAttached(Ref(This))), Move(Ref(This), Graveyard))]))`.
    fn aura_graveyard_sba() -> Ability {
        Ability::Innate(Box::new(Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Sba {
                when: Condition::Not(Box::new(Condition::LegallyAttached(Reference::This))),
                then: Box::new(Effect::Act(deckmaste_core::Action::Move(
                    Selection::Ref(Reference::This),
                    Zone::Graveyard,
                ))),
            }],
            characteristic_defining: false,
        })))
    }

    /// The Equipment-subtype shape: `Innate(Static([Cant(Attach(what:
    /// Ref(This), to: Not(Creature)))]))`.
    fn equipment_host_rule() -> Ability {
        Ability::Innate(Box::new(Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Attach {
                    what: Filter::Ref(Reference::This),
                    to: Filter::Not(Box::new(Filter::Characteristic(
                        CharacteristicFilter::Type(Type::Creature),
                    ))),
                },
            ))],
            characteristic_defining: false,
        })))
    }

    /// [CR#704.5m]: an Aura (carrying the Innate graveyard `Sba`) that is
    /// UNATTACHED fires the SBA → a `ZoneWillChange(Battlefield → Graveyard)`
    /// for it. Generic — driven by the `Sba` static, not the subtype.
    #[test]
    fn sba_attach_unattached_aura_goes_to_graveyard() {
        let mut state = game();
        let aura = on_field(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_graveyard_sba()],
        );
        // It is unattached → `LegallyAttached` is false → the SBA fires.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == aura)),
            "unattached Aura is moved to the graveyard ([CR#704.5m]); got {actions:?}"
        );
    }

    /// Lock a `LoseAllAbilities` (layer-6, end-of-game) continuous effect onto
    /// `id` — strips its normal/granted abilities, but NOT its `Innate` rules.
    fn lose_all_abilities(state: &mut GameState, id: crate::object::ObjectId) {
        use deckmaste_core::Duration;
        use deckmaste_core::Modification;

        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;

        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![id]),
            changes: vec![Modification::LoseAllAbilities],
            duration: Duration::EndOfGame,
            is_cda: false,
        });
    }

    /// Review #7 e2e — the whole point of making the Aura graveyard rule an
    /// `Innate` SBA ([CR#113.12,704.5m]): an Aura whose normal abilities are
    /// ALL stripped by an active `LoseAllAbilities` and is UNATTACHED still
    /// goes to the graveyard. The `Innate(Sba(...))` survives ability
    /// removal (layer-6 retain), so the sweep still fires it — emitting the
    /// battlefield→graveyard move, and (driven to completion) landing the
    /// reminted object in its owner's graveyard.
    #[test]
    fn ability_less_aura_still_graveyards() {
        let mut state = game();
        let aura = on_field(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_graveyard_sba()],
        );
        // Strip ALL of the Aura's abilities. The Innate SBA must survive.
        lose_all_abilities(&mut state, aura);

        // The sweep STILL emits the graveyard move for the unattached Aura.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::ZoneWillChange { object, to: Zone::Graveyard, .. } if *object == aura)),
            "Innate graveyard SBA survives LoseAllAbilities ([CR#113.12,704.5m]); got {actions:?}"
        );

        // Drive it to completion: the Aura ends up in its owner's graveyard.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step(); // the move applies (remint + LKI)
        let _ = state.step(); // its ZoneChanged fact
        assert!(
            state.objects.get(aura).is_none(),
            "old battlefield id is gone after the move"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "the ability-less Aura landed in its owner's graveyard"
        );
    }

    /// [CR#704.5m]: an Aura legally attached to a creature does NOT fire its
    /// graveyard SBA.
    #[test]
    fn sba_attach_legally_attached_aura_stays() {
        let mut state = game();
        let aura = on_field(
            &mut state,
            "Test Aura",
            vec![Type::Enchantment],
            vec![aura_graveyard_sba()],
        );
        let host = on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        state.objects.obj_mut(aura).attached_to = Some(host);
        let actions = sba::sweep(&state);
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneWillChange { object, .. } if *object == aura)),
            "legally-attached Aura stays put; got {actions:?}"
        );
    }

    /// [CR#704.5n]: an Equipment (no firing `Sba`) attached to an ILLEGAL host
    /// (a non-creature) becomes unattached and stays — the generic
    /// illegal-attachment cleanup, NO subtype branch.
    #[test]
    fn sba_attach_illegal_equipment_unattaches() {
        let mut state = game();
        let equip = on_field(
            &mut state,
            "Test Equipment",
            vec![Type::Artifact],
            vec![equipment_host_rule()],
        );
        // Illegally attached to a non-creature artifact.
        let rock = on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);
        state.objects.obj_mut(equip).attached_to = Some(rock);

        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::Unattached { attachment, former_host }
                if *attachment == equip && *former_host == rock)),
            "illegally-attached Equipment becomes unattached ([CR#704.5n]); got {actions:?}"
        );
        // It does NOT go to the graveyard (no firing Sba).
        assert!(
            !actions
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneWillChange { object, .. } if *object == equip)),
            "Equipment stays on the battlefield, not graveyard"
        );
    }

    /// [CR#704.5p]: a plain permanent with `attached_to` set to an illegal host
    /// and NO `Sba` → becomes unattached (engine-identical to [CR#704.5n]).
    #[test]
    fn sba_attach_plain_permanent_illegal_link_unattaches() {
        let mut state = game();
        // A plain artifact with no attachment rules at all, illegally linked.
        let thing = on_field(&mut state, "Thing", vec![Type::Artifact], vec![]);
        let host = on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        // Give the host a protection-shaped host-side Cant so the link is
        // illegal even though `thing` itself carries no restriction.
        let protected = on_field(
            &mut state,
            "Protected",
            vec![Type::Creature],
            vec![Ability::Static(StaticAbility {
                condition: None,
                effects: vec![StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Attach {
                        what: Filter::Any,
                        to: Filter::Ref(Reference::This),
                    },
                ))],
                characteristic_defining: false,
            })],
        );
        let _ = host;
        state.objects.obj_mut(thing).attached_to = Some(protected);

        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::Unattached { attachment, former_host }
                if *attachment == thing && *former_host == protected)),
            "plain permanent on an illegal host becomes unattached ([CR#704.5p]); got {actions:?}"
        );
    }

    // --- Ascend (permanent form) e2e ([CR#702.131b,702.131c]) ------------------

    /// [CR#702.131b]: the Ascend static grants the city's blessing once the
    /// controller has ten permanents, exactly once (idempotent / no sweep
    /// loop), and not at nine.
    #[test]
    fn ascend_permanent_grants_citys_blessing_at_ten() {
        use deckmaste_core::Action;
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::RelationFilter;
        use deckmaste_core::StateFilter;

        let mut state = game();
        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = PlayerId(0);

        // The Ascend static, built typed (mirrors the builtin macro's expansion).
        let gate = Condition::AllOf(vec![
            Condition::Compare(
                Count::CountOf(Box::new(Filter::AllOf(vec![
                    Filter::State(StateFilter::InZone(Zone::Battlefield)),
                    Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                        Reference::You,
                    )))),
                ]))),
                Cmp::AtLeast,
                Count::Literal(10),
            ),
            Condition::Not(Box::new(Condition::Is(
                Reference::You,
                Filter::State(StateFilter::Designated(name)),
            ))),
        ]);
        let ascend = Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Sba {
                when: gate,
                then: Box::new(Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::GetDesignation(name),
                ))),
            }],
            characteristic_defining: false,
        });
        let _ascender = on_field(
            &mut state,
            "Ascender",
            vec![Type::Enchantment],
            vec![ascend],
        );

        // Nine permanents (the ascender + 8 fillers) → no grant.
        for i in 0..8 {
            on_field(
                &mut state,
                &format!("Filler{i}"),
                vec![Type::Artifact],
                vec![],
            );
        }
        assert_eq!(state.zones.battlefield.len(), 9);
        assert!(
            sba::sweep(&state)
                .iter()
                .all(|e| !matches!(e, GameEvent::GotDesignation { .. })),
            "no blessing at nine permanents"
        );

        // Tenth permanent → the sweep emits the grant for p0.
        on_field(&mut state, "Filler8", vec![Type::Artifact], vec![]);
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation { player, name: n } if *player == p0 && *n == name)),
            "blessing granted at ten permanents; got {actions:?}"
        );

        // Apply it; the store holds it and a re-sweep emits nothing (no loop).
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(state.designations.players.contains_key(&(p0, name)));
        assert!(
            sba::sweep(&state)
                .iter()
                .all(|e| !matches!(e, GameEvent::GotDesignation { .. })),
            "already-held: the Not(Designated) guard stops re-granting"
        );
    }

    /// [CR#702.131c]: the city's blessing is a per-player designation — more
    /// than one player can hold it at once. Two players, each controlling ten
    /// permanents (each with their own Ascend static), both acquire it in a
    /// single sweep.
    #[test]
    #[allow(clippy::too_many_lines)] // one cohesive two-player e2e scenario
    fn citys_blessing_is_multi_holder() {
        use deckmaste_core::Action;
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::RelationFilter;
        use deckmaste_core::StateFilter;

        let name: deckmaste_core::Ident = "CitysBlessing".into();
        let p0 = PlayerId(0);
        let p1 = PlayerId(1);

        // The Ascend static, built typed (mirrors the builtin macro's
        // expansion). `ControlledBy(Ref(You))` resolves `You` to the carrying
        // object's controller via the Sba frame, so each ascender counts ITS
        // controller's permanents and grants to that controller.
        let ascend = || {
            Ability::Static(StaticAbility {
                condition: None,
                effects: vec![StaticEffect::Sba {
                    when: Condition::AllOf(vec![
                        Condition::Compare(
                            Count::CountOf(Box::new(Filter::AllOf(vec![
                                Filter::State(StateFilter::InZone(Zone::Battlefield)),
                                Filter::Relation(RelationFilter::ControlledBy(Box::new(
                                    Filter::Ref(Reference::You),
                                ))),
                            ]))),
                            Cmp::AtLeast,
                            Count::Literal(10),
                        ),
                        Condition::Not(Box::new(Condition::Is(
                            Reference::You,
                            Filter::State(StateFilter::Designated(name)),
                        ))),
                    ]),
                    then: Box::new(Effect::Act(Action::By(
                        Reference::You,
                        PlayerAction::GetDesignation(name),
                    ))),
                }],
                characteristic_defining: false,
            })
        };

        let mut state = game();

        // p0: ascender + 9 fillers, all controlled by p0 (on_field default).
        on_field(
            &mut state,
            "Ascender0",
            vec![Type::Enchantment],
            vec![ascend()],
        );
        for i in 0..9 {
            on_field(
                &mut state,
                &format!("P0Filler{i}"),
                vec![Type::Artifact],
                vec![],
            );
        }

        // p1: mint a second ascender + 9 fillers, then flip the controller of
        // those ten objects to p1 (on_field mints under p0).
        let mut p1_objs = Vec::new();
        p1_objs.push(on_field(
            &mut state,
            "Ascender1",
            vec![Type::Enchantment],
            vec![ascend()],
        ));
        for i in 0..9 {
            p1_objs.push(on_field(
                &mut state,
                &format!("P1Filler{i}"),
                vec![Type::Artifact],
                vec![],
            ));
        }
        for &id in &p1_objs {
            state.objects.obj_mut(id).controller = p1;
        }

        // Sanity: each player controls exactly ten battlefield permanents.
        let controlled = |state: &GameState, who: PlayerId| {
            state
                .zones
                .battlefield
                .iter()
                .filter(|&&id| state.objects.obj(id).controller == who)
                .count()
        };
        assert_eq!(state.zones.battlefield.len(), 20);
        assert_eq!(controlled(&state, p0), 10, "p0 controls ten permanents");
        assert_eq!(controlled(&state, p1), 10, "p1 controls ten permanents");

        // One sweep grants the blessing to BOTH players.
        let actions = sba::sweep(&state);
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation { player, name: n } if *player == p0 && *n == name)),
            "p0 gets the blessing; got {actions:?}"
        );
        assert!(
            actions.iter().any(|e| matches!(e,
                GameEvent::GotDesignation { player, name: n } if *player == p1 && *n == name)),
            "p1 gets the blessing; got {actions:?}"
        );

        // Apply all; both players end up holding the per-player designation.
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(actions))]);
        let _ = state.step();
        assert!(
            state.designations.players.contains_key(&(p0, name)),
            "p0 holds the city's blessing ([CR#702.131c])"
        );
        assert!(
            state.designations.players.contains_key(&(p1, name)),
            "p1 holds the city's blessing ([CR#702.131c])"
        );
    }
}
