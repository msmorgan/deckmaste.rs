use std::sync::Arc;

use deckmaste_core::ColorOrColorless;
use deckmaste_core::PhaseStep;
use deckmaste_core::Token;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;

/// Why a player lost ([CR#104.3,704.5]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LossReason {
    /// [CR#704.5a].
    LifeZero,
    /// [CR#704.5b].
    DrewFromEmpty,
    /// Ten or more poison counters ([CR#704.5c]; Two-Headed Giant swaps in
    /// the fifteen-counter TEAM check [CR#704.6b] — variant-gated).
    Poison,
    /// Concession ([CR#104.3a]) — immediate, any time, and UNSTOPPABLE:
    /// the single exception to card-beats-rules ([CR#101.1]); no `CantLose`
    /// gate touches it, and a controlled player's controller can't prevent
    /// it ([CR#723.6]).
    Conceded,
    /// "A player loses the game" effect outcome ([CR#104.3e]); suppressed by
    /// a matching `CantLose` gate.
    Effect,
}

/// A concrete occurrence: what `Emit` pushes through the (future) cant →
/// replace → apply pipe. Scheduled as an intent, returned from apply as the
/// occurred fact — the draw's library top binds at `Act(Draw)` apply time, and
/// a draw from an empty library applies as `DrewFromEmpty` instead.
/// The cause triple riding an event (mtg-rules events.md §3): the named
/// VERB view performed ("Sacrifice", "Discard", "Play", …), the AGENCY
/// that demanded it, and the AGENT — the causing object and its
/// controller, `None` for turn-based / state-based actions. Trigger
/// patterns (`CausePattern`) predicate over these coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cause {
    pub verb: deckmaste_core::Ident,
    pub agency: deckmaste_core::Agency,
    pub agent: Option<(ObjectId, PlayerId)>,
}

/// Named constructors for the cause triple. Each fixes the VERB spelling in
/// one place — a verb names a performed view ("Destroy", "Sacrifice", …) and a
/// typo'd verb silently never matches a trigger ([CR#603.2]), so the canonical
/// strings live here, not scattered across ~8 emission sites. The `agency`
/// coordinate stays a per-site argument: tap-for-cost ([CR#107.5]), tap-by-
/// effect ([CR#701.26a]), and tap-to-attack ([CR#508.1f]) are genuinely
/// different agencies of the same verb. `agent` is the causing object and its
/// controller, `None` for turn-based / state-based actions (mtg-rules
/// events.md §3). The open `Ident` verb vocabulary stays a seam.
impl Cause {
    /// The internal verb+agency+agent packing — every named constructor below
    /// routes through here so the only `Cause { … }` literal in the engine is
    /// this one.
    fn verb(
        verb: &str,
        agency: deckmaste_core::Agency,
        agent: Option<(ObjectId, PlayerId)>,
    ) -> Self {
        Cause {
            verb: verb.into(),
            agency,
            agent,
        }
    }

    /// "Destroy" ([CR#701.8a]) — one of "destroyed"'s exactly two causes
    /// ([CR#701.8b]; the other is the lethal-damage SBA, also this verb under
    /// `Agency::StateBasedAction`).
    #[must_use]
    pub fn destroy(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Destroy", agency, agent)
    }

    /// "Counter" ([CR#701.6a]) — a "becomes countered" view narrows by it.
    #[must_use]
    pub fn counter(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Counter", agency, agent)
    }

    /// "Tap" — distinguishable from the others only by `agency` ([CR#107.5]
    /// cost vs [CR#508.1f] attack vs [CR#701.26a] effect vs [CR#106.12] mana).
    #[must_use]
    pub fn tap(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Tap", agency, agent)
    }

    /// "Sacrifice" ([CR#701.21a]) — never a destruction (regeneration can't
    /// replace it).
    #[must_use]
    pub fn sacrifice(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Sacrifice", agency, agent)
    }

    /// "Discard" ([CR#701.9a]).
    #[must_use]
    pub fn discard(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Discard", agency, agent)
    }

    /// "Mill" ([CR#701.17a]) — the Library→Graveyard move's named view;
    /// "milled this way" reads find the moved cards through it ([CR#701.17c]).
    #[must_use]
    pub fn mill(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Mill", agency, agent)
    }

    /// "Draw" ([CR#121.1]) — the named view of a draw's Library → Hand move.
    /// The `Act(Draw)` apply tags the committed move with it so the SUCCESS
    /// fact (`FactKind::Drawn`, what "whenever you draw a card" reads) is
    /// distinguishable from a non-draw Library → Hand relocation ([CR#121.5]
    /// "without using the word draw").
    #[must_use]
    pub fn draw(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Draw", agency, agent)
    }

