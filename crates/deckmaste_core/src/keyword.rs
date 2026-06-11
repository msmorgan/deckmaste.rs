//! The intrinsic keyword abilities ([CR#702]) the engine treats as first-class:
//! a closed enum the combat code pattern-matches on, rather than plugin macros.
//! The original six are woven into the damage pipeline (first/double strike,
//! deathtouch, trample, vigilance, lifelink); Flying is carried here too as an
//! intrinsic evasion keyword, encoded on cards as `Keyword(Flying)`. Haste is
//! **not** here — it is a `Permission` macro (it only touches the
//! can-attack/tap rules); other non-intrinsic keywords are plugin macros.
//!
//! The grammar lands now; the behaviors arrive in later combat tasks.

use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use crate::Ability;
use crate::Expand;
use crate::Ident;

/// A keyword ability the engine treats as a first-class combat concept
/// ([CR#702]). Carried by [`Ability::Keyword`](crate::Ability::Keyword).
///
/// The printed name (and the RON spelling) is the variant identifier itself —
/// `FirstStrike`, not `First strike` — matching the project's convention of
/// reading enum variants verbatim in RON. [`as_str`](Self::as_str) /
/// [`Display`] / [`FromStr`] expose that mapping for the future
/// `Modification::LoseAbility(Ident)` / `HasAbility(Ident)` paths
/// ([CR#613.1f]), which name abilities by string.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, Expand)]
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
    /// [CR#702.15].
    Lifelink,
    /// [CR#702.9]: evasion — can be blocked only by creatures with flying
    /// and/or reach. Behavior arrives with a later combat task.
    Flying,
    /// A keyword COMPOSED of other abilities ([CR#702]), as opposed to the
    /// intrinsic variants above that the engine implements natively: its
    /// printed name plus the abilities it stands for, carried IN the
    /// grammar — so the name survives `expand_all` (provenance wrappers do
    /// not) and the `LoseAbility`/`CantHaveAbility`/`HasAbility` name paths
    /// match it through [`as_str`](Self::as_str) like any intrinsic.
    /// Produced by keyword macros (`Ward([...])`, `Islandwalk`, …) — RON:
    /// `Keyword(Composite(name: "Ward", abilities: [...]))`. The engine
    /// executes the carried abilities; display of parameterized forms lives
    /// in the macro decl's `template:` field (a deferred seam).
    Composite {
        name: Ident,
        abilities: Vec<Ability>,
    },
}

impl KeywordAbility {
    /// Every INTRINSIC variant, for iteration in tests and exhaustive
    /// mappings. The open-ended `Composite { .. }` form is deliberately
    /// absent.
    pub const ALL: [KeywordAbility; 7] = [
        KeywordAbility::FirstStrike,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Deathtouch,
        KeywordAbility::Trample,
        KeywordAbility::Vigilance,
        KeywordAbility::Lifelink,
        KeywordAbility::Flying,
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
            KeywordAbility::Lifelink => "Lifelink",
            KeywordAbility::Flying => "Flying",
            // `Ident` interns to a 'static str, so composite keywords keep
            // the same lifetime story as the intrinsics.
            KeywordAbility::Composite { name, .. } => name.as_str(),
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
            "Lifelink" => KeywordAbility::Lifelink,
            "Flying" => KeywordAbility::Flying,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            (KeywordAbility::Lifelink, "Lifelink"),
            (KeywordAbility::Flying, "Flying"),
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
