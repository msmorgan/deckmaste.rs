//! The continuous-effects layer system ([CR#613]): the one place an object's
//! characteristics are derived. Consumers read a [`LayeredView`], never the
//! printed face. Layers 4-7 (P/T sublayers 7a-7d) are implemented here;
//! layers 1-3 and the dependency tiebreaker ([CR#613.8]) are explicit seams
//! for later tasks.

use std::collections::BTreeMap;

use deckmaste_core::{
    Ability, Color, Count, Duration, Filter, Int, ManaSymbol, Modification, Scope, StaticEffect,
    Subtype, Supertype, Type, Zone,
};

use crate::object::{ObjectId, Timestamp};
use crate::state::GameState;

// ---------------------------------------------------------------------------
// Registry types (floating one-shot continuous effects)
// ---------------------------------------------------------------------------

/// An effect's target set after resolution. One-shot effects ([CR#611.2c])
/// lock `Of`/`These` to ids at creation; `Matching` stays floating.
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Characteristics {
    pub power: Option<Int>,
    pub toughness: Option<Int>,
    pub colors: Vec<Color>,
    pub card_types: Vec<Type>,
    pub subtypes: Vec<Subtype>,
    pub supertypes: Vec<Supertype>,
    pub abilities: Vec<Ability>,
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
    let face = crate::derive::face(state.def(id));
    // Colors come from the colored mana symbols in the cost ([CR#202.2]);
    // color_indicator is the fallback for objects with no mana cost.
    let mut colors: Vec<Color> = Vec::new();
    for c in face.mana_cost.iter().flat_map(symbol_colors) {
        if !colors.contains(&c) {
            colors.push(c);
        }
    }
    if colors.is_empty() {
        colors.clone_from(&face.color_indicator);
    }
    Characteristics {
        power: base_stat(face.power.as_ref()),
        toughness: base_stat(face.toughness.as_ref()),
        colors,
        card_types: face.types.clone(),
        subtypes: face.subtypes.clone(),
        supertypes: face.supertypes.clone(),
        abilities: crate::derive::abilities(state, id)
            .into_iter()
            .cloned()
            .collect(),
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
        for ability in crate::derive::abilities(state, obj.id) {
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

/// Resolve a `ScopeResolved` against the current working set, returning the
/// target object ids.
///
/// `Floating` filters against PRINTED characteristics via `target::matches`
/// (correct for anthem-like effects whose filter describes printed types;
/// the derived-map upgrade is a documented later limitation).
///
/// `Locked` ids are returned as-is (pre-snapshotted at creation).
fn resolve_scope(
    state: &GameState,
    working: &BTreeMap<ObjectId, Characteristics>,
    scope: &ScopeResolved,
) -> Vec<ObjectId> {
    match scope {
        ScopeResolved::Floating(filter) => working
            .keys()
            .copied()
            .filter(|&id| crate::target::matches(state, id, filter))
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

/// Apply one `Modification` to `c` at its layer. Only P/T arms (7a-7d) are
/// implemented; type/color/ability arms (4-6) are empty stubs for later tasks.
/// ([CR#613.1b,613.1c,613.1d] deferred arms are also stubs.)
#[allow(clippy::match_same_arms)] // stub arms will diverge as later tasks fill them
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
        // --- Layer 4: type-changing (later task) ---
        Modification::SetCardTypes(_)
        | Modification::AddCardTypes(_)
        | Modification::SetSubtypes(_)
        | Modification::AddSubtypes(_)
        | Modification::SetSupertypes(_)
        | Modification::AddSupertypes(_)
        | Modification::BecomeBasicLandType(_) => {}
        // --- Layer 5: color-changing (later task) ---
        Modification::SetColors(_) | Modification::AddColors(_) => {}
        // --- Layer 6: ability-adding/removing (later task) ---
        Modification::GainAbility(_)
        | Modification::LoseAbility(_)
        | Modification::LoseAllAbilities
        | Modification::CantHaveAbility(_) => {}
        // --- Deferred ([CR#613.1b,613.1c,613.1d]) ---
        Modification::SetController(_)
        | Modification::SetText(_)
        | Modification::SetBaseLoyalty(_)
        | Modification::SetBaseDefense(_) => {
            // [CR#613.1b/c/d] deferred
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
        }

        LayeredView(working)
    }
}
