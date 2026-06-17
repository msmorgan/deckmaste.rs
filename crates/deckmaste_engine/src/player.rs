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

/// One point of unspent mana ([CR#106.4]) with the riders the producing
/// effect attached to it ([CR#106.6] — riders live on the UNIT).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManaUnit {
    pub kind: ColorOrColorless,
    pub riders: Vec<ManaRider>,
}

/// Unspent mana ([CR#106.4]) as a flat list of units, in production order.
/// Small (rarely > ~10), so linear scans are fine.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManaPool(Vec<ManaUnit>);

impl ManaPool {
    /// A pool of exactly `units`, in the given order. Used to build the
    /// spendable sub-pool an affordability check runs over ([CR#106.6]).
    #[must_use]
    pub fn from_units(units: Vec<ManaUnit>) -> Self {
        Self(units)
    }

    /// Add `amount` plain (riderless) units of `mana`.
    pub fn add(&mut self, mana: ColorOrColorless, amount: Uint) {
        self.add_riders(mana, amount, &[]);
    }

    /// Add `amount` units of `mana`, each carrying a clone of `riders`
    /// ([CR#106.6a]: under a doubler every unit gets its own riders).
    pub fn add_riders(&mut self, mana: ColorOrColorless, amount: Uint, riders: &[ManaRider]) {
        for _ in 0..amount {
            self.0.push(ManaUnit {
                kind: mana,
                riders: riders.to_vec(),
            });
        }
    }

    /// Count of units of `mana` regardless of riders.
    ///
    /// # Panics
    ///
    /// Panics if the count overflows `Uint` (unreachable in practice — a pool
    /// with more than `Uint::MAX` units would be absurd).
    #[must_use]
    pub fn amount(&self, mana: ColorOrColorless) -> Uint {
        Uint::try_from(self.0.iter().filter(|u| u.kind == mana).count()).expect("pool fits Uint")
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn units(&self) -> &[ManaUnit] {
        &self.0
    }

    /// [CR#500.5,106.4]: drop every unit (blanket empty; persistence-aware
    /// emptying arrives in a later task as `empty_after`).
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// [CR#500.5,106.4]: empty the pool as the `ending` step/phase ends, but
    /// RETAIN any unit whose `Persistent` marker has not yet expired
    /// ([CR#702.189a]). A unit with several `Persistent` riders survives until
    /// the latest marker; a unit with none always empties.
    pub fn empty_after(&mut self, ending: deckmaste_core::Phase) {
        self.0.retain(|u| {
            u.riders.iter().any(|r| {
                matches!(r,
                    deckmaste_core::ManaRider::Persistent(m) if !marker_expired_at(*m, ending))
            })
        });
    }

    /// Remove the units at `indices` (a validated payment selection). Indices
    /// must be distinct and in range — callers validate first.
    pub fn remove_units(&mut self, indices: &[usize]) {
        let drop: std::collections::HashSet<usize> = indices.iter().copied().collect();
        let mut i = 0;
        self.0.retain(|_| {
            let keep = !drop.contains(&i);
            i += 1;
            keep
        });
    }
}

/// [CR#514.2,511.2]: has `marker` elapsed by the end of `ending`?
fn marker_expired_at(marker: deckmaste_core::TurnMarker, ending: deckmaste_core::Phase) -> bool {
    use deckmaste_core::CombatStep;
    use deckmaste_core::EndingStep;
    use deckmaste_core::Phase;
    use deckmaste_core::TurnMarker;
    match marker {
        TurnMarker::EndOfTurn => ending == Phase::Ending(EndingStep::Cleanup),
        TurnMarker::EndOfCombat => ending == Phase::Combat(CombatStep::EndOfCombat),
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
    use deckmaste_core::Filter;
    use deckmaste_core::ManaRider;
    use deckmaste_core::Phase;
    use deckmaste_core::TurnMarker;

    use super::*;

    fn some_rider() -> ManaRider {
        ManaRider::SpendOnly(Filter::Any)
    }

    #[test]
    fn mana_pool_adds_reads_and_clears() {
        let mut pool = ManaPool::default();
        assert!(pool.is_empty());
        pool.add(Color::White.into(), 2);
        pool.add(ColorOrColorless::Colorless, 1);
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
        pool.add(Color::Red.into(), 1); // plain
        pool.add_riders(
            Color::Green.into(),
            1,
            &[ManaRider::Persistent(TurnMarker::EndOfTurn)],
        );
        pool.empty_after(Phase::Beginning(BeginningStep::Upkeep)); // a non-final step
        assert_eq!(pool.amount(Color::Red.into()), 0); // plain mana emptied
        assert_eq!(pool.amount(Color::Green.into()), 1); // persistent survives the boundary
        pool.empty_after(Phase::Ending(EndingStep::Cleanup)); // turn's last step
        assert!(pool.is_empty()); // EndOfTurn expires at cleanup
    }

    #[test]
    fn pool_units_carry_riders_and_amount_counts_them() {
        let mut pool = ManaPool::default();
        pool.add(Color::Red.into(), 2); // two plain reds
        let rider = some_rider(); // a single ManaRider
        pool.add_riders(Color::Red.into(), 1, &[rider]); // one restricted red
        assert_eq!(pool.amount(Color::Red.into()), 3); // amount counts all reds
        assert_eq!(
            pool.units().iter().filter(|u| !u.riders.is_empty()).count(),
            1
        );
    }
}
