//! Self-play simulation harness: a deterministic greedy auto-player that drives
//! a two-player game to completion and summarizes it. Used by the `full_game`
//! integration test and benchmark — **not** part of the stable engine API
//! (hence `#[doc(hidden)]`).
//!
//! Card data is passed in as `Arc<Card>` so callers load their plugins once;
//! the hot path then measures engine work, not disk I/O. `play` takes a
//! `Strategy` per seat; the two built-in greedy strategies are role-based, not
//! card-name-based: `GreedyCreatures` (P0: develop land → cast creatures →
//! attack with everything) and `GreedyRemoval` (P1: develop land → cast
//! instants at the biggest threat, else the face).

use std::sync::Arc;

use deckmaste_core::Card;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Int;
use deckmaste_core::Phase;
use deckmaste_core::Type;
use deckmaste_core::Uint;

use crate::Action;
use crate::Decision;
use crate::GameConfig;
use crate::GameEvent;
use crate::GameOutcome;
use crate::GameState;
use crate::ManaPool;
use crate::ObjectId;
use crate::Occurrence;
use crate::PendingDecision;
use crate::PlayerConfig;
use crate::PlayerId;
use crate::Progress;
use crate::StartingPlayer;
use crate::StepOutcome;

const SPELLS_PER_DECK: usize = 23;
const LANDS_PER_DECK: usize = 17;

/// The four card faces a matchup needs: each player's spell and basic land.
/// P0's spell should be a creature, P1's an instant (the policy keys off type).
#[derive(Clone)]
pub struct DeckCards {
    pub p0_spell: Arc<Card>,
    pub p0_land: Arc<Card>,
    pub p1_spell: Arc<Card>,
    pub p1_land: Arc<Card>,
}

/// A summary of one played-out game.
#[derive(Debug, Clone, PartialEq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "a flat record of independent per-game outcome flags for stats"
)]
pub struct Summary {
    pub outcome: GameOutcome,
    pub turns: Uint,
    pub life: [Int; 2],
    /// An instant dealt 3 to a creature (only P0 has creatures → a kill).
    pub spell_killed_creature: bool,
    /// A creature dealt combat damage to a player proxy.
    pub creature_hit_player: bool,
    /// The loser is flagged lost and is at ≤ 0 life or drew from empty.
    pub loser_lost_for_real: bool,
    /// The loser lost by drawing from an empty library (vs. life ≤ 0).
    pub decked: bool,
}

// --- card classification (player proxies have no card_id; guard before def)
// ---

fn has_type(state: &GameState, id: ObjectId, ty: Type) -> bool {
    state.objects.obj(id).card_id().is_some()
        && match state.def(id) {
            Card::Normal(f) | Card::ModalDfc(f, _) => f.types.contains(&ty),
        }
}

fn is_creature(state: &GameState, id: ObjectId) -> bool {
    has_type(state, id, Type::Creature)
}
fn is_land(state: &GameState, id: ObjectId) -> bool {
    has_type(state, id, Type::Land)
}

/// The mana value of a card-backed object's printed cost ([CR#202.3]).
fn mana_value(state: &GameState, id: ObjectId) -> Uint {
    let cost = match state.def(id) {
        Card::Normal(f) | Card::ModalDfc(f, _) => &f.mana_cost,
    };
    cost.mana_value()
}

/// The cheapest mana value among hand cards of `ty` the player could aim to
/// cast.
fn cheapest_in_hand(state: &GameState, player: PlayerId, ty: Type) -> Option<Uint> {
    state.zones.hands[player.index()]
        .iter()
        .copied()
        .filter(|&o| has_type(state, o, ty))
        .map(|o| mana_value(state, o))
        .min()
}

fn opponent(p: PlayerId) -> PlayerId {
    PlayerId(1 - p.0)
}

// --- mana payment
// -------------------------------------------------------------

fn all_kinds() -> [ColorOrColorless; 6] {
    [
        ColorOrColorless::Colorless,
        ColorOrColorless::Color(Color::White),
        ColorOrColorless::Color(Color::Blue),
        ColorOrColorless::Color(Color::Black),
        ColorOrColorless::Color(Color::Red),
        ColorOrColorless::Color(Color::Green),
    ]
}

