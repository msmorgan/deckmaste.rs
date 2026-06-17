//! Trigger event-matching ([CR#603.2,603.6]), the trigger *scan* (emit a
//! `TriggerFired` per match, whose apply notes a `NotedTrigger`), and the
//! `PlaceTriggers` barrier ([CR#603.3]) that puts noted triggers on the stack
//! in APNAP order with an `OrderTriggers` decision and a target choice at
//! placement.
//!
//! Matching is pure predicates (`event_matches`, `filter_matches_snapshot`);
//! `scan_triggers` and `place_triggers` are the scheduling/agenda-touching
//! functions.

use deckmaste_core::Ability;
use deckmaste_core::CharacteristicFilter;
use deckmaste_core::Event;
use deckmaste_core::Filter;
use deckmaste_core::Reference;
use deckmaste_core::StateFilter;
use deckmaste_core::StateFilterEvent;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::decide::PendingDecision;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::stack::Frame;
use crate::stack::StackEntry;
use crate::stack::StackObject;
use crate::state::GameState;
use crate::step::Progress;

/// The last-known information a fired trigger carries to its placement and
/// resolution ([CR#603.10a], [CR#608.2]): `~`/`This`/source, the moved object,
/// and the affected player.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerBindings {
    /// The firing object's last-known self (`~`/`This`/source).
    pub this: Option<LkiSnapshot>,
    /// The moved object for a `ZoneMove` trigger.
    pub that_object: Option<LkiSnapshot>,
    pub that_player: Option<PlayerId>,
}

/// A trigger that has fired but is not yet on the stack ([CR#603.2]). Noted by
/// applying a `TriggerFired`; placed by the `PlaceTriggers` barrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotedTrigger {
    pub source: ObjectSource,
    pub ability: usize,
    pub controller: PlayerId,
    pub bindings: TriggerBindings,
}

/// A triggered ability whose placement is mid-flight: its stack id is minted
/// and the `ChooseTargets` decision is open ([CR#603.3d]). The trigger analogue
/// of `announcing` — but a trigger is *not* an announce (no cost, no priority
/// window), so it has its own staging slot. Answering `ChooseTargets` supplies
/// the targets and pushes the committed `StackEntry`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingTrigger {
    /// The freshly minted stack identity ([CR#405]).
    pub id: ObjectId,
    pub source: ObjectSource,
    pub ability: usize,
    pub controller: PlayerId,
    pub bindings: TriggerBindings,
}

impl GameState {
    /// [CR#603.2,603.6]: does `pattern` match `event`, for an ability on
    /// `watcher`?
    ///
    /// `Dies`/`Enters` are `Event::Expanded` macros over `ZoneMove`; they are
    /// looked through transparently via the `Expanded` arm.
    pub(crate) fn event_matches(
        &self,
        pattern: &Event,
        event: &GameEvent,
        watcher: ObjectSource,
    ) -> bool {
        match pattern {
            // Look through a remembered macro invocation — `Dies`, `Enters`,
            // `Landfall`, … all expand to one of the structural forms below.
            Event::Expanded(e) => self.event_matches(&e.value, event, watcher),

            // [CR#603.6]: a `ZoneMove` pattern matches a `ZoneChanged` fact
            // when the zone constraints hold and the `what` filter matches the
            // moved object's last-known state.
            Event::ZoneMove {
                what,
                from,
                to,
                face,
                cause,
            } => {
                // P0.W6 seam: the face coordinate — emitters don't track
                // face yet, so an "enters face down" pattern must trip, not
                // silently never fire.
                if face.is_some() {
                    todo!("P0.W6: face-coordinate matching");
                }
                match event {
                    GameEvent::ZoneChanged {
                        snapshot,
                        from: ef,
                        to: et,
                        cause: ec,
                        ..
                    } => {
                        zone_ok(*from, *ef)
                            && zone_ok(*to, Some(*et))
                            && cause
                                .as_ref()
                                .is_none_or(|c| self.cause_matches(c, ec.as_ref(), watcher))
                            && self.filter_matches_snapshot(what, snapshot, watcher)
                    }
                    _ => false,
                }
            }

            // [CR#603.2b]: when a step/phase begins, abilities that trigger "at
            // the beginning of" it trigger — match the `StepBegan` fact on the
            // exact phase. The `whose` coordinate ([CR#503.1] "your upkeep")
            // narrows by whose turn it is, relative to the watching ability's
            // controller: `Your` requires the watcher's controller to be the
            // active player, `AnOpponents` requires the active player to be
            // someone else, and `EachPlayers` fires on every player's turn.
            Event::BeginningOf(step, whose) => {
                use deckmaste_core::WhoseTurn;
                if !matches!(event, GameEvent::StepBegan(s) if s == step) {
                    return false;
                }
                let active = self.turn.active_player;
                match whose {
                    WhoseTurn::EachPlayers => true,
                    // The watcher's controller anchors "your"/"an opponent's";
                    // a step trigger fires from a live battlefield permanent,
                    // so its source resolves to a live controller.
                    WhoseTurn::Your => self.controller_of_source(watcher) == Some(active),
                    WhoseTurn::AnOpponents => self
                        .controller_of_source(watcher)
                        .is_some_and(|c| c != active),
                }
            }

            // [CR#603.2e]: a "becomes [state]" transition. The `Attacking`
            // designation ([CR#508.1a]) is a live event ([CR#603.6] — it reaches
            // the trigger stage like any occurrence): match `GameEvent::Attacking`
            // against the `becomes` state and run the `of` filter against the
            // still-live attacking object. "Becomes blocked" ([CR#509.3c])
            // watches the ATTACKER; its once-per-attacker dedup lives in
            // `scan_triggers`, so the point-wise match here stays naive.
            Event::StateBecomes { of, becomes, cause } => {
                // P0.W6 seam: phasing, turn-face, and OBJECT-scope
                // designation deltas have no fact shapes yet — a pattern
                // watching one must trip, not silently never fire.
                // (`ControlChanged` is shaped; its emitter is the layers-L2
                // seam, but the fact matches below.)
                if matches!(
                    becomes,
                    StateFilterEvent::Phased(_)
                        | StateFilterEvent::TurnedFace(_)
                        | StateFilterEvent::Designated(_)
                ) {
                    todo!("P0.W6: becomes-delta matching for {becomes:?}");
                }
                // The transitioning object (still live — [CR#603.2e] deltas
                // are never zone moves) and the event's cause, where the
                // fact carries one (the tap-cause table,
                // [CR#107.5,508.1f,701.26a]).
                let (live, ec) = match (becomes, event) {
                    (StateFilterEvent::Attacking, GameEvent::Attacking(o))
                    | (StateFilterEvent::Untapped, GameEvent::Untapped(o)) => (Some(*o), None),
                    (StateFilterEvent::Tapped, GameEvent::Tapped { object, cause }) => {
                        (Some(*object), cause.as_ref())
                    }
                    // The two sides of one declaration fact ([CR#509.1g..509.1h]):
                    // `Blocked` watches the ATTACKER ("becomes blocked",
                    // [CR#509.3c]); `Blocking` watches the BLOCKER ("whenever ~
                    // blocks", [CR#509.3a]). Bushido ([CR#702.45a]) unions both.
                    // SEAM: `scan_triggers` dedups `Blocked` facts once per
                    // ATTACKER (serving the [CR#509.3c] becomes-blocked count),
                    // so in a multi-block (one attacker, N blockers) only the
                    // first blocker's fact is scanned — a second blocker's
                    // "blocks" trigger is under-counted. Single-block (the
                    // common case) is correct.
                    (StateFilterEvent::Blocked, GameEvent::Blocked { attacker: o, .. })
                    | (StateFilterEvent::Blocking, GameEvent::Blocked { blocker: o, .. }) => {
                        (Some(*o), None)
                    }
                    // "Comes under the control of [a matching player]": the
                    // inner filter runs against the NEW controller's proxy
                    // ([CR#109.5]).
                    (
                        StateFilterEvent::ControlledBy(f),
                        GameEvent::ControlChanged { object, to },
                    ) => {
                        if !self.filter_matches_live(f, self.player(*to).object, watcher) {
                            return false;
                        }
                        (Some(*object), None)
                    }
                    _ => (None, None),
                };
                live.is_some_and(|o| {
                    cause
                        .as_ref()
                        .is_none_or(|c| self.cause_matches(c, ec, watcher))
                        && self.filter_matches_live(of, o, watcher)
                })
            }

            // [CR#603.2] over the action log: a verb was performed.
            Event::Performed { verb, by, on } => {
                self.performed_matches(verb, by, on, event, watcher)
            }

            // [CR#601.2c]: an object became the target of a spell/ability at
            // announce (ward is the family exemplar, [CR#702.21a]). Both ends
            // are live: the target by definition, and the targeting object —
            // the announcing spell sits in the stack zone (its remint is the
            // one deferred move), an ability announce rides its SOURCE (the
            // stack identity isn't minted until the announce promotes,
            // [CR#602.2a]), and a placing trigger carries its minted id.
            Event::BecomesTarget { what, by } => match event {
                GameEvent::BecameTarget { target, source } => {
                    self.filter_matches_live(what, *target, watcher)
                        && by
                            .as_ref()
                            .is_none_or(|f| self.filter_matches_live(f, *source, watcher))
                }
                _ => false,
            },

            // [CR#731.1a]: a GAME-scope designation transition ("day becomes
            // night" loses one designation and gains the other). An omitted
            // `becomes` watches any transition of that designation.
            Event::DesignationChanged { name, becomes } => match event {
                GameEvent::DesignationChanged {
                    name: en,
                    becomes: eb,
                } => name == en && becomes.as_ref().is_none_or(|b| Some(b) == eb.as_ref()),
                _ => false,
            },

            // "Whenever X or Y" is a pattern union ([CR#700.1]); it still
            // fires once per matching occurrence ([CR#603.2c]).
            Event::OneOf(events) => events.iter().any(|p| self.event_matches(p, event, watcher)),

            // [CR#608.2i]: `Used` is the self/object-scoped ability-use count,
            // resolved by `EventCount`'s frame path — `by` is resolved to a
            // concrete `ObjectId` and matched against `AbilityUsed`'s object.
            // The watcher-only matcher here cannot resolve that object
            // identity (its `watcher` is an `ObjectSource`, not the resolved
            // id), so `Happened(Used)` is unbuilt — trip rather than silently
            // never fire.
            Event::Used { .. } => todo!(
                "Event::Used is counted via EventCount's frame-resolved object path \
                 ([CR#608.2i]); the watcher-only matcher cannot resolve the object \
                 identity — Happened(Used) is unbuilt"
            ),
        }
    }

    /// [CR#603.2] over the action log: does the `Performed` pattern admit
    /// this fact? Two fact families carry verbs — dedicated events (Cast →
    /// `SpellCast`, `DealDamage` → `DamageDealt`), and cause-carried views
    /// riding zone moves (the W3 unification: Sacrifice/Discard/Play,
    /// [CR#701.21a,701.9a,701.18a], whose performer is the moved object's
    /// controller). Agent-performed narrowing (Karmic Justice's "a spell or
    /// ability an opponent controls destroys…") rides `ZoneMove`'s
    /// `CausePattern` instead — `Performed`'s `by` is the PERFORMER, which
    /// for the wired cause verbs is a player. A verb outside the wired set
    /// must trip, not silently never fire.
    fn performed_matches(
        &self,
        verb: &deckmaste_core::Ident,
        by: &Filter,
        on: &Filter,
        event: &GameEvent,
        watcher: ObjectSource,
    ) -> bool {
        match verb.as_str() {
            // [CR#601.2i]: "whenever you cast" — the spell is live on the
            // stack when the fact applies; its controller (as a proxy) is
            // the caster.
            "Cast" => match event {
                GameEvent::SpellCast(o) => {
                    let caster = self.player(self.objects.obj(*o).controller).object;
                    self.filter_matches_live(by, caster, watcher)
                        && self.filter_matches_live(on, *o, watcher)
                }
                _ => false,
            },
            // [CR#120.1]: `by` is the damage SOURCE, `on` the recipient
            // (both live — an SBA death follows the fact, [CR#704.5g]).
            "DealDamage" => match event {
                GameEvent::DamageDealt { source, target, .. } => {
                    self.filter_matches_live(by, *source, watcher)
                        && self.filter_matches_live(on, *target, watcher)
                }
                _ => false,
            },
            v @ ("Sacrifice" | "Discard" | "Play") => match event {
                GameEvent::ZoneChanged {
                    snapshot,
                    cause: Some(c),
                    ..
                } if c.verb.as_str() == v => {
                    let performer = self.player(snapshot.controller).object;
                    self.filter_matches_live(by, performer, watcher)
                        && self.filter_matches_snapshot(on, snapshot, watcher)
                }
                _ => false,
            },
            // [CR#121.1]: the intent of a draw. `by` is the drawing player;
            // `on` is ignored (draws have no target object, the player is both
            // actor and recipient).
            "Draw" => match event {
                GameEvent::WillDraw { player, .. } => {
                    self.filter_matches_live(by, self.player(*player).object, watcher)
                }
                _ => false,
            },
            // [CR#119.3]: `by` is the player losing/gaining life.
            "LoseLife" => match event {
                GameEvent::LifeLost { player, .. } => {
                    self.filter_matches_live(by, self.player(*player).object, watcher)
                }
                _ => false,
            },
            // [CR#119.3]: `by` is the player gaining life.
            "GainLife" => match event {
                GameEvent::LifeGained { player, .. } => {
                    self.filter_matches_live(by, self.player(*player).object, watcher)
                }
                _ => false,
            },
            other => todo!("engine-trigger-events: Performed verb {other:?} has no wired fact"),
        }
    }

