//! The pure, in-progress human choice servicing the current interactive
//! `DecisionPointKind`. No engine mutation, no ratatui — unit-tested headlessly.
//! Owns every selection invariant (target caps, blocker pairing); the driver
//! uses [`is_interactive`] to decide what to surface vs auto-resolve.
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::DecisionPointKind;
use deckmaste_engine::GameState;
use deckmaste_engine::LayeredView;
use deckmaste_engine::ObjectId;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;

use crate::shortcuts::PassMode;
use crate::shortcuts::PassState;
use crate::ui::BoardState;

/// The in-progress selection for the decision currently shown to the human.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interaction {
    /// A priority window. Acting is object-first (handled in `app` from the
    /// cursor); `sub` is the disambiguation popup when one object has >1
    /// action.
    Priority { sub: Option<AbilityPick> },
    /// Choose targets ([CR#601.2c]): a SET per spec slot. `legal[i]` is slot
    /// i's candidate set; `chosen[i]` is the picked set (toggle to add/remove —
    /// a plural slot takes several). Counts and cross-slot distinctness are the
    /// engine's to enforce at submission; the picker doesn't narrow, so
    /// over-picking is simply rejected on confirm.
    Targets {
        legal: Vec<Vec<ObjectId>>,
        chosen: Vec<Vec<ObjectId>>,
        active: usize,
    },
    /// Declare attackers: any subset of `legal`. `defender` is the defending
    /// player's proxy object — this UI attacks the player by default; a
    /// planeswalker-target picker is a follow-up ([CR#508.1b]).
    Attackers {
        legal: Vec<ObjectId>,
        chosen: Vec<ObjectId>,
        defender: ObjectId,
    },
    /// Declare blockers: `(blocker, attacker)` pairs. `pending` is a blocker
    /// awaiting the attacker it blocks. Attacker candidates come from the live
    /// combat state (derived in `app`/`ui`), not stored here.
    Blockers {
        legal: Vec<ObjectId>,
        pairs: Vec<(ObjectId, ObjectId)>,
        pending: Option<ObjectId>,
    },
    /// Discard: choose exactly `count` cards to discard from `legal` (the
    /// player's hand). Serves both the cleanup hand-size discard ([CR#514.1])
    /// and an effect-instructed discard ([CR#701.9b]). `chosen` is the
    /// in-progress selection, capped at `count`.
    Discard {
        legal: Vec<ObjectId>,
        chosen: Vec<ObjectId>,
        count: usize,
    },
}

/// Disambiguation popup: the legal actions on one selected object, when there
/// is more than one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityPick {
    pub object: ObjectId,
    pub actions: Vec<Action>,
    pub sel: usize,
}

/// Borrowed context [`Interaction::on_key`] needs beyond `self`: the mutable
/// board/pass state a few arms act on (e.g. the Blockers arm's steering),
/// plus read-only snapshots of the live game state, view, board cursor, and
/// (for object-first priority) the legal action list. Deliberately narrower
/// than `interactive_loop`'s full local set — only what the per-variant arms
/// actually touch; `app` still owns the shared-navigation and global-key
/// handling around the call.
pub struct KeyCtx<'a> {
    pub board: &'a mut BoardState,
    pub pass: &'a mut PassState,
    pub state: &'a GameState,
    pub view: &'a LayeredView,
    /// The cursor's object, if the focused board selection is one.
    pub cursor: Option<ObjectId>,
    /// The legal priority actions for the current stop, present iff it's a
    /// `Priority` decision — needed to resolve object-first Enter.
    pub legal: Option<&'a [Action]>,
}

/// What handling a key produced for the decision currently shown: at most one
/// submitted [`Decision`], a replacement [`Interaction`] (e.g. opening the
/// ability popup), an auto-tap-then-cast request, or an error message —
/// mirroring the loop locals `interactive_loop`'s dispatch match used to set
/// directly before this logic moved onto `Interaction`.
#[derive(Debug, Default)]
pub struct KeyOutcome {
    pub submit: Option<Decision>,
    pub replace: Option<Interaction>,
    pub autotap_cast: Option<ObjectId>,
    pub error: Option<String>,
}

/// The object an action concerns, if any (`None` for Pass/Concede/Special).
#[must_use]
pub fn action_object(action: &Action) -> Option<ObjectId> {
    match action {
        Action::PlayLand { object }
        | Action::CastSpell { object }
        | Action::ActivateAbility { object, .. } => Some(*object),
        Action::Pass | Action::Concede | Action::Special(_) => None,
    }
}

