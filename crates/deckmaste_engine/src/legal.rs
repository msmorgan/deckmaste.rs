//! Legal actions for the priority holder ([CR#117.1]). The list this computes
//! is both the advisory `legal` carried by the Priority decision and the
//! authoritative check at submission (state can't change in between: a
//! pending decision blocks stepping).

use deckmaste_core::Type;

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
        for (ability, a) in view.get(object).abilities.iter().enumerate() {
            if derive::tap_mana_ability(a).is_some() {
                if !obj.tapped
                    && !(obj.summoning_sick
                        && view.get(object).card_types.contains(&Type::Creature))
                {
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
