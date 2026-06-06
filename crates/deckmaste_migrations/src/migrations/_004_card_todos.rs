use std::sync::LazyLock;

use anyhow::Context;
use deckmaste_core::{Card, Color, Ident, ManaCost};
use regex::Regex;
use serde::Serialize;

use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;
use crate::ron_output::{one_line_if_single, to_string_pretty};

/// Numbers serialize untagged (`power: 2`); anything else keeps its tag
/// (`power: Other("*")`). Untagged variants must come last in the enum.
#[derive(Debug, PartialEq, Serialize)]
enum Stat {
    Other(String),
    #[serde(untagged)]
    Number(serde_json::Number),
}

/// A card file is always `Todo(layout: ..., faces: [...])`, with the
/// MTGJSON layout name verbatim and one anonymous struct per face.
#[derive(Serialize)]
enum CardFile {
    Todo {
        layout: Ident,
        faces: Vec<CardFaceTodo>,
    },

    #[serde(untagged)]
    Card(Box<Card>),
}

#[derive(Serialize)]
struct CardFaceTodo {
    name: String,
    #[serde(
        skip_serializing_if = "ManaCost::is_empty",
        serialize_with = "one_line_if_single"
    )]
    mana_cost: ManaCost,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    color_indicator: Vec<Color>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    supertypes: Vec<Ident>,
    #[serde(serialize_with = "one_line_if_single")]
    types: Vec<Ident>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    subtypes: Vec<Ident>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    text: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<Stat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toughness: Option<Stat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loyalty: Option<Stat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defense: Option<Stat>,
}

// We count non-null, non-"Banned" as legal.
fn is_supported(card: &AtomicCard) -> bool {
    card.legalities.vintage.as_deref().unwrap_or("Banned") != "Banned"
        && card.layout.as_str() != "reversible_card"
}

/// Maps windows-unsafe filename characters to their fullwidth equivalents,
/// e.g. "Fire // Ice" -> "Fire ／／ Ice".
fn to_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if "<>:\"/\\|?*".contains(c) {
                char::from_u32(c as u32 + 0xFEE0).expect("fullwidth ASCII is valid")
            } else {
                c
            }
        })
        .collect()
}