/// The legal priority actions that concern `object`, in `legal` order.
#[must_use]
pub fn actions_for(object: ObjectId, legal: &[Action]) -> Vec<Action> {
    legal
        .iter()
        .filter(|a| action_object(a) == Some(object))
        .cloned()
        .collect()
}

/// Whether the human should drive this decision (vs the driver auto-resolving
/// it via `Strategy`). Combat with an empty candidate set has nothing to
/// choose, so it auto-resolves. An attacker choice also needs a defending
/// target; a malformed decision must not reach the picker and panic. The
/// single source of truth for the driver partition.
#[must_use]
pub fn is_interactive(pending: &DecisionPointKind) -> bool {
    match pending {
        DecisionPointKind::Priority(deckmaste_engine::Priority { .. })
        | DecisionPointKind::ChooseTargets(deckmaste_engine::ChooseTargets { .. })
        | DecisionPointKind::Retarget(deckmaste_engine::Retarget { .. }) => true,
        DecisionPointKind::DeclareAttackers(deckmaste_engine::DeclareAttackers {
            legal,
            legal_targets,
            ..
        }) => !legal.is_empty() && !legal_targets.is_empty(),
        DecisionPointKind::DeclareBlockers(deckmaste_engine::DeclareBlockers { legal, .. }) => {
            !legal.is_empty()
        }
        // The human picks which cards to discard — cleanup hand-size and
        // effect-instructed discards alike. `count` is always > 0 when the
        // engine surfaces one; guard anyway so a degenerate 0 auto-resolves.
        DecisionPointKind::DiscardToHandSize(deckmaste_engine::DiscardToHandSize {
            count, ..
        })
        | DecisionPointKind::DiscardCards(deckmaste_engine::DiscardCards { count, .. }) => {
            *count > 0
        }
        _ => false,
    }
}

impl Interaction {
    /// Build the initial interaction for `pending`, or `None` when the human
    /// does not drive it here: an auto-resolved kind, an empty-candidate combat
    /// step, or a discard — whose candidate set is the player's hand (read from
    /// state), so the app builds it via [`Interaction::for_discard`].
    #[must_use]
    pub fn for_decision(pending: &DecisionPointKind) -> Option<Self> {
        Some(match pending {
            DecisionPointKind::Priority(deckmaste_engine::Priority { .. }) => Interaction::Priority { sub: None },
            DecisionPointKind::ChooseTargets(deckmaste_engine::ChooseTargets { legal, .. })
            // [CR#707.10c]: re-targeting a committed entry is the same
            // pick-one-per-spec interaction as an initial `ChooseTargets` —
            // `legal[i]` already includes the entry's current target, so
            // "leave it as-is" is always a candidate.
            | DecisionPointKind::Retarget(deckmaste_engine::Retarget { legal, .. }) => Interaction::Targets {
                legal: legal.clone(),
                chosen: vec![Vec::new(); legal.len()],
                active: 0,
            },
            DecisionPointKind::DeclareAttackers(deckmaste_engine::DeclareAttackers {
                legal,
                legal_targets,
                ..
            }) if !legal.is_empty() && !legal_targets.is_empty() => Interaction::Attackers {
                legal: legal.clone(),
                chosen: Vec::new(),
                // The defending player's proxy is normally the first legal
                // target ([CR#508.1b]); default every attacker to it. Keep the
                // lookup fallible so malformed pending state cannot crash the
                // TUI.
                defender: legal_targets.first().copied()?,
            },
            DecisionPointKind::DeclareBlockers(deckmaste_engine::DeclareBlockers { legal, .. }) if !legal.is_empty() => {
                Interaction::Blockers {
                    legal: legal.clone(),
                    pairs: Vec::new(),
                    pending: None,
                }
            }
            _ => return None,
        })
    }

    /// Build a discard picker over `hand`, requiring exactly `count` cards. The
    /// app supplies the hand because a discard decision names only the player
    /// and the count — the candidate cards are that player's whole hand.
    #[must_use]
    pub fn for_discard(hand: &[ObjectId], count: usize) -> Self {
        Interaction::Discard {
            legal: hand.to_vec(),
            chosen: Vec::new(),
            count,
        }
    }

    /// True for the board-dimming pick modes (everything but `Priority`).
    #[must_use]
    pub fn is_pick_mode(&self) -> bool {
        !matches!(self, Interaction::Priority { .. })
    }

