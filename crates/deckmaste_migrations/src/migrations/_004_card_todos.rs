use std::sync::LazyLock;

use deckmaste_core::{Color, ManaCost};
use regex::Regex;
use serde::Serialize;

use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;

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
        layout: String,
        faces: Vec<CardFace>,
    },
}

#[derive(Serialize)]
struct CardFace {
    name: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "one_line_if_single_opt"
    )]
    mana_cost: Option<ManaCost>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "one_line_if_single_opt"
    )]
    color_indicator: Option<Vec<Color>>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    supertypes: Vec<String>,
    #[serde(serialize_with = "one_line_if_single")]
    types: Vec<String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "one_line_if_single"
    )]
    subtypes: Vec<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "one_line_if_single_opt"
    )]
    text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<Stat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    toughness: Option<Stat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    loyalty: Option<Stat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    defense: Option<Stat>,
}

/// Serializes a single-element array on one line (`[Red]`); longer arrays
/// fall through to the chopped pretty-printer. ron's config cannot express
/// this, so the compact form is pre-rendered and embedded as a RawValue.
fn one_line_if_single<T: Serialize, S: serde::Serializer>(
    array: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    use serde::ser::Error as _;

    if array.len() != 1 {
        return array.serialize(serializer);
    }
    let compact = ron::Options::default()
        .to_string_pretty(
            &array,
            ron::ser::PrettyConfig::default()
                .escape_strings(false)
                .depth_limit(0),
        )
        .map_err(S::Error::custom)?;
    ron::value::RawValue::from_ron(&compact)
        .map_err(S::Error::custom)?
        .serialize(serializer)
}

fn one_line_if_single_opt<T: Serialize, A, S: serde::Serializer>(
    array: &Option<A>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    A: std::ops::Deref<Target = [T]>,
{
    let array = array.as_deref().expect("field is skipped when None");
    one_line_if_single(array, serializer)
}

fn ron_options() -> ron::Options {
    // Implicit Some keeps the optional fields unwrapped, and is a default
    // extension so no #![enable(...)] header is emitted.
    ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
}

/// Multi-line text is written verbatim and arrays are chopped, one element
/// per line. Tuples like `Hybrid(...)` mana symbols keep their members
/// inline (the default), and the faces list chops one face per line like
/// any other array.
fn pretty_config() -> ron::ser::PrettyConfig {
    ron::ser::PrettyConfig::default()
        .extensions(ron::extensions::Extensions::IMPLICIT_SOME)
        .escape_strings(false)
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

/// Uppercases the first character (ASCII only, like jq's ascii_upcase).
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

fn render_face(card: &AtomicCard, keyword_abilities: &[DataStr<'_>]) -> anyhow::Result<CardFace> {
    Ok(CardFace {
        name: card.face_name.as_deref().unwrap_or(&card.name).to_owned(),
        mana_cost: card.mana_cost.as_deref().map(str::parse).transpose()?,
        color_indicator: card
            .color_indicator
            .as_deref()
            .filter(|colors| !colors.is_empty())
            .map(|colors| colors.iter().map(|code| ron_color(code)).collect())
            .transpose()?,
        supertypes: card
            .supertypes
            .iter()
            .map(|t| t.as_str().to_owned())
            .collect(),
        types: card.types.iter().map(|t| t.as_str().to_owned()).collect(),
        subtypes: card
            .subtypes
            .iter()
            .map(|t| t.as_str().to_owned())
            .collect(),
        // One element per line of normalized oracle text -- one ability
        // each, except that modal/leveler lines stay split. Cards whose text
        // is nothing but reminder text (basic lands) end up with no text.
        text: card.text.as_deref().and_then(|text| {
            let text = crate::data::academyruins::normalize_quotes(text);
            let text = expand_keyword_lines(&strip_reminder_text(&text), keyword_abilities);
            let lines: Vec<String> = text
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect();
            (!lines.is_empty()).then_some(lines)
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
        let options = ron_options();

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
                layout: supported[0].layout.as_str().to_owned(),
                faces: supported
                    .iter()
                    .map(|face| render_face(face, &keyword_abilities))
                    .collect::<anyhow::Result<_>>()?,
            };
            let serialized = options
                .to_string_pretty(&card_file, pretty_config())
                .map_err(|e| anyhow::anyhow!(e))
                .map_err(|e| e.context(format!("serializing card {name:?}")))?;

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

    fn test_face(name: &str, vanilla: bool) -> CardFace {
        CardFace {
            name: name.to_owned(),
            mana_cost: Some(
                vec![
                    ManaSymbol::Hybrid(2.into(), Color::White),
                    ManaSymbol::Simple(Color::Green.into()),
                ]
                .into(),
            ),
            color_indicator: None,
            supertypes: vec![],
            types: vec!["Creature".to_owned()],
            subtypes: vec!["Time Lord".to_owned()],
            text: (!vanilla).then(|| {
                vec![
                    "Flying".to_owned(),
                    "Doctor's \"companion\" rule.".to_owned(),
                ]
            }),
            power: Some(stat("2")),
            toughness: Some(stat("*")),
            loyalty: None,
            defense: None,
        }
    }

    #[test]
    fn single_face_serialization() {
        let card = CardFile::Todo {
            layout: "normal".to_owned(),
            faces: vec![test_face("Solo", false)],
        };
        let serialized = ron_options()
            .to_string_pretty(&card, pretty_config())
            .unwrap();
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
            layout: "transform".to_owned(),
            faces: vec![test_face("Front", false), test_face("Back", true)],
        };
        let serialized = ron_options()
            .to_string_pretty(&card, pretty_config())
            .unwrap();
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
            mana_cost: [
                Hybrid(Generic(2), White),
                Green,
            ],
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
