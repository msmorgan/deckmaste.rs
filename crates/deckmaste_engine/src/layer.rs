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
use deckmaste_core::CollectionOp;
use deckmaste_core::Color;
use deckmaste_core::Condition;
use deckmaste_core::Count;
use deckmaste_core::Countable;
use deckmaste_core::Duration;
use deckmaste_core::Ident;
use deckmaste_core::Int;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::Predicate;
use deckmaste_core::Property;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::StatValue;
use deckmaste_core::StaticSpec;
use deckmaste_core::Subtype;
use deckmaste_core::Supertype;
use deckmaste_core::Type;
use deckmaste_core::TypeDef;
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
    Floating(Arc<deckmaste_core::Region<Predicate>>),
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
    /// Granted STATIC ROWS carried by a resolved one-shot ([CR#611.2c]) — a
    /// `Deontic` restriction ("target creature can't block this turn"), a
    /// `CostModifier`, a `CantHappen`. These are NOT characteristic
    /// modifications, so they never touch the hot layer pass (`layer::gather`
    /// reads only `changes`); they are consulted by the legality / cost /
    /// can't-happen readers directly. Held apart from `changes` because a
    /// resolved one-shot's restriction is not an ability of the object — a
    /// later ability-removal must NOT strip it (restriction effects modify the
    /// game rules, outside the characteristic layers, [CR#613.11]); printed
    /// rows stay layer-6-sensitive, these instance rows are immune. Empty for
    /// the characteristic-modifying (`Modify`/`Each`) instances.
    pub rows: Vec<StaticSpec>,
    pub duration: Duration,
    /// The minting resolution frame, kept ONLY for the two durations whose
    /// sweep must re-evaluate semantic data anchored on the source/controller:
    /// `UntilEvent`'s event filter and `ForAsLongAs`'s condition read
    /// `This`/`You` through it ([CR#603.10a]). `None` for every marker /
    /// `EndOfGame` duration, whose sweep needs no such context. Boxed so the
    /// common `None` case keeps `ContinuousEffect` small.
    pub origin: Option<Box<crate::stack::ExecutionFrame>>,
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
    pub card_types: Arc<Vec<TypeDef>>,
    pub subtypes: Arc<Vec<Subtype>>,
    pub supertypes: Arc<Vec<Supertype>>,
    pub abilities: Arc<Vec<Ability>>,
}

impl Characteristics {
    /// Whether the derived card types include the given canonical type,
    /// matched by NAME against the expanded `TypeDef`s (mirrors subtype
    /// name-matching in `matches_derived`). `Type` maps to its `Ident` via
    /// [`Type::name`].
    #[must_use]
    pub fn has_type(&self, t: Type) -> bool {
        let name = t.name();
        self.card_types.iter().any(|d| d.name == name)
    }
}

/// An object's full derived per-object state: its CR [`Characteristics`] plus
/// derived state that is *not* a characteristic — the controller ([CR#613.1b]
/// layer 2) and the layer-6 can't-have-ability prohibition set ([CR#613.1f]).
/// This is the value the layer pass mutates and the [`LayeredView`] stores.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DerivedObject {
    characteristics: Characteristics,
    /// Runtime closure companions aligned with `characteristics.abilities`.
    /// Printed and rule-conferred abilities carry empty entries; a layer-6
    /// grant from a resolved effect carries its grant-time captures here.
    ability_runtimes: Vec<crate::activation::AbilityRuntime>,
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

    /// Like [`get`](Self::get) but returns `None` for an id ABSENT from the
    /// view (a gone/leaving object) instead of panicking. The derived-ability
    /// fold ([`crate::derive::derived_abilities_of`]) consults this
    /// defensively: its callers pass live, card-backed ids (every one is in
    /// `base_map`), but a never-crash read is cheaper to reason about than
    /// a liveness precondition.
    #[must_use]
    pub(crate) fn try_get(&self, id: ObjectId) -> Option<&Characteristics> {
        self.0.get(&id).map(|d| &d.characteristics)
    }

    pub(crate) fn ability_runtime(
        &self,
        id: ObjectId,
        index: usize,
    ) -> Option<&crate::activation::AbilityRuntime> {
        self.0.get(&id)?.ability_runtimes.get(index)
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
        let ability_count = abilities.len();
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
                ability_runtimes: vec![crate::activation::AbilityRuntime::default(); ability_count],
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
pub(crate) fn base_colors(face: &deckmaste_card::CardFace) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();
    for c in face
        .characteristics
        .mana_cost
        .iter()
        .flat_map(symbol_colors)
    {
        if !colors.contains(&c) {
            colors.push(c);
        }
    }
    if colors.is_empty() {
        colors.clone_from(&face.characteristics.color_indicator);
    }
    colors
}

