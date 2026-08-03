//! Every real authored file parses through the forked reader, lowers, and comes
//! out byte-identical — "zero behaviour change" at the stored-byte level
//! (`docs/decisions/authoring-spelling-lowering.md` §5).
//!
//! Not built on `deckmaste_plugin`: its loader is `plugin-repoint`'s to
//! retarget, so this carries the small part of it a round-trip needs.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_authoring::macros::MacroDef;
use deckmaste_authoring::macros::MacroSet;
use deckmaste_authoring::macros::ParamTypeSet;
use deckmaste_lowering::Lower;
use serde::Serialize;
use serde::de::DeserializeOwned;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("workspace root resolves")
}

/// Every `*.ron` under `dir`, sorted. Skips `.todo.ron` stubs, which hold a
/// `Todo(…)` placeholder rather than a card.
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

/// The macro namespace the corpus is written against, built by
/// `make_empty_set` — [`load_macros`] for the authoring reader,
/// [`load_core_macros`] for the core one.
///
/// A definition may invoke a meta-macro from a file that has not loaded yet, so
/// failures are retried until a pass stops making progress.
fn load_macros_with(plugins: &[&str], make_empty_set: impl Fn() -> MacroSet) -> MacroSet {
    let root = workspace_root();
    let mut macros = make_empty_set();
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
                // Later plugins may override an inherited name.
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

/// The macro namespace the corpus is authored against.
fn load_macros(plugins: &[&str]) -> MacroSet {
    load_macros_with(plugins, deckmaste_authoring::macros::macro_set)
}

/// The param types macro definitions loaded into [`load_core_macros`] are
/// checked against, and macro-invocation arguments read through it are parsed
/// as. The core-typed twin of [`deckmaste_authoring::macros::param_types`] —
/// forked from `deckmaste_plugin::macros::param_types` (today's production
/// registry) for the same reason this file avoids depending on that crate: see
/// the module docs.
fn core_param_types() -> ParamTypeSet {
    let mut param_types = ParamTypeSet::default();
    param_types.add_typed::<deckmaste_core::Color>("Color");
    param_types.add_typed::<Vec<deckmaste_core::CostComponent>>("Cost");
    param_types.add_typed::<Vec<deckmaste_core::Ability>>("Abilities");
    param_types.add_typed::<Vec<deckmaste_core::Count>>("Counts");
    param_types.add_typed::<deckmaste_core::Ability>("Ability");
    param_types.add_typed::<deckmaste_core::Action>("Action");
    param_types.add_typed::<deckmaste_core::AsThough>("AsThough");
    param_types.add_typed::<deckmaste_core::Condition>("Condition");
    param_types.add_typed::<deckmaste_core::CostComponent>("CostComponent");
    param_types.add_typed::<deckmaste_core::Count>("Count");
    param_types.add_typed::<deckmaste_core::CounterRef>("CounterRef");
    param_types.add_typed::<deckmaste_core::Destination>("Destination");
    param_types.add_typed::<deckmaste_core::OneShotEffect>("OneShotEffect");
    param_types.add_typed::<deckmaste_core::EventFilter>("EventFilter");
    param_types.add_typed::<deckmaste_core::Predicate>("Predicate");
    param_types.add_typed::<deckmaste_core::Predicate>("CardTypePredicate");
    param_types.add_typed::<deckmaste_core::KeywordAbility>("KeywordAbility");
    param_types.add_typed::<deckmaste_core::ManaRider>("ManaRider");
    param_types.add_typed::<deckmaste_core::Modification>("Modification");
    param_types.add_typed::<deckmaste_core::NumericOp>("NumericOp");
    param_types.add_typed::<deckmaste_core::Quantity>("Quantity");
    param_types.add_typed::<deckmaste_core::Reference>("Reference");
    param_types.add_typed::<deckmaste_core::Replacement>("Replacement");
    param_types.add_typed::<deckmaste_core::Selection>("Selection");
    param_types.add_typed::<deckmaste_core::StaticEffect>("StaticEffect");
    param_types.add_typed::<deckmaste_core::TargetSpec>("TargetSpec");
    param_types.add_typed::<deckmaste_core::Subtype>("Subtype");
    param_types.add_typed::<deckmaste_core::TypeDef>("TypeDef");
    param_types.add_typed::<deckmaste_core::Zone>("Zone");
    param_types.add_typed::<deckmaste_core::Uint>("Uint");
    param_types
}

/// The same corpus namespace, built at CORE kinds — the reader the loader uses
/// today. Retired with `core-demacro`; until then it is the oracle that proves
/// the repoint changed no engine-side value.
fn load_core_macros(plugins: &[&str]) -> MacroSet {
    load_macros_with(plugins, || {
        MacroSet::new(deckmaste_core::ron::kinds())
            .with_options(deckmaste_core::ron::raw_options())
            .with_param_types(core_param_types())
    })
}

struct Swept {
    checked: usize,
    unparsed: usize,
    first_error: Option<String>,
}

/// Reads every file in `dir` as `A` and asserts that lowering both leaves its
/// stored bytes unchanged AND lands on the exact value the core reader
/// (`core_macros`) produces from the same source — structural equality, not
/// just matching serialization (which `skip_serializing_if` can hide a
/// difference behind).
///
/// A file that fails to PARSE is counted, not fatal; everything past the parse
/// is always fatal. See [`wizards_cards_lower_unchanged`] for who tolerates it.
fn sweep<A, C>(macros: &MacroSet, core_macros: &MacroSet, dir: &Path) -> Swept
where
    A: DeserializeOwned + Serialize + Lower<Target = C>,
    C: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
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
        // Read before `authored` is consumed by `lower()` below.
        let via_core: C = core_macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("core reader on {}: {e}", path.display()));
        let lowered = authored.lower();
        let after = deckmaste_core::ron::options()
            .to_string(&lowered)
            .unwrap_or_else(|e| panic!("writing lowered {}: {e}", path.display()));

        assert_eq!(
            before,
            after,
            "lowering changed the stored form of {}",
            path.display()
        );
        assert_eq!(
            lowered,
            via_core,
            "lowered core diverged from the core reader on {}",
            path.display()
        );
        swept.checked += 1;
    }
    swept
}