    /// Candidate ids selectable for the current step (the active spec's set for
    /// Targets, the legal pool for Attackers, the unpaired legal blockers for
    /// Blockers when no pairing is in progress). Empty for `Priority`. Blocker
    /// *attacker* candidates (pairing step) are derived from combat in
    /// `app`/`ui`.
    #[must_use]
    pub fn candidates(&self) -> Vec<ObjectId> {
        match self {
            Interaction::Priority { .. } => Vec::new(),
            Interaction::Targets { legal, active, .. } => {
                legal.get(*active).cloned().unwrap_or_default()
            }
            // Attackers and Discard are both "toggle a subset of these ids":
            // every listed id is a candidate (the caps are enforced in `toggle`,
            // not by hiding candidates, so chosen cards stay toggle-off-able).
            Interaction::Attackers { legal, .. } | Interaction::Discard { legal, .. } => {
                legal.clone()
            }
            Interaction::Blockers {
                legal,
                pairs,
                pending,
            } => {
                if pending.is_some() {
                    Vec::new()
                } else {
                    legal
                        .iter()
                        .copied()
                        .filter(|id| !pairs.iter().any(|(b, _)| b == id))
                        .collect()
                }
            }
        }
    }

    /// Whether `id` is selectable right now.
    #[must_use]
    pub fn is_candidate(&self, id: ObjectId) -> bool {
        self.candidates().contains(&id)
    }

    /// Whether `id` is part of the committed selection (rendered with a ✓).
    #[must_use]
    pub fn is_chosen(&self, id: ObjectId) -> bool {
        match self {
            Interaction::Priority { .. } => false,
            Interaction::Targets { chosen, .. } => chosen.iter().flatten().any(|&c| c == id),
            Interaction::Attackers { chosen, .. } | Interaction::Discard { chosen, .. } => {
                chosen.contains(&id)
            }
            Interaction::Blockers { pairs, pending, .. } => {
                *pending == Some(id) || pairs.iter().any(|(b, _)| *b == id)
            }
        }
    }

    /// Toggle `id` for the current step, enforcing legality and per-step caps.
    /// - Targets: sets the active spec's pick; toggling the same id clears it;
    ///   a different id is refused while the active spec already has a pick
    ///   (untoggle first — the prescribed-count cap).
    /// - Attackers: add/remove from the set (no cap).
    /// - Blockers: start (or cancel) a pairing for a legal, unpaired blocker.
    /// - Discard: add/remove from the set, capped at `count` (untoggle to
    ///   swap).
    /// - Priority: no-op.
    pub fn toggle(&mut self, id: ObjectId) {
        match self {
            Interaction::Priority { .. } => {}
            Interaction::Targets {
                legal,
                chosen,
                active,
            } => {
                let active = *active;
                if !legal.get(active).is_some_and(|c| c.contains(&id)) {
                    return;
                }
                // Add/remove `id` in the active slot's set — a plural slot takes
                // several. The engine enforces count/distinctness at submit, so
                // the picker doesn't cap.
                let slot = &mut chosen[active];
                if let Some(pos) = slot.iter().position(|&c| c == id) {
                    slot.remove(pos);
                } else {
                    slot.push(id);
                }
            }
            Interaction::Attackers { legal, chosen, .. } => {
                if !legal.contains(&id) {
                    return;
                }
                if let Some(pos) = chosen.iter().position(|&c| c == id) {
                    chosen.remove(pos);
                } else {
                    chosen.push(id);
                }
            }
            Interaction::Blockers {
                legal,
                pairs,
                pending,
            } => {
                if *pending == Some(id) {
                    *pending = None; // cancel the in-progress pairing
                } else if pending.is_none()
                    && legal.contains(&id)
                    && !pairs.iter().any(|(b, _)| *b == id)
                {
                    *pending = Some(id);
                }
            }
            // Add/remove from the set; refuse a new pick once `count` are
            // chosen (untoggle one first — the exact-count cap).
            Interaction::Discard {
                legal,
                chosen,
                count,
            } => {
                if !legal.contains(&id) {
                    return;
                }
                if let Some(pos) = chosen.iter().position(|&c| c == id) {
                    chosen.remove(pos);
                } else if chosen.len() < *count {
                    chosen.push(id);
                }
            }
        }
    }

