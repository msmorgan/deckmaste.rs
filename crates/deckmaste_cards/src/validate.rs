//! Validating a plugin's finished cards and tokens through the macro-aware
//! reader, plus a lint pass over parsed values for shapes that read fine but
//! are always authoring mistakes.
//!
//! **Token walking**: every `tokens/**/*.ron` is read as a [`Token`] with the
//! same macro scope and todo-skipping as cards.
//!
//! **Cost-eligibility lint**: for every parsed Card face and Token, every
//! `CostComponent::Do(action)` must satisfy `PlayerAction::is_cost_eligible()`.
//! Violations surface as [`Validation::lint_failures`] entries with a plain
//! message, separate from the parse-error [`Validation::failures`] vec (which
//! carries [`ron::error::SpannedError`] values that can't be constructed by
//! hand).
//!
//! Earlier lint candidate: degenerate sequences once
//! `Effect::Sequence(Vec<Effect>)` lands.

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use deckmaste_core::Ability;
use deckmaste_core::Card;
use deckmaste_core::CostComponent;
use deckmaste_core::Ident;
use deckmaste_core::Subtype;
use deckmaste_core::Token;
use deckmaste_core::plugin::CARDS_DIR;
use deckmaste_core::plugin::TOKENS_DIR;
use deckmaste_core::plugin::is_todo_file;
use deckmaste_core::plugin::is_todo_source;

use crate::plugin::Plugin;
use crate::plugin::read;
use crate::plugin::ron_files_recursive;

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

/// A finished card that disagrees with canon's reference version of the same
/// name — the same card implemented in two plugins, expanding to different
/// [`Card`] values.
pub struct CanonMismatch {
    /// The card file name shared by both plugins, e.g. `Grizzly Bears.ron`.
    pub file: String,
    /// canon's reference file.
    pub canon_path: PathBuf,
    /// The implementing plugin's file.
    pub plugin_path: PathBuf,
}