    /// "Play" ([CR#305.2,116.2a]) — the land-drop cause (an effect putting a
    /// land onto the battlefield is NOT a play, [CR#701.18a]).
    #[must_use]
    pub fn play(agency: deckmaste_core::Agency, agent: Option<(ObjectId, PlayerId)>) -> Self {
        Self::verb("Play", agency, agent)
    }

    /// `PutCounters` ([CR#122.1]) — putting counters on an object/player. A
    /// distinct verb from the spell-/ability-"Counter" above ([CR#701.6a]):
    /// these mark a permanent, they don't remove it from the stack.
    #[must_use]
    pub fn put_counters(
        agency: deckmaste_core::Agency,
        agent: Option<(ObjectId, PlayerId)>,
    ) -> Self {
        Self::verb("PutCounters", agency, agent)
    }

    /// `RemoveCounters` ([CR#122.1]) — removing counters from an object/player,
    /// whether by an effect or as the +1/+1 vs −1/−1 annihilation state-based
    /// action ([CR#704.5q], `Agency::StateBasedAction`).
    #[must_use]
    pub fn remove_counters(
        agency: deckmaste_core::Agency,
        agent: Option<(ObjectId, PlayerId)>,
    ) -> Self {
        Self::verb("RemoveCounters", agency, agent)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TurnBegan {
    pub player: PlayerId,
    pub turn: Uint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tapped {
    pub object: ObjectId,
    /// Tap causes are trigger-visible language ([CR#107.5] cost vs
    /// [CR#508.1f] attack vs [CR#701.26a] effect vs [CR#106.12] mana).
    pub cause: Option<Cause>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaAdded {
    pub player: PlayerId,
    pub mana: ColorOrColorless,
    pub amount: Uint,
    pub riders: Vec<deckmaste_core::ManaRider>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaEmptied {
    pub player: PlayerId,
    pub ending: deckmaste_core::PhaseStep,
}

/// [CR#701.7a,111.2]: `player` creates one token with the characteristics
/// `token` specifies. Its apply synthesizes a token entry in the card
/// table (owner = creator), mints the object straight onto the battlefield
/// (controller = creator), folds `AsEnters` self-replacements, and emits
/// the past-form `ZoneChange { from: None, to: Battlefield, .. }` fact so
/// enter-triggers fire. Creating N tokens is a `Batch` of N of these (one
/// instruction, simultaneous). "Whenever you create a token" triggers
/// match here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenCreated {
    pub player: PlayerId,
    pub token: Token,
}

/// [CR#114.1]: a player gets an emblem. Its apply synthesizes an
/// abilities-only def ([CR#114.3]) into the card table and mints the object
/// straight into the command zone, both owned and controlled by that player
/// ([CR#114.2]). Getting an emblem is not itself a zone-change — no
/// `ZoneChange` fact — but "whenever you get an emblem"-style triggers
/// would match here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmblemCreated {
    pub player: PlayerId,
    pub abilities: Vec<deckmaste_core::Ability>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerLost {
    pub player: PlayerId,
    pub reason: LossReason,
}

/// [CR#104.2b,104.1]: an effect-driven win. Applies by ending the game with
/// this player as the winner (others neither win nor lose) — distinct from
/// the derived last-player-standing win in `check_game_end`. A player who
/// would simultaneously win and lose loses instead ([CR#104.3f]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerWon {
    pub player: PlayerId,
}

/// A spell or ability was COPIED onto the stack ([CR#707.10] — a copy
/// is put on the stack, not cast). `copy` is filled by the apply (the
/// minted entry); `original` is the copied stack object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Copied {
    pub original: ObjectId,
    pub copy: Option<ObjectId>,
    pub controller: PlayerId,
}

/// [CR#602.2a] — an ability becomes activated. Applies by minting the
/// stack identity, promoting `announcing` onto the stack, and bumping the
/// activation ledger. The "whenever … activates an ability" trigger seam
/// (engine-trigger-events).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityActivated {
    pub source: ObjectId,
    pub ability: usize,
}

/// [CR#120.3] — damage to a creature (marked) or a player (life loss).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageDealt {
    pub source: ObjectId,
    pub target: ObjectId,
    pub amount: Uint,
    /// Combat damage ([CR#510.1]) vs everything else — the combat-damage
    /// step's assignments set it; effect damage (including a fight's,
    /// [CR#701.14d]) is `false`. The `Damage:combat` pattern refinement
    /// reads it.
    pub combat: bool,
}

