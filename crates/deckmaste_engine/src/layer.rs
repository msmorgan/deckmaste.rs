//! The continuous-effects layer system ([CR#613]): the one place an object's
//! characteristics are derived. Consumers read a [`LayeredView`], never the
//! printed face. Layers 4-7 (P/T sublayers 7a-7d) are implemented here;
//! layers 1-3 and the dependency tiebreaker ([CR#613.8]) are explicit seams
//! for later tasks.

use std::collections::BTreeMap;
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
use crate::object::Timestamp;
use crate::state::GameState;

// ---------------------------------------------------------------------------
// Registry types (floating one-shot continuous effects)
// ---------------------------------------------------------------------------

/// An effect's resolved target set, used throughout the pipeline.
/// `Locked` holds ids snapshotted at creation ([CR#611.2c], one-shot
/// `Of`/`These`). `Floating` holds a filter re-evaluated against the derived
/// map each layer (static `Matching`, and the deferred static-`Of`/`These` seam
/// currently produces `Locked(empty)`).
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
    pub scope: ScopeResolved,
    pub changes: Vec<Modification>,
    pub duration: Duration,
    pub is_cda: bool,
}

/// An object's derived characteristics ([CR#613]). `power`/`toughness` are
/// `None` for objects with no P/T; a printed `*` with no CDA resolves to `0`
/// ([CR#208.2a]).
/// All list-valued fields are `Arc`'d copy-on-write: base values share the
/// per-card caches built at `Cards::push`, and a mutating layer op clones via
/// `Arc::make_mut` only for the objects an effect actually touches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Characteristics {
    pub power: Option<Int>,
    pub toughness: Option<Int>,
    pub colors: Arc<Vec<Color>>,
    pub card_types: Arc<Vec<Type>>,
    pub subtypes: Arc<Vec<Subtype>>,
    pub supertypes: Arc<Vec<Supertype>>,
    pub abilities: Arc<Vec<Ability>>,
    /// Ability names the object can't have or gain ([CR#613.1f]).
    /// Populated by `CantHaveAbility`; consulted by `GainAbility`.
    pub cant_have: Vec<Ident>,
}

/// Every live object's derived characteristics, computed in one pass.
#[derive(Debug, Clone)]
pub struct LayeredView(BTreeMap<ObjectId, Characteristics>);

impl LayeredView {
    /// Returns the derived characteristics for `id`.
    ///
    /// # Panics
    ///
    /// Panics if `id` was not a live object when the view was computed.
    #[must_use]
    pub fn get(&self, id: ObjectId) -> &Characteristics {
        self.0.get(&id).expect("live ObjectId in LayeredView")
    }

    #[must_use]
    pub fn power(&self, id: ObjectId) -> Option<Int> { self.get(id).power }

    #[must_use]
    pub fn toughness(&self, id: ObjectId) -> Option<Int> { self.get(id).toughness }
}

