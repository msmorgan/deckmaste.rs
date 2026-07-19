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
use crate::event::Copied;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DamageRemoved;
use crate::event::EmblemCreated;
use crate::event::GameEvent;
use crate::event::GotDesignation;
use crate::event::LifeGained;
use crate::event::LifeLost;
use crate::event::ManaAdded;
use crate::event::PlayerLost;
use crate::event::PlayerWon;
use crate::event::Revealed;
use crate::event::Tapped;
use crate::event::TokenCreated;
use crate::event::ZoneChange;
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
                    .map(|object| {
                        GameEvent::Tapped(Tapped {
                            object,
                            cause: Some(Cause::tap(
                                Agency::EffectInstruction,
                                Some((frame.source, frame.controller)),
                            )),
                        })
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            PlayerAction::LoseLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost(
                    LifeLost {
                        player: actor,
                        amount,
                    },
                )))]
            }
            PlayerAction::GainLife(qty) => {
                let amount = self.eval_count(qty, frame);
                vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained(
                    LifeGained {
                        player: actor,
                        amount,
                    },
                )))]
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
                    .map(|object| {
                        GameEvent::ZoneChange(ZoneChange {
                            snapshot: None,
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
                self.move_items(reference, destination, None, frame)
            }
            // [CR#114.1]: the actor gets an emblem carrying `abilities`. The
            // synthesis + command-zone mint happens at apply (the `&mut self`
            // stage); here we only emit the fact. Getting an emblem is not a
            // zone-change trigger ([CR#114.5] — an emblem is never a
            // permanent; it never enters the battlefield).
            PlayerAction::GetEmblem(abilities) => {
                vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::EmblemCreated(EmblemCreated {
                        player: actor,
                        abilities: abilities.clone(),
                    }),
                ))]
            }
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
                    std::cmp::Ordering::Less => GameEvent::LifeLost(LifeLost {
                        player: actor,
                        amount: Uint::try_from(current - target).expect("positive difference"),
                    }),
                    std::cmp::Ordering::Greater => GameEvent::LifeGained(LifeGained {
                        player: actor,
                        amount: Uint::try_from(target - current).expect("positive difference"),
                    }),
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
                vec![WorkItem::Emit(Occurrence::single(GameEvent::PlayerWon(
                    PlayerWon { player: actor },
                )))]
            }
            // [CR#104.3e]: "you lose the game", suppressed by a matching
            // `CantLose` gate ([CR#101.1]).
            PlayerAction::LoseGame => {
                let view = self.layers();
                if self.gate_suppresses(&view, actor, deckmaste_core::OutcomeGateKind::CantLose) {
                    return vec![];
                }
                vec![WorkItem::Emit(Occurrence::single(GameEvent::PlayerLost(
                    PlayerLost {
                        player: actor,
                        reason: crate::event::LossReason::Effect,
                    },
                )))]
            }
            PlayerAction::RestartGame => {
                todo!("P0.W6: restart ([CR#727.1] — a terminal with carryover, not a reset)")
            }
            PlayerAction::Reveal { what, to } => {
                let object = self.eval_reference(what, frame);
                if self.objects.get(object).is_none() {
                    vec![]
                } else {
                    // Fail CLOSED: an authored subset-look (`to: Some(..)`)
                    // whose player doesn't resolve fizzles the reveal — it
                    // must NOT widen into the `to: None` "revealed to all
                    // players" form (bad authoring fizzles; a private look
                    // never silently goes public).
                    let to = match to {
                        None => None,
                        Some(who) => match self.eval_player_ref(who, frame) {
                            Some(player) => Some(vec![player]),
                            None => return vec![],
                        },
                    };
                    vec![WorkItem::Emit(Occurrence::single(GameEvent::Revealed(
                        Revealed {
                            objects: vec![object],
                            to,
                        },
                    )))]
                }
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
                // No reader grammar exists yet for chosen colors or piles, so
                // those writes remain loud. CardName is read by `Named(key)`.
                deckmaste_core::NotedKind::Color => todo!(
                    "engine seam: ChooseAndNote(Color) has no reader grammar \
                     (no chosen-color predicate) — write-only, unbuilt ([CR#607.2])"
                ),
                deckmaste_core::NotedKind::CardName => vec![WorkItem::ChooseNoteCardName {
                    player: actor,
                    key: *key,
                }],
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
                    vec![WorkItem::Emit(Occurrence::single(GameEvent::Copied(
                        Copied {
                            original,
                            copy: None,
                            controller: actor,
                        },
                    )))]
                } else {
                    vec![]
                }
            }
            // [CR#608.2g]: cast the referenced card DURING resolution — the
            // actor follows the [CR#601.2a..601.2i] steps (reusing the shared
            // announce chain), except no player receives priority after it's
            // cast; the cast spell becomes the topmost stack object and the
            // currently-resolving ability continues. The May "yes" branch that
            // reaches this arm was already gated on `can_cast_as_effect` (the
            // effect grants the permission, [CR#608.2g]), so a live castable
            // referent is expected; a reference that no longer resolves to a
            // castable object fizzles (authoring mistakes never crash).
            PlayerAction::Cast(what, for_cost) => {
                let object = self.eval_reference(what, frame);
                // `can_cast_as_effect` guards a null/stale/wrong-zone referent
                // (never-crash) and returns false, so a bad reference fizzles.
                // [CR#118.9,702.35a]: the alternative base cost (madness) rides
                // through to the announce so `PayCost` demands it, not the
                // printed mana cost.
                if self.can_cast_as_effect(actor, object, for_cost.as_ref()) {
                    self.cast_as_effect_items(object, actor, for_cost.clone())
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
                    .map(|object| {
                        GameEvent::CounterPlaced(CounterPlaced {
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
                    .map(|object| {
                        GameEvent::CounterRemoved(CounterRemoved {
                            object,
                            kind: kind.0,
                            amount: n,
                            cause: Some(crate::event::Cause::remove_counters(
                                deckmaste_core::Agency::EffectInstruction,
                                Some((frame.source, frame.controller)),
                            )),
                        })
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
                        vec![WorkItem::Emit(Occurrence::Single(GameEvent::ManaAdded(
                            ManaAdded {
                                player: actor,
                                mana: *mana,
                                amount,
                                riders,
                            },
                        )))]
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
                    // [CR#707.1] a token that's a copy of an object. The
                    // not-created guards fizzle to no token, never a panic:
                    // [CR#111.12] the source no longer exists (or never did),
                    // [CR#111.5] the resolved copiable values are an
                    // instant/sorcery (or another forbid `token_from_copiable`
                    // catches). `AdditionalEffect` exceptions
                    // (`crate::copy::additional_riders`) have no in-scope
                    // consumer for a token copy — no card in this campaign's
                    // corpus needs a copy-and-additional-effect token, and the
                    // `riders` slot they'd feed is the same
                    // `core-action-riders-cost-modes` `todo!()` seam above —
                    // so they're deliberately dropped here rather than routed
                    // through that unbuilt seam.
                    deckmaste_core::TokenSpec::Copy(spec) => {
                        let Some(src) = crate::copy::resolve_source(self, frame, &spec.source)
                        else {
                            return vec![];
                        };
                        let Some(cv) = crate::copy::copiable_values(self, src) else {
                            return vec![];
                        };
                        let cv = crate::copy::apply_exceptions(cv, &spec.exceptions);
                        match crate::copy::token_from_copiable(cv) {
                            Some(token) => token,
                            None => return vec![],
                        }
                    }
                };
                let n = self.eval_count(qty, frame);
                let events: Vec<GameEvent> = (0..n)
                    .map(|_| {
                        GameEvent::TokenCreated(TokenCreated {
                            player: actor,
                            token: token.clone(),
                        })
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
                        GameEvent::GotDesignation(GotDesignation {
                            player: actor,
                            name: *name,
                        }),
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
                    .map(|object| GameEvent::DamageRemoved(DamageRemoved { object }))
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

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use deckmaste_core::Action;
    use deckmaste_core::Card;
    use deckmaste_core::Count;
    use deckmaste_core::ObjectKind;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Type;
    use deckmaste_core::Uint;
    use deckmaste_core::Zone;

    use crate::PendingDecision;
    use crate::agenda::WorkItem;
    use crate::event::CoinFlipped;
    use crate::event::DieRolled;
    use crate::event::GameEvent;
    use crate::event::Occurrence;
    use crate::event::PlayerLost;
    use crate::event::PlayerWon;
    use crate::event::TokenCreated;
    use crate::event::ZoneChange;
    use crate::matches as obj_matches;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::state::GameState;
    use crate::step::Progress;
    use crate::step::StepOutcome;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;

    /// `AddMana(2, Green)` needs no choice and lands in the pool ([CR#106.4]);
    /// `AddMana(1, AnyColor)` surfaces `ChooseManaColor` with the five colors
    /// — colorless is not a color ([CR#105.4]) and is rejected.
    #[test]
    fn add_mana_specific_and_any_color() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaSpec;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let green = ColorOrColorless::Color(Color::Green);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(2),
                ManaSpec::Specific(green).into(),
            )),
            &frame,
        );
        let _ = state.step();
        assert_eq!(state.players[0].mana_pool.amount(green), 2);

        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::AnyColor.into(),
            )),
            &frame,
        );
        let _ = state.step(); // ManaColorOpened
        let StepOutcome::NeedsDecision(PendingDecision::ChooseManaColor(
            crate::decide::pending::ChooseManaColor {
                player,
                options,
                amount,
                ..
            },
        )) = state.step()
        else {
            panic!("expected ChooseManaColor, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!(options.len(), 5, "the five colors");
        assert_eq!(amount, 1);
        assert!(
            state
                .submit_decision(Decision::ManaColor(ColorOrColorless::Colorless))
                .is_err(),
            "colorless is not a color"
        );
        let blue = ColorOrColorless::Color(Color::Blue);
        state.submit_decision(Decision::ManaColor(blue)).unwrap();
        let _ = state.step(); // ManaAdded applies
        assert_eq!(state.players[0].mana_pool.amount(blue), 1);
    }

    /// [CR#106.1b]: the filterland production `AddMana(1, OneOfRuns([[W,W],
    /// [W,U], [U,U]]))` surfaces `ChooseManaMode`; an out-of-range index is
    /// rejected; picking the heterogeneous run lands both its mana at once.
    #[test]
    fn add_mana_one_of_runs_surfaces_mode_and_lands_the_run() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaSpec;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let white = ColorOrColorless::Color(Color::White);
        let blue = ColorOrColorless::Color(Color::Blue);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaSpec::OneOfRuns(vec![
                    vec![white, white],
                    vec![white, blue],
                    vec![blue, blue],
                ])
                .into(),
            )),
            &frame,
        );
        let _ = state.step(); // ManaModeOpened
        let StepOutcome::NeedsDecision(PendingDecision::ChooseManaMode(
            crate::decide::pending::ChooseManaMode {
                player, options, ..
            },
        )) = state.step()
        else {
            panic!("expected ChooseManaMode, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!(options.len(), 3, "the three runs");
        assert!(
            state.submit_decision(Decision::ManaMode(3)).is_err(),
            "index past the offered runs is illegal"
        );
        // Pick the {W}{U} run (index 1): one white and one blue land together.
        state.submit_decision(Decision::ManaMode(1)).unwrap();
        let _ = state.step(); // ManaAdded batch applies
        assert_eq!(state.players[0].mana_pool.amount(white), 1);
        assert_eq!(state.players[0].mana_pool.amount(blue), 1);
    }

    /// `AddMana(1, WithRiders{ mana: Red, riders: [SpendOnly(Any)] })` lands
    /// one red unit in the pool whose riders vec is non-empty ([CR#106.6]).
    #[test]
    fn add_mana_with_riders_lands_unit_carrying_riders() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaProduction;
        use deckmaste_core::ManaRider;
        use deckmaste_core::ManaSpec;
        use deckmaste_core::Predicate;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let red = ColorOrColorless::Color(Color::Red);
        let rider = ManaRider::SpendOnly(Predicate::Any);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::AddMana(
                Count::Literal(1),
                ManaProduction::WithRiders {
                    mana: ManaSpec::Specific(red),
                    riders: vec![rider],
                },
            )),
            &frame,
        );
        let _ = state.step(); // ManaAdded applies
        assert_eq!(state.players[0].mana_pool.amount(red), 1);
        let units_with_riders = state.players[0]
            .mana_pool
            .units()
            .iter()
            .filter(|u| !u.riders.is_empty())
            .count();
        assert_eq!(units_with_riders, 1, "one unit should carry riders");
    }

    /// [CR#105.2]: `AmongColorsOf` reads the referenced object's colors off
    /// the live layers view — `LayeredView::get` panics on an id absent from
    /// `state.objects`. A referent that's fully CEASED (a token that left the
    /// game, an LKI-only snapshot id with no live twin) must fizzle to no
    /// colors — no production at all — never crash. `source` (the Chrome-Mox
    /// stand-in mana-producing permanent) stays alive throughout — only the
    /// REFERENCED object (`imprinted`, reached via the lone-target `It`
    /// antecedent) goes away, mirroring how an imprinted/exiled card can
    /// cease independently of the producing permanent.
    #[test]
    fn among_colors_of_gone_referent_fizzles_empty() {
        use deckmaste_core::ManaSpec;

        let (mut state, source, imprinted) = two_permanents_on_field();
        let p0 = PlayerId(0);
        let frame = frame_src_targets(source, vec![imprinted]);
        state.objects.remove(imprinted);
        assert!(
            state.objects.get(imprinted).is_none(),
            "the referent is gone"
        );
        assert!(
            state.objects.get(source).is_some(),
            "the mana source is still live"
        );

        let pa = PlayerAction::AddMana(
            Count::Literal(1),
            ManaSpec::AmongColorsOf(Reference::It).into(),
        );
        // Must not panic dereferencing the gone id via `self.layers().get(..)`.
        let items = state.player_action_items(&pa, p0, &frame);
        assert!(
            items.is_empty(),
            "a gone AmongColorsOf referent has no colors to choose among, so no production"
        );
    }

    /// [CR#701.9b]: `Discard(2)` surfaces the card choice; a wrong-sized answer
    /// is rejected; the right answer discards through the Hand→Graveyard
    /// pipeline. Discarding more than the hand holds clamps to the whole hand
    /// ([CR#101.3]).
    #[test]
    fn discard_surfaces_choice_validates_and_clamps() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let hand_before = state.zones.hands[0].len();
        state.run_effect(
            OneShotEffect::Act(Action::discard(Reference::You, Count::Literal(2), false)),
            &frame,
        );
        // The carry future `Act` opens (and passes) its own [CR#616.1] window
        // before the `With`+`Choose` machinery surfaces the actual choice.
        let pending = (0..5)
            .find_map(|_| match state.step() {
                StepOutcome::NeedsDecision(p) => Some(p),
                _ => None,
            })
            .expect("a ChooseObjects decision surfaces within a few steps");
        let PendingDecision::ChooseObjects(crate::decide::pending::ChooseObjects {
            player,
            min,
            max,
            ..
        }) = pending
        else {
            panic!("expected ChooseObjects, got {pending:?}");
        };
        assert_eq!((player, min, max), (PlayerId(0), 2, 2));
        let one = vec![state.zones.hands[0][0]];
        assert!(
            state.submit_decision(Decision::Chosen(one)).is_err(),
            "exactly `count` cards must be chosen"
        );
        let two = state.zones.hands[0][..2].to_vec();
        state.submit_decision(Decision::Chosen(two)).unwrap();
        for _ in 0..30 {
            if state.zones.graveyards[0].len() == 2 {
                break;
            }
            let _ = state.step();
        }
        assert_eq!(state.zones.hands[0].len(), hand_before - 2);
        assert_eq!(state.zones.graveyards[0].len(), 2);

        // Clamp: an instruction to discard far more than the hand holds
        // discards the whole hand.
        state.run_effect(
            OneShotEffect::Act(Action::discard(Reference::You, Count::Literal(99), false)),
            &frame,
        );
        let pending = (0..5)
            .find_map(|_| match state.step() {
                StepOutcome::NeedsDecision(p) => Some(p),
                _ => None,
            })
            .expect("a ChooseObjects decision surfaces within a few steps");
        let PendingDecision::ChooseObjects(crate::decide::pending::ChooseObjects { max, .. }) =
            pending
        else {
            panic!("expected ChooseObjects, got {pending:?}");
        };
        assert_eq!(max as usize, hand_before - 2, "clamped to the hand size");
        let rest = state.zones.hands[0].clone();
        state.submit_decision(Decision::Chosen(rest)).unwrap();
        for _ in 0..30 {
            if state.zones.hands[0].is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert!(state.zones.hands[0].is_empty());
        assert_eq!(state.zones.graveyards[0].len(), hand_before);
    }

    /// A remembered `PlayerAction` macro invocation resolves through its
    /// expanded body.
    #[test]
    fn expanded_player_action_resolves_through_body() {
        use deckmaste_core::Expansion;
        use deckmaste_core::ExpansionArgs;

        let (state, src) = bear_on_field();
        let frame = frame_src(src);
        let body = PlayerAction::GainLife(Count::Literal(2));
        let expanded = PlayerAction::Expanded(Expansion {
            name: "GainTwo".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(body.clone()),
        });
        assert_eq!(
            state.action_items(&Action::by_you(expanded), &frame),
            state.action_items(&Action::by_you(body), &frame),
        );
    }

    /// [CR#701.7a,111.2]: `Create(2, token)` puts two token permanents onto
    /// the battlefield under the creator — owned by them, summoning-sick,
    /// kind `Token` (not `Card`, [CR#111.6]) — as ONE simultaneous batch of
    /// `TokenCreated` facts, each followed by its past-form `ZoneChange
    /// { from: None, to: Battlefield, .. }` fact.
    #[test]
    fn create_tokens_enter_battlefield() {
        use deckmaste_core::Token;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let token = Token {
            name: None,
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![Type::Artifact.def()],
            subtypes: vec![],
            abilities: vec![],
            power: None,
            toughness: None,
        };
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(2),
                token.into(),
                vec![],
            )),
            &frame,
        );
        // One simultaneous batch of two TokenCreated facts.
        let made = match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Batch(events))) => events,
            other => panic!("expected Applied(Batch), got {other:?}"),
        };
        assert_eq!(made.len(), 2);
        assert!(made.iter().all(|e| matches!(
            e,
            GameEvent::TokenCreated(TokenCreated {
                player: PlayerId(0),
                ..
            })
        )));

        let tokens: Vec<ObjectId> = state
            .zones
            .battlefield
            .iter()
            .copied()
            .filter(|&id| id != src)
            .collect();
        assert_eq!(tokens.len(), 2, "two tokens on the battlefield");
        for &t in &tokens {
            assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
            assert_eq!(
                state.owner_of(t),
                PlayerId(0),
                "[CR#111.2]: creator owns it"
            );
            assert_eq!(state.objects.obj(t).controller, PlayerId(0));
            assert!(state.objects.obj(t).summoning_sick, "[CR#302.6]");
            assert!(
                obj_matches(&state, t, &Predicate::type_(Type::Artifact)),
                "the creating effect's characteristics stick ([CR#111.3])"
            );
        }
        // The enter facts follow as ONE batch occurrence ([CR#603.3b] — the
        // two tokens were minted by one simultaneous instruction, so their
        // enter-triggers see one occurrence): from: None (created, not moved).
        match state.step() {
            StepOutcome::Progress(Progress::Applied(Occurrence::Batch(facts))) => {
                assert_eq!(facts.len(), 2, "both entry facts in one batch");
                assert!(facts.iter().all(|e| matches!(
                    e,
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: Some(_),
                        from: None,
                        to: Zone::Battlefield,
                        ..
                    })
                )));
            }
            other => panic!("expected the tokens' past-form ZoneChange batch, got {other:?}"),
        }
    }

    /// A `Create(_, Named(Treasure))` resolves the predefined token
    /// ([CR#111.10a]) from its rules-defined characteristics — no plugin handle
    /// needed — and puts a real Treasure-subtyped token onto the battlefield.
    #[test]
    fn create_named_treasure_token() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Named(deckmaste_core::TokenName::from("Treasure")),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the Treasure token on the battlefield");
        assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
        let card = state.objects.obj(t).card_id().expect("card-backed");
        assert!(
            state
                .cards
                .get(card)
                .front
                .subtypes
                .iter()
                .any(|s| s.name == "Treasure"),
            "[CR#111.10a]: the resolved Named token carries the Treasure subtype"
        );
        assert!(state.cards.get(card).is_token, "[CR#111.6]");
    }

    /// The builtin predefined Treasure token ([CR#111.10a]) creates with its
    /// declared subtype and the [CR#111.4] default name (subtypes + "Token").
    #[test]
    fn create_builtin_treasure_token() {
        let (mut state, src) = bear_on_field();
        let frame = frame_src(src);
        let treasure = builtin().token("Treasure").unwrap();
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                treasure.into(),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the Treasure token on the battlefield");
        assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
        let card = state.objects.obj(t).card_id().expect("card-backed");
        // Subtype asserted on the card entry directly — `Predicate::Subtype`
        // evaluation is the `engine-filter-breadth` item.
        assert!(
            state
                .cards
                .get(card)
                .front
                .subtypes
                .iter()
                .any(|s| s.name == "Treasure"),
            "declared subtype sticks"
        );
        assert!(state.cards.get(card).is_token, "[CR#111.6]");
        assert_eq!(
            crate::derive::face(&state.cards.get(card).def).name,
            "Treasure Token",
            "[CR#111.4]: unnamed token defaults to subtypes + \"Token\""
        );
    }

    // ====================================================================
    // core-copy-grammar Task 3 — `TokenSpec::Copy` execution ([CR#707.1])
    // ====================================================================

    /// [CR#707.2]: `Create(1, Copy(CopySpec{Object(target), []}))` mints
    /// exactly one token whose name, power/toughness, types, and subtypes
    /// match the copied source — a vanilla 2/2 Grizzly Bears on the
    /// battlefield. Name is carried EXACTLY ("Grizzly Bears"), not
    /// resynthesized from subtypes ("Bear Token") the way an unnamed token
    /// would be — the Spitting Image example [CR#707.2].
    #[test]
    fn token_copy_mints_a_token_matching_the_source() {
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::StatValue;

        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Copy(CopySpec {
                    source: CopySource::Object(Reference::Target(0)),
                    exceptions: vec![],
                }),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies

        let created: Vec<ObjectId> = state
            .zones
            .battlefield
            .iter()
            .copied()
            .filter(|&id| id != a && id != b)
            .collect();
        assert_eq!(
            created.len(),
            1,
            "[CR#707.1]: exactly one copy token is minted"
        );
        let t = created[0];
        assert_eq!(crate::target::object_kind(&state, t), ObjectKind::Token);
        let card = state.objects.obj(t).card_id().expect("card-backed");
        let face = crate::derive::face(&state.cards.get(card).def);
        assert_eq!(
            face.name, "Grizzly Bears",
            "[CR#707.2]: name matches the copied source EXACTLY (Spitting \
             Image example) — not resynthesized to \"Bear Token\""
        );
        assert_eq!(
            face.power,
            Some(StatValue::Number(2)),
            "[CR#707.2]: power matches the copied source"
        );
        assert_eq!(
            face.toughness,
            Some(StatValue::Number(2)),
            "[CR#707.2]: toughness matches the copied source"
        );
        assert!(
            obj_matches(&state, t, &Predicate::type_(Type::Creature)),
            "[CR#707.2]: types match the copied source"
        );
        assert!(
            face.subtypes.iter().any(|s| s.name == "Bear"),
            "[CR#707.2]: subtypes match the copied source (Grizzly Bears -> Bear)"
        );
    }

    /// [CR#111.12]: a copy of a nonexistent object (its announced target has
    /// since left / never existed) creates no token — zero `TokenCreated`
    /// facts, not a panic.
    #[test]
    fn token_copy_nonexistent_source_creates_nothing() {
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;

        let (state, a) = bear_on_field();
        let dead = ObjectId::from_raw(999);
        let frame = frame_src_targets(a, vec![dead]);
        assert_eq!(
            state.action_items(
                &Action::by_you(PlayerAction::Create(
                    Count::Literal(1),
                    deckmaste_core::TokenSpec::Copy(CopySpec {
                        source: CopySource::Object(Reference::Target(0)),
                        exceptions: vec![],
                    }),
                    vec![],
                )),
                &frame,
            ),
            vec![],
            "[CR#111.12]: a copy of a nonexistent object creates no token"
        );
    }

    /// [CR#111.5]: "if an effect would create a token that is a copy of an
    /// instant or sorcery card, no token is created" — zero tokens.
    #[test]
    fn token_copy_of_instant_or_sorcery_creates_nothing() {
        use deckmaste_core::CardFace;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;

        let (mut state, a) = bear_on_field();
        let bolt = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Lightning Bolt".into(),
                types: vec![Type::Instant.def()],
                ..CardFace::default()
            }),
        );
        let frame = frame_src_targets(a, vec![bolt]);
        assert_eq!(
            state.action_items(
                &Action::by_you(PlayerAction::Create(
                    Count::Literal(1),
                    deckmaste_core::TokenSpec::Copy(CopySpec {
                        source: CopySource::Object(Reference::Target(0)),
                        exceptions: vec![],
                    }),
                    vec![],
                )),
                &frame,
            ),
            vec![],
            "[CR#111.5]: a copy of an instant/sorcery card creates no token"
        );
    }

    /// [CR#707.9d] (Eternalize shape): `Modify(Power Set 4) +
    /// Modify(Toughness Set 4)` overrides the copied 2/2 source's P/T — the
    /// token is 4/4.
    #[test]
    fn token_copy_modify_power_toughness_overrides_pt() {
        use deckmaste_core::CopyException;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::StatValue;

        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Copy(CopySpec {
                    source: CopySource::Object(Reference::Target(0)),
                    exceptions: vec![
                        CopyException::Modify(Modification::Power(NumericOp::Set(Count::Literal(
                            4,
                        )))),
                        CopyException::Modify(Modification::Toughness(NumericOp::Set(
                            Count::Literal(4),
                        ))),
                    ],
                }),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies

        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != a && id != b)
            .expect("the copy token on the battlefield");
        let card = state.objects.obj(t).card_id().expect("card-backed");
        let face = crate::derive::face(&state.cards.get(card).def);
        assert_eq!(
            face.power,
            Some(StatValue::Number(4)),
            "[CR#707.9d]: Modify(Power Set 4) overrides the copied power"
        );
        assert_eq!(
            face.toughness,
            Some(StatValue::Number(4)),
            "[CR#707.9d]: Modify(Toughness Set 4) overrides the copied toughness"
        );
    }

    /// [CR#707.5]: "any enters-the-battlefield triggered abilities of the
    /// copy will have a chance to trigger" (the Wall of Omens example) — a
    /// token copy of a "when this enters, draw a card" creature fires that
    /// draw, proving the token enters via the SAME battlefield path any
    /// permanent does, not a bypass. The source is minted directly onto the
    /// battlefield (`mint_on_field`, no `ZoneChange` event), so its OWN ETB
    /// never fires — only the fresh token's entry can account for the draw
    /// asserted below.
    #[test]
    fn token_copy_etb_trigger_fires_through_the_normal_battlefield_path() {
        use deckmaste_core::Ability;
        use deckmaste_core::CardFace;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::EventFilter;
        use deckmaste_core::StatValue;
        use deckmaste_core::TriggeredAbility;

        use crate::decide::Action as Act;
        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, actor) = bear_on_field();
        let source_face = CardFace {
            name: "Wall of Omens".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(0)),
            toughness: Some(StatValue::Number(4)),
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                from: None,
                event: EventFilter::ZoneChange {
                    what: Predicate::Ref(Reference::This),
                    from: None,
                    to: Some(Zone::Battlefield),
                    cause: None,
                },
                condition: None,
                limits: Vec::new(),
                effect: OneShotEffect::draw(Reference::You, Count::Literal(1)),
            })],
            ..CardFace::default()
        };
        let src = mint_on_field(&mut state, Card::Normal(source_face));

        let frame = frame_src_targets(actor, vec![src]);
        let hand_before = state.zones.hands[PlayerId(0).index()].len();
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Copy(CopySpec {
                    source: CopySource::Object(Reference::Target(0)),
                    exceptions: vec![],
                }),
                vec![],
            )),
            &frame,
        );

        // Drive the stack empty: the copied ETB trigger must fire, go on the
        // stack, and resolve. Stops the moment priority is offered over an
        // empty stack (mirrors `resolve/action.rs`'s
        // `drive_declining_or_casting`, simplified — this fixture has no
        // casting/paying/ordering choices to answer).
        for _ in 0..200 {
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(PendingDecision::Priority(_)) => {
                    if state.stack.is_empty() {
                        break;
                    }
                    state.submit_decision(Decision::Act(Act::Pass)).unwrap();
                }
                StepOutcome::NeedsDecision(other) => {
                    panic!("unexpected decision while the ETB trigger resolves: {other:?}")
                }
                StepOutcome::GameOver(_) => break,
            }
        }

        assert_eq!(
            state.zones.hands[PlayerId(0).index()].len(),
            hand_before + 1,
            "[CR#707.5]: the token copy's ETB trigger (copied from the \
             source's \"when this enters, draw a card\") fires and resolves, \
             proving the token entered via the normal battlefield path"
        );
    }

    /// A Tarmogoyf-shaped P/T CDA ([CR#604.3]): `Modify(This,
    /// Several([Power(Set(CountOf(creatures))), Toughness(Set(CountOf(
    /// creatures)))]))` — mirrors `layer.rs`'s `creature_count_cda` test
    /// fixture, rebuilt here since that helper is private to `layer.rs`'s
    /// own test module.
    fn tarmogoyf_shaped_cda() -> deckmaste_core::Ability {
        use deckmaste_core::Ability;
        use deckmaste_core::Countable;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::StaticEffect;
        let count = Count::CountOf(Countable::Objects(std::sync::Arc::new(
            Predicate::creature(),
        )));
        Ability::r#static(StaticEffect::Modify(
            Reference::This,
            Modification::Several(vec![
                Modification::Power(NumericOp::Set(count.clone())),
                Modification::Toughness(NumericOp::Set(count)),
            ]),
        ))
    }

    /// [CR#707.9d] (CDA behavioral proof, deferred from Task 2): a token
    /// copy of a source carrying a P/T-defining CDA, WITH a `Modify(P/T Set
    /// N)` exception, is N/N and the CDA is dropped — not just at the
    /// `CopiableValues` fold (`deckmaste_engine::copy`'s own unit tests
    /// already cover that pure fold), but end to end: the live layer engine
    /// reads a PINNED N/N off the actual minted token, not a re-derived
    /// value that happens to equal N by coincidence (proven by adding
    /// another creature afterward and confirming the token's P/T doesn't
    /// move).
    #[test]
    fn token_copy_modify_pt_drops_source_cda_end_to_end() {
        use deckmaste_core::CardFace;
        use deckmaste_core::CopyException;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::StatValue;

        let mut state = game();
        let goyf = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Tarmogoyf".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::DefinedByAbility),
                toughness: Some(StatValue::DefinedByAbility),
                abilities: vec![tarmogoyf_shaped_cda()],
                ..CardFace::default()
            }),
        );
        let frame = frame_src_targets(goyf, vec![goyf]);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Copy(CopySpec {
                    source: CopySource::Object(Reference::Target(0)),
                    exceptions: vec![
                        CopyException::Modify(Modification::Power(NumericOp::Set(Count::Literal(
                            5,
                        )))),
                        CopyException::Modify(Modification::Toughness(NumericOp::Set(
                            Count::Literal(5),
                        ))),
                    ],
                }),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies

        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != goyf)
            .expect("the copy token on the battlefield");
        let view = state.layers();
        assert_eq!(
            view.power(t),
            Some(5),
            "[CR#707.9d]: Modify(Power Set 5) overrides — the source's P/T-\
             defining CDA is dropped, not carried alongside the override"
        );
        assert_eq!(
            view.toughness(t),
            Some(5),
            "[CR#707.9d]: same for toughness"
        );

        // Add another creature: a live CDA would re-derive to a new count
        // (3, here) — the token's P/T staying pinned at 5/5 proves the CDA
        // is genuinely gone, not coincidentally reading 5.
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Bear".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..CardFace::default()
            }),
        );
        let view = state.layers();
        assert_eq!(
            view.power(t),
            Some(5),
            "pinned 5/5: no CDA left to re-derive from the new creature count"
        );
    }

    /// [CR#707.9d]'s converse: a token copy of the SAME Tarmogoyf-shaped
    /// source with NO `Modify` exception carries the P/T-defining CDA
    /// along — its live power/toughness tracks the creature count
    /// dynamically, exactly like the source's own.
    #[test]
    fn token_copy_without_modify_carries_source_cda() {
        use deckmaste_core::CardFace;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::StatValue;

        let mut state = game();
        let goyf = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Tarmogoyf".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::DefinedByAbility),
                toughness: Some(StatValue::DefinedByAbility),
                abilities: vec![tarmogoyf_shaped_cda()],
                ..CardFace::default()
            }),
        );
        let frame = frame_src_targets(goyf, vec![goyf]);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Create(
                Count::Literal(1),
                deckmaste_core::TokenSpec::Copy(CopySpec {
                    source: CopySource::Object(Reference::Target(0)),
                    exceptions: vec![],
                }),
                vec![],
            )),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies

        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != goyf)
            .expect("the copy token on the battlefield");
        // Two creatures on the battlefield right now: goyf + the token
        // itself (the token's own copied CDA counts itself too).
        let view = state.layers();
        assert_eq!(
            view.power(t),
            Some(2),
            "[CR#707.9d]: no Modify exception — the CDA rides along and \
             derives the live creature count (goyf + the token itself)"
        );
        assert_eq!(view.toughness(t), Some(2), "same for toughness");

        // A third creature bumps the dynamic count — the copy's CDA must
        // track it live, exactly like the source's own CDA would.
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Bear".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..CardFace::default()
            }),
        );
        let view = state.layers();
        assert_eq!(
            view.power(t),
            Some(3),
            "the copied CDA re-derives to the new creature count, same as \
             any live CDA would"
        );
    }

    // ====================================================================
    // Task 4.6 — end-to-end (Enchant + Equip + Fortify + Reconfigure)
    // ====================================================================
    //
    // These drive a real `GameState` and assert real state, exercising the
    // ACTUAL keyword-macro (builtin) + subtype-confer (canon) paths where
    // feasible — the integration coverage that caught the composite-flatten
    // prerequisite. Helpers below build cards from the live plugin macros.

    /// [CR#104.2b]: `WinGame` resolves to a first-class `PlayerWon` event
    /// that ends the game with the actor as winner.
    #[test]
    fn win_game_verb_sets_win_outcome() {
        let (mut state, _bear) = bear_on_field();
        let frame = frame_for(&state, PlayerId(0));
        state.run_effect(OneShotEffect::act_by_you(PlayerAction::WinGame), &frame);
        let _ = state.step();
        assert_eq!(
            state.outcome,
            Some(crate::state::GameOutcome::Win(PlayerId(0)))
        );
    }

    /// Platinum Angel's "your opponents can't win the game" static
    /// ([CR#101.1]) suppresses the `WinGame` verb for the gated opponent —
    /// a graceful no-op, no event emitted.
    #[test]
    fn cant_win_gated_win_game_is_a_noop() {
        let (state, _ids) = battlefield_with(&["Platinum Angel"]);
        let frame = frame_for(&state, PlayerId(1));
        assert_eq!(
            state.action_items(&Action::by_you(PlayerAction::WinGame), &frame),
            vec![],
            "Platinum Angel's opponents-can't-win gate suppresses the WinGame verb"
        );
    }

    /// [CR#104.3e]: `LoseGame` resolves to `PlayerLost { reason: Effect }`;
    /// in a 2-player game the survivor wins last-standing ([CR#104.2a]).
    #[test]
    fn lose_game_verb_sets_loss_and_opponent_wins() {
        let (mut state, _bear) = bear_on_field();
        let frame = frame_for(&state, PlayerId(0));
        state.run_effect(OneShotEffect::act_by_you(PlayerAction::LoseGame), &frame);
        let _ = state.step();
        assert!(state.players[0].lost);
        assert_eq!(
            state.outcome,
            Some(crate::state::GameOutcome::Win(PlayerId(1)))
        );
    }

    /// Platinum Angel's "you can't lose the game" static suppresses the
    /// `LoseGame` verb for its controller — a graceful no-op.
    #[test]
    fn cant_lose_gated_lose_game_is_a_noop() {
        let (state, _ids) = battlefield_with(&["Platinum Angel"]);
        let frame = frame_for(&state, PlayerId(0));
        assert_eq!(
            state.action_items(&Action::by_you(PlayerAction::LoseGame), &frame),
            vec![],
            "Platinum Angel's you-can't-lose gate suppresses the LoseGame verb"
        );
    }

    /// [CR#104.3f]: a player who would simultaneously win and lose in the
    /// same batch loses instead — the arbitration in `apply_occurrence`
    /// drops the `PlayerWon` fact before the batch applies, so the OTHER
    /// player takes the last-standing win.
    #[test]
    fn win_and_lose_batch_arbitration_drops_the_win() {
        let (mut state, _bear) = bear_on_field();
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(vec![
            GameEvent::PlayerWon(PlayerWon {
                player: PlayerId(0),
            }),
            GameEvent::PlayerLost(PlayerLost {
                player: PlayerId(0),
                reason: crate::event::LossReason::LifeZero,
            }),
        ]))]);
        let _ = state.step();
        assert!(state.players[0].lost, "the loss still applies");
        assert_eq!(
            state.outcome,
            Some(crate::state::GameOutcome::Win(PlayerId(1))),
            "the dropped win lets player 1 take the last-standing win instead"
        );
    }

    /// [CR#705.1]: an uncalled `FlipCoins(3, false)` draws 3 coins straight
    /// from the seeded rng with NO decision and no winner/loser, as ONE
    /// simultaneous batch; the applied batch fixes "that many" to the number
    /// of heads. Seed-pinned (via `game()`'s seed 7): same seed ⇒ same draw,
    /// so the assertions are exact, not just shape checks.
    #[test]
    fn uncalled_flip_emits_batch_and_fixes_that_many_to_heads() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(3), false)),
            &frame,
        );
        drain_progress(&mut state, 20);

        let flips: Vec<(bool, Option<bool>)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped(CoinFlipped { heads, won, .. }) => Some((*heads, *won)),
                _ => None,
            })
            .collect();
        assert_eq!(flips.len(), 3, "3 CoinFlipped facts, one per drawn coin");
        assert!(
            flips.iter().all(|&(_, won)| won.is_none()),
            "an uncalled flip never records a winner/loser"
        );
        let heads = Uint::try_from(flips.iter().filter(|&&(h, _)| h).count())
            .expect("heads count fits Uint");
        assert_eq!(
            state.that_much,
            Some(heads),
            "\"that many\" is fixed to the number of heads"
        );
    }

    /// [CR#706.1]: `RollDice(3, 6)` draws 3 naturals in `1..=6` from the
    /// seeded rng, each `result == natural` (no modifier pipeline yet), as
    /// ONE simultaneous batch; the applied batch fixes "that many" to the
    /// summed results.
    #[test]
    fn dice_roll_emits_per_die_and_fixes_that_many_to_sum() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(3), 6)),
            &frame,
        );
        drain_progress(&mut state, 20);

        let rolls: Vec<(Uint, Uint)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::DieRolled(DieRolled {
                    natural, result, ..
                }) => Some((*natural, *result)),
                _ => None,
            })
            .collect();
        assert_eq!(rolls.len(), 3, "3 DieRolled facts, one per drawn die");
        assert!(
            rolls
                .iter()
                .all(|&(natural, result)| (1..=6).contains(&natural) && natural == result),
            "every natural lands in 1..=6 and result == natural (no modifier pipeline yet)"
        );
        let sum: Uint = rolls.iter().map(|&(_, result)| result).sum();
        assert_eq!(
            state.that_much,
            Some(sum),
            "\"that many\" is fixed to the summed results"
        );
    }

    /// [CR#701.9b]: `Discard { random: true, .. }` samples straight from the
    /// seeded rng via the general `Selection::Random` binder machinery
    /// (Task 8) — no `ChooseObjects` decision surfaces (no choice exists for
    /// a random discard), and the sampled cards move Hand→Graveyard.
    #[test]
    fn random_discard_samples_without_decision() {
        let mut state = game();
        let p0 = PlayerId(0);
        for i in 0..5 {
            mint_in_hand(&mut state, p0, &format!("Random Discard Card {i}"));
        }
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::Act(Action::discard(Reference::You, Count::Literal(2), true)),
            &frame,
        );
        // Drain until the discard resolves (graveyard gains 2) or SOME
        // decision surfaces first — a `ChooseObjects` decision here would
        // mean the "random" path wrongly asked the player to choose. The
        // normal game's own `Priority` window (reached once resolution
        // completes) is not that decision, so it is not itself a failure.
        for _ in 0..20 {
            if state.zones.graveyards[p0.index()].len() == 2 {
                break;
            }
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
        assert!(
            !matches!(
                state.pending,
                Some(PendingDecision::ChooseObjects(
                    crate::decide::pending::ChooseObjects { .. }
                ))
            ),
            "a random discard surfaces no ChooseObjects decision (no choice exists): {:?}",
            state.pending
        );
        assert_eq!(
            state.zones.hands[p0.index()].len(),
            3,
            "2 of 5 cards left the hand"
        );
        assert_eq!(
            state.zones.graveyards[p0.index()].len(),
            2,
            "the sampled 2 cards landed in the graveyard"
        );
    }

    /// The seeded rng is deterministic: two identical states (same seed),
    /// driven through the identical `FlipCoins` + `RollDice` sequence, draw
    /// identical heads/naturals.
    #[test]
    fn randomness_is_seed_deterministic() {
        fn drive() -> (Vec<bool>, Vec<Uint>) {
            let mut state = game();
            let p0 = PlayerId(0);
            let frame = frame_for(&state, p0);
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(3), false)),
                &frame,
            );
            drain_progress(&mut state, 20);
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(3), 6)),
                &frame,
            );
            drain_progress(&mut state, 20);

            let heads: Vec<bool> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::CoinFlipped(CoinFlipped { heads, .. }) => Some(*heads),
                    _ => None,
                })
                .collect();
            let naturals: Vec<Uint> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::DieRolled(DieRolled { natural, .. }) => Some(*natural),
                    _ => None,
                })
                .collect();
            (heads, naturals)
        }

        assert_eq!(
            drive(),
            drive(),
            "same seed ⇒ identical flip/roll sequence across independent states"
        );
    }

    // ========================================================================
    // `engine-randomness` Task 4: the `CallFlip` decision — called flips
    // ([CR#705.2]) draw per submitted call, scoring `won`, and either
    // re-surface the next call (multi-coin) or front-schedule the
    // accumulated batch.
    // ========================================================================

    /// [CR#705.2]: a single CALLED flip surfaces one `CallFlip` decision (no
    /// draw yet); submitting the call draws the coin and scores `won = (call
    /// == heads)`. Seed-pinned (via `game()`'s seed 7): the draw is
    /// deterministic, so the assertion is exact, not just a shape check.
    #[test]
    fn called_flip_surfaces_call_and_scores_won() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
            &frame,
        );
        drain_progress(&mut state, 20);
        let Some(PendingDecision::CallFlip(crate::decide::pending::CallFlip { player })) =
            state.pending.clone()
        else {
            panic!("expected a pending CallFlip, got {:?}", state.pending);
        };
        assert_eq!(player, p0);

        state.submit_decision(Decision::Answer(true)).unwrap();
        drain_progress(&mut state, 20);

        let flips: Vec<(bool, Option<bool>)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped(CoinFlipped { heads, won, .. }) => Some((*heads, *won)),
                _ => None,
            })
            .collect();
        assert_eq!(flips.len(), 1, "exactly one CoinFlipped fact");
        let (heads, won) = flips[0];
        assert_eq!(
            won,
            Some(heads),
            "the call was heads: won iff the draw landed heads"
        );
        assert_eq!(
            state.that_much,
            Some(Uint::from(won == Some(true))),
            "\"that many\" is fixed to the win count (0 or 1)"
        );
    }

    /// [CR#705.2]: a 3-coin CALLED flip pauses per coin — three sequential
    /// `CallFlip` decisions, each drawing (and scoring) only when its call is
    /// submitted — then front-schedules ONE simultaneous batch
    /// ([CR#603.3b]) once all three are called.
    #[test]
    fn multi_coin_called_flip_pauses_per_coin() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(3), true)),
            &frame,
        );
        drain_progress(&mut state, 20);

        for (i, call) in [true, false, true].into_iter().enumerate() {
            let Some(PendingDecision::CallFlip(crate::decide::pending::CallFlip { player })) =
                state.pending.clone()
            else {
                panic!(
                    "coin {i}: expected a pending CallFlip, got {:?}",
                    state.pending
                );
            };
            assert_eq!(player, p0, "coin {i}");
            state.submit_decision(Decision::Answer(call)).unwrap();
            drain_progress(&mut state, 20);
        }

        let flips: Vec<(bool, Option<bool>)> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped(CoinFlipped { heads, won, .. }) => Some((*heads, *won)),
                _ => None,
            })
            .collect();
        assert_eq!(flips.len(), 3, "3 CoinFlipped facts, one per called coin");
        assert!(
            flips.iter().all(|&(_, won)| won.is_some()),
            "every called flip records a winner/loser"
        );
        let won_count = Uint::try_from(flips.iter().filter(|&&(_, won)| won == Some(true)).count())
            .expect("win count fits Uint");
        assert_eq!(
            state.that_much,
            Some(won_count),
            "\"that many\" is fixed to the win count across the whole batch"
        );
    }

    /// A `CallFlip` decision only answers `Decision::Answer` — any other
    /// decision kind (here, a stray `Discard`) is rejected as `WrongKind`,
    /// leaving the `CallFlip` decision (and its stashed continuation) intact.
    #[test]
    fn call_flip_rejects_wrong_decision_kind() {
        use crate::decide::Decision;
        use crate::decide::DecisionError;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
            &frame,
        );
        drain_progress(&mut state, 20);
        assert!(matches!(
            state.pending,
            Some(PendingDecision::CallFlip(
                crate::decide::pending::CallFlip { .. }
            ))
        ));

        assert_eq!(
            state.submit_decision(Decision::Discard(vec![])),
            Err(DecisionError::WrongKind)
        );
        assert!(
            matches!(
                state.pending,
                Some(PendingDecision::CallFlip(
                    crate::decide::pending::CallFlip { .. }
                ))
            ),
            "a rejected wrong-kind decision leaves the CallFlip pending"
        );
    }

    // ========================================================================
    // `engine-randomness` Task 6: trigger integration ([CR#705.2,706.1]) +
    // full-random-surface reproducibility. Unlike `trigger.rs`'s
    // `scan_event`/`scan_triggers`-direct tests, these drive the REAL
    // `FlipCoins`/`RollDice`/`CallFlip` pipeline end to end (`run_effect` +
    // `drain_progress` + `submit_decision`, this module's own harness) so the
    // rng draws and the `won` scoring are genuine, not hand-built facts.
    // ========================================================================

    /// A synthetic in-Rust creature whose sole ability is a "draw a card"
    /// trigger watching `event` — the fire-detection body Task 6's trigger
    /// tests register on the battlefield. Mirrors `trigger.rs`'s
    /// `draw_on`/`put_synthetic_on_field` pair.
    fn put_watcher(
        state: &mut GameState,
        controller: PlayerId,
        event: deckmaste_core::EventFilter,
    ) -> ObjectSource {
        use deckmaste_core::Ability;
        use deckmaste_core::CardFace;
        use deckmaste_core::TriggeredAbility;

        let card = Card::Normal(CardFace {
            name: "Randomness Watcher".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                from: None,
                event,
                condition: None,
                limits: Vec::new(),
                effect: OneShotEffect::draw(Reference::You, Count::Literal(1)),
            })],
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        state.objects.obj(id).source
    }

    /// Steps `state` forward exactly `n` times, asserting every call makes
    /// `Progress` (never surfaces a decision or ends the game) — the
    /// surgical alternative to `drain_progress`'s open-ended draining for a
    /// test that drives MULTIPLE actions through one shared state.
    /// `drain_progress`'s generous bound is fine for a test that drives a
    /// single action and then only reads `state.history` (the Task 3/4
    /// tests above), but draining generously a SECOND time on the same
    /// `game()` fixture risks running past the action under test into this
    /// bare fixture's leftover ambient turn-structure agenda (the one
    /// `BeginStep` `GameState::new()` schedules), surfacing an unrelated
    /// `Priority`/`DeclareAttackers` decision that then blocks the next
    /// `run_effect` — `step()` returns `NeedsDecision` idempotently without
    /// popping the agenda while ANY decision is pending, including a stray
    /// one this test never answers. Panics with the offending outcome if a
    /// step count assumption is wrong, rather than failing many calls later
    /// with a confusing "wrong decision kind" mismatch.
    fn step_n(state: &mut GameState, n: usize) {
        for i in 0..n {
            match state.step() {
                StepOutcome::Progress(_) => {}
                other => panic!("step {i}/{n}: expected Progress, got {other:?}"),
            }
        }
    }

    /// [CR#705.2]: a trigger on `CoinFlipped { won: Some(true) }` notes only
    /// for a CALLED flip the flipper WON — never a loss, and (per the sibling
    /// test) never an uncalled flip at all. Seed-pinned (`game()`'s seed 7):
    /// drives single called flips — always calling heads — until this
    /// deterministic sequence lands a win, asserting along the way that every
    /// LOSS left the trigger silent, then that the WIN noted it exactly once.
    /// The retry count is bounded (50 — astronomically unreachable for a fair
    /// coin, so a real regression fails loud) but the actual path taken is
    /// fixed by the seed, so the test is not flaky.
    #[test]
    fn win_flip_trigger_fires_on_won_called_flip_only() {
        use deckmaste_core::EventFilter;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        let mut state = game();
        let p0 = PlayerId(0);
        let watcher = put_watcher(
            &mut state,
            p0,
            EventFilter::CoinFlipped {
                by: Predicate::Ref(Reference::You),
                won: Some(true),
            },
        );
        let frame = frame_for(&state, p0);

        let mut won = false;
        for i in 0..50 {
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
                &frame,
            );
            // Pop exactly the front-scheduled `FlipCoins` work item — it
            // sets `pending` directly; no decision surfaces on this step.
            step_n(&mut state, 1);
            let Some(PendingDecision::CallFlip(crate::decide::pending::CallFlip { player })) =
                state.pending.clone()
            else {
                panic!(
                    "attempt {i}: expected a pending CallFlip, got {:?}",
                    state.pending
                );
            };
            assert_eq!(player, p0, "attempt {i}");
            state.submit_decision(Decision::Answer(true)).unwrap();
            // Pop exactly the front-scheduled `CoinFlipped` batch — one
            // step, which also runs the trigger scan synchronously
            // (scheduling a `TriggerFired` at the front on a match).
            step_n(&mut state, 1);

            let flips: Vec<Option<bool>> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::CoinFlipped(CoinFlipped { won, .. }) => Some(*won),
                    _ => None,
                })
                .collect();
            assert_eq!(
                flips.len(),
                i + 1,
                "attempt {i}: one CoinFlipped fact per called flip so far"
            );
            if flips[i] == Some(true) {
                won = true;
                // The trigger matched — its `TriggerFired` sits at the
                // agenda front (scheduled by the scan inside the batch-apply
                // step above); one more step notes it into
                // `pending_triggers`.
                step_n(&mut state, 1);
                assert_eq!(
                    state.pending_triggers.len(),
                    1,
                    "attempt {i}: a WIN must note the win-only trigger exactly once: {:?}",
                    state.pending_triggers
                );
                assert_eq!(state.pending_triggers[0].source, watcher, "attempt {i}");
                assert_eq!(state.pending_triggers[0].controller, p0, "attempt {i}");
                break;
            }
            assert!(
                state.pending_triggers.is_empty(),
                "attempt {i}: a LOSS must not note the win-only trigger: {:?}",
                state.pending_triggers
            );
        }
        assert!(
            won,
            "seed 7's called-flip sequence never won within 50 attempts"
        );
    }

    /// [CR#705.2]: nobody wins or loses an UNCALLED flip — the same win-only
    /// trigger (`CoinFlipped { won: Some(true) }`) must stay silent on
    /// `FlipCoins(1, false)`, whatever the coin lands.
    #[test]
    fn win_flip_trigger_ignores_uncalled_flips() {
        use deckmaste_core::EventFilter;

        let mut state = game();
        let p0 = PlayerId(0);
        put_watcher(
            &mut state,
            p0,
            EventFilter::CoinFlipped {
                by: Predicate::Ref(Reference::You),
                won: Some(true),
            },
        );
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), false)),
            &frame,
        );
        drain_progress(&mut state, 20);

        let flips: Vec<Option<bool>> = state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped(CoinFlipped { won, .. }) => Some(*won),
                _ => None,
            })
            .collect();
        assert_eq!(
            flips,
            vec![None],
            "the uncalled flip drew but crowned no winner"
        );
        assert!(
            state.pending_triggers.is_empty(),
            "[CR#705.2]: an uncalled flip must never fire a win-only trigger: {:?}",
            state.pending_triggers
        );
    }

    /// [CR#706.1]: `DiceRolled` fires once PER DIE — a 2-die roll notes the
    /// watching trigger twice, not once for the whole roll.
    #[test]
    fn dice_trigger_fires_per_die() {
        use deckmaste_core::EventFilter;

        let mut state = game();
        let p0 = PlayerId(0);
        put_watcher(
            &mut state,
            p0,
            EventFilter::DiceRolled {
                by: Predicate::Ref(Reference::You),
            },
        );
        let frame = frame_for(&state, p0);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(2), 6)),
            &frame,
        );
        drain_progress(&mut state, 20);

        assert_eq!(
            state.pending_triggers.len(),
            2,
            "2 dice rolled must note the per-die trigger twice: {:?}",
            state.pending_triggers
        );
    }

    /// The ENTIRE random surface — uncalled flips, a called flip, dice, and a
    /// random discard — replays bit for bit from the same seed given the same
    /// scripted decisions. Two independent `GameState`s (same seed, same
    /// setup), driven through the identical script, log identical
    /// `CoinFlipped`/`DieRolled` facts and sample the identical hand cards
    /// for the random discard.
    ///
    /// The discard sample is compared by POSITION in each drive's own
    /// freshly-minted hand (`minted`), not by raw `ObjectId` — a zone move
    /// remints the object (a fresh `ObjectId`), so only the underlying
    /// `CardId`/`ObjectSource` spine is a stable identity; comparing by
    /// per-drive position sidesteps relying on that spine lining up
    /// numerically across two independently-constructed `GameState`s (it
    /// does, since nothing but the rng draws differs between them, but the
    /// position comparison needs no such assumption).
    ///
    /// The zone-move fact actually compared is the past-form `GameEvent::
    /// ZoneChange` (`from: Hand, to: Graveyard`) — the committed FACT a
    /// discard emits; the future-form `ZoneChange` is the pre-evolution
    /// INTENT and is never recorded to history (`record_history` skips it
    /// on purpose, folded into its downstream past form).
    #[test]
    fn full_random_surface_is_replayable() {
        use deckmaste_core::Uint;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        /// One drive's recorded surface: `(heads, won)` per coin, `(natural,
        /// result)` per die, and the discard sample's positions in `minted`.
        type DriveResult = (Vec<(bool, Option<bool>)>, Vec<(Uint, Uint)>, Vec<usize>);

        fn drive() -> DriveResult {
            let mut state = game();
            let p0 = PlayerId(0);
            let frame = frame_for(&state, p0);

            let minted: Vec<ObjectSource> = (0..5)
                .map(|i| {
                    let id = mint_in_hand(&mut state, p0, &format!("Replay Card {i}"));
                    state.objects.obj(id).source
                })
                .collect();

            // 2 uncalled flips — one PlayerAction, one `CoinFlipped` batch:
            // one step to dispatch `WorkItem::FlipCoins`, one to apply the
            // batch. (Precise `step_n`, not `drain_progress`'s open-ended
            // bound — see `step_n`'s doc: this test drives FOUR actions
            // through one shared state, and draining generously after each
            // one risks falling through into this bare `game()` fixture's
            // leftover ambient turn-structure agenda, surfacing a stray
            // `Priority` that then blocks the next `run_effect`.)
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(2), false)),
                &frame,
            );
            step_n(&mut state, 2);

            // 1 called flip — the same call answer on both drives.
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::FlipCoins(Count::Literal(1), true)),
                &frame,
            );
            step_n(&mut state, 1);
            assert!(matches!(
                state.pending,
                Some(PendingDecision::CallFlip(
                    crate::decide::pending::CallFlip { .. }
                ))
            ));
            state.submit_decision(Decision::Answer(true)).unwrap();
            step_n(&mut state, 1);

            // RollDice(2, 20).
            state.run_effect(
                OneShotEffect::act_by_you(PlayerAction::RollDice(Count::Literal(2), 20)),
                &frame,
            );
            step_n(&mut state, 2);

            // Random discard of 2 from the 5-card hand ([CR#701.9b]): the
            // discard composite's `With(Existing(Random(..)), Each(..))` body
            // (Task 8) opens its own performer-only carry `Act` window first
            // ([CR#616.1] — apply #1), then the `RunEffect{With}` samples the
            // seeded rng and binds the picks with no decision (apply #2),
            // then `RunEffect{Each}` recurses each pick into its own bound
            // single-move `Act(Discard)` and batches their windows into ONE
            // simultaneous intent `Emit` (apply #3), then applying THAT
            // intent batch captures LKI/remints and collects the evolved
            // past-form `ZoneChange` batch into `evolving_batch` rather than
            // applying it inline (apply #4 — `schedule_evolution`'s batch
            // path front-schedules it as its own follow-on `WorkItem::Emit`),
            // then applying THAT batch actually records the past-form
            // `ZoneChange` facts to history (apply #5).
            state.run_effect(
                OneShotEffect::Act(Action::discard(Reference::You, Count::Literal(2), true)),
                &frame,
            );
            step_n(&mut state, 5);

            let flips: Vec<(bool, Option<bool>)> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::CoinFlipped(CoinFlipped { heads, won, .. }) => Some((*heads, *won)),
                    _ => None,
                })
                .collect();
            let rolls: Vec<(Uint, Uint)> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::DieRolled(DieRolled {
                        natural, result, ..
                    }) => Some((*natural, *result)),
                    _ => None,
                })
                .collect();
            let discarded: Vec<usize> = state
                .history
                .entries()
                .filter_map(|e| match &e.fact {
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: Some(snapshot),
                        from: Some(Zone::Hand),
                        to: Zone::Graveyard,
                        ..
                    }) => minted.iter().position(|&m| m == snapshot.source),
                    _ => None,
                })
                .collect();

            (flips, rolls, discarded)
        }

        let (a_flips, a_rolls, a_discarded) = drive();
        let (b_flips, b_rolls, b_discarded) = drive();
        assert_eq!(
            a_flips.len(),
            3,
            "2 uncalled + 1 called ⇒ 3 CoinFlipped facts"
        );
        assert_eq!(
            a_flips, b_flips,
            "same seed ⇒ identical coin sequence (heads + won)"
        );
        assert_eq!(a_rolls.len(), 2, "2 dice ⇒ 2 DieRolled facts");
        assert_eq!(
            a_rolls, b_rolls,
            "same seed ⇒ identical die sequence (natural + result)"
        );
        assert_eq!(a_discarded.len(), 2, "2 cards sampled for the discard");
        assert_eq!(
            a_discarded, b_discarded,
            "same seed ⇒ identical random-discard sample"
        );
    }
}