/// A zone change ([CR#400.7,603.6]) — ONE phase-explicit event covering
/// both the INTENT (replaceable, not yet recorded) and the FACT
/// (unreplaceable, recorded, trigger-visible). `snapshot.is_none()` is
/// the FUTURE form: a live object is still about to move, replacements
/// act here ([CR#614]), and it is neither recorded to history nor
/// trigger-scanned. `apply` captures the object's LKI the instant before
/// the move, fills `snapshot` (`Some`), and the SAME variant becomes the
/// PAST form — the fact triggers (later tasks) fire on. The LKI-snapshot
/// / immediate-move semantics are unchanged from the old
/// `ZoneWillChange`/`ZoneChanged` pair ([CR#603.10a]): the move+remint
/// still happens atomically inside `apply`, not at some later "commit"
/// step. `enters` is present only when `to == Battlefield`. `position`
/// is present only when `to == Library`: the insertion index counted
/// from the top (`0` = top), clamped to the bottom when the library is
/// shorter ([CR#401.7]); `None` means the top.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneChange {
    /// The moved object's id. On the future form this is the still-live
    /// id about to move; on the past form it is the same (now-stale)
    /// id — `snapshot.object` mirrors it once `snapshot` is filled.
    pub object: ObjectId,
    /// `None` before apply (future/replaceable, unrecorded); `Some`
    /// after (past/recorded) — the moved object's LKI, captured the
    /// instant before the move ([CR#603.10a]). Boxed to keep the common
    /// non-zone-change arms of `GameEvent` cheap.
    pub snapshot: Option<Box<crate::lki::LkiSnapshot>>,
    pub from: Option<Zone>,
    pub to: Zone,
    pub enters: Option<EnterStatus>,
    pub position: Option<Uint>,
    /// The face shown on arrival — the master event's `face`
    /// coordinate; `None` = the default, face up ([CR#110.5b]). No
    /// emitter sets `Down` yet (morph/manifest are post-P0 macros);
    /// reveal-on-leave ([CR#708.9]) hooks here when they do.
    pub face: Option<deckmaste_core::Face>,
    /// `None` = an unattributed move; named views (sacrificed,
    /// discarded, played) ride here as cause triples.
    pub cause: Option<Cause>,
}

/// [CR#119.3]: a player loses life directly (not via damage).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifeLost {
    pub player: PlayerId,
    pub amount: Uint,
}

/// [CR#119.3]: a player gains life.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifeGained {
    pub player: PlayerId,
    pub amount: Uint,
}

/// [CR#508.1a,508.1b]: `attacker` was declared as an attacker attacking
/// `defending` — the defending player's proxy object or a planeswalker they
/// control ([CR#506.3,508.1b]). Its apply records both in `CombatState` and
/// taps the attacker ([CR#508.1f]). The "whenever ~ attacks" trigger seam
/// (`EventFilter::AttackDeclared`) reads `defending` as the against-target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attacking {
    pub attacker: ObjectId,
    pub defending: ObjectId,
}

/// [CR#509.1a]: a creature was declared as a blocker against `attacker`. Its
/// apply records the block in `CombatState` and marks `attacker` blocked
/// ([CR#509.1h]). The "whenever ~ blocks / becomes blocked" trigger seam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blocked {
    pub blocker: ObjectId,
    pub attacker: ObjectId,
}

/// A coin flip's outcome ([CR#705.1..705.2]). `won` is `Some` only for a
/// CALLED flip (the flipper called heads or tails and won or lost the
/// flip); `None` when the effect reads only heads/tails and no player
/// wins or loses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoinFlipped {
    pub player: PlayerId,
    pub heads: bool,
    pub won: Option<bool>,
}

/// A die roll's outcome ([CR#706.1..706.2]); an IGNORED roll is
/// considered never to have happened — no triggers ([CR#706.6]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DieRolled {
    pub player: PlayerId,
    pub sides: Uint,
    pub natural: Uint,
    pub result: Uint,
}

/// Counters placed on an object or player proxy ([CR#122.1]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterPlaced {
    pub object: ObjectId,
    pub kind: deckmaste_core::Ident,
    pub amount: Uint,
    /// The carrier's total of `kind` BEFORE this placement — filled at
    /// apply (emitters leave `0`), alongside `after`, so a chapter
    /// ability's `Crossed` gate reads "the total was less than N and
    /// became at least N" off the one fact [CR#714.2b] — a doubled
    /// placement is still ONE fact whose `after - before` is the doubled
    /// amount.
    pub before: Uint,
    /// The carrier's total of `kind` AFTER this placement (apply-filled,
    /// like `before`) [CR#714.2b].
    pub after: Uint,
    pub cause: Option<Cause>,
}

