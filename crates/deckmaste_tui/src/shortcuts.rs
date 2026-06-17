//! Runner-layer convenience over the decision loop (tui-shortcuts):
//! single-legal auto-resolve + per-player "pass" modes. Pure logic + small
//! state; no engine mutation, no ratatui — unit-tested headlessly. The engine
//! stays full-info and pure; what to auto-answer vs. surface is a runner
//! concern (like the autotapper).
use deckmaste_core::Phase;
use deckmaste_core::Uint;
use deckmaste_engine::Decision;
use deckmaste_engine::GameState;
use deckmaste_engine::PendingDecision;
use deckmaste_engine::PlayerId;

/// If every inner slice has exactly one element, the vector of those elements;
/// otherwise `None`. Generic so the "one candidate per slot" rule is testable
/// without constructing opaque `ObjectId`s.
#[must_use]
pub fn single_each<T: Copy>(legal: &[Vec<T>]) -> Option<Vec<T>> {
    if !legal.is_empty() && legal.iter().all(|slot| slot.len() == 1) {
        Some(legal.iter().map(|slot| slot[0]).collect())
    } else {
        None
    }
}

/// The forced answer to `pending` when exactly one legal answer exists and it
/// is not a priority window; otherwise `None`. Priority is never auto-resolved
/// here (passing is a timing choice, and auto-passing every pass-only window
/// globally would have no per-turn guard → both players pass to a decking
/// loss). The only interactive kind this changes is fully-forced targets; other
/// single-legal kinds (e.g. a lone trigger ordering) already auto-resolve via
/// the driver's `Strategy` partition (`is_interactive` returns false for them).
#[must_use]
pub fn auto_answer(pending: &PendingDecision) -> Option<Decision> {
    match pending {
        PendingDecision::ChooseTargets { legal, .. } => single_each(legal).map(Decision::Targets),
        _ => None,
    }
}

/// A player's armed convenience pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassMode {
    /// Reactive: auto-pass priority until something needs you (stack grew,
    /// combat entered, your main, or a forced decision). MTGO F4.
    Yield,
    /// Long skip: auto-pass priority until your next turn's precombat main.
    /// MTGO F6.
    Turn,
}

/// The turn coordinates a stop condition reads. Captured at arm time and
/// compared against the live value each time the armed player regains priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Snapshot {
    pub active: PlayerId,
    pub phase: Phase,
    pub turn: Uint,
    pub stack: usize,
}

/// Whether `player`'s armed `mode` should keep auto-passing this priority
/// window (`true`) or stop and surface to the human (`false`). `armed` is the
/// arm-time snapshot, `now` the live one. All "reached/entered" stops are
/// *since arming*, which composes with clear-on-stop so re-arming inside a
/// boundary won't re-stop.
#[must_use]
pub fn keep_passing(mode: PassMode, armed: &Snapshot, now: &Snapshot, player: PlayerId) -> bool {
    let stack_grew = now.stack > armed.stack;
    let entered_combat =
        matches!(now.phase, Phase::Combat(_)) && !matches!(armed.phase, Phase::Combat(_));
    let at_my_main =
        now.active == player && matches!(now.phase, Phase::PrecombatMain | Phase::PostcombatMain);
    let entered_my_main = at_my_main && now.phase != armed.phase;
    let next_precombat_main =
        now.active == player && now.phase == Phase::PrecombatMain && now.turn > armed.turn;
    match mode {
        PassMode::Yield => {
            !(stack_grew || entered_combat || entered_my_main || next_precombat_main)
        }
        PassMode::Turn => !next_precombat_main,
    }
}

impl Snapshot {
    /// Read the live turn coordinates from the game.
    #[must_use]
    pub fn of(state: &GameState) -> Self {
        Self {
            active: state.turn.active_player,
            phase: state.turn.current,
            turn: state.turn.turn_number,
            stack: state.stack.len(),
        }
    }
}

/// Per-player armed pass state. Indexed by `PlayerId::index()`.
#[derive(Debug, Clone)]
pub struct PassState {
    modes: [Option<PassMode>; 2],
    armed: [Option<Snapshot>; 2],
}

impl PassState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            modes: [None, None],
            armed: [None, None],
        }
    }

    /// Arm `mode` for `player`, snapshotting the current turn coordinates.
    pub fn arm(&mut self, player: PlayerId, mode: PassMode, state: &GameState) {
        self.modes[player.index()] = Some(mode);
        self.armed[player.index()] = Some(Snapshot::of(state));
    }

    /// Disarm `player`'s mode (clear-on-stop).
    pub fn clear(&mut self, player: PlayerId) {
        self.modes[player.index()] = None;
        self.armed[player.index()] = None;
    }

    /// `player`'s armed mode, if any.
    #[must_use]
    pub fn mode(&self, player: PlayerId) -> Option<PassMode> {
        self.modes[player.index()]
    }

    /// `player`'s arm-time snapshot (only meaningful while a mode is armed).
    #[must_use]
    pub fn armed(&self, player: PlayerId) -> Option<&Snapshot> {
        self.armed[player.index()].as_ref()
    }
}

