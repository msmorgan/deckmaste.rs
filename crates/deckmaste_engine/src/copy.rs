//! Copy-effect value machinery ([CR#707]): deriving a source's copiable
//! characteristics ([CR#707.2]) and folding a `CopySpec`'s exceptions
//! ([CR#707.9]) into them. Pure value transforms — no card grammar, no token
//! minting.
//!
//! `apply_exceptions` deliberately takes no `&GameState`: it is a plain fold
//! over `CopiableValues`. Two consequences,
//! documented at their call sites below:
//! - A `Modify` whose provided value is a DYNAMIC `Count` (not
//!   `Count::Literal`) can't be evaluated here — it needs game state — and is a
//!   TRUE no-op: neither the value NOR the source's characteristic-defining
//!   ability (see [CR#707.9d] below) is touched, since nothing was actually
//!   provided to replace either with. `apply_pt` shares the exact same
//!   `literal_value().is_some()` condition for both, so the two can't drift
//!   apart: a literal-only value write paired with an unconditional ability
//!   drop would strip a P/T-defining CDA while leaving `power`/`toughness`
//!   orphaned at `StatValue::DefinedByAbility`, which [CR#208.2a] reads as 0 —
//!   worse than doing nothing).
//! - `CardTypes` carries bare `Ident`s and `Subtypes` a name-keyed `SubtypeRef`
//!   (the engine reads `SubtypeRef::name`) that the live layer engine resolves
//!   through `state.types`/`state.subtypes`
//!   (`layer::resolve_type`/`resolve_subtype`) so a granted type/subtype's
//!   `confers` rides along; with no registry reachable here, an added
//!   type/subtype degrades to the SAME name-only shape those functions already
//!   fall back to when a name is absent from the registry (built-in card types
//!   get their structural `TypeDef` via `Type::def()`, which needs no
//!   registry). PARTIAL: a plugin-declared type/subtype's `confers` is lost
//!   this way.
//!
//! The [CR#707.9d] "drop the source's characteristic-defining ability"
//! clause is a PARTIAL implementation, covering the Power/Toughness axes
//! only, via a SHAPE heuristic (`defines_pt`) rather than a formal CDA flag —
//! the engine has none today (`ContinuousEffect::is_cda` is plumbed but never
//! set to `true` anywhere; see `layer.rs`'s `creature_count_cda` test
//! comment: "0 cards use it"). The other seven `Characteristic` axes
//! (`Retain` only — `Modify` has no analogous defining-ability concept for
//! them) are a documented no-op below. Generalizing the drop to those axes is
//! not implemented.

use deckmaste_core::Ability;
use deckmaste_core::Characteristic;
use deckmaste_core::CollectionOp;
use deckmaste_core::CopiableValues;
use deckmaste_core::CopyException;
use deckmaste_core::CopySource;
use deckmaste_core::EnterRider;
use deckmaste_core::Ident;
use deckmaste_core::Int;
use deckmaste_core::Modification;
use deckmaste_core::NumericOp;
use deckmaste_core::Reference;
use deckmaste_core::StatValue;
use deckmaste_core::StaticSpec;
use deckmaste_core::SubtypeRef;
use deckmaste_core::Token;
use deckmaste_core::Type;
use deckmaste_core::TypeDef;

use crate::ExecutionFrame;
use crate::GameState;
use crate::ObjectId;

/// Resolve a `CopySpec`'s source ([CR#707.1]) to a concrete object, or `None`
/// if it no longer exists (a target/self-card that has since left — the
/// caller's copy attempt fizzles on that source, [CR#608.2b]).
///
/// `CopySource::Object` reuses the resolve layer's own `Reference`
/// evaluation (`GameState::eval_reference`, the engine's single-object read —
/// see `resolve/query.rs`, consumed throughout `resolve/player_action.rs`).
/// `CopySource::SelfCard` reads "the card doing the copying... from its own
/// zone" (its doc comment) — exactly `frame.source(self)`, the exophoric binding
/// that the region's source parameter itself reads in a spell frame.
#[must_use]
pub fn resolve_source(
    state: &GameState,
    frame: &ExecutionFrame,
    source: &CopySource,
) -> Option<ObjectId> {
    let id = match source {
        CopySource::Object(reference) => state.eval_reference(reference, frame),
        CopySource::SelfCard => frame.source(state),
    };
    state.objects.get(id).map(|_| id)
}

/// The copiable characteristics ([CR#707.2]) of `source` — `None` if it has
/// none to copy (a player proxy, or a stale/nonexistent id).
///
/// Reads `source`'s printed face, the `base_values` pattern (`layer.rs`):
/// `state.objects.obj(id).card_id()` → `state.cards.get(card).def` →
/// `derive::face`. A minted TOKEN is card-backed too — `TokenCreated`
/// synthesizes its `Token` definition into the card table as a one-faced
/// `Card::Normal(CardFace)` before minting the object (`Cards::push_token`,
/// `step/mod.rs::apply_token_created`; the `ObjectSource` doc comment says so
/// directly: "A created token is `Card`-backed too"), so this ONE path
/// already covers both a real card and a token source — no separate
/// Token-shaped branch exists to write. Already-copied / face-down /
/// as-enters-P/T-modified sources degrade to the printed face for now
/// (`layer.rs:280`'s SEAM comment; downstream
/// (see `layer.rs:280`'s SEAM comment).
#[must_use]
pub fn copiable_values(state: &GameState, source: ObjectId) -> Option<CopiableValues> {
    let obj = state.objects.get(source)?;
    let card = obj.card_id()?;
    let face = crate::derive::face(&state.cards.get(card).def);
    Some(CopiableValues {
        name: face.name.clone(),
        mana_cost: face.mana_cost.clone(),
        color_indicator: face.color_indicator.clone(),
        supertypes: face.supertypes.clone(),
        types: face.types.clone(),
        subtypes: face.subtypes.clone(),
        abilities: face.abilities.clone(),
        power: face.power.clone(),
        toughness: face.toughness.clone(),
        loyalty: face.loyalty.clone(),
        defense: face.defense.clone(),
    })
}

