//! P0.W0 conformance rail: the keyword-classification drift-guard.
//!
//! The mtg-rules skill's `keywords-classified.json` is the pinned
//! authoritative classification (docs/rules-taxonomy.md §10): the
//! `KeywordAbility` enum may contain ONLY keywords it classes `intrinsic`,
//! and every `KeywordAbility`-kind plugin macro must be a NON-intrinsic
//! class (composite / composite-given / marker). Containment, not equality
//! — intrinsics the engine has not implemented (banding, companion, mutate,
//! phasing) are legitimately absent from the enum.
//!
//! The JSON ships with the skill, not this repo: located via
//! `$MTG_RULES_SKILL`, defaulting to `~/.claude/skills/mtg-rules/`. When it
//! is absent (CI without the skill) the tests SKIP loudly rather than fail.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::macros::MacroDef;
use deckmaste_core::KeywordAbility;

fn skill_dir() -> PathBuf {
    if let Ok(p) = std::env::var("MTG_RULES_SKILL") {
        return PathBuf::from(p);
    }
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home).join(".claude/skills/mtg-rules")
}

fn classification() -> Option<serde_json::Value> {
    let path = skill_dir().join("keywords-classified.json");
    let Ok(text) = std::fs::read_to_string(&path) else {
        eprintln!(
            "classification drift-guard SKIPPED: {} not readable \
             (install the mtg-rules skill or set MTG_RULES_SKILL)",
            path.display()
        );
        return None;
    };
    Some(serde_json::from_str(&text).expect("keywords-classified.json parses as JSON"))
}

/// The `class` of the keyword ABILITY whose `ident` matches (keyword
/// actions share names with abilities, so the kind filter is load-bearing).
fn class_of(json: &serde_json::Value, ident: &str) -> Option<String> {
    json["keywords"]
        .as_array()?
        .iter()
        .find(|k| k["ident"].as_str() == Some(ident) && k["kind"].as_str() == Some("ability"))
        .and_then(|k| k["class"].as_str())
        .map(str::to_owned)
}

#[test]
fn intrinsic_enum_matches_the_classification() {
    let Some(json) = classification() else { return };
    for kw in KeywordAbility::ALL {
        assert_eq!(
            class_of(&json, kw.as_str()).as_deref(),
            Some("intrinsic"),
            "KeywordAbility::{} is in the native enum but the skill does not \
             class it intrinsic — reclassify upstream or demote it to a macro",
            kw.as_str(),
        );
    }
}

fn ron_files_under(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            ron_files_under(&path, out);
        } else if path.extension().is_some_and(|e| e == "ron") {
            out.push(path);
        }
    }
}

#[test]
fn keyword_macros_match_the_classification() {
    let Some(json) = classification() else { return };
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut checked = 0;
    // The TRACKED plugins only — local-only artifacts (`for-review`, per its
    // README) don't exist on fresh clones/workspaces and carry no macros.
    for plugin in ["builtin", "canon", "testing"] {
        let mut files = Vec::new();
        ron_files_under(
            &root.join("plugins").join(plugin).join("macros"),
            &mut files,
        );
        // Meta-macro invocation files (KeywordAbility(name: "Flying", …),
        // subtype templates) don't parse as a literal MacroDef — resolve
        // those through the LOADED set instead, which also proves the
        // registration round-trip the literal scan never exercised.
        let loaded = deckmaste_cards::plugin::Plugin::load_with_sibling_prelude(
            root.join("plugins").join(plugin),
        )
        .unwrap_or_else(|e| panic!("loading plugin {plugin}: {e:#}"));
        for path in files {
            let text = std::fs::read_to_string(&path).unwrap();
            let literal = deckmaste_core::ron::options().from_str::<MacroDef>(&text);
            let name_and_kinds = if let Ok(def) = literal {
                (def.name, def.kinds.clone())
            } else {
                // An invocation file registers a macro under the file's
                // stem; not found under KeywordAbility = some other
                // kind's meta-invocation (subtype templates) — skip.
                let stem = path.file_stem().unwrap().to_str().unwrap();
                if let Some(def) = loaded.macros.get("KeywordAbility", stem) {
                    (def.name, def.kinds.clone())
                } else {
                    continue;
                }
            };
            let def_name = name_and_kinds.0;
            if !name_and_kinds
                .1
                .iter()
                .any(|k| k.as_str() == "KeywordAbility")
            {
                continue;
            }
            // Everything non-intrinsic is macro territory: composite,
            // composite-given (name-carrying until its primitive exists),
            // and marker (a name-only composite IS a marker's semantics).
            // Intrinsics belong in the enum; unknown names are drift.
            let class = class_of(&json, def_name.as_str());
            assert!(
                matches!(
                    class.as_deref(),
                    Some("composite" | "composite-given" | "marker")
                ),
                "keyword macro {:?} ({}) must be a non-intrinsic class \
                 per the skill classification, got {class:?}",
                def_name.as_str(),
                path.display(),
            );
            checked += 1;
        }
    }
    assert!(
        checked >= 3,
        "expected at least the builtin Flying/Lifelink/Reach keyword macros, checked {checked}"
    );
}
