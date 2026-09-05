use std::collections::VecDeque;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_core::BeginningStep;
use deckmaste_core::EndingStep;
use deckmaste_core::Int;
use deckmaste_core::PhaseStep;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use rand::RngExt;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::agenda::WorkItem;
use crate::combat::CombatState;
use crate::decide::DecisionPointKind;
use crate::event::GameEvent;
use crate::layer::ContinuousEffect;
use crate::object::Cards;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::object::ObjectStore;
use crate::player::PlayerId;
use crate::player::PlayerState;
use crate::stack::PendingStackEntry;
use crate::stack::StackEntry;
use crate::turn::TurnState;
use crate::zone::Zones;

/// One designation entry's value at game/player scope: a bare flag
/// (city's blessing), a unique holder (monarch, initiative), or a named
/// mode (day/night).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesignationValue {
    Flag,
    Holder(PlayerId),
    Mode(deckmaste_core::Ident),
}

/// One INSTANCE of an object-scope designation. The declaration's payload
/// (core `DesignationDecl.payload: Vec<StaticSpec>`) is a TEMPLATE; the
/// instance supplies its bindings: the grantor (goad's "attacks a player
/// other than [the goader]", [CR#701.15b..701.15c]) and the duration
/// ("until your next turn"). Multiple goaders = multiple instances, each
/// expiring on its own clock — never a merged set. Payload application is
/// the layers pipeline's designation source (seam); duration sweep
/// rides the effect-instance machinery (seam).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignationInstance {
    pub grantor: Option<PlayerId>,
    pub duration: Option<deckmaste_core::Duration>,
}

/// The generic designation registry — storage mirrors the data-driven
/// declaration model rather than per-mechanic fields; granting effects are
/// unbuilt seams, but `Designated(name)` filter reads are LIVE against it
/// (an empty store correctly means nothing is goaded/suspected/…).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DesignationStore {
    pub game: std::collections::HashMap<deckmaste_core::Ident, DesignationValue>,
    pub players: std::collections::HashMap<(PlayerId, deckmaste_core::Ident), DesignationValue>,
    pub objects:
        std::collections::HashMap<(ObjectId, deckmaste_core::Ident), Vec<DesignationInstance>>,
}

/// Transient combat-damage assignment ([CR#510.1]): the partial state that
/// accumulates across one or more `AssignCombatDamage` decisions before the
/// single simultaneous batch is dealt ([CR#510.2]). `Some` only between the
/// Combat Damage step's handler opening the first assignment decision and the
/// last one being answered (the trigger/cast analogue of `announcing`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombatDamage {
    /// Every `DamageDealt` chosen or forced so far, dealt together when the
    /// queue empties.
    pub buffer: Vec<crate::event::GameEvent>,
    /// Sources still owing a free-division decision (≥ 2 recipients), in
    /// declaration order. The front is the one currently surfaced.
    pub queue: Vec<PendingAssignment>,
}

/// One source whose combat-damage division is still pending: the source's
/// object id, its power, and its (≥ 2) live recipients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingAssignment {
    pub source: ObjectId,
    pub power: Uint,
    pub recipients: Vec<ObjectId>,
}

/// How the game ended ([CR#104]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOutcome {
    /// [CR#104.2a]: the last player standing.
    Win(PlayerId),
    /// [CR#104.4].
    Draw,
}

/// One player's setup.
#[derive(Debug, Clone)]
pub struct PlayerConfig {
    pub deck: Vec<Arc<Card>>,
}

/// Who takes the first turn.
#[derive(Debug, Clone, Copy)]
pub enum StartingPlayer {
    Fixed(PlayerId),
    /// Decided by the seeded rng.
    Random,
}

/// Game setup: decklists, the rng seed, and the pre-game constants.
#[derive(Debug, Clone)]
pub struct GameConfig {
    pub players: Vec<PlayerConfig>,
    pub seed: u64,
    pub starting_life: Int,
    pub starting_player: StartingPlayer,
    pub sba_rules: Vec<deckmaste_core::SbaRule>,
    pub conferral_rules: Vec<deckmaste_core::ConferralRule>,
    pub damage_result_rules: Vec<deckmaste_core::DamageResultRule>,
    pub counter_decls: std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::Counter>,
    /// The subtype registry ([CR#205.3]): `Ident → Subtype`. Unlike a counter's
    /// confers (looked up by name), a subtype's confers normally ride the card
    /// value — but a layer-4 `Subtypes(...)` modification carries only bare
    /// `Ident` names, so the engine needs this registry to resolve them back to
    /// `Subtype` structs (with their `confers`/`types`). Populated from the
    /// loaded plugin's `subtypes` at construction; empty means a granted
    /// subtype resolves to a minimal name-only `Subtype` (no inherent
    /// rules).
    pub subtypes: std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::Subtype>,
    /// The card-type registry ([CR#300.1]): `Ident → TypeDef`. Like subtypes,
    /// a `TypeDef`'s confers normally ride the card value — but a layer-4
    /// `CardTypes(...)` modification carries only bare `Ident` names, so the
    /// engine needs this registry to resolve them back to `TypeDef` structs
    /// (with their `permanent_type`/`confers`). Populated from the loaded plugin's
    /// `types` at construction; empty means a granted type resolves to a
    /// minimal name-only `TypeDef` (no inherent rules).
    pub types: std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::TypeDef>,
}

/// What to resume once a resolution-time decision is answered. Transient: set
/// while that decision is pending (alongside `pending`), taken on submit. The
/// resolution-time analogue of the `announcing` cast slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionContinuation {
    /// [CR#601.2b,702.33d]: the pending `YesNo` answers "pay this tagged
    /// optional cost (again)?" for the in-flight announce. Yes records the
    /// tag (+1) and adds `components` to the total ([CR#601.2f]); a
    /// repeatable row ([CR#702.33c] multikicker) re-offers the same row,
    /// otherwise the walk advances to the next declared row.
    OptionalCost {
        tag: deckmaste_core::CostTag,
        components: Vec<deckmaste_core::CostComponent>,
        repeatable: bool,
        index: usize,
    },
    /// A `ChooseModes` answer for the spell or activated ability currently
    /// being announced ([CR#601.2b,602.2b,700.2]). The answer is retained on
    /// the announce slot; later target and cost steps derive only from it.
    AnnounceModes,
    /// A `ChooseObjects` answer ([CR#608.2d]): write the picks to `dest`, then
    /// continue with `if_none` when the choice produced no objects.
    BindChoice {
        dest: deckmaste_core::DefId,
        frame: crate::stack::ExecutionFrame,
        if_none: deckmaste_core::Block,
    },
    BindNumber {
        dest: deckmaste_core::DefId,
        activation: crate::ActivationId,
    },
    BindSymbol {
        dest: deckmaste_core::DefId,
        activation: crate::ActivationId,
    },
    /// A `YesNo` answer for `Instruction::May` ([CR#118.12]): true → `effect`
    /// then `if_did`; false → `if_not` (or nothing).
    May {
        may: deckmaste_core::May,
        frame: crate::stack::ExecutionFrame,
    },
    /// A `ChooseModes` answer for `Instruction::Modal` ([CR#700.2]): run the
    /// chosen modes' effects in the order they were picked.
    Modal {
        modes: Vec<deckmaste_core::Mode>,
        frame: crate::stack::ExecutionFrame,
    },
    /// [CR#401.4]: walking the post-pick arrange decisions — `current` is the
    /// pile whose order choice is open, `remaining` the piles still to arrange.
    /// An `Arranged` answer reorders `current` in its library, then surfaces
    /// the next `remaining` pile (or finishes).
    ArrangePiles {
        current: ArrangePile,
        remaining: Vec<ArrangePile>,
    },
    /// A `CallFlip` answer ([CR#705.2]): draw the coin, score
    /// `won = (call == heads)`, then re-surface the next call (`remaining`)
    /// or front-schedule the accumulated `CoinFlipped` batch.
    CallFlip {
        player: crate::player::PlayerId,
        remaining: deckmaste_core::Uint,
        events: Vec<crate::event::GameEvent>,
    },
}

