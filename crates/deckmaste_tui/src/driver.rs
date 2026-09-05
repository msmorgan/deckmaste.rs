//! Drives the engine's step/submit loop, auto-resolving decisions the UI does
//! not yet handle through a `sim::Strategy`.
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionError;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameOutcome;
use deckmaste_engine::GameState;
use deckmaste_engine::StepOutcome;
use deckmaste_engine::sim::Strategy;
use deckmaste_plugin::provenance::ProvenanceIndex;

use crate::game::BuiltGame;
use crate::game::CardProvenance;
use crate::shortcuts::PassState;

/// Step budget for a full headless auto-play (and the smoke test): generous
/// enough that a healthy game finishes well within it.
pub(crate) const HEADLESS_BUDGET: usize = 100_000;

/// Why the driver stopped stepping.
#[derive(Debug)]
pub enum Stop {
    /// An interactive decision is pending (interactive mode only). Carries the
    /// pending decision so the UI can build an `Interaction` for it.
    Decision(DecisionPointKind),
    /// The game ended.
    GameOver(GameOutcome),
    /// The step budget was exhausted (headless mode only).
    Budget,
}

/// Owns the game and the auto-decider used for non-interactive decisions.
pub struct Driver {
    pub state: GameState,
    /// The semantic half of every deck card, by `CardId` — what the detail
    /// pane renders through now that lowered core carries no provenance.
    pub cards: CardProvenance,
    /// Semantic terms by lowered value, for derived state that never appeared
    /// on a card as written (granted and conferred abilities).
    pub provenance: ProvenanceIndex,
    strategy: Box<dyn Strategy>,
}

impl Driver {
    #[must_use]
    pub fn new(game: BuiltGame, strategy: Box<dyn Strategy>) -> Self {
        Self {
            state: game.state,
            cards: game.cards,
            provenance: game.provenance,
            strategy,
        }
    }