    /// Does the pattern's cause narrowing admit the event's cause triple
    /// ([CR#603.2] over the (verb, agency, agent) coordinates)?
    ///
    /// Every PRESENT coordinate must match; an omitted one matches anything.
    /// An event with NO cause (an unattributed move) fails every
    /// cause-narrowed pattern — "destroyed" admits exactly two causes
    /// ([CR#701.8b]) and a sacrifice is never one of them ([CR#701.21a]),
    /// while plain "dies" ([CR#700.4]) spells `cause: None` and never gets
    /// here. The agent filter runs against the LIVE causing object
    /// (Karmic-Justice predicates over "a spell or ability an opponent
    /// controls"): the agent is on the stack mid-resolution when its
    /// instructions emit, so it is live at scan time; turn-based and
    /// state-based actions have no agent and fail an agent-narrowed pattern.
    fn cause_matches(
        &self,
        pattern: &deckmaste_core::Cause,
        actual: Option<&crate::event::Cause>,
        watcher: ObjectSource,
    ) -> bool {
        let deckmaste_core::Cause::Cause(p) = pattern;
        let Some(cause) = actual else {
            return false;
        };
        if p.verb.as_ref().is_some_and(|v| *v != cause.verb) {
            return false;
        }
        if p.agency.is_some_and(|a| a != cause.agency) {
            return false;
        }
        match (&p.agent, cause.agent) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(f), Some((agent, _controller))) => self.filter_matches_live(f, agent, watcher),
        }
    }

    /// Evaluate `filter` against a *live* object `o` for an ability on
    /// `watcher`.
    ///
    /// The live counterpart of
    /// [`filter_matches_snapshot`](Self::filter_matches_snapshot): the
    /// transitioning object (an attacker/blocked creature) is still on the
    /// battlefield, so this is exactly [`crate::target::matches_with`] with the
    /// `watcher` supplied — the carrier anchors `Ref(This)`/`Ref(You)` and
    /// threads into nested relations/combinators.
    pub(crate) fn filter_matches_live(
        &self,
        filter: &Filter,
        o: ObjectId,
        watcher: ObjectSource,
    ) -> bool {
        crate::target::matches_with(self, o, filter, Some(watcher))
    }

    /// The controller of the live object minted from `source`, if it is
    /// still around — the anchor for `Ref(You)` in carrier-bound filters.
    pub(crate) fn controller_of_source(
        &self,
        source: ObjectSource,
    ) -> Option<crate::player::PlayerId> {
        self.objects
            .iter()
            .find(|ob| ob.source == source)
            .map(|ob| ob.controller)
    }

    /// Evaluate `filter` against the last-known state of a moved object
    /// (captured in `snapshot`), for an ability on `watcher`.
    ///
    /// Mirrors `target::matches` but sources characteristics from the snapshot
    /// instead of a live object — necessary for leaves where the object is
    /// already reminted/gone.
    #[allow(clippy::match_same_arms)] // distinct `=> false` reasons; seam arms diverge as later tasks land
    pub(crate) fn filter_matches_snapshot(
        &self,
        filter: &Filter,
        snapshot: &LkiSnapshot,
        watcher: ObjectSource,
    ) -> bool {
        match filter {
            // "this object": match only when the snapshot is the watching object
            // ([CR#603.10a] — self-dies / self-enters).
            Filter::Ref(Reference::This) => snapshot.source == watcher,

            // "a creature" — check the snapshot's printed card types.
            Filter::Characteristic(CharacteristicFilter::Type(ty)) => {
                snapshot_has_type(self, snapshot, *ty)
            }

            // "on the battlefield" (the `Permanent` macro and friends): the
            // snapshot is the object as it last existed in the zone it left
            // ([CR#603.10a]), so it matches the zone the event removed it from.
            Filter::State(StateFilter::InZone(zone)) => snapshot.left == *zone,

            // Characteristics read the PRINTED face (no layer view over a gone
            // object; LKI-derived characteristics are not captured — see the
            // snapshot-stat seam note). A player-proxy snapshot has no face.
            Filter::Characteristic(CharacteristicFilter::Named(n)) => {
                snapshot_face(self, snapshot).is_some_and(|f| f.name.as_str() == n.as_str())
            }
            Filter::Characteristic(CharacteristicFilter::Supertype(s)) => {
                snapshot_face(self, snapshot).is_some_and(|f| f.supertypes.contains(s))
            }
            Filter::Characteristic(CharacteristicFilter::ColorIs(c)) => {
                snapshot_face(self, snapshot)
                    .is_some_and(|f| crate::layer::base_colors(f).contains(c))
            }
            Filter::Characteristic(CharacteristicFilter::Multicolored) => {
                snapshot_face(self, snapshot)
                    .is_some_and(|f| crate::layer::base_colors(f).len() >= 2)
            }
            Filter::Characteristic(CharacteristicFilter::Colorless) => {
                snapshot_face(self, snapshot)
                    .is_some_and(|f| crate::layer::base_colors(f).is_empty())
            }
            // [CR#208,202.3]: the PRINTED stat. (LKI-derived P/T — a pumped
            // creature that died "with power 3" — is a capture seam: the
            // snapshot stores no P/T, so this reads the printed face for now.)
            Filter::Characteristic(CharacteristicFilter::Stat(stat, cmp, count)) => {
                crate::target::stat_satisfies(snapshot_stat(self, snapshot, *stat), *cmp, count)
            }

            // [CR#122.1]: counters are captured on the snapshot, so "had a
            // +1/+1 counter on it" reads the DYING object's last-known counters
            // ([CR#603.10a]) — the intervening-if of Undying/Persist
            // ([CR#702.93a,702.79a]). Mirrors the live `HasCounter` arm
            // (`target::matches_with`): present and positive.
            Filter::State(StateFilter::HasCounter(kind)) => {
                snapshot.counters.get(kind.as_str()).is_some_and(|&n| n > 0)
            }

            // [CR#110.5]: tap state is captured on the snapshot; flip/face/
            // phasing are not (P0.W6).
            Filter::State(StateFilter::Status(status)) => {
                use deckmaste_core::Status;
                match status {
                    Status::Tapped => snapshot.tapped,
                    Status::Untapped => !snapshot.tapped,
                    other => todo!(
                        "engine-filter-breadth: snapshot Status({other:?}) — flip/face/phasing \
                         state uncaptured (P0.W6)"
                    ),
                }
            }

            // The captured controller / the card owner, resolved to their LIVE
            // player proxies ([CR#108.3,109.5]); the inner filter runs live.
            Filter::Relation(deckmaste_core::RelationFilter::ControlledBy(f)) => {
                self.filter_matches_live(f, self.player(snapshot.controller).object, watcher)
            }
            Filter::Relation(deckmaste_core::RelationFilter::Owner(f)) => match snapshot.source {
                ObjectSource::Card(c) => {
                    let owner = self.cards.get(c).owner;
                    self.filter_matches_live(f, self.player(owner).object, watcher)
                }
                ObjectSource::Player(_) => false,
            },
            // A snapshot subject is a card, never a player proxy — the
            // player-side relations never match.
            Filter::Relation(
                deckmaste_core::RelationFilter::OpponentOf(_)
                | deckmaste_core::RelationFilter::Controls(_),
            ) => false,
            // Attachment relations over a snapshot: engine-attach tracks
            // `attached_to` on the LIVE object, but `LkiSnapshot` does not
            // capture it, so a gone object's last attachment is unknown. Reading
            // the live store by the stale id would be wrong; a loud seam beats a
            // silently-wrong `false` until the snapshot captures the relation.
            Filter::Relation(
                deckmaste_core::RelationFilter::AttachedTo(_)
                | deckmaste_core::RelationFilter::Attachment(_),
            ) => todo!(
                "engine-filter-breadth: snapshot attachment relations need LkiSnapshot to capture \
                 attached_to (engine-attach tracks it only on the live object)"
            ),

            // A gone object has no live stack entry, so it currently targets
            // nothing ([CR#115.9b] — departed objects are not read through LKI).
            Filter::State(StateFilter::Targets(_) | StateFilter::TargetCount(_)) => false,
            // [CR#607]: no linked-ability relation registry yet.
            Filter::State(StateFilter::RelatedBy(..)) => {
                todo!("engine-filter-breadth: snapshot RelatedBy needs a CR#607 relation registry")
            }

            // Logical combinators: recurse.
            Filter::AllOf(fs) => fs
                .iter()
                .all(|f| self.filter_matches_snapshot(f, snapshot, watcher)),
            Filter::OneOf(fs) => fs
                .iter()
                .any(|f| self.filter_matches_snapshot(f, snapshot, watcher)),
            Filter::Not(f) => !self.filter_matches_snapshot(f, snapshot, watcher),

            // Match-anything.
            Filter::Any => true,

            // Look through a remembered filter macro.
            Filter::Expanded(e) => self.filter_matches_snapshot(&e.value, snapshot, watcher),

            other => todo!("stage 3 does not evaluate snapshot filter {other:?}"),
        }
    }

    /// [CR#603.2,603.6]: after an occurrence applies, scan watching abilities
    /// for ones whose `event` pattern matches each occurred fact, and schedule
    /// a `TriggerFired` per match at the agenda front (so they apply in the
    /// occurrence's wake — [CR#603.3b]). Applying a `TriggerFired` is what
    /// *notes* the trigger; this only emits.
    ///
    /// Watchers ([CR#603.6]) are every live battlefield permanent, plus — for a
    /// `ZoneChanged` that LEFT the battlefield — the leaving object itself (via
    /// its snapshot), so its own dies-trigger is considered even though its
    /// abilities are gone from the battlefield ([CR#603.6c]). An *entering*
    /// object is already a live battlefield permanent, so it is not re-added.
    pub(crate) fn scan_triggers(&mut self, facts: &Occurrence) {
        let events: &[GameEvent] = match facts {
            Occurrence::Single(e) => std::slice::from_ref(e),
            Occurrence::Batch(es) => es,
        };
        let mut emits: Vec<WorkItem> = Vec::new();
        // [CR#509.3c]: "becomes blocked" fires once per ATTACKER — a
        // declaration blocking one attacker with N creatures is one batch of
        // N point-wise `Blocked` facts, so only the first per attacker is
        // scanned. (A per-blocker view — "becomes blocked by a creature",
        // [CR#509.3d] — would need the skipped facts; that pattern shape
        // doesn't exist yet.)
        let mut blocked_attackers: std::collections::HashSet<ObjectId> =
            std::collections::HashSet::new();
        for event in events {
            // Skip facts no trigger pattern watches; never scan a `TriggerFired`
            // (avoids any chance of recursion). `ZoneWillChange` is skipped because
            // trigger-matching happens on the downstream `ZoneChanged` fact (already
            // queued by the will-change apply at the agenda front — [CR#603.6]);
            // matching on the intent would double-fire every zone-move trigger.
            // `StepBegan` is NOT skipped — `BeginningOf` step/phase triggers
            // ([CR#603.2]) key off it (e.g. "at the beginning of combat on your
            // turn"); `TurnBegan` has no pattern shape that watches it.
            match event {
                GameEvent::TriggerFired { .. }
                | GameEvent::AbilityResolved(_)
                | GameEvent::TurnBegan { .. }
                | GameEvent::ZoneWillChange { .. } => continue,
                GameEvent::Blocked { attacker, .. } if !blocked_attackers.insert(*attacker) => {
                    continue;
                }
                _ => {}
            }
            self.scan_event(event, &mut emits);
        }
        if !emits.is_empty() {
            self.schedule_front(emits);
        }
    }

    /// Scan one occurred fact against every watcher, pushing a `TriggerFired`
    /// emit per match onto `emits`.
    fn scan_event(&self, event: &GameEvent, emits: &mut Vec<WorkItem>) {
        // The moved object's snapshot for a zone change (the trigger's
        // `that_object`); `None` for any other fact.
        let subject: Option<&LkiSnapshot> = match event {
            GameEvent::ZoneChanged { snapshot, .. } => Some(snapshot),
            _ => None,
        };

        // The watcher set ([CR#603.6]): every live battlefield permanent, plus
        // the leaving object's snapshot for a battlefield-leave.
        let mut watchers: Vec<Watcher> = self
            .zones
            .battlefield
            .iter()
            .map(|&id| Watcher::Live(id))
            .collect();
        if let GameEvent::ZoneChanged {
            snapshot,
            from: Some(Zone::Battlefield),
            ..
        } = event
        {
            // The leaving object — its abilities are no longer on the
            // battlefield, so add it explicitly ([CR#603.6c]).
            watchers.push(Watcher::Leaving(snapshot.clone()));
        }

        for watcher in watchers {
            let (source, controller, this) = match watcher {
                Watcher::Live(id) => {
                    let o = self.objects.obj(id);
                    (o.source, o.controller, LkiSnapshot::capture(self, id))
                }
                Watcher::Leaving(s) => (s.source, s.controller, s.clone()),
            };
            for (idx, ability) in crate::derive::abilities_of_source(self, source)
                .iter()
                .enumerate()
            {
                let Ability::Triggered(t) = ability else {
                    continue;
                };
                if !self.event_matches(&t.event, event, source) {
                    continue;
                }
                // [CR#603.10a,608.2]: the bindings the fired trigger carries —
                // also the context for the intervening-if gate ([CR#603.4]).
                let bindings = TriggerBindings {
                    this: Some(this.clone()),
                    that_object: subject.cloned(),
                    that_player: None,
                };
                // [CR#603.4]: the intervening-if gate — the condition is checked
                // when the event occurs (no targets are chosen yet, so the gate
                // frame carries none); it is rechecked at resolution.
                if let Some(c) = &t.condition {
                    let frame = Frame {
                        source: this.object,
                        controller,
                        targets: Vec::new(),
                        bindings: Some(bindings.clone()),
                        chosen: None,
                        x: None,
                        subject: None,
                        those: None,
                    };
                    if !self.condition_holds(c, &frame) {
                        continue;
                    }
                }
                emits.push(WorkItem::Emit(Occurrence::single(
                    GameEvent::TriggerFired {
                        source,
                        ability: Uint::try_from(idx).expect("ability index fits in Uint"),
                        controller,
                        bindings,
                    },
                )));
            }
        }
    }

    /// [CR#603.3]: the placement barrier. Puts noted triggers on the stack in
    /// APNAP order ([CR#603.3b]) — the active player's first (so they resolve
    /// last). One `step()` places at most one trigger (or surfaces a decision):
    /// it re-schedules itself to loop, with `CheckSbas` ahead, so a placement
    /// that produced new state re-sweeps before the next is placed.
    ///
    /// Returns `TriggersPlaced { placed }`: 1 when a trigger was placed, 0 when
    /// none were waiting or a decision (`OrderTriggers` / `ChooseTargets`) was
    /// surfaced instead.
    pub(crate) fn place_triggers(&mut self) -> Progress {
        if self.pending_triggers.is_empty() {
            return Progress::TriggersPlaced { placed: 0 };
        }

        // APNAP: the first player from the active player who still has a noted
        // trigger ([CR#603.3b]).
        let order = self.apnap_order();
        let Some(player) = order
            .into_iter()
            .find(|&p| self.pending_triggers.iter().any(|t| t.controller == p))
        else {
            // Triggers exist but none belong to a live APNAP player — drop them.
            self.pending_triggers.clear();
            return Progress::TriggersPlaced { placed: 0 };
        };

        let mine: Vec<NotedTrigger> = self
            .pending_triggers
            .iter()
            .filter(|t| t.controller == player)
            .cloned()
            .collect();

        if mine.len() > 1 {
            // [CR#603.3b]: this player orders their simultaneous triggers.
            self.pending = Some(PendingDecision::OrderTriggers {
                player,
                triggers: mine,
            });
            return Progress::TriggersPlaced { placed: 0 };
        }

        // Exactly one: place it now and loop (re-sweep, then place the next).
        let noted = self.take_first_trigger_of(player);
        let placed = self.place_one_trigger(noted);
        self.schedule_front(vec![WorkItem::CheckSbas, WorkItem::PlaceTriggers]);
        Progress::TriggersPlaced {
            placed: u32::from(placed),
        }
    }

    /// Controllers in APNAP order ([CR#603.3b]): the active player, then around
    /// the table, skipping lost players.
    fn apnap_order(&self) -> Vec<PlayerId> {
        let mut order = vec![self.turn.active_player];
        let mut p = self.turn.active_player;
        while order.len() < self.players.iter().filter(|pl| !pl.lost).count() {
            p = self.next_live_after(p);
            if p == self.turn.active_player {
                break;
            }
            order.push(p);
        }
        order
    }

    /// Removes and returns the FIRST noted trigger controlled by `player`
    /// (preserving the rest's order — the `OrderTriggers` reorder, when used,
    /// already fixed it).
    ///
    /// # Panics
    ///
    /// Panics if `player` controls no noted trigger — the caller guards this.
    pub(crate) fn take_first_trigger_of(&mut self, player: PlayerId) -> NotedTrigger {
        let i = self
            .pending_triggers
            .iter()
            .position(|t| t.controller == player)
            .expect("a noted trigger for this player");
        self.pending_triggers.remove(i)
    }

    /// Reorders `player`'s noted triggers in place to match `ordered` (the
    /// permutation chosen via `OrderTriggers`), leaving other players' notes
    /// untouched. Used by the `OrderTriggers` submission.
    pub(crate) fn reorder_pending_triggers(
        &mut self,
        player: PlayerId,
        ordered: Vec<NotedTrigger>,
    ) {
        let mut ordered = ordered.into_iter();
        for slot in &mut self.pending_triggers {
            if slot.controller == player {
                *slot = ordered
                    .next()
                    .expect("ordered covers this player's triggers");
            }
        }
        debug_assert!(
            ordered.next().is_none(),
            "ordered had more entries than this player's noted triggers",
        );
    }

    /// [CR#603.3c,603.3d]: place one noted trigger on the stack. If its ability
    /// targets and at least one legal target exists, mint the stack id and open
    /// a `ChooseTargets` decision (returns `false` — not yet placed). If it
    /// targets but has NO legal target, drop it ([CR#603.3c], returns `false`).
    /// If it does not target, push the committed `StackEntry` directly (returns
    /// `true`).
    pub(crate) fn place_one_trigger(&mut self, noted: NotedTrigger) -> bool {
        let specs = self.trigger_targets(noted.source, noted.ability);
        if specs.is_empty() {
            // No targets — push directly with a freshly minted stack id.
            let id = self
                .objects
                .mint(noted.source, noted.controller, Some(Zone::Stack));
            self.stack.push(StackEntry {
                id,
                object: StackObject::Triggered {
                    source: noted.source,
                    ability: noted.ability,
                    bindings: noted.bindings,
                },
                controller: noted.controller,
                targets: vec![],
                x: None,
            });
            return true;
        }

        // It targets ([CR#603.3d]). Mint the stack id FIRST so the
        // `Cant(Target)` filtering in `surface_target_choice` (shared with the
        // announce path) evaluates each forbidding row's `by` against the
        // trigger's real stack identity — hexproof's "abilities your opponents
        // control" reads the targeting object's controller, so a placeholder
        // id won't do.
        let controller = noted.controller;
        let id = self
            .objects
            .mint(noted.source, controller, Some(Zone::Stack));
        let _ = self.surface_target_choice(controller, specs, id);
        // [CR#603.3c]: a target with no legal choice — its candidates empty,
        // or every candidate forbidden by hexproof/protection — removes the
        // trigger from the stack, never placed. Retract the surfaced decision
        // and the minted-but-unused stack identity.
        let droppable = matches!(
            &self.pending,
            Some(PendingDecision::ChooseTargets { legal, .. }) if legal.iter().any(Vec::is_empty)
        );
        if droppable {
            self.pending = None;
            self.objects.remove(id);
            return false;
        }

        // Stage the in-flight placement; the target choice is already surfaced.
        self.placing_trigger = Some(crate::trigger::PendingTrigger {
            id,
            source: noted.source,
            ability: noted.ability,
            controller,
            bindings: noted.bindings,
        });
        false
    }

    /// The `TriggeredAbility.targets` for ability `ability` of `source`'s
    /// printed face. Empty when the ability is non-targeting.
    fn trigger_targets(&self, source: ObjectSource, ability: usize) -> Vec<TargetSpec> {
        match &crate::derive::abilities_of_source(self, source)[ability] {
            // Targets live on a top-level `Effect::Targeted` wrapper ([CR#115.1]).
            Ability::Triggered(t) => crate::resolve::top_targets(&t.effect).to_vec(),
            _ => unreachable!("a noted trigger indexes a Triggered ability"),
        }
    }

    /// [CR#603.3d]: a placing trigger's targets were chosen — push the
    /// committed `StackEntry` (using the already-minted stack id) and clear the
    /// staging slot. Called from the `ChooseTargets` submission.
    ///
    /// # Panics
    ///
    /// Panics if no trigger placement is in flight — an engine invariant (the
    /// staging slot is open across the decision), not caller input.
    pub(crate) fn commit_placing_trigger(&mut self, targets: Vec<ObjectId>) {
        let staged = self
            .placing_trigger
            .take()
            .expect("a trigger placement in flight");
        self.stack.push(StackEntry {
            id: staged.id,
            object: StackObject::Triggered {
                source: staged.source,
                ability: staged.ability,
                bindings: staged.bindings,
            },
            controller: staged.controller,
            targets,
            x: None,
        });
    }
}

