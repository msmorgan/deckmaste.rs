//! `extract` — emit every supported card as a Card-shaped `<name>.ron.todo`
//! (a `TodoCard`) whose abilities are `Unparsed` oracle lines. Reuses `_004`'s
//! individual normalization helpers (`strip_reminder_text`,
//! `expand_keyword_lines`, `self_ref_to_tilde`) and mtgjson field accessors;
//! the pipeline wiring is duplicated here and will consolidate when `_004` is
//! retired. Covers the `normal` and `modal_dfc` layouts (the two core `Card`
//! variants); other layouts are skipped until core `Card` grows variants for
//! them.

use std::path::Path;

use anyhow::Context;
use deckmaste_core::StatValue;
use deckmaste_core::plugin::card_file;

use crate::data::DataStr;
use crate::data::mtgjson::AtomicCard;
use crate::migrations::_004_card_todos::{
    expand_keyword_lines, is_supported, ron_color, self_ref_to_tilde, strip_reminder_text,
};
use crate::migrations::to_rust_ident;
use crate::todo_card::{RawIdent, TodoAbility, TodoCard, TodoCardFace, render};

/// One mtgjson type/subtype/supertype name → a bare-ident `RawIdent`
/// (`"Time Lord"` → `TimeLord`), matching the macro-invocation name the
/// macro-aware reader expands at graduation.
fn ident(name: &str) -> RawIdent { RawIdent(to_rust_ident(name)) }

/// mtgjson stat string → core `StatValue`: integers (incl. negative) are
/// `Number`; `X` is `Variable`; anything else (`*`, `1+*`) is
/// `DefinedByAbility`.
fn stat_value(text: &str) -> StatValue {
    if let Ok(n) = text.parse::<deckmaste_core::Int>() {
        StatValue::Number(n)
    } else if text == "X" {
        // `X` is loyalty defined by the casting cost (the only place `X` stats
        // appear on standard-legal cards); `*`/`1+*` take the DefinedByAbility arm.
        StatValue::Variable
    } else {
        StatValue::DefinedByAbility
    }
}

/// Builds a `TodoCardFace` from one mtgjson face: structured fields plus one
/// `Unparsed` ability per normalized oracle line (same normalization `_004`
/// uses: strip reminder text, split comma-joined keyword lines, `~` self-refs).
///
/// # Errors
/// If the mana cost or a color indicator fails to parse.
fn face(card: &AtomicCard, keyword_abilities: &[DataStr<'_>]) -> anyhow::Result<TodoCardFace> {
    let face_name = card.face_name.as_deref().unwrap_or(card.name.as_str());
    let is_legendary = card.supertypes.iter().any(|t| t.as_str() == "Legendary");
    let abilities = card.text.as_deref().map_or_else(Vec::new, |text| {
        let text = crate::data::academyruins::normalize_quotes(text);
        let text = expand_keyword_lines(&strip_reminder_text(&text), keyword_abilities);
        let text = self_ref_to_tilde(&text, face_name, is_legendary, keyword_abilities);
        text.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| TodoAbility::Unparsed(line.to_owned()))
            .collect()
    });
    Ok(TodoCardFace {
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
            .map(|c| ron_color(c.as_str()))
            .collect::<anyhow::Result<_>>()?,
        supertypes: card.supertypes.iter().map(|t| ident(t.as_str())).collect(),
        types: card.types.iter().map(|t| ident(t.as_str())).collect(),
        subtypes: card.subtypes.iter().map(|t| ident(t.as_str())).collect(),
        abilities,
        power: card.power.as_deref().map(stat_value),
        toughness: card.toughness.as_deref().map(stat_value),
        loyalty: card.loyalty.as_deref().map(stat_value),
        defense: card.defense.as_deref().map(stat_value),
    })
}

