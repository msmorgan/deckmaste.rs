//! The continuous-effects layer system ([CR#613]): the one place an object's
//! characteristics are derived. Consumers read a [`LayeredView`], never the
//! printed face. Layers 2 (control, [CR#613.1b]) and 4-7 (P/T sublayers 7a-7d)
//! are implemented, with the dependency tiebreaker ([CR#613.8]). Layer 1
//! (copy/face-down, [CR#613.2]) is a `base_values` seam and layer 3
//! (text-change, [CR#613.1c]) a documented no-op slot — both occupy their place
//! in the pass but await `core-copy-grammar` / `engine-face-down` / a [CR#612]
//! text-replacement engine (see the `engine-layers-1-copy-facedown-text` todo).

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::Duration;
use deckmaste_core::Filter;
use deckmaste_core::Ident;
use deckmaste_core::Int;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Modification;
use deckmaste_core::Scope;
use deckmaste_core::StaticEffect;
use deckmaste_core::Subtype;
use deckmaste_core::Supertype;
use deckmaste_core::Type;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::object::Timestamp;
use crate::player::PlayerId;
use crate::state::GameState;

// ---------------------------------------------------------------------------
// Registry types (floating one-shot continuous effects)
// ---------------------------------------------------------------------------

/// An effect's resolved target set, used throughout the pipeline.
/// `Locked` holds ids snapshotted at creation ([CR#611.2c], one-shot
/// `Of`/`These`; also a static `Of`/`These` resolved source-relative in
/// `gather` — see `resolve_source_relative`). `Floating` holds a filter
/// re-evaluated against the derived map each layer (static `Matching`).
#[derive(Debug, Clone)]
pub enum ScopeResolved {
    Locked(Vec<ObjectId>),
    Floating(Filter),
}

/// A floating one-shot continuous effect ([CR#611.2]). Lives in
/// `GameState.continuous` until its `duration` expires.
#[derive(Debug, Clone)]
pub struct ContinuousEffect {
    pub timestamp: Timestamp,
    /// The effect's controller, locked at creation ([CR#611.2c]). Resolves the
    /// `You` in a layer-2 `SetController(You)` ("you gain control of …"); for
    /// effects with no controller-relative reference it is inert.
    pub controller: PlayerId,
    pub scope: ScopeResolved,
    pub changes: Vec<Modification>,
    pub duration: Duration,
    pub is_cda: bool,
}

/// An object's derived characteristics ([CR#109.3]): the strict set the rules
/// name as characteristics. `power`/`toughness` are `None` for objects with no
/// P/T; a printed `*` with no CDA resolves to `0` ([CR#208.2a]).
/// All list-valued fields are `Arc`'d copy-on-write: base values share the
/// per-card caches built at `Cards::push`, and a mutating layer op clones via
/// `Arc::make_mut` only for the objects an effect actually touches.
///
/// Non-characteristic derived state (controller, the layer-6 can't-have set)
/// lives on [`DerivedObject`], not here — `Characteristics` is a named CR
/// concept and holds only characteristics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Characteristics {
    pub power: Option<Int>,
    pub toughness: Option<Int>,
    pub colors: Arc<Vec<Color>>,
    pub card_types: Arc<Vec<Type>>,
    pub subtypes: Arc<Vec<Subtype>>,
    pub supertypes: Arc<Vec<Supertype>>,
    pub abilities: Arc<Vec<Ability>>,
}

/// An object's full derived per-object state: its CR [`Characteristics`] plus
/// derived state that is *not* a characteristic — the controller ([CR#613.1b]
/// layer 2) and the layer-6 can't-have-ability prohibition set ([CR#613.1f]).
/// This is the value the layer pass mutates and the [`LayeredView`] stores.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DerivedObject {
    characteristics: Characteristics,
    /// Derived controller ([CR#613.1b]). Seeded from the object's base
    /// controller, overwritten by layer-2 control-change effects; reverts
    /// automatically when those effects expire (it is re-derived each pass).
    controller: PlayerId,
    /// Ability names the object can't have or gain ([CR#613.1f]).
    /// Populated by `CantHaveAbility`; consulted by `GainAbility`.
    cant_have: Vec<Ident>,
}

/// Every live object's derived state, computed in one pass.
#[derive(Debug, Clone)]
pub struct LayeredView(BTreeMap<ObjectId, DerivedObject>);

impl LayeredView {
    /// Returns the derived characteristics for `id`.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not a live object when the view was computed.
    #[must_use]
    pub fn get(&self, id: ObjectId) -> &Characteristics {
        &self.entry(id).characteristics
    }

    /// Returns the derived controller for `id` ([CR#613.1b]): the base
    /// controller as modified by any active layer-2 control-change effect.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not a live object when the view was computed.
    #[must_use]
    pub fn controller(&self, id: ObjectId) -> PlayerId {
        self.entry(id).controller
    }

    fn entry(&self, id: ObjectId) -> &DerivedObject {
        self.0.get(&id).expect("live ObjectId in LayeredView")
    }

    #[must_use]
    pub fn power(&self, id: ObjectId) -> Option<Int> {
        self.get(id).power
    }

    #[must_use]
    pub fn toughness(&self, id: ObjectId) -> Option<Int> {
        self.get(id).toughness
    }

    /// Test-only: a view holding a single object with the given derived
    /// abilities (controller `PlayerId(0)`, no other characteristics). Lets
    /// consumers of the derived ability list be unit-tested without standing up
    /// a whole `GameState`.
    #[cfg(test)]
    pub(crate) fn single_with_abilities(id: ObjectId, abilities: Vec<Ability>) -> Self {
        let mut working = BTreeMap::new();
        working.insert(
            id,
            DerivedObject {
                characteristics: Characteristics {
                    power: None,
                    toughness: None,
                    colors: Arc::new(Vec::new()),
                    card_types: Arc::new(Vec::new()),
                    subtypes: Arc::new(Vec::new()),
                    supertypes: Arc::new(Vec::new()),
                    abilities: Arc::new(abilities),
                },
                controller: PlayerId(0),
                cant_have: Vec::new(),
            },
        );
        LayeredView(working)
    }
}

/// Resolve a printed `StatValue` to a base number. `*` with no CDA is `0`
/// ([CR#208.2a]); CDAs (layer 7a) overwrite this later. Also the printed-stat
/// source for snapshot (LKI) `Stat` matching, which has no layer view.
pub(crate) fn base_stat(v: Option<&deckmaste_core::StatValue>) -> Option<Int> {
    match v {
        Some(deckmaste_core::StatValue::Number(n)) => Some(*n),
        Some(_) => Some(0), // DefinedByAbility / Variable: 0 until a 7a CDA sets it
        None => None,
    }
}

/// A face's base colors ([CR#202.2]): the colored mana symbols in the cost,
/// falling back to the color indicator for objects with no mana cost.
/// Computed once per card at setup (`Cards::push`) and cached.
pub(crate) fn base_colors(face: &deckmaste_core::CardFace) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();
    for c in face.mana_cost.iter().flat_map(symbol_colors) {
        if !colors.contains(&c) {
            colors.push(c);
        }
    }
    if colors.is_empty() {
        colors.clone_from(&face.color_indicator);
    }
    colors
}

/// Collect the colors contributed by one mana symbol ([CR#202.2]).
fn symbol_colors(sym: &ManaSymbol) -> impl Iterator<Item = Color> {
    let mut buf: [Option<Color>; 2] = [None; 2];
    match sym {
        ManaSymbol::Simple(s) => {
            buf[0] = s.color();
        }
        ManaSymbol::Hybrid(s, c) => {
            buf[0] = s.color();
            buf[1] = Some(*c);
        }
        ManaSymbol::Phyrexian(c, c2) => {
            buf[0] = Some(*c);
            buf[1] = *c2;
        }
        ManaSymbol::Variable | ManaSymbol::Snow => {}
    }
    buf.into_iter().flatten()
}

