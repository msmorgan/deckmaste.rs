//! Legal actions for the priority holder ([CR#117.1]). The list this computes
//! is both the advisory `legal` carried by the Priority decision and the
//! authoritative check at submission (state can't change in between: a
//! pending decision blocks stepping).

use std::ops::ControlFlow;

use deckmaste_core::Ability;
use deckmaste_core::DeedAgent;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::KeywordAbility;
use deckmaste_core::Predicate;
use deckmaste_core::StaticEffect;
use deckmaste_core::Type;

use crate::decide::Action;
use crate::derive;
use crate::layer::LayeredView;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;

/// The proposed-action pattern inside a deontic row, looking through the
/// `Expanded` provenance wrappers.
fn deontic_action(d: &Deontic) -> &DeonticAction {
    match d {
        Deontic::May(a) | Deontic::Cant(a) | Deontic::Must(a) | Deontic::Gate(a, _) => a,
        Deontic::Expanded(e) => deontic_action(&e.value),
    }
}

/// Whether `id`'s derived view carries any static matching `pred`, looking
/// through composites and macro `Expanded` wrappers at every level. A
/// short-circuiting wrapper over the single [`statics_on`] walker: it stops
/// the descent the moment `pred` accepts.
pub(crate) fn object_has_static<F: Fn(&StaticEffect) -> bool>(
    view: &LayeredView,
    id: ObjectId,
    pred: &F,
) -> bool {
    // A presence scan: look THROUGH `Conditionally` unconditionally (pass
    // `|_| true`), so this stays a pure characteristic-free read of the derived
    // view and never re-enters `layers()` via a condition.
    statics_on(
        view,
        id,
        &mut |_: &deckmaste_core::Condition| true,
        &mut |e| {
            if pred(e) { ControlFlow::Break(()) } else { ControlFlow::Continue(()) }
        },
    )
    .is_break()
}

pub(crate) fn statics_present<F: Fn(&StaticEffect) -> bool>(
    state: &GameState,
    view: &LayeredView,
    pred: F,
) -> bool {
    state
        .zones
        .battlefield
        .iter()
        .any(|&id| object_has_static(view, id, &pred))
}

/// P0.W1 presence guard ([CR#101.2,601.3] seam): the deontic grammar is
/// complete, but declaration legality does not evaluate the rows yet. Any
/// matching-verb row in the derived view trips the seam LOUDLY rather than
/// being silently ignored. Never delete a trip to silence it — convert it
/// to the legality evaluation.
fn guard_deontic_seam(
    state: &GameState,
    view: &LayeredView,
    row: fn(&Deontic) -> bool,
    what: &str,
) {
    let hit = statics_present(
        state,
        view,
        |e| matches!(e, StaticEffect::Deontic(d) if row(d)),
    );
    if hit {
        todo!("P0.W1: deontic {what} legality — rows present in the derived view go unevaluated");
    }
}

/// Whether a deontic row's polarity is `Cant`, through `Expanded` wrappers.
fn is_cant(d: &Deontic) -> bool {
    match d {
        Deontic::Cant(_) => true,
        Deontic::Expanded(e) => is_cant(&e.value),
        _ => false,
    }
}

/// Whether a deontic row's polarity is `May`, through `Expanded` wrappers.
fn is_may(d: &Deontic) -> bool {
    match d {
        Deontic::May(_) => true,
        Deontic::Expanded(e) => is_may(&e.value),
        _ => false,
    }
}

/// Whether a deontic row's polarity is `Must`, through `Expanded` wrappers.
fn is_must(d: &Deontic) -> bool {
    match d {
        Deontic::Must(_) => true,
        Deontic::Expanded(e) => is_must(&e.value),
        _ => false,
    }
}

/// The action under a `Cant` polarity, through `Expanded` wrappers.
fn cant_action(d: &Deontic) -> Option<&DeonticAction> {
    match d {
        Deontic::Cant(a) => Some(a),
        Deontic::Expanded(e) => cant_action(&e.value),
        _ => None,
    }
}

/// The action under a `Must` polarity, through `Expanded` wrappers.
fn must_action(d: &Deontic) -> Option<&DeonticAction> {
    match d {
        Deontic::Must(a) => Some(a),
        Deontic::Expanded(e) => must_action(&e.value),
        _ => None,
    }
}

/// The action under a `May` polarity, through `Expanded` wrappers.
fn may_action(d: &Deontic) -> Option<&DeonticAction> {
    match d {
        Deontic::May(a) => Some(a),
        Deontic::Expanded(e) => may_action(&e.value),
        _ => None,
    }
}

#[must_use]
pub fn legal_actions(state: &GameState, player: PlayerId) -> Vec<Action> {
    // One derived view serves the whole window — the mana-ability and cast
    // checks below read it per object instead of re-deriving the board.
    let view = state.layers();
    // [CR#104.3a] "at any time": a correct steppable engine ENUMERATES
    // concession at every boundary that emits choices — "you can also
    // concede". A runner that would rather not show it (or a bot that
    // must not pick it) filters; that is the runner's problem, not the
    // legality computation's.
    let mut legal = vec![Action::Pass, Action::Concede];

    // [CR#116.2a,305.2]: a land from hand — sorcery timing (own turn, main
    // phase, empty stack), up to the effective land plays per turn (one by
    // default; Exploration and other continuous statics raise it).
    if state.sorcery_speed_ok(player)
        && state.lands_played_this_turn(player) < state.effective_land_plays_per_turn(player)
    {
        for &object in &state.zones.hands[player.index()] {
            // Derived capability ([CR#613.1d,116.2a,701.18]): a card whose
            // layered view confers `May(Play)` — its Land type's default-deny
            // land-play marker — is playable as a land, keyed on the capability
            // rather than a `Type::Land` literal (per-face correct for MDFC
            // land//spell).
            if confers_may_play(state, &view, object) {
                legal.push(Action::PlayLand { object });
            }
        }
    }

    // Activated abilities of permanents you control: mana abilities are
    // stackless ([CR#605.3a]) and skip the full gate, but their {T} is still
    // physical — a tapped object can't pay it, and [CR#602.5a] blocks a
    // summoning-sick creature's {T} even for mana (haste = the kw-haste
    // seam); the rest run the full [CR#602.5] gate ([CR#602.2]: only the
    // controller activates).
    for &object in &state.zones.battlefield {
        let obj = state.objects.obj(object);
        // Derived controller ([CR#613.1b]): only the current controller may
        // activate ([CR#602.2]); a control-change effect moves this.
        if view.controller(object) != player {
            continue;
        }
        // [CR#602.5a]: a `{T}` mana ability is offered only when the object's
        // conferred `Cant(Activate(cost: IncludesTapSymbol))` rows permit it —
        // the summoning-sickness tap gate a `Creature` type confers (haste-
        // exempt). Keyed on the capability, not a `Type::Creature` literal.
        let tap_forbidden = cant_activate(state, &view, object, player, true);
        // Index the SAME Innate-PEELED list resolution reads ([CR#113.12]):
        // `begin_activate`, `decide`'s `ActivateAbility` arm, and `render`'s
        // `activated_ability`/`mana_ability` all index
        // `derive::usable_abilities` (see the "SAME list, SAME order"
        // invariant in `render.rs`). Peeling — not filtering — keeps a
        // conferred `Innate(Activated)` (a basic land's [CR#305.6] mana
        // ability) activatable by its controller while the indices stay
        // aligned.
        for (ability, a) in derive::usable_abilities(state, object).iter().enumerate() {
            // `tap_mana_ability` is the authoritative classifier here: its
            // subset scope (cost=[Tap], specific mana, no targets) defines
            // which abilities take the stackless path ([CR#605.3b]); widen it
            // and this routing together.
            if derive::tap_mana_ability(a).is_some() {
                if !obj.tapped && !tap_forbidden {
                    legal.push(Action::ActivateAbility { object, ability });
                }
            } else if let Some(act) = crate::activate::as_activated(a)
                && state.can_activate(&view, player, object, ability, act)
            {
                legal.push(Action::ActivateAbility { object, ability });
            }
        }
    }

    // [CR#601.3]: cast a spell from hand if timing + payment + targets permit.
    // Target/Attach rows (hexproof, protection, enchant) ride the same
    // guard: targeting legality and attach legality don't evaluate
    // deontics yet, and a board carrying such rows must trip LOUDLY at
    // the priority window rather than silently allow the choice.
    // Cant(Target) rows (hexproof, protection's targeted clause) are
    // EVALUATED at target-candidate computation, Must(Target) requirements
    // (the Flagbearer class) at target-choice submission, the flash
    // shape — May(Cast(window: InstantSpeed)) with no from/cost slot — is
    // EVALUATED as a timing lift in can_cast ([CR#702.8a]), and the land-play
    // marker — May(Play) with no `from` slot, which a card's Land type
    // confers — is EVALUATED as land-play legality via `confers_may_play`
    // (offered as PlayLand in the hand-scan above, [CR#116.2a,305.9,701.18]).
    // The guard keeps the unevaluated rest: every other Cast row shape (zone
    // permissions, alternative costs, non-May polarities), the from-zone /
    // non-May Play shapes, non-Cant/non-May Attach rows, and the May/Gate
    // Target polarities.
    guard_deontic_seam(
        state,
        &view,
        |d| match deontic_action(d) {
            DeonticAction::Cast {
                from, window, cost, ..
            } => {
                !(is_may(d)
                    && *window == Some(deckmaste_core::Timing::InstantSpeed)
                    && from.is_none()
                    && cost.is_none())
            }
            // `May(Play(from: None))` — the land-play marker a card's Land
            // type CONFERS — is EVALUATED as land-play legality (offered as
            // `Action::PlayLand` in the hand-scan above via `confers_may_play`,
            // [CR#116.2a,305.9,701.18]); mirrors the `Cast` arm's flash
            // exclusion. The remaining Play shapes (cast-from-zone `from:
            // Some`, and the non-May polarities) stay a loud seam.
            DeonticAction::Play { from, .. } => !(is_may(d) && from.is_none()),
            // `Cant(Attach)` and `May(Attach)` are both EVALUATED via
            // `attachment_legal` (default-deny: the `May` grant permits, the
            // `Cant` subtracts) at the [CR#701.3b] no-op + the
            // [CR#704.5m..704.5p] SBA sweep; only the remaining attach
            // polarities (Must/Gate — no card needs them yet) stay a loud seam.
            DeonticAction::Attach { .. } => !is_cant(d) && !is_may(d),
            DeonticAction::Target { .. } => !is_cant(d) && !is_must(d),
            _ => false,
        },
        "cast + from-zone/non-May play + non-Cant attach + May/Gate target",
    );
    // The former P0.W2 `CostModifier` presence guard converted to the real
    // [CR#601.2f] pipeline: `GameState::mana_cost` applies the rows (see
    // `cast::modified_mana_cost`), so `can_cast` below already gates on the
    // modified total.
    for &object in &state.zones.hands[player.index()] {
        if state.can_cast(&view, player, object) {
            legal.push(Action::CastSpell { object });
        }
    }

    legal
}

