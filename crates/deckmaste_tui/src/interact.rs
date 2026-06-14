//! The pure, in-progress human choice servicing the current interactive
//! `PendingDecision`. No engine mutation, no ratatui — unit-tested headlessly.
//! Owns every selection invariant (target caps, blocker pairing); the driver
//! uses [`is_interactive`] to decide what to surface vs auto-resolve.
use deckmaste_engine::Action;
use deckmaste_engine::Decision;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PendingDecision;

/// The in-progress selection for the decision currently shown to the human.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interaction {
    /// A priority window. Acting is object-first (handled in `app` from the
    /// cursor); `sub` is the disambiguation popup when one object has >1
    /// action.
    Priority { sub: Option<AbilityPick> },
    /// Choose targets: one object per spec. `legal[i]` is spec i's candidate
    /// set; `chosen[i]` is the pick (capped at one — untoggle to change).
    Targets {
        legal: Vec<Vec<ObjectId>>,
        chosen: Vec<Option<ObjectId>>,
        active: usize,
    },
    /// Declare attackers: any subset of `legal`.
    Attackers {
        legal: Vec<ObjectId>,
        chosen: Vec<ObjectId>,
    },
    /// Declare blockers: `(blocker, attacker)` pairs. `pending` is a blocker
    /// awaiting the attacker it blocks. Attacker candidates come from the live
    /// combat state (derived in `app`/`ui`), not stored here.
    Blockers {
        legal: Vec<ObjectId>,
        pairs: Vec<(ObjectId, ObjectId)>,
        pending: Option<ObjectId>,
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
/// choose, so it auto-resolves. The single source of truth for the driver
/// partition.
#[must_use]
pub fn is_interactive(pending: &PendingDecision) -> bool {
    match pending {
        PendingDecision::Priority { .. } | PendingDecision::ChooseTargets { .. } => true,
        PendingDecision::DeclareAttackers { legal, .. }
        | PendingDecision::DeclareBlockers { legal, .. } => !legal.is_empty(),
        _ => false,
    }
}

impl Interaction {
    /// Build the initial interaction for `pending`, or `None` if it is
    /// auto-resolved (mirrors [`is_interactive`]).
    #[must_use]
    pub fn for_decision(pending: &PendingDecision) -> Option<Self> {
        if !is_interactive(pending) {
            return None;
        }
        Some(match pending {
            PendingDecision::Priority { .. } => Interaction::Priority { sub: None },
            PendingDecision::ChooseTargets { legal, .. } => Interaction::Targets {
                legal: legal.clone(),
                chosen: vec![None; legal.len()],
                active: 0,
            },
            PendingDecision::DeclareAttackers { legal, .. } => Interaction::Attackers {
                legal: legal.clone(),
                chosen: Vec::new(),
            },
            PendingDecision::DeclareBlockers { legal, .. } => Interaction::Blockers {
                legal: legal.clone(),
                pairs: Vec::new(),
                pending: None,
            },
            // is_interactive already excluded every other kind.
            _ => unreachable!("is_interactive gated the match"),
        })
    }

    /// True for the board-dimming pick modes (everything but `Priority`).
    #[must_use]
    pub fn is_pick_mode(&self) -> bool { !matches!(self, Interaction::Priority { .. }) }

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
            Interaction::Attackers { legal, .. } => legal.clone(),
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
    pub fn is_candidate(&self, id: ObjectId) -> bool { self.candidates().contains(&id) }

    /// Whether `id` is part of the committed selection (rendered with a ✓).
    #[must_use]
    pub fn is_chosen(&self, id: ObjectId) -> bool {
        match self {
            Interaction::Priority { .. } => false,
            Interaction::Targets { chosen, .. } => chosen.iter().flatten().any(|&c| c == id),
            Interaction::Attackers { chosen, .. } => chosen.contains(&id),
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
                match chosen[active] {
                    Some(cur) if cur == id => chosen[active] = None,
                    Some(_) => {} // a different pick exists — untoggle it first
                    None => chosen[active] = Some(id),
                }
            }
            Interaction::Attackers { legal, chosen } => {
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
        }
    }

    /// Targets only: move to the next spec still needing a pick (wrapping
    /// forward). No-op for other variants or when every spec is satisfied.
    pub fn advance(&mut self) {
        if let Interaction::Targets { chosen, active, .. } = self {
            let n = chosen.len();
            for step in 1..=n {
                let i = (*active + step) % n;
                if chosen[i].is_none() {
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
                    *c = None;
                }
                *active = 0;
            }
            Interaction::Attackers { chosen, .. } => chosen.clear(),
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
            Interaction::Targets { chosen, .. } => {
                let picks: Option<Vec<ObjectId>> = chosen.iter().copied().collect();
                picks.map(Decision::Targets)
            }
            Interaction::Attackers { chosen, .. } => Some(Decision::Attackers(chosen.clone())),
            Interaction::Blockers { pairs, pending, .. } => {
                if pending.is_some() {
                    None // finish the in-progress pairing first
                } else {
                    Some(Decision::Blocks(pairs.clone()))
                }
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
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::Action;
    use deckmaste_engine::ObjectId;
    use deckmaste_engine::PendingDecision;
    use deckmaste_engine::PlayerId;

    use super::*;
    use crate::game;

    /// A few distinct real `ObjectId`s (no public constructor exists).
    fn ids() -> Vec<ObjectId> {
        let state = game::build_game().expect("build");
        state.zones.libraries[0].iter().copied().collect()
    }

    #[test]
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
    fn partition_surfaces_priority_targets_and_nonempty_combat() {
        let v = ids();
        let prio = PendingDecision::Priority {
            player: PlayerId(0),
            legal: vec![Action::Pass],
        };
        assert!(is_interactive(&prio));
        let atk = PendingDecision::DeclareAttackers {
            player: PlayerId(0),
            legal: vec![v[0]],
        };
        assert!(is_interactive(&atk));
        // Empty combat = nothing to choose = auto-resolved.
        let empty = PendingDecision::DeclareAttackers {
            player: PlayerId(0),
            legal: vec![],
        };
        assert!(!is_interactive(&empty));
        assert!(Interaction::for_decision(&empty).is_none());
        // A clearly auto-resolved kind.
        let discard = PendingDecision::DiscardCards {
            player: PlayerId(0),
            count: 1,
        };
        assert!(!is_interactive(&discard));
    }

    #[test]
    fn for_decision_builds_one_target_slot_per_spec() {
        let v = ids();
        let pending = PendingDecision::ChooseTargets {
            player: PlayerId(0),
            spec: vec![],
            legal: vec![vec![v[0], v[1]], vec![v[2]]],
        };
        match Interaction::for_decision(&pending).expect("interactive") {
            Interaction::Targets {
                legal,
                chosen,
                active,
            } => {
                assert_eq!(legal.len(), 2);
                assert_eq!(chosen, vec![None, None]);
                assert_eq!(active, 0);
            }
            other => panic!("expected Targets, got {other:?}"),
        }
    }

    #[test]
    fn targets_caps_at_one_per_spec_and_untoggles_to_change() {
        let v = ids();
        let (a, b) = (v[0], v[1]);
        let mut it = Interaction::for_decision(&PendingDecision::ChooseTargets {
            player: PlayerId(0),
            spec: vec![],
            legal: vec![vec![a, b]],
        })
        .expect("interactive");
        assert!(it.confirm().is_none()); // nothing chosen yet
        it.toggle(a);
        assert!(it.is_chosen(a));
        it.toggle(b); // refused: spec already has a pick (cap = 1)
        assert!(it.is_chosen(a) && !it.is_chosen(b));
        it.toggle(a); // untoggle
        it.toggle(b); // now allowed
        assert!(it.is_chosen(b) && !it.is_chosen(a));
        assert_eq!(it.confirm(), Some(Decision::Targets(vec![b])));
    }

    #[test]
    fn targets_advance_walks_specs_and_confirms_in_order() {
        let v = ids();
        let (a, b, c) = (v[0], v[1], v[2]);
        let mut it = Interaction::for_decision(&PendingDecision::ChooseTargets {
            player: PlayerId(0),
            spec: vec![],
            legal: vec![vec![a, b], vec![c]],
        })
        .expect("interactive");
        it.toggle(a); // spec 0 := a
        assert!(it.confirm().is_none()); // spec 1 still empty
        it.advance(); // move to spec 1
        it.toggle(c); // spec 1 := c
        assert_eq!(it.confirm(), Some(Decision::Targets(vec![a, c])));
    }

    #[test]
    fn targets_ignores_non_candidates() {
        let v = ids();
        let (a, off) = (v[0], v[3]);
        let mut it = Interaction::for_decision(&PendingDecision::ChooseTargets {
            player: PlayerId(0),
            spec: vec![],
            legal: vec![vec![a]],
        })
        .expect("interactive");
        it.toggle(off); // not a candidate
        assert!(!it.is_chosen(off));
        assert!(it.confirm().is_none());
    }

    #[test]
    fn attackers_toggle_is_a_free_subset_and_confirms_any_set() {
        let v = ids();
        let (a, b, off) = (v[0], v[1], v[3]);
        let mut it = Interaction::for_decision(&PendingDecision::DeclareAttackers {
            player: PlayerId(0),
            legal: vec![a, b],
        })
        .expect("interactive");
        // Empty set is a legal answer ("no attacks").
        assert_eq!(it.confirm(), Some(Decision::Attackers(vec![])));
        it.toggle(a);
        it.toggle(b);
        it.toggle(off); // ignored — not legal
        assert_eq!(it.confirm(), Some(Decision::Attackers(vec![a, b])));
        it.toggle(a); // untoggle
        assert_eq!(it.confirm(), Some(Decision::Attackers(vec![b])));
    }

    #[test]
    fn blockers_pairing_records_blocker_attacker_pairs() {
        let v = ids();
        let (b0, b1, atk0, atk1) = (v[0], v[1], v[2], v[3]);
        let mut it = Interaction::for_decision(&PendingDecision::DeclareBlockers {
            player: PlayerId(1),
            legal: vec![b0, b1],
        })
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
    fn blockers_unpair_undoes_pending_then_pairs() {
        let v = ids();
        let (b0, atk0) = (v[0], v[2]);
        let mut it = Interaction::for_decision(&PendingDecision::DeclareBlockers {
            player: PlayerId(1),
            legal: vec![b0],
        })
        .expect("interactive");
        it.toggle(b0);
        it.pair_with(atk0);
        it.toggle(b0); // a paired blocker is no longer a candidate, so re-toggle is a no-op
        assert!(it.candidates().is_empty());
        it.unpair_last(); // remove (b0, atk0)
        assert_eq!(it.confirm(), Some(Decision::Blocks(vec![])));
        assert_eq!(it.candidates(), vec![b0]); // available again
    }
}
