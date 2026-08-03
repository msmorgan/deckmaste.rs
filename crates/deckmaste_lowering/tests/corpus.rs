//! The corpus gate: every real authored file in the tree parses through the
//! FORKED reader, lowers, and comes out byte-identical.
//!
//! This is the check the per-variant tests cannot make. Those build minimal
//! values, one variant at a time, in isolation; these are the actual card,
//! token and rules-table files — deeply nested, macro-expanded, carrying real
//! `Expanded(…)` invocation provenance.
//!
//! It gates both halves of the claim at once:
//!
//! - `deckmaste_authoring` reads the whole corpus through its own kind registry
//!   and param-type registry (the fork's own gate), and
//! - `lower` is total on real data and changes no stored bytes — "zero
//!   behaviour change" measured at the stored-byte level
//!   (`docs/decisions/authoring-spelling-lowering.md` §5).
//!
//! Deliberately NOT built on `deckmaste_plugin`. That crate's loader is
//! `plugin-repoint`'s to retarget, and touching it here would hand that ticket
//! a half-migrated crate. All this gate needs is the ~40-line core of it: read
//! the definition files, parse each as a `MacroDef`, retry until a pass stops
//! making progress, insert. The registry side-tables the real loader also
//! builds — subtypes, types, counters, designations, keywords, verbs — serve
//! lints and the engine, and nothing here consults them.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_authoring::macros::MacroDef;
use deckmaste_authoring::macros::MacroSet;
use deckmaste_lowering::Lower;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// The workspace root, from this crate's manifest directory.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("workspace root resolves")
}

/// Every `*.ron` under `dir`, recursively, sorted so failures are reproducible.
///
/// Skips the legacy `.todo.ron` stubs: those hold a `Todo(…)` placeholder, not
/// a card. (`.ron.todo`, the ungraduated form, does not end in `.ron` and never
/// matches in the first place.)
fn ron_files(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(ron_files(&path));
        } else if path.extension().is_some_and(|e| e == "ron")
            && !path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".todo.ron"))
        {
            found.push(path);
        }
    }
    found.sort();
    found
}

/// The macro namespace the corpus is written against.
///
/// File order is alphabetical happenstance and a definition may invoke a
/// meta-macro from a file that has not loaded yet, so failures are retried
/// until a pass stops making progress; only then is the first one real. Later
/// plugins override inherited names, which is why this is `replace` and not
/// `insert`.
fn load_macros(plugins: &[&str]) -> MacroSet {
    let root = workspace_root();
    let mut macros = deckmaste_authoring::macros::macro_set();
    let mut pending: Vec<(PathBuf, String)> = plugins
        .iter()
        .flat_map(|plugin| ron_files(&root.join("plugins").join(plugin).join("macros")))
        .map(|path| {
            let source = fs::read_to_string(&path).expect("definition file is readable");
            (path, source)
        })
        .collect();

    while !pending.is_empty() {
        let attempted = pending.len();
        let mut failures = Vec::new();
        for (path, source) in pending {
            match macros.read_str::<MacroDef>(&source) {
                Ok(def) => macros
                    .replace(&def)
                    .unwrap_or_else(|e| panic!("inserting {}: {e}", path.display())),
                Err(error) => failures.push((path, source, error)),
            }
        }
        assert!(
            failures.len() < attempted,
            "no progress loading definitions; first failure: {} — {}",
            failures[0].0.display(),
            failures[0].2,
        );
        pending = failures
            .into_iter()
            .map(|(path, source, _)| (path, source))
            .collect();
    }
    macros
}

/// What a directory sweep found.
struct Swept {
    /// Files that parsed, lowered, and came out byte-identical.
    checked: usize,
    /// Files that did not parse at all, with the first such error.
    unparsed: usize,
    first_error: Option<String>,
}

/// Reads every file in `dir` as `A`, lowers it, and asserts the stored bytes
/// are unchanged.
///
/// A file that does not PARSE is counted, not fatal — see [`sweep_strict`] and
/// [`wizards_cards_lower_unchanged`] for which callers tolerate that and why.
/// Anything past the parse — lowering, and the byte comparison — is always
/// fatal: that is the property under test, and no corpus condition excuses it.
fn sweep<A>(macros: &MacroSet, dir: &Path) -> Swept
where
    A: DeserializeOwned + Serialize + Lower,
    A::Target: Serialize,
{
    let mut swept = Swept {
        checked: 0,
        unparsed: 0,
        first_error: None,
    };
    for path in ron_files(dir) {
        let source = fs::read_to_string(&path).expect("corpus file is readable");
        let authored: A = match macros.read_str(&source) {
            Ok(value) => value,
            Err(error) => {
                swept.unparsed += 1;
                swept
                    .first_error
                    .get_or_insert_with(|| format!("{}: {error}", path.display()));
                continue;
            }
        };

        let before = deckmaste_authoring::ron::options()
            .to_string(&authored)
            .unwrap_or_else(|e| panic!("writing authored {}: {e}", path.display()));
        let after = deckmaste_core::ron::options()
            .to_string(&authored.lower())
            .unwrap_or_else(|e| panic!("writing lowered {}: {e}", path.display()));

        assert_eq!(
            before,
            after,
            "lowering changed the stored form of {}",
            path.display()
        );
        swept.checked += 1;
    }
    swept
}

