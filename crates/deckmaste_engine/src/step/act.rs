//! The owned `Act` keyword-action apply — the verb dispatch carved out of
//! [`GameState::apply`]'s router. Unlike the `&self` [`EventApply`] handlers,
//! `Act`'s body consumes owned fields (it moves `contents: Box<ActContents>`),
//! so it takes the payload BY VALUE: [`GameState::apply_act`] and its per-verb
//! helpers. Keeping owned semantics avoids cloning the `Box` on every
//! keyword-action apply.
//!
//! [`EventApply`]: crate::step::EventApply

use deckmaste_core::Selection;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::event::Act;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;

/// The future window, stripped of resolution plumbing — reused as the record
/// the paired `FinalizeAct` commits, and as this apply's inert (skipped) return
/// value. `contained` passes through UNCHANGED — `finalize_act` reads it off
/// `act` to decide whether this window's eventual commit is trigger-visible
/// ([CR#616.1g]).
fn rebuilt_act(a: &Act) -> GameEvent {
    GameEvent::Act(Act {
        verb: a.verb,
        who: a.who,
        on: a.on,
        from: a.from,
        to: a.to,
        cause: a.cause.clone(),
        committed: false,
        contents: None,
        batch: a.batch,
        inherited: std::collections::HashSet::new(),
        contained: a.contained,
    })
}

impl GameState {
    /// Apply a keyword-action ([CR#701,616.1]) event — the verb dispatch.
    /// Returns the event that actually occurred: a passed window returns its
    /// own (resolution-stripped) `Act` record, a draw over an empty library
    /// returns [`GameEvent::DrewFromEmpty`] instead. Takes the payload BY VALUE
    /// (its body moves the owned `contents`); the router hands it the extracted
    /// `Act`.
    pub(crate) fn apply_act(&mut self, a: Act) -> GameEvent {
        // The committed PAST keyword-action fact — a pure record (its
        // characteristic change already committed); apply mutates nothing.
        if a.committed {
            return GameEvent::Act(a);
        }
        // [CR#701,616.1]: a FUTURE keyword-action window that PASSED
        // (unreplaced, uncanted) — the apply unwraps its contents DIRECTLY,
        // opening NO second replacement window, per the verb's commit
        // discipline. `FinalizeAct` then records the past name-fact iff the
        // characteristic change committed. A REPLACED or CANTED window never
        // reaches here. The committed PAST fact (`committed: true`, emitted by
        // `FinalizeAct`) applies as a pure record (guard above).
        if let Some(n) = a.batch {
            return self.apply_act_batch(a, n);
        }
        if a.verb.0.as_str() == "Draw"
            && a.on.is_none()
            && let Some(player) = a.who
        {
            return self.apply_act_draw(&a, player);
        }
        if a.contents.is_some() {
            if let Some((group, to_zone)) = a
                .contents
                .as_ref()
                .and_then(|c| crate::resolve::composite_body_group(&c.body))
            {
                return self.apply_act_mill(a, &group, to_zone);
            }
            if a.verb.0.as_str() == "Discard" {
                return self.apply_act_discard(a);
            }
            return self.apply_act_reorder_or_fight(a);
        }
        if let (Some(object), Some(to)) = (a.on, a.to)
            && self.objects.get(object).and_then(|o| o.zone) == a.from
        {
            return self.apply_act_bound_move(&a, object, to);
        }
        // A guard mismatch or degenerate shape at apply — commit nothing; the
        // planted `FinalizeAct` finds no change and the fact dies.
        rebuilt_act(&a)
    }

