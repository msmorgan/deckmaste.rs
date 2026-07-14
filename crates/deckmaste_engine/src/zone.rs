use std::collections::VecDeque;

use crate::object::ObjectId;

/// Zone contents ([CR#400]). Libraries/hands/graveyards are per player;
/// the battlefield, exile ([CR#406.2]), and command ([CR#408.1]) are shared.
/// The library's front is its top. The stack arrives with the stages that
/// need it (it lives on `GameState.stack`).
#[derive(Debug, Clone)]
pub struct Zones {
    pub libraries: Vec<VecDeque<ObjectId>>,
    pub hands: Vec<Vec<ObjectId>>,
    pub graveyards: Vec<Vec<ObjectId>>,
    pub battlefield: Vec<ObjectId>,
    pub exile: Vec<ObjectId>,
    /// The command zone ([CR#408.1]): a shared zone holding emblems (whose
    /// abilities function from here — [CR#114.4]) and, later, other
    /// command-zone objects.
    pub command: Vec<ObjectId>,
}

impl Zones {
    #[must_use]
    pub fn new(players: usize) -> Self {
        Self {
            libraries: vec![VecDeque::new(); players],
            hands: vec![Vec::new(); players],
            graveyards: vec![Vec::new(); players],
            battlefield: Vec::new(),
            exile: Vec::new(),
            command: Vec::new(),
        }
    }
}