/// Counters removed ([CR#122.1]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterRemoved {
    pub object: ObjectId,
    pub kind: deckmaste_core::Ident,
    pub amount: Uint,
    pub cause: Option<Cause>,
}

/// [CR#603.2]: a triggered ability triggered. Its apply notes it into
/// `pending_triggers`. Routed as an event so Stage-4 replacements/cant can
/// intercept (Panharmonicon/Hushwing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerFired {
    pub source: ObjectSource,
    pub ability: Uint,
    pub controller: PlayerId,
    /// `Some` for a delayed/reflexive ([CR#603.7,603.12]) trigger created
    /// at resolution — its by-value body, printed on no permanent. `None`
    /// for a printed trigger (`ability` indexes `abilities_of_source`).
    pub created: Option<Arc<deckmaste_core::TriggeredAbility>>,
    /// Boxed: `TriggerBindings` carries three LKI snapshots (~280 B) and
    /// dominated `GameEvent`'s size, cascading through every by-value event
    /// move in `step()`; a trigger fires far less often than events move, so
    /// the box allocates off the hot path. See `engine-event-size-boxing`.
    pub bindings: Box<crate::trigger::TriggerBindings>,
}

/// A triggered ability fired ([CR#603.2]) or an activated ability became
/// activated ([CR#602.2a]); the substantive "use" fact backing use-limit
/// and `EventCount` history reads ([CR#608.2i]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityUsed {
    pub object: ObjectId,
    pub ability: Uint,
}

/// [CR#608.2n]: a triggered or activated ability fizzled (an
/// intervening-if no longer held, or every target went illegal) and
/// vanishes — no zone move. [CR#701.6a]: a triggered or activated
/// ability was countered and vanishes the same way. [CR#707.10a]: a
/// countered COPY (of a spell or ability) also vanishes here — same
/// shape, no card behind it either. Its apply removes the stack entry
/// whose `id` is the carried (minted, for a spell copy freshly minted)
/// token, and the backing object with it — no remint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityCountered {
    pub id: ObjectId,
    pub cause: Cause,
}

/// Cards shown ([CR#701.20a]); `to: None` = revealed to ALL players,
/// `Some` = "look at" — the same operation shown to a subset
/// ([CR#701.20e]). Revealing never moves the card ([CR#701.20b]).
/// The reveal window lasts for the containing resolving instruction;
/// revealing itself changes no game-state characteristic or zone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Revealed {
    pub objects: Vec<ObjectId>,
    pub to: Option<Vec<PlayerId>>,
}

/// A GAME-scope designation transition in the W5 registry (day/night,
/// [CR#731.1] — "day becomes night" = losing one designation and
/// gaining the other, [CR#731.1a]). Shaped, unbuilt: designation
/// GRANTING effects are P0.W5/W6 seams. Object/player designation
/// deltas ride their own facts when granting lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignationChanged {
    pub name: deckmaste_core::Ident,
    pub becomes: Option<deckmaste_core::Ident>,
}

/// A player GAINED a player-scope designation ([CR#702.131c] — the city's
/// blessing). The object/player-scope counterpart to `DesignationChanged`
/// (which is game-scope). Idempotent at apply: a player who already holds
/// `name` is unchanged. The `GetDesignation` verb suppresses re-emission,
/// so this fact marks a genuine first acquisition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotDesignation {
    pub player: PlayerId,
    pub name: deckmaste_core::Ident,
}

/// An object became the target of the spell/ability `source` at
/// announce ([CR#601.2c]; ward is the family exemplar [CR#702.21a]).
/// Shaped, unbuilt: the announce flow emits it (P0.W7 seam).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BecameTarget {
    pub target: ObjectId,
    pub source: ObjectId,
}

/// An object changed controller — a becomes-delta, never a zone move
/// (the object keeps its identity). Shaped, unbuilt: control-changing
/// continuous effects are a layers seam (L2); its apply will re-home
/// the object and fire `EventFilter::ControlChanged` patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlChanged {
    pub object: ObjectId,
    pub to: PlayerId,
}

/// An attachment became attached to a host ([CR#701.3a]; a re-attach is a
/// new timestamp per [CR#701.3c]). A fact for "whenever ~ becomes
/// attached / equipped"; the relation mutation happens in the `Attach`
/// verb's resolution, so apply only records this. Trigger-matching breadth
/// is a seam (events are shaped, §9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attached {
    pub attachment: ObjectId,
    pub host: ObjectId,
}

/// An attachment became unattached ([CR#701.3d]). A fact, mirroring
/// [`Attached`](GameEvent::Attached); apply only records it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unattached {
    pub attachment: ObjectId,
    pub former_host: ObjectId,
}

