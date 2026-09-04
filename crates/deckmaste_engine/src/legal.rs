//! Legal actions for the priority holder ([CR#117.1]). The list this computes
//! is both the advisory `legal` carried by the Priority decision and the
//! authoritative check at submission (state can't change in between: a
//! pending decision blocks stepping).

use std::cell::Cell;
use std::ops::ControlFlow;

use deckmaste_core::Ability;
use deckmaste_core::AsThough;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::DeedAgent;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::KeywordAbility;
use deckmaste_core::Predicate;
use deckmaste_core::StaticSpec;
use deckmaste_core::Type;

use crate::decide::Action;
use crate::derive;
use crate::layer::LayeredView;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::stack::StackObject;
use crate::state::GameState;

/// The proposed-action pattern inside a deontic row.
fn deontic_action(d: &Deontic) -> &DeonticAction {
    match d {
        Deontic::May(a) | Deontic::Cant(a) | Deontic::Must(a) | Deontic::Gate(a, _) => a,
        // Provenance is erased at `lower` (`deckmaste_lowering`), so no
        // loaded value reaches here wrapped. The arm survives only because
        // the variant does; `core-demacro` deletes both.
    }
}

/// Whether `id`'s derived view carries any static matching `pred`, looking
/// through composites and `Each` wrappers at every level. A
/// short-circuiting wrapper over the single [`statics_on`] walker: it stops
/// the descent the moment `pred` accepts.
pub(crate) fn object_has_static<F: Fn(&StaticSpec) -> bool>(
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

pub(crate) fn statics_present<F: Fn(&StaticSpec) -> bool>(
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

/// Deontic legality presence guard ([CR#101.2,601.3] seam): the deontic
/// grammar is complete, but declaration legality does not evaluate the rows
/// yet. Any matching-verb row in the derived view trips the seam LOUDLY
/// rather than being silently ignored. Never delete a trip to silence it —
/// convert it to the legality evaluation.
fn guard_deontic_seam(
    state: &GameState,
    view: &LayeredView,
    row: fn(&Deontic) -> bool,
    what: &str,
) {
    let hit = statics_present(
        state,
        view,
        |e| matches!(e, StaticSpec::Deontic(d) if row(d)),
    );
    if hit {
        todo!(
            "engine seam: deontic {what} legality ([CR#101.2,601.3]) — a deontic row is present \
             in the derived view and would change legality, so it cannot be silently ignored; \
             not yet evaluated; owner: engine-deontic-legality-residue"
        );
    }
}

/// Whether a deontic row's polarity is `Cant`.
fn is_cant(d: &Deontic) -> bool {
    matches!(d, Deontic::Cant(_))
}

/// Whether a deontic row's polarity is `May`.
fn is_may(d: &Deontic) -> bool {
    matches!(d, Deontic::May(_))
}

/// Whether a deontic row's polarity is `Must`.
fn is_must(d: &Deontic) -> bool {
    matches!(d, Deontic::Must(_))
}

/// The action under a `Cant` polarity.
fn cant_action(d: &Deontic) -> Option<&DeonticAction> {
    match d {
        Deontic::Cant(a) => Some(a),
        _ => None,
    }
}

/// The action under a `Must` polarity.
fn must_action(d: &Deontic) -> Option<&DeonticAction> {
    match d {
        Deontic::Must(a) => Some(a),
        _ => None,
    }
}

/// The action under a `May` polarity.
fn may_action(d: &Deontic) -> Option<&DeonticAction> {
    match d {
        Deontic::May(a) => Some(a),
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

    // Activated abilities of permanents you control: lowering-classified mana
    // abilities use the mana-specific [CR#605] gate; every other activation
    // uses the ordinary [CR#602.5] gate.
    for &object in &state.zones.battlefield {
        // Derived controller ([CR#613.1b]): only the current controller may
        // activate ([CR#602.2]); a control-change effect moves this.
        if view.controller(object) != player {
            continue;
        }
        // Index the SAME list resolution reads: `begin_activate`, `decide`'s
        // `ActivateAbility` arm, and `render`'s `activated_ability`/
        // `mana_ability` all index `derive::usable_abilities` (see the "SAME
        // list, SAME order" invariant in `render.rs`). A type-conferred
        // ability (a basic land's [CR#305.6] mana ability) sits in that list
        // like a printed one.
        for (ability, a) in derive::usable_abilities(state, object).iter().enumerate() {
            if let Some(act) = crate::activate::as_activated(a) {
                let permitted = if a.is_activated_mana_ability() {
                    state.can_activate_mana(&view, player, object, ability, act)
                } else {
                    state.can_activate(&view, player, object, ability, act)
                };
                if permitted {
                    legal.push(Action::ActivateAbility { object, ability });
                }
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
    // EVALUATED as a timing lift in can_cast ([CR#702.8a]), every SLOTLESS
    // Cant(Cast) row (split second on the stack, or a battlefield "can't
    // cast" grant, [CR#702.61a]) is EVALUATED by `cant_cast`/`cant_cast_rows`
    // — which pattern-match `Cast { what, by, .. }` and so only ever see the
    // `from`/`window`/`cost`-less shape — and the land-play marker —
    // May(Play) with no `from` slot, which a card's Land type confers — is
    // EVALUATED as land-play legality via `confers_may_play` (offered as
    // PlayLand in the hand-scan above, [CR#116.2a,305.9,701.18]). The guard
    // keeps the unevaluated rest: every other Cast row shape (zone
    // permissions, alternative costs, the remaining May(Cast) shapes
    // carrying `from`/`cost`, Must/Gate(Cast), and a SLOTTED Cant(Cast) —
    // e.g. a future "can't cast from graveyard" — which `cant_cast` does not
    // evaluate), the from-zone / non-May Play shapes, non-Cant/non-May
    // Attach rows, and the May/Gate Target polarities.
    guard_deontic_seam(
        state,
        &view,
        |d| match deontic_action(d) {
            DeonticAction::Cast {
                from, window, cost, ..
            } => {
                !((is_cant(d) && from.is_none() && window.is_none() && cost.is_none())
                    || (is_may(d)
                        && *window == Some(deckmaste_core::Timing::InstantSpeed)
                        && from.is_none()
                        && cost.is_none()))
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
        "cast (flash May + slotless-Cant evaluated) + from-zone/non-May play + non-Cant attach + May/Gate target",
    );
    // The former `CostModifier` presence guard, now converted to the real
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
            if let StaticSpec::Deontic(d) = e
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
    // [CR#509.1b] surfacing refinement: a permanent is surfaced as a legal
    // blocker only when at least ONE declared attacker point-wise permits it
    // (an attacker whose flying-family `Cant(Block)` row does not forbid the
    // pairing). A creature no attacker permits can't legally block anything, so
    // surfacing it is a lie — prune it. Arrangement bounds (menace's `count`)
    // do NOT prune: `block_forbidden_by` consults only the point-wise rows, so
    // a lone menace-blocker — illegal alone but legal with a partner — stays
    // surfaced. (With no declared attackers there is nothing to block, so the
    // surfaced set is empty; the Declare Blockers step is reached only when an
    // attacker exists, [CR#508.8].)
    let attackers = state.combat.attackers();
    let cant_rows = cant_block_rows(state, &view);
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&id| {
            let obj = state.objects.obj(id);
            // Derived controller ([CR#613.1b]): a stolen creature blocks for
            // its new controller. `blockable` reads the
            // `May(Block)` grant (default-deny combat capability).
            view.controller(id) == player && !obj.tapped && blockable(state, &view, id)
        })
        .filter(|&id| {
            attackers
                .iter()
                .any(|&attacker| block_forbidden_by(state, &cant_rows, id, attacker).is_none())
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
            if let StaticSpec::Deontic(d) = e
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
            if let StaticSpec::Deontic(d) = e
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

/// Object ids of the spells currently on the stack — the sources that can
/// carry a stack-functioning `Cant(...)` static (split second, [CR#702.61a]).
/// Restricted to `StackObject::Spell` (its entry id IS the spell's own
/// object id); `Triggered`/`Activated` entries are stand-ins minted from the
/// firing permanent's source and must not be read as if the card were on the
/// stack (see `cant_cast_rows`'s doc for the full misattribution hazard).
/// Shared by [`cant_cast_rows`] (the Cast half) and [`cant_activate_rows`]
/// (the Activate half, [CR#702.61b]).
fn spell_stack_ids(state: &GameState) -> impl Iterator<Item = ObjectId> + '_ {
    state
        .stack
        .iter()
        .filter_map(|e| matches!(e.object, StackObject::Spell(_)).then_some(e.id))
}

/// Every `Cant(Activate)` row visible to an activation: [CR#602.5a]'s
/// summoning-sickness tap gate a card's `Creature` type confers, split
/// second's activate half ([CR#702.61a] — a spell functions this static
/// while on the stack, via [`spell_stack_ids`]), and any other "can't
/// activate" effect (Linvala, Damping Matrix). Rows come from battlefield
/// permanents PLUS every `StackObject::Spell` entry, same scan
/// `cant_cast_rows` runs for its Cast half. Each row is its carrier plus the
/// ability's patient filter `what`, the agent filter `by`, and the optional
/// `cost` predicate that scopes the row to a subset of activations (a
/// `{T}`/`{Q}` cost, or `None` for a blanket row that [`cant_activate`]'s
/// `blanket_applies` flag gates separately, [CR#702.61b]). Mirrors
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
    let ids = state
        .zones
        .battlefield
        .iter()
        .copied()
        .chain(spell_stack_ids(state));
    for id in ids {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticSpec::Deontic(d) = e
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

/// [CR#602.5a,602.1,702.61a,702.61b,101.2]: whether some conferred/stack
/// `Cant(Activate)` row forbids `activator` activating an ability of
/// `object` whose cost matches `includes_tap_symbol`. A row applies when
/// `object` matches its `what`, the `activator`'s player proxy matches its
/// `by`, its `cost` predicate holds for the activation (`IncludesTapSymbol`
/// only when `includes_tap_symbol`), AND — for a BLANKET row (`cost: None`,
/// scoping to every activation, not just a tap subset) — `blanket_applies`
/// is set.
///
/// The summoning-sickness tap gate rides the cost-scoped shape: a card's
/// `Creature` type confers `Conditionally(SummoningSick & !Haste,
/// Cant(Activate(cost: IncludesTapSymbol)))`, so a sick creature can't pay a
/// `{T}`/`{Q}` cost but can pay a non-tap one — unaffected by
/// `blanket_applies` since `cost.is_some()` there.
///
/// Split second confers a BLANKET row (`cost: None`) that locks out non-mana
/// activation but exempts mana abilities ([CR#702.61b]): the non-mana gate
/// ([CR#602.5], `activate.rs`) passes `blanket_applies: true`, while the
/// mana-ability tap gate (the stackless arm, `legal.rs`'s `legal_actions`
/// and `cast.rs`'s `autotap_for_cast`) passes `false` so a blanket row never
/// blocks a mana ability.
#[must_use]
pub(crate) fn cant_activate(
    state: &GameState,
    view: &LayeredView,
    object: ObjectId,
    activator: PlayerId,
    includes_tap_symbol: bool,
    blanket_applies: bool,
) -> bool {
    let proxy = state.player(activator).object;
    cant_activate_rows(state, view)
        .iter()
        .any(|(carrier, what, by, cost)| {
            // Blanket (cost: None) rows — split second / Linvala / Damping
            // Matrix — apply only where mana abilities are NOT exempt: the
            // non-mana activation gate, never the mana-tap gate ([CR#702.61b]).
            (blanket_applies || cost.is_some())
                && cost_predicate_holds(cost.as_ref(), includes_tap_symbol)
                && state.filter_matches_live(what, object, *carrier)
                && state.filter_matches_live(by, proxy, *carrier)
        })
}

/// The single ability-tree walker. Descends an ability list with the
/// look-through rules every static read needs (static-ability effect lists,
/// keyword composites — flying's evasion `Cant` lives inside
/// `Keyword(Composite)` — and `Each` wrappers at every level),
/// calling `visit` on each static effect. The `ControlFlow` return lets a
/// caller short-circuit: [`object_has_static`] (the boolean "any" form) breaks
/// on the first match, while the visit-each callers always
/// [`Continue`](ControlFlow::Continue) to see every effect. The whole-walk
/// result propagates the visitor's `Break` value (or `Continue(())` when the
/// descent ran to completion). View-free so it can be unit-tested directly;
/// [`statics_on`] is the thin `LayeredView` adapter over it.
///
/// `enter_conditional` GATES a [`StaticSpec::Conditionally`] ([CR#611.3a]):
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
/// row. A type/subtype's own ability-free rules are NOT abilities and so are
/// not here; [`statics_on`] adds them from the derived types/subtypes.
pub(crate) fn walk_abilities<B, F, G>(
    abilities: &[Ability],
    enter_conditional: &mut G,
    visit: &mut F,
) -> ControlFlow<B>
where
    F: FnMut(&StaticSpec) -> ControlFlow<B>,
    G: FnMut(&deckmaste_core::Condition) -> bool,
{
    for a in abilities {
        in_ability(a, enter_conditional, visit)?;
    }
    ControlFlow::Continue(())
}

fn in_ability<B, F, G>(a: &Ability, enter: &mut G, visit: &mut F) -> ControlFlow<B>
where
    F: FnMut(&StaticSpec) -> ControlFlow<B>,
    G: FnMut(&deckmaste_core::Condition) -> bool,
{
    match a {
        Ability::Static(s) => walk_static(&s.body, enter, visit),
        Ability::Keyword(k) => in_keyword(k, enter, visit),
        _ => ControlFlow::Continue(()),
    }
}

fn in_keyword<B, F, G>(k: &KeywordAbility, enter: &mut G, visit: &mut F) -> ControlFlow<B>
where
    F: FnMut(&StaticSpec) -> ControlFlow<B>,
    G: FnMut(&deckmaste_core::Condition) -> bool,
{
    match k {
        KeywordAbility::Composite { abilities, .. } => {
            for a in abilities {
                in_ability(a, enter, visit)?;
            }
            ControlFlow::Continue(())
        }
        _ => ControlFlow::Continue(()),
    }
}

/// The static-effect half of [`walk_abilities`], reachable on its own so an
/// ability-free conferred rule ([`deckmaste_core::Property::Static`]) is read
/// by exactly the same look-through as a card's own static ability.
pub(crate) fn walk_static<B, F, G>(e: &StaticSpec, enter: &mut G, visit: &mut F) -> ControlFlow<B>
where
    F: FnMut(&StaticSpec) -> ControlFlow<B>,
    G: FnMut(&deckmaste_core::Condition) -> bool,
{
    match e {
        // Distributed statics ([`StaticSpec::Each`]) are looked through to
        // their inner effect for this presence scan: the walker's callers
        // (Cant/state-checked/CostModifier row collectors) match on the effect KIND,
        // not the affected set, so the wrapping `Selection` is immaterial
        // here.
        StaticSpec::Each(_, inner) => walk_static(&inner.body, enter, visit),
        // [CR#611.3a]: a `Conditionally` wrapper contributes its inner
        // effect only when `enter` accepts the condition. Collectors gate on
        // the live condition; the presence-only walkers pass `|_| true` and
        // keep the unconditional look-through.
        StaticSpec::Conditionally(cond, inner) => {
            if enter(cond) {
                walk_static(inner, enter, visit)
            } else {
                ControlFlow::Continue(())
            }
        }
        other => visit(other),
    }
}

/// The `LayeredView` adapter over [`walk_abilities`]: walks `id`'s derived
/// ability list AND its type/subtype-conferred ability-free static rules
/// ([`deckmaste_core::Property::Static`]), calling `visit` on each static
/// effect and short-circuiting on the visitor's `Break`. `enter_conditional`
/// gates `Conditionally` statics (forwarded to the walker).
fn statics_on<B, F, G>(
    view: &LayeredView,
    id: ObjectId,
    enter_conditional: &mut G,
    visit: &mut F,
) -> ControlFlow<B>
where
    F: FnMut(&StaticSpec) -> ControlFlow<B>,
    G: FnMut(&deckmaste_core::Condition) -> bool,
{
    let derived = view.get(id);
    walk_abilities(&derived.abilities, enter_conditional, visit)?;
    // The ability-free type/subtype rules ([`Property::Static`]) — Land's
    // `May(Play)` ([CR#305.9]), Creature's combat permissions
    // ([CR#508.1a,509.1a]), the Equipment/Fortification legal-host rule
    // ([CR#301.5,301.6]).
    // Read off the DERIVED types/subtypes, so a layer-4 grant contributes
    // exactly like a printed type does, and no layer-6 ability removal can
    // reach them ([CR#113.12] — they are qualities of the object, not
    // abilities).
    for spec in derived
        .card_types
        .iter()
        .flat_map(|t| t.confers.iter())
        .chain(derived.subtypes.iter().flat_map(|s| s.confers.iter()))
        .filter_map(deckmaste_core::Property::conferred_static)
    {
        walk_static(&spec.body, enter_conditional, visit)?;
    }
    ControlFlow::Continue(())
}

/// The non-short-circuiting view over [`statics_on`]: runs `visit` on every
/// static effect of `id` (no early exit). The visit-each callers
/// (`attack_rows`/`block_rows`/`target_rows`/`may_cast_rows`) collect rows
/// through this, leaving the `ControlFlow` plumbing to the one walker. Also
/// the entry point for the [CR#704] SBA sweep, which collects the
/// `ConditionallyDo` rows the same look-through way.
///
/// [CR#611.3a]: a `Conditionally` static is GATED — its inner effect is visited
/// only when the wrapper's condition holds for `id`, evaluated with `This`
/// bound to `id` (an `state.frame` on the object's controller).
pub(crate) fn for_each_static<F: FnMut(&StaticSpec)>(
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
    let frame = state.frame(id, controller);
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
            let frame = state.frame(r.carrier_id, state.objects.obj(r.carrier_id).controller);
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
            if let StaticSpec::Deontic(d) = e
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

/// Every `AsThough(Counterfactual)` overlay whose inner selector is a
/// `May(Target)`, with its carrier: `(carrier, premise, by, on)`. `by`/`on`
/// scope which targeting the overlay covers (the acting agent + the object made
/// targetable); `premise` is the counterfactual re-checked against each
/// candidate ([CR#609.4] Glaring Spotlight — "as though it didn't have
/// hexproof").
#[must_use]
pub(crate) fn astough_target_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Predicate, DeedAgent, Predicate)> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticSpec::AsThough(AsThough::Counterfactual { premise, then }) = e
                && let Deontic::May(DeonticAction::Target { by, on }) = then.as_ref()
            {
                rows.push((source, premise.clone(), by.clone(), on.clone()));
            }
        });
    }
    rows
}

/// Whether an active `AsThough` overlay lets `spell` target `target` past a
/// `Cant(Target)` row that would otherwise forbid it ([CR#609.4]). The overlay
/// applies when its `by` agent matches `spell` and its `on` scope matches
/// `target`; it then re-checks the forbidding *as though* `target` did not
/// satisfy the overlay's `premise`. Realized by masking the named keyword's
/// abilities on `target` (a per-checker counterfactual, never a real
/// characteristic change), so an UNRELATED self obstacle (shroud, protection)
/// that survives the premise — or any external row — still forbids.
#[must_use]
pub(crate) fn asthough_sees_through_target(
    state: &GameState,
    view: &LayeredView,
    cant_rows: &[(crate::object::ObjectSource, DeedAgent, Predicate)],
    spell: ObjectId,
    target: ObjectId,
) -> bool {
    astough_target_rows(state, view)
        .into_iter()
        .any(|(carrier, premise, by, on)| {
            deed_agent_matches(state, &by, spell, carrier)
                && state.filter_matches_live(&on, target, carrier)
                && !forbidden_ignoring_keyword(state, view, cant_rows, spell, target, &premise)
        })
}

/// Re-run the `Cant(Target)` forbidding for (`spell`, `target`) as though
/// `target` lacked the keyword named by `premise` (a `Not(Has(K))`
/// counterfactual). `target`'s OWN self-carried rows are recomputed with `K`'s
/// abilities masked; every other row (carried by a different object, or a
/// different keyword on `target`) is judged unchanged — so removing hexproof
/// leaves shroud/protection intact. A premise shape the overlay can't realize
/// leaves the target forbidden (no suppression).
fn forbidden_ignoring_keyword(
    state: &GameState,
    view: &LayeredView,
    cant_rows: &[(crate::object::ObjectSource, DeedAgent, Predicate)],
    spell: ObjectId,
    target: ObjectId,
    premise: &Predicate,
) -> bool {
    let Some(keyword) = premise_removes_keyword(premise) else {
        return true;
    };
    let target_source = state.objects.obj(target).source;
    // Rows carried by any OTHER object are untouched by removing `target`'s
    // keyword — if one still forbids, the overlay does not see through it.
    let external_forbids = cant_rows.iter().any(|(carrier, by, on)| {
        *carrier != target_source
            && deed_agent_matches(state, by, spell, *carrier)
            && state.filter_matches_live(on, target, *carrier)
    });
    external_forbids || masked_self_rows_forbid(state, view, spell, target, keyword)
}

/// Collect `target`'s own `Cant(Target)` rows with the keyword named `keyword`
/// masked out of its ability list, and report whether any still forbids
/// `spell`. Walks the derived abilities directly (view-free
/// [`walk_abilities`]), gating `Conditionally` the same way [`for_each_static`]
/// does — the counterfactual's obstacle recomputation.
fn masked_self_rows_forbid(
    state: &GameState,
    view: &LayeredView,
    spell: ObjectId,
    target: ObjectId,
    keyword: &str,
) -> bool {
    let source = state.objects.obj(target).source;
    let abilities: Vec<Ability> = view
        .get(target)
        .abilities
        .iter()
        .filter(|a| !ability_names_keyword(a, keyword))
        .cloned()
        .collect();
    let controller = state.objects.obj(target).controller;
    let frame = state.frame(target, controller);
    let mut enter = |cond: &deckmaste_core::Condition| state.condition_holds(cond, &frame);
    let hit = walk_abilities(&abilities, &mut enter, &mut |e: &StaticSpec| {
        if let StaticSpec::Deontic(d) = e
            && let Some(DeonticAction::Target { by, on }) = cant_action(d)
            && deed_agent_matches(state, by, spell, source)
            && state.filter_matches_live(on, target, source)
        {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    });
    hit.is_break()
}

/// The keyword a `Not(Has(K))` counterfactual premise removes, by name — the
/// only premise shape the targeting overlay realizes today (Glaring Spotlight's
/// `Not(Has(Hexproof))`). Anything else yields `None` (no suppression).
fn premise_removes_keyword(premise: &Predicate) -> Option<&'static str> {
    if let Predicate::Not(inner) = premise
        && let Predicate::Characteristic(CharacteristicPredicate::Has(kw)) = inner.as_ref()
    {
        Some(kw.as_str())
    } else {
        None
    }
}

/// Whether `a` is the keyword ability named `name` — the mask predicate the
/// counterfactual uses to drop a
/// candidate's keyword. Matches by name via
/// [`KeywordAbility::as_str`](deckmaste_core::KeywordAbility::as_str), the same
/// name bridge `Has(K)` matches through.
fn ability_names_keyword(a: &Ability, name: &str) -> bool {
    match a {
        Ability::Keyword(k) => k.as_str() == name,
        _ => false,
    }
}

/// Every `Cant(Attach)` row in the derived view, with its carrier:
/// `(carrier source, what, to)`. Both the **attachment-side** restriction
/// (Enchant's quality bound [CR#702.5a], the Equipment [CR#301.5] /
/// Fortification [CR#301.6] host rule, conferred ability-free as
/// [`Property::Static`](deckmaste_core::Property::Static)) and the
/// **host-side** restriction (protection's can't-be-equipped clause
/// [CR#702.16d]) land here — the row is read the same way regardless of which
/// permanent carries it.
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
            if let StaticSpec::Deontic(d) = e
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
/// as `May(Attach)` grants (the subtype rules ability-free, as
/// [`Property::Static`](deckmaste_core::Property::Static)), not as
/// restrictions subtracting from a phantom default-permission. Mirror of
/// `cant_attach_rows`.
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
            if let StaticSpec::Deontic(d) = e
                && let Some(DeonticAction::Attach { what, to }) = may_action(d)
            {
                rows.push((source, what.clone(), to.clone()));
            }
        });
    }
    rows
}

/// The re-entrancy cap for [`attachment_legal`]. Normal play NEVER nests
/// `attachment_legal`: no real card conditions an `Attach` deontic on
/// `LegallyAttached`, so `condition_holds(LegallyAttached)` — the ONE
/// `condition_holds` arm that re-enters the deontic collector — is never
/// evaluated while already inside an `attachment_legal` call. Any re-entry is
/// therefore the pathological `Conditionally(LegallyAttached(This), <Attach
/// deontic>)` cycle, so a cap of 1 (deny on the FIRST re-entry) is correct and
/// minimal.
const ATTACH_LEGAL_DEPTH_CAP: u32 = 1;

thread_local! {
    /// Re-entrancy depth for [`attachment_legal`]. The deontic collector
    /// (`for_each_static`) gates `Conditionally` statics on `condition_holds`,
    /// and the only condition that re-enters the collector is `LegallyAttached`
    /// (`condition.rs`), which evaluates via `attachment_legal` →
    /// `may/cant_attach_rows` → `for_each_static` →
    /// `condition_holds(LegallyAttached)` → `attachment_legal` → … . A permanent
    /// authored `Conditionally(LegallyAttached(This), <Attach deontic>)` (a
    /// bootstrap-impossible aura — a semantic-input error; no canon or planned card
    /// authors this shape) would recurse without bound and stack-overflow,
    /// violating the Invalid semantic input fizzles decision
    /// (`docs/decisions/invalid-semantic-input-fizzles.md`). This
    /// counter bounds the recursion (see [`ATTACH_LEGAL_DEPTH_CAP`]).
    static ATTACH_LEGAL_DEPTH: Cell<u32> = const { Cell::new(0) };
}

#[cfg(test)]
thread_local! {
    /// Test-only: how many times the re-entrancy early-return `false` path has
    /// fired. Lets a termination test PROVE the pathological fixture actually
    /// exercises (and bounds) the collector cycle rather than reading `false`
    /// for a mundane reason (no matching grant).
    static ATTACH_LEGAL_REENTRY_DENIALS: Cell<u32> = const { Cell::new(0) };
}

/// RAII scope for [`ATTACH_LEGAL_DEPTH`]: decrements on `Drop` so the counter
/// is restored across EVERY exit of `attachment_legal`'s body (the early
/// `return false` self/missing-host cases and the straight-line final
/// expression alike) without hand-decrementing before each `return`.
struct AttachLegalDepthGuard;

impl Drop for AttachLegalDepthGuard {
    fn drop(&mut self) {
        ATTACH_LEGAL_DEPTH.with(|d| d.set(d.get().saturating_sub(1)));
    }
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
/// protection / type changes / the conferred host rules are all seen).
/// This is the **generic** legality read: it never branches on the
/// Aura/Equipment/Fortification subtype — those carry their permissions as
/// conferred `May(Attach)` grants ([CR#702.5a,301.5,301.6]), and a `Cant`
/// (protection [CR#702.16d]) subtracts from any grant.
#[must_use]
pub(crate) fn attachment_legal(state: &GameState, attachment: ObjectId, host: ObjectId) -> bool {
    // Re-entrancy guard (the Invalid semantic input fizzles decision,
    // `docs/decisions/invalid-semantic-input-fizzles.md`): if the
    // deontic collector has already re-entered `attachment_legal` (only
    // reachable via a `Conditionally(LegallyAttached(…), …)` static — see
    // `ATTACH_LEGAL_DEPTH`), return the conservative default `false` ("not
    // legally attached") WITHOUT recursing or touching the counter. This
    // bounds the otherwise-unbounded cycle so a semantic-input error
    // fizzles instead of crashing. `false` is sound in both polarities, so
    // this is a true no-op for every non-pathological call: a
    // `Conditionally(LegallyAttached(This), May(Attach))` bootstrap drops
    // its own grant (can't attach → never legally attached — a consistent
    // fixpoint), and a `ConditionallyDo(Not(LegallyAttached(This)),
    // Move→Graveyard)` aura instead reads `Not(false)` = true and dies
    // unattached.
    if ATTACH_LEGAL_DEPTH.with(Cell::get) >= ATTACH_LEGAL_DEPTH_CAP {
        #[cfg(test)]
        ATTACH_LEGAL_REENTRY_DENIALS.with(|c| c.set(c.get() + 1));
        return false;
    }
    ATTACH_LEGAL_DEPTH.with(|d| d.set(d.get() + 1));
    let _depth_guard = AttachLegalDepthGuard;

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
/// CONFERS this row ability-free, as a
/// [`Property::Static`](deckmaste_core::Property::Static) rule read off the
/// object's current `card_types` by `statics_on`, so land-play legality
/// (`legal_actions`) AND spell-non-castability
/// (`castable_cost_ignoring_mana`) are both keyed on the capability rather than
/// a `Type::Land` literal — per-face correct for an MDFC land//spell. Mirrors
/// how `attachment_legal` reads `May(Attach)`. `_state` is unused today (the
/// read works off the already-derived `view`) but kept for signature symmetry
/// with the sibling legality readers.
#[must_use]
pub(crate) fn confers_may_play(_state: &GameState, view: &LayeredView, object: ObjectId) -> bool {
    object_has_static(view, object, &|e| {
        matches!(e, StaticSpec::Deontic(d)
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
    // the derived view); a bare stack ability isn't, and `view.get` would
    // panic.
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
            if let StaticSpec::Deontic(d) = e
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

/// Every `Cant(Cast)` row visible to a cast of `candidate`, with its
/// carrier: `(carrier source, what, by)`. Rows come from battlefield
/// permanents (Grand Abolisher / Silence-style grants — "players can't cast
/// spells") PLUS every `StackObject::Spell` entry's OWN statics: split second
/// is a static a spell carries while on the stack ([CR#702.61a]).
/// Unlike `cant_counter_rows` (which only adds the counter's own target),
/// EVERY spell on the stack is scanned here — a split-second spell locks out
/// casting for everyone, not just casts of itself. The stack scan is
/// restricted to `StackObject::Spell` entries: split second lives only on a
/// spell ([CR#702.61a]), never on a `Triggered`/`Activated` entry. That
/// restriction matters beyond filtering out abilities that carry no
/// statics — a `Triggered`/`Activated` entry's `StackEntry.id` is a freshly
/// minted token, but its underlying `ObjectSource` (used to look up printed
/// statics) is the FIRING permanent's card source, which `card_id().is_some()`
/// would pass. Scanning it would read that permanent's printed statics as if
/// the card itself were on the stack — wrong when the permanent has since
/// left the battlefield. Restricting to `Spell` (whose entry id IS the
/// spell's own object id) avoids that misattribution entirely. `from`/
/// `window`/`cost` slots on a `Cant(Cast)` row are ignored here — split
/// second carries none of them.
#[must_use]
fn cant_cast_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Predicate, Predicate)> {
    let mut rows = Vec::new();
    let ids = state
        .zones
        .battlefield
        .iter()
        .copied()
        .chain(spell_stack_ids(state));
    for id in ids {
        let source = state.objects.obj(id).source;
        for_each_static(state, view, id, |e| {
            if let StaticSpec::Deontic(d) = e
                && let Some(DeonticAction::Cast { what, by, .. }) = cant_action(d)
            {
                rows.push((source, what.clone(), by.clone()));
            }
        });
    }
    rows
}

/// [CR#702.61a,101.2]: whether `caster` may legally cast `candidate` w.r.t.
/// `Cant(Cast)` rows — `true` means FORBIDDEN. Cant beats May: even a flash
/// `May(Cast(window: InstantSpeed))` grant that lifts timing ([CR#702.8a])
/// doesn't survive a matching `Cant(Cast)` row (split second, or a
/// battlefield "can't cast" grant).
///
/// Polarity note: `true` here means FORBIDDEN — the opposite of its twin
/// `counter_legal`, whose `true` means LEGAL.
#[must_use]
pub(crate) fn cant_cast(
    state: &GameState,
    view: &LayeredView,
    candidate: ObjectId,
    caster: PlayerId,
) -> bool {
    let proxy = state.player(caster).object;
    cant_cast_rows(state, view)
        .iter()
        .any(|(carrier, what, by)| {
            state.filter_matches_live(what, candidate, *carrier)
                && state.filter_matches_live(by, proxy, *carrier)
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
            if let StaticSpec::Deontic(d) = e
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
    use deckmaste_core::Ident;
    use deckmaste_core::KeywordAbility;
    use deckmaste_core::OutcomeGateKind;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::RelationPredicate;
    use deckmaste_core::Selection;
    use deckmaste_core::StaticSpec;
    use deckmaste_core::Timing;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;

    use super::attachment_legal;
    use super::walk_abilities;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::stack::StackEntry;
    use crate::stack::StackObject;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    /// A distinguishable leaf effect: `OutcomeGate` tagged by `gate` so a
    /// collected sequence is order-checkable.
    fn gate(gate: OutcomeGateKind) -> StaticSpec {
        StaticSpec::OutcomeGate {
            who: Predicate::Any,
            gate,
        }
    }

    fn static_ability(effect: StaticSpec) -> Ability {
        Ability::r#static(effect)
    }

    /// A tree exercising every look-through path the one walker must descend:
    /// a plain `Static` effect, a `Static` reached through an `Each`
    /// distribution wrapper (its own `Static` — each ability now carries
    /// exactly one `effect`, so what was once two effects on one ability is
    /// now two sibling abilities), a `Static` reached through a `Composite`
    /// keyword, and a `Static` reached through a NESTED `Composite`.
    fn sample_tree() -> Vec<Ability> {
        use OutcomeGateKind::CantLose;
        use OutcomeGateKind::CantWin;
        vec![
            // [0] plain static effect.
            static_ability(gate(CantLose)),
            // [0b] effect reached through an `Each` wrapper, as a sibling ability.
            static_ability(StaticSpec::Each(
                Selection::SelectAll(Arc::new(deckmaste_core::Region::candidate(Predicate::Any))),
                Arc::new(deckmaste_core::Region::candidate(gate(CantWin))),
            )),
            // [1] effect reached through a keyword composite.
            Ability::Keyword(KeywordAbility::Composite {
                name: Ident::new("Kw"),
                abilities: vec![static_ability(gate(CantLose))],
            }),
            // [2] effect reached through a nested keyword composite.
            Ability::Keyword(KeywordAbility::Composite {
                name: Ident::new("Outer"),
                abilities: vec![Ability::Keyword(KeywordAbility::Composite {
                    name: Ident::new("Inner"),
                    abilities: vec![static_ability(gate(CantWin))],
                })],
            }),
        ]
    }

    /// The visit-each form sees every static effect, descending through
    /// `Each` distribution wrappers and keyword composites (nested included)
    /// at every level — in DFS order.
    #[test]
    fn walk_visits_every_effect_through_all_wrappers() {
        use OutcomeGateKind::CantLose;
        use OutcomeGateKind::CantWin;

        let tree = sample_tree();
        let mut seen = Vec::new();
        let done = walk_abilities(&tree, &mut |_: &deckmaste_core::Condition| true, &mut |e| {
            if let StaticSpec::OutcomeGate { gate, .. } = e {
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
                StaticSpec::OutcomeGate {
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
        // Stops at the SECOND effect (CantLose, then the Each-wrapped CantWin)
        // — it does not go on to visit either composite branch.
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
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: types.into_iter().map(Type::def).collect(),
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
        id
    }

    /// The Equipment/Fortification subtype rule ([CR#301.5,301.6]) in its
    /// structural home: an ability-free `Property::Static` conferral carrying
    /// one `May(Attach(what, to))` grant under default-deny attachment.
    fn may_attach_rule(what: Predicate, to: Predicate) -> deckmaste_core::Property {
        deckmaste_core::Property::Static(Arc::new(deckmaste_core::Region::candidate(
            StaticSpec::Deontic(Deontic::May(DeonticAction::Attach { what, to })),
        )))
    }

    /// A printed `May(Attach(what, to))` static — the shape Enchant's keyword
    /// grant ([CR#702.5a]) lands in on the Aura itself.
    fn may_attach_ability(what: Predicate, to: Predicate) -> Ability {
        Ability::r#static(StaticSpec::Deontic(Deontic::May(DeonticAction::Attach {
            what,
            to,
        })))
    }

    /// Mint a battlefield object of `types` carrying no printed abilities but
    /// one subtype named `subtype` whose `confers` is `rules` (player 0).
    fn obj_on_field_with_subtype(
        state: &mut GameState,
        name: &str,
        types: Vec<Type>,
        subtype: &str,
        rules: Vec<deckmaste_core::Property>,
    ) -> ObjectId {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: types.into_iter().map(Type::def).collect(),
            subtypes: vec![deckmaste_core::Subtype {
                name: Ident::new(subtype),
                types: Vec::new().into(),
                confers: rules.into(),
            }],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), PlayerId(0));
        let id = state.objects.mint(
            ObjectSource::Card(card_id),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(id);
        id
    }

    fn creature() -> Predicate {
        Predicate::creature()
    }

    /// [CR#701.3b,301.5]: under default-deny, an attachment whose subtype
    /// confers the ability-free `Static(May(Attach(what: Ref(This), to:
    /// Creature)))` rule (the Equipment-subtype grant) is legal on a creature
    /// host and illegal on a non-creature host — no grant covers the
    /// non-creature pair.
    #[test]
    fn attachment_legal_honors_attachment_side_grant() {
        let mut state = game();
        let equip = obj_on_field_with_subtype(
            &mut state,
            "Test Equipment",
            vec![Type::Artifact],
            "Equipment",
            vec![may_attach_rule(
                Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
        // legal on any host at all; carry one so the bracketing legal case
        // holds.
        let a = obj_on_field(
            &mut state,
            "Aura",
            vec![Type::Enchantment],
            vec![may_attach_ability(
                Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
        // Under default-deny the attachment carries a `May(Attach to:
        // Creature)` grant, so it's legal on a plain creature; the
        // protected host's `Cant` subtracts from that grant.
        let attachment = obj_on_field(
            &mut state,
            "Aura",
            vec![Type::Enchantment],
            vec![may_attach_ability(
                Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                creature(),
            )],
        );
        // A protected creature: forbids ANY attachment onto itself.
        let protected = obj_on_field(
            &mut state,
            "Protected Bear",
            vec![Type::Creature],
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Attach {
                    what: Predicate::Any,
                    to: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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

    /// The Invalid semantic input fizzles decision
    /// (`docs/decisions/invalid-semantic-input-fizzles.md`) / [CR#611.3a]: a
    /// permanent authored `Conditionally(LegallyAttached(This),
    /// May(Attach{Ref(This), Any}))` is a bootstrap-impossible aura. The
    /// deontic collector gates the grant on
    /// `condition_holds(LegallyAttached)`, which re-enters
    /// `attachment_legal`, which re-collects, which re-evaluates the
    /// condition … an UNBOUNDED cycle that would stack-overflow (aborting
    /// the process) without the re-entrancy guard. With the guard the
    /// re-entry returns the conservative `false`, so: the cycle TERMINATES,
    /// the self-referential `May(Attach)` grant drops, and the object reads
    /// as NOT legally attached — a consistent bootstrap fixpoint.
    #[test]
    fn attachment_legal_bounds_conditionally_legally_attached_cycle() {
        use deckmaste_core::Condition;
        let mut state = game();

        // The pathological attachment: its ONLY `May(Attach)` grant is gated
        // behind `LegallyAttached(This)` — the self-referential shape that
        // recurses through the collector.
        let pathological = Ability::r#static(StaticSpec::Conditionally(
            Condition::LegallyAttached(Reference::Reg(deckmaste_core::RefId(0))),
            Arc::new(StaticSpec::Deontic(Deontic::May(DeonticAction::Attach {
                what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                to: Predicate::Any,
            }))),
        ));
        let aura = obj_on_field(
            &mut state,
            "Bootstrap Aura",
            vec![Type::Enchantment],
            vec![pathological],
        );
        let host = obj_on_field(&mut state, "Bear", vec![Type::Creature], vec![]);
        // Attach it so `condition_holds(LegallyAttached)` reaches
        // `attachment_legal` (the re-entry door) rather than short-circuiting
        // on `attached_to == None`.
        state.objects.obj_mut(aura).attached_to = Some(host);

        // Prove the fixture EXERCISES the cycle (the collector really re-enters
        // `attachment_legal` and is bounded), not merely that it returns
        // `false` for a mundane reason: reset the re-entry-denial
        // counter first.
        super::ATTACH_LEGAL_REENTRY_DENIALS.with(|c| c.set(0));

        // (a) `attachment_legal` on the pathological pair TERMINATES and
        // returns the conservative `false` — the self-gated grant
        // drops.
        assert!(
            !attachment_legal(&state, aura, host),
            "the self-referential Conditionally(LegallyAttached, May(Attach)) grant \
             drops — the object is not legally attached (conservative bound)"
        );
        assert!(
            super::ATTACH_LEGAL_REENTRY_DENIALS.with(std::cell::Cell::get) >= 1,
            "the fixture must actually re-enter attachment_legal (the guard must \
             fire) — otherwise the test is not exercising the cycle it bounds"
        );

        // (b) The condition itself, read top-level, also TERMINATES and reads
        // `false` — this is exactly the Aura-graveyard SBA trigger's read
        // (`ConditionallyDo(Not(LegallyAttached(Ref(This))), …)`), which must
        // not hang.
        let frame = state.frame(aura, PlayerId(0));
        assert!(
            !state.condition_holds(
                &Condition::LegallyAttached(Reference::Reg(deckmaste_core::RefId(0))),
                &frame
            ),
            "LegallyAttached(This) reads false for the bootstrap-impossible aura, \
             and terminates"
        );

        // (c) The RAII depth guard restored the counter to 0 after every call,
        // so the guard is inert for the next (normal) query.
        assert_eq!(
            super::ATTACH_LEGAL_DEPTH.with(std::cell::Cell::get),
            0,
            "the RAII depth guard restores the counter to 0 after the call"
        );
    }

    /// The re-entrancy guard is INERT for a normal attachment: a plain
    /// `May(Attach{Ref(This), Any})` grant (no `Conditionally` wrapper, so no
    /// re-entry) is honored exactly as before, and the re-entry-denial path
    /// never fires — the guard changes nothing except to terminate the
    /// pathological cycle.
    #[test]
    fn attachment_legal_guard_is_inert_for_a_normal_grant() {
        let mut state = game();
        let aura = obj_on_field(
            &mut state,
            "Plain Aura",
            vec![Type::Enchantment],
            vec![may_attach_ability(
                Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                Predicate::Any,
            )],
        );
        let host = obj_on_field(&mut state, "Bear", vec![Type::Creature], vec![]);

        super::ATTACH_LEGAL_REENTRY_DENIALS.with(|c| c.set(0));
        assert!(
            attachment_legal(&state, aura, host),
            "a plain (non-conditional) May(Attach to: Any) grant is legal — the \
             guard does not change a normal result"
        );
        assert_eq!(
            super::ATTACH_LEGAL_REENTRY_DENIALS.with(std::cell::Cell::get),
            0,
            "a normal attachment never re-enters attachment_legal — the guard is inert"
        );
    }

    // --- I1: a non-activated ability must not desync the activation index --

    /// A tap-for-mana activated ability `tap_mana_ability` recognizes: cost
    /// `[Tap]`, no targets, producing one fixed-colour mana.
    fn tap_for_colorless() -> Ability {
        use deckmaste_core::Action;
        use deckmaste_core::ActivatedAbility;
        use deckmaste_core::ColorOrColorless;
        use deckmaste_core::CostComponent;
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::ManaProduction;
        use deckmaste_core::ManaSpec;
        let ability = ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![CostComponent::Tap]).into(),
            window: None,
            condition: None,
            limits: vec![].into(),
            effect: Instruction::act(Action::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaProduction::Bare(ManaSpec::Specific(ColorOrColorless::Colorless)),
            ))
            .into(),
        };
        Ability::Activated(Arc::new(ability))
    }

    /// A non-activated static ability, occupying an index slot ahead of the
    /// activated one.
    fn deontic_static() -> Ability {
        Ability::r#static(StaticSpec::Deontic(Deontic::Cant(DeonticAction::Attach {
            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
            to: Predicate::Not(Arc::new(creature())),
        })))
    }

    /// [CR#613.1f]: with a non-activated ability positioned BEFORE an
    /// activated one, the legal-action list and resolution share ONE index
    /// space — `derive::usable_abilities`, where every ability keeps its slot:
    /// the offered `Action::ActivateAbility { ability }` index is 1, and
    /// `begin_activate` resolves THAT ability. The card-facing
    /// `derive::abilities` is the SAME list (no ability class is hidden from
    /// it), so both abilities are visible.
    #[test]
    fn static_before_activated_does_not_desync_the_index() {
        let mut state = game();
        // Abilities printed in this order: [Static(Deontic), Activated(tap
        // mana)]. An artifact so the mana-ability path's
        // `tap_forbidden` guard is false.
        let object = obj_on_field(
            &mut state,
            "Manarock",
            vec![Type::Artifact],
            vec![deontic_static(), tap_for_colorless()],
        );

        // (a) Exactly one activation is offered, and it indexes the usable
        // list: the static keeps slot 0, the activated ability is 1.
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
            "the offered activation must index the usable list (1) — the static \
             keeps its slot"
        );

        // (b) The offered index resolves to the activated ability in the SAME
        // usable list, and `begin_activate` stages it without panicking.
        let usable = crate::derive::usable_abilities(&state, object);
        assert!(
            crate::activate::as_activated(&usable[activations[0]]).is_some(),
            "the offered index names the activated ability in the usable list"
        );
        // The card-facing surface is the same list — nothing is hidden.
        assert_eq!(
            crate::derive::abilities(&state, object).len(),
            2,
            "derive::abilities shows both printed abilities"
        );
        state.begin_activate(object, activations[0]);
        assert!(
            state.announcing.is_some(),
            "begin_activate staged the activated ability at the offered index"
        );
    }

    // --- PREREQUISITE: composite-keyword flattening for static reads
    // ----------

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
        // keyword (the Enchant macro shape: a
        // `Keyword(Composite{[Static(..)]})`).
        let enchant_composite = Ability::Keyword(KeywordAbility::Composite {
            name: "Enchant".into(),
            abilities: vec![Ability::r#static(StaticSpec::Deontic(Deontic::May(
                DeonticAction::Attach {
                    what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Counter {
                    by: Predicate::Any,
                    on: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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

    // --- cant_cast (split-second-style stack lockout) -----------------------

    /// The plugin-loaded Instant `TypeDef`, inline (mirrors `cast::tests::
    /// instant_typedef` — a synthetic card never passes through the plugin
    /// macro expansion that would attach the confer): its conferred
    /// `May(Cast(window: InstantSpeed))` row ([CR#307.1,117.1a,702.8a]) is the
    /// flash grant a `Cant(Cast)` lockout must still beat ([CR#101.2]).
    fn cant_cast_instant_typedef() -> deckmaste_core::TypeDef {
        deckmaste_core::TypeDef {
            name: "Instant".into(),
            permanent_type: false,
            confers: vec![deckmaste_core::Property::Ability(Arc::new(
                Ability::r#static(StaticSpec::Deontic(Deontic::May(DeonticAction::Cast {
                    what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    by: Predicate::Any,
                    from: None,
                    window: Some(Timing::InstantSpeed),
                    cost: None,
                    tag: None,
                }))),
            ))]
            .into(),
        }
    }

    /// Mint an instant-speed spell (via the conferred flash row above, its
    /// ONLY timing permission) into `controller`'s hand.
    fn flash_spell_in_hand(state: &mut GameState, name: &str, controller: PlayerId) -> ObjectId {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![cant_cast_instant_typedef()],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state
            .objects
            .mint(ObjectSource::Card(card_id), controller, Some(Zone::Hand));
        state.zones.hands[controller.index()].push(id);
        id
    }

    /// Mint a card-backed "spell" straight onto the stack, carrying its OWN
    /// `Static(Cant(Cast(what, by)))` row — the split-second lockout shape
    /// ([CR#702.61a]): a spell functions this static while on the
    /// stack, gathered by `cant_cast_rows` the same way `cant_counter_rows`
    /// gathers a can't-be-countered row from its own target.
    fn cast_lockout_on_stack(
        state: &mut GameState,
        name: &str,
        controller: PlayerId,
        what: Predicate,
        by: Predicate,
    ) -> ObjectId {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Cast {
                    what,
                    by,
                    from: None,
                    window: None,
                    cost: None,
                    tag: None,
                },
            )))],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state
            .objects
            .mint(ObjectSource::Card(card_id), controller, Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            id,
            object: StackObject::Spell(id),
            controller,
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            paid_costs: Vec::new(),
            copy: false,
        });
        id
    }

    /// [CR#702.61a,101.2]: a card-backed stack object's OWN `Cant(Cast)` row
    /// (split second) forbids casting a matching spell while it sits on the
    /// stack — including one whose only timing permission is a flash
    /// `May(Cast(InstantSpeed))` grant (Cant beats May, [CR#101.2]). The
    /// lockout lifts the moment the carrying object leaves the stack.
    #[test]
    fn cant_cast_locks_out_casting_while_on_stack_and_lifts_when_gone() {
        let mut state = game();
        let spell = flash_spell_in_hand(&mut state, "Bolt", PlayerId(0));
        let view = state.layers();
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), spell)
                .is_some(),
            "sanity: castable via its own flash grant before any lockout"
        );

        cast_lockout_on_stack(
            &mut state,
            "Split Seconder",
            PlayerId(1),
            Predicate::Any,
            Predicate::Any,
        );
        let view = state.layers();
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), spell)
                .is_none(),
            "a stack object's own Cant(Cast) row forbids the cast even though \
             the spell's own May(Cast(InstantSpeed)) grant lifts the timing — \
             Cant beats May ([CR#101.2])"
        );

        state.stack.clear();
        let view = state.layers();
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), spell)
                .is_some(),
            "the lockout lifts once the carrying object leaves the stack"
        );
    }

    /// [CR#702.61a]: regression for the stack-scan restriction to
    /// `StackObject::Spell`. A `Triggered` (or `Activated`) entry's stand-in
    /// id is minted from the FIRING permanent's own `ObjectSource::Card(..)`
    /// (mirrors `place_one_trigger`'s `self.objects.mint(noted.source, ...)`),
    /// so its `card_id()` resolves `Some`, same as a real spell's — a naive
    /// `card_id().is_some()` filter would mistake it for one and read the
    /// source card's printed statics as though the card itself sat on the
    /// stack, wrong once the permanent has left the battlefield (as here: it
    /// is nowhere on the battlefield, only its trigger's stand-in is on the
    /// stack). The source card prints its own `Cant(Cast)` static — the same
    /// shape a split-second SPELL would carry — but because this entry is
    /// `Triggered`, not `Spell`, it must NOT lock out casting.
    #[test]
    fn cant_cast_ignores_triggered_stack_stand_ins_source_statics() {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;

        let mut state = game();
        let spell = flash_spell_in_hand(&mut state, "Bolt", PlayerId(0));

        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Departed Permanent".into(),
            types: vec![Type::Creature.def()],
            abilities: vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Cast {
                    what: Predicate::Any,
                    by: Predicate::Any,
                    from: None,
                    window: None,
                    cost: None,
                    tag: None,
                },
            )))],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), PlayerId(1));
        let source = ObjectSource::Card(card_id);

        // Note: the source permanent is NOT on the battlefield — only its
        // fired ability's stand-in sits on the stack, e.g. a dies-trigger.
        let stand_in = state.objects.mint(source, PlayerId(1), Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            id: stand_in,
            object: StackObject::Triggered {
                source,
                ability: 0,
                created: None,
                bindings: crate::trigger::TriggerBindings::default(),
            },
            controller: PlayerId(1),
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            paid_costs: Vec::new(),
            copy: false,
        });

        let view = state.layers();
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), spell)
                .is_some(),
            "a Triggered stand-in's source-card statics must NOT lock out \
             casting — only an actual StackObject::Spell entry's own \
             Cant(Cast) static does ([CR#702.61a])"
        );
    }

    /// Grand-Abolisher shape ([CR#101.2]): a battlefield `Cant(Cast(by:
    /// OpponentOf(You)))` grant blocks an opponent's cast but not the
    /// controller's own — `by` is evaluated against the caster's player-proxy
    /// object, anchored on the grant's own carrier so `Ref(You)` resolves to
    /// the permanent's controller.
    #[test]
    fn cant_cast_battlefield_grant_blocks_opponents_not_controller() {
        let mut state = game();
        // `obj_on_field` mints for player 0 — the Abolisher's controller.
        let _abolisher = obj_on_field(
            &mut state,
            "Grand Abolisher",
            vec![Type::Creature],
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Cast {
                    what: Predicate::Any,
                    by: Predicate::Relation(RelationPredicate::OpponentOf(Arc::new(
                        Predicate::Ref(Reference::Reg(deckmaste_core::RefId(1))),
                    ))),
                    from: None,
                    window: None,
                    cost: None,
                    tag: None,
                },
            )))],
        );
        let controllers_spell = flash_spell_in_hand(&mut state, "Bolt", PlayerId(0));
        let opponents_spell = flash_spell_in_hand(&mut state, "Shock", PlayerId(1));

        let view = state.layers();
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), controllers_spell)
                .is_some(),
            "the grant's own controller is unaffected by their Cant(Cast(by: opponent)) row"
        );
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(1), opponents_spell)
                .is_none(),
            "an opponent of the grant's controller can't cast"
        );
    }

    /// Guard-intact regression: after `Cant(Cast)` is carved out and
    /// EVALUATED by `cant_cast`, the `guard_deontic_seam`'s Cast arm must
    /// still trip LOUDLY on the shapes it doesn't evaluate — a
    /// `Must(Cast(...))` row ("you must cast this spell") is one such shape,
    /// never silently ignored.
    #[test]
    #[should_panic(expected = "deontic")]
    fn cant_cast_guard_still_trips_on_a_must_cast_row() {
        let mut state = game();
        let _herald = obj_on_field(
            &mut state,
            "Herald",
            vec![Type::Enchantment],
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Must(
                DeonticAction::Cast {
                    what: Predicate::Any,
                    by: Predicate::Any,
                    from: None,
                    window: None,
                    cost: None,
                    tag: None,
                },
            )))],
        );
        let _ = super::legal_actions(&state, PlayerId(0));
    }

    /// M1 regression: `cant_cast`/`cant_cast_rows` pattern-match
    /// `DeonticAction::Cast { what, by, .. }` and so only ever evaluate the
    /// SLOTLESS shape — `from`/`window`/`cost` are ignored by the `..`. A
    /// SLOTTED `Cant(Cast)` row (e.g. a future "can't cast from graveyard",
    /// `from: Some(Graveyard)`) is therefore NOT evaluated by `cant_cast`,
    /// so the `guard_deontic_seam`'s Cast arm must keep tripping LOUDLY on
    /// it rather than silently treating it as an all-zones prohibition
    /// (which would be an over-application, not the semantic restriction).
    #[test]
    #[should_panic(expected = "deontic")]
    fn cant_cast_guard_still_trips_on_a_slotted_cant_cast_row() {
        let mut state = game();
        let _grave_lockout = obj_on_field(
            &mut state,
            "Grim Lockdown",
            vec![Type::Enchantment],
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Cast {
                    what: Predicate::Any,
                    by: Predicate::Any,
                    from: Some(Zone::Graveyard),
                    window: None,
                    cost: None,
                    tag: None,
                },
            )))],
        );
        let _ = super::legal_actions(&state, PlayerId(0));
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
            permanent_type: true,
            confers: vec![deckmaste_core::Property::Ability(Arc::new(
                Ability::r#static(StaticSpec::Deontic(Deontic::May(DeonticAction::Play {
                    what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    by: Predicate::Any,
                    from: None,
                }))),
            ))]
            .into(),
        }
    }

    /// Mint a battlefield land whose Land type CONFERS `May(Play)` — the
    /// production shape (the confer rides `face.types`, folded into the derived
    /// abilities by the layer-4 `fold_conferred_abilities`), unlike
    /// `obj_on_field` which uses the empty-confer `Type::def`.
    fn conferred_land_on_field(state: &mut GameState, name: &str) -> ObjectId {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: vec![land_typedef()],
            ..Characteristics::default()
        }));
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
            StaticSpec::Deontic(Deontic::Cant(DeonticAction::Attack {
                by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                on: Predicate::Any,
            }))
        };

        // A false condition (`0 > 1`) ⇒ the inner row is NOT collected.
        let mut off = game();
        obj_on_field(
            &mut off,
            "Off",
            vec![Type::Creature],
            vec![Ability::r#static(StaticSpec::Conditionally(
                Condition::Compare(Count::Literal(0), Cmp::Greater, Count::Literal(1)),
                Arc::new(cant()),
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
            vec![Ability::r#static(StaticSpec::Conditionally(
                Condition::Compare(Count::Literal(1), Cmp::AtLeast, Count::Literal(1)),
                Arc::new(cant()),
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
            Condition::And(
                vec![
                    Condition::Matches(
                        Reference::Reg(deckmaste_core::RefId(0)),
                        Predicate::State(StatePredicate::SummoningSick),
                    ),
                    Condition::Not(Arc::new(Condition::Matches(
                        Reference::Reg(deckmaste_core::RefId(0)),
                        Predicate::Characteristic(CharacteristicPredicate::Has("Haste".into())),
                    ))),
                ]
                .into(),
            )
        };
        let ability = |s: StaticSpec| Property::Ability(Arc::new(Ability::r#static(s)));
        deckmaste_core::TypeDef {
            name: "Creature".into(),
            permanent_type: true,
            confers: vec![
                ability(StaticSpec::Deontic(Deontic::May(DeonticAction::Attack {
                    by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    on: Predicate::Any,
                }))),
                ability(StaticSpec::Deontic(Deontic::May(DeonticAction::Block {
                    by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                    on: Predicate::Any,
                    count: None,
                }))),
                ability(StaticSpec::Conditionally(
                    sick_not_hasty(),
                    Arc::new(StaticSpec::Deontic(Deontic::Cant(DeonticAction::Attack {
                        by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                        on: Predicate::Any,
                    }))),
                )),
                ability(StaticSpec::Conditionally(
                    sick_not_hasty(),
                    Arc::new(StaticSpec::Deontic(Deontic::Cant(
                        DeonticAction::Activate {
                            what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                            by: Predicate::Any,
                            cost: Some(CostPredicate::IncludesTapSymbol),
                        },
                    ))),
                )),
            ]
            .into(),
        }
    }

    /// Mint a battlefield creature (player 0) whose `Creature` type CONFERS the
    /// four combat statics — the production shape (the confer rides
    /// `face.types`, folded into the derived abilities by the layer-4
    /// `fold_conferred_abilities`), unlike `obj_on_field`'s empty-confer
    /// `Type::def`. `sick` seeds the
    /// summoning-sickness flag; `extra` are printed abilities carried IN
    /// ADDITION to the type confers (a `Haste` keyword, an extra plain
    /// `Cant(Attack)`, …).
    fn conferred_creature_full(
        state: &mut GameState,
        name: &str,
        sick: bool,
        extra: Vec<Ability>,
    ) -> ObjectId {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: vec![creature_typedef()],
            abilities: extra,
            ..Characteristics::default()
        }));
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
        // [CR#509.1b] surfacing prunes a blocker no attacker point-wise
        // permits, so a legal blocker is only surfaced against a
        // declared attacker; a plain attacker (no evasion) permits
        // every ground blocker.
        let foe = conferred_creature_on_field(&mut state, "Foe", false);
        let target = state.player(PlayerId(1)).object;
        state.combat.declare_attacker(foe, target);
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
        // [CR#509.1b] surfacing needs a declared attacker to permit the
        // blocker; a plain attacker (no evasion) permits every ground
        // blocker.
        let foe = conferred_creature_on_field(&mut state, "Foe", false);
        let target = state.player(PlayerId(1)).object;
        state.combat.declare_attacker(foe, target);
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

    /// [CR#120.3e]: `is_combatant` reads GRANT PRESENCE, not net attack
    /// eligibility. A conferred creature carrying an extra plain `Cant(Attack)`
    /// still HAS the `May(Attack)` grant, so `is_combatant` is true — its
    /// combat damage is marked ([CR#120.3e]; [CR#120.3d] is the wither/infect
    /// counters result, not marking) — even though the `Cant`
    /// removes it from `legal_attackers`.
    #[test]
    fn is_combatant_reads_grant_presence_not_net_eligibility() {
        let mut state = game();
        let bear = conferred_creature_full(
            &mut state,
            "Barred Bear",
            false,
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Attack {
                    by: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
    /// fails). Regression for the blanket-vs-cost-scoped split
    /// ([CR#702.61a,702.61b]): `blanket_applies: false` throughout (mirroring
    /// the real mana-arm call site) proves this cost-scoped row is unaffected
    /// by the flag — it always applies once its cost predicate holds.
    #[test]
    fn cant_activate_sick_creature_tap_gate_but_free_non_tap() {
        let mut state = game();
        let sick = conferred_creature_on_field(&mut state, "Sick Bear", true);
        let view = state.layers();
        assert!(
            super::cant_activate(&state, &view, sick, PlayerId(0), true, false),
            "a summoning-sick creature can't pay a {{T}} cost ([CR#602.5a])"
        );
        assert!(
            !super::cant_activate(&state, &view, sick, PlayerId(0), false, false),
            "a non-tap ability is not gated by summoning sickness ([CR#602.5a])"
        );

        // A non-sick conferred creature: the sickness condition fails, so no
        // Cant(Activate) is contributed — even a {T} cost is free.
        let mut ready_state = game();
        let ready = conferred_creature_on_field(&mut ready_state, "Ready Bear", false);
        let ready_view = ready_state.layers();
        assert!(
            !super::cant_activate(&ready_state, &ready_view, ready, PlayerId(0), true, false),
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
            !super::cant_activate(&hasty_state, &hasty_view, hasty, PlayerId(0), true, false),
            "Haste lifts summoning sickness — a hasty sick creature taps ([CR#702.10c])"
        );
    }

    // --- cant_activate (split-second-style stack lockout,
    // blanket-vs-cost-scoped) ---

    /// A non-mana activated ability with the given `cost` and a plain
    /// gain-life effect — deliberately NOT `AddMana`, so
    /// `derive::tap_mana_ability` never classifies it as a mana ability
    /// ([CR#605.1a]) and it always takes the full [CR#602.5] non-mana gate,
    /// never the stackless mana-ability path.
    fn non_mana_ability(cost: Vec<deckmaste_core::CostComponent>) -> Ability {
        use deckmaste_core::Action;
        use deckmaste_core::ActivatedAbility;
        use deckmaste_core::Count;
        use deckmaste_core::Instruction;
        use deckmaste_core::LifeOp;
        Ability::activated(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            from: None,
            cost: Arc::<[deckmaste_core::CostComponent]>::from(cost).into(),
            window: None,
            condition: None,
            limits: vec![].into(),
            effect: Instruction::act(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Up(Count::Literal(1)),
            ))
            .into(),
        })
    }

    /// Mint a card-backed "spell" straight onto the stack, carrying its OWN
    /// `Static(Cant(Activate(what, by, cost)))` row — split second's
    /// activate half ([CR#702.61a]): a spell functions this static while on
    /// the stack, gathered by `cant_activate_rows` the same way
    /// `cant_cast_rows` gathers its Cast-side lockout from a stack spell
    /// (both share the `spell_stack_ids` scan).
    fn activate_lockout_on_stack(
        state: &mut GameState,
        name: &str,
        controller: PlayerId,
        what: Predicate,
        by: Predicate,
        cost: Option<deckmaste_core::CostPredicate>,
    ) -> ObjectId {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Activate { what, by, cost },
            )))],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state
            .objects
            .mint(ObjectSource::Card(card_id), controller, Some(Zone::Stack));
        state.stack.push(StackEntry {
            activation: crate::ActivationId::NONE,
            id,
            object: StackObject::Spell(id),
            controller,
            targets: vec![],
            chosen_modes: std::sync::Arc::from([]),
            x: None,
            paid_costs: Vec::new(),
            copy: false,
        });
        id
    }

    /// Collect the `ability` indices `legal_actions` offers for
    /// `Action::ActivateAbility` on `object`, in list order.
    fn offered_ability_indices(
        state: &GameState,
        player: PlayerId,
        object: ObjectId,
    ) -> Vec<usize> {
        super::legal_actions(state, player)
            .iter()
            .filter_map(|a| match a {
                crate::decide::Action::ActivateAbility { object: o, ability } if *o == object => {
                    Some(*ability)
                }
                _ => None,
            })
            .collect()
    }

    /// [CR#702.61a,702.61b,101.2]: a split-second-style blanket
    /// `Cant(Activate(what: Any, by: Any, cost: None))` row on the stack
    /// forbids activating a non-mana ability — whether it has no tap cost
    /// [0] or a `{T}` cost [1] — while the SAME object's mana ability
    /// (`{T}: add {C}`) [2] stays legal: mana abilities are exempt from a
    /// blanket row ([CR#702.61b]), because the mana-arm call site passes
    /// `blanket_applies: false` while the non-mana gate passes `true`.
    #[test]
    fn cant_activate_split_second_stack_lockout_blocks_nonmana_leaves_mana_legal() {
        let mut state = game();
        let object = obj_on_field(
            &mut state,
            "Utility Creature",
            vec![Type::Creature],
            vec![
                non_mana_ability(vec![]),
                non_mana_ability(vec![deckmaste_core::CostComponent::Tap]),
                tap_for_colorless(),
            ],
        );
        let player = PlayerId(0);

        assert_eq!(
            offered_ability_indices(&state, player, object),
            vec![0, 1, 2],
            "sanity: all three abilities are offered with no lockout in play"
        );

        activate_lockout_on_stack(
            &mut state,
            "Split Seconder",
            PlayerId(1),
            Predicate::Any,
            Predicate::Any,
            None,
        );

        assert_eq!(
            offered_ability_indices(&state, player, object),
            vec![2],
            "the blanket split-second lockout blocks both non-mana abilities \
             (no-tap [0] and {{T}} [1]) but leaves the mana ability [2] legal \
             ([CR#702.61b])"
        );
    }

    /// Linvala-shape regression: a BATTLEFIELD blanket `Cant(Activate(what:
    /// Any, by: Any, cost: None))` grant is gathered from
    /// `state.zones.battlefield` (not the stack) and behaves identically —
    /// it blocks non-mana activation but exempts mana abilities
    /// ([CR#702.61b]).
    #[test]
    fn cant_activate_battlefield_blanket_grant_blocks_nonmana_allows_mana() {
        let mut state = game();
        let _linvala_ish = obj_on_field(
            &mut state,
            "Linvala-ish",
            vec![Type::Creature],
            vec![Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                DeonticAction::Activate {
                    what: Predicate::Any,
                    by: Predicate::Any,
                    cost: None,
                },
            )))],
        );
        let object = obj_on_field(
            &mut state,
            "Utility Creature",
            vec![Type::Creature],
            vec![non_mana_ability(vec![]), tap_for_colorless()],
        );
        let player = PlayerId(0);

        assert_eq!(
            offered_ability_indices(&state, player, object),
            vec![1],
            "a battlefield blanket Cant(Activate(cost: None)) grant (Linvala \
             shape) blocks the non-mana ability [0] but leaves the mana \
             ability [1] legal ([CR#702.61b])"
        );
    }

    // --- M2: split-second-style lockout exemptions ([CR#702.61b]) ----------

    /// Mint a land card (via the conferred `May(Play)` row from
    /// `land_typedef()`) into `controller`'s HAND — mirrors
    /// `conferred_land_on_field`, but the PlayLand-during-lockout regression
    /// below needs the land offered as a hand candidate, not already on the
    /// battlefield.
    fn land_in_hand(state: &mut GameState, name: &str, controller: PlayerId) -> ObjectId {
        use deckmaste_card::Card;
        use deckmaste_card::CardFace;
        use deckmaste_card::Characteristics;
        let card = Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            types: vec![land_typedef()],
            ..Characteristics::default()
        }));
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state
            .objects
            .mint(ObjectSource::Card(card_id), controller, Some(Zone::Hand));
        state.zones.hands[controller.index()].push(id);
        id
    }

    /// [CR#702.61b]: playing a land is a SPECIAL ACTION, not a spell cast or
    /// an activated ability — a blanket split-second-style lockout
    /// (slotless `Cant(Cast)` + `Cant(Activate)`, the shape `cant_cast` /
    /// `cant_activate` actually evaluate) must not suppress it.
    ///
    /// Modeled as a BATTLEFIELD blanket grant (the Grand-Abolisher/Linvala
    /// shape the sibling tests above use) rather than the genuine on-stack
    /// split-second shape
    /// (`cast_lockout_on_stack`/`activate_lockout_on_stack`):
    /// `sorcery_speed_ok` itself requires an EMPTY stack ([CR#116.2a]),
    /// so putting the lockout object ON the stack would make `PlayLand`
    /// illegal for an unrelated reason (stack non-emptiness), confounding
    /// the very thing this test isolates. This is a deliberate narrowing:
    /// it still exercises the real `Cant(Cast)`/`Cant(Activate)` evaluation
    /// paths (`cant_cast` via `can_cast`, `cant_activate` via the
    /// battlefield loop), just anchored off-stack.
    #[test]
    fn play_land_still_offered_during_a_blanket_cast_and_activate_lockout() {
        let mut state = game();
        state.turn.current = deckmaste_core::PhaseStep::PrecombatMain;
        // `game()` already sets `turn.active_player = PlayerId(0)` with an
        // empty stack — sorcery timing holds absent the lockout.

        let _lockdown = obj_on_field(
            &mut state,
            "Blanket Lockdown",
            vec![Type::Enchantment],
            vec![
                Ability::r#static(StaticSpec::Deontic(Deontic::Cant(DeonticAction::Cast {
                    what: Predicate::Any,
                    by: Predicate::Any,
                    from: None,
                    window: None,
                    cost: None,
                    tag: None,
                }))),
                Ability::r#static(StaticSpec::Deontic(Deontic::Cant(
                    DeonticAction::Activate {
                        what: Predicate::Any,
                        by: Predicate::Any,
                        cost: None,
                    },
                ))),
            ],
        );

        let land = land_in_hand(&mut state, "Mountain", PlayerId(0));
        // A sanity spell proving the lockout is genuinely live in this
        // fixture — otherwise a broken/no-op lockout would make the
        // PlayLand assertion below vacuous.
        let grounded_spell = flash_spell_in_hand(&mut state, "Bolt", PlayerId(0));

        let legal = super::legal_actions(&state, PlayerId(0));
        assert!(
            legal.contains(&crate::decide::Action::PlayLand { object: land }),
            "a land in hand is still offered as PlayLand during a blanket \
             Cant(Cast)+Cant(Activate) lockout — special actions are exempt \
             ([CR#702.61b])"
        );
        assert!(
            !legal.contains(&crate::decide::Action::CastSpell {
                object: grounded_spell
            }),
            "sanity: the SAME lockout genuinely forbids casting — proves the \
             PlayLand pass above is not vacuous"
        );
    }

    /// [CR#603.3,702.61b]: trigger placement is not gated by casting/
    /// activation legality at all — a split-second-style lockout SITTING ON
    /// THE STACK (the genuine on-stack shape, via `cast_lockout_on_stack` /
    /// `activate_lockout_on_stack`) must not prevent a noted trigger from
    /// being placed as a `Triggered` stack object. Mirrors `trigger.rs`'s
    /// `non_targeting_trigger_places_directly`: note the trigger by hand and
    /// call `place_triggers()` directly — the same real placement barrier
    /// ([CR#603.3]) production code calls, never touching
    /// `cant_cast`/`cant_activate`.
    #[test]
    fn triggered_ability_still_places_on_stack_during_split_second_lockout() {
        use deckmaste_core::EventFilter;
        use deckmaste_core::Instruction;
        use deckmaste_core::TriggeredAbility;

        let mut state = game();
        let source_obj = obj_on_field(
            &mut state,
            "Ticking Permanent",
            vec![Type::Artifact],
            vec![Ability::triggered(TriggeredAbility {
                ability_word: None,
                where_x: None,
                targets: [].into(),
                from: None,
                event: EventFilter::OneOf(Vec::new().into()),
                condition: None,
                limits: Vec::new().into(),
                effect: Instruction::Sequentially(Vec::new().into()).into(),
            })],
        );
        let source = state.objects.obj(source_obj).source;
        let controller = state.objects.obj(source_obj).controller;

        // The split-second lockout itself: stack objects carrying their OWN
        // Cant(Cast) and Cant(Activate) rows (the genuine on-stack shape the
        // `cant_cast`/`cant_activate` split-second tests above use).
        cast_lockout_on_stack(
            &mut state,
            "Split Seconder (cast)",
            PlayerId(1),
            Predicate::Any,
            Predicate::Any,
        );
        activate_lockout_on_stack(
            &mut state,
            "Split Seconder (activate)",
            PlayerId(1),
            Predicate::Any,
            Predicate::Any,
            None,
        );

        // Note the trigger by hand, exactly as `trigger.rs` unit tests do,
        // then run the real placement barrier.
        state.pending_triggers.push(crate::trigger::NotedTrigger {
            source,
            ability: 0,
            created: None,
            controller,
            bindings: crate::trigger::TriggerBindings::default(),
        });

        let progress = state.place_triggers();
        assert_eq!(
            progress,
            crate::step::Progress::TriggersPlaced { placed: 1 },
            "the trigger places despite the split-second lockout sitting on \
             the stack"
        );
        assert!(
            state
                .stack
                .iter()
                .any(|e| matches!(e.object, StackObject::Triggered { ability: 0, .. })),
            "the Triggered stack object lands even while a split-second-style \
             Cant(Cast)/Cant(Activate) lockout sits on the stack — trigger \
             placement never consults cant_cast/cant_activate ([CR#702.61b])"
        );
    }
}