/// Fold a `CopySpec`'s exceptions ([CR#707.9]) into `base` (the source's
/// `copiable_values`), in order. `AdditionalEffect` exceptions are NOT a
/// characteristic change — they contribute nothing here; collect them via
/// [`additional_riders`] instead.
#[must_use]
pub fn apply_exceptions(base: CopiableValues, exceptions: &[CopyException]) -> CopiableValues {
    let mut result = base;
    for exception in exceptions {
        match exception {
            CopyException::Modify(m) => apply_modification(&mut result, m),
            CopyException::Retain(ch) => retain_characteristic(&mut result, *ch),
            CopyException::AdditionalEffect(_) => {}
        }
    }
    result
}

/// The `AdditionalEffect` exceptions ("except it enters with N counters"),
/// for token/copy entry to apply as enter-riders — the ONLY
/// `CopyException` kind `apply_exceptions` does not fold into
/// `CopiableValues` ([CR#707.9e]).
#[must_use]
pub fn additional_riders(exceptions: &[CopyException]) -> Vec<EnterRider> {
    exceptions
        .iter()
        .filter_map(|exception| match exception {
            CopyException::AdditionalEffect(rider) => Some(rider.clone()),
            _ => None,
        })
        .collect()
}

/// Whether an [`EnterRider`] list holds anything the ETB-rider machinery
/// (the seam guarding `Action::Move`/`Action::MoveGroup`/`Action::Create`)
/// still needs built. Two riders are excluded from this check, for
/// different reasons: [`EnterRider::AsCopy`] is a layer-1a copy INPUT
/// ([CR#707.5]) applied by `layer::base_values`, not this function, so it
/// never trips this seam at all; every other rider (`Tapped`,
/// `UnderControlOf`, `UnderOwnersControl`, `Attacking`, `WithCounters`) is
/// now built (`enter_status_from_riders`, below) and folded in at mint by
/// `apply_zone_will_change`/`apply_token_created`. Only
/// [`EnterRider::FaceDown`] remains genuinely unbuilt — it needs the
/// face-down permanent state `engine-face-down` hasn't landed yet. A list
/// mixing `AsCopy`/a built rider with `FaceDown` still trips the `todo!()`
/// for `FaceDown`.
#[must_use]
pub fn has_unbuilt_enter_rider(riders: &[EnterRider]) -> bool {
    riders.iter().any(|r| matches!(r, EnterRider::FaceDown))
}

