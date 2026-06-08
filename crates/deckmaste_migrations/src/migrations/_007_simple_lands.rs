//! Graduates todos whose faces are all *simple* lands -- a tap-for-mana
//! activated ability and/or "~ enters tapped." -- into finished `Card`
//! definitions. The first migration to emit structured `abilities`. Reads
//! `_004`'s `~`-normalized text, so the enters-tapped line is
//! `"~ enters tapped."`.

use super::card_todo::{CardFaceTodo, CardFile};
use super::mana_ability::{TapAbility, parse_tap_ability, render_tap_ability};
use crate::layout::PluginLayout;

/// Basic land type subtypes. Each confers the intrinsic tap-for-mana ability
/// ([CR#305.6]), so a land carrying one needs no printed mana ability.
const BASIC_LAND_TYPES: [&str; 5] = ["Plains", "Island", "Swamp", "Mountain", "Forest"];

/// A simple-land face's abilities in printed order, or `None` if the face
/// isn't a simple land. Two channels by whether the face has a basic land type
/// ([CR#305.6]): an intrinsic-mana land must not *also* print a tap ability
/// (ambiguous), and a non-intrinsic land must produce mana some other way.
fn land_abilities(face: &CardFaceTodo) -> Option<Vec<TapAbility>> {
    if face.types != ["Land"]
        || !face.mana_cost.is_empty()
        || !face.color_indicator.is_empty()
        || face.power.is_some()
        || face.toughness.is_some()
        || face.loyalty.is_some()
        || face.defense.is_some()
    {
        return None;
    }

    let abilities: Vec<TapAbility> = face
        .text
        .iter()
        .map(|line| parse_tap_ability(line))
        .collect::<Option<_>>()?;

    if abilities
        .iter()
        .filter(|a| matches!(a, TapAbility::EntersTapped))
        .count()
        > 1
    {
        return None;
    }

    let has_basic_type = face
        .subtypes
        .iter()
        .any(|s| BASIC_LAND_TYPES.iter().any(|&b| *s == b));
    let makes_mana = abilities.iter().any(|a| matches!(a, TapAbility::Mana(_)));
    match (has_basic_type, makes_mana) {
        (true, true) => return None, // intrinsic mana + a printed one: ambiguous
        (false, false) => return None, // no way to make mana: not a simple land
        _ => {}
    }

    Some(abilities)
}

/// A face's fields at 4-space (Normal) indent, ending in a newline. The
/// `ModalDfc` path re-indents this body one level.
fn render_face(face: &CardFaceTodo, abilities: &[TapAbility]) -> anyhow::Result<String> {
    use std::fmt::Write;

    let mut out = String::new();
    writeln!(out, "    name: {:?},", face.name)?;
    if !face.supertypes.is_empty() {
        writeln!(out, "    supertypes: [{}],", face.supertypes.join(", "))?;
    }
    writeln!(out, "    types: [Land],")?;
    if !face.subtypes.is_empty() {
        writeln!(out, "    subtypes: [{}],", face.subtypes.join(", "))?;
    }
    if !abilities.is_empty() {
        writeln!(out, "    abilities: [")?;
        for ability in abilities {
            out.push_str(&render_tap_ability(ability)?);
        }
        writeln!(out, "    ],")?;
    }
    Ok(out)
}

/// Prefixes four spaces to every line (for a `ModalDfc` face body).
fn indent(body: &str) -> String { body.lines().map(|line| format!("    {line}\n")).collect() }

fn render_normal(face: &CardFaceTodo, abilities: &[TapAbility]) -> anyhow::Result<String> {
    Ok(format!(
        "Normal(\n{}\n)\n",
        render_face(face, abilities)?.trim_end_matches('\n')
    ))
}

fn render_modal_dfc(
    front: &CardFaceTodo,
    front_abilities: &[TapAbility],
    back: &CardFaceTodo,
    back_abilities: &[TapAbility],
) -> anyhow::Result<String> {
    let front = indent(&render_face(front, front_abilities)?);
    let back = indent(&render_face(back, back_abilities)?);
    Ok(format!(
        "ModalDfc(\n    (\n{front}    ),\n    (\n{back}    ),\n)\n"
    ))
}

