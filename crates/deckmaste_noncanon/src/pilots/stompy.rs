//! The creature seat. Priors: land drop, then deploy the biggest affordable
//! creature (no lookahead needed for simple development); combat via the
//! Tactician.
//!
//! # Mana-float protocol
//!
//! The engine only surfaces `CastSpell` when the pool can already pay the
//! cost. Stompy must float mana first when the pool is empty: we enumerate
//! creatures in hand that would be castable after floating, build the full
//! float + cast line, and execute it.  See the `sped_red` module for a longer
//! explanation.

use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::PendingDecision;

use crate::lookahead::Tactician;
use crate::observe::Observation;
use crate::pilot::Pilot;
use crate::pilot::PlannedQueue;
use crate::pilot::mechanical;
use crate::pilot::pool_total;
use crate::pilots::cast_line;

#[derive(Default)]
pub struct Stompy {
    planned: PlannedQueue,
}

impl Pilot for Stompy {
    fn decide(
        &mut self,
        obs: &Observation,
        tac: &Tactician<'_>,
        pending: &PendingDecision,
    ) -> Decision {
        if let Some(d) = self.planned.try_answer(obs, pending) {
            return d;
        }
        match pending {
            PendingDecision::Priority { legal, .. } => {
                if let Some(a) = legal.iter().find(|a| matches!(a, Action::PlayLand { .. })) {
                    return Decision::Act(a.clone());
                }
                // Sorcery-speed development on our own main phases only.
                let my_main = obs.active == obs.seat
                    && matches!(
                        obs.phase,
                        deckmaste_core::Phase::PrecombatMain
                            | deckmaste_core::Phase::PostcombatMain
                    );
                if my_main && obs.stack_size == 0 {
                    let mana_actions: Vec<Action> = legal
                        .iter()
                        .filter(|a| matches!(a, Action::ActivateAbility { .. }))
                        .cloned()
                        .collect();
                    let have = pool_total(&obs.my_pool);

                    // Biggest affordable creature first (by mana value).
                    // Include creatures whose cost we can reach by floating.
                    // Wave-0 assumption: mono-color costs + mono-color sources, so "enough
                    // mana actions" implies payable. Revisit with generic/off-color pips.
                    let mut castable: Vec<(crate::lookahead::Line, u32)> = obs
                        .my_hand
                        .iter()
                        .filter(|s| s.is_creature())
                        .filter_map(|spell| {
                            let need = (spell.mana_value as usize).saturating_sub(have);
                            if need > mana_actions.len() {
                                return None;
                            }
                            // Prefer an explicit CastSpell entry; synthesize
                            // one otherwise (we'll float first).
                            let cast_action = legal
                                .iter()
                                .find(|a| {
                                    matches!(a, Action::CastSpell { object } if *object == spell.id)
                                })
                                .cloned()
                                .unwrap_or(Action::CastSpell { object: spell.id });
                            // Creatures need no target; cast_line with None
                            // builds the float* + cast sequence.
                            cast_line(spell, None, cast_action, &mana_actions, obs)
                                .map(|line| (line, spell.mana_value))
                        })
                        .collect();
                    // Biggest creature first.
                    castable.sort_by_key(|(_, mv)| std::cmp::Reverse(*mv));
                    for (line, _) in castable {
                        let mut queued = line.queued;
                        // Each priority window deploys at most one creature; the
                        // remainder of the line (usually just the cast itself) is
                        // queued and answered on the next window.
                        if queued.is_empty() {
                            continue;
                        }
                        let first = queued.remove(0);
                        self.planned.plan(queued);
                        return first;
                    }
                }
                Decision::Act(Action::Pass)
            }
            PendingDecision::DeclareAttackers { legal, .. } => {
                Decision::Attackers(tac.best_attack(legal))
            }
            PendingDecision::DeclareBlockers { legal, .. } => {
                Decision::Blocks(tac.best_blocks(legal))
            }
            PendingDecision::ChooseTargets { legal, .. } => {
                Decision::Targets(legal.iter().map(|c| c[0]).collect())
            }
            other => mechanical(obs, other),
        }
    }
}