/// Fold an [`EnterRider`] list into an [`EnterStatus`] — the SAME target
/// struct the enters-replacement self-fold populates
/// (`GameState::as_enters_status`, `replace.rs`), so a rider and a
/// permanent's own `AsEnters` self-replacement converge on one mechanism at
/// `apply_zone_will_change`/`apply_token_created` (union for `tapped`,
/// extend for `counters`, `.or` for `attach_to` — `controller`/`attacking`
/// are rider-only, no self-replacement shape sets them). Callers must
/// already know [`has_unbuilt_enter_rider`] is `false` — `FaceDown` panics
/// here since nothing upstream should ever pass one through.
///
/// `default_controller`/`owner` seed the two controller-affecting riders:
/// `default_controller` is what the entering object's controller would be
/// with NO rider present (the pre-move object's live controller for a
/// `Move`/`MoveGroup`, the creating player for a `Create` token — both
/// equal what `apply_zone_will_change`/`apply_token_created` would otherwise
/// default to) — it seeds `EnterRider::Attacking(None)`'s "its controller"
/// read ([CR#508.4]) so that read reflects a same-list `UnderControlOf`/
/// `UnderOwnersControl` override rather than the stale pre-rider value.
/// `owner` is the entering object's owner, for `UnderOwnersControl`.
///
/// `Attacking`'s defending target is resolved NOW, against the pre-move
/// frame/state: the target (a player proxy or a planeswalker/battle) is
/// unaffected by the entering object's own remint, so resolving it before
/// the move is safe — the same reasoning the enchant-target `attach_to`
/// cast path already relies on (`resolve/mod.rs`). A resolved target that
/// no longer exists by apply time is filtered out by
/// `apply_zone_will_change`/`apply_token_created`, mirroring [CR#508.4a].
#[must_use]
pub(crate) fn enter_status_from_riders(
    state: &GameState,
    frame: &ExecutionFrame,
    riders: &[EnterRider],
    default_controller: crate::player::PlayerId,
    owner: crate::player::PlayerId,
) -> crate::event::EnterStatus {
    let mut status = crate::event::EnterStatus::default();
    for rider in riders {
        match rider {
            EnterRider::Tapped => status.tapped = true,
            // [CR#110.2a]: overrides the mint-time default controller.
            // Silently ignored on a non-player reference (never-crash on
            // invalid semantic input) — `status.controller` stays whatever
            // an earlier rider in the list set, if any.
            EnterRider::UnderControlOf(r) => {
                if let Some(p) = state.eval_player_ref(r, frame) {
                    status.controller = Some(p);
                }
            }
            EnterRider::UnderOwnersControl => status.controller = Some(owner),
            // [CR#122.6a,614.12]: same atomic-at-mint slot the self-fold's
            // `PutCounters(This, ...)` writes — `apply_zone_will_change`
            // extends both lists together, so a card enters with BOTH kinds
            // of counters in the same no-counterless-window batch.
            EnterRider::WithCounters(kind, count) => {
                let n = state.eval_count(count, frame);
                if n > 0 {
                    status.counters.push((kind.0, n));
                }
            }
            // `Attacking` is resolved in the second pass below, once every
            // controller rider in this SAME list has been folded —
            // order-independent. `AsCopy` is a layer-1a copy input, not this
            // seam (see doc comment). Neither writes `status` here.
            EnterRider::Attacking(_) | EnterRider::AsCopy(_) => {}
            EnterRider::FaceDown => {
                unreachable!("callers must gate on has_unbuilt_enter_rider before reaching here")
            }
        }
    }
    if let Some(target_ref) = riders.iter().find_map(|r| match r {
        EnterRider::Attacking(target) => Some(target),
        _ => None,
    }) {
        let effective_controller = status.controller.unwrap_or(default_controller);
        status.attacking = match target_ref {
            Some(r) => Some(state.eval_reference(r, frame)),
            // [CR#508.4]: "its controller chooses which defending player...
            // it's attacking" — in this engine's fixed 2-player field, the
            // sole legal choice is the entering controller's opponent
            // (mirroring `declare_attackers`' own defender computation),
            // so no decision needs surfacing.
            None => Some(
                state
                    .player(state.next_live_after(effective_controller))
                    .object,
            ),
        };
    }
    status
}

/// Map a copy's resolved [`CopiableValues`] to a [`Token`] for minting
/// ([CR#707.1]) — `None` if the token doesn't come
/// into being at all.
///
/// `name` is carried through explicitly — [CR#707.2]: "the copiable values
/// are the values derived from the text printed on the object (that text
/// being name, mana cost, color indicator, ...)" names `name` itself as a
/// copiable characteristic (the Spitting Image example is explicit: a token
/// that's a copy of Doomed Dissenter is named Doomed Dissenter, not "Human
/// Token"). This is UNLIKE a plain `Create(N, Token(...))`/`Named(...)`
/// token, whose effect never specifies a name and so always synthesizes one
/// at [CR#111.4] (subtypes + "Token") — a copy's source, by contrast,
/// always has one, so `Token::name` is set here rather than left `None`. An
/// empty `cv.name` (a faceless/nameless source) still falls back to `None`
/// (synthesis) rather than minting a token literally named "".
///
/// `Token` has no `mana_cost`/`loyalty`/`defense` slot (color rides
/// `color_indicator` per [CR#202.2e]; a token this grammar mints is never a
/// planeswalker/battle) — those
/// `CopiableValues` fields are dropped here, not carried anywhere else.
/// Every other field maps straight across.
///
/// Returns `None` when the copiable values would make the token an instant
/// or sorcery card ([CR#111.5]: "if an effect would create a token that is a
/// copy of an instant or sorcery card, no token is created" — a hard
/// decline, not a stripped-down land-in-limbo value). The broader [CR#111.5]
/// clause ("a rule or effect states that a permanent with one or more of
/// that token's characteristics can't enter the battlefield") has no other
/// concrete check implemented in the engine today — never-crash: an
/// unhandled forbid case simply isn't caught here, matching the rest of
/// this module's documented PARTIAL coverage.
#[must_use]
pub fn token_from_copiable(cv: CopiableValues) -> Option<Token> {
    let is_instant_or_sorcery = cv
        .types
        .iter()
        .any(|t| t.name == Type::Instant.name() || t.name == Type::Sorcery.name());
    if is_instant_or_sorcery {
        return None;
    }
    Some(Token {
        name: (!cv.name.is_empty()).then_some(cv.name),
        color_indicator: cv.color_indicator.into(),
        supertypes: cv.supertypes.into(),
        types: cv.types.into(),
        subtypes: cv.subtypes.into(),
        abilities: cv.abilities.into(),
        power: cv.power,
        toughness: cv.toughness,
    })
}

// ---------------------------------------------------------------------------
// `Modify` folding
// ---------------------------------------------------------------------------

