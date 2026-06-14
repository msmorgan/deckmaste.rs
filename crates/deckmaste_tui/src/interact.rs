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
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::Action;
    use deckmaste_engine::ObjectId;
    use deckmaste_engine::PendingDecision;
    use deckmaste_engine::PlayerId;

    use super::*;
    use crate::game;

    /// A few distinct real ObjectIds (no public constructor exists).
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
}
