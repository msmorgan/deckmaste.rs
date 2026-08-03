//! The provenance index and the walker it rides on.
//!
//! The walker (`deckmaste_authoring::AbilitySubterms`) is hand-written and so
//! incomplete by construction — a grammar position nobody taught it about is
//! silently not traversed, and the only symptom in production is prose quietly
//! degrading to `[unrendered]`. These tests are the compensating control: they
//! run the walker over the real corpus and check it against a count derived a
//! different way, so a missed position fails here instead of shipping.

use std::path::Path;
use std::path::PathBuf;

use deckmaste_authoring::AbilitySubterms as _;
use deckmaste_lowering::Lower as _;
use deckmaste_plugin::layout::CARDS_DIR;
use deckmaste_plugin::plugin::Plugin;
use deckmaste_plugin::plugin::read;
use deckmaste_plugin::plugin::ron_files_recursive;
use deckmaste_plugin::provenance::ProvenanceIndex;

fn workspace_plugin(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../plugins")
        .join(name)
}

fn canon() -> Plugin {
    Plugin::load_with_sibling_prelude(workspace_plugin("canon")).expect("load canon")
}

/// Every ability the walker reaches from `card`, transitively.
fn all_abilities(card: &deckmaste_authoring::Card) -> Vec<deckmaste_authoring::Ability> {
    let mut top = Vec::new();
    card.push_abilities(&mut top);
    let mut out: Vec<deckmaste_authoring::Ability> = Vec::new();
    let mut queue: Vec<deckmaste_authoring::Ability> = top.into_iter().cloned().collect();
    while let Some(a) = queue.pop() {
        queue.extend(a.nested_abilities().into_iter().cloned());
        out.push(a);
    }
    out
}

/// Card sources under `plugin/cards`, skipping `.todo.ron` stubs.
fn card_sources(root: &Path) -> Vec<(PathBuf, String)> {
    ron_files_recursive(&root.join(CARDS_DIR))
        .expect("walk cards dir")
        .into_iter()
        .filter(|p| !p.to_string_lossy().ends_with(".todo.ron"))
        .map(|p| {
            let s = read(&p).expect("read card");
            (p, s)
        })
        .collect()
}

/// The completeness oracle.
///
/// `GainAbility` is the layer-6 grant position ([CR#613.1f]) and the one the
/// index exists to catch. Counting its occurrences in a `Debug` rendering of
/// the whole card is a check the walker cannot cheat: `Debug` prints the entire
/// tree and knows nothing about which grammar positions the walker was taught.
/// Each occurrence contributes exactly one nested ability, so the walker's
/// transitive count can never be the smaller of the two unless it missed a
/// position.
#[test]
fn walker_reaches_every_gain_ability_in_the_canon_corpus() {
    let plugin = canon();
    let mut cards = 0usize;
    let mut grants = 0usize;
    for (path, source) in card_sources(&workspace_plugin("canon")) {
        let Ok(loaded) = plugin.card_from_str(&source) else { continue };
        let debug_grants = format!("{:?}", loaded.authored)
            .matches("GainAbility")
            .count();
        let walked = all_abilities(&loaded.authored).len();
        assert!(
            walked >= debug_grants,
            "{}: {debug_grants} GainAbility nodes in the tree but the walker \
             reached only {walked} abilities — a grammar position is missing \
             from AbilitySubterms",
            path.display(),
        );
        cards += 1;
        grants += debug_grants;
    }
    assert!(cards > 0, "no canon cards loaded");
    assert!(
        grants > 0,
        "corpus exercised no GainAbility position at all"
    );
}

/// The property the renderer depends on: it holds a core ability and needs the
/// authored term back.
#[test]
fn indexes_every_canon_ability_by_its_lowered_value() {
    let plugin = canon();
    let mut checked = 0usize;
    for (path, source) in card_sources(&workspace_plugin("canon")) {
        let Ok(loaded) = plugin.card_from_str(&source) else { continue };
        let mut index = ProvenanceIndex::default();
        index.insert_card(&loaded.authored);
        for authored in all_abilities(&loaded.authored) {
            let core = authored.clone().lower();
            assert!(
                index.ability(&core).is_some(),
                "{}: an ability's lowering has no provenance entry",
                path.display(),
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "corpus produced no abilities to check");
}

/// A layer-6 grant resolves. This is the index's whole reason to exist: the
/// engine pushes a verbatim clone of the `GainAbility` payload onto the object,
/// so the payload has to be a key in its own right, not just reachable.
#[test]
fn a_granted_ability_resolves_through_the_index() {
    let plugin = canon();
    let source = read(&plugin.card_path("Collective Resistance")).expect("read card");
    let loaded = plugin.card_from_str(&source).expect("load card");
    let mut index = ProvenanceIndex::default();
    index.insert_card(&loaded.authored);

    // Named explicitly so this test cannot pass on the card's OWN keywords.
    // The card grants hexproof and indestructible through
    // `Until(_, [Modify(_, Several([GainAbility(..), GainAbility(..)]))])` —
    // four grammar levels below the ability, which is exactly the depth the
    // walker has to survive.
    let nested = all_abilities(&loaded.authored);
    for want in ["Hexproof", "Indestructible"] {
        let found = nested
            .iter()
            .find(|a| format!("{a:?}").contains(want))
            .unwrap_or_else(|| {
                panic!("walker never reached the granted `{want}`; got {nested:#?}")
            });
        assert!(
            index.ability(&found.clone().lower()).is_some(),
            "the granted `{want}` is not indexed",
        );
    }
}

/// The outermost spelling wins. An `Expanded` wrapper and its own `value` lower
/// to the same core ability once the `Expansion` arms erase, and the wrapper is
/// the half carrying the invocation template — so a post-order insert would
/// silently keep the bare form and lose the good prose.
#[test]
fn keeps_the_outermost_spelling_when_two_forms_collide() {
    let plugin = canon();
    let source = read(&plugin.card_path("Serra's Blessing")).expect("read card");
    let loaded = plugin.card_from_str(&source).expect("load card");
    let mut index = ProvenanceIndex::default();
    index.insert_card(&loaded.authored);

    let mut top = Vec::new();
    loaded.authored.push_abilities(&mut top);
    for authored in top {
        if !matches!(authored, deckmaste_authoring::Ability::Expanded(_)) {
            continue;
        }
        let recovered = index
            .ability(&authored.clone().lower())
            .expect("indexed ability");
        assert!(
            matches!(recovered, deckmaste_authoring::Ability::Expanded(_)),
            "the index kept a bare form; the template-carrying wrapper was overwritten",
        );
    }
}

/// Registry conferral is indexed in the form the ENGINE builds — wrapped in
/// `Innate` by `Property::conferred_ability` — not as the raw payload.
#[test]
fn registry_conferral_is_indexed_in_its_conferred_form() {
    let plugin = canon();
    let mut conferred = 0usize;
    for subtype in plugin.subtypes.values() {
        for property in subtype.confers.iter() {
            let Some(ability) = property.conferred_ability() else { continue };
            assert!(
                plugin.provenance.ability(&ability).is_some(),
                "conferred ability for subtype `{}` is not indexed",
                subtype.name,
            );
            conferred += 1;
        }
    }
    assert!(conferred > 0, "no subtype in canon confers an ability");
}