/// All marked damage was removed from `object` ([CR#614.8,701.19a] —
/// the regeneration heal clause). Applied by zeroing `damage` on the
/// object and removing it from combat ([CR#701.19a] "remove from combat").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DamageRemoved {
    pub object: ObjectId,
}

/// A NAMED keyword action ([CR#701]) — the ONE present-tense event that
/// serves both roles: the guardable/replaceable moment AND the "it
/// happened" fact. `verb` is the printed keyword name-atom
/// (`Scry`/`Destroy`/…, the retired-`CauseVerb` [`VerbName`] namespace);
/// `who` is the RESOLVED performing player for the player-report verbs
/// (scry/surveil/fateseal/mill/draw) — the actor "whenever an opponent
/// draws" reads; `on` is the RESOLVED patient object SUBJECT(S) for the
/// object verbs (destroy/fight).
/// Matched by the [`EventFilter::Act`](deckmaste_core::EventFilter::Act)
/// master form (verb + who/on/cause) on BOTH
/// lanes: a `Cant(Destroy(…))` static ([CR#702.12b]) suppresses the whole
/// action so its body never runs ([CR#701.22b]), while a surviving `Act` in
/// the log is the "whenever you scry/surveil/…" trigger fact
/// ([CR#701.22d]) — no separate post-fact. RESULT-side
/// "destroyed"/"milled"/"drawn" triggers still key on the body's past-form
/// `ZoneChange`/`Drawn` fact ([CR#700.4]). A move-verb (`Act(Destroy)`)
/// carries its body facet in `from`/`to` and COMMITS that move directly on
/// apply — ONE dual-facet event through cant→replace→apply, never a second
/// replaceable future-form `ZoneChange` below it ([CR#616.1]); the reorder
/// verbs (scry/surveil/fateseal) run their body ahead of a `None`-shape
/// post-fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Act {
    pub verb: deckmaste_core::VerbName,
    pub who: Option<PlayerId>,
    /// The RESOLVED object SUBJECT(S) the verb acts on ([CR#701]):
    /// destroy/bound-discard carry one, the player-report verbs none, and
    /// Fight BOTH combatants ([CR#701.14a] — the window is symmetric; the
    /// committed past fact is emitted once per subject, `finalize_act`).
    /// Empty ≙ no object subject; every verb but Fight is `len() <= 1`.
    pub on: Vec<ObjectId>,
    /// The BODY facet ([CR#603.6]) — the composite's canonical realized
    /// zone-change, derived from the stored `Move` body BEFORE it runs
    /// (`Destroy(x)` → `from: Battlefield, to: Graveyard`). A move-verb
    /// carries `Some`; the apply COMMITS this move directly (atomic — no
    /// separate replaceable future-form `ZoneChange` below it). A reorder
    /// verb (scry/surveil/fateseal) reorders WITHIN a zone and
    /// carries `None`, so a `ZoneChange(→Graveyard)` query never
    /// matches it. Paired with the tag facet (`verb`/`on`/`who`) so
    /// ONE event is matchable on both ([CR#616.1]).
    pub from: Option<Zone>,
    pub to: Option<Zone>,
    /// The causal context this keyword action rides ([CR#701.8b]) — its
    /// `agency`/`agent`, used to tag the committed move the move-verb apply
    /// performs ([CR#701.8a]). `verb` duplicates the atom's name; a `None`
    /// cause is a keyword action with no cause-tagged move
    /// (scry/surveil reorder within a zone).
    pub cause: Option<Cause>,
    /// The phase marker ([CR#616.1], mirroring `ZoneChange`'s `snapshot`
    /// law): `false` = the FUTURE, replaceable window — the ONE cant →
    /// replace moment for every description of this keyword action; `true`
    /// = the committed, trigger-visible PAST fact, recorded by
    /// `FinalizeAct` only once the verb's characteristic change actually
    /// committed. `replaceable()` gates on `committed: false`;
    /// `record_history`/`scan_triggers` skip it; a committed `Act` opens no
    /// window.
    pub committed: bool,
    /// The keyword action's unresolved CONTENTS + resolution context —
    /// carried ONLY on the future form of a verb whose apply must unwrap a
    /// body it cannot reconstruct from the flat coordinates: mill's
    /// top-slice group derivation ([CR#701.17a]) and the scry family's
    /// arrange `RunEffect` ([CR#701.22a]). `None` for the single-move verbs
    /// (destroy/discard commit `on`/`from`/`to` directly) and draw (its
    /// apply late-binds the library top), and always `None` once committed
    /// (the recorded fact carries no resolution context — it never enters
    /// `FactView`). Also carried by the [`OneShotEffect::Batch`]
    /// aggregate window (`batch: Some(n)` below): `body` there is the
    /// stored PER-UNIT keyword action, replicated `n` times once the
    /// window passes.
    pub contents: Option<Box<ActContents>>,
    /// [CR#616.1g,121.2a]: `Some(n)` on the ONE aggregate window a
    /// `Batch(n, keyword-action)` resolve builds
    /// ([`OneShotEffect::Batch`]), carrying the batch cardinality
    /// so a count-multiplying replacement (Bruvac-style "mill twice
    /// as many") can read/ rewrite it via `Count::ThatMany`
    /// (`intent_magnitude`'s batch arm, [CR#107.3]) BEFORE any of
    /// the n contained per-entity futures exists ([CR#616.1g]: the
    /// outer effect is chosen before the inner one). `None` for
    /// every ordinary (non-aggregate) keyword-action
    /// window — including each of the n per-entity futures a PASSED
    /// aggregate's apply schedules, which are ordinary windows in their
    /// own right.
    pub batch: Option<Uint>,
    /// [CR#614.5]: the replacement lineage this window's OWN
    /// `replace_event` loop starts pre-applied with — "a replacement
    /// effect doesn't invoke itself repeatedly; it gets only one
    /// opportunity to affect an event OR ANY MODIFIED EVENTS THAT MAY
    /// REPLACE THAT EVENT." Populated by `schedule_body` (an `Instead`/
    /// `Also` body that re-emits an event of the SAME shape its own
    /// replacement watches must not be caught by it again) and by a
    /// PASSED aggregate `Batch` window's apply (the aggregate's own
    /// inherited-plus-applied set rides into each of the n contained
    /// per-entity futures it schedules, so THEY aren't re-caught either
    /// — the Archive Trap shape: "draw 2" instead of an infinite
    /// "draw 2, which becomes draw 2, which becomes …"). Empty for the
    /// overwhelming majority of keyword-action windows; always empty
    /// once `committed` (resolution plumbing, dropped like `contents`).
    pub inherited: std::collections::HashSet<crate::replace_registry::ReplacementKey>,
    /// [CR#616.1g,121.2a]: `true` on each of the `n` per-entity futures a
    /// PASSED aggregate `Batch` window's apply schedules — never on the
    /// aggregate itself. A "whenever you Verb" ACT-level trigger
    /// ([CR#701.22d]'s "fires after the process is complete" timing,
    /// e.g. "whenever you mill one or more cards") reads the
    /// count-tier's `Batch` as ONE instruction, so `finalize_act`
    /// suppresses a `contained` window's own committed-fact emission
    /// (the aggregate's own `FinalizeAct{AnyContained}` is the ONE
    /// trigger-visible commit for the whole batch). This does NOT touch
    /// the underlying `ZoneChange`/`Drawn` fact each contained future's
    /// apply still commits directly — a RESULT-side "whenever a card is
    /// milled/drawn" trigger ([CR#700.4]) keys on THAT, unaffected, so
    /// it still fires once per card exactly as it would outside a
    /// `Batch` ([CR#121.2] draw is genuinely per-card). `false` for
    /// every ordinary (non-contained) keyword-action window.
    pub contained: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    TurnBegan(TurnBegan),
    StepBegan(PhaseStep),
    Untapped(ObjectId),
    /// A permanent turned to its other face ([CR#701.27a]) — carries the
    /// object, like `Untapped`. Apply toggles its `Side`; identity is
    /// preserved ([CR#712.18]).
    Transformed(ObjectId),
    // No intent event is a bespoke variant: destruction, draw, AND discard are
    // keyword actions (above). Discard is the per-card `Act(Discard)`
    // ([CR#701.9a]) — the batched hand choice realizes one dual-facet event per
    // card, each committing its Hand → Graveyard move atomically; madness
    // ([CR#702.35a]) replaces it (the role the retired `WillDiscard`
    // placeholder reserved). Destroy is `Act(Destroy(x))`, a dual-facet event
    // whose apply COMMITS the Battlefield → Graveyard move directly (carrying
    // its `cause`, [CR#701.8a,701.8b]) — no separate replaceable future-form
    // `ZoneChange` below it; indestructible ([CR#702.12b]) cants it and regeneration
    // ([CR#701.19a]) replaces it, both keyed on `Act(Destroy(…))`. Draw is the
    // atomic single-card `Act(Draw)` ([CR#121.1,121.2]): its apply binds the
    // library top LATE and either commits the Library → Hand move (tagged
    // `cause: Draw`, the success fact) or sets `drew_from_empty`
    // ([CR#121.4,704.5b]) — "draw N" is `Repeat(n, Draw)`. A future draw
    // replacement (Notion Thief, Lab Maniac) keys on `Act(Draw)`.
    /// The failed-draw fact ([CR#121.4,704.5b]): a draw attempt over an empty
    /// library. Set by the `Act(Draw)` apply and by any direct emitter (a
    /// replacement rewriting a draw); the loss SBA keys on the flag it sets.
    DrewFromEmpty(PlayerId),

    Act(Act),

    Tapped(Tapped),
    ManaAdded(ManaAdded),
    ManaEmptied(ManaEmptied),

    TokenCreated(TokenCreated),
    EmblemCreated(EmblemCreated),
    /// [CR#704.5d,111.7]: a token found in a zone other than the battlefield
    /// ceases to exist. Its apply removes the object from its zone and the
    /// store outright — no remint, no `ZoneChange` fact (the token doesn't
    /// move, it stops existing). Zone-leave triggers already fired at the
    /// move that stranded it ([CR#111.7]'s note); anything still pointing at
    /// it reads the LKI that rode that fact.
    TokenCeased(ObjectId),
    PlayerLost(PlayerLost),
    PlayerWon(PlayerWon),
    /// [CR#601.2i] — a spell becomes cast. Applies by promoting `announcing`
    /// onto the stack. The Stage-3 "whenever you cast" seam.
    SpellCast(ObjectId),
    Copied(Copied),
    AbilityActivated(AbilityActivated),
    DamageDealt(DamageDealt),
    ZoneChange(ZoneChange),
    LifeLost(LifeLost),
    LifeGained(LifeGained),
    Attacking(Attacking),
    Blocked(Blocked),
    CoinFlipped(CoinFlipped),
    DieRolled(DieRolled),
    CounterPlaced(CounterPlaced),
    CounterRemoved(CounterRemoved),
    TriggerFired(TriggerFired),
    AbilityUsed(AbilityUsed),
    AbilityCountered(AbilityCountered),
    /// [CR#608.2n]: a triggered or activated ability finished resolving and
    /// vanishes — no zone move. [CR#707.10a]: a RESOLVED copy (of a spell)
    /// vanishes the same way instead of moving to a graveyard — it has no
    /// card to put there. Its apply removes the stack entry whose `id` is
    /// the carried (minted) token, and the backing object with it.
    AbilityResolved(ObjectId),
    Revealed(Revealed),
    DesignationChanged(DesignationChanged),
    GotDesignation(GotDesignation),
    /// A library was shuffled ([CR#701.24a]) — an INFORMATION event:
    /// order knowledge is destroyed for every player; revealed cards in
    /// it stop being revealed and become new objects ([CR#701.20d] —
    /// revealed-state reset is a P0.W6 seam). Why library actions never
    /// rewind: [CR#733.1].
    Shuffled(PlayerId),
    BecameTarget(BecameTarget),
    ControlChanged(ControlChanged),
    Attached(Attached),
    Unattached(Unattached),
    DamageRemoved(DamageRemoved),
}

