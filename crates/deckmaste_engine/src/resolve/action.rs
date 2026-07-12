//! `action_items`: lower an object-verb [`Action`] to events and work items.

use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::Anchor;
use deckmaste_core::Destination;
use deckmaste_core::Reference;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use super::occurrence_of;
use crate::agenda::WorkItem;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
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
            // [CR#701]: a named keyword action — run `body`, then emit the
            // keyword-action fact a "whenever you scry/surveil" trigger reads,
            // gated on the body ACTUALLY acting (scry 0 does nothing and emits
            // no event, [CR#701.22b]). The body's own items are scheduled ahead
            // of the fact by `schedule_front`, so the fact lands after the whole
            // keyword action completes (including any post-pick arrangement).
            Action::Composite { name, body } => {
                let mut items = vec![WorkItem::RunEffect {
                    effect: body.clone(),
                    frame: frame.clone(),
                }];
                if self.composite_body_acts(body, frame) {
                    items.push(WorkItem::Emit(Occurrence::single(
                        GameEvent::KeywordActionPerformed {
                            player: frame.controller,
                            name: *name,
                        },
                    )));
                }
                items
            }
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
                    .map(|target| GameEvent::DamageDealt {
                        source: dealer,
                        target,
                        amount,
                        // OneShotEffect damage is never combat damage ([CR#510.1]).
                        combat: false,
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#701.8a]: destroy = battlefield → graveyard, through the
            // replaceable `WillDestroy` intent so indestructible /
            // regeneration can intercede ([CR#702.12b]); its apply commits
            // the zone move when nothing replaces it. The cause names the
            // verb so the "destroyed" named view can narrow by it ([CR#701.8b]
            // — this verb or the lethal-damage SBA are its only two causes).
            Action::Destroy(sel) => {
                let events: Vec<GameEvent> = self
                    .eval_reference_set(sel, frame)
                    .into_iter()
                    .map(|object| GameEvent::WillDestroy {
                        object,
                        cause: Some(Cause::destroy(
                            Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )),
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
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
                            events.push(GameEvent::ZoneWillChange {
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
                            });
                        }
                        // [CR#707.10a]: a copy of a spell, or a triggered/
                        // activated ability (copy or not), ceases the same
                        // way — no card, no zone move.
                        Some(_) => {
                            events.push(GameEvent::AbilityCountered {
                                id: object,
                                cause: Cause::counter(
                                    Agency::EffectInstruction,
                                    Some((frame.source, frame.controller)),
                                ),
                            });
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
                        events.push(GameEvent::Attached { attachment, host });
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
                            GameEvent::Unattached {
                                attachment,
                                former_host,
                            }
                        })
                    })
                    .collect();
                vec![WorkItem::Emit(occurrence_of(events))]
            }
            // [CR#400.7]: a PLAIN zone move (no `WillDestroy` intent, no
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
            Action::Move(sel, destination, riders) => {
                // Enter riders ([CR#614.12]) await the enters-the-battlefield
                // machinery — a loud seam, like the other unbuilt verbs.
                if !riders.is_empty() {
                    todo!(
                        "core-action-riders-cost-modes seam: enter riders (tapped/attacking/\
                         with-counters) execute with the ETB machinery"
                    );
                }
                self.move_items(sel, destination, frame)
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
                    events.push(GameEvent::CounterRemoved {
                        object: from_id,
                        kind,
                        amount: n,
                        cause: remove_cause.clone(),
                    });
                    events.push(GameEvent::CounterPlaced {
                        object: to_id,
                        kind,
                        amount: n,
                        // Apply-computed totals ([CR#714.2b]).
                        before: 0,
                        after: 0,
                        cause: place_cause.clone(),
                    });
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
                let objects = self.eval_selection_set(group, frame);
                if objects.is_empty() {
                    return vec![];
                }
                let (to_zone, anchor) = match to {
                    Destination::Zone(z) => (*z, None),
                    Destination::Library(a) => (Zone::Library, Some(a)),
                };
                let events: Vec<GameEvent> = objects
                    .iter()
                    .map(|&object| GameEvent::ZoneWillChange {
                        object,
                        from: Some(self.objects.obj(object).zone.expect("move a zoned object")),
                        to: to_zone,
                        enters: None,
                        position: anchor.map(|a| self.library_index(object, a, frame)),
                        face: None,
                        cause: None,
                    })
                    .collect();
                let mut items = vec![WorkItem::Emit(occurrence_of(events))];
                if let Some(anchor) = anchor {
                    let (end, _) = self.anchor_end_offset(anchor, frame);
                    let library_owner = self.owner_of(objects[0]);
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
                    GameEvent::ControlChanged { object, to: player },
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
    /// [`PlayerAction::Move`] (both cause-free). A card moving to a
    /// [`Destination::Library`] anchor that it ALREADY occupies is a same-zone
    /// REPOSITION ([CR#401.7]) — a `RepositionLibrary` work item that keeps the
    /// `ObjectId` and fires no zone change (scry never removes a card from the
    /// library, [CR#701.22a]). Every other move is a genuine `ZoneWillChange`
    /// (remint); the zone-change intents batch as one occurrence.
    pub(super) fn move_items(
        &self,
        sel: &Reference,
        destination: &Destination,
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
            let from = self.objects.obj(object).zone.expect("move a zoned object");
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
                    zone_events.push(GameEvent::ZoneWillChange {
                        object,
                        from: Some(from),
                        to,
                        enters: None,
                        position,
                        face: None,
                        cause: None,
                    });
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
}