    /// Targets only: move to the next spec still needing a pick (wrapping
    /// forward). No-op for other variants or when every spec is satisfied.
    pub fn advance(&mut self) {
        if let Interaction::Targets { chosen, active, .. } = self {
            let n = chosen.len();
            for step in 1..=n {
                let i = (*active + step) % n;
                if chosen[i].is_empty() {
                    *active = i;
                    return;
                }
            }
        }
    }

    /// Reset the in-progress selection to empty (keeps the kind).
    pub fn cancel(&mut self) {
        match self {
            Interaction::Priority { sub } => *sub = None,
            Interaction::Targets { chosen, active, .. } => {
                for c in chosen.iter_mut() {
                    c.clear();
                }
                *active = 0;
            }
            Interaction::Attackers { chosen, .. } | Interaction::Discard { chosen, .. } => {
                chosen.clear();
            }
            Interaction::Blockers { pairs, pending, .. } => {
                pairs.clear();
                *pending = None;
            }
        }
    }

    /// The completed `Decision`, iff this interaction is complete and valid.
    /// `Priority` never confirms here (priority actions are submitted directly
    /// by `app` from the cursor / ability popup).
    #[must_use]
    pub fn confirm(&self) -> Option<Decision> {
        match self {
            Interaction::Priority { .. } => None,
            // Confirm once every slot holds at least one pick — a minimal
            // completeness gate; the engine enforces the exact per-slot counts
            // and cross-slot distinctness on submission ([CR#601.2c,115.7e]).
            Interaction::Targets { chosen, .. } => chosen
                .iter()
                .all(|slot| !slot.is_empty())
                .then(|| Decision::Targets(chosen.clone())),
            // [CR#508.1b]: every attacker attacks the defending player (the
            // proxy stored at build time); a planeswalker picker is a follow-up.
            Interaction::Attackers {
                chosen, defender, ..
            } => Some(Decision::Attackers(
                chosen.iter().map(|&a| (a, *defender)).collect(),
            )),
            Interaction::Blockers { pairs, pending, .. } => {
                if pending.is_some() {
                    None // finish the in-progress pairing first
                } else {
                    Some(Decision::Blocks(pairs.clone()))
                }
            }
            // Confirm only once exactly `count` cards are chosen — the engine
            // requires exactly that many distinct in-hand cards ([CR#701.9b]).
            Interaction::Discard { chosen, count, .. } => {
                (chosen.len() == *count).then(|| Decision::Discard(chosen.clone()))
            }
        }
    }

    /// Blockers only: record that the pending blocker blocks `attacker`, then
    /// clear the pending blocker. No-op when no pairing is in progress.
    pub fn pair_with(&mut self, attacker: ObjectId) {
        if let Interaction::Blockers { pairs, pending, .. } = self
            && let Some(blocker) = pending.take()
        {
            pairs.push((blocker, attacker));
        }
    }

    /// Blockers only: undo the in-progress pairing if any, else remove the
    /// last recorded pair.
    pub fn unpair_last(&mut self) {
        if let Interaction::Blockers { pairs, pending, .. } = self {
            if pending.is_some() {
                *pending = None;
            } else {
                pairs.pop();
            }
        }
    }

