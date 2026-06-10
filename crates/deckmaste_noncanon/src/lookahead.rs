//! Clone-and-simulate tactical lookahead.
//!
//! Honesty boundary: a rollout clones the full `GameState` (the only way to
//! step the engine), but the cloned hidden zones are never *consulted* — the
//! seat plays only its scripted `Line` then passes, the opponent is modeled
//! as passing (blocking greedily when forced to declare), and every horizon
//! stops at the next turn boundary, before any draw resolves. The evaluator
//! reads life, board, and hand *counts* only.

use std::cmp::Reverse;
use std::collections::VecDeque;

use deckmaste_core::Phase;
use deckmaste_core::Type;
use deckmaste_engine::Decision;
use deckmaste_engine::GameEvent;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Occurrence;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerId;
use deckmaste_engine::Progress;
use deckmaste_engine::StepOutcome;

use crate::observe::face;
use crate::observe::opponent;
use crate::pilot::answers;
use crate::pilot::pay;

/// A scripted prefix of the seat's upcoming decisions (e.g. float, float,
/// cast, choose targets). Everything after the script is "pass".
#[derive(Debug, Clone, Default)]
pub struct Line {
    pub queued: Vec<Decision>,
}

impl Line {
    #[must_use]
    pub fn new(queued: Vec<Decision>) -> Self {
        Self { queued }
    }

    /// The do-nothing baseline.
    #[must_use]
    pub fn pass() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Horizon {
    /// Stop when the postcombat main phase begins.
    EndOfCombat,
    /// Stop when the next turn begins (before its draw).
    EndOfTurn,
}

pub const WIN_SCORE: i64 = 1_000_000;
const ILLEGAL_LINE: i64 = i64::MIN / 2;
const ROLLOUT_GUARD: u32 = 100_000;

pub struct Tactician<'a> {
    state: &'a GameState,
    seat: PlayerId,
}

impl<'a> Tactician<'a> {
    #[must_use]
    pub fn new(state: &'a GameState, seat: PlayerId) -> Self {
        Self { state, seat }
    }

    /// Scores a candidate line from the current decision point.
    ///
    /// # Panics
    ///
    /// Never in practice: the `expect` inside is guarded by `answers`, which
    /// only fires when `queued.front()` is `Some`.
    #[must_use]
    pub fn score(&self, line: &Line, horizon: Horizon) -> i64 {
        let mut sim = self.state.clone();
        let mut queued: VecDeque<Decision> = line.queued.iter().cloned().collect();
        let mut guard = 0u32;
        loop {
            guard += 1;
            if guard > ROLLOUT_GUARD {
                return eval(&sim, self.seat);
            }
            match sim.step() {
                StepOutcome::Progress(p) => {
                    if horizon_reached(&p, horizon) {
                        return eval(&sim, self.seat);
                    }
                }
                StepOutcome::GameOver(o) => {
                    return match o {
                        GameOutcome::Win(w) if w == self.seat => WIN_SCORE,
                        GameOutcome::Win(_) => -WIN_SCORE,
                        GameOutcome::Draw => 0,
                    };
                }
                StepOutcome::NeedsDecision(pending) => {
                    let who = pending_player(&pending);
                    let d = if who == self.seat {
                        match queued.front() {
                            Some(head) if answers(head, &pending) => {
                                queued.pop_front().expect("front exists")
                            }
                            _ => {
                                queued.clear(); // diverged: fall back to passive
                                model_decide(&sim, &pending)
                            }
                        }
                    } else {
                        model_decide(&sim, &pending)
                    };
                    if sim.submit_decision(d).is_err() {
                        return ILLEGAL_LINE;
                    }
                }
            }
        }
    }

