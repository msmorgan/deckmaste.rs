//! Validating a plugin's finished cards and tokens through the macro-aware
//! reader, plus a lint pass over parsed values for shapes that read fine but
//! are always authoring mistakes.
//!
//! **Token walking**: every `tokens/**/*.ron` is read as a [`Token`] with the
//! same macro scope and todo-skipping as cards.
//!
//! **Cost-eligibility lint**: for every parsed Card face and Token, every
//! `CostComponent::Do(action)` must satisfy `Action::is_cost_eligible()`.
//! Violations surface as [`Validation::lint_failures`] entries with a plain
//! message, separate from the parse-error [`Validation::failures`] vec (which
//! carries [`ron::error::SpannedError`] values that can't be constructed by
//! hand).
//!
//! Earlier lint candidate: degenerate sequences once
//! `Effect::Sequence(Vec<Effect>)` lands.

use std::path::{Path, PathBuf};

use deckmaste_core::plugin::{CARDS_DIR, TOKENS_DIR, is_todo_source};
use deckmaste_core::{Ability, Card, CostComponent, Token};

use crate::plugin::{Plugin, read, ron_files_recursive};

/// A card or token file that failed to parse.
pub struct InvalidCard {
    pub path: PathBuf,
    pub error: ron::error::SpannedError,
}

/// What a validation pass saw: todos are skipped, everything else either
/// parsed (`valid`) or landed in `failures` or `lint_failures`.
pub struct Validation {
    /// Files that parsed successfully (cards + tokens combined).
    pub valid: usize,
    /// Todo stubs skipped (cards + tokens combined).
    pub todos: usize,
    /// Files that failed to parse as the expected type.
    pub failures: Vec<InvalidCard>,
    /// Cost-eligibility violations: `(path, message)` for every
    /// `CostComponent::Do(action)` where `!action.is_cost_eligible()`.
    pub lint_failures: Vec<(PathBuf, String)>,
}

/// Reads every non-todo `cards/**/*.ron` and `tokens/**/*.ron` in the plugin
/// — builtin sibling prelude in scope — as a [`Card`] / [`Token`]
/// respectively, collecting failures instead of stopping at the first. After
/// parsing, each value is linted for cost-eligibility (see module doc).
///
/// # Errors
/// If the plugin (or its prelude) fails to load, or a file isn't readable.
/// Files that read but don't parse are `failures`, not errors.
pub fn validate_plugin(plugin_dir: &Path) -> anyhow::Result<Validation> {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;
    let mut validation = Validation {
        valid: 0,
        todos: 0,
        failures: Vec::new(),
        lint_failures: Vec::new(),
    };

    // --- cards ---
    for path in ron_files_recursive(&plugin_dir.join(CARDS_DIR))? {
        let source = read(&path)?;
        if is_todo_source(&source) {
            validation.todos += 1;
            continue;
        }
        match plugin.macros.read_str::<Card>(&source) {
            Ok(card) => {
                lint_all_card_faces(&path, &card, &mut validation.lint_failures);
                validation.valid += 1;
            }
            Err(error) => validation.failures.push(InvalidCard { path, error }),
        }
    }

    // --- tokens ---
    for path in ron_files_recursive(&plugin_dir.join(TOKENS_DIR))? {
        let source = read(&path)?;
        if is_todo_source(&source) {
            validation.todos += 1;
            continue;
        }
        match plugin.macros.read_str::<Token>(&source) {
            Ok(token) => {
                lint_card_abilities(&path, &token.abilities, &mut validation.lint_failures);
                validation.valid += 1;
            }
            Err(error) => validation.failures.push(InvalidCard { path, error }),
        }
    }

    Ok(validation)
}

/// Lint all abilities across every face of a card.
fn lint_all_card_faces(path: &Path, card: &Card, out: &mut Vec<(PathBuf, String)>) {
    match card {
        Card::Normal(face) => lint_card_abilities(path, &face.abilities, out),
        Card::ModalDfc(front, back) => {
            lint_card_abilities(path, &front.abilities, out);
            lint_card_abilities(path, &back.abilities, out);
        }
    }
}

/// For each `Activated` ability, check every `Do(action)` cost component;
/// push a message if `!action.is_cost_eligible()`.
fn lint_card_abilities(path: &Path, abilities: &[Ability], out: &mut Vec<(PathBuf, String)>) {
    for ability in abilities {
        let Ability::Activated(activated) = ability else {
            continue;
        };
        for component in &activated.cost {
            let CostComponent::Do(action) = component else {
                continue;
            };
            if !action.is_cost_eligible() {
                out.push((
                    path.to_owned(),
                    format!(
                        "cost-ineligible action in Do(…): {action:?} is not allowed as a cost \
                         (only Sacrifice is cost-eligible)"
                    ),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use deckmaste_core::{
        Ability, Action, ActivatedAbility, CostComponent, Effect, ManaSpec, Quantity, Reference,
        Restriction, Selection, StaticAbility, StaticEffect, Token, Type,
    };

    use super::lint_card_abilities;

    fn dummy_path() -> PathBuf { PathBuf::from("test/dummy.ron") }

    /// `DrawCards` in a Do cost is flagged as ineligible.
    #[test]
    fn lint_flags_draw_cards_in_do_cost() {
        let token = Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![Ability::Activated(ActivatedAbility {
                cost: vec![CostComponent::Do(Action::DrawCards(Quantity::Literal(1)))],
                targets: vec![],
                effect: Effect::Act(Action::AddMana(Quantity::Literal(1), ManaSpec::AnyColor)),
            })],
        };
        let mut failures = Vec::new();
        lint_card_abilities(&dummy_path(), &token.abilities, &mut failures);
        assert_eq!(failures.len(), 1, "expected exactly one lint failure");
        assert!(
            failures[0].1.contains("DrawCards"),
            "message should mention the action: {}",
            failures[0].1
        );
    }

    /// `Sacrifice` in a Do cost is allowed (it is cost-eligible).
    #[test]
    fn lint_allows_sacrifice_in_do_cost() {
        let token = Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![Ability::Activated(ActivatedAbility {
                cost: vec![
                    CostComponent::Tap,
                    CostComponent::Do(Action::Sacrifice(Selection::from(Reference::This))),
                ],
                targets: vec![],
                effect: Effect::Act(Action::AddMana(Quantity::Literal(1), ManaSpec::AnyColor)),
            })],
        };
        let mut failures = Vec::new();
        lint_card_abilities(&dummy_path(), &token.abilities, &mut failures);
        assert!(failures.is_empty(), "Sacrifice should not be flagged");
    }

    /// Non-activated abilities are ignored by the lint.
    #[test]
    fn lint_ignores_non_activated_abilities() {
        let token = Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![Ability::Static(StaticAbility {
                condition: None,
                effects: vec![StaticEffect::Restriction(Restriction::CantAttack)],
                characteristic_defining: false,
            })],
        };
        let mut failures = Vec::new();
        lint_card_abilities(&dummy_path(), &token.abilities, &mut failures);
        assert!(failures.is_empty());
    }
}
