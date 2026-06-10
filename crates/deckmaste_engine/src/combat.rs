//! Combat-state registry ([CR#506]): which creatures are attacking, which
//! blocks which, and which attackers are blocked. Combat designations live
//! here — not as fields or tags on objects. Cleared at the end of the combat
//! phase ([CR#511.3]).
//!
//! Modern rules have no damage-assignment-order step ([CR#509.2] is just "the
//! active player gets priority"); a blocked creature's damage is divided among
//! its blockers as its controller chooses at assignment time ([CR#510.1c]), so
//! `blockers` is an unordered grouping, not a player-set order.

use std::collections::{BTreeMap, BTreeSet};

use deckmaste_core::{Ability, KeywordAbility};

use crate::layer::LayeredView;
use crate::object::ObjectId;
use crate::state::GameState;

/// Whether `object` has the intrinsic combat keyword `kw` ([CR#702]).
/// Reads the layer-6–derived ability list ([CR#613.1f]) so that granted and
/// removed keywords are honored. Takes the derived view rather than building
/// one — callers checking several creatures share a single `state.layers()`.
#[must_use]
pub fn has_keyword(view: &LayeredView, object: ObjectId, kw: &KeywordAbility) -> bool {
    view.get(object)
        .abilities
        .iter()
        .any(|a| matches!(a, Ability::Keyword(k) if k == kw))
}

/// [CR#702.7c,702.4]: whether `object` deals damage in the FIRST combat-damage
/// step — true iff it has first strike OR double strike.
#[must_use]
pub fn deals_first_strike(view: &LayeredView, object: ObjectId) -> bool {
    has_keyword(view, object, &KeywordAbility::FirstStrike)
        || has_keyword(view, object, &KeywordAbility::DoubleStrike)
}

/// [CR#702.4]: whether `object` deals damage in the REGULAR combat-damage step.
/// A double-striker deals in both steps; a first-striker (without double
/// strike) deals ONLY in the first step, so it does not deal here; every other
/// creature deals here. This filter naturally includes everyone when no first
/// strike exists.
#[must_use]
pub fn deals_regular_strike(view: &LayeredView, object: ObjectId) -> bool {
    !has_keyword(view, object, &KeywordAbility::FirstStrike)
        || has_keyword(view, object, &KeywordAbility::DoubleStrike)
}

/// [CR#510.4]: whether ANY combat creature (an attacker or a live blocker) has
/// first strike or double strike — the condition for opening the
/// `FirstCombatDamage` step. The set of combat creatures is every attacker plus
/// every live blocker.
#[must_use]
pub fn any_first_or_double_striker(state: &GameState) -> bool {
    let view = state.layers();
    let combat = &state.combat;
    combat.attackers().iter().any(|&a| {
        deals_first_strike(&view, a)
            || combat
                .blockers_of(a)
                .iter()
                .any(|&b| deals_first_strike(&view, b))
    })
}

/// Tracks all combat designations for the current combat phase.
///
/// - `attackers`: declared attackers, in declaration order.
/// - `blocks`: blocker → the attacker it blocks.
/// - `blockers`: attacker → its *live* blockers (pruned as they leave combat).
/// - `blocked`: the **sticky** "is a blocked creature" status ([CR#509.1h]) — a
///   creature stays blocked even after every blocker leaves combat.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CombatState {
    attackers: Vec<ObjectId>,
    blocks: BTreeMap<ObjectId, ObjectId>, // blocker -> the attacker it blocks
    blockers: BTreeMap<ObjectId, Vec<ObjectId>>, // attacker -> its live blockers
    blocked: BTreeSet<ObjectId>,          // sticky blocked status ([CR#509.1h])
}

impl CombatState {
    /// Returns `true` if `o` has been declared as an attacker.
    #[must_use]
    pub fn is_attacking(&self, o: ObjectId) -> bool { self.attackers.contains(&o) }

