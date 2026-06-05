use std::fmt::Write as _;
use std::sync::LazyLock;

use mtgjson::{AtomicCard, Color, Layout};
use regex::Regex;

pub(super) struct CardTodos;

/// Categorized subtype lists, in the lookup order of the jq version (the
/// first category containing a subtype wins).
struct SubtypeCategories([(&'static str, Vec<String>); 7]);

impl SubtypeCategories {
    fn load() -> anyhow::Result<Self> {
        Ok(Self([
            ("Creature", crate::data::creature_types()?),
            ("Artifact", crate::data::artifact_types()?),
            ("Enchantment", crate::data::enchantment_types()?),
            ("Land", crate::data::land_types()?),
            ("Battle", crate::data::battle_types()?),
            ("Planeswalker", crate::data::planeswalker_types()?),
            ("Spell", crate::data::spell_types()?),
        ]))
    }

    fn subtype_ron(&self, subtype: &str) -> anyhow::Result<String> {
        self.0
            .iter()
            .find(|(_, subtypes)| subtypes.iter().any(|s| s == subtype))
            .map(|(category, _)| format!("{category}({subtype})"))
            .ok_or_else(|| anyhow::anyhow!("subtype {subtype:?} not in any type catalog"))
    }
}

// We count non-null, non-"Banned" as legal.
fn is_supported(card: &AtomicCard) -> bool {
    card.legalities.vintage.as_deref().unwrap_or("Banned") != "Banned"
        && card.layout != Layout::ReversibleCard
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
/// of the surrounding spaces.
fn strip_reminder_text(text: &str) -> String {
    static PARENTHETICAL: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r" ?\([^)\n]+\)( ?)").unwrap());

    PARENTHETICAL.replace_all(text, "$1").into_owned()
}

/// Splits lines that are comma-separated lists of keyword abilities into one
/// keyword per line, e.g. "Flying, vigilance" -> "Flying\nVigilance".
fn expand_keyword_lines(text: &str, keyword_abilities: &[String]) -> String {
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

fn ron_color(color: Color) -> &'static str {
    match color {
        Color::White => "White",
        Color::Blue => "Blue",
        Color::Black => "Black",
        Color::Red => "Red",
        Color::Green => "Green",
    }
}

fn ron_color_code(code: &str) -> &'static str {
    match code {
        "W" => "White",
        "U" => "Blue",
        "B" => "Black",
        "R" => "Red",
        "G" => "Green",
        "C" => "Colorless",
        _ => unreachable!("colors are restricted by the symbol regex"),
    }
}

fn mana_symbol_ron(symbol: &str) -> anyhow::Result<String> {
    static SYMBOL: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"(?x)^\{(?:
                (?P<variable>X)
                |(?P<snow>S)
                |(?:
                    (?:
                        (?P<generic>[0-9]|[1-9][0-9]+)
                        |(?P<color>[WUBRGC])
                    )
                    (?:/(?P<hybrid>[WUBRG]))?
                    (?:/(?P<phyrexian>P))?
                )
            )\}$",
        )
        .unwrap()
    });

    let captures = SYMBOL
        .captures(symbol)
        .ok_or_else(|| anyhow::anyhow!("unrecognized mana symbol: {symbol:?}"))?;

    let mut ron = if captures.name("variable").is_some() {
        "Variable".to_owned()
    } else if captures.name("snow").is_some() {
        "Snow".to_owned()
    } else if let Some(generic) = captures.name("generic") {
        format!("Generic({})", generic.as_str())
    } else {
        ron_color_code(&captures["color"]).to_owned()
    };
    if let Some(hybrid) = captures.name("hybrid") {
        ron = format!("Hybrid({ron}, {})", ron_color_code(hybrid.as_str()));
    }
    if captures.name("phyrexian").is_some() {
        ron = format!("Phyrexian({ron})");
    }
    Ok(ron)
}

fn mana_cost_ron(mana_cost: &str) -> anyhow::Result<String> {
    static SYMBOLS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{[^}]+\}").unwrap());

    let symbols = SYMBOLS
        .find_iter(mana_cost)
        .map(|symbol| mana_symbol_ron(symbol.as_str()))
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(format!("[{}]", symbols.join(", ")))
}

/// `Number(2)` for values that parse as JSON numbers, `NonNumber("*")`
/// otherwise.
fn number_ron(value: &str) -> String {
    match serde_json::from_str::<serde_json::Number>(value) {
        Ok(number) => format!("Number({number})"),
        Err(_) => format!("NonNumber(\"{value}\")"),
    }
}

