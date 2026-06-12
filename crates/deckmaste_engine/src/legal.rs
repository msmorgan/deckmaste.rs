//! Legal actions for the priority holder ([CR#117.1]). The list this computes
//! is both the advisory `legal` carried by the Priority decision and the
//! authoritative check at submission (state can't change in between: a
//! pending decision blocks stepping).

use deckmaste_core::Ability;
use deckmaste_core::Deontic;
use deckmaste_core::DeonticAction;
use deckmaste_core::KeywordAbility;
use deckmaste_core::StaticEffect;
use deckmaste_core::Type;

use crate::decide::Action;
use crate::derive;
use crate::layer::LayeredView;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;
use crate::tally::Tally;

/// The proposed-action pattern inside a deontic row, looking through the
/// `Expanded` provenance wrappers.
fn deontic_action(d: &Deontic) -> &DeonticAction {
    match d {
        Deontic::May(a) | Deontic::Cant(a) | Deontic::Must(a) | Deontic::Gate(a, _) => a,
        Deontic::Expanded(e) => deontic_action(&e.value),
    }
}

/// Whether any battlefield object's derived abilities carry a static effect
/// matching `pred` — looking through static-ability effect lists, keyword
/// composites (flying's evasion `Cant` lives inside `Keyword(Composite)`),
/// and macro `Expanded` wrappers at every level.
fn statics_present<F: Fn(&StaticEffect) -> bool>(
    state: &GameState,
    view: &LayeredView,
    pred: F,
) -> bool {
    fn in_ability<F: Fn(&StaticEffect) -> bool>(a: &Ability, pred: &F) -> bool {
        match a {
            Ability::Static(s) => s.effects.iter().any(|e| in_static(e, pred)),
            Ability::Keyword(k) => in_keyword(k, pred),
            Ability::Expanded(e) => in_ability(&e.value, pred),
            _ => false,
        }
    }
    fn in_keyword<F: Fn(&StaticEffect) -> bool>(k: &KeywordAbility, pred: &F) -> bool {
        match k {
            KeywordAbility::Composite { abilities, .. } => {
                abilities.iter().any(|a| in_ability(a, pred))
            }
            KeywordAbility::Expanded(e) => in_keyword(&e.value, pred),
            _ => false,
        }
    }
    fn in_static<F: Fn(&StaticEffect) -> bool>(e: &StaticEffect, pred: &F) -> bool {
        match e {
            StaticEffect::Expanded(x) => in_static(&x.value, pred),
            other => pred(other),
        }
    }
    state
        .zones
        .battlefield
        .iter()
        .any(|&id| view.get(id).abilities.iter().any(|a| in_ability(a, &pred)))
}

/// P0.W1 presence guard ([CR#101.2,601.3] seam): the deontic grammar is
/// complete, but declaration legality does not evaluate the rows yet. Any
/// matching-verb row in the derived view trips the seam LOUDLY rather than
/// being silently ignored. Never delete a trip to silence it — convert it
/// to the legality evaluation.
fn guard_deontic_seam(
    state: &GameState,
    view: &LayeredView,
    verb: fn(&DeonticAction) -> bool,
    what: &str,
) {
    let hit = statics_present(
        state,
        view,
        |e| matches!(e, StaticEffect::Deontic(d) if verb(deontic_action(d))),
    );
    if hit {
        todo!("P0.W1: deontic {what} legality — rows present in the derived view go unevaluated");
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
    let mut legal = vec![Action::Pass];

    // [CR#116.2a,305.2]: a land from hand — sorcery timing (own turn, main
    // phase, empty stack), one per turn.
    if state.sorcery_speed_ok(player)
        && state.player(player).this_turn.count(Tally::LandsPlayed) < 1
    {
        for &object in &state.zones.hands[player.index()] {
            if derive::face(state.def(object)).types.contains(&Type::Land) {
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
        if obj.controller != player {
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

    // [CR#601.3a]: cast a spell from hand if timing + payment + targets permit.
    guard_deontic_seam(
        state,
        &view,
        |a| matches!(a, DeonticAction::Cast { .. } | DeonticAction::Play { .. }),
        "cast/play",
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
    guard_deontic_seam(
        state,
        &view,
        |a| matches!(a, DeonticAction::Attack { .. }),
        "attack",
    );
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&id| {
            let obj = state.objects.obj(id);
            obj.controller == player
                && !obj.tapped
                && !obj.summoning_sick
                && view.get(id).card_types.contains(&Type::Creature)
        })
        .collect()
}

/// [CR#509.1a]: the creatures `player` could declare as blockers — battlefield
/// creatures they control that are untapped. No summoning-sickness check: a
/// summoning-sick creature can block. Creature-type is read from the derived
/// layer view so that animated permanents can block.
#[must_use]
pub fn legal_blockers(state: &GameState, player: PlayerId) -> Vec<ObjectId> {
    let view = state.layers();
    guard_deontic_seam(
        state,
        &view,
        |a| matches!(a, DeonticAction::Block { .. }),
        "block",
    );
    state
        .zones
        .battlefield
        .iter()
        .copied()
        .filter(|&id| {
            let obj = state.objects.obj(id);
            obj.controller == player
                && !obj.tapped
                && view.get(id).card_types.contains(&Type::Creature)
        })
        .collect()
}
