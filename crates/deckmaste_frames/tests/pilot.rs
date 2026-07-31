//! Task 6 integration test: every loaded macro def with a non-empty
//! `frames:` list, plus every constructor-catalog entry, compiles clean
//! (G1) against `plugins/builtin`. Also locks in the round's two negative G5
//! findings — `ControlledByYou`'s attempted complement-side `FieldSlice`
//! frame and `SacrificeThis`'s attempted `~` frame — as regression tests, so
//! a later change to the compiler's behavior on either shape shows up here
//! rather than only in prose.
//!
//! See `docs/superpowers/research/2026-07-30-macro-frames/
//! pilot-constituency-findings.md` for the full writeup these tests are
//! evidence for.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::plugin::Plugin;
use deckmaste_english::CatalogKind;
use deckmaste_english::Catalogs;
use deckmaste_english::FragmentKind;
use deckmaste_frames::HoleClass;
use macro_ron::Ident;
use macro_ron::MacroDef;
use macro_ron::Params;
use macro_ron::frames::FrameSpec;
use macro_ron::frames::load_constructor_frames;

fn plugin_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")
}

/// The generated, CR-derived catalogs `cargo xtask english bracket`/the
/// corpus tooling load real oracle text against
/// (`crates/xtask/src/english/data.rs`'s `load_catalogs`, reimplemented here
/// rather than depended on: `xtask` is a binary crate, not a library this
/// crate should pull in). **Load-bearing, not incidental**: `Catalogs::
/// default()` — what `cargo xtask macro inspect` uses — has zero entries in
/// every catalog, so a `KeywordLine` frame can never parse against it (the
/// keyword-line grammar recognizes a keyword atom by catalog lookup, not
/// free parsing); see the G5 report for this finding in full, discovered
/// while authoring `Flying`/`Protection`.
fn real_catalogs() -> Catalogs {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
    let load = |name: &str| -> Vec<String> {
        let path = dir.join(format!("{name}.txt"));
        let file =
            File::open(&path).unwrap_or_else(|error| panic!("opening {}: {error}", path.display()));
        BufReader::new(file)
            .lines()
            .collect::<std::io::Result<Vec<_>>>()
            .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
    };
    Catalogs::default()
        .with_catalog(CatalogKind::KeywordAbility, load("keyword-abilities"))
        .with_catalog(CatalogKind::KeywordAction, load("keyword-actions"))
        .with_catalog(CatalogKind::AbilityWord, load("ability-words"))
        .with_catalog(CatalogKind::ArtifactType, load("artifact-types"))
        .with_catalog(CatalogKind::BattleType, load("battle-types"))
        .with_catalog(CatalogKind::CreatureType, load("creature-types"))
        .with_catalog(CatalogKind::EnchantmentType, load("enchantment-types"))
        .with_catalog(CatalogKind::LandType, load("land-types"))
        .with_catalog(CatalogKind::PlaneswalkerType, load("planeswalker-types"))
        .with_catalog(CatalogKind::SpellType, load("spell-types"))
        .with_catalog(CatalogKind::Supertype, load("supertypes"))
        .with_catalog(CatalogKind::CardType, load("card-types"))
}

/// Test-local only: no `kinds: [...]` -> `FragmentKind` table exists in
/// production code (confirmed absent from `deckmaste_frames`, `xtask`, and
/// `deckmaste_cards` while researching this task) — `cargo xtask macro
/// inspect` takes `--kind` on the command line instead. This heuristic only
/// needs to be right for the pilot's own kinds.
fn expected_kind(def: &MacroDef) -> FragmentKind {
    let kinds: Vec<&str> = def.kinds.iter().map(Ident::as_str).collect();
    if kinds.contains(&"KeywordAbility") {
        FragmentKind::KeywordLine
    } else if kinds.contains(&"CostComponent") {
        FragmentKind::Cost
    } else if kinds.iter().any(|kind| {
        matches!(
            *kind,
            "Predicate" | "Selection" | "TargetSpec" | "DesignationDecl"
        )
    }) {
        FragmentKind::Nominal
    } else if kinds.contains(&"Ability") {
        FragmentKind::Ability
    } else {
        FragmentKind::Sentence
    }
}

