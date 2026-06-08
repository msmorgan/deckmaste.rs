//! Legal actions for the priority holder ([CR#117.1]). The list this computes
//! is both the advisory `legal` carried by the Priority decision and the
//! authoritative check at submission (state can't change in between: a
//! pending decision blocks stepping).

use deckmaste_core::{StepOrPhase, Type};

use crate::decide::Action;
use crate::derive;
use crate::player::PlayerId;
use crate::state::GameState;

#[must_use]
pub fn legal_actions(state: &GameState, player: PlayerId) -> Vec<Action> {
    let mut legal = vec![Action::Pass];

    // [CR#116.2a,305.2]: a land from hand, own turn, main phase, stack empty
    // (trivially true in the skeleton), one per turn.
    let main = matches!(
        state.turn.current,
        StepOrPhase::PrecombatMain | StepOrPhase::PostcombatMain
    );
    if main && player == state.turn.active_player && state.player(player).lands_played_this_turn < 1
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
        for (ability, a) in derive::abilities(state, object).iter().enumerate() {
            if derive::tap_mana_ability(a).is_some() {
                legal.push(Action::ActivateAbility { object, ability });
            }
        }
    }

    // CR 601.3a: cast a spell from hand if timing + payment + targets permit.
    let stack_empty = state.stack.is_empty() && state.announcing.is_none();
    for &object in &state.zones.hands[player.index()] {
        if state.can_cast(player, object, main, stack_empty) {
            legal.push(Action::CastSpell { object });
        }
    }

    legal
}