    /// Returns `true` if `attacker` is a blocked creature. This is **sticky**
    /// ([CR#509.1h]): it stays true even after all of `attacker`'s blockers
    /// have left combat.
    #[must_use]
    pub fn is_blocked(&self, attacker: ObjectId) -> bool { self.blocked.contains(&attacker) }

    /// All declared attackers, in declaration order.
    #[must_use]
    pub fn attackers(&self) -> &[ObjectId] { &self.attackers }

    /// The *live* blockers of `attacker` ([CR#510.1c] divides damage among
    /// them). Empty slice if `attacker` is unblocked or its blockers have all
    /// left combat.
    #[must_use]
    pub fn blockers_of(&self, attacker: ObjectId) -> &[ObjectId] {
        self.blockers.get(&attacker).map_or(&[], Vec::as_slice)
    }

    /// The attacker that `blocker` is assigned to block, if any.
    #[must_use]
    pub fn attacker_of(&self, blocker: ObjectId) -> Option<ObjectId> {
        self.blocks.get(&blocker).copied()
    }

    /// Declares `o` as an attacker. Does nothing if already an attacker.
    pub(crate) fn declare_attacker(&mut self, o: ObjectId) {
        if !self.attackers.contains(&o) {
            self.attackers.push(o);
        }
    }

    /// Records `blocker` as blocking `attacker`: inserts into `blocks`, appends
    /// to `attacker`'s live blockers, and marks `attacker` blocked
    /// ([CR#509.1h]).
    pub(crate) fn declare_block(&mut self, blocker: ObjectId, attacker: ObjectId) {
        self.blocks.insert(blocker, attacker);
        self.blockers.entry(attacker).or_default().push(blocker);
        self.blocked.insert(attacker);
    }

    /// Removes `o` from combat. As an attacker: dropped from `attackers`,
    /// `blocked`, and its `blockers` entry. As a blocker: dropped from `blocks`
    /// and from every attacker's live-blocker vec — but the attacker stays
    /// `blocked` (sticky, [CR#509.1h]).
    pub(crate) fn remove_object(&mut self, o: ObjectId) {
        self.attackers.retain(|&a| a != o);
        self.blocked.remove(&o);
        self.blockers.remove(&o);
        self.blocks.remove(&o);
        for blockers in self.blockers.values_mut() {
            blockers.retain(|&b| b != o);
        }
    }

    /// Clears all combat designations ([CR#511.3]).
    pub(crate) fn clear(&mut self) {
        self.attackers.clear();
        self.blocks.clear();
        self.blockers.clear();
        self.blocked.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u32) -> ObjectId { ObjectId(n) }

    #[test]
    fn declaring_attacker_makes_is_attacking_true() {
        let mut cs = CombatState::default();
        let a = id(1);
        assert!(!cs.is_attacking(a));
        cs.declare_attacker(a);
        assert!(cs.is_attacking(a));
        assert_eq!(cs.attackers(), &[a]);
    }

    #[test]
    fn declare_attacker_is_idempotent() {
        let mut cs = CombatState::default();
        let a = id(1);
        cs.declare_attacker(a);
        cs.declare_attacker(a);
        assert_eq!(cs.attackers(), &[a]);
    }

    #[test]
    fn declare_block_records_attacker_of_blockers_of_and_blocked() {
        let mut cs = CombatState::default();
        let (a, b) = (id(1), id(2));
        cs.declare_attacker(a);
        cs.declare_block(b, a);
        assert_eq!(cs.attacker_of(b), Some(a));
        assert_eq!(cs.blockers_of(a), &[b]);
        assert!(cs.is_blocked(a));
    }

    #[test]
    fn multiple_blockers_on_one_attacker() {
        let mut cs = CombatState::default();
        let (a, b1, b2) = (id(1), id(2), id(3));
        cs.declare_attacker(a);
        cs.declare_block(b1, a);
        cs.declare_block(b2, a);
        assert_eq!(cs.blockers_of(a), &[b1, b2]);
        assert!(cs.is_blocked(a));
    }

