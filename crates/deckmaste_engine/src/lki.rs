//! Last-known information ([CR#603.10a], [CR#608.2]). A value snapshot of an
//! object captured at a zone change, so triggers and effects can read it after
//! the object itself is gone. The engine never retains dead objects.

use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::state::GameState;

/// An object's last-known state, captured the instant it changes zones.
/// Characteristics (types/P-T/name) derive from `source` via the card
/// definition — correct in Stage 3 (no continuous effects yet to have modified
/// them).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LkiSnapshot {
    /// The now-stale id — a label/reference token, not a live object.
    pub object: ObjectId,
    /// `CardId` spine: derives printed characteristics and the owner.
    pub source: ObjectSource,
    pub controller: PlayerId,
    pub tapped: bool,
    pub damage: Uint,
    /// The zone it left.
    pub left: Zone,
}

impl LkiSnapshot {
    /// Snapshot a currently-live object.
    ///
    /// # Panics
    ///
    /// Panics on a player proxy (no zone) or a stale `ObjectId`.
    #[must_use]
    pub fn capture(state: &GameState, id: ObjectId) -> Self {
        let o = state.objects.obj(id);
        LkiSnapshot {
            object: id,
            source: o.source,
            controller: o.controller,
            tapped: o.tapped,
            damage: o.damage,
            left: o.zone.expect("a zoned object has a zone to leave"),
        }
    }
}
