use serde::{Deserialize, Serialize};

/// An object's status ([CR#110.5]): the binary conditions a permanent can be
/// in. Filtered via `Filter`'s `Status` atom; matched as a transition via
/// `Event::StateBecomes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Status {
    /// [CR#110.5].
    Tapped,
    /// [CR#110.5].
    Untapped,
    /// [CR#110.5].
    FaceDown,
    /// [CR#110.5].
    FaceUp,
    /// [CR#110.5].
    PhasedOut,
}
