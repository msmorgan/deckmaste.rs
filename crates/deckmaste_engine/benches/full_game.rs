//! Per-game performance of the engine, tracked over time as the rules grow.
//!
//! Plugins are loaded once, outside the timed loop, so the measurement is
//! engine work (build state, shuffle, draw, cast, combat, SBAs) rather than
//! disk I/O. Two cases: a single fixed-seed game (low-variance regression
//! signal) and a 64-distinct-seed average (representative throughput).
//!
//! Run with `cargo bench -p deckmaste_engine`; reports land in
//! `target/criterion/`.

use std::hint::black_box;
use std::path::Path;
use std::sync::Arc;

use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;
use deckmaste_cards::plugin::Plugin;
use deckmaste_engine::sim::DeckCards;
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

fn full_game(c: &mut Criterion) {
    let cards = matchup();
    let mut group = c.benchmark_group("full_game");

    group.throughput(Throughput::Elements(1));
    group.bench_function("seed_1", |b| {
        b.iter(|| {
            sim::play(
                black_box(&cards),
                black_box(1),
                &sim::GreedyCreatures,
                &sim::GreedyRemoval,
            )
        });
    });

    group.throughput(Throughput::Elements(64));
    group.bench_function("64_seeds", |b| {
        b.iter(|| {
            for seed in 0..64u64 {
                black_box(sim::play(
                    black_box(&cards),
                    black_box(seed),
                    &sim::GreedyCreatures,
                    &sim::GreedyRemoval,
                ));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, full_game);
criterion_main!(benches);
