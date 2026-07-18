//! `action_items`: lower an object-verb [`Action`] to events and work items.

use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::Anchor;
use deckmaste_core::Destination;
use deckmaste_core::Reference;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use super::occurrence_of;
use crate::agenda::FinalizeWatch;
use crate::agenda::WorkItem;
use crate::event::AbilityCountered;
use crate::event::Act;
use crate::event::Attached;
use crate::event::Cause;
use crate::event::ControlChanged;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DamageDealt;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::Unattached;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::stack::Frame;
use crate::stack::StackObject;
use crate::state::GameState;

impl GameState {
    /// The `Emit` work item(s) a single-instruction `Action` produces. The
    /// source verbs (`DealDamage`, …) act with the source object as agent; the
    /// player verbs live under `By(who, …)`, where `who` resolves to the acting
    /// player and replaces the previously hard-coded `frame.controller`. Damage
    /// to a multi-valued selection is one simultaneous `Batch` (a later task);
    /// drawing N is N sequential `Single`s ([CR#121.2] — drawn one at a time).
    #[expect(
        clippy::too_many_lines,
        reason = "one exhaustive match lowering every Action variant into work items; the per-variant arms are cohesive and better read together than split across helpers"
    )]
    pub(crate) fn action_items(&self, action: &Action, frame: &Frame) -> Vec<WorkItem> {
        match action {
            // [CR#701,616.1]: a named keyword action is lowered to the
            // uniform ONE-window lane by `composite_items` — resolve the verb's
            // coordinates off its `name`/`body` (fizzling any that don't
            // resolve, [CR#701.8a]), open the single future `Act` window (where
            // every cant/replacement of every description competes), and plant
            // the `FinalizeAct` watcher that records the committed PAST
            // name-fact iff the verb's characteristic change actually landed.
            // The apply half owns the per-verb unwrap (`step.rs`); no content
            // event opens a second window.
            Action::Composite { name, body } => self.composite_items(name, body, frame),
            // The dealer is the resolved `source` — `This` (the default) is the
            // ability's source object / resolving spell, so the common case is
            // unchanged; an explicit source carries non-self-source damage and
            // "fight" ([CR#120.1,701.14a]).
            Action::DealDamage(source, qty, sel) => {
                let amount = self.eval_count(qty, frame);
                let dealer = self.eval_reference(source, frame);
                let targets = self.eval_reference_set(sel, frame);
                let events: Vec<GameEvent> = targets
                    .into_iter()
                    .map(|target| {
                        GameEvent::DamageDealt(DamageDealt {
                            source: dealer,
                            target,
                            amount,
                            // OneShotEffect damage is never combat damage ([CR#510.1]).
                            combat: false,
                        })
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.8a]: destroy has no bespoke verb — it is the
            // `Composite(Destroy(x), Move(x, Graveyard))` the `Action::destroy`
            // ctor / `Destroy` macro builds, handled by the `Composite` arm
            // above (which reads the body facet off the stored move and commits
            // it atomically on the one `Act(Destroy)` event).
            // The named player performs the verb: resolve `who` to the acting
            // player, then dispatch the `PlayerAction`. `By(You, …)` (the
            // implicit-you default) resolves to `frame.controller` — identical
            // to the previous hard-coded behavior.
            Action::By(who, pa) => {
                let actor = self.acting_player(who, frame);
                self.player_action_items(pa, actor, frame)
            }
            // [CR#701.6a]: countering cancels an object on the stack — it
            // never resolves. A countered SPELL is put into its owner's
            // graveyard (reminted off the stack, [CR#400.7]), cause-tagged
            // "Counter" so a "becomes countered" view can narrow by verb. A
            // countered ABILITY isn't a card and goes nowhere — it simply
            // ceases (remove from stack, no zone move). [CR#707.10a]: a
            // countered COPY (of a spell or ability) ceases the same way — no
            // card, no zone move; it shares the ABILITY arm's shape below. An
            // object already gone from the stack ([CR#608.2b]) is a no-op.
            // Spell is the happy path (ward's verb).
            Action::Counter(sel) => {
                let mut events = Vec::new();
                for object in self.eval_reference_set(sel, frame) {
                    // [CR#701.6a]: a "can't be countered" object is not
                    // countered — the instruction simply doesn't affect it (a
                    // `Cant(Counter)` deontic row, checked at the moment the
                    // counter would happen). Never crashes: an unmatched or
                    // gone object just fizzles like any no-op counter.
                    if !crate::legal::counter_legal(self, frame.source, object) {
                        continue;
                    }
                    match self
                        .stack
                        .iter()
                        .find(|e| e.id == object)
                        .map(|e| (e.copy, &e.object))
                    {
                        Some((false, StackObject::Spell(spell))) => {
                            events.push(GameEvent::ZoneChange(ZoneChange {
                                snapshot: None,
                                object: *spell,
                                from: Some(Zone::Stack),
                                to: Zone::Graveyard,
                                enters: None,
                                position: None,
                                face: None,
                                cause: Some(Cause::counter(
                                    Agency::EffectInstruction,
                                    Some((frame.source, frame.controller)),
                                )),
                            }));
                        }
                        // [CR#707.10a]: a copy of a spell, or a triggered/
                        // activated ability (copy or not), ceases the same
                        // way — no card, no zone move.
                        Some(_) => {
                            events.push(GameEvent::AbilityCountered(AbilityCountered {
                                id: object,
                                cause: Cause::counter(
                                    Agency::EffectInstruction,
                                    Some((frame.source, frame.controller)),
                                ),
                            }));
                        }
                        None => {}
                    }
                }
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.3a]: attach `what` to each resolved host. This builder is
            // pure (like every `action_items` arm — `Destroy`, `Tap`): it emits
            // the `Attached` fact, and the relation mutation (`attached_to`)
            // happens at that fact's apply ([CR#701.3c] gives the re-attach its
            // new timestamp). No-op — no fact, mirroring the Tap/Untap
            // transition-only idiom ([CR#603.2e]) — on the host it is already
            // on, or a host that is the attachment itself ([CR#303.4d]).
            Action::Attach { what, to } => {
                let hosts = self.eval_reference_set(to, frame);
                let mut events = Vec::new();
                for attachment in self.eval_reference_set(what, frame) {
                    for &host in &hosts {
                        if host == attachment {
                            continue; // [CR#303.4d]: can't attach to itself.
                        }
                        if self.objects.obj(attachment).attached_to == Some(host) {
                            continue; // [CR#701.3a]: already on that host.
                        }
                        // [CR#701.3b]: no-op on an illegal (what, host) pair —
                        // `attachment_legal` reads the attach deontics
                        // generically (default-deny: the attachment's own
                        // enchant/Innate `May(Attach)` grant must permit, the
                        // host's protection `Cant` subtracts), never the subtype.
                        if !crate::legal::attachment_legal(self, attachment, host) {
                            continue;
                        }
                        events.push(GameEvent::Attached(Attached { attachment, host }));
                    }
                }
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.3d]: unattach each selected attachment from its host —
            // emit the `Unattached` fact (the `attached_to` clear happens at the
            // fact's apply). No-op (no fact) on an attachment that isn't
            // attached, mirroring the transition-only idiom ([CR#603.2e]).
            Action::Unattach(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .filter_map(|attachment| {
                        self.objects.obj(attachment).attached_to.map(|former_host| {
                            GameEvent::Unattached(Unattached {
                                attachment,
                                former_host,
                            })
                        })
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#400.7]: a PLAIN zone move (no `Act(Destroy)` event, no
            // cause-verb fact) — each selected object moves from whatever zone
            // it's in to the `Destination`. The apply remints into the owner's
            // graveyard/hand/library (or the shared exile) — a hand
            // destination is the bounce family, subsuming the retired
            // `ReturnToHand` verb. NOT destruction (indestructible doesn't
            // apply) and NOT a sacrifice — the [CR#704.5m] Aura graveyard SBA's
            // mover. A library destination ([CR#401.7]) carries an insertion
            // index: `FromTop(n)` is a from-top index (0 = top); `FromBottom(n)`
            // is converted to one against the owner's current library size
            // (`len - n`), clamped at apply ("an index past the bottom places it
            // on the bottom"). This arm subsumes the former `PutInLibrary`.
            // `from` is the optional fizzle-guard ([CR#701.8a,701.9a,701.17a]):
            // `move_items` skips any object not currently in that zone —
            // never a panic, per-object like the existing gone-object skip.
            Action::Move(sel, destination, riders, from) => {
                // Enter riders ([CR#614.12]) await the enters-the-battlefield
                // machinery — a loud seam, like the other unbuilt verbs.
                if !riders.is_empty() {
                    todo!(
                        "core-action-riders-cost-modes seam: enter riders (tapped/attacking/\
                         with-counters) execute with the ETB machinery"
                    );
                }
                self.move_items(sel, destination, *from, frame)
            }
            // [CR#122]: move counters object→object — a remove from `from` plus
            // a place on `to`, emitted as one simultaneous batch (the apply
            // touches the two distinct counter maps). `Named` moves up to the
            // requested count of one kind (clamped to what `from` holds);
            // `AllKinds` moves every kind `from` holds at its full count — the
            // case a single-kind remove+put can't express. Counts that resolve
            // to zero (or an empty source) emit nothing.
            Action::MoveCounters(spec, from, to) => {
                let from_id = self.eval_reference(from, frame);
                let to_id = self.eval_reference(to, frame);
                // [CR#608.2b]: under a partial fizzle the entry resolved with
                // some target(s) illegal — a departed (reminted) source or
                // destination is not affected, so the two-endpoint move does
                // nothing rather than reading a gone object (never-crash). Both
                // endpoints live in the happy path.
                if self.objects.get(from_id).is_none() || self.objects.get(to_id).is_none() {
                    return vec![];
                }
                let held = &self.objects.obj(from_id).counters;
                let moves: Vec<(deckmaste_core::Ident, Uint)> = match spec {
                    deckmaste_core::CounterSpec::Named(kind, count) => {
                        let want = self.eval_count(count, frame);
                        let have = held.get(&kind.0).copied().unwrap_or(0);
                        let n = want.min(have);
                        if n == 0 { vec![] } else { vec![(kind.0, n)] }
                    }
                    deckmaste_core::CounterSpec::AllKinds => held
                        .iter()
                        .map(|(k, n)| (*k, *n))
                        .filter(|&(_, n)| n > 0)
                        .collect(),
                };
                let remove_cause = Some(crate::event::Cause::remove_counters(
                    deckmaste_core::Agency::EffectInstruction,
                    Some((frame.source, frame.controller)),
                ));
                let place_cause = Some(crate::event::Cause::put_counters(
                    deckmaste_core::Agency::EffectInstruction,
                    Some((frame.source, frame.controller)),
                ));
                let mut events = Vec::with_capacity(moves.len() * 2);
                for (kind, n) in moves {
                    events.push(GameEvent::CounterRemoved(CounterRemoved {
                        object: from_id,
                        kind,
                        amount: n,
                        cause: remove_cause.clone(),
                    }));
                    events.push(GameEvent::CounterPlaced(CounterPlaced {
                        object: to_id,
                        kind,
                        amount: n,
                        // Apply-computed totals ([CR#714.2b]).
                        before: 0,
                        after: 0,
                        cause: place_cause.clone(),
                    }));
                }
                if events.is_empty() {
                    vec![]
                } else {
                    vec![WorkItem::Emit(occurrence_of(events))]
                }
            }
            // `CreateReplacement` directly mutates `state.shields` — it is
            // intercepted in `run_effect` before `action_items` is called.
            // This arm is unreachable by design.
            Action::CreateReplacement { .. } => {
                unreachable!(
                    "CreateReplacement is handled in run_effect before action_items is called"
                )
            }
            // [CR#400.7,401.4]: relocate a GROUP as one simultaneous batch,
            // then arrange its landing when the destination is an ORDERED
            // library position — the cards arrive reminted, and
            // `ArrangeGroupLanding` orders the freshly-landed pile per the
            // `Arrangement` (Brainstorm's "on top … in any order"). A
            // non-library destination (unordered zone) needs no arrangement.
            Action::MoveGroup {
                group,
                arrangement,
                to,
                riders,
            } => {
                if !riders.is_empty() {
                    todo!(
                        "core-action-riders-cost-modes seam: MoveGroup enter riders execute \
                         with the ETB machinery"
                    );
                }
                // A group member with no zone to leave — a player proxy
                // (`zone: None`, which `Predicate::Any` matches) — is skipped,
                // never moved; an all-zoneless selection then fizzles via the
                // empty guard below (the engine-never-crashes-on-authoring
                // -mistakes ruling). Each surviving member carries its `from`
                // zone alongside its id, so the zone read can't panic.
                let objects: Vec<(ObjectId, Zone)> = self
                    .eval_selection_set(group, frame)
                    .into_iter()
                    .filter_map(|object| Some((object, self.objects.obj(object).zone?)))
                    .collect();
                if objects.is_empty() {
                    return vec![];
                }
                let (to_zone, anchor) = match to {
                    Destination::Zone(z) => (*z, None),
                    Destination::Library(a) => (Zone::Library, Some(a)),
                };
                let events: Vec<GameEvent> = objects
                    .iter()
                    .map(|&(object, from)| {
                        GameEvent::ZoneChange(ZoneChange {
                            snapshot: None,
                            object,
                            from: Some(from),
                            to: to_zone,
                            enters: None,
                            position: anchor.map(|a| self.library_index(object, a, frame)),
                            face: None,
                            cause: None,
                        })
                    })
                    .collect();
                let mut items = vec![WorkItem::Emit(occurrence_of(events))];
                if let Some(anchor) = anchor {
                    let (end, _) = self.anchor_end_offset(anchor, frame);
                    let library_owner = self.owner_of(objects[0].0);
                    let arranger = match arrangement {
                        // [CR#401.4]: "any order" is arranged by the cards'
                        // owner; RandomOrder/SameOrder carry no arranger (unused,
                        // defaulted to the owner).
                        deckmaste_core::Arrangement::ChosenOrder(r) => self.acting_player(r, frame),
                        deckmaste_core::Arrangement::AnyOrder
                        | deckmaste_core::Arrangement::RandomOrder
                        | deckmaste_core::Arrangement::SameOrder => library_owner,
                    };
                    let count = Uint::try_from(objects.len()).expect("group size fits Uint");
                    items.push(WorkItem::ArrangeGroupLanding {
                        arranger,
                        arrangement: arrangement.clone(),
                        library_owner,
                        end,
                        count,
                    });
                }
                items
            }
            // [CR#701.12b]: the patient comes under the referenced player's
            // control — a TRANSITION ([CR#603.2e]): a same-controller grant
            // emits nothing ("the exchange effect does nothing"), and a gone
            // patient yields no events (the enclosing `Simultaneously` then
            // voids the whole exchange, [CR#701.12a]). A gone reference
            // inside `to` still trips the layer view loudly — the
            // illegal-target machinery upstream is the graceful path.
            Action::GainControl(what, to) => {
                let object = self.eval_reference(what, frame);
                if self.objects.get(object).is_none() {
                    return vec![];
                }
                let player = self.acting_player(to, frame);
                if self.objects.obj(object).controller == player {
                    return vec![];
                }
                vec![WorkItem::Emit(Occurrence::single(
                    GameEvent::ControlChanged(ControlChanged { object, to: player }),
                ))]
            }
            // [CR#701.14a]: each fighting creature deals damage equal to
            // its power to the other — a PRIMITIVE verb with native
            // semantics, never `Simultaneously` sugar (fight is its own CR
            // event family).
            Action::ExtraPhase(..) => {
                todo!("engine seam: extra phases ([CR#500.8]) — turn-structure insertion unbuilt")
            }
            Action::BecomeDay | Action::BecomeNight => {
                todo!("engine seam: day/night designations ([CR#731.1]) unbuilt")
            }
            Action::TheRingTempts(_) => {
                todo!("engine seam: the Ring tempts you ([CR#701.54a]) — Ring machinery unbuilt")
            }
        }
    }

    /// The work item(s) a plain relocation ([CR#400.7]) produces — shared by
    /// the source-agent [`Action::Move`] and the player-agent
    /// [`PlayerAction::Move`] (both cause-free; the latter has no fizzle-guard
    /// slot, so it always calls here with `guard: None`). A card moving to a
    /// [`Destination::Library`] anchor that it ALREADY occupies is a same-zone
    /// REPOSITION ([CR#401.7]) — a `RepositionLibrary` work item that keeps the
    /// `ObjectId` and fires no zone change (scry never removes a card from the
    /// library, [CR#701.22a]). Every other move is a genuine future-form
    /// `ZoneChange` (remint); the zone-change intents batch as one occurrence.
    ///
    /// `guard` is [`Action::Move`]'s optional `from` fizzle-guard
    /// ([CR#701.8a,701.9a,701.17a]): when `Some(z)`, an object whose CURRENT
    /// zone isn't `z` is skipped — same never-crash treatment as the
    /// gone-object skip below, just a zone mismatch instead of a vanished
    /// object. Per-object, like the gone-object skip: other objects in a
    /// multi-object reference set still move normally.
    pub(super) fn move_items(
        &self,
        sel: &Reference,
        destination: &Destination,
        guard: Option<Zone>,
        frame: &Frame,
    ) -> Vec<WorkItem> {
        let mut items: Vec<WorkItem> = Vec::new();
        let mut zone_events: Vec<GameEvent> = Vec::new();
        for object in self.eval_reference_set(sel, frame) {
            // [CR#400.7,603.7c]: a bound role that resolved to a GONE object
            // (hidden destination, stale id) is skipped — the instruction
            // no-ops for it.
            if self.objects.get(object).is_none() {
                continue;
            }
            // A resolved member with no zone to leave — a player proxy
            // (`zone: None`, which `Predicate::Any` matches) — is skipped, never
            // moved (the engine-never-crashes-on-authoring-mistakes ruling).
            let Some(from) = self.objects.obj(object).zone else {
                continue;
            };
            // [CR#701.8a,701.9a,701.17a]: the `from` fizzle-guard — a zone
            // mismatch is a silent no-op for this object, never a panic.
            if let Some(z) = guard
                && from != z
            {
                continue;
            }
            match destination {
                Destination::Library(anchor) if from == Zone::Library => {
                    let (end, offset) = self.anchor_end_offset(anchor, frame);
                    items.push(WorkItem::RepositionLibrary {
                        object,
                        end,
                        offset,
                    });
                }
                _ => {
                    let to = match destination {
                        Destination::Zone(z) => *z,
                        Destination::Library(_) => Zone::Library,
                    };
                    let position = match destination {
                        Destination::Zone(_) => None,
                        Destination::Library(anchor) => {
                            Some(self.library_index(object, anchor, frame))
                        }
                    };
                    zone_events.push(GameEvent::ZoneChange(ZoneChange {
                        snapshot: None,
                        object,
                        from: Some(from),
                        to,
                        enters: None,
                        position,
                        face: None,
                        cause: None,
                    }));
                }
            }
        }
        if !zone_events.is_empty() {
            items.push(WorkItem::Emit(occurrence_of(zone_events)));
        }
        items
    }

    /// The (end, offset) a library [`Anchor`] names ([CR#401.7]) — the pile
    /// axis and count-from-that-end for a same-library reposition, resolved
    /// robustly to the card being pulled out first (unlike
    /// [`Self::library_index`], which pre-resolves a from-bottom anchor
    /// against the current size).
    fn anchor_end_offset(
        &self,
        anchor: &Anchor,
        frame: &Frame,
    ) -> (crate::agenda::LibraryEnd, Uint) {
        match anchor {
            Anchor::FromTop(c) => (crate::agenda::LibraryEnd::Top, self.eval_count(c, frame)),
            Anchor::FromBottom(c) => (crate::agenda::LibraryEnd::Bottom, self.eval_count(c, frame)),
        }
    }

    /// The from-top insertion index for a library [`Anchor`] ([CR#401.7]),
    /// resolved against the OWNER's current library size. `FromTop(n)` is the
    /// index itself (`0` = top); `FromBottom(n)` is `len - n` (`0` = the very
    /// bottom). The apply clamps an index past the bottom to the bottom, so a
    /// from-bottom anchor on a card entering from elsewhere lands correctly.
    fn library_index(&self, object: ObjectId, anchor: &Anchor, frame: &Frame) -> Uint {
        match anchor {
            Anchor::FromTop(c) => self.eval_count(c, frame),
            Anchor::FromBottom(c) => {
                let owner = self.owner_of(object);
                let len = Uint::try_from(self.zones.libraries[owner.index()].len())
                    .expect("library size fits Uint");
                len.saturating_sub(self.eval_count(c, frame))
            }
        }
    }

    /// Lower one keyword-action [`Action::Composite`] to the uniform
    /// [CR#616.1] one-window lane (the `Action::Composite` arm of
    /// [`Self::action_items`]). Resolve the atom's coordinates — a coordinate
    /// that fails to resolve, or a degenerate same-zone move, FIZZLES
    /// (`vec![]`, no window, [CR#701.8a]) — build the FUTURE `Act` window, and
    /// schedule `[Emit(Act), FinalizeAct]`. The window is the ONE cant →
    /// replace moment for every description of the action; the apply half owns
    /// the per-verb unwrap, and `FinalizeAct` records the past name-fact iff
    /// the characteristic change committed.
    ///
    /// Per-verb commit/finalize discipline: the move verbs
    /// (destroy/discard-single/mill) plant their `FinalizeAct` BEFORE the
    /// window so it observes the patients even through a redirecting
    /// replacement ([CR#701.9c] Megrim-under-madness); draw and the reorder
    /// verbs schedule their `FinalizeAct` from the APPLY (a per-card `mark`
    /// / an arrange-after-the-body ordering), so a replaced or canted
    /// action records nothing.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per keyword-action verb; splitting would scatter the dispatch"
    )]
    pub(crate) fn composite_items(
        &self,
        name: &deckmaste_core::VerbName,
        body: &deckmaste_core::OneShotEffect,
        frame: &Frame,
    ) -> Vec<WorkItem> {
        // The future window event. `contents` rides only when the apply must
        // unwrap a body it cannot rebuild from these flat coordinates (mill's
        // top-slice group, the reorder/fight arrange RunEffect).
        let future = |verb: &str,
                      who: Option<crate::player::PlayerId>,
                      on: Vec<ObjectId>,
                      from: Option<Zone>,
                      to: Option<Zone>,
                      cause: Option<Cause>,
                      carry: bool| {
            GameEvent::Act(Act {
                verb: deckmaste_core::VerbName::from(verb),
                who,
                on,
                from,
                to,
                cause,
                committed: false,
                contents: carry.then(|| {
                    Box::new(crate::event::ActContents {
                        body: body.clone(),
                        frame: frame.clone(),
                    })
                }),
                // An ordinary keyword-action window is never an aggregate.
                batch: None,
                // [CR#614.5]: carry forward whatever lineage `frame` itself
                // inherited (a passed aggregate `Batch` window's apply builds
                // ITS contained per-entity futures' frames with
                // `Anaphora::inherited_replacements` populated) — empty for the
                // overwhelming majority of ordinary keyword actions.
                inherited: frame.anaphora.inherited_replacements.clone(),
                // [CR#616.1g,121.2a]: this window is one of an aggregate's
                // contained per-entity futures iff `frame` says so.
                contained: frame.anaphora.contained_in_batch,
            })
        };
        // The move verbs' agent: the resolving source and its controller.
        let agent = Some((frame.source, frame.controller));

        // Dispatch on the verb NAME ([CR#701]); each verb reads its own
        // performer/patient references off the stored `body` (the same body
        // facets the renderer and Idris emitter read), so no typed atom is
        // needed. An unknown/mistyped verb name fizzles to no window — never a
        // panic (the engine-never-crashes-on-authoring-mistakes ruling).
        match name.as_str() {
            // ── Destroy: single Battlefield → Graveyard move ([CR#701.8a]) ──
            "Destroy" => {
                let Some(what) = composite_move_src(body) else {
                    return vec![];
                };
                let on = self.eval_reference(what, frame);
                let Some((to, guard)) = composite_body_move(body) else {
                    return vec![];
                };
                let Some(from) = self.move_from(on, guard) else {
                    return vec![]; // gone / zoneless / guard mismatch — fizzle
                };
                if from == to {
                    return vec![]; // degenerate same-zone move — fizzle [CR#701.8a]
                }
                let act = future(
                    "Destroy",
                    None,
                    vec![on],
                    Some(from),
                    Some(to),
                    Some(Cause::destroy(Agency::EffectInstruction, agent)),
                    false,
                );
                self.act_window(act, FinalizeWatch::Patients(vec![on]))
            }
            // ── Discard: bound single move, or the chosen-from-hand decision ──
            "Discard" => {
                if let Some(what) = deckmaste_core::discard_body_what(body) {
                    // "Discard this/that card" ([CR#702.29a]): a single move.
                    // The performer is the patient's controller (the affected
                    // player discards their own card, [CR#701.9b]).
                    let on = self.eval_reference(what, frame);
                    let player = self
                        .objects
                        .get(on)
                        .map_or(frame.controller, |o| o.controller);
                    let Some((to, guard)) = composite_body_move(body) else {
                        return vec![];
                    };
                    let Some(from) = self.move_from(on, guard) else {
                        return vec![];
                    };
                    if from == to {
                        return vec![];
                    }
                    let act = future(
                        "Discard",
                        Some(player),
                        vec![on],
                        Some(from),
                        Some(to),
                        Some(Cause::discard(Agency::EffectInstruction, agent)),
                        false,
                    );
                    self.act_window(act, FinalizeWatch::Patients(vec![on]))
                } else {
                    // The chosen/random discard's card choice is a real
                    // decision ([CR#701.9b]) that must be gated behind the
                    // SAME [CR#616.1] cant→replace window every keyword
                    // action gets ([CR#614.17] — "can't discard" suppresses
                    // the whole action before any choice is asked, exactly
                    // like Scry/Fight): carry the body as `contents` on a
                    // performer-only future Act. If the window passes, the
                    // apply's generic reorder/fight unwrap (`step.rs`)
                    // schedules the body as an ordinary `RunEffect` — the
                    // `With`+`Choose`/`Random` machinery (`resolve/effect.rs`)
                    // then either surfaces the `ChooseObjects` decision or
                    // samples the seeded rng, and each per-card `Each`
                    // iteration recurses back into THIS dispatch's bound
                    // single-move arm above (reusing its `Act(Discard)`
                    // construction), so "whenever a player discards a card"
                    // still fires once per card ([CR#701.9c]) and a canted
                    // window never asks the choice at all.
                    let Some(who) = composite_body_whose(body) else {
                        return vec![];
                    };
                    let Some(player) = self.eval_player_ref(who, frame) else {
                        return vec![]; // unresolvable performer — fizzle
                    };
                    let act = future(
                        "Discard",
                        Some(player),
                        vec![],
                        None,
                        None,
                        Some(Cause::discard(Agency::EffectInstruction, agent)),
                        true,
                    );
                    vec![WorkItem::Emit(Occurrence::single(act))]
                }
            }
            // ── Mill: simultaneous Library → Graveyard batch ([CR#701.17a]).
            // The slice-family shape (`Batch(n, Act(Composite(name: Mill, …)))`)
            // resolves through the aggregate lane (`batch_act_head` + the batch
            // apply's simultaneous slice); this arm covers a bare per-unit Mill
            // composite reaching the ordinary lane, milling its `MoveGroup`
            // slice as one batch. ──
            "Mill" => {
                let Some(who) = composite_body_whose(body) else {
                    return vec![];
                };
                let Some(player) = self.eval_player_ref(who, frame) else {
                    return vec![];
                };
                let patients: Vec<ObjectId> = composite_body_group(body)
                    .map(|(group, _)| self.eval_selection_set(&group, frame))
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|&o| self.objects.get(o).and_then(|x| x.zone) == Some(Zone::Library))
                    .collect();
                if patients.is_empty() {
                    return vec![]; // empty library / count 0 — fizzle [CR#701.17b]
                }
                let act = future(
                    "Mill",
                    Some(player),
                    vec![],
                    Some(Zone::Library),
                    Some(Zone::Graveyard),
                    Some(Cause::mill(Agency::EffectInstruction, agent)),
                    true,
                );
                self.act_window(act, FinalizeWatch::Patients(patients))
            }
            // ── Draw: one single-card window ([CR#121.2]); the count-many
            // sequence is the aggregate `Batch` lane's, which replicates this
            // per-unit window. ──
            "Draw" => {
                let Some(who) = composite_body_whose(body) else {
                    return vec![];
                };
                let Some(player) = self.eval_player_ref(who, frame) else {
                    return vec![];
                };
                // The draw's `FinalizeAct` is scheduled by its OWN apply (the
                // late-bound card and its `mark` are known only there), so only
                // the window is planted here.
                vec![WorkItem::Emit(Occurrence::single(future(
                    "Draw",
                    Some(player),
                    vec![],
                    None,
                    None,
                    Some(Cause::draw(Agency::EffectInstruction, agent)),
                    false,
                )))]
            }
            // ── Reorder verbs: scry / surveil / fateseal ([CR#701.22a]) ──
            verb @ ("Scry" | "Surveil" | "Fateseal") => {
                let Some(who) = composite_body_whose(body) else {
                    return vec![];
                };
                let Some(player) = self.eval_player_ref(who, frame) else {
                    return vec![];
                };
                if !self.composite_body_would_act(body, frame) {
                    return vec![]; // scry 0 / empty peek — fizzle [CR#701.22b]
                }
                // The arrange runs from the apply (a decision can't complete
                // inside an apply), which then schedules `FinalizeAct` — so a
                // replaced/canted reorder records nothing, and the surviving
                // fact lands AFTER the arrange ([CR#701.22d]).
                let act = future(verb, Some(player), vec![], None, None, None, true);
                vec![WorkItem::Emit(Occurrence::single(act))]
            }
            // ── Fight: If-guarded reciprocal damage ([CR#701.14a]) ──
            "Fight" => {
                let Some((a, _b)) = deckmaste_core::fight_body_fighters(body) else {
                    return vec![];
                };
                let on = self.eval_reference(a, frame);
                if self.objects.get(on).is_none() || !self.composite_body_would_act(body, frame) {
                    return vec![]; // gone fighter / guard fails — fizzle [CR#701.14b]
                }
                let act = future("Fight", None, vec![on], None, None, None, true);
                vec![WorkItem::Emit(Occurrence::single(act))]
            }
            // An unknown verb name performs no keyword action — fizzle, never
            // panic ([CR#701]).
            _ => vec![],
        }
    }

    /// The window items a resolve-time keyword action schedules: the future
    /// `Act` (the one [CR#616.1] window) plus its `FinalizeAct` watcher,
    /// planted BEFORE the window so it observes the outcome whether the
    /// action passes, is redirected, or is replaced away. `mark` freezes
    /// the resolution-event cursor so the watcher scans only what commits
    /// after this point.
    fn act_window(&self, act: GameEvent, watch: FinalizeWatch) -> Vec<WorkItem> {
        vec![
            WorkItem::Emit(Occurrence::single(act.clone())),
            WorkItem::FinalizeAct {
                act,
                watch,
                mark: self.resolution_events.len(),
            },
        ]
    }

    /// The zone a move verb's patient starts from, honoring the body `Move`'s
    /// optional `from:` fizzle-guard ([CR#701.8a,701.9a]): `None` (fizzle) when
    /// the object is gone/zoneless, or its current zone doesn't match a `Some`
    /// guard.
    fn move_from(&self, object: ObjectId, guard: Option<Zone>) -> Option<Zone> {
        let from = self.objects.get(object).and_then(|o| o.zone)?;
        match guard {
            Some(z) if z != from => None,
            _ => Some(from),
        }
    }
}

/// A single-move keyword action's body facet ([CR#603.6]): the plain
/// destination zone its stored `Move` relocates the patient to, plus that
/// `Move`'s optional `from:` fizzle-guard ([CR#701.8a,701.9a]). `Destroy`'s
/// body is `Move(x, Graveyard)` → `Some((Graveyard, None))`; a reorder / group
/// body (an `Each` / `MoveGroup`) → `None`. Read off the stored body ("matches
/// the expanded body", [CR#701.8a]) rather than a per-verb table.
fn composite_body_move(body: &deckmaste_core::OneShotEffect) -> Option<(Zone, Option<Zone>)> {
    use deckmaste_core::Action as A;
    use deckmaste_core::OneShotEffect as E;
    match body {
        E::Expanded(e) => composite_body_move(&e.value),
        E::Act(A::Move(_, Destination::Zone(z), _, guard)) => Some((*z, *guard)),
        _ => None,
    }
}

/// The BATCH relocation of a keyword-action [`Action::Composite`]
/// ([CR#701.17a,603.3b]): the [`Selection`](deckmaste_core::Selection) its
/// stored body's [`MoveGroup`](deckmaste_core::Action::MoveGroup) moves as ONE
/// simultaneous batch, plus the plain zone they land in. `Mill(who, n)`'s body
/// is `MoveGroup { group: TopOfLibrary(n, who), to: Graveyard }` →
/// `Some((group, Graveyard))`; a single-move (Destroy) or reorder (scry) body →
/// `None`. Read off the stored body rather than a per-verb table.
pub(crate) fn composite_body_group(
    body: &deckmaste_core::OneShotEffect,
) -> Option<(deckmaste_core::Selection, Zone)> {
    use deckmaste_core::Action as A;
    use deckmaste_core::OneShotEffect as E;
    match body {
        E::Expanded(e) => composite_body_group(&e.value),
        E::Act(A::MoveGroup {
            group,
            to: Destination::Zone(z),
            ..
        }) => Some((group.clone(), *z)),
        _ => None,
    }
}

/// The single-move patient a `Destroy` / bound-`Discard` body names — its
/// stored `Move`'s source reference ([CR#701.8a,702.29a]). Shares
/// [`discard_body_what`](deckmaste_core::discard_body_what)'s descent, so the
/// resolve lane reads the patient off the body exactly as the renderer and
/// Idris emitter do.
pub(crate) fn composite_move_src(
    body: &deckmaste_core::OneShotEffect,
) -> Option<&deckmaste_core::Reference> {
    deckmaste_core::discard_body_what(body)
}

/// The performer a slice / reorder / chosen-discard body names — the `whose`
/// of the top-of-library slice its body reads
/// (draw/mill/scry/surveil/fateseal), or a discard's
/// [`discard_body_whose`](deckmaste_core::discard_body_whose) (its `With`
/// binder's `Choose.by` / at-random hand-owner). `whose` defaults to `You`
/// (filled on read), so this is `Some` for every well-formed such body. Read
/// off the stored body rather than a per-verb atom.
pub(crate) fn composite_body_whose(
    body: &deckmaste_core::OneShotEffect,
) -> Option<&deckmaste_core::Reference> {
    use deckmaste_core::Action as A;
    use deckmaste_core::Binder;
    use deckmaste_core::OneShotEffect as E;
    use deckmaste_core::Selection as S;
    match body {
        E::Expanded(e) => composite_body_whose(&e.value),
        E::Each(each) => match &each.binder {
            Binder::Existing(S::TopOfLibrary { whose, .. }) => Some(whose),
            _ => None,
        },
        E::Act(A::MoveGroup {
            group: S::TopOfLibrary { whose, .. },
            ..
        }) => Some(whose),
        _ => deckmaste_core::discard_body_whose(body),
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::Anchor;
    use deckmaste_core::Binder;
    use deckmaste_core::Card;
    use deckmaste_core::CardFace;
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Count;
    use deckmaste_core::Destination;
    use deckmaste_core::Lookback;
    use deckmaste_core::Modification;
    use deckmaste_core::NumericOp;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Type;
    use deckmaste_core::Uint;
    use deckmaste_core::Zone;

    use crate::Decision;
    use crate::PendingDecision;
    use crate::agenda::WorkItem;
    use crate::event::AbilityCountered;
    use crate::event::Act;
    use crate::event::Attached;
    use crate::event::CounterPlaced;
    use crate::event::DamageDealt;
    use crate::event::EmblemCreated;
    use crate::event::GameEvent;
    use crate::event::LifeGained;
    use crate::event::LifeLost;
    use crate::event::Occurrence;
    use crate::event::Tapped;
    use crate::event::Unattached;
    use crate::event::ZoneChange;
    use crate::matches as obj_matches;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::resolve::fixtures::*;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::step::Progress;
    use crate::step::StepOutcome;
    use crate::test_support::frame_for;
    use crate::test_support::frame_src;
    use crate::test_support::frame_src_targets;
    use crate::trigger::TriggerBindings;

    /// A two-player game with player 0's deck = Darksteel Myr (an
    /// indestructible 0/1), one forced onto the battlefield.
    fn myr_on_field() -> (GameState, ObjectId) {
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let m = *state.zones.hands[0]
            .iter()
            .find(|&&o| obj_matches(&state, o, &Predicate::creature()))
            .expect("a Darksteel Myr in the opening hand");
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != m);
        state.objects.obj_mut(m).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(m);
        (state, m)
    }

    /// Mint (on the battlefield, player 0) an Equipment-shaped artifact
    /// carrying the default-deny `Innate(May(Attach(what: Ref(This), to:
    /// Creature)))` grant — an attachment that may legally attach to a
    /// creature host.
    fn may_attach_creature_equipment(state: &mut GameState) -> ObjectId {
        use deckmaste_core::Ability;
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::StaticEffect;
        let card = Card::Normal(CardFace {
            name: "Test Equipment".into(),
            types: vec![Type::Artifact.def()],
            abilities: vec![Ability::Innate(Arc::new(Ability::r#static(
                StaticEffect::Deontic(Deontic::May(DeonticAction::Attach {
                    what: Predicate::Ref(Reference::This),
                    to: Predicate::creature(),
                })),
            )))],
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

    /// Drives the agenda forward a bounded number of steps — assert on the
    /// post-condition, not the iteration count. A pending decision stops it.
    fn drain(state: &mut GameState) {
        for _ in 0..30 {
            if matches!(state.step(), StepOutcome::NeedsDecision(_)) {
                break;
            }
        }
    }

    /// [CR#701.3a]: `Attach` sets the attachment→host relation and records the
    /// `Attached` fact. Under default-deny the attachment carries a
    /// `May(Attach to: Creature)` grant so the (creature) host is legal.
    #[test]
    fn attach_sets_the_relation_and_emits_attached() {
        let (mut state, _bear, b) = two_permanents_on_field();
        // `b` is a creature (Grizzly Bears); mint a granted attachment for `a`.
        let a = may_attach_creature_equipment(&mut state);
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(a).attached_to,
            Some(b),
            "a is attached to b"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Attached(Attached { attachment, host })
                if *attachment == a && *host == b)
            ),
            "Attached fact recorded"
        );
    }

    /// [CR#701.3a]: attaching to the host it is already on is a no-op — no
    /// second `Attached` fact (transition-only, [CR#603.2e]).
    #[test]
    fn attach_to_current_host_is_a_noop() {
        let (mut state, _bear, b) = two_permanents_on_field();
        // Under default-deny the attachment needs a `May(Attach to: Creature)`
        // grant, or the drain's SBA sweep would unattach it from the (creature)
        // host `b` before the re-attach no-op is even observed.
        let a = may_attach_creature_equipment(&mut state);
        state.objects.obj_mut(a).attached_to = Some(b);
        let frame = frame_src_targets(a, vec![b]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, Some(b));
        assert!(
            !logged(&state, |e| matches!(
                e,
                GameEvent::Attached(Attached { .. })
            )),
            "no Attached fact for a re-attach to the current host"
        );
    }

    /// [CR#303.4d]: an attachment can't be attached to itself — a no-op.
    #[test]
    fn attach_to_self_is_a_noop() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, None, "host == what no-op");
        assert!(
            !logged(&state, |e| matches!(
                e,
                GameEvent::Attached(Attached { .. })
            )),
            "no Attached fact for a self-attach"
        );
    }

    /// [CR#701.3b]: `Attach` no-ops on an illegal host — under default-deny the
    /// attachment carries a conferred `Innate(May(Attach(what: Ref(This), to:
    /// Creature)))` grant (the Equipment-subtype shape), and the host is a
    /// non-creature, so no grant covers the pair: the link stays `None` and no
    /// `Attached` fact is recorded.
    #[test]
    fn attach_illegal_noop() {
        use deckmaste_core::CardFace;

        let mut state = game();
        // The attachment: an Equipment-shaped artifact whose May(Attach) grant
        // only covers creature hosts (mirrors the Equipment subtype confer).
        let equip = may_attach_creature_equipment(&mut state);

        // The host: a non-creature artifact "Rock".
        let rock_card = Card::Normal(CardFace {
            name: "Rock".into(),
            types: vec![Type::Artifact.def()],
            ..CardFace::default()
        });
        let rock_id = state.cards.push(Arc::new(rock_card), PlayerId(0));
        let rock = state.objects.mint(
            ObjectSource::Card(rock_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(rock);

        let frame = frame_src_targets(equip, vec![rock]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(equip).attached_to,
            None,
            "illegal attach no-ops ([CR#701.3b])"
        );
        assert!(
            !logged(&state, |e| matches!(
                e,
                GameEvent::Attached(Attached { .. })
            )),
            "no Attached fact for an illegal host"
        );
    }

    /// [CR#701.3d]: `Unattach` clears the relation and records the `Unattached`
    /// fact carrying the former host.
    #[test]
    fn unattach_clears_the_relation_and_emits_unattached() {
        let (mut state, a, b) = two_permanents_on_field();
        state.objects.obj_mut(a).attached_to = Some(b);
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::Act(Action::Unattach(Reference::This)),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(a).attached_to,
            None,
            "a is now unattached"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Unattached(Unattached { attachment, former_host })
                if *attachment == a && *former_host == b)
            ),
            "Unattached fact records the former host"
        );
    }

    /// [CR#701.3d]: unattaching an attachment that isn't attached is a no-op —
    /// no `Unattached` fact (transition-only, [CR#603.2e]).
    #[test]
    fn unattach_of_an_unattached_object_is_a_noop() {
        let (mut state, a, _b) = two_permanents_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::Act(Action::Unattach(Reference::This)),
            &frame,
        );
        drain(&mut state);
        assert_eq!(state.objects.obj(a).attached_to, None);
        assert!(
            !logged(&state, |e| matches!(
                e,
                GameEvent::Unattached(Unattached { .. })
            )),
            "no Unattached fact for an already-unattached object"
        );
    }

    /// A `Random` group bound into the frame drives a verb with NO surfaced
    /// decision: `Each(Random(Exactly 1, creature), Destroy(It))`
    /// over a frame whose `chosen` holds the RNG's pick destroys exactly that
    /// one creature. (Verbs take a single `Reference`, so plurality/choice is
    /// the enclosing `Each`; the `Random` inline-RNG resolution is a dormant
    /// seam that reads `frame.anaphora.chosen` — [CR#608.2d].)
    #[test]
    fn destroy_random_destroys_one_without_a_decision() {
        use deckmaste_core::Each;
        use deckmaste_core::Quantity;

        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);

        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        // The RNG's pick is bound into the frame before the group is read.
        let mut frame = frame_src(bear);
        frame.anaphora.chosen = Some(vec![theirs]);
        let before = [bear, theirs]
            .iter()
            .filter(|o| state.zones.battlefield.contains(o))
            .count();
        assert_eq!(before, 2);

        state.run_effect(
            OneShotEffect::Each(Each {
                binder: Binder::Existing(Selection::Random(Quantity::one(), creatures)),
                effect: Arc::new(OneShotEffect::Act(Action::destroy(Reference::It))),
            }),
            &frame,
        );
        // No decision: the Random group is already bound in the frame.
        assert!(
            !matches!(state.step(), StepOutcome::NeedsDecision(_)),
            "a bound Random group surfaces no decision"
        );
        // Pump the agenda to completion (bounded safety cap; assert on the
        // post-condition, not the iteration count).
        for _ in 0..30 {
            let alive = [bear, theirs]
                .iter()
                .filter(|o| state.zones.battlefield.contains(o))
                .count();
            if alive == 1 {
                break;
            }
            let _ = state.step();
        }
        let alive = [bear, theirs]
            .iter()
            .filter(|o| state.zones.battlefield.contains(o))
            .count();
        assert_eq!(alive, 1, "exactly one creature destroyed (the bound pick)");
    }

    /// `With(ChooseOne(creature), Destroy(That))` surfaces `ChooseObjects`; an
    /// out-of-range count and an out-of-pool object are rejected; a legal pick
    /// destroys exactly that creature ([CR#608.2d]). Choosing is a pre-step
    /// (`With`) bound as `That`, never part of the verb.
    #[test]
    fn destroy_choose_surfaces_decision_validates_and_destroys() {
        use deckmaste_core::Binder;
        use deckmaste_core::With;

        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, bear) = bear_on_field();
        let theirs = second_bear_to_player_1(&mut state);

        let creatures = Predicate::And(vec![
            Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
            Predicate::creature(),
        ]);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::With(With {
                binder: Binder::ChooseOne {
                    filter: creatures,
                    by: Reference::You,
                },
                body: Arc::new(OneShotEffect::Act(Action::destroy(Reference::That(
                    deckmaste_core::Sort::Permanent,
                )))),
            }),
            &frame,
        );

        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects(
            crate::decide::pending::ChooseObjects {
                player,
                candidates,
                min,
                max,
            },
        )) = state.step()
        else {
            panic!("expected ChooseObjects, got {:?}", state.pending);
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!((min, max), (1, 1));
        assert_eq!(
            candidates.len(),
            2,
            "both battlefield creatures are candidates"
        );

        // Too many (count 2 > max 1).
        assert!(
            state
                .submit_decision(Decision::Chosen(candidates.clone()))
                .is_err(),
            "count must be within [min, max]"
        );
        // Out of pool (a player proxy is not a creature).
        assert!(
            state
                .submit_decision(Decision::Chosen(vec![state.player(PlayerId(0)).object]))
                .is_err(),
            "every chosen object must be a candidate"
        );

        // Legal: destroy player 1's creature.
        state
            .submit_decision(Decision::Chosen(vec![theirs]))
            .unwrap();
        // Pump the agenda to completion (bounded safety cap; we assert on the
        // post-condition, not the iteration count).
        for _ in 0..30 {
            if !state.zones.battlefield.contains(&theirs) {
                break;
            }
            let _ = state.step();
        }
        assert!(
            !state.zones.battlefield.contains(&theirs),
            "the chosen creature is destroyed"
        );
        assert!(
            state.zones.battlefield.contains(&bear),
            "the unchosen creature survives"
        );
    }

    /// [CR#702.12b]: an indestructible permanent can't be destroyed — the
    /// `Destroy` action's `Act(Destroy)` event is suppressed by the
    /// event-side cant pass ([CR#614.17]) in `apply_occurrence`, so the
    /// Myr stays on the battlefield.
    #[test]
    fn indestructible_survives_destroy_action() {
        let (mut state, myr) = myr_on_field();
        let frame = frame_src(myr);
        state.run_effect(OneShotEffect::Act(Action::destroy(Reference::This)), &frame);
        // Act(Destroy) applies and schedules no zone move (replaced to nothing).
        let _ = state.step();
        assert!(
            state.objects.get(myr).is_some(),
            "indestructible object still exists"
        );
        assert!(
            state.zones.battlefield.contains(&myr),
            "still on the battlefield"
        );
        assert!(state.zones.graveyards[0].is_empty(), "not destroyed");
    }

    /// A destructible creature still dies: `Destroy` → `Act(Destroy)` (nothing
    /// replaces it) → future-form `ZoneChange(Battlefield → Graveyard)` →
    /// its past form, reminting it into its owner's graveyard.
    #[test]
    fn destroy_action_sends_a_normal_creature_to_its_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(OneShotEffect::Act(Action::destroy(Reference::This)), &frame);
        // Act(Destroy) → future-form ZoneChange → past-form ZoneChange.
        for _ in 0..3 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.graveyards[0].len(), 1);
    }

    /// [CR#400.7]: `Move(This, Graveyard)` is a PLAIN relocation — no
    /// `Act(Destroy)` event, so it's a direct future-form `ZoneChange
    /// (Battlefield → Graveyard)` → its past form, reminting the object into
    /// its OWNER's graveyard. (Indestructible would not save it — but a plain
    /// Grizzly Bears exercises the move path.)
    #[test]
    fn move_sends_this_to_owner_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Act(Action::move_to(Reference::This, Zone::Graveyard)),
            &frame,
        );
        // future-form ZoneChange → past-form ZoneChange (one fewer step than
        // Destroy — no Act(Destroy) replace stage).
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "moved into owner's graveyard"
        );
    }

    // ── engine-act-facet-contract ticket-item behaviors ──────────────────────

    /// Ticket item 2 ([CR#701.8a]): a degenerate keyword action — destroy a
    /// card that is ALREADY in the graveyard, so its body move would be a
    /// same-zone `Graveyard → Graveyard` no-op — FIZZLES entirely: no
    /// `Act(Destroy)` fact, no `ZoneChange`, no panic.
    #[test]
    fn degenerate_destroy_on_graveyard_card_fizzles() {
        let (mut state, bear) = bear_on_field();
        // Relocate the bear into its owner's graveyard by hand.
        state.zones.battlefield.retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(bear);

        let frame = frame_src(bear);
        state.run_effect(OneShotEffect::Act(Action::destroy(Reference::This)), &frame);
        let events = drain_events(&mut state, 30);

        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { .. }))),
            "a degenerate destroy fires no keyword-action fact"
        );
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneChange(ZoneChange { .. }))),
            "a degenerate destroy schedules no zone change"
        );
        assert!(
            !logged(&state, |e| matches!(e, GameEvent::Act(Act { .. }))),
            "no Act(Destroy) fact is recorded"
        );
        assert!(
            state.zones.graveyards[0].contains(&bear),
            "the card stays where it was"
        );
    }

    /// Ticket item 4 ([CR#614.17,701.17a]): a `CantHappen(ZoneChange(from:
    /// Library))` static suppresses the whole mill — the mill `Act` carries the
    /// Library → Graveyard body facet, so the cant window matches it and zero
    /// cards move: no `Act(Mill)` fact, no `ZoneChange`, no "you milled"
    /// trigger.
    #[test]
    fn suppressed_mill_leaves_no_fact() {
        use deckmaste_core::EventFilter;

        let cant = StaticEffect::CantHappen(EventFilter::ZoneChange {
            what: Predicate::Any,
            from: Some(Zone::Library),
            to: None,
            cause: None,
        });
        let (mut state, _warden) =
            crate::replace_registry::tests_support::creature_with_static(cant);
        let a = mint_in_library(&mut state, PlayerId(0), "A");
        let b = mint_in_library(&mut state, PlayerId(0), "B");

        let frame = frame_for(&state, PlayerId(0));
        state.run_effect(
            OneShotEffect::mill(Reference::You, Count::Literal(2)),
            &frame,
        );
        let events = drain_events(&mut state, 30);

        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Mill")),
            "a fully-suppressed mill fires no Act(Mill) fact"
        );
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneChange(ZoneChange { .. }))),
            "no cards move"
        );
        assert_eq!(
            state.zones.libraries[0].iter().copied().collect::<Vec<_>>(),
            vec![a, b],
            "the library is untouched"
        );
        assert!(
            !logged(
                &state,
                |e| matches!(e, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Mill")
            ),
            "no Act(Mill) fact is recorded"
        );
    }

    /// Ticket item 5 ([CR#701.8a]): a keyword action whose performer reference
    /// resolves to nobody (`Draw(It)` with no `It` bound) FIZZLES — no
    /// `Act(Draw)` fact, no `DrewFromEmpty`, no panic.
    #[test]
    fn unresolvable_who_fizzles_draw() {
        let (mut state, src) = bear_on_field();
        // `frame_src` binds no `It`, so `Draw(It, 1)` has no performer.
        let frame = frame_src(src);
        state.run_effect(
            OneShotEffect::draw(Reference::It, Count::Literal(1)),
            &frame,
        );
        let events = drain_events(&mut state, 30);

        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { .. }))),
            "an unresolvable draw fires no keyword-action fact"
        );
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::DrewFromEmpty(_))),
            "an unresolvable draw never reaches the empty-library loss"
        );
    }

    /// Ticket item 1 ([CR#616.1]): the mill `Act` is a genuine pre-commit
    /// replacement window — `Instead(Act(Mill) → GainLife(3))` replaces the
    /// mill BEFORE any card moves: the library is untouched, no `Act(Mill)`
    /// fact records, and life is gained instead.
    #[test]
    fn mill_replaced_before_cards_move() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::Replacement;
        use deckmaste_core::VerbName;

        let mut state = game();
        // A battlefield permanent (player 0's) carrying the replacement.
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Mill Warden".into(),
                types: vec![Type::Creature.def()],
                abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
                    Replacement::Instead {
                        would: EventFilter::Act {
                            verb: VerbName::from("Mill"),
                            who: Predicate::Any,
                            on: Predicate::Any,
                            cause: None,
                        },
                        instead: OneShotEffect::Act(Action::by_you(PlayerAction::GainLife(
                            Count::Literal(3),
                        ))),
                    },
                )))],
                ..CardFace::default()
            }),
        );
        let a = mint_in_library(&mut state, PlayerId(0), "A");
        let b = mint_in_library(&mut state, PlayerId(0), "B");
        let life_before = state.player(PlayerId(0)).life;

        let frame = frame_for(&state, PlayerId(0));
        state.run_effect(
            OneShotEffect::mill(Reference::You, Count::Literal(2)),
            &frame,
        );
        drain_events(&mut state, 30);

        assert_eq!(
            state.zones.libraries[0].iter().copied().collect::<Vec<_>>(),
            vec![a, b],
            "the mill was replaced before any card moved"
        );
        assert!(
            !logged(
                &state,
                |e| matches!(e, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Mill")
            ),
            "a replaced mill records no Act(Mill) fact"
        );
        assert_eq!(
            state.player(PlayerId(0)).life,
            life_before + 3,
            "the replacement's GainLife(3) ran"
        );
    }

    /// Finalization ([CR#701.9c]): a replacement that REDIRECTS the discard's
    /// move (to Exile) rather than fully replacing it still lets the
    /// `Act(Discard)` name-fact record — the Megrim-under-madness shape. The
    /// card lands in exile, and the discard fact is recorded so a
    /// "whenever ~ discards" trigger would still fire.
    #[test]
    fn redirected_discard_still_records_the_name_fact() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::Replacement;
        use deckmaste_core::VerbName;

        let mut state = game();
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Leyline".into(),
                types: vec![Type::Creature.def()],
                abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
                    Replacement::Instead {
                        would: EventFilter::Act {
                            verb: VerbName::from("Discard"),
                            who: Predicate::Any,
                            on: Predicate::Any,
                            cause: None,
                        },
                        instead: OneShotEffect::Act(Action::move_to(
                            Reference::EventObject,
                            Zone::Exile,
                        )),
                    },
                )))],
                ..CardFace::default()
            }),
        );
        // A card in player 0's hand to discard.
        let card = mint_in_hand(&mut state, PlayerId(0), "Discardee");

        let mut frame = frame_for(&state, PlayerId(0));
        frame.anaphora.it = Some(crate::stack::ItBinding::Object(
            crate::lki::LkiSnapshot::capture(&state, card),
        ));
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::It)),
            &frame,
        );
        drain_events(&mut state, 30);

        // Zone changes REMINT ids ([CR#400.7]), so membership is by backing
        // card, not the pre-move `ObjectId`.
        assert!(
            zone_has_named(&state, &state.zones.exile, "Discardee"),
            "the redirected discard put the card into exile"
        );
        assert!(
            !zone_has_named(&state, &state.zones.hands[0], "Discardee"),
            "the card left the hand"
        );
        assert!(
            state.zones.graveyards[0].is_empty(),
            "the redirect kept the card out of the graveyard"
        );
        assert!(
            logged(
                &state,
                |e| matches!(e, GameEvent::Act(Act { verb, committed: true, .. })
                    if verb.as_str() == "Discard")
            ),
            "the Act(Discard) name-fact still records ([CR#701.9c])"
        );
    }

    /// Ticket item 6 ([CR#701.22d]): the "whenever you scry" trigger fact —
    /// the `Act(Scry)` — is recorded only AFTER the arrange commits. The
    /// `Act(Scry)` applied fact must come after the last library reposition in
    /// the resolution trace.
    #[test]
    fn scry_fact_records_after_the_arrange() {
        let p0 = PlayerId(0);
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        drain_progress(&mut state, 60); // → the per-card top/bottom pick.
        state.submit_decision(Decision::Modes(vec![1])).unwrap(); // bottom.
        let trace = drain_progress(&mut state, 60);

        let repos = trace
            .iter()
            .rposition(|p| matches!(p, Progress::Repositioned(_)))
            .expect("the scry repositioned a card");
        let act = trace
            .iter()
            .position(|p| matches!(p, Progress::Applied(occ)
                if occ_has(occ, |e| matches!(e, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Scry"))))
            .expect("the scry fact was recorded");
        assert!(
            act > repos,
            "the Act(Scry) fact ({act}) records after the reposition ({repos})"
        );
    }

    /// Whether an occurrence carries an event matching `pred`.
    fn occ_has(occ: &Occurrence, pred: impl Fn(&GameEvent) -> bool) -> bool {
        match occ {
            Occurrence::Single(e) => pred(e),
            Occurrence::Batch(es) => es.iter().any(pred),
        }
    }

    #[test]
    fn action_items_for_tap_draw_loselife() {
        let (state, src) = bear_on_field();
        let frame = frame_src(src);

        // By(You, Tap(This)) -> one Single(Tapped(src)) carrying the
        // effect-instruction cause triple (events.md §3).
        let items = state.action_items(&Action::by_you(PlayerAction::Tap(Reference::This)), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Tapped(
                Tapped {
                    object: src,
                    cause: Some(crate::event::Cause {
                        verb: "Tap".into(),
                        agency: deckmaste_core::Agency::EffectInstruction,
                        agent: Some((src, PlayerId(0))),
                    }),
                }
            )))]
        );

        // draw_one(You) -> one single-card Act(Draw) for the controller
        // ([CR#121.2] per-card, `on: None` — the drawn card binds at apply;
        // the count-many sequence is the aggregate `Batch` lane's).
        let items = state.action_items(&Action::draw_one(Reference::You), &frame);
        assert_eq!(items.len(), 1);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::Act(Act {
                who: Some(PlayerId(0)),
                on,
                ..
            }))) if on.is_empty()
        )));

        // By(You, LoseLife(3)) -> one Single(LifeLost{player0, 3})
        let items = state.action_items(
            &Action::by_you(PlayerAction::LoseLife(Count::Literal(3))),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost(
                LifeLost {
                    player: PlayerId(0),
                    amount: 3,
                }
            )))]
        );
    }

    /// [CR#701.26a]: only an untapped permanent can be tapped — a tap
    /// instruction on an already-tapped object is a no-op, and a no-op is
    /// no event ([CR#603.2e] "becomes tapped" fires on the transition only).
    #[test]
    fn tap_effect_skips_already_tapped() {
        let (mut state, src) = bear_on_field();
        state.objects.obj_mut(src).tapped = true;
        let frame = frame_src(src);
        let items = state.action_items(&Action::by_you(PlayerAction::Tap(Reference::This)), &frame);
        assert_eq!(
            items,
            vec![],
            "tapping an already-tapped object emits nothing"
        );
    }

    /// [CR#701.26b]: the untap mirror — untapping an untapped object is a
    /// no-op, no event.
    #[test]
    fn untap_effect_skips_already_untapped() {
        let (state, src) = bear_on_field();
        let frame = frame_src(src);
        let items = state.action_items(
            &Action::by_you(PlayerAction::Untap(Reference::This)),
            &frame,
        );
        assert_eq!(
            items,
            vec![],
            "untapping an already-untapped object emits nothing"
        );
    }

    /// An explicit agent: `draw_one(It)` draws for the announced/bound player
    /// (`It`, read off the body's `TopOfLibrary` selection), not the
    /// controller. Targets player 1's proxy.
    #[test]
    fn action_items_explicit_agent_draws_for_target() {
        let (state, src) = bear_on_field();
        let p1_proxy = state.players[1].object;
        let frame = frame_src_targets(src, vec![p1_proxy]);
        let items = state.action_items(&Action::draw_one(Reference::It), &frame);
        assert_eq!(items.len(), 1);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::Act(Act {
                who: Some(PlayerId(1)),
                on,
                ..
            }))) if on.is_empty()
        )));
    }

    /// The Blood-Money shape ([CR#607.2a] fact-backed product groups): a
    /// `Noting`-wrapped destroy-all over three creatures, one of which
    /// can't be destroyed — "destroyed this way" is exactly the clause's
    /// enacted destroy-caused past-form `ZoneChange` facts, so the survivor is
    /// excluded BY CONSTRUCTION (its `Act(Destroy)` was canted; no move
    /// fact exists), and the two dies-facts share one history batch id
    /// ([CR#603.3b]).
    #[test]
    fn destroyed_this_way_product_group_excludes_indestructible_survivor() {
        let (mut state, a, b) = two_permanents_on_field();
        // The indestructible shape ([CR#702.12b] — destruction can't
        // happen), as the canted static.
        let survivor = {
            let source = "Normal(name: \"Darksteel Test\", types: [Creature], abilities: [\
                 Static(CantHappen(ZoneChange(what: Ref(This), \
                 from: Battlefield, to: Graveyard)))])";
            let card = builtin()
                .macros
                .read_str::<deckmaste_core::Card>(source)
                .unwrap();
            mint_on_field(&mut state, card)
        };

        let effect = OneShotEffect::Noting(deckmaste_core::Noting {
            key: "destroyed".into(),
            effect: Arc::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::SelectAll(Predicate::And(
                    vec![
                        Predicate::State(deckmaste_core::StatePredicate::InZone(Zone::Battlefield)),
                        Predicate::creature(),
                    ],
                ))),
                effect: Arc::new(OneShotEffect::Act(Action::destroy(Reference::It))),
            })),
        });
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        assert!(
            state.objects.get(survivor).is_some()
                && state.objects.obj(survivor).zone == Some(Zone::Battlefield),
            "the can't-be-destroyed creature survived"
        );
        let group = &state.noted[&deckmaste_core::Ident::from("destroyed")];
        assert_eq!(
            group.len(),
            2,
            "the product group is the ENACTED destroy facts, not the gathered set"
        );
        let members: Vec<ObjectId> = group.iter().map(|m| m.snapshot.object).collect();
        assert!(
            members.contains(&a) && members.contains(&b),
            "exactly the two destroyed creatures, by LKI"
        );
        // The dies-facts committed as ONE batch ([CR#603.3b]).
        let ids: Vec<Option<deckmaste_core::Uint>> = state
            .history
            .entries()
            .filter(|e| {
                matches!(
                    e.fact,
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: Some(_),
                        ..
                    })
                )
            })
            .map(|e| e.batch)
            .collect();
        assert_eq!(ids.len(), 2);
        assert!(ids[0].is_some() && ids[0] == ids[1], "one shared batch id");
    }

    /// "Cards milled this way" ([CR#701.17a,701.17c,607.2a]): a
    /// `Noting`-wrapped mill commits the three moves as ONE cause-carried
    /// batch, populates the product group from the enacted facts, and a
    /// following clause ACTS on the group through `AmongNoted` — exiling
    /// exactly the milled cards.
    #[test]
    fn cards_milled_this_way_reads_the_enacted_product_group() {
        let (mut state, a) = bear_on_field();
        let libsize = state.zones.libraries[0].len();
        assert!(libsize >= 3, "the harness deck has cards to mill");

        let effect = OneShotEffect::Sequentially(vec![
            OneShotEffect::Noting(deckmaste_core::Noting {
                key: "milled".into(),
                effect: Arc::new(OneShotEffect::mill(Reference::You, Count::Literal(3))),
            }),
            OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::AmongNoted(
                    "milled".into(),
                    deckmaste_core::Quantity::Range(None, None),
                )),
                effect: Arc::new(OneShotEffect::Act(Action::Move(
                    Reference::It,
                    deckmaste_core::Destination::Zone(Zone::Exile),
                    vec![],
                    None,
                ))),
            }),
        ]);
        let frame = frame_src(a);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        let group = &state.noted[&deckmaste_core::Ident::from("milled")];
        assert_eq!(group.len(), 3, "three enacted mill facts");
        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::ZoneChange(ZoneChange {
                    snapshot: Some(_),
                    cause: Some(c),
                    to: Zone::Graveyard,
                    ..
                }) if c.verb.as_str() == "Mill"
            )),
            "the moves carry the Mill cause ([CR#701.17a])"
        );
        assert_eq!(
            state.zones.libraries[0].len(),
            libsize - 3,
            "three cards left the library"
        );
        assert_eq!(
            state.zones.exile.len(),
            3,
            "the follow-on clause exiled exactly the cards milled this way"
        );
        assert!(
            state.zones.graveyards[0].is_empty(),
            "the milled cards moved on from the graveyard"
        );
    }

    /// [CR#701.17b,603.3b]: `OneShotEffect::mill(You, n)` moves the top `n` of
    /// the library to the graveyard as ONE simultaneous batch, clamped to
    /// library size — milling 100 from a bounded library mills the whole
    /// library (never an out-of-range panic), and the moves share one batch
    /// id ([CR#603.3b]).
    #[test]
    fn mill_clamps_to_library_size_as_one_batch() {
        let (mut state, a) = bear_on_field();
        let libsize = state.zones.libraries[0].len();
        assert!(
            (1..100).contains(&libsize),
            "a bounded non-empty library to over-mill"
        );
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::mill(Reference::You, Count::Literal(100)),
            &frame,
        );
        run_injected(&mut state);

        assert!(
            state.zones.libraries[0].is_empty(),
            "the whole library milled — clamped to size ([CR#701.17b])"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            libsize,
            "every library card reached the graveyard"
        );
        let batches: Vec<Option<deckmaste_core::Uint>> = state
            .history
            .entries()
            .filter(|e| {
                matches!(
                    e.fact,
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: Some(_),
                        to: Zone::Graveyard,
                        ..
                    })
                )
            })
            .map(|e| e.batch)
            .collect();
        assert_eq!(batches.len(), libsize, "one committed move per milled card");
        assert!(
            batches.iter().all(|b| b.is_some() && *b == batches[0]),
            "the milled cards land as ONE simultaneous batch ([CR#603.3b])"
        );
    }

    /// [CR#701.22d]: the aggregate `Act(Mill)` fact lands AFTER the milled cards
    /// reach the graveyard (a "whenever you mill" trigger keys on it), and
    /// [CR#701.17b,701.22b]: milling from an EMPTY library performs no keyword
    /// action — no `Act(Mill)`, so no trigger.
    #[test]
    fn act_mill_fires_post_commit_and_not_from_an_empty_library() {
        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::mill(Reference::You, Count::Literal(2)),
            &frame,
        );
        run_injected(&mut state);

        let facts: Vec<GameEvent> = state.history.entries().map(|e| e.fact.clone()).collect();
        let act_pos = facts
            .iter()
            .position(|f| matches!(f, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Mill"))
            .expect("the mill emits its aggregate Act(Mill) fact");
        let last_gy = facts
            .iter()
            .rposition(|f| {
                matches!(
                    f,
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: Some(_),
                        to: Zone::Graveyard,
                        ..
                    })
                )
            })
            .expect("the milled cards committed to the graveyard");
        assert!(
            act_pos > last_gy,
            "Act(Mill) fires AFTER the cards reach the graveyard ([CR#701.22d])"
        );

        // Empty the library, mill again: no keyword action, no Act(Mill).
        state.zones.libraries[0].clear();
        let before = state.history.entries().count();
        state.run_effect(
            OneShotEffect::mill(Reference::You, Count::Literal(2)),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            !state.history.entries().skip(before).any(
                |e| matches!(&e.fact, GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Mill")
            ),
            "an empty-library mill performs no keyword action ([CR#701.17b,701.22b])"
        );
    }

    /// [CR#616.1]: a Rest-in-Peace-style `→Graveyard` replacement redirects
    /// EACH milled card to exile. The aggregate `Batch(n, Act(Mill))` window
    /// takes the ONE [CR#616.1g] count-multiplier hit, then its PASSED apply
    /// commits the top-`n` slice as ONE `Occurrence::Batch` of per-card
    /// Library → Graveyard futures — each individually replaceable (Rest in
    /// Peace binds `EventObject` and rewrites the honored event `to` per card),
    /// so the whole slice lands in exile as one simultaneous batch.
    #[test]
    fn rest_in_peace_replaces_milled_cards_with_exile() {
        let (mut state, a) = bear_on_field();
        // "If a card would be put into a graveyard, exile it instead."
        let rip = deckmaste_core::Replacement::Instead {
            would: deckmaste_core::EventFilter::ZoneChange {
                what: Predicate::Any,
                from: None,
                to: Some(Zone::Graveyard),
                cause: None,
            },
            instead: OneShotEffect::Act(Action::move_to(Reference::EventObject, Zone::Exile)),
        };
        let card = Arc::new(Card::Normal(CardFace {
            name: "Rest in Peace".into(),
            types: vec![Type::Enchantment.def()],
            abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(rip)))],
            ..CardFace::default()
        }));
        let card_id = state.cards.push(card, PlayerId(0));
        let rip_id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(rip_id);

        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::mill(Reference::You, Count::Literal(2)),
            &frame,
        );
        run_injected(&mut state);

        assert!(
            state.zones.graveyards[0].is_empty(),
            "the Rest-in-Peace replacement kept the milled cards out of the graveyard"
        );
        assert_eq!(
            state.zones.exile.len(),
            2,
            "both milled cards were exiled instead ([CR#616.1])"
        );
    }

    /// [CR#701.9b]: the chosen-discard lane — ONE batched choice of `count`
    /// cards, realized as PER-CARD `Act(Discard)` events: a multi-discard
    /// logs one fact per card ("whenever a player discards a card" fires
    /// once per card — Liliana's Caress's ruling), and each committed
    /// Hand → Graveyard move carries the `Discard` cause.
    #[test]
    fn discard_choice_realizes_per_card_act_events() {
        use crate::Decision;
        use crate::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, a) = bear_on_field();
        let frame = frame_src(a);
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
            candidates,
            min,
            max,
        }) = pending
        else {
            panic!("expected the batched card choice, got {pending:?}");
        };
        assert_eq!(player, PlayerId(0));
        assert_eq!((min, max), (2, 2), "one choice of 2 cards");
        let picks = candidates[..2].to_vec();
        state
            .submit_decision(Decision::Chosen(picks.clone()))
            .unwrap();
        run_injected(&mut state);

        let acts = state
            .history
            .entries()
            .filter(|e| {
                matches!(&e.fact, crate::event::GameEvent::Act(Act { verb, who, on, .. })
                    if verb.as_str() == "Discard"
                        && *who == Some(PlayerId(0))
                        && on.first().is_some_and(|o| picks.contains(o)))
            })
            .count();
        assert_eq!(
            acts, 2,
            "a 2-card discard is TWO per-card Act(Discard) facts"
        );
        assert_eq!(state.zones.graveyards[0].len(), 2, "both cards committed");
        let discard_causes = state
            .history
            .entries()
            .filter(|e| {
                matches!(&e.fact, crate::event::GameEvent::ZoneChange(ZoneChange {
                    snapshot: Some(_),
                    cause: Some(c),
                    ..
                }) if c.verb.as_str() == "Discard")
            })
            .count();
        assert_eq!(
            discard_causes, 2,
            "each committed move carries the Discard cause"
        );
    }

    /// Whether `zone` holds an object printed with `name` — zone changes
    /// REMINT ids ([CR#400.7]), so post-move membership is checked by the
    /// backing card, never a pre-move `ObjectId`.
    fn zone_has_named(state: &GameState, zone: &[crate::object::ObjectId], name: &str) -> bool {
        zone.iter()
            .any(|&o| matches!(state.def(o), deckmaste_core::Card::Normal(f) if f.name == name))
    }

    /// [CR#702.35a]: the madness window — a replacement over `Act(Discard)`
    /// of a NAMED card reroutes THAT card's Hand → Graveyard to exile, and no
    /// other's: per-card events mean per-card replacement. (The fixture rides
    /// a battlefield permanent — the Bag-of-Holding shape — because statics
    /// are gathered from the battlefield today; sourcing madness's static
    /// from the HAND is the static-ability-zone-gating seam, not this
    /// window's.)
    #[test]
    fn madness_style_replacement_exiles_its_card_only() {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::EventFilter;
        use deckmaste_core::VerbName;

        use crate::Decision;
        use crate::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, a) = bear_on_field();
        // "If a player would discard [Madness Card], exile it instead."
        let madness = deckmaste_core::Replacement::Instead {
            would: EventFilter::Act {
                verb: VerbName::from("Discard"),
                who: Predicate::Any,
                on: Predicate::Characteristic(CharacteristicPredicate::Named(
                    "Madness Card".into(),
                )),
                cause: None,
            },
            instead: OneShotEffect::Act(Action::move_to(Reference::EventObject, Zone::Exile)),
        };
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Madness Watcher".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![Ability::r#static(StaticEffect::Replacement(Arc::new(
                    madness,
                )))],
                ..CardFace::default()
            }),
        );
        let mad = mint_in_hand(&mut state, PlayerId(0), "Madness Card");
        let plain = state.zones.hands[0][0];
        assert_ne!(mad, plain);

        let frame = frame_src(a);
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
        let PendingDecision::ChooseObjects(crate::decide::pending::ChooseObjects { .. }) = pending
        else {
            panic!("expected the batched card choice, got {pending:?}");
        };
        state
            .submit_decision(Decision::Chosen(vec![mad, plain]))
            .unwrap();
        run_injected(&mut state);

        assert!(
            zone_has_named(&state, &state.zones.exile, "Madness Card"),
            "the madness-watched card was exiled instead ([CR#702.35a])"
        );
        assert!(
            !zone_has_named(&state, &state.zones.graveyards[0], "Madness Card"),
            "the replaced discard never reaches the graveyard"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            1,
            "the OTHER discarded card still goes to the graveyard — per-card replacement"
        );
    }

    // ===== Madness matrix (Task 10): the real `Madness` keyword end-to-end =====

    /// A creature card FACE carrying Madness {1}{R} and a printed {3}{R} cost —
    /// a castable creature spell (so a madness cast is affordable when the pool
    /// covers {1}{R}).
    fn madness_creature(name: &str) -> CardFace {
        use deckmaste_core::Color;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::ManaCost;
        use deckmaste_core::ManaSymbol;
        use deckmaste_core::SimpleManaSymbol;
        use deckmaste_core::StatValue;
        CardFace {
            name: name.into(),
            mana_cost: ManaCost::from(vec![
                ManaSymbol::Simple(SimpleManaSymbol::Generic(3)),
                ManaSymbol::Simple(SimpleManaSymbol::Specific(ColorOrColorless::Color(
                    Color::Red,
                ))),
            ]),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            abilities: vec![keyword("Madness([Mana([Generic(1), Red])])")],
            ..CardFace::default()
        }
    }

    /// Grant `player` `n` red mana (enough to cover a `{1}{R}` madness cost
    /// with one Red + one Red-as-generic, or top up as a test needs).
    fn grant_red(state: &mut GameState, player: PlayerId, n: deckmaste_core::Uint) {
        state
            .player_mut(player)
            .mana_pool
            .add(deckmaste_core::Color::Red.into(), n);
    }

    /// Whether a COMMITTED discard name-fact of `card` is in the history — what
    /// a "whenever you discard" trigger (Megrim) keys on ([CR#701.9c]).
    fn discarded_fact(state: &GameState, card: ObjectId) -> bool {
        state.history.entries().any(|e| {
            matches!(&e.fact, GameEvent::Act(Act { verb, on, committed, .. })
                if verb.as_str() == "Discard" && on.as_slice() == [card] && *committed)
        })
    }

    /// A "whenever you discard a card, lose 2 life" fixture (Megrim's shape,
    /// [CR#701.9c] — the trigger keys on the finalized `Act(Discard)`
    /// name-fact).
    fn megrim_fixture(state: &mut GameState) {
        use deckmaste_core::EventFilter;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::VerbName;
        mint_on_field(
            state,
            Card::Normal(CardFace {
                name: "Megrim Fixture".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![Ability::triggered(TriggeredAbility {
                    ability_word: None,
                    where_x: None,
                    from: None,
                    event: EventFilter::Act {
                        verb: VerbName::from("Discard"),
                        who: Predicate::Ref(Reference::You),
                        on: Predicate::Any,
                        cause: None,
                    },
                    condition: None,
                    limits: Vec::new(),
                    effect: OneShotEffect::Act(Action::by_you(PlayerAction::LoseLife(
                        Count::Literal(2),
                    ))),
                })],
                ..CardFace::default()
            }),
        );
    }

    /// Whether a replacement key is sourced from an OFF-battlefield object —
    /// the shape of a madness self-replacement (its card is in hand/exile),
    /// distinct from a battlefield graveyard-hoser (Rest in Peace / Leyline).
    fn madness_source_off_field(
        state: &GameState,
        key: crate::replace_registry::ReplacementKey,
    ) -> bool {
        match key {
            crate::replace_registry::ReplacementKey::Static { source, .. } => state
                .objects
                .get(source)
                .is_some_and(|o| o.zone != Some(Zone::Battlefield)),
            crate::replace_registry::ReplacementKey::Floating(_) => false,
        }
    }

    /// Drive the stack to empty (all just-fired triggers resolved), answering
    /// the madness delayed trigger's "you may cast it" with `cast` (false =
    /// decline). Stops the moment priority is offered over an EMPTY stack — so
    /// the turn structure never advances past the triggers under test.
    fn drive_declining_or_casting(state: &mut GameState, cast: bool) {
        use crate::decide::Action as Act;
        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;
        for _ in 0..200 {
            // Mana empties at every step boundary ([CR#500.5]); the delayed
            // trigger resolves a step or two after the discard, so top the pool
            // back up to the madness cost while we wait for its cast offer.
            if cast && state.player(PlayerId(0)).mana_pool.is_empty() {
                grant_red(state, PlayerId(0), 2);
            }
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(PendingDecision::Priority(
                    crate::decide::pending::Priority { .. },
                )) => {
                    if state.stack.is_empty() {
                        return; // triggers all resolved; don't advance the turn
                    }
                    state.submit_decision(Decision::Act(Act::Pass)).unwrap();
                }
                StepOutcome::NeedsDecision(PendingDecision::YesNo(
                    crate::decide::pending::YesNo { .. },
                )) => {
                    state.submit_decision(Decision::Answer(cast)).unwrap();
                }
                StepOutcome::NeedsDecision(PendingDecision::PayMana(
                    crate::decide::pending::PayMana { cost, .. },
                )) => {
                    // The pool only ever holds the {1}{R} madness cost (topped up
                    // above), never the four mana the printed {3}{R} would need —
                    // a successful pay here IS the alternative cost ([CR#118.9]).
                    assert_eq!(
                        cost,
                        deckmaste_core::ManaCost::from(vec![
                            deckmaste_core::ManaSymbol::Simple(
                                deckmaste_core::SimpleManaSymbol::Generic(1)
                            ),
                            deckmaste_core::ManaSymbol::Simple(
                                deckmaste_core::SimpleManaSymbol::Specific(
                                    deckmaste_core::ColorOrColorless::Color(
                                        deckmaste_core::Color::Red
                                    )
                                )
                            ),
                        ]),
                        "the madness cast pays {{1}}{{R}}, not the printed {{3}}{{R}}"
                    );
                    let pay = state.auto_pay_pending();
                    state.submit_decision(Decision::Pay(pay)).unwrap();
                }
                StepOutcome::NeedsDecision(PendingDecision::OrderTriggers(
                    crate::decide::pending::OrderTriggers { triggers, .. },
                )) => {
                    let order: Vec<usize> = (0..triggers.len()).collect();
                    state.submit_decision(Decision::Order(order)).unwrap();
                }
                StepOutcome::NeedsDecision(PendingDecision::ChooseReplacement(
                    crate::decide::pending::ChooseReplacement { applicable, .. },
                )) => {
                    // Apply madness first — its self-replacement is sourced from
                    // the discarded card, which is OFF the battlefield (the
                    // graveyard-hoser sibling is a battlefield permanent).
                    let key = applicable
                        .iter()
                        .copied()
                        .find(|k| madness_source_off_field(state, *k))
                        .unwrap_or(applicable[0]);
                    state
                        .submit_decision(Decision::ReplacementChoice(key))
                        .unwrap();
                }
                StepOutcome::NeedsDecision(other) => {
                    panic!("unexpected decision while madness resolves: {other:?}")
                }
                StepOutcome::GameOver(_) => break,
            }
        }
    }

    /// [CR#702.35a,701.9c]: discarding a card with madness EXILES it instead of
    /// putting it in the graveyard, YET the discard name-fact still stands — so
    /// Megrim / "whenever you discard a card" fires (loses its controller 2
    /// life) even though the content committed to exile. The card's own hand-
    /// functioning self-replacement is what redirects it (the static-ability-
    /// zone gather closes the seam the older `madness_style_*` fixture noted).
    #[test]
    fn madness_discard_exiles_yet_fires_megrim() {
        let (mut state, _a) = bear_on_field();
        megrim_fixture(&mut state);
        let life_before = state.player(PlayerId(0)).life;
        let card = mint_in_hand_with(&mut state, PlayerId(0), madness_creature("Madcap Skills"));

        // "Discard this card" (the cycling-cost / bound shape): the frame's
        // source IS the hand card, so `This` = the discarded card.
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);

        assert!(
            zone_has_named(&state, &state.zones.exile, "Madcap Skills"),
            "madness exiles the discarded card instead of the graveyard ([CR#702.35a])"
        );
        assert!(
            !zone_has_named(&state, &state.zones.graveyards[0], "Madcap Skills"),
            "the replaced discard never reaches the graveyard"
        );
        assert!(
            discarded_fact(&state, card),
            "the discard name-fact survives the destination redirect ([CR#701.9c])"
        );

        // Drive the fired Megrim + madness triggers to resolution, declining
        // the cast so only Megrim's life loss lands.
        drive_declining_or_casting(&mut state, false);
        assert_eq!(
            state.player(PlayerId(0)).life,
            life_before - 2,
            "Megrim fired exactly once off the redirected discard ([CR#701.9c])"
        );
    }

    /// A Rest-in-Peace-shape static: "if a card would be put into a graveyard,
    /// exile it instead" ([CR#614.6]) — a `ZoneChange → Graveyard` replacement.
    fn rest_in_peace_fixture(state: &mut GameState) {
        let ability = builtin()
            .macros
            .read_str::<Ability>(
                "Static(Replacement(Instead(would: ZoneChange(what: Any, to: Graveyard), \
                 instead: Move(EventObject, Exile))))",
            )
            .unwrap();
        mint_on_field(
            state,
            Card::Normal(CardFace {
                name: "Rest in Peace Fixture".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![ability],
                ..CardFace::default()
            }),
        );
    }

    /// [CR#702.35a,614.6]: declining the madness cast moves the exiled card
    /// Exile → graveyard — an ORDINARY future move, so a Rest-in-Peace-shape
    /// "would be put into a graveyard, exile it instead" static bites it and
    /// re-exiles it (the madness + graveyard-hoser ruling falls out for free).
    #[test]
    fn declined_madness_cast_graveyard_move_is_bitten_by_rest_in_peace() {
        let (mut state, _a) = bear_on_field();
        rest_in_peace_fixture(&mut state);
        let card = mint_in_hand_with(&mut state, PlayerId(0), madness_creature("Madcap Skills"));
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);

        // The discard is contested by TWO replacements — madness and Rest in
        // Peace both send the card off its Hand → graveyard shape — so a
        // [CR#616.1] ChooseReplacement surfaces to the discarding player; the
        // driver applies madness first, then declines the cast. The declined
        // Exile → graveyard move is itself caught by Rest in Peace and
        // re-exiled — the card never reaches the graveyard.
        assert!(
            matches!(
                state.pending,
                Some(crate::decide::PendingDecision::ChooseReplacement(crate::decide::pending::ChooseReplacement { chooser, .. }))
                    if chooser == PlayerId(0)
            ),
            "the discarding player chooses the replacement order ([CR#616.1])"
        );
        drive_declining_or_casting(&mut state, false);
        assert!(
            !zone_has_named(&state, &state.zones.graveyards[0], "Madcap Skills"),
            "Rest in Peace exiled the declined card's graveyard move ([CR#614.6])"
        );
        assert!(
            zone_has_named(&state, &state.zones.exile, "Madcap Skills"),
            "the card stays exiled — the graveyard move was replaced"
        );
    }

    /// [CR#616.1]: a madness discard while a graveyard-hoser (Leyline of the
    /// Void / Rest in Peace shape) is out — two replacements would both send
    /// the card off its Hand → graveyard shape, so the discarding player
    /// chooses which applies. EITHER order exiles the card (madness first: to
    /// its own trigger; the hoser first: straight out) — the choice never lets
    /// it slip to the graveyard.
    #[test]
    fn madness_and_graveyard_hoser_are_an_owner_ordered_choice() {
        for madness_first in [true, false] {
            let (mut state, _a) = bear_on_field();
            rest_in_peace_fixture(&mut state);
            let card =
                mint_in_hand_with(&mut state, PlayerId(0), madness_creature("Madcap Skills"));
            let frame = frame_src(card);
            state.run_effect(
                OneShotEffect::Act(Action::discard_what(Reference::This)),
                &frame,
            );
            run_injected(&mut state);

            let Some(crate::decide::PendingDecision::ChooseReplacement(
                crate::decide::pending::ChooseReplacement {
                    chooser,
                    applicable,
                },
            )) = state.pending.clone()
            else {
                panic!("two applicable replacements surface an order choice ([CR#616.1])");
            };
            assert_eq!(chooser, PlayerId(0), "the discarding player chooses");
            assert_eq!(applicable.len(), 2, "madness and the hoser both apply");
            let key = applicable
                .iter()
                .copied()
                .find(|k| madness_source_off_field(&state, *k) == madness_first)
                .unwrap();
            state
                .submit_decision(Decision::ReplacementChoice(key))
                .unwrap();
            run_injected(&mut state);

            assert!(
                zone_has_named(&state, &state.zones.exile, "Madcap Skills"),
                "either order exiles the card (madness_first={madness_first})"
            );
            assert!(
                !zone_has_named(&state, &state.zones.graveyards[0], "Madcap Skills"),
                "neither order lets the card reach the graveyard (madness_first={madness_first})"
            );
        }
    }

    /// A creature FACE carrying TWO madness keywords ({1}{R} and {2}{B}) — the
    /// Falkenrath-Gorger-plus-own-madness "choose one" shape ([CR#702.35a]).
    fn double_madness_creature(name: &str) -> CardFace {
        let mut face = madness_creature(name);
        face.abilities
            .push(keyword("Madness([Mana([Generic(2), Black])])"));
        face
    }

    /// [CR#702.35a,616.1]: a card with TWO madness abilities discarded — both
    /// self-replacements apply, the owner chooses one, and it applies EXACTLY
    /// once (an `Instead` replaces the discard away, so the other never also
    /// fires): one exile, one delayed trigger.
    #[test]
    fn stacked_madness_applies_exactly_once() {
        let (mut state, _a) = bear_on_field();
        let card = mint_in_hand_with(
            &mut state,
            PlayerId(0),
            double_madness_creature("Madcap Skills"),
        );
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);

        let Some(crate::decide::PendingDecision::ChooseReplacement(
            crate::decide::pending::ChooseReplacement { applicable, .. },
        )) = state.pending.clone()
        else {
            panic!("two madness self-replacements surface an order choice ([CR#616.1])");
        };
        assert_eq!(applicable.len(), 2, "both madness abilities apply");
        state
            .submit_decision(Decision::ReplacementChoice(applicable[0]))
            .unwrap();
        run_injected(&mut state);

        assert!(
            zone_has_named(&state, &state.zones.exile, "Madcap Skills"),
            "the card exiled once"
        );
        // EXACTLY ONE exile move for the card ([CR#616.1] — one Instead replaces
        // the discard away; the other madness never also fires).
        let exiles = state
            .history
            .entries()
            .filter(|e| matches!(&e.fact, GameEvent::ZoneChange(ZoneChange { to, .. }) if *to == Zone::Exile))
            .count();
        assert_eq!(
            exiles, 1,
            "one madness applied — a single Hand → Exile move"
        );

        // Exactly ONE madness cast trigger — driving the stack offers the "you
        // may cast it" decision once, not twice ([CR#702.35a]).
        let offers = count_cast_offers(&mut state);
        assert_eq!(
            offers, 1,
            "one madness cast trigger, not two ([CR#702.35a])"
        );
    }

    /// Drive the stack to empty, DECLINING every "you may cast it" offer and
    /// counting how many surfaced.
    fn count_cast_offers(state: &mut GameState) -> usize {
        use crate::decide::Action as Act;
        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;
        let mut offers = 0;
        for _ in 0..200 {
            // Keep enough mana floating to afford either madness cost ({1}{R} or
            // {2}{B}) — else an unpayable cast auto-declines with no offer to
            // count ([CR#608.2g]). Mana empties at each step boundary.
            if state.player(PlayerId(0)).mana_pool.is_empty() {
                grant_red(state, PlayerId(0), 2);
                state
                    .player_mut(PlayerId(0))
                    .mana_pool
                    .add(deckmaste_core::Color::Black.into(), 1);
            }
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(PendingDecision::Priority(
                    crate::decide::pending::Priority { .. },
                )) => {
                    if state.stack.is_empty() {
                        break;
                    }
                    state.submit_decision(Decision::Act(Act::Pass)).unwrap();
                }
                StepOutcome::NeedsDecision(PendingDecision::YesNo(
                    crate::decide::pending::YesNo { .. },
                )) => {
                    offers += 1;
                    state.submit_decision(Decision::Answer(false)).unwrap();
                }
                StepOutcome::NeedsDecision(PendingDecision::OrderTriggers(
                    crate::decide::pending::OrderTriggers { triggers, .. },
                )) => {
                    let order: Vec<usize> = (0..triggers.len()).collect();
                    state.submit_decision(Decision::Order(order)).unwrap();
                }
                StepOutcome::NeedsDecision(other) => {
                    panic!("unexpected decision counting cast offers: {other:?}")
                }
                StepOutcome::GameOver(_) => break,
            }
        }
        offers
    }

    /// [CR#702.35a,118.9]: casting a madness card. The exiled card's delayed
    /// trigger offers "you may cast it for the madness cost {1}{R}"; accepting
    /// pays that alternative cost (NOT the printed {3}{R}) out of the pool and
    /// puts the creature on the stack, whence it resolves onto the battlefield
    /// — the exile is emptied.
    #[test]
    fn madness_cast_pays_the_alternative_cost_and_exile_empties() {
        let (mut state, _a) = bear_on_field();
        // Two red covers {1}{R} (one Red pip + one Red spent as generic); the
        // printed {3}{R} would need four — proving the alternative cost is used.
        grant_red(&mut state, PlayerId(0), 2);
        let card = mint_in_hand_with(&mut state, PlayerId(0), madness_creature("Madcap Skills"));
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            zone_has_named(&state, &state.zones.exile, "Madcap Skills"),
            "the discarded madness card is exiled first ([CR#702.35a])"
        );

        // Accept the cast; drive the stack (the spell resolves onto the field).
        // The pool is topped to exactly {1}{R} while we wait — never the four
        // mana the printed {3}{R} needs — so a resolving spell proves the
        // alternative cost was the one paid (the driver also asserts the cost).
        drive_declining_or_casting(&mut state, true);

        assert!(
            !zone_has_named(&state, &state.zones.exile, "Madcap Skills"),
            "casting empties the exile — the card left for the stack ([CR#608.2g])"
        );
        assert!(
            zone_has_named(&state, &state.zones.battlefield, "Madcap Skills"),
            "the 2/2 resolved onto the battlefield ([CR#608.2g])"
        );
        assert!(
            !zone_has_named(&state, &state.zones.graveyards[0], "Madcap Skills"),
            "a cast card never fell to the graveyard"
        );
    }

    // ===== Conferred-ability gathering (Task 10b) =====
    // The layer system already DERIVES conferred abilities (including
    // off-battlefield, via predicate-scoped `ConferralRule`s), but the
    // replacement gather and trigger scan used to read the PRINTED-only spine,
    // so a conditionally-conferred replacement/trigger never fired. These three
    // exercise the derived-ability read in both scans.

    /// [CR#616.1,702.35a]: a Falkenrath-Gorger-shape static confers a madness
    /// self-replacement onto an OWNED, off-battlefield, otherwise-VANILLA card.
    /// Discarding it opens the [CR#616.1] window off the CONFERRED ability (the
    /// card prints no madness), redirecting its Hand → Graveyard move to exile
    /// while the discard name-fact still stands ([CR#701.9c]). Without the
    /// derived-ability self-replacement gather, no window opens and the card
    /// falls straight to the graveyard.
    #[test]
    fn conferred_madness_off_battlefield_opens_the_replacement_window() {
        use deckmaste_core::ConferralRule;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Property;
        use deckmaste_core::Replacement;
        use deckmaste_core::StatePredicate;
        use deckmaste_core::VerbName;

        let (mut state, _a) = bear_on_field();
        // "If a player would discard THIS card, exile it instead." — conferred,
        // not printed, onto a matching card that isn't on the battlefield.
        let madness =
            Ability::r#static(StaticEffect::Replacement(Arc::new(Replacement::Instead {
                would: EventFilter::Act {
                    verb: VerbName::from("Discard"),
                    who: Predicate::Any,
                    on: Predicate::Ref(Reference::This),
                    cause: None,
                },
                instead: OneShotEffect::Act(Action::move_to(Reference::EventObject, Zone::Exile)),
            })));
        state.conferral_rules = vec![ConferralRule {
            scope: Predicate::And(vec![
                Predicate::Characteristic(CharacteristicPredicate::Named(
                    "Conferred Vampire".into(),
                )),
                Predicate::Not(Arc::new(Predicate::State(StatePredicate::InZone(
                    Zone::Battlefield,
                )))),
            ]),
            confer: Property::Ability(Arc::new(madness)),
        }];

        let card = mint_in_hand_with(
            &mut state,
            PlayerId(0),
            CardFace {
                name: "Conferred Vampire".into(),
                types: vec![Type::Creature.def()],
                ..CardFace::default()
            },
        );
        // Bound discard ("discard this card"): the frame's source IS the card.
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);

        assert!(
            zone_has_named(&state, &state.zones.exile, "Conferred Vampire"),
            "the CONFERRED madness self-replacement opened the [CR#616.1] window \
             and exiled the discarded card ([CR#702.35a])"
        );
        assert!(
            !zone_has_named(&state, &state.zones.graveyards[0], "Conferred Vampire"),
            "the replaced discard never reaches the graveyard"
        );
        assert!(
            discarded_fact(&state, card),
            "the discard name-fact survives the destination redirect ([CR#701.9c])"
        );
    }

    /// [CR#616.1]: a "lord" confers "if this would be destroyed, exile it
    /// instead" onto a matching BATTLEFIELD creature — a CONFERRED (not
    /// printed) replacement. Destroying the creature exiles it instead.
    /// Without the derived-ability battlefield sweep, the conferred shield
    /// is invisible and the creature is destroyed normally.
    #[test]
    fn conferred_destroy_replacement_on_the_battlefield_fires() {
        use deckmaste_core::ConferralRule;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Property;
        use deckmaste_core::Replacement;
        use deckmaste_core::VerbName;

        let (mut state, _a) = bear_on_field();
        let shield = Ability::r#static(StaticEffect::Replacement(Arc::new(Replacement::Instead {
            would: EventFilter::Act {
                verb: VerbName::from("Destroy"),
                who: Predicate::Any,
                on: Predicate::Ref(Reference::This),
                cause: None,
            },
            instead: OneShotEffect::Act(Action::move_to(Reference::EventObject, Zone::Exile)),
        })));
        state.conferral_rules = vec![ConferralRule {
            scope: Predicate::Characteristic(CharacteristicPredicate::Named("Destroy Host".into())),
            confer: Property::Ability(Arc::new(shield)),
        }];

        let host = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Destroy Host".into(),
                types: vec![Type::Creature.def()],
                ..CardFace::default()
            }),
        );
        let frame = frame_src(host);
        state.run_effect(OneShotEffect::Act(Action::destroy(Reference::This)), &frame);
        run_injected(&mut state);

        assert!(
            zone_has_named(&state, &state.zones.exile, "Destroy Host"),
            "the CONFERRED destroy-replacement exiled the creature ([CR#616.1])"
        );
        assert!(
            state.zones.graveyards[0].is_empty(),
            "the conferred replacement kept the creature out of the graveyard"
        );
        assert!(
            !zone_has_named(&state, &state.zones.battlefield, "Destroy Host"),
            "the creature left the battlefield — the destroy still committed a move"
        );
    }

    /// [CR#603.2]: a Megrim-shape trigger CONFERRED (not printed) onto a
    /// battlefield creature — "whenever you discard a card, lose 2 life" —
    /// fires off a discard and resolves its life loss. Without the
    /// derived-ability trigger scan, the conferred trigger is never
    /// enumerated and never fires.
    #[test]
    fn conferred_trigger_on_the_battlefield_fires() {
        use deckmaste_core::ConferralRule;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Property;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::VerbName;

        let (mut state, _a) = bear_on_field();
        let trigger = Ability::triggered(TriggeredAbility {
            ability_word: None,
            where_x: None,
            from: None,
            event: EventFilter::Act {
                verb: VerbName::from("Discard"),
                who: Predicate::Ref(Reference::You),
                on: Predicate::Any,
                cause: None,
            },
            condition: None,
            limits: Vec::new(),
            effect: OneShotEffect::Act(Action::by_you(PlayerAction::LoseLife(Count::Literal(2)))),
        });
        state.conferral_rules = vec![ConferralRule {
            scope: Predicate::Characteristic(CharacteristicPredicate::Named("Trigger Host".into())),
            confer: Property::Ability(Arc::new(trigger)),
        }];

        let _host = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Trigger Host".into(),
                types: vec![Type::Creature.def()],
                ..CardFace::default()
            }),
        );
        let life_before = state.player(PlayerId(0)).life;

        // Discard a plain hand card (bound; the discarding player is player 0).
        let card = mint_in_hand(&mut state, PlayerId(0), "Fodder");
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);
        // Drive the fired conferred trigger through the stack to resolution
        // (declining any offers — there are none here but the driver is shared).
        drive_declining_or_casting(&mut state, false);

        assert_eq!(
            state.player(PlayerId(0)).life,
            life_before - 2,
            "the CONFERRED trigger fired off the discard and resolved its life loss"
        );
    }

    // ===== Canon madness pair (Task 11) =====
    // Behavior of the two shipped canon cards, driven against the REAL card
    // data loaded from the canon plugin.

    /// [CR#616.1,702.35a]: STACKED madness applies once — the Falkenrath Gorger
    /// choose-one ruling with zero authored guard. A Vampire discarded from
    /// hand that BOTH prints madness AND is granted madness by Gorger's static
    /// (via the [CR#613] layer view, folded into the derived-ability scan) has
    /// two applicable `Instead` self-replacements; only ONE applies (an
    /// `Instead` replaces the discard intent entirely, so the sibling has
    /// nothing left to match, [CR#616.1f]) — a single exile, a single madness
    /// window.
    #[test]
    fn gorger_stacked_madness_applies_once() {
        let (mut state, _a) = bear_on_field();
        // Empty P0's opening hand (retire the cards to the library) so the
        // discard target is the only card in hand.
        for o in state.zones.hands[0].clone() {
            state.objects.obj_mut(o).zone = Some(Zone::Library);
            state.zones.libraries[0].push_back(o);
        }
        state.zones.hands[0].clear();
        // Falkenrath Gorger on the battlefield (P0's) — its card static grants
        // madness to P0's off-battlefield Vampires through the layer view, which
        // `derived_abilities_of` folds into the replacement scan. No conferral
        // rules are installed: the grant is the layer path, not a global rule.
        let _gorger = mint_on_field(&mut state, canon().card("Falkenrath Gorger").unwrap());
        // A Vampire in hand that ALSO prints its own madness — the two-source
        // stack.
        let vampire = mint_in_hand_with(
            &mut state,
            PlayerId(0),
            CardFace {
                name: "Printed-Madness Vampire".into(),
                types: vec![Type::Creature.def()],
                subtypes: vec![subtype("Vampire")],
                abilities: vec![keyword("Madness([Mana([Red])])")],
                ..CardFace::default()
            },
        );
        let frame = frame_src(vampire);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);

        // Two applicable madness self-replacements now surface a [CR#616.1]
        // replacement-selection: the card's PRINTED madness and the one Gorger's
        // static GRANTS it through the layer view. (Before the grant functioned,
        // only the printed one applied and no choice surfaced — the stack was
        // never actually exercised.) Both are sourced from the same
        // off-battlefield card, so whichever applies first exiles it and the
        // sibling then has nothing left to replace ([CR#616.1f]).
        let Some(crate::decide::PendingDecision::ChooseReplacement(
            crate::decide::pending::ChooseReplacement {
                chooser,
                applicable,
            },
        )) = state.pending.clone()
        else {
            panic!("the printed + Gorger-granted madness both apply — a [CR#616.1] order choice")
        };
        assert_eq!(
            chooser,
            PlayerId(0),
            "the discarding player chooses ([CR#616.1])"
        );
        assert_eq!(
            applicable.len(),
            2,
            "both the printed and the Gorger-granted madness self-replacement apply"
        );
        state
            .submit_decision(Decision::ReplacementChoice(applicable[0]))
            .unwrap();
        run_injected(&mut state);

        let in_exile = state
            .zones
            .exile
            .iter()
            .filter(|&&o| matches!(state.def(o), Card::Normal(f) if f.name == "Printed-Madness Vampire"))
            .count();
        assert_eq!(
            in_exile, 1,
            "exactly ONE exile — stacked madness applies once ([CR#616.1,702.35a])"
        );
        assert!(
            !zone_has_named(
                &state,
                &state.zones.graveyards[0],
                "Printed-Madness Vampire"
            ),
            "a single Instead applied — the card did not also fall to the graveyard"
        );
        assert!(
            discarded_fact(&state, vampire),
            "the discard name-fact stands under the redirect ([CR#701.9c])"
        );
    }

    /// A vanilla (no printed madness) Vampire creature card — its only possible
    /// madness source is Falkenrath Gorger's conferral.
    fn vanilla_vampire(name: &str) -> CardFace {
        CardFace {
            name: name.into(),
            types: vec![Type::Creature.def()],
            subtypes: vec![subtype("Vampire")],
            ..CardFace::default()
        }
    }

    /// Whether the layer view grants `id` a madness-shaped self-replacement —
    /// flatten the derived ability list (splicing the granted `Madness`
    /// composite keyword) and look for its `Static(Replacement)` member.
    fn grants_madness_replacement(
        view: &crate::layer::LayeredView,
        id: crate::object::ObjectId,
    ) -> bool {
        let mut flat = Vec::new();
        for a in view.get(id).abilities.iter() {
            crate::derive::flatten_composites(a, &mut flat);
        }
        flat.iter().any(|a| {
            matches!(a, Ability::Static(s) if matches!(s.as_ref(), StaticEffect::Replacement(_)))
        })
    }

    /// [CR#613,616.1,702.35a]: Falkenrath Gorger's printed static grants madness
    /// through the [CR#613] LAYER view, scoped exactly — active only while the
    /// Gorger is on the battlefield ([CR#611.3b]) and reaching only its
    /// CONTROLLER'S OWN off-battlefield Vampires (`Owner(Ref(You))`). Two
    /// players each hold a vanilla off-battlefield Vampire; only P0
    /// controls a Gorger. The layer view's derived ability list gains the
    /// madness self-replacement for P0's Vampire ONLY — proving the grant
    /// does not leak to every off-battlefield Vampire in the game (the bug
    /// a scope-less global `ConferralRule` would cause).
    #[test]
    fn gorger_madness_grant_is_owner_scoped_in_the_layer_view() {
        let (mut state, _a) = bear_on_field();
        let _gorger = mint_on_field(&mut state, canon().card("Falkenrath Gorger").unwrap());
        let mine = mint_in_hand_with(&mut state, PlayerId(0), vanilla_vampire("P0 Vampire"));
        let theirs = mint_in_hand_with(&mut state, PlayerId(1), vanilla_vampire("P1 Vampire"));

        let view = state.layers();
        assert!(
            grants_madness_replacement(&view, mine),
            "P0's off-battlefield Vampire gains Gorger's madness self-replacement \
             via the layer view (its owner controls the Gorger)"
        );
        assert!(
            !grants_madness_replacement(&view, theirs),
            "P1's off-battlefield Vampire does NOT gain madness — the grant is \
             owner-scoped ([CR#613], `Owner(Ref(You))`), not a global leak"
        );
    }

    /// [CR#616.1,702.35a]: the behavioral proof of owner-scoping, through the
    /// wired derived-ability replacement scan (`derived_abilities_of` folding
    /// the layer-granted madness). Only P0 controls a Gorger; both players
    /// discard their off-battlefield Vampire. P0's opens the [CR#616.1]
    /// madness window and exiles; P1's has no window and falls to the
    /// graveyard — the grant reaches exactly the Gorger controller's own
    /// cards.
    #[test]
    fn gorger_madness_window_opens_only_for_its_controllers_vampire() {
        let (mut state, _a) = bear_on_field();
        let _gorger = mint_on_field(&mut state, canon().card("Falkenrath Gorger").unwrap());
        let mine = mint_in_hand_with(&mut state, PlayerId(0), vanilla_vampire("P0 Vampire"));
        let theirs = mint_in_hand_with(&mut state, PlayerId(1), vanilla_vampire("P1 Vampire"));

        // Discard P0's Vampire: the conferred madness self-replacement redirects
        // its Hand → Graveyard to Exile ([CR#702.35a]).
        let frame = frame_src(mine);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);
        // Discard P1's Vampire: P1 controls no Gorger, so no window opens.
        let frame = frame_src(theirs);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);

        assert!(
            zone_has_named(&state, &state.zones.exile, "P0 Vampire"),
            "P0's Vampire (owner controls the Gorger) opened the madness window \
             and exiled ([CR#702.35a])"
        );
        assert!(
            zone_has_named(&state, &state.zones.graveyards[1], "P1 Vampire"),
            "P1's Vampire fell to the graveyard — no Gorger of P1's, no window"
        );
        assert!(
            !zone_has_named(&state, &state.zones.exile, "P1 Vampire"),
            "P1's Vampire did NOT exile — the grant did not leak across owners"
        );
    }

    /// [CR#121.2,616.1,702.35a]: Anje's Ravager's attack trigger — "discard your
    /// hand, then draw three cards" — discards every card in hand as its own
    /// `Act(Discard)` (a madness card among them opens its [CR#702.35a] window
    /// and exiles; a plain card reaches the graveyard) and then Batch-draws
    /// three ([CR#121.2]). Driven off the card's OWN authored trigger effect
    /// (the attack declaration that fires it is the `ThisAttacks` event macro,
    /// exercised with the trigger family).
    #[test]
    fn anje_ravager_attack_trigger_discards_hand_and_draws_three() {
        let (mut state, _a) = bear_on_field();
        // Retire P0's opening hand into the library (clears the hand AND leaves
        // the library deep enough to draw three).
        for o in state.zones.hands[0].clone() {
            state.objects.obj_mut(o).zone = Some(Zone::Library);
            state.zones.libraries[0].push_back(o);
        }
        state.zones.hands[0].clear();
        // Anje's Ravager on the battlefield — the trigger's source.
        let anje = mint_on_field(&mut state, canon().card("Anje's Ravager").unwrap());
        // A two-card hand: a madness card + a plain card.
        let mad = mint_in_hand_with(
            &mut state,
            PlayerId(0),
            CardFace {
                name: "Hand Madness".into(),
                types: vec![Type::Creature.def()],
                abilities: vec![keyword("Madness([Mana([Red])])")],
                ..CardFace::default()
            },
        );
        let plain = mint_in_hand(&mut state, PlayerId(0), "Hand Plain");
        // Anje's OWN attack-trigger effect (discard your hand, then draw three).
        let effect = {
            let Card::Normal(face) = state.def(anje) else {
                panic!("Anje's Ravager is single-faced")
            };
            face.abilities
                .iter()
                .find_map(|a| match a {
                    Ability::Triggered(t) => Some(t.effect.clone()),
                    _ => None,
                })
                .expect("Anje's Ravager has an attack trigger")
        };
        let frame = frame_src(anje);
        state.run_effect(effect, &frame);
        run_injected(&mut state);

        assert!(
            discarded_fact(&state, mad),
            "the madness card was discarded (its own Act(Discard))"
        );
        assert!(
            discarded_fact(&state, plain),
            "the plain card was discarded (its own Act(Discard))"
        );
        assert!(
            zone_has_named(&state, &state.zones.exile, "Hand Madness"),
            "the discarded madness card opened its window and exiled ([CR#702.35a])"
        );
        assert!(
            zone_has_named(&state, &state.zones.graveyards[0], "Hand Plain"),
            "the plain discard reached the graveyard ([CR#701.9a])"
        );
        assert_eq!(
            state.zones.hands[0].len(),
            3,
            "drew three after emptying the hand ([CR#121.2])"
        );
    }

    /// [CR#702.29a]: the BOUND discard ("discard this card") — no choice
    /// surfaces; the one dual-facet `Act(Discard)` commits the named card's
    /// Hand → Graveyard move atomically, exactly the destroy shape.
    #[test]
    fn discard_what_commits_the_bound_card_without_a_choice() {
        let (mut state, _a) = bear_on_field();
        let card = mint_in_hand(&mut state, PlayerId(0), "Cycled Card");
        // Cycling's frame: the ability's source IS the hand card (`This`).
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::discard_what(Reference::This)),
            &frame,
        );
        run_injected(&mut state);

        assert!(
            state.pending.is_none(),
            "the bound form surfaces no card choice ([CR#702.29a])"
        );
        assert!(
            zone_has_named(&state, &state.zones.graveyards[0], "Cycled Card"),
            "the named card was discarded"
        );
        assert!(
            !state.zones.hands[0].contains(&card),
            "the named card left the hand"
        );
        let acts = state
            .history
            .entries()
            .filter(|e| {
                matches!(&e.fact, crate::event::GameEvent::Act(Act { verb, on, .. })
                    if verb.as_str() == "Discard" && on.as_slice() == [card])
            })
            .count();
        assert_eq!(acts, 1, "ONE dual-facet Act(Discard) commits the move");
    }

    /// [CR#614.17]: a "can't discard" static (`CantHappen(Act(Discard(…)))`)
    /// suppresses the whole keyword action at schedule time — no choice
    /// surfaces, no event, no trigger.
    #[test]
    fn cant_discard_suppresses_the_whole_action() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::VerbName;

        let (mut state, a) = bear_on_field();
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "No Discards".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![Ability::r#static(StaticEffect::CantHappen(
                    EventFilter::Act {
                        verb: VerbName::from("Discard"),
                        who: Predicate::Any,
                        on: Predicate::Any,
                        cause: None,
                    },
                ))],
                ..CardFace::default()
            }),
        );
        let hand_before = state.zones.hands[0].len();
        let frame = frame_src(a);
        state.run_effect(
            OneShotEffect::Act(Action::discard(Reference::You, Count::Literal(1), false)),
            &frame,
        );
        run_injected(&mut state);

        assert!(state.pending.is_none(), "no card choice surfaces");
        assert_eq!(
            state.zones.hands[0].len(),
            hand_before,
            "no card left the hand"
        );
        assert!(
            !state.history.entries().any(|e| {
                matches!(&e.fact, crate::event::GameEvent::Act(Act { verb, .. })
                    if verb.as_str() == "Discard")
            }),
            "a canted discard performs no keyword action — no fact, no trigger"
        );
    }

    /// Cost-position discard ("Discard a card:", e.g. Blood token's
    /// activation cost, [CR#111.10g,701.9,601.2b]) rides the SAME
    /// `OneShotEffect::Act(Action::discard(..))` shape `verb_payment_items`
    /// (`cast.rs`) builds for a `CostComponent::Do(..)` verb — so the
    /// `ChooseObjects` decision surfaces and pays exactly as it does in
    /// effect position, and the committed `Act(Discard)` fact still fires a
    /// Megrim-shape "whenever you discard a card" trigger
    /// (`Act(Discard(Ref(You), Any))`) exactly once.
    #[test]
    fn discard_via_cost_shape_pays_and_fires_the_discard_trigger_once() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::VerbName;

        use crate::step::StepOutcome;

        let (mut state, a) = bear_on_field();
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Megrim Fixture".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![Ability::triggered(TriggeredAbility {
                    ability_word: None,
                    where_x: None,
                    from: None,
                    event: EventFilter::Act {
                        verb: VerbName::from("Discard"),
                        who: Predicate::Ref(Reference::You),
                        on: Predicate::Any,
                        cause: None,
                    },
                    condition: None,
                    limits: Vec::new(),
                    effect: OneShotEffect::Act(Action::by_you(PlayerAction::LoseLife(
                        Count::Literal(2),
                    ))),
                })],
                ..CardFace::default()
            }),
        );
        let hand_before = state.zones.hands[0].len();
        let life_before = state.player(PlayerId(0)).life;
        let frame = frame_src(a);
        // The shape `verb_payment_items` builds for a cost-position
        // `Do(Action::discard(..))`: `WorkItem::RunEffect{Act(discard), frame}`.
        state.run_effect(
            OneShotEffect::Act(Action::discard(Reference::You, Count::Literal(1), false)),
            &frame,
        );
        let pending = (0..5)
            .find_map(|_| match state.step() {
                StepOutcome::NeedsDecision(p) => Some(p),
                _ => None,
            })
            .expect("a ChooseObjects decision surfaces within a few steps");
        let PendingDecision::ChooseObjects(crate::decide::pending::ChooseObjects {
            candidates,
            min,
            ..
        }) = pending
        else {
            panic!("expected the batched card choice, got {pending:?}");
        };
        assert_eq!(min, 1, "one card to discard");
        state
            .submit_decision(Decision::Chosen(candidates[..1].to_vec()))
            .unwrap();
        run_injected(&mut state);

        assert_eq!(
            state.zones.hands[0].len(),
            hand_before - 1,
            "the cost-shape discard paid — one card left the hand"
        );

        // The discard is paid; drive priority so the Megrim-shape trigger it
        // fired can resolve.
        for _ in 0..30 {
            if state.player(PlayerId(0)).life != life_before {
                break;
            }
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(PendingDecision::Priority(
                    crate::decide::pending::Priority { .. },
                )) => {
                    state
                        .submit_decision(Decision::Act(crate::decide::Action::Pass))
                        .unwrap();
                }
                StepOutcome::NeedsDecision(other) => {
                    panic!("unexpected decision while the trigger resolves: {other:?}")
                }
                StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
            }
        }
        assert_eq!(
            state.player(PlayerId(0)).life,
            life_before - 2,
            "the Megrim-shape trigger fired exactly once"
        );
    }

    // --- P0.W5 resolution note slots ([CR#608.2c,607.2]) --------------------

    /// [CR#120.1,701.14a]: `DealDamage`'s explicit `source` is the dealer — the
    /// emitted `DamageDealt` carries it, NOT `frame.source`. The fight shape:
    /// `b` (the "second" slot) deals damage equal to its power to `a` (the
    /// "first" slot), with the frame source set to `a`.
    #[test]
    fn deal_damage_uses_explicit_source_not_frame_source() {
        let (mut state, a, b) = two_permanents_on_field();
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            OneShotEffect::Act(Action::DealDamage(
                Reference::Target(1),
                Count::StatOf(Reference::Target(1), deckmaste_core::Stat::Power),
                Reference::Target(0),
            )),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(a).total_damage(),
            2,
            "a took b's power (2) in damage"
        );
        assert!(
            logged(&state, |e| matches!(
                e,
                GameEvent::DamageDealt(DamageDealt { source, target, amount, .. })
                    if *source == b && *target == a && *amount == 2
            )),
            "DamageDealt carries the explicit source b, not frame.source a"
        );
    }

    /// `By(You, GainLife(3))` → one `LifeGained`; `By(You, Untap(This))` → one
    /// `Untapped` — the mirrors of `LoseLife`/`Tap` above. The bear is tapped
    /// first: untapping is transition-only ([CR#701.26b]).
    #[test]
    fn action_items_for_gainlife_untap() {
        let (mut state, src) = bear_on_field();
        state.objects.obj_mut(src).tapped = true;
        let frame = frame_src(src);

        let items = state.action_items(
            &Action::by_you(PlayerAction::GainLife(Count::Literal(3))),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained(
                LifeGained {
                    player: PlayerId(0),
                    amount: 3,
                }
            )))]
        );

        let items = state.action_items(
            &Action::by_you(PlayerAction::Untap(Reference::This)),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Untapped(src)))]
        );
    }

    /// [CR#122.1]: `PutCounters(This, P1P1Counter, 2)` emits one `CounterPlaced`
    /// per selected object, carrying the effect-instruction cause
    /// (events.md §3) — its agent is the resolving source's controller.
    /// Counter kinds are bare `CounterRef` idents, not symbolic strings.
    #[test]
    fn put_counters_emits_counter_placed() {
        use deckmaste_core::Agency;

        use crate::event::Cause;

        let (state, bear) = bear_on_field();
        let frame = frame_src(bear);
        let items = state.action_items(
            &Action::by_you(PlayerAction::PutCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(
                GameEvent::CounterPlaced(CounterPlaced {
                    object: bear,
                    kind: "P1P1Counter".into(),
                    amount: 2,
                    before: 0,
                    after: 0,
                    cause: Some(Cause::put_counters(
                        Agency::EffectInstruction,
                        Some((bear, PlayerId(0))),
                    )),
                })
            ))]
        );
    }

    /// [CR#122.1]: putting zero counters is a no-op — no event (so no
    /// "counter is put on" trigger fires for nothing).
    #[test]
    fn put_zero_counters_emits_nothing() {
        let (state, bear) = bear_on_field();
        let frame = frame_src(bear);
        let items = state.action_items(
            &Action::by_you(PlayerAction::PutCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(0),
            )),
            &frame,
        );
        assert_eq!(items, vec![]);
    }

    /// Applying `CounterPlaced` adds to the object's counter map, and a second
    /// placement of the same kind sums ([CR#122.1] — counters are
    /// interchangeable).
    #[test]
    fn counter_placed_apply_is_additive() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 1);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::PutCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterPlaced
        assert_eq!(
            state
                .objects
                .obj(bear)
                .counters
                .get(&deckmaste_core::Ident::from("P1P1Counter"))
                .copied(),
            Some(3)
        );
    }

    /// [CR#107.14,122.1]: "you get {E}{E}" is `PutCounters(You, Energy, N)` —
    /// a player-borne counter placement ([CR#122.1] — a counter is a marker on
    /// an object OR player). `Reference::You` resolves to the controller's
    /// proxy object, so the same apply path lands the energy on the PLAYER
    /// (energy sits on the player, [CR#107.14]), and a second gain sums
    /// ([CR#122.1] — counters are interchangeable).
    #[test]
    fn get_energy_adds_counters_to_the_player_proxy() {
        let (mut state, bear) = bear_on_field();
        let proxy = state.player(PlayerId(0)).object;
        let frame = frame_src(bear); // controller is player 0, so `You` = P0's proxy
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::PutCounters(
                Reference::You,
                "Energy".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterPlaced onto the player proxy
        assert_eq!(
            state
                .objects
                .obj(proxy)
                .counters
                .get(&deckmaste_core::Ident::from("Energy"))
                .copied(),
            Some(2),
            "you get {{E}}{{E}} places two energy on the player's proxy"
        );

        // A second "get {E}" sums with the first.
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::PutCounters(
                Reference::You,
                "Energy".into(),
                Count::Literal(1),
            )),
            &frame,
        );
        let _ = state.step();
        assert_eq!(
            state
                .objects
                .obj(proxy)
                .counters
                .get(&deckmaste_core::Ident::from("Energy"))
                .copied(),
            Some(3),
            "energy gains accumulate on the player [CR#122.1]"
        );

        // And `Count::CounterCount(You, Energy)` reads it back — "if you have
        // N energy" ([CR#107.14]).
        assert_eq!(
            state.eval_count(
                &Count::CounterCount(Arc::new(Reference::You), "Energy".into()),
                &frame
            ),
            3,
            "CounterCount(You, Energy) reads the player's energy total"
        );
    }

    /// Removing more counters than present clamps to zero and DROPS the key,
    /// so `HasCounter` and the layer-7c P/T read both see absence ([CR#122.1]).
    #[test]
    fn counter_removed_clamps_and_drops_key() {
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 1);
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::RemoveCounters(
                Reference::This,
                "P1P1Counter".into(),
                Count::Literal(2),
            )),
            &frame,
        );
        let _ = state.step(); // applies CounterRemoved
        assert!(
            !state
                .objects
                .obj(bear)
                .counters
                .contains_key(&deckmaste_core::Ident::from("P1P1Counter")),
            "a counter kind dropped to zero leaves no key behind"
        );
    }

    /// [CR#701.21a]: `Sacrifice(This)` emits the verb fact, which evolves into
    /// the Battlefield→Graveyard move — old id gone, fresh object in the
    /// owner's graveyard.
    #[test]
    fn sacrifice_this_remints_to_owners_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Sacrifice(Reference::This)),
            &frame,
        );
        // Sacrificed → future-form ZoneChange → past-form ZoneChange.
        for _ in 0..3 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.graveyards[0].len(), 1);
        assert_ne!(state.zones.graveyards[0][0], bear, "reminted");
    }

    /// A sacrifice rides the same death pipeline as a destroy: the sacrificed
    /// creature's own dies-trigger fires ([CR#603.6c] — the leaving object
    /// watches its own departure).
    #[test]
    fn sacrifice_fires_the_dying_objects_dies_trigger() {
        let card = Arc::new(canon().card("Footlight Fiend").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
                PlayerConfig {
                    deck: deck(&forest, 10),
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let card_id = state.cards.push(card, PlayerId(0));
        let gob = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(gob);

        let frame = frame_src(gob);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Sacrifice(Reference::This)),
            &frame,
        );
        for _ in 0..10 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert_eq!(
            state.pending_triggers.len(),
            1,
            "the self-dies trigger must be noted"
        );
        assert!(state.objects.get(gob).is_none(), "the sacrifice happened");
    }

    /// [CR#701.13a,406.2]: exile moves an object to the shared exile zone —
    /// from the battlefield, and (via the graveyard source arm) from a
    /// graveyard.
    #[test]
    fn exile_moves_objects_from_battlefield_and_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Move(
                Reference::This,
                deckmaste_core::Destination::Zone(Zone::Exile),
                vec![],
            )),
            &frame,
        );
        // future-form ZoneChange → past-form ZoneChange.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old id gone");
        assert_eq!(state.zones.exile.len(), 1);
        let exiled = state.zones.exile[0];
        assert_eq!(state.objects.obj(exiled).zone, Some(Zone::Exile));

        // From the graveyard: force a hand card into the graveyard, exile it.
        let card = *state.zones.hands[0].first().expect("a card in hand");
        state.zones.hands[0].retain(|&o| o != card);
        state.objects.obj_mut(card).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(card);
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::act_by_you(PlayerAction::Move(
                Reference::This,
                deckmaste_core::Destination::Zone(Zone::Exile),
                vec![],
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(card).is_none(), "old graveyard id gone");
        assert!(state.zones.graveyards[0].is_empty());
        assert_eq!(state.zones.exile.len(), 2);
    }

    /// [CR#400.7]: `Move(This, Hand)` moves the source to its owner's hand,
    /// reminting it — the old id is gone and a fresh object sits in hand
    /// (the bounce family, subsuming the retired `ReturnToHand` verb). The
    /// graveyard arm proves the move reads each object's current zone (like
    /// `Exile`), not a hard-coded battlefield source.
    #[test]
    fn return_to_hand_from_battlefield_and_graveyard() {
        let (mut state, bear) = bear_on_field();
        let hand_before = state.zones.hands[0].len();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Zone(Zone::Hand),
                vec![],
                None,
            )),
            &frame,
        );
        // future-form ZoneChange → past-form ZoneChange.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.hands[0].len(), hand_before + 1);
        let returned = *state.zones.hands[0].last().expect("a returned card");
        assert_eq!(state.objects.obj(returned).zone, Some(Zone::Hand));

        // From the graveyard ([CR#400.7] reads the current zone): force a hand
        // card into the graveyard, then return it to hand.
        let card = *state.zones.hands[0].first().expect("a card in hand");
        state.zones.hands[0].retain(|&o| o != card);
        state.objects.obj_mut(card).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(card);
        let gy_hand_before = state.zones.hands[0].len();
        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Zone(Zone::Hand),
                vec![],
                None,
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(card).is_none(), "old graveyard id gone");
        assert!(state.zones.graveyards[0].is_empty());
        assert_eq!(state.zones.hands[0].len(), gy_hand_before + 1);
    }

    /// [CR#701.8a,701.9a,701.17a]: `Move`'s `from` fizzle-guard. A card
    /// already in the graveyard is NOT in hand, so `Move(<it>, from: Hand,
    /// to: Graveyard)` fizzles for it — no `ZoneChange` event (either form),
    /// the object keeps its id (never reminted), and stepping the
    /// engine afterward doesn't panic (authoring/state-drift mismatches
    /// never crash it).
    #[test]
    fn move_from_guard_fizzles_on_zone_mismatch() {
        let (mut state, _bear) = bear_on_field();
        // Force a hand card straight into the graveyard (bypassing `Move`) so
        // its CURRENT zone is Graveyard, not the guard's named Hand.
        let card = *state.zones.hands[0].first().expect("a card in hand");
        state.zones.hands[0].retain(|&o| o != card);
        state.objects.obj_mut(card).zone = Some(Zone::Graveyard);
        state.zones.graveyards[0].push(card);
        let gy_before = state.zones.graveyards[0].len();

        let frame = frame_src(card);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Zone(Zone::Graveyard),
                vec![],
                Some(Zone::Hand),
            )),
            &frame,
        );
        // No future-form ZoneChange was scheduled; step a couple of times anyway to
        // prove the guard mismatch doesn't half-apply or panic.
        for _ in 0..2 {
            let _ = state.step();
        }

        assert!(
            state.objects.get(card).is_some(),
            "the object keeps its id — the guard mismatch fizzled the move"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            gy_before,
            "no card left or entered the graveyard"
        );
        let moved = state
            .history
            .scan(Lookback::ThisGame, state.turn.turn_number)
            .any(|e| matches!(e, GameEvent::ZoneChange(ZoneChange { .. })));
        assert!(
            !moved,
            "a from-guard mismatch fizzles: no zone-change event"
        );
    }

    /// A `MoveGroup` whose `SelectAll(Any)` selection sweeps in a zoneless
    /// member — a player proxy (`zone: None`, which `Predicate::Any` matches) —
    /// skips that member instead of panicking on its absent zone (the
    /// engine-never-crashes-on-authoring-mistakes ruling). The zoned members
    /// still move.
    #[test]
    fn move_group_skips_zoneless_members() {
        let (mut state, _bear) = bear_on_field();
        let proxy = state.player(PlayerId(0)).object;
        assert!(
            state.objects.obj(proxy).zone.is_none(),
            "a player proxy is minted zoneless"
        );
        let frame = frame_src(proxy);
        state.run_effect(
            OneShotEffect::Act(Action::MoveGroup {
                group: Selection::SelectAll(Predicate::Any),
                arrangement: deckmaste_core::Arrangement::AnyOrder,
                to: Destination::Zone(Zone::Graveyard),
                riders: vec![],
            }),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            zone_has_named(&state, &state.zones.graveyards[0], "Grizzly Bears"),
            "the zoned battlefield creature still moved to its graveyard"
        );
        assert!(
            state.objects.obj(proxy).zone.is_none(),
            "the zoneless proxy was skipped, never moved"
        );
    }

    /// [CR#701.6a]: countering a spell removes it from the stack and puts it
    /// into its owner's graveyard, reminted ([CR#400.7]) and cause-tagged
    /// "Counter" — the spell never resolves.
    #[test]
    fn counter_spell_goes_to_owners_graveyard() {
        let (mut state, bear) = bear_on_field();
        // Stand a hand card up as a spell on the stack, owned by player 0.
        let spell = state.zones.hands[0][0];
        state.zones.hands[PlayerId(0).index()].retain(|&o| o != spell);
        state.objects.obj_mut(spell).zone = Some(Zone::Stack);
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            copy: false,
        });
        let gy_before = state.zones.graveyards[0].len();

        // The source's effect counters that spell (chosen as Target(0)).
        let frame = frame_src_targets(bear, vec![spell]);
        state.run_effect(OneShotEffect::Act(Action::Counter(Reference::It)), &frame);
        // future-form ZoneChange → past-form ZoneChange.
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.stack.is_empty(), "spell removed from the stack");
        assert!(state.objects.get(spell).is_none(), "old stack id gone");
        assert_eq!(state.zones.graveyards[0].len(), gy_before + 1);
        let countered = *state.zones.graveyards[0].last().expect("a countered spell");
        assert_eq!(state.objects.obj(countered).zone, Some(Zone::Graveyard));
    }

    /// [CR#701.6a]: a spell carrying `Cant(Counter(on: Ref(This)))` ("this
    /// spell can't be countered") is NOT moved off the stack by a counter
    /// instruction — the eval hook on the counter-resolution path refuses the
    /// counter, so the spell stays (to resolve normally); nothing hits the
    /// graveyard. The mirror of `counter_spell_goes_to_owners_graveyard`.
    #[test]
    fn cant_be_countered_spell_survives_counter() {
        use deckmaste_core::Ability;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::StaticEffect;

        let (mut state, bear) = bear_on_field();
        // Mint an instant carrying "this spell can't be countered" and push it
        // onto the stack, owned/controlled by player 0.
        let card = Card::Normal(CardFace {
            name: "Uncounterable".into(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::r#static(StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Counter {
                    by: Predicate::Any,
                    on: Predicate::Ref(Reference::This),
                },
            )))],
            ..CardFace::default()
        });
        let cid = state.cards.push(Arc::new(card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            copy: false,
        });
        let gy_before = state.zones.graveyards[0].len();

        // The source's effect tries to counter that spell (chosen as Target(0)).
        let frame = frame_src_targets(bear, vec![spell]);
        state.run_effect(OneShotEffect::Act(Action::Counter(Reference::It)), &frame);
        // Process the (empty) emit the refused counter scheduled. The refusal
        // emits no future-form ZoneChange, so — unlike the happy path — there is no
        // follow-up item; step exactly once.
        let _ = state.step();

        assert!(
            state.stack.iter().any(|e| e.id == spell),
            "an uncounterable spell stays on the stack"
        );
        assert!(
            state.objects.get(spell).is_some(),
            "the spell object is still live (not reminted into a graveyard)"
        );
        assert_eq!(
            state.zones.graveyards[0].len(),
            gy_before,
            "nothing was countered into the graveyard"
        );
    }

    /// [CR#701.6a]: countering an ability removes it from the stack and it
    /// ceases (removed from stack and object store; no zone move).
    #[test]
    fn counter_ability_ceases() {
        let (mut state, bear) = bear_on_field();
        // Mint a token id for the ability.
        let ability_id = state.objects.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(Zone::Stack),
        );
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: ability_id,
            object: StackObject::Triggered {
                source: ObjectSource::Card(state.objects.obj(bear).card_id().unwrap()),
                ability: 0,
                created: None,
                bindings: TriggerBindings::default(),
            },
            controller: PlayerId(0),
            targets: vec![],
            x: None,
            copy: false,
        });

        // The source's effect counters that ability (chosen as Target(0)).
        let frame = frame_src_targets(bear, vec![ability_id]);
        state.run_effect(OneShotEffect::Act(Action::Counter(Reference::It)), &frame);
        // AbilityResolved applies.
        let _ = state.step();

        assert!(state.stack.is_empty(), "ability removed from the stack");
        assert!(
            state.objects.get(ability_id).is_none(),
            "minted ability id gone"
        );

        let found = state
            .history
            .scan(Lookback::ThisGame, state.turn.turn_number)
            .any(|e| matches!(e, GameEvent::AbilityCountered(AbilityCountered { id, .. }) if *id == ability_id));
        assert!(found, "AbilityCountered event must be recorded in history");
    }

    /// [CR#401.7]: `Move(This, Library(FromTop(0)))` puts the card on top;
    /// `Library(FromBottom(0))` puts it on the bottom (the placement no
    /// from-top index could name without the library size).
    #[test]
    fn move_to_library_top_and_bottom() {
        use deckmaste_core::Anchor;
        use deckmaste_core::Destination;

        let (mut state, bear) = bear_on_field();
        let bear_card = state.objects.obj(bear).card_id().expect("card-backed");
        let lib_before = state.zones.libraries[0].len();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Library(Anchor::FromTop(Count::Literal(0))),
                vec![],
                None,
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old id gone");
        assert_eq!(state.zones.libraries[0].len(), lib_before + 1);
        let top = *state.zones.libraries[0].front().expect("non-empty library");
        assert_eq!(state.objects.obj(top).card_id(), Some(bear_card));
        assert_eq!(state.objects.obj(top).zone, Some(Zone::Library));

        // Bottom of library ([CR#401.7]): FromBottom(0) lands at the back.
        let frame = frame_src(top);
        state.run_effect(
            OneShotEffect::Act(Action::Move(
                Reference::This,
                Destination::Library(Anchor::FromBottom(Count::Literal(0))),
                vec![],
                None,
            )),
            &frame,
        );
        for _ in 0..2 {
            let _ = state.step();
        }
        assert_eq!(state.zones.libraries[0].len(), lib_before + 1);
        let bottom = *state.zones.libraries[0].back().expect("non-empty library");
        assert_eq!(state.objects.obj(bottom).card_id(), Some(bear_card));
    }

    /// [CR#122]: `MoveCounters(AllKinds, from, to)` relocates every counter of
    /// every kind from the source onto the destination (Fate Transfer / Ozolith
    /// shape) — the case single-kind remove+put can't reach atomically.
    #[test]
    fn move_counters_all_kinds_relocates_every_counter() {
        use deckmaste_core::CounterSpec;
        let (mut state, a, b) = two_permanents_on_field();
        let p1p1: deckmaste_core::Ident = "P1P1Counter".into();
        let charge: deckmaste_core::Ident = "ChargeCounter".into();
        state.objects.obj_mut(a).counters.insert(p1p1, 2);
        state.objects.obj_mut(a).counters.insert(charge, 1);
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            OneShotEffect::Act(Action::MoveCounters(
                CounterSpec::AllKinds,
                Reference::Target(0),
                Reference::Target(1),
            )),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            state.objects.obj(a).counters.is_empty(),
            "source emptied of all counters"
        );
        assert_eq!(state.objects.obj(b).counters.get(&p1p1).copied(), Some(2));
        assert_eq!(state.objects.obj(b).counters.get(&charge).copied(), Some(1));
    }

    /// [CR#122]: `MoveCounters(Named(kind, n), from, to)` moves up to `n`
    /// counters of that kind (clamped to what the source holds) — Power Conduit
    /// / Leech Bonder shape.
    #[test]
    fn move_counters_named_moves_up_to_available() {
        use deckmaste_core::CounterRef;
        use deckmaste_core::CounterSpec;
        let (mut state, a, b) = two_permanents_on_field();
        let p1p1: deckmaste_core::Ident = "P1P1Counter".into();
        state.objects.obj_mut(a).counters.insert(p1p1, 3);
        let frame = frame_src_targets(a, vec![a, b]);
        state.run_effect(
            OneShotEffect::Act(Action::MoveCounters(
                CounterSpec::Named(CounterRef::from("P1P1Counter"), Count::Literal(2)),
                Reference::Target(0),
                Reference::Target(1),
            )),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(a).counters.get(&p1p1).copied(),
            Some(1),
            "2 of 3 moved, 1 remains on source"
        );
        assert_eq!(state.objects.obj(b).counters.get(&p1p1).copied(), Some(2));
    }

    /// A "host gets +n/+n" static targeting this attachment's host
    /// (`Of(AttachHostOf(This))`) — the equipped/enchanted-creature bonus.
    fn host_pump(n: u32) -> Ability {
        Ability::r#static(StaticEffect::Modify(
            Reference::AttachHostOf(Arc::new(Reference::This)),
            Modification::Several(vec![
                Modification::Power(NumericOp::Up(Count::Literal(n))),
                Modification::Toughness(NumericOp::Up(Count::Literal(n))),
            ]),
        ))
    }

    /// A vanilla 2/2 creature on the battlefield.
    fn vanilla_creature(state: &mut GameState, name: &str) -> ObjectId {
        mint_on_field(
            state,
            Card::Normal(CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(2)),
                ..CardFace::default()
            }),
        )
    }

    /// [CR#702.6a]: activate the Equipment's equip ability (sorcery speed)
    /// targeting a creature you control → the host's derived P/T includes the
    /// Equipment's "+1/+1" bonus (via the `Of(AttachHostOf(This))` path).
    #[test]
    fn equip_e2e() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Bear Host");
        // A real Equipment: the Equipment subtype confer (Innate May(Attach to:
        // Creature) grant) + the `equip {T}` keyword + "+1/+1 to the equipped
        // creature".
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Test Sword".into(),
                types: vec![Type::Artifact.def()],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])"), host_pump(1)],
                ..CardFace::default()
            }),
        );
        // Base host is 2/2.
        assert_eq!(state.layers().power(host), Some(2));

        // Drive the equip activated ability: the keyword + host_pump → the
        // activated ability is at filtered index 0 (no Innate to skew it here,
        // but resolve via the offered legal action to be faithful).
        let frame = frame_src_targets(equipment, vec![host]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);

        assert_eq!(
            state.objects.obj(equipment).attached_to,
            Some(host),
            "equip attached the Equipment to the host ([CR#701.3a])"
        );
        assert_eq!(
            state.layers().power(host),
            Some(3),
            "the equipped creature gets +1/+1 (host-targeting static landed)"
        );
        assert_eq!(state.layers().toughness(host), Some(3));
    }

    /// [CR#303.4,704.5m]: a CAST Aura resolves attached to the SPELL'S CHOSEN
    /// TARGET (the cast-path host wiring), buffs it +2/+2, and is sent to its
    /// owner's graveyard by the SBA when the host leaves.
    #[test]
    fn aura_cast_e2e() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Enchanted Bear");
        // A real Aura: Enchant(creature) keyword (targeting Spell + May(Attach)
        // grant + AsEnters) + the Aura subtype's Innate graveyard SBA + "+2/+2".
        let aura_card = Card::Normal(CardFace {
            name: "Test Aura".into(),
            types: vec![Type::Enchantment.def()],
            subtypes: vec![subtype("Aura")],
            abilities: vec![keyword("Enchant(Type(\"Creature\"))"), host_pump(2)],
            ..CardFace::default()
        });
        // Stand the Aura up as a spell on the stack, target = the host.
        let cid = state.cards.push(Arc::new(aura_card), PlayerId(0));
        let spell = state
            .objects
            .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Stack));
        state.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: spell,
            object: StackObject::Spell(spell),
            controller: PlayerId(0),
            targets: vec![vec![host]],
            x: None,
            copy: false,
        });
        // Resolve the Aura spell — it enters attached to its chosen target.
        // (`resolve_object` schedules the entering ZoneMove at the agenda front;
        // `run_injected` processes just that, without parking priority.)
        state.resolve_object(spell);
        run_injected(&mut state);

        let aura = *state
            .zones
            .battlefield
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(cid))
            .expect("the Aura entered the battlefield");
        assert_eq!(
            state.objects.obj(aura).attached_to,
            Some(host),
            "cast Aura enters attached to its chosen target ([CR#303.4])"
        );
        assert_eq!(
            state.layers().power(host),
            Some(4),
            "the enchanted creature gets +2/+2"
        );

        // Destroy the host (source = host, `This` = the dying creature); the SBA
        // sweep then sends the now-unattached Aura to the graveyard ([CR#704.5m]).
        let frame = frame_src(host);
        state.run_effect(OneShotEffect::Act(Action::destroy(Reference::This)), &frame);
        run_injected(&mut state);
        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            run_injected(&mut state);
        }
        let aura_gy = *state.zones.graveyards[PlayerId(0).index()]
            .iter()
            .find(|&&o| state.objects.obj(o).card_id() == Some(cid))
            .expect("the orphaned Aura was put into its owner's graveyard ([CR#704.5m])");
        assert_eq!(state.objects.obj(aura_gy).zone, Some(Zone::Graveyard));
    }

    /// [CR#704.5n]: when an equipped creature dies, the Equipment becomes
    /// unattached and STAYS on the battlefield (no graveyard SBA — that's
    /// Auras).
    #[test]
    fn equipment_host_dies_unattaches() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Doomed Bear");
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Sticky Sword".into(),
                types: vec![Type::Artifact.def()],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equipment).attached_to = Some(host);

        // Host dies.
        let frame = frame_src_targets(equipment, vec![host]);
        state.run_effect(OneShotEffect::Act(Action::destroy(Reference::It)), &frame);
        drain(&mut state);
        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            drain(&mut state);
        }
        assert_eq!(
            state.objects.obj(equipment).attached_to,
            None,
            "the Equipment became unattached when its host died ([CR#704.5n])"
        );
        assert!(
            state.zones.battlefield.contains(&equipment),
            "the Equipment STAYS on the battlefield (not graveyarded)"
        );
    }

    /// [CR#702.16d]: a creature that gains protection from a color drops a
    /// colored Equipment attached to it — the SBA re-runs `attachment_legal`
    /// (host-side protection `Cant(Attach)`) and unattaches.
    #[test]
    fn protection_drops_equipment() {
        use deckmaste_core::Color;
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;

        let mut state = game();
        // The host gains protection from red: a host-side `Cant(Attach(what:
        // red, to: This))` (the Protection-conferred shape, [CR#702.16d]).
        let host = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Protected Bear".into(),
                types: vec![Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(2)),
                toughness: Some(deckmaste_core::StatValue::Number(2)),
                abilities: vec![Ability::r#static(StaticEffect::Deontic(Deontic::Cant(
                    DeonticAction::Attach {
                        what: Predicate::Characteristic(CharacteristicPredicate::ColorIs(
                            Color::Red,
                        )),
                        to: Predicate::Ref(Reference::This),
                    },
                )))],
                ..CardFace::default()
            }),
        );
        // A RED Equipment attached to the host.
        let equipment = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Red Sword".into(),
                types: vec![Type::Artifact.def()],
                color_indicator: vec![Color::Red],
                subtypes: vec![subtype("Equipment")],
                abilities: vec![keyword("Equip([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equipment).attached_to = Some(host);
        // Sanity: it is currently illegal (protection) — the SBA will catch it.
        assert!(!crate::legal::attachment_legal(&state, equipment, host));

        for e in crate::sba::sweep(&state) {
            state.schedule_front(vec![WorkItem::Emit(Occurrence::single(e))]);
            drain(&mut state);
        }
        assert_eq!(
            state.objects.obj(equipment).attached_to,
            None,
            "the colored Equipment fell off the protected creature ([CR#702.16d])"
        );
    }

    /// [CR#702.67a]: a Fortification with `fortify` activated, targeting a land
    /// you control → attached to that land.
    #[test]
    fn fortify_attaches_to_land() {
        let mut state = game();
        let land = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Target Land".into(),
                types: vec![Type::Land.def()],
                ..CardFace::default()
            }),
        );
        let fortification = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Test Banner".into(),
                types: vec![Type::Artifact.def()],
                subtypes: vec![subtype("Fortification")],
                abilities: vec![keyword("Fortify([Tap])")],
                ..CardFace::default()
            }),
        );
        let frame = frame_src_targets(fortification, vec![land]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        drain(&mut state);
        assert_eq!(
            state.objects.obj(fortification).attached_to,
            Some(land),
            "fortify attached the Fortification to the land ([CR#702.67a])"
        );
    }

    /// [CR#702.151b]: a reconfigured Equipment attached to a creature stops
    /// being a creature; unattaching restores it. SEAM: the
    /// creature-suppression static needs condition-gated layer-4 type
    /// removal the engine doesn't have yet (see Reconfigure.ron) — so the
    /// suppression assertion is `#[ignore]`d; the attach/unattach mechanics
    /// are exercised here unignored.
    #[test]
    fn reconfigure_attaches_and_unattaches() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Recon Host");
        // A reconfigure Equipment creature (it IS a creature when unattached).
        let equip_creature = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Living Weapon".into(),
                types: vec![Type::Artifact.def(), Type::Creature.def()],
                subtypes: vec![subtype("Equipment")],
                power: Some(deckmaste_core::StatValue::Number(1)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                abilities: vec![keyword("Reconfigure([Tap])")],
                ..CardFace::default()
            }),
        );
        // Attach via reconfigure's first ability shape (Attach to a creature).
        let frame = frame_src_targets(equip_creature, vec![host]);
        state.run_effect(
            OneShotEffect::Act(Action::Attach {
                what: Reference::This,
                to: Reference::It,
            }),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(equip_creature).attached_to,
            Some(host),
            "reconfigure attached the Equipment to the creature ([CR#702.151a])"
        );

        // Unattach (reconfigure's second ability).
        let frame = frame_src(equip_creature);
        state.run_effect(
            OneShotEffect::Act(Action::Unattach(Reference::This)),
            &frame,
        );
        run_injected(&mut state);
        assert_eq!(
            state.objects.obj(equip_creature).attached_to,
            None,
            "reconfigure unattached the Equipment ([CR#702.151a])"
        );
    }

    /// [CR#702.151b]: SEAM — the creature-suppression static (a reconfigured
    /// Equipment isn't a creature while attached) needs condition-gated layer-4
    /// type removal the layer pipeline doesn't have yet (Reconfigure.ron seam).
    /// Ignored until that engine support lands.
    #[test]
    #[ignore = "engine-attach seam: conditional layer-4 type removal not built ([CR#702.151b]) — see Reconfigure.ron"]
    fn reconfigure_suppresses_creature() {
        let mut state = game();
        let host = vanilla_creature(&mut state, "Recon Host");
        let equip_creature = mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Living Weapon".into(),
                types: vec![Type::Artifact.def(), Type::Creature.def()],
                subtypes: vec![subtype("Equipment")],
                power: Some(deckmaste_core::StatValue::Number(1)),
                toughness: Some(deckmaste_core::StatValue::Number(1)),
                abilities: vec![keyword("Reconfigure([Tap])")],
                ..CardFace::default()
            }),
        );
        state.objects.obj_mut(equip_creature).attached_to = Some(host);
        // Would-be: attached → not a creature.
        let view = state.layers();
        assert!(
            !view.get(equip_creature).has_type(Type::Creature),
            "attached reconfigure Equipment is not a creature ([CR#702.151b])"
        );
    }

    /// [CR#702.131c]: the grant verb emits one `GotDesignation` for a player
    /// who lacks the designation, and nothing for one who already holds it
    /// (idempotent — keeps the SBA sweep convergent and avoids spurious facts).
    #[test]
    fn get_designation_emits_once_then_nothing() {
        use crate::state::DesignationValue;

        let mut state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let pa = deckmaste_core::PlayerAction::GetDesignation("CitysBlessing".into());

        let items = state.player_action_items(&pa, p0, &frame);
        assert_eq!(items.len(), 1, "first grant emits exactly one fact");

        // Grant it for real, then re-run: no event.
        state
            .designations
            .players
            .insert((p0, "CitysBlessing".into()), DesignationValue::Flag);
        let items = state.player_action_items(&pa, p0, &frame);
        assert!(items.is_empty(), "already-held designation emits nothing");
    }

    /// [CR#114.1]: the `GetEmblem` verb lowers to exactly one `EmblemCreated`
    /// fact carrying the emblem's abilities for the actor — the resolution wire
    /// the command-zone mint (`apply_emblem_created`) applies.
    #[test]
    fn get_emblem_emits_emblem_created_for_the_actor() {
        use crate::event::GameEvent;
        use crate::event::Occurrence;

        let state = game();
        let p0 = PlayerId(0);
        let frame = frame_for(&state, p0);
        let abilities = vec![deckmaste_core::Ability::r#static(
            deckmaste_core::StaticEffect::Modify(
                deckmaste_core::Reference::It,
                deckmaste_core::Modification::Power(deckmaste_core::NumericOp::Up(
                    deckmaste_core::Count::Literal(1),
                )),
            ),
        )];
        let pa = deckmaste_core::PlayerAction::GetEmblem(abilities.clone());

        let items = state.player_action_items(&pa, p0, &frame);
        assert_eq!(items.len(), 1, "GetEmblem emits exactly one fact");
        match &items[0] {
            crate::agenda::WorkItem::Emit(Occurrence::Single(GameEvent::EmblemCreated(
                EmblemCreated {
                    player,
                    abilities: emitted,
                },
            ))) => {
                assert_eq!(*player, p0, "the emblem goes to the actor ([CR#114.2])");
                assert_eq!(*emitted, abilities, "carries the payload abilities");
            }
            other => panic!("expected one EmblemCreated emit, got {other:?}"),
        }
    }

    // ---- scry / arrange (the recomposed keyword-action path) ----------------

    /// Mint a fresh card-backed object into `owner`'s library at the BOTTOM
    /// (`push_back`; the front is the top). Returns its id.
    fn mint_in_library(state: &mut GameState, owner: PlayerId, name: &str) -> ObjectId {
        let cid = state.cards.push(
            Arc::new(Card::Normal(CardFace {
                name: name.into(),
                types: vec![Type::Creature.def()],
                ..CardFace::default()
            })),
            owner,
        );
        let id = state
            .objects
            .mint(ObjectSource::Card(cid), owner, Some(Zone::Library));
        state.zones.libraries[owner.index()].push_back(id);
        id
    }

    /// The recomposed `scry n` effect ([CR#701.22a]): the committed north-star
    /// shape — `Composite Scry (Each (Existing (TopOfLibrary n)) (Modal 1-of-2
    /// [Move(It, Library(FromTop 0)), Move(It, Library(FromBottom 0))]))`.
    fn scry_effect(n: Uint) -> OneShotEffect {
        let mode = |anchor| deckmaste_core::Mode {
            effect: OneShotEffect::Act(Action::Move(
                Reference::It,
                Destination::Library(anchor),
                vec![],
                None,
            )),
            cost: None,
        };
        OneShotEffect::Act(Action::Composite {
            name: deckmaste_core::VerbName::from("Scry"),
            body: Arc::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(n),
                    whose: Reference::You,
                }),
                effect: Arc::new(OneShotEffect::Modal(deckmaste_core::Modal {
                    choose: deckmaste_core::ChooseSpec {
                        count: deckmaste_core::Quantity::Range(
                            Some(Count::Literal(1)),
                            Some(Count::Literal(1)),
                        ),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::You,
                        rider: None,
                    },
                    modes: vec![
                        mode(Anchor::FromTop(Count::Literal(0))),
                        mode(Anchor::FromBottom(Count::Literal(0))),
                    ],
                })),
            })),
        })
    }

    /// Step until a decision surfaces (or `n` steps elapse), returning the
    /// applied events seen along the way.
    fn drain_events(state: &mut GameState, n: usize) -> Vec<GameEvent> {
        let mut out = Vec::new();
        for p in drain_progress(state, n) {
            if let Progress::Applied(occ) = p {
                match occ {
                    Occurrence::Single(e) => out.push(e),
                    Occurrence::Batch(v) => out.extend(v),
                }
            }
        }
        out
    }

    /// [CR#701.22a,401.7]: scry-1 to the BOTTOM repositions the peeked card
    /// within the SAME library — the `ObjectId` is preserved, no `ZoneChange`
    /// fires (a pile of one surfaces no arrange decision), and the
    /// keyword-action event fires once the pick lands ([CR#701.22d]).
    #[test]
    fn scry_reposition_keeps_id_and_fires_no_zone_change() {
        let p0 = PlayerId(0);
        let mut state = game();
        let a = mint_in_library(&mut state, p0, "A");
        let b = mint_in_library(&mut state, p0, "B");
        let c = mint_in_library(&mut state, p0, "C");
        // library top→bottom = [a, b, c].
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        drain_events(&mut state, 60); // → the single Modal decision
        assert!(
            matches!(
                state.pending,
                Some(PendingDecision::ChooseModes(
                    crate::decide::pending::ChooseModes { .. }
                ))
            ),
            "scry surfaces the per-card top/bottom pick, got {:?}",
            state.pending
        );
        // The looker sees the peeked card.
        assert!(state.look_grants.contains(&(p0, a)), "peek grants look");
        // Pick mode 1 (bottom).
        state.submit_decision(Decision::Modes(vec![1])).unwrap();
        let events = drain_events(&mut state, 60);
        // `a` moved to the bottom, SAME id, no zone change.
        assert_eq!(
            state.zones.libraries[p0.index()]
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![b, c, a],
            "a repositioned to the bottom, keeping its id"
        );
        assert!(
            state.objects.get(a).is_some(),
            "the repositioned object id is preserved (not reminted)"
        );
        assert!(
            !events.iter().any(|e| matches!(
                e,
                GameEvent::ZoneChange(ZoneChange {
                    snapshot: Some(_),
                    ..
                })
            )),
            "a same-library reposition fires no past-form ZoneChange"
        );
        assert!(
            events.iter().any(|e| matches!(
                e,
                GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Scry"
            )),
            "scry-1 fires the keyword-action event"
        );
    }

    /// [CR#701.22b]: scry 0 does nothing and fires NO keyword event; scry N>0
    /// fires exactly one `Act`.
    #[test]
    fn scry_zero_fires_no_event_but_nonzero_does() {
        let p0 = PlayerId(0);

        // scry 0 over a stocked library: no decision, no event.
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(0), &frame);
        let events = drain_events(&mut state, 60);
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { .. }))),
            "scry 0 emits no keyword event ([CR#701.22b])"
        );

        // scry 1 fires exactly one.
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap();
        let events = drain_events(&mut state, 60);
        let scries = events
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Scry"
                )
            })
            .count();
        assert_eq!(scries, 1, "scry 1 fires exactly one keyword event");
    }

    /// [CR#401.4]: a pile of MORE THAN ONE card at a library end surfaces one
    /// arrange decision (both-on-top), while a scry whose picks split one card
    /// to each end surfaces NONE (every pile is a single card).
    #[test]
    fn scry_arrange_surfaces_only_for_multi_card_piles() {
        let p0 = PlayerId(0);

        // Both on top → one arrange decision over the two-card pile.
        let mut state = game();
        let a = mint_in_library(&mut state, p0, "A");
        let b = mint_in_library(&mut state, p0, "B");
        mint_in_library(&mut state, p0, "C");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(2), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // a → top
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // b → top
        drain_events(&mut state, 60);
        let Some(PendingDecision::ArrangePile(crate::decide::pending::ArrangePile {
            player,
            objects,
        })) = state.pending.clone()
        else {
            panic!("expected an ArrangePile decision, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the scrying player arranges");
        assert_eq!(
            objects
                .iter()
                .copied()
                .collect::<std::collections::HashSet<_>>(),
            [a, b].into_iter().collect(),
            "the top pile holds both peeked cards"
        );

        // One top, one bottom → two singleton piles → no arrange decision.
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        mint_in_library(&mut state, p0, "C");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(2), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // a → top
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![1])).unwrap(); // b → bottom
        let events = drain_events(&mut state, 60);
        assert!(
            !matches!(
                state.pending,
                Some(PendingDecision::ArrangePile(
                    crate::decide::pending::ArrangePile { .. }
                ))
            ),
            "two singleton piles surface no arrange decision, got {:?}",
            state.pending
        );
        assert!(
            events.iter().any(|e| matches!(
                e,
                GameEvent::Act(Act { verb, .. }) if verb.as_str() == "Scry"
            )),
            "the keyword event still fires"
        );
    }

    /// [CR#701.22b,614.17]: a `Cant(Act(name: "Scry"))` static on the
    /// battlefield suppresses the scry `Action::Composite`'s `Act` event, so
    /// its body never runs — no per-card decision surfaces, no `Act` fact
    /// fires, and the library is untouched. Absent the static, the same scry
    /// runs normally. Proves the keyword-action event flows through the guard
    /// (cant) pass, modelled on `replace_registry`'s
    /// indestructible/cant-happen patterns.
    #[test]
    fn cant_act_suppresses_composite_body() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::Predicate;
        use deckmaste_core::StaticEffect;

        let p0 = PlayerId(0);

        // ABSENT the cant: scry-1 runs, surfacing the per-card top/bottom pick.
        let mut state = game();
        mint_in_library(&mut state, p0, "A");
        mint_in_library(&mut state, p0, "B");
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        drain_events(&mut state, 60);
        assert!(
            matches!(
                state.pending,
                Some(PendingDecision::ChooseModes(
                    crate::decide::pending::ChooseModes { .. }
                ))
            ),
            "without the cant, scry runs and surfaces its pick, got {:?}",
            state.pending
        );

        // WITH a `Cant(Act(Scry(Any)))` static on the battlefield: the
        // Act event is suppressed, so the body never runs.
        let mut state = game();
        let a = mint_in_library(&mut state, p0, "A");
        let b = mint_in_library(&mut state, p0, "B");
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Scry Warden".into(),
                types: vec![Type::Creature.def()],
                abilities: vec![Ability::r#static(StaticEffect::CantHappen(
                    EventFilter::Act {
                        verb: deckmaste_core::VerbName::from("Scry"),
                        who: Predicate::Any,
                        on: Predicate::Any,
                        cause: None,
                    },
                ))],
                ..CardFace::default()
            }),
        );
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        let events = drain_events(&mut state, 60);
        assert!(
            !matches!(
                state.pending,
                Some(PendingDecision::ChooseModes(
                    crate::decide::pending::ChooseModes { .. }
                ))
            ),
            "the canted Act suppresses the scry body — its per-card pick never surfaces, got {:?}",
            state.pending
        );
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::Act(Act { .. }))),
            "a suppressed composite fires no keyword-action fact"
        );
        assert_eq!(
            state.zones.libraries[p0.index()]
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![a, b],
            "the library is untouched — the scry body never ran"
        );
    }

    /// [CR#701.22a]: a full scry-2 both-on-top round trip — the arrange decision
    /// orders the top pile, and the library ends up in the chosen order above
    /// the untouched rest, every id preserved.
    #[test]
    fn scry_two_both_top_round_trip() {
        let p0 = PlayerId(0);
        let mut state = game();
        let a = mint_in_library(&mut state, p0, "A");
        let b = mint_in_library(&mut state, p0, "B");
        let c = mint_in_library(&mut state, p0, "C");
        let d = mint_in_library(&mut state, p0, "D");
        // top→bottom = [a, b, c, d].
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(2), &frame);
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // a → top
        drain_events(&mut state, 60);
        state.submit_decision(Decision::Modes(vec![0])).unwrap(); // b → top
        drain_events(&mut state, 60);
        // Arrange the top pile as b, then a (top → down).
        state
            .submit_decision(Decision::Arranged(vec![b, a]))
            .unwrap();
        drain_events(&mut state, 60);
        assert_eq!(
            state.zones.libraries[p0.index()]
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![b, a, c, d],
            "the chosen order sits on top, the rest untouched, ids preserved"
        );
    }

    /// [CR#401.4]: Brainstorm's group put-back — `MoveGroup(AnyOrder)` from hand
    /// onto the top of the library moves the whole group (reminted, a real zone
    /// change) and surfaces ONE arrange decision over the landed pile, which
    /// then sits on top of the untouched rest of the library. The same
    /// ordered-landing surface as scry, generalized beyond it.
    #[test]
    fn move_group_any_order_surfaces_arrange_over_landed_pile() {
        let p0 = PlayerId(0);
        let mut state = game();
        let lib = mint_in_library(&mut state, p0, "Lib"); // one card on top
        mint_in_hand(&mut state, p0, "H1");
        mint_in_hand(&mut state, p0, "H2");
        let frame = frame_for(&state, p0);
        let effect = OneShotEffect::Act(Action::MoveGroup {
            group: Selection::SelectAll(Predicate::State(deckmaste_core::StatePredicate::InZone(
                Zone::Hand,
            ))),
            arrangement: deckmaste_core::Arrangement::AnyOrder,
            to: Destination::Library(Anchor::FromTop(Count::Literal(0))),
            riders: vec![],
        });
        state.run_effect(effect, &frame);
        drain_events(&mut state, 60);
        let Some(PendingDecision::ArrangePile(crate::decide::pending::ArrangePile {
            player,
            objects,
        })) = state.pending.clone()
        else {
            panic!("expected an ArrangePile decision, got {:?}", state.pending);
        };
        assert_eq!(player, p0, "the owner arranges an 'any order' group");
        assert_eq!(objects.len(), 2, "both moved cards form the top pile");
        assert!(
            state.zones.hands[p0.index()].is_empty(),
            "the hand cards left the hand"
        );
        assert_eq!(
            state.zones.libraries[p0.index()].len(),
            3,
            "library grew by the two moved cards"
        );
        // Arrange the pile, then it sits on top of the pre-existing card.
        state
            .submit_decision(Decision::Arranged(objects.clone()))
            .unwrap();
        drain_events(&mut state, 60);
        let top: Vec<_> = state.zones.libraries[p0.index()]
            .iter()
            .copied()
            .take(2)
            .collect();
        assert_eq!(top, objects, "the arranged pile sits on top");
        assert_eq!(
            state.zones.libraries[p0.index()].iter().copied().nth(2),
            Some(lib),
            "the pre-existing card is untouched beneath the pile"
        );
    }
}
