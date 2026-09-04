//! Corpus gates for the semantic-to-core boundary. Every authored file loads
//! through [`Plugin`] and lowers successfully, including the generated card
//! corpus when present. Rules-table sweeps are also checked against the tables
//! retained by the plugin loader.
//!
//! This module formerly compared lowered values with a second, core-kinded
//! parse of the same authored RON. That oracle was valid only while the two
//! grammars were shape-identical. Runnable costs deliberately end that era:
//! authored `Do`/`With` lower to core `Act`/`ChooseAndPay`, and core must not
//! accept the author-facing spellings. Variant-level lowering tests are the
//! divergence ledger; these corpus tests retain the non-tautological coverage
//! that every real artifact parses and lowers through the production path.
//!
//! The nineteen `core-regions-substrate` witness cards were once swept here
//! too, by a test that stripped each `.ron.todo`'s `Unparsed` line before
//! loading — which left fourteen of them contributing no region-bearing
//! ability at all. They are now per-card fixtures in
//! `deckmaste_engine/tests/region_witnesses.rs`, which spells each witnessed
//! ability by hand and runs it.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_lowering::Lower;
use deckmaste_plugin::layout::CARDS_DIR;
use deckmaste_plugin::macros::MacroSet;
use deckmaste_plugin::plugin::Plugin;
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

struct Swept {
    checked: usize,
    unparsed: usize,
    first_error: Option<String>,
}

/// One card through the production semantic reader and lowering boundary.
fn load_card(plugin: &Plugin, source: &str) -> anyhow::Result<deckmaste_card::Card> {
    Ok(plugin.card_from_str(source)?.core)
}

/// The token twin of [`load_card`].
fn load_token(plugin: &Plugin, source: &str) -> anyhow::Result<deckmaste_core::Token> {
    Ok(plugin.token_from_str(source)?.core)
}

/// One rules-table file, read and lowered exactly as `Plugin`'s
/// `load_sba_rules`/`load_conferral_rules`/`load_damage_result_rules` do it —
/// through the plugin's own macro scope, at semantics kinds, lowering at the
/// boundary. Those loaders expose only the concatenated result, so a per-file
/// sweep has to repeat their two lines; the tables the loader itself built are
/// checked against this sweep's output in
/// [`builtin_rules_tables_lower`].
fn load_rules<A>(macros: &MacroSet, source: &str) -> anyhow::Result<A::Target>
where
    A: DeserializeOwned + Lower,
{
    let semantic: A = macros.read_str(source)?;
    Ok(semantic.lower())
}

