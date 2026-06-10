use std::collections::VecDeque;

use crate::object::ObjectId;

/// Zone contents ([CR#400]). Libraries/hands/graveyards are per player;
/// the battlefield and exile ([CR#406.2]) are shared. The library's front is
/// its top. Stack and command arrive with the stages that need them.
#[derive(Debug, Clone)]
pub struct Zones {
    pub libraries: Vec<VecDeque<ObjectId>>,
    pub hands: Vec<Vec<ObjectId>>,
    pub graveyards: Vec<Vec<ObjectId>>,
    pub battlefield: Vec<ObjectId>,
    pub exile: Vec<ObjectId>,
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
        }
    }
}
