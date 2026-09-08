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
// Detection is by path presence. With no `rerun-if-changed` emitted, cargo
// re-runs this script whenever a package file changes, so ordinary edits pick
// up freshly-generated fixtures; a regen with no other change needs `touch
// build.rs` (CI does exactly that after staging).
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gen_catalogs)");
    println!("cargo::rustc-check-cfg=cfg(derived_cards)");
    let manifest = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by cargo");
    let data = Path::new(&manifest).join("../../data");
    if data.join("gen/catalogs-legacy").is_dir() {
        println!("cargo::rustc-cfg=gen_catalogs");
    }
    if data.join("derived/cards.jsonl").is_file() {
        println!("cargo::rustc-cfg=derived_cards");
    }
}
