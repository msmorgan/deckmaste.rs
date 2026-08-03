//! The write-back identity oracle, on the real loader: every authored file in
//! the tree loads through [`Plugin`] and the restricted read API, the two
//! writers agree on what it stores — the authoring writer's rendering of the
//! EXPANDED authored half is byte-identical to the core writer's rendering of
//! its engine image — and that image is structurally equal to what the
//! pre-fork CORE-kinded reader produces from the same source, likewise
//! expanded: "zero behaviour change"
//! (`docs/decisions/authoring-spelling-lowering.md` §5). Both sides compare
//! EXPANDED forms because `lower` erases invocation provenance (spec §12,
//! `runtime-prose-link`) — the core grammar is a compiled artifact and never
//! carries it, so an unexpanded authored rendering would disagree with the core
//! rendering on principle, not by accident.
//!
//! The byte comparison is between the two WRITERS, not against the file on
//! disk — canon files carry `//` header comments that RON serialization never
//! emits, so disk equality was never available. What it proves is that a macro
//! invocation's BODY survives write-back identically through either grammar.
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
//! core_reader.read_str(&source).expand_all()` — the core-kinded reader is
//! the one the loader used before the repoint, so agreeing with it (once
//! both sides are expanded down to the same erasure-free shape) is what
//! proves the repoint changed no engine-side value. It retires with
//! `core-demacro`.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_lowering::Lower;
use deckmaste_plugin::layout::CARDS_DIR;
use deckmaste_plugin::macros::MacroDef;
use deckmaste_plugin::macros::MacroSet;
use deckmaste_plugin::macros::ParamTypeSet;
use deckmaste_plugin::plugin::Plugin;
use macro_ron::Expand;
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
    // `before` is the EXPANDED authored half, not the raw one: `loaded.core`
    // (below) no longer carries invocation syntax post-erasure, so the
    // authored side has to shed the same wrappers to stay comparable —
    // otherwise this is comparing an invocation spelling against a body on
    // principle, not by accident.
    Ok((
        authored_ron(&loaded.authored.clone().expand_all())?,
        loaded.core,
    ))
}

/// The token twin of [`load_card`].
fn load_token(plugin: &Plugin, source: &str) -> anyhow::Result<(String, deckmaste_core::Token)> {
    let loaded = plugin.token_from_str(source)?;
    Ok((
        authored_ron(&loaded.authored.clone().expand_all())?,
        loaded.core,
    ))
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
    A: DeserializeOwned + Serialize + Lower + Expand + Clone,
{
    let authored: A = macros.read_str(source)?;
    let before = authored_ron(&authored.clone().expand_all())?;
    Ok((before, authored.lower()))
}

/// Reads every file in `dir` through `load` (the loader path under test) and
/// asserts that its engine image both writes to the same bytes the authored
/// half does through the authoring writer AND lands on the exact value the core
/// reader (`core_macros`) produces from the same source — structural equality,
/// not just matching serialization (which `skip_serializing_if` can hide a
/// difference behind).
///
/// A file that fails to LOAD is counted, not fatal; everything past the load is
/// always fatal. Every caller asserts the count is zero — see
/// [`wizards_cards_lower_unchanged`] for why the count exists at all.
fn sweep<C>(
    core_macros: &MacroSet,
    dir: &Path,
    load: impl Fn(&str) -> anyhow::Result<(String, C)>,
    expand: impl Fn(C) -> C,
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
            "the authoring and core writers disagree on the stored form of {}",
            path.display()
        );
        // Erasure is exactly `Expand`: `lower` drops each invocation wrapper
        // down to its body, which is what `expand_all` does on the core
        // side. Comparing against the EXPANDED oracle keeps the gate at full
        // strength rather than excusing the difference an unexpanded
        // `Expanded` wrapper would otherwise introduce.
        assert_eq!(
            core,
            expand(via_core),
            "the loader's engine image diverged from the core reader on {}",
            path.display()
        );
        swept.checked += 1;
        loaded_all.push(core);
    }
    (swept, loaded_all)
}

