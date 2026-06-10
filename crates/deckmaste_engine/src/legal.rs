//! Legal actions for the priority holder ([CR#117.1]). The list this computes
//! is both the advisory `legal` carried by the Priority decision and the
//! authoritative check at submission (state can't change in between: a
//! pending decision blocks stepping).

use deckmaste_core::{Phase, Type};

use crate::decide::Action;
use crate::derive;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;
use crate::tally::Tally;

#[must_use]
pub fn legal_actions(state: &GameState, player: PlayerId) -> Vec<Action> {
    // One derived view serves the whole window — the mana-ability and cast
    // checks below read it per object instead of re-deriving the board.
    let view = state.layers();
    let mut legal = vec![Action::Pass];

    // [CR#116.2a,305.2]: a land from hand, own turn, main phase, stack empty
    // (trivially true in the skeleton), one per turn.
    let main = matches!(
        state.turn.current,
        Phase::PrecombatMain | Phase::PostcombatMain
    );
    if main
        && player == state.turn.active_player
        && state.player(player).this_turn.count(Tally::LandsPlayed) < 1
    {
        for &object in &state.zones.hands[player.index()] {
            if derive::face(state.def(object)).types.contains(&Type::Land) {
                legal.push(Action::PlayLand { object });
            }
        }
    }

    // [CR#605.3a]: mana abilities of untapped permanents you control. The
    // only cost the skeleton can pay is {T}.
    for &object in &state.zones.battlefield {
        let obj = state.objects.obj(object);
        if obj.controller != player || obj.tapped {
            continue;
        }
        for (ability, a) in view.get(object).abilities.iter().enumerate() {
            if derive::tap_mana_ability(a).is_some() {
                legal.push(Action::ActivateAbility { object, ability });
            }
        }
    }

    // [CR#601.3a]: cast a spell from hand if timing + payment + targets permit.
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