impl Default for PassState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_each_extracts_forced_slots() {
        assert_eq!(single_each(&[vec![1]]), Some(vec![1]));
        assert_eq!(single_each(&[vec![1], vec![2]]), Some(vec![1, 2]));
    }

    #[test]
    fn single_each_rejects_multi_empty_and_none() {
        assert_eq!(single_each(&[vec![1], vec![2, 3]]), None);
        assert_eq!(single_each::<i32>(&[vec![]]), None);
        assert_eq!(single_each::<i32>(&[]), None);
    }

    #[test]
    fn auto_answer_never_resolves_priority() {
        let p = PendingDecision::Priority {
            player: PlayerId(0),
            legal: vec![],
        };
        assert_eq!(auto_answer(&p), None);
    }

    #[test]
    fn auto_answer_ignores_non_target_kinds() {
        let p = PendingDecision::DiscardToHandSize {
            player: PlayerId(0),
            count: 1,
        };
        assert_eq!(auto_answer(&p), None);
    }

    fn snap(active: Uint, phase: Phase, turn: Uint, stack: usize) -> Snapshot {
        Snapshot {
            active: PlayerId(active),
            phase,
            turn,
            stack,
        }
    }

    #[test]
    fn yield_keeps_passing_when_nothing_changed() {
        let s = snap(1, Phase::Ending(deckmaste_core::EndingStep::End), 1, 0);
        assert!(keep_passing(PassMode::Yield, &s, &s, PlayerId(0)));
    }

    #[test]
    fn yield_stops_on_stack_growth() {
        let armed = snap(1, Phase::PrecombatMain, 1, 0);
        let now = snap(1, Phase::PrecombatMain, 1, 1);
        assert!(!keep_passing(PassMode::Yield, &armed, &now, PlayerId(0)));
    }

    #[test]
    fn yield_stops_when_combat_entered_but_not_when_armed_in_combat() {
        let pre = snap(1, Phase::PrecombatMain, 1, 0);
        let atk = snap(
            1,
            Phase::Combat(deckmaste_core::CombatStep::DeclareAttackers),
            1,
            0,
        );
        assert!(!keep_passing(PassMode::Yield, &pre, &atk, PlayerId(0)));
        let begin = snap(
            1,
            Phase::Combat(deckmaste_core::CombatStep::BeginningOfCombat),
            1,
            0,
        );
        let blk = snap(
            1,
            Phase::Combat(deckmaste_core::CombatStep::DeclareBlockers),
            1,
            0,
        );
        assert!(keep_passing(PassMode::Yield, &begin, &blk, PlayerId(0)));
    }

    #[test]
    fn yield_stops_at_my_next_precombat_main() {
        let armed = snap(1, Phase::Ending(deckmaste_core::EndingStep::End), 1, 0);
        let now = snap(0, Phase::PrecombatMain, 2, 0);
        assert!(!keep_passing(PassMode::Yield, &armed, &now, PlayerId(0)));
    }

    #[test]
    fn turn_ignores_stack_and_combat() {
        let armed = snap(1, Phase::PrecombatMain, 1, 0);
        let stack = snap(1, Phase::PrecombatMain, 1, 5);
        let combat = snap(
            1,
            Phase::Combat(deckmaste_core::CombatStep::DeclareAttackers),
            1,
            0,
        );
        assert!(keep_passing(PassMode::Turn, &armed, &stack, PlayerId(0)));
        assert!(keep_passing(PassMode::Turn, &armed, &combat, PlayerId(0)));
    }

    #[test]
    fn turn_stops_only_at_my_next_precombat_main() {
        let armed = snap(
            0,
            Phase::Beginning(deckmaste_core::BeginningStep::Upkeep),
            2,
            0,
        );
        let same_turn_main = snap(0, Phase::PrecombatMain, 2, 0);
        let next_turn_main = snap(0, Phase::PrecombatMain, 4, 0);
        assert!(keep_passing(
            PassMode::Turn,
            &armed,
            &same_turn_main,
            PlayerId(0)
        ));
        assert!(!keep_passing(
            PassMode::Turn,
            &armed,
            &next_turn_main,
            PlayerId(0)
        ));
    }

    use crate::game;

    #[test]
    fn arm_records_mode_and_snapshot_then_clear_removes_it() {
        let state = game::build_game().expect("build demo game");
        let mut pass = PassState::new();
        assert_eq!(pass.mode(PlayerId(0)), None);

        pass.arm(PlayerId(0), PassMode::Yield, &state);
        assert_eq!(pass.mode(PlayerId(0)), Some(PassMode::Yield));
        assert_eq!(
            pass.armed(PlayerId(0)).map(|s| s.turn),
            Some(state.turn.turn_number)
        );
        assert_eq!(pass.mode(PlayerId(1)), None, "arming P0 leaves P1 alone");

        pass.clear(PlayerId(0));
        assert_eq!(pass.mode(PlayerId(0)), None);
        assert!(pass.armed(PlayerId(0)).is_none());
    }

    #[test]
    fn snapshot_of_reads_live_turn_coordinates() {
        let state = game::build_game().expect("build demo game");
        let s = Snapshot::of(&state);
        assert_eq!(s.active, state.turn.active_player);
        assert_eq!(s.turn, state.turn.turn_number);
        assert_eq!(s.stack, state.stack.len());
    }
}