/// A future keyword action's unresolved contents + the resolution frame it was
/// scheduled in ([CR#603.6] — a keyword action IS its contents). Rides
/// [`GameEvent::Act::contents`] so the apply half can unwrap a body the flat
/// `Act` coordinates can't reconstruct: mill derives its top-slice group and
/// commits the batch ([CR#701.17a]); the scry family schedules its arrange as
/// agenda work ([CR#701.22a] — a decision can't complete inside an apply). The
/// frame anchors the body's references (`You`/`It`/`This`, `Count`s). Dropped
/// once the action commits — the recorded past fact carries no resolution
/// context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActContents {
    pub body: deckmaste_core::OneShotEffect,
    pub frame: crate::stack::Frame,
}

/// How a permanent enters the battlefield ([CR#110.5] status; face-down is
/// later). Present on a `ZoneChange` only when `to == Battlefield`. (Not
/// `Copy`: it carries the enters-with-counters list.)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnterStatus {
    pub tapped: bool,
    /// The host this permanent enters attached to ([CR#303.4]), folded in from
    /// its own `AsEnters(Attach(This, …))` self-replacement so it enters
    /// attached atomically — no observable unattached window. `None` = enters
    /// unattached. Resolved against the entering object's id once it is minted
    /// (the attach is part of the entering `ZoneMove`, not a post-entry
    /// trigger).
    pub attach_to: Option<ObjectId>,
    /// Counters the permanent enters with ([CR#122.6a,614.1c]) — `(kind,
    /// count)` pairs folded in from its own `AsEnters(PutCounters(This,
    /// kind, n))` self-replacement, placed atomically at mint before the
    /// past-form `ZoneChange` fact (so no observable counterless window, and
    /// the entering P/T already reflects them).
    pub counters: Vec<(deckmaste_core::Ident, deckmaste_core::Uint)>,
}