fn pool_total(pool: &ManaPool) -> usize {
    all_kinds().iter().map(|&k| pool.amount(k) as usize).sum()
}

// --- strategies: per-seat decision-makers
// -------------------------------------

/// A per-seat decision-maker: given the full game state and the decision the
/// engine has surfaced, it returns the answer. One instance drives one seat for
/// the whole game, so a strategy is free to make cross-cutting calls (e.g. stop
/// trading removal for creatures once it is far enough ahead on life). The two
/// greedy seats below share `greedy_priority`; a different strategy may answer
/// every decision its own way.
pub trait Strategy {
    /// Answer the surfaced `pending` decision in the context of `state`.
    fn decide(&self, state: &GameState, pending: &PendingDecision) -> Decision;
}

/// P0's seat: develop a land, then float and cast a creature whenever it can,
/// and attack with everything (the all-out swing lives in `mechanical`'s
/// `DeclareAttackers`). It casts no targeted spells, so it never chooses
/// targets.
pub struct GreedyCreatures;

impl Strategy for GreedyCreatures {
    fn decide(&self, state: &GameState, pending: &PendingDecision) -> Decision {
        match pending {
            PendingDecision::Priority { player, legal } => Decision::Act(greedy_priority(
                state,
                *player,
                legal,
                Type::Creature,
                false,
            )),
            other => mechanical(state, other),
        }
    }
}

/// P1's seat: act only on its own main phases (main-phase-speed removal) —
/// develop a land, then float and cast its instant — aiming it at the board's
/// biggest threat, or the opponent's face when there's nothing worth trading
/// for.
pub struct GreedyRemoval;

impl Strategy for GreedyRemoval {
    fn decide(&self, state: &GameState, pending: &PendingDecision) -> Decision {
        match pending {
            PendingDecision::Priority { player, legal } => {
                Decision::Act(greedy_priority(state, *player, legal, Type::Instant, true))
            }
            PendingDecision::ChooseTargets { player, legal, .. } => {
                Decision::Targets(choose_targets(state, *player, legal))
            }
            other => mechanical(state, other),
        }
    }
}

/// A self-play seat for the rich demo decks (Goblins vs Elves). Like
/// `GreedyCreatures` it develops land and casts creatures with all-out attacks
/// (and, like both matchup seats, never blocks), but — unlike the
/// Bears-vs-Bolts seats — it also makes a *legal* choice for the decisions
/// those decks can surface that the matchup seats treat as impossible: it
/// chooses targets for the burn / sac-outlet pings ("any target"), and divides
/// a multi-blocked attacker's combat damage. It plays only legally, not well.
///
/// The targeting arm runs in the deterministic demo (seed `0xD00D`); the
/// damage-division arm is a defensive capability — no multi-block arises in
/// that line because neither seat blocks — covered directly by
/// `greedy_demo_divides_a_multi_blocked_attacker` in the combat suite.
///
/// Kept separate so `GreedyCreatures`/`GreedyRemoval` and `mechanical`'s
/// intentional `unreachable!`s still assert the narrow Bears-vs-Bolts matchup
/// (no targeting from the creature seat, no multi-block) — see
/// `auto_play_produces_only_legal_decisions` / `demo_auto_plays_to_completion`.
pub struct GreedyDemo;

impl Strategy for GreedyDemo {
    fn decide(&self, state: &GameState, pending: &PendingDecision) -> Decision {
        match pending {
            PendingDecision::Priority { player, legal } => Decision::Act(greedy_priority(
                state,
                *player,
                legal,
                Type::Creature,
                false,
            )),
            // The demo's burn / sac-outlet pings ("any target") and any other
            // targeted effect: a legal candidate per spec slot.
            PendingDecision::ChooseTargets { player, legal, .. } => {
                Decision::Targets(choose_targets_any(state, *player, legal))
            }
            // A multi-blocked attacker ([CR#510.1c]): any split summing to the
            // source's power is legal; dump it all on the first recipient.
            PendingDecision::AssignCombatDamage {
                source, recipients, ..
            } => Decision::Assignment(assign_all_to_first(state, *source, recipients)),
            other => mechanical(state, other),
        }
    }
}

