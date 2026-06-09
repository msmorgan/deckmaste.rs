//! `resolve` — the ability-by-ability rewriter. For each `cards/*.ron.todo`,
//! every `TodoAbility::Unparsed(line)` is run through an ordered registry of
//! ability parsers; the first that structures the line becomes a
//! `TodoAbility::Parsed(<bare ability RON>)`. Pure per-card map; idempotent.

use std::path::Path;

use deckmaste_core::plugin::is_ron_todo_file;

use crate::todo_card::{RawIdent, TodoAbility, TodoCard, TodoCardFace, render};

/// The coarse card category a parser needs to decide framing: a `Spell` card
/// is an instant or sorcery (its effect text is a `Spell` ability); everything
/// else is a `Permanent` (effect text lives inside triggered/activated/static
/// frames). Computed once per face and handed to every parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardKind {
    Spell,
    Permanent,
}

impl CardKind {
    /// Classifies a face by its raw type line.
    #[must_use]
    pub fn of(types: &[RawIdent]) -> Self {
        if types
            .iter()
            .any(|t| matches!(t.0.as_str(), "Instant" | "Sorcery"))
        {
            CardKind::Spell
        } else {
            CardKind::Permanent
        }
    }
}

/// One ability parser: a normalized oracle line -> the bare RON of one ability
/// (`Flying`, `Activated(cost: [Tap], effect: AddMana(1, Green))`), or `None`
/// to decline.
pub type AbilityParser = fn(&str, CardKind) -> anyhow::Result<Option<String>>;

/// The registry, in priority order. First match wins.
pub const REGISTRY: &[AbilityParser] = &[
    crate::parsers::mana_ability::resolve_line,
    crate::parsers::keyword_ability::resolve_line,
    crate::parsers::spell_ability::resolve_line,
    crate::parsers::triggered_ability::resolve_line,
];

/// Replaces every `Unparsed` line a parser in `registry` can structure with the
/// `Parsed` RON. Returns whether anything changed.
fn resolve_face(face: &mut TodoCardFace, registry: &[AbilityParser]) -> anyhow::Result<bool> {
    let kind = CardKind::of(&face.types);
    let mut changed = false;
    for ability in &mut face.abilities {
        let TodoAbility::Unparsed(line) = ability else {
            continue;
        };
        for parser in registry {
            if let Some(ron) = parser(line, kind)? {
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

    /// A registry that structures the line "Flying" only.
    #[allow(clippy::unnecessary_wraps)]
    fn flying_only(line: &str, _kind: CardKind) -> anyhow::Result<Option<String>> {
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
    fn card_kind_classifies_by_type() {
        let ident = |s: &str| RawIdent(s.to_owned());
        assert_eq!(CardKind::of(&[ident("Instant")]), CardKind::Spell);
        assert_eq!(CardKind::of(&[ident("Sorcery")]), CardKind::Spell);
        assert_eq!(CardKind::of(&[ident("Creature")]), CardKind::Permanent);
        // Multi-type artifact-creature is a permanent; no Instant/Sorcery.
        assert_eq!(
            CardKind::of(&[ident("Artifact"), ident("Creature")]),
            CardKind::Permanent
        );
        // No type line at all defaults to permanent framing.
        assert_eq!(CardKind::of(&[]), CardKind::Permanent);
    }

    #[test]
    fn spell_card_resolves_damage_but_permanent_does_not() {
        // An instant with Lightning Bolt's line resolves to a Spell ability.
        let mut bolt = TodoCard::Normal(TodoCardFace {
            name: "Bolt".into(),
            types: vec![RawIdent("Instant".into())],
            abilities: vec![TodoAbility::Unparsed(
                "~ deals 3 damage to any target.".into(),
            )],
            ..Default::default()
        });
        assert!(resolve_card(&mut bolt).unwrap());
        let TodoCard::Normal(face) = &bolt else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r)
                if r == "Spell(targets: [AnyTarget], effect: DealDamage(Target(0), 3))"
        ));

        // The same line on a creature is NOT a spell ability: it stays Unparsed.
        let mut creature = TodoCard::Normal(TodoCardFace {
            name: "X".into(),
            types: vec![RawIdent("Creature".into())],
            abilities: vec![TodoAbility::Unparsed(
                "~ deals 3 damage to any target.".into(),
            )],
            ..Default::default()
        });
        assert!(!resolve_card(&mut creature).unwrap());
        let TodoCard::Normal(face) = &creature else { panic!() };
        assert!(matches!(&face.abilities[0], TodoAbility::Unparsed(_)));

        // A Sorcery resolves an untargeted effect to a `Spell` with no targets field.
        let mut divination = TodoCard::Normal(TodoCardFace {
            name: "Divination".into(),
            types: vec![RawIdent("Sorcery".into())],
            abilities: vec![TodoAbility::Unparsed("Draw two cards.".into())],
            ..Default::default()
        });
        assert!(resolve_card(&mut divination).unwrap());
        let TodoCard::Normal(face) = &divination else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r) if r == "Spell(effect: Draw(2))"
        ));
    }

    #[test]
    fn creature_trigger_resolves_through_registry() {
        let mut card = TodoCard::Normal(TodoCardFace {
            name: "Arsonist".into(),
            types: vec![RawIdent("Creature".into())],
            abilities: vec![TodoAbility::Unparsed(
                "When ~ dies, it deals 1 damage to any target.".into(),
            )],
            ..Default::default()
        });
        assert!(resolve_card(&mut card).unwrap());
        let TodoCard::Normal(face) = &card else { panic!() };
        assert!(matches!(
            &face.abilities[0],
            TodoAbility::Parsed(r)
                if r == "Triggered(event: ThisDies, targets: [AnyTarget], effect: DealDamage(Target(0), 1))"
        ));
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
