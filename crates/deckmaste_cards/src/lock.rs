//! `cards.elab.lock`: a per-card content hash of the elaborated IR for the
//! hand-authored plugins ([[cards-elab-load-gate]]) — the elaborator's analog
//! of `cr-citations.lock`. `cargo xtask elaborate --lock` (re)computes and
//! writes it; CI's "clean" is a fresh `--lock` run producing no diff (this
//! ticket's idempotency requirement) — there is no separate `--check`: the
//! lockfile IS the check, exactly like `cr-citations.lock`'s own committed
//! artifact.
//!
//! The hash is over each card's fully macro-EXPANDED form
//! ([`deckmaste_core::Expand::expand_all`], what the engine evaluates) PLUS
//! its computed anaphor resolutions
//! ([`crate::elaborate::elaborate_with_resolutions`] — which antecedent each
//! `It`/`That(Sort)`/`They`/`ThatMany`/`The(label)` bound to): the wire
//! serializes surface anaphors (the anaphor-surface wire ruling), so the
//! lock is what pins the COMPUTED indices — a macro or table change that
//! re-points a reference changes the resolution trace even when the
//! authored file doesn't, and that's exactly the drift this file exists to
//! catch loudly. A `Stage::Deny` plugin's cards are always
//! elaboration-CLEAN by the time `--lock` can run (an unclean one already
//! failed `Plugin::load`), so the interesting content is the resolved
//! VALUE, not pass/fail.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::Context;
use deckmaste_core::Card;
use deckmaste_core::Expand;
use deckmaste_core::plugin::CARDS_DIR;
use deckmaste_core::plugin::is_todo_source;
use sha2::Digest;
use sha2::Sha256;

use crate::plugin::Plugin;
use crate::plugin::read;
use crate::plugin::ron_files_recursive;

/// The hand-authored plugins the lock covers, in the fixed order `compute`
/// walks them. `wizards` (generated, `Stage::Warn`) is deliberately excluded
/// — see the module doc.
pub const HAND_AUTHORED: [&str; 4] = ["builtin", "canon", "testing", "demo"];

/// The committed lockfile's path, relative to the workspace root.
pub const FILE_NAME: &str = "cards.elab.lock";

/// One entry's stable key: `"<plugin>/<card name>"` (disambiguates
/// same-named cards across plugins; canon and wizards both finish some of
/// builtin's names).
#[must_use]
pub fn key(plugin: &str, card_name: &str) -> String {
    format!("{plugin}/{card_name}")
}

/// The 16-hex-char truncated SHA-256 of `value` — the same truncation
/// `cr-citations.lock` uses, so both lockfiles read the same at a glance.
fn hash(value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    digest.iter().take(8).fold(String::new(), |mut out, byte| {
        let _ = write!(out, "{byte:02x}");
        out
    })
}

