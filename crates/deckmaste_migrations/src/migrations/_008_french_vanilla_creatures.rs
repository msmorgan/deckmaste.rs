//! Graduates "french vanilla" creatures -- a single normal face whose entire
//! text is keyword abilities ([CR#702]) -- into parked definitions. Keyword
//! macros aren't real yet, so every conversion is `Blocked` (`.ron.pending`).
//! `_006` runs first, so no-text (vanilla) creatures never reach here.

use super::card_todo::{CardFaceTodo, CardFile, Graduation, Stat};
use super::{creature_face, keyword_ability};
use crate::layout::PluginLayout;

/// A single normal creature face with numeric power/toughness and at least one
/// text line, or `None`. `*` stats are characteristic-defining abilities, so
/// they stay todos (as in `_006`).
fn french_vanilla_face(card: &CardFile) -> Option<&CardFaceTodo> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face]
            if *layout == "normal"
                && face.types.iter().any(|t| *t == "Creature")
                && !face.text.is_empty()
                && matches!(face.power, Some(Stat::Number(_)))
                && matches!(face.toughness, Some(Stat::Number(_)))
                && face.loyalty.is_none()
                && face.defense.is_none() =>
        {
            Some(face)
        }
        _ => None,
    }
}

/// Renders the parked definition for a french vanilla creature, or `None` if
/// some text line isn't keyword abilities.
fn french_vanilla(card: &CardFile) -> anyhow::Result<Option<Graduation>> {
    let Some(face) = french_vanilla_face(card) else {
        return Ok(None);
    };
    let mut abilities = Vec::new();
    for line in &face.text {
        let Some(block) = keyword_ability::render_keyword_line(line)? else {
            return Ok(None);
        };
        abilities.push(block);
    }
    let definition = creature_face::render_creature(face, &abilities)?;
    Ok(Some(Graduation::Blocked(definition)))
}

pub(super) struct FrenchVanillaCreatures;

impl super::Migration for FrenchVanillaCreatures {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, french_vanilla)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }

    /// Serra Angel: Flying + Vigilance, parked with the abilities block.
    #[test]
    fn serra_angel_parks() {
        let card = todo(
            r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Serra Angel",
            mana_cost: [Generic(3), White, White],
            types: ["Creature"],
            subtypes: ["Angel"],
            text: ["Flying", "Vigilance"],
            power: 4,
            toughness: 4,
        ),
    ],
)
"#,
        );
        let Graduation::Blocked(def) = french_vanilla(&card).unwrap().expect("converts") else {
            panic!("expected a Blocked parked definition");
        };
        assert_eq!(
            def,
            r#"Normal(
    name: "Serra Angel",
    mana_cost: [
        Generic(3),
        White,
        White,
    ],
    types: [Creature],
    subtypes: [Angel],
    abilities: [
        Flying,
        Vigilance,
    ],
    power: 4,
    toughness: 4,
)
"#
        );
    }

    /// A single comma-joined keyword line.
    #[test]
    fn comma_joined_keywords() {
        let card = todo(
            r#"Todo(layout: "normal", faces: [(name: "Wind Drake", mana_cost: [Generic(2), Blue], types: ["Creature"], subtypes: ["Drake"], text: ["Flying"], power: 2, toughness: 2)])"#,
        );
        assert!(french_vanilla(&card).unwrap().is_some());
    }

    #[test]
    fn declines_non_keyword_text_and_star_stats() {
        // A non-keyword text line: not french vanilla.
        let rules = todo(
            r#"Todo(layout: "normal", faces: [(name: "Talker", types: ["Creature"], subtypes: ["Human"], text: ["When this dies, draw a card."], power: 1, toughness: 1)])"#,
        );
        assert!(french_vanilla(&rules).unwrap().is_none());

        // `*` power: a CDA, declines.
        let star = todo(
            r#"Todo(layout: "normal", faces: [(name: "Stary", types: ["Creature"], subtypes: ["Horror"], text: ["Flying"], power: Other("*"), toughness: 1)])"#,
        );
        assert!(french_vanilla(&star).unwrap().is_none());

        // A difficult keyword (Protection) declines its card.
        let prot = todo(
            r#"Todo(layout: "normal", faces: [(name: "Warder", types: ["Creature"], subtypes: ["Knight"], text: ["Protection from black"], power: 2, toughness: 2)])"#,
        );
        assert!(french_vanilla(&prot).unwrap().is_none());
    }
}
