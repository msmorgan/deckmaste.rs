use deckmaste_core::Ident;
use serde::Serialize;

use super::card_todo::{CardFaceTodo, CardFile, Stat};
use crate::layout::PluginLayout;

/// A todo is a convertible vanilla creature when it's a single normal face
/// with Creature among its types, no rules text, and plain numeric stats.
/// `*` stats are characteristic-defining abilities in disguise, so they
/// stay todos with the other text-bearing cards.
fn vanilla_creature_face(card: &CardFile) -> Option<&CardFaceTodo> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face]
            if *layout == "normal"
                && face.types.iter().any(|t| *t == "Creature")
                && face.text.is_empty()
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

/// One leaf value (mana symbol, color, stat) spelled by the shared ron
/// config — tuple members stay inline, so `Hybrid(Generic(2), White)`
/// keeps its canonical spacing. The template owns the file shape; ron
/// only spells the tokens.
fn leaf<T: Serialize>(value: &T) -> anyhow::Result<String> {
    Ok(crate::ron_output::ron_options()
        .to_string_pretty(value, crate::ron_output::pretty_config())?)
}

/// `    field: [a, b],` — ident arrays stay inline; nothing for `[]`.
fn ident_line(out: &mut String, field: &str, idents: &[Ident]) {
    use std::fmt::Write;

    if !idents.is_empty() {
        writeln!(out, "    {field}: [{}],", idents.join(", ")).unwrap();
    }
}

/// The finished definition in the builtin/cards house style: ident arrays
/// inline, multi-symbol mana costs chopped like the hand-written
/// Grizzly Bears.
fn render_creature(face: &CardFaceTodo) -> anyhow::Result<String> {
    use std::fmt::Write;

    let mut out = String::new();
    writeln!(out, "Normal(")?;
    writeln!(out, "    name: {:?},", face.name)?;
    match &*face.mana_cost {
        [] => {}
        [symbol] => writeln!(out, "    mana_cost: [{}],", leaf(symbol)?)?,
        symbols => {
            writeln!(out, "    mana_cost: [")?;
            for symbol in symbols {
                writeln!(out, "        {},", leaf(symbol)?)?;
            }
            writeln!(out, "    ],")?;
        }
    }
    if !face.color_indicator.is_empty() {
        let colors: Vec<String> = face
            .color_indicator
            .iter()
            .map(leaf)
            .collect::<anyhow::Result<_>>()?;
        writeln!(out, "    color_indicator: [{}],", colors.join(", "))?;
    }
    ident_line(&mut out, "supertypes", &face.supertypes);
    ident_line(&mut out, "types", &face.types);
    ident_line(&mut out, "subtypes", &face.subtypes);
    // The predicate guarantees both stats; a creature file without them
    // would be a silent authoring error, so fail loudly instead.
    let (Some(power), Some(toughness)) = (&face.power, &face.toughness) else {
        anyhow::bail!(
            "vanilla creature {:?} is missing power/toughness",
            face.name
        );
    };
    writeln!(out, "    power: {},", leaf(power)?)?;
    writeln!(out, "    toughness: {},", leaf(toughness)?)?;
    writeln!(out, ")")?;
    Ok(out)
}

pub(super) struct VanillaCreatures;

impl super::Migration for VanillaCreatures {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, |card| {
            Ok(vanilla_creature_face(card)
                .map(render_creature)
                .transpose()?
                .map(super::card_todo::Graduation::Final))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }

    fn render(source: &str) -> String {
        let card = todo(source);
        let face = vanilla_creature_face(&card).expect("fixture converts");
        render_creature(face).unwrap()
    }

