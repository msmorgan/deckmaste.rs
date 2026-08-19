use std::sync::Arc;

use macro_ron::Ident;
use macro_ron::SupportsMacros;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SupportsMacros)]
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
#[derive(Debug, Clone, Eq, Deserialize, Expand, Serialize)]
pub struct Subtype {
    pub name: Ident,
    pub types: Arc<[Type]>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub confers: Arc<[Property]>,
}

// A subtype's identity is its NAME: it always comes from the one macro of that
// name, so `types`/`confers` are redundant to compare — they ride along in the
// `Arc` for free, and equality never touches them. (A same-name value that
// *drifts* from its declaration is a future, essentially test-only concern; the
// load-time subtype lint still catches an UNdeclared name via the registry.)
impl PartialEq for Subtype {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::hash::Hash for Subtype {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// An open, plugin-declared card type ([CR#300.1]): its open name, whether it
/// is a PERMANENT type ([CR#608.3] — a spell of this type enters the
/// battlefield on resolution), and what it CONFERS on its bearer. Types are
/// macro-expanded from bare names (`Creature`) to full structs before core
/// ever sees them — exactly like [`Subtype`]. `confers` reuses `Vec<Property>`
/// (`May(Cast(…))` / `May(Play(…))` are `Ability(Static(Deontic(…)))`
/// Properties); no new type.
#[derive(Debug, Clone, Eq, Deserialize, Expand, Serialize)]
pub struct TypeDef {
    pub name: Ident,
    pub permanent: bool,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub confers: Arc<[Property]>,
}

// Identity is the NAME (see [`Subtype`]'s eq): a type ref always resolves to
// the one macro of that name, so `permanent`/`confers` are redundant to
// compare.
impl PartialEq for TypeDef {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::hash::Hash for TypeDef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
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
            confers: [].into(),
        }
    }
}

/// A card-type reference at a FILTER position ([CR#109.3]): `Type(Creature)`.
/// It carries the RESOLVED [`TypeDef`] (the macro-aware reader expands the bare
/// name to the full struct — `confers` and all — before core deserializes it),
/// but SERIALIZES as the bare name (`Creature`), never the struct — the filter
/// channel's compact write form, so regenerated filters don't repeat every
/// type's `confers` across thousands of sites. Deserialize delegates to
/// `Arc<TypeDef>`'s `deserialize_struct("TypeDef", …)` channel (NOT a
/// `deserialize_newtype_struct` wrapper), so the macro-aware reader intercepts
/// a bare `Creature` as a `TypeDef`-kind macro and expands it in place — nested
/// in a newtype variant (`Type(Creature)`) or forwarded through a macro frame
/// (`PermanentOfType`'s `Type(Param(0))`) alike; an undeclared name has no
/// macro and fails to parse (validation for free). The type-LINE (`types:
/// [Type]`) keeps the closed [`Type`] enum; this is the open, plugin-declared
/// filter form. Identity is the def's by-name eq — `confers` ride along in the
/// `Arc` and are never compared.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Expand)]
pub struct TypeRef(pub Arc<TypeDef>);

impl<'de> Deserialize<'de> for TypeRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(TypeRef(Arc::<TypeDef>::deserialize(deserializer)?))
    }
}

impl TypeRef {
    /// The referenced type's canonical open name ([CR#300.1]) — the match key.
    #[must_use]
    pub fn name(&self) -> Ident {
        self.0.name
    }

    /// The referenced type's name as a string — the match key as `&str` (see
    /// [`TypeRef::name`]), mirroring [`SubtypeRef::as_str`].
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0.name.as_str()
    }

    /// A NAME-ONLY ref (`permanent: false`, empty `confers`) for render/engine
    /// code that constructs a filter ref from a bare [`Ident`] — identity is
    /// by-name, so the fabricated `permanent`/`confers` never participate in a
    /// match. Not for reconstructing the authoritative def (parse/registry
    /// carry that); use [`Type::def`] / the loaded registry for that.
    #[must_use]
    pub fn named(name: Ident) -> Self {
        TypeRef(Arc::new(TypeDef {
            name,
            permanent: false,
            confers: [].into(),
        }))
    }
}

impl From<Type> for TypeRef {
    /// The structural ref for a closed [`Type`] — name + `permanent`, EMPTY
    /// `confers` (see [`Type::def`]). By-name eq makes it match a parse-built
    /// ref whose `Arc` carries the full `confers`.
    fn from(t: Type) -> Self {
        TypeRef(Arc::new(t.def()))
    }
}