    /// [CR#616.1g,121.2a]: a PASSED aggregate `Batch` window — its own
    /// [CR#616.1] window already ran (this method is only reached once
    /// cant/replace let the window through), so schedule the `n` contained
    /// per-entity futures NOW, outer-first ([CR#616.1g]: the aggregate was
    /// chosen/rewritten before any of them existed). Each goes through the
    /// ORDINARY, unmodified per-verb `Act` lane
    /// (`composite_items`/`act_window`) via `Repeat`'s existing lazy
    /// self-rescheduling mechanism — bounded memory even for a huge `n`
    /// (the same never-crash discipline `Repeat`/`Batch` already honor).
    /// [CR#614.5]: `inherited` — the lineage this aggregate window itself was
    /// seeded/accumulated with — rides forward into each contained future's own
    /// frame, so whatever replacement produced THIS aggregate (if any) can't
    /// re-catch the very products its own application created (the Archive Trap
    /// shape). `contained_in_batch` marks each of the n minted futures so
    /// `finalize_act` suppresses its own ACT-level commit — this aggregate's
    /// `FinalizeAct{AnyContained}` is the one trigger-visible fact for the
    /// whole batch.
    fn apply_act_batch(&mut self, a: Act, n: Uint) -> GameEvent {
        let rebuilt = rebuilt_act(&a);
        let verb = a.verb;
        let contents = a.contents.expect(
            "an aggregate Batch window always carries its per-unit \
             body in contents",
        );
        if verb.0.as_str() == "Mill" {
            // [CR#701.17a,603.3b,616.1]: the aggregate Mill window PASSED —
            // commit its top-`n` library slice as ONE simultaneous batch of
            // per-card Library → Graveyard FUTURES. `Occurrence::Batch` replaces
            // each member on its OWN ([CR#616.1]: Rest in Peace redirects each
            // milled card to exile), and the destination rides the EVENT (an
            // Instead rewrites it), NOT a body re-read. `n` is the aggregate
            // count a count-multiplier already bit at the ONE window
            // ([CR#121.2a]); the slice clamps to library size ([CR#701.17b]).
            // The one trigger-visible `Act(Mill)` fact lands AFTER the cards
            // move, via `FinalizeAct{AnyContained}` ([CR#701.22d]).
            let player = a.who.expect("a Mill aggregate names its performer");
            let to_zone = crate::resolve::composite_body_group(&contents.body)
                .map_or(Zone::Graveyard, |(_, z)| z);
            let slice: Vec<ObjectId> = self.zones.libraries[player.index()]
                .iter()
                .take(n as usize)
                .copied()
                .collect();
            let mill_events: Vec<GameEvent> = slice
                .into_iter()
                .map(|object| {
                    GameEvent::ZoneChange(ZoneChange {
                        snapshot: None,
                        object,
                        from: Some(Zone::Library),
                        to: to_zone,
                        enters: None,
                        position: None,
                        face: None,
                        cause: a.cause.clone(),
                    })
                })
                .collect();
            let mark = self.resolution_events.len();
            self.schedule_front(vec![
                WorkItem::Emit(Occurrence::Batch(mill_events)),
                WorkItem::FinalizeAct {
                    act: rebuilt.clone(),
                    watch: crate::agenda::FinalizeWatch::AnyContained(verb),
                    mark,
                },
            ]);
            rebuilt
        } else {
            let mut repeat_frame = contents.frame;
            repeat_frame.anaphora.inherited_replacements = a.inherited;
            repeat_frame.anaphora.contained_in_batch = true;
            // `mark` is captured NOW, at apply — after cant/replace already
            // decided the aggregate PASSES — so it only covers what THESE `n`
            // contained futures do ([CR#614.1]: a REPLACED aggregate never
            // reaches this arm at all, so it never plants a `FinalizeAct` to
            // spuriously fire off an unrelated later same-verb resolution).
            let mark = self.resolution_events.len();
            self.schedule_front(vec![
                WorkItem::RunEffect {
                    effect: Box::new(deckmaste_core::OneShotEffect::Repeat(
                        deckmaste_core::Count::Literal(n),
                        Box::new(contents.body),
                    )),
                    frame: repeat_frame,
                },
                WorkItem::FinalizeAct {
                    act: rebuilt.clone(),
                    watch: crate::agenda::FinalizeWatch::AnyContained(verb),
                    mark,
                },
            ]);
            rebuilt
        }
    }

    /// [CR#121.1,121.2]: the atomic single-card draw — the drawn card is the
    /// library top, bound LATE. A card present → commit Library → Hand (cause
    /// `Draw`, the `FactKind::Drawn` success fact) and schedule THIS draw's own
    /// `FinalizeAct` (a per-card `mark`, so it observes only its own move);
    /// empty → `DrewFromEmpty`, no draw fact ([CR#121.4,704.5b]). The empty
    /// check runs BEFORE the move. `that_much = 1` — one card per draw.
    fn apply_act_draw(&mut self, a: &Act, player: PlayerId) -> GameEvent {
        if let Some(&top) = self.zones.libraries[player.index()].front() {
            let rebuilt = rebuilt_act(a);
            self.that_much = Some(1);
            let mark = self.resolution_events.len();
            // The emitting lane owns the attribution ([CR#703.4d]): the effect
            // lane tags `EffectInstruction` + its source; don't reconstruct a
            // sourceless cause here.
            let draw_cause = a.cause.clone().or_else(|| {
                Some(crate::event::Cause::draw(
                    deckmaste_core::Agency::EffectInstruction,
                    None,
                ))
            });
            self.schedule_front(vec![
                WorkItem::Emit(Occurrence::single(GameEvent::ZoneChange(ZoneChange {
                    snapshot: None,
                    object: top,
                    from: Some(Zone::Library),
                    to: Zone::Hand,
                    enters: None,
                    position: None,
                    face: None,
                    cause: draw_cause,
                }))),
                WorkItem::FinalizeAct {
                    act: rebuilt.clone(),
                    watch: crate::agenda::FinalizeWatch::Performer(player),
                    mark,
                },
            ]);
            rebuilt
        } else {
            self.player_mut(player).drew_from_empty = true;
            GameEvent::DrewFromEmpty(player)
        }
    }

