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

use deckmaste_card::Card;
use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::Int;
use deckmaste_core::PhaseStep;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;
use deckmaste_core::Uint;

use crate::Action;
use crate::DamageDealt;
use crate::Decision;
use crate::DecisionPointKind;
use crate::GameConfig;
use crate::GameEvent;
use crate::GameOutcome;
use crate::GameState;
use crate::ManaPool;
use crate::ObjectId;
use crate::Occurrence;
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
            Card::Normal(f)
            | Card::DoubleFaced { front: f, .. }
            | Card::Split { left: f, .. }
            | Card::Flip { normal: f, .. }
            | Card::Adventurer { normal: f, .. } => {
                f.characteristics.types.iter().any(|t| t.name == ty.name())
            }
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
        Card::Normal(f)
        | Card::DoubleFaced { front: f, .. }
        | Card::Split { left: f, .. }
        | Card::Flip { normal: f, .. }
        | Card::Adventurer { normal: f, .. } => &f.characteristics.mana_cost,
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
    fn decide(&self, state: &GameState, pending: &DecisionPointKind) -> Decision;
}

/// P0's seat: develop a land, then float and cast a creature whenever it can,
/// and attack with everything (the all-out swing lives in `mechanical`'s
/// `DeclareAttackers`). It casts no targeted spells, so it never chooses
/// targets.
pub struct GreedyCreatures;

impl Strategy for GreedyCreatures {
    fn decide(&self, state: &GameState, pending: &DecisionPointKind) -> Decision {
        match pending {
            DecisionPointKind::Priority(crate::decide::pending::Priority { player, legal }) => {
                Decision::Act(greedy_priority(
                    state,
                    *player,
                    legal,
                    Type::Creature,
                    false,
                ))
            }
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
    fn decide(&self, state: &GameState, pending: &DecisionPointKind) -> Decision {
        match pending {
            DecisionPointKind::Priority(crate::decide::pending::Priority { player, legal }) => {
                Decision::Act(greedy_priority(state, *player, legal, Type::Instant, true))
            }
            DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
                player,
                legal,
                ..
            }) => Decision::Targets(choose_targets(state, *player, legal)),
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
/// The targeting arm runs in the deterministic demo (its fixed seed); the
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
    fn decide(&self, state: &GameState, pending: &DecisionPointKind) -> Decision {
        match pending {
            DecisionPointKind::Priority(crate::decide::pending::Priority { player, legal }) => {
                Decision::Act(greedy_priority(
                    state,
                    *player,
                    legal,
                    Type::Creature,
                    false,
                ))
            }
            // The demo's burn / sac-outlet pings ("any target") and any other
            // targeted effect: a legal candidate set per spec slot.
            DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets {
                player,
                spec,
                legal,
                ..
            }) => Decision::Targets(choose_targets_any(state, *player, spec, legal)),
            // A multi-blocked attacker ([CR#510.1c]): any split summing to the
            // source's power is legal; dump it all on the first recipient.
            DecisionPointKind::AssignCombatDamage(crate::decide::pending::AssignCombatDamage {
                source,
                recipients,
                ..
            }) => Decision::Assignment(assign_all_to_first(state, *source, recipients)),
            other => mechanical(state, other),
        }
    }
}

/// Choose a legal target SET per `TargetSpec` slot. For a single-slot spec it
/// reuses the removal heuristic (trim the board, else the face); for multi-slot
/// specs it takes each slot's minimum-count first-legal candidates, honoring
/// within-slot + `Distinct` constraints ([`pick_target_set`]). The demo's
/// targeted cards are single "any target" pings, so the heuristic path is what
/// runs in practice.
fn choose_targets_any(
    state: &GameState,
    player: PlayerId,
    specs: &[TargetSpec],
    legal: &[Vec<ObjectId>],
) -> Vec<Vec<ObjectId>> {
    if legal.len() == 1 {
        return choose_targets(state, player, legal);
    }
    pick_target_set(specs, legal)
}

/// A mechanically-valid per-slot target set ([CR#601.2c,115.7e]): choose each
/// slot's MINIMUM count of first-legal candidates, greedily skipping any that
/// would break within-slot distinctness or a `Distinct` constraint against an
/// already-chosen sibling slot. Slots fill in order, so a `Distinct` slot sees
/// its (earlier-indexed, per the semantic convention) siblings' picks; the
/// submission validator is the true enforcement regardless. Always valid when
/// the specs are announce-satisfiable; a min-0 slot contributes an empty set.
pub(crate) fn pick_target_set(specs: &[TargetSpec], legal: &[Vec<ObjectId>]) -> Vec<Vec<ObjectId>> {
    let mut chosen: Vec<Vec<ObjectId>> = vec![Vec::new(); specs.len()];
    for (i, spec) in specs.iter().enumerate() {
        let (min, _max) = crate::resolve::slot_count_bounds(spec);
        let min = usize::try_from(min).expect("min count fits usize");
        let siblings = crate::resolve::distinct_siblings(spec);
        for &cand in &legal[i] {
            if chosen[i].len() >= min {
                break;
            }
            let clashes = chosen[i].contains(&cand)
                || siblings
                    .iter()
                    .any(|&s| chosen.get(s).is_some_and(|set| set.contains(&cand)));
            if !clashes {
                chosen[i].push(cand);
            }
        }
    }
    chosen
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
                PhaseStep::PrecombatMain | PhaseStep::PostcombatMain
            );
        if !my_main {
            return Action::Pass;
        }
    }

    // 1. Develop: a land if offered (legality enforces one per turn).
    if let Some(a) = legal.iter().find(|a| matches!(a, Action::PlayLand { .. })) {
        return a.clone();
    }
    let have = pool_total(&state.players[player.index()].mana_pool);
    let usable_mana_sources = legal
        .iter()
        .filter(|action| match action {
            Action::ActivateAbility { object, ability } => {
                state.mana_ability(*object, *ability).is_some()
                    && crate::payment::automatic_activation_cost_usable(state, *object, *ability)
            }
            _ => false,
        })
        .count();

    // 2. Deploy when this deliberately simple monocolor runner can reach the
    // spell's mana value from its pool plus currently usable mana sources. The
    // engine intentionally offers the proposal without proving affordability;
    // avoiding a futile announce/decline loop is strategy policy.
    if let Some(a) = legal.iter().find(|a| {
        matches!(a, Action::CastSpell { object }
                if has_type(state, *object, want)
                    && have + usable_mana_sources >= mana_value(state, *object) as usize)
    }) {
        return a.clone();
    }
    // 3. Ramp: float one mana toward the cheapest castable card, but only when the
    //    untapped lands can actually reach its cost.
    if let Some(mv) = cheapest_in_hand(state, player, want)
        && have < mv as usize
        && have + usable_mana_sources >= mv as usize
        && let Some(a) = legal.iter().find(|action| match action {
            Action::ActivateAbility { object, ability } => {
                state.mana_ability(*object, *ability).is_some()
                    && crate::payment::automatic_activation_cost_usable(state, *object, *ability)
            }
            _ => false,
        })
    {
        return a.clone();
    }
    Action::Pass
}