/// Choose one legal object per `TargetSpec` slot. For a single-slot spec it
/// reuses the removal heuristic (trim the board, else the face); for multi-slot
/// specs it takes the first legal candidate of each slot — every choice is
/// drawn from that slot's offered set, so it always validates. The demo's
/// targeted cards are single "any target" pings, so the heuristic path is what
/// runs in practice.
fn choose_targets_any(
    state: &GameState,
    player: PlayerId,
    legal: &[Vec<ObjectId>],
) -> Vec<ObjectId> {
    if legal.len() == 1 {
        return choose_targets(state, player, legal);
    }
    legal
        .iter()
        .map(|set| {
            *set.first()
                .expect("each spec offers at least one legal target")
        })
        .collect()
}

/// Divide a multi-blocked attacker's combat damage ([CR#510.1c]): assign the
/// source's whole power to its first recipient. This sums to power and names a
/// single distinct recipient, so it is always legal — even for a trample
/// source, since assigning nothing to the defending player keeps the
/// "lethal-to-blockers-first" clause ([CR#702.19b]) vacuous.
fn assign_all_to_first(
    state: &GameState,
    source: ObjectId,
    recipients: &[ObjectId],
) -> Vec<(ObjectId, Uint)> {
    // The validator checks the sum against the queued assignment's recorded
    // power, so read that exact value rather than the live layered power.
    let power = state
        .combat_damage
        .as_ref()
        .and_then(|cd| cd.queue.iter().find(|a| a.source == source))
        .map_or(0, |a| a.power);
    let first = *recipients
        .first()
        .expect("a multi-blocked source has recipients");
    vec![(first, power)]
}

/// The greedy priority core both seats run, parameterized by the card type it
/// is trying to resolve and whether it holds priority until its own main phase.
/// A pure function of the visible state.
fn greedy_priority(
    state: &GameState,
    player: PlayerId,
    legal: &[Action],
    want: Type,
    hold_for_own_main: bool,
) -> Action {
    // Main-phase-speed seats (removal) hold their fire until their own main.
    if hold_for_own_main {
        let my_main = state.turn.active_player == player
            && matches!(
                state.turn.current,
                Phase::PrecombatMain | Phase::PostcombatMain
            );
        if !my_main {
            return Action::Pass;
        }
    }

    // 1. Develop: a land if offered (legality enforces one per turn).
    if let Some(a) = legal.iter().find(|a| matches!(a, Action::PlayLand { .. })) {
        return a.clone();
    }
    // 2. Deploy: cast our spell when the pool already funds it.
    if let Some(a) = legal
        .iter()
        .find(|a| matches!(a, Action::CastSpell { object } if has_type(state, *object, want)))
    {
        return a.clone();
    }
    // 3. Ramp: float one mana toward the cheapest castable card, but only when the
    //    untapped lands can actually reach its cost.
    if let Some(mv) = cheapest_in_hand(state, player, want) {
        let have = pool_total(&state.players[player.index()].mana_pool);
        let untapped = legal
            .iter()
            .filter(|a| matches!(a, Action::ActivateAbility { .. }))
            .count();
        if have < mv as usize
            && have + untapped >= mv as usize
            && let Some(a) = legal
                .iter()
                .find(|a| matches!(a, Action::ActivateAbility { .. }))
        {
            return a.clone();
        }
    }
    Action::Pass
}

/// Removal targeting: trim the board when the opponent has 2+ creatures (kill
/// one — exercising the lethal SBA), otherwise burn the opponent's face (which
/// also lets a lone attacker connect). A single `AnyTarget` spec.
fn choose_targets(state: &GameState, player: PlayerId, legal: &[Vec<ObjectId>]) -> Vec<ObjectId> {
    let opp = opponent(player);
    let candidates = &legal[0];
    let board_creatures = state
        .zones
        .battlefield
        .iter()
        .filter(|&&o| state.objects.obj(o).controller == opp && is_creature(state, o))
        .count();
    if board_creatures >= 2
        && let Some(&creature) = candidates
            .iter()
            .find(|&&id| state.objects.obj(id).controller == opp && is_creature(state, id))
    {
        return vec![creature];
    }
    vec![state.players[opp.index()].object]
}

