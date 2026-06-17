//! End-to-end games for the Bears-vs-Bolts matchup, driven by the shared
//! self-play harness (`deckmaste_engine::sim`): a 40-card "Bears" deck
//! (23 `Grizzly Bears` + 17 `Forest`) versus a 40-card "Bolts" deck
//! (23 `Lightning Bolt` + 17 `Mountain`) — the classic
//! Grizzly-Bears-vs-Lightning-Bolt matchup, modeled with the canon plugin's
//! fakes so no card IP rides in the engine tests.
//!
//! A focused single game pins the behavior; a determinism check pins
//! reproducibility; and an `#[ignore]`d 50k-game Monte-Carlo run (parallelized
//! with rayon) reports matchup statistics and doubles as a stress test.
//! Per-game *performance* is tracked separately by `benches/full_game.rs`
//! (criterion).

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::PlayerId;
use deckmaste_engine::sim::DeckCards;
use deckmaste_engine::sim::Summary;
use deckmaste_engine::sim::{self};
use rayon::prelude::*;

/// Fixed seed for the focused/deterministic games.
const SEED: u64 = 1;

/// Loads the four card faces once (the plugins are read from disk here, never
/// on the hot path).
fn matchup() -> DeckCards {
    let canon = Plugin::load_with_sibling_prelude(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
    )
    .unwrap();
    let builtin =
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap();
    DeckCards {
        p0_spell: Arc::new(canon.card("Grizzly Bears").unwrap()),
        p0_land: Arc::new(builtin.card("Forest").unwrap()),
        p1_spell: Arc::new(canon.card("Lightning Bolt").unwrap()),
        p1_land: Arc::new(builtin.card("Mountain").unwrap()),
    }
}

#[test]
fn full_game_bears_vs_bolts_plays_to_a_winner() {
    let summary = sim::play(&matchup(), SEED, &sim::GreedyCreatures, &sim::GreedyRemoval);
    eprintln!("Bears vs Bolts (seed {SEED}): {summary:?}");

    assert!(
        matches!(summary.outcome, GameOutcome::Win(_)),
        "the game reaches a definite winner: {summary:?}"
    );
    assert!(
        summary.loser_lost_for_real,
        "the loser lost via a real loss condition (life ≤ 0 or deck-out): {summary:?}"
    );
    assert!(
        summary.creature_hit_player,
        "at least one Bear connected for combat damage to a player: {summary:?}"
    );
    assert!(
        summary.spell_killed_creature,
        "at least one Bolt killed a Bear via the lethal SBA: {summary:?}"
    );
}

#[test]
fn full_game_is_deterministic() {
    let cards = matchup();
    assert_eq!(
        sim::play(&cards, SEED, &sim::GreedyCreatures, &sim::GreedyRemoval),
        sim::play(&cards, SEED, &sim::GreedyCreatures, &sim::GreedyRemoval),
        "same seed + same strategies must reproduce the same game"
    );
}

fn pct(n: u32, total: u32) -> f64 {
    100_f64 * f64::from(n) / f64::from(total)
}

/// Shuffles up 50,000 games (seeds `0..50_000`, run in parallel with rayon) and
/// prints matchup statistics. `#[ignore]`d so it stays out of the normal suite;
/// it also serves as a 50k-game stress test — every game must terminate in a
/// real result.
///
/// Run it with:
/// `cargo test --release -p deckmaste_engine --test full_game -- --ignored
/// --nocapture`
#[test]
#[ignore = "Monte-Carlo: 50k full games — run with --release ... -- --ignored --nocapture"]
#[expect(
    clippy::cast_precision_loss,
    reason = "stats aggregation over short games; f64 has ample mantissa"
)]
fn bears_vs_bolts_50k_game_stats() {
    const GAMES: u32 = 50_000;

    let cards = matchup();
    let start = std::time::Instant::now();
    let summaries: Vec<Summary> = (0..u64::from(GAMES))
        .into_par_iter()
        .map(|seed| sim::play(&cards, seed, &sim::GreedyCreatures, &sim::GreedyRemoval))
        .collect();
    let elapsed = start.elapsed();

    let (mut p0_wins, mut p1_wins, mut draws) = (0u32, 0u32, 0u32);
    let (mut by_lethal, mut by_deckout) = (0u32, 0u32);
    let (mut spell_kills, mut creature_connects) = (0u32, 0u32);
    let mut winner_life_sum = 0i64;
    let mut turns: Vec<u32> = Vec::with_capacity(GAMES as usize);

    for s in &summaries {
        let winner = match &s.outcome {
            GameOutcome::Win(p) => Some(*p),
            GameOutcome::Draw => None,
        };
        match winner {
            Some(PlayerId(0)) => {
                p0_wins += 1;
                winner_life_sum += i64::from(s.life[0]);
            }
            Some(_) => {
                p1_wins += 1;
                winner_life_sum += i64::from(s.life[1]);
            }
            None => draws += 1,
        }
        if winner.is_some() {
            assert!(
                s.loser_lost_for_real,
                "a decisive game's loser didn't really lose: {s:?}"
            );
            if s.decked {
                by_deckout += 1;
            } else {
                by_lethal += 1;
            }
        }
        spell_kills += u32::from(s.spell_killed_creature);
        creature_connects += u32::from(s.creature_hit_player);
        turns.push(s.turns);
    }

    turns.sort_unstable();
    let decisive = p0_wins + p1_wins;
    let turn_sum: u64 = turns.iter().map(|&t| u64::from(t)).sum();
    let mean_turns = turn_sum as f64 / f64::from(GAMES);
    let winner_life_mean = winner_life_sum as f64 / f64::from(decisive.max(1));

    eprintln!("\n=== Bears vs Bolts — {GAMES} shuffled games (parallel) ===");
    eprintln!(
        "Wins  P0 Bears : {p0_wins:6}  ({:.1}%)",
        pct(p0_wins, GAMES)
    );
    eprintln!(
        "      P1 Bolts : {p1_wins:6}  ({:.1}%)",
        pct(p1_wins, GAMES)
    );
    eprintln!("      draws    : {draws:6}  ({:.1}%)", pct(draws, GAMES));
    eprintln!(
        "End   life <= 0: {by_lethal:6}  ({:.1}%)",
        pct(by_lethal, GAMES)
    );
    eprintln!(
        "      deck-out : {by_deckout:6}  ({:.1}%)",
        pct(by_deckout, GAMES)
    );
    eprintln!(
        "Turns mean {mean_turns:.1}  median {}  p90 {}  min {}  max {}",
        turns[turns.len() / 2],
        turns[turns.len() * 9 / 10],
        turns[0],
        turns.last().unwrap()
    );
    eprintln!("Winner avg remaining life: {winner_life_mean:.1}");
    eprintln!(
        "Sanity  Bolt killed a Bear in {:.1}% of games;  a Bear connected in {:.1}%",
        pct(spell_kills, GAMES),
        pct(creature_connects, GAMES)
    );
    eprintln!(
        "Ran {GAMES} games in {elapsed:.2?}  ({:.0} games/sec, all cores)\n",
        f64::from(GAMES) / elapsed.as_secs_f64()
    );

    assert_eq!(
        p0_wins + p1_wins + draws,
        GAMES,
        "every one of the {GAMES} games produced a result"
    );
}
