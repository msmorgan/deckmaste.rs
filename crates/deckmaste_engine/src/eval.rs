//! The ONE event evaluator ([CR#603.2] and kin) — `engine-one-evaluator`.
//!
//! Every consumer of an `EventFilter` pattern — the trigger scan, the
//! replacement/cant gather, delayed triggers, `Happened`/`EventCount`/
//! `EventSum` history reads — evaluates through [`GameState::eval`] over one
//! [`FactView`] fact record, parameterized by [`Lane`]. The three divergent
//! bridge matchers (`engine-eventfilter-bridge`'s live matcher, would-matcher
//! and history scan) are gone; snapshot-vs-live candidate reading is a
//! per-participant [`Part`] value, not a second matcher.
//!
//! A [`FactView`] is built from a [`GameEvent`] two ways:
//! - **live** ([`FactView::of`]): participants stay [`Part::Obj`] live ids —
//!   the trigger/replacement lanes, where the fact is being scanned in its own
//!   wake and participants are (normally) still around;
//! - **per-fact LKI** ([`FactView::into_lki`]): at history-RECORD time every
//!   live card participant is snapshotted, so a later history read matches the
//!   participant *as it was* ([CR#603.10a]) — a `Happened(Damage(to:
//!   Type(Creature)))` over a recipient that has since died reads the snapshot
//!   instead of panicking on a stale id.

use std::borrow::Cow;

use deckmaste_core::EventFilter;
use deckmaste_core::Ident;
use deckmaste_core::Lookback;
use deckmaste_core::PhaseStep;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::StateChange;
use deckmaste_core::Uint;
use deckmaste_core::WhoseTurn;
use deckmaste_core::Zone;

use crate::event::AbilityActivated;
use crate::event::AbilityCountered;
use crate::event::AbilityUsed;
use crate::event::Act;
use crate::event::Attached;
use crate::event::Attacking;
use crate::event::BecameTarget;
use crate::event::Blocked;
use crate::event::Cause;
use crate::event::CoinFlipped;
use crate::event::ControlChanged;
use crate::event::Copied;
use crate::event::CounterPlaced;
use crate::event::CounterRemoved;
use crate::event::DamageDealt;
use crate::event::DamageRemoved;
use crate::event::DesignationChanged;
use crate::event::DieRolled;
use crate::event::EmblemCreated;
use crate::event::GameEvent;
use crate::event::GotDesignation;
use crate::event::LifeGained;
use crate::event::LifeLost;
use crate::event::ManaAbilityActivated;
use crate::event::ManaAdded;
use crate::event::ManaEmptied;
use crate::event::ManaProduced;
use crate::event::PlayerLost;
use crate::event::PlayerWon;
use crate::event::Revealed;
use crate::event::Tapped;
use crate::event::TappedForMana;
use crate::event::TokenCreated;
use crate::event::TriggerFired;
use crate::event::TurnBegan;
use crate::event::Unattached;
use crate::event::ZoneChange;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
use crate::state::GameState;

/// The consumer position a pattern is evaluated in. Per-atom lane admission
/// is a soundness property of the Idris model (an ill-placed atom is
/// unrepresentable there), not a load-time check; the lane here selects only
/// the residual semantic differences the evaluator itself owns — today, how
/// [`EventFilter::Nth`] counts the current occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Lane {
    /// `Triggered.event` — a live fact scanned in its own wake ([CR#603.2]);
    /// the fact is already in the history log when the scan runs.
    Trigger,
    /// `Replacement.would` / `CantHappen` — a would-fact ([CR#614]): the
    /// intent has NOT occurred (or been recorded) yet.
    Replacement,
    /// `Delayed.event` — fire-once against a live fact ([CR#603.7c]); same
    /// candidate semantics as [`Lane::Trigger`]. Consumed by
    /// [`GameState::event_matches_delayed`](crate::GameState::event_matches_delayed).
    Delayed,
    /// `Happened`/`EventCount`/`EventSum` — a recorded fact, matched against
    /// its per-fact LKI view ([CR#608.2i]).
    History,
    /// `Predicate::Where(Happened…)`'s candidate-relative history read — the
    /// snapshot lane ([CR#603.10a]); history semantics with the candidate
    /// bound as `It` by the filter layer. Declared to the normative eval
    /// signature; today those reads route through `condition_holds` →
    /// `Happened` (the History lane) with `It` bound by the snapshot
    /// matcher's `Where` arm.
    #[expect(
        dead_code,
        reason = "Where-lane history reads route through condition_holds → Happened today; \
                  the lane is declared to the normative eval signature ahead of a direct \
                  consumer"
    )]
    Snapshot,
}

/// What the evaluator resolves references and carrier anchors through — the
/// `bindings` of the §3.4 signature. `watcher` anchors `Ref(This)`/`Ref(You)`
/// in participant filters; `frame`, when the consumer holds one (history
/// reads inside a resolution/condition), resolves bound references
/// (`Used(of: …)` beyond the self-scoped `This` — object-scoped identity,
/// [CR#400.7]).
#[derive(Clone, Copy)]
pub(crate) struct Bindings<'a> {
    pub watcher: ObjectSource,
    pub frame: Option<&'a ExecutionFrame>,
    /// Floating-shield gather mode ([CR#614.3]): participant slots are
    /// skipped (the shield already matched its subject by IDENTITY — its
    /// `what` is typically a `Ref(EventObject)` no frameless gather could
    /// re-resolve); the event SHAPE (kind, zones, cause, refinements) still
    /// evaluates. Never set outside `floating_watches`.
    pub shape_only: bool,
}

impl Bindings<'_> {
    /// The common frameless bindings: a watcher anchor, full participant
    /// evaluation.
    pub(crate) fn watcher(watcher: ObjectSource) -> Self {
        Bindings {
            watcher,
            frame: None,
            shape_only: false,
        }
    }
}

/// One event participant, as the evaluator reads it — the `CandidateView`
/// of the §3.4 design: a live board object, a player (proxies never die),
/// or a last-known-information snapshot ([CR#603.10a]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Part<'a> {
    /// A live (or — after the participant left without a captured snapshot —
    /// stale) object id. A stale id is matched by IDENTITY only: filters
    /// beyond match-anything read `false`, never panic.
    Obj(ObjectId),
    /// A player, resolved to their live proxy object at evaluation time.
    Player(PlayerId),
    /// A snapshot view — the moved object of a past-form `ZoneChange` fact, or
    /// any card participant of a history-recorded fact (per-fact LKI).
    Gone(Cow<'a, LkiSnapshot>),
}

impl Part<'_> {
    /// The participant's object identity — a snapshot keeps its (stale) id
    /// as a reference token ([CR#400.7]).
    fn id(&self, state: &GameState) -> ObjectId {
        match self {
            Part::Obj(id) => *id,
            Part::Player(p) => state.player(*p).object,
            Part::Gone(s) => s.object,
        }
    }

    /// Re-borrow as an owned-lifetime part, snapshotting live card objects —
    /// the per-fact LKI capture ([CR#603.10a]) run at history-record time.
    fn into_lki(self, state: &GameState) -> Part<'static> {
        match self {
            Part::Obj(id) => match state.objects.get(id).map(|o| o.source) {
                Some(ObjectSource::Card(_)) => {
                    Part::Gone(Cow::Owned(LkiSnapshot::capture(state, id)))
                }
                Some(ObjectSource::Player(p)) => Part::Player(p),
                // Already gone with nothing to capture (e.g. the directly
                // recorded `AbilityUsed` of a sacrificed source): keep the id
                // — identity reads ([CR#400.7]) still work, filters don't.
                None => Part::Obj(id),
            },
            Part::Player(p) => Part::Player(p),
            Part::Gone(s) => Part::Gone(Cow::Owned(s.into_owned())),
        }
    }
}