/// Collect the colors contributed by one mana symbol ([CR#202.2]).
pub(crate) fn symbol_colors(sym: &ManaSymbol) -> impl Iterator<Item = Color> {
    let mut buf: [Option<Color>; 2] = [None; 2];
    match sym {
        ManaSymbol::Simple(s) => {
            buf[0] = s.color();
        }
        // A hybrid symbol is ALL its component colors ([CR#107.4e]); the left
        // half may be generic or colorless (no color) or a color, the right is
        // always a color.
        ManaSymbol::Hybrid(left, right) => {
            buf[0] = left.color();
            buf[1] = Some(*right);
        }
        // A Phyrexian symbol is its color ([CR#107.4f]); a hybrid Phyrexian
        // adds its second component color.
        ManaSymbol::Phyrexian(c, other) => {
            buf[0] = Some(*c);
            buf[1] = *other;
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
/// (layer 1b, [CR#708.2]). The layer-1a GRAMMAR now exists
/// (`core-copy-grammar` Task 6): [`deckmaste_core::EnterRider::AsCopy`]
/// ([CR#707.5], an enter-riding copy input on `Move`/`Create`) and
/// [`deckmaste_core::StaticSpec::BecomesCopy`] ([CR#707.4], a continuous
/// copy static gathered like [`deckmaste_core::StaticSpec::Modify`]) both
/// carry the shared [`deckmaste_core::CopySpec`] — but neither is CONSUMED
/// yet; this still reads the printed face unconditionally.
/// `engine-layers-1-copy-facedown-text` owns the application: when an object
/// carries a gathered `AsCopy`/`BecomesCopy` input (or a face-down spec,
/// `engine-face-down`'s), branch here to derive the copiable values from it
/// instead — that is the entirety of layer 1's effect on `Characteristics`,
/// since after layer 1 the characteristics *are* the copiable values
/// ([CR#613.2c]).
fn base_values(state: &GameState, id: ObjectId) -> DerivedObject {
    let obj = state.objects.obj(id);
    let card = obj.card_id().expect("card-backed object");
    let instance = state.cards.get(card);
    // The object's CURRENT face and its matching precomputed cache
    // ([CR#712.8d,712.8e]): a back-up two-faced permanent derives from its
    // back face.
    let face = crate::derive::face_of(state, id);
    let cache = instance.face_cache(obj.side);
    DerivedObject {
        characteristics: Characteristics {
            power: base_stat(face.characteristics.power.as_ref()),
            toughness: base_stat(face.characteristics.toughness.as_ref()),
            colors: Arc::clone(&cache.colors),
            card_types: Arc::clone(&cache.card_types),
            subtypes: Arc::clone(&cache.subtypes),
            supertypes: Arc::clone(&cache.supertypes),
            abilities: Arc::clone(&cache.printed),
        },
        ability_runtimes: vec![crate::activation::AbilityRuntime::default(); cache.printed.len()],
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
        Modification::CardTypes(_)
        | Modification::Subtypes(_)
        | Modification::Supertypes(_)
        | Modification::AllCreatureTypes
        | Modification::BecomeBasicLandType(_) => Some(Layer::L4),
        // Layer 5: color-changing ([CR#613.1e]).
        Modification::Colors(_) => Some(Layer::L5),
        // Layer 6: ability-adding/removing ([CR#613.1f]).
        Modification::GainAbility(_)
        | Modification::LoseAbility(_)
        | Modification::LoseAllAbilities
        | Modification::CantHaveAbility(_) => Some(Layer::L6),
        // Layer 7a/7b/7c: power/toughness ([CR#613.4]). A `Set` base op is 7a
        // for a CDA ([CR#604.3]), else 7b ([CR#613.4a,613.4b]); `Up`/`Down`
        // modifications are 7c ([CR#613.4c]).
        Modification::Power(op) | Modification::Toughness(op) => match op {
            NumericOp::Set(_) => {
                if is_cda {
                    Some(Layer::L7a)
                } else {
                    Some(Layer::L7b)
                }
            }
            NumericOp::Up(_) | NumericOp::Down(_) => Some(Layer::L7c),
        },
        // Layer 7d: switch ([CR#613.4d]).
        Modification::SwitchPowerToughness => Some(Layer::L7d),
        // No layer. Loyalty/defense are not [CR#613] characteristics here;
        // `Several` is a change-bundling expansion artifact that
        // `Modification::flatten` (run at the `gather` boundary) splices away
        // and strips before the layer pass, so it never reaches it — defensive.
        Modification::BaseLoyalty(_) | Modification::BaseDefense(_) | Modification::Several(_) => {
            None
        } // Provenance is erased at `lower` (`deckmaste_lowering`), so no
          // loaded value reaches here wrapped. The arm survives only because
          // the variant does; `core-demacro` deletes both.
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
    /// Gates wrapped around this static effect ([CR#611.3a]). Every gate is
    /// re-evaluated against the in-progress derived map before the effect is
    /// considered or applied; nested `Conditionally` wrappers are conjunctive.
    conditions: Vec<Condition>,
    /// The effect's controller ([CR#611.2c]) — resolves `You` in a layer-2
    /// `SetController`. For a static ability it is the source permanent's base
    /// controller; for a registry effect it is the locked value it carries.
    controller: PlayerId,
    scope: ScopeResolved,
    changes: Vec<Modification>,
    /// Closure companions aligned with `changes`; non-grant entries are empty.
    grant_runtimes: Vec<crate::activation::AbilityRuntime>,
    /// The effect's carrier ([CR#611.2c]) — the object whose `Ref(This)`/
    /// `Ref(You)` a matching-set static resolves against. The source permanent
    /// for a static ability; `Player(controller)` for a spell-built
    /// floating effect (its source spell has left the stack, so `You`
    /// anchors on the locked controller's proxy, which is always live).
    /// Threaded into `resolve_scope` → `matches_derived` so a tribal-lord
    /// scope resolves instead of panicking.
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
                if **reference == deckmaste_core::Reference::source_parameter() =>
            {
                Count::Literal(holder.get(kind.as_str()).copied().unwrap_or(0))
            }
            other => other.clone(),
        }
    };
    // Bake the `Count` each numeric op carries (Set/Up/Down all hold one).
    let bake_op = |op: &NumericOp| -> NumericOp {
        match op {
            // `Set` carries a `StatValue`; only its dynamic-count embed has a
            // `Count` to bake — a printed/CDA value passes through untouched.
            NumericOp::Set(sv) => NumericOp::Set(match sv {
                StatValue::Count(c) => StatValue::Count(bake(c)),
                other => other.clone(),
            }),
            NumericOp::Up(n) => NumericOp::Up(bake(n)),
            NumericOp::Down(n) => NumericOp::Down(bake(n)),
        }
    };
    changes
        .iter()
        .map(|m| match m {
            Modification::Power(op) => Modification::Power(bake_op(op)),
            Modification::Toughness(op) => Modification::Toughness(bake_op(op)),
            Modification::BaseLoyalty(op) => Modification::BaseLoyalty(bake_op(op)),
            Modification::BaseDefense(op) => Modification::BaseDefense(bake_op(op)),
            other => other.clone(),
        })
        .collect()
}

/// Collect all active static `Modify` effects from battlefield permanents,
/// plus any floating one-shot effects from `state.continuous`, plus
/// counter-conferred `Continuous` boosts ([CR#122.1]).
///
/// `derived` is the fixpoint hook ([CR#613.7] re-evaluation): on the FIRST
/// iteration of the layer pass it is `None`, so the static-ability effect
/// sources are read from each object's PRINTED list — the cycle-safe base that
/// breaks the `layers() → derive::abilities → layers()` recursion. On later
/// iterations it is `Some(working)`, the derived map the previous iteration
/// produced, so a static ability that was itself GRANTED by a layer-6
/// `GainAbility` is now visible and gathered as its own effect source. Counter
/// boosts and floating one-shots are unaffected — they never come from the
/// ability list — so they are gathered identically every iteration.
fn gather(
    state: &GameState,
    derived: Option<&BTreeMap<ObjectId, DerivedObject>>,
) -> Vec<ActiveEffect> {
    let mut effects = Vec::new();
    for obj in state.objects.iter() {
        if obj.card_id().is_none() {
            continue; // player proxy — no static abilities
        }
        // Static abilities function only on the battlefield ([CR#611.3b]) —
        // EXCEPT an emblem's, which function from the command zone ([CR#114.4]).
        // Gate the command admittance on "emblem" so an ordinary command-zone
        // object (none today) isn't swept in.
        let emblem_in_command = obj.zone == Some(Zone::Command)
            && obj.card_id().is_some_and(|c| state.cards.get(c).is_emblem);
        if obj.zone != Some(Zone::Battlefield) && !emblem_in_command {
            continue;
        }
        let timestamp = obj.timestamp;
        // Instruction-source abilities come from the DERIVED list once the fixpoint
        // is running ([CR#613.7]): the first iteration reads the PRINTED list
        // (the cycle-safe base that breaks the `layers() → derive::abilities →
        // layers()` recursion), and every later iteration reads the working
        // derived map the previous iteration produced — so a static ability
        // GRANTED by a layer-6 `GainAbility` is gathered as its own effect
        // source (a lord that grants a lord; [CR#613.7] dependency
        // re-evaluation). Reading the derived list is NOT recursive: it indexes
        // the already-computed `working` map, never re-entering `layers()`.
        //
        // Flatten the source list the same way the engine-internal enumeration
        // does (`derive::flatten_composites`): splice the members of a
        // composite keyword. A
        // `KeywordAbility`-kind macro (Changeling, Devoid) expands to
        // `Keyword(Composite { abilities: [Static(...)] })`; without splicing,
        // the `Static` it carries would never be gathered, so the keyword's
        // continuous effect (devoid → colorless, changeling → every creature
        // type) would silently do nothing. This mirrors the flat ability space
        // `abilities_of_source` builds for the trigger scan.
        let mut sources: Vec<(
            Ability,
            Vec<(deckmaste_core::RefId, crate::activation::Value)>,
        )> = Vec::new();
        match derived.and_then(|d| d.get(&obj.id)) {
            // Later iterations: the derived ability list (granted statics
            // included). `Arc<Vec<Ability>>`, indexed not re-derived.
            Some(d) => {
                for (ability, runtime) in
                    d.characteristics.abilities.iter().zip(&d.ability_runtimes)
                {
                    if runtime.flattened.is_empty() {
                        let mut flat = Vec::new();
                        crate::derive::flatten_composites(ability, &mut flat);
                        sources.extend(flat.into_iter().map(|ability| (ability, Vec::new())));
                    } else {
                        sources.extend(
                            runtime
                                .flattened
                                .iter()
                                .map(|entry| (entry.ability.clone(), entry.captures.clone())),
                        );
                    }
                }
            }
            // First iteration (or an object absent from the derived map):
            // printed abilities.
            None => {
                for ability in crate::derive::printed_abilities(state, obj.id) {
                    let mut flat = Vec::new();
                    crate::derive::flatten_composites(ability, &mut flat);
                    sources.extend(flat.into_iter().map(|ability| (ability, Vec::new())));
                }
            }
        }
        for (ability, captures) in &sources {
            let Ability::Static(effect) = ability else {
                continue;
            };
            if let Some((conditions, scope, changes)) =
                static_effect_scope(state, obj.id, effect, captures)
            {
                let grant_runtimes = state.capture_grant_runtimes_in_created_region(
                    &changes,
                    effect,
                    &state.frame(obj.id, obj.controller),
                    captures,
                );
                effects.push(ActiveEffect {
                    timestamp,
                    // The 7a/CDA layer distinction is deferred (0 cards use
                    // it; the surface no longer carries a
                    // `characteristic_defining` flag at all) — every gathered
                    // static routes through the non-CDA layers.
                    is_cda: false,
                    conditions,
                    // A static ability's continuous effect is controlled by the
                    // permanent it is on ([CR#611.2c]); its `You` is that
                    // permanent's controller.
                    controller: obj.controller,
                    scope,
                    changes,
                    grant_runtimes,
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
                let deckmaste_core::Property::Continuous(reference, change) = prop else {
                    continue;
                };
                // `Property::Continuous` is always single-object ([CR#122.1] —
                // plurality, if a counter ever needs it, would distribute with
                // an `Each`-shaped conferral, which the `Property` grammar does
                // not carry today), so this is always a locked, source-relative
                // resolve — never `Floating`.
                let scope = ScopeResolved::Locked(resolve_source_relative(
                    state,
                    obj.id,
                    reference,
                    None,
                    &[],
                ));
                let changes = vec![change.clone()];
                effects.push(ActiveEffect {
                    timestamp: obj.timestamp,
                    is_cda: false,
                    conditions: Vec::new(),
                    controller: obj.controller,
                    scope,
                    // Flatten first (so `bake` sees the flat P/T ops), then bake
                    // the self-scoped counter counts — the same single boundary
                    // as the static-ability path.
                    changes: bake_counter_counts(&Modification::flatten(&changes), &obj.counters),
                    grant_runtimes: vec![
                        crate::activation::AbilityRuntime::default();
                        Modification::flatten(&changes).len()
                    ],
                    watcher: Some(obj.source),
                    locked: None,
                });
            }
        }
    }
    // Append floating one-shot continuous effects from the registry.
    for ce in &state.continuous {
        let changes = Modification::flatten(&ce.changes).to_vec();
        let grant_runtimes = state
            .continuous_grant_runtimes
            .get(&ce.timestamp)
            .cloned()
            .filter(|runtimes| runtimes.len() == changes.len())
            .unwrap_or_else(|| vec![crate::activation::AbilityRuntime::default(); changes.len()]);
        effects.push(ActiveEffect {
            timestamp: ce.timestamp,
            is_cda: ce.is_cda,
            conditions: Vec::new(),
            controller: ce.controller,
            scope: ce.scope.clone(),
            // Same single boundary: a floating one-shot's `changes` (a granted
            // `+N/+N until end of turn`) is flattened before the layer pass.
            changes,
            grant_runtimes,
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

/// Resolve one gathered `StaticSpec` (a static ability's `effect`, or a
/// counter's conferred `Property::Continuous` lowered to the same shape by the
/// caller) into a `(ScopeResolved, flattened changes)` pair, SOURCE-RELATIVE
/// (no [`ExecutionFrame`](crate::resolve::ExecutionFrame); mirrors [`resolve_source_relative`]).
///
/// `Modify(reference, change)` — the single-object shape — resolves the one
/// reference and LOCKS it ([CR#613.6]: the affected set is fixed at first
/// application). `Each(SelectAll(filter), Modify(It, change))` — the
/// distributor shape ("every creature you control gets +1/+1") — stays a
/// `Floating(filter)` scope: the filter is NOT expanded to objects here: the
/// existing `Floating` machinery re-evaluates it against each layer pass's
/// derived characteristics ([CR#613.6]), which this function must not
/// shortcut.
///
/// Any other shape — `Each` over a non-`SelectAll` `Selection` (`Union`,
/// `Random`, …), or an `Each` whose inner effect isn't a bare
/// `Modify(It, _)` — is a documented seam: no current macro or card produces
/// it, so it contributes no effect (`None`) rather than guessing a scope.
/// Every other `StaticSpec` variant (`Deontic`, `CostModifier`, …) is
/// likewise `None` here — the layer gather only ever contributes `Modify`/
/// `Each`-of-`Modify` effects; the rest are read by their own consumers
/// (`legal.rs`, `cast.rs`, `trigger.rs`, …).
fn static_effect_scope(
    state: &GameState,
    obj: ObjectId,
    region: &deckmaste_core::Region<StaticSpec>,
    captures: &[(deckmaste_core::RefId, crate::activation::Value)],
) -> Option<(Vec<Condition>, ScopeResolved, Vec<Modification>)> {
    match &region.body {
        StaticSpec::Modify(reference, change) => Some((
            Vec::new(),
            ScopeResolved::Locked(resolve_source_relative(
                state,
                obj,
                reference,
                Some(&region.params),
                captures,
            )),
            Modification::flatten(std::slice::from_ref(change)).to_vec(),
        )),
        StaticSpec::Each(Selection::SelectAll(filter), inner) => match &inner.body {
            StaticSpec::Modify(deckmaste_core::Reference::Reg(reference), change)
                if matches!(
                    inner.provenance_of(*reference),
                    Some(deckmaste_core::Provenance::Candidate(_))
                ) =>
            {
                Some((
                    Vec::new(),
                    ScopeResolved::Floating(filter.clone()),
                    Modification::flatten(std::slice::from_ref(change)).to_vec(),
                ))
            }
            _ => None,
        },
        // [CR#611.3a]: keep the wrapper's predicate on the gathered effect.
        // It is evaluated later, against the in-progress layer map, rather
        // than here against printed values or through a recursive `layers()`.
        // Nested wrappers form a conjunction: every enclosing condition must
        // still hold for the innermost modification to apply.
        StaticSpec::Conditionally(condition, inner) => {
            let nested = deckmaste_core::Region::new(region.params.clone(), inner.as_ref().clone());
            let (mut conditions, scope, changes) =
                static_effect_scope(state, obj, &nested, captures)?;
            conditions.insert(0, condition.clone());
            Some((conditions, scope, changes))
        }
        _ => None,
    }
}

/// Resolve a `Reference` inside a static ability's effect (its `Modify`/`Each`
/// references) to concrete object ids, SOURCE-RELATIVE: `This` is the static's
/// source object `source` (the carrying permanent), and `gather` has **no**
/// [`ExecutionFrame`], so only the references whose value is fixed by the source's own
/// relations are resolvable here. The rest are a documented seam (see below)
/// and resolve to the empty set.
///
/// Returns a (possibly empty) vec — the empty set both for an ExecutionFrame-dependent
/// reference and for a relation that isn't established (an unattached
/// attachment's `AttachHostOf(This)` has no host yet). The caller LOCKS the
/// result ([CR#613.6]: the affected set is fixed at first application).
///
/// [`ExecutionFrame`]: crate::resolve::ExecutionFrame
fn resolve_source_relative(
    state: &GameState,
    source: ObjectId,
    reference: &deckmaste_core::Reference,
    params: Option<&[deckmaste_core::Param]>,
    captures: &[(deckmaste_core::RefId, crate::activation::Value)],
) -> Vec<ObjectId> {
    use deckmaste_core::Reference;
    match reference {
        // The carrying object itself.
        &Reference::Reg(reference) => match params
            .and_then(|params| params.get(reference.0 as usize))
        {
            Some(param) if param.provenance == deckmaste_core::Provenance::Source => vec![source],
            Some(param) if matches!(param.provenance, deckmaste_core::Provenance::Capture(_)) => {
                captures
                    .iter()
                    .find(|(here, _)| *here == reference)
                    .map_or_else(Vec::new, |(_, value)| captured_objects(value))
            }
            None if params.is_none() => vec![source],
            _ => Vec::new(),
        },
        // The host an attachment is attached to ([CR#301.5,303.4]) — read the
        // attachment→host link off the resolved inner object. No host (an
        // unattached attachment) → empty, so nothing is buffed.
        Reference::AttachHostOf(inner) => {
            resolve_source_relative(state, source, inner, params, captures)
                .into_iter()
                .filter_map(|id| state.objects.get(id).and_then(|o| o.attached_to))
                .collect()
        }
        // ExecutionFrame-dependent references only: `Target`, bindings (`Bound`,
        // `Linked`, `EventObject`, `EventActor`) — and the player-valued
        // `You`/`ControllerOf`/`OwnerOf` — cannot be resolved without an `ExecutionFrame`
        // in gather, so they stay an empty locked set (a documented seam; these
        // need `eval_reference`, which `gather` deliberately lacks to avoid the
        // layers()→eval recursion).
        _ => Vec::new(),
    }
}

fn captured_objects(value: &crate::activation::Value) -> Vec<ObjectId> {
    use crate::activation::Value;
    match value {
        Value::Object(product) => product.current.into_iter().collect(),
        Value::Objects(products) | Value::Pile(products) => products
            .iter()
            .filter_map(|product| product.current)
            .collect(),
        Value::Unavailable | Value::Number(_) | Value::Symbol(_) => Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Scope resolution
// ---------------------------------------------------------------------------

/// Evaluate a `Predicate` against a single object's DERIVED characteristics in
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
/// (`And([…, Not(Ref(This)), ControlledBy(Ref(You))])`). The live trigger
/// lane (`filter_matches_live`) threads its watcher the same way.
fn matches_derived(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    id: ObjectId,
    filter: &deckmaste_core::Predicate,
    watcher: Option<ObjectSource>,
    params: Option<&[deckmaste_core::Param]>,
) -> bool {
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::Predicate;
    // `Any` is a wildcard sentinel — it must always match, even for ids that
    // aren't in `working` (e.g. player proxies). Checked before the map lookup.
    if let Predicate::Any = filter {
        return true;
    }
    let Some(c) = working.get(&id).map(|d| &d.characteristics) else {
        return static_reference_matches(state, id, filter, watcher, params);
    };
    // Combinators (`And`/`Or`/`Not`; `Any` handled above so it
    // matches even for ids absent from `working`) recurse through this same
    // derived matcher via the shared walker; characteristic leaves fall through
    // below and everything else delegates to the printed matcher.
    if let Some(result) = crate::target::walk_combinators(filter, |f| {
        matches_derived(state, working, id, f, watcher, params)
    }) {
        return result;
    }
    match filter {
        Predicate::Characteristic(CharacteristicPredicate::Type(name)) => {
            c.card_types.iter().any(|t| t.name == name.name())
        }
        Predicate::Characteristic(CharacteristicPredicate::Supertype(s)) => {
            c.supertypes.contains(s)
        }
        Predicate::Characteristic(CharacteristicPredicate::ColorIs(col)) => c.colors.contains(col),
        Predicate::Characteristic(CharacteristicPredicate::Multicolored) => c.colors.len() >= 2,
        Predicate::Characteristic(CharacteristicPredicate::Colorless) => c.colors.is_empty(),
        // Subtype matching against derived: `working[id].subtypes` are Subtype
        // structs; the filter carries an Ident name. Match by name.
        Predicate::Characteristic(CharacteristicPredicate::Subtype(name)) => {
            c.subtypes.iter().any(|s| s.name == name.name())
        }
        // `Has` is derivable from the working map — check the derived
        // ability list.
        Predicate::Characteristic(CharacteristicPredicate::Has(name)) => {
            c.abilities.iter().any(|a| ability_is_named(a, &name.0))
        }
        // Stat over DERIVED P/T (in the working map); mana value is printed
        // (layer-stable), read without the layer view. Evaluated HERE rather
        // than delegated so the derived matcher never re-enters `state.layers()`
        // mid-build via `target::matches`'s layers-reading Stat arm.
        Predicate::Characteristic(CharacteristicPredicate::Stat(stat, cmp, count)) => {
            use deckmaste_core::Stat;
            let value = match stat {
                Stat::Power => c.power,
                Stat::Toughness => c.toughness,
                Stat::ManaValue => Some(
                    Int::try_from(
                        crate::derive::face(state.def(id))
                            .characteristics
                            .mana_cost
                            .mana_value(),
                    )
                    .expect("mana value fits Int"),
                ),
                // [CR#209.1,306.5a]: loyalty is the PRINTED loyalty
                // characteristic off the card face — never the live counter
                // count (current loyalty is `CounterCount(This,
                // LoyaltyCounter)`). `base_stat` maps `Number(n)→n`,
                // `DefinedByAbility`/`Variable`/absent → 0.
                Stat::Loyalty => base_stat(
                    crate::derive::face(state.def(id))
                        .characteristics
                        .loyalty
                        .as_ref(),
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
        Predicate::Ref(_) => static_reference_matches(state, id, filter, watcher, params),
        Predicate::Relation(deckmaste_core::RelationPredicate::ControlledBy(inner)) => {
            matches_derived(
                state,
                working,
                state.player(state.objects.obj(id).controller).object,
                inner,
                watcher,
                params,
            )
        }
        Predicate::Relation(deckmaste_core::RelationPredicate::Owner(inner)) => matches_derived(
            state,
            working,
            state.player(state.owner_of(id)).object,
            inner,
            watcher,
            params,
        ),
        // `Named` and everything non-characteristic (zone, status, kind,
        // combat, relations, refs …): delegate to the printed matcher, threading
        // the carrier `watcher` so a scope's `Ref(This)`/`Ref(You)` (and the
        // `Ref(You)` nested inside a `ControlledBy`) anchors against the host
        // instead of hitting the frameless-targeting `todo!`. None of the
        // delegated arms re-enter `state.layers()` — the recursion-safety
        // invariant this matcher rests on. That holds for EVERY id it sees, and
        // it sees every live card-backed object, not only battlefield permanents:
        // `base_map` derives objects in all zones, so a predicate-scoped
        // conferral / `SelectAll` scope resolves over that whole set — an
        // off-battlefield (in-hand/graveyard) card is matched here too. Each
        // delegated arm is zone-agnostic — `Named` reads the printed face;
        // relations resolve over player proxies / object iteration; combat and
        // state read stored fields — so the layers()-free guarantee is
        // independent of the id's zone. Characteristic leaves are handled above.
        _ => crate::target::matches_with(state, id, filter, watcher),
    }
}

fn static_reference_matches(
    state: &GameState,
    id: ObjectId,
    filter: &Predicate,
    watcher: Option<ObjectSource>,
    params: Option<&[deckmaste_core::Param]>,
) -> bool {
    let Predicate::Ref(Reference::Reg(reference)) = filter else {
        return matches!(filter, Predicate::Any);
    };
    let provenance = params
        .and_then(|params| params.get(reference.0 as usize))
        .map(|param| &param.provenance);
    match (provenance, watcher) {
        (Some(deckmaste_core::Provenance::Candidate(_)), _) => true,
        (Some(deckmaste_core::Provenance::Source), Some(source)) => {
            state.objects.obj(id).source == source
        }
        (Some(deckmaste_core::Provenance::Controller), Some(source)) => {
            let controller = state.controller_of_source(source);
            matches!(state.objects.obj(id).source,
                ObjectSource::Player(player) if Some(player) == controller)
        }
        _ => false,
    }
}

/// Resolve a `ScopeResolved` against the current working set, returning the
/// target object ids.
///
/// `Floating` filters against DERIVED characteristics via `matches_derived`,
/// realizing [CR#613.6]'s requirement that affected sets are re-evaluated
/// against "the characteristics produced by earlier layers". This means an
/// anthem catches permanents animated to Creature by a same-pass L4 effect,
/// and a `SelectAll(Enchantment)` set correctly includes only objects still
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
            .filter(|&id| {
                matches_derived(
                    state,
                    working,
                    id,
                    &filter.body,
                    watcher,
                    Some(&filter.params),
                )
            })
            .collect(),
        ScopeResolved::Locked(ids) => ids.clone(),
    }
}

/// Evaluate a predicate against the in-progress layer map when `id` has
/// characteristics there, falling back to the ordinary live matcher for
/// player proxies (which deliberately have no [`DerivedObject`]).
fn condition_predicate_matches(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    id: ObjectId,
    filter: &Predicate,
    watcher: Option<ObjectSource>,
) -> bool {
    if working.contains_key(&id) {
        matches_derived(state, working, id, filter, watcher, None)
    } else {
        crate::target::matches_with(state, id, filter, watcher)
    }
}

/// Evaluate one conditional-static gate against the IN-PROGRESS derived map.
/// This is the layer-time sibling of [`GameState::condition_holds`]: it keeps
/// the same condition meanings while routing characteristic predicates and
/// counts through `working`, never recursively rebuilding `state.layers()`.
fn condition_holds_derived(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    condition: &Condition,
    watcher: Option<ObjectSource>,
    controller: PlayerId,
) -> bool {
    match condition {
        Condition::Exists(filter) => state
            .objects
            .iter()
            .any(|object| condition_predicate_matches(state, working, object.id, filter, watcher)),
        Condition::Matches(reference, filter) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .is_some_and(|id| condition_predicate_matches(state, working, id, filter, watcher))
        }
        Condition::Compare(a, op, b) => {
            let lhs = eval_count(a, state, working, watcher, controller).max(0);
            let rhs = eval_count(b, state, working, watcher, controller).max(0);
            op.apply(lhs.cast_unsigned(), rhs.cast_unsigned())
        }
        Condition::And(conditions) => conditions.iter().all(|condition| {
            condition_holds_derived(state, working, condition, watcher, controller)
        }),
        Condition::Or(conditions) => conditions.iter().any(|condition| {
            condition_holds_derived(state, working, condition, watcher, controller)
        }),
        Condition::Not(condition) => {
            !condition_holds_derived(state, working, condition, watcher, controller)
        }
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
        Condition::YourTurn => state.turn.active_player == controller,
        Condition::TurnOf(filter) => condition_predicate_matches(
            state,
            working,
            state.player(state.turn.active_player).object,
            filter,
            watcher,
        ),
        Condition::DuringPhase(phase) => state.turn.current == *phase,
        // History and paid-cost gates do not read derived characteristics.
        // Reuse the canonical evaluator with a bare carrier frame so their
        // event/stack semantics do not drift from trigger and resolution use.
        condition @ (Condition::DealtDamageBy(..)
        | Condition::Happened { .. }
        | Condition::PaidCost(_)
        | Condition::CastWith(_)) => state
            .objects
            .iter()
            .find(|object| Some(object.source) == watcher)
            .is_some_and(|source| {
                state.condition_holds(condition, &state.frame(source.id, controller))
            }),
        // `LegallyAttached` needs the deontic attachment view, which cannot be
        // rebuilt recursively while this view is in progress. A crossing gate
        // likewise requires a triggering event's before/after frame channel.
        // Neither has a sound layer-local value; malformed uses fizzle false.
        Condition::LegallyAttached(_) | Condition::Crossed { .. } => false,
    }
}

fn effect_conditions_hold(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    effect: &ActiveEffect,
) -> bool {
    effect.conditions.iter().all(|condition| {
        condition_holds_derived(state, working, condition, effect.watcher, effect.controller)
    })
}

// ---------------------------------------------------------------------------
// Apply
// ---------------------------------------------------------------------------

/// Resolve a `Count`'s `Reference` to a concrete `ObjectId` against the working
/// pass, anchoring carrier refs (`This`, `AttachHostOf(This)`, …) to the
/// effect's `watcher` ([CR#611.2c]). The watcher's live carrier object (the one
/// whose `source == watcher`) is the `This` `resolve_source_relative` resolves
/// from. A `ExecutionFrame`-only reference (`Target`, bindings, the player-valued
/// `You`/`ControllerOf`/`OwnerOf`) yields nothing here — the same documented
/// seam `resolve_source_relative` carries — so a count built on it defaults to
/// `0`.
fn resolve_count_ref(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    reference: &deckmaste_core::Reference,
    watcher: Option<ObjectSource>,
    controller: PlayerId,
) -> Option<ObjectId> {
    use deckmaste_core::Reference;
    match reference {
        reference if reference == &Reference::controller_parameter() => {
            Some(state.player(controller).object)
        }
        Reference::ControllerOf(inner) => {
            let id = resolve_count_ref(state, working, inner, watcher, controller)?;
            let player = working.get(&id).map_or_else(
                || state.objects.get(id).map(|o| o.controller),
                |d| Some(d.controller),
            )?;
            Some(state.player(player).object)
        }
        Reference::OwnerOf(inner) => {
            let id = resolve_count_ref(state, working, inner, watcher, controller)?;
            state
                .objects
                .get(id)
                .and_then(crate::object::GameObject::card_id)
                .map(|_| state.player(state.owner_of(id)).object)
        }
        _ => {
            let source = state.objects.iter().find(|o| Some(o.source) == watcher)?.id;
            resolve_source_relative(state, source, reference, None, &[])
                .into_iter()
                .next()
        }
    }
}

/// `Count::Divide`/`Count::Half`'s shared rounded-division op — factored out
/// of `eval_count` to keep that function under the line-count lint. A zero
/// divisor fizzles to 0 (never-crash) rather than panicking.
fn eval_divide(mode: deckmaste_core::RoundMode, a: Int, b: Int) -> Int {
    // Clamp both operands non-negative right where they enter the division, so
    // the RoundUp truncation bound below (quotient <= a <= i32::MAX) — and the
    // zero-divisor fizzle — hold regardless of the caller. `Count::Divide`
    // already `.max(0)`s both operands, but correctness must not rest on that;
    // a negative divisor now fizzles to 0 like a zero one.
    let a = a.max(0);
    let b = b.max(0);
    if b == 0 {
        0
    } else {
        match mode {
            // `a as i64 + b as i64 - 1` cannot overflow i64, and with the
            // operands clamped non-negative above (`b >= 1` here) the ceil
            // quotient is at most `a <= i32::MAX`, so this never truncates;
            // widening avoids i32 overflow near `i32::MAX` because signed
            // `div_ceil` is unstable (int_roundings) on stable.
            #[expect(
                clippy::cast_possible_truncation,
                reason = "quotient bounded by a <= i32::MAX; a and b are clamped \
                non-negative inside eval_divide (no longer caller-dependent) — see comment above"
            )]
            deckmaste_core::RoundMode::RoundUp => {
                ((i64::from(a) + i64::from(b) - 1) / i64::from(b)) as i32
            }
            deckmaste_core::RoundMode::RoundDown => a / b,
        }
    }
}

/// Evaluate a `Count` to an `Int` against the IN-PROGRESS derived map
/// (`working`) being built this pass — never `self.layers()` (that would
/// recurse the layer build) and never an `ExecutionFrame` (which the layer pass lacks).
/// This is the layer-side sibling of `resolve.rs::eval_count`, mirroring each
/// variant's meaning but sourcing derived P/T from `working` instead of a
/// rebuilt view.
///
/// FIXPOINT BOUNDARY: counts are evaluated against `working` exactly as applied
/// so far this pass. The MTG layer order ([CR#613.3,613.7]) guarantees a count
/// that depends on an EARLIER layer already sees that layer's final value (a 7a
/// CDA counting creatures reads the post-L4 types). A count depending on
/// another object's SAME-or-LATER-layer derived value is the separate
/// `engine-layers-fixpoint` ticket; this evaluator does not iterate.
/// [CR#107.1b,613] "equal to its power" and friends: resolve `reference`, read
/// the DERIVED stat off `working` (mana value / loyalty / defense are
/// layer-stable base state), clamping a negative magnitude to 0. Factored out
/// of [`eval_count`]'s dispatch so that match stays within the line budget.
fn eval_stat_of(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    reference: &deckmaste_core::Reference,
    stat: deckmaste_core::Stat,
    watcher: Option<ObjectSource>,
    controller: PlayerId,
) -> Int {
    use deckmaste_core::Stat;
    let Some(id) = resolve_count_ref(state, working, reference, watcher, controller) else {
        return 0;
    };
    let value = match stat {
        Stat::Power => working
            .get(&id)
            .and_then(|d| d.characteristics.power)
            .unwrap_or(0),
        Stat::Toughness => working
            .get(&id)
            .and_then(|d| d.characteristics.toughness)
            .unwrap_or(0),
        Stat::ManaValue => Int::try_from(
            crate::derive::face(state.def(id))
                .characteristics
                .mana_cost
                .mana_value(),
        )
        .expect("mana value fits Int"),
        // [CR#209.1,306.5a]: loyalty is the PRINTED loyalty characteristic off
        // the card face — never the live counter count (current loyalty is
        // `CounterCount(This, LoyaltyCounter)`). `base_stat` maps `Number(n)→n`,
        // `DefinedByAbility`/`Variable`/absent → 0.
        Stat::Loyalty => base_stat(
            crate::derive::face(state.def(id))
                .characteristics
                .loyalty
                .as_ref(),
        )
        .unwrap_or(0),
        Stat::Defense => Int::try_from(
            state
                .objects
                .obj(id)
                .counters
                .get("DefenseCounter")
                .copied()
                .unwrap_or(0),
        )
        .expect("defense fits Int"),
    };
    // [CR#107.1b,613]: a stat used as a magnitude clamps negative to 0.
    value.max(0)
}

/// The thresholds `abilities` watch a crossing of — the [`Condition::Crossed`]
/// intervening-if of each triggered ability ([CR#714.2b]). [CR#714.2d]'s final
/// chapter number is the greatest of them.
pub(crate) fn watched_thresholds(abilities: &[Ability]) -> impl Iterator<Item = &Count> {
    abilities
        .iter()
        .filter_map(|a| match a {
            Ability::Triggered(t) => t.condition.as_ref(),
            _ => None,
        })
        .filter_map(|c| match c {
            deckmaste_core::Condition::Crossed { thresholds, .. } => Some(thresholds),
            _ => None,
        })
        .flat_map(|thresholds| thresholds.iter())
}

#[expect(
    clippy::too_many_lines,
    reason = "one arm per Count kind — the value language's full surface, mirroring \
    resolve.rs::eval_count's own expect"
)]
fn eval_count(
    n: &Count,
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    watcher: Option<ObjectSource>,
    controller: PlayerId,
) -> Int {
    match n {
        Count::Literal(v) => (*v).cast_signed(),
        // "For each …": the filter's cardinality over the working derived map,
        // matched the same way scopes are ([CR#613.6] — `matches_derived`), so a
        // count over types/colors sees the values earlier layers produced.
        Count::CountOf(source) => match source {
            // [CR#119.1]: a player-filter's cardinality reads exactly like an
            // object filter's — both range over Entities
            // (`Entity(Player)`), matched by the same `matches_derived`.
            Countable::Objects(filter) | Countable::Players(filter) => {
                let count = working
                    .keys()
                    .copied()
                    .filter(|&id| {
                        matches_derived(
                            state,
                            working,
                            id,
                            &filter.body,
                            watcher,
                            Some(&filter.params),
                        )
                    })
                    .count();
                Int::try_from(count).expect("object count fits Int")
            }
            // [CR#700.5]: devotion — pip count over a resolved object's
            // printed cost. A non-object / unresolved reference contributes
            // 0 (never-crash), mirroring the other `resolve_count_ref`
            // consumers above.
            Countable::ManaSymbols(reference, pred) => {
                let Some(id) = resolve_count_ref(state, working, reference, watcher, controller)
                else {
                    return 0;
                };
                let Some(o) = state.objects.get(id) else {
                    return 0;
                };
                if o.card_id().is_none() {
                    return 0;
                }
                let n = crate::derive::face(state.def(id))
                    .characteristics
                    .mana_cost
                    .iter()
                    .filter(|sym| pred.matches(sym))
                    .count();
                Int::try_from(n).expect("pip count fits Int")
            }
            // [CR#105.2]: the cardinality of a one-object "set" — 1 when the
            // reference resolves to a real card-backed object, else 0.
            Countable::Singleton(reference) => {
                let Some(id) = resolve_count_ref(state, working, reference, watcher, controller)
                else {
                    return 0;
                };
                if state
                    .objects
                    .get(id)
                    .and_then(crate::object::GameObject::card_id)
                    .is_none()
                {
                    return 0;
                }
                1
            }
            // [CR#107.4]: mana spent to cast/activate a referenced object,
            // filtered by `pred` (Adamant). No mana-spent tracking in the
            // engine yet — fizzles to 0, like `ManaSymbols`'s siblings.
            Countable::ManaSpentMatching(..) => 0,
        },
        // "Equal to its power": resolve the reference, read the DERIVED stat off
        // `working`. Mana value / loyalty / defense are layer-stable base state
        // (read off the card face / counter map, as `resolve.rs` does). A
        // negative result counts as `0` ([CR#107.1b,613]).
        Count::StatOf(reference, stat) => {
            eval_stat_of(state, working, reference, *stat, watcher, controller)
        }
        // [CR#119.1,402.2]: a player's numeric attribute — read straight off
        // state (players have no [CR#613] layers). A non-player or unresolved
        // reference contributes 0.
        Count::PlayerStatOf(reference, attr) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .and_then(|id| state.objects.get(id))
                .and_then(|o| match o.source {
                    ObjectSource::Player(p) => Some(p),
                    ObjectSource::Card(_) => None,
                })
                .map_or(0, |p| {
                    Int::try_from(state.player_attr(p, *attr)).expect("player attr fits Int")
                })
        }
        // [CR#102.1]: how many opponents the referenced player has.
        Count::Opponents(reference) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .and_then(|id| state.objects.get(id))
                .and_then(|o| match o.source {
                    ObjectSource::Player(p) => Some(p),
                    ObjectSource::Card(_) => None,
                })
                .map_or(0, |p| {
                    Int::try_from(state.opponent_count(p)).expect("opponent count fits Int")
                })
        }
        // [CR#122.1]: the count of a counter kind on the resolved object, read
        // off the raw counter map (base state — no derived/recursion). An absent
        // object or kind is `0`.
        Count::CounterCount(reference, kind) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .and_then(|id| state.objects.get(id))
                .and_then(|o| o.counters.get(kind.as_str()).copied())
                .map_or(0, |c| Int::try_from(c).expect("counter count fits Int"))
        }
        // [CR#714.2d]: the greatest chapter number among the object's own
        // abilities, read off the derivation in progress so a stripped chapter
        // ([CR#613.1f]) stops counting. 0 when nothing watches a crossing.
        Count::GreatestWatchedThreshold(reference) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .and_then(|id| working.get(&id))
                .map_or(0, |derived| {
                    watched_thresholds(&derived.characteristics.abilities)
                        .map(|t| eval_count(t, state, working, watcher, controller))
                        .max()
                        .unwrap_or(0)
                })
        }
        // [CR#120.3]: marked damage on the resolved object (base state).
        Count::Damage(reference) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .and_then(|id| state.objects.get(id))
                .map_or(0, |o| {
                    Int::try_from(o.total_damage()).expect("damage fits Int")
                })
        }
        // [CR#106.4]: the referenced player's total floated mana — read
        // straight off the pool (base state, no layers). A non-player or
        // unresolved reference contributes 0.
        Count::ManaAvailable(reference) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .and_then(|id| state.objects.get(id))
                .and_then(|o| match o.source {
                    ObjectSource::Player(p) => Some(p),
                    ObjectSource::Card(_) => None,
                })
                .map_or(0, |p| {
                    Int::try_from(state.player(p).mana_pool.units().len())
                        .expect("mana pool fits Int")
                })
        }
        Count::ManaAvailableKind(reference, kind) => {
            resolve_count_ref(state, working, reference, watcher, controller)
                .and_then(|id| state.objects.get(id))
                .and_then(|o| match o.source {
                    ObjectSource::Player(p) => Some(p),
                    ObjectSource::Card(_) => None,
                })
                .map_or(0, |p| {
                    Int::try_from(state.player(p).mana_pool.amount(*kind))
                        .expect("mana pool fits Int")
                })
        }
        // [CR#704.5q]: the lesser of two magnitudes.
        Count::Min(a, b) => eval_count(a, state, working, watcher, controller)
            .min(eval_count(b, state, working, watcher, controller)),
        // [CR#107.1] basic value arithmetic over the derived view. `Minus`
        // floors at 0 ([CR#107.1b]); `Half` rounds per the mode on the
        // non-negative magnitude.
        Count::Max(a, b) => eval_count(a, state, working, watcher, controller)
            .max(eval_count(b, state, working, watcher, controller)),
        Count::Plus(a, b) => eval_count(a, state, working, watcher, controller)
            .saturating_add(eval_count(b, state, working, watcher, controller)),
        Count::Minus(a, b) => (eval_count(a, state, working, watcher, controller)
            - eval_count(b, state, working, watcher, controller))
        .max(0),
        Count::Times(a, b) => eval_count(a, state, working, watcher, controller)
            .saturating_mul(eval_count(b, state, working, watcher, controller)),
        Count::Half(mode, inner) => {
            let v = eval_count(inner, state, working, watcher, controller).max(0);
            match mode {
                // Widened to i64 to avoid overflow near i32::MAX (signed
                // div_ceil is unstable on stable); the quotient is bounded
                // by `v <= i32::MAX`, so this never truncates.
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "quotient is bounded by v <= i32::MAX; see comment above"
                )]
                deckmaste_core::RoundMode::RoundUp => ((i64::from(v) + 1) / 2) as i32,
                deckmaste_core::RoundMode::RoundDown => v / 2,
            }
        }
        // [CR#107.1a]: `Half`'s general twin. A zero divisor fizzles to 0
        // (never-crash) rather than panicking on integer division.
        Count::Divide(mode, a, b) => eval_divide(
            *mode,
            eval_count(a, state, working, watcher, controller).max(0),
            eval_count(b, state, working, watcher, controller).max(0),
        ),
        // [CR#107.1]: remainder — parity checks. A zero divisor fizzles to 0.
        Count::Mod(a, b) => {
            let b = eval_count(b, state, working, watcher, controller).max(0);
            if b == 0 {
                0
            } else {
                eval_count(a, state, working, watcher, controller).max(0) % b
            }
        }
        // [CR#107.1]: exponentiation — doubling effects build `Pow(2, X)`.
        Count::Pow(base, exp) => eval_count(base, state, working, watcher, controller)
            .max(0)
            .saturating_pow(
                eval_count(exp, state, working, watcher, controller)
                    .max(0)
                    .cast_unsigned(),
            ),
        // [CR#107.3]: distinct-union count over the derived working set.
        // Printed axes read the card face; power/toughness read the in-progress
        // derived characteristics, matched the same way `CountOf` is.
        Count::CountDistinct(characteristic, source) => match source {
            Countable::Objects(filter) => {
                let mut seen = std::collections::BTreeSet::new();
                for id in working.keys().copied() {
                    if matches_derived(
                        state,
                        working,
                        id,
                        &filter.body,
                        watcher,
                        Some(&filter.params),
                    ) {
                        for key in distinct_keys_derived(state, working, id, *characteristic) {
                            seen.insert(key);
                        }
                    }
                }
                Int::try_from(seen.len()).expect("distinct count fits Int")
            }
            // [CR#105.2]: the distinct-union axis read off a SINGLE object —
            // Embiggen's "number of card types it has" =
            // `CountDistinct(Types, Singleton(This))`.
            Countable::Singleton(reference) => {
                let Some(id) = resolve_count_ref(state, working, reference, watcher, controller)
                else {
                    return 0;
                };
                if state
                    .objects
                    .get(id)
                    .and_then(crate::object::GameObject::card_id)
                    .is_none()
                {
                    return 0;
                }
                let n = distinct_keys_derived(state, working, id, *characteristic).len();
                Int::try_from(n).expect("distinct count fits Int")
            }
            // None of these is a forced distinct-union path ([CR#700.5]
            // devotion has no distinct-union reading; `ManaSpentMatching` has
            // no engine tracking either; Idris's `readableOn` never grants
            // `Players` a characteristic axis — a player has no printed
            // characteristic to distinctly union) — fizzle to 0.
            Countable::ManaSymbols(..)
            | Countable::ManaSpentMatching(..)
            | Countable::Players(..) => 0,
        },
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
        // Announce-time / history context (`ThatMuch`, `EventCount`,
        // `EventSum`) is unavailable during layer derivation — those
        // need a resolution `ExecutionFrame` (`resolve.rs::eval_count`), so a continuous
        // effect built on one defaults to `0` here (a documented seam).
        //
        // [CR#107.1]: `Aggregate`'s per-element fold binds `It` via a resolution
        // `ExecutionFrame` sub-binding (`resolve.rs::eval_count`'s `Selection::Pick`-style
        // loop) — the layer pass has no `ExecutionFrame` to bind against, so a CDA built
        // on an aggregate fold defaults to `0` here too (unforced: no CDA needs
        // one yet).
        // [CR#115.9a]: `TargetsOf` reads the announcing stack entry's target
        // list — announce-time context this ExecutionFrame-less layer pass lacks
        // (same seam as `TimesPaid`'s paid-cost record) — defaults to 0.
        Count::Reg(_)
        | Count::EventCount(..)
        | Count::EventSum(..)
        | Count::TimesPaid(_)
        | Count::TargetsOf(_)
        | Count::Aggregate(..) => 0,
    }
}

