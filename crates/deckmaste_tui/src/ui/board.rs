//! Pure UI state for the board view: which zone is focused, the remembered
//! selection per zone, and the auto-following perspective. No ratatui types and
//! no engine mutation — unit-tested headlessly.
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PlayerId;

use crate::ui::zones;

/// A focusable zone. The two battlefields are keyed by player so the columns
/// stay fixed as the perspective flips; `Hand`/`Graveyard` are the perspective
/// player's (private hand, owned graveyard); `Stack`/`Exile` are shared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Battlefield(PlayerId),
    Stack,
    Hand,
    Graveyard,
    Exile,
}

/// The focusable zones in `Tab` order (two-player hotseat).
pub const ZONES: [Zone; 6] = [
    Zone::Battlefield(PlayerId(0)),
    Zone::Battlefield(PlayerId(1)),
    Zone::Stack,
    Zone::Hand,
    Zone::Graveyard,
    Zone::Exile,
];

/// The zone a one-letter hotkey selects, relative to `perspective`: `b` your
/// battlefield, `o` the opponent's, `h` hand, `s` stack, `g` graveyard, `e`
/// exile. `None` for any other key.
#[must_use]
pub fn zone_for_key(key: char, perspective: PlayerId) -> Option<Zone> {
    Some(match key {
        'b' => Zone::Battlefield(perspective),
        'o' => Zone::Battlefield(opponent(perspective)),
        'h' => Zone::Hand,
        's' => Zone::Stack,
        'g' => Zone::Graveyard,
        'e' => Zone::Exile,
        _ => return None,
    })
}

/// The other player in the two-player hotseat.
#[must_use]
pub fn opponent(p: PlayerId) -> PlayerId { PlayerId(1 - p.0) }

/// A selectable item within a zone. Battlefield/hand resolve to a live object;
/// the stack resolves to an index into `state.stack`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selected {
    Object(deckmaste_engine::ObjectId),
    StackEntry(usize),
}

/// Read-only UI state threaded through the render + key loop.
#[derive(Debug, Clone)]
pub struct BoardState {
    /// Index into [`ZONES`] of the focused zone.
    focus: usize,
    /// Remembered selection index per zone (parallel to [`ZONES`]).
    selected: [usize; ZONES.len()],
    /// The player whose hand is revealed; auto-follows the pending decider.
    pub perspective: PlayerId,
}

impl BoardState {
    /// Initial state: P0 battlefield focused, perspective on P0.
    #[must_use]
    pub fn new() -> Self {
        Self {
            focus: 0,
            selected: [0; ZONES.len()],
            perspective: PlayerId(0),
        }
    }

    /// The currently focused zone.
    #[must_use]
    pub fn focused_zone(&self) -> Zone { ZONES[self.focus] }

    /// Whether `zone` is the focused one (for border highlighting).
    #[must_use]
    pub fn is_focused(&self, zone: Zone) -> bool { self.focused_zone() == zone }

    /// The remembered (un-clamped) selection index for `zone`.
    #[must_use]
    pub fn selection_index(&self, zone: Zone) -> usize { self.selected[zone_pos(zone)] }

    /// Move focus to the next/previous zone in the ring (wrapping).
    pub fn cycle_zone(&mut self, forward: bool) {
        let n = ZONES.len();
        self.focus = if forward { (self.focus + 1) % n } else { (self.focus + n - 1) % n };
    }

    /// Jump focus directly to `zone` (the letter-hotkey path).
    pub fn focus_zone(&mut self, zone: Zone) { self.focus = zone_pos(zone); }

    /// Move the selection within the focused zone (wrapping). `len` is the
    /// focused zone's current item count; a no-op when the zone is empty.
    pub fn step_selection(&mut self, forward: bool, len: usize) {
        if len == 0 {
            return;
        }
        let cur = self.selected[self.focus].min(len - 1);
        self.selected[self.focus] = if forward { (cur + 1) % len } else { (cur + len - 1) % len };
    }

