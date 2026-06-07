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

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use deckmaste_core::plugin::{CARDS_DIR, TOKENS_DIR, is_todo_source};
use deckmaste_core::{Ability, Card, CostComponent, Ident, Subtype, Token};

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
                lint_all_card_faces(
                    &path,
                    &card,
                    &plugin.subtypes,
                    &mut validation.lint_failures,
                );
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

/// Lint all abilities and subtypes across every face of a card.
fn lint_all_card_faces(
    path: &Path,
    card: &Card,
    declared_subtypes: &HashMap<Ident, Subtype>,
    out: &mut Vec<(PathBuf, String)>,
) {
    match card {
        Card::Normal(face) => {
            lint_card_abilities(path, &face.abilities, out);
            lint_card_subtypes(path, &face.subtypes, declared_subtypes, out);
        }
        Card::ModalDfc(front, back) => {
            lint_card_abilities(path, &front.abilities, out);
            lint_card_subtypes(path, &front.subtypes, declared_subtypes, out);
            lint_card_abilities(path, &back.abilities, out);
            lint_card_subtypes(path, &back.subtypes, declared_subtypes, out);
        }
    }
}

/// For each `Activated` ability, check every `Do(action)` cost component;
/// push a message if `!action.is_cost_eligible()`. A remembered cost macro
/// (`CostComponent::Expanded`) is looked through to the `Do` it expanded to,
/// so `SacrificeThis` and friends stay validated.
fn lint_card_abilities(path: &Path, abilities: &[Ability], out: &mut Vec<(PathBuf, String)>) {
    for ability in abilities {
        let Ability::Activated(activated) = ability else {
            continue;
        };
        for component in &activated.cost {
            let Some(action) = cost_action(component) else {
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

/// For each subtype on a face, check that it equals the plugin's declaration
/// of that name. Post-expansion a bare reference and an inline literal are
/// indistinguishable — the lint enforces declaration provenance, catching
/// undeclared names and inline literals that drift from declarations.
fn lint_card_subtypes(
    path: &Path,
    subtypes: &[Subtype],
    declared_subtypes: &HashMap<Ident, Subtype>,
    out: &mut Vec<(PathBuf, String)>,
) {
    for subtype in subtypes {
        match declared_subtypes.get(&subtype.name) {
            Some(declared) if declared == subtype => {}
            Some(declared) => out.push((
                path.to_owned(),
                format!(
                    "bare-subtype-in-card: {:?} is declared but its value differs from the \
                     declaration (expected {declared:?})",
                    subtype.name.as_str()
                ),
            )),
            None => out.push((
                path.to_owned(),
                format!(
                    "bare-subtype-in-card: {:?} does not match any declared subtype",
                    subtype.name.as_str()
                ),
            )),
        }
    }
}

/// The `Do(action)` a cost component reduces to, looking through any
/// remembered macro invocation (`CostComponent::Expanded`). `None` for
/// non-`Do` components (mana, tap, untap).
fn cost_action(component: &CostComponent) -> Option<&deckmaste_core::Action> {
    match component {
        CostComponent::Do(action) => Some(action),
        CostComponent::Expanded(expansion) => cost_action(&expansion.value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use deckmaste_core::{
        Ability, Action, ActivatedAbility, CostComponent, Effect, Expansion, ExpansionArgs,
        ManaSpec, Quantity, Reference, Restriction, Selection, StaticAbility, StaticEffect,
        Subtype, Token, Type,
    };

    use super::{lint_card_abilities, lint_card_subtypes};

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

    /// An ineligible action hidden inside a remembered cost macro
    /// (`CostComponent::Expanded`) is still flagged: the lint looks through
    /// the invocation to the `Do` it expanded to.
    #[test]
    fn lint_looks_through_expanded_cost_macros() {
        let token = Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![Ability::Activated(ActivatedAbility {
                cost: vec![CostComponent::Expanded(Expansion {
                    name: "BadCost".into(),
                    args: ExpansionArgs::none(),
                    value: Box::new(CostComponent::Do(Action::DrawCards(Quantity::Literal(1)))),
                })],
                targets: vec![],
                effect: Effect::Act(Action::AddMana(Quantity::Literal(1), ManaSpec::AnyColor)),
            })],
        };
        let mut failures = Vec::new();
        lint_card_abilities(&dummy_path(), &token.abilities, &mut failures);
        assert_eq!(
            failures.len(),
            1,
            "expected the inner DrawCards to be flagged"
        );
        assert!(failures[0].1.contains("DrawCards"), "{}", failures[0].1);
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

    /// A face with a subtype whose name is not in the declared set produces a
    /// `bare-subtype-in-card` finding.
    #[test]
    fn lint_flags_undeclared_subtype() {
        let undeclared = Subtype {
            name: "Undeclared".into(),
            types: vec![Type::Land],
            confers: vec![],
        };
        let declared: HashMap<_, _> = HashMap::new();
        let mut failures = Vec::new();
        lint_card_subtypes(&dummy_path(), &[undeclared], &declared, &mut failures);
        assert_eq!(failures.len(), 1, "expected exactly one lint failure");
        assert!(
            failures[0].1.contains("bare-subtype-in-card"),
            "message should contain the lint name: {}",
            failures[0].1
        );
        assert!(
            failures[0].1.contains("Undeclared"),
            "message should mention the subtype name: {}",
            failures[0].1
        );
    }

    /// A face whose subtypes match declarations exactly produces no findings.
    #[test]
    fn lint_passes_declared_matching_subtype() {
        let declared_subtype = Subtype {
            name: "Forest".into(),
            types: vec![Type::Land],
            confers: vec![],
        };
        let declared: HashMap<_, Subtype> = [("Forest".into(), declared_subtype.clone())]
            .into_iter()
            .collect();
        let mut failures = Vec::new();
        lint_card_subtypes(&dummy_path(), &[declared_subtype], &declared, &mut failures);
        assert!(
            failures.is_empty(),
            "matching declaration should not be flagged"
        );
    }

    /// A name that IS declared, carried with a different value: the case the
    /// lint exists for — post-expansion it's indistinguishable from a bare
    /// reference, so only equality against the declaration catches it.
    #[test]
    fn lint_flags_declared_name_with_drifted_value() {
        let declared_subtype = Subtype {
            name: "Forest".into(),
            types: vec![Type::Land],
            confers: vec![],
        };
        let drifted = Subtype {
            name: "Forest".into(),
            types: vec![Type::Creature],
            confers: vec![],
        };
        let declared: HashMap<_, Subtype> =
            [("Forest".into(), declared_subtype)].into_iter().collect();
        let mut failures = Vec::new();
        lint_card_subtypes(&dummy_path(), &[drifted], &declared, &mut failures);
        assert_eq!(failures.len(), 1, "expected exactly one lint failure");
        assert!(
            failures[0].1.contains("bare-subtype-in-card"),
            "message should contain the lint name: {}",
            failures[0].1
        );
        assert!(
            failures[0].1.contains("differs"),
            "message should mention drift: {}",
            failures[0].1
        );
    }
}
