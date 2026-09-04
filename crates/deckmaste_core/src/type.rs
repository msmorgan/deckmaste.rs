use std::sync::Arc;

use crate::Ident;
use serde::Deserialize;
use serde::Serialize;

use crate::Property;

// [CR#300.1]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
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
/// Embedded in the lowered value, so a core card describes the entirety of its
/// behavior.
///
/// Subtypes are open-ended data, declared by plugins (usually as macro
/// definitions produced by meta-macros like
/// `LandType(name: "Forest", template: "Forest")`) rather than baked in as
/// Rust variants. Semantic lowering resolves authored subtype names to the
/// full declaration before core sees them.
#[derive(Debug, Clone, Eq, Deserialize, Serialize)]
pub struct Subtype {
    pub name: Ident,
    pub types: Arc<[Type]>,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub confers: Arc<[Property]>,
}

// A subtype's identity is its NAME: semantic lowering resolves it from one
// declaration, so `types`/`confers` are redundant to compare — they ride along in the
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
/// resolved from authored bare names (`Creature`) to full structs before core
/// ever sees them — exactly like [`Subtype`]. `confers` reuses `Vec<Property>`
/// (`May(Cast(…))` / `May(Play(…))` are `Ability(Static(Deontic(…)))`
/// Properties); no new type.
#[derive(Debug, Clone, Eq, Deserialize, Serialize)]
pub struct TypeDef {
    pub name: Ident,
    pub permanent_type: bool,
    #[serde(default, skip_serializing_if = "crate::slice_is_empty")]
    pub confers: Arc<[Property]>,
}

// Identity is the NAME (see [`Subtype`]'s eq): a type ref always resolves to
// one declaration for that name, so `permanent_type`/`confers` are redundant
// to compare.
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
    pub const fn permanent_type(self) -> bool {
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

    /// The STRUCTURAL canonical [`TypeDef`] — name + `permanent_type`, EMPTY
    /// `confers`. Fixtures use it so structure-only tests need no plugin load;
    /// real games get `confers` via the plugin-loaded registry. NOT `const`:
    /// `Ident::new` interns at runtime.
    #[must_use]
    pub fn def(self) -> TypeDef {
        TypeDef {
            name: self.name(),
            permanent_type: self.permanent_type(),
            confers: [].into(),
        }
    }
}

/// A resolved card-type reference at a filter position. Core snapshots carry
/// the complete [`TypeDef`]; the semantics layer owns compact authored names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TypeRef(pub Arc<TypeDef>);

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

    /// A NAME-ONLY ref (`permanent_type: false`, empty `confers`) for
    /// render/engine code that constructs a filter ref from a bare [`Ident`]
    /// — identity is by-name, so the fabricated `permanent_type`/`confers`
    /// never participate in a match. Not for reconstructing the authoritative
    /// def (parse/registry carry that); use [`Type::def`] / the loaded
    /// registry for that.
    #[must_use]
    pub fn named(name: Ident) -> Self {
        TypeRef(Arc::new(TypeDef {
            name,
            permanent_type: false,
            confers: [].into(),
        }))
    }
}

impl From<Type> for TypeRef {
    /// The structural ref for a closed [`Type`] — name + `permanent_type`,
    /// EMPTY `confers` (see [`Type::def`]). By-name eq makes it match a
    /// parse-built ref whose `Arc` carries the full `confers`.
    fn from(t: Type) -> Self {
        TypeRef(Arc::new(t.def()))
    }
}

/// A resolved subtype reference at a filter position. Core snapshots carry
/// the complete [`Subtype`]; compact authored names are semantic syntax.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SubtypeRef(pub Arc<Subtype>);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typedef_round_trips_and_skips_empty_confers() {
        let land = TypeDef {
            name: "Land".into(),
            permanent_type: true,
            confers: [].into(),
        };
        let written = crate::ron::options().to_string(&land).unwrap();
        // Plain top-level structs serialize without their type name prefix
        // (unlike enum-variant positions); empty `confers` is skipped.
        assert_eq!(written, r#"(name:"Land",permanent_type:true)"#);
        assert_eq!(
            crate::ron::options().from_str::<TypeDef>(&written).unwrap(),
            land
        );
    }

    #[test]
    fn def_is_structural_name_and_permanent_type_empty_confers() {
        assert_eq!(
            Type::Creature.def(),
            TypeDef {
                name: "Creature".into(),
                permanent_type: true,
                confers: [].into()
            }
        );
        assert_eq!(
            Type::Instant.def(),
            TypeDef {
                name: "Instant".into(),
                permanent_type: false,
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
            assert!(t.permanent_type(), "{t:?} is a permanent type");
        }
        for t in [Type::Instant, Type::Sorcery, Type::Kindred, Type::Dungeon] {
            assert!(!t.permanent_type(), "{t:?} is not a permanent type");
        }
    }

    /// Land witness: Land is a Permanent Type ([CR#110.4]) even though a
    /// land card is never a Permanent Spell ([CR#110.4b]) — it's played, not
    /// cast ([CR#305.1]), so it never resolves as a spell in the first place.
    /// `permanent_type` records ONLY the Permanent Type membership; nothing
    /// here encodes cast-ability or the Spell/Permanent role distinction.
    #[test]
    fn land_is_a_permanent_type() {
        assert!(Type::Land.permanent_type());
        assert!(Type::Land.def().permanent_type);
    }
}
