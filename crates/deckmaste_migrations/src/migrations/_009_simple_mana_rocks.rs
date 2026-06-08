//! Graduates simple mana rocks -- a single normal non-creature artifact face
//! whose text is tap-for-mana abilities (and optional keyword abilities) --
//! into finished definitions. A keyword present parks the card (`Blocked`),
//! since keyword macros aren't real yet; otherwise it's `Final`.

use std::fmt::Write;

use super::card_todo::{CardFaceTodo, CardFile, Graduation};
use super::mana_ability::{TapAbility, parse_tap_ability, render_tap_ability};
use super::{creature_face, keyword_ability};
use crate::layout::PluginLayout;

/// One recognized line on a mana rock.
enum RockLine {
    Tap(TapAbility),
    /// A keyword line, already rendered to one or more ability blocks.
    Keyword(String),
}

/// A single normal non-creature artifact face, or `None`.
fn rock_face(card: &CardFile) -> Option<&CardFaceTodo> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face]
            if *layout == "normal"
                && face.types.iter().any(|t| *t == "Artifact")
                && !face.types.iter().any(|t| *t == "Creature")
                && face.power.is_none()
                && face.toughness.is_none()
                && face.loyalty.is_none()
                && face.defense.is_none() =>
        {
            Some(face)
        }
        _ => None,
    }
}

/// Renders the artifact face shell with a pre-rendered abilities block. Like
/// `creature_face::render_creature` but with no power/toughness.
fn render_rock(face: &CardFaceTodo, abilities: &[String]) -> anyhow::Result<String> {
    let mut out = String::new();
    writeln!(out, "Normal(")?;
    writeln!(out, "    name: {:?},", face.name)?;
    creature_face::mana_cost_block(&mut out, face)?;
    creature_face::color_indicator_line(&mut out, face)?;
    creature_face::ident_line(&mut out, "supertypes", &face.supertypes);
    creature_face::ident_line(&mut out, "types", &face.types);
    creature_face::ident_line(&mut out, "subtypes", &face.subtypes);
    creature_face::abilities_block(&mut out, abilities);
    writeln!(out, ")")?;
    Ok(out)
}

/// `Final` for a keyword-free rock, `Blocked` when a keyword is present, `None`
/// if the face isn't a simple mana rock.
fn simple_rock(card: &CardFile) -> anyhow::Result<Option<Graduation>> {
    let Some(face) = rock_face(card) else {
        return Ok(None);
    };

    let mut lines = Vec::new();
    for line in &face.text {
        let recognized = match parse_tap_ability(line) {
            Some(tap) => RockLine::Tap(tap),
            None => match keyword_ability::render_keyword_line(line)? {
                Some(block) => RockLine::Keyword(block),
                None => return Ok(None),
            },
        };
        lines.push(recognized);
    }

    // Must actually make mana, else it isn't a mana rock.
    let makes_mana = lines
        .iter()
        .any(|l| matches!(l, RockLine::Tap(TapAbility::Mana(_))));
    if !makes_mana {
        return Ok(None);
    }
    // At most one enters-tapped line.
    if lines
        .iter()
        .filter(|l| matches!(l, RockLine::Tap(TapAbility::EntersTapped)))
        .count()
        > 1
    {
        return Ok(None);
    }

    let blocked = lines.iter().any(|l| matches!(l, RockLine::Keyword(_)));
    let abilities = lines
        .iter()
        .map(|line| match line {
            RockLine::Tap(tap) => render_tap_ability(tap),
            RockLine::Keyword(block) => Ok(block.clone()),
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let definition = render_rock(face, &abilities)?;
    Ok(Some(if blocked {
        Graduation::Blocked(definition)
    } else {
        Graduation::Final(definition)
    }))
}

pub(super) struct SimpleManaRocks;

impl super::Migration for SimpleManaRocks {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, simple_rock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }
    fn final_def(source: &str) -> String {
        match simple_rock(&todo(source)).unwrap().expect("converts") {
            Graduation::Final(def) => def,
            Graduation::Blocked(_) => panic!("expected Final"),
        }
    }

    #[test]
    fn sol_ring() {
        assert_eq!(
            final_def(
                r#"Todo(layout: "normal", faces: [(name: "Sol Ring", mana_cost: [Generic(1)], types: ["Artifact"], text: ["{T}: Add {C}{C}."])])"#
            ),
            r#"Normal(
    name: "Sol Ring",
    mana_cost: [Generic(1)],
    types: [Artifact],
    abilities: [
        Activated(
            cost: [Tap],
            effect: AddMana(2, Colorless),
        ),
    ],
)
"#
        );
    }

    #[test]
    fn worn_powerstone_enters_tapped() {
        let def = final_def(
            r#"Todo(layout: "normal", faces: [(name: "Worn Powerstone", mana_cost: [Generic(3)], types: ["Artifact"], text: ["~ enters tapped.", "{T}: Add {C}{C}."])])"#,
        );
        assert!(def.contains("Replacement(AsEnters(effect: Tap(This)))"));
        assert!(def.contains("effect: AddMana(2, Colorless),"));
    }

    #[test]
    fn manalith_any_color() {
        assert!(
            final_def(
                r#"Todo(layout: "normal", faces: [(name: "Manalith", mana_cost: [Generic(3)], types: ["Artifact"], text: ["{T}: Add one mana of any color."])])"#
            )
            .contains("effect: AddMana(1, AnyColor),")
        );
    }

    #[test]
    fn heterogeneous_rock_is_a_sequence() {
        assert!(
            final_def(
                r#"Todo(layout: "normal", faces: [(name: "Fountain", mana_cost: [Generic(2)], types: ["Artifact"], text: ["{T}: Add {W}{U}."])])"#
            )
            .contains("effect: Sequence([AddMana(1, White), AddMana(1, Blue)]),")
        );
    }

    #[test]
    fn keyworded_rock_is_blocked() {
        let card = todo(
            r#"Todo(layout: "normal", faces: [(name: "Warded Stone", mana_cost: [Generic(2)], types: ["Artifact"], text: ["Ward {2}", "{T}: Add {C}."])])"#,
        );
        let Graduation::Blocked(def) = simple_rock(&card).unwrap().expect("converts") else {
            panic!("expected Blocked");
        };
        assert!(def.contains("Ward([Mana([Generic(2)])]),"));
        assert!(def.contains("effect: AddMana(1, Colorless),"));
    }

    #[test]
    fn declines_creatures_extra_cost_and_no_mana() {
        // Artifact creature -> _010, not a rock.
        let golem = todo(
            r#"Todo(layout: "normal", faces: [(name: "Myr", mana_cost: [Generic(2)], types: ["Artifact", "Creature"], subtypes: ["Myr"], text: ["{T}: Add {C}."], power: 1, toughness: 1)])"#,
        );
        assert!(simple_rock(&golem).unwrap().is_none());

        // Extra activation cost: not simple.
        let signet = todo(
            r#"Todo(layout: "normal", faces: [(name: "Signet", mana_cost: [Generic(2)], types: ["Artifact"], text: ["{1}, {T}: Add {W}{U}."])])"#,
        );
        assert!(simple_rock(&signet).unwrap().is_none());

        // Makes no mana: not a mana rock.
        let inert = todo(
            r#"Todo(layout: "normal", faces: [(name: "Inert", mana_cost: [Generic(1)], types: ["Artifact"], text: ["~ enters tapped."])])"#,
        );
        assert!(simple_rock(&inert).unwrap().is_none());
    }
}