/// The distinct-value keys object `id` contributes to a
/// [`Count::CountDistinct`] union along `characteristic`, read from the
/// in-progress derived view. The layer-time twin of `resolve.rs`'s
/// `distinct_keys`: power/toughness come from the derived `working` map (no
/// `self.layers()` recursion), the rest from the printed face.
fn distinct_keys_derived(
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    id: ObjectId,
    characteristic: deckmaste_core::Characteristic,
) -> Vec<String> {
    use deckmaste_core::Characteristic as Ch;
    let face = crate::derive::face(state.def(id));
    match characteristic {
        Ch::Types => face
            .characteristics
            .types
            .iter()
            .map(|t| format!("{t:?}"))
            .collect(),
        Ch::Subtypes => face
            .characteristics
            .subtypes
            .iter()
            .map(|s| s.name.to_string())
            .collect(),
        // [CR#205.3i]: only the five basic land types contribute keys.
        Ch::BasicLandTypes => face
            .characteristics
            .subtypes
            .iter()
            .map(|s| s.name.to_string())
            .filter(|n| deckmaste_core::BASIC_LAND_TYPES.contains(&n.as_str()))
            .collect(),
        Ch::Supertypes => face
            .characteristics
            .supertypes
            .iter()
            .map(|s| format!("{s:?}"))
            .collect(),
        Ch::Name => vec![face.characteristics.name.to_string()],
        Ch::ManaCost => vec![format!("{}", face.characteristics.mana_cost.mana_value())],
        Ch::Colors => {
            let mut colors: Vec<deckmaste_core::Color> =
                face.characteristics.color_indicator.clone();
            for sym in face.characteristics.mana_cost.iter() {
                colors.extend(symbol_colors(sym));
            }
            colors.iter().map(|c| format!("{c:?}")).collect()
        }
        Ch::Power => working
            .get(&id)
            .and_then(|d| d.characteristics.power)
            .map(|p| vec![p.max(0).to_string()])
            .unwrap_or_default(),
        Ch::Toughness => working
            .get(&id)
            .and_then(|d| d.characteristics.toughness)
            .map(|t| vec![t.max(0).to_string()])
            .unwrap_or_default(),
        Ch::Defense => vec![
            state
                .objects
                .obj(id)
                .counters
                .get("DefenseCounter")
                .copied()
                .unwrap_or(0)
                .to_string(),
        ],
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
        _ => false,
    }
}

