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

use crate::event::Cause;
use crate::event::GameEvent;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::stack::Frame;
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
pub(crate) struct Bindings<'a> {
    pub watcher: ObjectSource,
    pub frame: Option<&'a Frame>,
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
    /// A snapshot view — the moved object of a `ZoneChanged` fact, or any
    /// card participant of a history-recorded fact (per-fact LKI).
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
    /// The acted-upon PATIENT ([CR#608.2k], kind-poly [CR#120.3]): the
    /// damage recipient, the blocked attacker, the attach host, the targeted
    /// object, the defending player, the designation gainer.
    pub patient: Option<Part<'a>>,
    /// The responsible-player ACTOR: the caster/drawer/gainer/flipper, the
    /// new controller, the active player of a step onset.
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
    /// (`Act(Destroy)`, `WillDraw`, `ZoneWillChange`) and its downstream fact
    /// present the same kinds their patterns watch, each at its own
    /// pipeline stage.
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
            // [CR#603.6]: the zone-change FACT — the moved object rides as
            // its captured snapshot ([CR#603.10a]).
            GameEvent::ZoneChanged {
                snapshot,
                from,
                to,
                cause,
                ..
            } => {
                v = FactView::bare(FactKind::ZoneChange, state);
                v.object = Some(Part::Gone(Cow::Borrowed(snapshot)));
                v.actor = Some(snapshot.controller);
                v.from = *from;
                v.to = Some(*to);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#400.7]: the zone-change INTENT — the object is still live.
            GameEvent::ZoneWillChange {
                object,
                from,
                to,
                cause,
                ..
            } => {
                v = FactView::bare(FactKind::ZoneChange, state);
                v.object = Some(part(*object));
                v.actor = controller_of(*object);
                v.from = *from;
                v.to = Some(*to);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#121.1]: the draw intent. Per-fact granularity is one card
            // ([CR#121.2]); the `amount` channel stays empty — a multi-card
            // amount BOUND is the `Drawn:amount` cap, deliberately kept.
            GameEvent::WillDraw { player, .. } => {
                v = FactView::bare(FactKind::Drawn, state);
                v.actor = Some(*player);
            }
            // [CR#701]: the named keyword-action event — guard/replace moment
            // AND trigger fact in one. `verb` is the matched keyword name;
            // `on` (object verbs) is the resolved patient, riding the `object`
            // slot so `Act(Destroy(pred))` narrows it; `who` (player-report
            // verbs) is the performing player, riding the `actor` slot so
            // `Act(Scry(pred))` narrows it.
            GameEvent::Act {
                verb,
                who,
                on,
                cause,
            } => {
                v = FactView::bare(FactKind::Act, state);
                v.act_name = Some(Cow::Borrowed(&verb.0));
                v.object = on.as_ref().map(|o| part(*o));
                v.actor = *who;
                // "destroyed this way" provenance rides the atom's cause.
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            // [CR#120.3]: damage — source object, kind-poly recipient.
            GameEvent::DamageDealt {
                source,
                target,
                amount,
                combat,
            } => {
                v = FactView::bare(FactKind::Damage, state);
                v.source = Some(part(*source));
                v.patient = Some(part(*target));
                v.amount = Some(*amount);
                v.combat = Some(*combat);
            }
            GameEvent::LifeGained { player, amount } => {
                v = FactView::bare(FactKind::LifeGained, state);
                v.actor = Some(*player);
                v.amount = Some(*amount);
            }
            GameEvent::LifeLost { player, amount } => {
                v = FactView::bare(FactKind::LifeLost, state);
                v.actor = Some(*player);
                v.amount = Some(*amount);
            }
            // [CR#122.1]: counter deltas.
            GameEvent::CounterPlaced {
                object,
                kind,
                amount,
                cause,
                ..
            } => {
                v = FactView::bare(FactKind::CounterPlaced, state);
                v.object = Some(part(*object));
                v.counter = Some(Cow::Borrowed(kind));
                v.amount = Some(*amount);
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            GameEvent::CounterRemoved {
                object,
                kind,
                amount,
                cause,
            } => {
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
            GameEvent::Copied {
                original,
                copy,
                controller,
            } => {
                v = FactView::bare(FactKind::Copied, state);
                v.object = Some(part(copy.unwrap_or(*original)));
                v.actor = Some(*controller);
            }
            // [CR#602.2a]: `what` matches the ability's SOURCE object.
            GameEvent::AbilityActivated { source, .. } => {
                v = FactView::bare(FactKind::ActivatedAb, state);
                v.object = Some(part(*source));
                v.actor = controller_of(*source);
            }
            // [CR#508.1k]: the attacker is the object; the thing attacked
            // ([CR#508.1b,506.3]) is the patient — the defending player's
            // proxy (read as a `Player` part, [CR#508.5]) or a planeswalker
            // they control (an `Obj` part, [CR#508.1b]). Captured from the
            // declaration (not read back live) so a history view keeps the
            // defender of record.
            GameEvent::Attacking {
                attacker,
                defending,
            } => {
                v = FactView::bare(FactKind::AttackDeclared, state);
                v.object = Some(part(*attacker));
                v.actor = controller_of(*attacker);
                v.patient = Some(part(*defending));
            }
            // [CR#509.1g..509.1h]: one fact, two views — the blocker is the
            // object, the blocked attacker the patient.
            GameEvent::Blocked { blocker, attacker } => {
                v = FactView::bare(FactKind::BlockDeclared, state);
                v.object = Some(part(*blocker));
                v.patient = Some(part(*attacker));
            }
            // [CR#701.3a]: attachment onto host.
            GameEvent::Attached { attachment, host } => {
                v = FactView::bare(FactKind::Attached, state);
                v.object = Some(part(*attachment));
                v.patient = Some(part(*host));
            }
            // [CR#603.2e]: the residual status transitions.
            GameEvent::Tapped { object, cause } => {
                v = FactView::bare(FactKind::StateBecame(StateChange::Tapped), state);
                v.object = Some(part(*object));
                v.cause = cause.as_ref().map(Cow::Borrowed);
            }
            GameEvent::Untapped(o) => {
                v = FactView::bare(FactKind::StateBecame(StateChange::Untapped), state);
                v.object = Some(part(*o));
            }
            // [CR#601.2c]: the targeted object is the patient, the targeting
            // stack object the source-side participant.
            GameEvent::BecameTarget { target, source } => {
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
            GameEvent::ControlChanged { object, to } => {
                v = FactView::bare(FactKind::ControlChanged, state);
                v.object = Some(part(*object));
                v.actor = Some(*to);
            }
            // [CR#109.3]: a game-scope transition has no carrier participant.
            GameEvent::DesignationChanged { name, becomes } => {
                v = FactView::bare(FactKind::DesignationChanged, state);
                v.designation = Some((Cow::Borrowed(name), becomes.as_ref().map(Cow::Borrowed)));
            }
            // [CR#702.131c]: a player-scope gain — the gaining player is the
            // carrier the pattern's `of` runs against.
            GameEvent::GotDesignation { player, name } => {
                v = FactView::bare(FactKind::DesignationChanged, state);
                v.designation = Some((Cow::Borrowed(name), None));
                v.patient = Some(Part::Player(*player));
                v.actor = Some(*player);
            }
            // [CR#701.7a,111.2]: the fact carries the token SPEC — no minted
            // object participant (the `TokenCreated:what` cap).
            GameEvent::TokenCreated { player, .. } => {
                v = FactView::bare(FactKind::TokenCreated, state);
                v.actor = Some(*player);
            }
            // [CR#608.2i]: the substantive ability-use fact — object-scoped
            // identity ([CR#400.7]), so the raw id is the read.
            GameEvent::AbilityUsed { object, .. } => {
                v = FactView::bare(FactKind::Used, state);
                v.object = Some(Part::Obj(*object));
            }
            // [CR#705.1]: the physical outcome, plus the call-relative
            // WIN/LOSS ([CR#705.2]) when the flip was called.
            GameEvent::CoinFlipped { player, won, .. } => {
                v = FactView::bare(FactKind::CoinFlipped, state);
                v.actor = Some(*player);
                v.won = *won;
            }
            GameEvent::DieRolled { player, result, .. } => {
                v = FactView::bare(FactKind::DiceRolled, state);
                v.actor = Some(*player);
                v.amount = Some(*result);
            }
            // Plumbing and information events no pattern atom watches.
            GameEvent::TurnBegan { .. }
            | GameEvent::TriggerFired { .. }
            | GameEvent::AbilityResolved(_)
            | GameEvent::AbilityCountered { .. }
            | GameEvent::DrewFromEmpty(_)
            | GameEvent::TokenCeased(_)
            // [CR#114.1]: getting an emblem is not a watchable zone change —
            // no "whenever you get an emblem" pattern exists yet.
            | GameEvent::EmblemCreated { .. }
            | GameEvent::PlayerLost { .. }
            | GameEvent::PlayerWon { .. }
            | GameEvent::ManaAdded { .. }
            | GameEvent::ManaEmptied { .. }
            | GameEvent::Revealed { .. }
            | GameEvent::Shuffled(_)
            | GameEvent::Unattached { .. }
            | GameEvent::DamageRemoved { .. } => return None,
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
/// [`FactKind`] downstream ([CR#603.6]): `ZoneWillChange` evolves into the
/// recorded `ZoneChanged`. Triggers fire on the FACT (the scan and the
/// trigger adapter refuse these — matching the intent would double-fire
/// every zone-move trigger), and history views are suppressed for them (the
/// downstream fact is the one counted); the REPLACEMENT lane is exactly where
/// they are matched ([CR#614]).
///
/// `Act(Destroy(x))` is NOT listed though it also evolves into a
/// `ZoneWillChange`: its own fact view is `FactKind::Act` (not `ZoneChange`),
/// so a "dies"/"destroyed" `ZoneChange` trigger never matches the intent — no
/// double-fire — while the cant/replacement lane matches it AS an `Act`.
pub(crate) fn shadowed_by_fact(event: &GameEvent) -> bool {
    matches!(event, GameEvent::ZoneWillChange { .. })
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
    match f {
        Predicate::Expanded(e) => deref_filter(&e.value),
        other => other,
    }
}

/// Looks through remembered `Reference` macros to the structural reference.
pub(crate) fn deref_reference(r: &Reference) -> &Reference {
    match r {
        Reference::Expanded(e) => deref_reference(&e.value),
        other => other,
    }
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
            // Look through a remembered macro invocation (`Dies`, `Enters`).
            EventFilter::Expanded(e) => self.eval(&e.value, fact, lane, bindings),

            // [CR#603.6]: zone constraints + cause narrowing + the moved
            // object's filter against its candidate view.
            EventFilter::ZoneChange {
                what,
                from,
                to,
                cause,
            } => {
                fact.kind == FactKind::ZoneChange
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

            // [CR#119.3].
            EventFilter::LifeGained { who, amount } => {
                fact.kind == FactKind::LifeGained
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && self.actor_matches(who, fact.actor, bindings)
            }
            EventFilter::LifeLost { who, amount } => {
                fact.kind == FactKind::LifeLost
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && self.actor_matches(who, fact.actor, bindings)
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

            // [CR#701]: the named keyword-action pre-intent — matched by the
            // `KeywordActionPattern` atom, decomposed per-verb. The verb TAG
            // fixes the name; the atom's `Predicate` arg narrows the fact's
            // performer (`actor`, for the player-report verbs) or patient
            // (`object`, for the object verbs). Count is not carried — a
            // keyword-action trigger matches any amount. Indestructible /
            // regeneration and `Cant(Act(…))` bite here.
            EventFilter::Act(pattern) => {
                use deckmaste_core::KeywordActionPattern as Kap;
                let named = |n: &str| fact.act_name.as_deref().map(Ident::as_str) == Some(n);
                fact.kind == FactKind::Act
                    && match pattern {
                        Kap::Scry(who) => {
                            named("Scry") && self.actor_matches(who, fact.actor, bindings)
                        }
                        Kap::Surveil(who) => {
                            named("Surveil") && self.actor_matches(who, fact.actor, bindings)
                        }
                        Kap::Fateseal(who) => {
                            named("Fateseal") && self.actor_matches(who, fact.actor, bindings)
                        }
                        Kap::Mill(who) => {
                            named("Mill") && self.actor_matches(who, fact.actor, bindings)
                        }
                        Kap::Draw(who) => {
                            named("Draw") && self.actor_matches(who, fact.actor, bindings)
                        }
                        Kap::Destroy(patient) => {
                            named("Destroy")
                                && self.part_matches(patient, fact.object.as_ref(), bindings)
                        }
                        Kap::Fight(a, b) => {
                            named("Fight")
                                && self.part_matches(a, fact.object.as_ref(), bindings)
                                && self.part_matches(b, fact.patient.as_ref(), bindings)
                        }
                    }
            }

            // [CR#122.1]: an omitted pattern `kind` watches any counter kind.
            EventFilter::CounterPlaced { kind, on, amount } => {
                fact.kind == FactKind::CounterPlaced
                    && kind
                        .as_ref()
                        .is_none_or(|r| Some(&r.0) == fact.counter.as_deref())
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
                    && self.part_matches(on, fact.object.as_ref(), bindings)
            }
            EventFilter::CounterRemoved { kind, on, amount } => {
                fact.kind == FactKind::CounterRemoved
                    && kind
                        .as_ref()
                        .is_none_or(|r| Some(&r.0) == fact.counter.as_deref())
                    && self.amount_ok(amount.as_ref(), fact.amount, bindings)
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
                fact.kind == FactKind::ActivatedAb
                    && self.actor_matches(who, fact.actor, bindings)
                    && self.part_matches(what, fact.object.as_ref(), bindings)
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
            // phasing/turn-face have no fact shape (P0.W6), so their
            // patterns match no record (kept caps `StateBecame:Phased`/
            // `:TurnedFace`).
            EventFilter::StateBecame { of, becomes } => {
                fact.kind == FactKind::StateBecame(becomes.clone())
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
            EventFilter::DesignationChanged { name, of } => {
                fact.kind == FactKind::DesignationChanged
                    && fact
                        .designation
                        .as_ref()
                        .is_some_and(|(n, _)| n.as_ref() == name)
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
                        Reference::This => self
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

            // [CR#106.12]: no engine fact for "a land was tapped for mana"
            // exists yet — `PlayerAction::AddMana`'s resolution never
            // records WHICH permanent produced the mana (a P0.W3-adjacent
            // seam, alongside the coin-flip/dice-roll apply seam below).
            // [CR#901.9]: nor does the Planechase planar die have one
            // (Plane cards — a different game-object type — aren't modeled
            // by this engine at all). Both never match: a documented
            // absent-subsystem fizzle, not a soundness claim.
            EventFilter::TapForMana { .. } | EventFilter::RollPlanarDie { .. } => false,

            // [CR#731.1a]: the day/night designation transitions.
            EventFilter::BecameDay => {
                fact.kind == FactKind::DesignationChanged
                    && fact.designation.as_ref().is_some_and(|(n, b)| {
                        n.as_str() == "DayNight"
                            && b.as_deref().is_some_and(|b| b.as_str() == "Day")
                    })
            }
            EventFilter::BecameNight => {
                fact.kind == FactKind::DesignationChanged
                    && fact.designation.as_ref().is_some_and(|(n, b)| {
                        n.as_str() == "DayNight"
                            && b.as_deref().is_some_and(|b| b.as_str() == "Night")
                    })
            }

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
                    .map(|frame| frame.controller)
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
        match part {
            None => matches!(deref_filter(filter), Predicate::Any),
            Some(Part::Obj(id)) => {
                if self.objects.get(*id).is_some() {
                    self.filter_matches_live(filter, *id, bindings.watcher)
                } else {
                    matches!(deref_filter(filter), Predicate::Any)
                }
            }
            Some(Part::Player(p)) => {
                self.filter_matches_live(filter, self.player(*p).object, bindings.watcher)
            }
            Some(Part::Gone(snapshot)) => {
                self.filter_matches_snapshot(filter, snapshot, bindings.watcher)
            }
        }
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
        self.history
            .in_window_for(
                within,
                self.turn.turn_number,
                self.turn.current,
                self.turn.active_player,
                self.controller_of_source(bindings.watcher)
                    .unwrap_or(self.turn.active_player),
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
    fn watcher_frame(&self, watcher: ObjectSource) -> Option<Frame> {
        self.objects
            .iter()
            .find(|ob| ob.source == watcher)
            .map(|ob| Frame::bare(ob.id, ob.controller))
    }
}
