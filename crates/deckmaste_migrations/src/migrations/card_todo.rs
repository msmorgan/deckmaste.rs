//! The card todo file shape: written by `_004`, read back by every later
//! migration that turns todos into real definitions. Plain ron/serde on
//! both sides — todo files quote everything, so no macro awareness needed.

use anyhow::Context;
use deckmaste_core::{Color, Ident, ManaCost};
use serde::{Deserialize, Serialize};

use crate::layout::PluginLayout;
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

/// Walks the plugin's cards directory and graduates every `<name>.todo.ron`
/// for which `convert` produces a finished definition: the definition is
/// written to `<name>.ron` and the stub is deleted. Stubs `convert` declines
/// are left in place. cards/ is flat: every stub `_004` writes goes through
/// `card_todo_file`, one path segment per card, so no recursion here.
pub(super) fn convert_todos(
    plugin: &PluginLayout,
    convert: impl Fn(&CardFile) -> anyhow::Result<Option<String>>,
) -> anyhow::Result<()> {
    let cards_dir = plugin.cards_dir()?;
    let mut paths: Vec<_> = std::fs::read_dir(&cards_dir)?
        .map(|entry| entry.map(|e| e.path()))
        .collect::<Result<_, _>>()?;
    paths.sort();

    for path in paths {
        if !path.is_file() || !deckmaste_core::plugin::is_todo_file(&path) {
            continue;
        }
        let source = std::fs::read_to_string(&path)?;
        let card: CardFile = crate::ron_output::ron_options()
            .from_str(&source)
            .with_context(|| format!("parsing todo {}", path.display()))?;
        let Some(definition) = convert(&card)? else {
            continue;
        };

        // Cheap guard: the output must still be valid RON. Bare idents
        // like `Plains` are deliberately unresolved here -- only the
        // macro-aware reader (`cargo xtask validate`) can judge them.
        ron::value::RawValue::from_ron(&definition)
            .with_context(|| format!("invalid render for {}", path.display()))?;

        // The `is_todo_file` filter guarantees a `.todo.ron` name, so the
        // graduated `.ron` name is always present.
        let final_path = path.with_file_name(
            path.file_name()
                .and_then(|name| name.to_str())
                .and_then(deckmaste_core::plugin::final_for_todo)
                .with_context(|| format!("not a todo file name: {}", path.display()))?,
        );
        std::fs::write(&final_path, definition)?;
        std::fs::remove_file(&path)?;
        eprintln!(
            "wrote {} (removed {})",
            final_path.display(),
            path.display()
        );
    }
    Ok(())
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

    /// A stub `convert` accepts graduates to `<name>.ron` and the
    /// `<name>.todo.ron` is deleted; a declined stub and a finished card
    /// already in place are both left untouched.
    #[test]
    fn convert_graduates_and_deletes_stub() {
        const ACCEPT_FINAL: &str = "Normal(\n    name: \"Accept\",\n    types: [Land],\n)\n";
        const DONE_FINAL: &str = "Normal(\n    name: \"Done\",\n    types: [Land],\n)\n";
        let stub = |name: &str| {
            format!(
                "Todo(\n    layout: \"normal\",\n    faces: [\n        \
                 (\n            name: {name:?},\n            types: [\"Land\"],\n        ),\n    ],\n)\n"
            )
        };

        let root = tempfile::tempdir().unwrap();
        let plugin = PluginLayout::new(root.path()).unwrap();
        let cards = plugin.cards_dir().unwrap();
        std::fs::write(cards.join("Accept.todo.ron"), stub("Accept")).unwrap();
        std::fs::write(cards.join("Decline.todo.ron"), stub("Decline")).unwrap();
        std::fs::write(cards.join("Done.ron"), DONE_FINAL).unwrap();

        convert_todos(&plugin, |card| {
            let CardFile::Todo { faces, .. } = card;
            Ok((faces[0].name == "Accept").then(|| ACCEPT_FINAL.to_owned()))
        })
        .unwrap();

        // Accepted: stub deleted, final written.
        assert!(!cards.join("Accept.todo.ron").exists());
        assert_eq!(
            std::fs::read_to_string(cards.join("Accept.ron")).unwrap(),
            ACCEPT_FINAL
        );
        // Declined: stub kept, no final produced.
        assert!(cards.join("Decline.todo.ron").exists());
        assert!(!cards.join("Decline.ron").exists());
        // A finished card is never a stub, so it is left as-is.
        assert_eq!(
            std::fs::read_to_string(cards.join("Done.ron")).unwrap(),
            DONE_FINAL
        );
    }
}