/// Apply one `Modification` to `c` at its layer.
/// Layers 4 (types/supertypes), 5 (colors), 6 (abilities), and 7a-7d (P/T) are
/// implemented. Subtypes ([CR#613.1d]) and `BecomeBasicLandType` ([CR#305.7])
/// are explicit deferred stubs; controller/text ([CR#613.1b,613.1c]) and
/// loyalty/defense (no 613 layer) are also stubs.
/// `state`/`working`/`obj_id`/`watcher` are threaded for the count-bearing
/// P/T arms: a dynamic `Count` ([CR#604.3] CDAs, "+X/+X for each …") is
/// evaluated against `working` via `eval_count` BEFORE the object's entry is
/// borrowed mutably, anchoring carrier refs to the effect's `watcher`. The
/// immutable `working` read is scoped to end before the `get_mut`, so there is
/// no borrow conflict.
fn apply(
    m: &Modification,
    grant_runtime: &crate::activation::AbilityRuntime,
    effect_controller: PlayerId,
    state: &GameState,
    working: &mut BTreeMap<ObjectId, DerivedObject>,
    obj_id: ObjectId,
    watcher: Option<ObjectSource>,
) {
    match m {
        // --- Layer 7a/7b/7c: power / toughness ([CR#613.4]) ---
        // `Set` overwrites the base (7a/7b); `Up`/`Down` modify an EXISTING
        // value (7c) — a 7c op never materializes a P/T on a P/T-less object (a
        // +1/+1 counter or a pump on a non-creature does nothing P/T-wise,
        // [CR#122.1a,613.4c]). The count is evaluated against `working`
        // immutably before the `get_mut`.
        Modification::Power(op) => {
            let v = eval_numeric_op(op, state, working, watcher, effect_controller);
            if let Some(d) = working.get_mut(&obj_id) {
                apply_numeric(op, &mut d.characteristics.power, v);
            }
        }
        Modification::Toughness(op) => {
            let v = eval_numeric_op(op, state, working, watcher, effect_controller);
            if let Some(d) = working.get_mut(&obj_id) {
                apply_numeric(op, &mut d.characteristics.toughness, v);
            }
        }
        // Non-count ops: borrow the entry mutably and mutate in place.
        m => apply_static(m, grant_runtime, effect_controller, state, working, obj_id),
    }
}

/// Evaluate a numeric op's amount to a scalar against the working state.
/// `Up`/`Down` carry a plain [`Count`]; `Set` carries a [`StatValue`] — it can
/// set a printed scalar, a dynamic count, or (unresolvable here) a CDA `*`/`X`
/// marker, which reads as 0 ([CR#208.2a]).
fn eval_numeric_op(
    op: &NumericOp,
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    watcher: Option<ObjectSource>,
    controller: PlayerId,
) -> Int {
    match op {
        NumericOp::Set(v) => eval_stat_value(v, state, working, watcher, controller),
        NumericOp::Up(c) | NumericOp::Down(c) => eval_count(c, state, working, watcher, controller),
    }
}

/// A [`StatValue`] evaluated against the working state — a printed `Number`
/// outright, a `Count` embed through [`eval_count`], and an unresolvable `X` /
/// CDA marker as 0 ([CR#208.2a] — "use 0 instead of that number").
fn eval_stat_value(
    v: &StatValue,
    state: &GameState,
    working: &BTreeMap<ObjectId, DerivedObject>,
    watcher: Option<ObjectSource>,
    controller: PlayerId,
) -> Int {
    match v {
        StatValue::Number(n) => *n,
        StatValue::Count(c) => eval_count(c, state, working, watcher, controller),
        StatValue::Variable | StatValue::DefinedByAbility => 0,
    }
}

/// Apply an already-evaluated numeric op (`v` = its count) to one optional stat
/// field. `Set` overwrites unconditionally (layer 7a/7b base set); `Up`/`Down`
/// modify only an EXISTING value (layer 7c, [CR#613.4c]).
fn apply_numeric(op: &NumericOp, field: &mut Option<Int>, v: Int) {
    match op {
        NumericOp::Set(_) => *field = Some(v),
        NumericOp::Up(_) => {
            if let Some(cur) = field {
                *field = Some(*cur + v);
            }
        }
        NumericOp::Down(_) => {
            if let Some(cur) = field {
                *field = Some(*cur - v);
            }
        }
    }
}

