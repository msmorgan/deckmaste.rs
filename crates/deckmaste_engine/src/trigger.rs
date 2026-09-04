//! Trigger event-matching ([CR#603.2,603.6]), the trigger *scan* (emit a
//! `TriggerFired` per match, whose apply notes a `NotedTrigger`), and the
//! `PlaceTriggers` barrier ([CR#603.3]) that puts noted triggers on the stack
//! in APNAP order with an `OrderTriggers` decision and a target choice at
//! placement.
//!
//! Matching is pure predicates (`event_matches`, `filter_matches_snapshot`);
//! `scan_triggers` and `place_triggers` are the scheduling/agenda-touching
//! functions.

use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::Count;
use deckmaste_core::EventFilter;
use deckmaste_core::Ident;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::StatePredicate;
use deckmaste_core::StaticSpec;
use deckmaste_core::TargetSpec;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::decide::DecisionPointKind;
use crate::event::Act;
use crate::event::Attacking;
use crate::event::BecameTarget;
use crate::event::Blocked;
use crate::event::ControlChanged;
use crate::event::CounterPlaced;
use crate::event::DamageDealt;
use crate::event::GameEvent;
use crate::event::LifeGained;
use crate::event::LifeLost;
use crate::event::ManaAbilityActivated;
use crate::event::ManaAdded;
use crate::event::ManaProduced;
use crate::event::Occurrence;
use crate::event::Tapped;
use crate::event::TappedForMana;
use crate::event::TriggerFired;
use crate::event::TurnBegan;
use crate::event::ZoneChange;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
use crate::stack::StackEntry;
use crate::stack::StackObject;
use crate::state::GameState;
use crate::step::Progress;

/// The acted-upon participant of an event ([CR#608.2k]) — the PATIENT, bound
/// distinctly from the agent so a two-object event names both. Kind-poly: a
/// damage recipient (or destroyed/countered thing) may be an object or a player
/// ([CR#120.3]). An object patient carries its LKI snapshot (regeneration,
/// wither read it like any moved object); a player patient carries the id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventPatient {
    /// A card/token recipient — read as `EventPatient` resolving to its object.
    Object(LkiSnapshot),
    /// A player recipient — read as `EventPatient` resolving to the proxy.
    Player(PlayerId),
}

/// The last-known information a fired trigger carries to its placement and
/// resolution ([CR#603.10a], [CR#608.2]): `~`/`This`/source, and the event's
/// provenance-explicit roles ([CR#603.2e,608.2k]) — the agent (the moved/acting
/// object), the actor (the responsible player), the kind-poly patient (the
/// acted-upon thing), and the combat defending player.
///
/// `Default` is all-`None` (a frameless body); construct with `..` for the
/// roles an event actually supplies.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TriggerBindings {
    /// The firing object's last-known self (`~`/`This`/source).
    pub this: Option<LkiSnapshot>,
    /// The event OBJECT — the moved object of a `ZoneMove`, the damage source.
    /// Read through the region's `EventObject` parameter.
    pub that_object: Option<LkiSnapshot>,
    /// The event ACTOR — the responsible player ("that player"). Read through
    /// the region's `EventActor` parameter.
    pub that_player: Option<PlayerId>,
    /// The event PATIENT — the acted-upon thing (damage recipient, …),
    /// kind-poly ([CR#120.3]). Read through the region's `EventPatient`
    /// parameter.
    pub that_patient: Option<EventPatient>,
    /// Types the causing mana-production fact actually added.
    pub produced_mana: Vec<deckmaste_core::ColorOrColorless>,
    /// The combat DEFENDING player ([CR#506.2,508.5]) — always a player. Read
    /// through the region's `DefendingPlayer` parameter.
    pub defending_player: Option<PlayerId>,
    /// The event MAGNITUDE — the amount of an amount-carrying event (damage
    /// dealt, life lost/gained), fixed at fire time; the set mirrors the
    /// event-role channel and enters the ability through its `EventAmount`
    /// region parameter ("whenever you gain life, … that much").
    pub event_amount: Option<Uint>,
    /// The firing counter event's `(before, after)` totals ([CR#714.2b]) —
    /// the channel `Condition::Crossed` reads, at the intervening-if gate
    /// and again at the resolution recheck ([CR#603.4]). `None` for facts
    /// that fix no totals.
    pub crossed: Option<(Uint, Uint)>,
    /// ADR law 7: the DECLARED captures of a delayed or reflexive body
    /// ([CR#603.7,603.12]), snapshotted when the creating effect created it
    /// ([CR#603.7a]) and keyed by the created region's own parameter. Empty
    /// for a printed trigger, which captures nothing — its whole context is
    /// its own event roles. This is the ONLY channel by which a value reaches
    /// a created body from the region that created it; a body's `this` is its
    /// own `Source` parameter ([CR#603.7d,603.7e]), not a capture.
    pub(crate) captures: Vec<(deckmaste_core::RefId, crate::activation::Value)>,
}

/// A trigger that has fired but is not yet on the stack ([CR#603.2]). Noted by
/// applying a `TriggerFired`; placed by the `PlaceTriggers` barrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotedTrigger {
    pub source: ObjectSource,
    pub ability: usize,
    /// `Some` for a delayed/reflexive trigger ([CR#603.7,603.12]) — its
    /// by-value body carried from creation (see [`CreatedTrigger`]); `None`
    /// for a printed trigger indexed by `ability`.
    pub created: Option<Arc<deckmaste_core::TriggeredAbility>>,
    pub controller: PlayerId,
    pub bindings: TriggerBindings,
}

/// A delayed or reflexive triggered ability ([CR#603.7,603.12]) created during
/// a spell's or ability's resolution. Because it is printed on no permanent,
/// the live `abilities_of_source` scan never sees it — the engine keeps its
/// body plus the context it fires in (source/controller per [CR#603.7d,603.7e],
/// and the [`TriggerBindings`] that anchor `~`/`This`) on `GameState`.
///
/// A DELAYED ability is stored in `GameState::delayed_triggers` and fires once,
/// the next time its `ability.event` occurs ([CR#603.7b]); the scan removes it
/// then. A REFLEXIVE ability is never stored — [CR#603.12] checks it against
/// events that already occurred earlier in the SAME resolution, immediately at
/// creation, so it fires (or not) on the spot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatedTrigger {
    /// The delayed/reflexive ability's source ([CR#603.7d,603.7e]).
    pub source: ObjectSource,
    /// The player who controlled the creating spell/ability as it resolved
    /// ([CR#603.7d,603.7e]).
    pub controller: PlayerId,
    /// The ability body — its `event` is the fire pattern, its `effect` runs
    /// on the stack when it fires.
    pub ability: Arc<deckmaste_core::TriggeredAbility>,
    /// The context captured at creation: `~`/`This` (the creating object's
    /// last-known self) and any event roles it should carry to resolution.
    pub bindings: TriggerBindings,
}

fn triggered_effect_region(ability: &deckmaste_core::TriggeredAbility) -> deckmaste_core::Region {
    let mut effect = ability.effect.clone();
    if effect.params.is_empty() {
        effect.params = deckmaste_core::triggered_region_params(ability.targets.len());
    }
    effect
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
    pub activation: crate::ActivationId,
    pub source: ObjectSource,
    pub ability: usize,
    /// `Some` for a delayed/reflexive trigger ([CR#603.7,603.12]) — carried
    /// through placement to the committed `StackEntry`.
    pub created: Option<Arc<deckmaste_core::TriggeredAbility>>,
    pub controller: PlayerId,
    pub bindings: TriggerBindings,
}

impl GameState {
    /// [CR#603.2,603.6]: does `pattern` match `event`, for an ability on
    /// `watcher`? The Trigger-lane entry into THE one evaluator
    /// ([`GameState::eval`], `engine-one-evaluator`): the event becomes its
    /// [`crate::eval::FactView`] fact record and evaluates frameless (a
    /// trigger scan holds no resolution frame — `Ref(This)`/`Ref(You)`
    /// anchor through `watcher`).
    pub(crate) fn event_matches(
        &self,
        pattern: &EventFilter,
        event: &GameEvent,
        watcher: ObjectSource,
    ) -> bool {
        self.event_matches_with_bindings(
            pattern,
            event,
            crate::eval::Lane::Trigger,
            crate::eval::Bindings::watcher(watcher),
        )
    }

    fn event_matches_with_frame(
        &self,
        pattern: &EventFilter,
        event: &GameEvent,
        watcher: ObjectSource,
        frame: &ExecutionFrame,
    ) -> bool {
        self.event_matches_with_bindings(
            pattern,
            event,
            crate::eval::Lane::Trigger,
            crate::eval::Bindings {
                watcher,
                frame: Some(frame),
                shape_only: false,
            },
        )
    }

    fn event_matches_with_bindings(
        &self,
        pattern: &EventFilter,
        event: &GameEvent,
        lane: crate::eval::Lane,
        bindings: crate::eval::Bindings<'_>,
    ) -> bool {
        // [CR#603.6]: triggers fire on committed FACTS. A pre-evolution
        // intent (`Act(Destroy)`/future-form `ZoneChange`) presents the same
        // fact-record kind as its downstream past-form fact — matching it
        // here would double-fire every zone-move trigger, so the trigger
        // lane refuses intents outright (the replacement lane is where
        // they match, [CR#614]).
        if crate::eval::shadowed_by_fact(event) {
            return false;
        }
        let Some(fact) = crate::eval::FactView::of(self, event) else {
            return false;
        };
        self.eval(pattern, &fact, lane, &bindings)
    }

    /// [CR#603.7c]: does a DELAYED/reflexive trigger's `pattern` match `event`?
    /// The fire-once lane ([`crate::eval::Lane::Delayed`]): same live-fact
    /// candidate semantics as [`event_matches`](Self::event_matches), but the
    /// pattern belongs to an ability the live scan never sees (it is printed on
    /// no permanent — see [`CreatedTrigger`]). `watcher` anchors
    /// `Ref(This)`/`Ref(You)` to the ability's captured source.
    pub(crate) fn event_matches_delayed(
        &self,
        pattern: &EventFilter,
        event: &GameEvent,
        watcher: ObjectSource,
    ) -> bool {
        self.event_matches_with_bindings(
            pattern,
            event,
            crate::eval::Lane::Delayed,
            crate::eval::Bindings::watcher(watcher),
        )
    }

    fn event_matches_delayed_with_frame(
        &self,
        pattern: &EventFilter,
        event: &GameEvent,
        watcher: ObjectSource,
        frame: &ExecutionFrame,
    ) -> bool {
        self.event_matches_with_bindings(
            pattern,
            event,
            crate::eval::Lane::Delayed,
            crate::eval::Bindings {
                watcher,
                frame: Some(frame),
                shape_only: false,
            },
        )
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
        filter: &Predicate,
        o: ObjectId,
        watcher: ObjectSource,
    ) -> bool {
        self.filter_matches_live_with_activation(filter, o, watcher, crate::ActivationId::NONE)
    }

