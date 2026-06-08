use std::sync::LazyLock;

use anyhow::Context;
use deckmaste_core::Color;
use deckmaste_core::plugin::{card_file, card_todo_file};
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

/// Replaces every whole-word, case-sensitive occurrence of `name` with `~`.
/// The substring pre-check skips building a regex for the common case of a
/// card that never mentions the name at all.
fn replace_whole_word(text: &str, name: &str) -> String {
    if name.is_empty() || !text.contains(name) {
        return text.to_owned();
    }
    let pattern = format!(r"\b{}\b", regex::escape(name));
    // Always valid: an escaped literal wrapped in word boundaries.
    let re = Regex::new(&pattern).expect("escaped name is a valid regex");
    re.replace_all(text, "~").into_owned()
}

/// Replaces a card's references to *itself* with the `~` self-reference sigil.
///
/// Replaces every whole-word, case-sensitive occurrence of `face_name`. For a
/// legendary face whose name has a comma, the pre-comma short name (e.g.
/// "Boromir" from "Boromir, Gondor's Hope") is also collapsed -- unless it is
/// a keyword ability ("Storm") or shorter than three characters ("Me"), both
/// too collision-prone to replace blindly.
fn self_ref_to_tilde(
    text: &str,
    face_name: &str,
    is_legendary: bool,
    keyword_abilities: &[DataStr<'_>],
) -> String {
    let out = replace_whole_word(text, face_name);

    let short = is_legendary
        .then(|| face_name.split_once(',').map(|(short, _)| short.trim()))
        .flatten()
        .filter(|short| short.chars().count() >= 3)
        .filter(|short| {
            !keyword_abilities
                .iter()
                .any(|kw| kw.as_str().eq_ignore_ascii_case(short))
        });

    match short {
        Some(short) => replace_whole_word(&out, short),
        None => out,
    }
}

fn ron_color(code: &str) -> anyhow::Result<Color> {
    Color::from_code(code).ok_or_else(|| anyhow::anyhow!("unrecognized color indicator: {code:?}"))
}

fn render_face(
    card: &AtomicCard,
    keyword_abilities: &[DataStr<'_>],
) -> anyhow::Result<CardFaceTodo> {
    let face_name = card.face_name.as_deref().unwrap_or(&card.name);
    let is_legendary = card.supertypes.iter().any(|t| t.as_str() == "Legendary");
    Ok(CardFaceTodo {
        name: face_name.to_owned(),
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
            let text = self_ref_to_tilde(&text, face_name, is_legendary, keyword_abilities);
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

            // Skip once the finished card exists (converted by a later
            // migration or hand-written); otherwise write the stub beside it.
            if !super::is_unimplemented(&dest_dir.join(card_file(name))) {
                continue;
            }

            let card = CardFile::Todo {
                layout: supported[0].layout.as_str().into(),
                faces: supported
                    .iter()
                    .map(|face| render_face(face, &keyword_abilities))
                    .collect::<anyhow::Result<_>>()?,
            };
            let serialized =
                to_string_pretty(&card).with_context(|| format!("serializing card {name:?}"))?;

            let dest = dest_dir.join(card_todo_file(name));
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

    #[test]
    fn self_reference() {
        let no_kw: &[DataStr] = &[];

        // Full name -> ~, every occurrence, case-sensitive whole words.
        assert_eq!(
            self_ref_to_tilde(
                "Storm of Steel deals 2 damage.",
                "Storm of Steel",
                false,
                no_kw
            ),
            "~ deals 2 damage."
        );
        assert_eq!(
            self_ref_to_tilde(
                "Exile Long Rest. Return Long Rest.",
                "Long Rest",
                false,
                no_kw
            ),
            "Exile ~. Return ~."
        );
        // Possessive: the apostrophe is a word boundary, so the name still matches.
        assert_eq!(
            self_ref_to_tilde(
                "Norman Osborn's controller draws.",
                "Norman Osborn",
                false,
                no_kw
            ),
            "~'s controller draws."
        );
        // "named <self>" collapses to "named ~".
        assert_eq!(
            self_ref_to_tilde(
                "A deck can have any number of cards named Rat Colony.",
                "Rat Colony",
                false,
                no_kw
            ),
            "A deck can have any number of cards named ~."
        );
        // Only the card's own name is touched, not another card's.
        assert_eq!(
            self_ref_to_tilde(
                "Norman Osborn can't be blocked.",
                "Green Goblin",
                false,
                no_kw
            ),
            "Norman Osborn can't be blocked."
        );

        // Legendary: pre-comma short name also collapses.
        assert_eq!(
            self_ref_to_tilde(
                "Boromir can't be blocked.",
                "Boromir, Gondor's Hope",
                true,
                no_kw
            ),
            "~ can't be blocked."
        );
        // Short name matches whole words only -- "Gut" leaves "guts" alone.
        assert_eq!(
            self_ref_to_tilde(
                "Gut attacks. The guts spill.",
                "Gut, True Soul Zealot",
                true,
                no_kw
            ),
            "~ attacks. The guts spill."
        );
        // Short-name pass is legendary-only: a comma'd non-legend keeps its prefix.
        assert_eq!(
            self_ref_to_tilde(
                "Borrowing 100 arrows.",
                "Borrowing 100,000 Arrows",
                false,
                no_kw
            ),
            "Borrowing 100 arrows."
        );
        // Guard: short name shorter than three chars is too risky -- skip it.
        assert_eq!(
            self_ref_to_tilde("Me draws a card.", "Me, the Immortal", true, no_kw),
            "Me draws a card."
        );
        // Guard: short name that is a keyword ability is skipped.
        let storm: &[DataStr] = &["Storm".into()];
        assert_eq!(
            self_ref_to_tilde("Storm gets +1/+1.", "Storm, the Tempest", true, storm),
            "Storm gets +1/+1."
        );
        // ...but a distinctive short name still collapses even when other text
        // happens to contain a keyword.
        assert_eq!(
            self_ref_to_tilde(
                "Ral deals 1 damage to any target.",
                "Ral, Storm Conduit",
                true,
                storm
            ),
            "~ deals 1 damage to any target."
        );

        // No self-reference: text is returned unchanged.
        assert_eq!(
            self_ref_to_tilde("Draw a card.", "Some Other Card", false, no_kw),
            "Draw a card."
        );
    }
}
