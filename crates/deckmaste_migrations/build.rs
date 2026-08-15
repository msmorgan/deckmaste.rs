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
// Detection is by directory presence. With no `rerun-if-changed` emitted, cargo
// re-runs this script whenever a package file changes, so ordinary edits pick
// up a freshly-fetched catalog dir; a fetch with no other change needs `touch
// build.rs` (CI does exactly that after staging).
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(scryfall_catalogs)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    let data = Path::new(&manifest).join("../../data");
    if data.join("catalogs").is_dir() && data.join("rules").is_dir() {
        println!("cargo::rustc-cfg=scryfall_catalogs");
    }
}
