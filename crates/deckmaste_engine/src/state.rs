use std::collections::VecDeque;
use std::sync::Arc;

use deckmaste_core::BeginningStep;
use deckmaste_core::Card;
use deckmaste_core::EndingStep;
use deckmaste_core::Int;
use deckmaste_core::Phase;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use rand::RngExt;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::agenda::WorkItem;
use crate::combat::CombatState;
use crate::decide::PendingDecision;
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
/// (core `DesignationDecl.payload: Vec<StaticEffect>`) is a TEMPLATE; the
/// instance supplies its bindings: the grantor (goad's "attacks a player
/// other than [the goader]", [CR#701.15b..701.15c]) and the duration
/// ("until your next turn"). Multiple goaders = multiple instances, each
/// expiring on its own clock — never a merged set. Payload application is
/// the layers pipeline's designation source (P0.W5 seam); duration sweep
/// rides the effect-instance machinery (seam).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignationInstance {
    pub grantor: Option<PlayerId>,
    pub duration: Option<deckmaste_core::Duration>,
}

/// The generic designation registry — storage mirrors the data-driven
/// declaration model rather than per-mechanic fields; granting effects are
/// P0.W5 seams, but `Designated(name)` filter reads are LIVE against it
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
}

/// What to resume once a resolution-time decision is answered. Transient: set
/// while that decision is pending (alongside `pending`), taken on submit. The
/// resolution-time analogue of the `announcing` cast slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChoiceContinuation {
    /// A `ChooseObjects` answer ([CR#608.2d]): bind the picks into
    /// `frame.chosen`, then re-run `effect` (the action whose `Choose`/`Random`
    /// selection produced the decision).
    BindChoice {
        effect: Box<deckmaste_core::Effect>,
        frame: crate::stack::Frame,
    },
    /// A `YesNo` answer for `Effect::May` ([CR#118.12]): true → `effect` then
    /// `if_did`; false → `if_not` (or nothing).
    May {
        may: deckmaste_core::May,
        frame: crate::stack::Frame,
    },
    /// A `ChooseModes` answer for `Effect::Modal` ([CR#700.2]): run the chosen
    /// modes' effects in the order they were picked.
    Modal {
        modes: Vec<deckmaste_core::Mode>,
        frame: crate::stack::Frame,
    },
    /// A `YesNo` answer for `Effect::Unless` ([CR#118.12a,608.2d]): yes → pay
    /// the `unless` cost (skipping `effect`); no → run `effect`. `who` is
    /// the paying player, so each cost component runs as that player's
    /// action.
    Unless {
        effect: Box<deckmaste_core::Effect>,
        who: deckmaste_core::Reference,
        unless: Vec<deckmaste_core::CostComponent>,
        frame: crate::stack::Frame,
    },
    /// A `Distribute` answer ([CR#701.22a]): stashes the effect `name`
    /// (e.g. "Scry") for the event emitted in Task 8. Consumed in
    /// `submit_distribution`; Task 8 reads `name` before taking it.
    Distribute { name: deckmaste_core::Ident },
}

/// Suspended replacement-loop state ([CR#616.1]) — preserved across a
/// `ChooseReplacement` decision so the loop can resume after the player picks.
///
/// When `replace_event` needs a player choice (≥ 2 applicable replacements),
/// it stores the in-progress event and lineage here, surfaces
/// `PendingDecision::ChooseReplacement`, and returns `Suspend`. The submit
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

