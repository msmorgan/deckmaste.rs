// Detects the Scryfall catalog dump (`data/catalogs`) AND the CR rules
// snapshot (`data/rules`) at build time and, when both are present, sets the
// `scryfall_catalogs` cfg. The parser tests that mint `Subtype`/keyword atoms
// load the catalogs (creature-types, keyword-abilities, …) through
// `deckmaste_data`, and the keyword-ability parser additionally reads
// `data/rules/keywords.json` (`KEYWORD_NAMES`) — with either dir missing the
// catalogs come up empty and those parses decline, so the cfg requires BOTH
// (a catalogs-only checkout used to compile the cfg on and fail the gated
// tests). Those tests carry `#[cfg_attr(not(scryfall_catalogs), ignore =
// "…")]`, so they run locally (where `data/` is populated) and on CI (which
// fetches and caches the upstream inputs before building) and report as
// `ignored` on a bare checkout.
//
// Detection is by directory presence. The script declares both directories
// (or, while one is absent, its nearest existing ancestor under `data/`), so a
// fetch re-runs it. With no `data/` at all it declares nothing, and cargo
// re-runs it whenever a package file changes (CI also runs `touch build.rs`
// after staging).
use std::path::{Path, PathBuf};

const DATA: &str = "../../data";

fn main() {
    println!("cargo::rustc-check-cfg=cfg(scryfall_catalogs)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    let manifest = Path::new(&manifest);
    declare_inputs(manifest, &["../../data/catalogs", "../../data/rules"]);
    let data = manifest.join(DATA);
    if data.join("catalogs").is_dir() && data.join("rules").is_dir() {
        println!("cargo::rustc-cfg=scryfall_catalogs");
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
