use std::error::Error;
use std::path::Path;

use deckmaste_lexical_source::{LoadError, load_workspace};

#[test]
fn loader_preserves_filesystem_and_decoding_sources_with_paths() {
    let root = tempfile::tempdir().unwrap();
    let path = root
        .path()
        .join("crates/deckmaste_lexical_source/lexicon/core.ron");
    let error = load_workspace(root.path()).err().unwrap();
    match &error {
        LoadError::Io {
            path: actual,
            source,
            ..
        } => {
            assert_eq!(actual, &path);
            assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
        }
        other => panic!("{other:?}"),
    }
    assert!(error.source().unwrap().is::<std::io::Error>());
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "(lexemes: [").unwrap();
    let error = load_workspace(root.path()).err().unwrap();
    match &error {
        LoadError::Decode {
            path: actual,
            source,
        } => {
            assert_eq!(actual, &path);
            assert!(source.span.start.line > 0);
        }
        other => panic!("{other:?}"),
    }
    assert!(
        error
            .source()
            .unwrap()
            .is::<Box<ron::error::SpannedError>>()
    );
}

#[test]
fn loader_reports_unresolved_paradigm_identities() {
    let root = tempfile::tempdir().unwrap();
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("lexicon");
    let dest = root.path().join("crates/deckmaste_lexical_source/lexicon");
    std::fs::create_dir_all(&dest).unwrap();
    for file in ["core.ron", "vocabulary.rs", "verbs.ron", "overrides.ron"] {
        std::fs::copy(source.join(file), dest.join(file)).unwrap();
    }
    let path = dest.join("core.ron");
    let text = std::fs::read_to_string(&path).unwrap().replace(
        "core_verb_paradigms: {",
        "core_verb_paradigms: {\"core-verb:Missing\": (forms: [], frames: []),",
    );
    std::fs::write(path, text).unwrap();
    match load_workspace(root.path()).err().unwrap() {
        LoadError::UnresolvedParadigms { owners } => assert_eq!(owners, ["core-verb:Missing"]),
        other => panic!("{other:?}"),
    }
}
