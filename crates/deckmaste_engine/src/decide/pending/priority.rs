use crate::decide::Action;
use crate::player::PlayerId;

/// [CR#117]: the holder may act or pass. `legal` is advisory UI data —
/// submission re-validates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Priority {
    pub player: PlayerId,
    pub legal: Vec<Action>,
}