/// Every hand-authored plugin's finished (non-todo) cards, hashed by their
/// expanded form, keyed `"<plugin>/<name>"`.
///
/// # Errors
/// If a plugin fails to load (including failing its own load-time
/// elaboration gate — see `Plugin::load`), or a card file fails to read or
/// parse.
pub fn compute(workspace_root: &Path) -> anyhow::Result<BTreeMap<String, String>> {
    let mut checksums = BTreeMap::new();
    for &plugin_name in &HAND_AUTHORED {
        let plugin_dir = workspace_root.join("plugins").join(plugin_name);
        let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)
            .with_context(|| format!("loading plugins/{plugin_name}"))?;
        for path in ron_files_recursive(&plugin_dir.join(CARDS_DIR))? {
            let source = read(&path)?;
            if is_todo_source(&source) {
                continue;
            }
            let card: Card = plugin
                .macros
                .read_str(&source)
                .with_context(|| format!(r#"parsing "{}""#, path.display()))?;
            let name = crate::plugin::card_lookup_name(&card).to_owned();
            let expanded = deckmaste_core::ron::options()
                .to_string(&card.clone().expand_all())
                .with_context(|| format!(r#"serializing "{}""#, path.display()))?;
            // The computed resolutions join the hash: the wire keeps the
            // SURFACE anaphors, so the lock is what pins where each one
            // bound — a re-pointing table/macro change drifts the trace
            // even when the authored spelling didn't change.
            let registries = plugin.registries();
            let (_, resolutions) = crate::elaborate::elaborate_with_resolutions(&card, &registries);
            let mut content = expanded;
            for resolution in &resolutions {
                content.push('\n');
                let _ = write!(content, "{resolution}");
            }
            checksums.insert(key(plugin_name, &name), hash(&content));
        }
    }
    Ok(checksums)
}

/// Renders `checksums` as `cards.elab.lock`'s RON body — a hand-written
/// template (not a serde mirror), sorted by key for a deterministic,
/// reviewable diff.
#[must_use]
pub fn render(checksums: &BTreeMap<String, String>) -> String {
    let mut out = String::from(
        "// Generated by `cargo xtask elaborate --lock`. Do not hand-edit —\n\
         // re-run the command and review the diff instead.\n\
         (\n    checksums: {\n",
    );
    for (key, digest) in checksums {
        let _ = writeln!(out, "        {key:?}: {digest:?},");
    }
    out.push_str("    },\n)\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_stable_and_sixteen_hex_chars() {
        let a = hash("Normal(name: \"Foo\")");
        let b = hash("Normal(name: \"Foo\")");
        let c = hash("Normal(name: \"Bar\")");
        assert_eq!(a, b, "hashing is deterministic");
        assert_ne!(a, c, "different content hashes differently");
        assert_eq!(a.len(), 16);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    // The body between the header comment and the trailing `)` parses as
    // plain RON (no macro layer needed — the lock is inert data).
    #[derive(serde::Deserialize)]
    struct Lock {
        checksums: BTreeMap<String, String>,
    }

    #[test]
    fn render_is_sorted_and_round_trips_through_ron() {
        let mut checksums = BTreeMap::new();
        checksums.insert(key("canon", "Zulaport Cutthroat"), "aaaa".to_string());
        checksums.insert(key("builtin", "Plains"), "bbbb".to_string());
        let rendered = render(&checksums);
        let builtin_at = rendered.find("builtin/Plains").unwrap();
        let canon_at = rendered.find("canon/Zulaport Cutthroat").unwrap();
        assert!(
            builtin_at < canon_at,
            "keys render in sorted order:\n{rendered}"
        );
        let body = rendered.lines().skip(2).collect::<Vec<_>>().join("\n");
        let parsed: Lock = ron::from_str(&body).expect("renders as valid RON");
        assert_eq!(parsed.checksums, checksums);
    }

    /// A synthetic workspace root shaped like the real one
    /// (`plugins/{builtin,canon,testing,demo}`), one finished card each in
    /// canon/testing.
    fn synthetic_workspace() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join("plugins/builtin/macros")).unwrap();
        for (plugin, card) in [
            ("canon", "Fine Bear"),
            ("testing", "Other Bear"),
            ("demo", "Third Bear"),
        ] {
            let cards = root.path().join("plugins").join(plugin).join("cards");
            std::fs::create_dir_all(&cards).unwrap();
            std::fs::write(
                cards.join(format!("{card}.ron")),
                format!(r#"Normal(name: "{card}", types: [Creature], power: 2, toughness: 2)"#),
            )
            .unwrap();
        }
        root
    }

    /// `compute` run twice with no changes produces byte-identical
    /// checksums — the `--lock` idempotency this ticket requires.
    #[test]
    fn compute_is_idempotent() {
        let root = synthetic_workspace();
        let first = compute(root.path()).unwrap();
        let second = compute(root.path()).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.len(), 3, "one finished card in each of 3 plugins");
    }

    /// A card-value change (not just re-authoring) changes exactly that
    /// card's checksum and none other — "drift is loud, never silent".
    #[test]
    fn a_card_value_change_drifts_only_its_own_checksum() {
        let root = synthetic_workspace();
        let before = compute(root.path()).unwrap();

        let drifted_path = root.path().join("plugins/canon/cards/Fine Bear.ron");
        std::fs::write(
            &drifted_path,
            r#"Normal(name: "Fine Bear", types: [Creature], power: 3, toughness: 2)"#,
        )
        .unwrap();
        let after = compute(root.path()).unwrap();

        assert_eq!(before.len(), after.len());
        for (name, before_hash) in &before {
            let after_hash = &after[name];
            if name == "canon/Fine Bear" {
                assert_ne!(before_hash, after_hash, "the drifted card must re-hash");
            } else {
                assert_eq!(before_hash, after_hash, "{name} must be unaffected");
            }
        }
    }
}