/// Resolve a printed `StatValue` to a base number. `*` with no CDA is `0`
/// ([CR#208.2a]); CDAs (layer 7a) overwrite this later.
fn base_stat(v: Option<&deckmaste_core::StatValue>) -> Option<Int> {
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

/// Base characteristics from the printed face ([CR#613.1]): the object's
/// characteristics before any continuous effect. v1 reads the printed face
/// (no copy/face-down handling).
fn base_values(state: &GameState, id: ObjectId) -> Characteristics {
    let card = state.objects.obj(id).card_id().expect("card-backed object");
    let instance = state.cards.get(card);
    let face = crate::derive::face(&instance.def);
    Characteristics {
        power: base_stat(face.power.as_ref()),
        toughness: base_stat(face.toughness.as_ref()),
        colors: Arc::clone(&instance.colors),
        card_types: Arc::clone(&instance.card_types),
        subtypes: Arc::clone(&instance.subtypes),
        supertypes: Arc::clone(&instance.supertypes),
        abilities: Arc::clone(&instance.printed),
        cant_have: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Layer ordering
// ---------------------------------------------------------------------------

/// The layer a `Modification` op lives in ([CR#613.3,613.4]).
/// Only the variants needed for P/T sublayers 7a-7d are enumerated; types,
/// colors, and abilities (4-6) are explicit seams for later tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Layer {
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
/// Returns `None` for ops that are deferred to later tasks
/// ([CR#613.1b,613.1c,613.1d]).
fn layer_of(m: &Modification, is_cda: bool) -> Option<Layer> {
    match m {
        // Layer 4: type-changing ([CR#613.4a]).
        Modification::SetCardTypes(_)
        | Modification::AddCardTypes(_)
        | Modification::SetSubtypes(_)
        | Modification::AddSubtypes(_)
        | Modification::SetSupertypes(_)
        | Modification::AddSupertypes(_)
        | Modification::BecomeBasicLandType(_) => Some(Layer::L4),
        // Layer 5: color-changing ([CR#613.4b]).
        Modification::SetColors(_) | Modification::AddColors(_) => Some(Layer::L5),
        // Layer 6: ability-adding/removing ([CR#613.4c]).
        Modification::GainAbility(_)
        | Modification::LoseAbility(_)
        | Modification::LoseAllAbilities
        | Modification::CantHaveAbility(_) => Some(Layer::L6),
        // Layer 7a or 7b: SetPower/SetToughness ([CR#613.4d]).
        // CDAs ([CR#604.3]) apply in 7a; all other set-ops apply in 7b.
        Modification::SetPower(_) | Modification::SetToughness(_) => {
            if is_cda {
                Some(Layer::L7a)
            } else {
                Some(Layer::L7b)
            }
        }
        // Layer 7c: P/T modification ([CR#613.4d]).
        Modification::AddPower(_) | Modification::AddToughness(_) => Some(Layer::L7c),
        // Layer 7d: switch ([CR#613.4d]).
        Modification::SwitchPowerToughness => Some(Layer::L7d),
        // Deferred to later tasks ([CR#613.1b,613.1c,613.1d]).
        Modification::SetController(_)
        | Modification::SetText(_)
        | Modification::SetBaseLoyalty(_)
        | Modification::SetBaseDefense(_) => None,
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
    scope: ScopeResolved,
    changes: Vec<Modification>,
    /// Locked target set: `None` until first applied layer resolves the scope
    /// ([CR#613.6] — scope is locked at first layer of application).
    locked: Option<Vec<ObjectId>>,
}

/// Collect all active static `Modify` effects from battlefield permanents,
/// plus any floating one-shot effects from `state.continuous`.
///
/// Only unconditional effects are gathered here; `sa.condition` evaluation
/// is a documented seam for a later task.
fn gather(state: &GameState) -> Vec<ActiveEffect> {
    let mut effects = Vec::new();
    for obj in state.objects.iter() {
        if obj.card_id().is_none() {
            continue; // player proxy — no static abilities
        }
        // Static abilities function only on the battlefield ([CR#611.3a]).
        if obj.zone != Some(Zone::Battlefield) {
            continue;
        }
        let timestamp = obj.timestamp;
        // v1: uses printed_abilities (not the derived view) to break the
        // layers() → derive::abilities → layers() recursion. As a result, a
        // static ability that is itself *granted* by a layer-6 effect won't be
        // re-gathered as an effect source (no fixpoint). No fixture requires that.
        for ability in crate::derive::printed_abilities(state, obj.id) {
            let Ability::Static(sa) = ability else { continue };
            // Conditions skipped — a seam for later ([CR#604.3]).
            for effect in &sa.effects {
                let StaticEffect::Modify { of, changes } = effect else { continue };
                // Convert Scope to ScopeResolved. Static Of/These reference
                // resolution stays a seam — gather has no Frame to eval
                // references; these are left as Locked(empty) matching the
                // prior behavior. Floating Matching scopes stay floating.
                let scope = match of {
                    Scope::Matching(f) => ScopeResolved::Floating(f.clone()),
                    Scope::Of(_) | Scope::These(_) => ScopeResolved::Locked(Vec::new()),
                };
                effects.push(ActiveEffect {
                    timestamp,
                    is_cda: sa.characteristic_defining,
                    scope,
                    changes: changes.clone(),
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
            scope: ce.scope.clone(),
            changes: ce.changes.clone(),
            locked: None,
        });
    }
    effects
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
fn matches_derived(
    state: &GameState,
    working: &BTreeMap<ObjectId, Characteristics>,
    id: ObjectId,
    filter: &deckmaste_core::Filter,
) -> bool {
    use deckmaste_core::CharacteristicFilter;
    use deckmaste_core::Filter;
    // `Any` is a wildcard sentinel — it must always match, even for ids that
    // aren't in `working` (e.g. player proxies). Checked before the map lookup.
    if let Filter::Any = filter {
        return true;
    }
    let Some(c) = working.get(&id) else { return false };
    match filter {
        Filter::Any => unreachable!("handled above"),
        Filter::Characteristic(CharacteristicFilter::Type(t)) => c.card_types.contains(t),
        Filter::Characteristic(CharacteristicFilter::Supertype(s)) => c.supertypes.contains(s),
        Filter::Characteristic(CharacteristicFilter::ColorIs(col)) => c.colors.contains(col),
        // Subtype matching against derived: `working[id].subtypes` are Subtype
        // structs; the filter carries an Ident name. Match by name.
        Filter::Characteristic(CharacteristicFilter::Subtype(name)) => {
            c.subtypes.iter().any(|s| &s.name == name)
        }
        // `HasAbility` is derivable from the working map — check the derived
        // ability list. `Named` and `Stat` are not straightforwardly derivable
        // and fall through to `target::matches` (which is unimplemented there
        // today — pre-existing, not reachable by current fixtures).
        Filter::Characteristic(CharacteristicFilter::HasAbility(name)) => {
            c.abilities.iter().any(|a| ability_is_named(a, name))
        }
        // Combinators: recurse through matches_derived so characteristic leaves
        // see the derived map.
        Filter::AllOf(fs) => fs.iter().all(|f| matches_derived(state, working, id, f)),
        Filter::OneOf(fs) => fs.iter().any(|f| matches_derived(state, working, id, f)),
        Filter::Not(f) => !matches_derived(state, working, id, f),
        Filter::Expanded(e) => matches_derived(state, working, id, &e.value),
        // Named / Stat and everything else (zone, status, kind, relations, …):
        // delegate to the printed matcher.
        _ => crate::target::matches(state, id, filter),
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
    working: &BTreeMap<ObjectId, Characteristics>,
    scope: &ScopeResolved,
) -> Vec<ObjectId> {
    match scope {
        ScopeResolved::Floating(filter) => working
            .keys()
            .copied()
            .filter(|&id| matches_derived(state, working, id, filter))
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
fn ability_is_named(a: &Ability, name: &Ident) -> bool {
    match a {
        Ability::Keyword(kw) => name == kw.as_str(),
        Ability::Expanded(e) => ability_is_named(&e.value, name),
        _ => false,
    }
}

/// Apply one `Modification` to `c` at its layer.
/// Layers 4 (types/supertypes), 5 (colors), 6 (abilities), and 7a-7d (P/T) are
/// implemented. Subtypes ([CR#613.1a]) and `BecomeBasicLandType` ([CR#305.7])
/// are explicit deferred stubs; controller/text/loyalty/defense
/// ([CR#613.1b,613.1c,613.1d]) are also stubs.
#[allow(clippy::match_same_arms)] // deferred stub arms will diverge as later tasks fill them
fn apply(m: &Modification, c: &mut Characteristics) {
    match m {
        // --- Layer 7a / 7b: set base P/T ---
        Modification::SetPower(n) => c.power = Some(eval_count(n)),
        Modification::SetToughness(n) => c.toughness = Some(eval_count(n)),
        // --- Layer 7c: add/subtract P/T ---
        Modification::AddPower(n) => {
            c.power = Some(c.power.unwrap_or(0) + eval_count(n));
        }
        Modification::AddToughness(n) => {
            c.toughness = Some(c.toughness.unwrap_or(0) + eval_count(n));
        }
        // --- Layer 7d: switch ---
        Modification::SwitchPowerToughness => std::mem::swap(&mut c.power, &mut c.toughness),
        // --- Layer 4: type-changing ([CR#613.4a]) ---
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
        // [CR#613.1a] subtype-set deferred: Ident→Subtype reconcile (no fixture yet)
        // `Modification::SetSubtypes`/`AddSubtypes` carry `Vec<Ident>` but
        // `Characteristics::subtypes` holds `Vec<Subtype>` (structs with confers/types).
        // There is no clean `Ident → Subtype` conversion without plugin data, so these
        // arms are left as explicit no-ops until a fixture exercises them.
        Modification::SetSubtypes(_) | Modification::AddSubtypes(_) => {}
        // [CR#305.7] deferred: replace land subtypes + strip abilities + grant basic
        // mana ability (no fixture yet). Do NOT implement the mana-ability construction.
        Modification::BecomeBasicLandType(_) => {}
        // --- Layer 5: color-changing ([CR#613.4b]) ---
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
            if !c.cant_have.iter().any(|n| ability_is_named(a, n)) {
                Arc::make_mut(&mut c.abilities).push((**a).clone());
            }
        }
        Modification::LoseAllAbilities => c.abilities = Arc::new(Vec::new()),
        Modification::LoseAbility(name) => {
            Arc::make_mut(&mut c.abilities).retain(|x| !ability_is_named(x, name));
        }
        Modification::CantHaveAbility(name) => {
            // Remove any already-present instance of the named ability, then
            // record the prohibition so future GainAbility skips it.
            Arc::make_mut(&mut c.abilities).retain(|x| !ability_is_named(x, name));
            c.cant_have.push(*name);
        }
        // --- Deferred ([CR#613.1b,613.1c,613.1d]) ---
        Modification::SetController(_)
        | Modification::SetText(_)
        | Modification::SetBaseLoyalty(_)
        | Modification::SetBaseDefense(_) => {
            // [CR#613.1b,613.1c,613.1d] deferred
        }
    }
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
        let mut working: BTreeMap<ObjectId, Characteristics> = BTreeMap::new();
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
            Layer::L4,
            Layer::L5,
            Layer::L6,
            Layer::L7a,
            Layer::L7b,
            Layer::L7c,
            Layer::L7d,
        ] {
            // Indices of effects that have at least one op in this layer.
            let mut order: Vec<usize> = (0..effects.len())
                .filter(|&i| {
                    effects[i]
                        .changes
                        .iter()
                        .any(|m| layer_of(m, effects[i].is_cda) == Some(layer))
                })
                .collect();

            // Sort: CDAs first ([CR#613.3]), then by timestamp ([CR#613.7]).
            // Dependency ordering ([CR#613.8]) is a deferred seam.
            order.sort_by_key(|&i| (!effects[i].is_cda, effects[i].timestamp));

            for i in order {
                // Lock the target set at first applied layer ([CR#613.6]).
                if effects[i].locked.is_none() {
                    let targets = resolve_scope(self, &working, &effects[i].scope);
                    effects[i].locked = Some(targets);
                }
                // `locked` is always `Some` here — either it was set just above
                // or it was set in an earlier layer iteration.
                let Some(targets) = effects[i].locked.as_deref() else {
                    unreachable!("locked is always set before this point")
                };
                // Copy targets + is_cda to release the borrow on effects[i] so the
                // inner loop can borrow effects[i].changes.
                let targets: Vec<ObjectId> = targets.to_vec();
                let is_cda = effects[i].is_cda;
                for obj_id in targets {
                    if let Some(c) = working.get_mut(&obj_id) {
                        for m in &effects[i].changes {
                            if layer_of(m, is_cda) == Some(layer) {
                                apply(m, c);
                            }
                        }
                    }
                }
            }

            // [CR#613.4c],[CR#122]: +1/+1 and -1/-1 counters modify P/T in
            // layer 7c, after 7b set-effects and before 7d switch. Applied
            // directly (not as Modifications) because Count is unsigned;
            // 7c additions commute, so order vs. other 7c effects is irrelevant.
            if layer == Layer::L7c {
                // Counters live on the object (not derived); layers() is &self so no mid-pass
                // race.
                for (&id, c) in &mut working {
                    let counters = &self.objects.obj(id).counters;
                    let plus = counters.get("+1/+1").copied().unwrap_or(0).cast_signed();
                    let minus = counters.get("-1/-1").copied().unwrap_or(0).cast_signed();
                    let delta = plus - minus;
                    if delta != 0 {
                        if let Some(p) = c.power {
                            c.power = Some(p + delta);
                        }
                        if let Some(t) = c.toughness {
                            c.toughness = Some(t + delta);
                        }
                    }
                }
                // Keyword counters ([CR#122] payloads) are DEFERRED: no
                // counter-decl registry is wired in the engine
                // yet. (Follow-up: gather a counter's CounterDecl.payload
                // StaticEffect into layer 6 once the registry exists.)
            }
        }

        LayeredView(working)
    }
}
