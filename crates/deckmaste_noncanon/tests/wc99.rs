//! The WC99 matchup suite. Gated: run with
//! `cargo test -p deckmaste_noncanon --features noncanon_tests`.
//! `GAMES=n` overrides the batch size.
#![cfg(feature = "noncanon_tests")]

use std::sync::Arc;

use deckmaste_core::Card;
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

/// Default 50: lookahead-piloted games run ~2.4s each on subset decks and
/// ~5s on the full proxied lists (dev profile) — the two green gates total
/// roughly six minutes. Override with `GAMES=n` for quick iterations.
fn batch_size() -> u64 {
    std::env::var("GAMES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(50)
}

fn run_batch(decks: &[Vec<Arc<Card>>; 2], rules: &EngineRules) -> Vec<GameRecord> {
    let p0 = MatchupStrategy::sped_red(PlayerId(0));
    let p1 = MatchupStrategy::stompy(PlayerId(1));
    (0..batch_size())
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

/// Phase 1 (greedy Strategy seats) subset smoke: the supported-subset matchup
/// completes and its core non-combat mechanics fire. The
/// all-Shocks-vs-all-Elves subset is a one-sided burn race under greedy play —
/// SpedRed burns the face out (~turn 16) before Stompy's mana-tapped Elves ever
/// attack — so combat and balanced outcomes are LOOKAHEAD-DEPENDENT and parked
/// for Phase 2.
#[test]
fn wc99_subset_gate() {
    let src = CardSource::load();
    let decks = [
        deck::build_subset(&wc99::SPED_RED, wc99::SPED_RED_ALLOWLIST, &src),
        deck::build_subset(&wc99::STOMPY, wc99::STOMPY_ALLOWLIST, &src),
    ];
    let records = run_batch(&decks, &src.engine_rules());
    let (_red, _green, probes) = report("wc99 subset", &records);

    // Mechanics greedy play exercises reliably in this subset.
    assert!(probes.lands_played > 0);
    assert!(probes.spells_cast > 0);
    assert!(
        probes.spell_damage_to_players > 0,
        "burn never went to the face"
    );
    assert!(probes.nonland_taps > 0, "Llanowar Elves never made mana");
    // Phase 2 (lookahead decorator) re-arms the lookahead-dependent asserts:
    // `attacks_declared` / `creature_damage_to_players` (greedy Stompy taps its
    // Elves out to flood, so none survive to attack) and the `red > 0 &&
    // green > 0` balance (greedy makes the subset a one-sided burn race).
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
    let records = run_batch(&decks, &src.engine_rules());
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
    let records = run_batch(&decks, &src.engine_rules());
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

fn subset_decks(src: &CardSource) -> [Vec<Arc<Card>>; 2] {
    [
        deck::build_subset(&wc99::SPED_RED, wc99::SPED_RED_ALLOWLIST, src),
        deck::build_subset(&wc99::STOMPY, wc99::STOMPY_ALLOWLIST, src),
    ]
}

/// At 2 life the burn seat should find the Shock-the-face lethal and never
/// durdle to a deck-out. Greedy play does not reliably plan lethal, so this is
/// parked until the Phase 2 lookahead decorator returns.
#[test]
#[ignore = "Phase 2: re-arm when the lookahead Strategy decorator returns"]
fn sped_red_finds_burn_lethal() {
    let src = CardSource::load();
    let p0 = MatchupStrategy::sped_red(PlayerId(0));
    let p1 = MatchupStrategy::pass_bot(PlayerId(1));
    for seed in 0..5u64 {
        let rec = play_game(
            Setup {
                decks: subset_decks(&src),
                seed,
                starting_life: 2,
                rules: src.engine_rules(),
            },
            &p0,
            &p1,
        );
        assert_eq!(
            rec.outcome,
            GameOutcome::Win(PlayerId(0)),
            "seed {seed}: {rec:?}"
        );
        assert!(rec.turns <= 4, "seed {seed}: lethal took too long: {rec:?}");
        assert!(
            rec.probes.spell_damage_to_players >= 1,
            "seed {seed}: {rec:?}"
        );
    }
}

/// At low life the creature seat should deploy and attack for the kill. Parked
/// until the Phase 2 lookahead decorator returns (greedy attack-all may trade
/// rather than race).
#[test]
#[ignore = "Phase 2: re-arm when the lookahead Strategy decorator returns"]
fn stompy_attacks_for_lethal() {
    let src = CardSource::load();
    let p0 = MatchupStrategy::pass_bot(PlayerId(0));
    let p1 = MatchupStrategy::stompy(PlayerId(1));
    for seed in 0..5u64 {
        let rec = play_game(
            Setup {
                decks: subset_decks(&src),
                seed,
                starting_life: 3,
                rules: src.engine_rules(),
            },
            &p0,
            &p1,
        );
        assert_eq!(
            rec.outcome,
            GameOutcome::Win(PlayerId(1)),
            "seed {seed}: {rec:?}"
        );
    }
}