/// Checks `plugin_dir`'s finished cards against canon as the authority: for
/// every card canon defines, if `plugin_dir` also finishes a card of that
/// name, the two must expand to the same [`Card`]. Each plugin expands its
/// own copy under its own macro scope, so this catches an implementation that
/// drifted from the reference even when both files parse.
///
/// canon is the sibling directory named `canon`. An absent canon — or
/// `plugin_dir` itself *being* canon — yields no mismatches.
///
/// # Errors
/// If canon or the plugin fails to load, a file isn't readable, or canon's
/// own reference card doesn't parse. A *plugin* card that doesn't parse is
/// left to [`validate_plugin`] to report, not an error here.
pub fn check_against_canon(plugin_dir: &Path) -> anyhow::Result<Vec<CanonMismatch>> {
    let canon_dir = plugin_dir
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .join("canon");
    if !canon_dir.is_dir() {
        return Ok(vec![]);
    }
    // Validating canon itself: there's nothing to compare it against.
    let same_dir = match (canon_dir.canonicalize(), plugin_dir.canonicalize()) {
        (Ok(canon), Ok(plugin)) => canon == plugin,
        _ => false,
    };
    if same_dir {
        return Ok(vec![]);
    }

    let canon = Plugin::load_with_sibling_prelude(&canon_dir)
        .with_context(|| format!(r#"loading canon reference "{}""#, canon_dir.display()))?;
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;

    let mut mismatches = Vec::new();
    for canon_path in ron_files_recursive(&canon_dir.join(CARDS_DIR))? {
        if is_todo_file(&canon_path) {
            continue;
        }
        let canon_source = read(&canon_path)?;
        if is_todo_source(&canon_source) {
            continue;
        }
        let Some(file) = canon_path.file_name().and_then(|f| f.to_str()) else {
            continue;
        };

        // Only cards this plugin has actually finished are in scope.
        let plugin_path = plugin_dir.join(CARDS_DIR).join(file);
        if !plugin_path.is_file() {
            continue;
        }
        let plugin_source = read(&plugin_path)?;
        if is_todo_source(&plugin_source) {
            continue;
        }

        let canon_card: Card = canon
            .macros
            .read_str(&canon_source)
            .with_context(|| format!(r#"parsing canon "{}""#, canon_path.display()))?;
        // A plugin card that won't parse is already a validate_plugin failure.
        let Ok(plugin_card) = plugin.macros.read_str::<Card>(&plugin_source) else {
            continue;
        };
        if canon_card != plugin_card {
            mismatches.push(CanonMismatch {
                file: file.to_owned(),
                canon_path,
                plugin_path,
            });
        }
    }
    Ok(mismatches)
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
                         (only Sacrifice/Exile/Tap/Untap/Discard/LoseLife are cost-eligible)"
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
fn cost_action(component: &CostComponent) -> Option<&deckmaste_core::PlayerAction> {
    match component {
        CostComponent::Do(action) => Some(action),
        CostComponent::Expanded(expansion) => cost_action(&expansion.value),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;
    use std::path::PathBuf;

    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::ActivatedAbility;
    use deckmaste_core::CostComponent;
    use deckmaste_core::Count;
    use deckmaste_core::Effect;
    use deckmaste_core::Expansion;
    use deckmaste_core::ExpansionArgs;
    use deckmaste_core::ManaSpec;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::Restriction;
    use deckmaste_core::Selection;
    use deckmaste_core::StaticAbility;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Subtype;
    use deckmaste_core::Token;
    use deckmaste_core::Type;

    /// `Effect::Act(By(You, AddMana(1, AnyColor)))` — the produced-mana effect
    /// the test tokens carry, in the new player-agent shape.
    fn add_one_any() -> Effect {
        Effect::Act(Action::By(
            Reference::You,
            PlayerAction::AddMana(Count::Literal(1), ManaSpec::AnyColor),
        ))
    }

    use super::check_against_canon;
    use super::lint_card_abilities;
    use super::lint_card_subtypes;

    fn dummy_path() -> PathBuf { PathBuf::from("test/dummy.ron") }

    /// `Draw` in a Do cost is flagged as ineligible.
    #[test]
    fn lint_flags_draw_cards_in_do_cost() {
        let token = Token {
            supertypes: vec![],
            types: vec![Type::Artifact],
            subtypes: vec![],
            abilities: vec![Ability::Activated(ActivatedAbility {
                cost: vec![CostComponent::Do(PlayerAction::Draw(Count::Literal(1)))],
                condition: None,
                limits: vec![],
                targets: vec![],
                effect: add_one_any(),
            })],
        };
        let mut failures = Vec::new();
        lint_card_abilities(&dummy_path(), &token.abilities, &mut failures);
        assert_eq!(failures.len(), 1, "expected exactly one lint failure");
        assert!(
            failures[0].1.contains("Draw"),
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
                    CostComponent::Do(PlayerAction::Sacrifice(Selection::from(Reference::This))),
                ],
                condition: None,
                limits: vec![],
                targets: vec![],
                effect: add_one_any(),
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
                    value: Box::new(CostComponent::Do(PlayerAction::Draw(Count::Literal(1)))),
                })],
                condition: None,
                limits: vec![],
                targets: vec![],
                effect: add_one_any(),
            })],
        };
        let mut failures = Vec::new();
        lint_card_abilities(&dummy_path(), &token.abilities, &mut failures);
        assert_eq!(failures.len(), 1, "expected the inner Draw to be flagged");
        assert!(failures[0].1.contains("Draw"), "{}", failures[0].1);
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

    const FOO_1_1: &str =
        r#"Normal(name: "Foo", mana_cost: [Green], types: [Creature], power: 1, toughness: 1)"#;
    const FOO_2_2: &str =
        r#"Normal(name: "Foo", mana_cost: [Green], types: [Creature], power: 2, toughness: 2)"#;

    fn write_card(root: &Path, plugin: &str, file: &str, source: &str) {
        let cards = root.join(plugin).join("cards");
        std::fs::create_dir_all(&cards).unwrap();
        std::fs::write(cards.join(file), source).unwrap();
    }

    /// Two plugins finishing the same-named card with different values: the
    /// implementation has drifted from canon, so it's a mismatch.
    #[test]
    fn canon_mismatch_detected_when_implementation_drifts() {
        let root = tempfile::tempdir().unwrap();
        write_card(root.path(), "canon", "Foo.ron", FOO_1_1);
        write_card(root.path(), "wizards", "Foo.ron", FOO_2_2);
        let mismatches = check_against_canon(&root.path().join("wizards")).unwrap();
        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].file, "Foo.ron");
    }

    /// Same name, same value: the implementation matches canon.
    #[test]
    fn canon_match_when_implementation_agrees() {
        let root = tempfile::tempdir().unwrap();
        write_card(root.path(), "canon", "Foo.ron", FOO_1_1);
        write_card(root.path(), "wizards", "Foo.ron", FOO_1_1);
        assert!(
            check_against_canon(&root.path().join("wizards"))
                .unwrap()
                .is_empty()
        );
    }

    /// A canon card the plugin hasn't finished — only a todo stub, with an
    /// unrelated finished card alongside — is not compared.
    #[test]
    fn canon_skips_cards_the_plugin_has_not_implemented() {
        let root = tempfile::tempdir().unwrap();
        write_card(root.path(), "canon", "Foo.ron", FOO_1_1);
        write_card(
            root.path(),
            "wizards",
            "Foo.todo.ron",
            r#"Todo(layout: "normal")"#,
        );
        write_card(root.path(), "wizards", "Bar.ron", FOO_2_2);
        assert!(
            check_against_canon(&root.path().join("wizards"))
                .unwrap()
                .is_empty()
        );
    }

    /// Validating canon against itself compares nothing.
    #[test]
    fn canon_checked_against_itself_is_empty() {
        let root = tempfile::tempdir().unwrap();
        write_card(root.path(), "canon", "Foo.ron", FOO_1_1);
        assert!(
            check_against_canon(&root.path().join("canon"))
                .unwrap()
                .is_empty()
        );
    }
}
