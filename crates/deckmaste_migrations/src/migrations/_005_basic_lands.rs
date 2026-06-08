use super::card_todo::{CardFaceTodo, CardFile};
use crate::layout::PluginLayout;

/// A todo is a convertible basic land when it's a single normal face that
/// is nothing but a name plus Basic Land types. Any leftover rules text
/// (Wastes' "{T}: Add {C}.") means the ability model can't express the
/// card yet, so it stays a todo.
fn basic_land_face(card: &CardFile) -> Option<&CardFaceTodo> {
    let CardFile::Todo { layout, faces } = card;
    match faces.as_slice() {
        [face]
            if *layout == "normal"
                && face.types == ["Land"]
                && face.supertypes.iter().any(|s| *s == "Basic")
                && face.text.is_empty() =>
        {
            Some(face)
        }
        _ => None,
    }
}

/// The finished definition in the builtin/cards house style: bare idents,
/// arrays inline.
fn render_land(face: &CardFaceTodo) -> String {
    format!(
        "\
Normal(
    name: {name:?},
    supertypes: [{supertypes}],
    types: [Land],
    subtypes: [{subtypes}],
)
",
        name = face.name,
        supertypes = face.supertypes.join(", "),
        subtypes = face.subtypes.join(", "),
    )
}

pub(super) struct BasicLands;

impl super::Migration for BasicLands {
    fn apply(&self, plugin: &PluginLayout) -> anyhow::Result<()> {
        super::card_todo::convert_todos(plugin, |card| {
            Ok(basic_land_face(card)
                .map(render_land)
                .map(super::card_todo::Graduation::Final))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn todo(source: &str) -> CardFile { ron_options().from_str(source).unwrap() }

    const PLAINS: &str = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Plains",
            supertypes: ["Basic"],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
    ],
)
"#;

    const SNOW_PLAINS: &str = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Snow-Covered Plains",
            supertypes: [
                "Basic",
                "Snow",
            ],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
    ],
)
"#;

    const WASTES: &str = r#"Todo(
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

    #[test]
    fn converts_plains() {
        let card = todo(PLAINS);
        let face = basic_land_face(&card).expect("Plains converts");
        assert_eq!(
            render_land(face),
            r#"Normal(
    name: "Plains",
    supertypes: [Basic],
    types: [Land],
    subtypes: [Plains],
)
"#
        );
    }

    #[test]
    fn converts_snow_lands_with_inline_supertypes() {
        let card = todo(SNOW_PLAINS);
        let face = basic_land_face(&card).expect("Snow-Covered Plains converts");
        assert_eq!(
            render_land(face),
            r#"Normal(
    name: "Snow-Covered Plains",
    supertypes: [Basic, Snow],
    types: [Land],
    subtypes: [Plains],
)
"#
        );
    }

    /// Wastes has no basic land type: its "{T}: Add {C}." is printed
    /// ability text the model can't express yet, so it stays a todo.
    #[test]
    fn skips_wastes_printed_ability() {
        assert!(basic_land_face(&todo(WASTES)).is_none());
    }

    #[test]
    fn skips_nonbasic_lands() {
        let nonbasic = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Urza's Saga",
            supertypes: ["Legendary"],
            types: [
                "Enchantment",
                "Land",
            ],
            subtypes: [
                "Urza's",
                "Saga",
            ],
        ),
    ],
)
"#;
        assert!(basic_land_face(&todo(nonbasic)).is_none());
    }

    /// A plain dual land has `types: ["Land"]` but no Basic supertype:
    /// only the supertype check rejects it.
    #[test]
    fn skips_lands_without_basic_supertype() {
        let tundra = r#"Todo(
    layout: "normal",
    faces: [
        (
            name: "Tundra",
            types: ["Land"],
            subtypes: [
                "Plains",
                "Island",
            ],
        ),
    ],
)
"#;
        assert!(basic_land_face(&todo(tundra)).is_none());
    }

    #[test]
    fn skips_multiface_and_nonnormal_layouts() {
        let mdfc = r#"Todo(
    layout: "modal_dfc",
    faces: [
        (
            name: "A",
            supertypes: ["Basic"],
            types: ["Land"],
            subtypes: ["Plains"],
        ),
        (
            name: "B",
            supertypes: ["Basic"],
            types: ["Land"],
            subtypes: ["Island"],
        ),
    ],
)
"#;
        assert!(basic_land_face(&todo(mdfc)).is_none());
    }
}