/// [`sweep`] for the hand-authored corpora, where a file failing to parse IS a
/// failure: canon, builtin and the rules tables are curated and every file in
/// them must read.
fn sweep_strict<A>(macros: &MacroSet, dir: &Path) -> usize
where
    A: DeserializeOwned + Serialize + Lower,
    A::Target: Serialize,
{
    let swept = sweep::<A>(macros, dir);
    assert_eq!(
        swept.unparsed,
        0,
        "{} file(s) under {} did not parse; first: {}",
        swept.unparsed,
        dir.display(),
        swept.first_error.as_deref().unwrap_or("—"),
    );
    swept.checked
}

fn plugin_dir(plugin: &str, sub: &str) -> PathBuf {
    workspace_root().join("plugins").join(plugin).join(sub)
}

#[test]
fn canon_and_builtin_cards_lower_unchanged() {
    let macros = load_macros(&["builtin", "canon"]);
    let builtin =
        sweep_strict::<deckmaste_authoring::Card>(&macros, &plugin_dir("builtin", "cards"));
    let canon = sweep_strict::<deckmaste_authoring::Card>(&macros, &plugin_dir("canon", "cards"));
    // A silent zero would make this gate vacuous.
    assert!(builtin > 0 && canon > 0, "no cards found to check");
}

#[test]
fn builtin_tokens_lower_unchanged() {
    let macros = load_macros(&["builtin"]);
    let checked =
        sweep_strict::<deckmaste_authoring::Token>(&macros, &plugin_dir("builtin", "tokens"));
    assert!(checked > 0, "no tokens found to check");
}

/// The `rules/` engine tables are authored containers too (§4's container
/// table), which is why their types forked along with the card grammar.
#[test]
fn builtin_rules_tables_lower_unchanged() {
    let macros = load_macros(&["builtin"]);
    let rules = plugin_dir("builtin", "rules");
    let sba = sweep_strict::<Vec<deckmaste_authoring::SbaRule>>(&macros, &rules.join("sba"));
    let grant =
        sweep_strict::<Vec<deckmaste_authoring::ConferralRule>>(&macros, &rules.join("grant"));
    let damage =
        sweep_strict::<Vec<deckmaste_authoring::DamageResultRule>>(&macros, &rules.join("damage"));
    assert!(
        sba > 0 && grant > 0 && damage > 0,
        "no rules tables found to check"
    );
}

/// The generated corpus: far larger than canon and machine-written, so it
/// exercises shapes a hand-authored file never reaches.
///
/// Unlike the curated corpora this one is swept LENIENTLY on parse, because it
/// is known-incomplete by construction: `cargo xtask generate` emits cards
/// referencing macros nobody has defined yet and says so as it runs ("other
/// failures: 57", a list of unresolved macro names). `plugins/wizards` is also
/// gitignored and regenerated per workspace, so its exact contents differ
/// between checkouts — two provisioned workspaces here held 7,294 and 7,353
/// cards. Demanding that every generated card parse would assert something the
/// tree does not claim, and would fail differently depending on where it ran.
///
/// What IS asserted, and is the whole point: every generated card that parses
/// lowers with its stored bytes unchanged. That is checked strictly — only the
/// parse is forgiven. Closing the parse gap belongs to the corpus effort;
/// `plugin-repoint` owns the wizards save-load round-trip proper.
///
/// The skip count is printed rather than swallowed, and floored, so the gate
/// cannot quietly decay into checking nothing.
#[test]
fn wizards_cards_lower_unchanged() {
    let macros = load_macros(&["builtin", "wizards"]);
    let swept = sweep::<deckmaste_authoring::Card>(&macros, &plugin_dir("wizards", "cards"));
    let total = swept.checked + swept.unparsed;
    println!(
        "wizards: {} of {} cards parsed and lowered unchanged; {} unparsed (corpus debt)",
        swept.checked, total, swept.unparsed,
    );
    if let Some(first) = &swept.first_error {
        println!("  first unparsed: {first}");
    }
    assert!(total > 0, "no wizards cards found at all");
    assert!(
        swept.checked * 4 > total * 3,
        "only {} of {} wizards cards parsed — the corpus or the reader has regressed \
         well past known debt",
        swept.checked,
        total,
    );
}
