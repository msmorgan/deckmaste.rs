//! The byte-identity oracle, on the real loader: every authored file in the
//! tree loads through [`Plugin`] and the restricted read API, its engine image
//! re-serializes to the exact bytes on disk, and that image is structurally
//! equal to what the pre-fork CORE-kinded reader produces from the same source
//! — "zero behaviour change" at the stored-byte level
//! (`docs/decisions/authoring-spelling-lowering.md` §5).
//!
//! Moved here from `deckmaste_lowering/tests/corpus.rs`, which hand-rolled the
//! small part of a plugin loader a round-trip needs precisely to avoid
//! depending on `deckmaste_plugin`. That constraint is gone — but the arrow
//! points the other way now (plugin depends on lowering), so the test moved
//! rather than the dependency being added.
//!
//! Inside this crate `lower(loaded.authored) == loaded.core` is a TAUTOLOGY:
//! [`Plugin::card_from_str`] builds `core` by lowering `authored`. The
//! assertion that still carries information is `loaded.core ==
//! core_reader.read_str(&source)` — the core-kinded reader is the one the
//! loader used before the repoint, so agreeing with it is what proves the
//! repoint changed no engine-side value. It retires with `core-demacro`.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_lowering::Lower;
use deckmaste_plugin::macros::MacroDef;
use deckmaste_plugin::macros::MacroSet;
use deckmaste_plugin::macros::ParamTypeSet;
use deckmaste_plugin::plugin::Plugin;
use serde::Serialize;
use serde::de::DeserializeOwned;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("workspace root resolves")
}

