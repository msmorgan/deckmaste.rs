use serde::{Deserialize, Serialize};

/// A bound variable: a value fixed earlier (at announce, by the rules of
/// the position, or by a binder) and referenced later. References name
/// *objects*; amounts live in [`crate::Quantity`].
///
/// Players are objects — `You`, `ControllerOf`, `OwnerOf` resolve to
/// player objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Reference {
    /// The object this ability is printed on / the resolving spell.
    This,
    /// The controller of this ability (CR 109.5).
    You,
    /// The nth target this ability announced (CR 115.3, 601.2c).
    Target(usize),
    /// The object that participated in the enclosing trigger's event —
    /// bound by the trigger's event pattern (CR 603.2e, "that creature").
    ThatObject,
    /// The player that participated in the enclosing trigger's event —
    /// bound by the trigger's event pattern (CR 603.2e, "that player").
    ThatPlayer,
    /// A named role bound by an event pattern or instruction (e.g. the
    /// attacker vs. the blocker).
    Bound(crate::Ident),
    /// Information remembered by a linked ability (CR 607): the object
    /// exiled with this, the chosen value, the cost paid.
    Linked(crate::Ident),
    /// The controller of a referenced object (CR 109.5).
    ControllerOf(Box<Reference>),
    /// The owner of a referenced object (CR 108.3).
    OwnerOf(Box<Reference>),
    /// The object this Aura enchants (CR 303.4).
    EnchantedObject,
    /// The creature this Equipment is attached to (CR 301.5).
    EquippedCreature,
    /// The object a referenced attachment is attached to (CR 701.3 family).
    AttachedTo(Box<Reference>),
}
