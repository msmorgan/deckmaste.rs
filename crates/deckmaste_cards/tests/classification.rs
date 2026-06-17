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

/// A keyword macro under `keyword/derived/` is a parameterized instance of a
/// base composite (rules-taxonomy §10), classified by its delegate rather than
/// by its own name — the skill enumerates only the umbrella keyword.
fn is_derived(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == "derived")
}

/// The leading identifier of a macro body — the base keyword a `derived/`
/// variant delegates to, e.g. `Landwalk` from
/// `Landwalk(quality: Subtype("Desert"))`. Empty when the body is not a macro
/// invocation, which then fails the classification lookup loudly (a derived
/// macro MUST delegate to a named base).
fn delegate_head(body: &str) -> String {
    body.trim_start()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[test]
fn delegate_head_reads_the_base_keyword() {
    assert_eq!(
        delegate_head(r#"Landwalk(quality: Subtype("Desert"))"#),
        "Landwalk"
    );
    assert_eq!(delegate_head("  Cycling(cost: Mana(\"{2}\"))"), "Cycling");
    // A non-delegating body yields no base keyword → lookup fails loudly.
    assert_eq!(delegate_head(r#"(name: "x")"#), "");
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
            // A literal MacroDef, or — for a meta-macro invocation file that
            // doesn't parse as one — the macro it registers under its stem,
            // resolved through the LOADED set (which also proves the
            // registration round-trip the literal scan never exercises). Not
            // found under KeywordAbility = some other kind's meta-invocation
            // (subtype templates) — skip.
            let def: MacroDef =
                if let Ok(def) = deckmaste_core::ron::options().from_str::<MacroDef>(&text) {
                    def
                } else {
                    let stem = path.file_stem().unwrap().to_str().unwrap();
                    let Some(def) = loaded.macros.get("KeywordAbility", stem) else {
                        continue;
                    };
                    def.clone()
                };
            if !def.kinds.iter().any(|k| k.as_str() == "KeywordAbility") {
                continue;
            }
            // Base composites are classified by their own name. Per-type
            // *variant* macros under `keyword/derived/` (landwalk-by-subtype,
            // typecycling, …) are parameterized instances of a base composite:
            // rules-taxonomy §10 models "landwalk [type-expression]" as ONE
            // composite, so the skill classifies only the umbrella. A derived
            // macro therefore inherits the class of the base keyword its body
            // delegates to (`Landwalk` from `Landwalk(quality: …)`). A derived
            // macro that delegates to an UNclassified base still fails the
            // lookup, so the guard keeps its teeth.
            //
            // Everything non-intrinsic is macro territory: composite,
            // composite-given (name-carrying until its primitive exists), and
            // marker (a name-only composite IS a marker's semantics).
            // Intrinsics belong in the enum; unknown names are drift.
            let key = if is_derived(&path) {
                delegate_head(def.body())
            } else {
                def.name.as_str().to_owned()
            };
            let class = class_of(&json, &key);
            assert!(
                matches!(
                    class.as_deref(),
                    Some("composite" | "composite-given" | "marker")
                ),
                "keyword macro {:?} ({}) must resolve to a non-intrinsic class \
                 per the skill classification (lookup key {key:?}), got {class:?}",
                def.name.as_str(),
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
