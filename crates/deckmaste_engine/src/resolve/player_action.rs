//! `player_action_items`: lower the former-`PlayerAction` family of
//! [`Action`](deckmaste_core::Action) verbs (draw, discard, mana, tokens,
//! randomness, …) to events and work items.

use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::ChosenValueKind;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CopyRetarget;
use deckmaste_core::CopySource;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaSpec;
use deckmaste_core::RetargetMode;
use deckmaste_core::Selection;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use super::occurrence_of;
use crate::agenda::WorkItem;
use crate::event::Act;
use crate::event::Cause;
use crate::event::Copied;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DamageRemoved;
use crate::event::DesignationChanged;
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
use crate::stack::ExecutionFrame;
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
    /// The `Emit` work item(s) one former-`PlayerAction` verb produces. Each
    /// arm resolves its OWN agent/patient/recipient slot inline (the `By`
    /// wrapper's centrally pre-resolved `actor` is gone — every verb spells
    /// its own role slot now, Law 2); agent-silent verbs (`Tap`/`Untap`/
    /// `PutCounters`/`RemoveCounters`/`Reveal`/`RemoveDamage`) resolve none.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per player verb; splitting would scatter the dispatch"
    )]
    pub(super) fn player_action_items(
        &self,
        action: &Action,
        frame: &ExecutionFrame,
    ) -> Vec<WorkItem> {
        use crate::event::Occurrence;
        // [CR#601.2h]: a cost-eligible verb performed to pay a cost is that
        // AGENCY, not a plain effect instruction — see the mirrored
        // computation and doc in `resolve/action.rs::composite_items`. Tap
        // stays out of this: its cost-payment `Tapped` events are stamped
        // directly at their own call sites (cast.rs et al.), never through
        // this dispatch, so this arm's `Cause::tap` below is always the
        // effect-instruction reading.
        let agency = if frame.payment.is_some() {
            Agency::CostPayment
        } else {
            Agency::EffectInstruction
        };
        // [CR#118.10]: the paying payment's id, when there is one.
        let payment_id = frame.payment.map(|p| p.id);
        match action {
            Action::Tap(sel) => {
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
                                Some((frame.source(self), frame.controller(self))),
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
            // [CR#119.3,119.5,119.9] the merged `GainLife`/`LoseLife`/`SetLife`
            // family: `patient` is spelled directly (no more `By`-resolved
            // `actor`), and `Set` still resolves as a gain/loss of the
            // necessary difference below — never a bespoke "set" event.
            //
            // Two-Headed Giant note: a team shares ONE life total in that
            // variant — [CR#810.4], the shared-total rule (the plan's
            // original [CR#119.1a] cite is only the STARTING total, 30 not
            // 20, and does not say this). This resolution writes only
            // `patient`'s own `life`; propagating the delta to `patient`'s
            // teammate too is an unmodeled variant-gated seam (no
            // multiplayer/team engine seat exists yet), the same boundary
            // `LossReason::Poison`'s doc (`event.rs`) already draws for the
            // fifteen-counter team check.
            Action::ChangeLife(patient, op) => {
                let patient = self.acting_player(patient, frame);
                // [CR#119.9]'s source: the resolving ability and its
                // controller, the shared `(frame.source(self), frame.controller(self))`
                // cause binding every other verb uses — never the patient
                // (there is no agent role, [CR#119.9,119.10]).
                let gain_cause = Some(
                    Cause::gain_life(agency, Some((frame.source(self), frame.controller(self))))
                        .with_payment(payment_id),
                );
                let lose_cause = Some(
                    Cause::lose_life(agency, Some((frame.source(self), frame.controller(self))))
                        .with_payment(payment_id),
                );
                match op {
                    LifeOp::Down(qty) => {
                        let amount = self.eval_count(qty, frame);
                        vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost(
                            LifeLost {
                                player: patient,
                                amount,
                                cause: lose_cause,
                            },
                        )))]
                    }
                    LifeOp::Up(qty) => {
                        let amount = self.eval_count(qty, frame);
                        vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained(
                            LifeGained {
                                player: patient,
                                amount,
                                cause: gain_cause,
                            },
                        )))]
                    }
                    // [CR#119.5]: set-to-N resolves as a gain or loss of the
                    // difference — triggers see the gain/loss, never a "set";
                    // equal totals produce no event (transition-only).
                    LifeOp::Set(qty) => {
                        let target = deckmaste_core::Int::try_from(self.eval_count(qty, frame))
                            .expect("life total fits in i32");
                        let current = self.player(patient).life;
                        let event = match target.cmp(&current) {
                            std::cmp::Ordering::Less => GameEvent::LifeLost(LifeLost {
                                player: patient,
                                amount: Uint::try_from(current - target)
                                    .expect("positive difference"),
                                cause: lose_cause,
                            }),
                            std::cmp::Ordering::Greater => GameEvent::LifeGained(LifeGained {
                                player: patient,
                                amount: Uint::try_from(target - current)
                                    .expect("positive difference"),
                                cause: gain_cause,
                            }),
                            std::cmp::Ordering::Equal => return vec![],
                        };
                        vec![WorkItem::Emit(Occurrence::Single(event))]
                    }
                }
            }
            Action::Untap(sel) => {
                // [CR#701.26b]: the mirror of `Tap` above — only a tapped
                // permanent can be untapped, a no-op is no event.
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .filter(|&object| self.objects.obj(object).tapped)
                    .map(|object| {
                        GameEvent::Untapped(
                            object,
                            Some(Cause::untap(
                                Agency::EffectInstruction,
                                Some((frame.source(self), frame.controller(self))),
                            )),
                        )
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            Action::Sacrifice(agent, what) => {
                // [CR#701.21a]: the agent moves the sacrificed permanent to its
                // owner's graveyard — the `Sacrificed` verb fact evolves into
                // the zone move at apply. That the reference names a
                // permanent the agent controls is the grammar's contract; a
                // legality pass is a later seam. `_actor` is still resolved
                // for its validating panic (the reference must name a
                // player) — the SACRIFICER stays in the actor channel via
                // the fact's own derivation (`FactView::of`'s `ZoneChange`
                // arm reads the moved object's controller, which
                // [CR#701.21a] guarantees IS the sacrificer; Mayhem Devil
                // reads it there), not through the cause tuple below.
                let _actor = self.acting_player(agent, frame);
                let events: Vec<GameEvent> = self
                    .eval_reference_set(what, frame)
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
                            cause: Some(
                                // The shared `(frame.source(self), frame.controller(self))`
                                // cause binding every other verb uses (design
                                // record §10.2) — NOT the sacrificer, which
                                // can differ under an edict ("target player
                                // sacrifices a creature").
                                Cause::sacrifice(agency, Some((frame.source(self), frame.controller(self))))
                                    .with_payment(payment_id),
                            ),
                        })
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // NOTE: the player-agent twin of `Action::Move` was DELETED
            // ([CR#400.7]; unreachable + unattested) — `Action::Move` is
            // agent-silent and dispatched directly by `action_items`, never
            // reaching this function.
            // [CR#114.1]: the recipient gets an emblem carrying `abilities`. The
            // synthesis + command-zone mint happens at apply (the `&mut self`
            // stage); here we only emit the fact. Getting an emblem is not a
            // zone-change trigger ([CR#114.5] — an emblem is never a
            // permanent; it never enters the battlefield).
            Action::GetEmblem(recipient, abilities) => {
                let actor = self.acting_player(recipient, frame);
                vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::EmblemCreated(EmblemCreated {
                        player: actor,
                        abilities: abilities.to_vec(),
                    }),
                ))]
            }
            // [CR#701.24a]: shuffle a collection; the Shuffled apply
            // randomizes via the seeded rng. `Selection::LibraryOf(who)` is
            // the whole-library shape ([CR#701.24a] "a library"); the
            // face-down-pile shape ([CR#701.24a]'s other object) rides a
            // pile-valued region register and is an engine seam (never wired
            // to a `Shuffled` event before this reshape either — fizzles,
            // never panics).
            Action::Shuffle(sel) => match sel {
                Selection::LibraryOf(whose) => match self.eval_player_ref(whose, frame) {
                    Some(player) => {
                        vec![WorkItem::Emit(Occurrence::single(GameEvent::Shuffled(
                            player,
                        )))]
                    }
                    None => vec![],
                },
                // engine seam: shuffling a face-down pile (or any other
                // collection shape) has no `Shuffled`-event target yet.
                _ => vec![],
            },
            // Unbuilt seams: outcome verbs (immediate, gate-checked at the
            // OUTCOME layer — never deontic rows) and reveal/look.
            // [CR#104.2b]: "the patient wins the game" — a first-class win
            // event, suppressed by a matching `CantWin` gate ([CR#101.1]).
            // The last-player-standing win ([CR#104.2a]) never rides this verb.
            Action::WinGame(patient) => {
                let patient = self.acting_player(patient, frame);
                let view = self.layers();
                if self.gate_suppresses(&view, patient, deckmaste_core::OutcomeGateKind::CantWin) {
                    return vec![];
                }
                vec![WorkItem::Emit(Occurrence::single(GameEvent::PlayerWon(
                    PlayerWon { player: patient },
                )))]
            }
            // [CR#104.3e]: "the patient loses the game", suppressed by a
            // matching `CantLose` gate ([CR#101.1]).
            Action::LoseGame(patient) => {
                let patient = self.acting_player(patient, frame);
                let view = self.layers();
                if self.gate_suppresses(&view, patient, deckmaste_core::OutcomeGateKind::CantLose) {
                    return vec![];
                }
                vec![WorkItem::Emit(Occurrence::single(GameEvent::PlayerLost(
                    PlayerLost {
                        player: patient,
                        reason: crate::event::LossReason::Effect,
                    },
                )))]
            }
            Action::RestartGame => {
                todo!(
                    "engine seam: restarting the game ([CR#727.1] — a terminal \
                     with carryover, not a reset); owner: engine-restart-game"
                )
            }
            Action::Reveal { what, to } => {
                let object = self.eval_reference(what, frame);
                if self.objects.get(object).is_none() {
                    vec![]
                } else {
                    // Fail CLOSED: a semantic subset-look (`to: Some(..)`)
                    // whose player doesn't resolve fizzles the reveal — it
                    // must NOT widen into the `to: None` "revealed to all
                    // players" form (invalid semantic input fizzles; a private look
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
            // [CR#608.2c,608.2d,607.2] a resolution choice stored under a
            // note key, KIND-GATED. Only kinds with an existing engine READER
            // get wired (reader-gated); write-only kinds stay LOUD per-kind
            // (naming the kind), so a card reaching an unbuilt kind trips a
            // labeled seam rather than silently no-op'ing. The verb is
            // `&self`, so it only SCHEDULES the surfacing work item; the
            // `&mut self` handler opens the decision. The kind-space is now
            // [`ChosenValueKind`] (object-set note kinds are separate
            // store-side values, never reached through this node).
            Action::ChooseValue(who, kind, key) => {
                let actor = self.acting_player(who, frame);
                match kind {
                    // Number → an activation-register binding.
                    ChosenValueKind::Number => vec![WorkItem::ChooseNoteNumber {
                        player: actor,
                        key: *key,
                    }],
                    // No reader grammar exists yet for chosen colors
                    // (no chosen-color predicate), so this is a write with no
                    // consumer today. It IS grammar-spellable ([CR#607.2]),
                    // so a card reaching this arm is a live path — fizzle
                    // (no work item, nothing noted) rather than panic.
                    // CardName is read by `Named(key)`.
                    ChosenValueKind::Color => vec![],
                    ChosenValueKind::CardName => vec![WorkItem::ChooseNoteCardName {
                        player: actor,
                        key: *key,
                    }],
                }
            }
            // [CR#707.10]: put a copy of `spec` onto the stack — the APPLY
            // mints it (needs &mut). A reference that doesn't resolve to a
            // live stack entry fizzles (semantic-input errors never crash).
            // `retarget`/`CopySource::SelfCard` semantics are unwired this
            // task (T7) — only the `AsIs` + `CopySource::Object` shape that
            // was previously spellable is live; anything else fizzles as a
            // documented seam rather than silently dropping the instruction.
            Action::CopySpell {
                controller,
                spec,
                retarget,
            } => {
                let actor = self.acting_player(controller, frame);
                if !matches!(retarget, CopyRetarget::AsIs) {
                    // engine seam: MayChooseNew/TargetsThat retarget modes
                    // are T7's to wire.
                    return vec![];
                }
                let CopySource::Object(what) = &spec.source else {
                    // engine seam: a self-card copy source is new grammar
                    // this task didn't wire an emission path for.
                    return vec![];
                };
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
            // [CR#707.12]: "cast a copy of [source]" — NOT `CopySpell`'s
            // stack-copy above; this runs the full [CR#601.2a..601.2h]
            // casting pipeline in the source's own zone. That pipeline
            // (legality, cost payment, targeting) needs the shared
            // announce/cast machinery `cast_as_effect_items` above already
            // drives for `Action::Cast`, wired for a COPY object
            // rather than a live card — grammar-only here, so this arm
            // fizzles (no events) rather than running it.
            // execution: engine-copy-permanent-spells
            Action::CastCopy(_agent, _spec) => vec![],
            // [CR#608.2g]: cast the referenced card DURING resolution — the
            // agent follows the [CR#601.2a..601.2i] steps (reusing the shared
            // announce chain), except no player receives priority after it's
            // cast; the cast spell becomes the topmost stack object and the
            // currently-resolving ability continues. The May "yes" branch that
            // reaches this arm was already gated on `can_cast_as_effect` (the
            // effect grants the permission, [CR#608.2g]), so a live castable
            // referent is expected; a reference that no longer resolves to a
            // castable object fizzles (semantic-input errors never crash).
            Action::Cast(agent, what, for_cost) => {
                let actor = self.acting_player(agent, frame);
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
            // player) fizzles — semantic-input errors never crash the engine.
            //
            // `mode` is the [CR#115.7a..115.7c] three-way discriminant PLUS
            // `ChooseNew` ([CR#115.7d]), distinguished by PRINTED WORDING
            // (Redirect's "change any number of targets" vs. Bolt Bend's
            // "change target" vs. "change all targets" vs. "choose new
            // targets"). Only `ChooseNew` is exercised by canon or testing
            // fixtures today, and it alone drives the legal-set derivation
            // `open_choose_new_targets` implements. Running that SAME
            // derivation under `ChangeAll`/`ChangeOne`/`ChangeAny`'s name
            // would be silently wrong — a different slot-count contract per
            // mode — so those three fizzle explicitly (no work item) rather
            // than mimicking `ChooseNew`, until a witness demands their own
            // derivations.
            Action::Retarget { mode, of, by } => {
                if !matches!(mode, RetargetMode::ChooseNew) {
                    return vec![];
                }
                let entry = self.eval_reference(of, frame);
                match self.eval_player_ref(by, frame) {
                    Some(player) => vec![WorkItem::Retarget { player, entry }],
                    None => vec![],
                }
            }
            // [CR#705.1]: flip `count` coins — the draw happens in the work
            // item (the rng needs `&mut`); the applied batch fixes "that
            // many" to the number of won (called) / heads (uncalled) flips.
            Action::FlipCoins(agent, count, called) => {
                let actor = self.acting_player(agent, frame);
                let count = self.eval_count(count, frame);
                vec![WorkItem::FlipCoins {
                    player: actor,
                    count,
                    called: *called,
                }]
            }
            // [CR#121.1]: `actor` draws ONE card. Only the window is planted
            // here — the draw's `FinalizeAct` is scheduled by its OWN apply,
            // where the late-bound card and its `mark` are first known. That
            // apply binds the library top LATE and empty-checks BEFORE the
            // move, so an empty library loses the game ([CR#121.4,104.3c])
            // instead of silently no-opping.
            //
            // There is no body and no patient: drawing is irreducible
            // ([CR#121.5] — a Library → Hand move made without the word "draw"
            // is not a draw), which is why this is a footing verb here rather
            // than a `Composite` over a move. "Draw N" is `Batch(n, …)` over
            // this ([CR#121.2]); the aggregate window that a count-referring
            // replacement bites ([CR#121.2a]) is `batch_act_head`'s.
            Action::DrawCard(agent) => {
                let actor = self.acting_player(agent, frame);
                vec![WorkItem::Emit(Occurrence::single(GameEvent::Act(Act {
                    verb: deckmaste_core::VerbName::from("Draw"),
                    who: Some(actor),
                    on: vec![],
                    from: None,
                    to: None,
                    cause: Some(Cause::draw(
                        Agency::EffectInstruction,
                        Some((frame.source(self), frame.controller(self))),
                    )),
                    committed: false,
                    contents: None,
                    // A single card draw is never itself an aggregate.
                    batch: None,
                    inherited: self.activation_inherited_replacements(frame.activation),
                    // [CR#616.1g,121.2a]: set iff this draw is one of an
                    // aggregate `Batch`'s contained per-card futures.
                    contained: self.activation_contained_in_batch(frame.activation),
                })))]
            }
            // [CR#706.1]: roll `count` `sides`-sided dice — draw in the work
            // item; the applied batch fixes "that many" to the summed
            // results ([CR#706.2] — `result = natural` until the modifier
            // pipeline lands, engine-replace-roll).
            Action::RollDice(agent, count, sides) => {
                let actor = self.acting_player(agent, frame);
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
            Action::RollPlanarDie(_agent) => vec![],
            // [CR#122.1]: place/remove `n` counters of `kind` on each selected
            // object or player proxy. `n == 0` (or an empty selection) is a
            // no-op, so no event fires — a "counter is put on" trigger never
            // sees a zero placement. The cause carries the effect-instruction
            // agent so "you put a counter" reads ([CR#603.2e]-style transition
            // views) resolve to the right controller.
            Action::PutCounters(sel, kind, count) => {
                let objects = self.eval_reference_set(sel, frame);
                // [CR#608.2b]: an illegal target or departed `This` recipient
                // cannot be affected. Resolve it before a quantity that may
                // itself read that object (Heroes' Bane's power).
                if objects.is_empty() {
                    return vec![];
                }
                let n = self.eval_count(count, frame);
                if n == 0 {
                    return vec![];
                }
                let events: Vec<GameEvent> = objects
                    .into_iter()
                    .map(|object| {
                        GameEvent::CounterPlaced(CounterPlaced {
                            object,
                            // The event carries the resolved Ident name (engine
                            // state is Ident-keyed); the semantic ref is a `CounterRef`.
                            kind: kind.0,
                            amount: n,
                            // Apply-computed totals ([CR#714.2b]).
                            before: 0,
                            after: 0,
                            cause: Some(
                                crate::event::Cause::put_counters(
                                    agency,
                                    Some((frame.source(self), frame.controller(self))),
                                )
                                .with_payment(payment_id),
                            ),
                        })
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            Action::RemoveCounters(sel, kind, count) => {
                let objects = self.eval_reference_set(sel, frame);
                if objects.is_empty() {
                    return vec![];
                }
                let n = self.eval_count(count, frame);
                if n == 0 {
                    return vec![];
                }
                let events: Vec<GameEvent> = objects
                    .into_iter()
                    .map(|object| {
                        GameEvent::CounterRemoved(CounterRemoved {
                            object,
                            kind: kind.0,
                            amount: n,
                            cause: Some(
                                crate::event::Cause::remove_counters(
                                    agency,
                                    Some((frame.source(self), frame.controller(self))),
                                )
                                .with_payment(payment_id),
                            ),
                        })
                    })
                    .collect();
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            Action::AddMana(recipient, qty, production) => {
                let actor = self.acting_player(recipient, frame);
                let amount = self.eval_count(qty, frame);
                let (spec, mut riders) = match production {
                    deckmaste_core::ManaProduction::Bare(spec) => (spec, Vec::new()),
                    deckmaste_core::ManaProduction::WithRiders { mana, riders } => {
                        (mana, riders.to_vec())
                    }
                };
                // [CR#107.4h]: mana from a snow source carries `Snow`
                // provenance regardless of the riders the ability text
                // declares — snow-ness is a property of the producing source,
                // not the effect.
                riders.extend(self.snow_provenance(frame.source(self)));
                let provenance = crate::player::ManaProvenance {
                    source: Some(frame.source(self)),
                    action: self.resolving_mana_actions.last().copied(),
                };
                match spec {
                    // A fixed production needs no choice.
                    ManaSpec::Specific(mana) => {
                        vec![WorkItem::Emit(Occurrence::Single(GameEvent::ManaAdded(
                            ManaAdded {
                                player: actor,
                                mana: *mana,
                                amount,
                                riders,
                                provenance,
                                units: Vec::new(),
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
                        provenance,
                    }],
                    ManaSpec::OneOf(options) => vec![WorkItem::ChooseManaColor {
                        player: actor,
                        options: options.to_vec(),
                        amount,
                        riders,
                        provenance,
                    }],
                    // [CR#106.1b]: the filterland cycle — the actor picks one
                    // of several multi-symbol runs on resolution, then that
                    // run's whole sequence of mana is produced.
                    ManaSpec::OneOfRuns(options) => vec![WorkItem::ChooseManaMode {
                        player: actor,
                        options: options.to_vec(),
                        amount,
                        riders,
                        provenance,
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
                        // color; a semantic/zone-changed mismatch fizzles
                        // like any other stale reference).
                        if options.is_empty() {
                            vec![]
                        } else {
                            vec![WorkItem::ChooseManaColor {
                                player: actor,
                                options,
                                amount,
                                riders,
                                provenance,
                            }]
                        }
                    }
                    ManaSpec::ProducedByEvent => {
                        let mut options = Vec::new();
                        for mana in &self.activation_produced_mana(frame.activation) {
                            if !options.contains(mana) {
                                options.push(*mana);
                            }
                        }
                        match options.as_slice() {
                            [] => vec![],
                            [mana] => vec![WorkItem::Emit(Occurrence::Single(
                                GameEvent::ManaAdded(ManaAdded {
                                    player: actor,
                                    mana: *mana,
                                    amount,
                                    riders,
                                    provenance,
                                    units: Vec::new(),
                                }),
                            ))],
                            _ => vec![WorkItem::ChooseManaColor {
                                player: actor,
                                options,
                                amount,
                                riders,
                                provenance,
                            }],
                        }
                    }
                }
            }
            Action::Create {
                agent,
                count: qty,
                token: spec,
                riders,
            } => {
                let actor = self.acting_player(agent, frame);
                // `AsCopy` riders fizzle here too — see
                // `crate::copy::has_unbuilt_enter_rider`. (The dedicated
                // token-copy spelling is `TokenSpec::Copy` below, already
                // fully wired; an `AsCopy` rider alongside it would be a
                // redundant, never-crash-safe semantic input.)
                if crate::copy::has_unbuilt_enter_rider(riders) {
                    todo!(
                        "engine seam: token enter riders ([CR#708]) — face-down arrival has \
                         grammar but no execution; owner: engine-face-down"
                    );
                }
                // A created token always enters the battlefield ([CR#614.12]
                // doc comment on `Action::Create`), so every rider is
                // computed once and shared by every minted copy — the
                // creating player IS the default controller and the owner
                // ([CR#111.2]), so both `enter_status_from_riders` seeds are
                // `actor`.
                let enters = if riders.is_empty() {
                    None
                } else {
                    Some(crate::copy::enter_status_from_riders(
                        self, frame, riders, actor, actor,
                    ))
                };
                // [CR#701.7a]: one instruction puts all N tokens onto the
                // battlefield — one simultaneous batch of `TokenCreated`
                // facts. (Token copies — `Create` of a copy-defined token —
                // wait on the copy grammar, `core-copy-grammar`.) The FACT
                // carries the resolved inline definition; a `TokenSpec::Named`
                // predefined token ([CR#111.10]) resolves to its rules-defined
                // characteristics here.
                let token = match spec {
                    deckmaste_core::TokenSpec::Token(token) => (**token).clone(),
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
                            enters: enters.clone(),
                        })
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            Action::GetDesignation(recipient, name) => {
                let actor = self.acting_player(recipient, frame);
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
            Action::SetGameDesignation(name, value) => {
                if self.designations.game.get(name).is_some_and(
                    |current| matches!(current, crate::state::DesignationValue::Mode(v) if v == value),
                ) {
                    vec![]
                } else {
                    vec![WorkItem::Emit(Occurrence::Single(
                        GameEvent::DesignationChanged(DesignationChanged {
                            name: *name,
                            becomes: Some(*value),
                        }),
                    ))]
                }
            }
            // [CR#614.8,701.19a]: remove all marked damage from each selected
            // object and remove it from combat if it's attacking or blocking.
            // This is the regeneration "heal" clause — its apply zeroes damage
            // and calls `combat.remove_object`.
            Action::RemoveDamage(sel) => {
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
            // [CR#118.12] spec §14.1: bare effect-position `Pay` is
            // unspellable — the slotless well-formedness gate is a
            // cards-layer validation lint, not the parser. If one somehow
            // still reaches here, fizzle rather than crash (this is a
            // guarded seam, never a live path that used to work).
            Action::Pay(_) => vec![],
            // Provenance is erased at `lower` (`deckmaste_lowering`), so no
            // loaded value reaches here wrapped. The arm survives only because
            // the variant does; `core-demacro` deletes both.
            // `action_items` dispatches every object/effect-agent verb
            // directly and only falls through to this function for the
            // former-`PlayerAction` family — these variants never reach here.
            other @ (Action::DealDamage(..)
            | Action::Counter(_)
            | Action::Transform(_)
            | Action::Cease(_)
            | Action::Attach { .. }
            | Action::Unattach(_)
            | Action::Move(..)
            | Action::MoveGroup { .. }
            | Action::GainControl(..)
            | Action::ExtraPhase(..)
            | Action::MoveCounters(..)
            | Action::CreateReplacement { .. }
            | Action::Composite { .. }) => {
                unreachable!(
                    "action_items dispatches object verbs directly; {other:?} never reaches \
                     player_action_items"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::empty_line_after_doc_comments,
        reason = "related behavioral test rationale is intentionally grouped"
    )]

    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_core::Action;
    use deckmaste_core::ChosenValueKind;
    use deckmaste_core::Count;
    use deckmaste_core::Instruction;
    use deckmaste_core::ObjectClass;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Type;
    use deckmaste_core::Uint;
    use deckmaste_core::Zone;

    use crate::DecisionPointKind;
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

    /// `EnterRider::AsCopy` fizzles instead of tripping the
    /// `core-action-riders-cost-modes` `todo!()` ([CR#707.5]) — moving a
    /// hand card onto the battlefield with an `AsCopy` rider completes the
    /// move (the copy itself isn't installed yet; that's
    /// `engine-layers-1-copy-facedown-text`'s job, `crate::copy::
    /// has_unbuilt_enter_rider`'s seam) rather than panicking, proving the
    /// fizzle is genuinely safe on this card-reachable path.
    #[test]
    fn move_as_copy_rider_fizzles_without_panicking() {
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::Destination;
        use deckmaste_core::EnterRider;

        let (mut state, _bear) = bear_on_field();
        let second = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        let before = state.zones.battlefield.len();
        let frame = frame_src(&state, second);
        let spec = CopySpec {
            source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(0))),
            exceptions: vec![],
        };
        state.run_effect(
            Instruction::act(Action::Move(
                Reference::Reg(deckmaste_core::RefId(0)),
                Destination::Zone(Zone::Battlefield),
                vec![EnterRider::AsCopy(spec)].into(),
                None,
            )),
            &frame,
        );
        // Never panics — drain the agenda like the other Move tests
        // (future-form ZoneChange -> past-form ZoneChange).
        for _ in 0..3 {
            let _ = state.step();
        }
        assert_eq!(
            state.zones.battlefield.len(),
            before + 1,
            "the move completes: a second permanent lands on the battlefield \
             even though the AsCopy rider isn't applied yet"
        );
    }

    /// Move `object` onto the battlefield with `riders` and drain the agenda,
    /// returning the NEW object id it became (a `Move` remints, [CR#400.7]).
    /// Panics if no new battlefield member appeared.
    fn move_onto_battlefield_with_riders(
        state: &mut GameState,
        object: ObjectId,
        riders: Vec<deckmaste_core::EnterRider>,
    ) -> ObjectId {
        use deckmaste_core::Destination;

        let before: std::collections::HashSet<ObjectId> =
            state.zones.battlefield.iter().copied().collect();
        let frame = frame_src(state, object);
        state.run_effect(
            Instruction::act(Action::Move(
                Reference::Reg(deckmaste_core::RefId(0)),
                Destination::Zone(Zone::Battlefield),
                riders.into(),
                None,
            )),
            &frame,
        );
        for _ in 0..3 {
            let _ = state.step();
        }
        *state
            .zones
            .battlefield
            .iter()
            .find(|o| !before.contains(o))
            .expect("a new object landed on the battlefield")
    }

    /// `EnterRider::Tapped` ([CR#603.6d]) executes at `Action::Move`: the
    /// [CR#603.6d]-style "search library, put onto the battlefield tapped"
    /// shape lands the permanent already tapped, mechanically — no combat,
    /// no controller change.
    #[test]
    fn move_tapped_rider_taps_the_entering_permanent() {
        use deckmaste_core::EnterRider;

        let (mut state, _bear) = bear_on_field();
        let second = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        let new = move_onto_battlefield_with_riders(&mut state, second, vec![EnterRider::Tapped]);
        assert!(
            state.objects.obj(new).tapped,
            "the Tapped rider taps the entering permanent"
        );
    }

    /// `EnterRider::WithCounters` ([CR#122.6a,614.12]) places the named
    /// counters atomically at mint — the reanimation "+1/+1 counter on it"
    /// shape, executed via `Action::Move` (Otherworldly Journey's delayed
    /// "return that card to the battlefield ... with a +1/+1 counter on it"
    /// exercises the exact same rider structurally).
    #[test]
    fn move_with_counters_rider_places_counters_atomically() {
        use deckmaste_core::CounterRef;
        use deckmaste_core::EnterRider;
        use deckmaste_core::Ident;

        let (mut state, _bear) = bear_on_field();
        let second = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        let new = move_onto_battlefield_with_riders(
            &mut state,
            second,
            vec![EnterRider::WithCounters(
                CounterRef::from("P1P1Counter"),
                Count::Literal(2),
            )],
        );
        assert_eq!(
            state
                .objects
                .obj(new)
                .counters
                .get(&Ident::from("P1P1Counter"))
                .copied(),
            Some(2),
            "the WithCounters rider places 2 +1/+1 counters at mint"
        );
    }

    /// `EnterRider::UnderControlOf` ([CR#110.2a]) overrides the mint-time
    /// controller to the named player — the Cloudshift-style "return that
    /// card to the battlefield under your control" shape, where "your"
    /// names a SPECIFIC player rather than defaulting to the mover.
    #[test]
    fn move_under_control_of_rider_overrides_controller() {
        use deckmaste_core::EnterRider;

        let (mut state, _bear) = bear_on_field();
        let second = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        let new = move_onto_battlefield_with_riders(
            &mut state,
            second,
            vec![EnterRider::UnderControlOf(Reference::OpponentOf(
                std::sync::Arc::new(Reference::Reg(deckmaste_core::RefId(1))),
            ))],
        );
        assert_eq!(
            state.objects.obj(new).controller,
            PlayerId(1),
            "UnderControlOf(Opponent) gives the entering permanent to player 1, \
             not its owner (player 0)"
        );
    }

    /// `EnterRider::UnderOwnersControl` ([CR#110.2a]) — the blink/reanimation
    /// "under its owner's control" wording, spelled without naming a player.
    /// Owner and mover coincide in this fixture (the card's owner IS the
    /// mover), so this pins the rider is READ at all, not that it differs
    /// from the ordinary default.
    #[test]
    fn move_under_owners_control_rider_sets_controller_to_owner() {
        use deckmaste_core::EnterRider;

        let (mut state, _bear) = bear_on_field();
        let second = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        let owner = state.owner_of(second);
        let new = move_onto_battlefield_with_riders(
            &mut state,
            second,
            vec![EnterRider::UnderOwnersControl],
        );
        assert_eq!(
            state.objects.obj(new).controller,
            owner,
            "UnderOwnersControl sets the entering permanent's controller to its owner"
        );
    }

    /// `EnterRider::Attacking(Some(_))` ([CR#508.4]) folds the entering
    /// creature into the current combat's attacker set against the named
    /// target — the Ninjutsu-style "put onto the battlefield tapped and
    /// attacking" shape. No `GameEvent::Attacking` fires for it ([CR#508.4]:
    /// "they never attacked"), so pairing WITHOUT a `Tapped` rider here also
    /// pins that no automatic declaration-tap sneaks in.
    #[test]
    fn move_attacking_rider_declares_the_entering_creature_an_attacker() {
        use deckmaste_core::EnterRider;

        let (mut state, _bear) = bear_on_field();
        let second = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        let defender = state.player(PlayerId(1)).object;
        let new = move_onto_battlefield_with_riders(
            &mut state,
            second,
            vec![EnterRider::Attacking(Some(Reference::OpponentOf(
                std::sync::Arc::new(Reference::Reg(deckmaste_core::RefId(1))),
            )))],
        );
        assert!(
            state.combat.is_attacking(new),
            "the Attacking rider declares the entering creature an attacker"
        );
        assert_eq!(
            state.combat.target_of(new),
            Some(defender),
            "it attacks the named target (the opponent)"
        );
        assert!(
            !state.objects.obj(new).tapped,
            "entering attacking does NOT itself tap the creature ([CR#508.4] is \
             distinct from the [CR#508.1f] declaration-tap) — only an explicit \
             Tapped rider would"
        );
    }

    /// `EnterRider::Attacking(None)` — the target left unspecified, so
    /// [CR#508.4] has the entering creature's controller choose. This
    /// engine's fixed 2-player field makes that choice forced: the sole
    /// legal defending player is the entering controller's opponent.
    #[test]
    fn move_attacking_rider_with_no_target_defaults_to_the_sole_opponent() {
        use deckmaste_core::EnterRider;

        let (mut state, _bear) = bear_on_field();
        let second = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a second Grizzly Bears in the opening hand");
        let defender = state.player(PlayerId(1)).object;
        let new = move_onto_battlefield_with_riders(
            &mut state,
            second,
            vec![EnterRider::Attacking(None)],
        );
        assert_eq!(
            state.combat.target_of(new),
            Some(defender),
            "an unspecified Attacking target defaults to the sole opponent"
        );
    }

    /// `Action::Create` folds its own rider list the same way `Action::Move`
    /// does ([CR#508.4,614.12]) — "create a token tapped" mints it already
    /// tapped, proving the token-minting call site is wired too, not just
    /// the zone-move one.
    #[test]
    fn create_tapped_rider_taps_the_minted_token() {
        use deckmaste_core::EnterRider;

        let (mut state, bear) = bear_on_field();
        let frame = frame_src(&state, bear);
        let token = deckmaste_core::Token {
            name: Some("Test Token".into()),
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Creature.def()].into(),
            subtypes: vec![].into(),
            abilities: vec![].into(),
            power: None,
            toughness: None,
        };
        let before: std::collections::HashSet<ObjectId> =
            state.zones.battlefield.iter().copied().collect();
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: token.into(),
                riders: vec![EnterRider::Tapped].into(),
            }),
            &frame,
        );
        for _ in 0..3 {
            let _ = state.step();
        }
        let new = *state
            .zones
            .battlefield
            .iter()
            .find(|o| !before.contains(o))
            .expect("the created token landed on the battlefield");
        assert!(
            state.objects.obj(new).tapped,
            "Action::Create's Tapped rider taps the minted token"
        );
    }

    /// [CR#115.7a..115.7d]: `mode`'s four-way discriminant is now branched
    /// explicitly. Only `ChooseNew` ([CR#115.7d]) is exercised by canon or
    /// testing fixtures, and it alone schedules the retarget work item
    /// (unchanged behavior). `ChangeAll`/`ChangeOne`/`ChangeAny`
    /// ([CR#115.7a..115.7c]) each need their own legal-set derivation that
    /// isn't built — running `ChooseNew`'s under their name would be
    /// silently wrong, so they fizzle (no work item) instead.
    #[test]
    fn retarget_only_choose_new_mode_is_wired_the_rest_fizzle() {
        use deckmaste_core::RetargetMode;

        let (state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        for mode in [
            RetargetMode::ChangeAll,
            RetargetMode::ChangeOne,
            RetargetMode::ChangeAny,
        ] {
            let act = Action::Retarget {
                mode: mode.clone(),
                of: Reference::Reg(deckmaste_core::RefId(0)),
                by: Reference::Reg(deckmaste_core::RefId(1)),
            };
            let items = state.player_action_items(&act, &frame);
            assert!(
                items.is_empty(),
                "{mode:?} is an unwired discriminant — fizzles rather than \
                 silently running ChooseNew's derivation"
            );
        }
        let act = Action::Retarget {
            mode: RetargetMode::ChooseNew,
            of: Reference::Reg(deckmaste_core::RefId(0)),
            by: Reference::Reg(deckmaste_core::RefId(1)),
        };
        let items = state.player_action_items(&act, &frame);
        assert_eq!(items.len(), 1, "ChooseNew still schedules the retarget");
        assert!(matches!(items[0], WorkItem::Retarget { .. }));
    }

    /// [CR#607.2]: `ChooseValue(who, Color, key)` is grammar-spellable, so a
    /// card reaching it is a live path — no reader grammar exists for a
    /// chosen color yet, so it must fizzle (no work item, nothing noted)
    /// rather than panic. `CardName`/`Number` keep their existing readers.
    #[test]
    fn choose_value_color_fizzles_no_reader_grammar_yet() {
        let (state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let act = Action::ChooseValue(
            Reference::Reg(deckmaste_core::RefId(1)),
            ChosenValueKind::Color,
            deckmaste_core::Ident::from("k"),
        );
        let items = state.player_action_items(&act, &frame);
        assert!(
            items.is_empty(),
            "ChooseValue(Color) fizzles — no reader grammar exists yet"
        );
    }

    /// `AddMana(2, Green)` needs no choice and lands in the pool ([CR#106.4]);
    /// `AddMana(1, AnyColor)` surfaces `ChooseManaColor` with the five colors
    /// — colorless is not a color ([CR#105.4]) and is rejected.
    #[test]
    fn add_mana_specific_and_any_color() {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaSpec;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let green = ColorOrColorless::Color(Color::Green);
        state.run_effect(
            Instruction::act(Action::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(2),
                ManaSpec::Specific(green).into(),
            )),
            &frame,
        );
        let _ = state.step();
        assert_eq!(state.players[0].mana_pool.amount(green), 2);

        state.run_effect(
            Instruction::act(Action::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::AnyColor.into(),
            )),
            &frame,
        );
        let _ = state.step(); // ManaColorOpened
        let StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaColor(
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
                .is_err_and(|err| {
                    err.to_string()
                        .contains("not one of the offered mana options")
                }),
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
        use crate::decide::DecisionPointKind;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let white = ColorOrColorless::Color(Color::White);
        let blue = ColorOrColorless::Color(Color::Blue);
        state.run_effect(
            Instruction::act(Action::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::OneOfRuns(
                    vec![vec![white, white], vec![white, blue], vec![blue, blue]].into(),
                )
                .into(),
            )),
            &frame,
        );
        let _ = state.step(); // ManaModeOpened
        let StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaMode(
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
            state
                .submit_decision(Decision::ManaMode(3))
                .is_err_and(|err| err.to_string().contains("not one of the offered mana runs")),
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
        let frame = frame_src(&state, src);
        let red = ColorOrColorless::Color(Color::Red);
        let rider = ManaRider::SpendOnly(Predicate::Any);
        state.run_effect(
            Instruction::act(Action::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaProduction::WithRiders {
                    mana: ManaSpec::Specific(red),
                    riders: vec![rider].into(),
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
    /// REFERENCED object (`imprinted`, reached via its explicit target
    /// register) goes away, mirroring how an imprinted/exiled card can
    /// cease independently of the producing permanent.

    /// [CR#701.9b]: `Discard(2)` surfaces the card choice; a wrong-sized answer
    /// is rejected; the right answer discards through the Hand→Graveyard
    /// pipeline. Discarding more than the hand holds clamps to the whole hand
    /// ([CR#101.3]).
    #[test]
    fn discard_surfaces_choice_validates_and_clamps() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let hand_before = state.zones.hands[0].len();
        state.run_effect(
            Instruction::act(Action::discard(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(2),
                false,
            )),
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
        let DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
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
            state
                .submit_decision(Decision::Chosen(one))
                .is_err_and(|err| err.to_string().contains("illegal object selection")),
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
            Instruction::act(Action::discard(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(99),
                false,
            )),
            &frame,
        );
        let pending = (0..5)
            .find_map(|_| match state.step() {
                StepOutcome::NeedsDecision(p) => Some(p),
                _ => None,
            })
            .expect("a ChooseObjects decision surfaces within a few steps");
        let DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects { max, .. }) =
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

    /// [CR#701.7a,111.2]: `Create(2, token)` puts two token permanents onto
    /// the battlefield under the creator — owned by them, summoning-sick,
    /// kind `Token` (not `Card`, [CR#111.6]) — as ONE simultaneous batch of
    /// `TokenCreated` facts, each followed by its past-form `ZoneChange
    /// { from: None, to: Battlefield, .. }` fact.
    #[test]
    fn create_tokens_enter_battlefield() {
        use deckmaste_core::Token;

        let (mut state, src) = bear_on_field();
        let frame = frame_src(&state, src);
        let token = Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![].into(),
            abilities: vec![].into(),
            power: None,
            toughness: None,
        };
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(2),
                token: token.into(),
                riders: vec![].into(),
            }),
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
            assert!(crate::target::is_object_class(
                &state,
                t,
                ObjectClass::Token
            ));
            // A token on the battlefield is a Token AND a Permanent
            // ([CR#111.1,110.1]), and is NOT a Card ([CR#108.2b]) — the
            // classes are independently testable and overlap.
            assert!(crate::target::is_object_class(
                &state,
                t,
                ObjectClass::Permanent
            ));
            assert!(!crate::target::is_object_class(
                &state,
                t,
                ObjectClass::Card
            ));
            assert!(obj_matches(
                &state,
                t,
                &Predicate::And(std::sync::Arc::from([
                    Predicate::Class(ObjectClass::Token),
                    Predicate::Class(ObjectClass::Permanent),
                ]))
            ));
            assert_eq!(
                state.owner_of(t),
                PlayerId(0),
                "[CR#111.2]: creator owns it"
            );
            assert_eq!(state.objects.obj(t).controller, PlayerId(0));
            assert!(state.objects.obj(t).summoning_sick, "[CR#302.6]");
            assert!(
                obj_matches(&state, t, &Predicate::r#type(Type::Artifact)),
                "the creating effect's characteristics stick ([CR#111.3])"
            );
        }
        // The enter facts follow as ONE batch occurrence ([CR#603.2c] — the
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
        let frame = frame_src(&state, src);
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Named(deckmaste_core::TokenName::from(
                    "Treasure",
                )),
                riders: vec![].into(),
            }),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the Treasure token on the battlefield");
        assert!(crate::target::is_object_class(
            &state,
            t,
            ObjectClass::Token
        ));
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
        let frame = frame_src(&state, src);
        let treasure = builtin().token("Treasure").unwrap().core;
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: treasure.into(),
                riders: vec![].into(),
            }),
            &frame,
        );
        let _ = state.step(); // the TokenCreated batch applies
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the Treasure token on the battlefield");
        assert!(crate::target::is_object_class(
            &state,
            t,
            ObjectClass::Token
        ));
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
            &*crate::derive::face(&state.cards.get(card).def)
                .characteristics
                .name,
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
        let frame = frame_src_targets(&state, a, vec![b]);
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Copy(
                    CopySpec {
                        source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                        exceptions: vec![],
                    }
                    .into(),
                ),
                riders: vec![].into(),
            }),
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
        assert!(crate::target::is_object_class(
            &state,
            t,
            ObjectClass::Token
        ));
        let card = state.objects.obj(t).card_id().expect("card-backed");
        let face = crate::derive::face(&state.cards.get(card).def);
        assert_eq!(
            &*face.characteristics.name, "Grizzly Bears",
            "[CR#707.2]: name matches the copied source EXACTLY (Spitting \
             Image example) — not resynthesized to \"Bear Token\""
        );
        assert_eq!(
            face.characteristics.power,
            Some(StatValue::Number(2)),
            "[CR#707.2]: power matches the copied source"
        );
        assert_eq!(
            face.characteristics.toughness,
            Some(StatValue::Number(2)),
            "[CR#707.2]: toughness matches the copied source"
        );
        assert!(
            obj_matches(&state, t, &Predicate::r#type(Type::Creature)),
            "[CR#707.2]: types match the copied source"
        );
        assert!(
            face.characteristics
                .subtypes
                .iter()
                .any(|s| s.name == "Bear"),
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
        let frame = frame_src_targets(&state, a, vec![dead]);
        assert_eq!(
            state.action_items(
                &Action::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: deckmaste_core::TokenSpec::Copy(
                        CopySpec {
                            source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                            exceptions: vec![],
                        }
                        .into()
                    ),
                    riders: vec![].into(),
                },
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
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;

        let (mut state, a) = bear_on_field();
        let bolt = mint_on_field(
            &mut state,
            Card::Normal(CardFace::from(Characteristics {
                name: "Lightning Bolt".into(),
                types: vec![Type::Instant.def()],
                ..Characteristics::default()
            })),
        );
        let frame = frame_src_targets(&state, a, vec![bolt]);
        assert_eq!(
            state.action_items(
                &Action::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: deckmaste_core::TokenSpec::Copy(
                        CopySpec {
                            source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                            exceptions: vec![],
                        }
                        .into()
                    ),
                    riders: vec![].into(),
                },
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
        let frame = frame_src_targets(&state, a, vec![b]);
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Copy(
                    CopySpec {
                        source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                        exceptions: vec![
                            CopyException::Modify(Modification::Power(NumericOp::Set(
                                StatValue::Number(4),
                            ))),
                            CopyException::Modify(Modification::Toughness(NumericOp::Set(
                                StatValue::Number(4),
                            ))),
                        ],
                    }
                    .into(),
                ),
                riders: vec![].into(),
            }),
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
            face.characteristics.power,
            Some(StatValue::Number(4)),
            "[CR#707.9d]: Modify(Power Set 4) overrides the copied power"
        );
        assert_eq!(
            face.characteristics.toughness,
            Some(StatValue::Number(4)),
            "[CR#707.9d]: Modify(Toughness Set 4) overrides the copied toughness"
        );
    }

    // ====================================================================
    // core-copy-grammar Task 9 — the copy-CONSUMING keyword macros
    // (Populate/Embalm/Eternalize/Offspring) drive Task 3's landed token-copy
    // execution. Each behavioral test extracts the token-minting `Create`
    // effect from the EXPANDED macro (so the macro→CopySpec mapping is proven
    // too, not just the runtime), then drives it through the resolve path and
    // asserts the minted token's characteristics ([CR#707.1,707.9]). Graveyard
    // activation for Embalm/Eternalize is exercised at the effect level (the
    // `SelfCard` source reads the on-battlefield fixture's printed face — the
    // full exile-then-activate flow is disproportionate and the copy semantics
    // are identical either way, since the exiled card's copiable values are its
    // printed face here [CR#707.2]).
    // ====================================================================

    /// Expand a copy-consuming keyword macro and pull out the region
    /// that mints the token copy — the Activated ability's effect for the
    /// graveyard keywords (Embalm/Eternalize), the ETB Triggered ability's for
    /// Offspring. Proves the macro really produces a copy `Create` at all.
    fn keyword_copy_effect(invocation: &str) -> deckmaste_core::Region {
        use deckmaste_core::KeywordAbility;
        use deckmaste_lowering::Lower;

        let semantic: deckmaste_semantics::KeywordAbility =
            builtin().macros.read_str(invocation).unwrap();
        let kw: KeywordAbility = semantic.lower();
        let KeywordAbility::Composite { abilities, .. } = kw else {
            panic!("expected a lowered Composite body, got {kw:?}")
        };
        if let Some(activated) = abilities.iter().find_map(|ability| ability.as_activated()) {
            assert!(
                activated.cost.iter().any(|component| matches!(
                    component,
                    deckmaste_core::CostComponent::Act { action, .. }
                        if matches!(
                            action.as_action(),
                            Action::Move(
                                Reference::Reg(deckmaste_core::RefId(0)),
                                deckmaste_core::Destination::Zone(Zone::Exile),
                                _,
                                _
                            )
                        )
                )),
                "the lowered graveyard-keyword cost binds exile to This"
            );
            assert!(
                activated.cost.iter().all(|component| !matches!(
                    component,
                    deckmaste_core::CostComponent::Choose(_)
                        | deckmaste_core::CostComponent::Sample(_)
                        | deckmaste_core::CostComponent::Search(_)
                )),
                "the bound exile cost needs no payment-time decision ([CR#601.2b])"
            );
        }
        abilities
            .iter()
            .find_map(|a| {
                a.as_activated()
                    .map(|act| act.effect.clone())
                    .or_else(|| a.as_triggered().map(|trig| trig.effect.clone()))
            })
            .expect("the copy-minting effect (Activated for Embalm/Eternalize, Triggered for Offspring)")
    }

    fn run_test_region(
        state: &mut GameState,
        region: &deckmaste_core::Region,
        mut frame: crate::stack::ExecutionFrame,
    ) {
        frame.activation = state.enter_region(region, &frame);
        for instruction in region.body.iter().cloned() {
            state.run_effect(instruction, &frame);
        }
    }

    /// The lone token minted onto the battlefield by the effect just run —
    /// its resolved printed face, for the copy-characteristic assertions.
    fn minted_copy_face<'a>(
        state: &'a GameState,
        exclude: &[ObjectId],
    ) -> &'a deckmaste_card::CardFace {
        let &t = state
            .zones
            .battlefield
            .iter()
            .find(|&id| !exclude.contains(id))
            .expect("a freshly minted copy token on the battlefield");
        assert!(
            crate::target::is_object_class(state, t, ObjectClass::Token),
            "[CR#111.1]: the copy is a token"
        );
        let card = state.objects.obj(t).card_id().expect("card-backed");
        crate::derive::face(&state.cards.get(card).def)
    }

    /// Eternalize ([CR#702.129a]): the macro's activated-ability effect mints a
    /// token copy of the exiled card EXCEPT it's a 4/4 black Zombie — name
    /// carried from the source, P/T 4/4, black, and a Zombie in addition to its
    /// other types ([CR#707.9d]).
    #[test]
    fn eternalize_macro_mints_a_4_4_black_zombie_copy() {
        use deckmaste_core::Color;
        use deckmaste_core::StatValue;

        let (mut state, src) = bear_on_field();
        let effect = keyword_copy_effect("Eternalize([Mana([Generic(4),Black])])");
        let frame = frame_src(&state, src);
        run_test_region(&mut state, &effect, frame);
        let _ = state.step(); // the TokenCreated batch applies

        let face = minted_copy_face(&state, &[src]);
        assert_eq!(
            &*face.characteristics.name, "Grizzly Bears",
            "[CR#707.2]: the copy keeps the source's name"
        );
        assert_eq!(
            face.characteristics.power,
            Some(StatValue::Number(4)),
            "[CR#702.129a,707.9d]: eternalize copy is 4/4"
        );
        assert_eq!(face.characteristics.toughness, Some(StatValue::Number(4)));
        assert!(
            face.characteristics.color_indicator.contains(&Color::Black),
            "[CR#702.129a]: eternalize copy is black; got {:?}",
            face.characteristics.color_indicator
        );
        assert!(
            face.characteristics
                .subtypes
                .iter()
                .any(|s| s.name == "Zombie"),
            "[CR#702.129a]: eternalize copy is a Zombie in addition to its other types; got {:?}",
            face.characteristics.subtypes
        );
    }

    /// Embalm ([CR#702.128a]): the macro's activated-ability effect mints a
    /// token copy EXCEPT it's a white Zombie — the source's printed P/T rides
    /// through unchanged (no P/T exception), only color and subtype are set.
    #[test]
    fn embalm_macro_mints_a_white_zombie_copy_retaining_pt() {
        use deckmaste_core::Color;
        use deckmaste_core::StatValue;

        let (mut state, src) = bear_on_field();
        let effect = keyword_copy_effect("Embalm([Mana([Generic(3),White])])");
        let frame = frame_src(&state, src);
        run_test_region(&mut state, &effect, frame);
        let _ = state.step();

        let face = minted_copy_face(&state, &[src]);
        assert_eq!(
            &*face.characteristics.name, "Grizzly Bears",
            "[CR#707.2]: name carried"
        );
        assert_eq!(
            face.characteristics.power,
            Some(StatValue::Number(2)),
            "[CR#702.128a]: embalm keeps the source's P/T (2/2)"
        );
        assert_eq!(face.characteristics.toughness, Some(StatValue::Number(2)));
        assert!(
            face.characteristics.color_indicator.contains(&Color::White),
            "[CR#702.128a]: embalm copy is white; got {:?}",
            face.characteristics.color_indicator
        );
        assert!(
            face.characteristics
                .subtypes
                .iter()
                .any(|s| s.name == "Zombie"),
            "[CR#702.128a]: embalm copy is a Zombie; got {:?}",
            face.characteristics.subtypes
        );
    }

    /// Offspring ([CR#702.175a]): the macro's ETB-triggered effect (the branch
    /// taken when the offspring cost was paid) mints a 1/1 token copy of the
    /// entering permanent ([CR#707.9d]).
    #[test]
    fn offspring_macro_mints_a_1_1_copy() {
        use deckmaste_core::StatValue;

        let (mut state, src) = bear_on_field();
        let effect = keyword_copy_effect("Offspring([Mana([Generic(1)])])");
        let frame = frame_src(&state, src);
        run_test_region(&mut state, &effect, frame);
        let _ = state.step();

        let face = minted_copy_face(&state, &[src]);
        assert_eq!(
            &*face.characteristics.name, "Grizzly Bears",
            "[CR#707.2]: name carried"
        );
        assert_eq!(
            face.characteristics.power,
            Some(StatValue::Number(1)),
            "[CR#702.175a]: offspring copy is 1/1"
        );
        assert_eq!(face.characteristics.toughness, Some(StatValue::Number(1)));
    }

    /// Populate ([CR#701.36a]): drives the FULL `Populate` macro expansion —
    /// `With(ChooseOne(creature-token-you-control), Create(1,
    /// Copy(Object(That))))` — end to end. Mints a real creature token,
    /// answers the surfaced `ChooseObjects` with it, and asserts the minted
    /// copy matches. This proves the macro RESOLVES: a `With` one-binder
    /// binds its pick as `Reference::That`, which lowering resolves to the
    /// deciding instruction's own dest register, so the copy source
    /// reads `Object(That)` — `Object(It)` would fizzle unbound. Also proves
    /// the token-source copy path (`copiable_values` over a `push_token`
    /// face, [CR#111.4]) and the "you control"/"a token" candidate filter.
    #[test]
    fn populate_macro_copies_the_chosen_creature_token() {
        use deckmaste_core::StatValue;
        use deckmaste_core::Subtype;
        use deckmaste_core::Token;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, src) = bear_on_field();
        // Mint the creature token to be populated (a 2/2 Bear token you
        // control).
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: Token {
                    name: None,
                    color_indicator: vec![].into(),
                    supertypes: vec![].into(),
                    types: vec![Type::Creature.def()].into(),
                    subtypes: vec![Subtype {
                        name: "Bear".into(),
                        types: vec![Type::Creature].into(),
                        confers: vec![].into(),
                    }]
                    .into(),
                    abilities: vec![].into(),
                    power: Some(StatValue::Number(2)),
                    toughness: Some(StatValue::Number(2)),
                }
                .into(),
                riders: vec![].into(),
            }),
            &frame_src(&state, src),
        );
        run_injected(&mut state);
        let &token = state
            .zones
            .battlefield
            .iter()
            .find(|&&id| id != src)
            .expect("the creature token to populate");

        // Drive the ACTUAL macro expansion, through the SEMANTICS path
        // (`semantics::Instruction` → `lower()`), the path production now
        // takes.
        let semantic: deckmaste_semantics::OneShotEffect =
            builtin().macros.read_str("Populate").unwrap();
        let frame = frame_src(&state, src);
        schedule_lowered_effect(&mut state, semantic, 0, &frame);

        // The `With(ChooseOne(...))` surfaces the pick.
        let _ = state.step();
        let Some(DecisionPointKind::ChooseObjects(choose)) = state.pending.clone() else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert!(
            choose.candidates.contains(&token),
            "[CR#701.36a]: the creature token is a populate candidate"
        );
        assert!(
            !choose.candidates.contains(&src),
            "[CR#701.36a]: a non-token creature (the source Grizzly Bears) is NOT a populate candidate"
        );
        state
            .submit_decision(Decision::Chosen(vec![token]))
            .expect("choosing the creature token is legal");

        // Pump to completion; the copy token mints.
        for _ in 0..30 {
            let _ = state.step();
            if state.pending.is_some() {
                break;
            }
        }

        let face = minted_copy_face(&state, &[src, token]);
        assert_eq!(
            &*face.characteristics.name, "Bear Token",
            "[CR#701.36a,111.4]: the copy of an unnamed creature token carries its \
             synthesized name (subtypes + \"Token\")"
        );
        assert_eq!(
            face.characteristics.power,
            Some(StatValue::Number(2)),
            "[CR#701.36a]: populate copy matches the source token's P/T"
        );
        assert_eq!(face.characteristics.toughness, Some(StatValue::Number(2)));
        assert!(
            face.characteristics
                .subtypes
                .iter()
                .any(|s| s.name == "Bear"),
            "[CR#701.36a]: populate copy matches the source token's subtypes; got {:?}",
            face.characteristics.subtypes
        );
    }

    /// Amass ([CR#701.47a]): drives the FULL `Amass` macro expansion —
    /// `With(ChooseOne(Army creature you control), Sequentially([PutCounters(
    /// That, +1/+1, N), If(Not(Matches(That, subtype)), Continuously(Modify(
    /// That, Add subtype), EndOfGame))]))` — end to end. Mints an Army
    /// creature, answers the surfaced `ChooseObjects` with it, and asserts
    /// the chosen creature ends with the N +1/+1 counters AND gains the
    /// amassed subtype.
    ///
    /// This proves the macro RESOLVES, catching a bug the expansion-only test
    /// (`deckmaste_plugin::builtin::amass_decomposes_into_core_primitives`)
    /// can't see: a `With` one-binder binds its pick as `Reference::That`,
    /// never `It`, so all three body clauses must read `That`; `It` would
    /// fizzle unbound, placing no counters and adding no subtype. It also
    /// proves the one-shot `Continuously(Modify(That, ...))` captures the
    /// resolved object id at CREATION (a `Locked` scope stamped by
    /// `eval_reference`), so the subtype lands through the layer
    /// pass even though the creating effect is long gone.
    #[test]
    fn amass_grows_and_subtypes_the_chosen_army() {
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::StatValue;
        use deckmaste_core::Subtype;

        use crate::decide::Decision;

        let (mut state, src) = bear_on_field();
        // An Army creature you (player 0) control — the amass target. A 2/2
        // base so the pre-growth object never risks an SBA before the
        // counters land; the amass grows it to 4/4.
        let army = mint_on_field(
            &mut state,
            Card::Normal(CardFace::from(Characteristics {
                name: "Zombie Army".into(),
                types: vec![Type::Creature.def()],
                subtypes: vec![Subtype {
                    name: "Army".into(),
                    types: vec![Type::Creature].into(),
                    confers: vec![].into(),
                }],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..Characteristics::default()
            })),
        );

        // Drive the ACTUAL macro expansion: amass Zombies 2 (the subtype is a
        // declared `Subtype` param, spelled bare) — through the SEMANTICS path
        // (`semantics::Instruction` → `lower()`), the path production now
        // takes.
        let semantic: deckmaste_semantics::OneShotEffect =
            builtin().macros.read_str("Amass(Zombie, 2)").unwrap();
        let frame = frame_src(&state, src);
        schedule_lowered_effect(&mut state, semantic, 0, &frame);

        // Step 1's guard (`Not(Exists(Army creature you control))`) is FALSE —
        // the Army above already exists — so no guard token is created; step to
        // the `With(ChooseOne(...))` that surfaces the pick.
        let mut choose = None;
        for _ in 0..30 {
            match state.step() {
                StepOutcome::NeedsDecision(DecisionPointKind::ChooseObjects(c)) => {
                    choose = Some(c);
                    break;
                }
                StepOutcome::NeedsDecision(other) => panic!("unexpected decision {other:?}"),
                StepOutcome::Progress(_) => {}
                StepOutcome::GameOver(_) => panic!("game ended before the amass choice"),
            }
        }
        let choose = choose.expect("amass surfaces a ChooseObjects for the Army creature");
        assert!(
            choose.candidates.contains(&army),
            "[CR#701.47a]: the Army creature you control is an amass candidate"
        );
        assert!(
            !choose.candidates.contains(&src),
            "[CR#701.47a]: a non-Army creature (the source Grizzly Bears) is NOT an amass candidate"
        );
        state
            .submit_decision(Decision::Chosen(vec![army]))
            .expect("choosing the Army creature is legal");

        // Pump the growth + becomes to completion.
        for _ in 0..30 {
            let _ = state.step();
            if state.pending.is_some() {
                break;
            }
        }

        // "Put N +1/+1 counters on that creature." — the `That`-bound growth
        // landed (an unbound `It` would have placed none).
        let p1p1: deckmaste_core::Ident = "P1P1Counter".into();
        assert_eq!(
            state.objects.obj(army).counters.get(&p1p1).copied(),
            Some(2),
            "[CR#701.47a]: N=2 +1/+1 counters on the chosen Army"
        );

        // "If it isn't a [subtype], it becomes a [subtype] in addition to its
        // other types." — the one-shot `Continuously(Modify(That, Add Zombie))`
        // locked the chosen id at creation, so the layer pass adds Zombie while
        // retaining Army ([CR#701.47a], [CR#611.2a] no stated duration).
        let view = state.layers();
        let subtypes = &view.get(army).subtypes;
        assert!(
            subtypes.iter().any(|s| s.name == "Zombie"),
            "[CR#701.47a]: the chosen Army becomes a Zombie in addition; got {subtypes:?}"
        );
        assert!(
            subtypes.iter().any(|s| s.name == "Army"),
            "[CR#701.47a]: 'in addition to its other types' — Army is retained; got {subtypes:?}"
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
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::Ability;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::EventFilter;
        use deckmaste_core::StatValue;
        use deckmaste_core::TriggeredAbility;

        use crate::decide::Action as Act;
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let (mut state, actor) = bear_on_field();
        let source_face = CardFace::from(Characteristics {
            name: "Wall of Omens".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(0)),
            toughness: Some(StatValue::Number(4)),
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                targets: [].into(),
                from: None,
                event: EventFilter::ZoneChange {
                    what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    from: None,
                    to: Some(Zone::Battlefield),
                    cause: None,
                },
                condition: None,
                limits: Vec::new().into(),
                effect: Instruction::draw(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                )
                .into(),
            })],
            ..Characteristics::default()
        });
        let src = mint_on_field(&mut state, Card::Normal(source_face));

        let frame = frame_src_targets(&state, actor, vec![src]);
        let hand_before = state.zones.hands[PlayerId(0).index()].len();
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Copy(
                    CopySpec {
                        source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                        exceptions: vec![],
                    }
                    .into(),
                ),
                riders: vec![].into(),
            }),
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
                StepOutcome::NeedsDecision(DecisionPointKind::Priority(_)) => {
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
        use deckmaste_core::StatValue;
        use deckmaste_core::StaticSpec;
        let count = Count::CountOf(Countable::Objects(Arc::new(
            deckmaste_core::Region::candidate(Predicate::creature()),
        )));
        Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Set(StatValue::Count(count.clone()))),
                    Modification::Toughness(NumericOp::Set(StatValue::Count(count))),
                ]
                .into(),
            ),
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
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::CopyException;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::StatValue;

        let mut state = game();
        let goyf = mint_on_field(
            &mut state,
            Card::Normal(CardFace::from(Characteristics {
                name: "Tarmogoyf".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::DefinedByAbility),
                toughness: Some(StatValue::DefinedByAbility),
                abilities: vec![tarmogoyf_shaped_cda()],
                ..Characteristics::default()
            })),
        );
        let frame = frame_src_targets(&state, goyf, vec![goyf]);
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Copy(
                    CopySpec {
                        source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                        exceptions: vec![
                            CopyException::Modify(Modification::Power(NumericOp::Set(
                                StatValue::Number(5),
                            ))),
                            CopyException::Modify(Modification::Toughness(NumericOp::Set(
                                StatValue::Number(5),
                            ))),
                        ],
                    }
                    .into(),
                ),
                riders: vec![].into(),
            }),
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
            Card::Normal(CardFace::from(Characteristics {
                name: "Bear".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..Characteristics::default()
            })),
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
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::StatValue;

        let mut state = game();
        let goyf = mint_on_field(
            &mut state,
            Card::Normal(CardFace::from(Characteristics {
                name: "Tarmogoyf".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::DefinedByAbility),
                toughness: Some(StatValue::DefinedByAbility),
                abilities: vec![tarmogoyf_shaped_cda()],
                ..Characteristics::default()
            })),
        );
        let frame = frame_src_targets(&state, goyf, vec![goyf]);
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Copy(
                    CopySpec {
                        source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                        exceptions: vec![],
                    }
                    .into(),
                ),
                riders: vec![].into(),
            }),
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
            Card::Normal(CardFace::from(Characteristics {
                name: "Bear".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..Characteristics::default()
            })),
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
    // core-copy-grammar Task 4 — `Filter::CardCopy` object-kind
    // classification ([CR#109.1])
    // ====================================================================

    /// Pins the [CR#109.1] distinction directly, not just "copies are
    /// CardCopy": [CR#109.1] lists "a copy of a card" and "a token" as
    /// DISTINCT object kinds; [CR#111.1] defines a token as "a marker used
    /// to represent any permanent that isn't represented by a card" with no
    /// carve-out for a copy-token; and [CR#707.10a]'s copy-cease SBA targets
    /// a copy of a spell/card, never a token — a token (copy or not) ceases
    /// under its OWN rule instead ([CR#111.7]). So BOTH a minted token copy
    /// (`TokenSpec::Copy`) and a plain token (`TokenSpec::Token`) classify
    /// `Token` and do NOT match `Filter::CardCopy`. (The genuinely
    /// `CardCopy`-classifying case — a card-less spell copy stranded off
    /// the stack, [CR#707.10a] — is `off_stack_copy_classifies_as_card_copy`
    /// in `tests/stack.rs`, since it needs the stack-copy harness, not this
    /// token-mint one.) Both classifications go through the SAME
    /// `is_object_class` / `Predicate::Class` path an SBA `scope` predicate
    /// evaluates (`sba.rs`'s `if !crate::matches(state, id, &rule.scope)`
    /// calls the exact `crate::matches` used here).
    #[test]
    fn token_copy_and_plain_token_both_classify_as_token_not_card_copy() {
        use deckmaste_core::CopySource;
        use deckmaste_core::CopySpec;
        use deckmaste_core::Token;

        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(&state, a, vec![b]);

        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: deckmaste_core::TokenSpec::Copy(
                    CopySpec {
                        source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
                        exceptions: vec![],
                    }
                    .into(),
                ),
                riders: vec![].into(),
            }),
            &frame,
        );
        let _ = state.step(); // the copy-token TokenCreated batch applies

        let plain_token = Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Artifact.def()].into(),
            subtypes: vec![].into(),
            abilities: vec![].into(),
            power: None,
            toughness: None,
        };
        state.run_effect(
            Instruction::act(Action::Create {
                agent: Reference::Reg(deckmaste_core::RefId(1)),
                count: Count::Literal(1),
                token: plain_token.into(),
                riders: vec![].into(),
            }),
            &frame,
        );
        let _ = state.step(); // the plain-token TokenCreated batch applies

        let created: Vec<ObjectId> = state
            .zones
            .battlefield
            .iter()
            .copied()
            .filter(|&id| id != a && id != b)
            .collect();
        assert_eq!(created.len(), 2, "both tokens minted");

        for &t in &created {
            assert!(
                crate::target::is_object_class(&state, t, ObjectClass::Token),
                "[CR#109.1,111.1]: every minted token — copy or not — \
                 classifies as Token, never CardCopy"
            );
            assert!(
                !obj_matches(&state, t, &Predicate::Class(ObjectClass::CopyOfACard)),
                "[CR#707.10a]: Filter::CardCopy must not match a token — a \
                 token's cease rule is [CR#111.7], not the copy-cease SBA"
            );
        }
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
        state.run_effect(
            Instruction::act(Action::WinGame(Reference::Reg(deckmaste_core::RefId(1)))),
            &frame,
        );
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
            state.action_items(
                &Action::WinGame(Reference::Reg(deckmaste_core::RefId(1))),
                &frame
            ),
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
        state.run_effect(
            Instruction::act(Action::LoseGame(Reference::Reg(deckmaste_core::RefId(1)))),
            &frame,
        );
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
            state.action_items(
                &Action::LoseGame(Reference::Reg(deckmaste_core::RefId(1))),
                &frame
            ),
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

    /// [CR#706.1]: `RollDice(3, 6)` draws 3 naturals in `1..=6` from the
    /// seeded rng, each `result == natural` (no modifier pipeline yet), as
    /// ONE simultaneous batch; the applied batch fixes "that many" to the
    /// summed results.

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
            Instruction::act(Action::discard(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(2),
                true,
            )),
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
                Some(DecisionPointKind::ChooseObjects(
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
                Instruction::act(Action::FlipCoins(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(3),
                    false,
                )),
                &frame,
            );
            drain_progress(&mut state, 20);
            state.run_effect(
                Instruction::act(Action::RollDice(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(3),
                    6,
                )),
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

    /// [CR#705.2]: a 3-coin CALLED flip pauses per coin — three sequential
    /// `CallFlip` decisions, each drawing (and scoring) only when its call is
    /// submitted — then front-schedules ONE simultaneous batch
    /// ([CR#603.2c]) once all three are called.

    /// A `CallFlip` decision only answers `Decision::Answer` — any other
    /// decision kind (here, a stray `Discard`) is rejected as `WrongKind`,
    /// leaving the `CallFlip` decision (and its stashed continuation) intact.
    #[test]
    fn call_flip_rejects_wrong_decision_kind() {
        use crate::decide::Decision;
        use crate::decide::DecisionError;
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::act(Action::FlipCoins(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                true,
            )),
            &frame,
        );
        drain_progress(&mut state, 20);
        assert!(matches!(
            state.pending,
            Some(DecisionPointKind::CallFlip(
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
                Some(DecisionPointKind::CallFlip(
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
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::Ability;
        use deckmaste_core::TriggeredAbility;

        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Randomness Watcher".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                targets: [].into(),
                from: None,
                event,
                condition: None,
                limits: Vec::new().into(),
                effect: Instruction::draw(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                )
                .into(),
            })],
            ..Characteristics::default()
        }));
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
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let watcher = put_watcher(
            &mut state,
            p0,
            EventFilter::CoinFlipped {
                by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                won: Some(true),
            },
        );
        let frame = frame_for(&state, p0);

        let mut won = false;
        for i in 0..50 {
            state.run_effect(
                Instruction::act(Action::FlipCoins(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    true,
                )),
                &frame,
            );
            // Pop exactly the front-scheduled `FlipCoins` work item — it
            // sets `pending` directly; no decision surfaces on this step.
            step_n(&mut state, 1);
            let Some(DecisionPointKind::CallFlip(crate::decide::pending::CallFlip { player })) =
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
                by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                won: Some(true),
            },
        );
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::act(Action::FlipCoins(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                false,
            )),
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
                by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            },
        );
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::act(Action::RollDice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(2),
                6,
            )),
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
        use crate::decide::DecisionPointKind;

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
                Instruction::act(Action::FlipCoins(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(2),
                    false,
                )),
                &frame,
            );
            step_n(&mut state, 2);

            // 1 called flip — the same call answer on both drives.
            state.run_effect(
                Instruction::act(Action::FlipCoins(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                    true,
                )),
                &frame,
            );
            step_n(&mut state, 1);
            assert!(matches!(
                state.pending,
                Some(DecisionPointKind::CallFlip(
                    crate::decide::pending::CallFlip { .. }
                ))
            ));
            state.submit_decision(Decision::Answer(true)).unwrap();
            step_n(&mut state, 1);

            // RollDice(2, 20).
            state.run_effect(
                Instruction::act(Action::RollDice(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(2),
                    20,
                )),
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
                Instruction::act(Action::discard(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(2),
                    true,
                )),
                &frame,
            );
            for _ in 0..20 {
                let recorded = state
                    .history
                    .entries()
                    .filter(|entry| {
                        matches!(
                            entry.fact,
                            GameEvent::ZoneChange(ZoneChange {
                                from: Some(Zone::Hand),
                                to: Zone::Graveyard,
                                ..
                            })
                        )
                    })
                    .count();
                if recorded == 2 {
                    break;
                }
                let _ = state.step();
            }

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

    /// [CR#105.2]: `AmongColorsOf` reads the referenced object's colors off
    /// the live layers view — `LayeredView::get` panics on an id absent from
    /// `state.objects`. A referent that's fully CEASED (a token that left the
    /// game, an LKI-only snapshot id with no live twin) must fizzle to no
    /// colors — no production at all — never crash. `source` (the Chrome-Mox
    /// stand-in mana-producing permanent) stays alive throughout — only the
    /// REFERENCED object (`imprinted`, reached through its announced-target
    /// register) goes away, mirroring how an imprinted/exiled card can cease
    /// independently of the producing permanent.
    #[test]
    fn among_colors_of_gone_referent_fizzles_empty() {
        use deckmaste_core::ManaSpec;

        // `frame_src_targets` declares source(0), controller(1), the four
        // event roles(2..=5), then the announced target at register 6.
        const TARGET: deckmaste_core::RefId = deckmaste_core::RefId(6);

        let (mut state, source, imprinted) = two_permanents_on_field();
        let frame = frame_src_targets(&state, source, vec![imprinted]);
        state.objects.remove(imprinted);
        assert!(
            state.objects.get(imprinted).is_none(),
            "the referent is gone"
        );
        assert!(
            state.objects.get(source).is_some(),
            "the mana source is still live"
        );

        let act = Action::AddMana(
            Reference::controller_parameter(),
            Count::Literal(1),
            ManaSpec::AmongColorsOf(Reference::Reg(TARGET)).into(),
        );
        // Must not panic dereferencing the gone id via `self.layers().get(..)`.
        let items = state.player_action_items(&act, &frame);
        assert!(
            items.is_empty(),
            "a gone AmongColorsOf referent has no colors to choose among, so no production"
        );
    }

    /// [CR#705.2]: a single CALLED flip surfaces one `CallFlip` decision (no
    /// draw yet); submitting the call draws the coin and scores `won = (call
    /// == heads)`. Seed-pinned (via `game()`'s seed 7): the draw is
    /// deterministic, so the assertion is exact, not just a shape check.
    #[test]
    fn called_flip_surfaces_call_and_scores_won() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::act(Action::FlipCoins(
                Reference::controller_parameter(),
                Count::Literal(1),
                true,
            )),
            &frame,
        );
        drain_progress(&mut state, 20);
        let Some(DecisionPointKind::CallFlip(crate::decide::pending::CallFlip { player })) =
            state.pending.clone()
        else {
            panic!("expected a pending CallFlip, got {:?}", state.pending);
        };
        assert_eq!(player, p0);

        state.submit_decision(Decision::Answer(true)).unwrap();
        drain_progress(&mut state, 20);

        let flips = coin_flips(&state);
        assert_eq!(flips.len(), 1, "exactly one CoinFlipped fact");
        let (heads, won) = flips[0];
        assert_eq!(
            won,
            Some(heads),
            "the call was heads: won iff the draw landed heads"
        );
    }

    /// [CR#706.1]: `RollDice(3, 6)` draws 3 naturals in `1..=6` from the
    /// seeded rng, each `result == natural` (no modifier pipeline yet), as
    /// ONE simultaneous batch.
    #[test]
    fn dice_roll_emits_per_die_results() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::act(Action::RollDice(
                Reference::controller_parameter(),
                Count::Literal(3),
                6,
            )),
            &frame,
        );
        drain_progress(&mut state, 20);

        let rolls = die_rolls(&state);
        assert_eq!(rolls.len(), 3, "3 DieRolled facts, one per drawn die");
        assert!(
            rolls
                .iter()
                .all(|&(natural, result)| (1..=6).contains(&natural) && natural == result),
            "every natural lands in 1..=6 and result == natural (no modifier pipeline yet)"
        );
    }

    /// [CR#705.2]: a 3-coin CALLED flip pauses per coin — three sequential
    /// `CallFlip` decisions, each drawing (and scoring) only when its call is
    /// submitted — then front-schedules ONE simultaneous batch
    /// ([CR#603.2c]) once all three are called.
    #[test]
    fn multi_coin_called_flip_pauses_per_coin() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::act(Action::FlipCoins(
                Reference::controller_parameter(),
                Count::Literal(3),
                true,
            )),
            &frame,
        );
        drain_progress(&mut state, 20);

        for (i, call) in [true, false, true].into_iter().enumerate() {
            let Some(DecisionPointKind::CallFlip(crate::decide::pending::CallFlip { player })) =
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

        let flips = coin_flips(&state);
        assert_eq!(flips.len(), 3, "3 CoinFlipped facts, one per called coin");
        assert!(
            flips.iter().all(|&(_, won)| won.is_some()),
            "every called flip records a winner/loser"
        );
    }

    /// [CR#705.1]: an uncalled `FlipCoins(3, false)` draws 3 coins straight
    /// from the seeded rng with NO decision and no winner/loser, as ONE
    /// simultaneous batch. Seed-pinned (via `game()`'s seed 7): same seed ⇒
    /// same draw, so the assertions are exact, not just shape checks.
    #[test]
    fn uncalled_flip_emits_batch_without_a_decision() {
        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        state.run_effect(
            Instruction::act(Action::FlipCoins(
                Reference::controller_parameter(),
                Count::Literal(3),
                false,
            )),
            &frame,
        );
        drain_progress(&mut state, 20);

        let flips = coin_flips(&state);
        assert_eq!(flips.len(), 3, "3 CoinFlipped facts, one per drawn coin");
        assert!(
            flips.iter().all(|&(_, won)| won.is_none()),
            "an uncalled flip never records a winner/loser"
        );
    }

    /// Every `CoinFlipped` fact in the history, as `(heads, won)`.
    fn coin_flips(state: &GameState) -> Vec<(bool, Option<bool>)> {
        state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::CoinFlipped(CoinFlipped { heads, won, .. }) => Some((*heads, *won)),
                _ => None,
            })
            .collect()
    }

    /// Every `DieRolled` fact in the history, as `(natural, result)`.
    fn die_rolls(state: &GameState) -> Vec<(Uint, Uint)> {
        state
            .history
            .entries()
            .filter_map(|e| match &e.fact {
                GameEvent::DieRolled(DieRolled {
                    natural, result, ..
                }) => Some((*natural, *result)),
                _ => None,
            })
            .collect()
    }

    /// [CR#107.3,705.2,706.2]: a flip/roll batch fixes the magnitude anaphor —
    /// won flips for a called flip, heads for an uncalled one, summed results
    /// for dice — so "flip three coins, then draw that many cards" reads the
    /// batch's own tally.
    ///
    /// The four assertions this carries are the halves the restored
    /// `called_flip_surfaces_call_and_scores_won`,
    /// `multi_coin_called_flip_pauses_per_coin`,
    /// `uncalled_flip_emits_batch_without_a_decision` and
    /// `dice_roll_emits_per_die_results` used to make against
    /// `GameState.that_much`.
    #[test]
    fn flip_and_roll_batches_fix_the_magnitude_anaphor() {
        const TALLY: deckmaste_core::RefId = deckmaste_core::RefId(8);

        let tally_after = |action: Action| {
            let mut state = game();
            let source = state.player(PlayerId(0)).object;
            let frame = frame_src(&state, source);
            state.run_effect(
                Instruction::producing(deckmaste_core::DefId(TALLY.0), action),
                &frame,
            );
            drain_progress(&mut state, 20);
            while let Some(crate::decide::DecisionPointKind::CallFlip(_)) = state.pending.clone() {
                state
                    .submit_decision(crate::decide::Decision::Answer(true))
                    .unwrap();
                drain_progress(&mut state, 20);
            }
            let tally = state.activation_number(frame.activation, TALLY);
            (state, tally)
        };

        let (state, tally) = tally_after(Action::FlipCoins(
            Reference::controller_parameter(),
            Count::Literal(3),
            false,
        ));
        let heads = Uint::try_from(coin_flips(&state).iter().filter(|&&(h, _)| h).count())
            .expect("heads count fits Uint");
        assert_eq!(tally, Some(heads), "an uncalled flip tallies heads");

        let (state, tally) = tally_after(Action::FlipCoins(
            Reference::controller_parameter(),
            Count::Literal(3),
            true,
        ));
        let wins = Uint::try_from(
            coin_flips(&state)
                .iter()
                .filter(|&&(_, won)| won == Some(true))
                .count(),
        )
        .expect("win count fits Uint");
        assert_eq!(tally, Some(wins), "a called flip tallies wins");

        let (state, tally) = tally_after(Action::RollDice(
            Reference::controller_parameter(),
            Count::Literal(3),
            6,
        ));
        let sum: Uint = die_rolls(&state).iter().map(|&(_, result)| result).sum();
        assert_eq!(tally, Some(sum), "a dice roll tallies the summed results");
    }

    /// A complete lowered card-shaped effect reads the runtime flip result
    /// through the register its first instruction defined, then gains that
    /// much life. This exercises semantic lowering, instruction scheduling,
    /// the runtime writer, and the ordinary `Count::Reg` reader together.
    #[test]
    fn a_lowered_coin_card_reads_its_number_of_heads() {
        let mut state = game();
        let player = PlayerId(0);
        let frame = frame_for(&state, player);
        schedule_lowered_effect(
            &mut state,
            deckmaste_semantics::OneShotEffect::Sequentially(
                [
                    deckmaste_semantics::OneShotEffect::Act(
                        deckmaste_semantics::Action::FlipCoins(
                            deckmaste_semantics::Reference::You,
                            deckmaste_semantics::Count::Literal(50),
                            false,
                        ),
                    ),
                    deckmaste_semantics::OneShotEffect::Act(
                        deckmaste_semantics::Action::ChangeLife(
                            deckmaste_semantics::Reference::You,
                            deckmaste_semantics::LifeOp::Up(deckmaste_semantics::Count::ThatMany),
                        ),
                    ),
                ]
                .into(),
            ),
            0,
            &frame,
        );
        drain_progress(&mut state, 30);

        let heads = deckmaste_core::Int::try_from(
            coin_flips(&state)
                .iter()
                .filter(|&&(heads, _)| heads)
                .count(),
        )
        .expect("fifty heads fit Int");
        assert!(
            heads > 0,
            "seed 7 makes the register-read assertion nonvacuous"
        );
        assert_eq!(
            state.player(player).life,
            20 + heads,
            "the later life-gain instruction read the flip instruction's register"
        );
    }

    /// A lowered random-discard effect records the number of cards that
    /// actually moved, then exposes that batch magnitude to the following
    /// instruction through `ThatMany`.
    #[test]
    fn a_lowered_discard_card_reads_cards_actually_discarded() {
        let mut state = game();
        let player = PlayerId(0);
        for i in 0..5 {
            mint_in_hand(&mut state, player, &format!("Magnitude Card {i}"));
        }
        let frame = frame_for(&state, player);
        schedule_lowered_effect(
            &mut state,
            deckmaste_semantics::OneShotEffect::Sequentially(
                [
                    deckmaste_semantics::OneShotEffect::Act(deckmaste_semantics::Action::discard(
                        deckmaste_semantics::Reference::You,
                        deckmaste_semantics::Count::Literal(2),
                        true,
                    )),
                    deckmaste_semantics::OneShotEffect::Act(
                        deckmaste_semantics::Action::ChangeLife(
                            deckmaste_semantics::Reference::You,
                            deckmaste_semantics::LifeOp::Up(deckmaste_semantics::Count::ThatMany),
                        ),
                    ),
                ]
                .into(),
            ),
            0,
            &frame,
        );
        drain_progress(&mut state, 50);

        assert_eq!(state.zones.graveyards[player.index()].len(), 2);
        assert_eq!(
            state.player(player).life,
            22,
            "the later life-gain instruction read the discard batch's card count"
        );
    }
}