/// [CR#508.1a]: the permanents `player` could declare as attackers —
/// battlefield permanents they control that are untapped and carry the
/// `May(Attack)` grant (their `Creature` type's default-deny combat capability,
/// read from the derived layer view so animated creatures are included), minus
/// any matching `Cant(Attack)` row. Summoning sickness is itself a conferred
/// `Cant(Attack)` ([CR#302.6,702.10b] — gated on `SummoningSick && !Haste`), so
/// it is subtracted here, not checked literally. Must(Attack) requirements
/// ([CR#508.1d]) are a declaration-time seam.
#[must_use]
pub fn legal_attackers(state: &GameState, player: PlayerId) -> Vec<ObjectId> {
    let view = state.layers();
    // May(Attack) grants are EVALUATED (via `attackable`), Cant(Attack) rows
    // (defender, [CR#702.3b]; "can't attack" and the sickness confer) are
    // EVALUATED below, and Must(Attack) requirements ("attacks if able", goad)
    // are EVALUATED at declaration submission ([CR#508.1d]); the guard narrows
    // to the Gate(Attack) polarity (a toll on attacking), which nothing
    // evaluates yet.
    guard_deontic_seam(
        state,
        &view,
        |d| {
            !is_cant(d)
                && !is_must(d)
                && !is_may(d)
                && matches!(deontic_action(d), DeonticAction::Attack { .. })
        },
        "attack (Gate polarity)",
    );
    let rows = cant_attack_rows(state, &view);
    // [CR#508.1a]: in the two-player game the attacked player is the
    // defender — the non-active player's proxy carries the `on` slot.
    let defender_proxy = state
        .players
        .iter()
        .find(|p| p.id != player)
        .map(|p| p.object);
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&id| {
            let obj = state.objects.obj(id);
            // Derived controller ([CR#613.1b]): a stolen creature attacks for
            // its new controller, not its owner. `attackable` reads the
            // `May(Attack)` grant (default-deny combat capability); summoning
            // sickness is now a conferred `Cant(Attack)` in `rows` below, not a
            // literal `!summoning_sick` check ([CR#302.6,508.1a]).
            view.controller(id) == player
                && !obj.tapped
                && attackable(state, &view, id)
                && !rows.iter().any(|(carrier, by, on)| {
                    state.filter_matches_live(by, id, *carrier)
                        && defender_proxy
                            .is_some_and(|d| state.filter_matches_live(on, d, *carrier))
                })
        })
        .collect()
}

/// [CR#506.3,508.1b]: what the active player may attack in the two-player
/// game — the sole `defender`'s player-proxy object plus every planeswalker
/// (`Type::Planeswalker`) that `defender` controls ([CR#508.1b]). Battles and
/// multi-defender games are separate tickets. Planeswalker type is read from
/// the derived layer view so animated/type-changed permanents are honored.
#[must_use]
pub fn legal_attack_targets(state: &GameState, defender: PlayerId) -> Vec<ObjectId> {
    let view = state.layers();
    let mut targets = vec![state.player(defender).object];
    targets.extend(state.zones.battlefield.iter().copied().filter(|&id| {
        view.controller(id) == defender && view.get(id).has_type(Type::Planeswalker)
    }));
    targets
}

/// Every `Attack` row of the polarity `pick` extracts in the derived view,
/// with its carrier — point-wise by construction (`Attack{by, on}` carries
/// no arrangement bound).
fn attack_rows(
    state: &GameState,
    view: &LayeredView,
    pick: fn(&Deontic) -> Option<&DeonticAction>,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Attack { by, on }) = pick(d)
            {
                rows.push((source, by.clone(), on.clone()));
            }
        });
    }
    rows
}

/// Every `Cant(Attack)` row in the derived view ([CR#702.3b] defender,
/// "can't attack" effects).
#[must_use]
fn cant_attack_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    attack_rows(state, view, cant_action)
}

/// Every `May(Attack)` GRANT in the derived view — the permission side of
/// attack eligibility under default-deny ([CR#508.1a]): declaring a permanent
/// as an attacker is a capability nothing has by default; a card's `Creature`
/// type CONFERS `May(Attack(by: Ref(This)))`. Mirror of `cant_attack_rows` on
/// the `May` polarity.
#[must_use]
fn may_attack_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    attack_rows(state, view, may_action)
}

/// [CR#508.1a]: whether `id` is an attacker CANDIDATE under default-deny — some
/// conferred `May(Attack)` grant names it as an attacker (its `Creature` type's
/// grant, keyed on `by`). This is grant PRESENCE, not net eligibility: the
/// tapped / summoning-sick / `Cant(Attack)` subtractions are applied by
/// `legal_attackers`. `is_combatant` is the identically-computed combat-damage
/// twin ([CR#120.3e]).
#[must_use]
pub(crate) fn attackable(state: &GameState, view: &LayeredView, id: ObjectId) -> bool {
    may_attack_rows(state, view)
        .iter()
        .any(|(carrier, by, _on)| state.filter_matches_live(by, id, *carrier))
}

/// [CR#120.3e,120.3c]: whether `id` is a COMBATANT — it carries the
/// `May(Attack)` grant (its `Creature` type confers it). Combat damage is
/// MARKED on a combatant ([CR#120.3e]); a non-combatant permanent is not
/// marked (a planeswalker instead loses loyalty, [CR#120.3c]). This is grant
/// PRESENCE, not net attack eligibility: a
/// creature forbidden to attack (a `Cant(Attack)` row) is still a combatant
/// whose damage is marked. Same read as [`attackable`], under the
/// damage-marking rule rather than the declaration rule.
#[must_use]
pub(crate) fn is_combatant(state: &GameState, view: &LayeredView, id: ObjectId) -> bool {
    attackable(state, view, id)
}

/// Every `Must(Attack)` row in the derived view — attack requirements
/// ([CR#508.1d]: "attacks if able" effects, goad).
#[must_use]
pub(crate) fn must_attack_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    attack_rows(state, view, must_action)
}

/// [CR#509.1a]: the permanents `player` could declare as blockers —
/// battlefield permanents they control that are untapped and carry the
/// `May(Block)` grant (their `Creature` type's default-deny combat capability,
/// read from the derived layer view so animated creatures can block). No
/// summoning-sickness check: a summoning-sick creature can block.
#[must_use]
pub fn legal_blockers(state: &GameState, player: PlayerId) -> Vec<ObjectId> {
    let view = state.layers();
    // May(Block) grants are EVALUATED (via `blockable`), Cant(Block) rows —
    // point-wise (flying) AND arrangement-level (menace's `count`) — and
    // Must(Block) requirements ([CR#509.1c] must-block) are EVALUATED at block
    // submission now; the guard narrows to the Gate(Block) polarity (a toll on
    // blocking), which nothing evaluates yet.
    guard_deontic_seam(
        state,
        &view,
        |d| {
            !is_cant(d)
                && !is_must(d)
                && !is_may(d)
                && matches!(deontic_action(d), DeonticAction::Block { .. })
        },
        "block (Gate polarity)",
    );
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&id| {
            let obj = state.objects.obj(id);
            // Derived controller ([CR#613.1b]): a stolen creature blocks for its
            // new controller. `blockable` reads the `May(Block)` grant
            // (default-deny combat capability).
            view.controller(id) == player && !obj.tapped && blockable(state, &view, id)
        })
        .collect()
}

/// One `Block`-action deontic row from the derived view (the polarity is
/// the collector's): the carrier it sits on, the `by`/`on` filters, and
/// the arrangement bound when present (menace's `count`, [CR#702.111b]).
pub(crate) struct BlockRow {
    pub carrier: crate::object::ObjectSource,
    /// The carrier's live object id — anchors `Ref(This)`/`StatOf(This)` in a
    /// `count` bound (frame source) when an arrangement bound is non-literal.
    pub carrier_id: ObjectId,
    pub by: Predicate,
    pub on: Predicate,
    pub count: Option<deckmaste_core::CountBound>,
}

/// Every `Block` row of the polarity `pick` extracts in the derived view,
/// with its carrier.
fn block_rows(
    state: &GameState,
    view: &LayeredView,
    pick: fn(&Deontic) -> Option<&DeonticAction>,
) -> Vec<BlockRow> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Block { by, on, count }) = pick(d)
            {
                rows.push(BlockRow {
                    carrier: source,
                    carrier_id: id,
                    by: by.clone(),
                    on: on.clone(),
                    count: count.clone(),
                });
            }
        });
    }
    rows
}

