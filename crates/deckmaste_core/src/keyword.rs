//! The native keyword abilities ([CR#702]) the engine implements directly:
//! a closed enum the combat code pattern-matches on, rather than plugin macros.
//! Per the keyword classification (intrinsic / composite / composite-given —
//! docs/rules-taxonomy.md §10, pinned to the mtg-rules skill v1.7.0), the
//! variants are exactly the five implemented true intrinsics: first/double
//! strike, deathtouch, and trample own prospective combat-damage machinery
//! ([CR#510.1]); vigilance owns dedicated declare-attackers text
//! ([CR#702.20a..702.20b,508.1f]). Every non-intrinsic is a
//! `KeywordAbility`-kind plugin macro invoked INSIDE the wrapper — cards
//! always read `Keyword(Flying)` — expanding to
//! [`Composite`](KeywordAbility::Composite); see
//! `plugins/builtin/macros/keyword/`: flying carries its real evasion
//! `Cant` ([CR#702.9b]); lifelink and reach carry their names alone
//! (lifelink's combat hook matches by NAME through
//! [`as_str`](KeywordAbility::as_str) — the look-through seam — pending the
//! damage-result-rewrite stage; reach is the marker keyword). Haste is a
//! flag the standing summoning-sickness `Cant` rows read in their own
//! conditions ([CR#508.1a,602.5a]; [CR#702.10b..702.10c,302.6] mirror
//! them); indestructible = an event-side "can't be destroyed", the
//! replacement-family seam.
//!
//! The grammar lands now; the behaviors arrive in later combat tasks.

use std::fmt;
use std::str::FromStr;

use crate::Ability;
use crate::Expansion;
use crate::Ident;
use crate::SupportsMacros;

/// A keyword name at a REFERENCE position ("has flying", `Has(Flying)`):
/// spelled as a bare identifier, like `kinds: [Subtype]` and param-type
/// names — the enum channel reads it, and the writer emits it bare. It is
/// a NAME, never a value: nothing expands (so the evasion family's
/// self-references — flying's "without flying or reach" — cannot recurse),
/// and matching is by name through the same path as
/// [`as_str`](KeywordAbility::as_str). Validating the name against the
/// keyword namespace (native enum ∪ `KeywordAbility`-kind macros) is a
/// lint, not a parse concern (`filter.rs` doc) — the lint arrives with
/// the keyword buildout's filter visitor, and its bar is "doesn't assert
/// nonsense" (an unknown keyword name), not shape-policing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeywordRef(pub Ident);

impl KeywordRef {
    #[must_use]
    pub fn as_str(&self) -> &'static str { self.0.as_str() }
}

impl From<&str> for KeywordRef {
    fn from(s: &str) -> Self { KeywordRef(s.into()) }
}

impl crate::Expand for KeywordRef {
    // A leaf: a name, never an expandable value.
    fn expand_all(self) -> Self { self }
}

impl serde::Serialize for KeywordRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // A unit variant writes as a bare identifier in RON.
        serializer.serialize_unit_variant("KeywordRef", 0, self.0.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for KeywordRef {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // A bare identifier is a unit enum variant in the serde data model —
        // the same channel `kinds: [Subtype]` and param-type names read
        // through (ron's `any` channel DROPS identifier names; the enum
        // channel preserves them).
        struct NameVisitor;
        impl<'de> serde::de::Visitor<'de> for NameVisitor {
            type Value = KeywordRef;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a keyword name (bare identifier)")
            }
            fn visit_enum<A: serde::de::EnumAccess<'de>>(
                self,
                data: A,
            ) -> Result<Self::Value, A::Error> {
                use serde::de::VariantAccess;
                let (ident, variant) = data.variant_seed(macro_ron::IdentSeed)?;
                variant.unit_variant()?;
                Ok(KeywordRef(ident))
            }
        }
        deserializer.deserialize_enum("", &[], NameVisitor)
    }
}

/// A keyword ability the engine treats as a first-class combat concept
/// ([CR#702]). Carried by [`Ability::Keyword`](crate::Ability::Keyword).
///
/// The printed name (and the RON spelling) is the variant identifier itself —
/// `FirstStrike`, not `First strike` — matching the project's convention of
/// reading enum variants verbatim in RON. [`as_str`](Self::as_str) /
/// [`Display`] / [`FromStr`] expose that mapping for the future
/// `Modification::LoseAbility(Ident)` / `Has(KeywordRef)` paths
/// ([CR#613.1f]), which name abilities by string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum KeywordAbility {
    /// [CR#702.7].
    FirstStrike,
    /// [CR#702.4].
    DoubleStrike,
    /// [CR#702.2].
    Deathtouch,
    /// [CR#702.19].
    Trample,
    /// [CR#702.20].
    Vigilance,
    /// A keyword COMPOSED of other abilities ([CR#702]), as opposed to the
    /// intrinsic variants above that the engine implements natively: its
    /// printed name plus the abilities it stands for, carried IN the
    /// grammar — so the name survives `expand_all` (provenance wrappers do
    /// not) and the `LoseAbility`/`CantHaveAbility`/`Has` name paths
    /// match it through [`as_str`](Self::as_str) like any intrinsic.
    /// Produced by keyword macros (`Ward([...])`, `Islandwalk`, …) — RON:
    /// `Keyword(Composite(name: "Ward", abilities: [...]))`. The engine
    /// executes the carried abilities; display of parameterized forms lives
    /// in the macro decl's `template:` field (a deferred seam).
    Composite {
        name: Ident,
        abilities: Vec<Ability>,
    },
    /// A remembered `KeywordAbility` macro invocation — the non-intrinsic
    /// keywords, invoked INSIDE the wrapper so card definitions always call
    /// out keyword-ness explicitly: `Keyword(Flying)`, `Keyword(Ward([…]))`.
    /// Serialized as the invocation, not the struct.
    #[macro_ron(expanded)]
    Expanded(Expansion<KeywordAbility>),
}