    pub(crate) fn filter_matches_live_with_activation(
        &self,
        filter: &Predicate,
        o: ObjectId,
        watcher: ObjectSource,
        activation: crate::ActivationId,
    ) -> bool {
        crate::target::matches_with_activation(self, o, filter, Some(watcher), activation)
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
    #[allow(
        clippy::match_same_arms,
        reason = "distinct `=> false` seam arms with per-arm rationale that \
        diverge as later tasks land; #[allow] not #[expect] to avoid churn as \
        they split"
    )]
    #[cfg(test)]
    pub(crate) fn filter_matches_snapshot(
        &self,
        filter: &Predicate,
        snapshot: &LkiSnapshot,
        watcher: ObjectSource,
    ) -> bool {
        self.filter_matches_snapshot_with_activation(
            filter,
            snapshot,
            watcher,
            crate::ActivationId::NONE,
        )
    }

    #[allow(
        clippy::match_same_arms,
        reason = "separate false arms document distinct unsupported snapshot predicates"
    )]
    #[allow(
        clippy::too_many_lines,
        reason = "one arm per predicate leaf; splitting the match hides which leaves are answered"
    )]
    pub(crate) fn filter_matches_snapshot_with_activation(
        &self,
        filter: &Predicate,
        snapshot: &LkiSnapshot,
        watcher: ObjectSource,
        activation: crate::ActivationId,
    ) -> bool {
        // Combinators (`And`/`Or`/`Not`/`Expanded`/`Any`) recurse through
        // this same matcher via the shared walker; leaves fall through below.
        if let Some(result) = crate::target::walk_combinators(filter, |f| {
            self.filter_matches_snapshot_with_activation(f, snapshot, watcher, activation)
        }) {
            return result;
        }
        match filter {
            Predicate::Ref(Reference::Reg(register)) => {
                if self.activation_reference_is(
                    activation,
                    *register,
                    &deckmaste_core::Provenance::Source,
                ) {
                    return snapshot.source == watcher;
                }
                self.activation_product(activation, *register).map_or_else(
                    || {
                        activation == crate::ActivationId::NONE
                            && match register.0 {
                                0 => snapshot.source == watcher,
                                1 => self
                                    .controller_of_source(watcher)
                                    .is_some_and(|controller| snapshot.controller == controller),
                                _ => false,
                            }
                    },
                    |product| {
                        product.current == Some(snapshot.object)
                            || product
                                .lki
                                .as_ref()
                                .is_some_and(|bound| bound.object == snapshot.object)
                    },
                )
            }
            // "a creature" — check the snapshot's printed card types.
            Predicate::Characteristic(CharacteristicPredicate::Type(ty)) => {
                snapshot_has_type(self, snapshot, ty.name())
            }

            // "on the battlefield" (the `Permanent` macro and friends): the
            // snapshot is the object as it last existed in the zone it left
            // ([CR#603.10a]), so it matches the zone the event removed it from.
            Predicate::State(StatePredicate::InZone(zone)) => snapshot.left == *zone,

            // Characteristics read the PRINTED face (no layer view over a gone
            // object; LKI-derived characteristics are not captured — see the
            // snapshot-stat seam note). A player-proxy snapshot has no face.
            Predicate::Characteristic(CharacteristicPredicate::Named(n)) => {
                snapshot_face(self, snapshot).is_some_and(|f| &*f.name == n.as_str())
            }
            Predicate::Characteristic(CharacteristicPredicate::NamedReg(reference)) => self
                .activation_symbol(activation, *reference)
                .is_some_and(|expected| {
                    snapshot_face(self, snapshot).is_some_and(|f| f.name.as_ref() == expected)
                }),
            Predicate::Characteristic(CharacteristicPredicate::Supertype(s)) => {
                snapshot_face(self, snapshot).is_some_and(|f| f.supertypes.contains(s))
            }
            Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => {
                snapshot_face(self, snapshot)
                    .is_some_and(|f| crate::layer::base_colors(f).contains(c))
            }
            Predicate::Characteristic(CharacteristicPredicate::Multicolored) => {
                snapshot_face(self, snapshot)
                    .is_some_and(|f| crate::layer::base_colors(f).len() >= 2)
            }
            Predicate::Characteristic(CharacteristicPredicate::Colorless) => {
                snapshot_face(self, snapshot)
                    .is_some_and(|f| crate::layer::base_colors(f).is_empty())
            }
            // [CR#208,202.3]: the PRINTED stat. (LKI-derived P/T — a pumped
            // creature that died "with power 3" — is a capture seam: the
            // snapshot stores no P/T, so this reads the printed face for now.)
            Predicate::Characteristic(CharacteristicPredicate::Stat(stat, cmp, count)) => {
                crate::target::stat_satisfies(snapshot_stat(self, snapshot, *stat), *cmp, count)
            }

            // [CR#122.1]: counters are captured on the snapshot, so "had a
            // +1/+1 counter on it" reads the DYING object's last-known counters
            // ([CR#603.10a]) — the intervening-if of Undying/Persist
            // ([CR#702.93a,702.79a]). Mirrors the live `HasCounter` arm
            // (`target::matches_with`): present and positive.
            Predicate::State(StatePredicate::HasCounter(kind)) => {
                snapshot.counters.get(kind.as_str()).is_some_and(|&n| n > 0)
            }

            // [CR#110.5]: tap state is captured on the snapshot; flip/face/
            // phasing are not — a filter over an unevaluated status must trip
            // rather than silently read a default.
            Predicate::State(StatePredicate::Status(status)) => {
                use deckmaste_core::Status;
                match status {
                    Status::Tapped => snapshot.tapped,
                    Status::Untapped => !snapshot.tapped,
                    Status::PhasedOut | Status::PhasedIn => todo!(
                        "engine seam: snapshot status {status:?} ([CR#702.26a]) — phasing state \
                         uncaptured; owner: engine-phasing"
                    ),
                    Status::FaceDown | Status::FaceUp => todo!(
                        "engine seam: snapshot status {status:?} ([CR#708.1]) — face-down state \
                         uncaptured; owner: engine-face-down"
                    ),
                    Status::Flipped | Status::Unflipped => todo!(
                        "engine seam: snapshot status {status:?} ([CR#710.3]) — flipped state \
                         uncaptured; owner: engine-flipped-status"
                    ),
                }
            }

            // The captured controller / the card owner, resolved to their LIVE
            // player proxies ([CR#108.3,109.5]); the inner filter runs live.
            Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(f)) => self
                .filter_matches_live_with_activation(
                    f,
                    self.player(snapshot.controller).object,
                    watcher,
                    activation,
                ),
            Predicate::Relation(deckmaste_core::RelationPredicate::Owner(f)) => {
                match snapshot.source {
                    ObjectSource::Card(c) => {
                        let owner = self.cards.get(c).owner;
                        self.filter_matches_live_with_activation(
                            f,
                            self.player(owner).object,
                            watcher,
                            activation,
                        )
                    }
                    ObjectSource::Player(_) => false,
                }
            }
            // A snapshot subject is a card, never a player proxy — the
            // player-side relations never match.
            Predicate::Relation(
                deckmaste_core::RelationPredicate::OpponentOf(_)
                | deckmaste_core::RelationPredicate::Controls(_),
            ) => false,
            // [CR#301.5,303.4]: the departed attachment's captured host
            // ([CR#603.10a]) — mirrors `target::matches_with`'s `AttachedTo`
            // arm, but reads the CAPTURED relation (the live object is
            // already gone) and resolves the host LIVE. The shape a departed
            // Aura's leaves-the-battlefield ability needs to read its last
            // host. Unattached reads false, never a panic.
            Predicate::Relation(deckmaste_core::RelationPredicate::AttachedTo(inner)) => {
                snapshot.attached_to.is_some_and(|host| {
                    self.filter_matches_live_with_activation(inner, host, watcher, activation)
                })
            }
            // The inverse: some LIVE object still points its `attached_to` at
            // this object's now-stale id — captured before the attach
            // relation is cleared, since LKI is taken before the
            // simultaneous state-based sweep ([CR#704.8]) — and matches
            // `inner`. `snapshot.object` is a label/reference token here,
            // never a live lookup: slotmap's generational keys make the
            // comparison self-checking, so a reused id can never
            // false-match.
            Predicate::Relation(deckmaste_core::RelationPredicate::Attachment(inner)) => {
                self.objects.iter().any(|o| {
                    o.attached_to == Some(snapshot.object)
                        && self
                            .filter_matches_live_with_activation(inner, o.id, watcher, activation)
                })
            }

            // A gone object has no live stack entry, so it currently targets
            // nothing ([CR#115.9b] — departed objects are not read through LKI).
            Predicate::State(StatePredicate::Targets(_) | StatePredicate::TargetCount(_)) => false,
            // [CR#607]: no linked-ability relation registry yet.
            Predicate::State(StatePredicate::RelatedBy(..)) => {
                todo!(
                    "engine seam: snapshot RelatedBy ([CR#607.1]) — no linked-ability relation \
                     registry; owner: engine-filter-breadth"
                )
            }
            // A gone object has left its ordered zone — [`Predicate::Adjacent`]
            // reads the LIVE `zones.graveyards`/`zones.libraries` order
            // ([`crate::target::ordered_zone_position`]), which no longer
            // contains it; never a panic, just no match.
            Predicate::Adjacent(..) => false,
            // See `target::matches_with`'s identical arm: the past-form
            // `ZoneChange` history keys on the pre-move stale id, with no persistent
            // link back to a live object — genuinely unbuilt, not a
            // convenient-wrong default.
            Predicate::State(StatePredicate::WasPutFrom(_)) => false,
            // A snapshot holds a damage TOTAL, not deal-time marks ([CR#120.3]).
            Predicate::State(StatePredicate::WasDealtDamageBy(_)) => false,
            // [CR#302.6]: a gone/moved object has no summoning-sickness state
            // (snapshot doesn't capture it) — sound never-crash default.
            Predicate::State(StatePredicate::SummoningSick) => false,

            // [CR#603.10a]: the candidate-relative condition bridge — the
            // GONE candidate binds as `It` (its snapshot), `This`/`You`
            // anchor to the watcher's LIVE carrier, and the condition
            // evaluates as the read occurs (the Where-in-snapshot lift,
            // engine-one-evaluator; mirrors the live `Predicate::Where` arm of
            // `target::matches_with`, including its gone-carrier-no-match
            // discipline).
            Predicate::Where(cond) => match self
                .objects
                .iter()
                .find(|ob| ob.source == watcher)
                .map(|ob| (ob.id, ob.controller))
            {
                None => false,
                Some((carrier, controller)) => {
                    let mut frame = self.frame(carrier, controller);
                    if activation != crate::ActivationId::NONE {
                        frame.activation = activation;
                    }
                    frame.activation = self.enter_candidate_region_snapshot(cond, &frame, snapshot);
                    self.condition_holds(&cond.body, &frame)
                }
            },

            // Combinators (`And`/`Or`/`Not`/`Any`) are handled by
            // `walk_combinators` before this match.
            //
            // Provenance is erased at `lower` (`deckmaste_lowering`), so no
            // loaded value reaches here wrapped. The arm survives only because
            // the variant does; `core-demacro` deletes both. Named explicitly
            // so the seam below reports only genuinely unbuilt shapes.
            other => todo!(
                "engine seam: stage 3 does not evaluate snapshot filter {other:?} — the LKI \
                 matcher covers only part of the live matcher's leaves; \
                 owner: engine-snapshot-predicate-breadth"
            ),
        }
    }

    /// [CR#603.2,603.6]: after an occurrence applies, scan watching abilities
    /// for ones whose `event` pattern matches each occurred fact, and schedule
    /// a `TriggerFired` per match at the agenda front (so they apply in the
    /// occurrence's wake — [CR#603.3b]). Applying a `TriggerFired` is what
    /// *notes* the trigger; this only emits.
    ///
    /// Watchers ([CR#603.6]) are every live battlefield permanent, plus — for a
    /// past-form `ZoneChange` that LEFT the battlefield — the leaving object
    /// itself (via its snapshot), so its own dies-trigger is considered
    /// even though its abilities are gone from the battlefield
    /// ([CR#603.6c]). An *entering* object is already a live battlefield
    /// permanent, so it is not re-added.
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
        // [CR#603.2c]: a `OneOrMore` pattern matches a batch occurrence ONCE
        // — (watcher source, ability) pairs that already fired for a member
        // of THIS occurrence are skipped for its later members.
        let mut batch_fired: std::collections::HashSet<(ObjectSource, usize)> =
            std::collections::HashSet::new();
        // [CR#603.7b]: the indices of `delayed_triggers` that fired this
        // occurrence — a delayed trigger fires ONCE (the next time its event
        // occurs), then is removed. Collected across the occurrence's facts so
        // one delayed trigger never fires twice within a batch, and drained
        // after the scan (a `&self` `scan_event` cannot mutate the registry).
        let mut fired_delayed: Vec<usize> = Vec::new();
        for event in events {
            // Skip facts no trigger pattern watches; never scan a
            // `TriggerFired` (avoids any chance of recursion). The
            // future-form `ZoneChange` (`snapshot: None`) is
            // skipped because trigger-matching happens on
            // the downstream past-form fact (already queued by the will-change
            // apply at the agenda front — [CR#603.6]); matching on the intent
            // would double-fire every zone-move trigger.
            // `StepBegan` is NOT skipped — `StepBegins` step/phase triggers
            // ([CR#603.2]) key off it (e.g. "at the beginning of combat on your
            // turn"); `TurnBegan` has no pattern shape that watches it.
            match event {
                GameEvent::TriggerFired(TriggerFired { .. })
                | GameEvent::AbilityResolved(_)
                | GameEvent::TurnBegan(TurnBegan { .. })
                | GameEvent::ZoneChange(ZoneChange { snapshot: None, .. })
                // The FUTURE keyword-action window is not trigger-scanned; its
                // committed PAST form (`FinalizeAct`) carries the
                // "whenever you scry/discard/…" trigger fact ([CR#701.22d]).
                | GameEvent::Act(Act { committed: false, .. }) => continue,
                GameEvent::Blocked(Blocked { attacker, .. }) if !blocked_attackers.insert(*attacker) => {
                    continue;
                }
                _ => {}
            }
            self.scan_event(event, &mut emits, &mut batch_fired);
            self.scan_delayed(event, &mut emits, &mut fired_delayed);
        }
        // [CR#603.7b]: retire the delayed triggers that fired (highest index
        // first, so earlier removals don't shift the ones still to remove).
        fired_delayed.sort_unstable();
        fired_delayed.dedup();
        for idx in fired_delayed.into_iter().rev() {
            self.delayed_triggers.remove(idx);
        }
        if emits.is_empty() {
            return;
        }
        let (triggered_mana, ordinary): (Vec<_>, Vec<_>) = emits
            .into_iter()
            .partition(|item| matches!(item, WorkItem::ResolveTriggeredMana { .. }));
        if !ordinary.is_empty() {
            self.schedule_front(ordinary);
        }
        if triggered_mana.is_empty() {
            return;
        }
        let deferred = self
            .resolving_mana_actions
            .last()
            .copied()
            .and_then(|action| {
                self.agenda.iter().position(|item| {
                    matches!(item, WorkItem::FinishManaAction(id) if *id == action)
                        || matches!(item, WorkItem::FinishTriggeredMana { action: id, .. } if *id == action)
                        || matches!(item, WorkItem::CompleteTriggeredMana(id) if *id == action)
                })
            });
        if let Some(marker) = deferred {
            let mut insert_at = marker + 1;
            while matches!(
                self.agenda.get(insert_at),
                Some(WorkItem::ResolveTriggeredMana { .. })
            ) {
                insert_at += 1;
            }
            for item in triggered_mana {
                self.agenda.insert(insert_at, item);
                insert_at += 1;
            }
        } else {
            // A mana-added trigger with no active causing mana ability still
            // resolves in the addition's immediate wake, before the containing
            // resolution advances.
            self.schedule_front(triggered_mana);
        }
    }

    /// [CR#603.7]: scan the delayed-trigger registry against one occurred
    /// `event`, pushing a `TriggerFired` (carrying the delayed ability by
    /// value) per newly-matching entry onto `emits` and recording its index in
    /// `fired` so the caller retires it. A registry entry already in `fired`
    /// (matched an earlier fact of THIS occurrence) is skipped — a delayed
    /// trigger fires once ([CR#603.7b]).
    ///
    /// Unlike a printed trigger there is no live watcher: the ability watches
    /// its `event` regardless of where (or whether) its source still is — the
    /// "no longer in the expected zone" check is [CR#603.7c]'s RESOLUTION-time
    /// concern, not a fire gate. The event's provenance roles (the moved
    /// object, magnitude, …) are merged onto the captured bindings so the fired
    /// body can read them ("… deals that much", "when ~ leaves, exile it").
    fn scan_delayed(&self, event: &GameEvent, emits: &mut Vec<WorkItem>, fired: &mut Vec<usize>) {
        let roles = self.event_roles(event);
        for (idx, ct) in self.delayed_triggers.iter().enumerate() {
            if fired.contains(&idx) {
                continue;
            }
            let bindings = roles.bindings_over(ct.bindings.clone());
            let frame = self.created_gate_frame(ct, &bindings);
            // [CR#603.4]: an intervening-if is checked as the event occurs; it
            // is rechecked at resolution (the shared `Triggered` resolve arm).
            let matches =
                self.event_matches_delayed_with_frame(&ct.ability.event, event, ct.source, &frame)
                    && ct
                        .ability
                        .condition
                        .as_ref()
                        .is_none_or(|condition| self.condition_holds(condition, &frame));
            self.remove_activation_family(frame.activation);
            if !matches {
                continue;
            }
            emits.push(WorkItem::Emit(Occurrence::single(GameEvent::TriggerFired(
                TriggerFired {
                    source: ct.source,
                    ability: 0,
                    controller: ct.controller,
                    created: Some(ct.ability.clone()),
                    bindings: Box::new(bindings),
                },
            ))));
            fired.push(idx);
        }
    }

    /// The intervening-if gate frame ([CR#603.4]) for a created (delayed/
    /// reflexive) trigger: `~`/`This` from its captured snapshot, the
    /// controller from [CR#603.7d,603.7e], and the firing event's endophoric
    /// roles (no targets chosen at the gate). Mirrors the printed-trigger
    /// gate in `scan_event`.
    fn created_gate_frame(
        &self,
        ct: &CreatedTrigger,
        bindings: &TriggerBindings,
    ) -> ExecutionFrame {
        let source = bindings
            .this
            .as_ref()
            .map_or_else(|| self.player(ct.controller).object, |s| s.object);
        let mut frame = self.frame(source, ct.controller);
        self.frame_set_source_lki(&mut frame, bindings.this.clone());
        self.frame_set_defending_player(&mut frame, bindings.defending_player);
        self.frame_set_event_bindings(
            &mut frame,
            bindings.that_object.clone(),
            bindings.that_player,
            bindings.that_patient.clone(),
        );
        self.frame_set_event_extras(
            &mut frame,
            bindings.event_amount,
            bindings.produced_mana.clone(),
            bindings.crossed,
        );
        let region = triggered_effect_region(&ct.ability);
        frame.activation = self.enter_created_region(&region, &frame, &bindings.captures);
        frame
    }

    /// The firing event's provenance roles ([CR#603.2e,608.2k,120.3,714.2b]) —
    /// the AGENT (the acting/moved object) as `that_object` with its
    /// responsible-player ACTOR as `that_player`, the kind-poly PATIENT as
    /// `that_patient`, the combat DEFENDING player, the amount-carrying
    /// MAGNITUDE (`EventAmount`), and a counter event's before/after totals
    /// (`crossed`). Derived per `GameEvent` kind once; shared by the printed
    /// (`scan_event`) and delayed (`scan_delayed`) scans so both fire bodies
    /// read the same `EventObject`/`EventActor`/`EventPatient`/`EventAmount`/…
    /// roles. Card-backed roles carry an LKI snapshot ([CR#603.10a]); a
    /// player-proxy id is zoneless and never snapshotted ([CR#120.3]); a
    /// stale/missing id yields no role.
    pub(crate) fn event_roles(&self, event: &GameEvent) -> EventRoles {
        let (that_object, that_player, that_patient) = match event {
            // The past-form zone-change FACT carries the moved object's snapshot.
            GameEvent::ZoneChange(ZoneChange {
                snapshot: Some(snapshot),
                ..
            }) => (Some(snapshot.as_ref().clone()), None, None),
            // [CR#603.2e] becomes-state transitions: the transitioning object
            // is the agent ("it") with its controller the actor — Exalted
            // ([CR#702.83a]) reads the lone attacker via `EventObject`.
            GameEvent::Attacking(Attacking { attacker: o, .. })
            | GameEvent::Untapped(o, _)
            | GameEvent::Tapped(Tapped { object: o, .. }) => {
                let (agent, actor) = self.event_agent(*o);
                (agent, actor, None)
            }
            // [CR#601.2c] becomes-target / [CR#120.3] damage: the source/agent
            // binds as `EventObject`, the recipient (object or player) as the
            // kind-poly patient. Ward's `Counter(EventObject)` ([CR#702.21a])
            // counters "it" (the source on the stack), not the warded permanent.
            GameEvent::BecameTarget(BecameTarget { target, source })
            | GameEvent::DamageDealt(DamageDealt { source, target, .. }) => {
                let (agent, actor) = self.event_agent(*source);
                (agent, actor, self.event_patient(*target))
            }
            // [CR#109.5] control change: the moved object is the agent, the new
            // controller the responsible actor.
            GameEvent::ControlChanged(ControlChanged { object, to }) => {
                let (agent, _) = self.event_agent(*object);
                (agent, Some(*to), None)
            }
            GameEvent::ManaAbilityActivated(ManaAbilityActivated {
                source, controller, ..
            })
            | GameEvent::ManaProduced(ManaProduced {
                source, controller, ..
            })
            | GameEvent::TappedForMana(TappedForMana {
                source, controller, ..
            }) => (Some(source.clone()), Some(*controller), None),
            GameEvent::ManaAdded(ManaAdded {
                player, provenance, ..
            }) => {
                let (agent, _) = provenance
                    .source
                    .map_or((None, None), |source| self.event_agent(source));
                (agent, Some(*player), None)
            }
            // [CR#601.2i]: once a spell becomes cast, its controller is the
            // responsible player carried by "that player" in the triggered
            // ability, and the spell itself is the event object.
            GameEvent::SpellCast(object) => {
                let (agent, actor) = self.event_agent(*object);
                (agent, actor, None)
            }
            // [CR#701]: a committed keyword-action fact — its object SUBJECT
            // (a per-subject fact carries at most one) binds as the agent
            // ("it": Foe-Razer's counters land on THAT fighting creature),
            // the performer (or the subject's controller, the event_agent
            // precedent above) as the actor. Uniform for every verb: scry/
            // draw newly bind their performer, a destroy fact's usually-gone
            // subject yields no agent (event_agent's stale-id rule).
            GameEvent::Act(Act { on, who, .. }) => {
                let (agent, actor) = match on.first() {
                    Some(&subject) => self.event_agent(subject),
                    None => (None, None),
                };
                (agent, (*who).or(actor), None)
            }
            _ => (None, None, None),
        };
        // The combat DEFENDING player of an attack-declaration fact
        // ([CR#506.2,508.5]) — in a 2-player game the sole opponent of the
        // attacker's controller.
        let defending_player = match event {
            // The defending player controls the thing attacked ([CR#508.1b]):
            // a planeswalker's controller, or the attacked player's own proxy.
            GameEvent::Attacking(Attacking { defending, .. }) => {
                self.objects.get(*defending).map(|o| o.controller)
            }
            _ => None,
        };
        // The event MAGNITUDE — the amount-carrying set the apply funnel fixes
        // into the `EventAmount` parameter ("whenever you gain life, … that
        // much").
        let event_amount = match event {
            GameEvent::DamageDealt(DamageDealt { amount, .. })
            | GameEvent::LifeLost(LifeLost { amount, .. })
            | GameEvent::LifeGained(LifeGained { amount, .. })
            | GameEvent::ManaAdded(ManaAdded { amount, .. }) => Some(*amount),
            GameEvent::ManaProduced(ManaProduced { produced, .. })
            | GameEvent::TappedForMana(TappedForMana { produced, .. }) => {
                Some(Uint::try_from(produced.len()).expect("produced mana count fits in Uint"))
            }
            // [CR#616.1g,121.2a]: a `Batch(n, keyword-action)` aggregate window
            // carries its own cardinality, and that IS the magnitude "that
            // many" names — a count-multiplying replacement (Bruvac's "mills
            // twice that many cards instead") reads it off the replaced
            // AGGREGATE intent, before any contained per-entity future exists.
            // An ordinary (non-aggregate) `Act` carries `None`, like every
            // other amount-less event.
            GameEvent::Act(crate::event::Act { batch, .. }) => *batch,
            _ => None,
        };
        // The counter event's before/after totals ([CR#714.2b]) — the
        // `Condition::Crossed` channel.
        let crossed = match event {
            GameEvent::CounterPlaced(CounterPlaced { before, after, .. }) => {
                Some((*before, *after))
            }
            _ => None,
        };
        let produced_mana = match event {
            GameEvent::ManaProduced(ManaProduced { produced, .. })
            | GameEvent::TappedForMana(TappedForMana { produced, .. }) => {
                produced.iter().map(|unit| unit.kind).collect()
            }
            GameEvent::ManaAdded(ManaAdded { mana, amount, .. }) => {
                vec![*mana; usize::try_from(*amount).expect("mana amount fits in usize")]
            }
            _ => Vec::new(),
        };
        EventRoles {
            that_object,
            that_player,
            that_patient,
            defending_player,
            event_amount,
            crossed,
            produced_mana,
        }
    }

    /// The AGENT roles of an object that ACTED in an event
    /// ([CR#603.2e,608.2k]): its LKI snapshot (`that_object`) and the
    /// responsible-player ACTOR (`that_player`, its controller). An agent
    /// is an object, so a player-proxy id (zoneless — [CR#120.3]) or a
    /// stale/missing id yields no snapshot.
    fn event_agent(&self, id: ObjectId) -> (Option<LkiSnapshot>, Option<PlayerId>) {
        match self.objects.get(id) {
            Some(o) if matches!(o.source, ObjectSource::Card(_)) => {
                (Some(LkiSnapshot::capture(self, id)), Some(o.controller))
            }
            _ => (None, None),
        }
    }

    /// The kind-poly PATIENT of an event ([CR#120.3]): a card/token recipient
    /// carries its LKI snapshot, a player-proxy recipient carries its id
    /// (zoneless — never snapshotted). A stale/missing id yields no patient.
    /// Mirrors `resolve::bind_element` / `replace_registry::schedule_body`.
    fn event_patient(&self, id: ObjectId) -> Option<EventPatient> {
        match self.objects.get(id)?.source {
            ObjectSource::Player(p) => Some(EventPatient::Player(p)),
            ObjectSource::Card(_) => Some(EventPatient::Object(LkiSnapshot::capture(self, id))),
        }
    }

    /// Scan one occurred fact against every watcher, pushing a `TriggerFired`
    /// emit per match onto `emits`. `batch_fired` is the occurrence-scoped
    /// dedup set for `OneOrMore` patterns ([CR#603.2c] — a batch quantifier
    /// matches the occurrence once, not per member).
    fn scan_event(
        &self,
        event: &GameEvent,
        emits: &mut Vec<WorkItem>,
        batch_fired: &mut std::collections::HashSet<(ObjectSource, usize)>,
    ) {
        // The firing event's provenance roles ([CR#603.2e,608.2k,120.3]) —
        // agent/actor/patient, combat defender, magnitude, counter totals —
        // derived once per fact and shared with the delayed-trigger scan.
        let roles = self.event_roles(event);

        // The watcher set ([CR#603.6,113.6b]): every live battlefield
        // permanent, plus every object in a graveyard or hand (for
        // graveyard/hand- FUNCTIONING triggers — Madness, Bridge from
        // Below; a per-ability `from`-zone gate below keeps a
        // battlefield-default ability on such an object from firing),
        // plus the leaving object's snapshot for a battlefield-leave.
        let mut watchers: Vec<Watcher> = self
            .zones
            .battlefield
            .iter()
            .map(|&id| Watcher::Live(id))
            .collect();
        for zone in self.zones.graveyards.iter().chain(self.zones.hands.iter()) {
            watchers.extend(zone.iter().map(|&id| Watcher::Live(id)));
        }
        // [CR#114.4]: an emblem's abilities function from the command zone —
        // add the command-zone emblems as live watchers (the per-ability
        // from-zone gate below treats Command as their default function zone).
        watchers.extend(
            self.zones
                .command
                .iter()
                .filter(|&&id| {
                    self.objects
                        .obj(id)
                        .card_id()
                        .is_some_and(|c| self.cards.get(c).is_emblem)
                })
                .map(|&id| Watcher::Live(id)),
        );
        if let GameEvent::ZoneChange(ZoneChange {
            snapshot: Some(snapshot),
            from: Some(Zone::Battlefield),
            ..
        }) = event
        {
            // The leaving object — its abilities are no longer on the
            // battlefield, so add it explicitly ([CR#603.6c]).
            watchers.push(Watcher::Leaving(snapshot.as_ref().clone()));
        }

        for watcher in watchers {
            // [CR#113.6,113.6b]: the zone this watcher currently functions in —
            // its live zone, or the battlefield for a just-left object (its
            // dies-trigger functions from the battlefield it left).
            let watcher_zone = match &watcher {
                Watcher::Live(id) => self.objects.obj(*id).zone,
                Watcher::Leaving(_) => Some(Zone::Battlefield),
            };
            // The live object carrying this watcher, if any — needed to fold in
            // its predicate-scoped CONFERRED abilities below (a leaving/gone
            // object has none). Captured before `watcher` is consumed.
            let live_id = match &watcher {
                Watcher::Live(id) => Some(*id),
                Watcher::Leaving(_) => None,
            };
            let (source, controller, this) = match watcher {
                Watcher::Live(id) => {
                    let o = self.objects.obj(id);
                    (o.source, o.controller, LkiSnapshot::capture(self, id))
                }
                Watcher::Leaving(s) => (s.source, s.controller, s.clone()),
            };
            // [CR#113.6b,114.4]: the DEFAULT function zone of this watcher's
            // abilities — the battlefield for an ordinary object, but the
            // command zone for an emblem (whose abilities have no `from` yet
            // still function from there).
            let default_zone = match source {
                ObjectSource::Card(c) if self.cards.get(c).is_emblem => Zone::Command,
                _ => Zone::Battlefield,
            };
            // DERIVED enumeration: the printed spine (index-stable,
            // `printed_len` long) followed by this object's
            // CONFERRED triggered abilities ([CR#603.2]), so a
            // Falkenrath-Gorger-shape conferral's trigger
            // participates in the scan where the printed-only spine would miss
            // it.
            let (abilities, printed_len, ability_captures) =
                crate::derive::derived_abilities_of(self, live_id, source);
            // Whether this watcher is a battlefield permanent showing its BACK
            // face — exactly when `abilities_of_source` sources its section-1
            // printed abilities from the back face ([CR#712.8e]), so those
            // triggers must be captured by value (see `created` below).
            let showing_back = live_id.is_some_and(|id| {
                let o = self.objects.obj(id);
                o.side == crate::object::Side::Back && o.zone == Some(Zone::Battlefield)
            });
            for (idx, ability) in abilities.iter().enumerate() {
                let Some(t) = ability.as_triggered() else {
                    continue;
                };
                // [CR#113.6,113.6b]: the ability triggers only while its source
                // is in its function-zone (`from`, default battlefield —
                // command for an emblem). A graveyard/hand
                // object considers only its graveyard/hand
                // triggers; a battlefield permanent only its
                // battlefield ones; an emblem only its command-zone ones.
                if watcher_zone != Some(t.from.unwrap_or(default_zone)) {
                    continue;
                }
                // [CR#603.10a,608.2]: the bindings the fired trigger carries —
                // also the context for the event pattern and intervening-if
                // gate ([CR#603.4]). Both are inside the trigger's declared
                // region, so provenance parameters resolve by register.
                let bindings = roles.bindings_over(TriggerBindings {
                    this: Some(this.clone()),
                    captures: ability_captures[idx].clone(),
                    ..TriggerBindings::default()
                });
                let region = triggered_effect_region(t);
                let frame = self.trigger_gate_frame(&region, controller, &bindings);
                let matches = self.event_matches_with_frame(&t.event, event, source, &frame)
                    && t.condition
                        .as_ref()
                        .is_none_or(|condition| self.condition_holds(condition, &frame));
                self.remove_activation_family(frame.activation);
                if !matches {
                    continue;
                }
                // [CR#603.2c]: a `OneOrMore` pattern matches the batch ONCE —
                // skip a (source, ability) that already fired for an earlier
                // member of this occurrence. (The intervening-if below can't
                // split members: it reads the same post-occurrence state for
                // all of them.)
                if contains_one_or_more(&t.event) && !batch_fired.insert((source, idx)) {
                    continue;
                }
                // [CR#603.2d]: trigger multipliers (Panharmonicon, Yarok,
                // Doubling Season's trigger half). The same fired trigger is
                // emitted `1 + extra` times — NOT a copy, so each emit is an
                // independent `TriggerFired` that places its own stack instance
                // ([CR#603.3]) and chooses its own modes/targets. Multipliers
                // ADD.
                let extra = self.trigger_multiplier_extra(event, this.object);
                if ability.is_triggered_mana_ability() {
                    for _ in 0..=extra {
                        emits.push(WorkItem::ResolveTriggeredMana {
                            source,
                            ability: idx,
                            triggered: Arc::new(t.clone()),
                            controller,
                            bindings: bindings.clone(),
                        });
                    }
                    continue;
                }
                // A trigger carries its body BY VALUE — the channel
                // delayed/reflexive triggers use ([CR#603.7,603.12]) — when the
                // by-index `(source, idx)` re-read at resolution can't recover
                // it:
                //
                //  * a CONFERRED trigger (idx past the printed spine) has no
                //    index-stable identity, and
                //  * a back-up permanent's section-1 PRINTED trigger is sourced
                //    from its BACK face ([CR#712.8e]); if the permanent leaves
                //    the battlefield between firing and resolution ([CR#603.3]
                //    — a trigger resolves independently of its source),
                //    `abilities_of_source` finds no live battlefield object and
                //    falls back to the FRONT face (a different, possibly
                //    shorter list) — an out-of-bounds / wrong-body read.
                //    Capturing the back body now sidesteps it.
                //
                // A front-up printed trigger keeps `created: None` and resolves
                // by index, unchanged.
                let created =
                    (idx >= printed_len || showing_back).then(|| std::sync::Arc::new(t.clone()));
                let fired = GameEvent::TriggerFired(TriggerFired {
                    source,
                    ability: Uint::try_from(idx).expect("ability index fits in Uint"),
                    controller,
                    created,
                    bindings: Box::new(bindings),
                });
                for _ in 0..=extra {
                    emits.push(WorkItem::Emit(Occurrence::single(fired.clone())));
                }
            }
        }
    }

    /// The number of ADDITIONAL times a trigger fires under
    /// [`StaticSpec::TriggerMultiplier`] statics ([CR#603.2d]):
    /// Panharmonicon, Yarok, Doubling Season's trigger half. A multiplier
    /// applies when its `cause` matches the triggering `event` AND the fired
    /// trigger's source permanent (`trig_object`) matches the multiplier's
    /// `affected` filter (default "you control", anchored on the multiplier's
    /// controller). Multipliers ADD — two Panharmonicons give +2 (a 3× total),
    /// never compound.
    ///
    /// Effect sources are read from PRINTED abilities (cycle-safe, like the
    /// object-layer and player-attribute scans). Seam: a fired trigger whose
    /// source is leaving the battlefield carries a stale `trig_object`, so a
    /// `Predicate`-based `affected` can't re-find it and contributes 0 — the
    /// canonical doublers act on ENTER triggers, whose source stays live.
    fn trigger_multiplier_extra(&self, event: &GameEvent, trig_object: ObjectId) -> Uint {
        let mut extra: Uint = 0;
        for &mid in &self.zones.battlefield {
            let multiplier_source = self.objects.obj(mid).source;
            for ability in crate::derive::abilities_of_source(self, multiplier_source) {
                let Ability::Static(effect) = &ability else {
                    continue;
                };
                if let StaticSpec::TriggerMultiplier {
                    cause,
                    extra: count,
                    affected,
                } = &effect.body
                {
                    // The fact that fired the trigger must match the cause, and
                    // the trigger's source permanent must match `affected`
                    // (with `Ref(You)` anchored on the
                    // multiplier's controller).
                    if self.event_matches(cause, event, multiplier_source)
                        && self.filter_matches_live(affected, trig_object, multiplier_source)
                    {
                        extra = extra.saturating_add(literal_count(count));
                    }
                }
            }
        }
        extra
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
            // Triggers exist but none belong to a live APNAP player — drop
            // them.
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
            self.pending = Some(DecisionPointKind::OrderTriggers(
                crate::decide::pending::OrderTriggers {
                    player,
                    triggers: mine,
                },
            ));
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
        let specs = self.trigger_targets(noted.source, noted.ability, noted.created.as_deref());
        if specs.is_empty() {
            // No targets — push directly with a freshly minted stack id.
            let id = self
                .objects
                .mint(noted.source, noted.controller, Some(Zone::Stack));
            let activation = self.enter_trigger_activation(
                noted.source,
                noted.ability,
                noted.created.as_deref(),
                noted.controller,
                &noted.bindings,
            );
            self.stack.push(StackEntry {
                paid_costs: Vec::new(),
                id,
                activation,
                object: StackObject::Triggered {
                    source: noted.source,
                    ability: noted.ability,
                    created: noted.created,
                    bindings: noted.bindings,
                },
                controller: noted.controller,
                targets: vec![],
                chosen_modes: std::sync::Arc::from([]),
                x: None,
                copy: false,
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
        let activation = self.enter_trigger_activation(
            noted.source,
            noted.ability,
            noted.created.as_deref(),
            controller,
            &noted.bindings,
        );
        let _ = self.surface_target_choice(controller, specs, id, activation);
        // [CR#603.3c]: a target spec that can't be satisfied — a slot with
        // fewer legal candidates than its minimum, or Distinct slots with no
        // distinct representatives ([CR#115.7e]) — removes the trigger from the
        // stack, never placed. Retract the surfaced decision and the
        // minted-but-unused stack identity.
        let droppable = matches!(
            &self.pending,
            Some(DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets { spec, legal, .. }))
                if !crate::resolve::announce_satisfiable(spec, legal)
        );
        if droppable {
            self.pending = None;
            self.objects.remove(id);
            return false;
        }

        // Stage the in-flight placement; the target choice is already surfaced.
        self.placing_trigger = Some(crate::trigger::PendingTrigger {
            id,
            activation,
            source: noted.source,
            ability: noted.ability,
            created: noted.created,
            controller,
            bindings: noted.bindings,
        });
        false
    }

    /// The `TriggeredAbility.targets` of a noted trigger's ability. For a
    /// delayed/reflexive trigger the body is `created` (carried by value); for
    /// a printed trigger it is `abilities_of_source(source)[ability]`. Empty
    /// when the ability is non-targeting.
    fn trigger_targets(
        &self,
        source: ObjectSource,
        ability: usize,
        created: Option<&deckmaste_core::TriggeredAbility>,
    ) -> Vec<TargetSpec> {
        if let Some(t) = created {
            return t.targets.to_vec();
        }
        let ability = &crate::derive::abilities_of_source(self, source)[ability];
        let t = ability
            .as_triggered()
            .expect("a noted trigger indexes a Triggered ability");
        t.targets.to_vec()
    }

    fn enter_trigger_activation(
        &mut self,
        source: ObjectSource,
        ability: usize,
        created: Option<&deckmaste_core::TriggeredAbility>,
        controller: PlayerId,
        bindings: &TriggerBindings,
    ) -> crate::ActivationId {
        let region = created.map_or_else(
            || {
                let abilities = crate::derive::abilities_of_source(self, source);
                let trigger = abilities[ability].as_triggered().expect("trigger index");
                triggered_effect_region(trigger)
            },
            triggered_effect_region,
        );
        self.trigger_gate_frame(&region, controller, bindings)
            .activation
    }

    fn trigger_gate_frame<T>(
        &self,
        region: &deckmaste_core::Region<T>,
        controller: PlayerId,
        bindings: &TriggerBindings,
    ) -> ExecutionFrame {
        let source = bindings.this.as_ref().map_or_else(
            || self.player(controller).object,
            |snapshot| snapshot.object,
        );
        let mut frame = self.frame(source, controller);
        self.frame_set_source_lki(&mut frame, bindings.this.clone());
        self.frame_set_defending_player(&mut frame, bindings.defending_player);
        self.frame_set_event_bindings(
            &mut frame,
            bindings.that_object.clone(),
            bindings.that_player,
            bindings.that_patient.clone(),
        );
        self.frame_set_event_extras(
            &mut frame,
            bindings.event_amount,
            bindings.produced_mana.clone(),
            bindings.crossed,
        );
        frame.activation = self.enter_created_region(region, &frame, &bindings.captures);
        frame
    }

    /// [CR#603.3d]: a placing trigger's targets were chosen — push the
    /// committed `StackEntry` (using the already-minted stack id) and clear the
    /// staging slot. Called from the `ChooseTargets` submission.
    ///
    /// # Panics
    ///
    /// Panics if no trigger placement is in flight — an engine invariant (the
    /// staging slot is open across the decision), not caller input.
    pub(crate) fn commit_placing_trigger(&mut self, targets: Vec<Vec<ObjectId>>) {
        let staged = self
            .placing_trigger
            .take()
            .expect("a trigger placement in flight");
        self.activation_set_targets(staged.activation, &targets);
        self.stack.push(StackEntry {
            paid_costs: Vec::new(),
            id: staged.id,
            activation: staged.activation,
            object: StackObject::Triggered {
                source: staged.source,
                ability: staged.ability,
                created: staged.created,
                bindings: staged.bindings,
            },
            controller: staged.controller,
            targets,
            chosen_modes: Arc::from([]),
            x: None,
            copy: false,
        });
    }
}

/// A candidate watching object ([CR#603.6]) for the trigger scan: a live
/// battlefield permanent, or a just-left object carried by its snapshot.
enum Watcher {
    Live(ObjectId),
    Leaving(LkiSnapshot),
}

/// The firing event's provenance roles ([CR#603.2e,608.2k,120.3,714.2b]),
/// derived once by [`GameState::event_roles`] and folded onto a trigger's
/// bindings by [`EventRoles::bindings_over`]. Shared by the printed and the
/// delayed/reflexive scans so both carry identical event context to
/// resolution.
pub(crate) struct EventRoles {
    that_object: Option<LkiSnapshot>,
    that_player: Option<PlayerId>,
    that_patient: Option<EventPatient>,
    defending_player: Option<PlayerId>,
    pub(crate) event_amount: Option<Uint>,
    crossed: Option<(Uint, Uint)>,
    produced_mana: Vec<deckmaste_core::ColorOrColorless>,
}

impl EventRoles {
    /// Fold these event roles onto `base` — the trigger's own captured
    /// context (its `this`/`~` snapshot) — producing the bindings the fired
    /// ability carries. `base.this` is preserved; every event role overwrites.
    pub(crate) fn bindings_over(&self, base: TriggerBindings) -> TriggerBindings {
        TriggerBindings {
            this: base.this,
            that_object: self.that_object.clone(),
            that_player: self.that_player,
            that_patient: self.that_patient.clone(),
            defending_player: self.defending_player,
            event_amount: self.event_amount,
            crossed: self.crossed,
            produced_mana: self.produced_mana.clone(),
            // A capture is fixed at CREATION ([CR#603.7a]); the firing event
            // supplies roles, never captures.
            captures: base.captures,
        }
    }
}

/// A `TriggerMultiplier`'s `extra` count as a literal. Only `Count::Literal` is
/// supported (the doublers are literal `1`s); a dynamic count is a documented
/// seam and contributes 0 additional firings until the resolve-time evaluator
/// is threaded in. Also the frameless amount-bound evaluator of the one
/// evaluator (`crate::eval`).
pub(crate) fn literal_count(count: &Count) -> Uint {
    match count {
        Count::Literal(v) => *v,
        _ => 0,
    }
}

/// Whether a pattern carries the batch quantifier ([CR#603.2c] `OneOrMore`)
/// anywhere — such a pattern matches a multi-fact occurrence ONCE, not per
/// member: `scan_event` dedups its fires across the batch.
fn contains_one_or_more(pattern: &EventFilter) -> bool {
    match pattern {
        EventFilter::OneOrMore(_) => true,
        EventFilter::AllOf(events) | EventFilter::OneOf(events) => {
            events.iter().any(contains_one_or_more)
        }
        EventFilter::Not(inner)
        | EventFilter::Nth { of: inner, .. }
        | EventFilter::When(inner, _)
        | EventFilter::Within(inner, _) => contains_one_or_more(inner),
        _ => false,
    }
}

/// The printed face behind a snapshot, or `None` for a player-proxy snapshot
/// (no card spine).
fn snapshot_face<'a>(
    state: &'a GameState,
    snapshot: &LkiSnapshot,
) -> Option<&'a deckmaste_card::CardFace> {
    match snapshot.source {
        ObjectSource::Card(card_id) => Some(crate::derive::face(&state.cards.get(card_id).def)),
        ObjectSource::Player(_) => None,
    }
}