/// Choose `count` cards to discard — shed lands first, keeping action cards.
/// Serves both the cleanup hand-size discard and a resolving discard.
fn choose_discards(state: &GameState, player: PlayerId, count: Uint) -> Vec<ObjectId> {
    let hand = &state.zones.hands[player.index()];
    let mut picks: Vec<ObjectId> = hand
        .iter()
        .copied()
        .filter(|&o| is_land(state, o))
        .collect();
    picks.extend(hand.iter().copied().filter(|&o| !is_land(state, o)));
    picks.truncate(count as usize);
    picks
}

/// The forced / uniform decisions, identical for both seats in this matchup. A
/// strategy delegates here for everything but its own priority actions (and,
/// for removal, targeting) — so `Priority` and `ChooseTargets` are unreachable
/// here, which also asserts that the creature seat never chooses targets.
fn mechanical(state: &GameState, pending: &PendingDecision) -> Decision {
    match pending {
        PendingDecision::DiscardToHandSize { player, count }
        | PendingDecision::DiscardCards { player, count } => {
            Decision::Discard(choose_discards(state, *player, *count))
        }
        // Greedy default: the first offered option (printed order).
        PendingDecision::ChooseManaColor { options, .. } => {
            Decision::ManaColor(*options.first().expect("a mana choice offers options"))
        }
        // Route through `auto_pay_pending` so the autotapper honors the
        // subject's `SpendOnly` restrictions ([CR#106.6]).
        PendingDecision::PayMana { .. } => Decision::Pay(state.auto_pay_pending()),
        PendingDecision::OrderTriggers { triggers, .. } => {
            Decision::Order((0..triggers.len()).collect())
        }
        // Attack with everything legal (the creature seat swings; the removal
        // seat has no creatures, so its set is always empty).
        PendingDecision::DeclareAttackers { legal, .. } => Decision::Attackers(legal.clone()),
        // The defender never has a creature to block with.
        PendingDecision::DeclareBlockers { .. } => Decision::Blocks(vec![]),
        // Unblocked attackers are forced (one recipient); no multi-block arises.
        PendingDecision::AssignCombatDamage {
            source, recipients, ..
        } => unreachable!(
            "no multi-block in this matchup (source {source:?}, recipients {recipients:?})"
        ),
        PendingDecision::Priority { .. } | PendingDecision::ChooseTargets { .. } => {
            unreachable!("priority and targeting are a strategy's own concern")
        }
        PendingDecision::ChooseObjects {
            candidates, min, ..
        } => Decision::Chosen(
            candidates
                .iter()
                .copied()
                .take(usize::try_from(*min).expect("min fits usize"))
                .collect(),
        ),
        // [CR#601.2b]: the headless strategy announces the minimum X=0 (always
        // legal and payable). A smarter X is a follow-up.
        PendingDecision::ChooseXValue { .. } => Decision::XValue(0),
        other => todo!("P0.W3: strategy for shell decision kind {other:?}"),
    }
}

/// The seat a surfaced decision is waiting on — every variant names its player.
fn pending_player(pending: &PendingDecision) -> PlayerId {
    match pending {
        PendingDecision::Priority { player, .. }
        | PendingDecision::DiscardToHandSize { player, .. }
        | PendingDecision::DiscardCards { player, .. }
        | PendingDecision::ChooseManaColor { player, .. }
        | PendingDecision::ChooseTargets { player, .. }
        | PendingDecision::PayMana { player, .. }
        | PendingDecision::OrderTriggers { player, .. }
        | PendingDecision::DeclareAttackers { player, .. }
        | PendingDecision::DeclareBlockers { player, .. }
        | PendingDecision::AssignCombatDamage { player, .. }
        | PendingDecision::ChooseXValue { player, .. }
        | PendingDecision::LegendRule { player, .. } => *player,
        other => todo!("P0.W3: strategy for shell decision kind {other:?}"),
    }
}

// --- driving the game
// ---------------------------------------------------------

