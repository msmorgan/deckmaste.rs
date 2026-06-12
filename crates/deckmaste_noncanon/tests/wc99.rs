//! The WC99 matchup suite. Gated: run with
//! `cargo test -p deckmaste_noncanon --features noncanon_tests`.
//! `GAMES=n` overrides the batch size.
#![cfg(feature = "noncanon_tests")]

use std::sync::Arc;

use deckmaste_core::Card;
use deckmaste_engine::GameOutcome;
use deckmaste_noncanon::deck;
use deckmaste_noncanon::game::GameRecord;
use deckmaste_noncanon::game::Setup;
use deckmaste_noncanon::game::play_game;
use deckmaste_noncanon::pilots::sped_red::SpedRed;
use deckmaste_noncanon::pilots::stompy::Stompy;
use deckmaste_noncanon::probe::Probes;
use deckmaste_noncanon::source::CardSource;
use deckmaste_noncanon::wc99;

/// Default 50: lookahead-piloted games run ~2.4s each on subset decks and
/// ~5s on the full proxied lists (dev profile) — the two green gates total
/// roughly six minutes. Override with `GAMES=n` for quick iterations.
fn batch_size() -> u64 {
    std::env::var("GAMES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(50)
}

fn run_batch(decks: &[Vec<Arc<Card>>; 2]) -> Vec<GameRecord> {
    (0..batch_size())
        .map(|seed| {
            let rec = play_game(
                Setup {
                    decks: [decks[0].clone(), decks[1].clone()],
                    seed,
                    starting_life: 20,
                },
                &mut SpedRed::default(),
                &mut Stompy::default(),
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

/// The green gate: the supported-subset matchup completes, exercises its
/// mechanics, and produces non-degenerate outcomes across a seed batch.
#[test]
fn wc99_subset_gate() {
    let src = CardSource::load();
    let decks = [
        deck::build_subset(&wc99::SPED_RED, wc99::SPED_RED_ALLOWLIST, &src),
        deck::build_subset(&wc99::STOMPY, wc99::STOMPY_ALLOWLIST, &src),
    ];
    let records = run_batch(&decks);
    let (red, green, probes) = report("wc99 subset", &records);

    // Mechanic probes — wave 0. Extend this block with every allowlist wave.
    assert!(probes.lands_played > 0);
    assert!(probes.spells_cast > 0);
    assert!(probes.attacks_declared > 0);
    assert!(
        probes.creature_damage_to_players > 0,
        "creatures never connected"
    );
    assert!(
        probes.spell_damage_to_players > 0,
        "burn never went to the face"
    );
    // NOT asserted: spell_damage_to_creatures. In the all-Shocks-vs-all-Elves
    // subset, face is eval-optimal (2 life dwarfs a 1/1's board value), so a
    // well-piloted SpedRed only shoots a creature to stop lethal — rare
    // enough to make the assertion flaky. It returns to the probe set once
    // creature-targeting is structurally correct (Jackal Pup, Fireslinger,
    // River Boa waves).
    assert!(probes.nonland_taps > 0, "Llanowar Elves never made mana");
    // NOT asserted: abilities_activated / battlefield_to_graveyard — the
    // first general activation (Mogg Fanatic's sac) is blocked on engine
    // verb-cost support (`cost_summary` rejects `Do(...)` costs), and
    // nothing dies under optimal play while red can't block and won't
    // shoot 1/1s. Both assertions arm with that engine wave.

    // Win-rate sanity: real shuffled games, neither seat degenerate.
    assert!(red > 0, "Sped Red never won");
    assert!(green > 0, "Stompy never won");
}

/// The full 60s with Unsupported-proxied abilities: every card loads, casts,
/// and fights as a vanilla version of itself; abilities arrive incrementally
/// by replacing their `Unsupported(...)` wrappers. This is the green gate for
/// the REAL deck shapes (curves, bodies, burn density).
#[test]
fn wc99_full_proxied_gate() {
    let src = CardSource::load();
    let decks = [
        deck::build_full(&wc99::SPED_RED, &src),
        deck::build_full(&wc99::STOMPY, &src),
    ];
    let records = run_batch(&decks);
    let (red, green, probes) = report("wc99 full-proxied", &records);

    // The full shapes bring combat both ways: red has bodies now.
    assert!(probes.spells_cast > 0);
    assert!(probes.attacks_declared > 0);
    assert!(probes.creature_damage_to_players > 0);
    assert!(
        probes.spell_damage_to_players > 0,
        "burn never went to the face"
    );
    // Win-rate sanity only — proxy decks aren't the faithful matchup yet.
    assert!(red > 0, "Sped Red never won");
    assert!(green > 0, "Stompy never won");
}

/// The red target: the faithful matchup. Stays ignored until no
/// `Unsupported(...)` wrapper remains in either list AND the win-rate band
/// below is tuned against an observed distribution.
#[test]
#[ignore = "faithful WC99 matchup: ungate when no Unsupported abilities remain in either deck"]
fn wc99_full_matchup() {
    let src = CardSource::load();
    let decks = [
        deck::build_full(&wc99::SPED_RED, &src),
        deck::build_full(&wc99::STOMPY, &src),
    ];
    let records = run_batch(&decks);
    let (red, green, probes) = report("wc99 full", &records);

    // Full probe set grows with graduation waves (echo paid AND declined,
    // regeneration shielded, Rancor returned, manland animated and attacked,
    // Wasteland traded, Cursed Scroll activated, ...). Wave-0 floor:
    assert!(probes.spells_cast > 0);
    assert!(
        red > 0 && green > 0,
        "degenerate full matchup: {red}/{green}"
    );
    // Historical-expectation band — TIGHTEN when un-ignoring, after observing
    // a real distribution (placeholder: neither seat below 20% over a batch).
    let total = red + green;
    assert!(
        red * 5 >= total && green * 5 >= total,
        "lopsided: {red}/{green}"
    );
}
