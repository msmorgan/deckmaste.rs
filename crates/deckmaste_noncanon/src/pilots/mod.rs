//! The archetype pilots, plus the draw-go baseline.

pub mod sped_red;
pub mod stompy;

use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PendingDecision;

use crate::lookahead::Line;
use crate::lookahead::Tactician;
use crate::observe::ObjView;
use crate::observe::Observation;
use crate::pilot::Pilot;
use crate::pilot::mechanical;
use crate::pilot::pool_total;

/// Builds the candidate line for casting `spell` (a card in hand) at an
/// optional target: float mana off `mana_actions` (untapped sources surfaced
/// in the current legal list) until the printed cost is covered, then cast,
/// then choose targets. Mono-colored wave-0 simplification: any floated mana
/// pays any pip; revisit when a deck plays off-color sources.
#[must_use]
pub fn cast_line(
    spell: &ObjView,
    target: Option<ObjectId>,
    cast: Action,
    mana_actions: &[Action],
    obs: &Observation,
) -> Option<Line> {
    let have = pool_total(&obs.my_pool);
    let need = (spell.mana_value as usize).saturating_sub(have);
    if mana_actions.len() < need {
        return None; // can't reach the cost this turn
    }
    let mut queued: Vec<Decision> = mana_actions[..need]
        .iter()
        .map(|a| Decision::Act(a.clone()))
        .collect();
    queued.push(Decision::Act(cast));
    if let Some(t) = target {
        queued.push(Decision::Targets(vec![t]));
    }
    Some(Line::new(queued))
}

/// The legal `ActivateAbility` actions that are safe float sources: mana
/// abilities only. The engine surfaces ALL activated abilities at priority,
/// so without this filter a general activation (e.g. Mogg Fanatic's
/// sacrifice) would be queued as if it produced mana. Lands get a shortcut —
/// basics' mana abilities are intrinsic (not printed), and every land in the
/// current allowlists is a pure mana source; revisit at the first manland.
#[must_use]
pub fn mana_floats(legal: &[Action], obs: &Observation) -> Vec<Action> {
    legal
        .iter()
        .filter(|a| {
            let Action::ActivateAbility { object, ability } = a else {
                return false;
            };
            obs.battlefield
                .iter()
                .find(|v| v.id == *object)
                .is_some_and(|v| {
                    v.is_land()
                        || v.abilities
                            .get(*ability)
                            .is_some_and(crate::observe::is_mana_ability)
                })
        })
        .cloned()
        .collect()
}

/// Draw-go: passes every priority, never attacks or blocks. The baseline
/// opponent for deterministic behavior tests, and a deck-out smoke fixture.
pub struct PassBot;

impl Pilot for PassBot {
    fn decide(
        &mut self,
        obs: &Observation,
        _tac: &Tactician<'_>,
        pending: &PendingDecision,
    ) -> Decision {
        match pending {
            PendingDecision::Priority { .. } => Decision::Act(Action::Pass),
            PendingDecision::DeclareAttackers { .. } => Decision::Attackers(vec![]),
            PendingDecision::DeclareBlockers { .. } => Decision::Blocks(vec![]),
            PendingDecision::ChooseTargets { legal, .. } => {
                // The engine never surfaces an empty candidate set for a
                // slot; if it ever does, the panic here is the loudest
                // available signal.
                Decision::Targets(legal.iter().map(|c| c[0]).collect())
            }
            other => mechanical(obs, other),
        }
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::GameOutcome;

    use super::PassBot;
    use super::sped_red::SpedRed;
    use super::stompy::Stompy;
    use crate::deck;
    use crate::game::Setup;
    use crate::game::play_game;
    use crate::source::CardSource;
    use crate::wc99;

    fn subset_decks(src: &CardSource) -> [Vec<std::sync::Arc<deckmaste_core::Card>>; 2] {
        [
            deck::build_subset(&wc99::SPED_RED, wc99::SPED_RED_ALLOWLIST, src),
            deck::build_subset(&wc99::STOMPY, wc99::STOMPY_ALLOWLIST, src),
        ]
    }

    /// At 2 life, the burn pilot finds the Shock-the-face lethal line and
    /// never durdles to a deck-out.
    #[test]
    fn sped_red_finds_burn_lethal() {
        let src = CardSource::load();
        for seed in 0..5u64 {
            let rec = play_game(
                Setup {
                    decks: subset_decks(&src),
                    seed,
                    starting_life: 2,
                },
                &mut SpedRed::default(),
                &mut PassBot,
            );
            assert_eq!(
                rec.outcome,
                GameOutcome::Win(deckmaste_engine::PlayerId(0)),
                "seed {seed}: {rec:?}"
            );
            // Both orderings covered: even on the draw, the first main phase
            // (turn 2) has Mountain + Shock for the 2-life kill.
            assert!(rec.turns <= 4, "seed {seed}: lethal took too long: {rec:?}");
            assert!(
                rec.probes.spell_damage_to_players >= 1,
                "seed {seed}: {rec:?}"
            );
        }
    }

    /// At low life, the creature pilot deploys and attacks for the kill.
    #[test]
    fn stompy_attacks_for_lethal() {
        let src = CardSource::load();
        for seed in 0..5u64 {
            let rec = play_game(
                Setup {
                    decks: subset_decks(&src),
                    seed,
                    starting_life: 3,
                },
                &mut PassBot,
                &mut Stompy::default(),
            );
            assert_eq!(
                rec.outcome,
                GameOutcome::Win(deckmaste_engine::PlayerId(1)),
                "seed {seed}: {rec:?}"
            );
            // At 3 life vs a PassBot, Stompy deploys on turn 1 or 2 and attacks
            // into the open — the first creature through closes it out.
            assert!(rec.probes.attacks_declared >= 1, "seed {seed}: {rec:?}");
            assert!(
                rec.probes.creature_damage_to_players >= 1,
                "seed {seed}: {rec:?}"
            );
        }
    }
}
