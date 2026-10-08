// Detects the two local-only `data/` fixtures xtask's english/macros tests
// parse real oracle text against, and sets a cfg per fixture:
//
// - `gen_catalogs`  — `data/gen/catalogs-legacy`, the CR-derived bare-text
//   catalogs `cargo xtask catalogs text` writes from `data/rules/cr.txt`.
// - `derived_cards` — `data/derived/cards.jsonl`, the Scryfall-derived Oracle
//   snapshot (`deckmaste_plugin::fidelity::ORACLE_SNAPSHOT`).
//
// Neither exists in a bare checkout: the whole `data/` tree is gitignored by
// the no-committed-corpus policy. CI fetches and caches the upstream inputs,
// derives both fixtures, then re-runs build scripts before the test gate. In an
// unstaged checkout, tests that need them carry
// `#[cfg_attr(not(<cfg>), ignore = "…")]` and report as `ignored`.
// `OracleDataArgs::load` reads cards.jsonl AND the catalogs, so its callers
// carry BOTH attributes.
//
// Detection is by path presence. The script declares each fixture path (or,
// while one is absent, its nearest existing ancestor under `data/`), so a
// fixture appearing, disappearing or being regenerated re-runs it. With no
// `data/` at all it declares nothing, and cargo re-runs it whenever a package
// file changes (CI also runs `touch build.rs` after staging).
use std::path::{Path, PathBuf};

const DATA: &str = "../../data";

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gen_catalogs)");
    println!("cargo::rustc-check-cfg=cfg(derived_cards)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    let manifest = Path::new(&manifest);
    declare_inputs(
        manifest,
        &[
            "../../data/gen/catalogs-legacy",
            "../../data/derived/cards.jsonl",
        ],
    );
    let data = manifest.join(DATA);
    if data.join("gen/catalogs-legacy").is_dir() {
        println!("cargo::rustc-cfg=gen_catalogs");
    }
    if data.join("derived/cards.jsonl").is_file() {
        println!("cargo::rustc-cfg=derived_cards");
    }
}

/// The path to declare for `input` (relative to the package root, so a
/// `target/` copied into another workspace watches that workspace's files): the
/// input itself when present, otherwise its nearest existing ancestor below the
/// `data/` root, so the input's appearance re-runs this script. `None` when
/// `data/` itself is absent.
fn watched(manifest: &Path, input: &str) -> Option<PathBuf> {
    let mut rel = PathBuf::from(input);
    loop {
        if manifest.join(&rel).exists() {
            return Some(rel);
        }
        if rel == Path::new(DATA) || !rel.pop() {
            return None;
        }
    }
}

/// Declares `build.rs` and every input, or nothing when `data/` is absent:
/// cargo then falls back to re-running the script whenever a package file
/// changes, which is the only way to notice `data/` appearing later.
fn declare_inputs(manifest: &Path, inputs: &[&str]) {
    let watched: Option<Vec<PathBuf>> = inputs.iter().map(|i| watched(manifest, i)).collect();
    if let Some(paths) = watched {
        println!("cargo::rerun-if-changed=build.rs");
        for path in paths {
            println!("cargo::rerun-if-changed={}", path.display());
        }
    }
}