/// The core-side twin of [`macro_ron::Expand::expand_all`] for a whole card:
/// `deckmaste_card` does not depend on `macro_ron` at all (its own module
/// docs), so `Card`/`CardFace` have no derived `Expand` impl to call —
/// unlike every field type they carry, which does (the grammar leaves, all
/// the way down to [`deckmaste_core::Ability`], derive it). This recurses
/// field by field instead, matching what a derived `expand_all` would do.
fn expand_card(card: deckmaste_card::Card) -> deckmaste_card::Card {
    fn expand_face(face: deckmaste_card::CardFace) -> deckmaste_card::CardFace {
        deckmaste_card::CardFace {
            name: face.name,
            mana_cost: face.mana_cost.expand_all(),
            color_indicator: face.color_indicator.expand_all(),
            supertypes: face.supertypes.expand_all(),
            types: face.types.expand_all(),
            subtypes: face.subtypes.expand_all(),
            abilities: face.abilities.expand_all(),
            power: face.power.expand_all(),
            toughness: face.toughness.expand_all(),
            loyalty: face.loyalty.expand_all(),
            defense: face.defense.expand_all(),
        }
    }
    match card {
        deckmaste_card::Card::Normal(face) => deckmaste_card::Card::Normal(expand_face(face)),
        deckmaste_card::Card::TwoFaced {
            layout,
            front,
            back,
        } => deckmaste_card::Card::TwoFaced {
            layout,
            front: expand_face(front),
            back: expand_face(back),
        },
    }
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
    expand: impl Fn(C) -> C,
) -> Vec<C>
where
    C: DeserializeOwned + Serialize + PartialEq + std::fmt::Debug,
{
    let (swept, loaded) = sweep(core_macros, dir, load, expand);
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
        expand_card,
    );

    let canon = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/canon"))
        .expect("canon loads over the builtin prelude");
    let canon_cards = sweep_strict(
        &load_core_macros(&["builtin", "canon"]),
        &plugin_dir("canon", "cards"),
        |source| load_card(&canon, source),
        expand_card,
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
    let tokens = sweep_strict(
        &core_macros,
        &plugin_dir("builtin", "tokens"),
        |source| load_token(&builtin, source),
        |t: deckmaste_core::Token| t.expand_all(),
    );
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

    let sba = sweep_strict(
        &core_macros,
        &rules.join("sba"),
        |source| load_rules::<Vec<deckmaste_authoring::SbaRule>>(&builtin.macros, source),
        |v: Vec<deckmaste_core::SbaRule>| v.expand_all(),
    );
    let grant = sweep_strict(
        &core_macros,
        &rules.join("grant"),
        |source| load_rules::<Vec<deckmaste_authoring::ConferralRule>>(&builtin.macros, source),
        |v: Vec<deckmaste_core::ConferralRule>| v.expand_all(),
    );
    let damage = sweep_strict(
        &core_macros,
        &rules.join("damage"),
        |source| load_rules::<Vec<deckmaste_authoring::DamageResultRule>>(&builtin.macros, source),
        |v: Vec<deckmaste_core::DamageResultRule>| v.expand_all(),
    );
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

/// The generated corpus, every card of which must load. The count exists
/// because `plugins/wizards` is gitignored and regenerated, so a checkout holds
/// whatever vintage it last generated; it is REPORTED with the failure rather
/// than tolerated. A regeneration that introduces genuinely unparseable shapes
/// is a signal worth seeing, not a tolerance worth keeping silent — and without
/// this assertion a loader change that broke nearly the whole corpus would
/// still report green.
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
    let (swept, _) = sweep(
        &core_macros,
        &dir,
        |source| load_card(&wizards, source),
        expand_card,
    );

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

/// Every face of an authored card, front-to-back. `deckmaste_authoring::Card`
/// has no public accessor for this (its two variants are the whole public
/// surface), so — like the same-shaped private helpers in
/// `deckmaste_legacy_render::fidelity` and `xtask::macros::pilot` — each
/// caller that needs "every face" writes the two-arm match once.
fn authored_faces(card: &deckmaste_authoring::Card) -> Vec<&deckmaste_authoring::CardFace> {
    match card {
        deckmaste_authoring::Card::Normal(face) => vec![face],
        deckmaste_authoring::Card::TwoFaced { front, back, .. } => vec![front, back],
    }
}

/// Every `Ability` reachable from `ability`, at ANY depth, in pre-order —
/// `ability` itself plus the full transitive closure of
/// [`nested_abilities`](deckmaste_authoring::Ability::nested_abilities).
/// This is the authored-side walk
/// [`ProvenanceIndex::insert_ability_owned`](deckmaste_plugin::provenance::ProvenanceIndex)
/// does when indexing (`crates/deckmaste_plugin/src/provenance.rs`) — an
/// ability granting an ability that itself grants an ability is indexed at
/// every depth there, so this test has to visit every depth too, or a
/// context-dependence bug two-or-more `Ability`-levels down would pass
/// silently. Reuses `nested_abilities` (one level) recursively rather than
/// re-implementing the walk.
fn authored_ability_subterms(
    ability: &deckmaste_authoring::Ability,
) -> Vec<&deckmaste_authoring::Ability> {
    let mut out = vec![ability];
    for child in ability.nested_abilities() {
        out.extend(authored_ability_subterms(child));
    }
    out
}

/// One authored source file, loaded through the real plugin API, keeping the
/// authored/core pair intact — the pair [`load_card`] already reads, minus
/// the write-back re-serialization
/// [`every_authored_ability_subterm_appears_in_its_lowered_card`] does not
/// need.
fn load_card_pair(plugin: &Plugin, path: &Path) -> anyhow::Result<deckmaste_plugin::LoadedCard> {
    let source = fs::read_to_string(path)?;
    plugin.card_from_str(&source)
}

/// Every top-level [`deckmaste_core::Ability`] on a lowered card's faces —
/// the core-side twin of [`authored_faces`], stopping at the face's own
/// `abilities` list. [`nested_in_any`] does the descent past that point.
fn core_abilities(card: &deckmaste_card::Card) -> impl Iterator<Item = &deckmaste_core::Ability> {
    let faces: Vec<&deckmaste_card::CardFace> = match card {
        deckmaste_card::Card::Normal(face) => vec![face],
        deckmaste_card::Card::TwoFaced { front, back, .. } => vec![front, back],
    };
    faces.into_iter().flat_map(|face| face.abilities.iter())
}

/// The core-side twin of [`deckmaste_authoring::AbilitySubterms`]: pushes
/// every `deckmaste_core::Ability` reachable from a value, stopping AT each
/// one. Hand-written for the same reason the authoring trait is (no blanket
/// `impl<T> Trait for T` on stable Rust) and covering the same ability-
/// bearing positions, for the same reason: this test only means what it
/// claims to mean if the two walkers — the one deciding which authored
/// subterms to check, and this one deciding where to look for their lowered
/// images — agree on which grammar positions carry an `Ability`.
trait CoreAbilitySubterms {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>);
}