/// Resolve a layer-4 subtype `Ident` to a full `Subtype` ([CR#205.3]): look it
/// up in the engine's subtype registry (`state.subtypes`) so the granted
/// subtype's inherent rules (`confers` — e.g. a basic land type's [CR#305.6]
/// mana ability) ride along. An `Ident` absent from the registry yields a
/// minimal name-only `Subtype` (no `confers`, no `types`) so the type still
/// applies, carrying no inherent rules.
fn resolve_subtype(state: &GameState, name: &Ident) -> Subtype {
    state
        .subtypes
        .get(name)
        .cloned()
        .unwrap_or_else(|| Subtype {
            name: *name,
            types: Vec::new().into(),
            confers: Vec::new().into(),
        })
}

/// Resolve a layer-4 type `Ident` to a full `TypeDef` ([CR#300.1]): look it up
/// in `state.types` so a granted type's `confers` ride along. An `Ident` absent
/// from the registry yields a minimal name-only `TypeDef` (`permanent_type:
/// false`, no `confers`) — the type still applies, carrying no rules (fizzle,
/// never crash).
fn resolve_type(state: &GameState, name: &Ident) -> TypeDef {
    state.types.get(name).cloned().unwrap_or_else(|| TypeDef {
        name: *name,
        permanent_type: false,
        confers: Vec::new().into(),
    })
}