/// The master-form KIND of a fact record — which `EventFilter` family it can
/// match. One fact, one kind; verb-view patterns (`Played`) match a
/// `ZoneChange`-kind record through their emitted entailment row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FactKind {
    ZoneChange,
    Damage,
    LifeGained,
    LifeLost,
    Drawn,
    CounterPlaced,
    CounterRemoved,
    Cast,
    Copied,
    ActivatedAb,
    ManaAbilityActivated,
    ManaProduced,
    ManaAdded,
    TappedForMana,
    AttackDeclared,
    BlockDeclared,
    Attached,
    /// A status transition ([CR#603.2e]) carrying WHICH state was entered.
    StateBecame(StateChange),
    BecomesTarget,
    StepBegins,
    ControlChanged,
    /// Designation deltas ([CR#109.3]): game-scope transitions and
    /// player-scope gains share the kind; a game-scope record has no
    /// carrier participant.
    DesignationChanged,
    TokenCreated,
    Used,
    CoinFlipped,
    DiceRolled,
    /// [CR#701.24a]: a library/pile was shuffled.
    Shuffled,
    /// [CR#701.20a]: cards were revealed.
    Revealed,
    /// A named keyword action ([CR#701]) — the `GameEvent::Act` event
    /// that [`EventFilter::Act`](deckmaste_core::EventFilter::Act) watches.
    Act,
}

/// THE one fact record ([CR#603.2]): `{kind, object, patient, actor, source,
/// from, to, cause, amount, counter, …, time}` — what every lane's patterns
/// evaluate against. Built from a [`GameEvent`] by [`FactView::of`];
/// participant slots are [`Part`] candidate views. (The design's `batch` and
/// `before`/`after` channels stay on [`crate::history::HistEntry`] and the
/// trigger bindings respectively — no pattern atom reads them.)
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FactView<'a> {
    pub kind: FactKind,
    /// The event OBJECT — the moved/acting thing (`what`/`by`/`of`/`on`
    /// slots): the moved object, the cast spell, the attacker, the blocker,
    /// the tapped object, the attachment, the counter carrier.
    pub object: Option<Part<'a>>,
    /// EVERY resolved object subject of an `Act` fact ([CR#701]) — `object`
    /// above holds the first, for the Replacement-lane `ZoneChange`
    /// relaxation's singleton move-verb read (`eval.rs:725-752`); the `Act`
    /// matcher's `on` is ∃-over-subjects; empty ≙ no subject. Non-`Act` kinds
    /// leave it empty.
    pub subjects: Vec<Part<'a>>,
    /// The acted-upon PATIENT ([CR#608.2k], kind-poly [CR#120.3]): the
    /// damage recipient, the blocked attacker, the attach host, the targeted
    /// object, the defending player, the designation gainer.
    pub patient: Option<Part<'a>>,
    /// The responsible-player ACTOR: the caster/drawer/flipper, the new
    /// controller, the active player of a step onset. Life gain/loss carries
    /// no actor — the player is the PATIENT ([CR#119.9,119.10]).
    pub actor: Option<PlayerId>,
    /// The performing SOURCE object: the damage source, the targeting
    /// spell/ability.
    pub source: Option<Part<'a>>,
    pub from: Option<Zone>,
    pub to: Option<Zone>,
    pub cause: Option<Cow<'a, Cause>>,
    pub amount: Option<Uint>,
    /// The counter kind of a counter fact ([CR#122.1]).
    pub counter: Option<Cow<'a, Ident>>,
    /// Combat vs noncombat damage ([CR#510.1]).
    pub combat: Option<bool>,
    /// A called coin flip's win/loss for the flipper ([CR#705.2]); `None`
    /// for an uncalled flip (and every non-flip fact).
    pub won: Option<bool>,
    /// The step/phase of a `StepBegins` record ([CR#603.2b]).
    pub step: Option<PhaseStep>,
    /// A designation record's `(name, becomes)` ([CR#109.3,731.1a]) —
    /// `becomes` only for game-scope transitions (day/night).
    pub designation: Option<(Cow<'a, Ident>, Option<Cow<'a, Ident>>)>,
    /// The printed keyword-action name of an `Act` fact ([CR#701]) —
    /// what [`EventFilter::Act`](deckmaste_core::EventFilter::Act) matches by.
    pub act_name: Option<Cow<'a, Ident>>,
    /// The turn the fact occurred in ([CR#608.2i] windows; the current turn
    /// for a live fact).
    pub time: Uint,
    /// The fact's history-log position — set at record; `None` for a live
    /// fact (the log tip). Bounds [`EventFilter::Nth`] ordinals in history
    /// lanes.
    pub seq: Option<usize>,
}

impl<'a> FactView<'a> {
    /// An all-`None` record of `kind`, stamped with the current turn — the
    /// per-kind builder arms fill in what the fact supplies.
    fn bare(kind: FactKind, state: &GameState) -> Self {
        FactView {
            kind,
            object: None,
            subjects: Vec::new(),
            patient: None,
            actor: None,
            source: None,
            from: None,
            to: None,
            cause: None,
            amount: None,
            counter: None,
            combat: None,
            won: None,
            step: None,
            designation: None,
            act_name: None,
            time: state.turn.turn_number,
            seq: None,
        }
    }

