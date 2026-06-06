use std::sync::LazyLock;

use anyhow::Context;
use deckmaste_core::Color;
use deckmaste_core::plugin::card_file;
use regex::Regex;

use super::card_todo::{CardFaceTodo, CardFile, stat};
use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;
use crate::ron_output::to_string_pretty;

// We count non-null, non-"Banned" as legal.
fn is_supported(card: &AtomicCard) -> bool {
    card.legalities.vintage.as_deref().unwrap_or("Banned") != "Banned"
        && card.layout.as_str() != "reversible_card"
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

/// `capitalize(item).starts_with(keyword)` without building the string.
fn starts_with_capitalized(item: &str, keyword: &str) -> bool {
    let mut item_chars = item.chars();
    let mut keyword_chars = keyword.chars();
    match (item_chars.next(), keyword_chars.next()) {
        (_, None) => true,
        (None, Some(_)) => false,
        (Some(i), Some(k)) => {
            i.to_ascii_uppercase() == k && item_chars.as_str().starts_with(keyword_chars.as_str())
        }
    }
}

/// Splits lines that are comma-separated lists of keyword abilities into one
/// keyword per line, e.g. "Flying, vigilance" -> "Flying\nVigilance".
/// Most lines aren't keyword lists, so nothing is allocated until one is.
fn expand_keyword_lines(text: &str, keyword_abilities: &[DataStr<'_>]) -> String {
    let is_keyword = |item: &str| {
        keyword_abilities
            .iter()
            .any(|keyword| starts_with_capitalized(item, keyword))
    };
    text.split('\n')
        .flat_map(|line| {
            if line.split(", ").all(is_keyword) {
                line.split(", ").map(capitalize).collect()
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

            let dest = dest_dir.join(card_file(name));
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
    use super::*;

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
}