/// Whether the snapshot's card has the named type in its printed face
/// (matched by name against the expanded `TypeDef`s).
fn snapshot_has_type(state: &GameState, snapshot: &LkiSnapshot, ty: Ident) -> bool {
    snapshot_face(state, snapshot).is_some_and(|f| f.types.iter().any(|t| t.name == ty))
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
        // [CR#209.1,306.5a]: loyalty is the PRINTED loyalty characteristic off
        // the snapshot's face — never the counter count (current loyalty is
        // `CounterCount(This, LoyaltyCounter)`), mirroring the P/T arms above.
        deckmaste_core::Stat::Loyalty => crate::layer::base_stat(face.loyalty.as_ref()),
        deckmaste_core::Stat::Defense => Some(
            deckmaste_core::Int::try_from(
                snapshot
                    .counters
                    .get("DefenseCounter")
                    .copied()
                    .unwrap_or(0),
            )
            .expect("defense fits Int"),
        ),
    }
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::empty_line_after_doc_comments,
        reason = "related behavioral test rationale is intentionally grouped"
    )]
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Condition;
    use deckmaste_core::EventFilter;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;
    use deckmaste_plugin::plugin::Plugin;

    use crate::agenda::WorkItem;
    use crate::event::AbilityUsed;
    use crate::event::BecameTarget;
    use crate::event::Blocked;
    use crate::event::ControlChanged;
    use crate::event::CounterPlaced;
    use crate::event::DamageDealt;
    use crate::event::DesignationChanged;
    use crate::event::GameEvent;
    use crate::event::GotDesignation;
    use crate::event::LifeGained;
    use crate::event::LifeLost;
    use crate::event::Occurrence;
    use crate::event::Tapped;
    use crate::event::TriggerFired;
    use crate::event::ZoneChange;
    use crate::lki::LkiSnapshot;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::stack::ExecutionFrame;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;
    use crate::target::matches;

    /// A minimal player-anchored gate frame (no bindings, no targets) for the
    /// trigger-fire intervening-if check ([CR#603.4]).
    fn gate_frame(state: &GameState, player: PlayerId) -> ExecutionFrame {
        state.frame(state.player(player).object, player)
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
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Predicate::creature()))
            .expect("a Grizzly Bears in the opening hand");
        state.zones.hands[0].retain(|&o| o != bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    /// Capture a snapshot of `id` while it's still live, then build a fake
    /// past-form `ZoneChange` as if it moved `from → to`.
    fn zone_changed_event(state: &GameState, id: ObjectId, from: Zone, to: Zone) -> GameEvent {
        let snapshot = LkiSnapshot::capture(state, id);
        GameEvent::ZoneChange(ZoneChange {
            object: id,
            face: None,
            cause: None,
            snapshot: Some(Box::new(snapshot)),
            from: Some(from),
            to,
            enters: None,
            position: None,
        })
    }

    // -------------------------------------------------------------------------
    // TriggerMultiplier (Panharmonicon, [CR#603.2d])
    // -------------------------------------------------------------------------

    /// Put `card` onto the battlefield under `controller`, returning its id.
    fn put_bf(
        state: &mut GameState,
        card: Arc<deckmaste_card::Card>,
        controller: PlayerId,
    ) -> ObjectId {
        let card_id = state.cards.push(card, controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// How many `TriggerFired` emits in `emits` name `source`.
    fn count_fired(emits: &[WorkItem], source: ObjectSource) -> usize {
        emits
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(TriggerFired { source: s, .. })))
                        if *s == source
                )
            })
            .count()
    }

    fn empty_two_player_game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// [CR#603.2d]: Panharmonicon makes an ETB triggered ability of a permanent
    /// you control trigger an additional time. Elvish Visionary's `ThisEnters`
    /// draw fires twice with one Panharmonicon out, once without it.
    #[test]
    fn panharmonicon_doubles_your_etb_trigger() {
        let canon = canon();
        let mut state = empty_two_player_game();
        let visionary = put_bf(
            &mut state,
            Arc::new(canon.card("Elvish Visionary").unwrap().core),
            PlayerId(0),
        );
        let vis_source = state.objects.obj(visionary).source;
        let pan = put_bf(
            &mut state,
            Arc::new(canon.card("Panharmonicon").unwrap().core),
            PlayerId(0),
        );

        let etb = zone_changed_event(&state, visionary, Zone::Hand, Zone::Battlefield);

        let mut emits = Vec::new();
        state.scan_event(&etb, &mut emits, &mut std::collections::HashSet::new());
        assert_eq!(
            count_fired(&emits, vis_source),
            2,
            "one Panharmonicon → the ETB trigger fires an additional time"
        );

        // Remove Panharmonicon: the trigger fires exactly once.
        state.zones.battlefield.retain(|&o| o != pan);
        let mut emits = Vec::new();
        state.scan_event(&etb, &mut emits, &mut std::collections::HashSet::new());
        assert_eq!(
            count_fired(&emits, vis_source),
            1,
            "no multiplier → a single fire"
        );
    }

    /// [CR#603.2d]: multipliers ADD, never compound — two Panharmonicons make
    /// the ETB trigger fire three times (1 + 1 + 1), not four.
    #[test]
    fn two_panharmonicons_add_to_three_fires() {
        let canon = canon();
        let mut state = empty_two_player_game();
        let visionary = put_bf(
            &mut state,
            Arc::new(canon.card("Elvish Visionary").unwrap().core),
            PlayerId(0),
        );
        let vis_source = state.objects.obj(visionary).source;
        for _ in 0..2 {
            put_bf(
                &mut state,
                Arc::new(canon.card("Panharmonicon").unwrap().core),
                PlayerId(0),
            );
        }

        let etb = zone_changed_event(&state, visionary, Zone::Hand, Zone::Battlefield);
        let mut emits = Vec::new();
        state.scan_event(&etb, &mut emits, &mut std::collections::HashSet::new());
        assert_eq!(
            count_fired(&emits, vis_source),
            3,
            "two Panharmonicons add (3×), not compound (4×)"
        );
    }

    /// An OPPONENT's Panharmonicon does not double your ETB trigger (the
    /// default `affected` is "you control" — the multiplier's controller,
    /// [CR#603.2d]).
    #[test]
    fn opponents_panharmonicon_does_not_double_your_trigger() {
        let canon = canon();
        let mut state = empty_two_player_game();
        let visionary = put_bf(
            &mut state,
            Arc::new(canon.card("Elvish Visionary").unwrap().core),
            PlayerId(0),
        );
        let vis_source = state.objects.obj(visionary).source;
        // Panharmonicon controlled by the OPPONENT (player 1).
        put_bf(
            &mut state,
            Arc::new(canon.card("Panharmonicon").unwrap().core),
            PlayerId(1),
        );

        let etb = zone_changed_event(&state, visionary, Zone::Hand, Zone::Battlefield);
        let mut emits = Vec::new();
        state.scan_event(&etb, &mut emits, &mut std::collections::HashSet::new());
        assert_eq!(
            count_fired(&emits, vis_source),
            1,
            "the opponent's doubler doesn't reach your permanent's trigger"
        );
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
        let watcher_card = Arc::new(canon.card("Moonlit Wake").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
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
            let bears = Arc::new(canon.card("Grizzly Bears").unwrap().core);
            let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
            let bid = state.objects.mint(
                ObjectSource::Card(bear_card),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(bid);
            bid
        };

        // Build the past-form `ZoneChange` for the Grizzly Bears dying.
        let event = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);

        // The pattern from Dies(Type(Creature)) — built directly.
        let pattern = EventFilter::ZoneChange {
            cause: None,
            what: Predicate::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            state.event_matches(&pattern, &event, watcher_source),
            "Dies(Type(\"Creature\")) must match a creature dying"
        );
    }

    /// … and must NOT match a creature entering (wrong direction).
    #[test]
    fn dies_type_creature_does_not_match_creature_entering() {
        let canon = canon();
        let bears = Arc::new(canon.card("Grizzly Bears").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Predicate::creature()))
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
            attached_to: None,
            left: Zone::Hand,
        };
        let enter_event = GameEvent::ZoneChange(ZoneChange {
            object: bear,
            snapshot: Some(Box::new(snapshot)),
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
            enters: None,
            position: None,
            face: None,
            cause: None,
        });

        let dies_pattern = EventFilter::ZoneChange {
            cause: None,
            what: Predicate::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            !state.event_matches(&dies_pattern, &enter_event, watcher_source),
            "Dies(Type(\"Creature\")) must NOT match an entering event"
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
            attached_to: None,
            left: Zone::Battlefield,
        };
        let event = GameEvent::ZoneChange(ZoneChange {
            object: land,
            snapshot: Some(Box::new(snapshot)),
            from: Some(Zone::Battlefield),
            to: Zone::Graveyard,
            enters: None,
            position: None,
            face: None,
            cause: None,
        });

        let dies_pattern = EventFilter::ZoneChange {
            cause: None,
            what: Predicate::creature(),
            from: Some(Zone::Battlefield),
            to: Some(Zone::Graveyard),
        };

        assert!(
            !state.event_matches(&dies_pattern, &event, watcher_source),
            "Dies(Type(\"Creature\")) must NOT match a non-creature"
        );
    }

    // -------------------------------------------------------------------------
    // Dies(Ref(This)) — self-dies
    // -------------------------------------------------------------------------

    /// `Dies(Ref(This))` matches only when the dying object IS the watcher.
    #[test]
    fn dies_this_matches_only_self_death() {
        let canon = canon();
        let dies_card = Arc::new(canon.card("Footlight Fiend").unwrap().core);
        let bears = Arc::new(canon.card("Grizzly Bears").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });

        // Place the dies-trigger creature on the battlefield.
        let trigger_obj = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Predicate::creature()))
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
        let self_dies = EventFilter::ZoneChange {
            cause: None,
            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
        let etb_card = Arc::new(canon.card("Elvish Visionary").unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });

        // The ETB creature starts in hand; we want to simulate it entering.
        let etb_obj = *state.zones.hands[0]
            .iter()
            .find(|&&o| matches(&state, o, &Predicate::creature()))
            .expect("ETB creature in hand");
        let etb_source = state.objects.obj(etb_obj).source;

        // Pattern: Enters(Ref(This))
        let self_enters = EventFilter::ZoneChange {
            cause: None,
            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            from: None,
            to: Some(Zone::Battlefield),
        };

        // Build a past-form ZoneChange snapshot for the ETB creature entering.
        let enters_snapshot = LkiSnapshot {
            object: etb_obj,
            source: etb_source,
            controller: PlayerId(0),
            tapped: false,
            damage: 0,
            counters: std::collections::HashMap::new(),
            attached_to: None,
            // For an enter, the snapshot's `left` represents what zone it came
            // from — Hand in this case.
            left: Zone::Hand,
        };
        let enters_event = GameEvent::ZoneChange(ZoneChange {
            object: etb_obj,
            snapshot: Some(Box::new(enters_snapshot)),
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
            enters: None,
            position: None,
            face: None,
            cause: None,
        });

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

        // A future-form ZoneChange (not the past form) must not match.
        let will_change_event = GameEvent::ZoneChange(ZoneChange {
            snapshot: None,
            object: etb_obj,
            from: Some(Zone::Hand),
            to: Zone::Battlefield,
            enters: None,
            position: None,
            face: None,
            cause: None,
        });
        assert!(
            !state.event_matches(&self_enters, &will_change_event, etb_source),
            "Enters triggers must not fire on the future-form ZoneChange (only on the past form)"
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
        let cond = Condition::Exists(Predicate::r#type(Type::Creature));
        assert!(
            state.condition_holds(&cond, &gate_frame(&state, PlayerId(0))),
            "Exists(Type(\"Creature\")) should hold when a creature is on the battlefield"
        );
    }

    #[test]
    fn condition_holds_exists_creature_false_when_no_creature() {
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        let cond = Condition::Exists(Predicate::r#type(Type::Creature));
        assert!(
            !state.condition_holds(&cond, &gate_frame(&state, PlayerId(0))),
            "Exists(Type(\"Creature\")) should NOT hold when no creatures are present"
        );
    }

    // -------------------------------------------------------------------------
    // Training's intervening-if ([CR#702.149a,603.4]): "at least one other
    // creature with power greater than this creature's power attacks" — the
    // `Compare(CountOf(... Where(StatOf vs This) ...), AtLeast, 1)` the keyword
    // macro emits. `CountOf` threads the carrier watcher into the filter, so
    // the candidate-vs-carrier power comparison resolves `This` to the
    // carrier.
    // -------------------------------------------------------------------------

    /// Build a board (all P0) of declared attackers: a 3/3 Centaur Courser
    /// (the Training carrier) plus the named extra creatures. Returns the state
    /// and the carrier id. A frame is built separately with `this` bound to the
    /// carrier so `frame_watcher` anchors `This` to it.
    #[allow(
        dead_code,
        reason = "shared fixture retained for neighboring trigger cases"
    )]
    fn attacking_board(extras: &[&str]) -> (GameState, ObjectId) {
        let courser = Arc::new(canon().card("Centaur Courser").unwrap().core);
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&courser); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&courser); 10],
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
        let put = |state: &mut GameState, name: &str| {
            let card = Arc::new(canon().card(name).unwrap().core);
            let cid = state.cards.push(card, PlayerId(0));
            let id = state.objects.mint(
                crate::object::ObjectSource::Card(cid),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            let def = state
                .player(state.next_live_after(state.objects.obj(id).controller))
                .object;
            state.combat.declare_attacker(id, def);
            id
        };
        let carrier = put(&mut state, "Centaur Courser"); // 3/3
        for name in extras {
            put(&mut state, name);
        }
        (state, carrier)
    }

    /// A trigger-fire gate frame whose `this` binding is the carrier — mirrors
    /// the real intervening-if frame (the exophoric `this` set to the firing
    /// object's snapshot), so `frame_watcher` resolves `This` to the carrier.
    #[allow(
        dead_code,
        reason = "shared fixture retained for neighboring trigger cases"
    )]
    fn carrier_gate_frame(state: &GameState, carrier: ObjectId) -> ExecutionFrame {
        let mut frame = state.frame(carrier, PlayerId(0));
        state.frame_set_source_lki(
            &mut frame,
            Some(crate::lki::LkiSnapshot::capture(state, carrier)),
        );
        frame
    }

    // -------------------------------------------------------------------------
    // Event macro round-trip: Dies / Enters expand correctly
    // -------------------------------------------------------------------------

    /// Confirm that `Dies(Type(Creature))` parses and lowers to `ZoneChange`.
    #[test]
    fn dies_macro_expands_to_zone_move() {
        use deckmaste_core::EventFilter;

        let semantic: deckmaste_semantics::EventFilter =
            canon().macros.read_str("Dies(Type(Creature))").unwrap();
        let event: EventFilter = deckmaste_lowering::Lower::lower(semantic);
        assert_eq!(
            event,
            EventFilter::ZoneChange {
                cause: None,
                what: Predicate::creature(),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
            }
        );
    }

    /// Confirm that `Enters(Ref(This))` expands to `ZoneMove { to: Battlefield,
    /// from: None }`.
    #[test]
    fn enters_macro_expands_to_zone_move() {
        use deckmaste_core::EventFilter;

        let semantic: deckmaste_semantics::EventFilter =
            canon().macros.read_str("Enters(Ref(This))").unwrap();
        let event: EventFilter = deckmaste_lowering::Lower::lower(semantic);
        assert_eq!(
            event,
            EventFilter::ZoneChange {
                cause: None,
                what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
        use deckmaste_core::EventFilter;

        let semantic: deckmaste_semantics::EventFilter = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let event: EventFilter = deckmaste_lowering::Lower::lower(semantic);
        assert_eq!(
            event,
            EventFilter::ZoneChange {
                what: Predicate::creature(),
                from: Some(Zone::Battlefield),
                to: Some(Zone::Graveyard),
                cause: Some(Cause::Cause(CausePattern {
                    verb: Some(deckmaste_core::VerbName::from("Destroy")),
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
        let GameEvent::ZoneChange(ZoneChange {
            object,
            snapshot: Some(snapshot),
            from,
            to,
            enters,
            position,
            face,
            ..
        }) = zone_changed_event(state, id, from, to)
        else {
            unreachable!("zone_changed_event builds a past-form ZoneChange");
        };
        GameEvent::ZoneChange(ZoneChange {
            object,
            snapshot: Some(snapshot),
            from,
            to,
            enters,
            position,
            face,
            cause: Some(cause),
        })
    }

    /// The SBA destruction cause ([CR#701.8b] — one of "destroyed"'s two
    /// admitted causes; no agent, [CR#704] actions have none).
    fn sba_destroy_cause() -> crate::event::Cause {
        crate::event::Cause {
            verb: "Destroy".into(),
            agency: deckmaste_core::Agency::StateBasedAction,
            agent: None,
            payment: None,
        }
    }

    /// The canon `Destroyed(Type(Creature))` view matches a creature dying
    /// to the lethal-damage SBA ([CR#701.8b,704.5g]) …
    #[test]
    fn destroyed_matches_destroy_caused_death() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        // Parsed through the SEMANTICS path (`semantics::EventFilter` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::EventFilter = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let pattern: EventFilter = deckmaste_lowering::Lower::lower(semantic);
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
        // Parsed through the SEMANTICS path (`semantics::EventFilter` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::EventFilter = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let pattern: EventFilter = deckmaste_lowering::Lower::lower(semantic);
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
        // Parsed through the SEMANTICS path (`semantics::EventFilter` →
        // `lower()`), the path production now takes.
        let semantic: deckmaste_semantics::EventFilter = canon()
            .macros
            .read_str("Destroyed(Type(Creature))")
            .unwrap();
        let pattern: EventFilter = deckmaste_lowering::Lower::lower(semantic);
        let event = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            crate::event::Cause {
                verb: "Sacrifice".into(),
                agency: deckmaste_core::Agency::EffectInstruction,
                agent: None,
                payment: None,
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
        let pattern = EventFilter::ZoneChange {
            cause: None,
            what: Predicate::creature(),
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
        let pattern = EventFilter::ZoneChange {
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: None,
                agency: Some(deckmaste_core::Agency::StateBasedAction),
                agent: None,
            })),
            what: Predicate::Any,
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
                payment: None,
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
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = EventFilter::ZoneChange {
            cause: Some(deckmaste_core::Cause::Cause(CausePattern {
                verb: Some(deckmaste_core::VerbName::from("Destroy")),
                agency: None,
                agent: Some(Predicate::r#type(Type::Creature)),
            })),
            what: Predicate::Any,
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
                payment: None,
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
    // StateBecame: becomes-tapped / becomes-untapped ([CR#603.2e])
    // -------------------------------------------------------------------------

    /// `StateBecame(of: Ref(This), becomes: Tapped)` matches the watcher's
    /// own tap fact, anchored by the `of` filter — and not another object's.
    #[test]
    fn becomes_tapped_matches_the_tap_fact() {
        use deckmaste_core::StateChange;

        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(0));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(0),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = EventFilter::StateBecame {
            of: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            becomes: StateChange::Tapped,
            cause: None,
        };
        let own_tap = GameEvent::Tapped(Tapped {
            object: bear,
            cause: None,
        });
        let other_tap = GameEvent::Tapped(Tapped {
            object: other,
            cause: None,
        });
        assert!(
            state.event_matches(&pattern, &own_tap, watcher_source),
            "the watcher's own tap matches"
        );
        assert!(
            !state.event_matches(&pattern, &other_tap, watcher_source),
            "another object's tap fails the of-filter"
        );
        assert!(
            !state.event_matches(&pattern, &GameEvent::Untapped(bear, None), watcher_source),
            "an untap is not a tap"
        );
    }

    /// `StateBecame(becomes: Untapped)` matches the untap fact and not the
    /// tap fact.
    #[test]
    fn becomes_untapped_matches_the_untap_fact() {
        use deckmaste_core::StateChange;

        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::StateBecame {
            of: Predicate::creature(),
            becomes: StateChange::Untapped,
            cause: None,
        };
        assert!(state.event_matches(&pattern, &GameEvent::Untapped(bear, None), watcher_source));
        assert!(!state.event_matches(
            &pattern,
            &GameEvent::Tapped(Tapped {
                object: bear,
                cause: None
            }),
            watcher_source
        ));
    }

    /// An UNNARROWED `StateBecame` pattern (`cause` omitted) matches the tap
    /// fact regardless of WHY it tapped (cost [CR#107.5] vs effect
    /// [CR#701.26a]) — the omitted-cause match-anything default, same as
    /// every other cause-narrowed filter (`ZoneChange`, `Act`).
    #[test]
    fn becomes_tapped_matches_any_tap_cause() {
        use deckmaste_core::StateChange;

        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::StateBecame {
            of: Predicate::Any,
            becomes: StateChange::Tapped,
            cause: None,
        };
        let cost_tap = GameEvent::Tapped(Tapped {
            object: bear,
            cause: Some(crate::event::Cause {
                verb: "Tap".into(),
                agency: deckmaste_core::Agency::CostPayment,
                agent: None,
                payment: None,
            }),
        });
        let effect_tap = GameEvent::Tapped(Tapped {
            object: bear,
            cause: Some(crate::event::Cause {
                verb: "Tap".into(),
                agency: deckmaste_core::Agency::EffectInstruction,
                agent: None,
                payment: None,
            }),
        });
        assert!(state.event_matches(&pattern, &cost_tap, watcher_source));
        assert!(state.event_matches(&pattern, &effect_tap, watcher_source));
    }

    /// A NARROWED `StateBecame` pattern (T5's new coordinate) matches only
    /// the cause it names — a cost-payment tap doesn't satisfy an
    /// effect-instruction narrowing and vice versa.
    #[test]
    fn becomes_tapped_cause_narrowing_matches_only_its_agency() {
        use deckmaste_core::Agency;
        use deckmaste_core::StateChange;

        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::StateBecame {
            of: Predicate::Any,
            becomes: StateChange::Tapped,
            cause: Some(deckmaste_core::Cause::Cause(deckmaste_core::CausePattern {
                verb: None,
                agency: Some(Agency::CostPayment),
                agent: None,
            })),
        };
        let cost_tap = GameEvent::Tapped(Tapped {
            object: bear,
            cause: Some(crate::event::Cause {
                verb: "Tap".into(),
                agency: Agency::CostPayment,
                agent: None,
                payment: None,
            }),
        });
        let effect_tap = GameEvent::Tapped(Tapped {
            object: bear,
            cause: Some(crate::event::Cause {
                verb: "Tap".into(),
                agency: Agency::EffectInstruction,
                agent: None,
                payment: None,
            }),
        });
        assert!(
            state.event_matches(&pattern, &cost_tap, watcher_source),
            "cost-payment narrowing matches the cost tap"
        );
        assert!(
            !state.event_matches(&pattern, &effect_tap, watcher_source),
            "cost-payment narrowing excludes the effect tap"
        );
    }

    // -------------------------------------------------------------------------
    // BlockDeclared: becomes-blocked ([CR#509.3c]) — attacker-side, deduped
    // -------------------------------------------------------------------------

    /// `BlockDeclared(of: Ref(This))` watches the ATTACKER — "becomes
    /// blocked" is the attacker's view of the declaration ([CR#509.3c]); the
    /// blocker-side "blocks" view ([CR#509.3a]) is the `by` position.
    #[test]
    fn becomes_blocked_matches_the_attacker_not_the_blocker() {
        let (mut state, attacker) = bear_on_field();
        let attacker_source = state.objects.obj(attacker).source;
        let blocker = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
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
        let pattern = EventFilter::BlockDeclared {
            by: Predicate::Any,
            of: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
        };
        let event = GameEvent::Blocked(Blocked { blocker, attacker });
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
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
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
            GameEvent::Blocked(Blocked {
                blocker: b1,
                attacker: tantiv,
            }),
            GameEvent::Blocked(Blocked {
                blocker: b2,
                attacker: tantiv,
            }),
        ]));
        let fired = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
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
            let card = Arc::new(canon().card("Deepwood Tantiv").unwrap().core);
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
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
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
            GameEvent::Blocked(Blocked {
                blocker: b1,
                attacker: t1,
            }),
            GameEvent::Blocked(Blocked {
                blocker: b2,
                attacker: t2,
            }),
        ]));
        let fired = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
                )
            })
            .count();
        assert_eq!(fired, 2, "each attacker's own transition fires");
    }

    // -------------------------------------------------------------------------
    // Act(Fight): symmetric per-subject "whenever ~ fights" ([CR#701.14a])
    // -------------------------------------------------------------------------

    /// [CR#701.14a]: "whenever ~ fights" matches when ~ is EITHER combatant —
    /// each per-subject committed fact carries one fighter on `on`, and the
    /// uniform arm matches the pattern's `on` slot against it.
    #[test]
    fn fight_pattern_on_slot_matches_either_fighters_fact() {
        use deckmaste_core::VerbName;

        use crate::event::Act;

        let (mut state, a) = bear_on_field();
        let b = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = EventFilter::Act {
            verb: VerbName::from("Fight"),
            who: Predicate::Any,
            on: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            cause: None,
        };
        let fact_for = |subject: ObjectId| {
            GameEvent::Act(Act {
                verb: VerbName::from("Fight"),
                who: None,
                on: vec![subject],
                from: None,
                to: None,
                cause: None,
                committed: true,
                contents: None,
                batch: None,
                inherited: std::collections::HashSet::new(),
                contained: false,
            })
        };
        let watcher_b = state.objects.obj(b).source;
        assert!(state.event_matches(&pattern, &fact_for(b), watcher_b));
        assert!(
            !state.event_matches(&pattern, &fact_for(a), watcher_b),
            "the OTHER fighter's fact does not match This=b"
        );
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
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(1));
            state
                .objects
                .mint(ObjectSource::Card(card), PlayerId(1), Some(Zone::Stack))
        };
        let pattern = EventFilter::BecomesTarget {
            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            by: Predicate::Any,
            source: None,
        };
        let event = GameEvent::BecameTarget(BecameTarget {
            target: bear,
            source: spell,
        });
        assert!(
            state.event_matches(&pattern, &event, watcher_source),
            "the watcher's own became-target fact matches"
        );
        let other_event = GameEvent::BecameTarget(BecameTarget {
            target: spell,
            source: bear,
        });
        assert!(
            !state.event_matches(&pattern, &other_event, watcher_source),
            "a fact targeting a different object fails the what-filter"
        );

        // Ward's by-narrowing: an opponent-controlled stack object matches;
        // one the watcher's own controller controls does not.
        let by_opponent = EventFilter::BecomesTarget {
            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            by: Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(Arc::new(
                Predicate::Relation(deckmaste_core::RelationPredicate::OpponentOf(Arc::new(
                    Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                ))),
            ))),
            source: None,
        };
        assert!(
            state.event_matches(&by_opponent, &event, watcher_source),
            "an opponent's spell passes the by-filter"
        );
        let own_spell = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(0));
            state
                .objects
                .mint(ObjectSource::Card(card), PlayerId(0), Some(Zone::Stack))
        };
        let own_event = GameEvent::BecameTarget(BecameTarget {
            target: bear,
            source: own_spell,
        });
        assert!(
            !state.event_matches(&by_opponent, &own_event, watcher_source),
            "the watcher's controller's own spell fails the by-filter"
        );
    }

    // -------------------------------------------------------------------------
    // Onset master forms ([CR#603.2] over the action log)
    // -------------------------------------------------------------------------

    /// Prowess's pattern ([CR#702.108a]): `Cast(who: Ref(You), what:
    /// noncreature spell)` matches the controller's own noncreature cast
    /// ([CR#601.2i]) — and not an opponent's cast or a creature spell.
    #[test]
    fn performed_cast_matches_own_noncreature_cast() {
        use deckmaste_core::ObjectClass;

        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::Cast {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            what: Predicate::And(
                vec![
                    Predicate::Class(ObjectClass::Spell),
                    Predicate::Not(Arc::new(Predicate::r#type(Type::Creature))),
                ]
                .into(),
            ),
        };
        let mut spell_on_stack = |name: &str, controller: PlayerId| {
            let card = Arc::new(canon().card(name).unwrap().core);
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

        let roles = state.event_roles(&GameEvent::SpellCast(opp_bolt));
        assert_eq!(
            roles.that_object.as_ref().map(|snapshot| snapshot.object),
            Some(opp_bolt),
            "the cast spell is the event object"
        );
        assert_eq!(
            roles.that_player,
            Some(PlayerId(1)),
            "the spell's controller is the cast event actor"
        );
    }

    /// `Damage(source: Ref(This))` matches the watcher's own damage facts
    /// ([CR#120.1]) — `source` is the SOURCE, `to` the recipient.
    #[test]
    fn performed_deal_damage_matches_source_and_target() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = EventFilter::Damage {
            source: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        let own_damage = GameEvent::DamageDealt(DamageDealt {
            source: bear,
            target: other,
            amount: 2,
            combat: false,
        });
        let others_damage = GameEvent::DamageDealt(DamageDealt {
            source: other,
            target: bear,
            amount: 2,
            combat: false,
        });
        assert!(
            state.event_matches(&pattern, &own_damage, watcher_source),
            "the watcher dealing damage matches source: Ref(This)"
        );
        assert!(
            !state.event_matches(&pattern, &others_damage, watcher_source),
            "damage dealt BY another source does not"
        );
    }

    /// The `combat` refinement discriminates combat vs noncombat damage
    /// ([CR#120.2a,510.1]): `Damage(combat: true)` fires ONLY on the
    /// combat-damage step's assignments, `Damage(combat: false)` ONLY on
    /// effect/fight damage ([CR#701.14d]), and an omitted `combat` matches
    /// either — the "deals combat damage to a player" trigger family relies on
    /// this, and must never fire on noncombat damage.
    #[test]
    fn combat_refinement_discriminates_combat_vs_noncombat() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let combat_damage = GameEvent::DamageDealt(DamageDealt {
            source: bear,
            target: other,
            amount: 2,
            combat: true,
        });
        let noncombat_damage = GameEvent::DamageDealt(DamageDealt {
            source: bear,
            target: other,
            amount: 2,
            combat: false,
        });

        let wants_combat = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: Some(true),
            amount: None,
        };
        assert!(
            state.event_matches(&wants_combat, &combat_damage, watcher_source),
            "Damage(combat: true) fires on combat damage"
        );
        assert!(
            !state.event_matches(&wants_combat, &noncombat_damage, watcher_source),
            "Damage(combat: true) must NOT fire on noncombat damage"
        );

        let wants_noncombat = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: Some(false),
            amount: None,
        };
        assert!(
            state.event_matches(&wants_noncombat, &noncombat_damage, watcher_source),
            "Damage(combat: false) fires on noncombat damage"
        );
        assert!(
            !state.event_matches(&wants_noncombat, &combat_damage, watcher_source),
            "Damage(combat: false) must NOT fire on combat damage"
        );

        let any_damage = EventFilter::Damage {
            source: Predicate::Any,
            to: Predicate::Any,
            combat: None,
            amount: None,
        };
        assert!(
            state.event_matches(&any_damage, &combat_damage, watcher_source)
                && state.event_matches(&any_damage, &noncombat_damage, watcher_source),
            "an omitted combat refinement matches either kind"
        );
    }

    /// A sacrifice view is a cause-narrowed `ZoneChange` ([CR#701.21a] — the
    /// zone-change unification retired the dedicated verb facts): "you
    /// sacrifice" is
    /// spelled as the moved object being controlled by you.
    #[test]
    fn performed_sacrifice_matches_cause_carried_move() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::ZoneChange {
            what: Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(Arc::new(
                Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            ))),
            from: None,
            to: None,
            cause: Some(deckmaste_core::Cause::Cause(deckmaste_core::CausePattern {
                verb: Some(deckmaste_core::VerbName::from("Sacrifice")),
                agency: None,
                agent: None,
            })),
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
                    payment: None,
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
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
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
        // An unattributed death is not a sacrifice ([CR#700.4] vs
        // [CR#701.21a]).
        let plain_death = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
        assert!(
            !state.event_matches(&pattern, &plain_death, watcher_source),
            "an uncaused move performs no verb"
        );
    }

    // -------------------------------------------------------------------------
    // Becomes-deltas: control change + game-scope designation ([CR#603.2e])
    // -------------------------------------------------------------------------

    /// `ControlChanged(of, to)` matches a `ControlChanged` fact whose NEW
    /// controller satisfies `to` — a control change is never a zone move
    /// ([CR#603.2e]; the object keeps its identity).
    #[test]
    fn controlled_by_matches_control_change() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(1));
            let id = state.objects.mint(
                ObjectSource::Card(card),
                PlayerId(1),
                Some(Zone::Battlefield),
            );
            state.zones.battlefield.push(id);
            id
        };
        let pattern = EventFilter::ControlChanged {
            of: Predicate::creature(),
            to: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
        };
        let to_you = GameEvent::ControlChanged(ControlChanged {
            object: other,
            to: PlayerId(0),
        });
        let to_them = GameEvent::ControlChanged(ControlChanged {
            object: other,
            to: PlayerId(1),
        });
        assert!(
            state.event_matches(&pattern, &to_you, watcher_source),
            "a creature coming under YOUR control matches to: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &to_them, watcher_source),
            "one coming under an opponent's control does not"
        );
    }

    /// The game-scope day/night flip ([CR#731.1a] — "day becomes night"
    /// loses one designation and gains the other) is a generic named enum
    /// designation transition. `to` distinguishes Day from Night; omitting it
    /// matches either transition. A game-scope fact has no carrier, so only
    /// the match-anything `of` is satisfiable.
    #[test]
    fn designation_changed_matches_game_scope_flip() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let to_night = GameEvent::DesignationChanged(DesignationChanged {
            name: "DayNight".into(),
            becomes: Some("Night".into()),
        });
        let to_day = GameEvent::DesignationChanged(DesignationChanged {
            name: "DayNight".into(),
            becomes: Some("Day".into()),
        });
        let named_any = EventFilter::DesignationChanged {
            name: "DayNight".into(),
            of: Predicate::Any,
            to: None,
        };
        let became_night = EventFilter::DesignationChanged {
            name: "DayNight".into(),
            of: Predicate::Any,
            to: Some("Night".into()),
        };
        let named_narrowed = EventFilter::DesignationChanged {
            name: "DayNight".into(),
            of: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            to: None,
        };
        let wrong_name = EventFilter::DesignationChanged {
            name: "Monarch".into(),
            of: Predicate::Any,
            to: None,
        };
        assert!(state.event_matches(&named_any, &to_night, watcher_source));
        assert!(state.event_matches(&became_night, &to_night, watcher_source));
        assert!(!state.event_matches(&became_night, &to_day, watcher_source));
        assert!(!state.event_matches(&named_narrowed, &to_night, watcher_source));
        assert!(!state.event_matches(&wrong_name, &to_night, watcher_source));
        let got = GameEvent::GotDesignation(GotDesignation {
            player: PlayerId(0),
            name: "Monarch".into(),
        });
        let monarch_you = EventFilter::DesignationChanged {
            name: "Monarch".into(),
            of: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            to: None,
        };
        assert!(
            state.event_matches(&monarch_you, &got, watcher_source),
            "a player-scope gain runs `of` against the gaining player's proxy"
        );
        assert!(
            !state.event_matches(
                &monarch_you,
                &GameEvent::GotDesignation(GotDesignation {
                    player: PlayerId(1),
                    name: "Monarch".into(),
                }),
                watcher_source
            ),
            "an opponent's gain fails of: Ref(You)"
        );
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
        let creature = Predicate::creature();
        let pattern = EventFilter::OneOf(
            vec![
                EventFilter::ZoneChange {
                    cause: None,
                    what: creature.clone(),
                    from: Some(Zone::Battlefield),
                    to: Some(Zone::Graveyard),
                },
                EventFilter::ZoneChange {
                    cause: None,
                    what: creature,
                    from: None,
                    to: Some(Zone::Battlefield),
                },
            ]
            .into(),
        );

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
    /// `Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0)))` in the `what`
    /// position — the "this object" form.
    #[test]
    fn dies_this_filter_ref_reference_this() {
        use deckmaste_core::EventFilter;

        let semantic: deckmaste_semantics::EventFilter =
            canon().macros.read_str("Dies(Ref(This))").unwrap();
        let event: EventFilter = deckmaste_lowering::Lower::lower(semantic);
        let EventFilter::ZoneChange { what, .. } = event else {
            panic!("expected ZoneChange inner");
        };
        assert_eq!(
            what,
            Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            "Dies(Ref(This)) must use Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0)))"
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

        let card = Arc::new(canon().card(card_name).unwrap().core);
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        });
        // Load builtin rules so data-driven SBAs (lethal-damage destroy,
        // toughness-0 move, etc.) fire when these tests drive `CheckSbas`.
        state.sba_rules = builtin().sba_rules;
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
        let card = Arc::new(canon().card(name).unwrap().core);
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
    /// lethal damage: stepping past the SBA destroy (`CheckSbas` → future-form
    /// `ZoneChange` → past-form `ZoneChange` → `TriggerFired` apply) notes
    /// exactly one trigger, whose `this` binding is the LKI snapshot of the
    /// (now-gone) battlefield id.
    #[test]
    fn dies_trigger_notes_into_pending_triggers() {
        use crate::agenda::WorkItem;

        let (mut state, goblin) = fixture_on_field("Footlight Fiend");
        // toughness 1 → 1 damage is lethal.
        state.objects.obj_mut(goblin).set_marked_damage(1);

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

    /// [CR#700.4] "when it dies" off a COMPOSITE destroy: an effect
    /// `Act(Destroy)` — now committed atomically (no separate future-form
    /// `ZoneChange`) — still produces the past-form `ZoneChange
    /// (Battlefield→Graveyard)` fact the self-dies trigger fires on. The
    /// trigger fires on the committed body fact, not the `Act` tag, so it
    /// fires exactly once.
    #[test]
    fn dies_trigger_fires_off_a_composite_destroy() {
        use deckmaste_core::Action;
        use deckmaste_core::Instruction;
        use deckmaste_core::Reference;

        let (mut state, goblin) = fixture_on_field("Footlight Fiend");
        let frame = crate::test_support::frame_src(&state, goblin);
        // An EFFECT destroy (not the lethal-damage SBA) — the dual-facet
        // `Act(Destroy)` commits Battlefield→Graveyard on apply.
        state.run_effect(
            Instruction::Act(Action::destroy(Reference::Reg(deckmaste_core::RefId(0)))),
            &frame,
        );
        for _ in 0..30 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert!(
            state.objects.get(goblin).is_none(),
            "the composite destroy reminted the goblin into the graveyard"
        );
        assert_eq!(
            state.pending_triggers.len(),
            1,
            "the self-dies trigger fires exactly once off the composite destroy"
        );
        assert_eq!(
            state.pending_triggers[0]
                .bindings
                .this
                .as_ref()
                .unwrap()
                .object,
            goblin,
            "LKI snapshot of the destroyed goblin",
        );
    }

    /// A non-watching board: a `Grizzly Bears` dying notes NOTHING (it has
    /// no triggered abilities, and no other watcher cares).
    #[test]
    fn vanilla_creature_dying_notes_nothing() {
        use crate::agenda::WorkItem;

        let (mut state, bear) = fixture_on_field("Grizzly Bears");
        // Grizzly Bears has toughness 2; set lethal damage.
        state.objects.obj_mut(bear).set_marked_damage(2);

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

    /// A transforming DFC whose "when ~ transforms" watcher (draw a card) is
    /// printed IDENTICALLY on both faces, so it's visible to the scan
    /// regardless of which face is showing when the fact fires — isolating
    /// the bare `StateBecame(Transformed)` trigger from the destination-
    /// narrowed "transforms into X" reading ([CR#701.27e]), which is out of
    /// scope here.
    fn transform_watcher_dfc() -> deckmaste_card::Card {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::DoubleFacedLayout;
        use deckmaste_core::Ability;
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::StatValue;
        use deckmaste_core::StateChange;
        use deckmaste_core::TriggeredAbility;

        let watch = Ability::triggered(TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event: EventFilter::StateBecame {
                of: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                becomes: StateChange::Transformed,
                cause: None,
            },
            condition: None,
            limits: Vec::new().into(),
            effect: Instruction::draw(Reference::Reg(deckmaste_core::RefId(1)), Count::Literal(1))
                .into(),
        });
        let face = |name: &str| CardFace {
            name: name.into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(1)),
            toughness: Some(StatValue::Number(1)),
            abilities: vec![watch.clone()],
            ..CardFace::default()
        };
        Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front: face("Front Watcher"),
            back: face("Back Watcher"),
        }
    }

    /// [CR#701.27a,603.2e]: resolving a real `Action::Transform` emits the
    /// `GameEvent::Transformed` fact (Task 4), which the DFC's own bare
    /// "when ~ transforms" watcher fires off — noting its ability into
    /// `pending_triggers` exactly once. Drives the real Transform action
    /// (not a synthetic fact) so the event reaches the scan through the
    /// ordinary agenda pipeline, mirroring
    /// `dies_trigger_notes_into_pending_triggers`'s NOTE-step assertion
    /// shape.
    #[test]
    fn transform_fires_when_transforms_trigger() {
        use deckmaste_core::Action;
        use deckmaste_core::Instruction;

        use crate::object::Side;

        let mut state = empty_game();
        let dfc = put_synthetic_on_field(&mut state, transform_watcher_dfc(), PlayerId(0));
        let frame = crate::test_support::frame_src(&state, dfc);
        state.run_effect(
            Instruction::Act(Action::Transform(Reference::Reg(deckmaste_core::RefId(0)))),
            &frame,
        );

        for _ in 0..30 {
            if !state.pending_triggers.is_empty() {
                break;
            }
            let _ = state.step();
        }

        assert_eq!(
            state.pending_triggers.len(),
            1,
            "the when-~-transforms watcher must fire exactly once off the real Transformed event"
        );
        let noted = &state.pending_triggers[0];
        assert_eq!(noted.controller, PlayerId(0));
        assert_eq!(
            noted.bindings.this.as_ref().map(|s| s.object),
            Some(dfc),
            "the transformed DFC's own snapshot binds as `this`"
        );
        assert!(
            state.objects.get(dfc).is_some(),
            "unlike a dies-trigger, transform does not remove the permanent from the battlefield"
        );
        assert_eq!(
            state.objects.obj(dfc).side,
            Side::Back,
            "the DFC actually flipped [CR#712.18]"
        );
    }

    /// A hand-authored "Delverish ↔ Aberration" transforming DFC — front a
    /// vanilla 1/1 Creature, back a 3/2 Creature carrying Trample, a static
    /// keyword the front face lacks. Unlike `transform_watcher_dfc`, it has
    /// NO abilities of its own — `delver_transforms_end_to_end` drives the
    /// flip directly via a real `Action::Transform`, not a card-granted
    /// trigger/activated ability, and the asymmetric Trample isolates the
    /// face-aware layered-view read from the P/T flip.
    fn delverish_aberration() -> deckmaste_card::Card {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::DoubleFacedLayout;
        use deckmaste_core::Ability;
        use deckmaste_core::KeywordAbility;
        use deckmaste_core::StatValue;

        let front = CardFace {
            name: "Delverish".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(1)),
            toughness: Some(StatValue::Number(1)),
            ..CardFace::default()
        };
        let back = CardFace {
            name: "Aberration".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(3)),
            toughness: Some(StatValue::Number(2)),
            abilities: vec![Ability::Keyword(KeywordAbility::Trample)],
            ..CardFace::default()
        };
        Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front,
            back,
        }
    }

    /// Delver-style end-to-end integration test ([CR#712]): a hand-authored
    /// transforming DFC driven through the REAL `Action::Transform` path
    /// (Task 5/6), asserting the full contract in one pass rather than
    /// piecemeal:
    ///  1. It enters FRONT-up ([CR#712.14]) — 1/1 "Delverish", no Trample.
    ///  2. An until-end-of-turn +2/+2 applied BEFORE transforming survives the
    ///     flip on the SAME `ObjectId` ([CR#712.18]): back base 3/2 + 2/2 =
    ///     5/4, and the back face's Trample (absent on the front) becomes
    ///     visible through the layered view — exercising the face-aware read
    ///     end-to-end through a genuine transform, not a hand-set `side`.
    ///  3. A genuine zone change (dies to the graveyard, then returns to the
    ///     battlefield as a NEW object, [CR#400.7]) resets to FRONT-up
    ///     ([CR#712.14]): back to 1/1 "Delverish", no Trample.
    ///
    /// If Tasks 1-6 are complete this needs no new production code; a
    /// failing assertion here points at a real gap (e.g. a direct
    /// front/back read that bypasses `face_of`/the layered view).
    #[test]
    fn delver_transforms_end_to_end() {
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::Duration;
        use deckmaste_core::Instruction;
        use deckmaste_core::KeywordAbility;
        use deckmaste_core::Modification;
        use deckmaste_core::NumericOp;
        use deckmaste_core::TurnMarker;

        use crate::combat::has_keyword;
        use crate::layer::ContinuousEffect;
        use crate::layer::ScopeResolved;
        use crate::object::Side;
        use crate::object::Timestamp;

        let mut state = empty_game();
        let delver = put_synthetic_on_field(&mut state, delverish_aberration(), PlayerId(0));

        // 1. Enters front-up [CR#712.14].
        let view = state.layers();
        assert_eq!(view.power(delver), Some(1), "front-up shows front power 1");
        assert_eq!(
            view.toughness(delver),
            Some(1),
            "front-up shows front toughness 1"
        );
        assert!(
            !has_keyword(&view, delver, &KeywordAbility::Trample),
            "the front face carries no Trample"
        );

        // 2. Apply an until-end-of-turn +2/+2, then transform via the REAL
        // Action::Transform path — not a synthetic `side` write.
        state.continuous.push(ContinuousEffect {
            timestamp: Timestamp(1_000),
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![delver]),
            changes: vec![
                Modification::Power(NumericOp::Up(Count::Literal(2))),
                Modification::Toughness(NumericOp::Up(Count::Literal(2))),
            ],
            duration: Duration::FixedUntil(TurnMarker::EndOfTurn),
            rows: vec![],
            origin: None,
            is_cda: false,
        });
        assert_eq!(
            state.layers().power(delver),
            Some(3),
            "1/1 front + 2/2 UEOT = 3/3 pre-transform"
        );

        let frame = crate::test_support::frame_src(&state, delver);
        state.run_effect(
            Instruction::Act(Action::Transform(Reference::Reg(deckmaste_core::RefId(0)))),
            &frame,
        );
        // Break the instant the flip lands — NOT on `agenda.is_empty()`,
        // which would over-drain past this one-shot effect and into
        // `empty_game()`'s ambient turn-structure cascade (eventually
        // opening an unanswered `Priority` decision that then blocks every
        // later `step()` in this test).
        for _ in 0..30 {
            if state.objects.obj(delver).side == Side::Back {
                break;
            }
            let _ = state.step();
        }

        assert_eq!(
            state.objects.obj(delver).side,
            Side::Back,
            "the permanent actually flipped [CR#701.27a]"
        );
        assert!(
            state.objects.get(delver).is_some(),
            "transform preserves the SAME ObjectId — no remint [CR#712.18]"
        );

        let view = state.layers();
        assert_eq!(
            view.power(delver),
            Some(5),
            "back base power 3 + 2/2 UEOT (survived the flip) = 5 [CR#712.18]"
        );
        assert_eq!(
            view.toughness(delver),
            Some(4),
            "back base toughness 2 + 2/2 UEOT (survived the flip) = 4 [CR#712.18]"
        );
        assert!(
            has_keyword(&view, delver, &KeywordAbility::Trample),
            "the back face's Trample is now visible through the layered view"
        );

        // 3. A genuine zone change resets to front-up [CR#712.14]: dies to
        // the graveyard (a real remint, [CR#400.7]) then returns to the
        // battlefield as yet another new object.
        state.run_effect(
            Instruction::Act(Action::destroy(Reference::Reg(deckmaste_core::RefId(0)))),
            &frame,
        );
        for _ in 0..30 {
            if state.objects.get(delver).is_none() {
                break;
            }
            let _ = state.step();
        }
        assert!(
            state.objects.get(delver).is_none(),
            "the back-up object is gone — destroyed into the graveyard [CR#400.7]"
        );
        let in_graveyard = *state.zones.graveyards[0]
            .last()
            .expect("the destroyed DFC reminted into P0's graveyard");

        let gy_frame = crate::test_support::frame_src(&state, in_graveyard);
        state.run_effect(
            Instruction::Act(Action::move_to(
                Reference::Reg(deckmaste_core::RefId(0)),
                Zone::Battlefield,
            )),
            &gy_frame,
        );
        for _ in 0..30 {
            if !state.zones.battlefield.is_empty() {
                break;
            }
            let _ = state.step();
        }

        let reentered = *state
            .zones
            .battlefield
            .last()
            .expect("the DFC returned to the battlefield as a new object");
        assert_ne!(
            reentered, delver,
            "returning from the graveyard mints a NEW ObjectId [CR#400.7]"
        );
        assert_eq!(
            state.objects.obj(reentered).side,
            Side::Front,
            "re-entering the battlefield resets to front-up [CR#712.14]"
        );
        let view = state.layers();
        assert_eq!(
            view.power(reentered),
            Some(1),
            "back on the battlefield, front-up shows front power 1 again"
        );
        assert_eq!(view.toughness(reentered), Some(1), "…and front toughness 1");
        assert!(
            !has_keyword(&view, reentered, &KeywordAbility::Trample),
            "the reset front face carries no Trample"
        );
    }

    /// Delver of Secrets' REAL front ability ([CR#701.27a]) driven end-to-end
    /// through the whole conditional resolve path — the behavior the canon
    /// `Delver of Secrets.ron` encodes. The effect mirrors the blessed Idris
    /// model (Cards.idr `card_DelverOfSecrets`):
    /// `If(Matches(Single(TopOfLibrary 1), Or([Instant, Sorcery])),
    ///     May(Sequentially([Reveal(Single(TopOfLibrary 1)),
    /// Transform(This)])))` — the top card is read DIRECTLY
    /// (`Single(TopOfLibrary(1))`), no binder, so this also proves the
    /// engine resolves that reference in BOTH `Matches` and `Reveal`.
    /// Unlike `delver_transforms_end_to_end` (a bare `Act(Transform)`), the
    /// top card's card TYPE decides whether the optional transform even
    /// offers:
    ///  1. an INSTANT on top → the condition holds, the reveal is accepted, and
    ///     ~ transforms to the 3/2 back ([CR#712.18]);
    ///  2. a CREATURE on top → the condition fails, nothing offers, and ~ stays
    ///     the 1/1 front.
    #[test]
    fn delver_upkeep_reveals_instant_and_transforms() {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::DoubleFacedLayout;
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::If;
        use deckmaste_core::Instruction;
        use deckmaste_core::May;
        use deckmaste_core::Selection;
        use deckmaste_core::StatValue;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::object::Side;
        use crate::step::StepOutcome;

        // Delver as a real DoubleFaced permanent (front 1/1, back 3/2 Flying) so
        // `Transform(This)` has a back face to flip to. The ability under test
        // is supplied to `run_effect` directly — the trigger-fire path is
        // covered by `delver_transforms_end_to_end` and the StepBegins tests.
        fn delver() -> Card {
            Card::DoubleFaced {
                layout: DoubleFacedLayout::Transforming,
                front: CardFace {
                    name: "Delver of Secrets".into(),
                    types: vec![Type::Creature.def()],
                    power: Some(StatValue::Number(1)),
                    toughness: Some(StatValue::Number(1)),
                    ..CardFace::default()
                },
                back: CardFace {
                    name: "Insectile Aberration".into(),
                    types: vec![Type::Creature.def()],
                    power: Some(StatValue::Number(3)),
                    toughness: Some(StatValue::Number(2)),
                    ..CardFace::default()
                },
            }
        }

        // Mint a card of type `ty` onto the TOP (front) of P0's library.
        fn put_on_top(state: &mut GameState, name: &str, ty: Type) {
            let cid = state.cards.push(
                Arc::new(Card::Normal(CardFace {
                    name: name.into(),
                    types: vec![ty.def()],
                    ..CardFace::default()
                })),
                PlayerId(0),
            );
            let id = state
                .objects
                .mint(ObjectSource::Card(cid), PlayerId(0), Some(Zone::Library));
            state.zones.libraries[0].push_front(id);
        }

        // `Single(TopOfLibrary(1))` — the sole top card of your library, read
        // directly (both by the condition and by the reveal), no binder.
        let top_ref = || {
            Reference::Single(Arc::new(Selection::TopOfLibrary {
                count: Count::Literal(1),
                whose: Reference::Reg(deckmaste_core::RefId(1)),
            }))
        };
        let upkeep_effect = || {
            Instruction::If(If {
                condition: Condition::Matches(
                    top_ref(),
                    Predicate::Or(
                        vec![
                            Predicate::Characteristic(CharacteristicPredicate::Type(
                                Type::Instant.into(),
                            )),
                            Predicate::Characteristic(CharacteristicPredicate::Type(
                                Type::Sorcery.into(),
                            )),
                        ]
                        .into(),
                    ),
                ),
                then: Arc::new(Instruction::May(May {
                    who: Reference::Reg(deckmaste_core::RefId(1)),
                    effect: Arc::new(Instruction::Sequentially(
                        vec![
                            Instruction::Act(Action::Reveal {
                                what: top_ref(),
                                to: None,
                            }),
                            Instruction::Act(Action::Transform(Reference::Reg(
                                deckmaste_core::RefId(0),
                            ))),
                        ]
                        .into(),
                    )),
                    if_did: None,
                    if_not: None,
                })),
                otherwise: None,
            })
        };

        // 1. Instant on top → the reveal is accepted → ~ transforms.
        {
            let mut state = empty_game();
            let d = put_synthetic_on_field(&mut state, delver(), PlayerId(0));
            put_on_top(&mut state, "Opt", Type::Instant);
            assert_eq!(
                state.objects.obj(d).side,
                Side::Front,
                "enters front-up [CR#712.14]"
            );
            assert_eq!(state.layers().power(d), Some(1), "the front face is 1/1");

            state.run_effect(upkeep_effect(), &crate::test_support::frame_src(&state, d));
            // Drive resolution, saying "yes" to the optional reveal; break the
            // instant the flip lands (NOT on an empty agenda — that would drain
            // into `empty_game`'s ambient turn cascade, [CR#712.18]).
            for _ in 0..40 {
                if state.objects.obj(d).side == Side::Back {
                    break;
                }
                if let StepOutcome::NeedsDecision(DecisionPointKind::YesNo(_)) = state.step() {
                    state.submit_decision(Decision::Answer(true)).unwrap();
                }
            }
            assert_eq!(
                state.objects.obj(d).side,
                Side::Back,
                "an instant on top → the condition holds, reveal accepted → ~ transforms \
                 [CR#701.27a]"
            );
            assert_eq!(
                state.layers().power(d),
                Some(3),
                "the back face reads 3/2 through the layered view [CR#712.18]"
            );
        }

        // 2. Creature on top → the condition fails → nothing offers, ~ stays
        //    front. A YesNo surfacing here would mean the type gate wrongly
        //    passed.
        {
            let mut state = empty_game();
            let d = put_synthetic_on_field(&mut state, delver(), PlayerId(0));
            put_on_top(&mut state, "Grizzly Bears", Type::Creature);

            state.run_effect(upkeep_effect(), &crate::test_support::frame_src(&state, d));
            for _ in 0..40 {
                match state.step() {
                    StepOutcome::NeedsDecision(DecisionPointKind::YesNo(_)) => {
                        panic!("a non-instant/sorcery top card must NOT offer a may-reveal");
                    }
                    // Any other decision is `empty_game`'s ambient turn cascade
                    // (priority, etc.) — the Delver effect has already drained.
                    StepOutcome::NeedsDecision(_) | StepOutcome::GameOver(_) => break,
                    StepOutcome::Progress(_) => {}
                }
            }
            assert_eq!(
                state.objects.obj(d).side,
                Side::Front,
                "a creature on top → the condition fails → ~ stays the 1/1 front [CR#701.27a]"
            );
            assert_eq!(state.layers().power(d), Some(1), "still the 1/1 front");
        }
    }

    // -------------------------------------------------------------------------
    // StepBegins — step/phase-entry triggers ([CR#603.2b])
    // -------------------------------------------------------------------------

    /// A synthetic Goblin Rabblemaster: a red Goblin creature whose sole
    /// ability is "At the beginning of combat on your turn, create a 1/1 red
    /// Goblin creature token with haste" — i.e.
    /// `Triggered(event: StepBegins(at: Combat(BeginningOfCombat), whose:
    /// Your), effect: Create(1, Token(1/1 red Goblin)))`.
    fn rabblemaster() -> deckmaste_card::Card {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_core::Ability;
        use deckmaste_core::Action;
        use deckmaste_core::Color;
        use deckmaste_core::CombatStep;
        use deckmaste_core::Count;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Instruction;
        use deckmaste_core::PhaseStep;
        use deckmaste_core::Reference;
        use deckmaste_core::StatValue;
        use deckmaste_core::Token;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::WhoseTurn;

        let goblin_token = Token {
            name: None,
            color_indicator: vec![Color::Red].into(),
            supertypes: vec![].into(),
            types: vec![Type::Creature.def()].into(),
            subtypes: vec![].into(),
            abilities: vec![].into(),
            power: Some(StatValue::Number(1)),
            toughness: Some(StatValue::Number(1)),
        };
        Card::Normal(CardFace {
            name: "Rabblemaster".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                targets: [].into(),
                from: None,
                event: EventFilter::StepBegins {
                    at: PhaseStep::Combat(CombatStep::BeginningOfCombat),
                    whose: WhoseTurn::Your,
                },
                condition: None,
                limits: Vec::new().into(),
                effect: Instruction::Act(Action::Create {
                    agent: Reference::Reg(deckmaste_core::RefId(1)),
                    count: Count::Literal(1),
                    token: goblin_token.into(),
                    riders: vec![].into(),
                })
                .into(),
            })],
            ..CardFace::default()
        })
    }

    /// Force a synthetic in-Rust card onto the battlefield under `controller`.
    fn put_synthetic_on_field(
        state: &mut GameState,
        card: deckmaste_card::Card,
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
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// [CR#603.2b]: when the beginning-of-combat step begins, a
    /// "at the beginning of combat on your turn" ability fires — the
    /// `StepBegan` fact must reach the scan, and `WhoseTurn::Your` must pass
    /// on the controller's own turn.
    #[test]
    fn beginning_of_combat_your_turn_fires_on_controllers_turn() {
        use deckmaste_core::CombatStep;
        use deckmaste_core::PhaseStep;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        let rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Combat(CombatStep::BeginningOfCombat),
        )));

        let fired: Vec<&WorkItem> = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
                )
            })
            .collect();
        assert_eq!(
            fired.len(),
            1,
            "beginning-of-combat trigger must fire on the controller's turn"
        );
        let WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(TriggerFired {
            source,
            ..
        }))) = fired[0]
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
        use deckmaste_core::PhaseStep;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        // The opponent (P1) is the active player; the Rabblemaster's controller
        // is P0, so "on your turn" must not be satisfied.
        state.turn.active_player = PlayerId(1);
        let _rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Combat(CombatStep::BeginningOfCombat),
        )));

        let fired = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
                )
            })
            .count();
        assert_eq!(
            fired, 0,
            "a `Your`-scoped trigger must stay silent on an opponent's turn"
        );
    }

    /// A frameless "draw a card" delayed/reflexive triggered-ability body that
    /// fires on `event` — the shape both the delayed and reflexive tests
    /// register.
    fn draw_on(event: EventFilter) -> deckmaste_core::TriggeredAbility {
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::Reference;

        deckmaste_core::TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event,
            condition: None,
            limits: Vec::new().into(),
            effect: Instruction::draw(Reference::Reg(deckmaste_core::RefId(1)), Count::Literal(1))
                .into(),
        }
    }

    /// How many `TriggerFired` emits the scan/resolution left on the agenda.
    fn total_fired(state: &GameState) -> usize {
        use crate::agenda::WorkItem;
        use crate::event::Occurrence;
        state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
                )
            })
            .count()
    }

    /// [CR#603.7,603.7b]: a delayed triggered ability in the registry fires the
    /// NEXT time its event occurs — exactly once — and the scan then removes
    /// it.
    #[test]
    fn delayed_trigger_fires_once_on_the_later_event_then_is_gone() {
        use deckmaste_core::EndingStep;
        use deckmaste_core::EventFilter;
        use deckmaste_core::PhaseStep;
        use deckmaste_core::WhoseTurn;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        // "At the beginning of the next end step, draw a card" — created at a
        // prior resolution, so it lives in the registry, not on any permanent.
        state.delayed_triggers.push(super::CreatedTrigger {
            source: ObjectSource::Player(PlayerId(0)),
            controller: PlayerId(0),
            ability: Arc::new(draw_on(EventFilter::StepBegins {
                at: PhaseStep::Ending(EndingStep::End),
                whose: WhoseTurn::EachPlayers,
            })),
            bindings: super::TriggerBindings::default(),
        });

        // A different step onset does NOT fire it (and does not consume it).
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Beginning(deckmaste_core::BeginningStep::Upkeep),
        )));
        assert_eq!(
            total_fired(&state),
            0,
            "wrong step must not fire a delayed trigger"
        );
        assert_eq!(
            state.delayed_triggers.len(),
            1,
            "an unmatched delayed trigger stays registered"
        );

        // The end step begins: it fires exactly once, carrying its body by
        // value.
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Ending(EndingStep::End),
        )));
        assert_eq!(
            total_fired(&state),
            1,
            "the delayed trigger fires on its event"
        );
        assert!(
            state.delayed_triggers.is_empty(),
            "[CR#603.7b]: a delayed trigger is removed once it fires"
        );
        let fired = state
            .agenda
            .iter()
            .find_map(|w| match w {
                WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(TriggerFired {
                    created,
                    ..
                }))) => Some(created),
                _ => None,
            })
            .expect("a TriggerFired emit");
        assert!(
            fired.is_some(),
            "a delayed trigger carries its body by value"
        );

        // A SECOND end step must not re-fire it — the registry is empty.
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Ending(EndingStep::End),
        )));
        assert_eq!(
            total_fired(&state),
            1,
            "[CR#603.7b]: a delayed trigger fires only once — the later event is a no-op"
        );
    }

    /// [CR#603.12]: a reflexive triggered ability is checked immediately when
    /// created, against events that occurred EARLIER in the same resolution —
    /// it fires on the spot and is never persisted.
    #[test]
    fn reflexive_trigger_fires_on_the_same_resolution_event_then_is_gone() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::Instruction;
        use deckmaste_core::Predicate;

        let mut state = empty_game();
        let src = put_synthetic_on_field(&mut state, upkeep_trigger_from(None), PlayerId(0));
        let frame = state.frame(src, PlayerId(0));
        let ability = draw_on(EventFilter::LifeGained {
            who: Predicate::any(),
            amount: None,
        });

        // No earlier event in this resolution yet → the "when you do" window is
        // empty, so the reflexive trigger does NOT fire (it never waits for a
        // future event — [CR#603.12]).
        state.run_effect(Instruction::Reflexive(Arc::new(ability.clone())), &frame);
        assert_eq!(
            total_fired(&state),
            0,
            "a reflexive trigger fires only on an EARLIER event"
        );
        assert!(
            state.delayed_triggers.is_empty(),
            "[CR#603.12]: a reflexive trigger is never added to the delayed registry"
        );

        // Now an event occurs earlier in the resolution, THEN the reflexive
        // ability is created: it fires immediately on that event.
        state
            .resolution_events
            .push(GameEvent::LifeGained(LifeGained {
                player: PlayerId(0),
                amount: 3,
                cause: None,
            }));
        state.run_effect(Instruction::Reflexive(Arc::new(ability)), &frame);
        assert_eq!(
            total_fired(&state),
            1,
            "[CR#603.12]: a reflexive trigger fires on the same-resolution event"
        );
        assert!(
            state.delayed_triggers.is_empty(),
            "[CR#603.12]: a reflexive trigger is checked immediately, never persisted"
        );
    }

    /// [CR#603.7,603.12]: a DELAYED trigger created during a resolution is
    /// reflexive-UNIFIED. When its event has NOT yet occurred it registers for
    /// the next future occurrence (the ordinary delayed rule, [CR#603.7b]);
    /// when its event ALREADY occurred earlier in the SAME resolution — the
    /// exile a `With(Produce(...))` performed before installing the delayed
    /// trigger (madness's "when a card is exiled this way") — it fires
    /// immediately on that event and is NOT left in the registry. This is the
    /// behavior the produced-exile Madness delayed trigger relies on.
    #[test]
    fn delayed_trigger_installed_after_its_event_fires_reflexively() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::Instruction;
        use deckmaste_core::Predicate;

        let ability = draw_on(EventFilter::LifeGained {
            who: Predicate::any(),
            amount: None,
        });

        // No matching earlier event in this resolution → the delayed trigger
        // registers for a FUTURE occurrence and fires nothing now.
        let mut state = empty_game();
        let src = put_synthetic_on_field(&mut state, upkeep_trigger_from(None), PlayerId(0));
        let frame = state.frame(src, PlayerId(0));
        state.run_effect(Instruction::Delayed(Arc::new(ability.clone())), &frame);
        assert_eq!(
            total_fired(&state),
            0,
            "no earlier matching event: the delayed trigger fires nothing now"
        );
        assert_eq!(
            state.delayed_triggers.len(),
            1,
            "[CR#603.7b]: with no earlier event it registers for the future"
        );

        // The event already occurred earlier in this resolution, THEN the
        // delayed trigger is created: it fires ON THE SPOT (reflexive
        // unification) and is NOT registered.
        let mut state = empty_game();
        let src = put_synthetic_on_field(&mut state, upkeep_trigger_from(None), PlayerId(0));
        let frame = state.frame(src, PlayerId(0));
        state
            .resolution_events
            .push(GameEvent::LifeGained(LifeGained {
                player: PlayerId(0),
                amount: 3,
                cause: None,
            }));
        state.run_effect(Instruction::Delayed(Arc::new(ability)), &frame);
        assert_eq!(
            total_fired(&state),
            1,
            "[CR#603.12]: the event already occurred this resolution — the delayed trigger fires now"
        );
        assert!(
            state.delayed_triggers.is_empty(),
            "[CR#603.7b,603.12]: a reflexively-fired delayed trigger is not left in the registry"
        );
    }

    /// A synthetic creature whose sole ability is an "at the beginning of your
    /// upkeep, draw a card" trigger that FUNCTIONS from the given zone
    /// (`from`).
    fn upkeep_trigger_from(from: Option<Zone>) -> deckmaste_card::Card {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_core::Ability;
        use deckmaste_core::BeginningStep;
        use deckmaste_core::Count;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Instruction;
        use deckmaste_core::PhaseStep;
        use deckmaste_core::Reference;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::WhoseTurn;

        Card::Normal(CardFace {
            name: "Graveyard Echo".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                targets: [].into(),
                from,
                event: EventFilter::StepBegins {
                    at: PhaseStep::Beginning(BeginningStep::Upkeep),
                    whose: WhoseTurn::Your,
                },
                condition: None,
                limits: Vec::new().into(),
                effect: Instruction::draw(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Count::Literal(1),
                )
                .into(),
            })],
            ..CardFace::default()
        })
    }

    /// Put a synthetic in-Rust card into `controller`'s graveyard.
    fn put_synthetic_in_graveyard(
        state: &mut GameState,
        card: deckmaste_card::Card,
        controller: PlayerId,
    ) -> ObjectId {
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Graveyard),
        );
        state.zones.graveyards[controller.index()].push(id);
        id
    }

    /// [CR#113.6,113.6b]: a triggered ability functions only from its `from`
    /// zone. A `from: Graveyard` upkeep trigger fires while its source is in
    /// the graveyard; the same trigger left on the battlefield, and a
    /// battlefield- default trigger sitting in the graveyard, both stay
    /// silent.
    #[test]
    fn graveyard_trigger_fires_only_from_its_function_zone() {
        use deckmaste_core::BeginningStep;
        use deckmaste_core::PhaseStep;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);

        // The one that should fire: graveyard-functioning, in the graveyard.
        let gy = put_synthetic_in_graveyard(
            &mut state,
            upkeep_trigger_from(Some(Zone::Graveyard)),
            PlayerId(0),
        );
        // A battlefield-default trigger in the graveyard: out of its zone.
        let _bf_default_in_gy =
            put_synthetic_in_graveyard(&mut state, upkeep_trigger_from(None), PlayerId(0));
        // A graveyard-functioning trigger on the battlefield: out of its zone.
        let _gy_func_on_bf = put_synthetic_on_field(
            &mut state,
            upkeep_trigger_from(Some(Zone::Graveyard)),
            PlayerId(0),
        );

        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Beginning(BeginningStep::Upkeep),
        )));

        let fired: Vec<&WorkItem> = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
                )
            })
            .collect();
        assert_eq!(
            fired.len(),
            1,
            "only the graveyard-functioning trigger IN the graveyard fires"
        );
        let WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(TriggerFired {
            source,
            ..
        }))) = fired[0]
        else {
            unreachable!()
        };
        assert_eq!(
            *source,
            state.objects.obj(gy).source,
            "the firing trigger is the in-graveyard graveyard-functioning one"
        );
    }

    /// A different step's `StepBegan` (upkeep) does not match a
    /// beginning-of-combat trigger — the phase must match exactly
    /// ([CR#603.2b]).
    #[test]
    fn beginning_of_combat_trigger_ignores_other_steps() {
        use deckmaste_core::BeginningStep;
        use deckmaste_core::CombatStep;
        use deckmaste_core::PhaseStep;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        let _rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        // The upkeep step begins, not combat.
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Beginning(BeginningStep::Upkeep),
        )));
        assert_eq!(
            state
                .agenda
                .iter()
                .filter(|w| matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
                ))
                .count(),
            0,
            "a beginning-of-combat trigger must ignore the upkeep step"
        );
        // Sanity: the same trigger DOES fire on the matching step.
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Combat(CombatStep::BeginningOfCombat),
        )));
        assert_eq!(
            state
                .agenda
                .iter()
                .filter(|w| matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                        TriggerFired { .. }
                    )))
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
        use deckmaste_core::ObjectClass;
        use deckmaste_core::PhaseStep;

        use crate::agenda::WorkItem;
        use crate::decide::Action;
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::step::StepOutcome;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        state.turn.current = PhaseStep::PrecombatMain;
        let rabble = put_synthetic_on_field(&mut state, rabblemaster(), PlayerId(0));

        // Schedule the beginning-of-combat step directly, then step the engine,
        // passing priority whenever it asks so the placed trigger resolves.
        state.schedule_front(vec![WorkItem::BeginStep(PhaseStep::Combat(
            CombatStep::BeginningOfCombat,
        ))]);
        let goblin_exists = |state: &GameState| {
            state.zones.battlefield.iter().any(|&id| {
                id != rabble && crate::target::is_object_class(state, id, ObjectClass::Token)
            })
        };
        for _ in 0..200 {
            if goblin_exists(&state) {
                break;
            }
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(DecisionPointKind::Priority(
                    crate::decide::pending::Priority { .. },
                )) => {
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
        assert!(
            crate::target::is_object_class(&state, goblins[0], ObjectClass::Token),
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
            created: None,
            controller,
            bindings: super::TriggerBindings::default(),
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

    /// A transforming DFC on P0's battlefield showing its BACK face. Front: a
    /// vanilla 2/2 with ZERO abilities; back: a 3/3 whose SOLE printed ability
    /// is an "at the beginning of your upkeep, draw a card" trigger the
    /// front lacks — front printed len 0, back 1, the unequal shape that
    /// moves the back trigger into section 1 (`idx < printed_len`) of
    /// `derived_abilities_of`. Active player = P0 so `WhoseTurn::Your`
    /// matches. Returns the object id and the back trigger body.
    fn back_up_dfc_on_field_upkeep_draw() -> (GameState, ObjectId, deckmaste_core::TriggeredAbility)
    {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::DoubleFacedLayout;
        use deckmaste_core::Ability;
        use deckmaste_core::BeginningStep;
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::PhaseStep;
        use deckmaste_core::StatValue;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::WhoseTurn;

        use crate::object::Side;

        let back_trigger = TriggeredAbility {
            ability_word: None,
            where_x: None,
            targets: [].into(),
            from: None,
            event: EventFilter::StepBegins {
                at: PhaseStep::Beginning(BeginningStep::Upkeep),
                whose: WhoseTurn::Your,
            },
            condition: None,
            limits: Vec::new().into(),
            effect: Instruction::draw(Reference::Reg(deckmaste_core::RefId(1)), Count::Literal(1))
                .into(),
        };
        let front = CardFace {
            name: "Front Vanilla".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            ..CardFace::default()
        };
        let back_ability = Ability::triggered(back_trigger);
        let back_trigger = back_ability
            .as_triggered()
            .expect("the normalized ability remains triggered")
            .clone();
        let back = CardFace {
            name: "Back Upkeep Drawer".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(3)),
            toughness: Some(StatValue::Number(3)),
            abilities: vec![back_ability],
            ..CardFace::default()
        };
        let card = Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front,
            back,
        };
        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        let id = put_synthetic_on_field(&mut state, card, PlayerId(0));
        state.objects.obj_mut(id).side = Side::Back;
        (state, id, back_trigger)
    }

    /// [Task 5b regression][CR#712.8e]: a back-up permanent's section-1 printed
    /// trigger is captured BY VALUE at fire time (`created: Some`), NOT by the
    /// `(source, idx)` index channel. `abilities_of_source` now sources section
    /// 1 from the back face, so the index re-read at resolution would need
    /// the live battlefield object; capturing the body now makes the
    /// trigger independent of the (possibly-gone) source.
    #[test]
    fn back_up_permanent_captures_section1_trigger_by_value() {
        use deckmaste_core::BeginningStep;
        use deckmaste_core::PhaseStep;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let (mut state, _id, back_body) = back_up_dfc_on_field_upkeep_draw();
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Beginning(BeginningStep::Upkeep),
        )));

        let created = state
            .agenda
            .iter()
            .find_map(|w| match w {
                WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(tf))) => {
                    Some(tf.created.clone())
                }
                _ => None,
            })
            .expect("the back-up upkeep trigger fired");
        assert_eq!(
            created.as_deref(),
            Some(&back_body),
            "a back-up permanent's section-1 printed trigger must be captured by \
             value (created=Some) so the resolution re-read never indexes the \
             possibly-gone source"
        );
    }

    /// [Task 5b regression][CR#603.3,712.8e]: the scenario the by-value capture
    /// exists for. A back-up permanent fires a section-1 printed trigger, then
    /// LEAVES the battlefield (destroyed/sacrificed/bounced) before the trigger
    /// resolves — a trigger resolves independently of its source ([CR#603.3]).
    /// Resolution must NOT panic (no by-index re-read against the gone source's
    /// front face) and must resolve the BACK body. Before the fix the trigger
    /// was stored by index and this panicked (index-out-of-bounds on the
    /// shorter front list) at `resolve/mod.rs`.
    #[test]
    fn back_up_trigger_survives_source_leaving_before_resolution() {
        use deckmaste_core::BeginningStep;
        use deckmaste_core::PhaseStep;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;

        let (mut state, id, back_body) = back_up_dfc_on_field_upkeep_draw();
        state.scan_triggers(&Occurrence::single(GameEvent::StepBegan(
            PhaseStep::Beginning(BeginningStep::Upkeep),
        )));

        // Note the fired trigger exactly as the real pipeline would — carrying
        // the `created` the scan produced (Some, post-fix).
        let (source, ability, created, bindings) = state
            .agenda
            .iter()
            .find_map(|w| match w {
                WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(tf))) => Some((
                    tf.source,
                    tf.ability as usize,
                    tf.created.clone(),
                    (*tf.bindings).clone(),
                )),
                _ => None,
            })
            .expect("the back-up upkeep trigger fired");
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability,
            created,
            controller: PlayerId(0),
            bindings,
        });
        let _ = state.place_triggers();
        assert_eq!(state.stack.len(), 1, "the non-targeting trigger placed");
        let stack_id = state.stack[0].id;

        // The source LEAVES the battlefield before resolution.
        state.zones.battlefield.retain(|&x| x != id);
        state.zones.graveyards[0].push(id);
        state.objects.obj_mut(id).zone = Some(Zone::Graveyard);

        // Must not panic, and must resolve the BACK body (schedule its draw).
        state.resolve_object(stack_id);
        assert!(
            state.agenda.iter().any(|w| matches!(
                w,
                WorkItem::RunEffect { effect, .. }
                    if back_body.effect.body.iter().any(|body| body == effect.as_ref())
            )),
            "the trigger resolved with the back face's draw body, not a fizzle or \
             the front face"
        );
    }

    /// A targeting noted trigger surfaces a `ChooseTargets` at placement
    /// ([CR#603.3d]): the stack id is minted and staged in `placing_trigger`,
    /// and nothing is on the stack until the target is chosen. (The
    /// no-legal-target drop, [CR#603.3c], can't be hit here — "any target"
    /// always admits the two player proxies.)
    #[test]
    fn targeting_trigger_surfaces_choose_targets_at_placement() {
        use crate::decide::DecisionPointKind;

        let (mut state, gob) = fixture_on_field("Footlight Fiend");
        let source = state.objects.obj(gob).source;
        let controller = state.objects.obj(gob).controller;
        // Note the dies-trigger (ability 0, targets [AnyTarget]).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            created: None,
            controller,
            bindings: super::TriggerBindings::default(),
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "a target choice surfaces instead of an immediate placement"
        );
        let Some(DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
            player,
            legal,
            ..
        })) = &state.pending
        else {
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
        use crate::decide::DecisionPointKind;

        // A `Footlight Fiend` dies-trigger (P0, "any target").
        let (mut state, fiend) = fixture_on_field("Footlight Fiend");
        let source = state.objects.obj(fiend).source;
        let controller = state.objects.obj(fiend).controller; // P0
        // P1 (an opponent) controls a hexproof `Gladecover Scout`.
        let scout = put_on_field(&mut state, "Gladecover Scout", PlayerId(1));

        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            created: None,
            controller,
            bindings: super::TriggerBindings::default(),
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "a target choice surfaces instead of an immediate placement"
        );
        let Some(DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
            legal,
            ..
        })) = &state.pending
        else {
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
        use crate::decide::DecisionPointKind;

        let (mut state, fiend) = fixture_on_field("Footlight Fiend");
        let source = state.objects.obj(fiend).source;
        let controller = state.objects.obj(fiend).controller; // P0
        // The SAME player (P0) controls the hexproof `Gladecover Scout`.
        let scout = put_on_field(&mut state, "Gladecover Scout", PlayerId(0));

        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            created: None,
            controller,
            bindings: super::TriggerBindings::default(),
        });

        state.place_triggers();
        let Some(DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
            legal,
            ..
        })) = &state.pending
        else {
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
        // No curated canon card carries a creature-ONLY-target triggered
        // ability (canon's targeting triggers all use "any target", which
        // always admits the player proxies, so the drop can't be reached with
        // them). This synthesizes the minimal card that does — engine-path
        // scaffolding to exercise [CR#603.3c], not a corpus mock.
        let card = canon()
            .card_from_str(
                r#"Normal(
                    name: "Test Targeted Pinger",
                    mana_cost: [Red],
                    types: [Artifact],
                    abilities: [
                        Triggered(
                            event: ThisEnters,
                            effect: Targeted(targets: [TargetOne(Creature)], effect: DealDamage(This, 1, Target(0))),
                        ),
                    ],
                )"#,
            )
            .unwrap()
            .core;

        // The pinger (a non-creature, so not itself a legal "target creature")
        // belongs to P0; the only creature on the board is P1's hexproof Scout.
        let forest = Arc::new(builtin().card("Forest").unwrap().core);
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
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
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
        let source = state.objects.obj(pinger).source;
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            created: None,
            controller: PlayerId(0),
            bindings: super::TriggerBindings::default(),
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
        use crate::decide::DecisionPointKind;

        // Two dies-watchers under player 0 (non-targeting `LoseLife`).
        let (mut state, w0) = fixture_on_field("Moonlit Wake");
        let source = state.objects.obj(w0).source;
        let bindings = || super::TriggerBindings::default();
        // Two notes from the same controller (as if two creatures died at
        // once).
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            created: None,
            controller: PlayerId(0),
            bindings: bindings(),
        });
        state.pending_triggers.push(super::NotedTrigger {
            source,
            ability: 0,
            created: None,
            controller: PlayerId(0),
            bindings: bindings(),
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 0 },
            "ordering is needed first — nothing placed yet"
        );
        let Some(DecisionPointKind::OrderTriggers(crate::decide::pending::OrderTriggers {
            player,
            triggers,
        })) = &state.pending
        else {
            panic!("expected OrderTriggers, got {:?}", state.pending);
        };
        assert_eq!(*player, PlayerId(0));
        assert_eq!(triggers.len(), 2, "both of player 0's triggers offered");

        // A non-permutation is rejected.
        let err = state
            .submit_decision(Decision::Order(vec![0, 0]))
            .unwrap_err();
        assert!(err.to_string().contains("permutation"));
        let err = state.submit_decision(Decision::Order(vec![5])).unwrap_err();
        assert!(err.to_string().contains("permutation"));
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

    fn snap_cf(c: CharacteristicPredicate) -> Predicate {
        Predicate::Characteristic(c)
    }

    /// `Named` over a snapshot reads the printed face name ([CR#201]).
    #[test]
    fn snapshot_named_matches_printed_name() {
        let (state, snap, w) = bear_snapshot();
        assert!(state.filter_matches_snapshot(
            &snap_cf(CharacteristicPredicate::Named("Grizzly Bears".into())),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicPredicate::Named("Forest".into())),
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
            &snap_cf(CharacteristicPredicate::ColorIs(Color::Green)),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicPredicate::ColorIs(Color::Red)),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicPredicate::Multicolored),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicPredicate::Colorless),
            &snap,
            w
        ));
        // A Grizzly Bears is not a Basic.
        assert!(!state.filter_matches_snapshot(
            &snap_cf(CharacteristicPredicate::Supertype(Supertype::Basic)),
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
        let f = |s, c, n| snap_cf(CharacteristicPredicate::Stat(s, c, Count::Literal(n)));
        assert!(state.filter_matches_snapshot(&f(Stat::Power, Cmp::Eq, 2), &snap, w));
        assert!(state.filter_matches_snapshot(&f(Stat::Toughness, Cmp::AtLeast, 2), &snap, w));
        assert!(state.filter_matches_snapshot(&f(Stat::ManaValue, Cmp::Eq, 2), &snap, w));
        assert!(!state.filter_matches_snapshot(&f(Stat::Power, Cmp::Greater, 2), &snap, w));
    }

    /// `Status(Tapped)`/`Status(Untapped)` read the snapshot's tap flag.
    #[test]
    fn snapshot_status_reads_tap_flag() {
        use deckmaste_core::StatePredicate;
        use deckmaste_core::Status;
        let (mut state, bear) = bear_on_field();
        state.objects.obj_mut(bear).tapped = true;
        let snap = LkiSnapshot::capture(&state, bear);
        let w = state.objects.obj(bear).source;
        assert!(state.filter_matches_snapshot(
            &Predicate::State(StatePredicate::Status(Status::Tapped)),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &Predicate::State(StatePredicate::Status(Status::Untapped)),
            &snap,
            w
        ));
    }

    /// `ControlledBy`/`Owner` over a snapshot resolve the captured controller /
    /// the card owner to their LIVE player proxies; `OpponentOf`/`Controls`
    /// never match (a snapshot subject is a card, not a player).
    #[test]
    fn snapshot_relations_resolve_to_live_players() {
        use deckmaste_core::RelationPredicate;
        use deckmaste_core::StatePredicate;
        let (mut state, bear) = bear_on_field();
        let player = state.players[0].object;
        state
            .objects
            .obj_mut(player)
            .counters
            .insert("mark".into(), 1);
        let snap = LkiSnapshot::capture(&state, bear);
        let w = state.objects.obj(bear).source;
        let marked = || Arc::new(Predicate::State(StatePredicate::HasCounter("mark".into())));
        assert!(state.filter_matches_snapshot(
            &Predicate::Relation(RelationPredicate::ControlledBy(marked())),
            &snap,
            w
        ));
        assert!(state.filter_matches_snapshot(
            &Predicate::Relation(RelationPredicate::Owner(marked())),
            &snap,
            w
        ));
        assert!(!state.filter_matches_snapshot(
            &Predicate::Relation(RelationPredicate::OpponentOf(Arc::new(Predicate::Any))),
            &snap,
            w
        ));
    }

    /// [CR#301.5,303.4,603.10a]: a departed attachment's captured
    /// `attached_to` still resolves the LIVE host through
    /// `filter_matches_snapshot` — the shape a departed Aura's
    /// leaves-the-battlefield ability needs to read its last host. Mirrors
    /// `target::relation_filters_match_attachment_and_host`.
    #[test]
    fn snapshot_attached_to_resolves_the_captured_host() {
        use deckmaste_core::RelationPredicate;
        let (mut state, host) = bear_on_field();
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let attachment_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let attachment = state.objects.mint(
            ObjectSource::Card(attachment_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(attachment);
        state.objects.obj_mut(attachment).attached_to = Some(host);

        // Capture the attachment's LKI as if it just left the battlefield.
        let snap = LkiSnapshot::capture(&state, attachment);
        let w = state.objects.obj(attachment).source;

        assert!(
            state.filter_matches_snapshot(
                &Predicate::Relation(RelationPredicate::AttachedTo(Arc::new(
                    Predicate::creature()
                ))),
                &snap,
                w
            ),
            "the departed attachment's captured host is a live creature"
        );
    }

    /// The inverse relation: a departed host's snapshot still sees a LIVE
    /// object pointing its `attached_to` at the departed host's now-stale id
    /// — captured before the attach relation is cleared ([CR#704.8]).
    #[test]
    fn snapshot_attachment_reads_the_live_object_still_pointing_at_it() {
        use deckmaste_core::RelationPredicate;
        let (mut state, host) = bear_on_field();
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
        let attachment_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let attachment = state.objects.mint(
            ObjectSource::Card(attachment_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(attachment);
        state.objects.obj_mut(attachment).attached_to = Some(host);

        // Capture the HOST's LKI, as if it just left the battlefield while
        // the still-live attachment hasn't been cleaned up yet.
        let snap = LkiSnapshot::capture(&state, host);
        let w = state.objects.obj(host).source;

        assert!(
            state.filter_matches_snapshot(
                &Predicate::Relation(RelationPredicate::Attachment(Arc::new(Predicate::Any))),
                &snap,
                w
            ),
            "a live object still points its attached_to at the departed host's stale id"
        );
    }

    /// An unattached departed object reads both relations as `false`, never
    /// a panic — the fix for the seam this ticket closes.
    #[test]
    fn snapshot_attachment_relations_false_when_unattached() {
        use deckmaste_core::RelationPredicate;
        let (state, bear) = bear_on_field();
        let snap = LkiSnapshot::capture(&state, bear);
        let w = state.objects.obj(bear).source;

        assert!(
            !state.filter_matches_snapshot(
                &Predicate::Relation(RelationPredicate::AttachedTo(Arc::new(Predicate::Any))),
                &snap,
                w
            ),
            "an unattached departed object is not AttachedTo anything"
        );
        assert!(
            !state.filter_matches_snapshot(
                &Predicate::Relation(RelationPredicate::Attachment(Arc::new(Predicate::Any))),
                &snap,
                w
            ),
            "a departed object with nothing attached to it has no Attachment"
        );
    }

    // -------------------------------------------------------------------------
    // AbilityUsed history fact
    // -------------------------------------------------------------------------

    /// When a triggered ability fires (the `TriggerFired` event is applied),
    /// an `AbilityUsed` fact must be recorded in history for that same
    /// source/ability pair.
    #[test]
    fn trigger_fire_records_ability_used() {
        use deckmaste_core::Lookback;

        use crate::agenda::WorkItem;

        let (mut state, goblin) = fixture_on_field("Footlight Fiend");
        // toughness 1 → 1 damage is lethal.
        state.objects.obj_mut(goblin).set_marked_damage(1);

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
        let found = state.history.scan(Lookback::ThisGame, turn).any(|e| {
            matches!(
                e,
                GameEvent::AbilityUsed(AbilityUsed { object, ability })
                    if *object == goblin && *ability == 0
            )
        });
        assert!(
            found,
            "TriggerFired apply must record GameEvent::AbilityUsed(AbilityUsed {{ object: {goblin:?}, ability: 0 }}) in history",
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
    fn dies_watcher(limits: Vec<deckmaste_core::UseLimit>) -> deckmaste_card::Card {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_core::Ability;
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::EventFilter;
        use deckmaste_core::Instruction;
        use deckmaste_core::LifeOp;
        use deckmaste_core::Reference;
        use deckmaste_core::StatValue;
        use deckmaste_core::TriggeredAbility;
        use deckmaste_core::Zone;

        Card::Normal(CardFace {
            name: "Death Watcher".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                targets: [].into(),
                from: None,
                event: EventFilter::ZoneChange {
                    what: Predicate::creature(),
                    from: Some(Zone::Battlefield),
                    to: Some(Zone::Graveyard),
                    cause: None,
                },
                condition: None,
                limits: limits.into(),
                effect: Instruction::Act(Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Up(Count::Literal(1)),
                ))
                .into(),
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
        // Load builtin rules so the data-driven lethal-damage SBA fires when
        // `drain_sbas` drives `CheckSbas`.
        state.sba_rules = builtin().sba_rules;
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
        state.objects.obj_mut(bear).set_marked_damage(2);
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

    /// The firing event's magnitude rides `TriggerBindings.event_amount` —
    /// captured at fire time (mirroring the apply funnel's amount-carrying
    /// set) and seeding `Count::ThatMuch` for the fired ability's resolution
    /// ("whenever …, … that much").

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
    /// simultaneously is one batch of two past-form `ZoneChange` facts)
    /// triggers a once-per-turn ability only once. Both `TriggerFired`s are
    /// emitted in one scan pass; the apply-time gate dedups them.
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
    // Player-experienced facts — Draw / LoseLife / GainLife ([CR#121.1,119.3])
    // -------------------------------------------------------------------------

    /// `Drawn(who: Ref(You))` matches a draw's SUCCESS fact — the committed
    /// Library → Hand move tagged `cause: Draw` ([CR#121.2]) — for the
    /// watcher's controller ([CR#121.1]) and not another player's draw. The
    /// drawing player is the moved card's controller (the `snapshot`).
    #[test]
    fn performed_matches_draw_matches_drawn_zone_change() {
        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::Drawn {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            amount: None,
        };
        let draw_fact = |state: &GameState, card| {
            GameEvent::ZoneChange(ZoneChange {
                object: card,
                snapshot: Some(Box::new(LkiSnapshot::capture(state, card))),
                from: Some(Zone::Library),
                to: Zone::Hand,
                enters: None,
                position: None,
                face: None,
                cause: Some(crate::event::Cause {
                    verb: "Draw".into(),
                    agency: deckmaste_core::Agency::EffectInstruction,
                    agent: None,
                    payment: None,
                }),
            })
        };
        let you_card = state.objects.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(Zone::Hand),
        );
        let opp_card = state.objects.mint(
            ObjectSource::Player(PlayerId(1)),
            PlayerId(1),
            Some(Zone::Hand),
        );
        let you_draw = draw_fact(&state, you_card);
        let opp_draw = draw_fact(&state, opp_card);
        assert!(
            state.event_matches(&pattern, &you_draw, watcher_source),
            "your own draw matches by: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &opp_draw, watcher_source),
            "an opponent's draw fails by: Ref(You)"
        );
    }

    /// `LifeLost(who: Ref(You))` matches `LifeLost` for the watcher's
    /// controller — the event's PATIENT ([CR#119.3,119.9,119.10]) — and not
    /// another player's life loss.
    #[test]
    fn performed_matches_loselife_matches_life_lost() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::LifeLost {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            amount: None,
        };
        let you_lose = GameEvent::LifeLost(LifeLost {
            player: PlayerId(0),
            amount: 3,
            cause: None,
        });
        let opp_lose = GameEvent::LifeLost(LifeLost {
            player: PlayerId(1),
            amount: 3,
            cause: None,
        });
        assert!(
            state.event_matches(&pattern, &you_lose, watcher_source),
            "your own life loss matches by: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &opp_lose, watcher_source),
            "an opponent's life loss fails by: Ref(You)"
        );
    }

    /// `LifeGained(who: Ref(You))` matches `LifeGained` for the watcher's
    /// controller — the event's PATIENT ([CR#119.3,119.9]) — and not another
    /// player's life gain.
    #[test]
    fn performed_matches_gainlife_matches_life_gained() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::LifeGained {
            who: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
            amount: None,
        };
        let you_gain = GameEvent::LifeGained(LifeGained {
            player: PlayerId(0),
            amount: 4,
            cause: None,
        });
        let opp_gain = GameEvent::LifeGained(LifeGained {
            player: PlayerId(1),
            amount: 4,
            cause: None,
        });
        assert!(
            state.event_matches(&pattern, &you_gain, watcher_source),
            "your own life gain matches by: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &opp_gain, watcher_source),
            "an opponent's life gain fails by: Ref(You)"
        );
    }

    /// T5 exposure row: `Shuffled(by: Ref(You))` matches the watcher's own
    /// library shuffle ([CR#701.24a]) and not another player's — this is
    /// what makes Psychic Surgery's "whenever a player shuffles their
    /// library" authorable.
    #[test]
    fn shuffled_matches_the_shuffling_player() {
        let (state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let pattern = EventFilter::Shuffled {
            by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
        };
        assert!(
            state.event_matches(&pattern, &GameEvent::Shuffled(PlayerId(0)), watcher_source),
            "the watcher's own controller shuffling matches by: Ref(You)"
        );
        assert!(
            !state.event_matches(&pattern, &GameEvent::Shuffled(PlayerId(1)), watcher_source),
            "an opponent shuffling fails by: Ref(You)"
        );
    }

    /// T5 exposure row: `Revealed(what: …)` ∃-matches the revealed set
    /// ([CR#701.20a]) — narrows by an object among those shown, not by any
    /// performer coordinate (the revealer is unmodeled, [CR#701.20]).
    #[test]
    fn revealed_matches_a_card_in_the_revealed_set() {
        use crate::event::Revealed;

        let (mut state, bear) = bear_on_field();
        let watcher_source = state.objects.obj(bear).source;
        let other = {
            let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
            let card = state.cards.push(bears, PlayerId(0));
            state
                .objects
                .mint(ObjectSource::Card(card), PlayerId(0), None)
        };
        let pattern = EventFilter::Revealed {
            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
        };
        let reveals_bear = GameEvent::Revealed(Revealed {
            objects: vec![bear],
            to: None,
        });
        let reveals_other_only = GameEvent::Revealed(Revealed {
            objects: vec![other],
            to: None,
        });
        assert!(
            state.event_matches(&pattern, &reveals_bear, watcher_source),
            "the watcher among the revealed set matches what: Ref(This)"
        );
        assert!(
            !state.event_matches(&pattern, &reveals_other_only, watcher_source),
            "a reveal not including the watcher fails what: Ref(This)"
        );
    }

    /// A helper for the lifted-refinement scan tests: a battlefield watcher
    /// enchantment whose single triggered ability carries `event` (RON,
    /// parsed under the canon macro scope so `Dies`-style names resolve).
    fn scan_watcher(
        state: &mut GameState,
        controller: PlayerId,
        event: &str,
    ) -> (ObjectId, ObjectSource) {
        let source = format!(
            "Normal(name: \"Refinement Watcher\", types: [Enchantment], abilities: [\
                 Triggered(event: {event}, effect: ChangeLife(You, Up(1))),\
             ])"
        );
        let card = Arc::new(canon().card_from_str(&source).unwrap().core);
        let id = put_bf(state, card, controller);
        let src = state.objects.obj(id).source;
        (id, src)
    }

    /// Fires of `source` currently emitted on the agenda.
    fn fired_count(state: &GameState, source: ObjectSource) -> usize {
        state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(TriggerFired { source: s, .. })))
                        if *s == source
                )
            })
            .count()
    }

    /// The lifted `Nth` ordinal end to end ([CR#603.2g]): an
    /// `Nth(n: 2, …, within: ThisTurn)` trigger stays silent on the first
    /// matching death and fires exactly once on the second — counted off
    /// the history log, which receives each fact before its scan (the
    /// production `apply_occurrence` ordering).
    #[test]
    fn nth_trigger_fires_only_on_the_second_matching_event() {
        let (mut state, bear) = bear_on_field();
        let other = put_on_field(&mut state, "Grizzly Bears", PlayerId(0));
        let (_watcher, watcher_source) = scan_watcher(
            &mut state,
            PlayerId(0),
            "Nth(n: 2, of: ZoneChange(what: Type(Creature), from: Battlefield, to: Graveyard), \
             within: ThisTurn)",
        );
        let turn = state.turn.turn_number;

        let first = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
        state.record_history_fact(turn, None, first.clone());
        state.scan_triggers(&Occurrence::single(first));
        assert_eq!(
            fired_count(&state, watcher_source),
            0,
            "the FIRST matching death is not the second"
        );

        let second = zone_changed_event(&state, other, Zone::Battlefield, Zone::Graveyard);
        state.record_history_fact(turn, None, second.clone());
        state.scan_triggers(&Occurrence::single(second));
        assert_eq!(
            fired_count(&state, watcher_source),
            1,
            "the SECOND matching death fires the ordinal trigger once"
        );
    }

    /// The lifted `When` refinement end to end ([CR#603.4]): the condition
    /// is evaluated as the event occurs, anchored on the watcher — the
    /// active player's watcher fires on `YourTurn`, an opponent's stays
    /// silent on the same fact.
    #[test]
    fn when_trigger_gates_on_the_condition_at_event_time() {
        for (controller, expected) in [(PlayerId(0), 1), (PlayerId(1), 0)] {
            let (mut state, bear) = bear_on_field();
            let (_watcher, watcher_source) = scan_watcher(
                &mut state,
                controller,
                "When(ZoneChange(what: Type(Creature), from: Battlefield, to: Graveyard), \
                 YourTurn)",
            );
            let died = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
            state.scan_triggers(&Occurrence::single(died));
            assert_eq!(
                fired_count(&state, watcher_source),
                expected,
                "controller {controller:?}: When(…, YourTurn) gates on whose turn it is"
            );
        }
    }

    /// The lifted Where-in-snapshot slot end to end ([CR#603.10a]): a
    /// dies-trigger whose `what` carries `Where(Is(It, …))` reads the DEAD
    /// candidate through its LKI snapshot bound as `It` — the named bear
    /// fires it, a different creature does not.
    #[test]
    fn where_in_snapshot_reads_the_dead_candidate_as_it() {
        let (mut state, bear) = bear_on_field();
        let fiend = put_on_field(&mut state, "Footlight Fiend", PlayerId(0));
        let (_watcher, watcher_source) = scan_watcher(
            &mut state,
            PlayerId(0),
            "ZoneChange(what: And([Type(Creature), Where(Matches(It, Named(\"Grizzly Bears\")))]), \
             from: Battlefield, to: Graveyard)",
        );

        let fiend_died = zone_changed_event(&state, fiend, Zone::Battlefield, Zone::Graveyard);
        state.scan_triggers(&Occurrence::single(fiend_died));
        assert_eq!(
            fired_count(&state, watcher_source),
            0,
            "a non-bear death fails the Where condition over It"
        );

        let bear_died = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
        state.scan_triggers(&Occurrence::single(bear_died));
        assert_eq!(
            fired_count(&state, watcher_source),
            1,
            "the named bear's death satisfies Where(Matches(It, Named(…)))"
        );
    }

    // -------------------------------------------------------------------------
    // Emblems ([CR#114.1]) — an abilities-only object minted into the command
    // zone; its static and triggered abilities function FROM there
    // ([CR#114.4]).
    // -------------------------------------------------------------------------

    /// Parse an emblem's abilities (a RON ability list under the canon macro
    /// scope) — the `Vec<Ability>` a `GetEmblem` payload carries ([CR#114.1]).
    fn emblem_abilities(inner: &str) -> Vec<deckmaste_core::Ability> {
        let src = format!("Normal(name: \"E\", types: [], abilities: [{inner}])");
        let card = canon().card_from_str(&src).unwrap().core;
        match card {
            deckmaste_card::Card::Normal(face) => face.abilities,
            other => panic!("unexpected emblem card shape: {other:?}"),
        }
    }

    /// [CR#114.4]: an emblem's STATIC ability functions from the command zone.
    /// The Glorious-Anthem static minted as an emblem for player 0 buffs that
    /// player's battlefield creature +1/+1, exactly as the same static on a
    /// battlefield permanent would — proving the layer gather admits a
    /// command-zone emblem. Also proves the object presents as an `Emblem`
    /// ([CR#114.5]), is owned/controlled by the getter ([CR#114.2]), and lives
    /// in the command zone.
    #[test]
    fn emblem_static_ability_functions_from_command_zone() {
        use deckmaste_core::ObjectClass;

        let (mut state, bear) = bear_on_field(); // player 0's 2/2 Grizzly Bears
        assert_eq!(state.layers().power(bear), Some(2), "baseline 2/2 bear");

        let abilities = emblem_abilities(
            "Static(Each(SelectAll(And([Creature, ControlledBy(Ref(You))])), \
             Modify(It, Several([Power(Up(1)), Toughness(Up(1))]))))",
        );
        state.apply_emblem_created(PlayerId(0), abilities);

        let emblem = *state
            .zones
            .command
            .last()
            .expect("emblem in the command zone");
        assert!(
            crate::target::is_object_class(&state, emblem, ObjectClass::Emblem),
            "the object presents as an Emblem ([CR#114.5])"
        );
        assert_eq!(
            state.objects.obj(emblem).controller,
            PlayerId(0),
            "controlled by the getter ([CR#114.2])"
        );
        assert_eq!(state.owner_of(emblem), PlayerId(0), "owned by the getter");
        assert_eq!(state.objects.obj(emblem).zone, Some(Zone::Command));

        let view = state.layers();
        assert_eq!(
            view.power(bear),
            Some(3),
            "the command-zone emblem's static buffs the controller's creature +1/+1"
        );
        assert_eq!(view.toughness(bear), Some(3), "…on toughness too");
    }

    /// [CR#114.4]: an emblem's TRIGGERED ability fires from the command zone. An
    /// emblem carrying "whenever a creature dies, gain 1 life" (default `from`,
    /// i.e. no explicit zone) fires when a creature dies — proving the trigger
    /// watcher set admits command-zone emblems and the from-zone gate treats an
    /// emblem's default function zone as the command zone.
    #[test]
    fn emblem_triggered_ability_fires_from_command_zone() {
        let (mut state, bear) = bear_on_field();
        let abilities = emblem_abilities(
            "Triggered(event: ZoneChange(what: Type(Creature), from: Battlefield, \
             to: Graveyard), effect: ChangeLife(You, Up(1)))",
        );
        state.apply_emblem_created(PlayerId(0), abilities);
        let emblem = *state.zones.command.last().unwrap();
        let emblem_source = state.objects.obj(emblem).source;

        let died = zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard);
        state.scan_triggers(&Occurrence::single(died));
        assert_eq!(
            fired_count(&state, emblem_source),
            1,
            "the emblem's triggered ability fires from the command zone ([CR#114.4])"
        );
    }

    /// [CR#114.5,408.1]: no state-based action removes an emblem — it isn't a
    /// permanent and a command-zone object can't be destroyed. After a full SBA
    /// sweep the emblem object is still present in the command zone (and the
    /// token-cease SBA, which keys on `is_token`, never touches it).
    #[test]
    fn emblem_persists_across_sba_sweep() {
        let (mut state, _bear) = bear_on_field();
        let abilities =
            emblem_abilities("Static(Each(SelectAll(Creature), Modify(It, Power(Up(1)))))");
        state.apply_emblem_created(PlayerId(0), abilities);
        let emblem = *state.zones.command.last().unwrap();

        let events = crate::sba::sweep(&state);
        assert!(
            !events
                .iter()
                .any(|e| matches!(e, GameEvent::TokenCeased(id) if *id == emblem)),
            "the SBA sweep never ceases an emblem ([CR#114.5,408.1])"
        );
        assert!(
            state.objects.get(emblem).is_some() && state.zones.command.contains(&emblem),
            "the emblem persists in the command zone"
        );
    }

    /// The previously-`todo!()` batch seam, end to end ([CR#603.2c]): a real
    /// `OneOrMore` trigger scanned over a two-death BATCH occurrence fires
    /// ONCE; the plain (unquantified) twin fires once per member.
    #[test]
    fn one_or_more_trigger_fires_once_per_batch() {
        let watcher_card = |quantified: bool| {
            let event = if quantified {
                "OneOrMore(ZoneChange(what: Type(Creature), from: Battlefield, to: Graveyard))"
            } else {
                "ZoneChange(what: Type(Creature), from: Battlefield, to: Graveyard)"
            };
            let source = format!(
                "Normal(name: \"Batch Watcher\", types: [Enchantment], abilities: [\
                     Triggered(event: {event}, effect: ChangeLife(You, Up(1))),\
                 ])"
            );
            Arc::new(builtin().card_from_str(&source).unwrap().core)
        };
        for (quantified, expected) in [(true, 1), (false, 2)] {
            let (mut state, bear) = bear_on_field();
            let other = {
                let bears = Arc::new(canon().card("Grizzly Bears").unwrap().core);
                let card = state.cards.push(bears, PlayerId(0));
                let id = state.objects.mint(
                    ObjectSource::Card(card),
                    PlayerId(0),
                    Some(Zone::Battlefield),
                );
                state.zones.battlefield.push(id);
                id
            };
            let watcher = put_bf(&mut state, watcher_card(quantified), PlayerId(0));
            let watcher_source = state.objects.obj(watcher).source;
            let batch = Occurrence::Batch(vec![
                zone_changed_event(&state, bear, Zone::Battlefield, Zone::Graveyard),
                zone_changed_event(&state, other, Zone::Battlefield, Zone::Graveyard),
            ]);
            state.scan_triggers(&batch);
            let fired = state
                .agenda
                .iter()
                .filter(|w| {
                    matches!(
                        w,
                        WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(TriggerFired {
                            source, ..
                        }))) if *source == watcher_source
                    )
                })
                .count();
            assert_eq!(
                fired, expected,
                "quantified={quantified}: OneOrMore matches the batch once, \
                 the plain pattern once per member"
            );
        }
    }

    /// The saga-chapter walk-through ([CR#714.2b]): three chapter abilities
    /// authored through the `Chapter` MACRO (`Chapter(n: [N], effect: …)` —
    /// the [CR#714.2b] "{rN}—[`Instruction`]" spelling, expanding to
    /// `OneOrMore(CounterPlaced(kind: LoreCounter, on: Ref(This)))` gated by
    /// `Crossed` at thresholds 1/2/3) against a counter-DOUBLED
    /// 0→2 lore jump arriving as ONE batch fact — chapter I fires
    /// (before 0 < 1 ≤ 2 after), chapter II fires (0 < 2 ≤ 2), chapter III
    /// stays silent (2 < 3). A later 2→4 jump fires ONLY chapter III —
    /// already-crossed thresholds never re-fire.
    #[test]
    fn saga_chapters_fire_on_crossed_thresholds_from_one_batch_fact() {
        let chapter = |n: u32| format!("Chapter(n: [{n}], effect: ChangeLife(You, Up(1)))");
        let source = format!(
            "Normal(name: \"Test Saga\", types: [Enchantment], abilities: [{}, {}, {}])",
            chapter(1),
            chapter(2),
            chapter(3),
        );
        let card = Arc::new(builtin().card_from_str(&source).unwrap().core);
        let (mut state, _bear) = bear_on_field();
        let saga = put_bf(&mut state, card, PlayerId(0));
        let saga_source = state.objects.obj(saga).source;
        state.agenda.clear();
        state.agenda.push_back(WorkItem::BeginStep(
            deckmaste_core::PhaseStep::PrecombatMain,
        ));

        let fired_chapters =
            |state: &mut GameState, amount: deckmaste_core::Uint| -> Vec<deckmaste_core::Uint> {
                // The lore jump arrives through the emit pipeline as ONE batch
                // fact (the post-replacement shape a doubler leaves) — apply
                // fills before/after, and the scan reads the occurred fact.
                state.schedule_front(vec![WorkItem::Emit(Occurrence::Batch(vec![
                    GameEvent::CounterPlaced(CounterPlaced {
                        object: saga,
                        kind: "LoreCounter".into(),
                        amount,
                        before: 0,
                        after: 0,
                        cause: None,
                    }),
                ]))]);
                let _ = state.step();
                let fired: Vec<deckmaste_core::Uint> = state
                    .agenda
                    .iter()
                    .filter_map(|w| match w {
                        WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(
                            TriggerFired {
                                source, ability, ..
                            },
                        ))) if *source == saga_source => Some(*ability),
                        _ => None,
                    })
                    .collect();
                state.agenda.clear();
                state.agenda.push_back(WorkItem::BeginStep(
                    deckmaste_core::PhaseStep::PrecombatMain,
                ));
                fired
            };

        assert_eq!(
            fired_chapters(&mut state, 2),
            vec![0, 1],
            "the 0→2 jump crosses thresholds 1 and 2 — chapters I and II fire once \
             each, chapter III stays silent"
        );
        assert_eq!(
            fired_chapters(&mut state, 2),
            vec![2],
            "the 2→4 jump crosses only threshold 3 — chapters I and II are already \
             past theirs"
        );
    }

    /// The [CR#701.21a] entailment, trigger half: a plain "dies" trigger
    /// (the `Dies` macro — no cause narrow) FIRES on a sacrifice, because
    /// the sacrifice fact IS the entailed Battlefield→Graveyard move — no
    /// per-verb engine arm.
    #[test]
    fn dies_trigger_fires_on_a_sacrifice() {
        let source = "Normal(name: \"Death Watcher\", types: [Enchantment], abilities: [\
             Triggered(event: Dies(Type(Creature)), effect: ChangeLife(You, Up(1))),\
         ])";
        let card = Arc::new(canon().card_from_str(source).unwrap().core);
        let (mut state, bear) = bear_on_field();
        let watcher = put_bf(&mut state, card, PlayerId(0));
        let watcher_source = state.objects.obj(watcher).source;
        let sacrifice = zone_changed_with_cause(
            &state,
            bear,
            Zone::Battlefield,
            Zone::Graveyard,
            crate::event::Cause::sacrifice(deckmaste_core::Agency::CostPayment, None),
        );
        state.scan_triggers(&Occurrence::single(sacrifice));
        let fired = state
            .agenda
            .iter()
            .filter(|w| {
                matches!(
                    w,
                    WorkItem::Emit(Occurrence::Single(GameEvent::TriggerFired(TriggerFired { source, .. })))
                        if *source == watcher_source
                )
            })
            .count();
        assert_eq!(fired, 1, "a dies trigger sees a sacrifice [CR#701.21a]");
    }

    /// The previously-`todo!()` `Happened(Used)` seam ([CR#608.2i]): the
    /// self-scoped `Used(of: This)` resolves through the watcher's live
    /// object in the generic matcher, so the HISTORY lane reads it.
    #[test]
    fn happened_used_self_scoped_reads_history() {
        let (mut state, bear) = bear_on_field();
        let controller = state.objects.obj(bear).controller;
        let gate = Condition::happened(
            EventFilter::Used {
                of: Reference::Reg(deckmaste_core::RefId(0)),
            },
            deckmaste_core::Lookback::ThisGame,
        );
        let frame = state.frame(bear, controller);
        assert!(!state.condition_holds(&gate, &frame), "no use recorded yet");
        state.record_history_fact(
            1,
            None,
            GameEvent::AbilityUsed(AbilityUsed {
                object: bear,
                ability: 0,
            }),
        );
        assert!(
            state.condition_holds(&gate, &frame),
            "Happened(Used(of: This)) sees the recorded self-use"
        );
        // Another object's use is NOT this object's ([CR#400.7]).
        let other_gate_frame = state.frame(state.player(controller).object, controller);
        assert!(
            !state.condition_holds(&gate, &other_gate_frame),
            "object-scoped: a different carrier does not match"
        );
    }

    /// A per-candidate predicate region: candidate at parameter zero, the
    /// carrier (`This`) at parameter one — the shape
    /// `region::candidate_region` builds at lowering.
    fn candidate_region<T>(body: T) -> Arc<deckmaste_core::Region<T>> {
        Arc::new(deckmaste_core::Region::new(
            Arc::from([
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(0),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Candidate(
                        deckmaste_core::Domain::Entity,
                    ),
                },
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(1),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Source,
                },
            ]),
            body,
        ))
    }

    /// A synthetic "whenever a creature is dealt damage, you gain that much
    /// life" watcher — the trigger-bound magnitude lane. Re-spelled from the
    /// deleted `pain_gainer`: `Count::ThatMuch` is the `EventAmount` register
    /// the triggered region declares ([CR#107.3]).
    fn pain_gainer() -> deckmaste_card::Card {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_core::Ability;
        use deckmaste_core::Action;
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::LifeOp;
        use deckmaste_core::StatValue;
        use deckmaste_core::TriggeredAbility;

        Card::Normal(CardFace {
            name: "Pain Gainer".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                targets: [].into(),
                from: None,
                event: EventFilter::Damage {
                    source: Predicate::Any,
                    to: Predicate::creature(),
                    combat: None,
                    amount: None,
                },
                condition: None,
                limits: vec![].into(),
                effect: deckmaste_core::Region::new(
                    deckmaste_core::event_region_params(),
                    Instruction::Act(Action::ChangeLife(
                        Reference::controller_parameter(),
                        LifeOp::Up(Count::Reg(EVENT_AMOUNT)),
                    ))
                    .into(),
                ),
            })],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(4)),
            ..CardFace::default()
        })
    }

    /// The firing event's magnitude in the fixed event-role prefix
    /// ([`deckmaste_core::event_region_params`]).
    const EVENT_AMOUNT: deckmaste_core::RefId = deckmaste_core::RefId(6);

    /// The Training intervening-if as the macro spells it, re-spelled against
    /// the candidate regions the discourse stage introduced: the counted
    /// candidate is register 0 of its own region and the carrier is register 1.
    fn training_condition() -> Condition {
        use deckmaste_core::Cmp;
        use deckmaste_core::Count;
        use deckmaste_core::Reference;
        use deckmaste_core::Stat;
        use deckmaste_core::StatePredicate;
        const CANDIDATE: deckmaste_core::RefId = deckmaste_core::RefId(0);
        const CARRIER: deckmaste_core::RefId = deckmaste_core::RefId(1);
        Condition::Compare(
            Count::CountOf(deckmaste_core::Countable::Objects(candidate_region(
                Predicate::And(
                    vec![
                        Predicate::creature(),
                        Predicate::State(StatePredicate::Attacking),
                        Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(CARRIER)))),
                        Predicate::Where(candidate_region(Condition::Compare(
                            Count::StatOf(Reference::Reg(CANDIDATE), Stat::Power),
                            Cmp::Greater,
                            Count::StatOf(Reference::Reg(CARRIER), Stat::Power),
                        ))),
                    ]
                    .into(),
                ),
            ))),
            deckmaste_core::Cmp::AtLeast,
            Count::Literal(1),
        )
    }

    /// Does NOT hold when the only other attacker is lesser/equal power: a 2/2
    /// Grizzly Bears (lesser) and a second 3/3 Courser (equal — `Greater` is
    /// strict) both fail the comparison, so no "other creature with greater
    /// power" exists.
    #[test]
    fn training_condition_fails_without_a_greater_power_attacker() {
        let (state, carrier) = attacking_board(&["Grizzly Bears", "Centaur Courser"]); // 2/2, 3/3
        assert!(
            !state.condition_holds(&training_condition(), &carrier_gate_frame(&state, carrier)),
            "lesser (2/2) and equal (3/3) co-attackers do not satisfy 'greater power' ([CR#702.149a])"
        );
    }

    /// Holds when another attacker (4/4 Fangren Hunter) has power greater than
    /// the 3/3 carrier.
    #[test]
    fn training_condition_holds_with_a_greater_power_attacker() {
        let (state, carrier) = attacking_board(&["Fangren Hunter"]); // 4/4 > 3/3
        assert!(
            state.condition_holds(&training_condition(), &carrier_gate_frame(&state, carrier)),
            "a 4/4 co-attacker has greater power than the 3/3 carrier ([CR#702.149a])"
        );
    }

    /// The firing event's magnitude rides `TriggerBindings.event_amount` —
    /// captured at fire time — and reaches the fired ability through its
    /// declared `EventAmount` region parameter ("whenever …, … that much").
    /// Re-spelled from `trigger_bound_that_much_reads_the_firing_events_
    /// magnitude`: the binding field was `that_much` and the read was
    /// `Count::ThatMuch`; both are now the one declared register.
    #[test]
    fn trigger_bound_event_amount_reads_the_firing_events_magnitude() {
        use deckmaste_core::Zone;

        use crate::stack::StackEntry;
        use crate::stack::StackObject;

        let mut state = empty_game();
        state.turn.active_player = PlayerId(0);
        let gainer = put_synthetic_on_field(&mut state, pain_gainer(), PlayerId(0));
        state.agenda.clear();
        let life_before = state.players[0].life;

        // 3 damage to the gainer through the emit funnel — the post-
        // replacement fact the trigger scan sees.
        let source = state.players[1].object;
        state.schedule_front(vec![WorkItem::Emit(Occurrence::Single(
            GameEvent::DamageDealt(DamageDealt {
                source,
                target: gainer,
                amount: 3,
                combat: false,
            }),
        ))]);
        for _ in 0..10 {
            if state.agenda.is_empty() {
                break;
            }
            let _ = state.step();
        }

        // Fire-time capture: the noted trigger carries the magnitude.
        assert_eq!(noted_for(&state, gainer), 1, "the damage notes the trigger");
        let noted = state.pending_triggers[0].clone();
        assert_eq!(noted.bindings.event_amount, Some(3));

        // Resolve the trigger directly (no priority dance): the `EventAmount`
        // register reads the seeded magnitude, not a same-resolution apply.
        let id = state
            .objects
            .mint(noted.source, noted.controller, Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            paid_costs: Vec::new(),
            id,
            object: StackObject::Triggered {
                source: noted.source,
                ability: noted.ability,
                created: noted.created.clone(),
                bindings: noted.bindings.clone(),
            },
            controller: noted.controller,
            targets: Vec::new(),
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            copy: false,
        });
        state.resolve_object(id);
        for _ in 0..10 {
            if state.agenda.is_empty() {
                break;
            }
            let _ = state.step();
        }
        assert_eq!(
            state.players[0].life,
            life_before + 3,
            "GainLife through the EventAmount register reads the trigger-bound 3"
        );
    }
}