/// Removal targeting: trim the board when the opponent has 2+ creatures (kill
/// one — exercising the lethal SBA), otherwise burn the opponent's face (which
/// also lets a lone attacker connect). A single quantity-one `AnyTarget` spec,
/// so the answer is one slot holding one target.
fn choose_targets(
    state: &GameState,
    player: PlayerId,
    legal: &[Vec<ObjectId>],
) -> Vec<Vec<ObjectId>> {
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
        return vec![vec![creature]];
    }
    vec![vec![state.players[opp.index()].object]]
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
pub(crate) fn mechanical(state: &GameState, pending: &DecisionPointKind) -> Decision {
    match pending {
        DecisionPointKind::DiscardToHandSize(crate::decide::pending::DiscardToHandSize {
            player,
            count,
        })
        | DecisionPointKind::DiscardCards(crate::decide::pending::DiscardCards { player, count }) => {
            Decision::Discard(choose_discards(state, *player, *count))
        }
        // Greedy default: the first offered option (printed order).
        DecisionPointKind::ChooseManaColor(crate::decide::pending::ChooseManaColor {
            options,
            ..
        }) => Decision::ManaColor(*options.first().expect("a mana choice offers options")),
        // Greedy default: the first offered run (printed order).
        DecisionPointKind::ChooseManaMode(crate::decide::pending::ChooseManaMode { .. }) => {
            Decision::ManaMode(0)
        }
        // Route through `auto_pay_pending` so the autotapper honors the
        // subject's `SpendOnly` restrictions ([CR#106.6]).
        DecisionPointKind::PayMana(crate::decide::pending::PayMana { .. }) => {
            Decision::Pay(state.auto_pay_pending())
        }
        DecisionPointKind::Payment(_) => state.auto_payment_pending().expect("Payment is pending"),
        DecisionPointKind::ChooseManaReversals(crate::decide::pending::ChooseManaReversals {
            legal,
            ..
        }) => Decision::ManaReversals(
            legal
                .iter()
                .max_by_key(|set| set.len())
                .cloned()
                .expect("a mana-reversal prompt offers at least one legal set"),
        ),
        DecisionPointKind::OrderTriggers(crate::decide::pending::OrderTriggers {
            triggers,
            ..
        }) => Decision::Order((0..triggers.len()).collect()),
        // Attack with everything legal (the creature seat swings; the removal
        // seat has no creatures, so its set is always empty). Each attacker
        // attacks the defending player's proxy ([CR#508.1b]) — the sole
        // legal target when no planeswalkers are in play.
        DecisionPointKind::DeclareAttackers(crate::decide::pending::DeclareAttackers {
            legal,
            legal_targets,
            ..
        }) => {
            let target = *legal_targets
                .first()
                .expect("the defending player's proxy is always a legal target");
            Decision::Attackers(legal.iter().map(|&a| (a, target)).collect())
        }
        // The defender never has a creature to block with.
        DecisionPointKind::DeclareBlockers(crate::decide::pending::DeclareBlockers { .. }) => {
            Decision::Blocks(vec![])
        }
        // Unblocked attackers are forced (one recipient); no multi-block arises.
        DecisionPointKind::AssignCombatDamage(crate::decide::pending::AssignCombatDamage {
            source,
            recipients,
            ..
        }) => unreachable!(
            "no multi-block in this matchup (source {source:?}, recipients {recipients:?})"
        ),
        DecisionPointKind::Priority(crate::decide::pending::Priority { .. })
        | DecisionPointKind::ChooseTargets(crate::decide::pending::ChooseTargets { .. }) => {
            unreachable!("priority and targeting are a strategy's own concern")
        }
        DecisionPointKind::ChooseObjects(crate::decide::pending::ChooseObjects {
            candidates,
            min,
            ..
        }) => Decision::Chosen(
            candidates
                .iter()
                .copied()
                .take(usize::try_from(*min).expect("min fits usize"))
                .collect(),
        ),
        // [CR#601.2b,608.2c]: the headless strategy announces the minimum (0)
        // for both the X-announce and a resolution note number ("choose a
        // number") — both answered through the `Decision::XValue` shape. A
        // smarter value is a follow-up; the note-number arm closes that slice
        // of the shell-decision-strategy seam.
        DecisionPointKind::ChooseXValue(crate::decide::pending::ChooseXValue { .. })
        | DecisionPointKind::ChooseNoteNumber(crate::decide::pending::ChooseNoteNumber {
            ..
        }) => Decision::XValue(0),
        DecisionPointKind::ChooseNoteCardName(crate::decide::pending::ChooseNoteCardName {
            player,
            ..
        }) => {
            let name = state.zones.hands[player.index()].first().map_or_else(
                || "Mountain".to_owned(),
                |&id| {
                    crate::derive::face(state.def(id))
                        .characteristics
                        .name
                        .to_string()
                },
            );
            Decision::CardName(name)
        }
        // [CR#705.2]: the call is strategically null (win is a fair coin
        // either way) — the headless strategy always calls heads.
        DecisionPointKind::CallFlip(crate::decide::pending::CallFlip { .. }) => {
            Decision::Answer(true)
        }
        // [CR#707.10c]: keeping every current target is always legal (the
        // union rule) — the headless strategy re-targets nothing.
        DecisionPointKind::Retarget(crate::decide::pending::Retarget {
            entry,
            spec,
            legal,
            ..
        }) => Decision::Targets(keep_current_targets(state, *entry, spec, legal)),
        other => todo!(
            "engine seam: shell decision strategy for {other:?} — no mechanical/pending-player \
             handling wired for this DecisionPointKind kind yet; owner: engine-shell-decision-strategies"
        ),
    }
}

