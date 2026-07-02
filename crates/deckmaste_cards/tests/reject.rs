//! The elaborator's reject fixtures: every active checker rule has at least
//! one minimal RON fixture under `tests/reject/<E-CODE>/` that must fail with
//! EXACTLY that code — the Rust twins of `idris/src/Spec.idr`'s `failing`
//! blocks (each fixture's header comment names its twin where one exists).
//!
//! Layout: the DIRECTORY is the expected error code; every `*.ron` inside is
//! one fixture (a `Card`, or a `Token` when named `*.token.ron`). Fixtures
//! parse under the builtin macro scope — parsing is not the gate here, the
//! elaboration walk is.

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::elaborate;
use deckmaste_cards::elaborate::Code;
use deckmaste_cards::elaborate::ElabError;
use deckmaste_cards::elaborate::Registries;
use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Token;

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn reject_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/reject")
}

/// The fixture's elaboration errors — a fixture that fails to PARSE or
/// elaborates CLEAN is a test failure, not an empty list.
fn elaborate_fixture(plugin: &Plugin, path: &Path) -> Vec<ElabError> {
    let source = std::fs::read_to_string(path).unwrap();
    let registries = Registries {
        subtypes: &plugin.subtypes,
        counters: &plugin.counters,
    };
    let is_token = path
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| f.ends_with(".token.ron"));
    if is_token {
        let token: Token = plugin
            .macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("fixture {} must parse: {e}", path.display()));
        elaborate::elaborate_token(&token, &registries)
            .err()
            .unwrap_or_else(|| panic!("fixture {} elaborated clean", path.display()))
    } else {
        let card: Card = plugin
            .macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("fixture {} must parse: {e}", path.display()));
        match elaborate::elaborate(&card, &registries) {
            Ok(_) => panic!("fixture {} elaborated clean", path.display()),
            Err(errors) => errors,
        }
    }
}

/// Every fixture fails with exactly its directory's code (all findings carry
/// it — a fixture tripping a SECOND rule is not minimal), and every active
/// code has at least one fixture.
#[test]
fn every_code_has_a_fixture_and_every_fixture_fails_with_exactly_its_code() {
    let plugin = builtin();
    let mut covered: BTreeSet<&'static str> = BTreeSet::new();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(reject_root())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    dirs.sort();
    for dir in dirs {
        let code_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
        let code = *Code::ALL
            .iter()
            .find(|c| c.as_str() == code_name)
            .unwrap_or_else(|| panic!("reject/{code_name} names no manifest code"));
        let mut fixtures: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "ron"))
            .collect();
        fixtures.sort();
        assert!(!fixtures.is_empty(), "reject/{code_name} has no fixtures");
        for fixture in fixtures {
            let errors = elaborate_fixture(&plugin, &fixture);
            for error in &errors {
                assert_eq!(
                    error.code,
                    code,
                    "fixture {} must fail with exactly {code}, got {error}",
                    fixture.display(),
                );
            }
            covered.insert(code.as_str());
        }
    }
    for code in Code::ALL {
        assert!(
            covered.contains(code.as_str()),
            "no reject fixture for {code}"
        );
    }
}