    /// The fact record of `event`, with live participants — `None` for
    /// plumbing events no pattern can watch (`TriggerFired`, mana movements,
    /// reveals, …). One total mapping serves every lane: an INTENT
    /// (`Act(Destroy)`, `Act(Draw)`, the future-form `ZoneChange`) and its
    /// downstream fact present the same kinds their patterns watch, each at
    /// its own pipeline stage.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per GameEvent kind — the fact table's full surface"
    )]
    pub(crate) fn of(state: &GameState, event: &'a GameEvent) -> Option<FactView<'a>> {
        // A participant id, classified live: a player proxy reads as the
        // player (proxies never die), anything else as a (possibly stale)
        // object id.
        let part = |id: ObjectId| match state.objects.get(id).map(|o| o.source) {
            Some(ObjectSource::Player(p)) => Part::Player(p),
            _ => Part::Obj(id),
        };
        let controller_of = |id: ObjectId| state.objects.get(id).map(|o| o.controller);

        let mut v: FactView<'a>;
        match event {
            // [CR#603.6]: the zone-change FACT (past-form `ZoneChange`) — the
            // moved object rides as its captured snapshot ([CR#603.10a]).
            GameEvent::ZoneChange(ZoneChange {
                snapshot: Some(snapshot),
                from,
                to,
                cause,
                ..
            }) => {
                // [CR#121.2,121.5]: a committed Library → Hand move tagged
                // `cause: Draw` IS the SUCCESS draw fact — `FactKind::Drawn`,
                // what "whenever you draw a card" / `CardsDrawn` read — NOT a
                // generic zone change (a non-draw tutor-to-hand keeps
                // `FactKind::ZoneChange`, so "without using the word draw"
                // stays honest). The attempt-level fact is the `Act(Draw)`.
                let drawn = *to == Zone::Hand
                    && cause.as_ref().is_some_and(|c| c.verb.as_str() == "Draw");
                v = FactView::bare(
                    if drawn {
                        FactKind::Drawn
                    } else {
                        FactKind::ZoneChange
                    },
                    state,
                );
                v.object = Some(Part::Gone(Cow::Borrowed(snapshot.as_ref())));
                v.actor = Some(snapshot.controller);
                v.from = *from;
                v.to = Some(*to);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#400.7]: the zone-change INTENT — the object is still live.
            GameEvent::ZoneChange(ZoneChange { snapshot: None,
                object,
                from,
                to,
                cause,
                ..
            }) => {
                v = FactView::bare(FactKind::ZoneChange, state);
                v.object = Some(part(*object));
                v.actor = controller_of(*object);
                v.from = *from;
                v.to = Some(*to);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#701]: the named keyword-action event — guard/replace moment
            // AND trigger fact in one. `verb` is the matched keyword name;
            // `on` (object verbs) is the resolved patient, riding the `object`
            // slot so `Act(Destroy(pred))` narrows it; `who` (player-report
            // verbs) is the performing player, riding the `actor` slot so
            // `Act(Scry(pred))` narrows it.
            GameEvent::Act(Act {
                verb,
                who,
                on,
                from,
                to,
                cause,
                batch,
                // Both phases lower identically ([CR#603.6]) — the future
                // window and its committed past fact present the same
                // name/result descriptions their patterns watch; `contents`/
                // `inherited` are resolution plumbing no pattern reads.
                committed: _,
                contents: _,
                inherited: _,
                contained: _,
            }) => {
                v = FactView::bare(FactKind::Act, state);
                v.act_name = Some(Cow::Borrowed(&verb.0));
                v.object = on.first().map(|&o| part(o));
                v.subjects = on.iter().map(|&o| part(o)).collect();
                v.actor = *who;
                // The BODY facet ([CR#603.6]): a move-verb (`Act(Destroy)`)
                // carries its realized zone-change so a `ZoneChange(→Graveyard)`
                // replacement (Rest in Peace) bites the SAME event as the tag
                // facet ([CR#616.1]); a reorder verb leaves these `None`.
                v.from = *from;
                v.to = *to;
                // "destroyed this way" provenance rides the atom's cause.
                v.cause = cause.as_ref().map(Cow::Borrowed);
                // [CR#616.1g,121.2a]: the `Batch` aggregate's own cardinality
                // — `EventFilter::Act` itself doesn't consult it ("Count is
                // not carried" above), but exposing it keeps the fact record
                // honest for any other consumer keyed on `amount`.
                v.amount = *batch;
            }
            // [CR#120.3]: damage — source object, kind-poly recipient.
            GameEvent::DamageDealt(DamageDealt {
                source,
                target,
                amount,
                combat,
            }) => {
                v = FactView::bare(FactKind::Damage, state);
                v.source = Some(part(*source));
                v.patient = Some(part(*target));
                v.amount = Some(*amount);
                v.combat = Some(*combat);
            }
            // [CR#119.9,119.10]: the player is the PATIENT — "a source
            // CAUSES a player to gain/lose life", no agent role.
            GameEvent::LifeGained(LifeGained {
                player,
                amount,
                cause,
            }) => {
                v = FactView::bare(FactKind::LifeGained, state);
                v.patient = Some(Part::Player(*player));
                v.amount = Some(*amount);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            GameEvent::LifeLost(LifeLost {
                player,
                amount,
                cause,
            }) => {
                v = FactView::bare(FactKind::LifeLost, state);
                v.patient = Some(Part::Player(*player));
                v.amount = Some(*amount);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#122.1]: counter deltas.
            GameEvent::CounterPlaced(CounterPlaced {
                object,
                kind,
                amount,
                cause,
                ..
            }) => {
                v = FactView::bare(FactKind::CounterPlaced, state);
                v.object = Some(part(*object));
                v.counter = Some(Cow::Borrowed(kind));
                v.amount = Some(*amount);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            GameEvent::CounterRemoved(CounterRemoved {
                object,
                kind,
                amount,
                cause,
            }) => {
                v = FactView::bare(FactKind::CounterRemoved, state);
                v.object = Some(part(*object));
                v.counter = Some(Cow::Borrowed(kind));
                v.amount = Some(*amount);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#601.2i]: the spell is live on the stack when the fact
            // applies; its controller is the caster.
            GameEvent::SpellCast(o) => {
                v = FactView::bare(FactKind::Cast, state);
                v.object = Some(part(*o));
                v.actor = controller_of(*o);
            }
            // [CR#707.10]: a copy is put on the stack, not cast — `Cast`
            // does not fire here. `object` is the minted copy; falls back
            // to the copied original for a pre-apply view (the apply hasn't
            // filled `copy` in yet).
            GameEvent::Copied(Copied {
                original,
                copy,
                controller,
            }) => {
                v = FactView::bare(FactKind::Copied, state);
                v.object = Some(part(copy.unwrap_or(*original)));
                v.actor = Some(*controller);
            }
            // [CR#602.2a]: `what` matches the ability's SOURCE object.
            GameEvent::AbilityActivated(AbilityActivated { source, .. }) => {
                v = FactView::bare(FactKind::ActivatedAb, state);
                v.object = Some(part(*source));
                v.actor = controller_of(*source);
            }
            GameEvent::ManaAbilityActivated(ManaAbilityActivated {
                source,
                controller,
                ..
            }) => {
                v = FactView::bare(FactKind::ManaAbilityActivated, state);
                v.object = Some(Part::Gone(Cow::Borrowed(source)));
                v.actor = Some(*controller);
            }
            GameEvent::ManaProduced(ManaProduced {
                source,
                controller,
                produced,
                ..
            }) => {
                v = FactView::bare(FactKind::ManaProduced, state);
                v.object = Some(Part::Gone(Cow::Borrowed(source)));
                v.actor = Some(*controller);
                v.amount = Some(
                    Uint::try_from(produced.len()).expect("produced mana count fits in Uint"),
                );
            }
            GameEvent::ManaAdded(ManaAdded {
                player,
                provenance,
                units,
                ..
            }) => {
                v = FactView::bare(FactKind::ManaAdded, state);
                v.object = provenance.source.map(part);
                v.actor = Some(*player);
                v.amount = Some(
                    Uint::try_from(units.len()).expect("added mana count fits in Uint"),
                );
            }
            GameEvent::TappedForMana(TappedForMana {
                source,
                controller,
                produced,
                ..
            }) => {
                v = FactView::bare(FactKind::TappedForMana, state);
                v.object = Some(Part::Gone(Cow::Borrowed(source)));
                v.actor = Some(*controller);
                v.amount = Some(
                    Uint::try_from(produced.len()).expect("produced mana count fits in Uint"),
                );
            }
            // [CR#508.1k]: the attacker is the object; the thing attacked
            // ([CR#508.1b,506.3]) is the patient — the defending player's
            // proxy (read as a `Player` part, [CR#508.5]) or a planeswalker
            // they control (an `Obj` part, [CR#508.1b]). Captured from the
            // declaration (not read back live) so a history view keeps the
            // defender of record.
            GameEvent::Attacking(Attacking {
                attacker,
                defending,
            }) => {
                v = FactView::bare(FactKind::AttackDeclared, state);
                v.object = Some(part(*attacker));
                v.actor = controller_of(*attacker);
                v.patient = Some(part(*defending));
            }
            // [CR#509.1g..509.1h]: one fact, two views — the blocker is the
            // object, the blocked attacker the patient.
            GameEvent::Blocked(Blocked { blocker, attacker }) => {
                v = FactView::bare(FactKind::BlockDeclared, state);
                v.object = Some(part(*blocker));
                v.patient = Some(part(*attacker));
            }
            // [CR#701.3a]: attachment onto host.
            GameEvent::Attached(Attached { attachment, host }) => {
                v = FactView::bare(FactKind::Attached, state);
                v.object = Some(part(*attachment));
                v.patient = Some(part(*host));
            }
            // [CR#603.2e]: the residual status transitions.
            GameEvent::Tapped(Tapped { object, cause }) => {
                v = FactView::bare(FactKind::StateBecame(StateChange::Tapped), state);
                v.object = Some(part(*object));
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            GameEvent::Untapped(o, cause) => {
                v = FactView::bare(FactKind::StateBecame(StateChange::Untapped), state);
                v.object = Some(part(*o));
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#701.27a,712.18]: the transform status transition — same
            // shape as `Tapped`/`Untapped`, matched by `StateBecame`.
            GameEvent::Transformed(o) => {
                v = FactView::bare(FactKind::StateBecame(StateChange::Transformed), state);
                v.object = Some(part(*o));
            }
            // [CR#601.2c]: the targeted object is the patient, the targeting
            // stack object the source-side participant.
            GameEvent::BecameTarget(BecameTarget { target, source }) => {
                v = FactView::bare(FactKind::BecomesTarget, state);
                v.patient = Some(part(*target));
                v.source = Some(part(*source));
            }
            // [CR#603.2b]: a step onset; the ACTOR is the active player —
            // captured per fact, so `whose` ([CR#503.1]) evaluates in
            // history reads too.
            GameEvent::StepBegan(ps) => {
                v = FactView::bare(FactKind::StepBegins, state);
                v.step = Some(*ps);
                v.actor = Some(state.turn.active_player);
            }
            // [CR#613.1b]: the object keeps its identity; the new controller
            // is the responsible actor.
            GameEvent::ControlChanged(ControlChanged { object, to }) => {
                v = FactView::bare(FactKind::ControlChanged, state);
                v.object = Some(part(*object));
                v.actor = Some(*to);
            }
            // [CR#109.3]: a game-scope transition has no carrier participant.
            GameEvent::DesignationChanged(DesignationChanged { name, becomes }) => {
                v = FactView::bare(FactKind::DesignationChanged, state);
                v.designation = Some((Cow::Borrowed(name), becomes.as_ref().map(Cow::Borrowed)));
            }
            // [CR#702.131c]: a player-scope gain — the gaining player is the
            // carrier the pattern's `of` runs against.
            GameEvent::GotDesignation(GotDesignation { player, name }) => {
                v = FactView::bare(FactKind::DesignationChanged, state);
                v.designation = Some((Cow::Borrowed(name), None));
                v.patient = Some(Part::Player(*player));
                v.actor = Some(*player);
            }
            // [CR#701.7a,111.2]: the fact carries the token SPEC — no minted
            // object participant (the `TokenCreated:what` cap).
            GameEvent::TokenCreated(TokenCreated { player, .. }) => {
                v = FactView::bare(FactKind::TokenCreated, state);
                v.actor = Some(*player);
            }
            // [CR#608.2i]: the substantive ability-use fact — object-scoped
            // identity ([CR#400.7]), so the raw id is the read.
            GameEvent::AbilityUsed(AbilityUsed { object, .. }) => {
                v = FactView::bare(FactKind::Used, state);
                v.object = Some(Part::Obj(*object));
            }
            // [CR#705.1]: the physical outcome, plus the call-relative
            // WIN/LOSS ([CR#705.2]) when the flip was called.
            GameEvent::CoinFlipped(CoinFlipped { player, won, .. }) => {
                v = FactView::bare(FactKind::CoinFlipped, state);
                v.actor = Some(*player);
                v.won = *won;
            }
            GameEvent::DieRolled(DieRolled { player, result, .. }) => {
                v = FactView::bare(FactKind::DiceRolled, state);
                v.actor = Some(*player);
                v.amount = Some(*result);
            }
            // [CR#701.24a]: a library/pile was shuffled — the shuffler is
            // the actor, no other participant.
            GameEvent::Shuffled(player) => {
                v = FactView::bare(FactKind::Shuffled, state);
                v.actor = Some(*player);
            }
            // [CR#701.20a]: cards revealed — ∃-matched over the revealed
            // set via `subjects` (the `Act` arm's precedent); the revealer
            // stays derived, no dedicated participant slot.
            GameEvent::Revealed(Revealed { objects, .. }) => {
                v = FactView::bare(FactKind::Revealed, state);
                v.object = objects.first().map(|&o| part(o));
                v.subjects = objects.iter().map(|&o| part(o)).collect();
            }
            // Plumbing and information events no pattern atom watches.
            GameEvent::TurnBegan(TurnBegan { .. })
            | GameEvent::TriggerFired(TriggerFired { .. })
            | GameEvent::AbilityResolved(_)
            | GameEvent::AbilityCountered(AbilityCountered { .. })
            | GameEvent::DrewFromEmpty(_)
            | GameEvent::TokenCeased(_)
            // [CR#114.1]: getting an emblem is not a watchable zone change —
            // no "whenever you get an emblem" pattern exists yet.
            | GameEvent::EmblemCreated(EmblemCreated { .. })
            | GameEvent::PlayerLost(PlayerLost { .. })
            | GameEvent::PlayerWon(PlayerWon { .. })
            | GameEvent::ManaEmptied(ManaEmptied { .. })
            | GameEvent::Unattached(Unattached { .. })
            | GameEvent::DamageRemoved(DamageRemoved { .. }) => return None,
        }
        Some(v)
    }

    /// The per-fact LKI capture ([CR#603.10a]), run at history-RECORD time:
    /// every live card participant becomes a snapshot, so later history
    /// matching reads the participant as it was — never the live store
    /// through a stale id.
    pub(crate) fn into_lki(self, state: &GameState) -> FactView<'static> {
        FactView {
            kind: self.kind,
            object: self.object.map(|p| p.into_lki(state)),
            subjects: self
                .subjects
                .into_iter()
                .map(|p| p.into_lki(state))
                .collect(),
            patient: self.patient.map(|p| p.into_lki(state)),
            actor: self.actor,
            source: self.source.map(|p| p.into_lki(state)),
            from: self.from,
            to: self.to,
            cause: self.cause.map(|c| Cow::Owned(c.into_owned())),
            amount: self.amount,
            counter: self.counter.map(|c| Cow::Owned(c.into_owned())),
            combat: self.combat,
            won: self.won,
            step: self.step,
            designation: self.designation.map(|(n, b)| {
                (
                    Cow::Owned(n.into_owned()),
                    b.map(|b| Cow::Owned(b.into_owned())),
                )
            }),
            act_name: self.act_name.map(|n| Cow::Owned(n.into_owned())),
            time: self.time,
            seq: self.seq,
        }
    }
}

/// A PRE-EVOLUTION intent whose committed fact lands in the same
/// [`FactKind`] downstream ([CR#603.6]): the future-form `ZoneChange`
/// (`snapshot: None`) evolves into the recorded past-form fact (`snapshot:
/// Some(..)`). Triggers fire on the FACT (the scan and the trigger adapter
/// refuse these — matching the intent would double-fire every zone-move
/// trigger), and history views are suppressed for them (the downstream fact
/// is the one counted); the REPLACEMENT lane is exactly where they are
/// matched ([CR#614]).
///
/// `Act(Destroy(x))` is NOT listed though it also evolves into a future-form
/// `ZoneChange`: its own fact view is `FactKind::Act` (not `ZoneChange`),
/// so a "dies"/"destroyed" `ZoneChange` trigger never matches the intent — no
/// double-fire — while the cant/replacement lane matches it AS an `Act`.
pub(crate) fn shadowed_by_fact(event: &GameEvent) -> bool {
    matches!(
        event,
        GameEvent::ZoneChange(ZoneChange { snapshot: None, .. })
    )
}

/// Does the recorded/current turn `time` fall inside `within`, seen from
/// `current_turn` ([CR#608.2i])? The sub-turn windows (`ThisCombat`/
/// `ThisStep`/`SinceYour`) need step markers the log records but no window
/// derivation consumes yet — they are not yet in the emittable surface
/// (`Lookback:*` rows; engine-history-windows), so no evaluable query
/// reaches them.
pub(crate) fn window_contains(within: Lookback, time: Uint, current_turn: Uint) -> bool {
    match within {
        Lookback::ThisTurn => time == current_turn,
        Lookback::ThisGame => true,
        Lookback::LastTurn => time + 1 == current_turn,
        Lookback::ThisCombat | Lookback::ThisStep | Lookback::SinceYour(_) => unreachable!(
            "sub-turn history windows are not yet supported: awaiting \
             engine-history-windows"
        ),
    }
}

/// `pattern` zone constraint vs the fact's zone coordinate: an omitted
/// pattern zone matches anything.
pub(crate) fn zone_ok(constraint: Option<Zone>, actual: Option<Zone>) -> bool {
    match constraint {
        None => true,
        Some(_) => constraint == actual,
    }
}

/// Looks through remembered `Predicate` macros to the structural filter.
pub(crate) fn deref_filter(f: &Predicate) -> &Predicate {
    f
}

/// Looks through remembered `Reference` macros to the structural reference.
pub(crate) fn deref_reference(r: &Reference) -> &Reference {
    r
}

impl GameState {
    /// THE one evaluator ([CR#603.2,603.6,614,608.2i]): does `pred` match the
    /// fact record `fact`, evaluated in `lane` with `bindings`?
    ///
    /// Master forms map onto [`FactKind`]s; verb-view forms (`Played`) match
    /// through their emitted entailment row; object-valued atoms evaluate
    /// their embedded `Predicate` against the participant's [`Part`] candidate
    /// view (live object, player proxy, or LKI snapshot) — snapshot
    /// semantics are a participant value, not a second matcher. Coordinates
    /// the fact record cannot supply (`TokenCreated:what` pre-mint specs,
    /// `BecomesTarget:source`) read `false` — each is unrepresentable in the
    /// Idris model, so no card carries one and `false` is a defensive floor.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per EventFilter node — the grammar's full surface in one dispatch"
    )]
    pub(crate) fn eval(
        &self,
        pred: &EventFilter,
        fact: &FactView<'_>,
        lane: Lane,
        bindings: &Bindings<'_>,
    ) -> bool {
        match pred {
            // Provenance is erased at `lower` (`deckmaste_lowering`), so no
            // loaded value reaches here wrapped. The arm survives only because
            // the variant does; `core-demacro` deletes both.

            // [CR#603.6]: zone constraints + cause narrowing + the moved
            // object's filter against its candidate view.
            EventFilter::ZoneChange {
                what,
                from,
                to,
                cause,
            } => {
                // Dual-facet ([CR#603.6,616.1]): a plain `ZoneChange` fact, OR —
                // in the REPLACEMENT lane only — a keyword-action `Act` carrying
                // a body-facet zone shape (a move-verb: `from`/`to` present). So
                // a `→Graveyard` replacement (Rest in Peace) bites the destroy
                // composite's BODY facet, gathered with regeneration (the TAG
                // facet, matched by the `Act` arm) into ONE applicable-set. The
                // relaxation is Replacement-only: the committed past-form
                // `ZoneChange` fact still carries the dies/enters TRIGGER, so matching the
                // `Act` there too would double-fire. A reorder `Act` (from/to
                // `None`) never matches — `zone_ok` fails, and a shapeless
                // `ZoneChange` query is gated out by the facet check.
                let body_facet = lane == Lane::Replacement
                    && fact.kind == FactKind::Act
                    && (fact.from.is_some() || fact.to.is_some());
                (fact.kind == FactKind::ZoneChange || body_facet)
                    && zone_ok(*from, fact.from)
                    && zone_ok(*to, fact.to)
                    && cause
                        .as_ref()
                        .is_none_or(|c| self.cause_matches(c, fact.cause.as_deref(), bindings))
                    && self.part_matches(what, fact.object.as_ref(), bindings)
            }

            // [CR#120.1,510.1]: source/recipient filters, the fact-carried
            // combat flag, and the dealt-amount bound.
            EventFilter::Damage {
                source,
                to,
                combat,
                amount,
            } => {
                fact.kind == FactKind::Damage
                    && combat.is_none_or(|want| Some(want) == fact.combat)
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && self.part_matches(source, fact.source.as_ref(), bindings)
                    && self.part_matches(to, fact.patient.as_ref(), bindings)
            }

            // [CR#119.3,119.9,119.10]: the player is the PATIENT.
            EventFilter::LifeGained { who, amount } => {
                fact.kind == FactKind::LifeGained
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && self.part_matches(who, fact.patient.as_ref(), bindings)
            }
            EventFilter::LifeLost { who, amount } => {
                fact.kind == FactKind::LifeLost
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && self.part_matches(who, fact.patient.as_ref(), bindings)
            }

            // [CR#121.1]: per-fact granularity is one card ([CR#121.2]), so
            // the record carries NO amount and a bound never matches — the
            // deliberately KEPT `Drawn:amount` cap (a multi-draw is N facts;
            // "draw two or more cards" is batch semantics, not a per-fact
            // bound).
            EventFilter::Drawn { who, amount } => {
                fact.kind == FactKind::Drawn
                    && (amount.is_none() || fact.amount.is_some())
                    && self.actor_matches(who, fact.actor, bindings)
            }

            // [CR#701]: the named keyword action, ONE master form. The verb
            // TAG fixes the name (`fact.act_name`); `who`/`on` narrow the
            // fact's performer/subject(s) through ONE uniform mapping for
            // every verb (see the arm body). `cause` narrows the cause triple.
            //
            // LANE-SPLIT ([CR#616.1,616.1f]): the trigger/history lanes match
            // the finalized NAME-fact only (verb + who/on/cause) — a redirected
            // madness discard still fires "whenever you discard" ([CR#701.9c]).
            // The Replacement (would) lane ADDITIONALLY requires the event's
            // realized zone facets to still match the verb's canonical shape
            // (the emitted entailment row — the same `from`/`to` facets the
            // `ZoneChange` relaxation above reads off an `Act`). The check is
            // DIVERGENCE-only: it fails only when the event EXPOSES a content
            // coordinate that the verb fixes and it has moved OFF canonical
            // (stacked madness / Leyline-first: a discard's `to` is now Exile,
            // not Graveyard) — the auto-guard, with zero semantic clause. A
            // coordinate the event doesn't expose (an aggregate `Batch`
            // window, a pre-choice cant-check — from/to `None`) is NOT a
            // divergence, so the would-lane matches on name there; likewise a
            // reorder verb (scry/surveil/fateseal/fight) fixes no zone shape.
            EventFilter::Act {
                verb,
                who,
                on,
                cause,
            } => {
                let name_ok = fact.kind == FactKind::Act
                    && fact.act_name.as_deref().map(Ident::as_str) == Some(verb.as_str());
                // ONE uniform mapping ([CR#701]; the Idris emitter's
                // Actor/Agent facets): `who` narrows the performing player,
                // `on` the object subject(s) — ∃ over the fact's subject
                // set, so a symmetric verb (Fight, [CR#701.14a]) matches on
                // EITHER combatant; an empty set ≙ no subject (only `Any`).
                let coords_ok = self.actor_matches(who, fact.actor, bindings)
                    && self.subjects_match(on, &fact.subjects, bindings);
                let cause_ok = cause
                    .as_ref()
                    .is_none_or(|c| self.cause_matches(c, fact.cause.as_deref(), bindings));
                let shape_ok = lane != Lane::Replacement
                    || crate::entail::entailment(verb.as_str()).is_none_or(|row| {
                        row.from
                            .zip(fact.from)
                            .is_none_or(|(canon, got)| canon == got)
                            && row.to.zip(fact.to).is_none_or(|(canon, got)| canon == got)
                    });
                name_ok && coords_ok && cause_ok && shape_ok
            }

            // [CR#122.1]: an omitted pattern `kind` watches any counter kind;
            // `cause` narrows the placement's cause triple ("as a cost").
            EventFilter::CounterPlaced {
                kind,
                on,
                amount,
                cause,
            } => {
                fact.kind == FactKind::CounterPlaced
                    && kind
                        .as_ref()
                        .is_none_or(|r| Some(&r.0) == fact.counter.as_deref())
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && cause
                        .as_ref()
                        .is_none_or(|c| self.cause_matches(c, fact.cause.as_deref(), bindings))
                    && self.part_matches(on, fact.object.as_ref(), bindings)
            }
            EventFilter::CounterRemoved {
                kind,
                on,
                amount,
                cause,
            } => {
                fact.kind == FactKind::CounterRemoved
                    && kind
                        .as_ref()
                        .is_none_or(|r| Some(&r.0) == fact.counter.as_deref())
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && cause
                        .as_ref()
                        .is_none_or(|c| self.cause_matches(c, fact.cause.as_deref(), bindings))
                    && self.part_matches(on, fact.object.as_ref(), bindings)
            }

            // [CR#601.2i].
            EventFilter::Cast { who, what } => {
                fact.kind == FactKind::Cast
                    && self.actor_matches(who, fact.actor, bindings)
                    && self.part_matches(what, fact.object.as_ref(), bindings)
            }

            // [CR#707.10]: the magecraft family's "or copy" half — `Cast`
            // does not fire for copies.
            EventFilter::Copied { who, what } => {
                fact.kind == FactKind::Copied
                    && self.actor_matches(who, fact.actor, bindings)
                    && self.part_matches(what, fact.object.as_ref(), bindings)
            }

            // [CR#701.18a]: the verb-view form — its fact shape (a
            // `ZoneChange` with the Play cause) comes from the emitted
            // entailment row, never hardcoded per-verb.
            EventFilter::Played { who, what } => {
                let row = crate::entail::entailment("Play").expect("emitted Play entailment row");
                debug_assert_eq!(row.kind, "ZoneChange", "a play rides a zone move");
                fact.kind == FactKind::ZoneChange
                    && fact
                        .cause
                        .as_deref()
                        .is_some_and(|c| c.verb.as_str() == row.verb)
                    && zone_ok(row.from, fact.from)
                    && zone_ok(row.to, fact.to)
                    && self.actor_matches(who, fact.actor, bindings)
                    && self.part_matches(what, fact.object.as_ref(), bindings)
            }

            // [CR#602.2a].
            EventFilter::ActivatedAb { who, what } => {
                matches!(
                    fact.kind,
                    FactKind::ActivatedAb | FactKind::ManaAbilityActivated
                ) && self.actor_matches(who, fact.actor, bindings)
                    && self.part_matches(what, fact.object.as_ref(), bindings)
            }

            EventFilter::ManaAbilityActivated { what, by } => {
                fact.kind == FactKind::ManaAbilityActivated
                    && self.part_matches(what, fact.object.as_ref(), bindings)
                    && self.actor_matches(by, fact.actor, bindings)
            }

            EventFilter::ManaProduced { what, by } => {
                fact.kind == FactKind::ManaProduced
                    && self.part_matches(what, fact.object.as_ref(), bindings)
                    && self.actor_matches(by, fact.actor, bindings)
            }

            EventFilter::ManaAdded { what, by } => {
                fact.kind == FactKind::ManaAdded
                    && self.part_matches(what, fact.object.as_ref(), bindings)
                    && self.actor_matches(by, fact.actor, bindings)
            }

            // [CR#508.1k]: the defending player rides the record as its
            // patient ([CR#506.2,508.5]).
            EventFilter::AttackDeclared { by, against } => {
                fact.kind == FactKind::AttackDeclared
                    && self.part_matches(by, fact.object.as_ref(), bindings)
                    && self.part_matches(against, fact.patient.as_ref(), bindings)
            }

            // [CR#509.1g..509.1h]: `by` the blocker, `of` the blocked
            // attacker.
            EventFilter::BlockDeclared { by, of } => {
                fact.kind == FactKind::BlockDeclared
                    && self.part_matches(by, fact.object.as_ref(), bindings)
                    && self.part_matches(of, fact.patient.as_ref(), bindings)
            }

            // [CR#701.3a].
            EventFilter::Attached { what, to } => {
                fact.kind == FactKind::Attached
                    && self.part_matches(what, fact.object.as_ref(), bindings)
                    && self.part_matches(to, fact.patient.as_ref(), bindings)
            }

            // [CR#603.2e]: the transition must be the very state named —
            // phasing/turn-face have no fact shape, so their
            // patterns match no record (kept caps `StateBecame:Phased`/
            // `:TurnedFace`).
            EventFilter::StateBecame { of, becomes, cause } => {
                fact.kind == FactKind::StateBecame(becomes.clone())
                    && cause
                        .as_ref()
                        .is_none_or(|c| self.cause_matches(c, fact.cause.as_deref(), bindings))
                    && self.part_matches(of, fact.object.as_ref(), bindings)
            }

            // [CR#601.2c]: `what` the targeted thing, `by` the targeting
            // stack object. The hexproof-from `source` arm
            // ([CR#702.11d,702.16b]) has no record coordinate — the KEPT
            // `BecomesTarget:source` cap; a pattern carrying it matches
            // nothing.
            EventFilter::BecomesTarget { what, by, source } => {
                fact.kind == FactKind::BecomesTarget
                    && source.is_none()
                    && self.part_matches(what, fact.patient.as_ref(), bindings)
                    && self.part_matches(by, fact.source.as_ref(), bindings)
            }

            // [CR#603.2b]: the step onset; `whose` ([CR#503.1]) compares the
            // watcher's controller against the fact's recorded active
            // player, so history reads see the turn of record.
            EventFilter::StepBegins { at, whose } => {
                if fact.kind != FactKind::StepBegins || fact.step != Some(*at) {
                    return false;
                }
                match whose {
                    WhoseTurn::EachPlayers => true,
                    WhoseTurn::Your => {
                        self.controller_of_source(bindings.watcher).is_some()
                            && self.controller_of_source(bindings.watcher) == fact.actor
                    }
                    WhoseTurn::AnOpponents => self
                        .controller_of_source(bindings.watcher)
                        .is_some_and(|c| fact.actor.is_some_and(|a| a != c)),
                }
            }

            // [CR#613.1b].
            EventFilter::ControlChanged { of, to } => {
                fact.kind == FactKind::ControlChanged
                    && self.part_matches(of, fact.object.as_ref(), bindings)
                    && self.actor_matches(to, fact.actor, bindings)
            }

            // [CR#109.3]: a game-scope record has no carrier — only the
            // match-anything default is satisfiable; a player-scope gain
            // runs `of` against the gaining player.
            EventFilter::DesignationChanged { name, of, to } => {
                fact.kind == FactKind::DesignationChanged
                    && fact.designation.as_ref().is_some_and(|(n, value)| {
                        n.as_ref() == name
                            && to.as_ref().is_none_or(|want| {
                                value.as_ref().is_some_and(|got| got.as_ref() == want)
                            })
                    })
                    && self.part_matches(of, fact.patient.as_ref(), bindings)
            }

            // [CR#701.7a,111.2]: the record carries the token SPEC only — a
            // non-default `what` is the KEPT `TokenCreated:what` cap
            // (pre-mint spec matching needs a spec-side filter evaluator)
            // and matches nothing.
            EventFilter::TokenCreated { what, by } => {
                fact.kind == FactKind::TokenCreated
                    && matches!(deref_filter(what), Predicate::Any)
                    && self.actor_matches(by, fact.actor, bindings)
            }

            // [CR#701.24a]: `by` narrows the shuffling player (Psychic
            // Surgery's "whenever a player shuffles their library").
            EventFilter::Shuffled { by } => {
                fact.kind == FactKind::Shuffled && self.actor_matches(by, fact.actor, bindings)
            }

            // [CR#701.20a]: `what` ∃-matches the revealed set — the `Act`
            // arm's `subjects_match` precedent; no performer coordinate
            // (the revealer stays derived, not a fact participant).
            EventFilter::Revealed { what } => {
                fact.kind == FactKind::Revealed
                    && self.subjects_match(what, &fact.subjects, bindings)
            }

            // [CR#608.2i,400.7]: object-scoped identity — `of` resolves
            // through the frame when the consumer holds one, else through
            // the watcher's live object (the self-scoped `This`). A bound
            // non-`This` reference outside any frame is unrepresentable in
            // the Idris model (the binder is required; frameless lanes stay
            // `This`).
            EventFilter::Used { of } => {
                if fact.kind != FactKind::Used {
                    return false;
                }
                let Some(object) = fact.object.as_ref() else {
                    return false;
                };
                let used = object.id(self);
                match bindings.frame {
                    Some(frame) => self.eval_reference(of, frame) == used,
                    None => match deref_reference(of) {
                        reference if reference == &Reference::source_parameter() => self
                            .objects
                            .iter()
                            .find(|ob| ob.source == bindings.watcher)
                            .is_some_and(|ob| ob.id == used),
                        // No frame, no binder: unreachable from sound
                        // data, no match — never a panic.
                        _ => false,
                    },
                }
            }

            EventFilter::CoinFlipped { by, won } => {
                fact.kind == FactKind::CoinFlipped
                    && match won {
                        // Unfiltered: any flip, called or not.
                        None => true,
                        // [CR#705.2]: a win/loss filter never matches an
                        // uncalled flip — no player wins or loses one.
                        Some(w) => fact.won == Some(*w),
                    }
                    && self.actor_matches(by, fact.actor, bindings)
            }

            // [CR#706.1].
            EventFilter::DiceRolled { by } => {
                fact.kind == FactKind::DiceRolled && self.actor_matches(by, fact.actor, bindings)
            }

            EventFilter::TapForMana { what, by } => {
                fact.kind == FactKind::TappedForMana
                    && self.part_matches(what, fact.object.as_ref(), bindings)
                    && self.actor_matches(by, fact.actor, bindings)
            }

            // [CR#901.9]: Plane cards and the planar die are not modeled.
            EventFilter::RollPlanarDie { .. } => false,

            // [CR#603.2]: refinement conjunction — one occurrence, every
            // sub-pattern.
            EventFilter::AllOf(events) => events.iter().all(|p| self.eval(p, fact, lane, bindings)),

            // [CR#603.2c]: pattern union, still once per occurrence.
            EventFilter::OneOf(events) => events.iter().any(|p| self.eval(p, fact, lane, bindings)),

            // [CR#603.2]: the complement refinement — never an anchor (the
            // load-time kind pass requires an anchored sibling).
            EventFilter::Not(inner) => !self.eval(inner, fact, lane, bindings),

            // [CR#603.2c]: against one fact the batch quantifier matches iff
            // its operand does; the ONCE-per-occurrence discipline is the
            // scan's batch dedup.
            EventFilter::OneOrMore(inner) => self.eval(inner, fact, lane, bindings),

            // [CR#603.2g]: the nth matching occurrence within a lookback —
            // 1-based, counted off the history log. In fact lanes the
            // occurrence being evaluated is already recorded (the scan runs
            // in the fact's wake); a REPLACEMENT would-fact has not occurred
            // yet, so it counts as one past its recorded predecessors; a
            // history fact counts its predecessors up to its own log
            // position.
            EventFilter::Nth { n, of, within } => {
                if !self.eval(of, fact, lane, bindings) {
                    return false;
                }
                let upto = match lane {
                    Lane::Trigger | Lane::Delayed | Lane::Replacement => None,
                    Lane::History | Lane::Snapshot => fact.seq,
                };
                let mut count = self.matching_history_count(of, *within, upto, bindings);
                if lane == Lane::Replacement {
                    count += 1;
                }
                u64::from(*n) == count as u64
            }

            // [CR#603.4]: the pattern-level game-state refinement — the
            // condition is evaluated as the event occurs, in the consumer's
            // frame when it holds one, else in a minimal frame anchored on
            // the watcher's live carrier. (History lanes keep the `When`
            // cap: an as-it-occurred condition is not reconstructible from
            // the log.)
            EventFilter::When(inner, condition) => {
                if !self.eval(inner, fact, lane, bindings) {
                    return false;
                }
                match bindings.frame {
                    Some(frame) => self.condition_holds(condition, frame),
                    None => self
                        .watcher_frame(bindings.watcher)
                        .is_some_and(|frame| self.condition_holds(condition, &frame)),
                }
            }

            // [CR#608.2i]: the history window refinement — the fact's
            // recorded turn against the lookback (a live fact's time is the
            // current turn; live lanes REFUSE `Within` as vacuous — a
            // soundness property of the Idris model).
            EventFilter::Within(inner, within) => {
                let controller = bindings
                    .frame
                    .map(|frame| frame.controller(self))
                    .or_else(|| self.controller_of_source(bindings.watcher));
                let in_window = match (*within, fact.seq, controller) {
                    (Lookback::SinceYour(_), Some(seq), Some(controller)) => {
                        self.history.position_in_window_for(
                            seq,
                            *within,
                            self.turn.turn_number,
                            self.turn.current,
                            self.turn.active_player,
                            controller,
                        )
                    }
                    (Lookback::SinceYour(_), _, _) => false,
                    _ => window_contains(*within, fact.time, self.turn.turn_number),
                };
                in_window && self.eval(inner, fact, lane, bindings)
            }

            // [CR#702.40a,603.3]: the storm refinement — the fact matches iff
            // its log position (the monotonic cast ORDER, [CR#601.2i]) is
            // strictly before the referenced object's OWN cast. "Other …
            // before it" falls straight out of `<`: the reference's own cast
            // is not before itself, and a spell cast in RESPONSE (a later log
            // position) is excluded even though it shares the turn tally. A
            // history-lane refinement: a live fact has no `seq` and never
            // matches, and with no frame there is no `This` to anchor.
            EventFilter::Before(reference) => {
                let Some(frame) = bindings.frame else {
                    return false;
                };
                let anchor = self.eval_reference(reference, frame);
                let Some(cast_seq) = self.cast_seq_of(anchor) else {
                    return false;
                };
                fact.seq.is_some_and(|seq| seq < cast_seq)
            }
        }
    }

    /// The history-log position of `object`'s own `SpellCast` fact
    /// ([CR#601.2i]) — the monotonic cast-order anchor
    /// [`EventFilter::Before`] compares against. `None` when the object has
    /// no recorded cast this game (a live or uncast reference — the `Before`
    /// refinement then matches nothing, never a panic).
    fn cast_seq_of(&self, object: ObjectId) -> Option<usize> {
        self.history
            .in_window(Lookback::ThisGame, self.turn.turn_number)
            .find(|(_, entry)| matches!(&entry.fact, GameEvent::SpellCast(o) if *o == object))
            .map(|(seq, _)| seq)
    }

    /// Does the pattern's cause narrowing admit the fact's cause triple
    /// ([CR#603.2] over the (verb, agency, agent) coordinates)? Every
    /// PRESENT coordinate must match; an omitted one matches anything; a
    /// fact with NO cause fails every cause-narrowed pattern ("destroyed"
    /// admits exactly two causes [CR#701.8b], a sacrifice is never one of
    /// them [CR#701.21a]). The agent filter runs against the causing object
    /// — live at scan/would time; a departed agent in a history read
    /// matches only the match-anything default (agents are not yet in the
    /// per-fact LKI capture).
    fn cause_matches(
        &self,
        pattern: &deckmaste_core::Cause,
        actual: Option<&Cause>,
        bindings: &Bindings<'_>,
    ) -> bool {
        let deckmaste_core::Cause::Cause(p) = pattern;
        let Some(cause) = actual else {
            return false;
        };
        if p.verb.is_some_and(|v| v.as_str() != cause.verb.as_str()) {
            return false;
        }
        if p.agency.is_some_and(|a| a != cause.agency) {
            return false;
        }
        match (&p.agent, cause.agent) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(f), Some((agent, _controller))) => {
                self.part_matches(f, Some(&Part::Obj(agent)), bindings)
            }
        }
    }

    /// An amount BOUND against the fact's amount channel: an omitted bound
    /// matches anything; a bound over a record with no amount matches
    /// nothing (the kept `Drawn:amount` cap's shape).
    fn amount_ok(
        &self,
        bound: Option<&deckmaste_core::CountBound>,
        amount: Option<Uint>,
        bindings: &Bindings<'_>,
    ) -> bool {
        match bound {
            None => true,
            Some(bound) => amount.is_some_and(|a| {
                bound.satisfied_by(a, |c| match bindings.frame {
                    Some(frame) => self.eval_count(c, frame),
                    None => crate::trigger::literal_count(c),
                })
            }),
        }
    }

    /// A participant FILTER against its candidate view: a live object runs
    /// the live matcher, a player their proxy, a snapshot the LKI matcher —
    /// the `CandidateView` parameter of the one-evaluator design. A missing
    /// participant (a game-scope designation's carrier) and a stale live id
    /// (the participant left with no captured snapshot) match only the
    /// match-anything default — never a panic through the live store.
    fn part_matches(
        &self,
        filter: &Predicate,
        part: Option<&Part<'_>>,
        bindings: &Bindings<'_>,
    ) -> bool {
        if bindings.shape_only {
            return true;
        }
        let activation = bindings
            .frame
            .map_or(crate::ActivationId::NONE, |frame| frame.activation);
        match part {
            None => matches!(deref_filter(filter), Predicate::Any),
            Some(Part::Obj(id)) => {
                if self.objects.get(*id).is_some() {
                    self.filter_matches_live_with_activation(
                        filter,
                        *id,
                        bindings.watcher,
                        activation,
                    )
                } else {
                    matches!(deref_filter(filter), Predicate::Any)
                }
            }
            Some(Part::Player(p)) => self.filter_matches_live_with_activation(
                filter,
                self.player(*p).object,
                bindings.watcher,
                activation,
            ),
            Some(Part::Gone(snapshot)) => self.filter_matches_snapshot_with_activation(
                filter,
                snapshot,
                bindings.watcher,
                activation,
            ),
        }
    }

    /// The ∃-over-subjects filter of an `Act` fact's subject set: empty (a
    /// subjectless verb) matches only the match-anything default — exactly
    /// `part_matches`'s missing-participant rule lifted to a list; a
    /// singleton is `part_matches` verbatim.
    fn subjects_match(
        &self,
        filter: &Predicate,
        subjects: &[Part<'_>],
        bindings: &Bindings<'_>,
    ) -> bool {
        if subjects.is_empty() {
            return self.part_matches(filter, None, bindings);
        }
        subjects
            .iter()
            .any(|p| self.part_matches(filter, Some(p), bindings))
    }

    /// A player-slot filter (`who`/`by`/`to`-player) against the fact's
    /// actor, via their live proxy.
    fn actor_matches(
        &self,
        filter: &Predicate,
        actor: Option<PlayerId>,
        bindings: &Bindings<'_>,
    ) -> bool {
        self.part_matches(filter, actor.map(Part::Player).as_ref(), bindings)
    }

    /// The number of recorded facts matching `of` within `within`
    /// ([CR#608.2i]), optionally bounded to log positions `<= upto` — the
    /// [`EventFilter::Nth`] ordinal's counting read.
    fn matching_history_count(
        &self,
        of: &EventFilter,
        within: Lookback,
        upto: Option<usize>,
        bindings: &Bindings<'_>,
    ) -> usize {
        // Fail CLOSED like the `EventFilter::Within` arm: a departed
        // watcher's `SinceYour` window matches nothing — never borrow the
        // active player's anchor as a guess. (The owner argument is unused
        // by every other `Lookback`.)
        let owner = match self.controller_of_source(bindings.watcher) {
            Some(owner) => owner,
            None if matches!(within, Lookback::SinceYour(_)) => return 0,
            None => self.turn.active_player,
        };
        self.history
            .in_window_for(
                within,
                self.turn.turn_number,
                self.turn.current,
                self.turn.active_player,
                owner,
            )
            .filter(|(seq, _)| upto.is_none_or(|u| *seq <= u))
            .filter_map(|(_, entry)| entry.view.as_ref())
            .filter(|view| self.eval(of, view, Lane::History, bindings))
            .count()
    }

    /// A minimal condition frame anchored on the watcher's LIVE carrier —
    /// what a frameless lane evaluates a [`EventFilter::When`] condition in.
    /// `None` when the carrier is gone (`This`/`You` would be unresolvable;
    /// the pattern then matches nothing, mirroring the live `Predicate::Where`
    /// discipline).
    fn watcher_frame(&self, watcher: ObjectSource) -> Option<ExecutionFrame> {
        self.objects
            .iter()
            .find(|ob| ob.source == watcher)
            .map(|ob| self.frame(ob.id, ob.controller))
    }
}