/// Renders the finished definition for a card whose every face is a simple
/// land, or `None` if it doesn't qualify.
fn simple_land(card: &CardFile) -> anyhow::Result<Option<String>> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face] if *layout == "normal" => match land_abilities(face) {
            Some(abilities) => Ok(Some(render_normal(face, &abilities)?)),
            None => Ok(None),
        },
        [front, back] if *layout == "modal_dfc" => {
            match (land_abilities(front), land_abilities(back)) {
                (Some(fa), Some(ba)) => Ok(Some(render_modal_dfc(front, &fa, back, &ba)?)),
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}

pub(super) struct SimpleLands;

impl super::Migration for SimpleLands {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, |card| {
            Ok(simple_land(card)?.map(super::card_todo::Graduation::Final))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }
    fn render(source: &str) -> String {
        simple_land(&todo(source))
            .unwrap()
            .expect("fixture converts")
    }
    fn declines(source: &str) -> bool { simple_land(&todo(source)).unwrap().is_none() }

    /// Explicit dual (no basic land type): enters-tapped static then the
    /// `OneOf` mana ability, in printed order.
    #[test]
    fn coastal_tower_dual() {
        let card = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Coastal Tower",
            types: ["Land"],
            text: [
                "~ enters tapped.",
                "{T}: Add {W} or {U}.",
            ],
        ),
    ],
)
"#;
        assert_eq!(
            render(card),
            r#"Normal(
    name: "Coastal Tower",
    types: [Land],
    abilities: [
        Static(
            effects: [Replacement(AsEnters(effect: Tap(This)))],
        ),
        Activated(
            cost: [Tap],
            effect: AddMana(1, OneOf([White, Blue])),
        ),
    ],
)
"#
        );
    }

    /// Intrinsic dual with no text: bare type line, no abilities (the mana
    /// ability comes from the Island/Mountain subtypes).
    #[test]
    fn volcanic_island_intrinsic_no_text() {
        let card = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Volcanic Island",
            types: ["Land"],
            subtypes: [
                "Island",
                "Mountain",
            ],
        ),
    ],
)
"#;
        assert_eq!(
            render(card),
            r#"Normal(
    name: "Volcanic Island",
    types: [Land],
    subtypes: [Island, Mountain],
)
"#
        );
    }

    /// Intrinsic dual that enters tapped: type line + the enters-tapped
    /// static only, still no activated mana ability.
    #[test]
    fn contaminated_aquifer_intrinsic_tapped() {
        let card = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Contaminated Aquifer",
            types: ["Land"],
            subtypes: [
                "Island",
                "Swamp",
            ],
            text: ["~ enters tapped."],
        ),
    ],
)
"#;
        assert_eq!(
            render(card),
            r#"Normal(
    name: "Contaminated Aquifer",
    types: [Land],
    subtypes: [Island, Swamp],
    abilities: [
        Static(
            effects: [Replacement(AsEnters(effect: Tap(This)))],
        ),
    ],
)
"#
        );
    }

    /// Wastes: a Basic supertype but no basic land type *subtype*, so the
    /// `{T}: Add {C}.` is real printed text -- the explicit channel. Now
    /// expressible (it was blocked when `_005` ran).
    #[test]
    fn wastes_colorless() {
        let card = r#"Todo(
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
        assert_eq!(
            render(card),
            r#"Normal(
    name: "Wastes",
    supertypes: [Basic],
    types: [Land],
    abilities: [
        Activated(
            cost: [Tap],
            effect: AddMana(1, Colorless),
        ),
    ],
)
"#
        );
    }

    /// Single-color tap land and an any-color land render their scalar spec.
    #[test]
    fn single_color_and_any_color() {
        let mono = r#"Todo(
    layout: "normal",
    faces: [(name: "Forested Hill", types: ["Land"], text: ["{T}: Add {G}."])],
)
"#;
        assert!(render(mono).contains("effect: AddMana(1, Green),"));

        let rainbow = r#"Todo(
    layout: "normal",
    faces: [(name: "Rainbow Vale", types: ["Land"], text: ["{T}: Add one mana of any color."])],
)
"#;
        assert!(render(rainbow).contains("effect: AddMana(1, AnyColor),"));
    }

    /// Pathway MDFC: both faces single-color tap lands -> `ModalDfc`, one
    /// `Activated` per face, body re-indented one level.
    #[test]
    fn pathway_modal_dfc() {
        let card = r#"Todo(
    layout: "modal_dfc",
    faces: [
        (
            name: "Barkchannel Pathway",
            types: ["Land"],
            text: ["{T}: Add {G}."],
        ),
        (
            name: "Tidechannel Pathway",
            types: ["Land"],
            text: ["{T}: Add {U}."],
        ),
    ],
)
"#;
        assert_eq!(
            render(card),
            r#"ModalDfc(
    (
        name: "Barkchannel Pathway",
        types: [Land],
        abilities: [
            Activated(
                cost: [Tap],
                effect: AddMana(1, Green),
            ),
        ],
    ),
    (
        name: "Tidechannel Pathway",
        types: [Land],
        abilities: [
            Activated(
                cost: [Tap],
                effect: AddMana(1, Blue),
            ),
        ],
    ),
)
"#
        );
    }

    #[test]
    fn declines_conditional_enters_tapped() {
        // "enters tapped unless ..." is not the bare enters-tapped line.
        let card = r#"Todo(
    layout: "normal",
    faces: [(name: "Conditional", types: ["Land"], text: [
        "~ enters tapped unless you control two or more other lands.",
        "{T}: Add {G}.",
    ])],
)
"#;
        assert!(declines(card));
    }

    #[test]
    fn declines_intrinsic_with_printed_mana() {
        // Basic land type (intrinsic mana) AND a printed tap ability: ambiguous.
        let card = r#"Todo(
    layout: "normal",
    faces: [(name: "Weird Forest", types: ["Land"], subtypes: ["Forest"], text: ["{T}: Add {C}."])],
)
"#;
        assert!(declines(card));
    }

    #[test]
    fn declines_non_land_and_statful_and_extra_cost() {
        let creature = r#"Todo(
    layout: "normal",
    faces: [(name: "Bear", types: ["Creature"], subtypes: ["Bear"], power: 2, toughness: 2)],
)
"#;
        assert!(declines(creature));

        // Manland: Land with power/toughness is not a simple land.
        let manland = r#"Todo(
    layout: "normal",
    faces: [(name: "Mutavault-like", types: ["Land"], text: ["{T}: Add {C}."], power: 2, toughness: 2)],
)
"#;
        assert!(declines(manland));

        // Extra activation cost beyond {T} is out of scope.
        let extra = r#"Todo(
    layout: "normal",
    faces: [(name: "Pricey", types: ["Land"], text: ["{1}, {T}: Add one mana of any color."])],
)
"#;
        assert!(declines(extra));

        // A land with no way to make mana and no basic type: not simple.
        let inert = r#"Todo(
    layout: "normal",
    faces: [(name: "Inert", types: ["Land"], text: ["~ enters tapped."])],
)
"#;
        assert!(declines(inert));
    }

    /// A modal_dfc with a non-land face (spell // land) declines wholesale.
    #[test]
    fn declines_mdfc_with_nonland_face() {
        let card = r#"Todo(
    layout: "modal_dfc",
    faces: [
        (name: "Spell Side", types: ["Sorcery"], text: ["Draw a card."]),
        (name: "Land Side", types: ["Land"], text: ["{T}: Add {G}."]),
    ],
)
"#;
        assert!(declines(card));
    }

    /// The rendered output is always valid RON (the `convert_todos` guard).
    #[test]
    fn rendered_output_is_valid_ron() {
        let card = r#"Todo(
    layout: "normal",
    faces: [(name: "Coastal Tower", types: ["Land"], text: ["~ enters tapped.", "{T}: Add {W} or {U}."])],
)
"#;
        ron::value::RawValue::from_ron(&render(card)).expect("valid RON");
    }
}
