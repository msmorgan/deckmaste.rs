//! The corpus dry-run's archived hand-audit sample ([[cards-corpus-dry-run]]):
//! `tests/r2_audit/sample.txt` names 200 anaphor-reading faces (strata
//! documented in the file), and `tests/r2_audit/resolutions.txt` pins the
//! resolution table (which antecedent every reference bound to) each of them
//! elaborated to when the R2 ambiguity gate was calibrated and FROZEN STRICT.
//!
//! A failure here means a table/macro/card change MOVED a pinned binding —
//! that is a reviewed event: re-audit the affected faces against their oracle
//! text, then re-bless with `R2_AUDIT_BLESS=1 cargo test -p deckmaste_cards
//! --test r2_audit`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::elaborate;
use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;

fn plugins_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins")
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/r2_audit")
        .join(name)
}

/// Renders the sample's resolution tables — the exact `--dump` view: every
/// resolved binding in walk order, then any elaboration errors — and
/// compares them against the archived audit fixture.
#[test]
fn audit_sample_resolutions_are_pinned() {
    let root = plugins_root();
    let sample = std::fs::read_to_string(fixture("sample.txt")).unwrap();

    // One load per plugin (wizards is the expensive one), shared across its
    // sampled faces.
    let mut plugins: BTreeMap<String, Plugin> = BTreeMap::new();

    let mut rendered = String::new();
    for line in sample.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (plugin_name, _) = line
            .split_once('/')
            .unwrap_or_else(|| panic!("sample line without a plugin prefix: {line:?}"));
        let plugin = plugins
            .entry(plugin_name.to_owned())
            .or_insert_with(|| Plugin::load_with_sibling_prelude(root.join(plugin_name)).unwrap());
        let path = root.join(line);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("reading {}: {e}", path.display()));
        let card: Card = plugin
            .macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("parsing {}: {e}", path.display()));
        let registries = plugin.registries();
        let (result, resolutions) = elaborate::elaborate_with_resolutions(&card, &registries);

        writeln!(rendered, "== {line}").unwrap();
        for resolution in &resolutions {
            writeln!(rendered, "  {resolution}").unwrap();
        }
        if let Err(errors) = result {
            for error in &errors {
                writeln!(rendered, "  !! {error}").unwrap();
            }
        }
    }

    let golden_path = fixture("resolutions.txt");
    if std::env::var_os("R2_AUDIT_BLESS").is_some() {
        std::fs::write(&golden_path, &rendered).unwrap();
        return;
    }
    let golden = std::fs::read_to_string(&golden_path)
        .expect("tests/r2_audit/resolutions.txt missing — bless with R2_AUDIT_BLESS=1");
    if rendered != golden {
        let divergence = rendered
            .lines()
            .zip(golden.lines())
            .position(|(a, b)| a != b)
            .map_or_else(
                || "one side has extra trailing lines".to_owned(),
                |i| {
                    format!(
                        "first divergence at line {}:\n  computed: {}\n  pinned:   {}",
                        i + 1,
                        rendered.lines().nth(i).unwrap_or(""),
                        golden.lines().nth(i).unwrap_or(""),
                    )
                },
            );
        panic!(
            "R2 audit resolutions drifted from the archived sample — a table/macro/card \
             change moved a pinned binding. Re-audit the affected faces, then re-bless \
             with R2_AUDIT_BLESS=1. {divergence}"
        );
    }
}