/// Transaction-external diagnostics emitted by legal engine recovery paths.
/// Tournament-policy interpretation belongs to clients and judges, not the
/// rules engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineIncident {
    PaymentDeclined(PaymentDeclined),
}

/// A spell or activated-ability announcement abandoned before payment was
/// submitted. Task-local replay metadata records whether [CR#733.1] forced any
/// speculative operations to survive recovery before the player receives
/// priority again and may take a different action ([CR#733.2]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentDeclined {
    pub player: crate::player::PlayerId,
    pub subject: crate::payment::PaymentSubject,
    pub forced_retained_records: Vec<crate::payment::PaymentRecordId>,
    pub crossed_reversal_barrier: bool,
    pub crossed_observation_barrier: bool,
}

/// The resolution-scoped collector for a post-pick arrangement ([CR#401.4]):
/// each ordered-library landing an armed `Each`/`MoveGroup` batch produces
/// records here; the `ArrangePiles` finalizer groups them into piles and
/// clears it. `arranger` is the player who orders the piles (the effect's
/// controller — for fateseal, the fatesealing player over the opponent's
/// library).
#[derive(Debug, Clone)]
pub struct ArrangeScope {
    pub arranger: crate::player::PlayerId,
    pub landings: Vec<ArrangeLanding>,
}

/// One ordered-library landing recorded during an armed arrange scope: the
/// `object` came to rest at `end` of `library_owner`'s library.
#[derive(Debug, Clone)]
pub struct ArrangeLanding {
    pub library_owner: crate::player::PlayerId,
    pub end: crate::agenda::LibraryEnd,
    pub object: crate::object::ObjectId,
}

/// A pile of more than one card landing at one library end, awaiting its
/// arrange decision ([CR#401.4]). `objects` is the pile in its current library
/// order; the `Arranged` answer is a permutation of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrangePile {
    pub library_owner: crate::player::PlayerId,
    pub end: crate::agenda::LibraryEnd,
    pub objects: Vec<crate::object::ObjectId>,
}

/// Suspended replacement-loop state ([CR#616.1]) — preserved across a
/// `ChooseReplacement` decision so the loop can resume after the player picks.
///
/// When `replace_event` needs a player choice (≥ 2 applicable replacements),
/// it stores the in-progress event and lineage here, surfaces
/// `DecisionPointKind::ChooseReplacement`, and returns `Suspend`. The submit
/// handler resumes by running `apply_one` on the chosen replacement, then
/// re-entering the replacement loop on the (possibly modified) event.
#[derive(Debug, Clone)]
pub struct ReplaceState {
    /// The event whose replacement choice is pending.
    pub current: crate::event::GameEvent,
    /// [CR#614.5] lineage: replacements already applied to the current event
    /// chain — none of these may be applied again during this occurrence.
    pub applied: std::collections::HashSet<crate::replace_registry::ReplacementKey>,
    /// Remaining events from the same batch occurrence that have not yet been
    /// processed. Empty for a `Occurrence::Single`; non-empty when a batch
    /// event suspends mid-processing.
    pub remaining: Vec<crate::event::GameEvent>,
}

/// Resolution-local registers suspended while a stackless mana ability runs.
/// A mana ability has its own resolution under [CR#605.3b,605.4a] even when it
/// interrupts another resolving effect, so its note and lookback state must
/// not overwrite the containing resolution's.
#[derive(Debug, Clone)]
pub(crate) struct ResolutionScopeSnapshot {
    moved_chain: Vec<(crate::object::ObjectId, crate::object::ObjectId)>,
    resolution_events: Vec<GameEvent>,
    resolution_contained_act_commits: std::collections::HashMap<deckmaste_core::VerbName, u64>,
    resolution_contained_act_serial: u64,
    arrange_scope: Option<ArrangeScope>,
}