/// A file that fails to LOAD is counted, not fatal; everything past the load is
/// always fatal. Every caller asserts the count is zero — see
/// [`wizards_cards_lower`] for why the count exists at all.
fn sweep<C>(dir: &Path, load: impl Fn(&str) -> anyhow::Result<C>) -> (Swept, Vec<C>) {
    let mut swept = Swept {
        checked: 0,
        unparsed: 0,
        first_error: None,
    };
    let mut loaded_all = Vec::new();
    for path in ron_files(dir) {
        let source = fs::read_to_string(&path).expect("corpus file is readable");
        let core = match load(&source) {
            Ok(core) => core,
            Err(error) => {
                swept.unparsed += 1;
                swept
                    .first_error
                    .get_or_insert_with(|| format!("{}: {error:#}", path.display()));
                continue;
            }
        };

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
fn sweep_strict<C>(dir: &Path, load: impl Fn(&str) -> anyhow::Result<C>) -> Vec<C> {
    let (swept, loaded) = sweep(dir, load);
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

/// Each corpus is swept in the macro scope used by its production loader.
#[test]
fn canon_and_builtin_cards_lower() {
    let builtin = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/builtin"))
        .expect("builtin loads");
    let builtin_cards = sweep_strict(&plugin_dir("builtin", "cards"), |source| {
        load_card(&builtin, source)
    });

    let canon = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/canon"))
        .expect("canon loads over the builtin prelude");
    let canon_cards = sweep_strict(&plugin_dir("canon", "cards"), |source| {
        load_card(&canon, source)
    });

    assert!(
        !builtin_cards.is_empty() && !canon_cards.is_empty(),
        "no cards found to check"
    );
}

#[test]
fn builtin_tokens_lower() {
    let builtin = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/builtin"))
        .expect("builtin loads");
    let tokens = sweep_strict(&plugin_dir("builtin", "tokens"), |source| {
        load_token(&builtin, source)
    });
    assert!(!tokens.is_empty(), "no tokens found to check");
}

/// The `rules/` tables are semantic containers too (spec §4), and the loader
/// lowers them at load. Each table is swept per file, then the concatenation is
/// checked against the table `Plugin` itself built — so this is a statement
/// about the loader, not about a re-implementation of it.
#[test]
fn builtin_rules_tables_lower() {
    let builtin = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/builtin"))
        .expect("builtin loads");
    let rules = plugin_dir("builtin", "rules");

    let sba = sweep_strict(&rules.join("sba"), |source| {
        load_rules::<Vec<deckmaste_semantics::SbaRule>>(&builtin.macros, source)
    });
    let grant = sweep_strict(&rules.join("grant"), |source| {
        load_rules::<Vec<deckmaste_semantics::ConferralRule>>(&builtin.macros, source)
    });
    let damage = sweep_strict(&rules.join("damage"), |source| {
        load_rules::<Vec<deckmaste_semantics::DamageResultRule>>(&builtin.macros, source)
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
fn wizards_cards_lower() {
    let wizards = Plugin::load_with_sibling_prelude(workspace_root().join("plugins/wizards"))
        .expect("wizards loads over the builtin prelude");
    let dir = plugin_dir("wizards", "cards");
    let (swept, _) = sweep(&dir, |source| load_card(&wizards, source));

    let total = swept.checked + swept.unparsed;
    println!("wizards: {} of {total} cards lowered", swept.checked);
    assert!(total > 0, "no wizards cards found at all");
    assert_eq!(
        swept.unparsed,
        0,
        "{} of {total} wizards card(s) did not load; first: {}",
        swept.unparsed,
        swept.first_error.as_deref().unwrap_or("—"),
    );
}

/// Every face of a semantic card, front-to-back. `deckmaste_semantics::Card`
/// has no public accessor for this (its two variants are the whole public
/// surface), so — like the same-shaped private helpers in
/// `deckmaste_legacy_render::fidelity` and `xtask::macros::pilot` — each
/// caller that needs "every face" writes the two-arm match once.
fn semantic_faces(card: &deckmaste_semantics::Card) -> Vec<&deckmaste_semantics::CardFace> {
    match card {
        deckmaste_semantics::Card::Normal(face) => vec![face],
        deckmaste_semantics::Card::TwoFaced { front, back, .. } => vec![front, back],
    }
}

/// One semantic source file, loaded through the real plugin API, keeping the
/// semantic/core pair intact — the pair [`load_card`] already reads, minus
/// the write-back re-serialization
/// [`every_top_level_semantic_ability_appears_in_its_lowered_card`] does not
/// need.
fn load_card_pair(plugin: &Plugin, path: &Path) -> anyhow::Result<deckmaste_plugin::LoadedCard> {
    let source = fs::read_to_string(path)?;
    plugin.card_from_str(&source)
}

/// Every top-level [`deckmaste_core::Ability`] on a lowered card's faces —
/// the core-side twin of [`semantic_faces`], stopping at the face's own
/// `abilities` list. [`nested_in_any`] does the descent past that point.
fn core_abilities(card: &deckmaste_card::Card) -> impl Iterator<Item = &deckmaste_core::Ability> {
    let parts: Vec<&deckmaste_card::Characteristics> = match card {
        deckmaste_card::Card::Normal(face) => vec![&face.characteristics],
        deckmaste_card::Card::DoubleFaced { front, back, .. }
        | deckmaste_card::Card::Split {
            left: front,
            right: back,
        } => vec![&front.characteristics, &back.characteristics],
        deckmaste_card::Card::Flip {
            normal,
            alternative,
        }
        | deckmaste_card::Card::Adventurer {
            normal,
            adventure: alternative,
        } => vec![&normal.characteristics, alternative],
    };
    parts.into_iter().flat_map(|part| part.abilities.iter())
}

/// The core-side twin of [`deckmaste_semantics::AbilitySubterms`]: pushes
/// every `deckmaste_core::Ability` reachable from a value, stopping AT each
/// one. Hand-written for the same reason the semantics trait is (no blanket
/// `impl<T> Trait for T` on stable Rust) and covering the same ability-
/// bearing positions, for the same reason: this test only means what it
/// claims to mean if the two walkers — the one deciding which semantic
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

impl<T: CoreAbilitySubterms> CoreAbilitySubterms for deckmaste_core::Region<T> {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        self.body.push_abilities(out);
    }
}

impl CoreAbilitySubterms for deckmaste_core::Block {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        self.0.push_abilities(out);
    }
}

impl CoreAbilitySubterms for deckmaste_core::KeywordAbility {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::Composite { abilities, .. } => abilities.push_abilities(out),
            Self::FirstStrike
            | Self::DoubleStrike
            | Self::Deathtouch
            | Self::Trample
            | Self::Vigilance => {}
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::StaticSpec {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::Modify(_, m) => m.push_abilities(out),
            Self::Each(_, e) => e.push_abilities(out),
            Self::Conditionally(_, e) => e.push_abilities(out),
            // A copy delivery site ([CR#707.4]); mirrors the semantics side.
            Self::BecomesCopy(_, spec) => spec.push_abilities(out),
            // No other `StaticSpec` shape carries an `Ability` — same
            // catch-all the semantics twin uses, for the same reason.
            _ => {}
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::Modification {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        // Exhaustive on purpose, mirroring the semantics side: this is THE
        // layer-6 grant position ([CR#613.1f]).
        match self {
            Self::GainAbility(a) => a.push_abilities(out),
            Self::Several(ms) => ms.push_abilities(out),
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

impl CoreAbilitySubterms for deckmaste_core::Instruction {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::Act { action, .. } => action.push_abilities(out),
            Self::Sequentially(es) | Self::Simultaneously(es) => es.push_abilities(out),
            Self::Continuously(c) => c.effect.push_abilities(out),
            Self::Until(_, es) => es.push_abilities(out),
            Self::Choose(_) | Self::ChooseValue(_) | Self::Let(_) | Self::Remember(_) => {}
            Self::Search(search) => search.if_none.push_abilities(out),
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
            Self::Each(e) => e.body.push_abilities(out),
            Self::Distribute(d) => d.body.push_abilities(out),
            Self::Delayed(t) | Self::Reflexive(t) => t.effect.push_abilities(out),
            Self::Modal(m) => {
                for mode in m.modes.iter() {
                    mode.effect.push_abilities(out);
                }
            }
            Self::Repeat(_, e) | Self::Batch(_, e) => e.push_abilities(out),
            Self::RevealUntil(r) => r.body.push_abilities(out),
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::TokenSpec {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::Token(t) => t.abilities.push_abilities(out),
            // Mirrors the semantics side: a predefined token's abilities are
            // built on demand.
            Self::Named(_) => {}
            // A copy token's COPIABLE characteristics come from the copied
            // object, but a copy EXCEPTION can carry a `GainAbility` payload
            // ([CR#707.9a]) that lives only inside this `CopySpec`; mirrors
            // the semantics side.
            Self::Copy(spec) => spec.push_abilities(out),
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::CopySpec {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        self.exceptions.push_abilities(out);
    }
}

impl CoreAbilitySubterms for deckmaste_core::CopyException {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        // Exhaustive, mirroring the semantics side: `Modify` is the
        // [CR#707.9a] "except it has [ability]" clause.
        match self {
            Self::Modify(m) => m.push_abilities(out),
            Self::AdditionalEffect(rider) => rider.push_abilities(out),
            Self::Retain(_) => {}
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::EnterRider {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            // A copy delivery site ([CR#707.5]); mirrors the semantics side.
            Self::AsCopy(spec) => spec.push_abilities(out),
            Self::Tapped
            | Self::FaceDown
            | Self::UnderControlOf(_)
            | Self::UnderOwnersControl
            | Self::Attacking(_)
            | Self::WithCounters(..) => {}
        }
    }
}

impl CoreAbilitySubterms for deckmaste_core::Action {
    fn push_abilities<'a>(&'a self, out: &mut Vec<&'a deckmaste_core::Ability>) {
        match self {
            Self::GetEmblem(_, abilities) => abilities.push_abilities(out),
            // `riders` can carry `EnterRider::AsCopy`, a copy delivery site;
            // mirrors the semantics side.
            Self::Create { token, riders, .. } => {
                token.push_abilities(out);
                riders.push_abilities(out);
            }
            // A copy delivery site ([CR#707.12]); mirrors the semantics side.
            Self::CastCopy(_, spec) => spec.push_abilities(out),
            // As with `StaticSpec`: dozens of variants, few ability-
            // bearing, and the list churns; the catch-all mirrors the
            // semantics side.
            _ => {}
        }
    }
}

/// The core-side twin of [`deckmaste_semantics::Ability::nested_abilities`]:
/// every `Ability` nested one level inside this one, in pre-order.
fn core_nested_abilities(ability: &deckmaste_core::Ability) -> Vec<&deckmaste_core::Ability> {
    let mut out = Vec::new();
    match ability {
        deckmaste_core::Ability::Static(e) => e.body.push_abilities(&mut out),
        deckmaste_core::Ability::Activated(a) => a.effect.push_abilities(&mut out),
        deckmaste_core::Ability::Triggered(a) => a.effect.push_abilities(&mut out),
        deckmaste_core::Ability::Spell(a) => a.effect.push_abilities(&mut out),
        deckmaste_core::Ability::Keyword(k) => k.push_abilities(&mut out),
    }
    out
}

/// Whether `image` appears anywhere below one of `lowered`'s members — the
/// descent [`core_abilities`] deliberately stops short of (it only walks
/// each face's own `abilities` list). Mirrors the semantics side's split
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

/// Every top-level semantic ability remains represented verbatim after
/// lowering. Nested carried abilities capture registers from their enclosing
/// region, so their correct image is context-dependent and is certified by
/// loading and validating the whole card rather than by lowering the subterm
/// in isolation.
#[test]
fn every_top_level_semantic_ability_appears_in_its_lowered_card() {
    let canon = Plugin::load_with_sibling_prelude(plugin_dir("canon", "")).unwrap();
    let mut checked = 0;
    for source in ron_files(&plugin_dir("canon", CARDS_DIR)) {
        let loaded = load_card_pair(&canon, &source).unwrap();
        let lowered: Vec<&deckmaste_core::Ability> = core_abilities(&loaded.core).collect();
        for face in semantic_faces(&loaded.semantic) {
            for semantic in &face.abilities {
                let image = semantic.clone().lower();
                assert!(
                    lowered.iter().any(|a| **a == image) || nested_in_any(&lowered, &image),
                    "{}: a top-level semantic ability's lowering is absent from the lowered card",
                    source.display(),
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "corpus produced no top-level abilities");
}

/// The top region of one core ability — the parameter list lowering assigned
/// it. `None` for an ability shape that carries no region of its own.
fn ability_region_params(ability: &deckmaste_core::Ability) -> Option<&[deckmaste_core::Param]> {
    use deckmaste_core::Ability;
    Some(match ability {
        Ability::Static(region) => &region.params,
        Ability::Activated(a) => &a.effect.params,
        Ability::Triggered(a) => &a.effect.params,
        Ability::Spell(a) => &a.effect.params,
        Ability::Keyword(_) => return None,
    })
}

/// Pair the semantic ability tree with the core ability tree one level at a
/// time, in pre-order, asserting the two have the SAME SHAPE at every depth.
///
/// A semantic `Expanded` node is macro provenance, erased at `lower` (spec
/// §12), so it descends WITHOUT consuming a core level — the invocation and
/// its value are one core ability.
fn pair_subterms<'a>(
    source: &std::path::Path,
    semantic: &'a deckmaste_semantics::Ability,
    core: &'a deckmaste_core::Ability,
    out: &mut Vec<(
        &'a deckmaste_semantics::Ability,
        &'a deckmaste_core::Ability,
    )>,
) {
    if let deckmaste_semantics::Ability::Expanded(expanded) = semantic {
        pair_subterms(source, &expanded.value, core, out);
        return;
    }
    out.push((semantic, core));
    let children = semantic.nested_abilities();
    let core_children = core_nested_abilities(core);
    assert_eq!(
        children.len(),
        core_children.len(),
        "{}: lowering changed the nesting shape — {} semantic child abilities became {}",
        source.display(),
        children.len(),
        core_children.len(),
    );
    for (child, core_child) in children.into_iter().zip(core_children) {
        pair_subterms(source, child, core_child, out);
    }
}

/// Every semantic ability subterm is represented at its OWN nesting depth in
/// the lowered card, and a carried region crosses its boundary with nothing
/// but declared captures.
///
/// RE-SPELLED. The claim this test used to make — that a subterm lowered in
/// isolation equals its image inside the card — is false BY DESIGN and is
/// permanently superseded (see
/// `docs/decisions/core-explicit-regions.md`, law 7). A carried region's
/// value is a function of its ENCLOSING register file: `in_carried_region`
/// appends a `Capture` parameter per enclosing register, and an isolated
/// lowering has no enclosing region to count. There is no honest normalization
/// that recovers the equality — erasing the captures erases exactly the
/// declaration law 7 introduced, and shifting register ordinals would need the
/// isolated side to know the enclosing region's definition count, which is
/// precisely the context it does not have. The depth-zero equality check
/// [`every_top_level_semantic_ability_appears_in_its_lowered_card`] is the
/// surviving equality claim.
///
/// What this test recovers instead is the coverage that was actually lost —
/// every depth, not just zero — as two claims that DO hold in context:
///
/// 1. The semantic subterm tree and the lowered nested-ability tree are the
///    same shape at every depth. A nested ability dropped, duplicated, or
///    re-parented by lowering fails here.
/// 2. A carried region's parameters are its intrinsic prefix — byte-identical
///    to what the isolated lowering assigns — followed by NOTHING but
///    `Provenance::Capture` parameters. That is law 7's "nothing else crosses a
///    region boundary", checked over the corpus rather than asserted.
#[test]
fn every_semantic_ability_subterm_appears_at_its_own_depth_in_its_lowered_card() {
    let canon = Plugin::load_with_sibling_prelude(plugin_dir("canon", "")).unwrap();
    let mut checked = 0;
    let mut carried = 0;
    for source in ron_files(&plugin_dir("canon", CARDS_DIR)) {
        let loaded = load_card_pair(&canon, &source).unwrap();
        let lowered: Vec<&deckmaste_core::Ability> = core_abilities(&loaded.core).collect();
        let mut top = lowered.iter().copied();
        for face in semantic_faces(&loaded.semantic) {
            for semantic in &face.abilities {
                let core = top.next().unwrap_or_else(|| {
                    panic!(
                        "{}: fewer lowered abilities than semantic ones",
                        source.display()
                    )
                });
                let mut pairs = Vec::new();
                pair_subterms(&source, semantic, core, &mut pairs);
                for (depth, (semantic, core)) in pairs.iter().enumerate() {
                    checked += 1;
                    let image = (*semantic).clone().lower();
                    let (Some(intrinsic), Some(actual)) =
                        (ability_region_params(&image), ability_region_params(core))
                    else {
                        continue;
                    };
                    assert!(
                        actual.len() >= intrinsic.len(),
                        "{}: a carried region lost parameters its isolated lowering declares",
                        source.display(),
                    );
                    assert_eq!(
                        &actual[..intrinsic.len()],
                        intrinsic,
                        "{}: a carried region's intrinsic parameter prefix diverged from the \
                         isolated lowering's",
                        source.display(),
                    );
                    for extra in &actual[intrinsic.len()..] {
                        assert!(
                            matches!(extra.provenance, deckmaste_core::Provenance::Capture(_)),
                            "{}: a carried region declares {:?} beyond its intrinsic prefix — \
                             only a declared capture crosses a region boundary",
                            source.display(),
                            extra.provenance,
                        );
                        carried += 1;
                    }
                    let _ = depth;
                }
            }
        }
    }
    assert!(checked > 0, "corpus produced no ability subterms");
    assert!(
        carried > 0,
        "no carried region in the corpus declares a capture — this test would pass vacuously"
    );
}
