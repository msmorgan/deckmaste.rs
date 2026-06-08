//! Shared rendering for a finished card face in the builtin/cards house style:
//! the leaf/line primitives plus `render_creature`. `_006` (no abilities),
//! `_008`, and `_010` render creatures here; `_009` reuses the primitives for
//! its artifact face. `format!` templates own the file shape; ron only spells
//! leaf tokens.

use std::fmt::Write;

use deckmaste_core::Ident;
use serde::Serialize;

use super::card_todo::CardFaceTodo;

/// One leaf value spelled by the shared ron config (tuple members stay inline,
/// so `Hybrid(Generic(2), White)` keeps its canonical spacing).
pub(super) fn leaf<T: Serialize>(value: &T) -> anyhow::Result<String> {
    Ok(crate::ron_output::ron_options()
        .to_string_pretty(value, crate::ron_output::pretty_config())?)
}

/// `    field: [a, b],` for a non-empty ident array; nothing for `[]`. Each
/// ident is mapped through [`super::to_rust_ident`] so that names containing
/// non-alphanumeric characters (e.g. `Assembly-Worker`, `Time Lord`) become
/// their corresponding macro-invocation names (`AssemblyWorker`, `TimeLord`).
pub(super) fn ident_line(out: &mut String, field: &str, idents: &[Ident]) {
    if !idents.is_empty() {
        let items: Vec<String> = idents
            .iter()
            .map(|id| super::to_rust_ident(id.as_str()))
            .collect();
        writeln!(out, "    {field}: [{}],", items.join(", ")).unwrap();
    }
}

/// The `mana_cost:` block: omitted when empty, one line when single, chopped
/// one symbol per line otherwise. Matches the hand-written Grizzly Bears.
pub(super) fn mana_cost_block(out: &mut String, face: &CardFaceTodo) -> anyhow::Result<()> {
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
    Ok(())
}

/// `    color_indicator: [Green],` when present; nothing otherwise.
pub(super) fn color_indicator_line(out: &mut String, face: &CardFaceTodo) -> anyhow::Result<()> {
    if !face.color_indicator.is_empty() {
        let colors: Vec<String> = face
            .color_indicator
            .iter()
            .map(leaf)
            .collect::<anyhow::Result<_>>()?;
        writeln!(out, "    color_indicator: [{}],", colors.join(", "))?;
    }
    Ok(())
}

/// The `abilities: [...]` block from pre-rendered ability blocks (each already
/// at the 8-space items indent with a trailing comma+newline); omitted when
/// empty.
pub(super) fn abilities_block(out: &mut String, abilities: &[String]) {
    if !abilities.is_empty() {
        writeln!(out, "    abilities: [").unwrap();
        for block in abilities {
            out.push_str(block);
        }
        writeln!(out, "    ],").unwrap();
    }
}

/// Renders a finished `Normal(...)` creature face. `abilities` are pre-rendered
/// ability blocks; the block is omitted when empty. Power and toughness are
/// required — the caller's predicate guarantees numeric stats, so a missing one
/// is a loud authoring error.
pub(super) fn render_creature(face: &CardFaceTodo, abilities: &[String]) -> anyhow::Result<String> {
    let mut out = String::new();
    writeln!(out, "Normal(")?;
    writeln!(out, "    name: {:?},", face.name)?;
    mana_cost_block(&mut out, face)?;
    color_indicator_line(&mut out, face)?;
    ident_line(&mut out, "supertypes", &face.supertypes);
    ident_line(&mut out, "types", &face.types);
    ident_line(&mut out, "subtypes", &face.subtypes);
    abilities_block(&mut out, abilities);
    let (Some(power), Some(toughness)) = (&face.power, &face.toughness) else {
        anyhow::bail!("creature {:?} is missing power/toughness", face.name);
    };
    writeln!(out, "    power: {},", leaf(power)?)?;
    writeln!(out, "    toughness: {},", leaf(toughness)?)?;
    writeln!(out, ")")?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ron_output::ron_options;

    fn face(source: &str) -> CardFaceTodo {
        let CardFile::Todo { mut faces, .. } = ron_options().from_str(source).unwrap();
        faces.remove(0)
    }
    use super::super::card_todo::CardFile;

    /// No abilities: byte-identical to the hand-written Grizzly Bears.
    #[test]
    fn vanilla_creature_matches_builtin() {
        let bears = face(
            r#"Todo(layout: "normal", faces: [(name: "Grizzly Bears", mana_cost: [Generic(1), Green], types: ["Creature"], subtypes: ["Bear"], power: 2, toughness: 2)])"#,
        );
        assert_eq!(
            render_creature(&bears, &[]).unwrap(),
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

    /// With an abilities block (pre-rendered ability lines).
    #[test]
    fn creature_with_abilities_block() {
        let bird = face(
            r#"Todo(layout: "normal", faces: [(name: "Birds of Paradise", mana_cost: [Green], types: ["Creature"], subtypes: ["Bird"], power: 0, toughness: 1)])"#,
        );
        let abilities = [
            "        Flying,\n".to_owned(),
            "        Activated(\n            cost: [Tap],\n            effect: AddMana(1, AnyColor),\n        ),\n".to_owned(),
        ];
        assert_eq!(
            render_creature(&bird, &abilities).unwrap(),
            r#"Normal(
    name: "Birds of Paradise",
    mana_cost: [Green],
    types: [Creature],
    subtypes: [Bird],
    abilities: [
        Flying,
        Activated(
            cost: [Tap],
            effect: AddMana(1, AnyColor),
        ),
    ],
    power: 0,
    toughness: 1,
)
"#
        );
    }
}
