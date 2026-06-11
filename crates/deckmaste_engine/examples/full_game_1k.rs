//! Trim profiling runner: 1,000 Bears-vs-Bolts games (override with `GAMES=n`),
//! single core, no rayon. A scaled-down sibling of the `#[ignore]`d 50k
//! Monte-Carlo report in `tests/full_game.rs`, meant to be run under
//! `perf record`.
//!
//! ```sh
//! RUSTFLAGS=-Cforce-frame-pointers=yes CARGO_PROFILE_RELEASE_DEBUG=true \
//!     cargo build --release -p deckmaste_engine --example full_game_1k
//! perf record -g -F 997 target/release/examples/full_game_1k
//! ```

use std::path::Path;
use std::sync::Arc;

use deckmaste_cards::plugin::Plugin;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::sim::DeckCards;
use deckmaste_engine::sim::Summary;
use deckmaste_engine::sim::{self};

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

#[expect(
    clippy::cast_precision_loss,
    reason = "timing stats over small counts; f64 has ample mantissa"
)]
fn main() {
    let games: u64 = std::env::var("GAMES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1_000);

    let cards = matchup();
    let start = std::time::Instant::now();
    let summaries: Vec<Summary> = (0..games)
        .map(|seed| sim::play(&cards, seed, &sim::GreedyCreatures, &sim::GreedyRemoval))
        .collect();
    let elapsed = start.elapsed();

    let decisive = summaries
        .iter()
        .filter(|s| matches!(s.outcome, GameOutcome::Win(_)))
        .count();
    let turns: u64 = summaries.iter().map(|s| u64::from(s.turns)).sum();
    eprintln!(
        "{games} games in {elapsed:.2?} single-core ({:.0} games/sec, {:.3} ms/game); \
         {decisive} decisive, {turns} total turns",
        games as f64 / elapsed.as_secs_f64(),
        elapsed.as_secs_f64() * 1e3 / games as f64,
    );
}