/// Every `Cant(Block)` row in the derived view ([CR#702.9b] flying-family
/// evasion, [CR#702.111b] menace's bound).
#[must_use]
pub(crate) fn cant_block_rows(state: &GameState, view: &LayeredView) -> Vec<BlockRow> {
    block_rows(state, view, cant_action)
}

/// Every `May(Block)` GRANT in the derived view — the permission side of block
/// eligibility under default-deny ([CR#509.1a]): declaring a permanent as a
/// blocker is a capability nothing has by default; a card's `Creature` type
/// CONFERS `May(Block(by: Ref(This)))`. Mirror of `cant_block_rows` on the
/// `May` polarity.
#[must_use]
fn may_block_rows(state: &GameState, view: &LayeredView) -> Vec<BlockRow> {
    block_rows(state, view, may_action)
}

/// [CR#509.1a]: whether `id` is a blocker CANDIDATE under default-deny — some
/// conferred `May(Block)` grant names it as a blocker (its `Creature` type's
/// grant, keyed on `by`). Grant PRESENCE, not net eligibility: the tapped /
/// `Cant(Block)` subtractions are applied by `legal_blockers`.
#[must_use]
pub(crate) fn blockable(state: &GameState, view: &LayeredView, id: ObjectId) -> bool {
    may_block_rows(state, view)
        .iter()
        .any(|r| state.filter_matches_live(&r.by, id, r.carrier))
}

/// Every `Must(Block)` row in the derived view — block requirements
/// ([CR#509.1c]: "blocks if able" effects, "all creatures able to block …
/// do so").
#[must_use]
pub(crate) fn must_block_rows(state: &GameState, view: &LayeredView) -> Vec<BlockRow> {
    block_rows(state, view, must_action)
}

/// Every `Cant(Untap)` row in the derived view ([CR#502.3]: "effects can
/// keep one or more of a player's permanents from untapping"; the continuous
/// "doesn't untap" family, e.g. the aura's "enchanted creature doesn't untap
/// during its controller's untap step"). Each row is its carrier plus the
/// one patient filter `what` — untapping is a turn-based action, not a deed
/// with an agent slot. Gathered once by the untap step, then matched against
/// each of the active player's permanents with [`untap_forbidden_by`].
#[must_use]
pub(crate) fn cant_untap_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Predicate)> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Untap { what }) = cant_action(d)
            {
                rows.push((source, what.clone()));
            }
        });
    }
    rows
}

/// [CR#502.3]: whether any continuous `Cant(Untap)` row (pre-gathered by
/// [`cant_untap_rows`]) forbids `id` from untapping — the untap step's
/// turn-based action consults this to leave a restricted permanent tapped.
#[must_use]
pub(crate) fn untap_forbidden_by(
    state: &GameState,
    rows: &[(crate::object::ObjectSource, Predicate)],
    id: ObjectId,
) -> bool {
    rows.iter()
        .any(|(carrier, what)| state.filter_matches_live(what, id, *carrier))
}

/// Every `Cant(Activate)` row in the derived view ([CR#602.5a]: the
/// summoning-sickness tap gate a card's `Creature` type confers, and any other
/// "can't activate" effect). Each row is its carrier plus the ability's patient
/// filter `what`, the agent filter `by`, and the optional `cost` predicate that
/// scopes the row to a subset of activations (a `{T}`/`{Q}` cost). Mirrors
/// [`cant_untap_rows`] on the `Activate` action.
#[must_use]
fn cant_activate_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(
    crate::object::ObjectSource,
    Predicate,
    Predicate,
    Option<deckmaste_core::CostPredicate>,
)> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Activate { what, by, cost }) = cant_action(d)
            {
                rows.push((source, what.clone(), by.clone(), cost.clone()));
            }
        });
    }
    rows
}

/// Whether a `Cant(Activate)` row's `cost` predicate matches an activation
/// whose cost `includes_tap_symbol`. `None` = the row scopes to ANY activation;
/// `IncludesTapSymbol` matches only a `{T}`/`{Q}` cost ([CR#602.5a]).
#[must_use]
fn cost_predicate_holds(
    cost: Option<&deckmaste_core::CostPredicate>,
    includes_tap_symbol: bool,
) -> bool {
    match cost {
        None => true,
        Some(deckmaste_core::CostPredicate::IncludesTapSymbol) => includes_tap_symbol,
    }
}

/// [CR#602.5a,602.1]: whether some conferred `Cant(Activate)` row forbids
/// `activator` activating an ability of `object` whose cost matches
/// `includes_tap_symbol`. A row applies when `object` matches its `what`, the
/// `activator`'s player proxy matches its `by`, AND its `cost` predicate holds
/// for the activation (`IncludesTapSymbol` only when `includes_tap_symbol`).
/// The summoning-sickness tap gate rides this: a card's `Creature` type confers
/// `Conditionally(SummoningSick & !Haste, Cant(Activate(cost:
/// IncludesTapSymbol)))`, so a sick creature can't pay a `{T}`/`{Q}` cost but
/// can pay a non-tap one.
#[must_use]
pub(crate) fn cant_activate(
    state: &GameState,
    view: &LayeredView,
    object: ObjectId,
    activator: PlayerId,
    includes_tap_symbol: bool,
) -> bool {
    let proxy = state.player(activator).object;
    cant_activate_rows(state, view)
        .iter()
        .any(|(carrier, what, by, cost)| {
            cost_predicate_holds(cost.as_ref(), includes_tap_symbol)
                && state.filter_matches_live(what, object, *carrier)
                && state.filter_matches_live(by, proxy, *carrier)
        })
}

/// The single ability-tree walker. Descends an ability list with the
/// look-through rules every static read needs (static-ability effect lists,
/// keyword composites — flying's evasion `Cant` lives inside
/// `Keyword(Composite)` — and macro `Expanded` wrappers at every level),
/// calling `visit` on each static effect. The `ControlFlow` return lets a
/// caller short-circuit: [`object_has_static`] (the boolean "any" form) breaks
/// on the first match, while the visit-each callers always
/// [`Continue`](ControlFlow::Continue) to see every effect. The whole-walk
/// result propagates the visitor's `Break` value (or `Continue(())` when the
/// descent ran to completion). View-free so it can be unit-tested directly;
/// [`statics_on`] is the thin `LayeredView` adapter over it.
///
/// `enter_conditional` GATES a [`StaticEffect::Conditionally`] ([CR#611.3a]):
/// the walker descends into the inner effect only when `enter_conditional`
/// accepts the wrapper's condition. Row collectors pass a closure that
/// evaluates the condition against the game state (via [`for_each_static`]), so
/// a conditional static contributes only while its condition holds; the
/// presence-only callers ([`object_has_static`], the cost-modifier self-scan)
/// pass `|_| true` and keep the old unconditional look-through.
///
/// The composite-keyword splice (`in_keyword`) is the legality/SBA twin of the
/// layer `gather`'s `derive::flatten_composites`: a keyword macro that confers
/// a static (Enchant → `May(Attach)`, [CR#702.5a]) lands as
/// `Keyword(Composite { abilities: [Static(...)] })`, so a static-read path
/// that did NOT descend into the composite would silently miss the conferred
/// row. `Innate` is peeled here too, so subtype-conferred
/// `Innate(May(Attach))` ([CR#301.5,301.6]) and `Innate(Static([Sba(...)]))`
/// ([CR#704.5m]) are seen.
pub(crate) fn walk_abilities<B, F, G>(
    abilities: &[Ability],
    enter_conditional: &mut G,
    visit: &mut F,
) -> ControlFlow<B>
where
    F: FnMut(&StaticEffect) -> ControlFlow<B>,
    G: FnMut(&deckmaste_core::Condition) -> bool,
{
    fn in_ability<B, F, G>(a: &Ability, enter: &mut G, visit: &mut F) -> ControlFlow<B>
    where
        F: FnMut(&StaticEffect) -> ControlFlow<B>,
        G: FnMut(&deckmaste_core::Condition) -> bool,
    {
        match a {
            Ability::Static(s) => in_static(s, enter, visit),
            Ability::Keyword(k) => in_keyword(k, enter, visit),
            Ability::Expanded(e) => in_ability(&e.value, enter, visit),
            // Peel `Innate` — its inner static is consumed normally
            // ([CR#113.12,604.1]).
            Ability::Innate(inner) => in_ability(inner, enter, visit),
            _ => ControlFlow::Continue(()),
        }
    }
    fn in_keyword<B, F, G>(k: &KeywordAbility, enter: &mut G, visit: &mut F) -> ControlFlow<B>
    where
        F: FnMut(&StaticEffect) -> ControlFlow<B>,
        G: FnMut(&deckmaste_core::Condition) -> bool,
    {
        match k {
            KeywordAbility::Composite { abilities, .. } => {
                for a in abilities {
                    in_ability(a, enter, visit)?;
                }
                ControlFlow::Continue(())
            }
            KeywordAbility::Expanded(e) => in_keyword(&e.value, enter, visit),
            _ => ControlFlow::Continue(()),
        }
    }
    fn in_static<B, F, G>(e: &StaticEffect, enter: &mut G, visit: &mut F) -> ControlFlow<B>
    where
        F: FnMut(&StaticEffect) -> ControlFlow<B>,
        G: FnMut(&deckmaste_core::Condition) -> bool,
    {
        match e {
            StaticEffect::Expanded(x) => in_static(&x.value, enter, visit),
            // Distributed statics ([`StaticEffect::Each`]) are looked through to
            // their inner effect for this presence scan: the walker's callers
            // (Cant/Sba/CostModifier row collectors) match on the effect KIND,
            // not the affected set, so the wrapping `Selection` is immaterial
            // here.
            StaticEffect::Each(_, inner) => in_static(inner, enter, visit),
            // [CR#611.3a]: a `Conditionally` wrapper contributes its inner
            // effect only when `enter` accepts the condition. Collectors gate on
            // the live condition; the presence-only walkers pass `|_| true` and
            // keep the unconditional look-through.
            StaticEffect::Conditionally(cond, inner) => {
                if enter(cond) {
                    in_static(inner, enter, visit)
                } else {
                    ControlFlow::Continue(())
                }
            }
            other => visit(other),
        }
    }
    for a in abilities {
        in_ability(a, enter_conditional, visit)?;
    }
    ControlFlow::Continue(())
}

