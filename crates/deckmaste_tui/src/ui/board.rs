//! Pure UI state for the board view: which zone is focused, the remembered
//! selection per zone, and the auto-following perspective. No ratatui types and
//! no engine mutation — unit-tested headlessly.
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::PlayerId;

use crate::ui::zones;

/// A focusable zone. The two battlefields are keyed by player so the columns
/// stay fixed as the perspective flips.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Battlefield(PlayerId),
    Stack,
    Hand,
}

/// The focusable zones in `Tab` order (two-player hotseat).
pub const ZONES: [Zone; 4] = [
    Zone::Battlefield(PlayerId(0)),
    Zone::Battlefield(PlayerId(1)),
    Zone::Stack,
    Zone::Hand,
];

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
        assert_eq!(b.focused_zone(), Zone::Hand);
        b.cycle_zone(true); // forward wrap to the first
        assert_eq!(b.focused_zone(), Zone::Battlefield(PlayerId(0)));
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

        b.cycle_zone(false); // focus Hand (wrap back from Battlefield(P0))
        assert_eq!(b.focused_zone(), Zone::Hand);
        assert!(matches!(
            b.selected(state, &view),
            Some(Selected::Object(_))
        ));

        b.cycle_zone(false); // focus Stack (empty at opening)
        assert_eq!(b.focused_zone(), Zone::Stack);
        assert!(b.selected(state, &view).is_none());
    }
}