    /// Map a key event to an outcome for the decision currently shown. Owns
    /// the per-variant key semantics that lived in `interactive_loop`'s
    /// dispatch match: `Enter`/`Esc`/`Space`/`Backspace` act on the variant's
    /// own selection state; every other key is a no-op (the caller runs
    /// global-key handling BEFORE this dispatch, so those keys never reach
    /// here).
    #[must_use]
    pub fn on_key(&mut self, key: KeyEvent, ctx: &mut KeyCtx) -> KeyOutcome {
        let mut outcome = KeyOutcome::default();
        match self {
            // ---- Priority, ability popup open ----
            Interaction::Priority { sub: Some(pick) } => match key.code {
                KeyCode::Enter => {
                    outcome.submit = Some(Decision::Act(pick.actions[pick.sel].clone()));
                }
                KeyCode::Esc => outcome.replace = Some(Interaction::Priority { sub: None }),
                _ => {}
            },
            // ---- Priority, object-first ----
            // Pass is `a`, not Space — Space is the giant easy-to-fat-finger key
            // and still toggles selections in the pick modes below, so binding
            // priority-pass off it stops accidental advances.
            Interaction::Priority { sub: None } => match key.code {
                KeyCode::Char('a') | KeyCode::F(2) => {
                    outcome.submit = Some(Decision::Act(Action::Pass));
                }
                KeyCode::Char('y') | KeyCode::F(4) => {
                    ctx.pass
                        .arm(ctx.board.perspective, PassMode::Yield, ctx.state);
                    outcome.submit = Some(Decision::Act(Action::Pass));
                }
                KeyCode::Char('P') | KeyCode::F(6) => {
                    ctx.pass
                        .arm(ctx.board.perspective, PassMode::Turn, ctx.state);
                    outcome.submit = Some(Decision::Act(Action::Pass));
                }
                KeyCode::Enter => match (ctx.cursor, ctx.legal) {
                    (Some(id), Some(legal)) => {
                        let acts = actions_for(id, legal);
                        match acts.len() {
                            // No legal action right now — but a spell may be
                            // castable if its mana were floated. Defer to the
                            // autotapper (run after the match); it no-ops back
                            // to the error message when it can't cover the cost.
                            0 => outcome.autotap_cast = Some(id),
                            1 => outcome.submit = Some(Decision::Act(acts[0].clone())),
                            _ => {
                                outcome.replace = Some(Interaction::Priority {
                                    sub: Some(AbilityPick {
                                        object: id,
                                        actions: acts,
                                        sel: 0,
                                    }),
                                });
                            }
                        }
                    }
                    _ => outcome.error = Some("select a card or permanent first".to_string()),
                },
                _ => {}
            },
            // ---- Targets ----
            it @ Interaction::Targets { .. } => match key.code {
                KeyCode::Char(' ') => {
                    if let Some(id) = ctx.cursor {
                        it.toggle(id);
                    }
                }
                KeyCode::Enter => {
                    if let Some(d) = it.confirm() {
                        outcome.submit = Some(d);
                    } else {
                        it.advance();
                    }
                }
                KeyCode::Esc => it.cancel(),
                _ => {}
            },
            // ---- Attackers / Discard (toggle a subset of the dimmed board,
            //      then submit; the cursor's object is the one toggled) ----
            it @ (Interaction::Attackers { .. } | Interaction::Discard { .. }) => match key.code {
                KeyCode::Char(' ') => {
                    if let Some(id) = ctx.cursor {
                        it.toggle(id);
                    }
                }
                KeyCode::Enter => outcome.submit = it.confirm(),
                KeyCode::Esc => it.cancel(),
                _ => {}
            },
            // ---- Blockers ----
            it @ Interaction::Blockers { .. } => {
                let pairing = matches!(
                    it,
                    Interaction::Blockers {
                        pending: Some(_),
                        ..
                    }
                );
                match key.code {
                    KeyCode::Char(' ') if !pairing => {
                        if let Some(id) = ctx.cursor {
                            it.toggle(id);
                            // Pairing just started: steer to the live attackers
                            // (which aren't in `candidates()`).
                            if matches!(
                                it,
                                Interaction::Blockers {
                                    pending: Some(_),
                                    ..
                                }
                            ) && let Some(&atk) = ctx.state.combat.attackers().first()
                            {
                                ctx.board.steer_to(atk, ctx.state, ctx.view);
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if pairing {
                            if let Some(id) = ctx.cursor {
                                it.pair_with(id);
                                // Back to the defender's remaining blockers.
                                if let Some(&next) = it.candidates().first() {
                                    ctx.board.steer_to(next, ctx.state, ctx.view);
                                }
                            }
                        } else if let Some(d) = it.confirm() {
                            outcome.submit = Some(d);
                        }
                    }
                    KeyCode::Backspace => it.unpair_last(),
                    KeyCode::Esc => it.cancel(),
                    _ => {}
                }
            }
        }
        outcome
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::Action;
    use deckmaste_engine::DecisionPointKind;
    use deckmaste_engine::ObjectId;
    use deckmaste_engine::PlayerId;

    use super::*;
    use crate::game;

    /// A few distinct real `ObjectId`s (no public constructor exists).
    fn ids() -> Vec<ObjectId> {
        let state = game::build_game().expect("build").state;
        state.zones.libraries[0].iter().copied().collect()
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn action_object_extracts_the_object_or_none() {
        let id = ids()[0];
        assert_eq!(action_object(&Action::CastSpell { object: id }), Some(id));
        assert_eq!(action_object(&Action::PlayLand { object: id }), Some(id));
        assert_eq!(
            action_object(&Action::ActivateAbility {
                object: id,
                ability: 0
            }),
            Some(id)
        );
        assert_eq!(action_object(&Action::Pass), None);
        assert_eq!(action_object(&Action::Concede), None);
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn actions_for_filters_to_the_selected_object() {
        let v = ids();
        let (a, b) = (v[0], v[1]);
        let legal = vec![
            Action::Pass,
            Action::CastSpell { object: a },
            Action::ActivateAbility {
                object: a,
                ability: 0,
            },
            Action::PlayLand { object: b },
        ];
        let for_a = actions_for(a, &legal);
        assert_eq!(for_a.len(), 2);
        assert!(for_a.iter().all(|x| action_object(x) == Some(a)));
        assert_eq!(actions_for(b, &legal).len(), 1);
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn partition_surfaces_priority_targets_and_nonempty_combat() {
        let v = ids();
        let prio = DecisionPointKind::Priority(deckmaste_engine::Priority {
            player: PlayerId(0),
            legal: vec![Action::Pass],
        });
        assert!(is_interactive(&prio));
        let atk = DecisionPointKind::DeclareAttackers(deckmaste_engine::DeclareAttackers {
            player: PlayerId(0),
            legal: vec![v[0]],
            legal_targets: vec![v[1]],
        });
        assert!(is_interactive(&atk));
        // Empty combat = nothing to choose = auto-resolved.
        let empty = DecisionPointKind::DeclareAttackers(deckmaste_engine::DeclareAttackers {
            player: PlayerId(0),
            legal: vec![],
            legal_targets: vec![v[1]],
        });
        assert!(!is_interactive(&empty));
        assert!(Interaction::for_decision(&empty).is_none());
        // Discards now surface to the human — the picker is built from the hand.
        let discard = DecisionPointKind::DiscardCards(deckmaste_engine::DiscardCards {
            player: PlayerId(0),
            count: 1,
        });
        assert!(is_interactive(&discard));
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn discard_is_interactive_and_built_over_the_hand() {
        let v = ids();
        // Both the cleanup hand-size discard and an effect discard surface.
        assert!(is_interactive(&DecisionPointKind::DiscardToHandSize(
            deckmaste_engine::DiscardToHandSize {
                player: PlayerId(0),
                count: 1,
            }
        )));
        assert!(is_interactive(&DecisionPointKind::DiscardCards(
            deckmaste_engine::DiscardCards {
                player: PlayerId(0),
                count: 2,
            }
        )));
        let it = Interaction::for_discard(&[v[0], v[1], v[2]], 2);
        assert_eq!(it.candidates(), vec![v[0], v[1], v[2]]);
        assert!(it.is_pick_mode());
        assert!(it.confirm().is_none(), "nothing chosen yet");
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn discard_caps_at_count_and_confirms_exactly_count_cards() {
        let v = ids();
        let (a, b, c) = (v[0], v[1], v[2]);
        let mut it = Interaction::for_discard(&[a, b, c], 2);
        it.toggle(a);
        it.toggle(b);
        assert!(it.is_chosen(a) && it.is_chosen(b));
        it.toggle(c); // refused — already at the cap of 2
        assert!(!it.is_chosen(c));
        assert_eq!(it.confirm(), Some(Decision::Discard(vec![a, b])));
        it.toggle(a); // untoggle one → no longer exactly 2
        assert!(it.confirm().is_none());
        it.toggle(c); // now [b, c]
        assert_eq!(it.confirm(), Some(Decision::Discard(vec![b, c])));
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn discard_ignores_non_hand_ids_and_cancel_clears() {
        let v = ids();
        let (a, off) = (v[0], v[3]);
        let mut it = Interaction::for_discard(&[a], 1);
        it.toggle(off); // not in hand — ignored
        assert!(!it.is_chosen(off));
        it.toggle(a);
        assert_eq!(it.confirm(), Some(Decision::Discard(vec![a])));
        it.cancel();
        assert!(it.confirm().is_none(), "cancel clears the selection");
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn for_decision_builds_one_target_slot_per_spec() {
        let v = ids();
        let pending = DecisionPointKind::ChooseTargets(deckmaste_engine::ChooseTargets {
            player: PlayerId(0),
            spec: vec![],
            legal: vec![vec![v[0], v[1]], vec![v[2]]],
        });
        match Interaction::for_decision(&pending).expect("interactive") {
            Interaction::Targets {
                legal,
                chosen,
                active,
            } => {
                assert_eq!(legal.len(), 2);
                assert_eq!(chosen, vec![Vec::<ObjectId>::new(), Vec::new()]);
                assert_eq!(active, 0);
            }
            other => panic!("expected Targets, got {other:?}"),
        }
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn targets_toggle_adds_and_removes_within_a_slot() {
        let v = ids();
        let (a, b) = (v[0], v[1]);
        let mut it = Interaction::for_decision(&DecisionPointKind::ChooseTargets(
            deckmaste_engine::ChooseTargets {
                player: PlayerId(0),
                spec: vec![],
                legal: vec![vec![a, b]],
            },
        ))
        .expect("interactive");
        assert!(it.confirm().is_none()); // nothing chosen yet
        it.toggle(a);
        assert!(it.is_chosen(a));
        // A slot takes several — the engine caps counts at submission, so the
        // picker adds rather than refusing (a plural "1–3 targets" slot).
        it.toggle(b);
        assert!(it.is_chosen(a) && it.is_chosen(b));
        assert_eq!(it.confirm(), Some(Decision::Targets(vec![vec![a, b]])));
        it.toggle(a); // remove a
        assert!(it.is_chosen(b) && !it.is_chosen(a));
        assert_eq!(it.confirm(), Some(Decision::Targets(vec![vec![b]])));
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn targets_advance_walks_specs_and_confirms_in_order() {
        let v = ids();
        let (a, b, c) = (v[0], v[1], v[2]);
        let mut it = Interaction::for_decision(&DecisionPointKind::ChooseTargets(
            deckmaste_engine::ChooseTargets {
                player: PlayerId(0),
                spec: vec![],
                legal: vec![vec![a, b], vec![c]],
            },
        ))
        .expect("interactive");
        it.toggle(a); // spec 0 := a
        assert!(it.confirm().is_none()); // spec 1 still empty
        it.advance(); // move to spec 1
        it.toggle(c); // spec 1 := c
        assert_eq!(
            it.confirm(),
            Some(Decision::Targets(vec![vec![a], vec![c]]))
        );
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn targets_ignores_non_candidates() {
        let v = ids();
        let (a, off) = (v[0], v[3]);
        let mut it = Interaction::for_decision(&DecisionPointKind::ChooseTargets(
            deckmaste_engine::ChooseTargets {
                player: PlayerId(0),
                spec: vec![],
                legal: vec![vec![a]],
            },
        ))
        .expect("interactive");
        it.toggle(off); // not a candidate
        assert!(!it.is_chosen(off));
        assert!(it.confirm().is_none());
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn attackers_toggle_is_a_free_subset_and_confirms_any_set() {
        let v = ids();
        let (a, b, off, def) = (v[0], v[1], v[3], v[2]);
        let mut it = Interaction::for_decision(&DecisionPointKind::DeclareAttackers(
            deckmaste_engine::DeclareAttackers {
                player: PlayerId(0),
                legal: vec![a, b],
                legal_targets: vec![def],
            },
        ))
        .expect("interactive");
        // Empty set is a legal answer ("no attacks").
        assert_eq!(it.confirm(), Some(Decision::Attackers(vec![])));
        it.toggle(a);
        it.toggle(b);
        it.toggle(off); // ignored — not legal
        // Each attacker defaults to attacking the defending player's proxy.
        assert_eq!(
            it.confirm(),
            Some(Decision::Attackers(vec![(a, def), (b, def)]))
        );
        it.toggle(a); // untoggle
        assert_eq!(it.confirm(), Some(Decision::Attackers(vec![(b, def)])));
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn blockers_pairing_records_blocker_attacker_pairs() {
        let v = ids();
        let (b0, b1, atk0, atk1) = (v[0], v[1], v[2], v[3]);
        let mut it = Interaction::for_decision(&DecisionPointKind::DeclareBlockers(
            deckmaste_engine::DeclareBlockers {
                player: PlayerId(1),
                legal: vec![b0, b1],
            },
        ))
        .expect("interactive");
        // Empty = "no blocks" is a legal answer.
        assert_eq!(it.confirm(), Some(Decision::Blocks(vec![])));
        it.toggle(b0); // start pairing b0
        assert!(it.confirm().is_none()); // pairing in progress
        it.pair_with(atk0); // b0 blocks atk0
        assert!(it.is_chosen(b0));
        it.toggle(b1);
        it.pair_with(atk1);
        assert_eq!(
            it.confirm(),
            Some(Decision::Blocks(vec![(b0, atk0), (b1, atk1)]))
        );
    }

    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn blockers_unpair_undoes_pending_then_pairs() {
        let v = ids();
        let (b0, atk0) = (v[0], v[2]);
        let mut it = Interaction::for_decision(&DecisionPointKind::DeclareBlockers(
            deckmaste_engine::DeclareBlockers {
                player: PlayerId(1),
                legal: vec![b0],
            },
        ))
        .expect("interactive");
        it.toggle(b0);
        it.pair_with(atk0);
        it.toggle(b0); // a paired blocker is no longer a candidate, so re-toggle is a no-op
        assert!(it.candidates().is_empty());
        it.unpair_last(); // remove (b0, atk0)
        assert_eq!(it.confirm(), Some(Decision::Blocks(vec![])));
        assert_eq!(it.candidates(), vec![b0]); // available again
    }

    #[test]
    fn attacker_without_a_defending_target_is_not_interactive() {
        let pending = DecisionPointKind::DeclareAttackers(deckmaste_engine::DeclareAttackers {
            player: PlayerId(0),
            legal: vec![ObjectId::default()],
            legal_targets: vec![],
        });
        assert!(!is_interactive(&pending));
        assert_eq!(Interaction::for_decision(&pending), None);
    }

    #[cfg(feature = "slow-tests")]
    #[test]
    #[cfg_attr(
        not(wizards_corpus),
        ignore = "requires generated plugins/wizards corpus"
    )]
    fn interactive_path_produces_only_legal_decisions() {
        use deckmaste_engine::DecisionPointKind;
        use deckmaste_engine::sim::GreedyCreatures;
        use deckmaste_engine::sim::Strategy;

        use crate::driver::Driver;
        use crate::driver::Stop;

        let mut driver = Driver::new(
            game::build_game().expect("build demo game"),
            Box::new(GreedyCreatures),
        );
        let strategy = GreedyCreatures;
        let mut stop = driver.run_to_decision().expect("first stop");
        // Bound the loop so a logic bug can't hang the test.
        for _ in 0..200_000 {
            let pending = match &stop {
                Stop::GameOver(_) | Stop::Budget => return, // terminated cleanly
                Stop::Decision(p) => p.clone(),
            };
            let decision = match &pending {
                // Priority: use the demo's monocolor reach heuristic. The
                // engine now offers spell proposals before affordability is
                // proven, so blindly taking the first cast action can loop on
                // announce/decline when the available sources are insufficient.
                DecisionPointKind::Priority(_) => strategy.decide(&driver.state, &pending),
                // Targets: first candidate per spec, built through Interaction.
                DecisionPointKind::ChooseTargets(deckmaste_engine::ChooseTargets { .. }) => {
                    let mut it = Interaction::for_decision(&pending).expect("interactive");
                    loop {
                        let cand = it.candidates();
                        if let Some(&first) = cand.first() {
                            it.toggle(first);
                        }
                        match it.confirm() {
                            Some(d) => break d,
                            None => it.advance(),
                        }
                    }
                }
                // Attackers: swing with everything, built through Interaction.
                DecisionPointKind::DeclareAttackers(deckmaste_engine::DeclareAttackers {
                    legal,
                    ..
                }) => {
                    let mut it = Interaction::for_decision(&pending).expect("interactive");
                    for &id in legal {
                        it.toggle(id);
                    }
                    it.confirm().expect("attackers confirm")
                }
                // Blockers: declare no blocks, built through Interaction.
                DecisionPointKind::DeclareBlockers(deckmaste_engine::DeclareBlockers {
                    ..
                }) => {
                    let it = Interaction::for_decision(&pending).expect("interactive");
                    it.confirm().expect("blocks confirm")
                }
                // Discard: drop the first `count` cards, built through for_discard.
                DecisionPointKind::DiscardToHandSize(deckmaste_engine::DiscardToHandSize {
                    player,
                    count,
                })
                | DecisionPointKind::DiscardCards(deckmaste_engine::DiscardCards {
                    player,
                    count,
                }) => {
                    let hand: Vec<_> = driver.state.zones.hands[player.index()].clone();
                    let mut it = Interaction::for_discard(&hand, *count as usize);
                    for &id in hand.iter().take(*count as usize) {
                        it.toggle(id);
                    }
                    it.confirm().expect("discard confirm")
                }
                other => panic!("run_to_decision surfaced a non-interactive kind: {other:?}"),
            };
            stop = driver
                .submit(decision)
                .expect("every Interaction-built decision is legal");
        }
        panic!("game did not terminate within the decision budget");
    }
}