/// Indents every line by `indent`, leaving the bodies of multi-line raw
/// strings (r#" ... "#) and empty lines untouched.
fn indent_ron(text: &str, indent: &str) -> String {
    let mut in_raw = false;
    let mut lines = Vec::new();
    for line in text.split('\n') {
        if in_raw || line.is_empty() {
            lines.push(line.to_owned());
        } else {
            lines.push(format!("{indent}{line}"));
        }
        if !in_raw && line.ends_with("r#\"") {
            in_raw = true;
        } else if in_raw && line.starts_with("\"#") {
            in_raw = false;
        }
    }
    lines.join("\n")
}

fn render_face(
    card: &AtomicCard,
    keyword_abilities: &[String],
    categories: &SubtypeCategories,
) -> anyhow::Result<String> {
    let mut out = String::from("Todo(\n");

    let name = card.face_name.as_ref().unwrap_or(&card.name);
    writeln!(out, "    name: r#\"{name}\"#,")?;

    if let Some(mana_cost) = &card.mana_cost {
        writeln!(out, "    mana_cost: {},", mana_cost_ron(mana_cost)?)?;
    }

    if let Some(color_indicator) = card.color_indicator.as_deref().filter(|ci| !ci.is_empty()) {
        let colors: Vec<&str> = color_indicator.iter().map(|&c| ron_color(c)).collect();
        writeln!(out, "    color_indicator: [{}],", colors.join(", "))?;
    }

    writeln!(out, "    types: [{}],", card.card_types.join(", "))?;

    if !card.supertypes.is_empty() {
        writeln!(out, "    supertypes: [{}],", card.supertypes.join(", "))?;
    }

    if !card.subtypes.is_empty() {
        let subtypes = card
            .subtypes
            .iter()
            .map(|subtype| categories.subtype_ron(subtype))
            .collect::<anyhow::Result<Vec<_>>>()?;
        writeln!(out, "    subtypes: [{}],", subtypes.join(", "))?;
    }

    // NOTE: The jq version rendered cards without rules text with the
    // literal text "null"; we omit the field instead.
    if let Some(text) = &card.text {
        let text = expand_keyword_lines(&strip_reminder_text(text), keyword_abilities);
        writeln!(out, "    text: r#\"\n{text}\n\"#,")?;
    }

    let numbers = [
        ("power", &card.power),
        ("toughness", &card.toughness),
        ("loyalty", &card.loyalty),
        ("defense", &card.defense),
    ];
    for (field, value) in numbers {
        if let Some(value) = value {
            writeln!(out, "    {field}: {},", number_ron(value))?;
        }
    }

    out.push(')');
    Ok(out)
}

impl super::Migration for CardTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let atomic_cards = crate::data::atomic_cards()?;
        let keyword_abilities = crate::data::keywords()?.keyword_abilities;
        let categories = SubtypeCategories::load()?;
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

            let rendered = supported
                .iter()
                .map(|face| render_face(face, &keyword_abilities, &categories))
                .collect::<anyhow::Result<Vec<_>>>()
                .map_err(|e| e.context(format!("rendering card {name:?}")))?;
            let contents = match <[String; 1]>::try_from(rendered) {
                Ok([single]) => single,
                Err(faces) => format!("(\n{},\n)", indent_ron(&faces.join(",\n"), "    ")),
            };

            std::fs::write(&dest, contents + "\n")?;
            eprintln!("wrote {}", dest.display());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
    }

    #[test]
    fn keyword_lines() {
        let keywords = vec![
            "Flying".to_owned(),
            "Vigilance".to_owned(),
            "Equip".to_owned(),
        ];
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
    fn mana_costs() {
        assert_eq!(mana_cost_ron("{1}{G}").unwrap(), "[Generic(1), Green]");
        assert_eq!(mana_cost_ron("{X}{S}").unwrap(), "[Variable, Snow]");
        assert_eq!(
            mana_cost_ron("{2/W}{C/B}").unwrap(),
            "[Hybrid(Generic(2), White), Hybrid(Colorless, Black)]"
        );
        assert_eq!(
            mana_cost_ron("{G/U/P}{W/P}").unwrap(),
            "[Phyrexian(Hybrid(Green, Blue)), Phyrexian(White)]"
        );
        assert!(mana_cost_ron("{HW}").is_err());
    }

    #[test]
    fn numbers() {
        assert_eq!(number_ron("2"), "Number(2)");
        assert_eq!(number_ron("-1"), "Number(-1)");
        assert_eq!(number_ron("*"), "NonNumber(\"*\")");
        assert_eq!(number_ron("1+*"), "NonNumber(\"1+*\")");
        assert_eq!(number_ron("X"), "NonNumber(\"X\")");
    }

    #[test]
    fn raw_string_aware_indent() {
        let text = "Todo(\n    text: r#\"\nNot indented\n\n\"#,\n)";
        assert_eq!(
            indent_ron(text, "    "),
            "    Todo(\n        text: r#\"\nNot indented\n\n\"#,\n    )"
        );
    }
}
