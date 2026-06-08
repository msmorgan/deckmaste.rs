use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::Expansion;

#[cfg(test)]
mod tests {
    use super::Reference;

    fn read(source: &str) -> Reference { crate::ron::options().from_str(source).unwrap() }

    #[test]
    fn attach_host_of_round_trips() {
        let v = Reference::AttachHostOf(Box::new(Reference::This));
        let w = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&w), v);
    }
}

/// A bound variable: a value fixed earlier (at announce, by the rules of
/// the position, or by a binder) and referenced later. References name
/// *objects*; amounts live in [`crate::Quantity`].
///
/// Players are objects — `You`, `ControllerOf`, `OwnerOf` resolve to
/// player objects.
///
/// `Deserialize` is derived (the macro reader synthesizes the `Expanded`
/// stream); `Serialize` is **manual** so `Expanded` writes the invocation
/// back rather than the literal struct — the other variants mirror the derive.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Reference {
    /// The object this ability is printed on / the resolving spell.
    This,
    /// The controller of this ability ([CR#109.5]).
    You,
    /// The nth target this ability announced ([CR#115.3,601.2c]).
    Target(usize),
    /// The object that participated in the enclosing trigger's event —
    /// bound by the trigger's event pattern ([CR#603.2e], "that creature").
    ThatObject,
    /// The player that participated in the enclosing trigger's event —
    /// bound by the trigger's event pattern ([CR#603.2e], "that player").
    ThatPlayer,
    /// A named role bound by an event pattern or instruction (e.g. the
    /// attacker vs. the blocker).
    Bound(crate::Ident),
    /// Information remembered by a linked ability ([CR#607]): the object
    /// exiled with this, the chosen value, the cost paid.
    Linked(crate::Ident),
    /// The controller of a referenced object ([CR#109.5]).
    ControllerOf(Box<Reference>),
    /// The owner of a referenced object ([CR#108.3]).
    OwnerOf(Box<Reference>),
    /// The permanent that attachment R is attached to — attachment→host
    /// direction ([CR#301.5,303.4]).  Covers Equipment hosts, Aura
    /// enchantees, and Fortification hosts alike.
    AttachHostOf(Box<Reference>),
    /// What is attached to R — host→attachment direction (inverse of
    /// [`AttachHostOf`]).
    AttachedTo(Box<Reference>),
    /// A remembered `Reference` macro invocation.
    Expanded(Expansion<Reference>),
}

impl Serialize for Reference {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // serialize_*_variant index arguments are ignored by RON. Mirrors the
        // shapes the derive produced, plus the `Expanded` invocation arm.
        match self {
            Reference::This => serializer.serialize_unit_variant("Reference", 0, "This"),
            Reference::You => serializer.serialize_unit_variant("Reference", 1, "You"),
            Reference::Target(n) => {
                serializer.serialize_newtype_variant("Reference", 2, "Target", n)
            }
            Reference::ThatObject => {
                serializer.serialize_unit_variant("Reference", 3, "ThatObject")
            }
            Reference::ThatPlayer => {
                serializer.serialize_unit_variant("Reference", 4, "ThatPlayer")
            }
            Reference::Bound(role) => {
                serializer.serialize_newtype_variant("Reference", 5, "Bound", role)
            }
            Reference::Linked(key) => {
                serializer.serialize_newtype_variant("Reference", 6, "Linked", key)
            }
            Reference::ControllerOf(r) => {
                serializer.serialize_newtype_variant("Reference", 7, "ControllerOf", r)
            }
            Reference::OwnerOf(r) => {
                serializer.serialize_newtype_variant("Reference", 8, "OwnerOf", r)
            }
            Reference::AttachHostOf(r) => {
                serializer.serialize_newtype_variant("Reference", 9, "AttachHostOf", r)
            }
            Reference::AttachedTo(r) => {
                serializer.serialize_newtype_variant("Reference", 10, "AttachedTo", r)
            }
            // The invocation, not the struct.
            Reference::Expanded(e) => e.serialize(serializer),
        }
    }
}
