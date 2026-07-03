//! The elaborator's accept fixtures: the "twin" of `tests/reject/` — for
//! every reject fixture's rule, a minimal card/token exercising the SAME
//! construct bound correctly, so it elaborates clean. These are what
//! `cargo xtask elaborate --dump` is exercised against
//! ([[cards-elab-load-gate]]'s `--dump` coverage requirement): a fixture
//! whose construct is one of the binding/capability/cost-declaration codes
//! must resolve at least one binding when dumped.
//!
//! Layout mirrors `tests/reject/`: the DIRECTORY is the code the fixture's
//! construct is associated with; every `*.ron` inside is one fixture (a
//! `Card`, or a `Token` when named `*.token.ron`).

use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_cards::elaborate;
use deckmaste_cards::elaborate::Code;
use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Token;

fn builtin() -> Plugin {
    Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
}

fn accept_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/accept")
}

/// Every accept fixture parses AND elaborates clean; a fixture that fails
/// either is a test failure. Also collects, per fixture, whether `--dump`
/// (`elaborate_with_resolutions`) found at least one resolved binding —
/// asserted separately below, since the pure well-formedness floors
/// (`E-FLOOR-*`, `E-KIND-FILTER`, `E-COST-INELIGIBLE`) have nothing to
/// resolve.
fn accept_fixture_resolution_count(plugin: &Plugin, path: &Path) -> usize {
    let source = std::fs::read_to_string(path).unwrap();
    let registries = plugin.registries();
    let is_token = path
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| f.ends_with(".token.ron"));
    if is_token {
        let token: Token = plugin
            .macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("fixture {} must parse: {e}", path.display()));
        let (result, resolutions) =
            elaborate::elaborate_token_with_resolutions(&token, &registries);
        result.unwrap_or_else(|errors| {
            panic!(
                "accept fixture {} must elaborate clean, got: {errors:?}",
                path.display()
            )
        });
        resolutions.len()
    } else {
        let card: Card = plugin
            .macros
            .read_str(&source)
            .unwrap_or_else(|e| panic!("fixture {} must parse: {e}", path.display()));
        let (result, resolutions) = elaborate::elaborate_with_resolutions(&card, &registries);
        result.unwrap_or_else(|errors| {
            panic!(
                "accept fixture {} must elaborate clean, got: {errors:?}",
                path.display()
            )
        });
        resolutions.len()
    }
}

/// Every accept fixture elaborates clean; every active code has at least
/// one twin; every binding/capability/cost-`{X}` code's twin resolves at
/// least one binding (`--dump` has something to show).
#[test]
fn every_accept_fixture_elaborates_clean_and_covers_its_code() {
    let plugin = builtin();
    let mut covered: BTreeSet<String> = BTreeSet::new();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(accept_root())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    dirs.sort();
    for dir in dirs {
        let code_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
        let code = *Code::ALL
            .iter()
            .find(|c| c.as_str() == code_name)
            .unwrap_or_else(|| panic!("accept/{code_name} names no manifest code"));
        let mut fixtures: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "ron"))
            .collect();
        fixtures.sort();
        assert!(!fixtures.is_empty(), "accept/{code_name} has no fixtures");
        let mut resolved_any = false;
        for fixture in &fixtures {
            if accept_fixture_resolution_count(&plugin, fixture) > 0 {
                resolved_any = true;
            }
        }
        // The structural floors/kind-filter/cost-eligibility codes have
        // nothing to bind — everything else (binding, capability, {X}) is
        // exactly the domain `--dump` reports on and must show something.
        let is_binding_code = !matches!(
            code,
            Code::FloorTypes
                | Code::FloorSubtype
                | Code::FloorLoyalty
                | Code::FloorDefense
                | Code::FloorTokenTypes
                | Code::FloorRange
                | Code::FloorTargetQty
                | Code::FloorBlockQty
                | Code::FloorModalCount
                | Code::FloorModalEmpty
                | Code::FloorDivide
                | Code::FloorDestination
                | Code::CostIneligible
                | Code::KindFilter
                // Class discipline — a marked Prevention binds nothing.
                | Code::PosPrevention
        );
        assert!(
            !is_binding_code || resolved_any,
            "accept/{code_name}: expected --dump to resolve at least one binding"
        );
        covered.insert(code.as_str().to_owned());
    }
    for code in Code::ALL {
        assert!(
            covered.contains(code.as_str()),
            "no accept fixture for {code}"
        );
    }
}