impl KeywordAbility {
    /// Every INTRINSIC variant, for iteration in tests and exhaustive
    /// mappings. The open-ended `Composite { .. }` form is deliberately
    /// absent.
    pub const ALL: [KeywordAbility; 5] = [
        KeywordAbility::FirstStrike,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Deathtouch,
        KeywordAbility::Trample,
        KeywordAbility::Vigilance,
    ];

    /// The printed name — identical to the variant identifier and the RON
    /// spelling. The pairing with [`from_str`](Self::from_str) is the
    /// name<->value bridge the `Ident`-keyed modification ops need.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            KeywordAbility::FirstStrike => "FirstStrike",
            KeywordAbility::DoubleStrike => "DoubleStrike",
            KeywordAbility::Deathtouch => "Deathtouch",
            KeywordAbility::Trample => "Trample",
            KeywordAbility::Vigilance => "Vigilance",
            // `Ident` interns to a 'static str, so composite keywords keep
            // the same lifetime story as the intrinsics.
            KeywordAbility::Composite { name, .. } => name.as_str(),
            // An unexpanded invocation answers with its expansion's name, so
            // the name bridge holds whether or not `expand_all` has run.
            KeywordAbility::Expanded(e) => e.value.as_str(),
        }
    }
}

impl fmt::Display for KeywordAbility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

impl FromStr for KeywordAbility {
    type Err = ();

    /// Intrinsics only: a `Composite { .. }` carries its expansion, so it
    /// cannot be constructed from a bare name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "FirstStrike" => KeywordAbility::FirstStrike,
            "DoubleStrike" => KeywordAbility::DoubleStrike,
            "Deathtouch" => KeywordAbility::Deathtouch,
            "Trample" => KeywordAbility::Trample,
            "Vigilance" => KeywordAbility::Vigilance,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Expand;

    /// `as_str`/`from_str` round-trip and pin the printed (RON) spelling of
    /// every variant — the variant identifier, no spaces.
    #[test]
    fn name_mapping_is_the_variant_identifier() {
        let expected = [
            (KeywordAbility::FirstStrike, "FirstStrike"),
            (KeywordAbility::DoubleStrike, "DoubleStrike"),
            (KeywordAbility::Deathtouch, "Deathtouch"),
            (KeywordAbility::Trample, "Trample"),
            (KeywordAbility::Vigilance, "Vigilance"),
        ];
        for (kw, name) in &expected {
            assert_eq!(kw.as_str(), *name);
            assert_eq!(kw.to_string(), *name);
            assert_eq!(name.parse::<KeywordAbility>(), Ok(kw.clone()));
        }
        // `ALL` and the mapping table cover the same variants.
        assert_eq!(KeywordAbility::ALL.len(), expected.len());
    }

    /// Each intrinsic variant serializes to its bare identifier in RON.
    #[test]
    fn ron_round_trips_each_variant() {
        for kw in KeywordAbility::ALL {
            let written = crate::ron::options().to_string(&kw).unwrap();
            assert_eq!(written, kw.as_str());
            let read: KeywordAbility = crate::ron::options().from_str(&written).unwrap();
            assert_eq!(read, kw);
        }
    }

    fn ward() -> KeywordAbility {
        KeywordAbility::Composite {
            name: Ident::from("Ward"),
            abilities: Vec::new(),
        }
    }

    /// A composite keyword's `as_str`/`Display` is its printed name — the
    /// same bridge the `Ident`-keyed modification ops use for intrinsics —
    /// and a bare name does NOT parse into one (the expansion can't be
    /// conjured).
    #[test]
    fn composite_keyword_exposes_its_name() {
        assert_eq!(ward().as_str(), "Ward");
        assert_eq!(ward().to_string(), "Ward");
        assert_eq!("Ward".parse::<KeywordAbility>(), Err(()));
    }

    /// The composite form round-trips through RON under its own tag,
    /// leaving the intrinsics' bare-identifier spelling untouched.
    #[test]
    fn composite_keyword_round_trips_in_ron() {
        let written = crate::ron::options().to_string(&ward()).unwrap();
        assert_eq!(written, r#"Composite(name:"Ward",abilities:[])"#);
        let read: KeywordAbility = crate::ron::options().from_str(&written).unwrap();
        assert_eq!(read, ward());
    }

    /// THE point of the in-grammar name: `expand_all` strips `Expanded`
    /// provenance wrappers inside the carried abilities but the keyword's
    /// name survives.
    #[test]
    fn expand_all_keeps_the_name() {
        let kw = ward().expand_all();
        assert_eq!(kw.as_str(), "Ward");
    }
}