/// One complete runnable rules state. Payment frames clone this whole value in
/// the first implementation; the transaction controller remains outside it.
#[derive(Debug, Clone)]
pub struct GameImage {
    /// Fixed at game start, never mutated after.
    pub cards: Cards,
    pub players: Vec<PlayerState>,
    pub zones: Zones,
    pub objects: ObjectStore,
    pub stack: Vec<StackEntry>,
    /// The single in-flight announce ([CR#601.2] / [CR#602.2]); `Some` only
    /// between `BeginCast` and the `SpellCast` that promotes it onto the
    /// stack.
    pub announcing: Option<PendingStackEntry>,
    pub turn: TurnState,
    pub agenda: VecDeque<WorkItem>,
    pub pending: Option<DecisionPointKind>,
    /// The continuation waiting on an open `ChooseObjects` decision.
    pub choice: Option<DecisionContinuation>,
    pub outcome: Option<GameOutcome>,
    /// [CR#603.2]: triggers that have fired but are not yet on the stack.
    /// Populated only by applying a `TriggerFired` event; drained by the
    /// `PlaceTriggers` barrier.
    pub pending_triggers: Vec<crate::trigger::NotedTrigger>,
    /// [CR#603.3d]: a trigger whose placement is mid-flight — its stack id is
    /// minted and a `ChooseTargets` decision is open. The trigger analogue of
    /// `announcing`; `Some` only across that target choice.
    pub placing_trigger: Option<crate::trigger::PendingTrigger>,
    /// [CR#603.7]: the delayed-triggered-ability registry. A delayed ability
    /// is created during a resolution (`Instruction::Delayed`) and is printed
    /// on no permanent, so the live `abilities_of_source` scan never sees it;
    /// the trigger scan (`scan_delayed`) consults this registry alongside live
    /// permanents. Each entry fires ONCE — the next time its event occurs
    /// ([CR#603.7b]) — then the scan removes it. Reflexive abilities
    /// ([CR#603.12]) are NOT stored here (they are checked against earlier
    /// same-resolution events immediately at creation, never persisted).
    pub delayed_triggers: Vec<crate::trigger::CreatedTrigger>,
    /// [CR#607.1]: the linked memory each object carries — ADR law 8. Keyed by
    /// the object both linked abilities are printed on plus the cell name a
    /// `Remember` instruction declared, so a reading region's
    /// `Provenance::Linked` parameter is one indexed lookup at region entry.
    /// A cell is per-OBJECT, so a permanent that left and came back reads
    /// nothing: it is a new object ([CR#400.7]) and its abilities are not
    /// linked to the departed one's writes.
    pub(crate) memory: std::collections::HashMap<
        (crate::object::ObjectId, deckmaste_core::Ident),
        crate::activation::Value,
    >,
    /// [CR#400.7d]: "an ability of a permanent can reference information about
    /// the spell that became that permanent as it resolved, including what
    /// costs were paid to cast that spell". The announced optional-cost record
    /// rides the STACK ENTRY, which is gone before a kicked permanent's
    /// enters-the-battlefield trigger resolves; this is the one zone change
    /// [CR#702.33e]'s linked "if it was kicked" read has to survive, so the
    /// record crosses onto the permanent at the stack -> battlefield remint.
    pub(crate) paid_costs_by_object: std::collections::HashMap<
        crate::object::ObjectId,
        Vec<(deckmaste_core::CostTag, deckmaste_core::Uint)>,
    >,
    /// [CR#603.12]: the substantive facts applied SINCE the current stack entry
    /// began resolving — the resolution-scoped window a reflexive triggered
    /// ability ("when you do") looks back over at the instant it is created.
    /// Cleared when a stack entry begins resolving (same lifecycle as
    /// the resolution-local move chain and event lookback state,
    /// appended to by the history recorder for every non-meta fact.
    pub resolution_events: Vec<GameEvent>,
    /// Successful contained keyword actions since the current stack entry
    /// began resolving. This is an INTERNAL aggregate-finalization signal, not
    /// a game fact: contained `Act` commits stay absent from history and
    /// trigger scans, while their enclosing `Batch` can still learn that at
    /// least one contained future committed. Each verb retains only its most
    /// recent success serial, so a huge `Batch` stays bounded by the number of
    /// distinct verbs rather than its cardinality. Cleared with
    /// `resolution_events` at the fresh-resolution boundary.
    pub resolution_contained_act_commits: std::collections::HashMap<deckmaste_core::VerbName, u64>,
    /// Monotonic cursor for `resolution_contained_act_commits`; a
    /// `FinalizeMark` snapshots it so a stale success for the same verb cannot
    /// satisfy a later aggregate.
    pub resolution_contained_act_serial: u64,
    /// Combat-phase declarations ([CR#506]): attackers, blocks, and
    /// damage-assignment order. Cleared at end of combat ([CR#511.3]).
    pub combat: CombatState,
    /// [CR#510.1]: the in-flight combat-damage assignment — accumulated
    /// `DamageDealt` plus the sources still owing a free-division choice.
    /// `Some` only across the Combat Damage step's assignment decisions; the
    /// last answer deals the batch and clears this back to `None`.
    pub combat_damage: Option<CombatDamage>,
    pub rng: ChaCha8Rng,
    /// Floating one-shot continuous effects ([CR#611.2]): created by resolving
    /// spells/abilities via `Instruction::Continuously`, retained until their
    /// `duration` expires.
    pub continuous: Vec<ContinuousEffect>,
    /// Grant-time closure environments keyed by the timestamp of the floating
    /// continuous effect that owns them. Kept outside the public
    /// `ContinuousEffect` value so callers constructing ordinary effects do
    /// not need to know about engine activation values.
    pub(crate) continuous_grant_runtimes: std::collections::BTreeMap<
        crate::object::Timestamp,
        Vec<crate::activation::AbilityRuntime>,
    >,
    /// Floating one-shot/duration-bounded replacement effects ([CR#614.3]):
    /// regeneration shields and other "the next time …" replacements. Swept at
    /// end of turn; a `one_shot` instance is removed when it is the chosen
    /// replacement.
    pub shields: Vec<crate::replace_registry::ReplacementInstance>,
    /// Instruction-scoped "can't be regenerated" subjects ([CR#701.19c]): the
    /// objects whose regeneration shields must NOT be applied for the destroy
    /// occurrence currently being applied. Installed by
    /// `WorkItem::InstallRiders` (folded from a `ForThisEvent`
    /// `Cant(Regenerate)` rider in `Sequentially` lowering) immediately
    /// before the destroy runs, consulted in `gather_applicable`, and
    /// cleared at the end of the next `apply_occurrence` — the rider is in
    /// force only while its host destroy event executes. Transient (like
    /// `evolving_batch`/`moved_chain`); empty at rest.
    pub(crate) no_regen_subjects: Vec<crate::object::ObjectId>,
    /// The designation registry ([CR#109.3] non-characteristic state).
    pub designations: DesignationStore,
    /// The counter-kind registry ([CR#122.1]) — a counter on an object is
    /// stored as a bare `Ident → count`, so the bearings a counter confers
    /// (its `Continuous` boost, its `StateBased` SBA) are looked up here by
    /// name, unlike subtypes whose confers ride the card value. Populated from
    /// the loaded plugin's `counters` at game construction; empty means no
    /// counter confers anything (the pre-data behavior).
    pub counter_decls: std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::Counter>,
    /// The subtype registry ([CR#205.3]) — `Ident → Subtype`. The layer-4
    /// `Subtypes(...)` modifications carry a name-keyed `SubtypeRef`; this maps
    /// its `name` back to the full `Subtype` struct (with `confers`/`types`).
    /// Populated from
    /// the loaded plugin's `subtypes` at construction (like `counter_decls`);
    /// empty means a granted subtype carries no inherent rules. An `Ident`
    /// absent from this map applies as a minimal name-only `Subtype`.
    pub subtypes: std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::Subtype>,
    /// The card-type registry ([CR#300.1]) — `Ident → TypeDef`. The layer-4
    /// `CardTypes(...)` modifications carry bare `Ident` names; this maps
    /// them back to full `TypeDef` structs (with `permanent_type`/`confers`).
    /// Populated from the loaded plugin's `types` at construction (like
    /// `subtypes`); empty means a granted type carries no inherent rules. An
    /// `Ident` absent from this map applies as a minimal name-only `TypeDef`.
    pub types: std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::TypeDef>,
    /// Rules-defined SBAs in force this game ([CR#704]). Injected by the
    /// consumer after construction (like `counter_decls`); the SBA sweep reads
    /// it. Empty = no rules-defined SBAs (the engine still runs the imperative
    /// ones). A variant rule set swaps this vector.
    pub sba_rules: Vec<deckmaste_core::SbaRule>,
    /// Rules-defined conferrals in force this game — predicate-scoped grants
    /// applied globally, alongside a card's own printed/conferred abilities.
    /// Populated from the loaded plugin's `conferral_rules` at construction,
    /// like `sba_rules`. Empty = no rules-defined conferrals.
    pub conferral_rules: Vec<deckmaste_core::ConferralRule>,
    /// Rules-defined damage results in force this game ([CR#120.3c]
    /// planeswalker loyalty): when damage is dealt to a permanent matching
    /// a rule's `recipient`, that many `remove` counters are taken off it,
    /// IN ADDITION to any intrinsic result. Populated from the loaded
    /// plugin's `damage_result_rules` at construction, like `sba_rules`.
    /// Empty = no rules-defined damage results (e.g. planeswalkers stop
    /// losing loyalty to damage — the builtin
    /// `rules/damage/planeswalker-loyalty.ron` supplies it).
    pub damage_result_rules: Vec<deckmaste_core::DamageResultRule>,
    /// The resolution-scoped old→new move record ([CR#400.7j]): objects THIS
    /// resolution moved to a PUBLIC zone, as ordered `(pre-move, reminted)`
    /// pairs. Written by `apply_zone_will_change`, chased transitively when an
    /// action reads a region register, and cleared when a stack entry begins
    /// resolving — the same lifecycle as the resolution activation. Hidden
    /// destinations are never recorded ([CR#400.7] — the object is lost).
    pub moved_chain: Vec<(crate::object::ObjectId, crate::object::ObjectId)>,
    /// Turn/game event history ([CR#608.2i]): the append-only log the
    /// condition layer queries (`Count::EventCount`/`Count::EventSum`,
    /// `Condition::Happened`).
    pub history: crate::history::History,
    /// Suspended replacement-loop state ([CR#616.1]): set when a
    /// `ChooseReplacement` decision is surfaced, cleared when it is answered.
    /// `None` at all other times (the decision-open state machine mirrors
    /// `combat_damage` / `announcing`).
    pub replace_state: Option<ReplaceState>,
    /// Monotonically increasing counter for `InstanceId` assignment. Bumped on
    /// every `CreateReplacement` action; yields unique ids across the game.
    pub next_shield_id: u32,
    /// [CR#406.3] generalized: who is ALLOWED to see which object. Persistent,
    /// keyed by object IDENTITY (`ObjectId`), which is freshly minted on every
    /// zone change ([CR#400.7]) — so grants expire for free on shuffle. Written
    /// by looks (Distribute); the redacted per-player VIEW is a runner concern.
    pub look_grants: std::collections::HashSet<(crate::player::PlayerId, crate::object::ObjectId)>,
    /// Monotonically increasing batch-id source ([CR#603.2c]): every applied
    /// `Occurrence::Batch` records its member facts under one fresh id, so
    /// the history log keeps "these facts were one simultaneous event"
    /// ([CR#603.2c] — a batch is ONE occurrence).
    pub next_batch: Uint,
    /// Monotonically increasing payment-id source ([CR#118.10]): every
    /// cost payment a scheduler starts (`crate::cast::pay_cost`'s spell/
    /// activation arms, the `May(Pay)` toll path, an `AdditionalCost` arm)
    /// mints one fresh id here (`mint_payment`) and stamps it on every
    /// `ExecutionFrame` that payment's drain runs against
    /// ([`crate::stack::Payment`]). Deliberately separate from `next_batch`
    /// — a batch is [CR#603.2c] SIMULTANEITY grouping, a wholly different
    /// concept from a payment; conflating them would corrupt history reads.
    pub next_payment: Uint,
    /// The batch-evolution collector: `Some` only while `apply_occurrence`
    /// is applying a `Batch`'s members. Intent evolutions (`Act(Destroy)` →
    /// future-form `ZoneChange` → past-form `ZoneChange`, a draw's move, a
    /// created token's entry fact) push here instead of front-scheduling a
    /// `Single`, and the batch apply flushes the collection as ONE
    /// follow-on `Occurrence` — a simultaneous batch stays a batch through
    /// every evolution stage ([CR#603.2c]; a destroy-all's
    /// dies-facts are one occurrence).
    pub(crate) evolving_batch: Option<Vec<GameEvent>>,
    /// [CR#401.4]: the armed post-pick arrange collector. `Some` while an
    /// `Each`/`MoveGroup` whose body repositions cards into ordered library
    /// positions is resolving; each landing records here, and the
    /// `ArrangePiles` finalizer drains it. `None` at all other times, so a
    /// lone `Move(_, Library(_))` reposition (a definite position, no order
    /// choice) records nothing.
    pub(crate) arrange_scope: Option<crate::state::ArrangeScope>,
    /// Parent rules-control slots suspended beneath a nested payment action.
    pub(crate) control_stack: Vec<crate::control::ControlSnapshot>,
    /// Stackless mana actions whose effects are currently resolving. The top
    /// identity is copied into every unit of mana that effect produces.
    pub(crate) resolving_mana_actions: Vec<crate::player::ManaActionId>,
    /// Containing resolution scopes hidden beneath immediate stackless mana
    /// resolutions. Kept inside the cloned image so payment replay preserves
    /// the same nesting boundary.
    pub(crate) resolution_scope_stack: Vec<ResolutionScopeSnapshot>,
}