impl<T: CoreAbilitySubterms + ?Sized> CoreAbilitySubterms for std::sync::Arc<T> {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        (**self).push_abilities(out);
    }
}

impl<T: CoreAbilitySubterms> CoreAbilitySubterms for [T] {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        for x in self {
            x.push_abilities(out);
        }
    }
}

impl<T: CoreAbilitySubterms> CoreAbilitySubterms for Vec<T> {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        self.as_slice().push_abilities(out);
    }
}

impl<T: CoreAbilitySubterms> CoreAbilitySubterms for Option<T> {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        if let Some(x) = self {
            x.push_abilities(out);
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::Ability {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        out.push(self);
    }
}

impl CoreAbilitySubterms for deckmaste_core::KeywordAbility {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::Composite { abilities, .. } => abilities.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
            Self::FirstStrike
            | Self::DoubleStrike
            | Self::Deathtouch
            | Self::Trample
            | Self::Vigilance => {}
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::StaticEffect {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::Modify(_, m) => m.push_abilities(out),
            Self::Each(_, e) | Self::Conditionally(_, e) => e.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
            // No other `StaticEffect` shape carries an `Ability` — same
            // catch-all the authoring twin uses, for the same reason.
            _ => {}
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::Modification {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        // Exhaustive on purpose, mirroring the authoring side: this is THE
        // layer-6 grant position ([CR#613.1f]).
        match self {
            Self::GainAbility(a) => a.push_abilities(out),
            Self::Several(ms) => ms.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
            Self::Power(_)
            | Self::Toughness(_)
            | Self::SwitchPowerToughness
            | Self::Colors(_)
            | Self::CardTypes(_)
            | Self::Subtypes(_)
            | Self::Supertypes(_)
            | Self::LoseAbility(_)
            | Self::LoseAllAbilities
            | Self::CantHaveAbility(_)
            | Self::SetController(_)
            | Self::SetText(_)
            | Self::AllCreatureTypes
            | Self::BaseLoyalty(_)
            | Self::BaseDefense(_)
            | Self::BecomeBasicLandType(_) => {}
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::OneShotEffect {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::Act(a) => a.push_abilities(out),
            Self::Sequentially(es) | Self::Simultaneously(es) => es.push_abilities(out),
            Self::Continuously(c) => c.effect.push_abilities(out),
            Self::Until(_, es) => es.push_abilities(out),
            Self::Label(l) => l.effect.push_abilities(out),
            Self::SeparatePiles(s) => s.then.push_abilities(out),
            Self::ChoosePile(c) => c.then.push_abilities(out),
            Self::May(m) => {
                m.effect.push_abilities(out);
                m.if_did.push_abilities(out);
                m.if_not.push_abilities(out);
            }
            Self::If(i) => {
                i.then.push_abilities(out);
                i.otherwise.push_abilities(out);
            }
            Self::AdditionalCost(a) => a.body.push_abilities(out),
            Self::Each(e) => e.effect.push_abilities(out),
            Self::With(w) => w.body.push_abilities(out),
            Self::Distribute(d) => d.body.push_abilities(out),
            Self::Noting(n) => n.effect.push_abilities(out),
            Self::Delayed(t) | Self::Reflexive(t) => t.effect.push_abilities(out),
            Self::Modal(m) => {
                for mode in m.modes.iter() {
                    mode.effect.push_abilities(out);
                }
            }
            Self::Targeted(t) => t.effect.push_abilities(out),
            Self::Repeat(_, e) | Self::Batch(_, e) => e.push_abilities(out),
            Self::RevealUntil(r) => r.body.push_abilities(out),
            Self::Expanded(e) => e.value.push_abilities(out),
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::Action {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::GetEmblem(_, abilities) => abilities.push_abilities(out),
            // As with `StaticEffect`: dozens of variants, few ability-
            // bearing, and the list churns; the catch-all mirrors the
            // authoring side.
            _ => {}
        }
    }
}

/// The core-side twin of [`deckmaste_authoring::Ability::nested_abilities`]:
/// every `Ability` nested one level inside this one, in pre-order.
fn core_nested_abilities(ability: &deckmaste_core::Ability) -> Vec<&deckmaste_core::Ability> {
    let mut out = Vec::new();
    match ability {
        deckmaste_core::Ability::Static(e) => e.push_abilities(&mut out),
        deckmaste_core::Ability::Activated(a) => a.effect.push_abilities(&mut out),
        deckmaste_core::Ability::Triggered(a) => a.effect.push_abilities(&mut out),
        deckmaste_core::Ability::Spell(a) => a.effect.push_abilities(&mut out),
        deckmaste_core::Ability::Keyword(k) => k.push_abilities(&mut out),
        deckmaste_core::Ability::Innate(a) => out.push(a),
        deckmaste_core::Ability::Expanded(e) => out.push(&e.value),
    }
    out
}

/// Whether `image` appears anywhere below one of `lowered`'s members — the
/// descent [`core_abilities`] deliberately stops short of (it only walks
/// each face's own `abilities` list). Mirrors the authoring side's split
/// between `push_abilities` (stop at each `Ability`) and `nested_abilities`
/// (one level; the caller recurses): this function is the recursing caller.
fn nested_in_any(lowered: &[&deckmaste_core::Ability], image: &deckmaste_core::Ability) -> bool {
    lowered.iter().any(|a| ability_contains(a, image))
}

fn ability_contains(ability: &deckmaste_core::Ability, image: &deckmaste_core::Ability) -> bool {
    core_nested_abilities(ability)
        .into_iter()
        .any(|child| child == image || ability_contains(child, image))
}

/// Lowering is context-free at `Ability` granularity: an ability's image does
/// not depend on what surrounds it. Erasure is the crate's first non-identity
/// arm family, so this is the property that says the erasure did not smuggle
/// in a context dependence — every authored ability subterm's lowering, AT
/// ANY NESTING DEPTH (an ability granting an ability that itself grants an
/// ability, and so on), appears verbatim in its lowered card.
#[test]
fn every_authored_ability_subterm_appears_in_its_lowered_card() {
    let canon = Plugin::load_with_sibling_prelude(plugin_dir("canon", "")).unwrap();
    let mut checked = 0;
    for source in ron_files(&plugin_dir("canon", CARDS_DIR)) {
        let loaded = load_card_pair(&canon, &source).unwrap();
        let lowered: Vec<&deckmaste_core::Ability> = core_abilities(&loaded.core).collect();
        for face in authored_faces(&loaded.authored) {
            for authored in &face.abilities {
                for subterm in authored_ability_subterms(authored) {
                    let image = subterm.clone().lower();
                    assert!(
                        lowered.iter().any(|a| **a == image) || nested_in_any(&lowered, &image),
                        "{}: an authored ability subterm's lowering is absent from the \
                         lowered card — lowering is context-dependent at Ability granularity",
                        source.display(),
                    );
                    checked += 1;
                }
            }
        }
    }
    assert!(checked > 0, "corpus produced no ability subterms");
}
