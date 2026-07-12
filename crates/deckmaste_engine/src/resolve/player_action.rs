//! `player_action_items`: lower a [`PlayerAction`] (draw, discard, mana,
//! tokens, randomness, …) to events and work items.

use deckmaste_core::Agency;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::ManaSpec;
use deckmaste_core::PlayerAction;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use super::occurrence_of;
use crate::agenda::WorkItem;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::stack::Frame;
use crate::state::GameState;

/// "Any color" ([CR#106.1b]): the five colors ([CR#105.1]) — a player asked to
/// choose a color may not choose colorless ([CR#105.4]).
const ANY_COLOR: [ColorOrColorless; 5] = [
    ColorOrColorless::Color(Color::White),
    ColorOrColorless::Color(Color::Blue),
    ColorOrColorless::Color(Color::Black),
    ColorOrColorless::Color(Color::Red),
    ColorOrColorless::Color(Color::Green),
];

impl GameState {
    /// The `Emit` work item(s) one `PlayerAction` produces, performed by
    /// `actor` (the agent the enclosing `By` resolved to).
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per player verb; splitting would scatter the dispatch"
    )]
    pub(super) fn player_action_items(
        &self,
        action: &PlayerAction,
        actor: crate::player::PlayerId,
        frame: &Frame,
    ) -> Vec<WorkItem> {
        use crate::event::Occurrence;
        match action {
            PlayerAction::Tap(sel) => {
                // [CR#701.26a]: only an untapped permanent can be tapped — a
                // no-op is no event ([CR#603.2e] "becomes tapped" fires on
                // the transition only).
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .filter(|&object| !self.objects.obj(object).tapped)
                    .map(|object| GameEvent::Tapped {
                        object,
                        cause: Some(Cause::tap(
                            Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )),
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::Draw(qty) => {
                let n = self.eval_count(qty, frame);
                (0..n)
                    .map(|_| {
                        WorkItem::Emit(Occurrence::Single(GameEvent::WillDraw {
                            player: actor,
                            source: Some(frame.source),
                        }))
                    })
                    .collect()
            }
            PlayerAction::LoseLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                    player: actor,
                    amount,
                }))]
            }
            PlayerAction::GainLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                    player: actor,
                    amount,
                }))]
            }
            PlayerAction::Untap(sel) => {
                // [CR#701.26b]: the mirror of `Tap` above — only a tapped
                // permanent can be untapped, a no-op is no event.
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .filter(|&object| self.objects.obj(object).tapped)
                    .map(GameEvent::Untapped)
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::Sacrifice(sel) => {
                // [CR#701.21a]: the actor moves each selected permanent to its
                // owner's graveyard — the `Sacrificed` verb fact evolves into
                // the zone move at apply. That the selection names permanents
                // the actor controls is the grammar's contract; a legality
                // pass is a later seam.
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::ZoneWillChange {
                        object,
                        from: Some(Zone::Battlefield),
                        to: Zone::Graveyard,
                        enters: None,
                        position: None,
                        // [CR#701.21a]: never a destruction — regeneration
                        // can't replace it; the cause says so.
                        face: None,
                        cause: Some(Cause::sacrifice(
                            Agency::EffectInstruction,
                            Some((frame.source, actor)),
                        )),
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::Move(reference, destination, riders) => {
                if !riders.is_empty() {
                    todo!(
                        "core-action-riders-cost-modes seam: enter riders (tapped/attacking/\
                         with-counters) execute with the ETB machinery"
                    );
                }
                // [CR#400.7]: the actor relocates the referenced object to
                // `destination` from whatever zone it's in ([CR#406.2]).
                // Exiling is a pure zone move ([CR#701.13a]) — `Move(This,
                // Exile)`, e.g. a self-exile cost (Scavenge); a tuck rides the
                // same verb via a library anchor. The player-agent twin of
                // `Action::Move`.
                self.move_items(reference, destination, frame)
            }
            // P0.W5 seam: emblem minting into the command zone.
            PlayerAction::GetEmblem(..) => todo!("P0.W5: emblems ([CR#114.1])"),
            // [CR#701.24a]: shuffle the actor's library; the Shuffled
            // apply randomizes via the seeded rng.
            PlayerAction::Shuffle => {
                vec![WorkItem::Emit(Occurrence::single(GameEvent::Shuffled(
                    actor,
                )))]
            }
            // [CR#119.5]: set-to-N resolves as a gain or loss of the
            // difference — triggers see the gain/loss, never a "set";
            // equal totals produce no event (transition-only).
            PlayerAction::SetLife(qty) => {
                let target = deckmaste_core::Int::try_from(self.eval_count(qty, frame))
                    .expect("life total fits in i32");
                let current = self.player(actor).life;
                let event = match target.cmp(&current) {
                    std::cmp::Ordering::Less => GameEvent::LifeLost {
                        player: actor,
                        amount: Uint::try_from(current - target).expect("positive difference"),
                    },
                    std::cmp::Ordering::Greater => GameEvent::LifeGained {
                        player: actor,
                        amount: Uint::try_from(target - current).expect("positive difference"),
                    },
                    std::cmp::Ordering::Equal => return vec![],
                };
                vec![WorkItem::Emit(Occurrence::single(event))]
            }
            // P0.W6 seams: outcome verbs (immediate, gate-checked at the
            // OUTCOME layer — never deontic rows) and reveal/look.
            // [CR#104.2b]: "you win the game" — a first-class win event,
            // suppressed by a matching `CantWin` gate ([CR#101.1]). The
            // last-player-standing win ([CR#104.2a]) never rides this verb.
            PlayerAction::WinGame => {
                let view = self.layers();
                if self.gate_suppresses(&view, actor, deckmaste_core::OutcomeGateKind::CantWin) {
                    return vec![];
                }
                vec![WorkItem::Emit(Occurrence::single(GameEvent::PlayerWon {
                    player: actor,
                }))]
            }
            // [CR#104.3e]: "you lose the game", suppressed by a matching
            // `CantLose` gate ([CR#101.1]).
            PlayerAction::LoseGame => {
                let view = self.layers();
                if self.gate_suppresses(&view, actor, deckmaste_core::OutcomeGateKind::CantLose) {
                    return vec![];
                }
                vec![WorkItem::Emit(Occurrence::single(GameEvent::PlayerLost {
                    player: actor,
                    reason: crate::event::LossReason::Effect,
                }))]
            }
            PlayerAction::RestartGame => {
                todo!("P0.W6: restart ([CR#727.1] — a terminal with carryover, not a reset)")
            }
            PlayerAction::Reveal { .. } => {
                todo!("P0.W6: reveal/look (emit Revealed; window lifetime [CR#701.20a])")
            }
            // [CR#608.2c,608.2d,607.2] choose-and-note: a resolution choice
            // stored under a note key, KIND-GATED. Only note kinds with an
            // existing engine READER get wired (reader-gated); write-only kinds
            // stay LOUD per-kind (naming the kind), so a card reaching an
            // unbuilt kind trips a labeled seam rather than silently no-op'ing.
            // The verb is `&self`, so it only SCHEDULES the surfacing work
            // item; the `&mut self` handler opens the decision.
            PlayerAction::ChooseAndNote(key, kind) => match kind {
                // Number → the `Count::Noted` reader (scalar `resolution_notes`).
                deckmaste_core::NotedKind::Number => vec![WorkItem::ChooseNoteNumber {
                    player: actor,
                    key: *key,
                }],
                // Objects → the fact-backed `noted` product group, read by
                // `Selection::AmongNoted` (and the future `Reference::Linked`).
                deckmaste_core::NotedKind::Objects => vec![WorkItem::ChooseNoteObjects {
                    player: actor,
                    key: *key,
                }],
                // No reader grammar exists yet — no chosen-color/name predicate
                // (an `OfChosen`-equivalent), no `Selection::PilesOf` runtime —
                // so wiring the write would be dead machinery. Stay loud.
                deckmaste_core::NotedKind::Color => todo!(
                    "engine seam: ChooseAndNote(Color) has no reader grammar \
                     (no chosen-color predicate) — write-only, unbuilt ([CR#607.2])"
                ),
                deckmaste_core::NotedKind::CardName => todo!(
                    "engine seam: ChooseAndNote(CardName) has no reader grammar \
                     (no chosen-name predicate) — write-only, unbuilt ([CR#607.2])"
                ),
                deckmaste_core::NotedKind::Piles => todo!(
                    "engine seam: ChooseAndNote(Piles) needs the pile store + \
                     Selection::PilesOf reader — unbuilt ([CR#700.3a])"
                ),
            },
            // [CR#707.10]: put a copy of the referenced stack object onto
            // the stack — the APPLY mints it (needs &mut). A reference that
            // doesn't resolve to a live stack entry fizzles (authoring
            // mistakes never crash).
            PlayerAction::CopySpell(what) => {
                let original = self.eval_reference(what, frame);
                if self.stack.iter().any(|e| e.id == original) {
                    vec![WorkItem::Emit(Occurrence::single(GameEvent::Copied {
                        original,
                        copy: None,
                        controller: actor,
                    }))]
                } else {
                    vec![]
                }
            }
            // [CR#707.10c,115.7d]: `by` re-targets the committed stack
            // object `of` — the state-dependent legal-set derivation (fresh
            // legal candidates unioned with the current target) runs at
            // handler time (`step()`'s `open_choose_new_targets`, which also
            // re-checks `of` is still on the stack — it may leave between
            // this resolving and the work item running). This arm only
            // resolves the two references; an unresolvable `by` (not a
            // player) fizzles — authoring mistakes never crash the engine.
            PlayerAction::ChooseNewTargets { of, by } => {
                let entry = self.eval_reference(of, frame);
                match self.eval_player_ref(by, frame) {
                    Some(player) => vec![WorkItem::ChooseNewTargets { player, entry }],
                    None => vec![],
                }
            }
            // [CR#705.1]: flip `count` coins — the draw happens in the work
            // item (the rng needs `&mut`); the applied batch fixes "that
            // many" to the number of won (called) / heads (uncalled) flips.
            PlayerAction::FlipCoins(count, called) => {
                let count = self.eval_count(count, frame);
                vec![WorkItem::FlipCoins {
                    player: actor,
                    count,
                    called: *called,
                }]
            }
            // core-action-riders-cost-modes: shapes landed, execution seams.
            // [CR#701.17a]: mill — the actor puts that many cards from the
            // top of their library into their graveyard, as ONE simultaneous
            // batch of cause-carried moves ("milled this way" reads find
            // them by the Mill cause, [CR#701.17c]).
            PlayerAction::Mill(count) => {
                let n = self.eval_count(count, frame) as usize;
                // [CR#701.17b]: milling more cards than the library holds
                // mills as many as possible.
                let events: Vec<GameEvent> = self.zones.libraries[actor.index()]
                    .iter()
                    .take(n)
                    .map(|&object| GameEvent::ZoneWillChange {
                        object,
                        from: Some(Zone::Library),
                        to: Zone::Graveyard,
                        enters: None,
                        position: None,
                        face: None,
                        cause: Some(Cause::mill(
                            Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )),
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::VentureIntoDungeon => {
                todo!("engine seam: venture into the dungeon ([CR#701.49a]) — dungeons unbuilt")
            }
            // [CR#706.1]: roll `count` `sides`-sided dice — draw in the work
            // item; the applied batch fixes "that many" to the summed
            // results ([CR#706.2] — `result = natural` until the modifier
            // pipeline lands, engine-replace-roll).
            PlayerAction::RollDice(count, sides) => {
                let count = self.eval_count(count, frame);
                vec![WorkItem::RollDice {
                    player: actor,
                    count,
                    sides: *sides,
                }]
            }
            // [CR#901.9]: the Planechase planar die is a special action
            // whose whole surrounding subsystem (Plane cards, the chaos/
            // planeswalking abilities it triggers) this engine doesn't
            // model at all — a documented absent-subsystem no-op, never a
            // panic (no real card in this corpus needs it; no Plane-card
            // support exists to wire it TO even if a card did).
            PlayerAction::RollPlanarDie => vec![],
            // [CR#122.1]: place/remove `n` counters of `kind` on each selected
            // object or player proxy. `n == 0` (or an empty selection) is a
            // no-op, so no event fires — a "counter is put on" trigger never
            // sees a zero placement. The cause carries the effect-instruction
            // agent so "you put a counter" reads ([CR#603.2e]-style transition
            // views) resolve to the right controller.
            PlayerAction::PutCounters(sel, kind, count) => {
                let n = self.eval_count(count, frame);
                if n == 0 {
                    return vec![];
                }
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::CounterPlaced {
                        object,
                        // The event carries the resolved Ident name (engine
                        // state is Ident-keyed); the authored ref is a `CounterRef`.
                        kind: kind.0,
                        amount: n,
                        // Apply-computed totals ([CR#714.2b]).
                        before: 0,
                        after: 0,
                        cause: Some(crate::event::Cause::put_counters(
                            deckmaste_core::Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )),
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::RemoveCounters(sel, kind, count) => {
                let n = self.eval_count(count, frame);
                if n == 0 {
                    return vec![];
                }
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::CounterRemoved {
                        object,
                        kind: kind.0,
                        amount: n,
                        cause: Some(crate::event::Cause::remove_counters(
                            deckmaste_core::Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )),
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::AddMana(qty, production) => {
                let amount = self.eval_count(qty, frame);
                let (spec, mut riders) = match production {
                    deckmaste_core::ManaProduction::Bare(spec) => (spec, Vec::new()),
                    deckmaste_core::ManaProduction::WithRiders { mana, riders } => {
                        (mana, riders.clone())
                    }
                };
                // [CR#107.4h]: mana from a snow source carries `Snow`
                // provenance regardless of the riders the ability text
                // declares — snow-ness is a property of the producing source,
                // not the effect.
                riders.extend(self.snow_provenance(frame.source));
                match spec {
                    // A fixed production needs no choice.
                    ManaSpec::Specific(mana) => {
                        vec![WorkItem::Emit(Occurrence::Single(GameEvent::ManaAdded {
                            player: actor,
                            mana: *mana,
                            amount,
                            riders,
                        }))]
                    }
                    // [CR#106.1b]: the actor chooses on resolution — surfaced
                    // explicitly even when only one option exists (engine
                    // policy: every choice surfaces).
                    ManaSpec::AnyColor => vec![WorkItem::ChooseManaColor {
                        player: actor,
                        options: ANY_COLOR.to_vec(),
                        amount,
                        riders,
                    }],
                    ManaSpec::OneOf(options) => vec![WorkItem::ChooseManaColor {
                        player: actor,
                        options: options.clone(),
                        amount,
                        riders,
                    }],
                    // [CR#106.1b]: the filterland cycle — the actor picks one
                    // of several multi-symbol runs on resolution, then that
                    // run's whole sequence of mana is produced.
                    ManaSpec::OneOfRuns(options) => vec![WorkItem::ChooseManaMode {
                        player: actor,
                        options: options.clone(),
                        amount,
                        riders,
                    }],
                    // [CR#105.2]: Chrome Mox's imprint — the producer picks
                    // AMONG the referenced object's own colors, resolved
                    // fresh at activation (a live derived-characteristics
                    // read via `self.layers()`, not a remembered snapshot).
                    // WIRED: reusing the SAME `ChooseManaColor` choice
                    // machinery `AnyColor`/`OneOf` already surface, once the
                    // referenced object's colors are known.
                    ManaSpec::AmongColorsOf(r) => {
                        let id = self.eval_reference(r, frame);
                        // `LayeredView::get` panics on an id absent from
                        // `self.objects` — guard with the same non-panicking
                        // lookup the devotion `ManaSymbols`/`Singleton` arms
                        // use before ever touching `self.layers()`, so a
                        // fully-ceased referent (a token that left the game,
                        // an LKI-only snapshot id) fizzles instead of
                        // crashing.
                        let options: Vec<ColorOrColorless> = match self.objects.get(id) {
                            Some(_) => {
                                let view = self.layers();
                                view.get(id)
                                    .colors
                                    .iter()
                                    .map(|&c| ColorOrColorless::Color(c))
                                    .collect()
                            }
                            None => vec![],
                        };
                        // A colorless (or unresolvable/gone) referenced
                        // object has no colors to choose among — never a
                        // panic, never a fabricated fallback color: no
                        // production at all ([CR#105.2] presupposes ≥1
                        // color; an authoring/zone-changed mismatch fizzles
                        // like any other stale reference).
                        if options.is_empty() {
                            vec![]
                        } else {
                            vec![WorkItem::ChooseManaColor {
                                player: actor,
                                options,
                                amount,
                                riders,
                            }]
                        }
                    }
                    // [CR#106.12a]: sound only inside a `TapForMana`-
                    // triggered body ("of any type that land produced",
                    // Dictate of Karametra/Vorinclex) — the SAME
                    // absent-subsystem gap `EventFilter::TapForMana`
                    // documents in `eval.rs` (no engine fact records which
                    // permanent produced what). A documented no-op: adding
                    // zero mana is never-crash-safe and doesn't fabricate a
                    // plausible-but-wrong color the way falling back to
                    // colorless would.
                    ManaSpec::ProducedByEvent => vec![],
                }
            }
            PlayerAction::Discard {
                count,
                what,
                random,
            } => {
                if *random {
                    // [CR#701.9b]: random discard — no decision (no choice
                    // exists); the work item samples from the seeded rng
                    // when it applies (the hand may change before then).
                    let count = self.eval_count(count, frame);
                    return vec![WorkItem::DiscardRandom {
                        player: actor,
                        count,
                    }];
                }
                // [CR#701.9b]: the actor chooses which cards — surfaced as a
                // decision when the work item applies (the hand may change
                // before then). A named `what` (discard a *specific* card) as a
                // resolution EFFECT is unbuilt; cycling's "discard this card"
                // ([CR#702.29a]) is a COST, paid in `activate.rs`, never here.
                if let Some(sel) = what {
                    todo!("discard a named selection as a resolution effect: {sel:?}");
                }
                let count = self.eval_count(count, frame);
                vec![WorkItem::DiscardCards {
                    player: actor,
                    count,
                }]
            }
            PlayerAction::Create(qty, spec, riders) => {
                if !riders.is_empty() {
                    todo!(
                        "core-action-riders-cost-modes seam: token enter riders \
                         (tapped/attacking) execute with the ETB machinery"
                    );
                }
                // [CR#701.7a]: one instruction puts all N tokens onto the
                // battlefield — one simultaneous batch of `TokenCreated`
                // facts. (Token copies — `Create` of a copy-defined token —
                // wait on the copy grammar, `core-copy-grammar`.) The FACT
                // carries the resolved inline definition; a `TokenSpec::Named`
                // predefined token ([CR#111.10]) resolves to its rules-defined
                // characteristics here.
                let token = match spec {
                    deckmaste_core::TokenSpec::Token(token) => token.clone(),
                    deckmaste_core::TokenSpec::Named(name) => name
                        .resolve()
                        .expect("a Named token in a card resolves to a builtin definition"),
                };
                let n = self.eval_count(qty, frame);
                let events: Vec<GameEvent> = (0..n)
                    .map(|_| GameEvent::TokenCreated {
                        player: actor,
                        token: token.clone(),
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            PlayerAction::GetDesignation(name) => {
                // [CR#702.131c]: idempotent — a player who already holds the
                // designation gets no second grant and no fact (so the SBA
                // sweep converges and no spurious "got it" event is recorded).
                if self.designations.players.contains_key(&(actor, *name)) {
                    vec![]
                } else {
                    vec![WorkItem::Emit(Occurrence::Single(
                        GameEvent::GotDesignation {
                            player: actor,
                            name: *name,
                        },
                    ))]
                }
            }
            // [CR#614.8,701.19a]: remove all marked damage from each selected
            // object and remove it from combat if it's attacking or blocking.
            // This is the regeneration "heal" clause — its apply zeroes damage
            // and calls `combat.remove_object`.
            PlayerAction::RemoveDamage(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::DamageRemoved { object })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            // Look through a remembered macro invocation.
            PlayerAction::Expanded(e) => self.player_action_items(&e.value, actor, frame),
        }
    }
}