/// Who learns an event's full payload — the projection-boundary annotation
/// the per-player view (a RUNNER concern; the engine stays
/// full-information) consumes. mtg-rules information.md §6: replay,
/// netplay, and AI all read this layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Audience {
    /// Everyone sees the whole payload ([CR#400.2] public-zone default).
    Public,
    /// The full payload is for these players; everyone else learns only
    /// THAT the event happened (shape and count, not identity — a hidden
    /// zone's size is public, [CR#401.3,402.3]).
    Restricted(Vec<PlayerId>),
}

impl GameEvent {
    /// The event's audience, DERIVED from kind + payload (the
    /// schema-from-kind pattern — no stored field to drift). Coarse first
    /// pass: a move between two hidden zones discloses the card's identity
    /// to its owner alone ([CR#400.2] — a draw, a mulligan bottoming);
    /// either endpoint public makes the identity public history (a
    /// discard arrives face up in a public zone) even though a hidden
    /// destination then conceals the card. Subset reveals restrict to the
    /// named players ([CR#701.20e]). Refinements — face-down commitments
    /// ([CR#708.2]), stateful look grants ([CR#406.3]) — arrive with
    /// their machinery.
    #[must_use]
    pub fn audience(&self, state: &crate::state::GameState) -> Audience {
        match self {
            GameEvent::ZoneChange(ZoneChange {
                object,
                snapshot: None,
                from: Some(from),
                to,
                ..
            }) if from.is_hidden() && to.is_hidden() => {
                Audience::Restricted(vec![state.owner_of(*object)])
            }
            GameEvent::ZoneChange(ZoneChange {
                snapshot: Some(snapshot),
                from: Some(from),
                to,
                ..
            }) if from.is_hidden() && to.is_hidden() => match snapshot.source {
                ObjectSource::Card(card) => Audience::Restricted(vec![state.cards.get(card).owner]),
                ObjectSource::Player(p) => Audience::Restricted(vec![p]),
            },
            GameEvent::Revealed(Revealed {
                to: Some(players), ..
            }) => Audience::Restricted(players.clone()),
            _ => Audience::Public,
        }
    }
}