/// [CR#707.10c]: answer a `Retarget` re-target by keeping every
/// current target — the union rule makes "leave every slot unchanged" always
/// legal, so no strategy ever re-targets a committed entry. Reads the stack
/// entry's current targets directly; if the entry has since left the stack (a
/// race between the decision surfacing and being answered) falls back to the
/// first legal candidate per slot — still always legal, never a panic. Shared
/// by `mechanical()` and `StrategyEvaluator::fallback` (the union rule has no
/// preference to consult, so the answer is strategy-independent).
pub(crate) fn keep_current_targets(
    state: &GameState,
    entry: ObjectId,
    specs: &[TargetSpec],
    legal: &[Vec<ObjectId>],
) -> Vec<Vec<ObjectId>> {
    if let Some(e) = state.stack.iter().find(|e| e.id == entry)
        && e.targets.len() == legal.len()
    {
        return e.targets.clone();
    }
    // The entry left the stack between surfacing and answer ([CR#707.10c]) —
    // the write is a no-op, but the submission is still shape-validated, so hand
    // back a valid minimum set (within-slot + Distinct respected).
    pick_target_set(specs, legal)
}

/// The seat a surfaced decision is waiting on — every variant names its player.
pub(crate) fn pending_player(pending: &DecisionPointKind) -> PlayerId {
    pending.decider_player()
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
            if let GameEvent::DamageDealt(DamageDealt { target, amount, .. }) = ev {
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
        // Headless sim hardcodes empty rules-as-data (ticket
        // `sim-play-wire-rules-as-data` wires `sba_rules`/`conferral_rules`/
        // `damage_result_rules` from the loaded plugin). Until then a
        // planeswalker in sim takes no loyalty loss from damage — acceptable for
        // the current land/creature sim corpus.
        sba_rules: vec![],
        conferral_rules: vec![],
        damage_result_rules: vec![],
        counter_decls: std::collections::HashMap::new(),
        subtypes: std::collections::HashMap::new(),
        types: std::collections::HashMap::new(),
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
