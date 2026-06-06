//! The card todo file shape: written by `_004`, read back by every later
//! migration that turns todos into real definitions. Plain ron/serde on
//! both sides — todo files quote everything, so no macro awareness needed.

use deckmaste_core::{Color, Ident, ManaCost};
use serde::{Deserialize, Serialize};

use crate::ron_output::one_line_if_single;

/// Numbers serialize untagged (`power: 2`); anything else keeps its tag
/// (`power: Other("*")`). Untagged variants must come last in the enum.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum Stat {
    Other(String),
    #[serde(untagged)]
    Number(serde_json::Number),
}

/// `Number(2)` for values that parse as JSON numbers, `Other("*")`
/// otherwise.
pub(super) fn stat(value: &str) -> Stat {
    match serde_json::from_str(value) {
        Ok(number) => Stat::Number(number),
        Err(_) => Stat::Other(value.to_owned()),
    }
}

/// A card todo file is always `Todo(layout: ..., faces: [...])`, with the
/// MTGJSON layout name verbatim and one anonymous struct per face.
#[derive(Debug, Serialize, Deserialize)]
pub(super) enum CardFile {
    Todo {
        layout: Ident,
        faces: Vec<CardFaceTodo>,
    },
}

/// One face of a todo. Every field a skip attr can omit carries
/// `#[serde(default)]` so the same shape reads back — the skip attrs stay
/// load-bearing (`implicit_some` cannot omit `None` fields by itself).
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct CardFaceTodo {
    pub(super) name: String,
    #[serde(
        default,
        skip_serializing_if = "ManaCost::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) mana_cost: ManaCost,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) color_indicator: Vec<Color>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) supertypes: Vec<Ident>,
    #[serde(serialize_with = "one_line_if_single")]
    pub(super) types: Vec<Ident>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) subtypes: Vec<Ident>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    pub(super) text: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) power: Option<Stat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) toughness: Option<Stat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) loyalty: Option<Stat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) defense: Option<Stat>,
}

#[cfg(test)]
mod tests {
    use deckmaste_core::{Color, ManaSymbol};

    use super::*;
    use crate::ron_output::{ron_options, to_string_pretty};

    #[test]
    fn mana_symbols_serialize_flat() {
        // The Simple and Color wrapper variants are untagged: the nested
        // model must not show up in the RON.
        let symbols: ManaCost = "{R}{2}{C}{2/W}{W/P}{G/U/P}{X}{S}".parse().unwrap();
        assert_eq!(
            ron_options().to_string(&symbols).unwrap(),
            "[Red,Generic(2),Colorless,Hybrid(Generic(2),White),Phyrexian(White,None),\
             Phyrexian(Green,Blue),Variable,Snow]"
        );
    }

    #[test]
    fn stats() {
        let render = |value| ron_options().to_string(&stat(value)).unwrap();
        assert_eq!(render("2"), "2");
        assert_eq!(render("-1"), "-1");
        assert_eq!(render("*"), "Other(\"*\")");
        assert_eq!(render("1+*"), "Other(\"1+*\")");
        assert_eq!(render("X"), "Other(\"X\")");
    }

    /// A vanilla face has no mana cost and no text, like a basic land:
    /// optional fields must be skipped, not handed to the serializers.
    fn test_face(name: &str, vanilla: bool) -> CardFaceTodo {
        CardFaceTodo {
            name: name.to_owned(),
            mana_cost: if vanilla {
                ManaCost::default()
            } else {
                vec![
                    ManaSymbol::Hybrid(2.into(), Color::White),
                    ManaSymbol::Simple(Color::Green.into()),
                ]
                .into()
            },
            color_indicator: vec![],
            supertypes: vec![],
            types: vec!["Creature".into()],
            subtypes: vec!["Time Lord".into()],
            text: if vanilla {
                vec![]
            } else {
                vec![
                    "Flying".to_owned(),
                    "Doctor's \"companion\" rule.".to_owned(),
                ]
            },
            power: Some(stat("2")),
            toughness: Some(stat("*")),
            loyalty: None,
            defense: None,
        }
    }

    #[test]
    fn single_face_serialization() {
        let card = CardFile::Todo {
            layout: "normal".into(),
            faces: vec![test_face("Solo", false)],
        };
        let serialized = to_string_pretty(&card).unwrap();
        assert_eq!(
            serialized,
            r##"Todo(
    layout: "normal",
    faces: [
        (
            name: "Solo",
            mana_cost: [
                Hybrid(Generic(2), White),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Time Lord"],
            text: [
                "Flying",
                r#"Doctor's "companion" rule."#,
            ],
            power: 2,
            toughness: Other("*"),
        ),
    ],
)"##
        );
    }

    #[test]
    fn multi_face_serialization() {
        let card = CardFile::Todo {
            layout: "transform".into(),
            faces: vec![test_face("Front", false), test_face("Back", true)],
        };
        let serialized = to_string_pretty(&card).unwrap();
        // Each face is its own list element; Hybrid mana symbols stay
        // inline, and raw string bodies stay unindented.
        assert_eq!(
            serialized,
            r##"Todo(
    layout: "transform",
    faces: [
        (
            name: "Front",
            mana_cost: [
                Hybrid(Generic(2), White),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Time Lord"],
            text: [
                "Flying",
                r#"Doctor's "companion" rule."#,
            ],
            power: 2,
            toughness: Other("*"),
        ),
        (
            name: "Back",
            types: ["Creature"],
            subtypes: ["Time Lord"],
            power: 2,
            toughness: Other("*"),
        ),
    ],
)"##
        );
    }

    /// Reading the exact shape `_004` writes (real Snow-Covered Plains
    /// output): defaults fill the omitted fields.
    #[test]
    fn reads_004_output() {
        let source = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Snow-Covered Plains",
            supertypes: [
                "Basic",
                "Snow",
            ],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
    ],
)
"#;
        let CardFile::Todo { layout, faces } = ron_options().from_str(source).unwrap();
        assert_eq!(layout, "normal");
        assert_eq!(faces.len(), 1);
        assert_eq!(faces[0].name, "Snow-Covered Plains");
        assert_eq!(faces[0].supertypes, ["Basic", "Snow"]);
        assert_eq!(faces[0].subtypes, ["Plains"]);
        assert!(faces[0].mana_cost.is_empty());
        assert!(faces[0].text.is_empty());
        assert_eq!(faces[0].power, None);
    }

    /// Serialize -> deserialize -> serialize is a fixed point, stats and
    /// mana symbols included.
    #[test]
    fn todo_round_trip() {
        let card = CardFile::Todo {
            layout: "transform".into(),
            faces: vec![test_face("Front", false), test_face("Back", true)],
        };
        let serialized = to_string_pretty(&card).unwrap();
        let parsed: CardFile = ron_options().from_str(&serialized).unwrap();
        assert_eq!(to_string_pretty(&parsed).unwrap(), serialized);
    }
}