impl GameImage {
    pub(crate) fn begin_mana_resolution_scope(&mut self) {
        self.resolution_scope_stack.push(ResolutionScopeSnapshot {
            moved_chain: std::mem::take(&mut self.moved_chain),
            resolution_events: std::mem::take(&mut self.resolution_events),
            resolution_contained_act_commits: std::mem::take(
                &mut self.resolution_contained_act_commits,
            ),
            resolution_contained_act_serial: std::mem::take(
                &mut self.resolution_contained_act_serial,
            ),
            arrange_scope: self.arrange_scope.take(),
        });
    }

    pub(crate) fn finish_mana_resolution_scope(&mut self) {
        let snapshot = self
            .resolution_scope_stack
            .pop()
            .expect("a finishing mana action owns a resolution scope");
        self.moved_chain = snapshot.moved_chain;
        self.resolution_events = snapshot.resolution_events;
        self.resolution_contained_act_commits = snapshot.resolution_contained_act_commits;
        self.resolution_contained_act_serial = snapshot.resolution_contained_act_serial;
        self.arrange_scope = snapshot.arrange_scope;
    }
}

/// The public game façade: one committed image plus optional transactional
/// working images. Ordinary engine code continues to use field syntax through
/// `Deref`, always reaching the top active image when payment is in flight.
#[derive(Debug, Clone)]
pub struct GameState {
    pub(crate) committed: GameImage,
    pub(crate) payment: Option<crate::payment::PaymentController>,
    /// Recovery diagnostics survive transaction rollback and never affect
    /// rules execution directly.
    pub(crate) incidents: Vec<EngineIncident>,
    /// Transaction-external stable identity source for activated and
    /// triggered mana actions. Declined speculative images never reuse IDs.
    pub(crate) next_mana_action: u64,
    /// Information learned while a payment image is speculative. The ledger
    /// lives outside `GameImage`, so reconstructing or discarding an image
    /// cannot make a player forget an observation.
    pub(crate) payment_observations:
        std::collections::HashSet<(crate::player::PlayerId, crate::payment::LogicalObject)>,
    /// Concrete card/object spines minted by the active transaction, named by
    /// stable replay handles. Cleared once the outer transaction closes.
    pub(crate) payment_logical_objects:
        std::collections::HashMap<crate::object::ObjectSource, crate::payment::LogicalObject>,
    /// Activation records are resolution-local environments and deliberately
    /// live outside transactional game images.
    pub(crate) activations: std::cell::RefCell<crate::activation::ActivationTable>,
    pub(crate) next_activation: std::cell::Cell<u64>,
}