/// The `LayeredView` adapter over [`walk_abilities`]: walks `id`'s derived
/// ability list, calling `visit` on each static effect and short-circuiting on
/// the visitor's `Break`. `enter_conditional` gates `Conditionally` statics
/// (forwarded to the walker).
fn statics_on<B, F, G>(
    view: &LayeredView,
    id: ObjectId,
    enter_conditional: &mut G,
    visit: &mut F,
) -> ControlFlow<B>
where
    F: FnMut(&StaticEffect) -> ControlFlow<B>,
    G: FnMut(&deckmaste_core::Condition) -> bool,
{
    walk_abilities(&view.get(id).abilities, enter_conditional, visit)
}

/// The non-short-circuiting view over [`statics_on`]: runs `visit` on every
/// static effect of `id` (no early exit). The visit-each callers
/// (`attack_rows`/`block_rows`/`target_rows`/`may_cast_rows`) collect rows
/// through this, leaving the `ControlFlow` plumbing to the one walker. Also
/// the entry point for the [CR#704] SBA sweep, which collects `Sba` rows the
/// same look-through way.
///
/// [CR#611.3a]: a `Conditionally` static is GATED — its inner effect is visited
/// only when the wrapper's condition holds for `id`, evaluated with `This`
/// bound to `id` (a `Frame::bare` on the object's controller).
pub(crate) fn for_each_static<F: FnMut(&StaticEffect)>(
    state: &GameState,
    view: &LayeredView,
    id: ObjectId,
    mut visit: F,
) {
    // Gate `Conditionally` on the already-built `view`. This runs POST
    // `state.layers()` (every `for_each_static` caller is a legality/SBA/cost
    // query that first derives the view, never a step of the layer computation
    // itself), so a condition that reads characteristics triggers a FRESH,
    // non-recursive `state.layers()` — the escape `conferred_rule_abilities`
    // uses ([CR#611.3a]).
    let controller = state.objects.obj(id).controller;
    let frame = crate::stack::Frame::bare(id, controller);
    let mut enter = |cond: &deckmaste_core::Condition| state.condition_holds(cond, &frame);
    // The visitor never breaks, so the only outcome is the run-to-completion
    // `Continue(())`, deliberately discarded.
    let _ = statics_on(view, id, &mut enter, &mut |e| {
        visit(e);
        ControlFlow::<()>::Continue(())
    });
}

/// The carrier of the first POINT-WISE `Cant(Block)` row forbidding
/// `blocker` blocking `attacker`, if any — `by`/`on` evaluate against the
/// LIVE objects with the row's carrier as `This` ([CR#702.9b]: flying's
/// row sits on the attacker, so `on: Ref(This)` anchors there).
#[must_use]
pub(crate) fn block_forbidden_by(
    state: &GameState,
    rows: &[BlockRow],
    blocker: ObjectId,
    attacker: ObjectId,
) -> Option<crate::object::ObjectSource> {
    rows.iter()
        .filter(|r| r.count.is_none())
        .find(|r| {
            state.filter_matches_live(&r.by, blocker, r.carrier)
                && state.filter_matches_live(&r.on, attacker, r.carrier)
        })
        .map(|r| r.carrier)
}

/// The carrier of the first ARRANGEMENT-LEVEL `Cant(Block)` row (a `count`
/// bound) forbidding this attacker's whole blocker set ([CR#702.111b]
/// menace: a non-empty set of fewer than two is forbidden; an empty set is
/// not a blocking arrangement at all).
#[must_use]
pub(crate) fn arrangement_forbidden_by(
    state: &GameState,
    rows: &[BlockRow],
    attacker: ObjectId,
    blockers: &[ObjectId],
) -> Option<crate::object::ObjectSource> {
    rows.iter()
        .filter(|r| r.count.is_some())
        .find(|r| {
            if !state.filter_matches_live(&r.on, attacker, r.carrier) {
                return false;
            }
            let n = blockers
                .iter()
                .filter(|&&b| state.filter_matches_live(&r.by, b, r.carrier))
                .count();
            let Ok(n) = deckmaste_core::Uint::try_from(n) else {
                return false;
            };
            // An empty set is not a blocking arrangement at all (see this
            // fn's doc, [CR#702.111b]), so it is never forbidden — only a
            // non-empty set is judged.
            if n == 0 {
                return false;
            }
            // The bound rides the one count evaluator (`eval_count`) through
            // `CountBound::satisfied_by`, so a non-literal bound (e.g. "fewer
            // than its power" — `StatOf(This, Power)`) evaluates instead of
            // panicking. `Ref(This)`/`StatOf(This)` anchor on the carrier, so
            // build a carrier-source frame with no resolution context.
            let frame =
                crate::stack::Frame::bare(r.carrier_id, state.objects.obj(r.carrier_id).controller);
            r.count
                .as_ref()
                .expect("filtered to Some")
                .satisfied_by(n, |c| state.eval_count(c, &frame))
        })
        .map(|r| r.carrier)
}

/// Every `Target` row of the polarity `pick` extracts in the derived view,
/// with its carrier — `(carrier source, by, on)`: `by` is the two-slot
/// [`DeedAgent`] matching the targeting spell/ability
/// ([CR#702.11d,702.16b]), `on` the would-be target.
fn target_rows(
    state: &GameState,
    view: &LayeredView,
    pick: fn(&Deontic) -> Option<&DeonticAction>,
) -> Vec<(crate::object::ObjectSource, DeedAgent, Predicate)> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Target { by, on }) = pick(d)
            {
                rows.push((source, by.clone(), on.clone()));
            }
        });
    }
    rows
}

/// Whether `actor` (a targeting stack object / in-flight announce) matches
/// the two-slot deed agent: both PRESENT arms must hold —
/// `stack_object` against the actor itself, `source` against the actor's
/// source ([CR#113.7] an ability's generator; [CR#609.7a] a spell is itself
/// a source) — so hexproof-from-red's one row covers "red spells … or
/// abilities … from red sources" ([CR#702.11d]). The EMPTY agent (which the
/// Idris re-emit gate rejects) defensively matches nothing.
pub(crate) fn deed_agent_matches(
    state: &GameState,
    agent: &DeedAgent,
    actor: ObjectId,
    carrier: crate::object::ObjectSource,
) -> bool {
    if agent.is_empty() {
        return false;
    }
    let stack_object_ok = agent
        .stack_object
        .as_ref()
        .is_none_or(|f| state.filter_matches_live(f, actor, carrier));
    let source_ok = agent.source.as_ref().is_none_or(|f| {
        crate::target::source_of(state, actor)
            .is_some_and(|src| state.filter_matches_live(f, src, carrier))
    });
    stack_object_ok && source_ok
}

/// Every `Cant(Target)` row in the derived view ([CR#702.11b] hexproof,
/// [CR#702.16b] protection's targeted clause).
#[must_use]
pub(crate) fn cant_target_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, DeedAgent, Predicate)> {
    target_rows(state, view, cant_action)
}

/// Every `Must(Target)` row in the derived view — targeting requirements
/// (the Flagbearer class: "must choose at least one … if able" — a
/// choice-time constraint inside [CR#601.2c]'s legal-target selection).
#[must_use]
pub(crate) fn must_target_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, DeedAgent, Predicate)> {
    target_rows(state, view, must_action)
}

/// The carrier of the first `Cant(Target)` row forbidding `spell` (the
/// targeting stack object / in-flight announce) from targeting `target`.
#[must_use]
pub(crate) fn target_forbidden_by(
    state: &GameState,
    rows: &[(crate::object::ObjectSource, DeedAgent, Predicate)],
    spell: ObjectId,
    target: ObjectId,
) -> Option<crate::object::ObjectSource> {
    rows.iter()
        .find(|(carrier, by, on)| {
            deed_agent_matches(state, by, spell, *carrier)
                && state.filter_matches_live(on, target, *carrier)
        })
        .map(|(carrier, ..)| *carrier)
}

/// Every `Cant(Attach)` row in the derived view, with its carrier:
/// `(carrier source, what, to)`. Both the **attachment-side** restriction
/// (Enchant's quality bound [CR#702.5a], the Equipment [CR#301.5] /
/// Fortification [CR#301.6] host rule, conferred `Innate`) and the
/// **host-side** restriction (protection's can't-be-equipped clause
/// [CR#702.16d]) land here — the row is read the same way regardless of which
/// permanent carries it. The walker peels `Innate` ([CR#113.12]).
#[must_use]
fn cant_attach_rows(
    state: &GameState,
    view: &LayeredView,
    attachment: ObjectId,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    let mut rows = Vec::new();
    // The battlefield carries the host-side restrictions (protection) and any
    // already-in-play attachment's own rows; ALSO scan `attachment` itself when
    // it is not yet on the battlefield (the [CR#303.4f] enters-attached fold
    // evaluates legality before the Aura is placed), so its self-carried rows
    // are seen — the same "chain the candidate" idiom `may_cast_rows` uses.
    let off_field = (!state.zones.battlefield.contains(&attachment)).then_some(attachment);
    for id in state.zones.battlefield.iter().copied().chain(off_field) {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Attach { what, to }) = cant_action(d)
            {
                rows.push((source, what.clone(), to.clone()));
            }
        });
    }
    rows
}

