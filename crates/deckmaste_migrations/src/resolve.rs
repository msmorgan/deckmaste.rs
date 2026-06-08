//! `resolve` — the ability-by-ability rewriter. For each `cards/*.ron.todo`,
//! every `TodoAbility::Unparsed(line)` is run through an ordered registry of
//! ability parsers; the first that structures the line becomes a
//! `TodoAbility::Parsed(<bare ability RON>)`. Pure per-card map; idempotent.

use std::path::Path;

use deckmaste_core::plugin::is_ron_todo_file;

use crate::todo_card::{TodoAbility, TodoCard, TodoCardFace, render};

/// One ability parser: a normalized oracle line -> the bare RON of one ability
/// (`Flying`, `Activated(cost: [Tap], effect: AddMana(1, Green))`), or `None`
/// to decline.
pub type AbilityParser = fn(&str) -> anyhow::Result<Option<String>>;

/// The registry, in priority order. First match wins.
pub const REGISTRY: &[AbilityParser] = &[
    crate::migrations::mana_ability::resolve_line,
    crate::migrations::keyword_ability::resolve_line,
];

/// Replaces every `Unparsed` line a parser in `registry` can structure with the
/// `Parsed` RON. Returns whether anything changed.
fn resolve_face(face: &mut TodoCardFace, registry: &[AbilityParser]) -> anyhow::Result<bool> {
    let mut changed = false;
    for ability in &mut face.abilities {
        let TodoAbility::Unparsed(line) = ability else {
            continue;
        };
        for parser in registry {
            if let Some(ron) = parser(line)? {
                *ability = TodoAbility::Parsed(ron);
                changed = true;
                break;
            }
        }
    }
    Ok(changed)
}

/// Resolves a whole card against the default [`REGISTRY`]. Returns whether
/// anything changed (so callers can skip rewriting unchanged files).
///
/// # Errors
/// If any parser in the registry returns an error.
pub fn resolve_card(card: &mut TodoCard) -> anyhow::Result<bool> {
    resolve_card_with(card, REGISTRY)
}

/// `resolve_card` against a given registry (test seam).
///
/// # Errors
/// If any parser in `registry` returns an error.
pub fn resolve_card_with(card: &mut TodoCard, registry: &[AbilityParser]) -> anyhow::Result<bool> {
    let changed = match card {
        TodoCard::Normal(face) => resolve_face(face, registry)?,
        TodoCard::ModalDfc(front, back) => {
            let a = resolve_face(front, registry)?;
            let b = resolve_face(back, registry)?;
            a || b
        }
    };
    Ok(changed)
}

/// Resolves every `cards/*.ron.todo` in `plugin_dir` in place.
///
/// # Errors
/// If a file isn't readable/parsable as a `TodoCard`, or isn't writable.
pub fn resolve_cards(plugin_dir: &Path) -> anyhow::Result<()> {
    let cards = crate::layout::PluginLayout::new(plugin_dir)?.cards_dir()?;
    let mut paths: Vec<_> = std::fs::read_dir(&cards)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<_, _>>()?;
    paths.sort();
    for path in paths {
        if !path.is_file() || !is_ron_todo_file(&path) {
            continue;
        }
        // A malformed `.ron.todo` aborts the run (via `?`): it means a bug in
        // the step that wrote it, which the engineer should fix before resolving.
        let source = std::fs::read_to_string(&path)?;
        let mut card: TodoCard = crate::ron_output::ron_options().from_str(&source)?;
        if resolve_card(&mut card)? {
            std::fs::write(&path, render(&card)?)?;
            eprintln!("resolved {}", path.display());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todo_card::RawIdent;

    /// A registry that structures the line "Flying" only.
    #[allow(clippy::unnecessary_wraps)]
    fn flying_only(line: &str) -> anyhow::Result<Option<String>> {
        Ok((line == "Flying").then(|| "Flying".to_owned()))
    }

    #[test]
    fn resolve_replaces_known_lines_only() {
        use deckmaste_core::ManaCost;
        let mut card = TodoCard::Normal(TodoCardFace {
            name: "X".into(),
            mana_cost: ManaCost::default(),
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![RawIdent("Creature".into())],
            subtypes: vec![],
            abilities: vec![
                TodoAbility::Unparsed("Flying".into()),
                TodoAbility::Unparsed("When ~ dies, draw a card.".into()),
            ],
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
        });
        let changed = resolve_card_with(&mut card, &[flying_only]).unwrap();
        assert!(changed);
        let TodoCard::Normal(face) = &card else { panic!() };
        assert!(matches!(&face.abilities[0], TodoAbility::Parsed(r) if r == "Flying"));
        assert!(matches!(&face.abilities[1], TodoAbility::Unparsed(_))); // unchanged
        // Idempotent: a second pass changes nothing.
        assert!(!resolve_card_with(&mut card, &[flying_only]).unwrap());
    }

    #[test]
    fn resolve_modal_dfc_resolves_both_faces() {
        let mut card = TodoCard::ModalDfc(
            TodoCardFace {
                abilities: vec![TodoAbility::Unparsed("Flying".into())],
                ..Default::default()
            },
            TodoCardFace {
                abilities: vec![TodoAbility::Unparsed("Flying".into())],
                ..Default::default()
            },
        );
        assert!(resolve_card_with(&mut card, &[flying_only]).unwrap());
        let TodoCard::ModalDfc(front, back) = &card else { panic!() };
        assert!(matches!(&front.abilities[0], TodoAbility::Parsed(r) if r == "Flying"));
        assert!(matches!(&back.abilities[0], TodoAbility::Parsed(r) if r == "Flying"));
        // Idempotent.
        assert!(!resolve_card_with(&mut card, &[flying_only]).unwrap());
    }
}
