//! Task 6 integration test: every loaded macro def with a non-empty
//! `frames:` list, plus every constructor-catalog entry, compiles clean
//! (G1) against `plugins/builtin`. Also locks in two G5 findings as
//! regression tests, so a later change to the compiler's behavior on either
//! shape shows up here rather than only in prose:
//!
//! - `ControlledByYou`'s complement-side `FieldSlice` frame — originally a
//!   *negative* finding (`<Param(0)> you control` was refused by `classify()`),
//!   escalated, and **resolved** in-round by a Task 4 reopen that generalized
//!   `classify()`'s field-slice decision. The macro is now framed, and
//!   `controlled_by_you_complement_side_field_slice_compiles` pins the positive
//!   outcome.
//! - `SacrificeThis`'s attempted `~` frame — still a genuine negative finding:
//!   `~` compiles (a `SelfRef` hole) but is the wrong self-reference tool for
//!   this position. The macro is now framed with the *literal* wording instead
//!   (`"Sacrifice this permanent"`, which does parse and does match the
//!   checked-in template), so `SacrificeThis` is no longer unframed — but
//!   `sacrifice_this_self_reference_sigil_ compiles_as_self_ref` still pins
//!   that the `~` alternative compiles to the wrong AST shape, which is why it
//!   wasn't adopted.
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

/// The pilot's ten framed macro names — asserted explicitly (not just
/// counted) so a name typo or an accidental drop shows up as a named
/// failure rather than a silent count coincidence. `ControlledByYou` and
/// `SacrificeThis` joined this list in the fix round (see the module doc):
/// both were originally left unframed as G5 findings, and both are now
/// framed for different reasons (the compiler gap was fixed; the literal
/// wording turned out to be a real, reproducible constituent all along).
/// `Player` joined in Task 8's own fix round: it already existed
/// (`kinds: [Predicate]`, used elsewhere) but had no `frames:`, exactly
/// `Creature`'s pre-Task-6 gap — closed the same way (see
/// `docs/superpowers/research/2026-07-30-macro-frames/
/// pilot-constituency-findings.md`'s "Task 8, fix round 1" section).
const EXPECTED_FRAMED_MACROS: [&str; 10] = [
    "Flying",
    "Protection",
    "DealsDamageToEach",
    "Creature",
    "Draw",
    "Draws",
    "PumpThisUntilEot",
    "ControlledByYou",
    "SacrificeThis",
    "Player",
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
        let kind = deckmaste_frames::lexicon::macro_fragment_kind(def);
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
        // This test only needs *a* category for the frame to compile clean,
        // not the catalog's own resolution of `entry.kind` to
        // `FragmentKind` (that conversion is `deckmaste_frames::lexicon`
        // internal); picking by name keeps this test independent of it.
        let kind = if matches!(
            entry.constructor.as_str(),
            "Target" | "This" | "You" | "AnyTarget"
        ) {
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
        // The two nullary pro-forms (`This`, `You`) let a card's own subject
        // or the pronoun "you" recover as a RON constant instead of an
        // unmatched residual; `AnyTarget` is the announce-list pro-form and
        // `TargetedDealDamage` its accompanying body entry. All compile
        // clean here like any other entry, which is this test's job; the
        // matching side is covered in `deckmaste_frames::unify`'s suite.
        vec![
            "AnyTarget",
            "DealDamage",
            "GainLife",
            "Target",
            "TargetedDealDamage",
            "This",
            "You",
        ],
        "constructor catalog entry set changed"
    );
}

/// Pins the exact resolved wording for the two macros the fix round framed:
/// both started this task as G5 negative findings ("cannot be framed as
/// directed"), and both ended up framed anyway, for two different reasons —
/// `ControlledByYou` because the compiler gap it found got fixed;
/// `SacrificeThis` because the literal wording (as opposed to the `~`
/// alternative that motivated leaving it unframed in the first place) turned
/// out to be a real, reproducible constituent. This test is what replaces
/// the old "both are loaded but unframed" pin now that neither is unframed
/// any more — it still asserts something real: each macro has exactly one
/// frame, and it's the exact text this report's story depends on.
#[test]
fn controlled_by_you_and_sacrifice_this_carry_their_resolved_wording() {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir())
        .unwrap_or_else(|error| panic!("loading plugin: {error:#}"));
    for (name, expected_text) in [
        ("ControlledByYou", "<Param(0)> you control"),
        ("SacrificeThis", "Sacrifice this permanent"),
    ] {
        let def = unique_defs(&plugin)
            .into_iter()
            .find(|def| def.name.as_str() == name)
            .unwrap_or_else(|| panic!("`{name}` must be loaded from the pilot's macro files"));
        assert_eq!(
            def.frames().len(),
            1,
            "`{name}` is expected to carry exactly one frame; found {:?}",
            def.frames()
        );
        assert_eq!(
            def.frames()[0].text,
            expected_text,
            "`{name}`'s frame text changed out from under this pin"
        );
        assert!(
            def.frames()[0].when.is_empty() && def.frames()[0].position.is_none(),
            "`{name}`'s single frame is expected to be unguarded: {:?}",
            def.frames()[0]
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
/// `ControlledByYou` is now framed with exactly this text (see
/// `controlled_by_you_and_sacrifice_this_carry_their_resolved_wording`
/// above); this test independently re-derives the compile from the macro's
/// real params rather than trusting `def.frames()[0]`, so it still catches a
/// regression in `classify()` itself, not just in the `.ron` file's content.
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
///
/// `SacrificeThis` ended up framed anyway — with the *literal* wording
/// `"Sacrifice this permanent"` (see
/// `controlled_by_you_and_sacrifice_this_carry_their_resolved_wording`),
/// which is a real, if narrow (12/1575 corpus attestations), constituent —
/// so this test's role narrowed to exactly what it says: `~` compiles, to
/// the wrong shape, which is why it wasn't the frame adopted.
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