/// Every `May(Attach)` row in the derived view, with its carrier:
/// `(carrier source, what, to)`. This is the **grant** side of attachment
/// legality under default-deny: being attachable to a host is a capability
/// nothing has by default — the Equipment [CR#301.5] / Fortification
/// [CR#301.6] host rules and Enchant's quality bound [CR#702.5a] are conferred
/// as `May(Attach)` grants (via `Innate` [CR#113.12], peeled by the walker),
/// not as restrictions subtracting from a phantom default-permission. Mirror
/// of `cant_attach_rows`.
#[must_use]
fn may_attach_rows(
    state: &GameState,
    view: &LayeredView,
    attachment: ObjectId,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    let mut rows = Vec::new();
    // Scan the battlefield PLUS `attachment` itself when it is not yet in play:
    // an Aura's own `May(Attach)` grant must be visible while the [CR#303.4f]
    // enters-attached fold decides its host, before it is placed on the
    // battlefield (mirrors `may_cast_rows` chaining its off-battlefield
    // candidate).
    let off_field = (!state.zones.battlefield.contains(&attachment)).then_some(attachment);
    for id in state.zones.battlefield.iter().copied().chain(off_field) {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Attach { what, to }) = may_action(d)
            {
                rows.push((source, what.clone(), to.clone()));
            }
        });
    }
    rows
}

/// [CR#701.3b,303.4d]: whether `attachment` may legally be attached to `host`
/// — the ONE predicate used at both attach-time (the [CR#701.3b] no-op) and
/// SBA-time (the [CR#704.5m..704.5p] illegal-attachment sweep).
///
/// **Default-deny**: attachment is a *granted* capability. `false` when
/// `host == attachment` ([CR#303.4d] can't attach to itself) or `host` is gone;
/// otherwise true iff SOME applicable `May(Attach(what, to))` grant permits the
/// pair AND no applicable `Cant(Attach(what, to))` row forbids it — `what`
/// matched against the attachment and `to` against the host, both evaluated
/// against LIVE/derived characteristics with the row's carrier as `This` (so
/// protection / type changes / the conferred `Innate` host rules are all seen).
/// This is the **generic** legality read: it never branches on the
/// Aura/Equipment/Fortification subtype — those carry their permissions as
/// conferred `May(Attach)` grants ([CR#702.5a,301.5,301.6]), and a `Cant`
/// (protection [CR#702.16d]) subtracts from any grant.
#[must_use]
pub(crate) fn attachment_legal(state: &GameState, attachment: ObjectId, host: ObjectId) -> bool {
    // [CR#303.4d]: an attachment can't be attached to itself.
    if host == attachment {
        return false;
    }
    // A host that has left the game / is no longer live is not a legal host.
    if state.objects.get(host).is_none() {
        return false;
    }
    let view = state.layers();
    let may = may_attach_rows(state, &view, attachment);
    let cant = cant_attach_rows(state, &view, attachment);
    // A row matches this (attachment, host) pair when its `what` matches the
    // attachment AND its `to` matches the host, both anchored on the row's
    // carrier (so `Attach(what: Ref(This), …)` on the attachment, and
    // `Attach(to: Ref(This))` on the host, resolve their self-reference
    // correctly). Legal iff SOME `May` grant permits AND no `Cant` forbids.
    let matches = |carrier: &crate::object::ObjectSource, what: &Predicate, to: &Predicate| {
        state.filter_matches_live(what, attachment, *carrier)
            && state.filter_matches_live(to, host, *carrier)
    };
    may.iter()
        .any(|(carrier, what, to)| matches(carrier, what, to))
        && !cant
            .iter()
            .any(|(carrier, what, to)| matches(carrier, what, to))
}

/// Whether `object`'s derived view confers `May(Play(what: <self>))` — the
/// default-deny land-play marker ([CR#305.9,116.2a,701.18]). A card's Land type
/// CONFERS this row (folded into the derived abilities by `printed_of_face`),
/// so land-play legality (`legal_actions`) AND spell-non-castability
/// (`castable_cost_ignoring_mana`) are both keyed on the capability rather than
/// a `Type::Land` literal — per-face correct for an MDFC land//spell. Mirrors
/// how `attachment_legal` reads `May(Attach)`; `object_has_static` peels
/// `Innate`, so the type-conferred grant is seen. `_state` is unused today (the
/// read works off the already-derived `view`) but kept for signature symmetry
/// with the sibling legality readers.
#[must_use]
pub(crate) fn confers_may_play(_state: &GameState, view: &LayeredView, object: ObjectId) -> bool {
    object_has_static(view, object, &|e| {
        matches!(e, StaticEffect::Deontic(d)
            if matches!(may_action(d), Some(DeonticAction::Play { .. })))
    })
}

/// Every `Cant(Counter)` row visible to a counter of `target`, with its
/// carrier: `(carrier source, by, on)`. Rows come from battlefield permanents
/// (Dromoka-style grants — "spells you control can't be countered") PLUS the
/// `target`'s OWN abilities, since "this spell can't be countered" is a static
/// the spell carries while on the stack ([CR#701.6a]) — the same
/// battlefield + candidate gathering `may_cast_rows` uses for flash. A
/// countered ABILITY on the stack ([CR#113.3b]) is a non-card object with no
/// characteristics in the layered view, so its own statics are skipped (it
/// carries none); only the battlefield grants can reach it.
#[must_use]
fn cant_counter_rows(
    state: &GameState,
    view: &LayeredView,
    target: ObjectId,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    let mut rows = Vec::new();
    // The target's own row is only visible if it is a card-backed object (in
    // the derived view); a bare stack ability isn't, and `view.get` would panic.
    let self_row = state.objects.obj(target).card_id().is_some();
    let ids = state
        .zones
        .battlefield
        .iter()
        .copied()
        .chain(self_row.then_some(target));
    for id in ids {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Counter { by, on }) = cant_action(d)
            {
                rows.push((source, by.clone(), on.clone()));
            }
        });
    }
    rows
}

/// [CR#701.6a]: whether the countering source `by` may legally counter
/// `target` — the eval hook on the counter-resolution path. `false` iff some
/// applicable `Cant(Counter(by, on))` row forbids the pair: `by` matched
/// against the countering stack object, `on` against the countered object,
/// both anchored on the row's carrier (so a self-referential
/// `Cant(Counter(on: Ref(This)))` on the target resolves its `This`
/// correctly). A forbidden counter simply doesn't affect the object — it is
/// not moved off the stack.
#[must_use]
pub(crate) fn counter_legal(state: &GameState, by: ObjectId, target: ObjectId) -> bool {
    let view = state.layers();
    let rows = cant_counter_rows(state, &view, target);
    !rows.iter().any(|(carrier, by_pred, on_pred)| {
        state.filter_matches_live(by_pred, by, *carrier)
            && state.filter_matches_live(on_pred, target, *carrier)
    })
}

/// One `May(Cast)` row from the derived view: the carrier it sits on and
/// the permission's slots. `window` is the timing lift ([CR#702.8a]
/// flash); `from`/`cost` are the cast-from-zones / alternative-cost
/// unlocks, carried so the evaluation site can refuse shapes it doesn't
/// evaluate yet.
pub(crate) struct MayCastRow {
    pub carrier: crate::object::ObjectSource,
    pub what: Predicate,
    pub by: Predicate,
    pub from: Option<deckmaste_core::Zone>,
    pub window: Option<deckmaste_core::Timing>,
    pub cost: Option<deckmaste_core::AlternativeCost>,
}

/// Every `May(Cast)` row visible to a cast of `candidate`: rows carried by
/// battlefield permanents (Orrery-style grants) plus the candidate's OWN
/// rows — flash functions from the zone the card is played from
/// ([CR#702.8a]), the hand here.
#[must_use]
pub(crate) fn may_cast_rows(
    state: &GameState,
    view: &LayeredView,
    candidate: ObjectId,
) -> Vec<MayCastRow> {
    let mut rows = Vec::new();
    for &id in state.zones.battlefield.iter().chain([&candidate]) {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Cast {
                    what,
                    by,
                    from,
                    window,
                    cost,
                    // The alt-cost `tag` names the permission for the
                    // `CastWith`/`WasCastWith` readers; the announce record
                    // that consumes it is engine-alt-costs, so the
                    // permission derivation ignores it here.
                    tag: _,
                }) = may_action(d)
            {
                rows.push(MayCastRow {
                    carrier: source,
                    what: what.clone(),
                    by: by.clone(),
                    from: *from,
                    window: *window,
                    cost: cost.clone(),
                });
            }
        });
    }
    rows
}

