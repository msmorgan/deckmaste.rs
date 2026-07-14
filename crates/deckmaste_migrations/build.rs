// Detects the Scryfall catalog dump (`data/catalogs`) at build time and, when
// it is present, sets the `scryfall_catalogs` cfg. The parser tests that mint
// `Subtype`/keyword atoms load those catalogs (creature-types,
// keyword-abilities, …) through `crate::data`; without them `SUBTYPES` and the
// keyword catalog come up empty and subtype/keyword parses decline. Those tests
// carry `#[cfg_attr(not(scryfall_catalogs), ignore = "…")]`, so they run
// locally (where `data/catalogs` is populated) and report as `ignored` on a
// bare checkout — CI — which cannot materialize the gitignored `data/` dump.
//
// Detection is by directory presence. With no `rerun-if-changed` emitted, cargo
// re-runs this script whenever a package file changes, so ordinary edits pick
// up a freshly-fetched catalog dir; a fetch with no other change needs `touch
// build.rs`.
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(scryfall_catalogs)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    if Path::new(&manifest).join("../../data/catalogs").is_dir() {
        println!("cargo::rustc-cfg=scryfall_catalogs");
    }
}