/// The whole game. Fields are public for test construction and inspection;
/// [`GameState::step`] and [`GameState::submit_decision`] are the only
/// sanctioned mutators.
#[derive(Debug, Clone)]
pub struct GameState {
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
    pub pending: Option<PendingDecision>,
    /// The continuation waiting on an open `ChooseObjects` decision.
    pub choice: Option<ChoiceContinuation>,
    pub outcome: Option<GameOutcome>,
    /// [CR#603.2]: triggers that have fired but are not yet on the stack.
    /// Populated only by applying a `TriggerFired` event; drained by the
    /// `PlaceTriggers` barrier.
    pub pending_triggers: Vec<crate::trigger::NotedTrigger>,
    /// [CR#603.3d]: a trigger whose placement is mid-flight — its stack id is
    /// minted and a `ChooseTargets` decision is open. The trigger analogue of
    /// `announcing`; `Some` only across that target choice.
    pub placing_trigger: Option<crate::trigger::PendingTrigger>,
    /// Combat-phase designations ([CR#506]): attackers, blocks, and
    /// damage-assignment order. Cleared at end of combat ([CR#511.3]).
    pub combat: CombatState,
    /// [CR#510.1]: the in-flight combat-damage assignment — accumulated
    /// `DamageDealt` plus the sources still owing a free-division choice.
    /// `Some` only across the Combat Damage step's assignment decisions; the
    /// last answer deals the batch and clears this back to `None`.
    pub combat_damage: Option<CombatDamage>,
    pub rng: ChaCha8Rng,
    /// Floating one-shot continuous effects ([CR#611.2]): created by resolving
    /// spells/abilities via `Effect::Continuously`, retained until their
    /// `duration` expires.
    pub continuous: Vec<ContinuousEffect>,
    /// Floating one-shot/duration-bounded replacement effects ([CR#614.3]):
    /// regeneration shields and other "the next time …" replacements. Swept at
    /// end of turn; a `one_shot` instance is removed when it is the chosen
    /// replacement.
    pub shields: Vec<crate::replace_registry::ReplacementInstance>,
    /// The designation registry ([CR#109.3] non-characteristic state).
    pub designations: DesignationStore,
    /// The counter-kind registry ([CR#122.1]) — a counter on an object is
    /// stored as a bare `Ident → count`, so the bearings a counter confers
    /// (its `Continuous` boost, its `StateBased` SBA) are looked up here by
    /// name, unlike subtypes whose confers ride the card value. Populated from
    /// the loaded plugin's `counters` at game construction; empty means no
    /// counter confers anything (the pre-data behavior).
    pub counter_decls: std::collections::HashMap<deckmaste_core::Ident, deckmaste_core::Counter>,
    /// Rules-defined SBAs in force this game ([CR#704]). Injected by the
    /// consumer after construction (like `counter_decls`); the SBA sweep reads
    /// it. Empty = no rules-defined SBAs (the engine still runs the imperative
    /// ones). A variant rule set swaps this vector.
    pub sba_rules: Vec<deckmaste_core::SbaRule>,
    /// The "that much"/"that many" anaphora register (`Count::ThatMuch` —
    /// oracle-text magnitude anaphora; no single CR rule defines it): the
    /// amount the most recently APPLIED amount-carrying event fixed (damage
    /// dealt, life gained/lost — set by the `apply` funnel, so it reads what
    /// actually happened, post-replacement). Cleared when a stack entry
    /// begins resolving, so a read can only see an amount fixed by an
    /// earlier instruction of the same resolution.
    pub that_much: Option<Uint>,
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
}

impl GameState {
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

        Self {
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
                current: Phase::Ending(EndingStep::Cleanup),
                priority: None,
            },
            agenda: VecDeque::from([WorkItem::BeginStep(Phase::Beginning(BeginningStep::Untap))]),
            pending: None,
            choice: None,
            outcome: None,
            pending_triggers: Vec::new(),
            placing_trigger: None,
            combat: CombatState::default(),
            combat_damage: None,
            rng,
            continuous: Vec::new(),
            shields: Vec::new(),
            designations: DesignationStore::default(),
            counter_decls: std::collections::HashMap::new(),
            sba_rules: Vec::new(),
            that_much: None,
            history: crate::history::History::default(),
            replace_state: None,
            next_shield_id: 0,
            look_grants: std::collections::HashSet::new(),
        }
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
        self.stack.remove(i);
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
    /// trips a `P0.W1` seam before any instance carrying one is created.
    /// The choices.md §6 boundary record for the pending decision, schema
    /// derived from the kind (see `PendingDecision`'s schema methods).
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

    pub fn expire_end_of_turn(&mut self) {
        self.continuous.retain(|e| {
            !matches!(
                e.duration,
                deckmaste_core::Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
            )
        });
        self.shields.retain(|s| {
            !matches!(
                s.duration,
                deckmaste_core::Duration::FixedUntil(deckmaste_core::TurnMarker::EndOfTurn)
            )
        });
    }
}