    #[test]
    fn blockers_of_empty_when_no_blockers() {
        let cs = CombatState::default();
        assert_eq!(cs.blockers_of(id(99)), &[] as &[ObjectId]);
    }

    #[test]
    fn attacker_of_none_when_not_blocking() {
        let cs = CombatState::default();
        assert_eq!(cs.attacker_of(id(99)), None);
    }

    /// [CR#509.1h]: a creature remains blocked even after all the creatures
    /// blocking it leave combat.
    #[test]
    fn blocked_status_is_sticky_after_blocker_removed() {
        let mut cs = CombatState::default();
        let (a, b) = (id(1), id(2));
        cs.declare_attacker(a);
        cs.declare_block(b, a);
        cs.remove_object(b);
        // The lone blocker is gone, but the attacker is still a blocked creature.
        assert!(cs.is_blocked(a));
        assert_eq!(cs.blockers_of(a), &[] as &[ObjectId]);
        assert_eq!(cs.attacker_of(b), None);
    }

    #[test]
    fn remove_object_prunes_attacker_including_blocked() {
        let mut cs = CombatState::default();
        let (a, b) = (id(1), id(2));
        cs.declare_attacker(a);
        cs.declare_block(b, a);
        cs.remove_object(a);
        assert!(!cs.is_attacking(a));
        assert!(!cs.is_blocked(a));
        assert_eq!(cs.blockers_of(a), &[] as &[ObjectId]);
    }

    #[test]
    fn remove_object_prunes_blocker_from_blocks_and_live_blockers() {
        let mut cs = CombatState::default();
        let (a, b1, b2) = (id(1), id(2), id(3));
        cs.declare_attacker(a);
        cs.declare_block(b1, a);
        cs.declare_block(b2, a);
        cs.remove_object(b1);
        assert_eq!(cs.attacker_of(b1), None);
        assert_eq!(cs.blockers_of(a), &[b2]);
        assert!(cs.is_blocked(a));
    }

    /// An object that is both a blocked attacker (an `blockers` key) and a
    /// blocker of another attacker (a value in someone else's vec) is pruned
    /// from both roles in one call.
    #[test]
    fn remove_object_prunes_both_roles_at_once() {
        let mut cs = CombatState::default();
        let (a1, a2, x) = (id(1), id(2), id(3));
        // x blocks a1, and x is itself a blocked attacker (blocked by a2... in
        // v1 a creature isn't both, but the registry must still prune cleanly).
        cs.declare_attacker(a1);
        cs.declare_attacker(x);
        cs.declare_block(x, a1); // x blocks a1  -> x is a value in blockers[a1]
        cs.declare_block(a2, x); // a2 blocks x  -> x is a key in blockers, and blocked
        assert!(cs.is_blocked(x));
        assert_eq!(cs.blockers_of(a1), &[x]);
        cs.remove_object(x);
        assert!(!cs.is_blocked(x)); // gone as a blocked attacker
        assert_eq!(cs.blockers_of(x), &[] as &[ObjectId]); // gone as a key
        assert_eq!(cs.blockers_of(a1), &[] as &[ObjectId]); // gone as a value
        assert_eq!(cs.attacker_of(x), None); // gone from blocks
    }

    #[test]
    fn clear_empties_everything() {
        let mut cs = CombatState::default();
        let (a, b) = (id(1), id(2));
        cs.declare_attacker(a);
        cs.declare_block(b, a);
        cs.clear();
        assert!(!cs.is_attacking(a));
        assert!(!cs.is_blocked(a));
        assert_eq!(cs.blockers_of(a), &[] as &[ObjectId]);
        assert_eq!(cs.attacker_of(b), None);
        assert_eq!(cs.attackers(), &[] as &[ObjectId]);
    }
}