/// A candidate watching object ([CR#603.6]) for the trigger scan: a live
/// battlefield permanent, or a just-left object carried by its snapshot.
enum Watcher {
    Live(ObjectId),
    Leaving(LkiSnapshot),
}

/// Whether `zone_constraint` (from the trigger pattern) is satisfied by
/// `actual` (from the `ZoneChanged` event).
///
/// `None` in the pattern means "any zone" (open constraint); `Some(z)` requires
/// an exact match.
fn zone_ok(constraint: Option<Zone>, actual: Option<Zone>) -> bool {
    match constraint {
        None => true,
        Some(z) => actual == Some(z),
    }
}

/// The printed face behind a snapshot, or `None` for a player-proxy snapshot
/// (no card spine).
fn snapshot_face<'a>(
    state: &'a GameState,
    snapshot: &LkiSnapshot,
) -> Option<&'a deckmaste_core::CardFace> {
    match snapshot.source {
        ObjectSource::Card(card_id) => Some(crate::derive::face(&state.cards.get(card_id).def)),
        ObjectSource::Player(_) => None,
    }
}

/// Whether the snapshot's card has the given type in its printed face.
fn snapshot_has_type(state: &GameState, snapshot: &LkiSnapshot, ty: Type) -> bool {
    snapshot_face(state, snapshot).is_some_and(|f| f.types.contains(&ty))
}