impl std::ops::Deref for GameState {
    type Target = GameImage;

    fn deref(&self) -> &Self::Target {
        self.payment
            .as_ref()
            .and_then(|controller| controller.frames.last())
            .map_or(&self.committed, |frame| &frame.working)
    }
}

impl std::ops::DerefMut for GameState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.payment
            .as_mut()
            .and_then(|controller| controller.frames.last_mut())
            .map_or(&mut self.committed, |frame| &mut frame.working)
    }
}

impl GameState {
    /// Freeze every resolution-scoped success cursor a `FinalizeAct` may read.
    pub(crate) fn finalize_mark(&self) -> crate::agenda::FinalizeMark {
        crate::agenda::FinalizeMark {
            events: self.resolution_events.len(),
            contained_act_serial: self.resolution_contained_act_serial,
        }
    }

    /// Builds the card table, shuffles seeded, draws opening hands (no
    /// mulligans in the skeleton), and seeds the agenda with turn 1.
    ///
    /// # Panics
    ///
    /// Panics if player or card counts exceed `Uint` — config sizes are
    /// trusted setup input.
    #[must_use]
    pub fn new(config: GameConfig) -> Self {
        let n = config.players.len();
        let mut rng = ChaCha8Rng::seed_from_u64(config.seed);
        let starting = match config.starting_player {
            StartingPlayer::Fixed(p) => p,
            StartingPlayer::Random => PlayerId(rng.random_range(0..Uint::try_from(n).unwrap())),
        };

        let mut cards = Cards::default();
        let mut objects = ObjectStore::default();
        let mut zones = Zones::new(n);
        let mut players = Vec::with_capacity(n);

        for (i, player_config) in config.players.into_iter().enumerate() {
            let player = PlayerId(Uint::try_from(i).expect("player count fits in Uint"));
            let proxy = objects.mint(ObjectSource::Player(player), player, None);
            players.push(PlayerState::new(player, proxy, config.starting_life));

            let mut library: Vec<ObjectId> = player_config
                .deck
                .into_iter()
                .map(|def| {
                    let card = cards.push(def, player);
                    objects.mint(ObjectSource::Card(card), player, Some(Zone::Library))
                })
                .collect();
            library.shuffle(&mut rng);
            zones.libraries[i] = library.into();

            // Opening hand ([CR#103.5]): pre-game, not events.
            for _ in 0..7 {
                let Some(top) = zones.libraries[i].pop_front() else { break };
                objects.obj_mut(top).zone = Some(Zone::Hand);
                zones.hands[i].push(top);
            }
        }

        let committed = GameImage {
            cards,
            players,
            zones,
            objects,
            stack: Vec::new(),
            announcing: None,
            turn: TurnState {
                active_player: starting,
                turn_number: 0,
                // Pre-game placeholder; the first BeginStep(Beginning(Untap))
                // begins turn 1.
                current: PhaseStep::Ending(EndingStep::Cleanup),
                priority: None,
            },
            agenda: VecDeque::from([WorkItem::BeginStep(PhaseStep::Beginning(
                BeginningStep::Untap,
            ))]),
            pending: None,
            choice: None,
            outcome: None,
            pending_triggers: Vec::new(),
            placing_trigger: None,
            delayed_triggers: Vec::new(),
            memory: std::collections::HashMap::new(),
            paid_costs_by_object: std::collections::HashMap::new(),
            resolution_events: Vec::new(),
            resolution_contained_act_commits: std::collections::HashMap::new(),
            resolution_contained_act_serial: 0,
            combat: CombatState::default(),
            combat_damage: None,
            rng,
            continuous: Vec::new(),
            continuous_grant_runtimes: std::collections::BTreeMap::new(),
            shields: Vec::new(),
            no_regen_subjects: Vec::new(),
            designations: DesignationStore::default(),
            counter_decls: config.counter_decls,
            subtypes: config.subtypes,
            types: config.types,
            sba_rules: config.sba_rules,
            conferral_rules: config.conferral_rules,
            damage_result_rules: config.damage_result_rules,
            moved_chain: Vec::new(),
            history: crate::history::History::default(),
            replace_state: None,
            next_shield_id: 0,
            look_grants: std::collections::HashSet::new(),
            next_batch: 0,
            next_payment: 0,
            evolving_batch: None,
            arrange_scope: None,
            control_stack: Vec::new(),
            resolving_mana_actions: Vec::new(),
            resolution_scope_stack: Vec::new(),
        };
        Self {
            committed,
            payment: None,
            incidents: Vec::new(),
            next_mana_action: 0,
            payment_observations: std::collections::HashSet::new(),
            payment_logical_objects: std::collections::HashMap::new(),
            activations: std::cell::RefCell::new(std::collections::HashMap::new()),
            next_activation: std::cell::Cell::new(0),
        }
    }

    pub(crate) fn mint_mana_action(&mut self) -> crate::player::ManaActionId {
        let id = crate::player::ManaActionId(self.next_mana_action);
        self.next_mana_action = self
            .next_mana_action
            .checked_add(1)
            .expect("mana action id overflow");
        id
    }

    #[must_use]
    pub fn committed(&self) -> &GameImage {
        &self.committed
    }

    #[must_use]
    pub fn active(&self) -> &GameImage {
        self
    }