impl Serialize for TypeRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // A bare identifier in RON — the same channel `CounterRef` writes
        // through. The `confers` in the `Arc` are dropped from the text.
        serializer.serialize_unit_variant("TypeRef", 0, self.0.name.as_str())
    }
}

/// A subtype reference at a FILTER position ([CR#109.3]): `Subtype(Vampire)`.
/// The [`Subtype`] twin of [`TypeRef`] — resolved def in the `Arc`, bare-name
/// serialization, macro-expanded and validated on read, by-name identity. The
/// type-LINE (`subtypes: [Subtype]`) keeps the full [`Subtype`] struct; this is
/// the compact filter form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Expand)]
pub struct SubtypeRef(pub Arc<Subtype>);

impl<'de> Deserialize<'de> for SubtypeRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Delegates to `Arc<Subtype>`'s `deserialize_struct("Subtype", …)`
        // channel so a bare `Vampire` expands as a `Subtype`-kind macro — see
        // [`TypeRef`]'s Deserialize.
        Ok(SubtypeRef(Arc::<Subtype>::deserialize(deserializer)?))
    }
}

impl SubtypeRef {
    /// The referenced subtype's name ([CR#205.3]) — the match key.
    #[must_use]
    pub fn name(&self) -> Ident {
        self.0.name
    }

    /// A NAME-ONLY ref (empty `types`/`confers`) for render/engine code that
    /// constructs a filter ref from a bare [`Ident`] — identity is by-name (see
    /// [`TypeRef::named`]).
    #[must_use]
    pub fn named(name: Ident) -> Self {
        SubtypeRef(Arc::new(Subtype {
            name,
            types: [].into(),
            confers: [].into(),
        }))
    }

    /// The referenced subtype's name as a string — the layer-4 registry key
    /// (see [`SubtypeRef::name`]).
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        self.0.name.as_str()
    }
}

impl From<Ident> for SubtypeRef {
    /// A name-only ref (see [`SubtypeRef::named`]) — engine/render code holds a
    /// bare `Ident` (a layer-4 `Subtypes(...)` op key) and needs the ref form.
    fn from(name: Ident) -> Self {
        SubtypeRef::named(name)
    }
}

impl From<&str> for SubtypeRef {
    fn from(name: &str) -> Self {
        SubtypeRef::named(name.into())
    }
}

impl Serialize for SubtypeRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit_variant("SubtypeRef", 0, self.0.name.as_str())
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
            confers: [].into(),
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
                confers: [].into()
            }
        );
        assert_eq!(
            Type::Instant.def(),
            TypeDef {
                name: "Instant".into(),
                permanent: false,
                confers: [].into()
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

    fn closed_type_name(card_type: Type) -> &'static str {
        match card_type {
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
        }
    }

    #[test]
    fn type_line_remains_closed_to_the_ten_modeled_types() {
        let modeled = [
            Type::Artifact,
            Type::Battle,
            Type::Creature,
            Type::Dungeon,
            Type::Enchantment,
            Type::Instant,
            Type::Kindred,
            Type::Land,
            Type::Planeswalker,
            Type::Sorcery,
        ];
        assert_eq!(
            modeled.map(closed_type_name),
            [
                "Artifact",
                "Battle",
                "Creature",
                "Dungeon",
                "Enchantment",
                "Instant",
                "Kindred",
                "Land",
                "Planeswalker",
                "Sorcery",
            ]
        );

        let open = macro_ron::v2::read_str(
            "Chronicle.ron",
            r#"Type(
                name: "Chronicle",
                spelling: "chronicle",
                grammar: Noun(singular: "chronicle", plural: Unavailable),
            )"#,
        )
        .unwrap();
        assert_eq!(open.identity().name(), "Chronicle");
        assert!(
            crate::ron::options()
                .from_str::<Type>(open.identity().name())
                .is_err(),
            "an open parser-facing Type declaration must not extend the closed type-line enum"
        );

        for unsupported in ["Conspiracy", "Phenomenon", "Plane", "Scheme", "Vanguard"] {
            assert!(
                crate::ron::options().from_str::<Type>(unsupported).is_err(),
                "{unsupported} is a CR card type, but not a supported semantic type-line member"
            );
        }
    }
}