    /// The provenance a render pass needs, as one borrow.
    #[must_use]
    pub fn provenance_refs(&self) -> crate::game::ProvenanceRefs<'_> {
        crate::game::ProvenanceRefs {
            cards: &self.cards,
            index: &self.provenance,
        }
    }

    /// Escape valve: if the engine ever fails to open a decision window, return
    /// `Stop::Budget` rather than hang the UI.
    const DECISION_BUDGET: usize = 1_000_000;

    /// Steps the engine, auto-resolving (via `Strategy`) every decision for
    /// which `stop_pred` is false, and stopping on the first one for which
    /// it is true.
    ///
    /// # Errors
    /// Propagates a `DecisionError` if an auto-submitted decision is rejected
    /// (a wiring bug — `Strategy` is expected to answer legally).
    fn drive(
        &mut self,
        stop_pred: impl Fn(&DecisionPointKind) -> bool,
        budget: usize,
    ) -> Result<Stop, DecisionError> {
        for _ in 0..budget {
            match self.state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::GameOver(outcome) => return Ok(Stop::GameOver(outcome)),
                StepOutcome::NeedsDecision(pending) => {
                    if stop_pred(&pending) {
                        return Ok(Stop::Decision(pending));
                    }
                    let decision = self.strategy.decide(&self.state, &pending);
                    self.state.submit_decision(decision)?;
                }
            }
        }
        Ok(Stop::Budget)
    }

    /// Interactive: step until a human-driven decision is pending or the game
    /// ends.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn run_to_decision(&mut self) -> Result<Stop, DecisionError> {
        self.drive(crate::interact::is_interactive, Self::DECISION_BUDGET)
    }

    /// Interactive (priority only): used by the existing render/board unit
    /// tests.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    #[cfg(test)]
    pub fn run_to_priority(&mut self) -> Result<Stop, DecisionError> {
        self.drive(
            |p| {
                matches!(
                    p,
                    DecisionPointKind::Priority(deckmaste_engine::Priority { .. })
                )
            },
            Self::DECISION_BUDGET,
        )
    }

    /// Headless: auto-play both seats until game over or the step budget.
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn run_to_end(&mut self, budget: usize) -> Result<Stop, DecisionError> {
        self.drive(|_| false, budget)
    }

    /// Submit a decision, then run to the next interactive stop. The app loop
    /// now drives via [`Driver::submit_and_advance`]; this remains for unit
    /// tests.
    ///
    /// # Errors
    /// Returns the `DecisionError` if the engine rejects `decision` (e.g. an
    /// illegal selection); the caller keeps the current interaction and
    /// re-prompts.
    #[cfg(test)]
    pub fn submit(&mut self, decision: Decision) -> Result<Stop, DecisionError> {
        self.state.submit_decision(decision)?;
        self.run_to_decision()
    }

    /// Interactive with shortcuts: run to the next decision, then auto-resolve
    /// single-legal decisions and auto-pass priority for any armed `PassState`
    /// mode whose stop condition has not fired, until a genuine human decision
    /// (or game over / budget). Clears the surfaced decision's decider mode
    /// (clear-on-stop).
    ///
    /// # Errors
    /// As [`Driver::drive`].
    pub fn advance(&mut self, pass: &mut PassState) -> Result<Stop, DecisionError> {
        for _ in 0..Self::DECISION_BUDGET {
            let stop = self.run_to_decision()?;
            let Stop::Decision(pending) = &stop else {
                return Ok(stop);
            };
            // Feature 1: single-legal auto-resolve (never priority).
            if let Some(decision) = crate::shortcuts::auto_answer(pending) {
                self.state.submit_decision(decision)?;
                continue;
            }
            // Feature 2: per-player pass mode on a priority window.
            if let DecisionPointKind::Priority(deckmaste_engine::Priority { player, .. }) = pending
            {
                let player = *player;
                if let (Some(mode), Some(armed)) = (pass.mode(player), pass.armed(player)) {
                    let now = crate::shortcuts::Snapshot::of(&self.state);
                    if crate::shortcuts::keep_passing(mode, armed, &now, player) {
                        self.state.submit_decision(Decision::Act(Action::Pass))?;
                        continue;
                    }
                }
            }
            // Feature 3: auto-play the oldest land you can on your own main
            // (land drop unused). Reached only after the pass-mode check, so a
            // yielding player still plays their land when their main is reached.
            if crate::shortcuts::AUTOPLAY_LANDS
                && let DecisionPointKind::Priority(deckmaste_engine::Priority { player, legal }) =
                    pending
            {
                let player = *player;
                if let Some(action) = crate::shortcuts::oldest_playable_land(
                    &self.state.zones.hands[player.index()],
                    legal,
                ) {
                    self.state.submit_decision(Decision::Act(action))?;
                    continue;
                }
            }
            // Genuine human decision: clear the decider's mode, then surface it.
            let decider = pending.decider_player();
            pass.clear(decider);
            return Ok(stop);
        }
        Ok(Stop::Budget)
    }

    /// Submit a decision, then [`Driver::advance`].
    ///
    /// # Errors
    /// As [`Driver::advance`].
    pub fn submit_and_advance(
        &mut self,
        decision: Decision,
        pass: &mut PassState,
    ) -> Result<Stop, DecisionError> {
        self.state.submit_decision(decision)?;
        self.advance(pass)
    }

    /// Auto-tap-then-cast ([CR#106.4] runner convenience): float exactly enough
    /// mana for `player` to cast `object` by activating the land mana abilities
    /// [`GameState::autotap_for_cast`] planned, then launch the cast via
    /// [`Driver::submit_and_advance`]. Returns `Ok(None)` when nothing could be
    /// planned (the cast is illegal for a reason floating mana can't fix, or
    /// the untapped lands can't cover the cost) so the caller can show its
    /// usual "no legal action"; `Ok(Some(stop))` once the cast is on the
    /// stack and the game has advanced to the next stop. Each planned mana
    /// ability resolves stacklessly ([CR#605.3a]) and hands priority
    /// straight back to `player`, so the taps and the cast all land in one
    /// priority window.
    ///
    /// # Errors
    /// Propagates a `DecisionError` if any planned activation or the cast is
    /// rejected (a planning bug — every returned action is legal by
    /// construction).
    pub fn autotap_and_cast(
        &mut self,
        player: deckmaste_engine::PlayerId,
        object: deckmaste_engine::ObjectId,
        pass: &mut PassState,
    ) -> Result<Option<Stop>, DecisionError> {
        let Some(plan) = self.state.autotap_for_cast(player, object) else {
            return Ok(None);
        };
        for act in plan {
            self.state.submit_decision(Decision::Act(act))?;
            // Reopen the same priority window before the next submission (mana
            // abilities are stackless, so priority returns to `player`).
            match self.run_to_decision()? {
                Stop::Decision(_) => {}
                // A tap can't end the game; if it somehow did, surface that.
                other => return Ok(Some(other)),
            }
        }
        self.submit_and_advance(Decision::Act(Action::CastSpell { object }), pass)
            .map(Some)
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::sim::GreedyCreatures;

    use super::*;
    use crate::game;
    use crate::shortcuts::PassMode;

    /// First-legal answer to a surfaced non-priority decision, for driving a
    /// game in tests (mirrors the choices in interact.rs's integration
    /// test).
    fn answer(state: &GameState, pending: &DecisionPointKind) -> Decision {
        match pending {
            DecisionPointKind::ChooseTargets(deckmaste_engine::ChooseTargets { legal, .. }) => {
                Decision::Targets(legal.iter().map(|c| vec![c[0]]).collect())
            }
            DecisionPointKind::DeclareAttackers(deckmaste_engine::DeclareAttackers { .. }) => {
                Decision::Attackers(vec![])
            }
            DecisionPointKind::DeclareBlockers(deckmaste_engine::DeclareBlockers { .. }) => {
                Decision::Blocks(vec![])
            }
            // Discards now surface to the human, so the test driver answers them
            // too — drop the first `count` cards.
            DecisionPointKind::DiscardToHandSize(deckmaste_engine::DiscardToHandSize {
                player,
                count,
            })
            | DecisionPointKind::DiscardCards(deckmaste_engine::DiscardCards { player, count }) => {
                Decision::Discard(
                    state.zones.hands[player.index()]
                        .iter()
                        .copied()
                        .take(*count as usize)
                        .collect(),
                )
            }
            // Priority and every other interactive kind are handled by the caller.
            _ => Decision::Act(Action::Pass),
        }
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn advance_with_no_modes_surfaces_an_interactive_decision() {
        let game = game::build_game().expect("build demo game");
        let mut driver = Driver::new(game, Box::new(GreedyCreatures));
        let mut pass = PassState::new();
        match driver.advance(&mut pass).expect("no decision error") {
            Stop::Decision(p) => assert!(
                crate::interact::is_interactive(&p),
                "surfaced a non-interactive decision: {p:?}"
            ),
            other => panic!("expected an interactive decision at the opening, got {other:?}"),
        }
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn advance_leaves_land_plays_for_the_human() {
        use deckmaste_core::Type;

        // Reaching a main phase surfaces priority with a legal land play but
        // does not take it automatically. The loop only passes priority, so
        // the battlefield must still contain no land at that window.
        let game = game::build_game().expect("build demo game");
        let mut driver = Driver::new(game, Box::new(GreedyCreatures));
        let mut pass = PassState::new();
        let mut stop = driver.advance(&mut pass).expect("advance");
        for _ in 0..100 {
            match &stop {
                Stop::GameOver(_) | Stop::Budget => break,
                Stop::Decision(DecisionPointKind::Priority(deckmaste_engine::Priority {
                    legal,
                    ..
                })) => {
                    if legal
                        .iter()
                        .any(|action| matches!(action, Action::PlayLand { .. }))
                    {
                        let view = driver.state.layers();
                        assert!(
                            !driver
                                .state
                                .zones
                                .battlefield
                                .iter()
                                .any(|&id| view.get(id).has_type(Type::Land)),
                            "advance must leave the legal land play for the human"
                        );
                        return;
                    }
                    stop = driver
                        .submit_and_advance(Decision::Act(Action::Pass), &mut pass)
                        .expect("pass priority");
                }
                Stop::Decision(p) => {
                    let d = answer(&driver.state, p);
                    stop = driver.submit_and_advance(d, &mut pass).expect("answer");
                }
            }
        }
        panic!("never reached a priority window with a legal land play");
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn armed_pass_modes_stay_legal_and_terminate() {
        // Arm "Turn" for whoever holds priority on every priority window, and
        // make trivial-but-legal choices for combat/targets. With both seats
        // auto-passing, the game must still TERMINATE (each mode clears at its
        // own player's next precombat main — the mutual-pass guard — so turns
        // advance and the game ends, by decking if nothing else). No submission
        // may be rejected.
        let game = game::build_game().expect("build demo game");
        let mut driver = Driver::new(game, Box::new(GreedyCreatures));
        let mut pass = PassState::new();
        let mut stop = driver.advance(&mut pass).expect("no decision error");
        for _ in 0..10_000 {
            match stop {
                Stop::GameOver(_) | Stop::Budget => return, // terminated → guard works
                Stop::Decision(ref pending) => {
                    let decision =
                        if let DecisionPointKind::Priority(deckmaste_engine::Priority {
                            player,
                            ..
                        }) = pending
                        {
                            pass.arm(*player, PassMode::Turn, &driver.state);
                            Decision::Act(Action::Pass)
                        } else {
                            answer(&driver.state, pending)
                        };
                    stop = driver
                        .submit_and_advance(decision, &mut pass)
                        .expect("no decision error");
                }
            }
        }
        panic!("game did not terminate — mutual-pass guard failed");
    }

    #[cfg(feature = "slow-tests")]
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn auto_play_produces_only_legal_decisions() {
        use deckmaste_engine::sim::GreedyDemo;

        let game = game::build_game().expect("build demo game");
        let mut driver = Driver::new(game, Box::new(GreedyDemo));
        // Auto-play both seats. The point is the loop only ever submits legal
        // decisions (no DecisionError); reaching game over is a bonus, so a
        // step budget keeps the test bounded.
        let stop = driver
            .run_to_end(HEADLESS_BUDGET)
            .expect("no decision error");
        assert!(matches!(stop, Stop::GameOver(_) | Stop::Budget));
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn autotap_and_cast_floats_mana_and_lands_the_spell() {
        use deckmaste_engine::PlayerId;
        use deckmaste_engine::sim::GreedyDemo;

        // Drive the real demo to a P0 priority window where a hand spell is
        // castable ONLY via auto-tapping (no mana floated by hand), explicitly
        // playing lands as those actions become legal. Then autotap-and-cast it
        // and confirm a land got tapped and the spell left the hand — the whole
        // point: casting without a manual tap first.
        let game = game::build_game().expect("build demo game");
        let mut driver = Driver::new(game, Box::new(GreedyDemo));
        let mut pass = PassState::new();
        let me = PlayerId(0);
        let mut stop = driver.advance(&mut pass).expect("advance");
        let mut spell = None;
        for _ in 0..2000 {
            match &stop {
                Stop::GameOver(_) | Stop::Budget => break,
                Stop::Decision(DecisionPointKind::Priority(deckmaste_engine::Priority {
                    player,
                    legal,
                })) => {
                    if *player == me
                        && let Some(id) = driver.state.zones.hands[me.index()]
                            .iter()
                            .copied()
                            .find(|&id| driver.state.autotap_for_cast(me, id).is_some())
                    {
                        spell = Some(id);
                        break;
                    }
                    let action = legal
                        .iter()
                        .find(|action| matches!(action, Action::PlayLand { .. }))
                        .cloned()
                        .unwrap_or(Action::Pass);
                    stop = driver
                        .submit_and_advance(Decision::Act(action), &mut pass)
                        .expect("play land or pass");
                }
                Stop::Decision(p) => {
                    let d = answer(&driver.state, p);
                    stop = driver.submit_and_advance(d, &mut pass).expect("answer");
                }
            }
        }
        let spell = spell.expect("reached a window with an auto-tappable hand spell");

        let untapped_before = driver
            .state
            .zones
            .battlefield
            .iter()
            .filter(|&&id| {
                driver.state.objects.obj(id).controller == me
                    && !driver.state.objects.obj(id).tapped
            })
            .count();
        assert!(untapped_before > 0, "there must be an untapped land to tap");

        let next = driver
            .autotap_and_cast(me, spell, &mut pass)
            .expect("no decision error")
            .expect("a plan was executed");
        assert!(matches!(next, Stop::Decision(_) | Stop::GameOver(_)));

        // The spell left the hand (it was cast), and a land is now tapped.
        assert!(
            !driver.state.zones.hands[me.index()].contains(&spell),
            "the spell should have left the hand"
        );
        let untapped_after = driver
            .state
            .zones
            .battlefield
            .iter()
            .filter(|&&id| {
                driver.state.objects.obj(id).controller == me
                    && !driver.state.objects.obj(id).tapped
            })
            .count();
        assert!(
            untapped_after < untapped_before,
            "auto-tap must have tapped at least one land ({untapped_before} → {untapped_after})"
        );
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn run_to_decision_stops_on_an_interactive_kind() {
        let game = game::build_game().expect("build demo game");
        let mut driver = Driver::new(game, Box::new(GreedyCreatures));
        let stop = driver.run_to_decision().expect("no decision error");
        match stop {
            Stop::Decision(p) => assert!(
                crate::interact::is_interactive(&p),
                "stopped on a non-interactive decision: {p:?}"
            ),
            other => panic!("expected an interactive decision at the opening, got {other:?}"),
        }
    }
}
