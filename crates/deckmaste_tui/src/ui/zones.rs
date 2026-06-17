//! Pure enumeration of a focusable zone's selectable items, in display order.
use deckmaste_core::Type;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PlayerId;

use crate::ui::board::Selected;
use crate::ui::board::Zone;

/// The selectable items in `zone`, in display order. Each battlefield column
/// leads with that player's proxy object (the targetable "face" — players are
/// objects in no zone, so this is the only place to select them), then the
/// permanents they control by *derived* controller (so control-change effects
/// move a permanent to the controller's column); the hand is the perspective
/// player's; the stack is top-first (last entry resolves first).
#[must_use]
pub fn contents(
    state: &GameState,
    view: &LayeredView,
    perspective: PlayerId,
    zone: Zone,
) -> Vec<Selected> {
    match zone {
        Zone::Battlefield(player) => {
            // The permanents this player controls by *derived* controller, with
            // lands sorted to the bottom of the column. `sort_by_key` is stable,
            // so play order is preserved within the non-land and land groups, and
            // `false` (non-land) sorts before `true` (land).
            let mut perms: Vec<ObjectId> = state
                .zones
                .battlefield
                .iter()
                .copied()
                .filter(|&id| view.controller(id) == player)
                .collect();
            perms.sort_by_key(|&id| view.get(id).card_types.contains(&Type::Land));
            // Lead with the player's own proxy object (the targetable face).
            std::iter::once(Selected::Object(state.player(player).object))
                .chain(perms.into_iter().map(Selected::Object))
                .collect()
        }
        Zone::Hand => state.zones.hands[perspective.index()]
            .iter()
            .map(|&id| Selected::Object(id))
            .collect(),
        Zone::Graveyard => state.zones.graveyards[perspective.index()]
            .iter()
            .map(|&id| Selected::Object(id))
            .collect(),
        Zone::Exile => state
            .zones
            .exile
            .iter()
            .map(|&id| Selected::Object(id))
            .collect(),
        Zone::Stack => (0..state.stack.len())
            .rev()
            .map(Selected::StackEntry)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::PlayerId;
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::driver::Driver;
    use crate::game;

    fn opening() -> GameState {
        let mut d = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        d.run_to_priority().expect("priority");
        d.state
    }

    #[test]
    fn hand_is_the_perspective_players_hand() {
        let state = opening();
        let view = state.layers();
        let p0 = contents(&state, &view, PlayerId(0), Zone::Hand);
        let p1 = contents(&state, &view, PlayerId(1), Zone::Hand);
        assert_eq!(p0.len(), state.zones.hands[0].len());
        assert_eq!(p1.len(), state.zones.hands[1].len());
        assert!(!p0.is_empty(), "opening hand should be dealt");
    }

    #[test]
    fn battlefield_leads_with_the_player_then_controlled_permanents() {
        let state = opening();
        let view = state.layers();
        for player in [PlayerId(0), PlayerId(1)] {
            let items = contents(&state, &view, player, Zone::Battlefield(player));
            // The column leads with that player's own proxy (the targetable face).
            assert_eq!(items[0], Selected::Object(state.player(player).object));
            // The rest are permanents that player controls.
            for sel in &items[1..] {
                let Selected::Object(id) = *sel else { panic!("battlefield holds objects") };
                assert_eq!(view.controller(id), player);
            }
        }
    }

    #[test]
    fn graveyard_is_the_perspective_players_and_exile_is_shared() {
        let state = opening();
        let view = state.layers();
        for player in [PlayerId(0), PlayerId(1)] {
            let gy = contents(&state, &view, player, Zone::Graveyard);
            assert_eq!(gy.len(), state.zones.graveyards[player.index()].len());
        }
        // Exile is a single shared zone — same contents from either seat.
        let e0 = contents(&state, &view, PlayerId(0), Zone::Exile);
        let e1 = contents(&state, &view, PlayerId(1), Zone::Exile);
        assert_eq!(e0.len(), state.zones.exile.len());
        assert_eq!(e0, e1);
    }

    #[test]
    fn battlefield_sorts_lands_below_nonlands() {
        use deckmaste_core::Type;
        use deckmaste_engine::sim::GreedyDemo;
        use deckmaste_engine::sim::Strategy;

        use crate::driver::Stop;

        // Drive the demo (which develops both boards) until some player controls
        // both a land and a non-land, then assert every land sits below every
        // non-land in their column.
        let strat = GreedyDemo;
        let mut driver = Driver::new(game::build_game().expect("build"), Box::new(GreedyDemo));
        let mut stop = driver.run_to_decision().expect("first stop");
        for _ in 0..200_000 {
            let pending = match &stop {
                Stop::GameOver(_) | Stop::Budget => break,
                Stop::Decision(p) => p.clone(),
            };
            let view = driver.state.layers();
            for player in [PlayerId(0), PlayerId(1)] {
                let perms: Vec<_> =
                    contents(&driver.state, &view, player, Zone::Battlefield(player))[1..] // skip the leading player proxy
                        .iter()
                        .map(|s| match s {
                            Selected::Object(id) => *id,
                            Selected::StackEntry(_) => panic!("battlefield holds objects"),
                        })
                        .collect();
                let is_land = |id| view.get(id).card_types.contains(&Type::Land);
                let has_land = perms.iter().any(|&id| is_land(id));
                let has_nonland = perms.iter().any(|&id| !is_land(id));
                if has_land && has_nonland {
                    let first_land = perms.iter().position(|&id| is_land(id)).expect("a land");
                    assert!(
                        perms[first_land..].iter().all(|&id| is_land(id)),
                        "every land must sit below every non-land in P{}'s column: {perms:?}",
                        player.0
                    );
                    return;
                }
            }
            let decision = strat.decide(&driver.state, &pending);
            stop = driver.submit(decision).expect("legal decision");
        }
        panic!("never reached a board controlling both a land and a non-land");
    }

    #[test]
    fn stack_count_matches_and_is_top_first() {
        let state = opening();
        let view = state.layers();
        let stack = contents(&state, &view, PlayerId(0), Zone::Stack);
        assert_eq!(stack.len(), state.stack.len());
        // Top-first: first item indexes the last stack entry.
        if let Some(&Selected::StackEntry(i)) = stack.first() {
            assert_eq!(i, state.stack.len() - 1);
        }
    }
}
