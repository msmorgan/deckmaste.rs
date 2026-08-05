//! The Sped-Red-vs-Stompy Worlds 1999 matchup suite. The fixed-seed gate runs
//! with `cargo test -p deckmaste_noncanon --features noncanon_tests --test
//! wc99`. The ignored 50-game matchup is opt-in; `GAMES=n` overrides its batch
//! size.
#![cfg(feature = "noncanon_tests")]

use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::PlayerId;
use deckmaste_noncanon::deck;
use deckmaste_noncanon::game::GameRecord;
use deckmaste_noncanon::game::Setup;
use deckmaste_noncanon::game::play_game;
use deckmaste_noncanon::probe::Probes;
use deckmaste_noncanon::source::CardSource;
use deckmaste_noncanon::source::EngineRules;
use deckmaste_noncanon::strategy::MatchupStrategy;
use deckmaste_noncanon::wc99;

/// Default 50 full-list games. `GAMES=n` keeps local iteration bounded.
fn batch_size() -> u64 {
    std::env::var("GAMES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(50)
}

fn run_batch(
    decks: &[Vec<Arc<Card>>; 2],
    rules: &EngineRules,
    seeds: impl IntoIterator<Item = u64>,
) -> Vec<GameRecord> {
    let p0 = MatchupStrategy::sped_red(PlayerId(0));
    let p1 = MatchupStrategy::stompy(PlayerId(1));
    seeds
        .into_iter()
        .map(|seed| {
            let rec = play_game(
                Setup {
                    decks: [decks[0].clone(), decks[1].clone()],
                    seed,
                    starting_life: 20,
                    rules: rules.clone(),
                },
                &p0,
                &p1,
            );
            // Per-game invariants.
            assert!(rec.turns < 200, "seed {seed}: {rec:?}");
            if matches!(rec.outcome, GameOutcome::Win(_)) {
                assert!(rec.loser_lost_for_real, "seed {seed}: zombie loser {rec:?}");
            }
            rec
        })
        .collect()
}

#[expect(
    clippy::cast_precision_loss,
    reason = "average over small batch counts; f64 has ample mantissa"
)]
fn report(label: &str, records: &[GameRecord]) -> (u64, u64, Probes) {
    let mut wins = [0u64; 2];
    let mut decked = 0u64;
    let mut probes = Probes::default();
    let mut turns = 0u64;
    for r in records {
        if let GameOutcome::Win(w) = r.outcome {
            wins[w.index()] += 1;
        }
        decked += u64::from(r.decked);
        probes.merge(&r.probes);
        turns += u64::from(r.turns);
    }
    eprintln!(
        "{label}: {} games — red {} / green {} wins, {} decked, avg {:.1} turns\n  {probes:?}",
        records.len(),
        wins[0],
        wins[1],
        decked,
        turns as f64 / records.len() as f64,
    );
    (wins[0], wins[1], probes)
}

fn load_matchup() -> (CardSource, [Vec<Arc<Card>>; 2]) {
    let src = CardSource::load();
    let decks = [
        deck::build_full(&wc99::SPED_RED, &src),
        deck::build_full(&wc99::STOMPY, &src),
    ];
    (src, decks)
}

fn assert_matchup_activity(records: &[GameRecord], red: u64, green: u64, probes: &Probes) {
    assert!(probes.spells_cast > 0);
    assert!(probes.attacks_declared > 0);
    assert!(probes.creature_damage_to_players > 0);
    assert!(
        probes.spell_damage_to_players > 0,
        "burn never went to the face"
    );
    assert_eq!(red + green, records.len() as u64, "every game has a winner");
}

/// A deterministic, bounded slice of the exact 60-card Worlds 1999 matchup.
/// Seed 0 exercises both combat and RON-driven burn targeting.
#[test]
#[cfg_attr(not(wizards_corpus), ignore = "needs generated plugins/wizards corpus")]
fn fixed_seed_gate() {
    let (src, decks) = load_matchup();
    let records = run_batch(&decks, &src.engine_rules(), [0]);
    let (red, green, probes) = report("sped red vs stompy, Worlds 1999", &records);
    assert_matchup_activity(&records, red, green, &probes);
}

/// The intentionally expensive matchup batch. Run explicitly with
///
/// ```sh
/// cargo test -p deckmaste_noncanon --features noncanon_tests --test wc99 \
///   historical_full_matchup -- --ignored --nocapture
/// ```
#[test]
#[ignore = "slow 50-game matchup; run explicitly with --ignored"]
fn historical_full_matchup() {
    let (src, decks) = load_matchup();
    let records = run_batch(&decks, &src.engine_rules(), 0..batch_size());
    let (red, green, probes) = report("sped red vs stompy, Worlds 1999", &records);
    assert_matchup_activity(&records, red, green, &probes);

    if records.len() > 1 {
        assert!(red > 0, "Sped Red never won");
        assert!(green > 0, "Stompy never won");
    }
}
