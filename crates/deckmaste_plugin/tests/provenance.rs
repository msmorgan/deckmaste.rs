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

fn builtin() -> Plugin {
    Plugin::load(workspace_plugin("builtin")).expect("load builtin")
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

/// Each `GainAbility` node's payload, as the text `Debug` prints for it: the
/// remainder of the rendering that starts immediately after `GainAbility(`.
///
/// The payload's own `Debug` is written verbatim there, and no proper prefix of
/// a balanced rendering is itself balanced — so an ability whose `Debug` is a
/// prefix of one of these tails IS that payload, not merely something like it.
fn debug_grant_payloads(debug: &str) -> Vec<&str> {
    debug
        .match_indices("GainAbility(")
        .map(|(at, hit)| &debug[at + hit.len()..])
        .collect()
}

/// The first `n` chars of `s`, for a panic message on a value whose `Debug`
/// may be enormous.
fn head(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// The completeness oracle.
///
/// `GainAbility` is the layer-6 grant position ([CR#613.1f]) and the one the
/// index exists to catch. A `Debug` rendering of the whole card is a check the
/// walker cannot cheat: it prints the entire tree and knows nothing about which
/// grammar positions the walker was taught.
///
/// The comparison is payload-for-payload, not count-for-count: EVERY
/// `GainAbility` payload in the `Debug` rendering has to be matched, one for
/// one, against a distinct ability the walker actually reached. A missed
/// grammar position leaves its payload with nothing to match and fails here —
/// where merely comparing totals would let the card's own printed abilities pay
/// for a grant the walker never saw.
#[test]
fn walker_reaches_every_gain_ability_in_the_canon_corpus() {
    let plugin = canon();
    let mut cards = 0usize;
    let mut grants = 0usize;
    for (path, source) in card_sources(&workspace_plugin("canon")) {
        let Ok(loaded) = plugin.card_from_str(&source) else { continue };
        let debug = format!("{:?}", loaded.authored);
        let mut reached: Vec<String> = all_abilities(&loaded.authored)
            .iter()
            .map(|a| format!("{a:?}"))
            .collect();
        for payload in debug_grant_payloads(&debug) {
            let Some(at) = reached.iter().position(|a| payload.starts_with(a.as_str())) else {
                panic!(
                    "{}: a GainAbility payload the walker never reached — a grammar \
                     position is missing from AbilitySubterms: {}…",
                    path.display(),
                    head(payload, 160),
                );
            };
            reached.swap_remove(at);
            grants += 1;
        }
        cards += 1;
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

/// Keyword-counter conferral ([CR#122.1b]) is indexed in the form the ENGINE
/// builds. A counter reaches layer 6 by a different path than a subtype: the
/// layer pass folds `Property::Continuous(This, GainAbility(a))` out of the
/// counter registry and pushes `a` VERBATIM ([CR#613.1f]) — no `Innate`
/// wrapper. The keys checked here are read straight off the LOWERED registry
/// the engine is handed, so they are the engine's own values, not a
/// re-derivation of them.
#[test]
fn counter_conferral_is_indexed_as_the_bare_grant_payload() {
    let plugin = builtin();
    let mut grants = 0usize;
    for counter in plugin.counters.values() {
        for property in &counter.confers {
            let deckmaste_core::Property::Continuous(_, change) = property else {
                continue;
            };
            for m in deckmaste_core::Modification::flatten(std::slice::from_ref(change)).iter() {
                let deckmaste_core::Modification::GainAbility(granted) = m else {
                    continue;
                };
                assert!(
                    plugin.provenance.ability(granted).is_some(),
                    "the ability `{}` grants is not indexed",
                    counter.name,
                );
                grants += 1;
            }
        }
    }
    assert!(
        grants > 0,
        "no builtin counter confers an ability — the fixture is gone",
    );
}

/// A token's abilities are indexed. `Cards::push_token` mints a `CardId` past
/// the end of the card companion table, so a token permanent has no authored
/// card to fall back on and this index is its only prose channel.
///
/// Both sources: the plugin's own `tokens/` files, and the predefined tokens
/// ([CR#111.10]) a `TokenSpec::Named` resolves to, which are built in code and
/// appear in no plugin file at all.
#[test]
fn token_abilities_are_indexed() {
    let plugin = builtin();
    let treasure = plugin.token("Treasure").expect("builtin Treasure token");
    assert!(
        !treasure.core.abilities.is_empty(),
        "the Treasure token has an activated ability to recover",
    );
    for ability in treasure.core.abilities.iter() {
        assert!(
            plugin.provenance.ability(ability).is_some(),
            "a `tokens/` token ability is not indexed",
        );
    }

    let mut checked = 0usize;
    for predefined in deckmaste_authoring::PredefinedToken::ALL {
        for ability in predefined.token().abilities.iter() {
            let core = ability.clone().lower();
            assert!(
                plugin.provenance.ability(&core).is_some(),
                "predefined token `{}` has an unindexed ability",
                predefined.name(),
            );
            checked += 1;
        }
    }
    assert!(checked > 0, "no predefined token carries an ability");
}

/// The walker reaches an INLINE token's abilities through the `Create`
/// instruction that mints it — the position `AbilitySubterms for Action`
/// wildcarded away, leaving every such ability unreachable and so unindexed.
///
/// No canon card exercises it, so the fixture is synthetic: `Afterlife`
/// ([CR#702.135a]) is the builtin keyword whose body is `Create(… token:
/// Token(abilities: [Keyword(Flying)] …))`, and that flying exists ONLY inside
/// the `Create`. It is not printed on the host card, so nothing but the
/// `Create` traversal can put it in the index.
#[test]
fn indexes_an_inline_created_tokens_abilities() {
    let plugin = builtin();
    let source = r#"Normal(
      name: "Provenance Fixture",
      mana_cost: [White],
      types: [Creature],
      abilities: [Keyword(Afterlife(1))],
      power: 1,
      toughness: 1,
    )"#;
    let loaded = plugin.card_from_str(source).expect("load the fixture card");
    let mut index = ProvenanceIndex::default();
    index.insert_card(&loaded.authored);

    let reached = all_abilities(&loaded.authored);
    let flying = reached
        .iter()
        .find(|a| format!("{a:?}").contains("Flying"))
        .unwrap_or_else(|| {
            panic!("the walker never reached the created token's flying; got {reached:#?}")
        });
    assert!(
        index.ability(&flying.clone().lower()).is_some(),
        "the created token's flying is not indexed",
    );
}
