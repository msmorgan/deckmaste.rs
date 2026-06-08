use deckmaste_core::{ColorOrColorless, Int, Uint};

use crate::object::ObjectId;

/// A player identity: the index into `GameState::players`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlayerId(pub Uint);

impl PlayerId {
    #[must_use]
    pub fn index(self) -> usize { self.0 as usize }
}

/// Unspent mana ([CR#106.4]), by kind. Fixed six slots — deterministic, no
/// map ordering questions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManaPool([Uint; 6]);

fn slot(mana: ColorOrColorless) -> usize {
    use deckmaste_core::Color::{Black, Blue, Green, Red, White};
    match mana {
        ColorOrColorless::Colorless => 0,
        ColorOrColorless::Color(White) => 1,
        ColorOrColorless::Color(Blue) => 2,
        ColorOrColorless::Color(Black) => 3,
        ColorOrColorless::Color(Red) => 4,
        ColorOrColorless::Color(Green) => 5,
    }
}

impl ManaPool {
    pub fn add(&mut self, mana: ColorOrColorless, amount: Uint) { self.0[slot(mana)] += amount; }

    #[must_use]
    pub fn amount(&self, mana: ColorOrColorless) -> Uint { self.0[slot(mana)] }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.iter().all(|&n| n == 0) }

    /// [CR#500.4] / [CR#106.4]: the pool empties at each step and phase boundary.
    pub fn clear(&mut self) { self.0 = [0; 6]; }

    /// Removes `amount` of `mana`. Panics if the pool holds less — callers
    /// validate first.
    ///
    /// # Panics
    ///
    /// Panics if `amount` exceeds the pool's holding of `mana`.
    pub fn spend(&mut self, mana: ColorOrColorless, amount: Uint) {
        let slot = &mut self.0[slot(mana)];
        *slot = slot.checked_sub(amount).expect("pool covers the spend");
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
    pub lands_played_this_turn: Uint,
    /// [CR#704.5c] flag: tried to draw from an empty library.
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
            lands_played_this_turn: 0,
            drew_from_empty: false,
            lost: false,
            mana_pool: ManaPool::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Color;

    use super::*;

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
}