    /// Recompute the perspective from the pending decision; keep the last value
    /// when nothing is pending (game over / between decisions).
    pub fn sync(&mut self, state: &GameState) {
        if let Some(pending) = &state.pending {
            self.perspective = pending.decider_player();
        }
    }

    /// The number of selectable items in the focused zone (for
    /// `step_selection`).
    #[must_use]
    pub fn focused_len(&self, state: &GameState, view: &LayeredView) -> usize {
        zones::contents(state, view, self.perspective, self.focused_zone()).len()
    }

    /// Focus `zone` and put the cursor on the first slot holding `id`; returns
    /// whether `id` was present.
    fn select_object(
        &mut self,
        zone: Zone,
        id: ObjectId,
        state: &GameState,
        view: &LayeredView,
    ) -> bool {
        let items = zones::contents(state, view, self.perspective, zone);
        if let Some(pos) = items.iter().position(|s| *s == Selected::Object(id)) {
            self.focus = zone_pos(zone);
            self.selected[zone_pos(zone)] = pos;
            true
        } else {
            false
        }
    }

    /// Focus the first focusable zone that holds `id` and put the cursor there.
    /// Used to steer the cursor to the live attackers during blocker pairing
    /// (whose attacker candidates aren't in [`Interaction::candidates`]).
    pub fn steer_to(&mut self, id: ObjectId, state: &GameState, view: &LayeredView) -> bool {
        for &zone in &ZONES {
            if self.select_object(zone, id, state, view) {
                return true;
            }
        }
        false
    }

    /// Resolve the focused zone's selection to a [`Selected`], clamped to live
    /// contents. `None` when the focused zone is empty.
    #[must_use]
    pub fn selected(&self, state: &GameState, view: &LayeredView) -> Option<Selected> {
        let items = zones::contents(state, view, self.perspective, self.focused_zone());
        if items.is_empty() {
            return None;
        }
        Some(items[self.selected[self.focus].min(items.len() - 1)])
    }
}

impl Default for BoardState {
    fn default() -> Self { Self::new() }
}