fn apply_modification(result: &mut CopiableValues, m: &Modification) {
    match m {
        Modification::Power(op) => apply_pt(result, PtAxis::Power, op),
        Modification::Toughness(op) => apply_pt(result, PtAxis::Toughness, op),
        // [CR#613.4d] on a copiable snapshot: swap the two fields outright.
        Modification::SwitchPowerToughness => {
            std::mem::swap(&mut result.power, &mut result.toughness);
        }
        // Loyalty/defense have no 613 layer in the live engine (`layer.rs`
        // stubs `BaseLoyalty`/`BaseDefense` for that reason), but
        // `CopiableValues` mirrors `CardFace` directly, so "except it enters
        // with base loyalty N"/defense N is a plain field write — no CDA-drop
        // (loyalty/defense CDAs are outside `defines_pt`'s scope; see the
        // module doc).
        Modification::BaseLoyalty(op) => apply_numeric_field(&mut result.loyalty, op),
        Modification::BaseDefense(op) => apply_numeric_field(&mut result.defense, op),
        Modification::Colors(op) => apply_collection(&mut result.color_indicator, op),
        Modification::Supertypes(op) => apply_collection(&mut result.supertypes, op),
        Modification::CardTypes(op) => apply_type_op(&mut result.types, op),
        Modification::Subtypes(op) => apply_subtype_op(&mut result.subtypes, op),
        Modification::GainAbility(ability) => result.abilities.push((**ability).clone()),
        // Mirrors the live layer-6 `LoseAbility` arm (`layer.rs`).
        Modification::LoseAbility(name) => result
            .abilities
            .retain(|a| !crate::layer::ability_is_named(a, name)),
        Modification::LoseAllAbilities => result.abilities.clear(),
        // `Several` is normally flattened away before the engine ever sees it
        // (`Modification::flatten`, `continuous.rs`), but `apply_exceptions`
        // gets no such guarantee from its caller — recurse rather than
        // assume, so a not-yet-flattened exception still folds correctly
        // instead of silently dropping ops (never-crash).
        Modification::Several(changes) => {
            for change in changes.iter() {
                apply_modification(result, change);
            }
        }
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
        // Not meaningful for a copiable-characteristics SNAPSHOT
        // ([CR#707.2]): `CantHaveAbility` is a standing restriction (no
        // field on `CopiableValues` to carry it), `SetController`/`SetText`
        // are non-characteristic layers 2/3 with no snapshot field either,
        // and `AllCreatureTypes`/`BecomeBasicLandType` are themselves
        // deferred no-op stubs in the LIVE layer engine today (`layer.rs`).
        // Documented no-ops.
        Modification::CantHaveAbility(_)
        | Modification::SetController(_)
        | Modification::SetText(_)
        | Modification::AllCreatureTypes
        | Modification::BecomeBasicLandType(_) => {}
    }
}

/// Which P/T axis a `Modify`/`Retain` targets — the two axes
/// [`defines_pt`]'s CDA-drop heuristic covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PtAxis {
    Power,
    Toughness,
}

fn apply_pt(result: &mut CopiableValues, axis: PtAxis, op: &NumericOp) {
    // [CR#707.9d]: the source's P/T-defining ability is dropped only when
    // this op actually PROVIDES a value for the axis — a `Set` whose `Count`
    // is literal (the ONE case `apply_numeric_field` below also writes a
    // value for). A non-literal `Set` provides nothing (this pure fold has
    // no `&GameState` to evaluate a dynamic `Count` against, see the module
    // doc), so it must drop NOTHING either: stripping the ability while
    // leaving `power`/`toughness` at `StatValue::DefinedByAbility` would
    // orphan the value (no ability left to derive it from — [CR#208.2a]
    // reads that as 0, worse than a true no-op). Sharing this exact
    // condition with `apply_numeric_field`'s own `Set` + `literal_value()`
    // check keeps the two in lockstep instead of drifting apart.
    if let NumericOp::Set(count) = op
        && count.literal_value().is_some()
    {
        result.abilities.retain(|a| !defines_pt(a, axis));
    }
    let field = match axis {
        PtAxis::Power => &mut result.power,
        PtAxis::Toughness => &mut result.toughness,
    };
    apply_numeric_field(field, op);
}

/// Apply a `NumericOp` to a single `Option<StatValue>` field. `Set` writes a
/// literal value outright; a non-literal `Set` (a dynamic `Count` — this pure
/// fold has no `&GameState` to evaluate it against) is a documented no-op,
/// same for `Up`/`Down` against a non-`Number` base or a non-literal delta.
/// `Up`/`Down` are not expected on a copy exception (its exceptions read
/// "except it's ...", not "+N/+N", [CR#707.9]) but are handled anyway —
/// never-crash over never-reached.
fn apply_numeric_field(field: &mut Option<StatValue>, op: &NumericOp) {
    match op {
        NumericOp::Set(value) => {
            // Apply only a `Set` this stateless fold can resolve to a scalar (a
            // printed `Number`, or a literal-count embed); a dynamic count / `X`
            // / CDA marker has no `&GameState` here and stays a no-op.
            if let Some(n) = value.literal_value() {
                *field = Some(StatValue::Number(n));
            }
        }
        NumericOp::Up(count) | NumericOp::Down(count) => {
            if let (Some(StatValue::Number(base)), Some(n)) = (field.clone(), count.literal_value())
            {
                let delta = Int::try_from(n).expect("modifier magnitude fits Int");
                let signed = if matches!(op, NumericOp::Up(_)) { delta } else { -delta };
                *field = Some(StatValue::Number(base + signed));
            }
        }
    }
}

