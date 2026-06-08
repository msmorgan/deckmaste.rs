//! The stack (CR 405) and the single in-flight announce slot (CR 601.2 /
//! 602.2). Stage 2 ships only the `Spell` arm of `StackObject`; activated
//! abilities add a variant in stage 3 without reshaping the callers.

use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::player::PlayerId;

/// What sits on (or is going onto) the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackObject {
    /// A card moved to the stack and cast (CR 601.2a).
    Spell(ObjectId),
    // Activated { source: ObjectId, ability: usize },  // stage 3
}

impl StackObject {
    /// The object this entry is "on" — the spell, or (later) an ability's
    /// source. Used as `Frame::source` at resolution.
    #[must_use]
    pub fn object(&self) -> ObjectId {
        match self {
            StackObject::Spell(o) => *o,
        }
    }
}

/// A committed stack object: resolvable, and (stage 3) scanned by triggers and
/// SBAs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackEntry {
    pub object: StackObject,
    pub controller: PlayerId,
    /// Chosen at announce (CR 601.2c); read by `Reference::Target(n)`.
    pub targets: Vec<ObjectId>,
}

/// An announce in flight (CR 601.2 / 602.2). At most one exists, ever (no
/// priority is held during the procedure). Carries scratch a committed entry
/// never has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingStackEntry {
    pub object: StackObject,
    pub controller: PlayerId,
    /// Where a spell was cast from — for cast-from-zone effects, not undo;
    /// `Hand` in stage 2.
    pub origin: Zone,
    pub targets: Vec<ObjectId>,
}

/// The bindings an effect reads during resolution (CR 608.2). Grows `x`,
/// `that_object`, bound roles, etc. in later stages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub source: ObjectId,
    pub controller: PlayerId,
    pub targets: Vec<ObjectId>,
}