/// The count-free `Modification` arms (layers 2-6, 7d). Split out so the
/// count-bearing 7a-7c arms in `apply` can resolve their `Count` against
/// `working` immutably before taking the `&mut DerivedObject` here.
#[allow(
    clippy::match_same_arms,
    reason = "four intentionally-identical `{}` stub arms (BecomeBasicLandType, \
    AllCreatureTypes, SetText, BaseLoyalty/BaseDefense) kept distinct by their \
    per-arm comments and backing tickets (engine-layers-misc, \
    engine-layers-1-copy-facedown-text); #[expect] is deliberately avoided here \
    because it would churn every time one stub diverges while the others stay identical"
)]
fn apply_static(
    m: &Modification,
    grant_runtime: &crate::activation::AbilityRuntime,
    effect_controller: PlayerId,
    state: &GameState,
    working: &mut BTreeMap<ObjectId, DerivedObject>,
    obj_id: ObjectId,
) {
    let Some(d) = working.get_mut(&obj_id) else {
        return;
    };
    let c = &mut d.characteristics;
    match m {
        // The count-bearing P/T ops are handled in `apply`; never reach here.
        Modification::Power(_) | Modification::Toughness(_) => {
            unreachable!("count-bearing P/T ops are handled in `apply`")
        }
        // --- Layer 7d: switch ---
        Modification::SwitchPowerToughness => std::mem::swap(&mut c.power, &mut c.toughness),
        // --- Layer 4: type-changing ([CR#613.1d]) ---
        // The op's elements are bare `Ident` names; `Characteristics::card_types`
        // holds full `TypeDef` structs (with `permanent`/`confers`). Resolve each
        // name through the engine's type registry (`state.types`, populated from
        // the loaded plugin), so a granted type's `confers` ride along — exactly
        // like the `Subtypes` arm. An `Ident` absent from the registry applies as
        // a minimal name-only `TypeDef` (no `confers`): the type still applies,
        // it just carries no inherent rules (fizzle, never crash).
        Modification::CardTypes(op) => match op {
            CollectionOp::Set(names) => {
                c.card_types =
                    Arc::new(names.iter().map(|name| resolve_type(state, name)).collect());
            }
            CollectionOp::Add(name) => {
                let types = Arc::make_mut(&mut c.card_types);
                let resolved = resolve_type(state, name);
                if !types.iter().any(|t| t.name == resolved.name) {
                    types.push(resolved);
                }
            }
            CollectionOp::Remove(name) => {
                Arc::make_mut(&mut c.card_types).retain(|t| t.name != *name);
            }
        },
        Modification::Supertypes(op) => match op {
            CollectionOp::Set(ss) => c.supertypes = Arc::new(ss.to_vec()),
            CollectionOp::Add(s) => {
                let supertypes = Arc::make_mut(&mut c.supertypes);
                if !supertypes.contains(s) {
                    supertypes.push(*s);
                }
            }
            CollectionOp::Remove(s) => Arc::make_mut(&mut c.supertypes).retain(|x| x != s),
        },
        // --- Layer 4: subtype-changing ([CR#613.1d]) ---
        // The op's elements are name-keyed `SubtypeRef`s (`.name()` is the key);
        // `Characteristics::subtypes` holds full `Subtype` structs (with
        // `confers`/`types`). Resolve each name
        // through the engine's subtype registry (`state.subtypes`, populated from
        // the loaded plugin), so a granted subtype's inherent rules ride along —
        // e.g. a basic land type's mana ability ([CR#305.6]). An `Ident` absent
        // from the registry applies as a minimal name-only `Subtype` (no `confers`,
        // no `types`): the type still applies, it just carries no inherent rules.
        Modification::Subtypes(op) => match op {
            CollectionOp::Set(names) => {
                c.subtypes = Arc::new(
                    names
                        .iter()
                        .map(|name| resolve_subtype(state, &name.name()))
                        .collect(),
                );
            }
            CollectionOp::Add(name) => {
                let subtypes = Arc::make_mut(&mut c.subtypes);
                let resolved = resolve_subtype(state, &name.name());
                if !subtypes.iter().any(|s| s.name == resolved.name) {
                    subtypes.push(resolved);
                }
            }
            CollectionOp::Remove(name) => {
                Arc::make_mut(&mut c.subtypes).retain(|s| s.name != name.name());
            }
        },
        // [CR#305.7] deferred: replace land subtypes + strip abilities + grant basic
        // mana ability (no fixture yet). Do NOT implement the mana-ability construction.
        Modification::BecomeBasicLandType(_) => {}
        // TODO(kw-changeling): every-creature-type subtype fill
        // ([CR#702.73a,205.3m]) — no-op stub until built (reachable now that
        // Of(This) resolves source-relative in gather). The full fill needs
        // the declared creature-type registry threaded into the layer engine;
        // a changeling deriving as typeless-for-now is strictly better than a
        // panic, and matches the sibling deferred stubs in this match
        // (`Subtypes`/`BecomeBasicLandType`). Do NOT implement the fill here.
        Modification::AllCreatureTypes => {}
        // --- Layer 5: color-changing ([CR#613.1e]) ---
        Modification::Colors(op) => match op {
            CollectionOp::Set(cl) => c.colors = Arc::new(cl.to_vec()),
            CollectionOp::Add(x) => {
                let colors = Arc::make_mut(&mut c.colors);
                if !colors.contains(x) {
                    colors.push(*x);
                }
            }
            CollectionOp::Remove(x) => Arc::make_mut(&mut c.colors).retain(|y| y != x),
        },
        // --- Layer 6: ability-adding/removing ([CR#613.1f]) ---
        // `Arc::make_mut` realizes the copy-on-write: the shared per-card base
        // list is cloned only here, only for the objects an effect touches.
        Modification::GainAbility(a) => {
            // Respect any active "can't have" prohibition ([CR#613.1f]).
            if !d.cant_have.iter().any(|n| ability_is_named(a, n)) {
                Arc::make_mut(&mut c.abilities).push((**a).clone());
                d.ability_runtimes.push(grant_runtime.clone());
            }
        }
        // "Loses all abilities" strips every ability the object has, printed
        // and type-conferred alike ([CR#305.7]: a land whose subtype is set
        // loses the abilities its old land types gave it). A type's
        // ability-FREE rules are untouched because they are not abilities
        // ([CR#113.12]) and never entered this list.
        Modification::LoseAllAbilities => {
            retain_abilities(d, |_| false);
        }
        Modification::LoseAbility(name) => {
            retain_abilities(d, |x| !ability_is_named(x, name));
        }
        Modification::CantHaveAbility(name) => {
            // Remove any already-present instance of the named ability, then
            // record the prohibition so future GainAbility skips it.
            retain_abilities(d, |x| !ability_is_named(x, name));
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
        Modification::BaseLoyalty(_) | Modification::BaseDefense(_) => {
            // Loyalty/defense are not characteristics here; no layer applies.
        }
        // `Modification::flatten` at the `gather` boundary splices every
        // `Several` away and strips every `Expanded`, so neither reaches the
        // layer pass.
        Modification::Several(_) => {
            unreachable!("Several/Expanded are flattened before the engine")
        }
    }
}

fn retain_abilities(d: &mut DerivedObject, mut keep: impl FnMut(&Ability) -> bool) {
    let abilities = Arc::make_mut(&mut d.characteristics.abilities);
    let mut next_abilities = Vec::with_capacity(abilities.len());
    let mut next_runtimes = Vec::with_capacity(d.ability_runtimes.len());
    for (ability, runtime) in abilities.drain(..).zip(d.ability_runtimes.drain(..)) {
        if keep(&ability) {
            next_abilities.push(ability);
            next_runtimes.push(runtime);
        }
    }
    *abilities = next_abilities;
    d.ability_runtimes = next_runtimes;
}

/// Resolve a control-change effect's new controller ([CR#613.1b]) to a concrete
/// player. Per [CR#611.2c] the effect's references are locked when it is
/// created, so the controller parameter resolves to the effect's controller (the happy
/// path: "you gain control of …"). General `Reference` resolution needs the
/// resolve-time `ExecutionFrame` machinery (`engine-resolve-effects`); any other
/// reference is a documented seam that leaves the controller unchanged.
fn resolve_new_controller(
    reference: &deckmaste_core::Reference,
    effect_controller: PlayerId,
) -> Option<PlayerId> {
    if reference == &deckmaste_core::Reference::controller_parameter() {
        Some(effect_controller)
    } else {
        // SEAM: opponent / each-player / bound references need an `ExecutionFrame` to
        // resolve a specific player; not reachable by current control fixtures.
        None
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
    if !effect_conditions_hold(state, working, effect) {
        return Vec::new();
    }
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
    if !effect_conditions_hold(state, working, effect) {
        return;
    }
    if effect.locked.is_none() {
        effect.locked = Some(resolve_scope(state, working, &effect.scope, effect.watcher));
    }
    let targets = effect.locked.clone().expect("locked just set");
    for obj_id in targets {
        if working.contains_key(&obj_id) {
            for (m, runtime) in effect.changes.iter().zip(&effect.grant_runtimes) {
                if layer_of(m, effect.is_cda) == Some(layer) {
                    apply(
                        m,
                        runtime,
                        effect.controller,
                        state,
                        working,
                        obj_id,
                        effect.watcher,
                    );
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
        if probe.contains_key(&obj_id) {
            for (m, runtime) in d.changes.iter().zip(&d.grant_runtimes) {
                if layer_of(m, d.is_cda) == Some(layer) {
                    apply(
                        m,
                        runtime,
                        d.controller,
                        state,
                        &mut probe,
                        obj_id,
                        d.watcher,
                    );
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
    if e.is_cda != d.is_cda || (e.locked.is_some() && e.conditions.is_empty()) {
        return false;
    }
    let before: BTreeSet<ObjectId> = effect_targets(state, working, e).into_iter().collect();
    let probe = probe_apply(state, working, d, layer);
    let after: BTreeSet<ObjectId> = effect_targets(state, &probe, e).into_iter().collect();
    before != after
}

// ---------------------------------------------------------------------------
// Whole-pass fixpoint ([CR#613.7])
// ---------------------------------------------------------------------------

/// A content signature of a freshly-gathered effect set, used to detect the
/// fixpoint: when two successive iterations gather an equal set, no newly-
/// granted static appeared and derivation has converged ([CR#613.7]).
///
/// Built BEFORE the layer pass mutates any effect (so the per-effect `locked`
/// set — `None` at gather time — never enters the signature). The signature is
/// order-sensitive, but `gather` is deterministic ([CR#613.7] id/timestamp
/// order), so a stable effect set yields a byte-stable signature. Cardinality
/// alone would miss a same-iteration "one static vanished, another appeared"
/// swap; the full content tuple does not.
type EffectSignature = Vec<(
    Timestamp,
    bool,
    PlayerId,
    Vec<Condition>,
    ScopeSig,
    Vec<Modification>,
    Vec<crate::activation::AbilityRuntime>,
)>;

/// The `Eq`-able projection of a `ScopeResolved` for the signature.
/// `ScopeResolved` itself is not `Eq` (it is a working value), so project it.
#[derive(Clone, PartialEq, Eq)]
enum ScopeSig {
    Locked(Vec<ObjectId>),
    Floating(Arc<deckmaste_core::Region<Predicate>>),
}

fn effect_signature(effects: &[ActiveEffect]) -> EffectSignature {
    effects
        .iter()
        .map(|e| {
            let scope = match &e.scope {
                ScopeResolved::Locked(ids) => ScopeSig::Locked(ids.clone()),
                ScopeResolved::Floating(f) => ScopeSig::Floating(f.clone()),
            };
            (
                e.timestamp,
                e.is_cda,
                e.controller,
                e.conditions.clone(),
                scope,
                e.changes.clone(),
                e.grant_runtimes.clone(),
            )
        })
        .collect()
}

/// The base derived map ([CR#613.1]) for every card-backed object: printed
/// characteristics before any continuous effect. Rebuilt fresh each fixpoint
/// iteration so the pass always re-derives the FULL layer order from base —
/// see `run_layer_pass`.
fn base_map(state: &GameState) -> BTreeMap<ObjectId, DerivedObject> {
    let mut working: BTreeMap<ObjectId, DerivedObject> = BTreeMap::new();
    for obj in state.objects.iter() {
        if obj.card_id().is_none() {
            continue; // player proxy — no characteristics
        }
        working.insert(obj.id, base_values(state, obj.id));
    }
    working
}

/// Run the FULL [CR#613.3] layer pass over a fresh base map, applying `effects`
/// in layer + dependency order ([CR#613.8]). Consumes a starting `working`
/// (the base map) and the gathered `effects` (whose `locked` sets it mutates),
/// returning the derived map.
///
/// The pass re-derives from BASE through every layer in order — never "only
/// layer 6 and later". A static granted at layer 6 can produce an effect in any
/// earlier layer (a granted type-change is layer 4, a granted color effect
/// layer 5, a granted CDA layer 7a), so re-evaluation ([CR#613.7]) must replay
/// the whole order with the augmented effect set, not a suffix of it.
fn run_layer_pass(
    state: &GameState,
    mut working: BTreeMap<ObjectId, DerivedObject>,
    effects: &mut [ActiveEffect],
) -> BTreeMap<ObjectId, DerivedObject> {
    // Iterate layers in order, applying each effect's ops that belong to this
    // layer ([CR#613.3]).
    // [CR#305.6,611.3]: after L4 settles `card_types`/`subtypes`,
    // `fold_conferred_abilities` reinjects a layer-4-added type/subtype's
    // `Ability`-flavored confers into the derived ability list — BEFORE L6
    // ability additions/removals are applied (the call sits at the end of the
    // L4 iteration below).
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

        // Base order: CDAs first ([CR#613.3]), then by timestamp ([CR#613.7]).
        // This is the tiebreaker among independent effects and the fallback
        // inside a dependency loop ([CR#613.8b]).
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
                        d != e && depends_on(state, &working, &effects[e], &effects[d], layer)
                    })
                })
                .unwrap_or(0);
            let i = pending.remove(pos);
            apply_effect_in_layer(state, &mut working, &mut effects[i], layer);
        }
        // [CR#305.6,611.3]: once layer 4 has settled each object's card types
        // and subtypes, fold any type/subtype-conferred abilities into the
        // derived list — the SINGLE source of type/subtype conferral (the base
        // carries no confer of its own); running here — before layer 6 — lets a
        // later `LoseAllAbilities` strip the conferred abilities too
        // ([CR#305.7]).
        if layer == Layer::L4 {
            fold_conferred_abilities(&mut working);
        }
        // [CR#122.1a,613.4c,613.1f]: +1/+1 / -1/-1 P/T and keyword counters are
        // data-driven now — gathered as counter-conferred `Continuous` boosts
        // (see `gather` + `bake_counter_counts`) and applied as ordinary layer
        // `Modification`s, so no hardcoded 7c counter read remains here.
    }
    working
}

/// [CR#305.6,611.3]: the SINGLE source of type/subtype conferral. Folds every
/// object's CURRENT (post-layer-4) `card_types`/`subtypes` `Ability`-flavored
/// `confers` into its derived ability list, through the ONE emission path
/// ([`Property::conferred_ability`]) — PRINTED and
/// layer-4-ADDED types/subtypes alike, uniformly (an animate effect's
/// `Creature` type, a tribal `Subtypes(Add)`, contribute their conferred
/// abilities exactly like a printed type/subtype does).
///
/// No dedup: `printed_of_face` (the base) carries NO type/subtype confer, so
/// there is nothing here for a conferred ability to collide with — each is
/// pushed unconditionally, exactly once per pass. Removal falls out for free:
/// a type/subtype stripped at layer 4 is no longer in `card_types`/`subtypes`,
/// so its confers are simply not folded.
fn fold_conferred_abilities(working: &mut BTreeMap<ObjectId, DerivedObject>) {
    for d in working.values_mut() {
        let c = &mut d.characteristics;
        // Materialize the conferred abilities before touching `c.abilities`, so
        // the immutable borrow of `card_types`/`subtypes` ends first.
        let conferred: Vec<Ability> = c
            .card_types
            .iter()
            .flat_map(|t| t.confers.iter())
            .chain(c.subtypes.iter().flat_map(|s| s.confers.iter()))
            .filter_map(Property::conferred_ability)
            .collect();
        if conferred.is_empty() {
            continue;
        }
        let abilities = Arc::make_mut(&mut c.abilities);
        d.ability_runtimes.extend(
            std::iter::repeat_with(crate::activation::AbilityRuntime::default)
                .take(conferred.len()),
        );
        abilities.extend(conferred);
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
        // Whole-pass fixpoint ([CR#613.7] dependency / re-evaluation): a static
        // ability can itself be GRANTED by a layer-6 `GainAbility` (a lord that
        // grants a lord). The first pass gathers effect sources from PRINTED
        // abilities (the recursion-breaking base); each subsequent pass
        // re-gathers from the DERIVED ability list the previous pass produced,
        // so a newly-granted static is picked up as its own effect source. We
        // re-derive the FULL layer order from base each iteration (NOT just
        // layer 6+): a granted static can act in any layer, including ones
        // BEFORE layer 6 (a granted type-change is layer 4, a granted color
        // effect layer 5, a granted CDA layer 7a), so only a full re-evaluation
        // is correct ([CR#613.7]).
        //
        // The effect set grows monotonically toward the fixpoint (each new
        // grant only adds sources), and a real card interaction reaches it in
        // 1–2 iterations. The cap is a far-above-any-card guard against a
        // pathological non-convergence; exceeding it is a bug, not a deeper
        // board.
        //
        // PERF SEAM: re-deriving the whole pass a few times is the documented
        // per-rebuild cost (perf is explicitly low priority here); `layers()`
        // is already recomputed fresh on each call (caching is the noted later
        // optimization). No micro-optimization — a partial re-run would be
        // incorrect per the layer note above.
        const MAX_FIXPOINT_ITERS: usize = 16;

        // Iteration 0: gather from printed abilities (`derived = None`), run the
        // full pass from base.
        let mut effects = gather(self, None);
        let mut signature = effect_signature(&effects);
        let mut working = run_layer_pass(self, base_map(self), &mut effects);

        // Iterations 1..: re-gather from the derived map. If the gathered effect
        // set is unchanged, derivation has converged ([CR#613.7]) and `working`
        // already reflects it — done. Otherwise re-derive the full pass from
        // base with the augmented set and repeat.
        let mut converged = false;
        for _ in 1..MAX_FIXPOINT_ITERS {
            let mut next_effects = gather(self, Some(&working));
            let next_signature = effect_signature(&next_effects);
            if next_signature == signature {
                converged = true;
                break;
            }
            signature = next_signature;
            working = run_layer_pass(self, base_map(self), &mut next_effects);
        }
        // The cap is far above any real card interaction ([CR#613.7] reaches a
        // fixpoint in 1–2 iterations); hitting it means a non-converging effect
        // set, which is a bug rather than a deeper board.
        debug_assert!(
            converged,
            "layer fixpoint did not converge within {MAX_FIXPOINT_ITERS} iterations"
        );

        LayeredView(working)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::CharacteristicPredicate;
    use deckmaste_core::CollectionOp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Countable;
    use deckmaste_core::Duration;
    use deckmaste_core::KeywordAbility;
    use deckmaste_core::Modification;
    use deckmaste_core::NumericOp;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::StatValue;
    use deckmaste_core::StaticSpec;
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
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    #[test]
    fn type_predicate_matches_by_name_against_typedefs() {
        // A derived object whose card_types are the expanded Creature TypeDef
        // matches Type(Creature) by name.
        let chars = super::Characteristics {
            power: None,
            toughness: None,
            colors: Arc::new(vec![]),
            card_types: Arc::new(vec![Type::Creature.def()]),
            subtypes: Arc::new(vec![]),
            supertypes: Arc::new(vec![]),
            abilities: Arc::new(vec![]),
        };
        assert!(chars.has_type(Type::Creature));
        assert!(!chars.has_type(Type::Land));
    }

    /// A self-anthem `Static`: "creatures get +2/+2" — matches the carrying
    /// creature itself (a floating `SelectAll` set, no Stage-3 source-relative
    /// reference needed).
    fn pump_static() -> Ability {
        Ability::r#static(StaticSpec::Each(
            Selection::SelectAll(Arc::new(deckmaste_core::Region::candidate(
                Predicate::r#type(Type::Creature),
            ))),
            Arc::new(deckmaste_core::Region::candidate(StaticSpec::Modify(
                Reference::Reg(deckmaste_core::RefId(0)),
                Modification::Several(
                    vec![
                        Modification::Power(NumericOp::Up(Count::Literal(2))),
                        Modification::Toughness(NumericOp::Up(Count::Literal(2))),
                    ]
                    .into(),
                ),
            ))),
        ))
    }

    fn static_candidate_filter(body: Predicate) -> deckmaste_core::Region<Predicate> {
        use deckmaste_core::{DefId, Kind, Param, Provenance, Region};

        Region::new(
            Arc::from([
                Param {
                    def: DefId(0),
                    kind: Kind::Entity,
                    provenance: Provenance::Candidate(deckmaste_core::Domain::Entity),
                },
                Param {
                    def: DefId(1),
                    kind: Kind::Entity,
                    provenance: Provenance::Source,
                },
                Param {
                    def: DefId(2),
                    kind: Kind::Entity,
                    provenance: Provenance::Controller,
                },
            ]),
            body,
        )
    }

    fn controller_candidate_filter(body: Predicate) -> deckmaste_core::Region<Predicate> {
        use deckmaste_core::{DefId, Kind, Param, Provenance, Region};

        Region::new(
            Arc::from([
                Param {
                    def: DefId(0),
                    kind: Kind::Entity,
                    provenance: Provenance::Candidate(deckmaste_core::Domain::Entity),
                },
                Param {
                    def: DefId(1),
                    kind: Kind::Entity,
                    provenance: Provenance::Controller,
                },
            ]),
            body,
        )
    }

    /// Mint a 2/2 creature carrying `abilities` onto the battlefield (player
    /// 0).
    fn creature_on_field(mut state: GameState, abilities: Vec<Ability>) -> (GameState, ObjectId) {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Creature".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            abilities,
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        (state, id)
    }

    /// A 2/2 creature carrying `abilities` and printing one `subtype` whose
    /// `confers` supply the conferral under test.
    fn subtyped_creature(
        mut state: GameState,
        subtype: deckmaste_core::Subtype,
        abilities: Vec<Ability>,
    ) -> (GameState, ObjectId) {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Creature".into(),
            types: vec![Type::Creature.def()],
            subtypes: vec![subtype],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            abilities,
            ..Characteristics::default()
        }));
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
            rows: vec![],
            origin: None,
            is_cda: false,
        });
    }

    /// Whether `id`'s DERIVED ability list (what the SBA sweep,
    /// `attachment_legal`, and the deontic reads consult) carries a `Static`.
    /// This is the surface layer-6 ability removal acts on
    /// — gather's effect-source collection reads PRINTED abilities to break the
    /// `layers()` recursion (documented in `gather`), so P/T anthem application
    /// is intentionally NOT the right observable here.
    fn derived_has_static(state: &GameState, id: ObjectId) -> bool {
        state
            .layers()
            .get(id)
            .abilities
            .iter()
            .any(|a| matches!(a, Ability::Static(_)))
    }

    /// [CR#113.12]: `LoseAllAbilities` strips a granted ability (Trample) but
    /// cannot reach a type's ability-FREE rule — an Equipment-style
    /// `Property::Static(May(Attach))` conferral is a quality of the object,
    /// never entered the ability list, and is still read by the static walk
    /// afterwards.
    #[test]
    fn conferred_static_rule_survives_lose_all_abilities() {
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Property;
        use deckmaste_core::Subtype;

        let rule = Property::Static(Arc::new(deckmaste_core::Region::candidate(
            StaticSpec::Deontic(Deontic::May(DeonticAction::Attach {
                what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                to: Predicate::r#type(Type::Creature),
            })),
        )));
        let (mut state, id) = subtyped_creature(
            game(),
            Subtype {
                name: "Equipment".into(),
                types: Vec::new().into(),
                confers: vec![rule].into(),
            },
            vec![Ability::Keyword(KeywordAbility::Trample)],
        );

        let may_attach = |state: &GameState| {
            let view = state.layers();
            crate::legal::object_has_static(&view, id, &|e: &StaticSpec| {
                matches!(
                    e,
                    StaticSpec::Deontic(Deontic::May(DeonticAction::Attach { .. }))
                )
            })
        };
        assert!(may_attach(&state), "the conferred rule is read pre-removal");
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

        assert!(
            may_attach(&state),
            "the ability-free type rule survives LoseAllAbilities ([CR#113.12])"
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

    /// [CR#305.6]: a REGISTRY-conferred basic-land mana ability — emitted
    /// through the one conferral path (`Property::conferred_ability`) — is an
    /// ORDINARY ability of the object: the island HAS it for card-facing
    /// queries, it is activatable, and a lose-all-abilities effect REMOVES it
    /// ([CR#305.7]), leaving the island with no ability at all.
    #[test]
    fn conferred_basic_land_mana_is_an_ordinary_ability() {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::CostComponent;
        use deckmaste_core::Instruction;
        use deckmaste_core::ManaProduction;
        use deckmaste_core::ManaSpec;
        use deckmaste_core::Property;
        use deckmaste_core::Reference;
        use deckmaste_core::Subtype;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::step::StepOutcome;

        // The Island registry row's conferral ([CR#305.6]): "{T}: Add {U}",
        // carried on the subtype value exactly as the BasicLandType
        // meta-macro declares it.
        let island = Subtype {
            name: "Island".into(),
            types: vec![Type::Land].into(),
            confers: vec![Property::Ability(Arc::new(Ability::activated(
                deckmaste_core::ActivatedAbility {
                    ability_word: None,
                    targets: [].into(),
                    cost: Arc::<[CostComponent]>::from(vec![CostComponent::Tap]).into(),
                    from: None,
                    window: None,
                    condition: None,
                    limits: vec![].into(),
                    effect: Instruction::act(deckmaste_core::Action::AddMana(
                        Reference::Reg(deckmaste_core::RefId(1)),
                        Count::Literal(1),
                        ManaProduction::Bare(ManaSpec::Specific(ColorOrColorless::Color(
                            deckmaste_core::Color::Blue,
                        ))),
                    ))
                    .into(),
                },
            )))]
            .into(),
        };
        let mut state = game();
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Island".into(),
            types: vec![Type::Land.def()],
            subtypes: vec![island],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);

        let taps_for_blue = |state: &GameState| {
            crate::derive::usable_abilities(state, id).iter().any(|a| {
                crate::derive::tap_mana_ability(a)
                    == Some((ColorOrColorless::Color(deckmaste_core::Color::Blue), 1))
            })
        };
        assert!(taps_for_blue(&state), "the conferral taps for {{U}}");
        // HAS: a type-conferred ability occupies the ordinary hierarchy, so
        // card-facing queries see it ([CR#305.6] "has the intrinsic ability").
        assert_eq!(
            crate::derive::abilities(&state, id).len(),
            1,
            "the island HAS its conferred mana ability ([CR#305.6])"
        );

        state.begin_payment_proposal(PlayerId(0));
        state.activate_root_mana_ability(id, 0);
        for _ in 0..40 {
            if state.payment_depth() == 0
                && state
                    .player(PlayerId(0))
                    .mana_pool
                    .amount(ColorOrColorless::Color(deckmaste_core::Color::Blue))
                    == 1
            {
                break;
            }
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(DecisionPointKind::Payment(_)) => {
                    let decision = state
                        .auto_payment_pending()
                        .expect("automatic payment decision");
                    state
                        .submit_decision(decision)
                        .expect("automatic payment succeeds");
                }
                StepOutcome::NeedsDecision(DecisionPointKind::ChooseManaReversals(prompt)) => {
                    let maximal = prompt
                        .legal
                        .iter()
                        .max_by_key(|set| set.len())
                        .cloned()
                        .expect("a reversal prompt offers a legal set");
                    state
                        .submit_decision(Decision::ManaReversals(maximal))
                        .expect("automatic reversal succeeds");
                }
                other => panic!("unexpected decision while activating conferred mana: {other:?}"),
            }
        }
        assert!(
            state.objects.obj(id).tapped,
            "the conferred ability pays {{T}}"
        );
        assert_eq!(
            state
                .player(PlayerId(0))
                .mana_pool
                .amount(ColorOrColorless::Color(deckmaste_core::Color::Blue)),
            1,
            "the conferred ability adds {{U}} through the payment protocol"
        );

        // LACKS: "loses all abilities" removes the conferred ability like
        // printed text ([CR#305.7]) — the island no longer taps for blue and
        // reads as having no abilities.
        lose_all_abilities(&mut state, id);
        assert!(
            !taps_for_blue(&state),
            "LoseAllAbilities removes the conferred [CR#305.6] mana ability"
        );
        assert!(
            crate::derive::abilities(&state, id).is_empty(),
            "the island LACKS every ability after LoseAllAbilities"
        );
    }

    /// A printed static ability is removed from the derived ability list by
    /// `LoseAllAbilities` (so the SBA / legality reads no longer see it).
    #[test]
    fn normal_static_is_removed_by_lose_all_abilities() {
        let (mut state, id) = creature_on_field(game(), vec![pump_static()]);
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

    /// [CR#613.1f]: `LoseAbility(Trample)` removes the NAMED keyword from the
    /// derived list and nothing else — an unnamed static ability stays.
    #[test]
    fn lose_ability_removes_only_the_named_keyword() {
        let (mut state, id) = creature_on_field(
            game(),
            vec![pump_static(), Ability::Keyword(KeywordAbility::Trample)],
        );
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![id]),
            changes: vec![Modification::LoseAbility("Trample".into())],
            duration: Duration::EndOfGame,
            rows: vec![],
            origin: None,
            is_cda: false,
        });
        assert!(
            derived_has_static(&state, id),
            "an unnamed static survives LoseAbility(Trample)"
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

    /// [CR#113.12]: an ability-free type rule is not an ability, so the
    /// card-facing `derive::abilities` view never carries it — an object whose
    /// only conferral is a `Property::Static` rule reads as having no
    /// abilities, while its printed keyword stays visible.
    #[test]
    fn derive_abilities_omits_ability_free_type_rules() {
        use deckmaste_core::Deontic;
        use deckmaste_core::DeonticAction;
        use deckmaste_core::Property;
        use deckmaste_core::Subtype;

        let rule_subtype = || Subtype {
            name: "Equipment".into(),
            types: Vec::new().into(),
            confers: vec![Property::Static(Arc::new(
                deckmaste_core::Region::candidate(StaticSpec::Deontic(Deontic::May(
                    DeonticAction::Attach {
                        what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                        to: Predicate::r#type(Type::Creature),
                    },
                ))),
            ))]
            .into(),
        };

        // Rule-only: the card-facing list is empty.
        let (state, id) = subtyped_creature(game(), rule_subtype(), vec![]);
        assert!(
            crate::derive::abilities(&state, id).is_empty(),
            "an ability-free type rule is no ability at all"
        );

        // Rule + a printed keyword: the keyword is the only ability.
        let (state, id) = subtyped_creature(
            game(),
            rule_subtype(),
            vec![Ability::Keyword(KeywordAbility::Trample)],
        );
        let facing = crate::derive::abilities(&state, id);
        assert_eq!(facing.len(), 1, "only the printed ability is card-facing");
        assert!(matches!(
            facing[0],
            Ability::Keyword(KeywordAbility::Trample)
        ));
    }

    /// A host-targeting static: "enchanted/equipped creature gets +N/+N",
    /// authored as `Modify(AttachHostOf(This), +N/+N)`. The source-relative
    /// `AttachHostOf(This)` reference is exactly the Stage-3 seam this task
    /// resolves.
    fn host_pump_static(n: u32) -> Ability {
        use deckmaste_core::Reference;
        Ability::r#static(StaticSpec::Modify(
            Reference::AttachHostOf(Arc::new(Reference::Reg(deckmaste_core::RefId(0)))),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(Count::Literal(n))),
                    Modification::Toughness(NumericOp::Up(Count::Literal(n))),
                ]
                .into(),
            ),
        ))
    }

    /// Mint a bare (non-creature) permanent carrying `abilities` onto the
    /// battlefield (player 0) — used as an attachment whose static targets its
    /// host. No power/toughness, so it never gets caught by an anthem itself.
    fn permanent_on_field(mut state: GameState, abilities: Vec<Ability>) -> (GameState, ObjectId) {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Attachment".into(),
            types: vec![Type::Enchantment.def()],
            abilities,
            ..Characteristics::default()
        }));
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

    /// M1 formerly covered `These([This, This])` dedup: a static naming the
    /// same object twice through a fixed reference LIST had to apply its
    /// additive op only once. That list shape is GONE — `StaticSpec::Modify`
    /// now takes exactly one `Reference` ([CR#613.6] positional single-object
    /// contract), so two references can no longer collide inside one `Modify`;
    /// the dedup scenario is structurally impossible, not merely untested.
    /// This is the closest surviving single-`Modify` shape: a plain self-pump,
    /// applied exactly once by construction.
    #[test]
    fn single_modify_applies_its_change_once() {
        use deckmaste_core::Reference;

        let pump = Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Power(NumericOp::Up(Count::Literal(1))),
        ));
        let (state, id) = creature_on_field(game(), vec![pump]);
        assert_eq!(
            state.layers().power(id),
            Some(3),
            "Modify(This, +1) applies once: base 2 → 3"
        );
    }

    /// Load a card from the wizards corpus with the builtin keyword/subtype
    /// macros in scope (the real expansion path), forcing it onto the
    /// battlefield as player 0's permanent. Reads only the macro corpus plus
    /// the single named card file — not the whole 30k-card directory. Only ever
    /// called from `#[cfg_attr(not(wizards_corpus), ignore)]` tests, so the
    /// corpus is guaranteed present when this runs (build.rs sets the
    /// `wizards_corpus` cfg from the directory's presence).
    fn wizards_permanent(name: &str) -> (GameState, ObjectId) {
        use std::path::Path;

        use deckmaste_plugin::plugin::Plugin;

        let plugin = Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/wizards"),
        )
        .expect("load wizards plugin (builtin prelude)");
        let card = Arc::new(
            plugin
                .card(name)
                .unwrap_or_else(|e| panic!("load {name}: {e:?}"))
                .core,
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
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
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

    /// Devoid runs the IMPLEMENTED `Colors(Set([]))` arm (not a panic): a
    /// devoid permanent derives colorless ([CR#604.3,105.2c]). This is the
    /// intended, correct behavior un-gated alongside Changeling by Stage 3.
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn devoid_derives_colorless() {
        // Havoc Sower: a Black creature with `Keyword(Devoid)` — Devoid's CDA
        // `Modify(of: Of(This), changes: [Colors(Set([]))])` makes it colorless.
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
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_core::Subtype;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Test Tribe".into(),
            types: vec![Type::Creature.def()],
            subtypes: vec![Subtype {
                name: subtype.into(),
                types: vec![Type::Creature].into(),
                confers: vec![].into(),
            }],
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            abilities,
            ..Characteristics::default()
        }));
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
    /// i.e. `Matching(And([Creature, Not(Ref(This)), Subtype("Goblin"),
    /// ControlledBy(Ref(You))]))`. Its scope names both `~` (`Ref(This)`) and
    /// `you` (`Ref(You)`), which are exactly the carrier-bound refs the derived
    /// path must anchor against the host permanent.
    fn goblin_lord_static() -> Ability {
        use deckmaste_core::Reference;
        use deckmaste_core::RelationPredicate;
        Ability::r#static(StaticSpec::Each(
            Selection::SelectAll(Arc::new(static_candidate_filter(Predicate::And(
                vec![
                    Predicate::creature(),
                    Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(
                        deckmaste_core::RefId(1),
                    )))),
                    Predicate::Characteristic(CharacteristicPredicate::Subtype(
                        deckmaste_core::SubtypeRef::named("Goblin".into()),
                    )),
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::Reg(deckmaste_core::RefId(2)),
                    )))),
                ]
                .into(),
            )))),
            Arc::new(deckmaste_core::Region::candidate(StaticSpec::Modify(
                Reference::Reg(deckmaste_core::RefId(0)),
                Modification::Several(
                    vec![
                        Modification::Power(NumericOp::Up(Count::Literal(1))),
                        Modification::Toughness(NumericOp::Up(Count::Literal(1))),
                    ]
                    .into(),
                ),
            ))),
        ))
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

    /// engine-transform: a permanent's base characteristics reflect its CURRENT
    /// face ([CR#712.8d,712.8e]). A hand-authored transforming DFC shows FRONT
    /// P/T while `Side::Front`, and BACK P/T once its `side` flips to `Back`
    /// (driven directly here; Task 5 makes `Transform` set it).
    #[test]
    fn back_up_permanent_shows_back_face_characteristics() {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        use deckmaste_card::DoubleFacedLayout;
        use deckmaste_core::StatValue;
        use deckmaste_core::Type;

        use crate::object::Side;
        let front = CardFace::from(Characteristics {
            name: "Delverish".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(1)),
            toughness: Some(StatValue::Number(1)),
            ..Characteristics::default()
        });
        let back = CardFace::from(Characteristics {
            name: "Insectile Aberration".into(),
            types: vec![Type::Creature.def()],
            power: Some(StatValue::Number(3)),
            toughness: Some(StatValue::Number(2)),
            ..Characteristics::default()
        });
        let card = Card::DoubleFaced {
            layout: DoubleFacedLayout::Transforming,
            front,
            back,
        };
        let mut state = game();
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);

        assert_eq!(
            state.layers().power(id),
            Some(1),
            "front-up shows front P [CR#712.8d]"
        );

        state.objects.obj_mut(id).side = Side::Back;
        let view = state.layers();
        assert_eq!(view.power(id), Some(3), "back-up shows back P [CR#712.8e]");
        assert_eq!(view.toughness(id), Some(2), "back-up shows back T");
    }

    /// engine-static-scope-carrier: a spell-built FLOATING scope (Overrun's
    /// `Continuously(Modify(of: Matching(And([Creature,
    /// ControlledBy(Ref(You))] )), …))`) names `you` and so also rode the
    /// carrier `todo!`. The floating effect carries its controller, so
    /// `Ref(You)` resolves and the player's own creatures are buffed
    /// without panic.
    #[test]
    fn floating_controlled_by_you_scope_resolves() {
        use deckmaste_core::Reference;
        use deckmaste_core::RelationPredicate;

        // Two plain creatures: one player 0 controls, one player 1 controls.
        let (state, mine) = typed_creature(game(), "Beast", PlayerId(0), vec![]);
        let (mut state, theirs) = typed_creature(state, "Beast", PlayerId(1), vec![]);

        // An Overrun-shaped floating effect controlled by player 0.
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Floating(Arc::new(controller_candidate_filter(Predicate::And(
                vec![
                    Predicate::creature(),
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::Reg(deckmaste_core::RefId(1)),
                    )))),
                ]
                .into(),
            )))),
            changes: vec![
                Modification::Power(NumericOp::Up(Count::Literal(2))),
                Modification::Toughness(NumericOp::Up(Count::Literal(2))),
            ],
            duration: Duration::EndOfGame,
            rows: vec![],
            origin: None,
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

    // -----------------------------------------------------------------------
    // engine-layers-dynamic-counts: dynamic `Count` evaluation in the layer
    // engine (CDAs, "+X/+X for each …").
    // -----------------------------------------------------------------------

    /// A self-CDA-SHAPED static ([CR#604.3]) that SETS its own P/T to the
    /// number of creatures on the battlefield (the Tarmogoyf/creature-count
    /// pattern) — `Power`/`Toughness(Set(CountOf(Creature)))` scoped
    /// `Of(This)`. The 7a/CDA layer distinction is deferred (0 cards use it;
    /// `characteristic_defining` is gone from the surface), so this routes
    /// through plain 7b `Set` like any other static `Modify` — the dynamic
    /// `Count` re-derivation this test exercises is a property of `Modify`'s
    /// gather, not of the layer number.
    fn creature_count_cda() -> Ability {
        use deckmaste_core::Reference;
        let count = Count::CountOf(Countable::Objects(Arc::new(
            deckmaste_core::Region::candidate(Predicate::creature()),
        )));
        Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Set(StatValue::Count(count.clone()))),
                    Modification::Toughness(NumericOp::Set(StatValue::Count(count))),
                ]
                .into(),
            ),
        ))
    }

    /// A CDA setting P/T to `CountOf(creatures on the battlefield)` resolves to
    /// the live creature count, and tracks the board as creatures are added
    /// ([CR#604.3,613.6] — the count is re-derived each layer pass).
    #[test]
    fn cda_sets_pt_to_dynamic_creature_count() {
        // Lone creature carrying the CDA: it is the only creature → 1/1.
        let (state, goyf) = creature_on_field(game(), vec![creature_count_cda()]);
        let view = state.layers();
        assert_eq!(
            view.power(goyf),
            Some(1),
            "one creature on the battlefield → power 1"
        );
        assert_eq!(view.toughness(goyf), Some(1), "…and toughness 1");

        // Add a second (plain) creature: now two creatures → the CDA derives 2/2.
        let (state, _other) = creature_on_field(state, vec![]);
        let view = state.layers();
        assert_eq!(
            view.power(goyf),
            Some(2),
            "adding a creature bumps the CDA to power 2"
        );
        assert_eq!(view.toughness(goyf), Some(2), "…and toughness 2");
    }

    /// A self-pump static ([CR#613.4c] layer 7c) that adds `CountOf(creatures)`
    /// to its own P/T — "this creature gets +X/+X for each creature you
    /// control", here counting every creature for simplicity. Scoped
    /// `Of(This)`.
    fn creature_count_pump() -> Ability {
        use deckmaste_core::Reference;
        let count = Count::CountOf(Countable::Objects(Arc::new(
            deckmaste_core::Region::candidate(Predicate::creature()),
        )));
        Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(count.clone())),
                    Modification::Toughness(NumericOp::Up(count)),
                ]
                .into(),
            ),
        ))
    }

    /// A dynamic 7c pump ("+X/+X for each creature") adds the live count to an
    /// existing P/T: a lone 2/2 with the pump counts itself (one creature) → +1
    /// → 3/3; with a second creature present → +2 → 4/4.
    #[test]
    fn dynamic_7c_pump_adds_creature_count() {
        let (state, pumped) = creature_on_field(game(), vec![creature_count_pump()]);
        let view = state.layers();
        assert_eq!(
            view.power(pumped),
            Some(3),
            "base 2 + (1 creature) = 3 power"
        );
        assert_eq!(view.toughness(pumped), Some(3), "…and 3 toughness");

        // A second creature → the pump's count is now 2 → base 2 + 2 = 4.
        let (state, _other) = creature_on_field(state, vec![]);
        let view = state.layers();
        assert_eq!(
            view.power(pumped),
            Some(4),
            "base 2 + (2 creatures) = 4 power"
        );
    }

    /// [CR#613.4c]: a dynamic 7c add is a no-op P/T-wise on a P/T-less
    /// permanent — the existing 7c guard fires even when the count is nonzero.
    /// A non-creature permanent carrying "+X/+X for each creature" derives no
    /// power/toughness (the count would be 1, but there is nothing to add to).
    #[test]
    fn dynamic_7c_pump_is_noop_on_pt_less_permanent() {
        // The pump is on a bare Enchantment (no printed P/T); a separate
        // creature exists so the count is genuinely nonzero (1).
        let (state, _creature) = creature_on_field(game(), vec![]);
        let (state, ench) = permanent_on_field(state, vec![creature_count_pump()]);
        let view = state.layers();
        assert_eq!(
            view.power(ench),
            None,
            "a dynamic +X/+X never materializes P/T on a P/T-less permanent"
        );
        assert_eq!(view.toughness(ench), None, "…toughness stays None too");
    }

    // -----------------------------------------------------------------------
    // layers-layer-4-subtypes: layer-4 subtype-changing ([CR#613.1d]). A
    // `Subtypes(...)` modification carries name-keyed `SubtypeRef`s; the engine
    // resolves each through the injected subtype registry (`state.subtypes`),
    // so a granted subtype's `confers` ride along.
    // -----------------------------------------------------------------------

    /// Lock `Subtypes(...)` (layer-4) continuous changes carrying the named
    /// subtypes onto `id`.
    fn lock_subtype_mod(state: &mut GameState, id: ObjectId, changes: Vec<Modification>) {
        let timestamp = state.objects.next_timestamp();
        state.continuous.push(ContinuousEffect {
            timestamp,
            controller: PlayerId(0),
            scope: ScopeResolved::Locked(vec![id]),
            changes,
            duration: Duration::EndOfGame,
            rows: vec![],
            origin: None,
            is_cda: false,
        });
    }

    /// Register `name` in the engine's subtype registry as a `Creature` subtype
    /// conferring `Trample` ([CR#205.3]) — a stand-in for a tribal subtype
    /// whose membership carries an inherent rule. Lets a test prove the
    /// resolved `Subtype`'s `confers` ride along when the name is granted.
    fn register_trample_subtype(state: &mut GameState, name: &str) {
        use deckmaste_core::KeywordAbility;
        use deckmaste_core::Property;
        use deckmaste_core::Subtype;
        state.subtypes.insert(
            name.into(),
            Subtype {
                name: name.into(),
                types: vec![Type::Creature].into(),
                confers: vec![Property::Ability(Arc::new(Ability::Keyword(
                    KeywordAbility::Trample,
                )))]
                .into(),
            },
        );
    }

    /// [CR#613.1d]: `Subtypes(Add(Sliver))` on a creature adds the registered
    /// `Sliver` subtype to its derived list ("becomes a Sliver in addition to
    /// its other types"), and the registry entry's `confers` ride along (so a
    /// downstream consumer sees the inherent rule the subtype membership
    /// carries).
    #[test]
    fn add_subtypes_appends_registered_subtype_with_confers() {
        use deckmaste_core::Property;

        let (mut state, id) = creature_on_field(game(), vec![]);
        register_trample_subtype(&mut state, "Sliver");
        lock_subtype_mod(
            &mut state,
            id,
            vec![Modification::Subtypes(CollectionOp::Add("Sliver".into()))],
        );

        let view = state.layers();
        let sliver = view
            .get(id)
            .subtypes
            .iter()
            .find(|s| s.name == "Sliver")
            .expect("derived subtypes include the granted Sliver");
        // The registry's confers ride along on the resolved Subtype.
        assert!(
            sliver.confers.iter().any(|p| matches!(
                p,
                Property::Ability(a)
                    if matches!(&**a, Ability::Keyword(deckmaste_core::KeywordAbility::Trample))
            )),
            "the granted Sliver carries its registered confers; got {:?}",
            sliver.confers
        );
    }

    /// [CR#613.1d]: `Subtypes(Add(...))` is additive — it keeps the printed
    /// subtype ("in addition to its other types") and dedups, so granting a
    /// subtype the object already prints does not duplicate it.
    #[test]
    fn add_subtypes_is_additive_and_dedups() {
        // A creature that already prints the "Goblin" subtype.
        let (mut state, id) = typed_creature(game(), "Goblin", PlayerId(0), vec![]);
        register_trample_subtype(&mut state, "Sliver");
        lock_subtype_mod(
            &mut state,
            id,
            // Add Sliver AND a redundant Goblin (already printed): Goblin must
            // not be duplicated. Each `Add` is one element ([CR#613.1d]).
            vec![
                Modification::Subtypes(CollectionOp::Add("Sliver".into())),
                Modification::Subtypes(CollectionOp::Add("Goblin".into())),
            ],
        );

        let view = state.layers();
        let names: Vec<&str> = view
            .get(id)
            .subtypes
            .iter()
            .map(|s| s.name.as_str())
            .collect();
        assert!(names.contains(&"Goblin"), "printed Goblin survives");
        assert!(names.contains(&"Sliver"), "granted Sliver added");
        assert_eq!(
            names.iter().filter(|n| **n == "Goblin").count(),
            1,
            "Goblin is not duplicated, got {names:?}"
        );
    }

    /// [CR#613.1d]: `Subtypes(Set(...))` REPLACES the whole printed subtype list
    /// (the "is a … and is no longer …" / basic-land-type pattern), resolving
    /// each name through the registry.
    #[test]
    fn set_subtypes_replaces_printed_list() {
        // Printed Goblin → set to Sliver only.
        let (mut state, id) = typed_creature(game(), "Goblin", PlayerId(0), vec![]);
        register_trample_subtype(&mut state, "Sliver");
        lock_subtype_mod(
            &mut state,
            id,
            vec![Modification::Subtypes(CollectionOp::Set(
                vec!["Sliver".into()].into(),
            ))],
        );

        let view = state.layers();
        let names: Vec<&str> = view
            .get(id)
            .subtypes
            .iter()
            .map(|s| s.name.as_str())
            .collect();
        assert_eq!(names, vec!["Sliver"], "printed Goblin replaced by Sliver");
    }

    /// An `Ident` absent from the registry still applies as a minimal name-only
    /// `Subtype` — the type applies, it just carries no inherent rules (no
    /// `confers`, no `types`).
    #[test]
    fn unknown_subtype_applies_as_minimal() {
        // No registry entry for "Eldrazi".
        let (mut state, id) = creature_on_field(game(), vec![]);
        lock_subtype_mod(
            &mut state,
            id,
            vec![Modification::Subtypes(CollectionOp::Add("Eldrazi".into()))],
        );

        let view = state.layers();
        let eldrazi = view
            .get(id)
            .subtypes
            .iter()
            .find(|s| s.name == "Eldrazi")
            .expect("an unregistered subtype still applies by name");
        assert!(
            eldrazi.confers.is_empty() && eldrazi.types.is_empty(),
            "an unknown subtype carries no inherent rules: {eldrazi:?}"
        );
    }

    // -----------------------------------------------------------------------
    // engine-layers-fixpoint: a static ability GRANTED by a continuous effect
    // ([CR#613.7] re-evaluation) is itself gathered as an effect source. The
    // first layer pass reads effect sources from PRINTED abilities (the
    // recursion-breaking base); each later pass re-gathers from the DERIVED
    // ability list the previous pass produced, so a granted static functions.
    // -----------------------------------------------------------------------

    /// A SELF-pump `Static`: "this creature gets +1/+1", scoped `Of(This)` so a
    /// holder pumps only itself by exactly +1/+1. Used as the GRANTED ability
    /// (each creature that gains it pumps itself once), keeping the bonus
    /// unambiguous regardless of how many creatures hold it.
    fn self_pump_static() -> Ability {
        use deckmaste_core::Reference;
        Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Several(
                vec![
                    Modification::Power(NumericOp::Up(Count::Literal(1))),
                    Modification::Toughness(NumericOp::Up(Count::Literal(1))),
                ]
                .into(),
            ),
        ))
    }

    /// A lord-granting-a-lord `Static`: "other creatures you control have
    /// '<granted>'", i.e. `Each(SelectAll(And([Creature, Not(Ref(This))])),
    /// Modify(It, GainAbility(granted)))`. The distributor's filter names `~`
    /// (`Ref(This)`) so it grants every OTHER creature (not itself); the
    /// `granted` static is a layer-6 `GainAbility` payload that only
    /// functions once the fixpoint re-gathers it from the derived list.
    fn lord_granting_static(granted: Ability) -> Ability {
        use deckmaste_core::Reference;
        Ability::r#static(StaticSpec::Each(
            Selection::SelectAll(Arc::new(static_candidate_filter(Predicate::And(
                vec![
                    Predicate::creature(),
                    Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(
                        deckmaste_core::RefId(1),
                    )))),
                ]
                .into(),
            )))),
            Arc::new(deckmaste_core::Region::candidate(StaticSpec::Modify(
                Reference::Reg(deckmaste_core::RefId(0)),
                Modification::GainAbility(Arc::new(granted)),
            ))),
        ))
    }

    /// engine-layers-fixpoint, THE granting case: a creature whose static
    /// grants OTHER creatures a static that pumps the holder. Without the
    /// fixpoint the granted static would be added to the derived ability
    /// list (layer 6) but NEVER gathered as an effect source (gather ran
    /// once, from printed), so the pump would silently do nothing. WITH the
    /// fixpoint the second pass re-gathers the granted static from the
    /// derived list and its +1/+1 applies.
    #[test]
    fn granted_static_is_gathered_to_fixpoint() {
        // Grantor G grants every OTHER creature the self-pump.
        let (state, grantor) =
            creature_on_field(game(), vec![lord_granting_static(self_pump_static())]);
        // A plain creature B — should gain the granted self-pump and become 3/3.
        let (state, beneficiary) = creature_on_field(state, vec![]);

        let view = state.layers();

        // Layer 6 granted the static: B's DERIVED ability list now carries a
        // `Static` (the grant happened), even though B prints no abilities.
        assert!(
            view.get(beneficiary)
                .abilities
                .iter()
                .any(|a| matches!(a, Ability::Static(_))),
            "the granted static is present in B's derived ability list (layer 6)"
        );
        // The FIXPOINT payoff: the granted self-pump was re-gathered as an
        // effect source, so B derives 3/3 ([CR#613.7]).
        assert_eq!(
            view.power(beneficiary),
            Some(3),
            "the granted +1/+1 static applies via the fixpoint re-gather"
        );
        assert_eq!(view.toughness(beneficiary), Some(3), "…on toughness too");
        // The grantor does NOT grant itself (`Not(Ref(This))`), so it never
        // gains the self-pump and stays 2/2 — proving the carrier-bound scope
        // resolved against G even on the GRANTING static.
        assert_eq!(
            view.power(grantor),
            Some(2),
            "the grantor excludes itself (Not(Ref(This))) — no granted pump"
        );
    }

    /// Control for the granting case: WITHOUT the grantor present, a plain
    /// creature is never granted the pump and stays 2/2 — the +1/+1 in
    /// `granted_static_is_gathered_to_fixpoint` is entirely attributable to the
    /// grant + fixpoint, not to any baseline.
    #[test]
    fn ungranted_creature_is_not_pumped() {
        let (state, plain) = creature_on_field(game(), vec![]);
        let view = state.layers();
        assert!(
            view.get(plain).abilities.is_empty(),
            "a plain creature has no derived abilities to gather"
        );
        assert_eq!(
            view.power(plain),
            Some(2),
            "no grantor, no granted pump — base 2/2"
        );
    }

    /// Convergence / no-regression: an ordinary board with NO granted statics
    /// (a directly-carried self-anthem) derives correctly and reaches the
    /// fixpoint without the granted-source machinery changing anything — the
    /// re-gather sees the same effect set the printed gather did and converges
    /// immediately. Same result as a single non-iterating pass would give.
    #[test]
    fn ordinary_board_converges_unchanged() {
        // A creature directly carrying the self-anthem (+2/+2), no grants.
        let (state, id) = creature_on_field(game(), vec![pump_static()]);
        let view = state.layers();
        // `pump_static` is a `Matching(Creature)` +2/+2 on the lone creature →
        // it catches itself → 4/4. The fixpoint must not double-apply it.
        assert_eq!(
            view.power(id),
            Some(4),
            "a directly-carried +2/+2 anthem applies exactly once (base 2 → 4)"
        );
        assert_eq!(view.toughness(id), Some(4), "…toughness 4, applied once");
    }

    /// A same-layer effect can make another conditional effect's predicate
    /// true. The conditional is deliberately authored FIRST: dependency
    /// ordering must see that the later Trample grant changes its active set,
    /// apply that grant first, then apply the conditional Vigilance grant.
    #[test]
    fn same_layer_change_enables_conditional_static() {
        let conditional_vigilance = Ability::r#static(StaticSpec::Conditionally(
            Condition::Matches(
                Reference::Reg(deckmaste_core::RefId(0)),
                Predicate::Characteristic(CharacteristicPredicate::Has("Trample".into())),
            ),
            Arc::new(StaticSpec::Modify(
                Reference::Reg(deckmaste_core::RefId(0)),
                Modification::GainAbility(Arc::new(Ability::Keyword(KeywordAbility::Vigilance))),
            )),
        ));
        let grant_trample = Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::GainAbility(Arc::new(Ability::Keyword(KeywordAbility::Trample))),
        ));
        let (state, id) = creature_on_field(game(), vec![conditional_vigilance, grant_trample]);

        let view = state.layers();
        assert!(
            view.get(id)
                .abilities
                .iter()
                .any(|ability| matches!(ability, Ability::Keyword(KeywordAbility::Trample))),
            "the unconditional layer-6 grant applies"
        );
        assert!(
            view.get(id)
                .abilities
                .iter()
                .any(|ability| matches!(ability, Ability::Keyword(KeywordAbility::Vigilance))),
            "the in-progress Trample grant enables the conditional Vigilance grant"
        );
    }

    /// A granted-grants-granted chain still terminates and applies each link.
    /// Grantor G grants other creatures a static that ITSELF grants other
    /// creatures the self-pump. With two beneficiaries (B, C) the chain is:
    /// iter 1 gathers G's grant → B and C gain the *granting* static; iter 2
    /// gathers that → B and C grant EACH OTHER (and themselves are excluded by
    /// `Not(Ref(This))`) the self-pump; iter 3 gathers the self-pump → B and C
    /// each pump themselves. The fixpoint reaches this in a bounded number of
    /// iterations (well under the cap) without looping forever.
    #[test]
    fn granted_grants_granted_chain_terminates() {
        // The innermost ability each beneficiary eventually pumps with.
        let inner = self_pump_static();
        // A middle static that grants OTHER creatures `inner`.
        let middle = lord_granting_static(inner);
        // G grants OTHER creatures `middle`.
        let (state, _grantor) = creature_on_field(game(), vec![lord_granting_static(middle)]);
        let (state, b) = creature_on_field(state, vec![]);
        let (state, c) = creature_on_field(state, vec![]);

        // The critical property: this terminates (no infinite loop / no
        // debug_assert fire). B and C each end up pumped by the self-pump they
        // were granted by the OTHER beneficiary's middle static.
        let view = state.layers();
        assert_eq!(
            view.power(b),
            Some(3),
            "B is granted (by C's middle static) the self-pump → 3/3"
        );
        assert_eq!(
            view.power(c),
            Some(3),
            "C is granted (by B's middle static) the self-pump → 3/3"
        );
    }
}
