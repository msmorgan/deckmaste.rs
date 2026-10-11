use std::path::Path;

use deckmaste_macro_def::DerivedSurface;
use deckmaste_macro_def::GrammarRecipe;
use deckmaste_macro_def::ReadError;
use deckmaste_macro_def::ValidationError;
use deckmaste_macro_def::read_builtin_v2;
use deckmaste_macro_def::read_builtin_v2_helpers;

#[test]
fn helpers_reuse_normalized_terms_and_leave_fact_declarations_unchanged() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v2/builtin");
    let helpers = read_builtin_v2_helpers(&root).unwrap();
    for (name, kind, recipe, surface) in [
        (
            "historic",
            "predicate",
            GrammarRecipe::Adjective,
            "historic",
        ),
        ("outlaw", "predicate", GrammarRecipe::Noun, "outlaw"),
        ("commitsCrime", "event", GrammarRecipe::Noun, "crime"),
    ] {
        let declaration = helpers.iter().find(|row| row.name == name).unwrap();
        assert_eq!(declaration.kind, kind);
        let term = declaration.term.as_ref().unwrap();
        assert_eq!(term.recipe(), &recipe);
        assert_eq!(term.surfaces()[0].text(), surface);
        assert!(declaration.path.starts_with(&root));
    }
    let event = helpers
        .iter()
        .find(|row| row.name == "commitsCrime")
        .unwrap();
    let verb = event.grammar.verb.as_ref().unwrap();
    assert_eq!(verb.bare, "commit");
    assert_eq!(
        verb.participle,
        DerivedSurface::Override("committed".into())
    );
    assert_eq!(verb.preterite, DerivedSurface::Unavailable);
    assert_eq!(verb.gerund_participle, DerivedSurface::Unavailable);
    let facts = read_builtin_v2(root).unwrap();
    assert!(
        facts
            .iter()
            .all(|row| !["historic", "outlaw", "commitsCrime"].contains(&row.identity().name()))
    );
}

fn nursery(source: &str) -> tempfile::TempDir {
    let temporary = tempfile::tempdir().unwrap();
    for family in ["predicates", "events"] {
        std::fs::create_dir_all(temporary.path().join("builtin/macros").join(family)).unwrap();
    }
    std::fs::write(
        temporary
            .path()
            .join("builtin/macros/events/commitsCrime.ron"),
        source,
    )
    .unwrap();
    temporary
}

#[test]
fn undeclared_helpers_with_named_parameters_export_nothing() {
    let root = nursery(
        r#"(name: "commitsCrime", kinds: [GameEvent], params: {"player": Subject}, body: CommitsCrime(Param(player)))"#,
    );
    assert!(
        read_builtin_v2_helpers(root.path().join("builtin"))
            .unwrap()
            .is_empty()
    );
}

#[test]
fn helper_recipes_use_existing_surface_and_frame_validation() {
    for (grammar, expected) in [
        (
            r#"(term: Adjective(surface: ""))"#,
            ValidationError::InvalidSurface { field: "surface" },
        ),
        (
            r#"(verb: (bare: "commit", frames: []))"#,
            ValidationError::EmptyCustomVerbFrameSet,
        ),
        (
            r#"(term: Adjective(surface: "historic"), noun_modifier: true)"#,
            ValidationError::TypeWordGrammarMismatch,
        ),
    ] {
        let root = nursery(&format!(
            r#"(name: "commitsCrime", kinds: [GameEvent], params: {{"player": Subject}}, metadata: (grammar: {grammar}), body: CommitsCrime(Param(player)))"#
        ));
        let error = read_builtin_v2_helpers(root.path().join("builtin")).unwrap_err();
        let ReadError::Validate { source, .. } = error else { panic!("{error}") };
        assert_eq!(source, expected);
    }
}