    /// Picks the best attack declaration from the surfaced candidates.
    /// Exhaustive for narrow boards, power-ranked prefixes for wide ones.
    /// Ties prefer more attackers (aggro default).
    #[must_use]
    pub fn best_attack(&self, legal: &[ObjectId]) -> Vec<ObjectId> {
        let mut candidates: Vec<Vec<ObjectId>> = Vec::new();
        if legal.len() <= 6 {
            for mask in 0..(1u32 << legal.len()) {
                candidates.push(
                    legal
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, &o)| o)
                        .collect(),
                );
            }
        } else {
            let view = self.state.layers();
            let mut ranked: Vec<ObjectId> = legal.to_vec();
            ranked.sort_by_key(|&o| Reverse(view.power(o).unwrap_or(0)));
            for n in 0..=ranked.len() {
                candidates.push(ranked[..n].to_vec());
            }
        }
        candidates
            .into_iter()
            .map(|subset| {
                let score = self.score(
                    &Line::new(vec![Decision::Attackers(subset.clone())]),
                    Horizon::EndOfTurn,
                );
                (score, subset)
            })
            .max_by(|(sa, a), (sb, b)| sa.cmp(sb).then(a.len().cmp(&b.len())))
            .map(|(_, subset)| subset)
            .unwrap_or_default()
    }

    /// Picks blocks from a small candidate menu: no blocks, greedy value
    /// blocks, full chump, and (on narrow boards) each single block.
    #[must_use]
    pub fn best_blocks(&self, legal: &[ObjectId]) -> Vec<(ObjectId, ObjectId)> {
        let attackers = self.state.combat.attackers().to_vec();
        if attackers.is_empty() || legal.is_empty() {
            return vec![];
        }
        let view = self.state.layers();
        let mut candidates: Vec<Vec<(ObjectId, ObjectId)>> = vec![
            vec![],
            greedy_value_blocks(&view, legal, &attackers),
            full_chump(&view, legal, &attackers),
        ];
        if legal.len() <= 4 {
            for &b in legal {
                for &a in &attackers {
                    candidates.push(vec![(b, a)]);
                }
            }
        }
        candidates.sort();
        candidates.dedup();
        candidates
            .into_iter()
            .map(|blocks| {
                let score = self.score(
                    &Line::new(vec![Decision::Blocks(blocks.clone())]),
                    Horizon::EndOfTurn,
                );
                (score, blocks)
            })
            .max_by(|(sa, a), (sb, b)| sa.cmp(sb).then(b.len().cmp(&a.len())))
            .map(|(_, blocks)| blocks)
            .unwrap_or_default()
    }
}

/// Value blocks: each blocker takes the biggest attacker it kills while
/// surviving; one blocker per attacker.
fn greedy_value_blocks(
    view: &LayeredView,
    legal: &[ObjectId],
    attackers: &[ObjectId],
) -> Vec<(ObjectId, ObjectId)> {
    let mut taken: Vec<ObjectId> = Vec::new();
    let mut out = Vec::new();
    for &b in legal {
        let (bp, bt) = (view.power(b).unwrap_or(0), view.toughness(b).unwrap_or(0));
        let pick = attackers
            .iter()
            .filter(|a| !taken.contains(a))
            .filter(|&&a| {
                let (ap, at) = (view.power(a).unwrap_or(0), view.toughness(a).unwrap_or(0));
                bp >= at && bt > ap // kills it, survives it
            })
            .max_by_key(|&&a| view.power(a).unwrap_or(0));
        if let Some(&a) = pick {
            taken.push(a);
            out.push((b, a));
        }
    }
    out
}

/// Chump everything: blockers onto distinct attackers, biggest first.
fn full_chump(
    view: &LayeredView,
    legal: &[ObjectId],
    attackers: &[ObjectId],
) -> Vec<(ObjectId, ObjectId)> {
    let mut ranked: Vec<ObjectId> = attackers.to_vec();
    ranked.sort_by_key(|&a| Reverse(view.power(a).unwrap_or(0)));
    legal
        .iter()
        .zip(ranked.iter())
        .map(|(&b, &a)| (b, a))
        .collect()
}