fn positional_params(def: &MacroDef) -> Vec<String> {
    match &def.params {
        Params::Positional(types) => types
            .iter()
            .map(|ty| ty.name.as_str().to_string())
            .collect(),
        Params::Named(_) => panic!(
            "`{}` has a named param signature; the pilot set is positional-only",
            def.name.as_str()
        ),
    }
}

/// Every macro `plugin` has loaded, deduplicated by `(name, kind set)` — the
/// real identity of a macro definition. A raw `MacroSet::iter()` yields one
/// `(kind, def)` pair **per kind** a multi-kind macro (`Draw`, `Draws`, …)
/// was registered under, so without this dedup a two-kind macro would be
/// visited, and its frames compiled, twice.
fn unique_defs(plugin: &Plugin) -> Vec<&MacroDef> {
    let mut seen: HashMap<(&str, Vec<&str>), &MacroDef> = HashMap::new();
    for (_, def) in plugin.macros.iter() {
        let mut kinds: Vec<&str> = def.kinds.iter().map(Ident::as_str).collect();
        kinds.sort_unstable();
        seen.insert((def.name.as_str(), kinds), def);
    }
    seen.into_values().collect()
}

/// The pilot's seven framed macro names — asserted explicitly (not just
/// counted) so a name typo or an accidental drop shows up as a named
/// failure rather than a silent count coincidence.
const EXPECTED_FRAMED_MACROS: [&str; 7] = [
    "Flying",
    "Protection",
    "DealsDamageToEach",
    "Creature",
    "Draw",
    "Draws",
    "PumpThisUntilEot",
];

#[test]
fn every_framed_macro_and_constructor_entry_compiles_clean() {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir())
        .unwrap_or_else(|error| panic!("loading plugin: {error:#}"));
    let catalogs = real_catalogs();

    let mut framed_names: Vec<&str> = Vec::new();
    for def in unique_defs(&plugin) {
        if def.frames().is_empty() {
            continue;
        }
        framed_names.push(def.name.as_str());
        let params = positional_params(def);
        let kind = expected_kind(def);
        for (index, spec) in def.frames().iter().enumerate() {
            deckmaste_frames::compile(spec, kind, &params, &catalogs, &plugin.macros)
                .unwrap_or_else(|error| {
                    panic!(
                        "`{}` frame [{index}] ({:?}) at {kind:?}: {error:#}",
                        def.name.as_str(),
                        spec.text,
                    )
                });
        }
    }
    framed_names.sort_unstable();
    let mut expected = EXPECTED_FRAMED_MACROS.to_vec();
    expected.sort_unstable();
    assert_eq!(
        framed_names, expected,
        "the pilot's framed macro set changed; update EXPECTED_FRAMED_MACROS \
         (or investigate why one dropped out)"
    );
    assert!(
        framed_names.len() >= 7,
        "brief requires >=7 framed macros: {framed_names:?}"
    );

    let catalog = load_constructor_frames(&plugin_dir().join("frames"))
        .unwrap_or_else(|error| panic!("loading constructor frame catalog: {error:#}"));
    let mut constructor_names: Vec<&str> = Vec::new();
    for entry in &catalog {
        if entry.frames.is_empty() {
            continue;
        }
        constructor_names.push(entry.constructor.as_str());
        let kind = if entry.constructor == "Target" {
            FragmentKind::Nominal
        } else {
            FragmentKind::Sentence
        };
        for (index, spec) in entry.frames.iter().enumerate() {
            deckmaste_frames::compile(spec, kind, &entry.params, &catalogs, &plugin.macros)
                .unwrap_or_else(|error| {
                    panic!(
                        "constructor `{}` frame [{index}] ({:?}) at {kind:?}: {error:#}",
                        entry.constructor, spec.text,
                    )
                });
        }
    }
    constructor_names.sort_unstable();
    assert_eq!(
        constructor_names,
        vec!["DealDamage", "GainLife", "Target"],
        "constructor catalog entry set changed"
    );
}

/// Documents (and pins) the two intentionally-unframed pilot additions: both
/// macros are loaded and real, but neither carries a `frames:` field. See
/// the G5 report and the two tests below for why each was left unframed
/// rather than forced.
#[test]
fn the_two_g5_negative_findings_are_loaded_but_unframed() {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir())
        .unwrap_or_else(|error| panic!("loading plugin: {error:#}"));
    for name in ["ControlledByYou", "SacrificeThis"] {
        let def = unique_defs(&plugin)
            .into_iter()
            .find(|def| def.name.as_str() == name)
            .unwrap_or_else(|| panic!("`{name}` must be loaded from the pilot's macro files"));
        assert!(
            def.frames().is_empty(),
            "`{name}` is expected to stay unframed (see the G5 report); \
             found {:?} — update this test if that changed on purpose",
            def.frames()
        );
    }
}

