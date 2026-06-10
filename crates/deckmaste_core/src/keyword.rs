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

use serde::{Deserialize, Serialize};

/// A keyword ability the engine treats as a first-class combat concept
/// ([CR#702]). Carried by [`Ability::Keyword`](crate::Ability::Keyword).
///
/// The printed name (and the RON spelling) is the variant identifier itself —
/// `FirstStrike`, not `First strike` — matching the project's convention of
/// reading enum variants verbatim in RON. [`as_str`](Self::as_str) /
/// [`Display`] / [`FromStr`] expose that mapping for the future
/// `Modification::LoseAbility(Ident)` / `HasAbility(Ident)` paths
/// ([CR#613.1f]), which name abilities by string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
}

impl KeywordAbility {
    /// Every variant, for iteration in tests and exhaustive mappings.
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
        }
    }
}

impl fmt::Display for KeywordAbility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

impl FromStr for KeywordAbility {
    type Err = ();

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
        for (kw, name) in expected {
            assert_eq!(kw.as_str(), name);
            assert_eq!(kw.to_string(), name);
            assert_eq!(name.parse::<KeywordAbility>(), Ok(kw));
        }
        // `ALL` and the mapping table cover the same variants.
        assert_eq!(KeywordAbility::ALL.len(), expected.len());
    }

    /// Each variant serializes to its bare identifier in RON.
    #[test]
    fn ron_round_trips_each_variant() {
        for kw in KeywordAbility::ALL {
            let written = crate::ron::options().to_string(&kw).unwrap();
            assert_eq!(written, kw.as_str());
            let read: KeywordAbility = crate::ron::options().from_str(&written).unwrap();
            assert_eq!(read, kw);
        }
    }
}