/// [`sweep`] for the curated corpora, where failing to parse is itself a
/// failure.
fn sweep_strict<A, C>(macros: &MacroSet, core_macros: &MacroSet, dir: &Path) -> usize
where
    A: DeserializeOwned + Serialize + Lower<Target = C>,
    C: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
{
    let swept = sweep::<A, C>(macros, core_macros, dir);
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
    let core_macros = load_core_macros(&["builtin", "canon"]);
    let builtin = sweep_strict::<deckmaste_authoring::Card, deckmaste_card::Card>(
        &macros,
        &core_macros,
        &plugin_dir("builtin", "cards"),
    );
    let canon = sweep_strict::<deckmaste_authoring::Card, deckmaste_card::Card>(
        &macros,
        &core_macros,
        &plugin_dir("canon", "cards"),
    );
    assert!(builtin > 0 && canon > 0, "no cards found to check");
}

#[test]
fn builtin_tokens_lower_unchanged() {
    let macros = load_macros(&["builtin"]);
    let core_macros = load_core_macros(&["builtin"]);
    let checked = sweep_strict::<deckmaste_authoring::Token, deckmaste_core::Token>(
        &macros,
        &core_macros,
        &plugin_dir("builtin", "tokens"),
    );
    assert!(checked > 0, "no tokens found to check");
}

/// The `rules/` tables are authored containers too (spec §4).
#[test]
fn builtin_rules_tables_lower_unchanged() {
    let macros = load_macros(&["builtin"]);
    let core_macros = load_core_macros(&["builtin"]);
    let rules = plugin_dir("builtin", "rules");
    let sba = sweep_strict::<Vec<deckmaste_authoring::SbaRule>, Vec<deckmaste_core::SbaRule>>(
        &macros,
        &core_macros,
        &rules.join("sba"),
    );
    let grant = sweep_strict::<
        Vec<deckmaste_authoring::ConferralRule>,
        Vec<deckmaste_core::ConferralRule>,
    >(&macros, &core_macros, &rules.join("grant"));
    let damage = sweep_strict::<
        Vec<deckmaste_authoring::DamageResultRule>,
        Vec<deckmaste_core::DamageResultRule>,
    >(&macros, &core_macros, &rules.join("damage"));
    assert!(
        sba > 0 && grant > 0 && damage > 0,
        "no rules tables found to check"
    );
}

/// Swept leniently on parse: `plugins/wizards` is gitignored and regenerated,
/// so a checkout holds whatever vintage it last generated, and one older than a
/// grammar change spells syntax the reader no longer accepts. Lowering stays
/// strict; the counts are printed rather than asserted.
#[test]
fn wizards_cards_lower_unchanged() {
    let macros = load_macros(&["builtin", "wizards"]);
    let core_macros = load_core_macros(&["builtin", "wizards"]);
    let swept = sweep::<deckmaste_authoring::Card, deckmaste_card::Card>(
        &macros,
        &core_macros,
        &plugin_dir("wizards", "cards"),
    );
    let total = swept.checked + swept.unparsed;
    println!(
        "wizards: {} of {total} cards lowered unchanged; {} unparsed (corpus vintage)",
        swept.checked, swept.unparsed,
    );
    if let Some(first) = &swept.first_error {
        println!("  first unparsed: {first}");
    }
    assert!(total > 0, "no wizards cards found at all");
    assert!(
        swept.checked > 0,
        "no wizards card parsed at all — with {total} present that is a reader failure",
    );
}