fn apply_collection<T: Clone + PartialEq>(field: &mut Vec<T>, op: &CollectionOp<T>) {
    match op {
        CollectionOp::Set(values) => *field = values.to_vec(),
        CollectionOp::Add(value) => {
            if !field.contains(value) {
                field.push(value.clone());
            }
        }
        CollectionOp::Remove(value) => field.retain(|existing| existing != value),
    }
}

fn apply_type_op(types: &mut Vec<TypeDef>, op: &CollectionOp<Ident>) {
    match op {
        CollectionOp::Set(names) => *types = names.iter().map(minimal_type_def).collect(),
        CollectionOp::Add(name) => {
            let resolved = minimal_type_def(name);
            if !types.iter().any(|t| t.name == resolved.name) {
                types.push(resolved);
            }
        }
        CollectionOp::Remove(name) => types.retain(|t| t.name != *name),
    }
}

/// A registry-less `TypeDef` for a copy exception's added card type. The live
/// layer engine's `resolve_type` (`layer.rs`) looks names up in
/// `state.types` so a granted type's `confers` rides along; no `&GameState`
/// reaches `apply_exceptions` (module doc), so this can't do that lookup. It
/// falls back to the SAME registry-absent shape `resolve_type` itself uses
/// (name-only, `permanent_type: false`, no `confers`) — except for the six
/// built-in card types, which have a registry-free structural `TypeDef`
/// already (`Type::def()`, "fixtures use it so structure-only tests need no
/// plugin load"). A plugin-declared custom type's `confers` is lost either
/// way — flagged, `engine-copy-cda-generalize`.
fn minimal_type_def(name: &Ident) -> TypeDef {
    const BUILTIN: [Type; 10] = [
        Type::Artifact,
        Type::Battle,
        Type::Creature,
        Type::Dungeon,
        Type::Enchantment,
        Type::Instant,
        Type::Kindred,
        Type::Land,
        Type::Planeswalker,
        Type::Sorcery,
    ];
    BUILTIN.into_iter().find(|t| t.name() == *name).map_or(
        TypeDef {
            name: *name,
            permanent_type: false,
            confers: Vec::new().into(),
        },
        Type::def,
    )
}

fn apply_subtype_op(subtypes: &mut Vec<deckmaste_core::Subtype>, op: &CollectionOp<SubtypeRef>) {
    match op {
        CollectionOp::Set(names) => {
            *subtypes = names.iter().map(|n| minimal_subtype(&n.name())).collect();
        }
        CollectionOp::Add(name) => {
            if !subtypes.iter().any(|s| s.name == name.name()) {
                subtypes.push(minimal_subtype(&name.name()));
            }
        }
        CollectionOp::Remove(name) => subtypes.retain(|s| s.name != name.name()),
    }
}

/// A registry-less `Subtype` for a copy exception's added subtype — see
/// [`minimal_type_def`]'s doc. Unlike card types, NO subtype has a built-in
/// structural shape (`types`/`confers` are entirely plugin data, e.g. a
/// basic land type's mana ability, [CR#305.6]), so this always degrades to a
/// name-only `Subtype`. Flagged, `engine-copy-cda-generalize`.
fn minimal_subtype(name: &Ident) -> deckmaste_core::Subtype {
    deckmaste_core::Subtype {
        name: *name,
        types: Vec::new().into(),
        confers: Vec::new().into(),
    }
}

/// Whether `ability` is the P/T-DEFINING characteristic-defining ability
/// ([CR#604.3]) for `axis` — dropped by a copy exception that PROVIDES that
/// axis's value ([CR#707.9d]), so the copy doesn't ALSO carry an ability
/// that re-derives a value it isn't using.
///
/// HEURISTIC, not a formal signal: the engine has no working CDA flag today
/// — `ContinuousEffect::is_cda` is plumbed through the layer pipeline but
/// never set to `true` anywhere (`layer.rs`'s `creature_count_cda` test:
/// "The 7a/CDA layer distinction is deferred (0 cards use it...)"). This
/// instead recognizes the SHAPE a P/T CDA takes on a card today — a
/// `This`-scoped static `Modify` whose op (directly, or bundled in a
/// `Several`, the Tarmogoyf pattern: `Modify(This, Several([Power(Set(...)),
/// Toughness(Set(...))]))`) `Set`s the axis. It does NOT look inside
/// `Conditionally` or `Each`: neither shape is a P/T-defining CDA. Covers only
/// Power/Toughness; see `retain_characteristic` for the other axes.
/// Follow-up: `engine-copy-cda-generalize`.
fn defines_pt(ability: &Ability, axis: PtAxis) -> bool {
    match ability {
        Ability::Static(effect) => static_defines_pt(effect, axis),
        Ability::Activated(_) | Ability::Triggered(_) | Ability::Spell(_) | Ability::Keyword(_) => {
            false
        }
    }
}

fn static_defines_pt(region: &deckmaste_core::Region<StaticSpec>, axis: PtAxis) -> bool {
    match &region.body {
        StaticSpec::Modify(Reference::Reg(reference), modification)
            if region.provenance_of(*reference) == Some(&deckmaste_core::Provenance::Source) =>
        {
            modification_defines_pt(modification, axis)
        }
        _ => false,
    }
}

