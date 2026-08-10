use deckmaste_core::ColorOrColorless;
use deckmaste_core::Int;
use deckmaste_core::ManaRider;
use deckmaste_core::Uint;

use crate::object::ObjectId;

/// A player identity: the index into `GameState::players`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct PlayerId(pub Uint);

impl PlayerId {
    #[must_use]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

/// Stable identity of one unspent mana unit. IDs are never reused within a
/// game image, so a payment witness cannot silently retarget a neighboring
/// unit after the pool changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FloatingManaId(pub u64);

/// Identity of one activated or triggered mana action. The action is minted
/// when the full payment protocol starts a mana ability; pre-protocol producers
/// leave this absent while still recording their source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ManaActionId(pub u64);

/// Where a floating mana unit came from. Riders continue to describe how the
/// unit may be spent; provenance identifies the producing object/action for
/// rules reads and payment replay.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ManaProvenance {
    pub source: Option<ObjectId>,
    pub action: Option<ManaActionId>,
}

/// One point of unspent mana ([CR#106.4]) with the riders the producing
/// effect attached to it ([CR#106.6] — riders live on the UNIT).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaUnit {
    pub id: FloatingManaId,
    pub kind: ColorOrColorless,
    pub riders: Vec<ManaRider>,
    pub provenance: ManaProvenance,
}

/// Unspent mana ([CR#106.4]) as a flat list of units, in production order.
/// Small (rarely > ~10), so linear scans are fine.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManaPool {
    units: Vec<ManaUnit>,
    next_id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ManaPoolError {
    #[error("floating mana id {0:?} was submitted more than once")]
    Duplicate(FloatingManaId),
    #[error("floating mana id {0:?} is not in the pool")]
    Missing(FloatingManaId),
}

impl ManaPool {
    /// A pool of exactly `units`, in the given order. Used to build the
    /// spendable sub-pool an affordability check runs over ([CR#106.6]).
    ///
    /// # Panics
    ///
    /// Panics if the greatest supplied mana id cannot be incremented.
    #[must_use]
    pub fn from_units(units: Vec<ManaUnit>) -> Self {
        let next_id = units.iter().map(|unit| unit.id.0).max().map_or(0, |id| {
            id.checked_add(1).expect("floating mana id overflow")
        });
        debug_assert_eq!(
            units
                .iter()
                .map(|unit| unit.id)
                .collect::<std::collections::HashSet<_>>()
                .len(),
            units.len(),
            "a mana pool contains unique stable ids"
        );
        Self { units, next_id }
    }

    /// Add `amount` plain (riderless) units of `mana`.
    pub fn add(
        &mut self,
        mana: ColorOrColorless,
        amount: Uint,
        provenance: ManaProvenance,
    ) -> Vec<FloatingManaId> {
        self.add_riders(mana, amount, &[], provenance)
    }

    /// Add `amount` units of `mana`, each carrying a clone of `riders`
    /// ([CR#106.6a]: under a doubler every unit gets its own riders).
    ///
    /// # Panics
    ///
    /// Panics if allocating a new floating-mana id overflows.
    pub fn add_riders(
        &mut self,
        mana: ColorOrColorless,
        amount: Uint,
        riders: &[ManaRider],
        provenance: ManaProvenance,
    ) -> Vec<FloatingManaId> {
        let mut ids = Vec::with_capacity(amount as usize);
        for _ in 0..amount {
            let id = FloatingManaId(self.next_id);
            self.next_id = self
                .next_id
                .checked_add(1)
                .expect("floating mana id overflow");
            self.units.push(ManaUnit {
                id,
                kind: mana,
                riders: riders.to_vec(),
                provenance,
            });
            ids.push(id);
        }
        ids
    }

