//! Condition evaluation ([CR#603.4] intervening-if, [CR#602.5b] activation
//! restrictions). The `todo!` arms are the `engine-trigger-conditions` /
//! `engine-resolve-counts` / `engine-filter-breadth` seams — they widen this
//! dispatch rather than growing a second evaluator.

use deckmaste_core::Cmp;
use deckmaste_core::Condition;
use deckmaste_core::Count;
use deckmaste_core::Filter;
use deckmaste_core::Phase;
use deckmaste_core::StateFilter;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::player::PlayerId;
use crate::state::GameState;

impl GameState {
    /// Evaluate a `Condition` against the current game state, where `you` is
    /// the evaluating player (the ability's controller — the "you" of
    /// `YourTurn` and similar self-referential conditions).
    ///
    /// `you` is the ability's controller *at the moment of evaluation* — the
    /// activating player at gate time ([CR#602.5b]), or the stack entry's
    /// controller at resolution for an intervening-if ([CR#603.4]).
    pub(crate) fn condition_holds(&self, cond: &Condition, you: PlayerId) -> bool {
        match cond {
            // "if you control a creature" / "if a creature is on the battlefield"
            Condition::Exists(filter) => !crate::target::candidates(self, filter).is_empty(),

            // "if it is a [filter]" — Is(ref, filter): look up the ref and test.
            // Not reached by any Stage-3 fixture; wired as a seam.
            Condition::Is(_, _) => todo!("stage 3 does not evaluate Condition::Is"),

            // Numeric comparison.
            Condition::Compare(a, op, b) => {
                let lhs = self.eval_const_count(a);
                let rhs = self.eval_const_count(b);
                match op {
                    Cmp::Eq => lhs == rhs,
                    Cmp::AtLeast => lhs >= rhs,
                    Cmp::AtMost => lhs <= rhs,
                    Cmp::Greater => lhs > rhs,
                    Cmp::Less => lhs < rhs,
                }
            }

            Condition::AllOf(cs) => cs.iter().all(|c| self.condition_holds(c, you)),
            Condition::OneOf(cs) => cs.iter().any(|c| self.condition_holds(c, you)),
            Condition::Not(c) => !self.condition_holds(c, you),

            // Look through a macro.
            Condition::Expanded(e) => self.condition_holds(&e.value, you),

            Condition::Happened { .. } => todo!("stage 3 does not evaluate Condition::Happened"),

            // It is the evaluating player's turn.
            Condition::YourTurn => self.turn.active_player == you,

            // The current phase/step is exactly the given one.
            Condition::DuringPhase(p) => self.turn.current == *p,
        }
    }

    /// Frame-free `Count` evaluation for condition contexts. Unify with
    /// `resolve`'s frame-aware count evaluation when frames thread through
    /// conditions (`engine-resolve-counts`).
    fn eval_const_count(&self, count: &Count) -> Uint {
        match count {
            Count::Literal(n) => *n,
            Count::CountOf(f) => match &**f {
                // The Stack census includes the in-flight announce slot: an
                // announced spell is already in the stack ZONE before its
                // entry commits ([CR#601.2a]).
                Filter::State(StateFilter::InZone(Zone::Stack)) => {
                    Uint::try_from(self.stack.len() + usize::from(self.announcing.is_some()))
                        .expect("stack size fits in Uint")
                }
                other => todo!("engine-filter-breadth: CountOf({other:?}) in conditions"),
            },
            other => todo!("engine-resolve-counts: {other:?} in conditions"),
        }
    }