    #[must_use]
    pub fn incidents(&self) -> &[EngineIncident] {
        &self.incidents
    }

    #[cfg(test)]
    pub(crate) fn begin_test_frame(&mut self) {
        let working = self.active().clone();
        let payer = working.turn.active_player;
        let mut frame = crate::payment::PaymentFrame::proposal(working, payer);
        frame.activations = self.activations.borrow().clone();
        frame.next_activation = self.next_activation.get();
        self.payment
            .get_or_insert_with(crate::payment::PaymentController::default)
            .frames
            .push(frame);
    }

    /// Mints a fresh [`crate::stack::Payment`] id ([CR#118.10]) — call
    /// ONCE per cost payment (never per verb) and stamp the result on every
    /// `ExecutionFrame` that payment's drain runs against, so the whole payment
    /// shares one id.
    pub(crate) fn mint_payment(&mut self) -> crate::stack::Payment {
        let id = self.next_payment;
        self.next_payment += 1;
        crate::stack::Payment { id }
    }

    /// # Panics
    ///
    /// Panics on an out-of-range `PlayerId` — engine invariant, not caller
    /// input.
    #[must_use]
    pub fn player(&self, p: PlayerId) -> &PlayerState {
        &self.players[p.index()]
    }

    /// # Panics
    ///
    /// Panics on an out-of-range `PlayerId` — engine invariant, not caller
    /// input.
    pub fn player_mut(&mut self, p: PlayerId) -> &mut PlayerState {
        &mut self.players[p.index()]
    }

    /// Whether a battlefield `OutcomeGate` of `kind` applies to `player`
    /// ([CR#101.1,704.3], ADR U5 — precedence, not consumption). Folds over
    /// battlefield permanents; each gate's `who` predicate is matched against
    /// `player`'s proxy with the gate's carrier as the watcher, so `Ref(You)`
    /// and `OpponentOf(Ref(You))` anchor to the carrier's controller.
    #[must_use]
    pub fn gate_suppresses(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        kind: deckmaste_core::OutcomeGateKind,
    ) -> bool {
        let proxy = self.player(player).object;
        self.zones.battlefield.iter().any(|&carrier| {
            let source = self.objects.obj(carrier).source;
            let pred = |s: &deckmaste_core::StaticSpec| {
                matches!(
                    s,
                    deckmaste_core::StaticSpec::OutcomeGate { who, gate }
                        if *gate == kind
                            && crate::target::matches_with(self, proxy, who, Some(source))
                )
            };
            crate::legal::object_has_static(view, carrier, &pred)
        })
    }