    /// Count of units of `mana` regardless of riders.
    ///
    /// # Panics
    ///
    /// Panics if the count overflows `Uint` (unreachable in practice — a pool
    /// with more than `Uint::MAX` units would be absurd).
    #[must_use]
    pub fn amount(&self, mana: ColorOrColorless) -> Uint {
        Uint::try_from(self.units.iter().filter(|u| u.kind == mana).count())
            .expect("pool fits Uint")
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    #[must_use]
    pub fn units(&self) -> &[ManaUnit] {
        &self.units
    }

    /// [CR#500.5,106.4]: drop every unit (blanket empty; persistence-aware
    /// emptying arrives in a later task as `empty_after`).
    pub fn clear(&mut self) {
        self.units.clear();
    }

    /// [CR#500.5,106.4]: empty the pool as the `ending` step/phase ends, but
    /// RETAIN any unit whose `Persistent` marker has not yet expired
    /// ([CR#702.189a]). A unit with several `Persistent` riders survives until
    /// the latest marker; a unit with none always empties.
    pub fn empty_after(&mut self, ending: deckmaste_core::PhaseStep) {
        self.units.retain(|u| {
            u.riders.iter().any(|r| {
                matches!(r,
                    deckmaste_core::ManaRider::Persistent(m) if !marker_expired_at(*m, ending))
            })
        });
    }

    /// Remove the units at `indices` (a validated payment selection). Indices
    /// must be distinct and in range — callers validate first.
    ///
    /// # Panics
    ///
    /// Panics if `indices` was not validated against this pool.
    pub fn remove_units(&mut self, indices: &[usize]) {
        let ids: Vec<FloatingManaId> = indices
            .iter()
            .filter_map(|&index| self.units.get(index).map(|unit| unit.id))
            .collect();
        self.remove_ids(&ids)
            .expect("payment indices were validated against the pool snapshot");
    }

    #[must_use]
    pub fn get(&self, id: FloatingManaId) -> Option<&ManaUnit> {
        self.units.iter().find(|unit| unit.id == id)
    }

    /// Remove exactly the identified units, validating the whole submission
    /// before mutating the pool.
    ///
    /// # Errors
    ///
    /// Returns an error if an id is duplicated or absent from the pool.
    pub fn remove_ids(&mut self, ids: &[FloatingManaId]) -> Result<(), ManaPoolError> {
        let mut unique = std::collections::HashSet::with_capacity(ids.len());
        for &id in ids {
            if !unique.insert(id) {
                return Err(ManaPoolError::Duplicate(id));
            }
            if self.get(id).is_none() {
                return Err(ManaPoolError::Missing(id));
            }
        }
        self.units.retain(|unit| !unique.contains(&unit.id));
        Ok(())
    }
}

/// [CR#514.2,511.2]: has `marker` elapsed by the end of `ending`?
fn marker_expired_at(
    marker: deckmaste_core::TurnMarker,
    ending: deckmaste_core::PhaseStep,
) -> bool {
    use deckmaste_core::CombatStep;
    use deckmaste_core::EndingStep;
    use deckmaste_core::PhaseStep;
    use deckmaste_core::TurnMarker;
    match marker {
        TurnMarker::EndOfTurn => ending == PhaseStep::Ending(EndingStep::Cleanup),
        TurnMarker::EndOfCombat => ending == PhaseStep::Combat(CombatStep::EndOfCombat),
        // Seam: "until your next turn" needs turn-owner tracking; retained for now.
        TurnMarker::YourNextTurn => false,
    }
}

/// Per-player state. [CR#119]: life is signed.
#[derive(Debug, Clone)]
pub struct PlayerState {
    pub id: PlayerId,
    /// This player's proxy object (CR: players modeled as objects).
    pub object: ObjectId,
    /// [CR#119.3,119.9,119.10]: every route into this total is
    /// read-current → compute-delta → EMIT a `LifeGained`/`LifeLost`
    /// `GameEvent` ([CR#119.5] "set" included — `resolve/player_action.rs`'s
    /// `LifeOp::Set` arm resolves the necessary gain/loss, never writes
    /// here directly). `EventApply for LifeGained`/`LifeLost`
    /// (`step/player.rs`) is the ONLY production writer, reached after the
    /// event has passed the replacement window ([CR#614]) — a direct write
    /// here would bypass it. No production path writes `life` directly
    /// today; only test fixtures set it to seed a scenario's starting
    /// total.
    pub life: Int,
    pub max_hand_size: Uint,
    /// [CR#704.5b] flag: tried to draw from an empty library.
    pub drew_from_empty: bool,
    pub lost: bool,
    pub mana_pool: ManaPool,
}

impl PlayerState {
    #[must_use]
    pub fn new(id: PlayerId, object: ObjectId, life: Int) -> Self {
        Self {
            id,
            object,
            life,
            max_hand_size: 7,
            drew_from_empty: false,
            lost: false,
            mana_pool: ManaPool::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::BeginningStep;
    use deckmaste_core::Color;
    use deckmaste_core::EndingStep;
    use deckmaste_core::ManaRider;
    use deckmaste_core::PhaseStep;
    use deckmaste_core::Predicate;
    use deckmaste_core::TurnMarker;

    use super::*;

    fn some_rider() -> ManaRider {
        ManaRider::SpendOnly(Predicate::Any)
    }

    #[test]
    fn mana_pool_adds_reads_and_clears() {
        let mut pool = ManaPool::default();
        assert!(pool.is_empty());
        pool.add(Color::White.into(), 2, ManaProvenance::default());
        pool.add(ColorOrColorless::Colorless, 1, ManaProvenance::default());
        assert_eq!(pool.amount(Color::White.into()), 2);
        assert_eq!(pool.amount(ColorOrColorless::Colorless), 1);
        assert_eq!(pool.amount(Color::Green.into()), 0);
        assert!(!pool.is_empty());
        pool.clear();
        assert!(pool.is_empty());
    }

    #[test]
    fn persistent_mana_survives_until_its_marker() {
        let mut pool = ManaPool::default();
        pool.add(Color::Red.into(), 1, ManaProvenance::default()); // plain
        pool.add_riders(
            Color::Green.into(),
            1,
            &[ManaRider::Persistent(TurnMarker::EndOfTurn)],
            ManaProvenance::default(),
        );
        pool.empty_after(PhaseStep::Beginning(BeginningStep::Upkeep)); // a non-final step
        assert_eq!(pool.amount(Color::Red.into()), 0); // plain mana emptied
        assert_eq!(pool.amount(Color::Green.into()), 1); // persistent survives the boundary
        pool.empty_after(PhaseStep::Ending(EndingStep::Cleanup)); // turn's last step
        assert!(pool.is_empty()); // EndOfTurn expires at cleanup
    }

    #[test]
    fn pool_units_carry_riders_and_amount_counts_them() {
        let mut pool = ManaPool::default();
        pool.add(Color::Red.into(), 2, ManaProvenance::default()); // two plain reds
        let rider = some_rider(); // a single ManaRider
        pool.add_riders(Color::Red.into(), 1, &[rider], ManaProvenance::default()); // one restricted red
        assert_eq!(pool.amount(Color::Red.into()), 3); // amount counts all reds
        assert_eq!(
            pool.units().iter().filter(|u| !u.riders.is_empty()).count(),
            1
        );
    }

    #[test]
    fn pool_ids_survive_neighbor_removal() {
        let mut pool = ManaPool::default();
        let ids = pool.add(Color::Green.into(), 3, ManaProvenance::default());

        pool.remove_ids(&[ids[1]]).unwrap();

        assert_eq!(pool.get(ids[0]).unwrap().kind, Color::Green.into());
        assert_eq!(pool.get(ids[2]).unwrap().kind, Color::Green.into());
        assert!(pool.get(ids[1]).is_none());
    }
}