/// Uppercases the first character (ASCII only, like jq's `ascii_upcase`).
fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// Removes single-line parentheticals (reminder text), keeping at most one
/// of the surrounding spaces. Lines that consisted solely of reminder text
/// are dropped entirely.
fn strip_reminder_text(text: &str) -> String {
    static PARENTHETICAL: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r" ?\([^)\n]+\)( ?)").unwrap());

    text.split('\n')
        .filter_map(|line| {
            let stripped = PARENTHETICAL.replace_all(line, "$1");
            (!stripped.is_empty() || line.is_empty()).then(|| stripped.into_owned())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Splits lines that are comma-separated lists of keyword abilities into one
/// keyword per line, e.g. "Flying, vigilance" -> "Flying\nVigilance".
fn expand_keyword_lines(text: &str, keyword_abilities: &[DataStr<'_>]) -> String {
    text.split('\n')
        .flat_map(|line| {
            let items: Vec<String> = line.split(", ").map(capitalize).collect();
            if items.iter().all(|item| {
                keyword_abilities
                    .iter()
                    .any(|keyword| item.starts_with(keyword.as_str()))
            }) {
                items
            } else {
                vec![line.to_owned()]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn ron_color(code: &str) -> anyhow::Result<Color> {
    Color::from_code(code).ok_or_else(|| anyhow::anyhow!("unrecognized color indicator: {code:?}"))
}

/// `Number(2)` for values that parse as JSON numbers, `Other("*")`
/// otherwise.
fn stat(value: &str) -> Stat {
    match serde_json::from_str(value) {
        Ok(number) => Stat::Number(number),
        Err(_) => Stat::Other(value.to_owned()),
    }
}

fn render_face(
    card: &AtomicCard,
    keyword_abilities: &[DataStr<'_>],
) -> anyhow::Result<CardFaceTodo> {
    Ok(CardFaceTodo {
        name: card.face_name.as_deref().unwrap_or(&card.name).to_owned(),
        mana_cost: card
            .mana_cost
            .as_deref()
            .map(str::parse)
            .transpose()?
            .unwrap_or_default(),
        color_indicator: card
            .color_indicator
            .iter()
            .map(|code| ron_color(code))
            .collect::<anyhow::Result<_>>()?,
        supertypes: card.supertypes.iter().map(|t| t.as_str().into()).collect(),
        types: card.types.iter().map(|t| t.as_str().into()).collect(),
        subtypes: card.subtypes.iter().map(|t| t.as_str().into()).collect(),
        // One element per line of normalized oracle text -- one ability
        // each, except that modal/leveler lines stay split. Cards whose text
        // is nothing but reminder text (basic lands) end up with no text.
        text: card.text.as_deref().map_or_else(Vec::new, |text| {
            let text = crate::data::academyruins::normalize_quotes(text);
            let text = expand_keyword_lines(&strip_reminder_text(&text), keyword_abilities);
            text.split('\n')
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect()
        }),
        power: card.power.as_deref().map(stat),
        toughness: card.toughness.as_deref().map(stat),
        loyalty: card.loyalty.as_deref().map(stat),
        defense: card.defense.as_deref().map(stat),
    })
}

pub(super) struct CardTodos;

impl super::Migration for CardTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let atomic_bytes = crate::data::mtgjson::atomic_cards_bytes()?;
        let atomic_cards = crate::data::mtgjson::AtomicCards::parse(&atomic_bytes)?;
        let keywords_bytes = crate::data::academyruins::keywords_bytes()?;
        let keyword_abilities =
            crate::data::academyruins::Keywords::parse(&keywords_bytes)?.keyword_abilities;
        let dest_dir = plugin.cards_dir()?;

        for (name, faces) in &atomic_cards.data {
            let supported: Vec<&AtomicCard> = faces.iter().filter(|c| is_supported(c)).collect();
            if supported.is_empty() {
                continue;
            }

            let dest = dest_dir.join(format!("{}.ron", to_filename(name)));
            if !super::is_todo(&dest)? {
                continue;
            }

            let card_file = CardFile::Todo {
                layout: supported[0].layout.as_str().into(),
                faces: supported
                    .iter()
                    .map(|face| render_face(face, &keyword_abilities))
                    .collect::<anyhow::Result<_>>()?,
            };
            let serialized = to_string_pretty(&card_file)
                .with_context(|| format!("serializing card {name:?}"))?;

            std::fs::write(&dest, serialized + "\n")?;
            eprintln!("wrote {}", dest.display());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::ManaSymbol;

    use super::*;
    use crate::ron_output::ron_options;

    #[test]
    fn filenames() {
        assert_eq!(to_filename("Fire // Ice"), "Fire ／／ Ice");
        assert_eq!(to_filename("Question?"), "Question？");
        assert_eq!(to_filename("Lightning Bolt"), "Lightning Bolt");
    }

    #[test]
    fn reminder_text() {
        assert_eq!(
            strip_reminder_text("Flying (This creature can't be blocked except by...)"),
            "Flying"
        );
        // Matches the jq behavior: the captured trailing space survives
        // when the parenthetical starts the line.
        assert_eq!(strip_reminder_text("(Reminder) Foo"), " Foo");
        assert_eq!(strip_reminder_text("A (b) c"), "A c");
        // Lines that are nothing but reminder text disappear entirely.
        assert_eq!(
            strip_reminder_text("({R/P} can be paid with {R} or 2 life.)\nGain control."),
            "Gain control."
        );
    }

    #[test]
    fn keyword_lines() {
        let keywords: Vec<DataStr> = vec!["Flying".into(), "Vigilance".into(), "Equip".into()];
        assert_eq!(
            expand_keyword_lines("flying, vigilance", &keywords),
            "Flying\nVigilance"
        );
        assert_eq!(expand_keyword_lines("Equip {2}", &keywords), "Equip {2}");
        assert_eq!(
            expand_keyword_lines("Draw a card, then discard a card.", &keywords),
            "Draw a card, then discard a card."
        );
    }

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
}
