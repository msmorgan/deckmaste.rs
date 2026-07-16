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
            // [CR#701]: a named keyword action rides ONE present-tense `Act`
            // event that is BOTH the guardable/replaceable moment and the
            // "whenever you scry/surveil" trigger fact — no separate pre/post
            // pair. It is gated on the body ACTUALLY acting (scry 0 does nothing
            // → no `Act`, no trigger, [CR#701.22b]) and lands AFTER the body so
            // the trigger fires once the keyword action completes, including any
            // post-pick arrangement ([CR#701.22d]): the body's own items are
            // scheduled ahead of the `Act` emit by `schedule_front`. The guard
            // role does NOT need the event logged first — the cant CHECK below
            // runs at schedule time, deciding whether the body runs at all,
            // decoupled from where the surviving fact lands in the log.
            Action::Composite(atom, body) => {
                // Decompose the keyword-action atom into the present-tense `Act`
                // event's resolved coordinates: `who` (the performing player,
                // for the player-report verbs) rides the `actor` slot; `on` (the
                // patient object, for the object verbs) rides `object`. DESIGN
                // NOTE: these are RESOLVED ids, not symbolic `Reference`s — the
                // guard's `FactView` has no frame to resolve against.
                use deckmaste_core::KeywordAction as Ka;
                let (verb, who, on): (&str, Option<_>, Option<ObjectId>) = match atom {
                    Ka::Scry(who, _) => ("Scry", self.eval_player_ref(who, frame), None),
                    Ka::Surveil(who, _) => ("Surveil", self.eval_player_ref(who, frame), None),
                    Ka::Fateseal(who, _) => ("Fateseal", self.eval_player_ref(who, frame), None),
                    Ka::Mill(who, _) => ("Mill", self.eval_player_ref(who, frame), None),
                    Ka::Draw(who, _) => ("Draw", self.eval_player_ref(who, frame), None),
                    // Discard carries BOTH facets: `who` (the discarding
                    // player) always; `on` (the card) only in the BOUND form
                    // ("discard this card", [CR#702.29a]) — lifted off the
                    // body's single-move head below, so the dual-facet move
                    // lane commits it atomically (destroy's shape). The
                    // common chosen form has no patient yet (the choice is a
                    // resolution decision, [CR#701.9b]) — its per-card
                    // `Act(Discard)` events are minted AFTER the choice.
                    Ka::Discard(who, _) => (
                        "Discard",
                        self.eval_player_ref(who, frame),
                        deckmaste_core::discard_body_what(body)
                            .map(|what| self.eval_reference(what, frame)),
                    ),
                    Ka::Destroy(what) => ("Destroy", None, Some(self.eval_reference(what, frame))),
                    // Fight's first fighter rides `object` for existence + the
                    // `Fight(a, _)` narrow; the second is the body's business.
                    Ka::Fight(a, _) => ("Fight", None, Some(self.eval_reference(a, frame))),
                };
                // Dual-facet BODY shape ([CR#603.6]): descend the stored body's
                // HEAD to a canonical single relocation of the patient `on` to a
                // DIFFERENT zone (`Destroy(x)` → `Move(x, Graveyard)`). A
                // move-verb carries this shape and COMMITS it directly on apply
                // (ONE replaceable event); a reorder verb (scry/surveil/fateseal,
                // whose body head is an `Each`) has no such shape → `None`, and
                // its body runs ahead of the post-fact.
                let move_shape: Option<(Zone, Zone)> = on.and_then(|obj| {
                    let to = composite_body_head_move_to(body)?;
                    let from = self.objects.get(obj).and_then(|o| o.zone)?;
                    (from != to).then_some((from, to))
                });
                let cause = match atom {
                    // The draw lane's `Act` carries its own cause so the apply
                    // half commits the Library → Hand move with the real
                    // attribution (source + controller) instead of a
                    // reconstructed sourceless one — the turn-based draw emits
                    // the same event with `Agency::TurnBasedAction`.
                    Ka::Draw(..) => Some(Cause::draw(
                        Agency::EffectInstruction,
                        Some((frame.source, frame.controller)),
                    )),
                    // The committed move's cause names the VERB — "destroyed"
                    // ([CR#701.8b]) vs "discarded" ([CR#701.9a]) — so
                    // cause-narrowed triggers/reads find the right family.
                    Ka::Discard(..) => move_shape.map(|_| {
                        Cause::discard(
                            Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )
                    }),
                    _ => move_shape.map(|_| {
                        Cause::destroy(
                            Agency::EffectInstruction,
                            Some((frame.source, frame.controller)),
                        )
                    }),
                };
                let act = GameEvent::Act {
                    verb: deckmaste_core::VerbName::from(verb),
                    who,
                    on,
                    from: move_shape.map(|(f, _)| f),
                    to: move_shape.map(|(_, t)| t),
                    cause,
                };
                // Dual-facet BATCH shape ([CR#701.17a,603.3b]): a player-verb
                // whose stored body is a GROUP move to a plain zone (mill →
                // top-N to the graveyard). Realized as ONE simultaneous
                // cause-tagged batch, then the aggregate `Act` fact — reorder-
                // style (the trigger fires post-commit, [CR#701.22d]), NOT the
                // atomic-commit branch: the per-card graveyard `ZoneWillChange`s
                // are where a graveyard replacement (Rest-in-Peace) bites, so the
                // `Act` needs no per-object guard facet.
                let group_move = composite_body_group_move(body);
                let mut items = Vec::new();
                if move_shape.is_some() {
                    // [CR#701.8a,616.1]: a move-verb — emit the dual-facet `Act`
                    // whose apply commits the body move atomically. No body
                    // `RunEffect` (that would be a second replaceable event); the
                    // event-side cant pass in `apply_occurrence` suppresses an
                    // indestructible ([CR#702.12b]) patient.
                    items.push(WorkItem::Emit(Occurrence::single(act)));
                } else if let Some((group, to_zone)) = group_move {
                    // [CR#701.22b,614.17]: a "can't mill" static suppresses the
                    // whole keyword action — gate the batch here at schedule time,
                    // like the reorder branch.
                    let suppressed = crate::replace_registry::cant_event(self, &act);
                    if !suppressed {
                        // The result moves carry the atom's cause so "milled this
                        // way" reads find them ([CR#701.17c]); the batch clamps to
                        // library size via `TopOfLibrary`'s `take(n)` ([CR#701.17b]).
                        let cause = match atom {
                            Ka::Mill(..) => Some(Cause::mill(
                                Agency::EffectInstruction,
                                Some((frame.source, frame.controller)),
                            )),
                            _ => None,
                        };
                        // Fizzle, never panic: the macro-built group
                        // (`TopOfLibrary`) only yields library cards, but a
                        // raw-authored `Composite` body can select ZONELESS
                        // objects (player proxies) — skip those instead of
                        // panicking on bad authoring.
                        let events: Vec<GameEvent> = self
                            .eval_selection_set(&group, frame)
                            .into_iter()
                            .filter_map(|object| {
                                let from = self.objects.obj(object).zone?;
                                Some(GameEvent::ZoneWillChange {
                                    object,
                                    from: Some(from),
                                    to: to_zone,
                                    enters: None,
                                    position: None,
                                    face: None,
                                    cause: cause.clone(),
                                })
                            })
                            .collect();
                        // [CR#701.17b,701.22b]: an empty batch (empty library)
                        // performs no keyword action — no batch, no `Act`, no
                        // trigger. A non-empty attempt lands the batch, then the
                        // aggregate `Act` once the cards are in the graveyard.
                        if !events.is_empty() {
                            items.push(WorkItem::Emit(occurrence_of(events)));
                            items.push(WorkItem::Emit(Occurrence::single(act)));
                        }
                    }
                } else if let Ka::Draw(_, n) = atom {
                    // [CR#121.1,121.2]: the per-card draw lane. Draw N cards ONE
                    // AT A TIME — the engine emits N independent single-card
                    // `Act(Draw)` attempts (each its own replace/cant/empty
                    // opportunity, [CR#121.2]), NOT one batch (contrast mill)
                    // and NOT the stored body (which is the render/re-emit facet
                    // only — an executing Each would no-op on an empty library
                    // and MISS the draw-from-empty loss). Each `Act(Draw)`
                    // carries no patient (`on: None`); its APPLY binds the
                    // library top LATE and either commits the Library → Hand
                    // move (cause `Draw`, the success fact) or sets
                    // `drew_from_empty` ([CR#120.3,104.3c]) — the empty check
                    // runs BEFORE the move, so a last-card success is
                    // unambiguous. A "can't draw" static suppresses all N
                    // ([CR#614.17]).
                    if !crate::replace_registry::cant_event(self, &act) {
                        let count = self.eval_count(n, frame);
                        for _ in 0..count {
                            items.push(WorkItem::Emit(Occurrence::single(act.clone())));
                        }
                    }
                } else if let Ka::Discard(_, n) = atom {
                    // [CR#701.9b]: the CHOSEN-from-hand discard lane. The
                    // choice is batched up front (ONE decision picks `count`
                    // cards — or a uniform sample, for the body's `random`
                    // flag), then the submission mints one per-card
                    // `Act(Discard)` each — each its own replace/cant
                    // opportunity (madness exiles ITS card only,
                    // [CR#702.35a]) and its own "whenever a player discards
                    // a card" fact (a multi-discard fires such a trigger
                    // once per card). The stored body (an `Each` over
                    // `FromHand`) is the render/re-emit facet only — an
                    // executing body would need a synchronous read of a
                    // hidden-zone CHOICE. A "can't discard" static
                    // suppresses the whole action ([CR#614.17]); count
                    // clamps to hand size at the decision
                    // (`open_discard_cards`), and a 0-card discard performs
                    // no keyword action — no event, no trigger.
                    // (An unresolvable `who` is an authoring mistake —
                    // fizzle, never crash.)
                    if !crate::replace_registry::cant_event(self, &act)
                        && let Some(player) = who
                    {
                        let count = self.eval_count(n, frame);
                        let item = if deckmaste_core::discard_body_random(body) {
                            WorkItem::DiscardRandom { player, count }
                        } else {
                            WorkItem::DiscardCards { player, count }
                        };
                        items.push(item);
                    }
                } else {
                    // [CR#701.22b,614.17]: a reorder verb — a `Cant(Act(name,
                    // on))` static suppresses the whole keyword action, so the
                    // body must never run. Gate the body on the cant pass here at
                    // schedule time; the surviving `Act` emit still flows through
                    // the shared occurrence/apply cant+replacement path.
                    let suppressed = crate::replace_registry::cant_event(self, &act);
                    if !suppressed {
                        items.push(WorkItem::RunEffect {
                            effect: body.clone(),
                            frame: frame.clone(),
                        });
                        // [CR#701.22b]: a body that does nothing performs no
                        // keyword action, so no `Act` (and no trigger) — scry-0.
                        if self.composite_body_acts(body, frame) {
                            items.push(WorkItem::Emit(Occurrence::single(act)));
                        }
                    }
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

/// The BODY-facet destination of a keyword-action [`Action::Composite`]
/// ([CR#603.6]): the plain zone its stored body's HEAD relocates the patient
/// to. `Destroy`'s body is `Move(x, Graveyard)` → `Some(Graveyard)`; a reorder
/// verb's body head is an `Each`/`If` (scry/surveil/fateseal/fight) → `None`.
/// Read "matches the expanded body" ([CR#701.8a]) rather than a per-verb table,
/// so a later move-verb (Mill/Draw) needs no new arm here. Only the HEAD is
/// inspected — a within-`Each` reorder move (scry) is never mistaken for the
/// composite's own relocation.
fn composite_body_head_move_to(body: &deckmaste_core::OneShotEffect) -> Option<Zone> {
    use deckmaste_core::Action as A;
    use deckmaste_core::OneShotEffect as E;
    match body {
        E::Expanded(e) => composite_body_head_move_to(&e.value),
        E::Act(A::Move(_, Destination::Zone(z), _)) => Some(*z),
        _ => None,
    }
}

/// The BODY-facet GROUP relocation of a keyword-action [`Action::Composite`]
/// ([CR#701.17a,603.3b]): the [`Selection`](deckmaste_core::Selection) its
/// stored body's HEAD moves as ONE simultaneous batch, plus the plain zone they
/// land in. `Mill(who, n)`'s body is `MoveGroup { group: TopOfLibrary(n, who),
/// to: Graveyard }` → `Some((group, Graveyard))`; a single-move (Destroy) or a
/// reorder (scry, an `Each`) body → `None`. Read off the stored body ("matches
/// the expanded body", [CR#701.8a]) rather than a per-verb table.
fn composite_body_group_move(
    body: &deckmaste_core::OneShotEffect,
) -> Option<(deckmaste_core::Selection, Zone)> {
    use deckmaste_core::Action as A;
    use deckmaste_core::OneShotEffect as E;
    match body {
        E::Expanded(e) => composite_body_group_move(&e.value),
        E::Act(A::MoveGroup {
            group,
            to: Destination::Zone(z),
            ..
        }) => Some((group.clone(), *z)),
        _ => None,
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
    use crate::event::GameEvent;
    use crate::event::Occurrence;
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
            abilities: vec![Ability::Innate(Box::new(Ability::Static(
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
                |e| matches!(e, GameEvent::Attached { attachment, host }
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
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
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
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
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
            !logged(&state, |e| matches!(e, GameEvent::Attached { .. })),
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
                |e| matches!(e, GameEvent::Unattached { attachment, former_host }
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
            !logged(&state, |e| matches!(e, GameEvent::Unattached { .. })),
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
                effect: Box::new(OneShotEffect::Act(Action::destroy(Reference::It))),
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
                body: Box::new(OneShotEffect::Act(Action::destroy(Reference::That(
                    deckmaste_core::Sort::Permanent,
                )))),
            }),
            &frame,
        );

        let StepOutcome::NeedsDecision(PendingDecision::ChooseObjects {
            player,
            candidates,
            min,
            max,
        }) = state.step()
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
    /// replaces it) → `ZoneWillChange(Battlefield → Graveyard)` →
    /// `ZoneChanged`, reminting it into its owner's graveyard.
    #[test]
    fn destroy_action_sends_a_normal_creature_to_its_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(OneShotEffect::Act(Action::destroy(Reference::This)), &frame);
        // Act(Destroy) → ZoneWillChange → ZoneChanged.
        for _ in 0..3 {
            let _ = state.step();
        }
        assert!(state.objects.get(bear).is_none(), "old battlefield id gone");
        assert!(!state.zones.battlefield.contains(&bear));
        assert_eq!(state.zones.graveyards[0].len(), 1);
    }

    /// [CR#400.7]: `Move(This, Graveyard)` is a PLAIN relocation — no
    /// `Act(Destroy)` event, so it's a direct `ZoneWillChange(Battlefield →
    /// Graveyard)` → `ZoneChanged`, reminting the object into its OWNER's
    /// graveyard. (Indestructible would not save it — but a plain Grizzly Bears
    /// exercises the move path.)
    #[test]
    fn move_sends_this_to_owner_graveyard() {
        let (mut state, bear) = bear_on_field();
        let frame = frame_src(bear);
        state.run_effect(
            OneShotEffect::Act(Action::move_to(Reference::This, Zone::Graveyard)),
            &frame,
        );
        // ZoneWillChange → ZoneChanged (one fewer step than Destroy — no
        // Act(Destroy) replace stage).
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

    #[test]
    fn action_items_for_tap_draw_loselife() {
        let (state, src) = bear_on_field();
        let frame = frame_src(src);

        // By(You, Tap(This)) -> one Single(Tapped(src)) carrying the
        // effect-instruction cause triple (events.md §3).
        let items = state.action_items(&Action::by_you(PlayerAction::Tap(Reference::This)), &frame);
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::Tapped {
                object: src,
                cause: Some(crate::event::Cause {
                    verb: "Tap".into(),
                    agency: deckmaste_core::Agency::EffectInstruction,
                    agent: Some((src, PlayerId(0))),
                }),
            }))]
        );

        // draw(You, 2) -> two sequential single-card Act(Draw) for the
        // controller ([CR#121.2] per-card, `on: None` — the drawn card binds
        // at apply).
        let items = state.action_items(&Action::draw(Reference::You, Count::Literal(2)), &frame);
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::Act {
                who: Some(PlayerId(0)),
                on: None,
                ..
            }))
        )));

        // By(You, LoseLife(3)) -> one Single(LifeLost{player0, 3})
        let items = state.action_items(
            &Action::by_you(PlayerAction::LoseLife(Count::Literal(3))),
            &frame,
        );
        assert_eq!(
            items,
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeLost {
                player: PlayerId(0),
                amount: 3,
            }))]
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

    /// An explicit agent: `draw(It, 2)` draws for the announced/bound player
    /// (`It`), not the controller. Targets player 1's proxy.
    #[test]
    fn action_items_explicit_agent_draws_for_target() {
        let (state, src) = bear_on_field();
        let p1_proxy = state.players[1].object;
        let frame = frame_src_targets(src, vec![p1_proxy]);
        let items = state.action_items(&Action::draw(Reference::It, Count::Literal(2)), &frame);
        assert_eq!(items.len(), 2);
        assert!(items.iter().all(|item| matches!(
            item,
            WorkItem::Emit(Occurrence::Single(GameEvent::Act {
                who: Some(PlayerId(1)),
                on: None,
                ..
            }))
        )));
    }

    /// The Blood-Money shape ([CR#607.2a] fact-backed product groups): a
    /// `Noting`-wrapped destroy-all over three creatures, one of which
    /// can't be destroyed — "destroyed this way" is exactly the clause's
    /// enacted destroy-caused `ZoneChanged` facts, so the survivor is
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
            effect: Box::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::SelectAll(Predicate::And(
                    vec![
                        Predicate::State(deckmaste_core::StatePredicate::InZone(Zone::Battlefield)),
                        Predicate::creature(),
                    ],
                ))),
                effect: Box::new(OneShotEffect::Act(Action::destroy(Reference::It))),
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
            .filter(|e| matches!(e.fact, GameEvent::ZoneChanged { .. }))
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
                effect: Box::new(OneShotEffect::Act(Action::mill(
                    Reference::You,
                    Count::Literal(3),
                ))),
            }),
            OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::AmongNoted(
                    "milled".into(),
                    deckmaste_core::Quantity::Range(None, None),
                )),
                effect: Box::new(OneShotEffect::Act(Action::Move(
                    Reference::It,
                    deckmaste_core::Destination::Zone(Zone::Exile),
                    vec![],
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
                GameEvent::ZoneChanged { cause: Some(c), to: Zone::Graveyard, .. }
                    if c.verb.as_str() == "Mill"
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

    /// [CR#701.17b,603.3b]: `Action::mill(You, n)` moves the top `n` of the
    /// library to the graveyard as ONE simultaneous batch, clamped to library
    /// size — milling 100 from a bounded library mills the whole library (never
    /// an out-of-range panic), and the moves share one batch id ([CR#603.3b]).
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
            OneShotEffect::Act(Action::mill(Reference::You, Count::Literal(100))),
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
                    GameEvent::ZoneChanged {
                        to: Zone::Graveyard,
                        ..
                    }
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
            OneShotEffect::Act(Action::mill(Reference::You, Count::Literal(2))),
            &frame,
        );
        run_injected(&mut state);

        let facts: Vec<GameEvent> = state.history.entries().map(|e| e.fact.clone()).collect();
        let act_pos = facts
            .iter()
            .position(|f| matches!(f, GameEvent::Act { verb, .. } if verb.as_str() == "Mill"))
            .expect("the mill emits its aggregate Act(Mill) fact");
        let last_gy = facts
            .iter()
            .rposition(|f| {
                matches!(
                    f,
                    GameEvent::ZoneChanged {
                        to: Zone::Graveyard,
                        ..
                    }
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
            OneShotEffect::Act(Action::mill(Reference::You, Count::Literal(2))),
            &frame,
        );
        run_injected(&mut state);
        assert!(
            !state
                .history
                .entries()
                .skip(before)
                .any(|e| matches!(&e.fact, GameEvent::Act { verb, .. } if verb.as_str() == "Mill")),
            "an empty-library mill performs no keyword action ([CR#701.17b,701.22b])"
        );
    }

    /// [CR#616.1]: a Rest-in-Peace-style `→Graveyard` replacement bites the mill
    /// batch's per-card `ZoneWillChange`s directly — the milled cards are
    /// exiled instead of hitting the graveyard (Mill needs no dual-facet
    /// guard; the batch's own zone changes are the replaceable moment).
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
            abilities: vec![Ability::Static(StaticEffect::Replacement(Box::new(rip)))],
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
            OneShotEffect::Act(Action::mill(Reference::You, Count::Literal(2))),
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
        let _ = state.step(); // DiscardOpened
        let StepOutcome::NeedsDecision(PendingDecision::DiscardCards { player, count }) =
            state.step()
        else {
            panic!("expected the batched card choice, got {:?}", state.pending);
        };
        assert_eq!((player, count), (PlayerId(0), 2), "one choice of 2 cards");
        let picks = state.zones.hands[0][..2].to_vec();
        state
            .submit_decision(Decision::Discard(picks.clone()))
            .unwrap();
        run_injected(&mut state);

        let acts = state
            .history
            .entries()
            .filter(|e| {
                matches!(&e.fact, crate::event::GameEvent::Act { verb, who, on, .. }
                    if verb.as_str() == "Discard"
                        && *who == Some(PlayerId(0))
                        && on.is_some_and(|o| picks.contains(&o)))
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
                matches!(&e.fact, crate::event::GameEvent::ZoneChanged { cause: Some(c), .. }
                    if c.verb.as_str() == "Discard")
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
        use deckmaste_core::KeywordActionPattern;

        use crate::Decision;
        use crate::PendingDecision;
        use crate::step::StepOutcome;

        let (mut state, a) = bear_on_field();
        // "If a player would discard [Madness Card], exile it instead."
        let madness = deckmaste_core::Replacement::Instead {
            would: EventFilter::Act(KeywordActionPattern::Discard(
                Predicate::Any,
                Predicate::Characteristic(CharacteristicPredicate::Named("Madness Card".into())),
            )),
            instead: OneShotEffect::Act(Action::move_to(Reference::EventObject, Zone::Exile)),
        };
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "Madness Watcher".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![Ability::Static(StaticEffect::Replacement(Box::new(
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
        let _ = state.step();
        let StepOutcome::NeedsDecision(PendingDecision::DiscardCards { .. }) = state.step() else {
            panic!("expected the batched card choice, got {:?}", state.pending);
        };
        state
            .submit_decision(Decision::Discard(vec![mad, plain]))
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
            OneShotEffect::Act(Action::discard_what(Reference::You, Reference::This)),
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
                matches!(&e.fact, crate::event::GameEvent::Act { verb, on, .. }
                    if verb.as_str() == "Discard" && *on == Some(card))
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
        use deckmaste_core::KeywordActionPattern;

        let (mut state, a) = bear_on_field();
        mint_on_field(
            &mut state,
            Card::Normal(CardFace {
                name: "No Discards".into(),
                types: vec![Type::Enchantment.def()],
                abilities: vec![Ability::Static(StaticEffect::CantHappen(EventFilter::Act(
                    KeywordActionPattern::Discard(Predicate::Any, Predicate::Any),
                )))],
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
                matches!(&e.fact, crate::event::GameEvent::Act { verb, .. }
                    if verb.as_str() == "Discard")
            }),
            "a canted discard performs no keyword action — no fact, no trigger"
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
                GameEvent::DamageDealt { source, target, amount, .. }
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
            vec![WorkItem::Emit(Occurrence::Single(GameEvent::LifeGained {
                player: PlayerId(0),
                amount: 3,
            }))]
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
                GameEvent::CounterPlaced {
                    object: bear,
                    kind: "P1P1Counter".into(),
                    amount: 2,
                    before: 0,
                    after: 0,
                    cause: Some(Cause::put_counters(
                        Agency::EffectInstruction,
                        Some((bear, PlayerId(0))),
                    )),
                }
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
                &Count::CounterCount(Box::new(Reference::You), "Energy".into()),
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
        // Sacrificed → ZoneWillChange → ZoneChanged.
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
        // ZoneWillChange → ZoneChanged.
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
            )),
            &frame,
        );
        // ZoneWillChange → ZoneChanged.
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
        // ZoneWillChange → ZoneChanged.
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
            abilities: vec![Ability::Static(StaticEffect::Deontic(Deontic::Cant(
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
        // emits no ZoneWillChange, so — unlike the happy path — there is no
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
            .any(|e| matches!(e, GameEvent::AbilityCountered { id, .. } if *id == ability_id));
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
        Ability::Static(StaticEffect::Modify(
            Reference::AttachHostOf(Box::new(Reference::This)),
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
                abilities: vec![Ability::Static(StaticEffect::Deontic(Deontic::Cant(
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
        let abilities = vec![deckmaste_core::Ability::Static(
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
            crate::agenda::WorkItem::Emit(Occurrence::Single(GameEvent::EmblemCreated {
                player,
                abilities: emitted,
            })) => {
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
            )),
            cost: None,
        };
        OneShotEffect::Act(Action::Composite(
            deckmaste_core::KeywordAction::Scry(Reference::You, Count::Literal(n)),
            Box::new(OneShotEffect::Each(deckmaste_core::Each {
                binder: deckmaste_core::Binder::Existing(Selection::TopOfLibrary {
                    count: Count::Literal(n),
                    whose: Reference::You,
                }),
                effect: Box::new(OneShotEffect::Modal(deckmaste_core::Modal {
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
        ))
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
    /// within the SAME library — the `ObjectId` is preserved, no `ZoneChanged`
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
            matches!(state.pending, Some(PendingDecision::ChooseModes { .. })),
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
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::ZoneChanged { .. })),
            "a same-library reposition fires no ZoneChanged"
        );
        assert!(
            events.iter().any(|e| matches!(
                e,
                GameEvent::Act { verb, .. } if verb.as_str() == "Scry"
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
            !events.iter().any(|e| matches!(e, GameEvent::Act { .. })),
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
                    GameEvent::Act { verb, .. } if verb.as_str() == "Scry"
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
        let Some(PendingDecision::ArrangePile { player, objects }) = state.pending.clone() else {
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
            !matches!(state.pending, Some(PendingDecision::ArrangePile { .. })),
            "two singleton piles surface no arrange decision, got {:?}",
            state.pending
        );
        assert!(
            events.iter().any(|e| matches!(
                e,
                GameEvent::Act { verb, .. } if verb.as_str() == "Scry"
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
            matches!(state.pending, Some(PendingDecision::ChooseModes { .. })),
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
                abilities: vec![Ability::Static(StaticEffect::CantHappen(EventFilter::Act(
                    deckmaste_core::KeywordActionPattern::Scry(Predicate::Any),
                )))],
                ..CardFace::default()
            }),
        );
        let frame = frame_for(&state, p0);
        state.run_effect(scry_effect(1), &frame);
        let events = drain_events(&mut state, 60);
        assert!(
            !matches!(state.pending, Some(PendingDecision::ChooseModes { .. })),
            "the canted Act suppresses the scry body — its per-card pick never surfaces, got {:?}",
            state.pending
        );
        assert!(
            !events.iter().any(|e| matches!(e, GameEvent::Act { .. })),
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
        let Some(PendingDecision::ArrangePile { player, objects }) = state.pending.clone() else {
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
