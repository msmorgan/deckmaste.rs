//! Validating a plugin's finished cards and tokens through the macro-aware
//! reader, plus a lint pass over parsed values for shapes that read fine but
//! are always semantic-input errors.
//!
//! **Token walking**: every `tokens/**/*.ron` is read as a
//! semantic [`deckmaste_semantics::Token`] with the same macro scope and
//! todo-skipping as cards, then lowered to [`deckmaste_core::Token`].

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use deckmaste_card::Card;
use deckmaste_core::Ability;
use deckmaste_core::Ident;
use deckmaste_core::Subtype;
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
    /// The read failure as the restricted API reports it — no context is
    /// added, so this still Displays as the underlying
    /// [`ron::error::SpannedError`].
    pub error: anyhow::Error,
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
    /// Parsed-value semantic findings, such as undeclared types/subtypes or
    /// unknown keyword references, as `(path, message)` pairs.
    pub lint_failures: Vec<(PathBuf, String)>,
}

/// Reads every non-todo `cards/**/*.ron` and `tokens/**/*.ron` in the plugin
/// — builtin sibling prelude in scope — as a [`Card`] /
/// [`deckmaste_core::Token`] respectively, collecting failures instead of
/// stopping at the first. Parsed cards are then checked for declarations and
/// keyword-reference integrity.
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
        match plugin.card_from_str(&source) {
            Ok(card) => {
                lint_all_card_faces(
                    &path,
                    &card.core,
                    &plugin.subtypes,
                    &plugin.types,
                    &plugin.macros,
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
        match plugin.token_from_str(&source) {
            Ok(_) => validation.valid += 1,
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

        let canon_card = canon
            .card_from_str(&canon_source)
            .with_context(|| format!(r#"parsing canon "{}""#, canon_path.display()))?
            .core;
        // A plugin card that won't parse is already a validate_plugin failure.
        let Ok(plugin_card) = plugin.card_from_str(&plugin_source) else {
            continue;
        };
        if canon_card != plugin_card.core {
            mismatches.push(CanonMismatch {
                file: file.to_owned(),
                canon_path,
                plugin_path,
            });
        }
    }
    Ok(mismatches)
}

/// Lint all abilities, subtypes, and types across every face of a card.
fn lint_all_card_faces(
    path: &Path,
    card: &Card,
    declared_subtypes: &HashMap<Ident, Subtype>,
    declared_types: &HashMap<Ident, deckmaste_core::TypeDef>,
    macros: &macro_ron::MacroSet,
    out: &mut Vec<(PathBuf, String)>,
) {
    let lint_one = |chars: &deckmaste_card::Characteristics, out: &mut Vec<(PathBuf, String)>| {
        lint_card_subtypes(path, &chars.subtypes, declared_subtypes, out);
        lint_card_types(path, &chars.types, declared_types, out);
        lint_keyword_refs(path, &chars.abilities, macros, out);
    };
    match card {
        Card::Normal(face) => lint_one(&face.characteristics, out),
        Card::DoubleFaced { front, back, .. }
        | Card::Split {
            left: front,
            right: back,
        } => {
            lint_one(&front.characteristics, out);
            lint_one(&back.characteristics, out);
        }
        Card::Flip {
            normal,
            alternative,
        }
        | Card::Adventurer {
            normal,
            adventure: alternative,
        } => {
            lint_one(&normal.characteristics, out);
            lint_one(alternative, out);
        }
    }
}

/// Keyword-reference names must exist in the keyword namespace — the native
/// `KeywordAbility` enum or a `KeywordAbility`-kind macro. A bare-ident
/// reference like `Has(Flyng)` PARSES (names are a lint, not a parse
/// concern) but asserts a keyword that doesn't exist and silently never
/// matches; that is the "asserted nonsense" this lint exists for. Benign
/// extras are deliberately ignored.
///
/// Traversal rides the SERIALIZER rather than a hand-written AST visitor:
/// canonical re-serialization is the one total walk the grammar maintains
/// for free (a hand visitor silently misses every future variant), and
/// `Has(Name)` is the unit-variant spelling the writer emits.
fn lint_keyword_refs(
    path: &Path,
    abilities: &[Ability],
    macros: &macro_ron::MacroSet,
    out: &mut Vec<(PathBuf, String)>,
) {
    for ability in abilities {
        let Ok(ron) = deckmaste_core::ron::options().to_string(ability) else {
            continue;
        };
        let mut rest = ron.as_str();
        while let Some(i) = rest.find("Has(") {
            rest = &rest[i + 4..];
            let end = rest.find(')').unwrap_or(rest.len());
            let name = &rest[..end];
            let known = deckmaste_core::KeywordAbility::ALL
                .iter()
                .any(|k| k.as_str() == name)
                || macros.get("KeywordAbility", name).is_some();
            if !known {
                out.push((
                    path.to_owned(),
                    format!(
                        "unknown-keyword-reference: Has({name}) names no native keyword or \
                         KeywordAbility-kind macro"
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
            // Compare the FIELDS explicitly, not via `PartialEq` — `Subtype`'s
            // eq is by-name (a declared name always resolves to its one macro),
            // so a same-name value carrying drifted `types`/`confers` compares
            // equal and only a field check catches the drift this lint exists for.
            Some(declared)
                if declared.types == subtype.types && declared.confers == subtype.confers => {}
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

/// For each printed type on a face, check its name resolves in the plugin's
/// declared type registry. The Rust-side analogue of Idris's structural type
/// check; a typo lints here and fizzles at runtime (`resolve_type` → name-only
/// `TypeDef`), never crashing.
fn lint_card_types(
    path: &Path,
    types: &[deckmaste_core::TypeDef],
    declared_types: &HashMap<Ident, deckmaste_core::TypeDef>,
    out: &mut Vec<(PathBuf, String)>,
) {
    for ty in types {
        if !declared_types.contains_key(&ty.name) {
            out.push((
                path.to_owned(),
                format!(
                    "undeclared-type: {:?} names no declared card type",
                    ty.name.as_str()
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;
    use std::path::PathBuf;

    use deckmaste_core::Action;
    use deckmaste_core::CostComponent;
    use deckmaste_core::Ident;
    use deckmaste_core::Reference;
    use deckmaste_core::Subtype;
    use deckmaste_core::Type;
    use deckmaste_core::TypeDef;

    use super::check_against_canon;
    use super::lint_card_subtypes;
    use super::lint_card_types;

    fn dummy_path() -> PathBuf {
        PathBuf::from("test/dummy.ron")
    }

    /// A non-cost-eligible action cannot be assembled into core card data.
    #[test]
    fn core_boundary_rejects_ineligible_action_before_plugin_lint() {
        assert_eq!(
            CostComponent::try_do_action(Action::DrawCard(Reference::Reg(deckmaste_core::RefId(
                1
            )))),
            Err(deckmaste_core::RunnableCostActionError::Ineligible),
        );
    }

    /// The same invariant is checked while reading untrusted core RON.
    #[test]
    fn core_deserialization_rejects_ineligible_action_cost() {
        let parsed: Result<CostComponent, _> =
            deckmaste_core::ron::options().from_str("Act(action: DrawCard(Reg(1)))");
        assert!(
            parsed
                .unwrap_err()
                .to_string()
                .contains("not eligible to be performed as a cost")
        );
    }

    /// A face with a subtype whose name is not in the declared set produces a
    /// `bare-subtype-in-card` finding.
    #[test]
    fn lint_flags_undeclared_subtype() {
        let undeclared = Subtype {
            name: "Undeclared".into(),
            types: vec![Type::Land].into(),
            confers: vec![].into(),
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
            types: vec![Type::Land].into(),
            confers: vec![].into(),
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
            types: vec![Type::Land].into(),
            confers: vec![].into(),
        };
        let drifted = Subtype {
            name: "Forest".into(),
            types: vec![Type::Creature].into(),
            confers: vec![].into(),
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

    /// A face with a type whose name is not in the declared set produces an
    /// `undeclared-type` finding.
    #[test]
    fn lint_flags_undeclared_type() {
        let undeclared = TypeDef {
            name: "Bogus".into(),
            permanent_type: true,
            confers: vec![].into(),
        };
        let declared: HashMap<Ident, TypeDef> = HashMap::new();
        let mut failures = Vec::new();
        lint_card_types(&dummy_path(), &[undeclared], &declared, &mut failures);
        assert_eq!(failures.len(), 1, "expected exactly one lint failure");
        assert!(
            failures[0].1.contains("undeclared-type"),
            "message should contain the lint name: {}",
            failures[0].1
        );
        assert!(
            failures[0].1.contains("Bogus"),
            "message should mention the type name: {}",
            failures[0].1
        );
    }

    /// A face whose type matches a declared canonical type produces no
    /// findings.
    #[test]
    fn lint_passes_canonical_type() {
        let creature = Type::Creature.def();
        let declared: HashMap<Ident, TypeDef> = [("Creature".into(), creature.clone())]
            .into_iter()
            .collect();
        let mut failures = Vec::new();
        lint_card_types(&dummy_path(), &[creature], &declared, &mut failures);
        assert!(
            failures.is_empty(),
            "declared canonical type should not be flagged"
        );
    }

    // Types are `Vec<TypeDef>`; the bare-name `Creature` form expands via the
    // builtin `TypeDef` macros, which these self-contained tempdir plugins do
    // not load, so the fixtures spell the expanded struct inline.
    const FOO_1_1: &str = r#"Normal(name: "Foo", mana_cost: [Green], types: [TypeDef(name: "Creature", permanent: true)], power: 1, toughness: 1)"#;
    const FOO_2_2: &str = r#"Normal(name: "Foo", mana_cost: [Green], types: [TypeDef(name: "Creature", permanent: true)], power: 2, toughness: 2)"#;

    fn write_card(root: &Path, plugin: &str, file: &str, source: &str) {
        let cards = root.join(plugin).join("cards");
        std::fs::create_dir_all(&cards).unwrap();
        std::fs::write(cards.join(file), source).unwrap();
        write_card_root_prelude(root);
    }

    /// A card is read as restricted author vocabulary (spec §4), so every
    /// spelling in these fixtures needs its identity macro in scope — the card
    /// root `Normal`, the mana-cost `Green`, and `Number`, which the bare
    /// numeral `power: 1` splices to. Installed as a sibling
    /// `builtin/`, which is what `load_with_sibling_prelude` looks for, and
    /// COPIED from the real defs rather than restated so they cannot drift.
    fn write_card_root_prelude(root: &Path) {
        let macros = root.join("builtin").join("macros");
        std::fs::create_dir_all(&macros).unwrap();
        for def in ["Normal", "Green", "Number"] {
            std::fs::copy(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../plugins/builtin/macros/identity")
                    .join(format!("{def}.ron")),
                macros.join(format!("{def}.ron")),
            )
            .unwrap();
        }
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

#[cfg(test)]
mod keyword_ref_tests {
    use std::path::Path;
    use std::path::PathBuf;

    use deckmaste_core::Ability;
    use deckmaste_lowering::Lower;

    use crate::plugin::Plugin;

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    /// `Has(Flyng)` is asserted nonsense (flags); `Has(Flying)` (macro) and
    /// `Has(Trample)` (native) are known names (clean).
    #[test]
    fn unknown_keyword_reference_is_flagged() {
        let plugin = builtin();
        let read = |src: &str| -> Ability {
            let semantic: deckmaste_semantics::Ability = plugin.macros.read_str(src).unwrap();
            semantic.lower()
        };
        let typo = read("Static(Cant(Block(on: Ref(This), by: Not(Has(Flyng)))))");
        let fine =
            read("Static(Cant(Block(on: Ref(This), by: Not(Or([Has(Flying), Has(Trample)])))))");
        let mut out = Vec::new();
        super::lint_keyword_refs(
            &PathBuf::from("test/dummy.ron"),
            &[fine],
            &plugin.macros,
            &mut out,
        );
        assert!(out.is_empty(), "known names are clean: {out:?}");
        super::lint_keyword_refs(
            &PathBuf::from("test/dummy.ron"),
            &[typo],
            &plugin.macros,
            &mut out,
        );
        assert_eq!(out.len(), 1, "the typo is flagged once");
        assert!(
            out[0].1.contains("Has(Flyng)"),
            "message names the typo: {}",
            out[0].1
        );
    }
}
