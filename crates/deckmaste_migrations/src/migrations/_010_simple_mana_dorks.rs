//! Graduates simple mana dorks -- a single normal creature face (numeric P/T)
//! whose text is `{T}: Add …` mana abilities and optional keyword abilities --
//! into finished definitions. A keyword present parks the card (`Blocked`);
//! otherwise it's `Final`. Creatures never "enter tapped", so that line is not
//! accepted here.

use super::card_todo::{CardFaceTodo, CardFile, Graduation, Stat};
use super::mana_ability::{TapAbility, parse_tap_ability, render_tap_ability};
use super::{creature_face, keyword_ability};
use crate::layout::PluginLayout;

/// One recognized line on a mana dork.
enum DorkLine {
    Mana(TapAbility),
    Keyword(String),
}

/// A single normal creature face with numeric power/toughness, or `None`.
fn dork_face(card: &CardFile) -> Option<&CardFaceTodo> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face]
            if *layout == "normal"
                && face.types.iter().any(|t| *t == "Creature")
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

/// `Final` for a keyword-free dork, `Blocked` when a keyword is present, `None`
/// if the face isn't a simple mana dork.
fn simple_dork(card: &CardFile) -> anyhow::Result<Option<Graduation>> {
    let Some(face) = dork_face(card) else {
        return Ok(None);
    };

    let mut lines = Vec::new();
    for line in &face.text {
        let recognized = match parse_tap_ability(line) {
            // Creatures never enter tapped: only a mana ability counts as a tap line.
            Some(TapAbility::Mana(production)) => DorkLine::Mana(TapAbility::Mana(production)),
            Some(TapAbility::EntersTapped) => return Ok(None),
            None => match keyword_ability::render_keyword_line(line)? {
                Some(block) => DorkLine::Keyword(block),
                None => return Ok(None),
            },
        };
        lines.push(recognized);
    }

    // Must make mana, else it isn't a mana dork.
    if !lines.iter().any(|l| matches!(l, DorkLine::Mana(_))) {
        return Ok(None);
    }

    let blocked = lines.iter().any(|l| matches!(l, DorkLine::Keyword(_)));
    let abilities = lines
        .iter()
        .map(|line| match line {
            DorkLine::Mana(tap) => render_tap_ability(tap),
            DorkLine::Keyword(block) => Ok(block.clone()),
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let definition = creature_face::render_creature(face, &abilities)?;
    Ok(Some(if blocked {
        Graduation::Blocked(definition)
    } else {
        Graduation::Final(definition)
    }))
}

pub(super) struct SimpleManaDorks;

impl super::Migration for SimpleManaDorks {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, simple_dork)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }

    #[test]
    fn llanowar_elves_is_final() {
        let card = todo(
            r#"Todo(layout: "normal", faces: [(name: "Llanowar Elves", mana_cost: [Green], types: ["Creature"], subtypes: ["Elf", "Druid"], text: ["{T}: Add {G}."], power: 1, toughness: 1)])"#,
        );
        let Graduation::Final(def) = simple_dork(&card).unwrap().expect("converts") else {
            panic!("expected Final");
        };
        assert_eq!(
            def,
            r#"Normal(
    name: "Llanowar Elves",
    mana_cost: [Green],
    types: [Creature],
    subtypes: [Elf, Druid],
    abilities: [
        Activated(
            cost: [Tap],
            effect: AddMana(1, Green),
        ),
    ],
    power: 1,
    toughness: 1,
)
"#
        );
    }

    #[test]
    fn birds_of_paradise_is_blocked() {
        let card = todo(
            r#"Todo(layout: "normal", faces: [(name: "Birds of Paradise", mana_cost: [Green], types: ["Creature"], subtypes: ["Bird"], text: ["Flying", "{T}: Add one mana of any color."], power: 0, toughness: 1)])"#,
        );
        let Graduation::Blocked(def) = simple_dork(&card).unwrap().expect("converts") else {
            panic!("expected Blocked");
        };
        assert!(def.contains("        Flying,\n"));
        assert!(def.contains("effect: AddMana(1, AnyColor),"));
    }

    #[test]
    fn artifact_creature_dork_is_final() {
        // Gold Myr: an artifact creature that taps for mana, no keyword.
        let card = todo(
            r#"Todo(layout: "normal", faces: [(name: "Gold Myr", mana_cost: [Generic(2)], types: ["Artifact", "Creature"], subtypes: ["Myr"], text: ["{T}: Add {W}."], power: 1, toughness: 1)])"#,
        );
        assert!(matches!(
            simple_dork(&card).unwrap(),
            Some(Graduation::Final(_))
        ));
    }

    #[test]
    fn declines_non_mana_extra_cost_and_star() {
        // Text that's neither keyword nor a mana ability.
        let rules = todo(
            r#"Todo(layout: "normal", faces: [(name: "Sage", mana_cost: [Blue], types: ["Creature"], subtypes: ["Wizard"], text: ["{T}: Draw a card."], power: 0, toughness: 1)])"#,
        );
        assert!(simple_dork(&rules).unwrap().is_none());

        // No mana ability at all (only a keyword): that's french vanilla, not a dork.
        let fv = todo(
            r#"Todo(layout: "normal", faces: [(name: "Flyer", mana_cost: [Blue], types: ["Creature"], subtypes: ["Bird"], text: ["Flying"], power: 1, toughness: 1)])"#,
        );
        assert!(simple_dork(&fv).unwrap().is_none());

        // `*` power: a CDA, declines.
        let star = todo(
            r#"Todo(layout: "normal", faces: [(name: "Star", mana_cost: [Green], types: ["Creature"], subtypes: ["Elemental"], text: ["{T}: Add {G}."], power: Other("*"), toughness: 1)])"#,
        );
        assert!(simple_dork(&star).unwrap().is_none());
    }
}