/// The passive in-rollout policy for both the modeled opponent and the seat
/// once its line is exhausted: pass priority, never attack, block greedily,
/// answer forced decisions blandly. Reads public zones plus the pending
/// payload; the one soft spot (discard order during a modeled cleanup) picks
/// arbitrarily rather than by card quality, so no hidden information shapes
/// the outcome.
#[must_use]
pub fn model_decide(state: &GameState, pending: &PendingDecision) -> Decision {
    match pending {
        PendingDecision::Priority { .. } => Decision::Act(deckmaste_engine::Action::Pass),
        PendingDecision::DeclareAttackers { .. } => Decision::Attackers(vec![]),
        PendingDecision::DeclareBlockers { legal, .. } => {
            let view = state.layers();
            Decision::Blocks(greedy_value_blocks(&view, legal, state.combat.attackers()))
        }
        PendingDecision::ChooseTargets { legal, .. } => {
            Decision::Targets(legal.iter().map(|c| c[0]).collect())
        }
        PendingDecision::PayMana { cost, pool, .. } => Decision::Pay(pay(cost, pool)),
        PendingDecision::OrderTriggers { triggers, .. } => {
            Decision::Order((0..triggers.len()).collect())
        }
        PendingDecision::DiscardToHandSize { player, count } => Decision::Discard(
            state.zones.hands[player.index()]
                .iter()
                .copied()
                .take(*count as usize)
                .collect(),
        ),
        PendingDecision::AssignCombatDamage {
            source, recipients, ..
        } => {
            let view = state.layers();
            let mut left = view.power(*source).unwrap_or(0).max(0).unsigned_abs();
            let mut out = Vec::with_capacity(recipients.len());
            for (i, &r) in recipients.iter().enumerate() {
                let need = view.toughness(r).unwrap_or(1).max(0).unsigned_abs();
                let give = if i == recipients.len() - 1 { left } else { need.min(left) };
                left -= give;
                out.push((r, give));
            }
            Decision::Assignment(out)
        }
    }
}

#[must_use]
pub fn pending_player(pending: &PendingDecision) -> PlayerId {
    match pending {
        PendingDecision::Priority { player, .. }
        | PendingDecision::DiscardToHandSize { player, .. }
        | PendingDecision::ChooseTargets { player, .. }
        | PendingDecision::PayMana { player, .. }
        | PendingDecision::OrderTriggers { player, .. }
        | PendingDecision::DeclareAttackers { player, .. }
        | PendingDecision::DeclareBlockers { player, .. }
        | PendingDecision::AssignCombatDamage { player, .. } => *player,
    }
}

#[must_use]
pub fn events_of(progress: &Progress) -> &[GameEvent] {
    match progress {
        Progress::Applied(Occurrence::Single(e)) => std::slice::from_ref(e),
        Progress::Applied(Occurrence::Batch(es)) => es,
        _ => &[],
    }
}

fn horizon_reached(progress: &Progress, horizon: Horizon) -> bool {
    events_of(progress).iter().any(|e| match horizon {
        Horizon::EndOfCombat => matches!(e, GameEvent::StepBegan(Phase::PostcombatMain)),
        Horizon::EndOfTurn => matches!(e, GameEvent::TurnBegan { .. }),
    })
}

/// Position score from `seat`'s side: life swing dominates, then board
/// presence, then hand size (counts only — no hidden contents).
#[must_use]
pub fn eval(state: &GameState, seat: PlayerId) -> i64 {
    let opp = opponent(seat);
    let view = state.layers();
    let mut score = 0i64;
    score += (i64::from(state.players[seat.index()].life)
        - i64::from(state.players[opp.index()].life))
        * 100;
    for &o in &state.zones.battlefield {
        let controller = state.objects.obj(o).controller;
        let sign: i64 = if controller == seat { 1 } else { -1 };
        let f = face(state.def(o));
        if f.types.contains(&Type::Creature) {
            score += sign
                * (i64::from(view.power(o).unwrap_or(0)) * 8
                    + i64::from(view.toughness(o).unwrap_or(0)) * 4);
        } else {
            score += sign * 2; // lands/artifacts: small presence value
        }
    }
    score += (i64::try_from(state.zones.hands[seat.index()].len()).unwrap_or(0)
        - i64::try_from(state.zones.hands[opp.index()].len()).unwrap_or(0))
        * 12;
    score
}