    /// [CR#307.1,601.3a]: `player` could cast a sorcery right now — their
    /// turn, a main phase, stack (and announce slot) empty. The same facts
    /// the builtin `SorcerySpeed` condition macro reads; `kw-flash`'s
    /// `May(Cast(window: InstantSpeed))` will relax the spell-side caller.
    #[must_use]
    pub(crate) fn sorcery_speed_ok(&self, player: PlayerId) -> bool {
        player == self.turn.active_player
            && matches!(
                self.turn.current,
                Phase::PrecombatMain | Phase::PostcombatMain
            )
            && self.stack.is_empty()
            && self.announcing.is_none()
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::BeginningStep;
    use deckmaste_core::Cmp;
    use deckmaste_core::Condition;
    use deckmaste_core::Count;
    use deckmaste_core::Filter;
    use deckmaste_core::Phase;
    use deckmaste_core::StateFilter;
    use deckmaste_core::Zone;

    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        })
    }

    /// `YourTurn` is true for the active player, false for the other.
    /// `DuringPhase` matches exactly the current phase and no other.
    #[test]
    fn your_turn_and_phase() {
        let mut state = game();
        state.turn.active_player = PlayerId(0);
        state.turn.current = Phase::PrecombatMain;

        // YourTurn
        assert!(
            state.condition_holds(&Condition::YourTurn, PlayerId(0)),
            "YourTurn should hold for the active player"
        );
        assert!(
            !state.condition_holds(&Condition::YourTurn, PlayerId(1)),
            "YourTurn should not hold for the non-active player"
        );

        // DuringPhase — exact match
        assert!(
            state.condition_holds(&Condition::DuringPhase(Phase::PrecombatMain), PlayerId(0)),
            "DuringPhase(PrecombatMain) should hold during PrecombatMain"
        );
        assert!(
            !state.condition_holds(&Condition::DuringPhase(Phase::PostcombatMain), PlayerId(0)),
            "DuringPhase(PostcombatMain) should not hold during PrecombatMain"
        );
    }

    /// `Compare(CountOf(InZone(Stack)), Eq, Literal(0))` is the core of the
    /// builtin `SorcerySpeed` macro. Fresh game has an empty stack and no
    /// announce slot, so the condition holds. An in-flight announce makes it
    /// false — the announce slot counts as a stack occupant
    /// ([CR#601.2a]).
    #[test]
    fn compare_counts_stack_census() {
        let mut state = game();
        let cond = Condition::Compare(
            Count::CountOf(Box::new(Filter::State(StateFilter::InZone(Zone::Stack)))),
            Cmp::Eq,
            Count::Literal(0),
        );
        // Fresh game: stack empty, no announce slot.
        assert!(
            state.condition_holds(&cond, PlayerId(0)),
            "Compare(CountOf(InZone(Stack)), Eq, Literal(0)) should hold on a fresh game (stack empty)"
        );

        // In-flight announce: the slot counts as a stack occupant.
        let spell = state.objects.mint(
            crate::object::ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(deckmaste_core::Zone::Stack),
        );
        state.announcing = Some(crate::stack::PendingStackEntry {
            object: crate::stack::StackObject::Spell(spell),
            controller: PlayerId(0),
            origin: deckmaste_core::Zone::Hand,
            targets: vec![],
        });
        assert!(
            !state.condition_holds(&cond, PlayerId(0)),
            "Compare(CountOf(InZone(Stack)), Eq, Literal(0)) should be false with an in-flight announce"
        );
    }

    /// Combinators: `Not(AllOf([OneOf([])]))` is true because `OneOf([])` is
    /// vacuously false → `AllOf` of a false is false → `Not` of false is true.
    #[test]
    fn combinators() {
        let state = game();
        let p = PlayerId(0);
        let cond = Condition::Not(Box::new(Condition::AllOf(vec![Condition::OneOf(vec![])])));
        assert!(
            state.condition_holds(&cond, p),
            "Not(AllOf([OneOf([])])) should be true (vacuous OneOf false → AllOf false → Not true)"
        );
    }

    /// `sorcery_speed_ok` is gated by active player, main phase, and empty
    /// stack/announce.
    #[test]
    fn sorcery_speed_ok_gates() {
        let mut state = game();
        state.turn.active_player = PlayerId(0);
        state.turn.current = Phase::PrecombatMain;

        assert!(
            state.sorcery_speed_ok(PlayerId(0)),
            "sorcery_speed_ok should be true for active player in main phase with empty stack"
        );
        assert!(
            !state.sorcery_speed_ok(PlayerId(1)),
            "sorcery_speed_ok should be false for non-active player"
        );

        // Wrong phase
        state.turn.current = Phase::Beginning(BeginningStep::Upkeep);
        assert!(
            !state.sorcery_speed_ok(PlayerId(0)),
            "sorcery_speed_ok should be false outside main phases"
        );
    }
}
