// Detects the generated `plugins/wizards` corpus at build time and, when it is
// present, sets the `wizards_corpus` cfg. Tests that need the corpus carry
// `#[cfg_attr(not(wizards_corpus), ignore = "…")]`, so they run locally (where
// `cargo xtask generate plugins/wizards` has populated it) and on CI (which
// stages the data mirror and generates the corpus before building) and report
// as `ignored` on a bare checkout (the corpus is gitignored, built from the
// ~150 MB mtgjson dump under `data/`).
//
// Detection is by directory presence. With no `rerun-if-changed` emitted, cargo
// re-runs this script whenever a package file changes, so ordinary edits pick
// up a freshly-generated corpus; a regen with no other change needs `touch
// build.rs` (CI does exactly that after generating).
//
// KEEP IN SYNC: byte-identical copies of this file live in deckmaste_plugin,
// deckmaste_engine, deckmaste_tui, and deckmaste_noncanon — cfgs don't cross
// crate boundaries, so each crate needs its own copy.
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(wizards_corpus)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    if Path::new(&manifest).join("../../plugins/wizards").is_dir() {
        println!("cargo::rustc-cfg=wizards_corpus");
    }
}