/// The PRINTED value of `stat` for a snapshot, or `None` when the face lacks it
/// (a land has no power) or the subject is a player proxy. The snapshot stores
/// no derived P/T, so this is the printed face — an LKI-stat capture seam.
fn snapshot_stat(
    state: &GameState,
    snapshot: &LkiSnapshot,
    stat: deckmaste_core::Stat,
) -> Option<deckmaste_core::Int> {
    let face = snapshot_face(state, snapshot)?;
    match stat {
        deckmaste_core::Stat::Power => crate::layer::base_stat(face.power.as_ref()),
        deckmaste_core::Stat::Toughness => crate::layer::base_stat(face.toughness.as_ref()),
        deckmaste_core::Stat::ManaValue => Some(
            deckmaste_core::Int::try_from(face.mana_cost.mana_value())
                .expect("mana value fits Int"),
        ),
        deckmaste_core::Stat::Loyalty | deckmaste_core::Stat::Defense => {
            todo!("engine-filter-breadth: snapshot {stat:?} stat (counter machinery unbuilt)")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Condition;
    use deckmaste_core::Event;
    use deckmaste_core::Filter;
    use deckmaste_core::Reference;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use crate::event::GameEvent;
    use crate::lki::LkiSnapshot;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::stack::Frame;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::target::matches;

    /// A minimal player-anchored gate frame (no bindings, no targets) for the
    /// trigger-fire intervening-if check ([CR#603.4]).
    fn gate_frame(state: &GameState, player: PlayerId) -> Frame {
        Frame {
            source: state.player(player).object,
            controller: player,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        }
    }

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// Build a two-player game with one Grizzly Bears forced onto the
    /// battlefield, mirroring the `bear_on_field` helper in other test modules.
    fn bear_on_field() -> (GameState, ObjectId) {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Filter::creature()))
            .expect("a Grizzly Bears in the opening hand");
        state.zones.hands[0].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// Capture a snapshot of `id` while it's still live, then build a fake
    /// `ZoneChanged` as if it moved `from → to`.
    fn zone_changed_event(state: &GameState, id: ObjectId, from: Zone, to: Zone) -> GameEvent {
        let snapshot = LkiSnapshot::capture(state, id);
        GameEvent::ZoneChanged {
            face: None,

            cause: None,
            snapshot,
            from: Some(from),
            to,
        }
    }

    // -------------------------------------------------------------------------
    // Dies(Type(Creature))
    // -------------------------------------------------------------------------

    /// `Dies(Type(Creature))` expands to
    /// `ZoneMove { what: Type(Creature), from: Battlefield, to: Graveyard }`.
    /// It must match a creature dying (battlefield → graveyard) …
    #[test]
    fn dies_type_creature_matches_creature_dying() {
        let canon = canon();
        // The dies-watcher card uses Dies(Type(Creature)) in its event.
        let watcher_card = Arc::new(canon.card("Moonlit Wake").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&watcher_card); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        // Place the watcher on the battlefield so its ObjectSource is
        // accessible. (P0's deck is mono-watcher, so any card in hand is one.)
        let watcher = *state.zones.hands[0]
            .first()
            .expect("watcher in opening hand");
        state.zones.hands[0].retain(|&o| o != watcher);
        state.objects.obj_mut(watcher).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(watcher);
        let watcher_source = state.objects.obj(watcher).source;

        // Separately put a Grizzly Bears on the battlefield.
        let bear = {
            let bears = Arc::new(canon.card("Grizzly Bears").unwrap());
            let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
            let bid = state.objects.mint(
                ObjectSource::Card(bear_card),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(bid);
            bid
        };

        // Build the `ZoneChanged` for the Grizzly Bears dying.
        let event = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);

        // The pattern from Dies(Type(Creature)) — built directly.
        let pattern = Event::ZoneMove {
            face: None,
            cause: None,
            what: Filter::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            state.event_matches(&pattern, &event, watcher_source),
            "Dies(Type(Creature)) must match a creature dying"
        );
    }

    /// … and must NOT match a creature entering (wrong direction).
    #[test]
    fn dies_type_creature_does_not_match_creature_entering() {
        let canon = canon();
        let bears = Arc::new(canon.card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Filter::creature()))
            .unwrap();
        state.zones.hands[0].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        let watcher_source = state.objects.obj(bear).source;

        // Entering event (hand → battlefield).
        let snapshot = LkiSnapshot {
            object: bear,
            source: state.objects.obj(bear).source,
            controller: PlayerId(0),
            tapped: false,
            damage: 0,
            counters: std::collections::HashMap::new(),
            left: Zone::Hand,
        };
        let enter_event = GameEvent::ZoneChanged {
            snapshot,
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
            face: None,
            cause: None,
        };

        let dies_pattern = Event::ZoneMove {
            face: None,
            cause: None,
            what: Filter::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            !state.event_matches(&dies_pattern, &enter_event, watcher_source),
            "Dies(Type(Creature)) must NOT match an entering event"
        );
    }

    /// Must NOT match a non-creature dying.
    #[test]
    fn dies_type_creature_does_not_match_non_creature_dying() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;

        // Find a Forest in hand (not a creature).
        let land = *state.zones.hands[1]
            .first()
            .expect("player 1 has cards in hand");
        let land_source = state.objects.obj(land).source;

        // Simulate a non-creature leaving the battlefield (battlefield →
        // graveyard). The zone constraints DO match Dies, so the *filter* must
        // do the rejecting — the land lacks the Creature type.
        let snapshot = LkiSnapshot {
            object: land,
            source: land_source,
            controller: PlayerId(1),
            tapped: false,
            damage: 0,
            counters: std::collections::HashMap::new(),
            left: Zone::Battlefield,
        };
        let event = GameEvent::ZoneChanged {
            snapshot,
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            face: None,
            cause: None,
        };

        let dies_pattern = Event::ZoneMove {
            face: None,
            cause: None,
            what: Filter::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            !state.event_matches(&dies_pattern, &event, watcher_source),
            "Dies(Type(Creature)) must NOT match a non-creature"
        );
    }

    // -------------------------------------------------------------------------
    // Dies(Ref(This)) — self-dies
    // -------------------------------------------------------------------------

    /// `Dies(Ref(This))` matches only when the dying object IS the watcher.
    #[test]
    fn dies_this_matches_only_self_death() {
        let canon = canon();
        let dies_card = Arc::new(canon.card("Footlight Fiend").unwrap());
        let bears = Arc::new(canon.card("Grizzly Bears").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&dies_card); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });

        // Place the dies-trigger creature on the battlefield.
        let trigger_obj = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Filter::creature()))
            .expect("trigger creature in hand");
        state.zones.hands[0].retain(|&o| o != trigger_obj);
        state.objects.obj_mut(trigger_obj).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(trigger_obj);
        let trigger_source = state.objects.obj(trigger_obj).source;

        // Place a Grizzly Bears beside it.
        let bear_card_id = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let other = state.objects.mint(
            ObjectSource::Card(bear_card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(other);
        let other_source = state.objects.obj(other).source;

        // Pattern: Dies(Ref(This))
        let self_dies = Event::ZoneMove {
            face: None,
            cause: None,
            what: Filter::Ref(Reference::This),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        // Self-death: snapshot.source == watcher
        let self_death_event =
            zone_changed_event(&state, trigger_obj, Zone::Battlefield, Zone::Graveyard);
        assert!(
            state.event_matches(&self_dies, &self_death_event, trigger_source),
            "Dies(Ref(This)) must match when the dying object is the watcher"
        );

        // Other-death: snapshot.source != watcher
        let other_death_event =
            zone_changed_event(&state, other, Zone::Battlefield, Zone::Graveyard);
        assert!(
            !state.event_matches(&self_dies, &other_death_event, trigger_source),
            "Dies(Ref(This)) must NOT match when a different creature dies"
        );

        // Watcher's own death should NOT match from watcher's OWN perspective
        // when the other creature dies (wrong watcher).
        assert!(
            !state.event_matches(&self_dies, &self_death_event, other_source),
            "Dies(Ref(This)) must NOT match when the dying object is a different watcher"
        );
    }

    // -------------------------------------------------------------------------
    // Enters(Ref(This)) — self-enters
    // -------------------------------------------------------------------------

    /// `Enters(Ref(This))` matches when the object entering is the watcher.
    #[test]
    fn enters_this_matches_own_entry() {
        let canon = canon();
        let etb_card = Arc::new(canon.card("Elvish Visionary").unwrap());
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&etb_card); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });

        // The ETB creature starts in hand; we want to simulate it entering.
        let etb_obj = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Filter::creature()))
            .expect("ETB creature in hand");
        let etb_source = state.objects.obj(etb_obj).source;

        // Pattern: Enters(Ref(This))
        let self_enters = Event::ZoneMove {
            face: None,
            cause: None,
            what: Filter::Ref(Reference::This),
            from: None,
            to: Some(Zone::Battlefield),
        };

        // Build a ZoneChanged snapshot for the ETB creature entering.
        let enters_snapshot = LkiSnapshot {
            object: etb_obj,
            source: etb_source,
            controller: PlayerId(0),
            tapped: false,
            damage: 0,
            counters: std::collections::HashMap::new(),
            // For an enter, the snapshot's `left` represents what zone it came
            // from — Hand in this case.
            left: Zone::Hand,
        };
        let enters_event = GameEvent::ZoneChanged {
            snapshot: enters_snapshot,
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
            face: None,
            cause: None,
        };

        assert!(
            state.event_matches(&self_enters, &enters_event, etb_source),
            "Enters(Ref(This)) must match when the entering object is the watcher"
        );

        // A different (placeholder) ObjectSource must not match.
        let forest_obj = *state.zones.hands[1].first().expect("player 1 hand");
        let forest_source = state.objects.obj(forest_obj).source;
        assert!(
            !state.event_matches(&self_enters, &enters_event, forest_source),
            "Enters(Ref(This)) must NOT match when the entering object is not the watcher"
        );

        // A ZoneWillChange (not ZoneChanged) must not match.
        let will_change_event = GameEvent::ZoneWillChange {
            object: etb_obj,
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
            enters: None,
            position: None,
            face: None,
            cause: None,
        };
        assert!(
            !state.event_matches(&self_enters, &will_change_event, etb_source),
            "Enters triggers must not fire on ZoneWillChange (only on ZoneChanged)"
        );
    }

    // -------------------------------------------------------------------------
    // Intervening-if: condition_holds
    // -------------------------------------------------------------------------

    /// `Condition::Exists(Type(Creature))` is true when a creature is on the
    /// battlefield, false when none is.
    #[test]
    fn condition_holds_exists_creature_true_when_creature_present() {
        let (state, _bear) = bear_on_field();
        let cond = Condition::Exists(Filter::Characteristic(CharacteristicFilter::Type(
            Type::Creature,
        )));
        assert!(
            state.condition_holds(&cond, &gate_frame(&state, PlayerId(0))),
            "Exists(Type(Creature)) should hold when a creature is on the battlefield"
        );
    }

    #[test]
    fn condition_holds_exists_creature_false_when_no_creature() {
        let forest = Arc::new(builtin().card("Forest").unwrap());
        let state = GameState::new(GameConfig {
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
        let cond = Condition::Exists(Filter::Characteristic(CharacteristicFilter::Type(
            Type::Creature,
        )));
        assert!(
            !state.condition_holds(&cond, &gate_frame(&state, PlayerId(0))),
            "Exists(Type(Creature)) should NOT hold when no creatures are present"
        );
    }

    // -------------------------------------------------------------------------
    // Event macro round-trip: Dies / Enters expand correctly
    // -------------------------------------------------------------------------

    /// Confirm that `Dies(Type(Creature))` parses and produces the expected
    /// `Event::Expanded` shape wrapping `ZoneMove`.
    #[test]
    fn dies_macro_expands_to_zone_move() {
        use deckmaste_core::Event;

        let event: Event = canon().macros.read_str("Dies(Type(Creature))").unwrap();
        let Event::Expanded(expanded) = &event else {
            panic!("expected Event::Expanded, got {event:?}");
        };
        assert_eq!(expanded.name.as_str(), "Dies");
        assert_eq!(
            *expanded.value,
            Event::ZoneMove {
                face: None,
                cause: None,
                what: Filter::creature(),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
            }
        );
    }

    /// Confirm that `Enters(Ref(This))` expands to `ZoneMove { to: Battlefield,
    /// from: None }`.
    #[test]
    fn enters_macro_expands_to_zone_move() {
        use deckmaste_core::Event;

        let event: Event = canon().macros.read_str("Enters(Ref(This))").unwrap();
        let Event::Expanded(expanded) = &event else {
            panic!("expected Event::Expanded, got {event:?}");
        };
        assert_eq!(expanded.name.as_str(), "Enters");
        assert_eq!(
            *expanded.value,
            Event::ZoneMove {
                face: None,
                cause: None,
                what: Filter::Ref(Reference::This),
                from: None,
                to: Some(Zone::Battlefield),
            }
        );
    }

    /// `Destroyed(Type(Creature))` expands to the cause-narrowed dies-view
    /// — pins that the macro BODY parses (bodies parse lazily; without an
    /// expansion no test would ever read it) and that the cause spells the
    /// mandatory `Cause(verb: …)` variant form.
    #[test]
    fn destroyed_macro_expands_to_cause_narrowed_zone_move() {
        use deckmaste_core::Cause;
        use deckmaste_core::CausePattern;
        use deckmaste_core::Event;

        let event: Event = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let Event::Expanded(expanded) = &event else {
            panic!("expected Event::Expanded, got {event:?}");
        };
        assert_eq!(expanded.name.as_str(), "Destroyed");
        assert_eq!(
            *expanded.value,
            Event::ZoneMove {
                what: Filter::creature(),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                face: None,
                cause: Some(Cause::Cause(CausePattern {
                    verb: Some("Destroy".into()),
                    agency: None,
                    agent: None,
                })),
            }
        );
    }

    // -------------------------------------------------------------------------
    // Cause-pattern matching ([CR#603.2] over the cause triple)
    // -------------------------------------------------------------------------

    /// `zone_changed_event` with a cause triple riding the fact.
    fn zone_changed_with_cause(
        state: &GameState,
        id: ObjectId,
        from: Zone,
        to: Zone,
        cause: crate::event::Cause,
    ) -> GameEvent {
        let GameEvent::ZoneChanged {
            snapshot,
            from,
            to,
            face,
            ..
        } = zone_changed_event(state, id, from, to)
        else {
            unreachable!("zone_changed_event builds a ZoneChanged");
        };
        GameEvent::ZoneChanged {
            snapshot,
            from,
            to,
            face,
            cause: Some(cause),
        }
    }

    /// The SBA destruction cause ([CR#701.8b] — one of "destroyed"'s two
    /// admitted causes; no agent, [CR#704] actions have none).
    fn sba_destroy_cause() -> crate::event::Cause {
        crate::event::Cause {
            verb: "Destroy".into(),
            agency: deckmaste_core::Agency::StateBasedAction,
            agent: None,
        }
    }

    /// The canon `Destroyed(Type(Creature))` view matches a creature dying
    /// to the lethal-damage SBA ([CR#701.8b,704.5g]) …
    #[test]
    fn destroyed_matches_destroy_caused_death() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern: Event = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let event = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            sba_destroy_cause(),
        );
        assert!(
            state.event_matches(&pattern, &event, watcher_source),
            "Destroyed must match an SBA-destroyed creature"
        );
    }

    /// … but NOT an unattributed battlefield→graveyard move — a plain
    /// "dies" ([CR#700.4]) is not "destroyed" ([CR#701.8b]).
    #[test]
    fn destroyed_does_not_match_uncaused_death() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern: Event = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let event = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
        assert!(
            !state.event_matches(&pattern, &event, watcher_source),
            "a cause-narrowed pattern must not match an unattributed move"
        );
    }

    /// … and NOT a sacrifice — sacrificing is never destruction
    /// ([CR#701.21a]).
    #[test]
    fn destroyed_does_not_match_sacrifice() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern: Event = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let event = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            crate::event::Cause {
                verb: "Sacrifice".into(),
                agency: deckmaste_core::Agency::EffectInstruction,
                agent: None,
            },
        );
        assert!(
            !state.event_matches(&pattern, &event, watcher_source),
            "Destroyed must not match a sacrifice"
        );
    }

    /// A cause-agnostic `Dies` pattern keeps matching cause-carrying moves —
    /// "dies" is the battlefield→graveyard change regardless of cause
    /// ([CR#700.4]).
    #[test]
    fn dies_pattern_stays_cause_agnostic() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::ZoneMove {
            face: None,
            cause: None,
            what: Filter::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };
        let event = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            sba_destroy_cause(),
        );
        assert!(
            state.event_matches(&pattern, &event, watcher_source),
            "Dies must stay cause-agnostic"
        );
    }

    /// The agency coordinate narrows alone: a pattern pinned to the SBA
    /// agency matches the SBA destroy but not an effect's ([CR#701.8b]'s
    /// two routes are distinguishable).
    #[test]
    fn cause_agency_coordinate_narrows() {
        use deckmaste_core::CausePattern;

        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::ZoneMove {
            face: None,
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: None,
                agency: Some(deckmaste_core::Agency::StateBasedAction),
                agent: None,
            })),
            what: Filter::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };
        let sba = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            sba_destroy_cause(),
        );
        let effect = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            crate::event::Cause {
                verb: "Destroy".into(),
                agency: deckmaste_core::Agency::EffectInstruction,
                agent: None,
            },
        );
        assert!(state.event_matches(&pattern, &sba, watcher_source));
        assert!(!state.event_matches(&pattern, &effect, watcher_source));
    }

    /// The agent coordinate runs a live filter over the causing object
    /// (Karmic-Justice predicates, events.md §3); an agentless cause fails
    /// an agent-narrowed pattern.
    #[test]
    fn cause_agent_filter_narrows() {
        use deckmaste_core::CausePattern;

        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        // A second creature stands in as the causing object (any live
        // object works for the predicate; real causes are spells/abilities).
        let agent = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = Event::ZoneMove {
            face: None,
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some("Destroy".into()),
                agency: None,
                agent: Some(Filter::Characteristic(CharacteristicFilter::Type(
                    Type::Creature,
                ))),
            })),
            what: Filter::Any,
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };
        let with_agent = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            crate::event::Cause {
                verb: "Destroy".into(),
                agency: deckmaste_core::Agency::EffectInstruction,
                agent: Some((agent, PlayerId(1))),
            },
        );
        let agentless = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            sba_destroy_cause(),
        );
        assert!(
            state.event_matches(&pattern, &with_agent, watcher_source),
            "the agent filter must match the live causing object"
        );
        assert!(
            !state.event_matches(&pattern, &agentless, watcher_source),
            "an agentless cause must fail an agent-narrowed pattern"
        );
    }

    // -------------------------------------------------------------------------
    // StateBecomes: becomes-tapped / becomes-untapped ([CR#603.2e])
    // -------------------------------------------------------------------------

    /// `StateBecomes(of: Ref(This), becomes: Tapped)` matches the watcher's
    /// own tap fact, anchored by the `of` filter — and not another object's.
    #[test]
    fn becomes_tapped_matches_the_tap_fact() {
        use deckmaste_core::StateFilterEvent;

        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(0));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = Event::StateBecomes {
            of: Filter::Ref(Reference::This),
            becomes: StateFilterEvent::Tapped,
            cause: None,
        };
        let own_tap = GameEvent::Tapped {
            object: bear,
            cause: None,
        };
        let other_tap = GameEvent::Tapped {
            object: other,
            cause: None,
        };
        assert!(
            state.event_matches(&pattern, &own_tap, watcher_source),
            "the watcher's own tap matches"
        );
        assert!(
            !state.event_matches(&pattern, &other_tap, watcher_source),
            "another object's tap fails the of-filter"
        );
        assert!(
            !state.event_matches(&pattern, &GameEvent::Untapped(bear), watcher_source),
            "an untap is not a tap"
        );
    }

    /// `StateBecomes(becomes: Untapped)` matches the untap fact and not the
    /// tap fact.
    #[test]
    fn becomes_untapped_matches_the_untap_fact() {
        use deckmaste_core::StateFilterEvent;

        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::StateBecomes {
            of: Filter::creature(),
            becomes: StateFilterEvent::Untapped,
            cause: None,
        };
        assert!(state.event_matches(&pattern, &GameEvent::Untapped(bear), watcher_source));
        assert!(!state.event_matches(
            &pattern,
            &GameEvent::Tapped {
                object: bear,
                cause: None
            },
            watcher_source
        ));
    }

    /// The tap-cause table ([CR#107.5] cost vs [CR#701.26a] effect) narrows a
    /// becomes-tapped pattern through the same cause triple as `ZoneMove`.
    #[test]
    fn becomes_tapped_cause_narrows() {
        use deckmaste_core::CausePattern;
        use deckmaste_core::StateFilterEvent;

        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::StateBecomes {
            of: Filter::Any,
            becomes: StateFilterEvent::Tapped,
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: None,
                agency: Some(deckmaste_core::Agency::CostPayment),
                agent: None,
            })),
        };
        let cost_tap = GameEvent::Tapped {
            object: bear,
            cause: Some(crate::event::Cause {
                verb: "Tap".into(),
                agency: deckmaste_core::Agency::CostPayment,
                agent: None,
            }),
        };
        let effect_tap = GameEvent::Tapped {
            object: bear,
            cause: Some(crate::event::Cause {
                verb: "Tap".into(),
                agency: deckmaste_core::Agency::EffectInstruction,
                agent: None,
            }),
        };
        assert!(state.event_matches(&pattern, &cost_tap, watcher_source));
        assert!(!state.event_matches(&pattern, &effect_tap, watcher_source));
    }

    // -------------------------------------------------------------------------
    // StateBecomes: becomes-blocked ([CR#509.3c]) — attacker-side, deduped
    // -------------------------------------------------------------------------

    /// `StateBecomes(of: Ref(This), becomes: Blocked)` watches the ATTACKER
    /// — "becomes blocked" is the attacker's transition ([CR#509.3c]); the
    /// blocker-side "blocks" views ([CR#509.3a]) are a different shape.
    #[test]
    fn becomes_blocked_matches_the_attacker_not_the_blocker() {
        use deckmaste_core::StateFilterEvent;

        let (mut state, attacker) = bear_on_field();
        let attacker_source = state.objects.obj(attacker).source;
        let blocker = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let blocker_source = state.objects.obj(blocker).source;
        let pattern = Event::StateBecomes {
            of: Filter::Ref(Reference::This),
            becomes: StateFilterEvent::Blocked,
            cause: None,
        };
        let event = GameEvent::Blocked { blocker, attacker };
        assert!(
            state.event_matches(&pattern, &event, attacker_source),
            "the attacker is the transitioning object"
        );
        assert!(
            !state.event_matches(&pattern, &event, blocker_source),
            "the blocker is not"
        );
    }

    /// One attacker blocked by two creatures is ONE "becomes blocked" event
    /// ([CR#509.3c]; [CR#700.1]'s example): the scan fires the attacker's
    /// trigger once per declaration batch, not once per blocker.
    #[test]
    fn becomes_blocked_scan_dedups_per_attacker() {
        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        // Canon Deepwood Tantiv: "Whenever this creature becomes blocked,
        // you gain 2 life."
        let (mut state, tantiv) = fixture_on_field("Deepwood Tantiv");
        let mut bear = || {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let (b1, b2) = (bear(), bear());
        state.scan_triggers(&Occurrence::Batch(vec![
            GameEvent::Blocked {
                blocker: b1,
                attacker: tantiv,
            },
            GameEvent::Blocked {
                blocker: b2,
                attacker: tantiv,
            },
        ]));
        let fired = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired { .. }))
                )
            })
            .count();
        assert_eq!(fired, 1, "double-blocking one attacker fires once");
    }

    /// Two DIFFERENT attackers blocked in the same declaration each fire —
    /// the dedup is per attacker, not per batch.
    #[test]
    fn becomes_blocked_scan_fires_per_distinct_attacker() {
        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let (mut state, t1) = fixture_on_field("Deepwood Tantiv");
        let t2 = {
            let card = Arc::new(canon().card("Deepwood Tantiv").unwrap());
            let card_id = state.cards.push(card, PlayerId(0));
            let id = state.objects.mint(
                ObjectSource::Card(card_id),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let mut bear = || {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let (b1, b2) = (bear(), bear());
        state.scan_triggers(&Occurrence::Batch(vec![
            GameEvent::Blocked {
                blocker: b1,
                attacker: t1,
            },
            GameEvent::Blocked {
                blocker: b2,
                attacker: t2,
            },
        ]));
        let fired = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired { .. }))
                )
            })
            .count();
        assert_eq!(fired, 2, "each attacker's own transition fires");
    }

    // -------------------------------------------------------------------------
    // BecomesTarget ([CR#601.2c]) — announce-time targeting facts
    // -------------------------------------------------------------------------

    /// `BecomesTarget(what: Ref(This))` matches the watcher's own
    /// became-target fact; the `by` filter narrows the targeting object
    /// (ward's "a spell or ability an opponent controls", [CR#702.21a]).
    #[test]
    fn becomes_target_matches_what_and_by() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        // A stand-in targeting object controlled by the opponent: a second
        // creature, placed on the STACK as a casting spell would sit.
        let spell = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            state
                .objects
                .mint(ObjectSource::Card(card), PlayerId(1), Some(Zone::Stack))
        };
        let pattern = Event::BecomesTarget {
            what: Filter::Ref(Reference::This),
            by: None,
        };
        let event = GameEvent::BecameTarget {
            target: bear,
            source: spell,
        };
        assert!(
            state.event_matches(&pattern, &event, watcher_source),
            "the watcher's own became-target fact matches"
        );
        let other_event = GameEvent::BecameTarget {
            target: spell,
            source: bear,
        };
        assert!(
            !state.event_matches(&pattern, &other_event, watcher_source),
            "a fact targeting a different object fails the what-filter"
        );

        // Ward's by-narrowing: an opponent-controlled stack object matches;
        // one the watcher's own controller controls does not.
        let by_opponent = Event::BecomesTarget {
            what: Filter::Ref(Reference::This),
            by: Some(Filter::Relation(
                deckmaste_core::RelationFilter::ControlledBy(Box::new(Filter::Relation(
                    deckmaste_core::RelationFilter::OpponentOf(Box::new(Filter::Ref(
                        Reference::You,
                    ))),
                ))),
            )),
        };
        assert!(
            state.event_matches(&by_opponent, &event, watcher_source),
            "an opponent's spell passes the by-filter"
        );
        let own_spell = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(0));
            state
                .objects
                .mint(ObjectSource::Card(card), PlayerId(0), Some(Zone::Stack))
        };
        let own_event = GameEvent::BecameTarget {
            target: bear,
            source: own_spell,
        };
        assert!(
            !state.event_matches(&by_opponent, &own_event, watcher_source),
            "the watcher's controller's own spell fails the by-filter"
        );
    }

    // -------------------------------------------------------------------------
    // Performed — verb facts ([CR#603.2] over the action log)
    // -------------------------------------------------------------------------

    /// Prowess's pattern ([CR#702.108a]): `Performed(verb: "Cast", by:
    /// Ref(You), on: noncreature spell)` matches the controller's own
    /// noncreature cast ([CR#601.2i]) — and not an opponent's cast or a
    /// creature spell.
    #[test]
    fn performed_cast_matches_own_noncreature_cast() {
        use deckmaste_core::ObjectKind;

        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::Performed {
            verb: "Cast".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::AllOf(vec![
                Filter::Kind(ObjectKind::Spell),
                Filter::Not(Box::new(Filter::Characteristic(
                    CharacteristicFilter::Type(Type::Creature),
                ))),
            ]),
        };
        let mut spell_on_stack = |name: &str, controller: PlayerId| {
            let card = Arc::new(canon().card(name).unwrap());
            let card_id = state.cards.push(card, controller);
            state
                .objects
                .mint(ObjectSource::Card(card_id), controller, Some(Zone::Stack))
        };
        let own_bolt = spell_on_stack("Lightning Bolt", PlayerId(0));
        let opp_bolt = spell_on_stack("Lightning Bolt", PlayerId(1));
        let own_creature = spell_on_stack("Grizzly Bears", PlayerId(0));
        assert!(
            state.event_matches(&pattern, &GameEvent::SpellCast(own_bolt), watcher_source),
            "the controller's own noncreature cast matches"
        );
        assert!(
            !state.event_matches(&pattern, &GameEvent::SpellCast(opp_bolt), watcher_source),
            "an opponent's cast fails by: Ref(You)"
        );
        assert!(
            !state.event_matches(
                &pattern,
                &GameEvent::SpellCast(own_creature),
                watcher_source
            ),
            "a creature spell fails the on-filter"
        );
    }

    /// `Performed(verb: "DealDamage", by: Ref(This))` matches the watcher's
    /// own damage facts ([CR#120.1]) — `by` is the SOURCE, `on` the recipient.
    #[test]
    fn performed_deal_damage_matches_source_and_target() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = Event::Performed {
            verb: "DealDamage".into(),
            by: Filter::Ref(Reference::This),
            on: Filter::Any,
        };
        let own_damage = GameEvent::DamageDealt {
            source: bear,
            target: other,
            amount: 2,
        };
        let others_damage = GameEvent::DamageDealt {
            source: other,
            target: bear,
            amount: 2,
        };
        assert!(
            state.event_matches(&pattern, &own_damage, watcher_source),
            "the watcher dealing damage matches by: Ref(This)"
        );
        assert!(
            !state.event_matches(&pattern, &others_damage, watcher_source),
            "damage dealt BY another source does not"
        );
    }

    /// `Performed(verb: "Sacrifice", by: Ref(You))` matches the cause-carried
    /// view of a zone move ([CR#701.21a] — the W3 unification retired the
    /// dedicated verb facts): the performer is the moved object's controller.
    #[test]
    fn performed_sacrifice_matches_cause_carried_move() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::Performed {
            verb: "Sacrifice".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        let sacrifice = |state: &GameState, id| {
            zone_changed_with_cause(
                state,
                id,
                Zone::Battlefield,
                Zone::Graveyard,
                crate::event::Cause {
                    verb: "Sacrifice".into(),
                    agency: deckmaste_core::Agency::EffectInstruction,
                    agent: None,
                },
            )
        };
        assert!(
            state.event_matches(&pattern, &sacrifice(&state, bear), watcher_source),
            "your own sacrifice matches by: Ref(You)"
        );
        // An opponent's creature sacrificed: the performer is its controller,
        // not you.
        let theirs = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        assert!(
            !state.event_matches(&pattern, &sacrifice(&state, theirs), watcher_source),
            "an opponent's sacrifice fails by: Ref(You)"
        );
        // An unattributed death is not a sacrifice ([CR#700.4] vs [CR#701.21a]).
        let plain_death = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
        assert!(
            !state.event_matches(&pattern, &plain_death, watcher_source),
            "an uncaused move performs no verb"
        );
    }

    // -------------------------------------------------------------------------
    // Becomes-deltas: control change + game-scope designation ([CR#603.2e])
    // -------------------------------------------------------------------------

    /// `StateBecomes(becomes: ControlledBy(f))` matches a `ControlChanged`
    /// fact whose NEW controller satisfies `f` — a control change is never a
    /// zone move ([CR#603.2e]; the object keeps its identity).
    #[test]
    fn controlled_by_matches_control_change() {
        use deckmaste_core::StateFilterEvent;

        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = Event::StateBecomes {
            of: Filter::creature(),
            becomes: StateFilterEvent::ControlledBy(Filter::Ref(Reference::You)),
            cause: None,
        };
        let to_you = GameEvent::ControlChanged {
            object: other,
            to: PlayerId(0),
        };
        let to_them = GameEvent::ControlChanged {
            object: other,
            to: PlayerId(1),
        };
        assert!(
            state.event_matches(&pattern, &to_you, watcher_source),
            "a creature coming under YOUR control matches ControlledBy(Ref(You))"
        );
        assert!(
            !state.event_matches(&pattern, &to_them, watcher_source),
            "one coming under an opponent's control does not"
        );
    }

    /// `DesignationChanged(name, becomes)` matches the game-scope
    /// designation flip ([CR#731.1a] — "day becomes night" loses one
    /// designation and gains the other); an omitted `becomes` watches any
    /// transition of that designation.
    #[test]
    fn designation_changed_matches_game_scope_flip() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let to_night = GameEvent::DesignationChanged {
            name: "DayNight".into(),
            becomes: Some("Night".into()),
        };
        let exact = Event::DesignationChanged {
            name: "DayNight".into(),
            becomes: Some("Night".into()),
        };
        let any_flip = Event::DesignationChanged {
            name: "DayNight".into(),
            becomes: None,
        };
        let wrong_value = Event::DesignationChanged {
            name: "DayNight".into(),
            becomes: Some("Day".into()),
        };
        let wrong_name = Event::DesignationChanged {
            name: "Monarch".into(),
            becomes: Some("Night".into()),
        };
        assert!(state.event_matches(&exact, &to_night, watcher_source));
        assert!(state.event_matches(&any_flip, &to_night, watcher_source));
        assert!(!state.event_matches(&wrong_value, &to_night, watcher_source));
        assert!(!state.event_matches(&wrong_name, &to_night, watcher_source));
    }

    // -------------------------------------------------------------------------
    // OneOf — "whenever … or …" pattern unions ([CR#603.2c,700.1])
    // -------------------------------------------------------------------------

    /// `OneOf([Dies, Enters])` matches a death, an entry, and nothing else —
    /// the watcher's text defines a disjunctive event pattern ([CR#700.1]),
    /// still firing once per matching occurrence ([CR#603.2c]).
    #[test]
    fn one_of_matches_any_branch() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let creature = Filter::creature();
        let pattern = Event::OneOf(vec![
            Event::ZoneMove {
                face: None,
                cause: None,
                what: creature.clone(),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
            },
            Event::ZoneMove {
                face: None,
                cause: None,
                what: creature,
                from: None,
                to: Some(Zone::Battlefield),
            },
        ]);

        let dies = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
        let enters = zone_changed_event(&state, bear, Zone::Hand, Zone::Battlefield);
        let exiled = zone_changed_event(&state, bear, Zone::Graveyard, Zone::Exile);
        assert!(
            state.event_matches(&pattern, &dies, watcher_source),
            "the first branch matches a death"
        );
        assert!(
            state.event_matches(&pattern, &enters, watcher_source),
            "the second branch matches an entry"
        );
        assert!(
            !state.event_matches(&pattern, &exiled, watcher_source),
            "no branch matches an exile"
        );
    }

    /// Confirm that reading `Dies(Ref(This))` yields
    /// `Filter::Ref(Reference::This)` in the `what` position — the "this
    /// object" form.
    #[test]
    fn dies_this_filter_ref_reference_this() {
        use deckmaste_core::Event;

        let event: Event = canon().macros.read_str("Dies(Ref(This))").unwrap();
        let Event::Expanded(expanded) = &event else {
            panic!("expected Event::Expanded");
        };
        let Event::ZoneMove { what, .. } = expanded.value.as_ref() else {
            panic!("expected ZoneMove inner, got {:?}", expanded.value);
        };
        assert_eq!(
            what,
            &Filter::Ref(Reference::This),
            "Dies(Ref(This)) must use Filter::Ref(Reference::This)"
        );
    }

    // -------------------------------------------------------------------------
    // The trigger scan: noting into pending_triggers
    // -------------------------------------------------------------------------

    /// Force a single card from a named fixture onto the battlefield (as
    /// player 0's, freshly minted) and return the new id. Player 1's deck is
    /// Forest fodder so the game is well-formed.
    fn fixture_on_field(card_name: &str) -> (GameState, ObjectId) {
        use crate::object::ObjectSource;

        let card = Arc::new(canon().card(card_name).unwrap());
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
        let card_id = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }

    /// Force a named canon card onto the battlefield under `controller`,
    /// freshly minted, returning the new id.
    fn put_on_field(state: &mut GameState, name: &str, controller: PlayerId) -> ObjectId {
        let card = Arc::new(canon().card(name).unwrap());
        let card_id = state.cards.push(card, controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// A `Creature dies-trigger DealDamage AnyTarget` on the battlefield with
    /// lethal damage: stepping past the SBA destroy (`CheckSbas` →
    /// `ZoneWillChange` → `ZoneChanged` → `TriggerFired` apply) notes exactly
    /// one trigger, whose `this` binding is the LKI snapshot of the (now-gone)
    /// battlefield id.
    #[test]
    fn dies_trigger_notes_into_pending_triggers() {
        use crate::agenda::WorkItem;

        let (mut state, goblin) = fixture_on_field("Footlight Fiend");
        // toughness 1 → 1 damage is lethal.
        state.objects.obj_mut(goblin).damage = 1;

        state.schedule_front(vec![WorkItem::CheckSbas]);
        for _ in 0..30 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }

        assert_eq!(
            state.pending_triggers.len(),
            1,
            "the self-dies trigger must be noted exactly once"
        );
        let noted = &state.pending_triggers[0];
        assert_eq!(noted.ability, 0);
        assert_eq!(noted.controller, PlayerId(0));
        assert!(
            noted.bindings.this.is_some(),
            "LKI snapshot of the dead goblin"
        );
        // The snapshot's object id is the (now-gone) battlefield id.
        assert_eq!(noted.bindings.this.as_ref().unwrap().object, goblin);
        // The dead object is truly gone from the store.
        assert!(state.objects.get(goblin).is_none());
        // `that_object` for a zone-move trigger is the moved object's snapshot.
        assert_eq!(
            noted.bindings.that_object.as_ref().unwrap().object,
            goblin,
            "the moved object's snapshot rides as that_object"
        );
    }

    /// A non-watching board: a `Grizzly Bears` dying notes NOTHING (it has
    /// no triggered abilities, and no other watcher cares).
    #[test]
    fn vanilla_creature_dying_notes_nothing() {
        use crate::agenda::WorkItem;

        let (mut state, bear) = fixture_on_field("Grizzly Bears");
        // Grizzly Bears has toughness 2; set lethal damage.
        state.objects.obj_mut(bear).damage = 2;

        state.schedule_front(vec![WorkItem::CheckSbas]);
        for _ in 0..30 {
            // Stop once the bear is gone (the death has been fully processed).
            if state.objects.get(bear).is_none() && state.agenda.is_empty() {
                break;
            }
            let _ = state.step();
        }

        assert!(
            state.objects.get(bear).is_none(),
            "the bear should have died and reminted to the graveyard"
        );
        assert!(
            state.pending_triggers.is_empty(),
            "a vanilla creature dying watches nothing — no trigger noted"
        );
    }

    // -------------------------------------------------------------------------
    // BeginningOf — step/phase-entry triggers ([CR#603.2b])
    // -------------------------------------------------------------------------

    /// A synthetic Goblin Rabblemaster: a red Goblin creature whose sole
    /// ability is "At the beginning of combat on your turn, create a 1/1 red
    /// Goblin creature token with haste" — i.e.
    /// `Triggered(event: BeginningOf(Combat(BeginningOfCombat), Your),
    /// effect: Create(1, Token(1/1 red Goblin)))`.
    fn rabblemaster() -> deckmaste_core::Card {
        use deckmaste_core::Ability;
        use deckmaste_core::Action;
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        use deckmaste_core::Color;
        use deckmaste_core::CombatStep;
        use deckmaste_core::Count;
        use deckmaste_core::Effect;
        use deckmaste_core::Event;
        use deckmaste_core::Phase;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::Reference;
        use deckmaste_core::StatValue;
        use deckmaste_core::Token;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::WhoseTurn;

        let goblin_token = Token {
            color_indicator: vec![Color::Red],
            supertypes: vec![],
            types: vec![Type::Creature],
            subtypes: vec![],
            abilities: vec![],
            power: Some(StatValue::Number(1)),
            toughness: Some(StatValue::Number(1)),
        };
        Card::Normal(CardFace {
            name: "Rabblemaster".into(),
            types: vec![Type::Creature],
            abilities: vec![Ability::Triggered(TriggeredAbility {
                event: Event::BeginningOf(
                    Phase::Combat(CombatStep::BeginningOfCombat),
                    WhoseTurn::Your,
                ),
                condition: None,
                limits: Vec::new(),
                effect: Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::Create(Count::Literal(1), goblin_token.into()),
                )),
            })],
            ..CardFace::default()
        })
    }

    /// Force a synthetic in-Rust card onto the battlefield under `controller`.
    fn put_synthetic_on_field(
        state: &mut GameState,
        card: deckmaste_core::Card,
        controller: PlayerId,
    ) -> ObjectId {
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// A two-player game with no permanents, active player = P0.
    fn empty_game() -> GameState {
        let forest = Arc::new(builtin().card("Forest").unwrap());
        GameState::new(GameConfig {
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
        })
    }

    /// [CR#603.2b]: when the beginning-of-combat step begins, a
    /// "at the beginning of combat on your turn" ability fires — the
    /// `StepBegan` fact must reach the scan, and `WhoseTurn::Your` must pass
    /// on the controller's own turn.
    #[test]
    fn beginning_of_combat_your_turn_fires_on_controllers_turn() {
        use deckmaste_core::CombatStep;
        use deckmaste_core::Phase;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        let rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(Phase::Combat(
            CombatStep::BeginningOfCombat,
        ))));

        let fired: Vec<&WorkItem> = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired { .. }))
                )
            })
            .collect();
        assert_eq!(
            fired.len(),
            1,
            "beginning-of-combat trigger must fire on the controller's turn"
        );
        let WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired { source, .. })) = fired[0]
        else {
            unreachable!()
        };
        assert_eq!(*source, state.objects.obj(rabble).source);
    }

    /// [CR#503.1]: a `WhoseTurn::Your`-scoped step trigger does NOT fire on
    /// an opponent's turn.
    #[test]
    fn beginning_of_combat_your_turn_silent_on_opponents_turn() {
        use deckmaste_core::CombatStep;
        use deckmaste_core::Phase;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        // The opponent (P1) is the active player; the Rabblemaster's controller
        // is P0, so "on your turn" must not be satisfied.
        state.turn.active_player = PlayerId(1);
        let _rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(Phase::Combat(
            CombatStep::BeginningOfCombat,
        ))));

        let fired = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired { .. }))
                )
            })
            .count();
        assert_eq!(
            fired, 0,
            "a `Your`-scoped trigger must stay silent on an opponent's turn"
        );
    }

    /// A different step's `StepBegan` (upkeep) does not match a
    /// beginning-of-combat trigger — the phase must match exactly
    /// ([CR#603.2b]).
    #[test]
    fn beginning_of_combat_trigger_ignores_other_steps() {
        use deckmaste_core::BeginningStep;
        use deckmaste_core::CombatStep;
        use deckmaste_core::Phase;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        let _rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        // The upkeep step begins, not combat.
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(Phase::Beginning(
            BeginningStep::Upkeep,
        ))));
        assert_eq!(
            state
                .agenda
                .iter()
                .filter(|w| matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired { .. }))
                ))
                .count(),
            0,
            "a beginning-of-combat trigger must ignore the upkeep step"
        );
        // Sanity: the same trigger DOES fire on the matching step.
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(Phase::Combat(
            CombatStep::BeginningOfCombat,
        ))));
        assert_eq!(
            state
                .agenda
                .iter()
                .filter(|w| matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired { .. }))
                ))
                .count(),
            1,
            "the matching beginning-of-combat step fires it"
        );
    }

    /// End-to-end ([CR#603.2b,701.7a]): stepping the engine into the
    /// beginning-of-combat step on the controller's turn drives the whole
    /// pipeline (`StepBegan` → scan → place → resolve) and the Goblin token
    /// reaches the battlefield.
    #[test]
    fn rabblemaster_mints_a_goblin_at_beginning_of_combat() {
        use deckmaste_core::CombatStep;
        use deckmaste_core::ObjectKind;
        use deckmaste_core::Phase;

        use crate::agenda::WorkItem;
        use crate::decide::Action;
        use crate::decide::Decision;
        use crate::decide::PendingDecision;
        use crate::step::StepOutcome;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        state.turn.current = Phase::PrecombatMain;
        let rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        // Schedule the beginning-of-combat step directly, then step the engine,
        // passing priority whenever it asks so the placed trigger resolves.
        state.schedule_front(vec![WorkItem::BeginStep(Phase::Combat(
            CombatStep::BeginningOfCombat,
        ))]);
        let goblin_exists = |state: &GameState| {
            state.zones.battlefield.iter().any(|&id| {
                id != rabble && crate::target::object_kind(state, id) == ObjectKind::Token
            })
        };
        for _ in 0..200 {
            if goblin_exists(&state) {
                break;
            }
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(PendingDecision::Priority { .. }) => {
                    state.submit_decision(Decision::Act(Action::Pass)).unwrap();
                }
                StepOutcome::NeedsDecision(other) => {
                    panic!("unexpected decision before the token minted: {other:?}")
                }
                StepOutcome::GameOver(o) => panic!("unexpected game over: {o:?}"),
            }
        }

        let goblins: Vec<ObjectId> = state
            .zones
            .battlefield
            .iter()
            .copied()
            .filter(|&id| id != rabble)
            .collect();
        assert_eq!(
            goblins.len(),
            1,
            "exactly one Goblin token minted at beginning of combat"
        );
        assert_eq!(
            crate::target::object_kind(&state, goblins[0]),
            ObjectKind::Token,
            "[CR#111.6]: it is a token, not a card"
        );
    }

    // -------------------------------------------------------------------------
    // PlaceTriggers: placement on the stack ([CR#603.3])
    // -------------------------------------------------------------------------

    /// A single non-targeting noted trigger places DIRECTLY onto the stack as a
    /// `Triggered` object with a fresh id, no decision surfaced.
    #[test]
    fn non_targeting_trigger_places_directly() {
        use crate::stack::StackObject;

        let (mut state, etb) = fixture_on_field("Elvish Visionary");
        let source = state.objects.obj(etb).source;
        let controller = state.objects.obj(etb).controller;
        // Note one trigger by hand (ability 0 = the DrawCards etb).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller,
            bindings: super::TriggerBindings {
                this: None,
                that_object: None,
                that_player: None,
            },
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 1 },
            "the non-targeting trigger places without a decision"
        );
        assert!(state.pending.is_none(), "no decision surfaced");
        assert!(state.pending_triggers.is_empty(), "the note was consumed");
        assert_eq!(state.stack.len(), 1, "one Triggered object on the stack");
        let entry = &state.stack[0];
        assert!(
            matches!(entry.object, StackObject::Triggered { ability: 0, .. }),
            "the entry is the etb trigger"
        );
        assert_ne!(entry.id, etb, "the stack id is a freshly minted token");
        assert!(entry.targets.is_empty(), "a non-targeting trigger has none");
    }

    /// A targeting noted trigger surfaces a `ChooseTargets` at placement
    /// ([CR#603.3d]): the stack id is minted and staged in `placing_trigger`,
    /// and nothing is on the stack until the target is chosen. (The
    /// no-legal-target drop, [CR#603.3c], can't be hit here — "any target"
    /// always admits the two player proxies.)
    #[test]
    fn targeting_trigger_surfaces_choose_targets_at_placement() {
        use crate::decide::PendingDecision;

        let (mut state, gob) = fixture_on_field("Footlight Fiend");
        let source = state.objects.obj(gob).source;
        let controller = state.objects.obj(gob).controller;
        // Note the dies-trigger (ability 0, targets [AnyTarget]).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller,
            bindings: super::TriggerBindings {
                this: None,
                that_object: None,
                that_player: None,
            },
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "a target choice surfaces instead of an immediate placement"
        );
        let Some(PendingDecision::ChooseTargets { player, legal, .. }) = &state.pending else {
            panic!("expected ChooseTargets, got {:?}", state.pending);
        };
        assert_eq!(*player, controller);
        assert!(
            !legal[0].is_empty(),
            "AnyTarget admits the player proxies (and the goblin)"
        );
        assert!(
            state.placing_trigger.is_some(),
            "the placement is staged across the decision"
        );
        assert!(
            state.stack.is_empty(),
            "nothing placed until the target is chosen"
        );
    }

    /// [CR#702.11b]: a targeting trigger's legal candidates EXCLUDE an
    /// opponent's hexproof permanent — placement must apply the same
    /// `Cant(Target)` filtering the announce path does ([CR#601.2c]), or a
    /// trigger could illegally target a hexproof permanent.
    #[test]
    fn targeting_trigger_excludes_opponent_hexproof() {
        use crate::decide::PendingDecision;

        // A `Footlight Fiend` dies-trigger (P0, "any target").
        let (mut state, fiend) = fixture_on_field("Footlight Fiend");
        let source = state.objects.obj(fiend).source;
        let controller = state.objects.obj(fiend).controller; // P0
        // P1 (an opponent) controls a hexproof `Gladecover Scout`.
        let scout = put_on_field(&mut state, "Gladecover Scout", PlayerId(1));

        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller,
            bindings: super::TriggerBindings {
                this: None,
                that_object: None,
                that_player: None,
            },
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "a target choice surfaces instead of an immediate placement"
        );
        let Some(PendingDecision::ChooseTargets { legal, .. }) = &state.pending else {
            panic!("expected ChooseTargets, got {:?}", state.pending);
        };
        assert!(
            !legal[0].contains(&scout),
            "[CR#702.11b]: P0's trigger can't target P1's hexproof Scout"
        );
        // The filter didn't nuke the whole set — the player proxies (and the
        // fiend itself) are still legal "any target" choices.
        assert!(
            !legal[0].is_empty(),
            "any target still admits the players and the fiend"
        );
    }

    /// [CR#702.11b]: hexproof only stops OPPONENTS — a controller may still
    /// target their OWN hexproof permanent with a triggered ability (the
    /// filtering must not over-exclude).
    #[test]
    fn targeting_trigger_may_target_own_hexproof() {
        use crate::decide::PendingDecision;

        let (mut state, fiend) = fixture_on_field("Footlight Fiend");
        let source = state.objects.obj(fiend).source;
        let controller = state.objects.obj(fiend).controller; // P0
        // The SAME player (P0) controls the hexproof `Gladecover Scout`.
        let scout = put_on_field(&mut state, "Gladecover Scout", PlayerId(0));

        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller,
            bindings: super::TriggerBindings {
                this: None,
                that_object: None,
                that_player: None,
            },
        });

        state.place_triggers();
        let Some(PendingDecision::ChooseTargets { legal, .. }) = &state.pending else {
            panic!("expected ChooseTargets, got {:?}", state.pending);
        };
        assert!(
            legal[0].contains(&scout),
            "[CR#702.11b]: hexproof does not stop the permanent's own controller"
        );
    }

    /// [CR#603.3c]: a targeting trigger whose ONLY candidate is an opponent's
    /// hexproof permanent has no legal target — it is removed from the stack,
    /// never placed, and the stack id minted for the (now-empty) target choice
    /// is cleaned up rather than left dangling in the object store.
    #[test]
    fn targeting_trigger_with_only_hexproof_target_is_dropped() {
        use deckmaste_core::Card;

        // No curated canon card carries a creature-ONLY-target triggered
        // ability (canon's targeting triggers all use "any target", which
        // always admits the player proxies, so the drop can't be reached with
        // them). This synthesizes the minimal card that does — engine-path
        // scaffolding to exercise [CR#603.3c], not a corpus mock.
        let card: Card = canon()
            .macros
            .read_str(
                r#"Normal(
                    name: "Test Targeted Pinger",
                    mana_cost: [Red],
                    types: [Artifact],
                    abilities: [
                        Triggered(
                            event: ThisEnters,
                            effect: Targeted(targets: [TargetOne(Creature)], effect: DealDamage(Target(0), 1)),
                        ),
                    ],
                )"#,
            )
            .unwrap();

        // The pinger (a non-creature, so not itself a legal "target creature")
        // belongs to P0; the only creature on the board is P1's hexproof Scout.
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
        let pinger_card = state.cards.push(Arc::new(card), PlayerId(0));
        let pinger = state.objects.mint(
            ObjectSource::Card(pinger_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(pinger);
        put_on_field(&mut state, "Gladecover Scout", PlayerId(1));

        let objects_before = state.objects.iter().count();
        state.pending_triggers.push(super::NotedTrigger {
            source: state.objects.obj(pinger).source,
            ability: 0,
            controller: PlayerId(0),
            bindings: super::TriggerBindings {
                this: None,
                that_object: None,
                that_player: None,
            },
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "the trigger neither places nor is counted — it has no legal target"
        );
        assert!(
            state.pending.is_none(),
            "no ChooseTargets surfaces for a trigger with no legal target"
        );
        assert!(
            state.placing_trigger.is_none(),
            "no placement is staged for the dropped trigger"
        );
        assert!(state.stack.is_empty(), "nothing reached the stack");
        // The minted stack id was retracted, not orphaned: the object count is
        // back to where it started and no stray Stack-zone object lingers.
        assert_eq!(
            state.objects.iter().count(),
            objects_before,
            "the minted-but-unused stack id is removed"
        );
        assert!(
            state.objects.iter().all(|o| o.zone != Some(Zone::Stack)),
            "no dangling Stack-zone object from the dropped trigger"
        );
    }

    /// [CR#603.3b]: a player controlling TWO simultaneous triggers surfaces an
    /// `OrderTriggers` decision; the submitted permutation becomes the
    /// placement order (last placed resolves first).
    #[test]
    fn two_triggers_one_player_surface_order_triggers() {
        use crate::decide::Decision;
        use crate::decide::PendingDecision;

        // Two dies-watchers under player 0 (non-targeting `LoseLife`).
        let (mut state, w0) = fixture_on_field("Moonlit Wake");
        let source = state.objects.obj(w0).source;
        let bindings = || super::TriggerBindings {
            this: None,
            that_object: None,
            that_player: None,
        };
        // Two notes from the same controller (as if two creatures died at once).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller: PlayerId(0),
            bindings: bindings(),
        });
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            controller: PlayerId(0),
            bindings: bindings(),
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "ordering is needed first — nothing placed yet"
        );
        let Some(PendingDecision::OrderTriggers { player, triggers }) = &state.pending else {
            panic!("expected OrderTriggers, got {:?}", state.pending);
        };
        assert_eq!(*player, PlayerId(0));
        assert_eq!(triggers.len(), 2, "both of player 0's triggers offered");

        // A non-permutation is rejected.
        assert!(state.submit_decision(Decision::Order(vec![0, 0])).is_err());
        assert!(state.submit_decision(Decision::Order(vec![5])).is_err());
        // The valid permutation is accepted; placement resumes (re-scheduled).
        state.submit_decision(Decision::Order(vec![1, 0])).unwrap();
        assert!(state.pending.is_none());
        assert!(
            state
                .agenda
                .iter()
                .any(|w| matches!(w, crate::agenda::WorkItem::PlaceTriggers)),
            "placement is re-scheduled after ordering"
        );
    }

    // -------------------------------------------------------------------------
    // Snapshot matching: characteristic / status / relation arms over LKI
    // -------------------------------------------------------------------------

    /// Capture an LKI snapshot of a 2/2 green Grizzly Bears (owned & controlled
    /// by P0). Returns (state, snapshot, watcher = the bear's source).
    fn bear_snapshot() -> (GameState, LkiSnapshot, ObjectSource) {
        let (state, bear) = bear_on_field();
        let snap = LkiSnapshot::capture(&state, bear);
        let watcher = state.objects.obj(bear).source;
        (state, snap, watcher)
    }

    fn snap_cf(c: CharacteristicFilter) -> Filter {
        Filter::Characteristic(c)
    }

    /// `Named` over a snapshot reads the printed face name ([CR#201]).
    #[test]
    fn snapshot_named_matches_printed_name() {
        let (state, snap, w) = bear_snapshot();
        assert!(state.filter_matches_snapshot(
            &snap_cf(CharacteristicFilter::Named("Grizzly Bears".into())),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicFilter::Named("Forest".into())),
            &snap,
            w
        ));
    }

    /// Color / supertype over a snapshot read the printed face.
    #[test]
    fn snapshot_color_and_supertype_read_printed_face() {
        use deckmaste_core::Color;
        use deckmaste_core::Supertype;
        let (state, snap, w) = bear_snapshot();
        assert!(state.filter_matches_snapshot(
            &snap_cf(CharacteristicFilter::ColorIs(Color::Green)),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicFilter::ColorIs(Color::Red)),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicFilter::Multicolored),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicFilter::Colorless),
            &snap,
            w
        ));
        // A Grizzly Bears is not a Basic.
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicFilter::Supertype(Supertype::Basic)),
            &snap,
            w
        ));
    }

    /// `Stat` over a snapshot reads the printed power/toughness/mana value.
    #[test]
    fn snapshot_stat_reads_printed_stats() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::Stat;
        let (state, snap, w) = bear_snapshot();
        let f = |s, c, n| snap_cf(CharacteristicFilter::Stat(s, c, Count::Literal(n)));
        assert!(state.filter_matches_snapshot(&f(Stat::Power, Cmp::Eq, 2), &snap, w));
        assert!(state.filter_matches_snapshot(&f(Stat::Toughness, Cmp::AtLeast, 2), &snap, w));
        assert!(state.filter_matches_snapshot(&f(Stat::ManaValue, Cmp::Eq, 2), &snap, w));
        assert!(!state.filter_matches_snapshot(&f(Stat::Power, Cmp::Greater, 2), &snap, w));
    }

    /// `Status(Tapped)`/`Status(Untapped)` read the snapshot's tap flag.
    #[test]
    fn snapshot_status_reads_tap_flag() {
        use deckmaste_core::StateFilter;
        use deckmaste_core::Status;
        let (mut state, bear) = bear_on_field();
        state.objects.obj_mut(bear).tapped = true;
        let snap = LkiSnapshot::capture(&state, bear);
        let w = state.objects.obj(bear).source;
        assert!(state.filter_matches_snapshot(
            &Filter::State(StateFilter::Status(Status::Tapped)),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &Filter::State(StateFilter::Status(Status::Untapped)),
            &snap,
            w
        ));
    }

    /// `ControlledBy`/`Owner` over a snapshot resolve the captured controller /
    /// the card owner to their LIVE player proxies; `OpponentOf`/`Controls`
    /// never match (a snapshot subject is a card, not a player).
    #[test]
    fn snapshot_relations_resolve_to_live_players() {
        use deckmaste_core::RelationFilter;
        use deckmaste_core::StateFilter;
        let (mut state, bear) = bear_on_field();
        state
            .objects
            .obj_mut(state.players[0].object)
            .counters
            .insert("mark".into(), 1);
        let snap = LkiSnapshot::capture(&state, bear);
        let w = state.objects.obj(bear).source;
        let marked = || Box::new(Filter::State(StateFilter::HasCounter("mark".into())));
        assert!(state.filter_matches_snapshot(
            &Filter::Relation(RelationFilter::ControlledBy(marked())),
            &snap,
            w
        ));
        assert!(state.filter_matches_snapshot(
            &Filter::Relation(RelationFilter::Owner(marked())),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &Filter::Relation(RelationFilter::OpponentOf(Box::new(Filter::Any))),
            &snap,
            w
        ));
    }

    // -------------------------------------------------------------------------
    // AbilityUsed history fact
    // -------------------------------------------------------------------------

    /// When a triggered ability fires (the `TriggerFired` event is applied),
    /// an `AbilityUsed` fact must be recorded in history for that same
    /// source/ability pair.
    #[test]
    fn trigger_fire_records_ability_used() {
        use deckmaste_core::Window;

        use crate::agenda::WorkItem;

        let (mut state, goblin) = fixture_on_field("Footlight Fiend");
        // toughness 1 → 1 damage is lethal.
        state.objects.obj_mut(goblin).damage = 1;

        state.schedule_front(vec![WorkItem::CheckSbas]);
        for _ in 0..30 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }

        assert!(
            !state.pending_triggers.is_empty(),
            "precondition: the dies-trigger must have fired"
        );

        // The TriggerFired apply must have recorded an AbilityUsed fact
        // in history for ability index 0 on the Footlight Fiend.
        // Use-limits are object-scoped: record the per-instance ObjectId,
        // not the persistent CardId/ObjectSource ([CR#400.7]).
        let turn = state.turn.turn_number;
        let found = state.history.scan(Window::ThisGame, turn).any(|e| {
            matches!(
                e,
                GameEvent::AbilityUsed { object, ability }
                    if *object == goblin && *ability == 0
            )
        });
        assert!(
            found,
            "TriggerFired apply must record GameEvent::AbilityUsed {{ object: {goblin:?}, ability: 0 }} in history",
        );
    }

    // -------------------------------------------------------------------------
    // Triggered-ability use limits ([CR#603.2h] once-each-turn /
    // [CR#702.177a]-flavored once-per-game), enforced at the `TriggerFired`
    // apply (note) site ([CR#603.2c]).
    // -------------------------------------------------------------------------

    /// A synthetic creature whose sole ability is a "whenever a creature dies"
    /// trigger ([CR#603.6] `Dies(Creature)` → `ZoneMove(what: Creature,
    /// from: Battlefield, to: Graveyard)`) carrying `limits`, gaining its
    /// controller 1 life. Models `rabblemaster`.
    fn dies_watcher(limits: Vec<deckmaste_core::UseLimit>) -> deckmaste_core::Card {
        use deckmaste_core::Ability;
        use deckmaste_core::Action;
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        use deckmaste_core::Count;
        use deckmaste_core::Effect;
        use deckmaste_core::Event;
        use deckmaste_core::PlayerAction;
        use deckmaste_core::Reference;
        use deckmaste_core::StatValue;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::Zone;

        Card::Normal(CardFace {
            name: "Death Watcher".into(),
            types: vec![Type::Creature],
            abilities: vec![Ability::Triggered(TriggeredAbility {
                event: Event::ZoneMove {
                    what: Filter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Some(Zone::Graveyard),
                    face: None,
                    cause: None,
                },
                condition: None,
                limits,
                effect: Effect::Act(Action::By(
                    Reference::You,
                    PlayerAction::GainLife(Count::Literal(1)),
                )),
            })],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            ..CardFace::default()
        })
    }

    /// Build a two-player game with a `Death Watcher` (carrying `limits`) on
    /// the battlefield under P0, plus a separate function for spawning vanilla
    /// victims that can be lethally damaged.
    fn watcher_game(limits: Vec<deckmaste_core::UseLimit>) -> (GameState, ObjectId) {
        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        let watcher = put_synthetic_on_field(&mut state, dies_watcher(limits), PlayerId(0));
        // Drop the fresh game's queued turn machinery (`BeginStep(Untap)` and
        // its cascade) so `drain_sbas` drives ONLY the SBA-death cascade in
        // isolation — otherwise the leftover turn work runs once the sweep
        // settles and contaminates the count.
        state.agenda.clear();
        (state, watcher)
    }

    /// A fresh Grizzly Bears (toughness 2) on the battlefield under P1, marked
    /// with lethal damage so the next SBA sweep destroys it.
    fn doomed_bear(state: &mut GameState) -> ObjectId {
        let bear = put_on_field(state, "Grizzly Bears", PlayerId(1));
        state.objects.obj_mut(bear).damage = 2;
        bear
    }

    /// Drive `CheckSbas` (which self-perpetuates) until the doomed creatures
    /// are gone, applying any `TriggerFired` that the death scan emits. Does
    /// NOT schedule `PlaceTriggers`, so noted triggers stay in
    /// `pending_triggers` for inspection.
    fn drain_sbas(state: &mut GameState) {
        use crate::agenda::WorkItem;
        state.schedule_front(vec![WorkItem::CheckSbas]);
        for _ in 0..60 {
            if state.agenda.is_empty() {
                break;
            }
            let _ = state.step();
        }
    }

    /// Count noted triggers whose firing object (`bindings.this`) is `obj`.
    fn noted_for(state: &GameState, obj: ObjectId) -> usize {
        state
            .pending_triggers
            .iter()
            .filter(|n| n.bindings.this.as_ref().is_some_and(|t| t.object == obj))
            .count()
    }

    /// [CR#603.2h]: a once-per-turn triggered ability fires at most once each
    /// turn. Two creatures dying in sequence this turn note the watcher's
    /// trigger exactly once; a new turn resets it (the `ThisTurn` history
    /// window — no reset hook).
    #[test]
    fn once_per_turn_trigger_fires_once_per_turn() {
        use deckmaste_core::UseLimit;

        let (mut state, watcher) = watcher_game(vec![UseLimit::OncePerTurn]);

        // First death this turn → the watcher's trigger notes once.
        doomed_bear(&mut state);
        drain_sbas(&mut state);
        assert_eq!(
            noted_for(&state, watcher),
            1,
            "the first creature death notes the once-per-turn trigger"
        );

        // Second death SAME turn → the gate suppresses it; still exactly one.
        doomed_bear(&mut state);
        drain_sbas(&mut state);
        assert_eq!(
            noted_for(&state, watcher),
            1,
            "a second death the same turn must NOT note the once-per-turn trigger again"
        );

        // A new turn resets the `ThisTurn` window — the trigger fires again.
        state.turn.turn_number += 1;
        doomed_bear(&mut state);
        drain_sbas(&mut state);
        assert_eq!(
            noted_for(&state, watcher),
            2,
            "a new turn resets the once-per-turn limit — the trigger notes again"
        );
    }

    /// [CR#603.2c]: a single multi-occurrence event (two creatures dying
    /// simultaneously is one batch of two `ZoneChanged` facts) triggers a
    /// once-per-turn ability only once. Both `TriggerFired`s are emitted in
    /// one scan pass; the apply-time gate dedups them.
    #[test]
    fn once_per_turn_trigger_fires_once_for_simultaneous_occurrences() {
        use deckmaste_core::UseLimit;

        let (mut state, watcher) = watcher_game(vec![UseLimit::OncePerTurn]);

        // Two creatures lethally damaged → one SBA sweep destroys both in a
        // single `Occurrence::Batch`, so the watcher's "creature dies" trigger
        // is fired twice in one scan pass before either `TriggerFired` applies.
        doomed_bear(&mut state);
        doomed_bear(&mut state);
        drain_sbas(&mut state);

        assert_eq!(
            noted_for(&state, watcher),
            1,
            "simultaneous deaths fire a once-per-turn trigger exactly once ([CR#603.2c])"
        );
    }

    /// [CR#702.177a]-flavored once-per-game: the trigger fires once and never
    /// again — not later the same turn, not after a turn bump (the `ThisGame`
    /// window spans every turn).
    #[test]
    fn once_per_game_trigger_fires_once_per_game() {
        use deckmaste_core::UseLimit;

        let (mut state, watcher) = watcher_game(vec![UseLimit::OncePerGame]);

        doomed_bear(&mut state);
        drain_sbas(&mut state);
        assert_eq!(
            noted_for(&state, watcher),
            1,
            "the first creature death notes the once-per-game trigger"
        );

        // Same turn, another death → suppressed.
        doomed_bear(&mut state);
        drain_sbas(&mut state);
        assert_eq!(
            noted_for(&state, watcher),
            1,
            "a once-per-game trigger does not fire a second time the same turn"
        );

        // A later turn → still suppressed (the `ThisGame` window persists).
        state.turn.turn_number += 1;
        doomed_bear(&mut state);
        drain_sbas(&mut state);
        assert_eq!(
            noted_for(&state, watcher),
            1,
            "a once-per-game trigger never fires again, even in a later turn"
        );
    }

    // -------------------------------------------------------------------------
    // Performed — Draw / LoseLife / GainLife ([CR#121.1,119.3])
    // -------------------------------------------------------------------------

    /// `Performed(verb: "Draw", by: Ref(You))` matches `WillDraw` for the
    /// watcher's controller ([CR#121.1]) and not another player's draw.
    #[test]
    fn performed_matches_draw_matches_will_draw() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::Performed {
            verb: "Draw".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        let you_draw = GameEvent::WillDraw {
            player: PlayerId(0),
            source: None,
        };
        let opp_draw = GameEvent::WillDraw {
            player: PlayerId(1),
            source: None,
        };
        assert!(
            state.event_matches(&pattern, &you_draw, watcher_source),
            "your own draw matches by: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &opp_draw, watcher_source),
            "an opponent's draw fails by: Ref(You)"
        );
    }

    /// `Performed(verb: "LoseLife", by: Ref(You))` matches `LifeLost` for the
    /// watcher's controller ([CR#119.3]) and not another player's life loss.
    #[test]
    fn performed_matches_loselife_matches_life_lost() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::Performed {
            verb: "LoseLife".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        let you_lose = GameEvent::LifeLost {
            player: PlayerId(0),
            amount: 3,
        };
        let opp_lose = GameEvent::LifeLost {
            player: PlayerId(1),
            amount: 3,
        };
        assert!(
            state.event_matches(&pattern, &you_lose, watcher_source),
            "your own life loss matches by: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &opp_lose, watcher_source),
            "an opponent's life loss fails by: Ref(You)"
        );
    }

    /// `Performed(verb: "GainLife", by: Ref(You))` matches `LifeGained` for the
    /// watcher's controller ([CR#119.3]) and not another player's life gain.
    #[test]
    fn performed_matches_gainlife_matches_life_gained() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = Event::Performed {
            verb: "GainLife".into(),
            by: Filter::Ref(Reference::You),
            on: Filter::Any,
        };
        let you_gain = GameEvent::LifeGained {
            player: PlayerId(0),
            amount: 4,
        };
        let opp_gain = GameEvent::LifeGained {
            player: PlayerId(1),
            amount: 4,
        };
        assert!(
            state.event_matches(&pattern, &you_gain, watcher_source),
            "your own life gain matches by: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &opp_gain, watcher_source),
            "an opponent's life gain fails by: Ref(You)"
        );
    }
}
