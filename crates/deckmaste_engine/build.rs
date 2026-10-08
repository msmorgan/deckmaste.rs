// Detects the generated `plugins/wizards` corpus at build time and, when it is
// present, sets the `wizards_corpus` cfg. Tests that need the corpus carry
// `#[cfg_attr(not(wizards_corpus), ignore = "…")]`, so they run locally (where
// `cargo xtask generate plugins/wizards` has populated it) and on CI (which
// fetches upstream inputs and generates the corpus before building) and report
// as `ignored` on a bare checkout (the corpus is gitignored, built from the
// ~150 MB mtgjson dump under `data/`).
//
// Detection is by directory presence, and the declared inputs follow it. While
// the corpus is absent the script watches `plugins/` (cargo scans a directory
// recursively), so generating the corpus re-runs it. While the corpus is
// present it watches only `build.rs`: the corpus is ~31k files that every new
// workspace regenerates, and watching them would rebuild this crate in each
// one although only presence matters. Removing the corpus therefore needs
// `touch build.rs`.
//
// KEEP IN SYNC: byte-identical copies of this file live in deckmaste_plugin,
// deckmaste_engine, deckmaste_tui, and deckmaste_noncanon — cfgs don't cross
// crate boundaries, so each crate needs its own copy.
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(wizards_corpus)");
    println!("cargo::rerun-if-changed=build.rs");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    // Relative paths: cargo resolves them against the package root, so a
    // target/ copied into another workspace watches that workspace's files.
    if Path::new(&manifest).join("../../plugins/wizards").is_dir() {
        println!("cargo::rustc-cfg=wizards_corpus");
    } else {
        println!("cargo::rerun-if-changed=../../plugins");
    }
}