/// A scheduled occurrence: one event, or a set of simultaneous events applied
/// and matched together ([CR#603.3b], [CR#700.1]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Occurrence {
    Single(GameEvent),
    Batch(Vec<GameEvent>),
}

impl Occurrence {
    /// Convenience: wrap a single event.
    #[must_use]
    pub fn single(event: GameEvent) -> Self {
        Occurrence::Single(event)
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Agency;

    use super::Cause;
    use crate::object::ObjectId;
    use crate::player::PlayerId;

    /// Each named constructor fixes its canonical verb spelling and threads the
    /// per-site `agency` and `agent` through unchanged.
    #[test]
    fn named_constructors_fix_verb_and_thread_agency_agent() {
        let agent = Some((ObjectId::from_raw(7), PlayerId(1)));

        assert_eq!(
            Cause::destroy(Agency::StateBasedAction, None),
            Cause {
                verb: "Destroy".into(),
                agency: Agency::StateBasedAction,
                agent: None,
            }
        );
        assert_eq!(
            Cause::destroy(Agency::EffectInstruction, agent),
            Cause {
                verb: "Destroy".into(),
                agency: Agency::EffectInstruction,
                agent,
            }
        );
        assert_eq!(
            Cause::counter(Agency::EffectInstruction, agent)
                .verb
                .as_str(),
            "Counter"
        );
        assert_eq!(
            Cause::sacrifice(Agency::EffectInstruction, agent)
                .verb
                .as_str(),
            "Sacrifice"
        );
        assert_eq!(
            Cause::discard(Agency::EffectInstruction, None)
                .verb
                .as_str(),
            "Discard"
        );
        assert_eq!(
            Cause::play(Agency::SpecialAction, None).verb.as_str(),
            "Play"
        );

        // "Tap" is distinguished only by agency — same verb, different agency.
        let cost_tap = Cause::tap(Agency::CostPayment, agent);
        let effect_tap = Cause::tap(Agency::EffectInstruction, agent);
        assert_eq!(cost_tap.verb, effect_tap.verb);
        assert_eq!(cost_tap.verb.as_str(), "Tap");
        assert_ne!(cost_tap.agency, effect_tap.agency);
    }
}