/// Base derived state from the printed face ([CR#613.1]): the object's
/// characteristics and controller before any continuous effect.
///
/// SEAM — layer 1 ([CR#613.2]): the copiable values are the printed face
/// *as modified by* copy effects (layer 1a, [CR#707.2]) and face-down status
/// (layer 1b, [CR#708.2]). Neither input exists yet (`core-copy-grammar` /
/// `engine-face-down` own them), so this reads the printed face. When a copy
/// source or face-down spec is present on the object, branch here to derive the
/// copiable values from it instead — that is the entirety of layer 1's effect
/// on `Characteristics`, since after layer 1 the characteristics *are* the
/// copiable values ([CR#613.2c]).
fn base_values(state: &GameState, id: ObjectId) -> DerivedObject {
    let obj = state.objects.obj(id);
    let card = obj.card_id().expect("card-backed object");
    let instance = state.cards.get(card);
    let face = crate::derive::face(&instance.def);
    DerivedObject {
        characteristics: Characteristics {
            power: base_stat(face.power.as_ref()),
            toughness: base_stat(face.toughness.as_ref()),
            colors: Arc::clone(&instance.colors),
            card_types: Arc::clone(&instance.card_types),
            subtypes: Arc::clone(&instance.subtypes),
            supertypes: Arc::clone(&instance.supertypes),
            abilities: Arc::clone(&instance.printed),
        },
        // Base controller ([CR#108.4]): what the object would have absent any
        // control-change effect. Layer 2 may overwrite it.
        controller: obj.controller,
        cant_have: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Layer ordering
// ---------------------------------------------------------------------------

/// The layer a `Modification` op lives in ([CR#613.1,613.3,613.4]), in
/// application order. Layer 1 (copy/face-down, [CR#613.2]) is not a
/// `Modification` — it reshapes the *base* copiable values and is handled in
/// `base_values`, so it has no variant here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum Layer {
    L2,
    L3,
    L4,
    L5,
    L6,
    L7a,
    L7b,
    L7c,
    L7d,
}

/// Maps a `Modification` to the layer it applies in.
///
/// Returns `None` for ops with no [CR#613] layer (loyalty/defense, which are
/// not characteristics here).
fn layer_of(m: &Modification, is_cda: bool) -> Option<Layer> {
    match m {
        // Layer 2: control-changing ([CR#613.1b]).
        Modification::SetController(_) => Some(Layer::L2),
        // Layer 3: text-changing ([CR#613.1c,612]). The op lives in the
        // pass at the right position; its `apply` is a documented no-op until a
        // real word-replacement engine exists (its own todo).
        Modification::SetText(_) => Some(Layer::L3),
        // Layer 4: type-changing ([CR#613.1d]).
        Modification::SetCardTypes(_)
        | Modification::AddCardTypes(_)
        | Modification::SetSubtypes(_)
        | Modification::AddSubtypes(_)
        | Modification::SetSupertypes(_)
        | Modification::AddSupertypes(_)
        | Modification::AllCreatureTypes
        | Modification::BecomeBasicLandType(_) => Some(Layer::L4),
        // Layer 5: color-changing ([CR#613.1e]).
        Modification::SetColors(_) | Modification::AddColors(_) => Some(Layer::L5),
        // Layer 6: ability-adding/removing ([CR#613.1f]).
        Modification::GainAbility(_)
        | Modification::LoseAbility(_)
        | Modification::LoseAllAbilities
        | Modification::CantHaveAbility(_) => Some(Layer::L6),
        // Layer 7a or 7b: SetPower/SetToughness ([CR#613.4a,613.4b]).
        // CDAs ([CR#604.3]) apply in 7a; all other set-ops apply in 7b.
        Modification::SetPower(_) | Modification::SetToughness(_) => {
            if is_cda {
                Some(Layer::L7a)
            } else {
                Some(Layer::L7b)
            }
        }
        // Layer 7c: P/T modification ([CR#613.4c]).
        Modification::AddPower(_)
        | Modification::AddToughness(_)
        | Modification::SubtractPower(_)
        | Modification::SubtractToughness(_) => Some(Layer::L7c),
        // Layer 7d: switch ([CR#613.4d]).
        Modification::SwitchPowerToughness => Some(Layer::L7d),
        // No [CR#613] layer: loyalty/defense are not characteristics here.
        Modification::SetBaseLoyalty(_) | Modification::SetBaseDefense(_) => None,
    }
}

// ---------------------------------------------------------------------------
// Gather
// ---------------------------------------------------------------------------

/// An active static continuous effect ready to apply across one or more
/// layers ([CR#613.6]). Owns cloned data so no borrow escapes `state`.
struct ActiveEffect {
    timestamp: Timestamp,
    is_cda: bool,
    /// The effect's controller ([CR#611.2c]) — resolves `You` in a layer-2
    /// `SetController`. For a static ability it is the source permanent's base
    /// controller; for a registry effect it is the locked value it carries.
    controller: PlayerId,
    scope: ScopeResolved,
    changes: Vec<Modification>,
    /// The effect's carrier ([CR#611.2c]) — the object whose `Ref(This)`/
    /// `Ref(You)` a `Matching` scope resolves against. The source permanent for
    /// a static ability; `Player(controller)` for a spell-built floating effect
    /// (its source spell has left the stack, so `You` anchors on the locked
    /// controller's proxy, which is always live). Threaded into `resolve_scope`
    /// → `matches_derived` so a tribal-lord scope resolves instead of
    /// panicking.
    watcher: Option<ObjectSource>,
    /// Locked target set: `None` until first applied layer resolves the scope
    /// ([CR#613.6] — scope is locked at first layer of application).
    locked: Option<Vec<ObjectId>>,
}

/// Bake a counter confer's `Modification`s for a specific holder: a self-scoped
/// `CounterCount(This, k)` becomes `Literal(holder's count of k)` ([CR#122.1]).
/// The layer-side `eval_count` is literal-only, and a counter's `Continuous`
/// boost is self-scoped (it modifies its holder), so the holder's counter map
/// is the right source — equivalent to the old hardcoded 7c read, but
/// data-driven. Non-`Count` changes (`GainAbility` for keyword counters) and
/// non-`CounterCount` counts pass through untouched.
fn bake_counter_counts(
    changes: &[Modification],
    holder: &std::collections::HashMap<Ident, deckmaste_core::Uint>,
) -> Vec<Modification> {
    let bake = |count: &Count| -> Count {
        match count {
            Count::CounterCount(reference, kind)
                if matches!(&**reference, deckmaste_core::Reference::This) =>
            {
                Count::Literal(holder.get(kind.as_str()).copied().unwrap_or(0))
            }
            other => other.clone(),
        }
    };
    changes
        .iter()
        .map(|m| match m {
            Modification::AddPower(n) => Modification::AddPower(bake(n)),
            Modification::AddToughness(n) => Modification::AddToughness(bake(n)),
            Modification::SubtractPower(n) => Modification::SubtractPower(bake(n)),
            Modification::SubtractToughness(n) => Modification::SubtractToughness(bake(n)),
            Modification::SetPower(n) => Modification::SetPower(bake(n)),
            Modification::SetToughness(n) => Modification::SetToughness(bake(n)),
            Modification::SetBaseLoyalty(n) => Modification::SetBaseLoyalty(bake(n)),
            Modification::SetBaseDefense(n) => Modification::SetBaseDefense(bake(n)),
            other => other.clone(),
        })
        .collect()
}

/// Collect all active static `Modify` effects from battlefield permanents,
/// plus any floating one-shot effects from `state.continuous`, plus
/// counter-conferred `Continuous` boosts ([CR#122.1]).
///
/// Only unconditional effects are gathered here; `sa.condition` evaluation
/// is a documented seam for a later task.
fn gather(state: &GameState) -> Vec<ActiveEffect> {
    let mut effects = Vec::new();
    for obj in state.objects.iter() {
        if obj.card_id().is_none() {
            continue; // player proxy — no static abilities
        }
        // Static abilities function only on the battlefield ([CR#611.3b]).
        if obj.zone != Some(Zone::Battlefield) {
            continue;
        }
        let timestamp = obj.timestamp;
        // v1: uses printed_abilities (not the derived view) to break the
        // layers() → derive::abilities → layers() recursion. As a result, a
        // static ability that is itself *granted* by a layer-6 effect won't be
        // re-gathered as an effect source (no fixpoint). No fixture requires that.
        // Flatten the printed list the same way the engine-internal
        // enumeration does (`derive::flatten_composites`): peel `Innate`
        // ([CR#113.12]) — a subtype rule conferred as `Innate(Static(...))`
        // (the Aura/Equipment/Fortification attachment rules) still functions
        // as a [CR#604.1] static — AND splice the members of a composite
        // keyword. A `KeywordAbility`-kind macro (Changeling, Devoid) expands
        // to `Keyword(Composite { abilities: [Static(...)] })`; without
        // splicing, the `Static` it carries would never be gathered, so the
        // keyword's continuous effect (devoid → colorless, changeling → every
        // creature type) would silently do nothing. This mirrors the flat
        // ability space `abilities_of_source` builds for the trigger scan.
        let mut printed = Vec::new();
        for ability in crate::derive::printed_abilities(state, obj.id) {
            crate::derive::flatten_composites(ability, &mut printed);
        }
        for ability in &printed {
            let Ability::Static(sa) = ability else {
                continue;
            };
            // Conditions skipped — a seam for later ([CR#611.3a]).
            for effect in &sa.effects {
                let StaticEffect::Modify { of, changes } = effect else { continue };
                // Convert Scope to ScopeResolved. `Matching` floating scopes
                // stay floating. `Of`/`These` reference the carrying object's
                // relations — resolved here SOURCE-RELATIVE (`This = obj.id`,
                // no `Frame`) so a host-targeting static ("enchanted/equipped
                // creature gets +N/+N", `Of(AttachHostOf(This))`) lands on the
                // host. The resolved ids are LOCKED at this first application
                // ([CR#613.6]). References that genuinely need a `Frame`
                // (`Target`, bindings) cannot be resolved in gather and drop to
                // an empty locked set (a documented seam).
                let scope = match of {
                    Scope::Matching(f) => ScopeResolved::Floating(f.clone()),
                    Scope::Of(r) => {
                        ScopeResolved::Locked(resolve_source_relative(state, obj.id, r))
                    }
                    // Dedup the resolved ids ([CR#613.7] deterministic id
                    // order, mirroring `AttachedTo`'s host-set): overlapping
                    // references (e.g. `These([AttachHostOf(This), This])` on a
                    // self-attached object) must not apply an additive change
                    // twice.
                    Scope::These(rs) => ScopeResolved::Locked(
                        rs.iter()
                            .flat_map(|r| resolve_source_relative(state, obj.id, r))
                            .collect::<BTreeSet<ObjectId>>()
                            .into_iter()
                            .collect(),
                    ),
                };
                effects.push(ActiveEffect {
                    timestamp,
                    is_cda: sa.characteristic_defining,
                    // A static ability's continuous effect is controlled by the
                    // permanent it is on ([CR#611.2c]); its `You` is that
                    // permanent's controller.
                    controller: obj.controller,
                    scope,
                    changes: changes.clone(),
                    // The carrier is the source permanent itself: a `Matching`
                    // scope's `Ref(This)` is this object and `Ref(You)` is its
                    // controller ([CR#603.10a,109.5]).
                    watcher: Some(obj.source),
                    locked: None,
                });
            }
        }

        // [CR#122.1]: counter-conferred continuous effects (the boost flavor) —
        // strip-immune, gathered from the object's counter map + the registry,
        // NOT from its abilities (a +1/+1 counter still pumps under
        // `LoseAllAbilities`). A self-scoped `CounterCount(This, k)` is BAKED to
        // the holder's live count here, since the layer-side `eval_count` is
        // literal-only and a counter boost is self-scoped.
        for kind in obj.counters.keys() {
            let Some(decl) = state.counter_decls.get(kind) else {
                continue;
            };
            for prop in &decl.confers {
                let deckmaste_core::Property::Continuous { of, changes } = prop else {
                    continue;
                };
                let scope = match of {
                    Scope::Matching(f) => ScopeResolved::Floating(f.clone()),
                    Scope::Of(r) => {
                        ScopeResolved::Locked(resolve_source_relative(state, obj.id, r))
                    }
                    Scope::These(rs) => ScopeResolved::Locked(
                        rs.iter()
                            .flat_map(|r| resolve_source_relative(state, obj.id, r))
                            .collect::<BTreeSet<ObjectId>>()
                            .into_iter()
                            .collect(),
                    ),
                };
                effects.push(ActiveEffect {
                    timestamp: obj.timestamp,
                    is_cda: false,
                    controller: obj.controller,
                    scope,
                    changes: bake_counter_counts(changes, &obj.counters),
                    watcher: Some(obj.source),
                    locked: None,
                });
            }
        }
    }
    // Append floating one-shot continuous effects from the registry.
    for ce in &state.continuous {
        effects.push(ActiveEffect {
            timestamp: ce.timestamp,
            is_cda: ce.is_cda,
            controller: ce.controller,
            scope: ce.scope.clone(),
            changes: ce.changes.clone(),
            // A spell-built floating effect's source spell has left the stack by
            // the time the layer pass runs, so `Ref(You)` anchors on the locked
            // controller's player proxy (`controller_of_source(Player(p)) == p`,
            // always live) rather than the gone spell. A floating `Matching`
            // scope naming `Ref(This)` (no canonical card does) would resolve to
            // that proxy; the spell itself is not re-derivable here ([CR#611.2c]).
            watcher: Some(ObjectSource::Player(ce.controller)),
            locked: None,
        });
    }
    effects
}

/// Resolve a `Reference` inside a static ability's `Scope::Of`/`These` to
/// concrete object ids, SOURCE-RELATIVE: `This` is the static's source object
/// `source` (the carrying permanent), and `gather` has **no** [`Frame`], so
/// only the references whose value is fixed by the source's own relations are
/// resolvable here. The rest are a documented seam (see below) and resolve to
/// the empty set.
///
/// Returns a (possibly empty) vec — the empty set both for a Frame-dependent
/// reference and for a relation that isn't established (an unattached
/// attachment's `AttachHostOf(This)` has no host yet). The caller LOCKS the
/// result ([CR#613.6]: the affected set is fixed at first application).
///
/// [`Frame`]: crate::resolve::Frame
fn resolve_source_relative(
    state: &GameState,
    source: ObjectId,
    reference: &deckmaste_core::Reference,
) -> Vec<ObjectId> {
    use deckmaste_core::Reference;
    match reference {
        // The carrying object itself.
        Reference::This => vec![source],
        // The host an attachment is attached to ([CR#301.5,303.4]) — read the
        // attachment→host link off the resolved inner object. No host (an
        // unattached attachment) → empty, so nothing is buffed.
        Reference::AttachHostOf(inner) => resolve_source_relative(state, source, inner)
            .into_iter()
            .filter_map(|id| state.objects.get(id).and_then(|o| o.attached_to))
            .collect(),
        // The inverse (host→attachment): every object whose `attached_to`
        // points at the resolved host ([CR#613.7] deterministic id order). A
        // host with multiple attachments fans out to all of them.
        Reference::AttachedTo(inner) => {
            let hosts: BTreeSet<ObjectId> = resolve_source_relative(state, source, inner)
                .into_iter()
                .collect();
            state
                .objects
                .iter()
                .filter(|o| o.attached_to.is_some_and(|h| hosts.contains(&h)))
                .map(|o| o.id)
                .collect()
        }
        // Look through a remembered macro invocation.
        Reference::Expanded(e) => resolve_source_relative(state, source, &e.value),
        // Frame-dependent references only: `Target`, bindings (`Bound`,
        // `Linked`, `ThatObject`, `ThatPlayer`) — and the player-valued
        // `You`/`ControllerOf`/`OwnerOf` — cannot be resolved without a `Frame`
        // in gather, so they stay an empty locked set (a documented seam; these
        // need `eval_reference`, which `gather` deliberately lacks to avoid the
        // layers()→eval recursion).
        _ => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Scope resolution
// ---------------------------------------------------------------------------

/// Evaluate a `Filter` against a single object's DERIVED characteristics in
/// `working`, delegating non-characteristic leaves to the printed matcher.
///
/// This is the working-aware sibling of `target::matches` that realizes
/// [CR#613.6]'s rule that "affected sets" for multi-layer effects are
/// re-evaluated against the characteristics produced by earlier layers.
/// Without this, a `Matching(Type(Enchantment))` filter would still read the
/// printed type even after L4 has replaced it, making the lock unobservable.
///
/// Missing `id` in `working` (e.g. a player proxy) returns `false`.
///
/// `watcher` is the carrier of the effect whose scope is being evaluated — the
/// source permanent for a static ability, or `Player(controller)` for a
/// spell-built floating effect ([CR#611.2c]). It anchors `Ref(This)`/`Ref(You)`
/// in the scope: without it those refs hit the frameless-targeting `todo!` in
/// `target::matches` the moment a layer rebuild touches a tribal-lord scope
/// (`AllOf([…, Not(Ref(This)), ControlledBy(Ref(You))])`). The live trigger
/// lane (`filter_matches_live`) threads its watcher the same way.
fn matches_derived(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    id: ObjectId,
    filter: &deckmaste_core::Filter,
    watcher: Option<ObjectSource>,
) -> bool {
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Filter;
    // `Any` is a wildcard sentinel — it must always match, even for ids that
    // aren't in `working` (e.g. player proxies). Checked before the map lookup.
    if let Filter::Any = filter {
        return true;
    }
    let Some(c) = working.get(&id).map(|d| &d.characteristics) else {
        return false;
    };
    match filter {
        Filter::Any => unreachable!("handled above"),
        Filter::Characteristic(CharacteristicFilter::Type(t)) => c.card_types.contains(t),
        Filter::Characteristic(CharacteristicFilter::Supertype(s)) => c.supertypes.contains(s),
        Filter::Characteristic(CharacteristicFilter::ColorIs(col)) => c.colors.contains(col),
        Filter::Characteristic(CharacteristicFilter::Multicolored) => c.colors.len() >= 2,
        Filter::Characteristic(CharacteristicFilter::Colorless) => c.colors.is_empty(),
        // Subtype matching against derived: `working[id].subtypes` are Subtype
        // structs; the filter carries an Ident name. Match by name.
        Filter::Characteristic(CharacteristicFilter::Subtype(name)) => {
            c.subtypes.iter().any(|s| &s.name == name)
        }
        // `Has` is derivable from the working map — check the derived
        // ability list.
        Filter::Characteristic(CharacteristicFilter::Has(name)) => {
            c.abilities.iter().any(|a| ability_is_named(a, &name.0))
        }
        // Stat over DERIVED P/T (in the working map); mana value is printed
        // (layer-stable), read without the layer view. Evaluated HERE rather
        // than delegated so the derived matcher never re-enters `state.layers()`
        // mid-build via `target::matches`'s layers-reading Stat arm.
        Filter::Characteristic(CharacteristicFilter::Stat(stat, cmp, count)) => {
            use deckmaste_core::Stat;
            let value = match stat {
                Stat::Power => c.power,
                Stat::Toughness => c.toughness,
                Stat::ManaValue => Some(
                    Int::try_from(crate::derive::face(state.def(id)).mana_cost.mana_value())
                        .expect("mana value fits Int"),
                ),
                // [CR#122.1e,122.1g]: loyalty/defense are the object's
                // loyalty-/defense-counter counts (read off the counter map).
                Stat::Loyalty => Some(
                    Int::try_from(
                        state
                            .objects
                            .obj(id)
                            .counters
                            .get("LoyaltyCounter")
                            .copied()
                            .unwrap_or(0),
                    )
                    .expect("loyalty fits Int"),
                ),
                Stat::Defense => Some(
                    Int::try_from(
                        state
                            .objects
                            .obj(id)
                            .counters
                            .get("DefenseCounter")
                            .copied()
                            .unwrap_or(0),
                    )
                    .expect("defense fits Int"),
                ),
            };
            crate::target::stat_satisfies(value, *cmp, count)
        }
        // Combinators: recurse through matches_derived so characteristic leaves
        // see the derived map, carrying the same `watcher` so a nested
        // `Ref(This)`/`Ref(You)` still anchors against the host.
        Filter::AllOf(fs) => fs
            .iter()
            .all(|f| matches_derived(state, working, id, f, watcher)),
        Filter::OneOf(fs) => fs
            .iter()
            .any(|f| matches_derived(state, working, id, f, watcher)),
        Filter::Not(f) => !matches_derived(state, working, id, f, watcher),
        Filter::Expanded(e) => matches_derived(state, working, id, &e.value, watcher),
        // `Named` and everything non-characteristic (zone, status, kind,
        // combat, relations, refs …): delegate to the printed matcher, threading
        // the carrier `watcher` so a scope's `Ref(This)`/`Ref(You)` (and the
        // `Ref(You)` nested inside a `ControlledBy`) anchors against the host
        // instead of hitting the frameless-targeting `todo!`. None of the
        // delegated arms re-enter `state.layers()` for a battlefield permanent
        // (the only `id`s this matcher sees): `Named` reads the printed face;
        // relations resolve over player proxies / object iteration; combat and
        // state read stored fields. Characteristic leaves are all handled above.
        _ => crate::target::matches_with(state, id, filter, watcher),
    }
}

/// Resolve a `ScopeResolved` against the current working set, returning the
/// target object ids.
///
/// `Floating` filters against DERIVED characteristics via `matches_derived`,
/// realizing [CR#613.6]'s requirement that affected sets are re-evaluated
/// against "the characteristics produced by earlier layers". This means an
/// anthem catches permanents animated to Creature by a same-pass L4 effect,
/// and a `Matching(Enchantment)` scope correctly includes only objects still
/// typed Enchantment in the working map at the time of resolution.
///
/// `Locked` ids are returned as-is (pre-snapshotted at the first applied
/// layer, per [CR#613.6]).
fn resolve_scope(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    scope: &ScopeResolved,
    watcher: Option<ObjectSource>,
) -> Vec<ObjectId> {
    match scope {
        ScopeResolved::Floating(filter) => working
            .keys()
            .copied()
            .filter(|&id| matches_derived(state, working, id, filter, watcher))
            .collect(),
        ScopeResolved::Locked(ids) => ids.clone(),
    }
}

// ---------------------------------------------------------------------------
// Apply
// ---------------------------------------------------------------------------

/// Evaluate a `Count` to an `Int`. Only `Literal` is resolved here; all
/// other variants default to `0` (documented limitation — CDAs and dynamic
/// counts are a later task).
fn eval_count(n: &Count) -> Int {
    match n {
        Count::Literal(v) => (*v).cast_signed(),
        _ => 0,
    }
}

/// Whether `a` is the named ability identified by `name`. Uses the
/// `KeywordAbility::as_str()` mapping — the canonical printed name is the
/// variant identifier (e.g. `"Trample"`). Non-keyword abilities have no
/// simple name and return `false` in v1; `LoseAbility`/`CantHaveAbility`
/// are defined to target named keyword abilities ([CR#613.1f]).
pub(crate) fn ability_is_named(a: &Ability, name: &Ident) -> bool {
    match a {
        Ability::Keyword(kw) => name == kw.as_str(),
        Ability::Expanded(e) => ability_is_named(&e.value, name),
        _ => false,
    }
}

/// Apply one `Modification` to `c` at its layer.
/// Layers 4 (types/supertypes), 5 (colors), 6 (abilities), and 7a-7d (P/T) are
/// implemented. Subtypes ([CR#613.1d]) and `BecomeBasicLandType` ([CR#305.7])
/// are explicit deferred stubs; controller/text ([CR#613.1b,613.1c]) and
/// loyalty/defense (no 613 layer) are also stubs.
#[allow(clippy::match_same_arms)] // deferred stub arms will diverge as later tasks fill them
fn apply(m: &Modification, effect_controller: PlayerId, d: &mut DerivedObject) {
    let c = &mut d.characteristics;
    match m {
        // --- Layer 7a / 7b: set base P/T ---
        Modification::SetPower(n) => c.power = Some(eval_count(n)),
        Modification::SetToughness(n) => c.toughness = Some(eval_count(n)),
        // --- Layer 7c: add/subtract P/T ---
        // [CR#613.4c]: a 7c add/subtract modifies an EXISTING power/toughness —
        // it never materializes one on a P/T-less object (a +1/+1 counter or a
        // pump on a non-creature permanent does nothing P/T-wise, [CR#122.1a]).
        Modification::AddPower(n) => {
            if let Some(p) = c.power {
                c.power = Some(p + eval_count(n));
            }
        }
        Modification::AddToughness(n) => {
            if let Some(t) = c.toughness {
                c.toughness = Some(t + eval_count(n));
            }
        }
        Modification::SubtractPower(n) => {
            if let Some(p) = c.power {
                c.power = Some(p - eval_count(n));
            }
        }
        Modification::SubtractToughness(n) => {
            if let Some(t) = c.toughness {
                c.toughness = Some(t - eval_count(n));
            }
        }
        // --- Layer 7d: switch ---
        Modification::SwitchPowerToughness => std::mem::swap(&mut c.power, &mut c.toughness),
        // --- Layer 4: type-changing ([CR#613.1d]) ---
        Modification::AddCardTypes(ts) => {
            let types = Arc::make_mut(&mut c.card_types);
            for t in ts {
                if !types.contains(t) {
                    types.push(*t);
                }
            }
        }
        Modification::SetCardTypes(ts) => c.card_types = Arc::new(ts.clone()),
        Modification::AddSupertypes(ss) => {
            let supertypes = Arc::make_mut(&mut c.supertypes);
            for s in ss {
                if !supertypes.contains(s) {
                    supertypes.push(*s);
                }
            }
        }
        Modification::SetSupertypes(ss) => c.supertypes = Arc::new(ss.clone()),
        // [CR#613.1d] subtype-set deferred: Ident→Subtype reconcile (no fixture yet)
        // `Modification::SetSubtypes`/`AddSubtypes` carry `Vec<Ident>` but
        // `Characteristics::subtypes` holds `Vec<Subtype>` (structs with confers/types).
        // There is no clean `Ident → Subtype` conversion without plugin data, so these
        // arms are left as explicit no-ops until a fixture exercises them.
        Modification::SetSubtypes(_) | Modification::AddSubtypes(_) => {}
        // [CR#305.7] deferred: replace land subtypes + strip abilities + grant basic
        // mana ability (no fixture yet). Do NOT implement the mana-ability construction.
        Modification::BecomeBasicLandType(_) => {}
        // TODO(kw-changeling): every-creature-type subtype fill
        // ([CR#702.73a,205.3m]) — no-op stub until built (reachable now that
        // Of(This) resolves source-relative in gather). The full fill needs
        // the declared creature-type registry threaded into the layer engine;
        // a changeling deriving as typeless-for-now is strictly better than a
        // panic, and matches the sibling deferred stubs in this match
        // (`SetSubtypes`/`BecomeBasicLandType`). Do NOT implement the fill here.
        Modification::AllCreatureTypes => {}
        // --- Layer 5: color-changing ([CR#613.1e]) ---
        Modification::AddColors(cl) => {
            let colors = Arc::make_mut(&mut c.colors);
            for x in cl {
                if !colors.contains(x) {
                    colors.push(*x);
                }
            }
        }
        Modification::SetColors(cl) => c.colors = Arc::new(cl.clone()),
        // --- Layer 6: ability-adding/removing ([CR#613.1f]) ---
        // `Arc::make_mut` realizes the copy-on-write: the shared per-card base
        // list is cloned only here, only for the objects an effect touches.
        Modification::GainAbility(a) => {
            // Respect any active "can't have" prohibition ([CR#613.1f]).
            if !d.cant_have.iter().any(|n| ability_is_named(a, n)) {
                Arc::make_mut(&mut c.abilities).push((**a).clone());
            }
        }
        // [CR#113.12]: "loses all abilities" strips card abilities but NOT
        // `Innate` subtype rules (the Aura [CR#704.5m] graveyard SBA, the
        // Equipment/Fortification host restriction) — those are rules of the
        // object, not abilities it has, so they survive.
        Modification::LoseAllAbilities => {
            Arc::make_mut(&mut c.abilities).retain(Ability::is_innate);
        }
        Modification::LoseAbility(name) => {
            // [CR#113.12]: never remove an `Innate` ability. (A named-keyword
            // `LoseAbility` already misses `Innate`, which carries no keyword
            // name, but guard explicitly so the invariant is local.)
            Arc::make_mut(&mut c.abilities).retain(|x| x.is_innate() || !ability_is_named(x, name));
        }
        Modification::CantHaveAbility(name) => {
            // Remove any already-present instance of the named ability (except
            // `Innate`, [CR#113.12]), then record the prohibition so future
            // GainAbility skips it.
            Arc::make_mut(&mut c.abilities).retain(|x| x.is_innate() || !ability_is_named(x, name));
            d.cant_have.push(*name);
        }
        // --- Layer 2: control-changing ([CR#613.1b]) ---
        Modification::SetController(reference) => {
            if let Some(p) = resolve_new_controller(reference, effect_controller) {
                d.controller = p;
            }
        }
        // --- Layer 3: text-changing ([CR#613.1c]) — documented slot ---
        // A real [CR#612] word-replacement needs text→ability re-derivation the
        // engine does not do; tracked by its own todo. The op still occupies its
        // layer-3 position in the pass (so dependency ordering sees it).
        Modification::SetText(_) => {}
        // --- No [CR#613] layer ---
        Modification::SetBaseLoyalty(_) | Modification::SetBaseDefense(_) => {
            // Loyalty/defense are not characteristics here; no layer applies.
        }
    }
}

/// Resolve a control-change effect's new controller ([CR#613.1b]) to a concrete
/// player. Per [CR#611.2c] the effect's references are locked when it is
/// created, so `Reference::You` resolves to the effect's controller (the happy
/// path: "you gain control of …"). General `Reference` resolution needs the
/// resolve-time `Frame` machinery (`engine-resolve-effects`); any other
/// reference is a documented seam that leaves the controller unchanged.
fn resolve_new_controller(
    reference: &deckmaste_core::Reference,
    effect_controller: PlayerId,
) -> Option<PlayerId> {
    use deckmaste_core::Reference;
    match reference {
        Reference::You => Some(effect_controller),
        // SEAM: opponent / each-player / bound references need a `Frame` to
        // resolve a specific player; not reachable by current control fixtures.
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Dependency ordering ([CR#613.8])
// ---------------------------------------------------------------------------

/// The objects an effect applies to in the current pass state: its locked set
/// once locked ([CR#613.6]), otherwise its scope resolved against `working`.
fn effect_targets(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    effect: &ActiveEffect,
) -> Vec<ObjectId> {
    match &effect.locked {
        Some(ids) => ids.clone(),
        None => resolve_scope(state, working, &effect.scope, effect.watcher),
    }
}

/// Apply every op of `effect` that belongs to `layer` to its targets in
/// `working`, locking its scope on first application ([CR#613.6]: the affected
/// set is fixed at the first layer the effect applies and reused thereafter).
fn apply_effect_in_layer(
    state: &GameState,
    working: &mut BTreeMap<ObjectId, DerivedObject>,
    effect: &mut ActiveEffect,
    layer: Layer,
) {
    if effect.locked.is_none() {
        effect.locked = Some(resolve_scope(state, working, &effect.scope, effect.watcher));
    }
    let targets = effect.locked.clone().expect("locked just set");
    for obj_id in targets {
        if let Some(d) = working.get_mut(&obj_id) {
            for m in &effect.changes {
                if layer_of(m, effect.is_cda) == Some(layer) {
                    apply(m, effect.controller, d);
                }
            }
        }
    }
}

/// Speculatively apply `d`'s `layer` ops to a clone of `working`, WITHOUT
/// locking the real effect. Used only by `depends_on` to ask "what would the
/// board look like if `d` applied first?".
fn probe_apply(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    d: &ActiveEffect,
    layer: Layer,
) -> BTreeMap<ObjectId, DerivedObject> {
    let mut probe = working.clone();
    for obj_id in effect_targets(state, working, d) {
        if let Some(dobj) = probe.get_mut(&obj_id) {
            for m in &d.changes {
                if layer_of(m, d.is_cda) == Some(layer) {
                    apply(m, d.controller, dobj);
                }
            }
        }
    }
    probe
}

/// Whether `e` depends on `d` within `layer` ([CR#613.8a]): applying `d`'s
/// `layer` ops would change the set of objects `e` applies to.
///
/// This detects *affected-set* dependency — the dominant case the dependency
/// system exists for (type/control/color filters that decide what another
/// effect catches). A *value* dependency — an op whose magnitude reads a
/// characteristic `d` changes — is a documented limitation, not yet detected.
///
/// Two short-circuits: a locked effect's affected set is fixed ([CR#611.2c]) so
/// nothing can change it; and per [CR#613.8a] clause (c) an effect and a CDA
/// are never dependent unless both are CDAs.
fn depends_on(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    e: &ActiveEffect,
    d: &ActiveEffect,
    layer: Layer,
) -> bool {
    if e.is_cda != d.is_cda || e.locked.is_some() {
        return false;
    }
    let before: BTreeSet<ObjectId> = effect_targets(state, working, e).into_iter().collect();
    let probe = probe_apply(state, working, d, layer);
    let after: BTreeSet<ObjectId> = effect_targets(state, &probe, e).into_iter().collect();
    before != after
}

// ---------------------------------------------------------------------------
// GameState::layers
// ---------------------------------------------------------------------------

impl GameState {
    /// Derive every object's characteristics ([CR#613.5]: fresh, continuously).
    /// Recomputed on each call; callers that need many lookups call once and
    /// index the returned view. (Caching is a noted later optimization.)
    ///
    /// Only card-backed objects have a characteristics entry; player proxies
    /// ([CR#109]) are skipped — they are not game objects with a characteristic
    /// set in the rules sense.
    #[must_use]
    pub fn layers(&self) -> LayeredView {
        // Step 1: base values for all card-backed objects.
        let mut working: BTreeMap<ObjectId, DerivedObject> = BTreeMap::new();
        for obj in self.objects.iter() {
            if obj.card_id().is_none() {
                continue; // player proxy — no characteristics
            }
            working.insert(obj.id, base_values(self, obj.id));
        }

        // Step 2: gather all active static Modify effects.
        let mut effects = gather(self);

        // Step 3: iterate layers in order, applying each effect's ops that
        // belong to this layer ([CR#613.3]).
        // SEAM [CR#305.6]: after L4 resolves, subtype-conferred abilities
        // (e.g. a land subtype granting a mana ability) should be reinjected
        // before L6 ability additions/removals are applied. No fixture exercises
        // this path yet; it's a documented gap for a later task.
        for layer in [
            Layer::L2,
            Layer::L3,
            Layer::L4,
            Layer::L5,
            Layer::L6,
            Layer::L7a,
            Layer::L7b,
            Layer::L7c,
            Layer::L7d,
        ] {
            // Effects with at least one op in this layer.
            let mut pending: Vec<usize> = (0..effects.len())
                .filter(|&i| {
                    effects[i]
                        .changes
                        .iter()
                        .any(|m| layer_of(m, effects[i].is_cda) == Some(layer))
                })
                .collect();

            // Base order: CDAs first ([CR#613.3]), then by timestamp
            // ([CR#613.7]). This is the tiebreaker among independent effects and
            // the fallback inside a dependency loop ([CR#613.8b]).
            pending.sort_by_key(|&i| (!effects[i].is_cda, effects[i].timestamp));

            // Apply in dependency order ([CR#613.8b,613.8c]): repeatedly take the
            // earliest pending effect that depends on no other pending effect,
            // re-evaluating after each application. If none is independent (a
            // dependency loop), fall back to timestamp order — the first pending,
            // since `pending` is sorted ([CR#613.8b]).
            while !pending.is_empty() {
                let pos = pending
                    .iter()
                    .position(|&e| {
                        !pending.iter().any(|&d| {
                            d != e && depends_on(self, &working, &effects[e], &effects[d], layer)
                        })
                    })
                    .unwrap_or(0);
                let i = pending.remove(pos);
                apply_effect_in_layer(self, &mut working, &mut effects[i], layer);
            }
            // [CR#122.1a,613.4c,613.1f]: +1/+1 / -1/-1 P/T and keyword counters
            // are data-driven now — gathered as counter-conferred `Continuous`
            // boosts (see `gather` + `bake_counter_counts`) and applied as
            // ordinary layer `Modification`s, so no hardcoded 7c counter read
            // remains here.
        }

        LayeredView(working)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Count;
    use deckmaste_core::Duration;
    use deckmaste_core::Filter;
    use deckmaste_core::KeywordAbility;
    use deckmaste_core::Modification;
    use deckmaste_core::Scope;
    use deckmaste_core::StatValue;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use super::ContinuousEffect;
    use super::ScopeResolved;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
        })
    }

    /// A self-anthem `Static`: "creatures get +2/+2" — matches the carrying
    /// creature itself (a `Matching` floating scope, no Stage-3 source-relative
    /// reference needed). Wrapped or not per `innate`.
    fn pump_static(innate: bool) -> Ability {
        let s = Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Modify {
                of: Scope::Matching(Filter::Characteristic(CharacteristicFilter::Type(
                    Type::Creature,
                ))),
                changes: vec![
                    Modification::AddPower(Count::Literal(2)),
                    Modification::AddToughness(Count::Literal(2)),
                ],
            }],
            characteristic_defining: false,
        });
        if innate { Ability::Innate(Box::new(s)) } else { s }
    }

    /// Mint a 2/2 creature carrying `abilities` onto the battlefield (player
    /// 0).
    fn creature_on_field(mut state: GameState, abilities: Vec<Ability>) -> (GameState, ObjectId) {
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        let card = Card::Normal(CardFace {
            name: "Test Creature".into(),
            types: vec![Type::Creature],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            abilities,
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }

    /// Lock a `LoseAllAbilities` (layer-6) continuous effect onto `id`.
    fn lose_all_abilities(state: &mut GameState, id: ObjectId) {
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![id]),
            changes: vec![Modification::LoseAllAbilities],
            duration: Duration::EndOfGame,
            is_cda: false,
        });
    }

    /// Whether `id`'s DERIVED ability list (what the SBA sweep,
    /// `attachment_legal`, and the deontic reads consult) carries a `Static`
    /// (peeling `Innate`). This is the surface layer-6 ability removal acts on
    /// — gather's effect-source collection reads PRINTED abilities to break the
    /// `layers()` recursion (documented in `gather`), so P/T anthem application
    /// is intentionally NOT the right observable here.
    fn derived_has_static(state: &GameState, id: ObjectId) -> bool {
        state
            .layers()
            .get(id)
            .abilities
            .iter()
            .any(|a| matches!(a.peel_innate(), Ability::Static(_)))
    }

    /// [CR#113.12]: `LoseAllAbilities` strips a normal granted ability (Trample)
    /// but RETAINS an `Innate` static — the Innate static stays in the derived
    /// ability list (so the SBA / legality reads still see it), while the
    /// normal keyword is gone.
    #[test]
    fn innate_static_survives_lose_all_abilities() {
        let (mut state, id) = creature_on_field(
            game(),
            vec![pump_static(true), Ability::Keyword(KeywordAbility::Trample)],
        );

        // Before removal: the Innate static and Trample are both derived.
        assert!(
            derived_has_static(&state, id),
            "innate static present pre-removal"
        );
        assert!(
            state
                .layers()
                .get(id)
                .abilities
                .iter()
                .any(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample))),
            "Trample present pre-removal"
        );

        lose_all_abilities(&mut state, id);

        // After removal: the Innate static SURVIVES in the derived list, the
        // normal keyword is GONE.
        assert!(
            derived_has_static(&state, id),
            "innate static survives LoseAllAbilities ([CR#113.12])"
        );
        assert!(
            !state
                .layers()
                .get(id)
                .abilities
                .iter()
                .any(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample))),
            "Trample removed by LoseAllAbilities"
        );
    }

    /// [CR#113.12]: an `Expanded(Innate(...))` (a macro-expanded Innate, the
    /// shape a Stage-4 subtype conferral may produce) ALSO survives
    /// `LoseAllAbilities` — `is_innate` looks through the `Expanded` provenance
    /// wrapper, so the retention guard keeps it. Observed via `is_innate` on
    /// the derived list (not `peel_innate`, which stops at `Expanded`).
    #[test]
    fn expanded_innate_survives_lose_all_abilities() {
        use deckmaste_core::Expansion;
        use deckmaste_core::ExpansionArgs;

        // `Expanded(Innate(Static(+2/+2 anthem)))`.
        let expanded_innate = Ability::Expanded(Expansion {
            name: "SubtypeRule".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(pump_static(true)),
        });
        assert!(expanded_innate.is_innate(), "the test fixture is innate");
        let (mut state, id) = creature_on_field(game(), vec![expanded_innate]);

        let innate_present = |state: &GameState| {
            state
                .layers()
                .get(id)
                .abilities
                .iter()
                .any(Ability::is_innate)
        };
        assert!(
            innate_present(&state),
            "Expanded(Innate) present pre-removal"
        );
        lose_all_abilities(&mut state, id);
        assert!(
            innate_present(&state),
            "Expanded(Innate) survives LoseAllAbilities ([CR#113.12])"
        );
    }

    /// Guard against over-retaining: a NORMAL (non-Innate) conferred static is
    /// removed from the derived ability list by `LoseAllAbilities` (so the SBA
    /// / legality reads no longer see it).
    #[test]
    fn normal_static_is_removed_by_lose_all_abilities() {
        let (mut state, id) = creature_on_field(game(), vec![pump_static(false)]);
        assert!(
            derived_has_static(&state, id),
            "normal static present pre-removal"
        );
        lose_all_abilities(&mut state, id);
        assert!(
            !derived_has_static(&state, id),
            "normal static removed from the derived list by LoseAllAbilities"
        );
    }

    /// [CR#113.12]: `LoseAbility(Trample)` removes the named keyword from the
    /// derived list but never an `Innate` ability.
    #[test]
    fn lose_ability_does_not_remove_innate() {
        let (mut state, id) = creature_on_field(
            game(),
            vec![pump_static(true), Ability::Keyword(KeywordAbility::Trample)],
        );
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![id]),
            changes: vec![Modification::LoseAbility("Trample".into())],
            duration: Duration::EndOfGame,
            is_cda: false,
        });
        assert!(
            derived_has_static(&state, id),
            "innate static survives LoseAbility"
        );
        assert!(
            !state
                .layers()
                .get(id)
                .abilities
                .iter()
                .any(|a| matches!(a, Ability::Keyword(KeywordAbility::Trample))),
            "Trample removed by LoseAbility(Trample)"
        );
    }

    /// [CR#113.12]: the card-facing `derive::abilities` view filters `Innate`
    /// OUT — an object whose only ability is `Innate` reads as having none,
    /// while a co-present normal ability is still visible.
    #[test]
    fn derive_abilities_filters_innate_out() {
        // Innate-only: card-facing list is empty.
        let (state, id) = creature_on_field(game(), vec![pump_static(true)]);
        assert!(
            crate::derive::abilities(&state, id).is_empty(),
            "an object whose only ability is Innate reads as having none"
        );

        // Innate + a normal keyword: only the keyword is visible.
        let (state, id) = creature_on_field(
            game(),
            vec![pump_static(true), Ability::Keyword(KeywordAbility::Trample)],
        );
        let facing = crate::derive::abilities(&state, id);
        assert_eq!(
            facing.len(),
            1,
            "only the non-Innate ability is card-facing"
        );
        assert!(matches!(
            facing[0],
            Ability::Keyword(KeywordAbility::Trample)
        ));
    }

    /// A host-targeting static: "enchanted/equipped creature gets +N/+N",
    /// authored as `Modify { of: Of(AttachHostOf(This)), changes: [+N/+N] }`.
    /// The source-relative `AttachHostOf(This)` reference is exactly the
    /// Stage-3 seam this task resolves.
    fn host_pump_static(n: u32) -> Ability {
        use deckmaste_core::Reference;
        Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Modify {
                of: Scope::Of(Reference::AttachHostOf(Box::new(Reference::This))),
                changes: vec![
                    Modification::AddPower(Count::Literal(n)),
                    Modification::AddToughness(Count::Literal(n)),
                ],
            }],
            characteristic_defining: false,
        })
    }

    /// Mint a bare (non-creature) permanent carrying `abilities` onto the
    /// battlefield (player 0) — used as an attachment whose static targets its
    /// host. No power/toughness, so it never gets caught by an anthem itself.
    fn permanent_on_field(mut state: GameState, abilities: Vec<Ability>) -> (GameState, ObjectId) {
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        let card = Card::Normal(CardFace {
            name: "Test Attachment".into(),
            types: vec![Type::Enchantment],
            abilities,
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }

    /// Stage 3 end-to-end: a static whose affected set is a *source-relative
    /// reference* (`Of(AttachHostOf(This))`) lands on the attached host. Before
    /// this task `Of`/`These` resolved to `Locked(empty)` so the bonus
    /// vanished; now the gather resolves `This = obj.id` and reads the
    /// attachment→host link, so the 2/2 host derives as 3/3 ([CR#613.6]).
    #[test]
    fn static_lands_on_host() {
        // Host: a plain 2/2 creature, no abilities of its own.
        let (state, host) = creature_on_field(game(), vec![]);
        // Attachment: a permanent carrying "host gets +1/+1".
        let (mut state, attachment) = permanent_on_field(state, vec![host_pump_static(1)]);
        // Establish the relation (the Stage-1 verb does this in real play).
        state.objects.obj_mut(attachment).attached_to = Some(host);

        let view = state.layers();
        assert_eq!(
            view.power(host),
            Some(3),
            "the host-targeting +1/+1 static landed on the host's power"
        );
        assert_eq!(
            view.toughness(host),
            Some(3),
            "the host-targeting +1/+1 static landed on the host's toughness"
        );
    }

    /// The attachment's own characteristics are untouched by its host-targeting
    /// static — `AttachHostOf(This)` resolves to the host, not back to self.
    #[test]
    fn host_static_does_not_buff_the_attachment_itself() {
        let (state, host) = creature_on_field(game(), vec![]);
        let (mut state, attachment) = permanent_on_field(state, vec![host_pump_static(1)]);
        state.objects.obj_mut(attachment).attached_to = Some(host);

        let view = state.layers();
        // The attachment has no P/T at all — and certainly isn't pumped.
        assert_eq!(
            view.power(attachment),
            None,
            "the attachment is not its own host"
        );
    }

    /// Guard: an UNATTACHED attachment's host-targeting static resolves to no
    /// host, so nothing is buffed (the resolver yields an empty locked set).
    #[test]
    fn host_static_with_no_host_buffs_nothing() {
        let (state, host) = creature_on_field(game(), vec![]);
        let (state, _attachment) = permanent_on_field(state, vec![host_pump_static(1)]);

        let view = state.layers();
        assert_eq!(
            view.power(host),
            Some(2),
            "an unattached attachment buffs no host"
        );
    }

    /// M1: `These` dedups its resolved id list. A static carrying
    /// `Of: These([This, This])` with an *additive* op must apply that op only
    /// once — without the `BTreeSet` dedup in gather the duplicate reference
    /// would pump the source twice (+2 instead of +1).
    #[test]
    fn these_dedups_overlapping_references() {
        use deckmaste_core::Reference;

        let dup_pump = Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Modify {
                of: Scope::These(vec![Reference::This, Reference::This]),
                changes: vec![Modification::AddPower(Count::Literal(1))],
            }],
            characteristic_defining: false,
        });
        let (state, id) = creature_on_field(game(), vec![dup_pump]);
        assert_eq!(
            state.layers().power(id),
            Some(3),
            "These([This, This]) applies +1 ONCE: base 2 → 3 (not 4)"
        );
    }

    /// Load a card from the wizards corpus with the builtin keyword/subtype
    /// macros in scope (the real expansion path), forcing it onto the
    /// battlefield as player 0's permanent. Reads only the macro corpus plus
    /// the single named card file — not the whole 30k-card directory.
    fn wizards_permanent(name: &str) -> (GameState, ObjectId) {
        use std::path::Path;

        use deckmaste_cards::plugin::Plugin;

        let plugin = Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/wizards"),
        )
        .expect("load wizards plugin (builtin prelude)");
        let card = Arc::new(
            plugin
                .card(name)
                .unwrap_or_else(|e| panic!("load {name}: {e:?}")),
        );

        let mut state = game();
        let card_id = state.cards.push(card, PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }

    /// C1 regression: a Changeling permanent must NOT panic in
    /// `state.layers()`. `plugins/builtin/macros/keyword/Changeling.ron`
    /// confers `Modify(of: Of(This), changes: [AllCreatureTypes])`; now
    /// that `Of(This)` resolves source-relative in gather (Stage 3), Layer
    /// 4 runs the `AllCreatureTypes` arm. That arm was a `todo!()` (panic)
    /// before this fix and is now a no-op stub (every-creature-type fill is
    /// deferred — [CR#702.73a]). The derived characteristics are returned
    /// and, with the stub, carry only the printed subtype (no all-types
    /// fill yet).
    #[test]
    fn changeling_layers_does_not_panic() {
        // Avian Changeling: `Keyword(Changeling)` + `Keyword(Flying)`, a
        // 2/2 white Shapeshifter — exercises the keyword-macro path.
        let (state, id) = wizards_permanent("Avian Changeling");

        // The critical assertion: this call previously PANICKED.
        let view = state.layers();
        let c = view.get(id);

        // Sanity: the object derived (P/T preserved through the layer pass).
        assert_eq!(view.power(id), Some(2));
        assert_eq!(view.toughness(id), Some(2));
        // No-op stub: only the printed Shapeshifter subtype is present — the
        // every-creature-type fill is deliberately NOT performed yet.
        assert!(
            c.subtypes.iter().any(|s| s.name == "Shapeshifter"),
            "printed Shapeshifter subtype survives"
        );
        assert!(
            c.subtypes.len() < 50,
            "AllCreatureTypes is a no-op stub (no every-creature-type fill), \
             got {} subtypes",
            c.subtypes.len()
        );
    }

    /// Devoid runs the IMPLEMENTED `SetColors([])` arm (not a panic): a devoid
    /// permanent derives colorless ([CR#604.3,105.2c]). This is the intended,
    /// correct behavior un-gated alongside Changeling by Stage 3.
    #[test]
    fn devoid_derives_colorless() {
        // Havoc Sower: a Black creature with `Keyword(Devoid)` — Devoid's CDA
        // `Modify(of: Of(This), changes: [SetColors([])])` makes it colorless.
        let (state, id) = wizards_permanent("Havoc Sower");

        let view = state.layers();
        assert!(
            view.get(id).colors.is_empty(),
            "devoid derives colorless (empty color set), got {:?}",
            view.get(id).colors
        );
    }

    /// Mint a 2/2 creature of the named subtype carrying `abilities`,
    /// controlled by `controller`. The carrier-threading lord tests need
    /// both a subtype and a chooseable controller, which
    /// `creature_on_field` (player 0, no subtype) does not give.
    fn typed_creature(
        mut state: GameState,
        subtype: &str,
        controller: PlayerId,
        abilities: Vec<Ability>,
    ) -> (GameState, ObjectId) {
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        use deckmaste_core::Subtype;
        let card = Card::Normal(CardFace {
            name: "Test Tribe".into(),
            types: vec![Type::Creature],
            subtypes: vec![Subtype {
                name: subtype.into(),
                types: vec![Type::Creature],
                confers: vec![],
            }],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            abilities,
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            controller,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }

    /// The canonical tribal-lord static: "other Goblins you control get +1/+1",
    /// i.e. `Matching(AllOf([Creature, Not(Ref(This)), Subtype("Goblin"),
    /// ControlledBy(Ref(You))]))`. Its scope names both `~` (`Ref(This)`) and
    /// `you` (`Ref(You)`), which are exactly the carrier-bound refs the derived
    /// path must anchor against the host permanent.
    fn goblin_lord_static() -> Ability {
        use deckmaste_core::Reference;
        use deckmaste_core::RelationFilter;
        Ability::Static(StaticAbility {
            condition: None,
            effects: vec![StaticEffect::Modify {
                of: Scope::Matching(Filter::AllOf(vec![
                    Filter::creature(),
                    Filter::Not(Box::new(Filter::Ref(Reference::This))),
                    Filter::Characteristic(CharacteristicFilter::Subtype("Goblin".into())),
                    Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                        Reference::You,
                    )))),
                ])),
                changes: vec![
                    Modification::AddPower(Count::Literal(1)),
                    Modification::AddToughness(Count::Literal(1)),
                ],
            }],
            characteristic_defining: false,
        })
    }

    /// engine-static-scope-carrier: the derived/continuous-effect path threads
    /// the static's source permanent as the carrier, so a tribal-lord scope
    /// that names `~`/`you` (`Ref(This)`/`Ref(You)`) resolves against the
    /// host instead of panicking. The lord buffs ANOTHER controlled Goblin
    /// +1/+1, does NOT buff itself (`Not(Ref(This))`), and does NOT buff an
    /// opponent-controlled Goblin (`ControlledBy(Ref(You))`).
    #[test]
    fn tribal_lord_buffs_other_controlled_goblins() {
        // Lord: a Goblin carrying the "other Goblins you control get +1/+1"
        // static, controlled by player 0.
        let (state, lord) =
            typed_creature(game(), "Goblin", PlayerId(0), vec![goblin_lord_static()]);
        // A second Goblin, same controller — should be buffed.
        let (state, ally) = typed_creature(state, "Goblin", PlayerId(0), vec![]);
        // An opponent's Goblin — NOT buffed (scoped to the controller).
        let (state, foe) = typed_creature(state, "Goblin", PlayerId(1), vec![]);

        let view = state.layers();
        assert_eq!(
            view.power(ally),
            Some(3),
            "another controlled Goblin gets +1/+1"
        );
        assert_eq!(view.toughness(ally), Some(3), "…on toughness too");
        assert_eq!(
            view.power(lord),
            Some(2),
            "the lord does not buff itself (Not(Ref(This)))"
        );
        assert_eq!(
            view.power(foe),
            Some(2),
            "an opponent's Goblin is not buffed (ControlledBy(Ref(You)))"
        );
    }

    /// engine-static-scope-carrier: a spell-built FLOATING scope (Overrun's
    /// `Continuously(Modify(of: Matching(AllOf([Creature,
    /// ControlledBy(Ref(You))] )), …))`) names `you` and so also rode the
    /// carrier `todo!`. The floating effect carries its controller, so
    /// `Ref(You)` resolves and the player's own creatures are buffed
    /// without panic.
    #[test]
    fn floating_controlled_by_you_scope_resolves() {
        use deckmaste_core::Reference;
        use deckmaste_core::RelationFilter;

        // Two plain creatures: one player 0 controls, one player 1 controls.
        let (state, mine) = typed_creature(game(), "Beast", PlayerId(0), vec![]);
        let (mut state, theirs) = typed_creature(state, "Beast", PlayerId(1), vec![]);

        // An Overrun-shaped floating effect controlled by player 0.
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Floating(Filter::AllOf(vec![
                Filter::creature(),
                Filter::Relation(RelationFilter::ControlledBy(Box::new(Filter::Ref(
                    Reference::You,
                )))),
            ])),
            changes: vec![
                Modification::AddPower(Count::Literal(2)),
                Modification::AddToughness(Count::Literal(2)),
            ],
            duration: Duration::EndOfGame,
            is_cda: false,
        });

        let view = state.layers();
        assert_eq!(
            view.power(mine),
            Some(4),
            "the effect controller's creature is buffed (+2/+2)"
        );
        assert_eq!(
            view.power(theirs),
            Some(2),
            "the opponent's creature is untouched (ControlledBy(Ref(You)))"
        );
    }
}