/// Position of `zone` in [`ZONES`].
fn zone_pos(zone: Zone) -> usize {
    ZONES
        .iter()
        .position(|&z| z == zone)
        .expect("zone is in ZONES")
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::PlayerId;
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::driver::Driver;
    use crate::game;

    #[test]
    fn cycle_zone_wraps_both_ways() {
        let mut b = BoardState::new();
        assert_eq!(b.focused_zone(), Zone::Battlefield(PlayerId(0)));
        b.cycle_zone(false); // wrap back to the last zone
        assert_eq!(b.focused_zone(), *ZONES.last().unwrap());
        assert_eq!(b.focused_zone(), Zone::Exile);
        b.cycle_zone(true); // forward wrap to the first
        assert_eq!(b.focused_zone(), Zone::Battlefield(PlayerId(0)));
    }

    #[test]
    fn focus_zone_jumps_directly() {
        let mut b = BoardState::new();
        b.focus_zone(Zone::Graveyard);
        assert_eq!(b.focused_zone(), Zone::Graveyard);
        b.focus_zone(Zone::Battlefield(PlayerId(1)));
        assert_eq!(b.focused_zone(), Zone::Battlefield(PlayerId(1)));
    }

    #[test]
    fn zone_for_key_is_perspective_relative() {
        // From P0's seat, `b` is P0's battlefield and `o` is P1's.
        assert_eq!(
            zone_for_key('b', PlayerId(0)),
            Some(Zone::Battlefield(PlayerId(0)))
        );
        assert_eq!(
            zone_for_key('o', PlayerId(0)),
            Some(Zone::Battlefield(PlayerId(1)))
        );
        // From P1's seat the two flip, so `b` always means "mine".
        assert_eq!(
            zone_for_key('b', PlayerId(1)),
            Some(Zone::Battlefield(PlayerId(1)))
        );
        assert_eq!(
            zone_for_key('o', PlayerId(1)),
            Some(Zone::Battlefield(PlayerId(0)))
        );
        assert_eq!(zone_for_key('h', PlayerId(0)), Some(Zone::Hand));
        assert_eq!(zone_for_key('s', PlayerId(0)), Some(Zone::Stack));
        assert_eq!(zone_for_key('g', PlayerId(0)), Some(Zone::Graveyard));
        assert_eq!(zone_for_key('e', PlayerId(0)), Some(Zone::Exile));
        assert_eq!(zone_for_key('z', PlayerId(0)), None);
    }

    #[test]
    fn step_selection_wraps_and_empty_is_noop() {
        let mut b = BoardState::new();
        b.step_selection(true, 3); // 0 -> 1
        assert_eq!(b.selection_index(b.focused_zone()), 1);
        b.step_selection(false, 3); // 1 -> 0
        b.step_selection(false, 3); // 0 -> 2 (wrap)
        assert_eq!(b.selection_index(b.focused_zone()), 2);
        b.step_selection(true, 0); // empty zone: no-op
        assert_eq!(b.selection_index(b.focused_zone()), 2);
    }

    #[test]
    fn steer_to_focuses_the_zone_holding_an_object() {
        let mut d = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        d.run_to_priority().expect("priority");
        let state = &d.state;
        let view = state.layers();
        let mut b = BoardState::new();
        b.sync(state);

        // A card in the perspective player's hand: steering focuses Hand and
        // lands the cursor on it.
        let id = state.zones.hands[b.perspective.index()][0];
        assert!(b.steer_to(id, state, &view));
        assert_eq!(b.focused_zone(), Zone::Hand);
        assert!(matches!(
            b.selected(state, &view),
            Some(Selected::Object(x)) if x == id
        ));
    }

    #[test]
    fn blocker_step_steers_onto_a_legal_blocker_and_pairing_submits() {
        use deckmaste_engine::Decision;
        use deckmaste_engine::PendingDecision;
        use deckmaste_engine::sim::GreedyDemo;
        use deckmaste_engine::sim::Strategy;

        use crate::driver::Driver;
        use crate::driver::Stop;
        use crate::interact::Interaction;

        // The demo strategy develops both boards (taps mana, casts, swings);
        // we only override P1 to hold its creatures back so they stay untapped
        // and can block P0 — otherwise both seats tap out attacking and no
        // blocker is ever legal on the opponent's turn.
        let strat = GreedyDemo;
        let mut driver = Driver::new(game::build_game().expect("build"), Box::new(GreedyDemo));
        let mut stop = driver.run_to_decision().expect("first stop");
        for _ in 0..200_000 {
            let pending = match &stop {
                Stop::GameOver(_) | Stop::Budget => break,
                Stop::Decision(p) => p.clone(),
            };
            if let PendingDecision::DeclareBlockers { legal, .. } = &pending
                && !legal.is_empty()
            {
                let it = Interaction::for_decision(&pending).expect("interactive");
                let view = driver.state.layers();
                let mut board = BoardState::new();
                board.sync(&driver.state); // perspective follows the defender

                // Steering lands the cursor on a legal blocker.
                let first = it.candidates()[0];
                assert!(board.steer_to(first, &driver.state, &view));
                match board.selected(&driver.state, &view) {
                    Some(Selected::Object(id)) => {
                        assert!(legal.contains(&id), "cursor sits on a legal blocker");
                    }
                    other => panic!("expected an object selection, got {other:?}"),
                }

                // Pick that blocker, pair it with a live attacker, submit.
                let attacker = driver.state.combat.attackers()[0];
                let mut flow = it.clone();
                flow.toggle(first);
                flow.pair_with(attacker);
                assert_eq!(
                    flow.confirm(),
                    Some(Decision::Blocks(vec![(first, attacker)])),
                    "the pick→pair→submit flow yields a legal block"
                );
                return;
            }

            let decision = match &pending {
                // P1 holds creatures back so they can block P0's attacks.
                PendingDecision::DeclareAttackers { player, .. } if player.0 == 1 => {
                    Decision::Attackers(vec![])
                }
                // Everything else: let the demo strategy develop the board.
                _ => strat.decide(&driver.state, &pending),
            };
            stop = driver.submit(decision).expect("legal decision");
        }
        panic!("never reached a blockers step with legal blockers");
    }

    #[test]
    fn targeting_step_steers_onto_a_candidate_and_targets_path_submits() {
        use deckmaste_engine::Decision;
        use deckmaste_engine::PendingDecision;
        use deckmaste_engine::sim::GreedyDemo;
        use deckmaste_engine::sim::Strategy;

        use crate::driver::Driver;
        use crate::driver::Stop;
        use crate::interact::Interaction;

        // GreedyDemo develops both boards and casts the demo's burn (Bolt /
        // Shock / Mogg Fanatic), which surfaces a real targeting choice once
        // there's more than one legal target.
        let strat = GreedyDemo;
        let mut driver = Driver::new(game::build_game().expect("build"), Box::new(GreedyDemo));
        let mut stop = driver.run_to_decision().expect("first stop");
        for _ in 0..200_000 {
            let pending = match &stop {
                Stop::GameOver(_) | Stop::Budget => break,
                Stop::Decision(p) => p.clone(),
            };
            if let PendingDecision::ChooseTargets { legal, .. } = &pending
                && legal.iter().any(|spec| !spec.is_empty())
            {
                let mut it = Interaction::for_decision(&pending).expect("interactive");

                // Steering lands the cursor on a legal target candidate.
                let view = driver.state.layers();
                let mut board = BoardState::new();
                board.sync(&driver.state);
                let first = it.candidates()[0];
                assert!(board.steer_to(first, &driver.state, &view));
                match board.selected(&driver.state, &view) {
                    Some(Selected::Object(id)) => assert_eq!(id, first),
                    other => panic!("expected an object selection, got {other:?}"),
                }

                // Pick one candidate per spec through the Targets state machine;
                // the result must be a legal decision the engine accepts.
                let decision = loop {
                    if let Some(&cand) = it.candidates().first() {
                        it.toggle(cand);
                    }
                    match it.confirm() {
                        Some(d) => break d,
                        None => it.advance(),
                    }
                };
                assert!(matches!(decision, Decision::Targets(_)));
                driver
                    .submit(decision)
                    .expect("the chosen targets are legal");
                return;
            }

            let decision = strat.decide(&driver.state, &pending);
            stop = driver.submit(decision).expect("legal decision");
        }
        panic!("never reached an interactive targeting step");
    }

    #[test]
    fn sync_follows_pending_decider() {
        let mut d = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        d.run_to_priority().expect("priority");
        let decider = d
            .state
            .pending
            .as_ref()
            .expect("a pending decision")
            .decider_player();
        let mut b = BoardState::new();
        b.sync(&d.state);
        assert_eq!(b.perspective, decider);
    }

    #[test]
    fn selected_resolves_hand_and_is_none_for_empty_stack() {
        let mut d = Driver::new(
            game::build_game().expect("build"),
            Box::new(GreedyCreatures),
        );
        d.run_to_priority().expect("priority");
        let state = &d.state;
        let view = state.layers();
        let mut b = BoardState::new();
        b.sync(state);

        b.focus_zone(Zone::Hand);
        assert_eq!(b.focused_zone(), Zone::Hand);
        assert!(matches!(
            b.selected(state, &view),
            Some(Selected::Object(_))
        ));

        b.focus_zone(Zone::Stack); // empty at opening
        assert_eq!(b.focused_zone(), Zone::Stack);
        assert!(b.selected(state, &view).is_none());
    }
}