fn build_deck(spell: &Arc<Card>, land: &Arc<Card>) -> Vec<Arc<Card>> {
    let mut deck = vec![Arc::clone(spell); SPELLS_PER_DECK];
    deck.extend(vec![Arc::clone(land); LANDS_PER_DECK]);
    deck
}

/// Steps until the next decision or game over, returning the progress trace.
fn step_to_stop(state: &mut GameState) -> (Vec<Progress>, StepOutcome) {
    let mut trace = Vec::new();
    loop {
        match state.step() {
            StepOutcome::Progress(p) => trace.push(p),
            stop => return (trace, stop),
        }
    }
}

/// Records the milestone flags from a progress trace, given the (stable) player
/// proxy ids: a `DamageDealt` target tells player-vs-creature, the amount tells
/// creature combat (2) vs instant burn (3).
fn note_milestones(
    trace: &[Progress],
    proxies: [ObjectId; 2],
    spell_killed_creature: &mut bool,
    creature_hit_player: &mut bool,
) {
    for p in trace {
        let events: &[GameEvent] = match p {
            Progress::Applied(Occurrence::Single(e)) => std::slice::from_ref(e),
            Progress::Applied(Occurrence::Batch(es)) => es,
            _ => &[],
        };
        for ev in events {
            if let GameEvent::DamageDealt { target, amount, .. } = ev {
                let to_player = proxies.contains(target);
                if to_player && *amount == 2 {
                    *creature_hit_player = true;
                } else if !to_player && *amount == 3 {
                    *spell_killed_creature = true;
                }
            }
        }
    }
}

/// Plays one full game with the given per-seat strategies and returns its
/// summary. `p0` drives `PlayerId(0)`, `p1` drives `PlayerId(1)`.
///
/// Both players draw every turn from a finite 40-card library, so the game
/// always terminates (a board stall ends in a deck-out by ~turn 33).
///
/// # Panics
///
/// Panics if a game fails to terminate within sane bounds, or a strategy ever
/// submits a decision the engine rejects — both would signal an engine bug.
#[must_use]
pub fn play(cards: &DeckCards, seed: u64, p0: &dyn Strategy, p1: &dyn Strategy) -> Summary {
    let mut state = GameState::new(GameConfig {
        players: vec![
            PlayerConfig {
                deck: build_deck(&cards.p0_spell, &cards.p0_land),
            },
            PlayerConfig {
                deck: build_deck(&cards.p1_spell, &cards.p1_land),
            },
        ],
        seed,
        starting_life: 20,
        starting_player: StartingPlayer::Fixed(PlayerId(0)),
        sba_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
    });
    let strategies: [&dyn Strategy; 2] = [p0, p1];

    let proxies = [state.players[0].object, state.players[1].object];
    let mut spell_killed_creature = false;
    let mut creature_hit_player = false;

    let mut guard = 0u32;
    let outcome = loop {
        guard += 1;
        assert!(guard < 1_000_000, "game did not terminate (livelock?)");
        assert!(
            state.turn.turn_number < 200,
            "game ran absurdly long ({} turns)",
            state.turn.turn_number
        );

        let (trace, stop) = step_to_stop(&mut state);
        note_milestones(
            &trace,
            proxies,
            &mut spell_killed_creature,
            &mut creature_hit_player,
        );

        match stop {
            StepOutcome::GameOver(o) => break o,
            StepOutcome::NeedsDecision(pending) => {
                let who = pending_player(&pending);
                let d = strategies[who.index()].decide(&state, &pending);
                state
                    .submit_decision(d)
                    .expect("a strategy submits only legal decisions");
            }
            StepOutcome::Progress(_) => unreachable!("step_to_stop drained all progress"),
        }
    };

    let (loser_lost_for_real, decked) = match outcome {
        GameOutcome::Win(w) => {
            let p = &state.players[opponent(w).index()];
            (
                p.lost && (p.life <= 0 || p.drew_from_empty),
                p.drew_from_empty,
            )
        }
        GameOutcome::Draw => (false, false),
    };

    Summary {
        outcome,
        turns: state.turn.turn_number,
        life: [state.players[0].life, state.players[1].life],
        spell_killed_creature,
        creature_hit_player,
        loser_lost_for_real,
        decked,
    }
}
