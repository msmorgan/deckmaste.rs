// Detects the generated, CR-derived legacy catalogs
// (`data/gen/catalogs-legacy`) at build time and, when present, sets the
// `gen_catalogs` cfg. The unify/render fixtures parse real oracle text, so they
// load those catalogs through this crate's `real_catalogs()` test helpers —
// `Catalogs::default()` has zero entries, so a `KeywordLine` frame cannot parse
// against it at all and the fixture is load-bearing, not incidental. Those
// tests carry `#[cfg_attr(not(gen_catalogs), ignore = "…")]`, so they run
// wherever `cargo xtask catalogs text` has been run (locally, from the
// `data/rules/cr.txt`
// + `data/scryfall/oracle-cards.jsonl` snapshots) and report as `ignored` on a
// checkout without them — the whole `data/` tree is gitignored. CI fetches and
// caches the CR snapshot, derives these catalogs, then re-runs build scripts.
//
// Detection is by directory presence. The script declares the directory (or,
// while it is absent, its nearest existing ancestor under `data/`), so a regen
// re-runs it. With no `data/` at all it declares nothing, and cargo re-runs it
// whenever a package file changes (CI also runs `touch build.rs` after
// staging).
use std::path::{Path, PathBuf};

const DATA: &str = "../../data";

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gen_catalogs)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    let manifest = Path::new(&manifest);
    declare_inputs(manifest, &["../../data/gen/catalogs-legacy"]);
    if manifest.join(DATA).join("gen/catalogs-legacy").is_dir() {
        println!("cargo::rustc-cfg=gen_catalogs");
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