fn modification_defines_pt(m: &Modification, axis: PtAxis) -> bool {
    match m {
        Modification::Power(NumericOp::Set(_)) => axis == PtAxis::Power,
        Modification::Toughness(NumericOp::Set(_)) => axis == PtAxis::Toughness,
        Modification::Several(list) => list.iter().any(|m| modification_defines_pt(m, axis)),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// `Retain` folding
// ---------------------------------------------------------------------------

/// "Except it doesn't copy its [ch]" ([CR#707.9c,707.9d]). The VALUE swap —
/// substituting the copy's own natural value for `base`'s copied one — is
/// the entering/existing object's own data, which this pure fold never sees
/// (Task 3's job, at token-entry/copy-application time, the same way
/// `AdditionalEffect` rides through `additional_riders` instead of here); a
/// `Retain` therefore leaves `base`'s value untouched and only strips the
/// source's characteristic-DEFINING ability for the axis, so a dropped
/// characteristic doesn't drag along an ability that would re-derive it.
fn retain_characteristic(result: &mut CopiableValues, ch: Characteristic) {
    match ch {
        Characteristic::Power => result.abilities.retain(|a| !defines_pt(a, PtAxis::Power)),
        Characteristic::Toughness => result
            .abilities
            .retain(|a| !defines_pt(a, PtAxis::Toughness)),
        // PARTIAL, matching `Modify`'s CDA-drop above (`defines_pt`'s doc):
        // the defining-ability drop for every other axis is a documented
        // no-op — no shape/flag to find a
        // name/type/subtype/supertype/color/mana-cost/loyalty/defense-defining
        // ability by. Follow-up: `engine-copy-cda-generalize`.
        Characteristic::Colors
        | Characteristic::Types
        | Characteristic::Subtypes
        | Characteristic::BasicLandTypes
        | Characteristic::Supertypes
        | Characteristic::Defense
        | Characteristic::ManaCost
        | Characteristic::Name => {}
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_card::CardFace;
    use deckmaste_core::CollectionOp;
    use deckmaste_core::CopiableValues;
    use deckmaste_core::CopyException;
    use deckmaste_core::CopySource;
    use deckmaste_core::Count;
    use deckmaste_core::EnterRider;
    use deckmaste_core::Modification;
    use deckmaste_core::NumericOp;
    use deckmaste_core::Reference;
    use deckmaste_core::StatValue;
    use deckmaste_core::Token;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use super::*;
    use crate::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn base_bear() -> CopiableValues {
        CopiableValues {
            name: "Bear".into(),
            power: Some(StatValue::Number(2)),
            toughness: Some(StatValue::Number(2)),
            ..CopiableValues::default()
        }
    }

    #[test]
    fn modify_set_pt_overrides() {
        let out = apply_exceptions(
            base_bear(),
            &[CopyException::Modify(Modification::Power(NumericOp::Set(
                StatValue::Number(7),
            )))],
        );
        assert_eq!(
            out.power,
            Some(StatValue::Number(7)),
            "Modify(Power Set 7) overrides copied power [CR#707.9d]"
        );
    }

    #[test]
    fn additional_effect_leaves_characteristics_untouched() {
        let before = base_bear();
        let out = apply_exceptions(
            before.clone(),
            &[CopyException::AdditionalEffect(EnterRider::WithCounters(
                "P1P1Counter".into(),
                Count::Literal(1),
            ))],
        );
        assert_eq!(
            out, before,
            "AdditionalEffect is not a characteristic change [CR#707.9e]"
        );
    }

    /// A Tarmogoyf-shaped P/T CDA (`Modify(This, Several([Power(Set(...)),
    /// Toughness(Set(...))]))`) is dropped when a `Modify` exception
    /// overrides power — the CDA would otherwise ride along on the copy and
    /// try to re-derive a value it no longer supplies [CR#707.9d].
    #[test]
    fn modify_set_power_drops_pt_defining_cda() {
        let cda =
            Ability::r#static(StaticSpec::Modify(
                Reference::Reg(deckmaste_core::RefId(0)),
                Modification::Several(
                    vec![
                        Modification::Power(NumericOp::Set(StatValue::Count(Count::CountOf(
                            deckmaste_core::Countable::Objects(Arc::new(
                                deckmaste_core::Region::candidate(
                                    deckmaste_core::Predicate::creature(),
                                ),
                            )),
                        )))),
                        Modification::Toughness(NumericOp::Set(StatValue::Count(Count::CountOf(
                            deckmaste_core::Countable::Objects(Arc::new(
                                deckmaste_core::Region::candidate(
                                    deckmaste_core::Predicate::creature(),
                                ),
                            )),
                        )))),
                    ]
                    .into(),
                ),
            ));
        let base = CopiableValues {
            name: "Tarmogoyf".into(),
            power: Some(StatValue::DefinedByAbility),
            toughness: Some(StatValue::DefinedByAbility),
            abilities: vec![cda],
            ..CopiableValues::default()
        };
        let out = apply_exceptions(
            base,
            &[CopyException::Modify(Modification::Power(NumericOp::Set(
                StatValue::Number(3),
            )))],
        );
        assert_eq!(out.power, Some(StatValue::Number(3)), "power overridden");
        assert!(
            out.abilities.is_empty(),
            "the P/T-defining CDA is dropped, not copied alongside the override"
        );
    }

    /// Regression for a review fix: a `Modify(Power(Set(dynamic Count)))`
    /// can't actually provide a value here (no `&GameState` to evaluate the
    /// `Count` against), so it must be a TRUE no-op — neither the value NOR
    /// the P/T-defining ability moves. Before the fix, the ability drop ran
    /// unconditionally on any `Set` while the value write stayed
    /// literal-only, so this exact case stripped the CDA but left `power`
    /// stuck at `DefinedByAbility` with nothing left to derive it —
    /// [CR#208.2a] reads that as 0, worse than doing nothing.
    #[test]
    fn modify_set_power_non_literal_count_is_true_noop() {
        let cda = Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Power(NumericOp::Set(StatValue::Number(0))),
        ));
        let base = CopiableValues {
            power: Some(StatValue::DefinedByAbility),
            abilities: vec![cda.clone()],
            ..CopiableValues::default()
        };
        let dynamic = Count::CountOf(deckmaste_core::Countable::Objects(Arc::new(
            deckmaste_core::Region::candidate(deckmaste_core::Predicate::creature()),
        )));
        let out = apply_exceptions(
            base,
            &[CopyException::Modify(Modification::Power(NumericOp::Set(
                StatValue::Count(dynamic),
            )))],
        );
        assert_eq!(
            out.power,
            Some(StatValue::DefinedByAbility),
            "a non-literal Set can't provide a value — the copied value rides through unchanged"
        );
        assert_eq!(
            out.abilities,
            vec![cda],
            "and the P/T-defining ability is NOT dropped, since nothing replaced its value [CR#707.9d]"
        );
    }

    /// `Retain(Power)` leaves the copied power VALUE untouched (Task 3
    /// substitutes the copy's own value) but still drops the source's
    /// P/T-defining ability so it doesn't ride along unused.
    #[test]
    fn retain_power_keeps_value_drops_defining_ability() {
        let cda = Ability::r#static(StaticSpec::Modify(
            Reference::Reg(deckmaste_core::RefId(0)),
            Modification::Power(NumericOp::Set(StatValue::Number(0))),
        ));
        let base = CopiableValues {
            power: Some(StatValue::DefinedByAbility),
            abilities: vec![cda],
            ..CopiableValues::default()
        };
        let out = apply_exceptions(
            base,
            &[CopyException::Retain(deckmaste_core::Characteristic::Power)],
        );
        assert_eq!(
            out.power,
            Some(StatValue::DefinedByAbility),
            "Retain doesn't touch the copied value itself"
        );
        assert!(
            out.abilities.is_empty(),
            "Retain still drops the source's defining ability [CR#707.9d]"
        );
    }

    #[test]
    fn card_types_add_builtin_gets_structural_typedef() {
        let out = apply_exceptions(
            CopiableValues::default(),
            &[CopyException::Modify(Modification::CardTypes(
                CollectionOp::Add("Artifact".into()),
            ))],
        );
        assert_eq!(
            out.types,
            vec![Type::Artifact.def()],
            "a built-in type name resolves to its structural TypeDef with no registry"
        );
    }

    #[test]
    fn power_up_applies_to_numeric_base() {
        let out = apply_exceptions(
            base_bear(),
            &[CopyException::Modify(Modification::Power(NumericOp::Up(
                Count::Literal(1),
            )))],
        );
        // 2 -> 3, toughness untouched at 2.
        assert_eq!(out.power, Some(StatValue::Number(3)));
        assert_eq!(out.toughness, Some(StatValue::Number(2)));
    }

    fn bare_game() -> GameState {
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

    fn mint_card(state: &mut GameState, face: CardFace) -> ObjectId {
        let card = state.cards.push(Arc::new(Card::Normal(face)), PlayerId(0));
        state.objects.mint(
            ObjectSource::Card(card),
            PlayerId(0),
            Some(Zone::Battlefield),
        )
    }

    fn mint_token(state: &mut GameState, token: &Token) -> ObjectId {
        let card = state.cards.push_token(token, PlayerId(0));
        state.objects.mint(
            ObjectSource::Card(card),
            PlayerId(0),
            Some(Zone::Battlefield),
        )
    }

    #[test]
    fn copiable_values_reads_the_printed_face() {
        let mut state = bare_game();
        let id = mint_card(
            &mut state,
            CardFace {
                name: "Grizzly Bears".into(),
                types: vec![Type::Creature.def()],
                power: Some(StatValue::Number(2)),
                toughness: Some(StatValue::Number(2)),
                ..CardFace::default()
            },
        );
        let values = copiable_values(&state, id).expect("card-backed object has copiable values");
        assert_eq!(&*values.name, "Grizzly Bears");
        assert_eq!(values.power, Some(StatValue::Number(2)));
        assert_eq!(values.toughness, Some(StatValue::Number(2)));
    }

    /// The token-source branch investigated in the task brief: a minted
    /// token is `Card`-backed (`Cards::push_token` synthesizes a
    /// `Card::Normal(CardFace)`), so `copiable_values` reads it through the
    /// SAME path as a real card — no separate Token variant to branch on.
    #[test]
    fn copiable_values_reads_a_minted_token() {
        let mut state = bare_game();
        let token = Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Creature.def()].into(),
            subtypes: vec![deckmaste_core::Subtype {
                name: "Bear".into(),
                types: vec![Type::Creature].into(),
                confers: vec![].into(),
            }]
            .into(),
            abilities: vec![].into(),
            power: Some(StatValue::Number(3)),
            toughness: Some(StatValue::Number(3)),
        };
        let id = mint_token(&mut state, &token);
        let values = copiable_values(&state, id).expect("token-backed object has copiable values");
        assert_eq!(
            &*values.name, "Bear Token",
            "a token's synthesized name is subtypes + \"Token\" [CR#111.4]"
        );
        assert_eq!(values.power, Some(StatValue::Number(3)));
        assert_eq!(values.toughness, Some(StatValue::Number(3)));
    }

    #[test]
    fn resolve_source_self_card_is_the_frame_source() {
        let mut state = bare_game();
        let id = mint_card(&mut state, CardFace::default());
        let frame = state.frame(id, PlayerId(0));
        assert_eq!(
            resolve_source(&state, &frame, &CopySource::SelfCard),
            Some(id)
        );
    }

    #[test]
    fn resolve_source_nonexistent_object_is_none() {
        let state = bare_game();
        let dead = ObjectId::from_raw(999);
        let frame = state.frame(dead, PlayerId(0));
        assert_eq!(resolve_source(&state, &frame, &CopySource::SelfCard), None);
    }

    /// `CopySource::Object(reference)` — the "a copy of target creature"
    /// shape (Clone, Populate) — resolves a LIVE announced target to its
    /// `ObjectId` through the resolve layer's own `eval_reference`.
    #[test]
    fn resolve_source_object_reference_resolves_live_target() {
        let mut state = bare_game();
        let source = mint_card(&mut state, CardFace::default());
        let target = mint_card(&mut state, CardFace::default());
        let frame = crate::test_support::frame_src_targets(&state, source, vec![target]);
        assert_eq!(
            resolve_source(
                &state,
                &frame,
                &CopySource::Object(Reference::Reg(deckmaste_core::RefId(6)))
            ),
            Some(target),
            "CopySource::Object resolves a live announced target to its id"
        );
    }

    /// An unresolvable `CopySource::Object` reference — its announced target
    /// has since left play — is `None`, never a panic. `eval_reference`'s
    /// `Reference::Target` turns a departed slot member into the null id
    /// ([CR#608.2b] partial fizzle) rather than panicking; `resolve_source`'s
    /// own liveness check then turns that into `None`.
    #[test]
    fn resolve_source_object_reference_departed_target_is_none_not_panic() {
        let mut state = bare_game();
        let source = mint_card(&mut state, CardFace::default());
        let dead = ObjectId::from_raw(999);
        let frame = crate::test_support::frame_src_targets(&state, source, vec![dead]);
        assert_eq!(
            resolve_source(
                &state,
                &frame,
                &CopySource::Object(Reference::Reg(deckmaste_core::RefId(6)))
            ),
            None,
            "a departed target resolves to None, never a panic"
        );
    }

    /// `has_unbuilt_enter_rider` is the seam every rider-consuming
    /// `todo!()` (`resolve/action.rs`, `resolve/player_action.rs`) gates on:
    /// a rider list holding ONLY `AsCopy` entries — any count — never trips
    /// it (the layer-1a copy input fizzles here, applied downstream by
    /// `engine-layers-1-copy-facedown-text`); a genuinely BUILT rider
    /// (`Tapped`) no longer trips it either
    /// (`engine-enter-rider-execution`); an empty list never trips it (the
    /// existing no-op case); but `FaceDown` — the one rider still awaiting
    /// `engine-face-down`'s state machinery — always does, whether alone or
    /// mixed with `AsCopy`/a built rider.
    #[test]
    fn has_unbuilt_enter_rider_excludes_as_copy_and_built_riders() {
        let spec = deckmaste_core::CopySpec {
            source: CopySource::Object(Reference::Reg(deckmaste_core::RefId(6))),
            exceptions: vec![],
        };

        assert!(
            !has_unbuilt_enter_rider(&[]),
            "an empty rider list never trips the seam"
        );
        assert!(
            !has_unbuilt_enter_rider(&[EnterRider::AsCopy(spec.clone())]),
            "a lone AsCopy rider fizzles rather than tripping the ETB-rider seam"
        );
        assert!(
            !has_unbuilt_enter_rider(&[
                EnterRider::AsCopy(spec.clone()),
                EnterRider::AsCopy(spec.clone()),
            ]),
            "multiple AsCopy riders still fizzle"
        );
        assert!(
            !has_unbuilt_enter_rider(&[EnterRider::Tapped]),
            "a built rider (Tapped) no longer trips the seam"
        );
        assert!(
            has_unbuilt_enter_rider(&[EnterRider::FaceDown]),
            "the still-unbuilt FaceDown rider trips the seam"
        );
        assert!(
            has_unbuilt_enter_rider(&[EnterRider::AsCopy(spec), EnterRider::FaceDown]),
            "AsCopy mixed with the unbuilt rider still trips the seam for FaceDown"
        );
    }
}