fn plugin_dir(plugin: &str, sub: &str) -> PathBuf {
    workspace_root().join("plugins").join(plugin).join(sub)
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

/// The param types core-kinded macro definitions are checked against, and that
/// macro-invocation arguments read through them are parsed as. The core-typed
/// twin of [`deckmaste_authoring::macros::param_types`] — a fork of the
/// pre-repoint production registry, kept here only as long as the oracle it
/// feeds (see the module docs).
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

/// The corpus macro namespace built at CORE kinds — the oracle side. The
/// loader itself no longer has one of these (it reads at authoring kinds and
/// lowers), so this is the last hand-rolled definition walk in the tree; it
/// dies with `core-demacro`.
///
/// A definition may invoke a meta-macro from a file that has not loaded yet, so
/// failures are retried until a pass stops making progress — the same shape
/// [`Plugin::load`] uses.
fn load_core_macros(plugins: &[&str]) -> MacroSet {
    let root = workspace_root();
    let mut macros = MacroSet::new(deckmaste_core::ron::kinds())
        .with_options(deckmaste_core::ron::raw_options())
        .with_param_types(core_param_types());
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

struct Swept {
    checked: usize,
    unparsed: usize,
    first_error: Option<String>,
}

/// The stored form of an authored value: what a round-trip through the
/// authoring writer produces, to be compared against the same value's engine
/// image written by the core writer.
fn authored_ron<T: Serialize>(value: &T) -> anyhow::Result<String> {
    Ok(deckmaste_authoring::ron::options().to_string(value)?)
}

/// One file through the real card API: the authored half re-serialized, and
/// the engine image the loader produced from the SAME parse.
fn load_card(plugin: &Plugin, source: &str) -> anyhow::Result<(String, deckmaste_card::Card)> {
    let loaded = plugin.card_from_str(source)?;
    Ok((authored_ron(&loaded.authored)?, loaded.core))
}

/// The token twin of [`load_card`].
fn load_token(plugin: &Plugin, source: &str) -> anyhow::Result<(String, deckmaste_core::Token)> {
    let loaded = plugin.token_from_str(source)?;
    Ok((authored_ron(&loaded.authored)?, loaded.core))
}

/// One rules-table file, read and lowered exactly as `Plugin`'s
/// `load_sba_rules`/`load_conferral_rules`/`load_damage_result_rules` do it —
/// through the plugin's own macro scope, at authoring kinds, lowering at the
/// boundary. Those loaders expose only the concatenated result, so a per-file
/// sweep has to repeat their two lines; the tables the loader itself built are
/// checked against this sweep's output in
/// [`builtin_rules_tables_lower_unchanged`].
fn load_rules<A>(macros: &MacroSet, source: &str) -> anyhow::Result<(String, A::Target)>
where
    A: DeserializeOwned + Serialize + Lower,
{
    let authored: A = macros.read_str(source)?;
    let before = authored_ron(&authored)?;
    Ok((before, authored.lower()))
}

/// Reads every file in `dir` through `load` (the loader path under test) and
/// asserts that its engine image both leaves the stored bytes unchanged AND
/// lands on the exact value the core reader (`core_macros`) produces from the
/// same source — structural equality, not just matching serialization (which
/// `skip_serializing_if` can hide a difference behind).
///
/// A file that fails to LOAD is counted, not fatal; everything past the load is
/// always fatal. Every caller asserts the count is zero — see
/// [`wizards_cards_lower_unchanged`] for why the count exists at all.
fn sweep<C>(
    core_macros: &MacroSet,
    dir: &Path,
    load: impl Fn(&str) -> anyhow::Result<(String, C)>,
) -> (Swept, Vec<C>)
where
    C: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
{
    let mut swept = Swept {
        checked: 0,
        unparsed: 0,
        first_error: None,
    };
    let mut loaded_all = Vec::new();
    for path in ron_files(dir) {
        let source = fs::read_to_string(&path).expect("corpus file is readable");
        let (before, core) = match load(&source) {
            Ok(pair) => pair,
            Err(error) => {
                swept.unparsed += 1;
                swept
                    .first_error
                    .get_or_insert_with(|| format!("{}: {error:#}", path.display()));
                continue;
            }
        };

        let via_core: C = core_macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("core reader on {}: {e}", path.display()));
        let after = deckmaste_core::ron::options()
            .to_string(&core)
            .unwrap_or_else(|e| panic!("writing the engine image of {}: {e}", path.display()));

        assert_eq!(
            before,
            after,
            "loading changed the stored form of {}",
            path.display()
        );
        assert_eq!(
            core,
            via_core,
            "the loader's engine image diverged from the core reader on {}",
            path.display()
        );
        swept.checked += 1;
        loaded_all.push(core);
    }
    (swept, loaded_all)
}

/// The per-file rules tables concatenated, in the same file order
/// [`Plugin`]'s own loaders visit — both walks are fully path-sorted.
fn flatten<T>(per_file: Vec<Vec<T>>) -> Vec<T> {
    per_file.into_iter().flatten().collect()
}

/// [`sweep`] where failing to load is itself a failure — every corpus but the
/// generated one.
fn sweep_strict<C>(
    core_macros: &MacroSet,
    dir: &Path,
    load: impl Fn(&str) -> anyhow::Result<(String, C)>,
) -> Vec<C>
where
    C: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
{
    let (swept, loaded) = sweep(core_macros, dir, load);
    assert_eq!(
        swept.unparsed,
        0,
        "{} file(s) under {} did not load; first: {}",
        swept.unparsed,
        dir.display(),
        swept.first_error.as_deref().unwrap_or("—"),
    );
    loaded
}

/// Each corpus is swept in the scope it is really loaded in, and the oracle is
/// built over the SAME plugin list — a card must not be read at authoring
/// kinds under one namespace and at core kinds under a wider one, or a name
/// canon overrides would be compared against builtin's definition of it.
#[test]
fn canon_and_builtin_cards_lower_unchanged() {
    let builtin = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/builtin"))
        .expect("builtin loads");
    let builtin_cards = sweep_strict(
        &load_core_macros(&["builtin"]),
        &plugin_dir("builtin", "cards"),
        |source| load_card(&builtin, source),
    );

    let canon = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/canon"))
        .expect("canon loads over the builtin prelude");
    let canon_cards = sweep_strict(
        &load_core_macros(&["builtin", "canon"]),
        &plugin_dir("canon", "cards"),
        |source| load_card(&canon, source),
    );

    assert!(
        !builtin_cards.is_empty() && !canon_cards.is_empty(),
        "no cards found to check"
    );
}

#[test]
fn builtin_tokens_lower_unchanged() {
    let core_macros = load_core_macros(&["builtin"]);
    let builtin = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/builtin"))
        .expect("builtin loads");
    let tokens = sweep_strict(&core_macros, &plugin_dir("builtin", "tokens"), |source| {
        load_token(&builtin, source)
    });
    assert!(!tokens.is_empty(), "no tokens found to check");
}

/// The `rules/` tables are authored containers too (spec §4), and the loader
/// lowers them at load. Each table is swept per file, then the concatenation is
/// checked against the table `Plugin` itself built — so this is a statement
/// about the loader, not about a re-implementation of it.
#[test]
fn builtin_rules_tables_lower_unchanged() {
    let core_macros = load_core_macros(&["builtin"]);
    let builtin = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/builtin"))
        .expect("builtin loads");
    let rules = plugin_dir("builtin", "rules");

    let sba = sweep_strict(&core_macros, &rules.join("sba"), |source| {
        load_rules::<Vec<deckmaste_authoring::SbaRule>>(&builtin.macros, source)
    });
    let grant = sweep_strict(&core_macros, &rules.join("grant"), |source| {
        load_rules::<Vec<deckmaste_authoring::ConferralRule>>(&builtin.macros, source)
    });
    let damage = sweep_strict(&core_macros, &rules.join("damage"), |source| {
        load_rules::<Vec<deckmaste_authoring::DamageResultRule>>(&builtin.macros, source)
    });
    assert!(
        !sba.is_empty() && !grant.is_empty() && !damage.is_empty(),
        "no rules tables found to check"
    );

    assert_eq!(
        flatten(sba),
        builtin.sba_rules,
        "the per-file sweep and `Plugin::sba_rules` disagree"
    );
    assert_eq!(
        flatten(grant),
        builtin.conferral_rules,
        "the per-file sweep and `Plugin::conferral_rules` disagree"
    );
    assert_eq!(
        flatten(damage),
        builtin.damage_result_rules,
        "the per-file sweep and `Plugin::damage_result_rules` disagree"
    );
}

/// The generated corpus — 7,294 cards, every one of which must load. The count
/// exists because `plugins/wizards` is gitignored and regenerated, so a
/// checkout holds whatever vintage it last generated; it is REPORTED with the
/// failure rather than tolerated. A regeneration that introduces genuinely
/// unparseable shapes is a signal worth seeing, not a tolerance worth keeping
/// silent — and without this assertion a loader change that broke nearly the
/// whole corpus would still report green.
#[test]
#[cfg_attr(
    not(wizards_corpus),
    ignore = "needs the generated plugins/wizards corpus (cargo xtask generate)"
)]
fn wizards_cards_lower_unchanged() {
    let core_macros = load_core_macros(&["builtin", "wizards"]);
    let wizards = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/wizards"))
        .expect("wizards loads over the builtin prelude");
    let dir = plugin_dir("wizards", "cards");
    let (swept, _) = sweep(&core_macros, &dir, |source| load_card(&wizards, source));

    let total = swept.checked + swept.unparsed;
    println!(
        "wizards: {} of {total} cards lowered unchanged",
        swept.checked
    );
    assert!(total > 0, "no wizards cards found at all");
    assert_eq!(
        swept.unparsed,
        0,
        "{} of {total} wizards card(s) did not load; first: {}",
        swept.unparsed,
        swept.first_error.as_deref().unwrap_or("—"),
    );
}