    /// [CR#701.17a,603.3b]: mill — derive the top-slice group and commit it as
    /// ONE simultaneous batch (the evolving collector batches their
    /// `ZoneChange` facts), directly, no second window. Re-vet each card is
    /// still in the library (the schedule-time list may have gone stale).
    /// `FinalizeAct` was planted at the window.
    fn apply_act_mill(&mut self, a: Act, group: &Selection, to_zone: Zone) -> GameEvent {
        let rebuilt = rebuilt_act(&a);
        let contents = a
            .contents
            .expect("a mill window carries its top-slice body in contents");
        let patients = self.eval_selection_set(group, &contents.frame);
        debug_assert!(
            self.evolving_batch.is_none(),
            "a mill commits as its own Single occurrence"
        );
        self.evolving_batch = Some(Vec::new());
        for object in patients {
            if self.objects.get(object).and_then(|o| o.zone) == Some(Zone::Library) {
                self.apply_zone_will_change(
                    object,
                    Some(Zone::Library),
                    to_zone,
                    None,
                    None,
                    None,
                    a.cause.clone(),
                );
            }
        }
        self.flush_evolving_batch();
        rebuilt
    }

    /// [CR#701.9a,616.1]: the chosen/random discard's OUTER window is a pure
    /// cant/replace GATE for the whole instruction ([CR#614.17]: a blanket
    /// "can't discard" must suppress the choice itself, before any card is
    /// picked) — it names no patient of its own (the choice hasn't happened yet
    /// at schedule time), so it commits no `Act` fact of its own. Unlike
    /// Mill/Scry's one-fact-per-instruction grain, "whenever you discard a
    /// card" fires once PER CARD, off the per-card bound-move `Discard`
    /// window each picked card recurses into (the `Each`'s
    /// `Composite(Discard, Move(It, Graveyard))` effect, dispatched through
    /// the bound-single-move arm
    /// [`apply_act_bound_move`](Self::apply_act_bound_move)) — so just run the
    /// body (the `With`/`Choose`/`Random` decision + its `Each`); no
    /// `FinalizeAct` here, nothing at this level is itself trigger-visible.
    fn apply_act_discard(&mut self, a: Act) -> GameEvent {
        let rebuilt = rebuilt_act(&a);
        let contents = a
            .contents
            .expect("a discard window carries its choice body in contents");
        self.schedule_front(vec![WorkItem::RunEffect {
            effect: Box::new(contents.body),
            frame: contents.frame,
        }]);
        rebuilt
    }

    /// [CR#701.22a,701.14a]: a reorder / fight — schedule the arrange (or
    /// reciprocal-damage) body as agenda work (a decision can't complete inside
    /// an apply), then THIS action's `FinalizeAct`. Reached only because the
    /// window passed, so a replaced/canted action records nothing, and the
    /// surviving fact lands AFTER the body ([CR#701.22d]).
    fn apply_act_reorder_or_fight(&mut self, a: Act) -> GameEvent {
        let rebuilt = rebuilt_act(&a);
        let contents = a
            .contents
            .expect("a reorder/fight window carries its body in contents");
        let mark = self.resolution_events.len();
        self.schedule_front(vec![
            WorkItem::RunEffect {
                effect: Box::new(contents.body),
                frame: contents.frame,
            },
            WorkItem::FinalizeAct {
                act: rebuilt.clone(),
                watch: crate::agenda::FinalizeWatch::BodyRan,
                mark,
            },
        ]);
        rebuilt
    }

    /// [CR#701.8a,616.1]: destroy / bound discard — commit the patient's move
    /// DIRECTLY ([CR#603.10a] LKI snapshot, move+remint), no second window. The
    /// `from`-guard was re-checked by the dispatcher (the schedule-time vet may
    /// have gone stale); a mismatch never reaches here, so the committed move
    /// is live.
    fn apply_act_bound_move(&mut self, a: &Act, object: ObjectId, to: Zone) -> GameEvent {
        let rebuilt = rebuilt_act(a);
        self.apply_zone_will_change(object, a.from, to, None, None, None, a.cause.clone());
        rebuilt
    }
}
