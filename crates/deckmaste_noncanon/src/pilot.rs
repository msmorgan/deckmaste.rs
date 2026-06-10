//! The pilot seam: per-seat decision-makers that see only the honest
//! `Observation` plus a `Tactician` for lookahead questions.

use std::collections::VecDeque;

use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::Uint;
use deckmaste_engine::Decision;
use deckmaste_engine::ManaPool;
use deckmaste_engine::ObjectId;
use deckmaste_engine::Payment;
use deckmaste_engine::PendingDecision;

use crate::lookahead::Tactician;
use crate::observe::Observation;

pub trait Pilot {
    fn decide(
        &mut self,
        obs: &Observation,
        tac: &Tactician<'_>,
        pending: &PendingDecision,
    ) -> Decision;
}

/// Decisions a pilot planned ahead of time (the tail of a chosen line). Pops
/// only when the queued decision answers the pending kind; any mismatch
/// abandons the rest of the plan (the world diverged).
#[derive(Default)]
pub struct PlannedQueue(VecDeque<Decision>);

impl PlannedQueue {
    pub fn plan(&mut self, decisions: impl IntoIterator<Item = Decision>) {
        self.0 = decisions.into_iter().collect();
    }

    pub fn try_answer(&mut self, pending: &PendingDecision) -> Option<Decision> {
        if self.0.front().is_some_and(|d| answers(d, pending)) {
            self.0.pop_front()
        } else {
            self.0.clear();
            None
        }
    }
}

/// Does this decision kind answer this pending kind?
#[must_use]
pub fn answers(d: &Decision, p: &PendingDecision) -> bool {
    matches!(
        (d, p),
        (Decision::Act(_), PendingDecision::Priority { .. })
            | (Decision::Targets(_), PendingDecision::ChooseTargets { .. })
            | (
                Decision::Attackers(_),
                PendingDecision::DeclareAttackers { .. }
            )
            | (Decision::Blocks(_), PendingDecision::DeclareBlockers { .. })
    )
}

#[must_use]
pub fn all_kinds() -> [ColorOrColorless; 6] {
    [
        ColorOrColorless::Colorless,
        ColorOrColorless::Color(Color::White),
        ColorOrColorless::Color(Color::Blue),
        ColorOrColorless::Color(Color::Black),
        ColorOrColorless::Color(Color::Red),
        ColorOrColorless::Color(Color::Green),
    ]
}

#[must_use]
pub fn pool_total(pool: &ManaPool) -> usize {
    all_kinds().iter().map(|&k| pool.amount(k) as usize).sum()
}

/// Covers `cost` from `pool`: colored pips forced by color, generic pips
/// filled from leftovers. (Same policy as the engine's sim harness.)
#[must_use]
pub fn pay(cost: &ManaCost, pool: &ManaPool) -> Payment {
    let mut remaining: Vec<(ColorOrColorless, Uint)> =
        all_kinds().iter().map(|&k| (k, pool.amount(k))).collect();
    let mut generic_pips: Uint = 0;
    for sym in cost.iter() {
        match sym {
            ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => generic_pips += *n,
            ManaSymbol::Simple(SimpleManaSymbol::Specific(c)) => {
                if let Some(slot) = remaining.iter_mut().find(|(k, _)| *k == *c) {
                    slot.1 = slot.1.saturating_sub(1);
                }
            }
            _ => {}
        }
    }
    let mut generic = Vec::new();
    for _ in 0..generic_pips {
        if let Some(slot) = remaining.iter_mut().find(|(_, n)| *n > 0) {
            slot.1 -= 1;
            generic.push(slot.0);
        }
    }
    Payment { generic }
}

/// Greedy lethal split for a multi-blocked attacker: give each recipient its
/// toughness until power runs out, dump any remainder on the last.
fn assign_damage(
    obs: &Observation,
    source: ObjectId,
    recipients: &[ObjectId],
) -> Vec<(ObjectId, Uint)> {
    let power = obs
        .battlefield
        .iter()
        .find(|v| v.id == source)
        .and_then(|v| v.power)
        .unwrap_or(0)
        .max(0);
    let mut left = power.unsigned_abs();
    let mut out: Vec<(ObjectId, Uint)> = Vec::with_capacity(recipients.len());
    for (i, &r) in recipients.iter().enumerate() {
        let need = obs
            .battlefield
            .iter()
            .find(|v| v.id == r)
            .and_then(|v| v.toughness)
            .unwrap_or(1)
            .max(0);
        let mut give = need.unsigned_abs().min(left);
        if i == recipients.len() - 1 {
            give = left; // dump the remainder
        }
        left -= give;
        out.push((r, give));
    }
    out
}

/// The forced / uniform decisions every pilot delegates: discards (lands
/// first), mana payment, trigger order (as-presented), damage assignment.
/// `Priority`, `ChooseTargets`, and combat declarations are a pilot's own
/// concern and panic here.
#[must_use]
pub fn mechanical(obs: &Observation, pending: &PendingDecision) -> Decision {
    match pending {
        PendingDecision::DiscardToHandSize { count, .. } => {
            let mut picks: Vec<ObjectId> = obs
                .my_hand
                .iter()
                .filter(|v| v.is_land())
                .map(|v| v.id)
                .collect();
            picks.extend(obs.my_hand.iter().filter(|v| !v.is_land()).map(|v| v.id));
            picks.truncate(*count as usize);
            Decision::Discard(picks)
        }
        PendingDecision::PayMana { cost, pool, .. } => Decision::Pay(pay(cost, pool)),
        PendingDecision::OrderTriggers { triggers, .. } => {
            Decision::Order((0..triggers.len()).collect())
        }
        PendingDecision::AssignCombatDamage {
            source, recipients, ..
        } => Decision::Assignment(assign_damage(obs, *source, recipients)),
        PendingDecision::Priority { .. }
        | PendingDecision::ChooseTargets { .. }
        | PendingDecision::DeclareAttackers { .. }
        | PendingDecision::DeclareBlockers { .. } => {
            unreachable!("a pilot must answer {pending:?} itself")
        }
    }
}
