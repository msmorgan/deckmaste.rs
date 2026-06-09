use std::collections::VecDeque;
use std::sync::Arc;

use deckmaste_core::{BeginningStep, Card, EndingStep, Int, Phase, Uint, Zone};
use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

use crate::agenda::WorkItem;
use crate::combat::CombatState;
use crate::decide::PendingDecision;
use crate::layer::ContinuousEffect;
use crate::object::{Cards, ObjectId, ObjectSource, ObjectStore};
use crate::player::{PlayerId, PlayerState};
use crate::stack::{PendingStackEntry, StackEntry};
use crate::turn::TurnState;
use crate::zone::Zones;

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
            StartingPlayer::Random => PlayerId(rand::Rng::random_range(
                &mut rng,
                0..Uint::try_from(n).unwrap(),
            )),
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
            outcome: None,
            pending_triggers: Vec::new(),
            placing_trigger: None,
            combat: CombatState::default(),
            combat_damage: None,
            rng,
            continuous: Vec::new(),
        }
    }

    /// # Panics
    ///
    /// Panics on an out-of-range `PlayerId` — engine invariant, not caller
    /// input.
    #[must_use]
    pub fn player(&self, p: PlayerId) -> &PlayerState { &self.players[p.index()] }

    /// # Panics
    ///
    /// Panics on an out-of-range `PlayerId` — engine invariant, not caller
    /// input.
    pub fn player_mut(&mut self, p: PlayerId) -> &mut PlayerState { &mut self.players[p.index()] }

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

    /// Schedules items at the agenda front, preserving their order.
    pub(crate) fn schedule_front(&mut self, items: Vec<WorkItem>) {
        for item in items.into_iter().rev() {
            self.agenda.push_front(item);
        }
    }

    /// [CR#514.2]: discard "until end of turn" continuous effects at Cleanup.
    ///
    /// Other durations (`UntilEndOfCombat`, `UntilYourNextTurn`, `While`,
    /// `UntilEvent`, `EndOfGame`) are not yet swept — no fixture creates them
    /// and their sweep sites are deferred.
    pub fn expire_end_of_turn(&mut self) {
        self.continuous
            .retain(|e| !matches!(e.duration, deckmaste_core::Duration::UntilEndOfTurn));
    }
}
