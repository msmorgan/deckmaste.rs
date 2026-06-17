use serde::Deserialize;
use serde::Serialize;

use crate::Expand;

/// An object's status ([CR#110.5]): four categories, each with two values —
/// tapped/untapped, flipped/unflipped, face up/face down, phased in/phased
/// out. Filtered via `Filter`'s `Status` atom; matched as a transition via
/// `Event::StateBecomes`. Permanents enter untapped, unflipped, face up,
/// and phased in unless something says otherwise ([CR#110.5b]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Status {
    /// [CR#110.5].
    Tapped,
    /// [CR#110.5].
    Untapped,
    /// [CR#110.5].
    Flipped,
    /// [CR#110.5].
    Unflipped,
    /// [CR#110.5].
    FaceDown,
    /// [CR#110.5].
    FaceUp,
    /// [CR#110.5].
    PhasedOut,
    /// [CR#110.5].
    PhasedIn,
}

/// Which face an object shows as it changes zones — the zone-change master
/// event's `face` coordinate (mtg-rules events.md §2). `None` on an event
/// means the default: face up ([CR#110.5b]); cards in hidden zones have no
/// face status — they are hidden by ZONE, not by face ([CR#400.2]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Face {
    Up,
    Down,
}

/// The phased-in/phased-out pair ([CR#110.5] status category; [CR#702.26b]
/// — phasing is a status change, explicitly NOT a zone change). The
/// becomes-delta companion to [`Face`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Phasing {
    In,
    Out,
}

/// The face-down payload position — a single-variant enum on purpose
/// (the accretion point): [CR#406.3a]'s face-down-exile case (NO
/// characteristics at all) is a foreseeable `Nothing` sibling, landing
/// here without respelling existing files.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Expand, Serialize)]
pub enum FaceDownSpec {
    /// The characteristics the enabler LISTS ([CR#708.2]).
    Listed(FaceDownCharacteristics),
}

impl Default for FaceDownSpec {
    fn default() -> Self {
        FaceDownSpec::Listed(FaceDownCharacteristics::default())
    }
}

/// The characteristics a face-down object HAS ([CR#708.2]: only those
/// listed by the ability or rules that turned it face down — everything
/// else is absent, not inherited). `Default` is [CR#708.2a]: a 2/2
/// creature with no name, no text, no subtypes, and no mana cost.
///
/// The engine's face-down object model consumes this as the committed
/// payload (mtg-rules information.md §6): what the object shows while the
/// real card stays hidden, with look rights ([CR#708.5]), the
/// differentiation duty ([CR#708.6]), and reveal-on-leave ([CR#708.9]) as
/// engine seams.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Expand, Serialize)]
pub struct FaceDownCharacteristics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<crate::Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<crate::Subtype>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<crate::Ability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power: Option<crate::StatValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toughness: Option<crate::StatValue>,
}

impl Default for FaceDownCharacteristics {
    fn default() -> Self {
        FaceDownCharacteristics {
            name: None,
            types: vec![crate::Type::Creature],
            subtypes: Vec::new(),
            abilities: Vec::new(),
            power: Some(crate::StatValue::Number(2)),
            toughness: Some(crate::StatValue::Number(2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The face-down payload reads through the wrapper (`Listed(…)`), and
    /// the default is the [CR#708.2a] 2/2 creature.
    #[test]
    fn face_down_spec_reads_listed_and_defaults_to_the_two_two() {
        let spec: FaceDownSpec = crate::ron::options()
            .from_str("Listed(types: [Creature], power: 2, toughness: 2)")
            .unwrap();
        let FaceDownSpec::Listed(c) = &spec;
        assert_eq!(c.power, Some(crate::StatValue::Number(2)));
        assert_eq!(
            FaceDownSpec::default(),
            FaceDownSpec::Listed(FaceDownCharacteristics::default())
        );
    }
}