/// The G5 finding this test was minted to lock in has been **resolved**, by
/// the user ruling that reopened Task 4: `<Param(0)> you control` — the frame
/// directed for `ControlledByYou`, exercising `FieldSlice` from the side
/// `target <Param(0)>` doesn't reach — now compiles, with the hole claiming
/// `modifiers`+`head` and the frame keeping its `complements`.
///
/// The test is inverted rather than deleted, and this is what its previous
/// self asked for: it recorded that if the shape ever compiled, "the compiler
/// grew the complementary shape and the G5 report needs updating, not this
/// test loosening". The compiler did grow it, so the assertion now pins the
/// *positive* outcome. Still driven against the macro's own real, loaded
/// `params` rather than a hand-rolled fixture, so it fails loudly if
/// `ControlledByYou.ron`'s signature changes out from under it.
///
/// Note `ControlledByYou` still carries no `frames:` field — authoring it is
/// the lexicon's call, not the compiler's, and
/// `the_two_g5_negative_findings_are_loaded_but_unframed` above still holds.
#[test]
fn controlled_by_you_complement_side_field_slice_compiles() {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir())
        .unwrap_or_else(|error| panic!("loading plugin: {error:#}"));
    let def = unique_defs(&plugin)
        .into_iter()
        .find(|def| def.name.as_str() == "ControlledByYou")
        .expect("ControlledByYou must be loaded");
    let params = positional_params(def);

    let frame = deckmaste_frames::compile(
        &FrameSpec::bare("<Param(0)> you control"),
        FragmentKind::Nominal,
        &params,
        &real_catalogs(),
        &plugin.macros,
    )
    .unwrap_or_else(|error| {
        panic!("`<Param(0)> you control` is a real constituent and must compile: {error:#}")
    });

    assert_eq!(frame.holes.len(), 1);
    assert_eq!(
        frame.holes[0].class,
        deckmaste_frames::HoleClass::FieldSlice {
            claimed: vec!["modifiers", "head"],
        },
        "the frame owns the postmodifier; the hole is the premodifiers and head"
    );
}

/// G5 finding, locked in: `SacrificeThis`'s attempted `~` frame compiles —
/// `~` is syntactically valid self-reference — but produces a `SelfRef`
/// hole, which the round's self-reference machinery (`NounPhrase::ThisCard`,
/// `ThisCardForm::{FullName, AbbreviatedName}`) renders as the card's own
/// printed NAME, never the type-generic "this permanent"/"this creature"
/// wording the checked-in `template:` actually uses. This test pins the
/// *positive* half (it compiles, and the hole really is `SelfRef`); the G5
/// report carries the render-side evidence (`ThisCardForm`'s two variants,
/// the legacy `~ => subject` renderer, and the live Clue/Gold/Treasure/
/// Food/Blood token regression risk) that the wording itself is wrong.
#[test]
fn sacrifice_this_self_reference_sigil_compiles_as_self_ref() {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir())
        .unwrap_or_else(|error| panic!("loading plugin: {error:#}"));
    let def = unique_defs(&plugin)
        .into_iter()
        .find(|def| def.name.as_str() == "SacrificeThis")
        .expect("SacrificeThis must be loaded");
    assert_eq!(
        def.template(),
        Some("Sacrifice this permanent"),
        "this test must not be the thing that quietly changed the checked-in \
         template — that would silently alter live rendering of the Clue/Gold/\
         Treasure/Food/Blood tokens, which all invoke this macro"
    );
    let params = positional_params(def);

    let frame = deckmaste_frames::compile(
        &FrameSpec::bare("Sacrifice ~"),
        FragmentKind::Cost,
        &params,
        &real_catalogs(),
        &plugin.macros,
    )
    .unwrap_or_else(|error| panic!("`Sacrifice ~` was expected to compile cleanly: {error:#}"));
    assert_eq!(frame.holes.len(), 1);
    assert_eq!(frame.holes[0].class, HoleClass::SelfRef);
}