    /// [CR#102.4,810.1]: whether two players are on the same team — the single
    /// seam the team-relative player filters (`OpponentOf` = different team,
    /// `TeammateOf` = same team, excluding self) route through.
    ///
    /// Team membership is not yet modeled (there is no `team` field on
    /// `PlayerState`), so each player is their own singleton team: `p` and `q`
    /// share a team iff they are the same player. This makes `OpponentOf` mean
    /// "a different player" (correct for 1v1 and free-for-all) and `TeammateOf`
    /// match nobody (also correct outside Two-Headed Giant / team play). When
    /// real team grouping lands (`engine` team support), only this predicate
    /// changes; both relation filters follow.
    #[must_use]
    #[expect(
        clippy::unused_self,
        reason = "team-membership seam: reads `self`'s PlayerState teams once team play is modeled"
    )]
    pub(crate) fn same_team(&self, p: PlayerId, q: PlayerId) -> bool {
        p == q
    }

    /// [CR#102.1]: how many opponents player `p` has — the live players NOT on
    /// `p`'s team ([CR#102.4,810.1]). The reader behind
    /// [`Count::Opponents`](deckmaste_core::Count::Opponents); routes through
    /// [`same_team`](Self::same_team) so it follows team grouping once modeled.
    #[must_use]
    pub(crate) fn opponent_count(&self, p: PlayerId) -> Uint {
        Uint::try_from(
            self.players
                .iter()
                .filter(|q| !q.lost && !self.same_team(p, q.id))
                .count(),
        )
        .expect("opponent count fits Uint")
    }

    /// The card behind an object (card-backed objects only).
    ///
    /// # Panics
    ///
    /// Panics on a stale `ObjectId`, a fabricated `CardId`, or a player proxy
    /// — engine invariants, not caller input.
    #[must_use]
    pub fn def(&self, id: ObjectId) -> &Card {
        let card = self.objects.obj(id).card_id().expect("card-backed object");
        &self.cards.get(card).def
    }

    /// [CR#108.3]: a card's owner never changes; an object's owner is its
    /// card's.
    ///
    /// # Panics
    ///
    /// Panics on a stale `ObjectId`, a fabricated `CardId`, or a player proxy
    /// — engine invariants, not caller input.
    #[must_use]
    pub fn owner_of(&self, id: ObjectId) -> PlayerId {
        let card = self.objects.obj(id).card_id().expect("card-backed object");
        self.cards.get(card).owner
    }

    /// # Panics
    ///
    /// Panics if the player count overflows `Uint` — config sizes are trusted
    /// setup input.
    #[must_use]
    pub fn live_count(&self) -> Uint {
        Uint::try_from(self.players.iter().filter(|p| !p.lost).count())
            .expect("player count fits in Uint")
    }

    /// The next non-lost player after `p` in turn order (APNAP rotation).
    ///
    /// # Panics
    ///
    /// Panics if no player is live — the game would already be over.
    #[must_use]
    pub fn next_live_after(&self, p: PlayerId) -> PlayerId {
        let n = self.players.len();
        (1..=n)
            .map(|offset| PlayerId(Uint::try_from((p.index() + offset) % n).unwrap()))
            .find(|&q| !self.player(q).lost)
            .expect("at least one live player")
    }

    /// # Panics
    ///
    /// Panics if `object` is not in `player`'s hand — callers validate first.
    pub(crate) fn remove_from_hand(&mut self, player: PlayerId, object: ObjectId) {
        let hand = &mut self.zones.hands[player.index()];
        let i = hand
            .iter()
            .position(|&o| o == object)
            .expect("object in hand");
        hand.remove(i);
    }

    /// Removes `object` from `player`'s library ([CR#401]). Panics if absent.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not in `player`'s library — callers validate
    /// first.
    pub(crate) fn remove_from_library(&mut self, player: PlayerId, object: ObjectId) {
        let lib = &mut self.zones.libraries[player.index()];
        let i = lib
            .iter()
            .position(|&o| o == object)
            .expect("object in library");
        lib.remove(i);
    }

    /// Removes `object` from `player`'s graveyard ([CR#404]). Panics if
    /// absent.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not in `player`'s graveyard — callers validate
    /// first.
    pub(crate) fn remove_from_graveyard(&mut self, player: PlayerId, object: ObjectId) {
        let graveyard = &mut self.zones.graveyards[player.index()];
        let i = graveyard
            .iter()
            .position(|&o| o == object)
            .expect("object in graveyard");
        graveyard.remove(i);
    }

    /// Removes the committed stack entry whose `id` is `id` ([CR#405]). Keyed
    /// on `StackEntry.id` so it works for both spells (id == the spell object)
    /// and triggered abilities (id == a minted token).
    ///
    /// # Panics
    ///
    /// Panics if no entry has that id — engine invariant, not caller input.
    pub(crate) fn remove_stack_entry(&mut self, id: ObjectId) {
        let i = self
            .stack
            .iter()
            .position(|e| e.id == id)
            .expect("entry on stack");
        let activation = self.stack[i].activation;
        self.stack.remove(i);
        self.remove_activation_family(activation);
    }

    /// Removes `object` from the shared battlefield. Panics if absent.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not on the battlefield — engine invariant, not
    /// caller input.
    pub(crate) fn remove_from_battlefield(&mut self, object: ObjectId) {
        let i = self
            .zones
            .battlefield
            .iter()
            .position(|&o| o == object)
            .expect("object on battlefield");
        self.zones.battlefield.remove(i);
    }

    /// Removes `object` from the shared exile zone ([CR#406]). Panics if
    /// absent.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not in exile — engine invariant, not caller
    /// input.
    pub(crate) fn remove_from_exile(&mut self, object: ObjectId) {
        let i = self
            .zones
            .exile
            .iter()
            .position(|&o| o == object)
            .expect("object in exile");
        self.zones.exile.remove(i);
    }

    /// Schedules items at the agenda front, preserving their order.
    pub(crate) fn schedule_front(&mut self, items: Vec<WorkItem>) {
        for item in items.into_iter().rev() {
            self.agenda.push_front(item);
        }
    }

    /// [CR#514.2]: discard "until end of turn" continuous effects at Cleanup.
    ///
    /// Other durations (the remaining `FixedUntil` markers, `ForAsLongAs`,
    /// `UntilEvent`, `EndOfGame`) have no sweep/tracking yet — `resolve`
    /// trips a seam before any instance carrying one is created.
    /// The choices.md §6 boundary record for the pending decision, schema
    /// derived from the kind (see `DecisionPointKind`'s schema methods).
    #[must_use]
    pub fn decision_point(&self) -> Option<crate::decide::DecisionPoint> {
        self.pending
            .as_ref()
            .map(|pending| crate::decide::DecisionPoint {
                pending: pending.clone(),
                decider: pending.decider_spec(),
                lock: pending.lock(),
                visibility: pending.visibility(),
            })
    }

    /// [CR#514.2]: sweep every "until end of turn" floating effect and shield
    /// at Cleanup. Also re-sweeps "until end of combat" as a CATCH-ALL: a turn
    /// whose combat step is skipped never runs `expire_end_of_combat`, so
    /// without this backstop such an effect would leak past the turn it was
    /// created in ([CR#511.2] still ends it no later than the turn's end).
    pub fn expire_end_of_turn(&mut self) {
        use deckmaste_core::Duration::FixedUntil;
        use deckmaste_core::TurnMarker::EndOfCombat;
        use deckmaste_core::TurnMarker::EndOfTurn;
        self.continuous
            .retain(|e| !matches!(e.duration, FixedUntil(EndOfTurn | EndOfCombat)));
        self.prune_continuous_grant_runtimes();
        self.shields
            .retain(|s| !matches!(s.duration, FixedUntil(EndOfTurn | EndOfCombat)));
    }

    /// [CR#511.2]: "until end of combat" floating effects and shields expire as
    /// the combat phase ends. Called from `end_of_combat`, beside
    /// `combat.clear()`. (Cleanup re-sweeps these — see `expire_end_of_turn`.)
    pub(crate) fn expire_end_of_combat(&mut self) {
        use deckmaste_core::Duration::FixedUntil;
        use deckmaste_core::TurnMarker::EndOfCombat;
        self.continuous
            .retain(|e| !matches!(e.duration, FixedUntil(EndOfCombat)));
        self.prune_continuous_grant_runtimes();
        self.shields
            .retain(|s| !matches!(s.duration, FixedUntil(EndOfCombat)));
    }

    /// [CR#611.2a]: "until your next turn" floating effects and shields expire
    /// as their controller's next turn begins. Called from `begin_turn` after
    /// `active_player` advances; `new_active` is that fresh active player.
    /// "Your next turn" = the next turn the effect's CONTROLLER actually takes,
    /// so skipped turns are handled naturally (the sweep only fires when the
    /// new active player IS the controller). A continuous instance keys on its
    /// stored `controller`; a shield has no controller of its own, so it keys
    /// on its live SOURCE's controller (a source that has left can't be keyed
    /// — such a shield is retained, never a panic).
    pub(crate) fn expire_your_next_turn(&mut self, new_active: PlayerId) {
        use deckmaste_core::Duration::FixedUntil;
        use deckmaste_core::TurnMarker::YourNextTurn;
        self.continuous.retain(|e| {
            !(matches!(e.duration, FixedUntil(YourNextTurn)) && e.controller == new_active)
        });
        self.prune_continuous_grant_runtimes();
        // A shield reads its controller from the live source — take the list
        // out so the `self.objects` lookup doesn't overlap the `retain` borrow.
        let shields = std::mem::take(&mut self.shields);
        self.shields = shields
            .into_iter()
            .filter(|s| {
                !(matches!(s.duration, FixedUntil(YourNextTurn))
                    && self.objects.get(s.source).map(|o| o.controller) == Some(new_active))
            })
            .collect();
    }

    /// [CR#610.3]: after an occurrence is applied, END any `UntilEvent` floating
    /// effect or shield whose event has now HAPPENED — a POST-apply check, so
    /// the effect lasts through the very event that ends it and stops right
    /// after. The filter is matched against each applied fact with bindings
    /// anchored on the minting frame (`origin` for a continuous instance, the
    /// live source for a shield), mirroring `event_matches_delayed` — a
    /// floating one-shot's end is a fire-once fact match ([CR#603.7c]); the
    /// `Delayed` lane suits it (no live scan sees a printed ability here).
    pub(crate) fn sweep_event_durations(&mut self, facts: &crate::event::Occurrence) {
        use deckmaste_core::Duration::UntilEvent;
        let events: &[GameEvent] = match facts {
            crate::event::Occurrence::Single(e) => std::slice::from_ref(e),
            crate::event::Occurrence::Batch(es) => es,
        };
        let mut drop_ce = vec![false; self.continuous.len()];
        for (i, ce) in self.continuous.iter().enumerate() {
            if let (UntilEvent(filter), Some(frame)) = (&ce.duration, ce.origin.as_deref()) {
                let watcher = self.frame_watcher(frame);
                if events
                    .iter()
                    .any(|ev| self.event_matches_delayed(filter, ev, watcher))
                {
                    drop_ce[i] = true;
                }
            }
        }
        for i in (0..drop_ce.len()).rev() {
            if drop_ce[i] {
                self.continuous.remove(i);
            }
        }
        self.prune_continuous_grant_runtimes();
        let mut drop_sh = vec![false; self.shields.len()];
        for (i, s) in self.shields.iter().enumerate() {
            if let UntilEvent(filter) = &s.duration
                && let Some(obj) = self.objects.get(s.source)
                && events
                    .iter()
                    .any(|ev| self.event_matches_delayed(filter, ev, obj.source))
            {
                drop_sh[i] = true;
            }
        }
        for i in (0..drop_sh.len()).rev() {
            if drop_sh[i] {
                self.shields.remove(i);
            }
        }
    }

    /// [CR#611.2b]: re-check every `ForAsLongAs` floating effect / shield and
    /// REMOVE any whose condition no longer holds — removal IS the
    /// once-stopped-never-resumes latch (a removed instance is simply gone, so
    /// no stored `started`/`stopped` bool is needed). Run both after each
    /// occurrence and at every step transition. The condition is evaluated with
    /// `condition_holds` against a frame rebuilt from the minting `origin`
    /// (a continuous instance carries it; a shield rebuilds a bare frame from
    /// its live source).
    ///
    /// HAZARD: never call this inside `layer::gather`. `condition_holds` can
    /// recurse into `layers()` (an `Exists`/`Matches` condition derives the
    /// board); gather is mid-derivation, so evaluating a condition there would
    /// re-enter the layer pass.
    pub(crate) fn sweep_condition_durations(&mut self) {
        use deckmaste_core::Duration::ForAsLongAs;
        let mut drop_ce = vec![false; self.continuous.len()];
        for (i, ce) in self.continuous.iter().enumerate() {
            if let (ForAsLongAs(cond), Some(frame)) = (&ce.duration, ce.origin.as_deref())
                && !self.condition_holds(cond, frame)
            {
                drop_ce[i] = true;
            }
        }
        for i in (0..drop_ce.len()).rev() {
            if drop_ce[i] {
                self.continuous.remove(i);
            }
        }
        self.prune_continuous_grant_runtimes();
        let mut drop_sh = vec![false; self.shields.len()];
        for (i, s) in self.shields.iter().enumerate() {
            if let ForAsLongAs(cond) = &s.duration
                && let Some(obj) = self.objects.get(s.source)
                && !self.condition_holds(cond, &self.frame(s.source, obj.controller))
            {
                drop_sh[i] = true;
            }
        }
        for i in (0..drop_sh.len()).rev() {
            if drop_sh[i] {
                self.shields.remove(i);
            }
        }
    }

    fn prune_continuous_grant_runtimes(&mut self) {
        let live: std::collections::BTreeSet<_> = self
            .continuous
            .iter()
            .map(|effect| effect.timestamp)
            .collect();
        self.continuous_grant_runtimes
            .retain(|timestamp, _| live.contains(timestamp));
    }
}

