//! `TodoCard` — the in-progress, on-disk form of a card during resolution: a
//! mirror of core `Card` whose `abilities` may be `Unparsed("<oracle line>")`
//! placeholders (not yet rewritten as RON) or verbatim structured abilities.
//! `extract` builds and writes these; Plan 3's `resolve` reads, rewrites the
//! `Unparsed` entries, and writes them back; `graduate` ignores this type and
//! just tries to parse the file as core `Card`.

use deckmaste_core::{Color, ManaCost, StatValue, Subtype, Supertype, Type};
use ron::value::RawValue;
use serde::de::Deserializer;
use serde::ser::{Error as _, Serializer};
use serde::{Deserialize, Serialize};

/// One ability slot in a `.ron.todo`: either a not-yet-rewritten oracle line,
/// or a structured ability captured verbatim (so resolve preserves it and the
/// graduation reader sees a bare core ability).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TodoAbility {
    Unparsed(String),
    /// The verbatim RON of a structured ability (`Flying`, `Activated(…)`, …).
    Parsed(String),
}

// `Unparsed(String)` is an external newtype variant; everything else is a bare
// ability we must capture verbatim. So: capture the slot as a ron `RawValue`,
// then re-parse it as the one-variant helper to detect `Unparsed`. This drives
// ron's own parser (no hand-lexing of RON) and keeps structured abilities byte-
// exact. See `ability.rs`/`effect.rs` for the manual-serde precedent.
#[derive(Deserialize)]
enum UnparsedProbe {
    Unparsed(String),
}

impl<'de> Deserialize<'de> for TodoAbility {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw: Box<RawValue> = Deserialize::deserialize(deserializer)?;
        let ron_text = raw.get_ron(); // the verbatim RON of this slot
        match crate::ron_output::ron_options().from_str::<UnparsedProbe>(ron_text) {
            Ok(UnparsedProbe::Unparsed(text)) => Ok(TodoAbility::Unparsed(text)),
            Err(_) => Ok(TodoAbility::Parsed(ron_text.trim().to_owned())),
        }
    }
}

impl Serialize for TodoAbility {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TodoAbility::Unparsed(text) => {
                serializer.serialize_newtype_variant("TodoAbility", 0, "Unparsed", text)
            }
            // Emit the structured ability's RON verbatim, as a bare value.
            TodoAbility::Parsed(ron_text) => RawValue::from_ron(ron_text)
                .map_err(S::Error::custom)?
                .serialize(serializer),
        }
    }
}

/// A face whose abilities are `TodoAbility`. Field set mirrors
/// `deckmaste_core::CardFace`; the skip/default attrs match it so a fully
/// resolved face is byte-identical to a core `CardFace`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TodoCardFace {
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "ManaCost::is_empty",
        serialize_with = "crate::ron_output::one_line_if_single"
    )]
    pub mana_cost: ManaCost,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::ron_output::one_line_if_single"
    )]
    pub color_indicator: Vec<Color>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::ron_output::one_line_if_single"
    )]
    pub supertypes: Vec<Supertype>,
    #[serde(serialize_with = "crate::ron_output::one_line_if_single")]
    pub types: Vec<Type>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "crate::ron_output::one_line_if_single"
    )]
    pub subtypes: Vec<Subtype>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<TodoAbility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<StatValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<StatValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<StatValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<StatValue>,
}

/// The two layouts core `Card` supports.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum TodoCard {
    Normal(TodoCardFace),
    ModalDfc(TodoCardFace, TodoCardFace),
}

/// Renders a `TodoCard` to house-style RON (shared `ron_output` config). When
/// every ability is `Parsed`, the output is byte-identical to a finished core
/// `Card`, so graduation's rename produces a valid `.ron`.
///
/// # Errors
///
/// Returns an error if the value cannot be serialized as RON (e.g. a `Parsed`
/// ability holding text that is not valid RON).
pub fn render(card: &TodoCard) -> anyhow::Result<String> {
    Ok(crate::ron_output::to_string_pretty(card)? + "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(source: &str) -> TodoCard { crate::ron_output::ron_options().from_str(source).unwrap() }

    /// An `Unparsed("…")` ability reads as the placeholder; a bare structured
    /// ability (`Flying`) reads as `Parsed` holding its verbatim RON.
    #[test]
    fn abilities_classify() {
        let card = read(
            r#"Normal(name: "X", types: [Creature], abilities: [Unparsed("Draw a card."), Flying], power: 1, toughness: 1)"#,
        );
        let TodoCard::Normal(face) = card else {
            panic!("expected Normal");
        };
        assert_eq!(face.abilities.len(), 2);
        assert!(matches!(&face.abilities[0], TodoAbility::Unparsed(s) if s == "Draw a card."));
        assert!(matches!(&face.abilities[1], TodoAbility::Parsed(raw) if raw.trim() == "Flying"));
    }

    /// Keystone: a fully-resolved `TodoCard` (every ability `Parsed`) renders
    /// byte-identical to a finished core `Card` file, so graduation's rename
    /// produces a valid `.ron`. Uses the real `plugins/canon/Lightning Bolt`
    /// bytes: a `mana_cost: [Red]` inline single array and a multi-line `Spell`
    /// ability whose `RawValue` capture must reproduce it verbatim — the
    /// bare-`3`/`AnyTarget` macro sugar a finished card may use is preserved
    /// untouched (the graduate reader, not this layer, parses it as `Card`).
    #[test]
    fn resolved_matches_canon_byte_for_byte() {
        let source = r#"Normal(
    name: "Lightning Bolt",
    mana_cost: [Red],
    types: [Instant],
    abilities: [
        Spell(
            targets: [AnyTarget],
            effect: DealDamage(Target(0), 3),
        ),
    ],
)
"#;
        // Every ability classified as `Parsed` (no `Unparsed` left).
        let card = read(source);
        let TodoCard::Normal(face) = &card else {
            panic!("expected Normal");
        };
        assert!(matches!(&face.abilities[..], [TodoAbility::Parsed(_)]));

        // Byte-for-byte reproduction of the on-disk house style: the inline
        // single `mana_cost`, the multi-line `Spell`, and the macro sugar all
        // survive the read/render round-trip unchanged.
        assert_eq!(render(&card).unwrap(), source);
    }

    /// Round-trip: read then render reproduces the source (house style).
    #[test]
    fn round_trips() {
        let source = r#"Normal(
    name: "X",
    types: [Creature],
    abilities: [
        Unparsed("Draw a card."),
        Flying,
    ],
    power: 1,
    toughness: 1,
)
"#;
        let rendered = render(&read(source)).unwrap();
        assert_eq!(rendered, source);
    }
}