/// The `TodoCard` for a card's supported faces, or `None` if its layout isn't a
/// core `Card` variant (only `normal` / `modal_dfc`).
///
/// # Errors
/// If a face fails to build (see [`face`]).
fn todo_card(
    layout: &str,
    faces: &[&AtomicCard],
    keyword_abilities: &[DataStr<'_>],
) -> anyhow::Result<Option<TodoCard>> {
    Ok(match (layout, faces) {
        ("normal", [f]) => Some(TodoCard::Normal(face(f, keyword_abilities)?)),
        ("modal_dfc", [front, back]) => Some(TodoCard::ModalDfc(
            face(front, keyword_abilities)?,
            face(back, keyword_abilities)?,
        )),
        _ => None,
    })
}

/// Writes a `<name>.ron.todo` for every supported card that isn't already
/// finished (`<name>.ron`) or in progress (`<name>.ron.todo`).
///
/// # Errors
/// If the mtgjson/keyword data is unreadable or a card fails to render.
pub fn extract_cards(plugin_dir: &Path) -> anyhow::Result<()> {
    let layout = crate::layout::PluginLayout::new(plugin_dir)?;
    let cards_dir = layout.cards_dir()?;
    let atomic_bytes = crate::data::mtgjson::atomic_cards_bytes()?;
    let atomic = crate::data::mtgjson::AtomicCards::parse(&atomic_bytes)?;
    let keywords_bytes = crate::data::academyruins::keywords_bytes()?;
    let keyword_abilities =
        crate::data::academyruins::Keywords::parse(&keywords_bytes)?.keyword_abilities;

    for (name, all_faces) in &atomic.data {
        let supported: Vec<&AtomicCard> = all_faces.iter().filter(|c| is_supported(c)).collect();
        if supported.is_empty() {
            continue;
        }
        let final_path = cards_dir.join(card_file(name.as_str()));
        let todo_path = cards_dir.join(format!("{}.todo", card_file(name.as_str())));
        if final_path.exists() || todo_path.exists() {
            continue; // already finished or already in progress
        }
        let Some(card) = todo_card(supported[0].layout.as_str(), &supported, &keyword_abilities)?
        else {
            continue; // unsupported layout
        };
        std::fs::write(&todo_path, render(&card)?)
            .with_context(|| format!("writing {}", todo_path.display()))?;
        eprintln!("wrote {}", todo_path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::mtgjson::Legalities;

    /// A minimal normal-layout creature fixture; `text` is the oracle text.
    fn creature(text: Option<&'static str>) -> AtomicCard<'static> {
        AtomicCard {
            name: "Test Bear".into(),
            face_name: None,
            mana_cost: None,
            color_indicator: vec![],
            types: vec!["Creature".into()],
            supertypes: vec![],
            subtypes: vec!["Bear".into()],
            text: text.map(Into::into),
            power: Some("2".into()),
            toughness: Some("2".into()),
            loyalty: None,
            defense: None,
            layout: "normal".into(),
            legalities: Legalities::default(),
        }
    }

    /// `stat_value` is the only branchy conversion; pin every arm.
    #[test]
    fn stat_value_branches() {
        assert_eq!(stat_value("2"), StatValue::Number(2));
        assert_eq!(stat_value("-1"), StatValue::Number(-1));
        assert_eq!(stat_value("*"), StatValue::DefinedByAbility);
        assert_eq!(stat_value("1+*"), StatValue::DefinedByAbility);
        assert_eq!(stat_value("X"), StatValue::Variable);
    }

    /// A bare-`normal` creature with one keyword line: structured fields become
    /// `RawIdent`s, the oracle line becomes a single `Unparsed` ability.
    #[test]
    fn normal_creature_builds() {
        let card = creature(Some("Flying"));

        let todo = todo_card("normal", &[&card], &[]).unwrap().unwrap();
        let TodoCard::Normal(face) = &todo else {
            panic!("expected Normal");
        };
        assert_eq!(face.types, [RawIdent("Creature".into())]);
        assert_eq!(face.subtypes, [RawIdent("Bear".into())]);
        assert_eq!(face.power, Some(StatValue::Number(2)));
        assert_eq!(face.toughness, Some(StatValue::Number(2)));
        // Empty keyword list: "Flying" stays one non-keyword line -> one ability.
        assert!(
            matches!(&face.abilities[..], [TodoAbility::Unparsed(s)] if s == "Flying"),
            "abilities = {:?}",
            face.abilities
        );

        // Render round-trip: the on-disk house style carries the bare subtype
        // ident and the Unparsed oracle line.
        let rendered = render(&todo).unwrap();
        assert!(rendered.contains("subtypes: [Bear]"), "{rendered}");
        assert!(rendered.contains(r#"Unparsed("Flying")"#), "{rendered}");
    }

    /// `text: None` → no oracle text → empty `abilities` vec; the rendered
    /// output omits the `abilities` field entirely.
    #[test]
    fn no_text_yields_empty_abilities() {
        let card = creature(None);
        let TodoCard::Normal(face) = todo_card("normal", &[&card], &[]).unwrap().unwrap() else {
            panic!("expected Normal");
        };
        assert!(face.abilities.is_empty());
        // The rendered card omits the abilities field entirely.
        let rendered = render(&TodoCard::Normal(face)).unwrap();
        assert!(!rendered.contains("abilities"));
    }

    /// A non-core layout (`split`) yields no `TodoCard`.
    #[test]
    fn unsupported_layout_is_skipped() {
        let card = AtomicCard {
            name: "Whatever".into(),
            face_name: None,
            mana_cost: None,
            color_indicator: vec![],
            types: vec!["Instant".into()],
            supertypes: vec![],
            subtypes: vec![],
            text: None,
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
            layout: "split".into(),
            legalities: Legalities::default(),
        };
        assert!(todo_card("split", &[&card], &[]).unwrap().is_none());
    }
}