    const GRIZZLY_BEARS: &str = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Grizzly Bears",
            mana_cost: [
                Generic(1),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Bear"],
            power: 2,
            toughness: 2,
        ),
    ],
)
"#;

    /// Multi-symbol mana costs chop one per line — byte-identical to the
    /// hand-written builtin/cards/Grizzly Bears.ron.
    #[test]
    fn grizzly_bears_matches_builtin() {
        assert_eq!(
            render(GRIZZLY_BEARS),
            r#"Normal(
    name: "Grizzly Bears",
    mana_cost: [
        Generic(1),
        Green,
    ],
    types: [Creature],
    subtypes: [Bear],
    power: 2,
    toughness: 2,
)
"#
        );
    }

    /// Single-symbol costs stay on one line; multi-type arrays are inline.
    #[test]
    fn artifact_creature() {
        let golem = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Obsianus Golem",
            mana_cost: [Generic(6)],
            types: [
                "Artifact",
                "Creature",
            ],
            subtypes: ["Golem"],
            power: 4,
            toughness: 6,
        ),
    ],
)
"#;
        assert_eq!(
            render(golem),
            r#"Normal(
    name: "Obsianus Golem",
    mana_cost: [Generic(6)],
    types: [Artifact, Creature],
    subtypes: [Golem],
    power: 4,
    toughness: 6,
)
"#
        );
    }

    /// No mana cost at all (the line is omitted), a color indicator, and
    /// Land Creature types.
    #[test]
    fn dryad_arbor() {
        let dryad = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Dryad Arbor",
            color_indicator: [Green],
            types: [
                "Land",
                "Creature",
            ],
            subtypes: [
                "Forest",
                "Dryad",
            ],
            power: 1,
            toughness: 1,
        ),
    ],
)
"#;
        assert_eq!(
            render(dryad),
            r#"Normal(
    name: "Dryad Arbor",
    color_indicator: [Green],
    types: [Land, Creature],
    subtypes: [Forest, Dryad],
    power: 1,
    toughness: 1,
)
"#
        );
    }

    /// Supertypes render inline when present.
    #[test]
    fn legendary_vanilla() {
        let jedit = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Jedit Ojanen",
            mana_cost: [
                Generic(4),
                White,
                White,
                Blue,
            ],
            supertypes: ["Legendary"],
            types: ["Creature"],
            subtypes: [
                "Cat",
                "Warrior",
            ],
            power: 5,
            toughness: 5,
        ),
    ],
)
"#;
        assert_eq!(
            render(jedit),
            r#"Normal(
    name: "Jedit Ojanen",
    mana_cost: [
        Generic(4),
        White,
        White,
        Blue,
    ],
    supertypes: [Legendary],
    types: [Creature],
    subtypes: [Cat, Warrior],
    power: 5,
    toughness: 5,
)
"#
        );
    }

    #[test]
    fn skips_creatures_with_text() {
        let courier = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Transguild Courier",
            mana_cost: [Generic(4)],
            types: [
                "Artifact",
                "Creature",
            ],
            subtypes: ["Golem"],
            text: ["Transguild Courier is all colors."],
            power: 3,
            toughness: 3,
        ),
    ],
)
"#;
        assert!(vanilla_creature_face(&todo(courier)).is_none());
    }

    /// `*` stats mean a characteristic-defining ability: not vanilla even
    /// with no other text.
    #[test]
    fn skips_star_stats() {
        let goyf = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Goyfish",
            mana_cost: [
                Generic(1),
                Green,
            ],
            types: ["Creature"],
            subtypes: ["Lhurgoyf"],
            power: Other("*"),
            toughness: Other("1+*"),
        ),
    ],
)
"#;
        assert!(vanilla_creature_face(&todo(goyf)).is_none());
    }

    #[test]
    fn skips_noncreatures_and_statless() {
        let land = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Wastes",
            supertypes: ["Basic"],
            types: ["Land"],
            text: ["{T}: Add {C}."],
        ),
    ],
)
"#;
        assert!(vanilla_creature_face(&todo(land)).is_none());
    }

    /// No real vanilla creature lacks a subtype today, but the line must
    /// drop cleanly if one ever does.
    #[test]
    fn no_subtypes() {
        let synthetic = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Nameless Race",
            mana_cost: [
                Generic(3),
                Black,
            ],
            types: ["Creature"],
            power: 6,
            toughness: 4,
        ),
    ],
)
"#;
        assert_eq!(
            render(synthetic),
            r#"Normal(
    name: "Nameless Race",
    mana_cost: [
        Generic(3),
        Black,
    ],
    types: [Creature],
    power: 6,
    toughness: 4,
)
"#
        );
    }
}