#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;
    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::Deontic;
    use deckmaste_core::DeonticAction;
    use deckmaste_core::Expansion;
    use deckmaste_core::Ident;
    use deckmaste_core::KeywordAbility;
    use deckmaste_core::OutcomeGateKind;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use super::attachment_legal;
    use super::walk_abilities;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    /// A distinguishable leaf effect: `OutcomeGate` tagged by `gate` so a
    /// collected sequence is order-checkable.
    fn gate(gate: OutcomeGateKind) -> StaticEffect {
        StaticEffect::OutcomeGate {
            who: Predicate::Any,
            gate,
        }
    }

    fn expand<T>(value: T) -> Expansion<T> {
        Expansion {
            name: Ident::new("Wrapper"),
            args: deckmaste_core::ExpansionArgs::none(),
            template: None,
            value: Box::new(value),
        }
    }

    fn static_ability(effect: StaticEffect) -> Ability {
        Ability::Static(effect)
    }

    /// A tree exercising every look-through path the one walker must descend:
    /// a plain `Static` effect, an `Expanded`-wrapped effect (its own
    /// `Static` — each ability now carries exactly one `effect`, so what was
    /// once two effects on one ability is now two sibling abilities), a
    /// `Static` reached through a `Composite` keyword, and a `Static` reached
    /// through an `Expanded` ability wrapper.
    fn sample_tree() -> Vec<Ability> {
        use OutcomeGateKind::CantLose;
        use OutcomeGateKind::CantWin;
        vec![
            // [0] plain static effect.
            static_ability(gate(CantLose)),
            // [0b] an Expanded-wrapped effect, as a sibling ability.
            static_ability(StaticEffect::Expanded(expand(gate(CantWin)))),
            // [1] effect reached through a keyword composite.
            Ability::Keyword(KeywordAbility::Composite {
                name: Ident::new("Kw"),
                abilities: vec![static_ability(gate(CantLose))],
            }),
            // [2] effect reached through an Expanded ability wrapper.
            Ability::Expanded(expand(static_ability(gate(CantWin)))),
        ]
    }

    /// The visit-each form sees every static effect, descending through
    /// static-ability effect lists, keyword composites, and `Expanded`
    /// wrappers at every level — in DFS order.
    #[test]
    fn walk_visits_every_effect_through_all_wrappers() {
        use OutcomeGateKind::CantLose;
        use OutcomeGateKind::CantWin;

        let tree = sample_tree();
        let mut seen = Vec::new();
        let done = walk_abilities(&tree, &mut |_: &deckmaste_core::Condition| true, &mut |e| {
            if let StaticEffect::OutcomeGate { gate, .. } = e {
                seen.push(*gate);
            }
            ControlFlow::<()>::Continue(())
        });
        assert!(done.is_continue(), "a non-breaking walk runs to completion");
        assert_eq!(seen, vec![CantLose, CantWin, CantLose, CantWin]);
    }

    /// The boolean "any" form short-circuits: the visitor `Break`s on the
    /// first match, and the descent stops there rather than visiting the rest.
    #[test]
    fn walk_short_circuits_on_break() {
        let tree = sample_tree();
        let mut visited = 0usize;
        let hit = walk_abilities(&tree, &mut |_: &deckmaste_core::Condition| true, &mut |e| {
            visited += 1;
            if matches!(
                e,
                StaticEffect::OutcomeGate {
                    gate: OutcomeGateKind::CantWin,
                    ..
                }
            ) {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        assert!(hit.is_break(), "the matching effect must break the walk");
        // Stops at the SECOND effect (CantLose, then the wrapped CantWin) —
        // it does not go on to visit the composite/Expanded branches.
        assert_eq!(visited, 2, "the walk must not visit effects past the match");
    }

    /// No match anywhere: the boolean form reports `Continue` (false), having
    /// visited every effect.
    #[test]
    fn walk_no_match_runs_to_completion() {
        let tree = sample_tree();
        let mut visited = 0usize;
        let res = walk_abilities(
            &tree,
            &mut |_: &deckmaste_core::Condition| true,
            &mut |_e| {
                visited += 1;
                ControlFlow::<()>::Continue(())
            },
        );
        assert!(res.is_continue());
        assert_eq!(
            visited, 4,
            "every leaf effect is visited when nothing breaks"
        );
    }

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

    /// Mint a battlefield object of `types` carrying `abilities` (player 0).
    fn obj_on_field(
        state: &mut GameState,
        name: &str,
        types: Vec<Type>,
        abilities: Vec<Ability>,
    ) -> ObjectId {
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        let card = Card::Normal(CardFace {
            name: name.into(),
            types: types.into_iter().map(Type::def).collect(),
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
        id
    }

    /// An `Innate` static carrying a single `May(Attach(what, to))` grant — the
    /// conferred attachable-to-host shape (Equipment/Fortification subtype
    /// rule) under default-deny attachment.
    fn innate_may_attach(what: Predicate, to: Predicate) -> Ability {
        Ability::Innate(Box::new(Ability::Static(StaticEffect::Deontic(
            Deontic::May(DeonticAction::Attach { what, to }),
        ))))
    }

    fn creature() -> Predicate {
        Predicate::creature()
    }

    /// [CR#701.3b,301.5]: under default-deny, an attachment with
    /// `Innate(May(Attach(what: Ref(This), to: Creature)))` (the
    /// Equipment-subtype grant) is legal on a creature host and illegal on a
    /// non-creature host — no grant covers the non-creature pair.
    #[test]
    fn attachment_legal_honors_attachment_side_grant() {
        let mut state = game();
        let equip = obj_on_field(
            &mut state,
            "Test Equipment",
            vec![Type::Artifact],
            vec![innate_may_attach(
                Predicate::Ref(Reference::This),
                creature(),
            )],
        );
        let creature_host = obj_on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        let noncreature_host = obj_on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);

        assert!(
            attachment_legal(&state, equip, creature_host),
            "Equipment's May(Attach to: Creature) grant is legal on a creature host"
        );
        assert!(
            !attachment_legal(&state, equip, noncreature_host),
            "Equipment is illegal on a non-creature host — no grant covers it ([CR#301.5])"
        );
    }

    /// The default-deny floor: a plain permanent carrying NO `May(Attach)`
    /// grant is not a legal attachment onto ANY host — a Mountain cannot be
    /// attached to anything. Attach is a capability that is OFF by default;
    /// the grants on Equipment/Aura/Fortification are what turn it on.
    #[test]
    fn attachment_legal_denies_a_grantless_permanent_on_any_host() {
        let mut state = game();
        let mountain = obj_on_field(&mut state, "Mountain", vec![Type::Land], vec![]);
        let creature_host = obj_on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        let artifact_host = obj_on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);

        assert!(
            !attachment_legal(&state, mountain, creature_host),
            "a grantless permanent is not attachable to a creature — default-deny"
        );
        assert!(
            !attachment_legal(&state, mountain, artifact_host),
            "a grantless permanent is not attachable to any host"
        );
    }

    /// [CR#303.4d]: an attachment can't be attached to itself, and a missing
    /// host is never legal.
    #[test]
    fn attachment_legal_false_on_self_and_missing_host() {
        let mut state = game();
        // Under default-deny the attachment needs a `May(Attach)` grant to be
        // legal on any host at all; carry one so the bracketing legal case holds.
        let a = obj_on_field(
            &mut state,
            "Aura",
            vec![Type::Enchantment],
            vec![innate_may_attach(
                Predicate::Ref(Reference::This),
                creature(),
            )],
        );
        assert!(
            !attachment_legal(&state, a, a),
            "can't attach to itself ([CR#303.4d])"
        );
        // A host id with no live object (use a from_raw invalid id is awkward;
        // instead assert the self case + a normal legal case to bracket it).
        let host = obj_on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        assert!(
            attachment_legal(&state, a, host),
            "a granted attachment is legal on a matching live host"
        );
    }

    /// [CR#702.16d]: a HOST-side `Cant(Attach(what: Any, to: Ref(This)))`
    /// (protection's can't-be-equipped clause shape) makes the host illegal for
    /// any attachment — the row is read off the host the same generic way.
    #[test]
    fn attachment_legal_honors_host_side_cant() {
        let mut state = game();
        // Under default-deny the attachment carries a `May(Attach to: Creature)`
        // grant, so it's legal on a plain creature; the protected host's `Cant`
        // subtracts from that grant.
        let attachment = obj_on_field(
            &mut state,
            "Aura",
            vec![Type::Enchantment],
            vec![innate_may_attach(
                Predicate::Ref(Reference::This),
                creature(),
            )],
        );
        // A protected creature: forbids ANY attachment onto itself.
        let protected = obj_on_field(
            &mut state,
            "Protected Bear",
            vec![Type::Creature],
            vec![Ability::Static(StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Attach {
                    what: Predicate::Any,
                    to: Predicate::Ref(Reference::This),
                },
            )))],
        );
        let plain = obj_on_field(&mut state, "Plain Bear", vec![Type::Creature], vec![]);

        assert!(
            !attachment_legal(&state, attachment, protected),
            "host-side Cant(Attach to: Ref(This)) forbids attachment ([CR#702.16d])"
        );
        assert!(
            attachment_legal(&state, attachment, plain),
            "an unprotected host is legal"
        );
    }

    // --- I1: Innate filtering must not desync the activated-ability index ----

    /// A tap-for-mana activated ability `tap_mana_ability` recognizes: cost
    /// `[Tap]`, no targets, producing one fixed-colour mana.
    fn tap_for_colorless() -> Ability {
        use deckmaste_core::Action;
        use deckmaste_core::ActivatedAbility;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::CostComponent;
        use deckmaste_core::Count;
        use deckmaste_core::ManaProduction;
        use deckmaste_core::ManaSpec;
        use deckmaste_core::OneShotEffect;
        use deckmaste_core::PlayerAction;
        Ability::Activated(ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![CostComponent::Tap].into(),
            window: None,
            condition: None,
            limits: vec![],
            effect: OneShotEffect::Act(Action::By(
                Reference::You,
                PlayerAction::AddMana(
                    Count::Literal(1),
                    ManaProduction::Bare(ManaSpec::Specific(ColorOrColorless::Colorless)),
                ),
            )),
        })
    }

    /// An `Innate` static (any conferred rule): PEELED in place — never
    /// dropped — by the usable list, so it occupies an index slot.
    fn innate_static() -> Ability {
        Ability::Innate(Box::new(Ability::Static(StaticEffect::Deontic(
            Deontic::Cant(DeonticAction::Attach {
                what: Predicate::Ref(Reference::This),
                to: Predicate::Not(Box::new(creature())),
            }),
        ))))
    }

    /// [CR#113.12,613.1f]: with an `Innate` ability positioned BEFORE an
    /// activated one, the legal-action list and resolution must share ONE
    /// index space — the PEELED usable list (`derive::usable_abilities`),
    /// where the Innate keeps its slot (peeled, never dropped): the offered
    /// `Action::ActivateAbility { ability }` index is 1, and
    /// `begin_activate` resolves THAT ability. The card-facing
    /// `derive::abilities` still FILTERS the Innate out (the [CR#113.12]
    /// invisibility surface) — it is no longer the index space.
    #[test]
    fn innate_before_activated_does_not_desync_the_index() {
        let mut state = game();
        // Abilities printed in this order: [Innate(Static), Activated(tap mana)].
        // An artifact so the mana-ability path's `sick_creature` guard is false.
        let object = obj_on_field(
            &mut state,
            "Manarock",
            vec![Type::Artifact],
            vec![innate_static(), tap_for_colorless()],
        );

        // (a) Exactly one activation is offered, and it indexes the PEELED
        // usable list: the Innate keeps slot 0, the activated ability is 1.
        let actions = super::legal_actions(&state, PlayerId(0));
        let activations: Vec<_> = actions
            .iter()
            .filter_map(|a| match a {
                crate::decide::Action::ActivateAbility { object: o, ability } if *o == object => {
                    Some(*ability)
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            activations,
            vec![1],
            "the offered activation must index the PEELED usable list (1) — the \
             Innate keeps its slot"
        );

        // (b) The offered index resolves to the activated ability in the SAME
        // usable list, and `begin_activate` stages it without panicking.
        let usable = crate::derive::usable_abilities(&state, object);
        assert!(
            crate::activate::as_activated(&usable[activations[0]]).is_some(),
            "the offered index names the activated ability in the usable list"
        );
        // The card-facing surface still hides the conferral ([CR#113.12]).
        assert_eq!(
            crate::derive::abilities(&state, object).len(),
            1,
            "derive::abilities filters the Innate out of the card-facing list"
        );
        state.begin_activate(object, activations[0]);
        assert!(
            state.announcing.is_some(),
            "begin_activate staged the activated ability at the offered index"
        );
    }

    // --- PREREQUISITE: composite-keyword flattening for static reads ----------

    /// [CR#702.5a]: the **Enchant** keyword confers its `May(Attach)` grant
    /// nested inside a `Keyword(Composite { abilities: [Static(...)] })` (the
    /// macro body shape). `attachment_legal` reads `May(Attach)` via
    /// `statics_on`, which must look THROUGH the composite (splice its members)
    /// for the conferred grant to be visible — otherwise the enchant host bound
    /// is silently ignored and the Aura is illegal everywhere. This mirrors the
    /// layer `gather` composite flattening (`derive::flatten_composites`).
    #[test]
    fn attachment_legal_sees_may_attach_nested_in_composite_keyword() {
        use deckmaste_core::KeywordAbility;
        let mut state = game();
        // An attachment whose ONLY `May(Attach)` grant lives inside a composite
        // keyword (the Enchant macro shape: a `Keyword(Composite{[Static(..)]})`).
        let enchant_composite = Ability::Keyword(KeywordAbility::Composite {
            name: "Enchant".into(),
            abilities: vec![Ability::Static(StaticEffect::Deontic(Deontic::May(
                DeonticAction::Attach {
                    what: Predicate::Ref(Reference::This),
                    to: creature(),
                },
            )))],
        });
        let aura = obj_on_field(
            &mut state,
            "Composite Aura",
            vec![Type::Enchantment],
            vec![enchant_composite],
        );
        let creature_host = obj_on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        let noncreature_host = obj_on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);

        assert!(
            attachment_legal(&state, aura, creature_host),
            "composite-conferred May(Attach to: Creature) allows a creature host \
             — statics_on must flatten the composite keyword"
        );
        assert!(
            !attachment_legal(&state, aura, noncreature_host),
            "composite-conferred May(Attach to: Creature) does not cover a non-creature host \
             ([CR#702.5a])"
        );
    }

    /// [CR#701.6a]: `counter_legal` reads the target's OWN
    /// `Cant(Counter(on: Ref(This)))` — "this spell can't be countered" — and
    /// refuses the counter (`false`), while a plain object without the row can
    /// be countered (`true`). `Ref(This)` on the row anchors to the object
    /// carrying it, so the self-referential form matches exactly its own
    /// carrier.
    #[test]
    fn counter_legal_honors_self_referential_cant() {
        use super::counter_legal;
        let mut state = game();
        let uncounterable = obj_on_field(
            &mut state,
            "Uncounterable",
            vec![Type::Instant],
            vec![Ability::Static(StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Counter {
                    by: Predicate::Any,
                    on: Predicate::Ref(Reference::This),
                },
            )))],
        );
        let plain = obj_on_field(&mut state, "Plain", vec![Type::Instant], vec![]);
        // The countering source — any object; the `by: Any` agent matches it.
        let source = obj_on_field(&mut state, "Counterspell", vec![Type::Instant], vec![]);

        assert!(
            !counter_legal(&state, source, uncounterable),
            "a `Cant(Counter(on: Ref(This)))` object can't be countered"
        );
        assert!(
            counter_legal(&state, source, plain),
            "an object with no can't-be-countered row can be countered"
        );
    }

    // --- land-play (May(Play) capability) -----------------------------------

    /// A Land `TypeDef` carrying the conferred `May(Play(what: Ref(This)))`
    /// marker inline. In real games the plugin registry attaches this confer;
    /// a bare `Type::Land.def()` has EMPTY confers (decision 6), so fixtures
    /// exercising the capability build it here (mirrors `cast::tests`'
    /// `instant_typedef`).
    fn land_typedef() -> deckmaste_core::TypeDef {
        deckmaste_core::TypeDef {
            name: "Land".into(),
            permanent: true,
            confers: vec![deckmaste_core::Property::Ability(Box::new(
                Ability::Static(StaticEffect::Deontic(Deontic::May(DeonticAction::Play {
                    what: Predicate::Ref(Reference::This),
                    by: Predicate::Any,
                    from: None,
                }))),
            ))],
        }
    }

    /// Mint a battlefield land whose Land type CONFERS `May(Play)` — the
    /// production shape (the confer rides `face.types`, folded into the derived
    /// abilities by `printed_of_face`), unlike `obj_on_field` which uses the
    /// empty-confer `Type::def`.
    fn conferred_land_on_field(state: &mut GameState, name: &str) -> ObjectId {
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        let card = Card::Normal(CardFace {
            name: name.into(),
            types: vec![land_typedef()],
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    /// The capability reader sees the conferred `May(Play)` marker on a land
    /// and nothing on a permanent that lacks it ([CR#305.9,116.2a,701.18])
    /// — the grant `legal.rs`/`cast.rs` key land-play +
    /// spell-non-castability on.
    #[test]
    fn confers_may_play_reads_the_conferred_marker() {
        let mut state = game();
        let land = conferred_land_on_field(&mut state, "Mountain");
        // A grantless permanent (an artifact via the empty-confer `def()`).
        let rock = obj_on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);
        let view = state.layers();
        assert!(
            super::confers_may_play(&state, &view, land),
            "a Land's conferred May(Play) marker is seen ([CR#701.18])"
        );
        assert!(
            !super::confers_may_play(&state, &view, rock),
            "a permanent with no May(Play) confer is not land-playable (default-deny)"
        );
    }

    /// decision 4 regression (the guard landmine): a land on the battlefield
    /// now carries a `May(Play(from: None))` row via its type confer. The
    /// guard's `Play` arm is narrowed to EVALUATE exactly that shape as
    /// land-play legality, so computing legal actions with a land in play
    /// must NOT hit the `guard_deontic_seam` `todo!()`. Before the
    /// narrowing this panics.
    #[test]
    fn battlefield_land_with_conferred_may_play_does_not_trip_guard() {
        let mut state = game();
        let _land = conferred_land_on_field(&mut state, "Mountain");
        // Pure computation; the guard scans the battlefield unconditionally.
        let legal = super::legal_actions(&state, PlayerId(0));
        assert!(
            legal.contains(&crate::decide::Action::Pass),
            "the priority window still enumerates Pass with a land in play"
        );
    }

    // --- Conditionally gating in the deontic-collector walk -----------------

    /// [CR#611.3a]: `for_each_static` GATES a `Conditionally` static — a row
    /// collector sees the inner effect only when the condition holds, no longer
    /// looking through unconditionally. Uses a pure `Compare` condition (no
    /// characteristics), so the gate is exercised independently of combat and
    /// of any `layers()` re-entry.
    #[test]
    fn conditionally_gates_the_cant_attack_row() {
        use deckmaste_core::Cmp;
        use deckmaste_core::Condition;
        use deckmaste_core::Count;
        // The inner `Cant(Attack)` static the `Conditionally` wraps.
        let cant = || {
            StaticEffect::Deontic(Deontic::Cant(DeonticAction::Attack {
                by: Predicate::Ref(Reference::This),
                on: Predicate::Any,
            }))
        };

        // A false condition (`0 > 1`) ⇒ the inner row is NOT collected.
        let mut off = game();
        obj_on_field(
            &mut off,
            "Off",
            vec![Type::Creature],
            vec![Ability::Static(StaticEffect::Conditionally(
                Condition::Compare(Count::Literal(0), Cmp::Greater, Count::Literal(1)),
                Box::new(cant()),
            ))],
        );
        let v = off.layers();
        assert!(
            super::cant_attack_rows(&off, &v).is_empty(),
            "false condition ⇒ no Cant(Attack) row (the gate suppresses the inner effect)"
        );

        // A true condition (`1 >= 1`) ⇒ exactly the inner row is collected.
        let mut on = game();
        obj_on_field(
            &mut on,
            "On",
            vec![Type::Creature],
            vec![Ability::Static(StaticEffect::Conditionally(
                Condition::Compare(Count::Literal(1), Cmp::AtLeast, Count::Literal(1)),
                Box::new(cant()),
            ))],
        );
        let v = on.layers();
        assert_eq!(
            super::cant_attack_rows(&on, &v).len(),
            1,
            "true condition ⇒ exactly the inner Cant(Attack) row is collected"
        );
    }

    // --- combatant capability (May(Attack)/May(Block) type confer) ----------

    /// A `Creature` `TypeDef` carrying the four combat confers inline — the two
    /// `May(Attack)`/`May(Block)` grants and the summoning-sickness `Cant`-pair
    /// (attack + tap-activate), each `Cant` gated `Conditionally` on
    /// `SummoningSick && !Has(Haste)`. In real games the plugin registry
    /// attaches these via `Creature.ron`; a bare `Type::Creature.def()` has
    /// EMPTY confers (decision 6), so fixtures exercising the combat capability
    /// build the confers here (mirrors `land_typedef`).
    fn creature_typedef() -> deckmaste_core::TypeDef {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::Condition;
        use deckmaste_core::CostPredicate;
        use deckmaste_core::Property;
        use deckmaste_core::StatePredicate;
        let sick_not_hasty = || {
            Condition::And(vec![
                Condition::Matches(
                    Reference::This,
                    Predicate::State(StatePredicate::SummoningSick),
                ),
                Condition::Not(Box::new(Condition::Matches(
                    Reference::This,
                    Predicate::Characteristic(CharacteristicPredicate::Has("Haste".into())),
                ))),
            ])
        };
        let ability = |s: StaticEffect| Property::Ability(Box::new(Ability::Static(s)));
        deckmaste_core::TypeDef {
            name: "Creature".into(),
            permanent: true,
            confers: vec![
                ability(StaticEffect::Deontic(Deontic::May(DeonticAction::Attack {
                    by: Predicate::Ref(Reference::This),
                    on: Predicate::Any,
                }))),
                ability(StaticEffect::Deontic(Deontic::May(DeonticAction::Block {
                    by: Predicate::Ref(Reference::This),
                    on: Predicate::Any,
                    count: None,
                }))),
                ability(StaticEffect::Conditionally(
                    sick_not_hasty(),
                    Box::new(StaticEffect::Deontic(Deontic::Cant(
                        DeonticAction::Attack {
                            by: Predicate::Ref(Reference::This),
                            on: Predicate::Any,
                        },
                    ))),
                )),
                ability(StaticEffect::Conditionally(
                    sick_not_hasty(),
                    Box::new(StaticEffect::Deontic(Deontic::Cant(
                        DeonticAction::Activate {
                            what: Predicate::Ref(Reference::This),
                            by: Predicate::Any,
                            cost: Some(CostPredicate::IncludesTapSymbol),
                        },
                    ))),
                )),
            ],
        }
    }

    /// Mint a battlefield creature (player 0) whose `Creature` type CONFERS the
    /// four combat statics — the production shape (the confer rides
    /// `face.types`, folded into the derived abilities by `printed_of_face`),
    /// unlike `obj_on_field`'s empty-confer `Type::def`. `sick` seeds the
    /// summoning-sickness flag; `extra` are printed abilities carried IN
    /// ADDITION to the type confers (a `Haste` keyword, an extra plain
    /// `Cant(Attack)`, …).
    fn conferred_creature_full(
        state: &mut GameState,
        name: &str,
        sick: bool,
        extra: Vec<Ability>,
    ) -> ObjectId {
        use deckmaste_core::Card;
        use deckmaste_core::CardFace;
        let card = Card::Normal(CardFace {
            name: name.into(),
            types: vec![creature_typedef()],
            abilities: extra,
            ..CardFace::default()
        });
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.objects.obj_mut(id).summoning_sick = sick;
        state.zones.battlefield.push(id);
        id
    }

    /// A conferred creature with no extra printed abilities.
    fn conferred_creature_on_field(state: &mut GameState, name: &str, sick: bool) -> ObjectId {
        conferred_creature_full(state, name, sick, vec![])
    }

    /// [CR#508.1a,509.1a]: a non-sick conferred creature is a legal attacker AND
    /// a legal blocker — its `Creature` type grants `May(Attack)`/`May(Block)`
    /// — while a permanent with NO combat confer (a plain artifact) is neither.
    /// Combat capability is default-deny, turned on by the type's grant.
    #[test]
    fn conferred_creature_attacks_and_blocks_noncreature_neither() {
        let mut state = game();
        let bear = conferred_creature_on_field(&mut state, "Bear", false);
        let rock = obj_on_field(&mut state, "Rock", vec![Type::Artifact], vec![]);
        let attackers = super::legal_attackers(&state, PlayerId(0));
        let blockers = super::legal_blockers(&state, PlayerId(0));
        assert!(
            attackers.contains(&bear),
            "a conferred creature may attack ([CR#508.1a])"
        );
        assert!(
            blockers.contains(&bear),
            "a conferred creature may block ([CR#509.1a])"
        );
        assert!(
            !attackers.contains(&rock),
            "a non-creature has no May(Attack) grant — default-deny"
        );
        assert!(
            !blockers.contains(&rock),
            "a non-creature has no May(Block) grant — default-deny"
        );
    }

    /// [CR#302.6,702.10b]: a summoning-sick conferred creature is NOT a legal
    /// attacker — the type's `Conditionally(SummoningSick & !Haste,
    /// Cant(Attack))` confer subtracts it — but IS a legal blocker
    /// (sickness never gates blocking, [CR#509.1a]). Granting the SAME
    /// creature `Haste` lifts the sickness `Cant(Attack)` (the condition's
    /// `Not(Has(Haste))` fails), so it becomes a legal attacker.
    #[test]
    fn sick_creature_no_attack_yes_block_haste_lifts_it() {
        let mut state = game();
        let sick = conferred_creature_on_field(&mut state, "Sick Bear", true);
        assert!(
            !super::legal_attackers(&state, PlayerId(0)).contains(&sick),
            "a summoning-sick creature can't attack ([CR#302.6])"
        );
        assert!(
            super::legal_blockers(&state, PlayerId(0)).contains(&sick),
            "a summoning-sick creature can still block ([CR#509.1a])"
        );

        // The SAME sick creature WITH Haste: the sickness Cant(Attack)
        // condition's `Not(Has(Haste))` fails, so the Cant is not contributed —
        // it becomes a legal attacker ([CR#702.10b]).
        let mut hasty_state = game();
        let hasty = conferred_creature_full(
            &mut hasty_state,
            "Hasty Bear",
            true,
            vec![Ability::Keyword(KeywordAbility::Composite {
                name: "Haste".into(),
                abilities: vec![],
            })],
        );
        assert!(
            super::legal_attackers(&hasty_state, PlayerId(0)).contains(&hasty),
            "Haste lifts summoning sickness — a hasty sick creature attacks ([CR#702.10b])"
        );
    }

    /// [CR#120.3d,120.3e]: `is_combatant` reads GRANT PRESENCE, not net attack
    /// eligibility. A conferred creature carrying an extra plain `Cant(Attack)`
    /// still HAS the `May(Attack)` grant, so `is_combatant` is true — its
    /// combat damage is marked ([CR#120.3d]) — even though the `Cant`
    /// removes it from `legal_attackers`.
    #[test]
    fn is_combatant_reads_grant_presence_not_net_eligibility() {
        let mut state = game();
        let bear = conferred_creature_full(
            &mut state,
            "Barred Bear",
            false,
            vec![Ability::Static(StaticEffect::Deontic(Deontic::Cant(
                DeonticAction::Attack {
                    by: Predicate::Ref(Reference::This),
                    on: Predicate::Any,
                },
            )))],
        );
        let view = state.layers();
        assert!(
            super::is_combatant(&state, &view, bear),
            "the May(Attack) grant is present ⇒ a combatant whose damage is marked"
        );
        assert!(
            !super::legal_attackers(&state, PlayerId(0)).contains(&bear),
            "the extra plain Cant(Attack) removes it from legal attackers"
        );
    }

    /// [CR#602.5a,602.1]: a summoning-sick conferred creature can't activate a
    /// tap ability — its type's `Conditionally(SummoningSick & !Haste,
    /// Cant(Activate(cost: IncludesTapSymbol)))` confer forbids it — but a
    /// NON-tap ability is free (the `cost` predicate doesn't match). Granting
    /// the same creature `Haste` lifts the `Cant` (the `Not(Has(Haste))`
    /// fails).
    #[test]
    fn sick_creature_tap_gate_but_free_non_tap() {
        let mut state = game();
        let sick = conferred_creature_on_field(&mut state, "Sick Bear", true);
        let view = state.layers();
        assert!(
            super::cant_activate(&state, &view, sick, PlayerId(0), true),
            "a summoning-sick creature can't pay a {{T}} cost ([CR#602.5a])"
        );
        assert!(
            !super::cant_activate(&state, &view, sick, PlayerId(0), false),
            "a non-tap ability is not gated by summoning sickness ([CR#602.5a])"
        );

        // A non-sick conferred creature: the sickness condition fails, so no
        // Cant(Activate) is contributed — even a {T} cost is free.
        let mut ready_state = game();
        let ready = conferred_creature_on_field(&mut ready_state, "Ready Bear", false);
        let ready_view = ready_state.layers();
        assert!(
            !super::cant_activate(&ready_state, &ready_view, ready, PlayerId(0), true),
            "a non-sick creature freely taps ([CR#602.5a])"
        );

        // The SAME sick creature WITH Haste: `Not(Has(Haste))` fails, so the
        // Cant(Activate) is not contributed — the {T} cost is free.
        let mut hasty_state = game();
        let hasty = conferred_creature_full(
            &mut hasty_state,
            "Hasty Bear",
            true,
            vec![Ability::Keyword(KeywordAbility::Composite {
                name: "Haste".into(),
                abilities: vec![],
            })],
        );
        let hasty_view = hasty_state.layers();
        assert!(
            !super::cant_activate(&hasty_state, &hasty_view, hasty, PlayerId(0), true),
            "Haste lifts summoning sickness — a hasty sick creature taps ([CR#702.10c])"
        );
    }
}
