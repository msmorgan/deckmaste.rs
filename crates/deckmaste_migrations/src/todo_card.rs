//! `TodoCard` — the in-progress, on-disk form of a card during resolution: a
//! mirror of core `Card` whose `abilities` may be `Unparsed("<oracle line>")`
//! placeholders (not yet rewritten as RON) or verbatim structured abilities.
//! `extract` builds and writes these; Plan 3's `resolve` reads, rewrites the
//! `Unparsed` entries, and writes them back; `graduate` ignores this type and
//! just tries to parse the file as core `Card`.

use deckmaste_core::Color;
use deckmaste_core::ManaCost;
use deckmaste_core::StatValue;
use ron::value::RawValue;
use serde::Deserialize;
use serde::Serialize;
use serde::de::Deserializer;
use serde::ser::Error as _;
use serde::ser::Serializer;

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
            // Any non-`Unparsed` slot is a structured ability: the probe failure
            // means "not that variant", not "invalid RON". A genuinely malformed
            // slot will also land here; `serialize` will catch it when it calls
            // `RawValue::from_ron` on the stored string.
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

/// A bare (unquoted) RON identifier — a macro-invocation name like `Bear`,
/// `Creature`, or `TimeLord`. Card files reference their types, supertypes, and
/// subtypes by bare name; the macro-aware reader expands them. We cannot use
/// the core `Type` / `Supertype` / `Subtype` types here, because `Subtype` is a
/// struct whose bare form (`Bear`) is macro sugar that plain serde cannot
/// parse. So we capture and re-emit the ident verbatim via `RawValue` — the
/// same trick `TodoAbility::Parsed` uses for structured abilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawIdent(pub String);

impl<'de> Deserialize<'de> for RawIdent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw: Box<RawValue> = Deserialize::deserialize(deserializer)?;
        Ok(RawIdent(raw.get_ron().trim().to_owned()))
    }
}

impl Serialize for RawIdent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        RawValue::from_ron(&self.0)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}

/// A face whose abilities are `TodoAbility`. Field set mirrors
/// `deckmaste_core::CardFace`; the skip/default attrs match it so a fully
/// resolved face is byte-identical to a core `CardFace`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct TodoCardFace {
    pub name: String,
    #[serde(default, skip_serializing_if = "ManaCost::is_empty")]
    pub mana_cost: ManaCost,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub color_indicator: Vec<Color>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertypes: Vec<RawIdent>,
    pub types: Vec<RawIdent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subtypes: Vec<RawIdent>,
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

    fn read(source: &str) -> TodoCard {
        crate::ron_output::ron_options().from_str(source).unwrap()
    }

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
    /// bytes: a chopped `mana_cost` array and a multi-line `Spell` ability
    /// whose `RawValue` capture must reproduce it verbatim — the
    /// bare-`3`/`AnyTarget` macro sugar a finished card may use is
    /// preserved untouched (the graduate reader, not this layer, parses it
    /// as `Card`).
    #[test]
    fn resolved_matches_canon_byte_for_byte() {
        let source = r#"Normal(
  name: "Lightning Bolt",
  mana_cost: [
    Red,
  ],
  types: [
    Instant,
  ],
  abilities: [
    Spell(
      effect: Targeted(targets: [AnyTarget], effect: DealDamage(Target(0), 3)),
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

    /// A creature WITH subtypes (bare macro idents) round-trips byte-for-byte
    /// against the real on-disk canon card.
    #[test]
    fn subtyped_card_matches_canon_byte_for_byte() {
        let source = include_str!("../../../plugins/canon/cards/Grizzly Bears.ron");
        assert_eq!(render(&read(source)).unwrap(), source);
    }

    #[test]
    fn modal_dfc_round_trips() {
        // Exercises the ModalDfc variant with one Unparsed and one structured
        // (Parsed) ability on different faces. `Flying` is a bare keyword whose
        // verbatim RawValue round-trips without indentation concerns.
        // Note: ron serialises ModalDfc tuple fields inline — `ModalDfc((<face1>),
        // (<face2>))`.
        let source = r#"ModalDfc((
  name: "Front",
  types: [
    Instant,
  ],
  abilities: [
    Unparsed("Draw a card."),
  ],
), (
  name: "Back",
  types: [
    Sorcery,
  ],
  abilities: [
    Flying,
  ],
))
"#;
        assert_eq!(render(&read(source)).unwrap(), source);
    }

    /// A Normal face with structured fields + one Unparsed; every array is
    /// chopped one element per line (no single-element-inline shortcut).
    #[test]
    fn renders_normal_house_style() {
        let card = read(
            r#"Normal(name: "Grizzly Bears", mana_cost: [Generic(1), Green], types: [Creature], abilities: [Unparsed("When ~ dies, draw a card.")], power: 2, toughness: 2)"#,
        );
        assert_eq!(
            render(&card).unwrap(),
            r#"Normal(
  name: "Grizzly Bears",
  mana_cost: [
    Generic(1),
    Green,
  ],
  types: [
    Creature,
  ],
  abilities: [
    Unparsed("When ~ dies, draw a card."),
  ],
  power: 2,
  toughness: 2,
)
"#
        );
    }

    /// Round-trip: read then render reproduces the source (house style).
    #[test]
    fn round_trips() {
        let source = r#"Normal(
  name: "X",
  types: [
    Creature,
  ],
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
