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
// Detection is by directory presence. With no `rerun-if-changed` emitted, cargo
// re-runs this script whenever a package file changes, so ordinary edits pick
// up freshly-generated catalogs; a regen with no other change needs `touch
// build.rs` (CI does exactly that after staging).
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gen_catalogs)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    if Path::new(&manifest)
        .join("../../data/gen/catalogs-legacy")
        .is_dir()
    {
        println!("cargo::rustc-cfg=gen_catalogs");
    }
}