/// Whether a `Duration` names a lifetime the engine can actually SWEEP — the
/// canonical guard shared by both floating-instance mint sites (continuous
/// effects, [CR#611.2], and replacement shields, [CR#614.3]). Every duration
/// is sweepable EXCEPT `ForThisEvent`, which is an instruction-scoped rider
/// ([CR#611.2a]) folded onto its host occurrence in `Sequentially` lowering
/// and never a standalone instance — minting one would leak forever, silently.
pub(crate) fn duration_sweepable(d: &deckmaste_core::Duration) -> bool {
    !matches!(d, deckmaste_core::Duration::ForThisEvent)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_core::Zone;

    use super::*;
    use crate::decide::Action;
    use crate::decide::DecisionPointKind;
    use crate::decide::pending::Priority;
    use crate::event::LossReason;
    use crate::event::PlayerLost;
    use crate::stack::PendingStackEntry;
    use crate::stack::StackObject;
    use crate::trigger::PendingTrigger;
    use crate::trigger::TriggerBindings;

    fn fixture() -> GameState {
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

    #[test]
    fn active_image_hides_uncommitted_changes() {
        let mut state = fixture();
        let committed_life = state.committed().players[0].life;

        state.begin_test_frame();
        state.player_mut(PlayerId(0)).life -= 1;

        assert_eq!(state.player(PlayerId(0)).life, committed_life - 1);
        assert_eq!(state.committed().players[0].life, committed_life);
    }

    #[test]
    fn suspend_and_resume_restores_all_control_slots() {
        let mut state = fixture();
        let player = PlayerId(0);
        let id = state.player(player).object;
        state.announcing = Some(PendingStackEntry {
            activation: crate::ActivationId::NONE,
            id,
            object: StackObject::Spell(id),
            controller: player,
            origin: Zone::Hand,
            targets: vec![],
            chosen_modes: Arc::from([]),
            x: None,
            concretized: None,
            paid_costs: vec![],
            optional_components: vec![],
            alternative_cost: None,
        });
        state.pending = Some(DecisionPointKind::Priority(Priority {
            player,
            legal: vec![Action::Pass],
        }));
        state.choice = Some(DecisionContinuation::AnnounceModes);
        state.placing_trigger = Some(PendingTrigger {
            activation: crate::ActivationId::NONE,
            id,
            source: ObjectSource::Player(player),
            ability: 0,
            created: None,
            controller: player,
            bindings: TriggerBindings::default(),
        });
        state.replace_state = Some(ReplaceState {
            current: GameEvent::PlayerLost(PlayerLost {
                player,
                reason: LossReason::Conceded,
            }),
            applied: std::collections::HashSet::new(),
            remaining: vec![],
        });
        let mut image = state.active().clone();

        image.suspend_control();
        assert!(image.announcing.is_none());
        assert!(image.pending.is_none());
        assert!(image.choice.is_none());
        assert!(image.placing_trigger.is_none());
        assert!(image.replace_state.is_none());

        image.resume_control().unwrap();
        assert!(image.announcing.is_some());
        assert!(matches!(
            image.pending,
            Some(DecisionPointKind::Priority(_))
        ));
        assert_eq!(image.choice, Some(DecisionContinuation::AnnounceModes));
        assert!(image.placing_trigger.is_some());
        assert!(matches!(
            image.replace_state.as_ref().map(|state| &state.current),
            Some(GameEvent::PlayerLost(PlayerLost {
                player: PlayerId(0),
                reason: LossReason::Conceded,
            }))
        ));
    }
}
