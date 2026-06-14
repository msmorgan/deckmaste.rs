//! Legal actions for the priority holder ([CR#117.1]). The list this computes
//! is both the advisory `legal` carried by the Priority decision and the
//! authoritative check at submission (state can't change in between: a
//! pending decision blocks stepping).

use std::ops::ControlFlow;

use deckmaste_core::Ability;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::Filter;
use deckmaste_core::KeywordAbility;
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
    statics_on(view, id, &mut |e| {
        if pred(e) { ControlFlow::Break(()) } else { ControlFlow::Continue(()) }
    })
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

/// Whether `id` carries a static that replaces its DESTRUCTION — indestructible
/// ([CR#702.12b]), or a regeneration shield once those exist. The `WillDestroy`
/// apply drops the destroy when it does; the lethal-damage SBA routes through
/// the same intent so it, too, spares such permanents ([CR#704.5g]).
pub(crate) fn replaced_from_destruction(view: &LayeredView, id: ObjectId) -> bool {
    object_has_static(
        view,
        id,
        &|s| matches!(s, StaticEffect::Replacement(r) if replaces_destruction(r)),
    )
}

/// Whether a replacement row replaces DESTRUCTION — an `Instead` whose
/// `would` (looked through `Expanded`, e.g. the `Destroyed` macro) is a
/// battlefield→graveyard `ZoneMove`. The kw-indestructible guard keys on
/// it: replacements are unapplied (stage-4 seam), and a destroy that
/// silently ignores an indestructible row would be wrong, not just
/// incomplete.
pub(crate) fn replaces_destruction(r: &deckmaste_core::Replacement) -> bool {
    use deckmaste_core::Event;
    use deckmaste_core::Replacement;
    use deckmaste_core::Zone;
    fn destruction_event(e: &Event) -> bool {
        match e {
            Event::Expanded(x) => destruction_event(&x.value),
            Event::ZoneMove { from, to, .. } => {
                *from == Some(Zone::Battlefield) && *to == Some(Zone::Graveyard)
            }
            Event::OneOf(events) => events.iter().any(destruction_event),
            _ => false,
        }
    }
    match r {
        Replacement::Expanded(x) => replaces_destruction(&x.value),
        Replacement::Instead { would, .. } => destruction_event(would),
        _ => false,
    }
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

/// P0.W2 presence guard ([CR#601.2f] seam): `CostModifier` rows are
/// grammar-complete, but no cost-modification pipeline applies them yet —
/// a row in the derived view would silently change nothing. Loud instead;
/// converts to the [CR#601.2f] pipeline, never gets deleted.
fn guard_cost_modifier_seam(state: &GameState, view: &LayeredView) {
    if statics_present(state, view, |e| {
        matches!(e, StaticEffect::CostModifier { .. })
    }) {
        todo!("P0.W2: cost modification pipeline — CostModifier rows present go unapplied");
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
    // phase, empty stack), one per turn.
    if state.sorcery_speed_ok(player)
        && state.eval_query(deckmaste_core::QueryKey::LandsPlayedThisTurn, player) < 1
    {
        for &object in &state.zones.hands[player.index()] {
            // Derived type ([CR#613.1d]): a card that is a land in the layered
            // view is playable as a land, exactly as the battlefield reads do.
            if view.get(object).card_types.contains(&Type::Land) {
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
        let sick_creature =
            obj.summoning_sick && view.get(object).card_types.contains(&Type::Creature);
        for (ability, a) in view.get(object).abilities.iter().enumerate() {
            // `tap_mana_ability` is the authoritative classifier here: its
            // subset scope (cost=[Tap], specific mana, no targets) defines
            // which abilities take the stackless path ([CR#605.3b]); widen it
            // and this routing together.
            if derive::tap_mana_ability(a).is_some() {
                if !obj.tapped && !sick_creature {
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
    // (the Flagbearer class) at target-choice submission, and the flash
    // shape — May(Cast(window: InstantSpeed)) with no from/cost slot — is
    // EVALUATED as a timing lift in can_cast ([CR#702.8a]); the guard
    // keeps the unevaluated rest: every other Cast row shape (zone
    // permissions, alternative costs, non-May polarities), Play/Attach
    // rows of any polarity, and the May/Gate Target polarities.
    guard_deontic_seam(
        state,
        &view,
        |d| match deontic_action(d) {
            DeonticAction::Cast {
                from, window, cost, ..
            } => {
                !(is_may(d)
                    && *window == Some(deckmaste_core::Window::InstantSpeed)
                    && from.is_none()
                    && cost.is_none())
            }
            DeonticAction::Play { .. } | DeonticAction::Attach { .. } => true,
            DeonticAction::Target { .. } => !is_cant(d) && !is_must(d),
            _ => false,
        },
        "cast/play/attach + May/Gate target",
    );
    guard_cost_modifier_seam(state, &view);
    for &object in &state.zones.hands[player.index()] {
        if state.can_cast(&view, player, object) {
            legal.push(Action::CastSpell { object });
        }
    }

    legal
}

/// [CR#508.1a]: the creatures `player` could declare as attackers — battlefield
/// creatures they control that are untapped and not summoning-sick
/// ([CR#302.6]). Creature-type is read from the derived layer view so that
/// permanents animated into creatures by continuous effects are included.
/// Cost/restriction checks (e.g. defender, "can't attack") are a later seam.
#[must_use]
pub fn legal_attackers(state: &GameState, player: PlayerId) -> Vec<ObjectId> {
    let view = state.layers();
    // Cant(Attack) rows (defender, [CR#702.3b]; "can't attack" effects) are
    // EVALUATED below, and Must(Attack) requirements ("attacks if able",
    // goad) are EVALUATED at declaration submission ([CR#508.1d]); the
    // guard narrows to the May/Gate Attack polarities (May lifts, Gate
    // tolls), which nothing evaluates yet.
    guard_deontic_seam(
        state,
        &view,
        |d| !is_cant(d) && !is_must(d) && matches!(deontic_action(d), DeonticAction::Attack { .. }),
        "attack (May/Gate polarities)",
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
            // its new controller, not its owner.
            view.controller(id) == player
                && !obj.tapped
                && !obj.summoning_sick
                && view.get(id).card_types.contains(&Type::Creature)
                && !rows.iter().any(|(carrier, by, on)| {
                    state.filter_matches_live(by, id, *carrier)
                        && defender_proxy
                            .is_some_and(|d| state.filter_matches_live(on, d, *carrier))
                })
        })
        .collect()
}

/// Every `Attack` row of the polarity `pick` extracts in the derived view,
/// with its carrier — point-wise by construction (`Attack{by, on}` carries
/// no arrangement bound).
fn attack_rows(
    state: &GameState,
    view: &LayeredView,
    pick: fn(&Deontic) -> Option<&DeonticAction>,
) -> Vec<(crate::object::ObjectSource, Filter, Filter)> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(view, id, |e| {
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
) -> Vec<(crate::object::ObjectSource, Filter, Filter)> {
    attack_rows(state, view, cant_action)
}

/// Every `Must(Attack)` row in the derived view — attack requirements
/// ([CR#508.1d]: "attacks if able" effects, goad).
#[must_use]
pub(crate) fn must_attack_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Filter, Filter)> {
    attack_rows(state, view, must_action)
}

/// [CR#509.1a]: the creatures `player` could declare as blockers — battlefield
/// creatures they control that are untapped. No summoning-sickness check: a
/// summoning-sick creature can block. Creature-type is read from the derived
/// layer view so that animated permanents can block.
#[must_use]
pub fn legal_blockers(state: &GameState, player: PlayerId) -> Vec<ObjectId> {
    let view = state.layers();
    // Cant(Block) rows — point-wise (flying) AND arrangement-level
    // (menace's `count`) — and Must(Block) requirements ([CR#509.1c]
    // must-block) are EVALUATED at block submission now; the guard
    // narrows to the May/Gate Block polarities (May lifts, Gate tolls),
    // which nothing evaluates yet.
    guard_deontic_seam(
        state,
        &view,
        |d| !is_cant(d) && !is_must(d) && matches!(deontic_action(d), DeonticAction::Block { .. }),
        "block (May/Gate polarities)",
    );
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&id| {
            let obj = state.objects.obj(id);
            // Derived controller ([CR#613.1b]): a stolen creature blocks for its
            // new controller.
            view.controller(id) == player
                && !obj.tapped
                && view.get(id).card_types.contains(&Type::Creature)
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
    pub by: Filter,
    pub on: Filter,
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
        for_each_static(view, id, |e| {
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

/// Every `Must(Block)` row in the derived view — block requirements
/// ([CR#509.1c]: "blocks if able" effects, "all creatures able to block …
/// do so").
#[must_use]
pub(crate) fn must_block_rows(state: &GameState, view: &LayeredView) -> Vec<BlockRow> {
    block_rows(state, view, must_action)
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
fn walk_abilities<B, F: FnMut(&StaticEffect) -> ControlFlow<B>>(
    abilities: &[Ability],
    visit: &mut F,
) -> ControlFlow<B> {
    fn in_ability<B, F: FnMut(&StaticEffect) -> ControlFlow<B>>(
        a: &Ability,
        visit: &mut F,
    ) -> ControlFlow<B> {
        match a {
            Ability::Static(s) => {
                for e in &s.effects {
                    in_static(e, visit)?;
                }
                ControlFlow::Continue(())
            }
            Ability::Keyword(k) => in_keyword(k, visit),
            Ability::Expanded(e) => in_ability(&e.value, visit),
            _ => ControlFlow::Continue(()),
        }
    }
    fn in_keyword<B, F: FnMut(&StaticEffect) -> ControlFlow<B>>(
        k: &KeywordAbility,
        visit: &mut F,
    ) -> ControlFlow<B> {
        match k {
            KeywordAbility::Composite { abilities, .. } => {
                for a in abilities {
                    in_ability(a, visit)?;
                }
                ControlFlow::Continue(())
            }
            KeywordAbility::Expanded(e) => in_keyword(&e.value, visit),
            _ => ControlFlow::Continue(()),
        }
    }
    fn in_static<B, F: FnMut(&StaticEffect) -> ControlFlow<B>>(
        e: &StaticEffect,
        visit: &mut F,
    ) -> ControlFlow<B> {
        match e {
            StaticEffect::Expanded(x) => in_static(&x.value, visit),
            other => visit(other),
        }
    }
    for a in abilities {
        in_ability(a, visit)?;
    }
    ControlFlow::Continue(())
}

/// The `LayeredView` adapter over [`walk_abilities`]: walks `id`'s derived
/// ability list, calling `visit` on each static effect and short-circuiting on
/// the visitor's `Break`.
fn statics_on<B, F: FnMut(&StaticEffect) -> ControlFlow<B>>(
    view: &LayeredView,
    id: ObjectId,
    visit: &mut F,
) -> ControlFlow<B> {
    walk_abilities(&view.get(id).abilities, visit)
}

/// The non-short-circuiting view over [`statics_on`]: runs `visit` on every
/// static effect of `id` (no early exit). The visit-each callers
/// (`attack_rows`/`block_rows`/`target_rows`/`may_cast_rows`) collect rows
/// through this, leaving the `ControlFlow` plumbing to the one walker.
fn for_each_static<F: FnMut(&StaticEffect)>(view: &LayeredView, id: ObjectId, mut visit: F) {
    // The visitor never breaks, so the only outcome is the run-to-completion
    // `Continue(())`, deliberately discarded.
    let _ = statics_on(view, id, &mut |e| {
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
            let frame = crate::stack::Frame {
                source: r.carrier_id,
                controller: state.objects.obj(r.carrier_id).controller,
                targets: Vec::new(),
                bindings: None,
                chosen: None,
                x: None,
            };
            r.count
                .as_ref()
                .expect("filtered to Some")
                .satisfied_by(n, |c| state.eval_count(c, &frame))
        })
        .map(|r| r.carrier)
}

/// Every `Target` row of the polarity `pick` extracts in the derived view,
/// with its carrier — `(carrier source, by, on)`: `by` matches the
/// targeting spell/ability, `on` the would-be target.
fn target_rows(
    state: &GameState,
    view: &LayeredView,
    pick: fn(&Deontic) -> Option<&DeonticAction>,
) -> Vec<(crate::object::ObjectSource, Filter, Filter)> {
    let mut rows = Vec::new();
    for &id in &state.zones.battlefield {
        let source = state.objects.obj(id).source;
        for_each_static(view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Target { by, on }) = pick(d)
            {
                rows.push((source, by.clone(), on.clone()));
            }
        });
    }
    rows
}

/// Every `Cant(Target)` row in the derived view ([CR#702.11b] hexproof,
/// [CR#702.16b] protection's targeted clause).
#[must_use]
pub(crate) fn cant_target_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Filter, Filter)> {
    target_rows(state, view, cant_action)
}

/// Every `Must(Target)` row in the derived view — targeting requirements
/// (the Flagbearer class: "must choose at least one … if able" — a
/// choice-time constraint inside [CR#601.2c]'s legal-target selection).
#[must_use]
pub(crate) fn must_target_rows(
    state: &GameState,
    view: &LayeredView,
) -> Vec<(crate::object::ObjectSource, Filter, Filter)> {
    target_rows(state, view, must_action)
}

/// The carrier of the first `Cant(Target)` row forbidding `spell` (the
/// targeting stack object / in-flight announce) from targeting `target`.
#[must_use]
pub(crate) fn target_forbidden_by(
    state: &GameState,
    rows: &[(crate::object::ObjectSource, Filter, Filter)],
    spell: ObjectId,
    target: ObjectId,
) -> Option<crate::object::ObjectSource> {
    rows.iter()
        .find(|(carrier, by, on)| {
            state.filter_matches_live(by, spell, *carrier)
                && state.filter_matches_live(on, target, *carrier)
        })
        .map(|(carrier, ..)| *carrier)
}

/// One `May(Cast)` row from the derived view: the carrier it sits on and
/// the permission's slots. `window` is the timing lift ([CR#702.8a]
/// flash); `from`/`cost` are the cast-from-zones / alternative-cost
/// unlocks, carried so the evaluation site can refuse shapes it doesn't
/// evaluate yet.
pub(crate) struct MayCastRow {
    pub carrier: crate::object::ObjectSource,
    pub what: Filter,
    pub by: Filter,
    pub from: Option<deckmaste_core::Zone>,
    pub window: Option<deckmaste_core::Window>,
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
    fn may_action(d: &Deontic) -> Option<&DeonticAction> {
        match d {
            Deontic::May(a) => Some(a),
            Deontic::Expanded(e) => may_action(&e.value),
            _ => None,
        }
    }
    let mut rows = Vec::new();
    for &id in state.zones.battlefield.iter().chain([&candidate]) {
        let source = state.objects.obj(id).source;
        for_each_static(view, id, |e| {
            if let StaticEffect::Deontic(d) = e
                && let Some(DeonticAction::Cast {
                    what,
                    by,
                    from,
                    window,
                    cost,
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

    use deckmaste_core::Ability;
    use deckmaste_core::Expansion;
    use deckmaste_core::Filter;
    use deckmaste_core::Ident;
    use deckmaste_core::KeywordAbility;
    use deckmaste_core::OutcomeGateKind;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;

    use super::walk_abilities;

    /// A distinguishable leaf effect: `OutcomeGate` tagged by `gate` so a
    /// collected sequence is order-checkable.
    fn gate(gate: OutcomeGateKind) -> StaticEffect {
        StaticEffect::OutcomeGate {
            who: Filter::Any,
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

    fn static_ability(effects: Vec<StaticEffect>) -> Ability {
        Ability::Static(StaticAbility {
            condition: None,
            effects,
            characteristic_defining: false,
        })
    }

    /// A tree exercising every look-through path the one walker must descend:
    /// a plain `Static` effect, an `Expanded`-wrapped effect inside a `Static`,
    /// a `Static` reached through a `Composite` keyword, and a `Static` reached
    /// through an `Expanded` ability wrapper.
    fn sample_tree() -> Vec<Ability> {
        use OutcomeGateKind::CantLose;
        use OutcomeGateKind::CantWin;
        vec![
            // [0] plain static effect, plus an Expanded-wrapped effect.
            static_ability(vec![
                gate(CantLose),
                StaticEffect::Expanded(expand(gate(CantWin))),
            ]),
            // [1] effect reached through a keyword composite.
            Ability::Keyword(KeywordAbility::Composite {
                name: Ident::new("Kw"),
                abilities: vec![static_ability(vec![gate(CantLose)])],
            }),
            // [2] effect reached through an Expanded ability wrapper.
            Ability::Expanded(expand(static_ability(vec![gate(CantWin)]))),
        ]
    }

    /// The visit-each form sees every static effect, descending through
    /// static-ability effect lists, keyword composites, and `Expanded`
    /// wrappers at every level — in DFS order.
    #[test]
    fn walk_visits_every_effect_through_all_wrappers() {
        let tree = sample_tree();
        let mut seen = Vec::new();
        let done = walk_abilities(&tree, &mut |e| {
            if let StaticEffect::OutcomeGate { gate, .. } = e {
                seen.push(*gate);
            }
            ControlFlow::<()>::Continue(())
        });
        assert!(done.is_continue(), "a non-breaking walk runs to completion");
        use OutcomeGateKind::CantLose;
        use OutcomeGateKind::CantWin;
        assert_eq!(seen, vec![CantLose, CantWin, CantLose, CantWin]);
    }

    /// The boolean "any" form short-circuits: the visitor `Break`s on the
    /// first match, and the descent stops there rather than visiting the rest.
    #[test]
    fn walk_short_circuits_on_break() {
        let tree = sample_tree();
        let mut visited = 0usize;
        let hit = walk_abilities(&tree, &mut |e| {
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
        let res = walk_abilities(&tree, &mut |_e| {
            visited += 1;
            ControlFlow::<()>::Continue(())
        });
        assert!(res.is_continue());
        assert_eq!(
            visited, 4,
            "every leaf effect is visited when nothing breaks"
        );
    }
}
