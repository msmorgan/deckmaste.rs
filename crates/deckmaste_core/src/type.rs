use macro_ron::Ident;
use serde::Deserialize;
use serde::Serialize;

use crate::Expand;
use crate::Property;

// [CR#300.1]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Type {
    Artifact,
    Battle,
    // Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Kindred,
    Land,
    // Phenomenon,
    // Plane,
    Planeswalker,
    // Scheme,
    Sorcery,
    // Vanguard,
}

// [CR#205.4a]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

/// A subtype: its name, the card types it can appear on ([CR#205.3]), and
/// what it confers on its bearers — how [CR#305.6] gives basic lands their
/// mana abilities, as plugin data rather than an engine special case.
/// Embedded in the value: a macro-expanded card describes the entirety of
/// its behavior.
///
/// Subtypes are open-ended data, declared by plugins (usually as macro
/// definitions produced by meta-macros like
/// `LandType(name: "Forest", template: "Forest")`) rather than baked in as
/// Rust variants.
/// Plain serde on both sides; card files reference declared subtypes by bare
/// name (`Forest`), which the macro-aware reader expands to the full
/// declaration before this type ever sees it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Subtype {
    pub name: Ident,
    pub types: Vec<Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub confers: Vec<Property>,
}

/// An open, plugin-declared card type ([CR#300.1]): its open name, whether it
/// is a PERMANENT type ([CR#608.3] — a spell of this type enters the
/// battlefield on resolution), and what it CONFERS on its bearer. Types are
/// macro-expanded from bare names (`Creature`) to full structs before core
/// ever sees them — exactly like [`Subtype`]. `confers` reuses `Vec<Property>`
/// (`May(Cast(…))` / `May(Play(…))` are `Ability(Static(Deontic(…)))`
/// Properties); no new type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct TypeDef {
    pub name: Ident,
    pub permanent: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub confers: Vec<Property>,
}

impl Type {
    /// This type's canonical open name ([CR#300.1]) — the `Ident` a `TypeDef`
    /// and every name-matching predicate key on.
    #[must_use]
    pub fn name(self) -> Ident {
        Ident::from(match self {
            Type::Artifact => "Artifact",
            Type::Battle => "Battle",
            Type::Creature => "Creature",
            Type::Dungeon => "Dungeon",
            Type::Enchantment => "Enchantment",
            Type::Instant => "Instant",
            Type::Kindred => "Kindred",
            Type::Land => "Land",
            Type::Planeswalker => "Planeswalker",
            Type::Sorcery => "Sorcery",
        })
    }

    /// Whether a spell of this type enters the battlefield on resolution
    /// ([CR#608.3]) rather than resolving as a one-shot effect. The six
    /// permanent types are true; Instant/Sorcery/Kindred/Dungeon false.
    #[must_use]
    pub const fn permanent(self) -> bool {
        matches!(
            self,
            Type::Artifact
                | Type::Battle
                | Type::Creature
                | Type::Enchantment
                | Type::Land
                | Type::Planeswalker
        )
    }

    /// The STRUCTURAL canonical [`TypeDef`] — name + `permanent`, EMPTY
    /// `confers`. Fixtures use it so structure-only tests need no plugin load;
    /// real games get `confers` via the plugin-loaded registry. NOT `const`:
    /// `Ident::new` interns at runtime.
    #[must_use]
    pub fn def(self) -> TypeDef {
        TypeDef {
            name: self.name(),
            permanent: self.permanent(),
            confers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typedef_round_trips_and_skips_empty_confers() {
        let land = TypeDef {
            name: "Land".into(),
            permanent: true,
            confers: Vec::new(),
        };
        let written = crate::ron::options().to_string(&land).unwrap();
        // Plain top-level structs serialize without their type name prefix
        // (unlike enum-variant positions); empty `confers` is skipped.
        assert_eq!(written, r#"(name:"Land",permanent:true)"#);
        assert_eq!(
            crate::ron::options().from_str::<TypeDef>(&written).unwrap(),
            land
        );
    }

    #[test]
    fn def_is_structural_name_and_permanent_empty_confers() {
        assert_eq!(
            Type::Creature.def(),
            TypeDef {
                name: "Creature".into(),
                permanent: true,
                confers: Vec::new()
            }
        );
        assert_eq!(
            Type::Instant.def(),
            TypeDef {
                name: "Instant".into(),
                permanent: false,
                confers: Vec::new()
            }
        );
        assert_eq!(Type::Land.name(), Ident::from("Land"));
        // Every permanent type flags true; the two spell types + Kindred + Dungeon
        // false.
        for t in [
            Type::Artifact,
            Type::Battle,
            Type::Creature,
            Type::Enchantment,
            Type::Land,
            Type::Planeswalker,
        ] {
            assert!(t.permanent(), "{t:?} is a permanent type");
        }
        for t in [Type::Instant, Type::Sorcery, Type::Kindred, Type::Dungeon] {
            assert!(!t.permanent(), "{t:?} is not a permanent type");
        }
    }
}
