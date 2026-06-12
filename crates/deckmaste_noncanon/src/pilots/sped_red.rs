//! The burn seat. Priors: always make the land drop; otherwise consider
//! every (burn spell × target) line plus passing, and let lookahead pick.
//! Combat is delegated to the Tactician.
//!
//! # Mana-float protocol
//!
//! The engine only surfaces `CastSpell` when the pool can already pay the
//! cost. So the first time a new-priority window opens with `ActivateAbility`
//! (mana ability) but no matching `CastSpell`, we look for hand spells that
//! WOULD become castable after floating. We score the intended full line
//! (float + cast + target) via `tac.score` — the rollout's sequential
//! dispatch handles the float→cast→target sequence correctly. If the score
//! beats passing we start floating immediately; on the next priority window
//! (after each mana ability) the `PlannedQueue` answers subsequent floats, and
//! once the pool can pay, the engine finally surfaces `CastSpell` which the
//! queue handles too.

use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PendingDecision;

use crate::lookahead::Horizon;
use crate::lookahead::Line;
use crate::lookahead::Tactician;
use crate::observe::ObjView;
use crate::observe::Observation;
use crate::pilot::Pilot;
use crate::pilot::PlannedQueue;
use crate::pilot::mechanical;
use crate::pilot::pool_total;
use crate::pilots::cast_line;
use crate::pilots::mana_floats;

#[derive(Default)]
pub struct SpedRed {
    planned: PlannedQueue,
}

impl Pilot for SpedRed {
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
                // 1. Land drop, always.
                if let Some(a) = legal.iter().find(|a| matches!(a, Action::PlayLand { .. })) {
                    return Decision::Act(a.clone());
                }
                let mana_actions = mana_floats(legal, obs);

                // 2. Score every burn line (float* + cast + target), where the spell may or may
                //    not already appear as CastSpell.  We enumerate spells in hand that are NOT
                //    lands (Mountains sometimes appear as CastSpell with zero cost — ignore
                //    them) and have at least one valid target.
                let have = pool_total(&obs.my_pool);
                let pass_score = tac.score(&crate::lookahead::Line::pass(), Horizon::EndOfTurn);
                let mut best: Option<(i64, crate::lookahead::Line)> = None;

                // Build the candidate cast-spell actions: prefer an explicit
                // CastSpell entry (engine confirmed it's legal now) but also
                // synthesize one for spells we can reach via floating.
                // Dedup by spell name first: wave-0 decks can hold up to 44
                // identical Shocks — identical names produce identical lines,
                // so we score the first copy and skip the rest.
                let candidate_spells: Vec<(Action, &ObjView)> = {
                    let mut seen_names: std::collections::HashSet<&str> =
                        std::collections::HashSet::new();
                    let mut v: Vec<(Action, &ObjView)> = Vec::new();
                    for spell in obs.my_hand.iter().filter(|s| !s.is_land()) {
                        if !seen_names.insert(spell.name.as_str()) {
                            continue; // already scored a copy of this card name
                        }
                        let need = (spell.mana_value as usize).saturating_sub(have);
                        if need > mana_actions.len() {
                            continue; // can't afford even with all available mana
                        }
                        // Find the matching CastSpell action if present.
                        let cast_action = legal
                            .iter()
                            .find(|a| matches!(a, Action::CastSpell { object } if *object == spell.id))
                            .cloned();
                        if let Some(cast) = cast_action {
                            v.push((cast, spell));
                        } else if need == 0 && have == 0 {
                            // need == 0 with have == 0 means mana_value == 0:
                            // a zero-cost spell. The engine surfaces CastSpell
                            // for those when they're castable; if it didn't,
                            // some other restriction applies. Skip rather
                            // than synthesize.
                        } else {
                            // Not yet in legal (pool can't pay yet) but will be
                            // once we float `need` mana.  Build the full
                            // lookahead line to test legality via score.
                            let cast = Action::CastSpell { object: spell.id };
                            v.push((cast, spell));
                        }
                    }
                    v
                };

                for (cast_action, spell) in candidate_spells {
                    // Creature spells take no targets at cast; everything
                    // else tries the face first, then each opposing creature.
                    let target_menu: Vec<Option<ObjectId>> = if spell.is_creature() {
                        vec![None]
                    } else {
                        let mut t: Vec<Option<ObjectId>> = vec![Some(obs.opp_proxy)];
                        t.extend(
                            obs.opp_battlefield()
                                .filter(|v| v.is_creature())
                                .map(|v| Some(v.id)),
                        );
                        t
                    };
                    for t in target_menu {
                        let Some(line) =
                            cast_line(spell, t, cast_action.clone(), &mana_actions, obs)
                        else {
                            continue;
                        };
                        let score = tac.score(&line, Horizon::EndOfTurn);
                        if best.as_ref().is_none_or(|(b, _)| score > *b) {
                            best = Some((score, line));
                        }
                    }
                }

                // 3. Activation lines: non-mana activated abilities of our own permanents (e.g.
                //    Mogg Fanatic's sacrifice), tried at the burn target menu. Scored like cast
                //    lines — a mismatched line rolls out as ILLEGAL and loses. Known horizon
                //    artifact: end-of-turn eval can't see a sacrificed body's future turns, so
                //    sac-for-face fires eagerly.
                for action in legal {
                    let Action::ActivateAbility { object, ability } = action else {
                        continue;
                    };
                    let Some(perm) = obs.my_battlefield().find(|v| v.id == *object) else {
                        continue;
                    };
                    if perm.is_land()
                        || perm
                            .abilities
                            .get(*ability)
                            .is_none_or(crate::observe::is_mana_ability)
                    {
                        continue; // mana sources are floats, not plays
                    }
                    let mut targets: Vec<ObjectId> = vec![obs.opp_proxy];
                    targets.extend(
                        obs.opp_battlefield()
                            .filter(|v| v.is_creature())
                            .map(|v| v.id),
                    );
                    for t in targets {
                        let line = Line::new(vec![
                            Decision::Act(action.clone()),
                            Decision::Targets(vec![t]),
                        ]);
                        let score = tac.score(&line, Horizon::EndOfTurn);
                        if best.as_ref().is_none_or(|(b, _)| score > *b) {
                            best = Some((score, line));
                        }
                    }
                }
                match best {
                    Some((score, line)) if score > pass_score => {
                        let mut queued = line.queued;
                        let first = queued.remove(0);
                        self.planned.plan(queued);
                        first
                    }
                    _ => Decision::Act(Action::Pass),
                }
            }
            PendingDecision::DeclareAttackers { legal, .. } => {
                Decision::Attackers(tac.best_attack(legal))
            }
            PendingDecision::DeclareBlockers { legal, .. } => {
                Decision::Blocks(tac.best_blocks(legal))
            }
            PendingDecision::ChooseTargets { legal, .. } => {
                // A planned line normally answers this; if the plan was
                // abandoned, fall back to the first candidates.
                Decision::Targets(legal.iter().map(|c| c[0]).collect())
            }
            other => mechanical(obs, other),
        }
    }
}
