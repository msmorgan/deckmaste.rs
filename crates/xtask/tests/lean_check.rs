//! `cargo xtask lean-check` end to end: the gate really invokes `lake`, so
//! these tests prove the whole path — read a plugin, emit its cards, build the
//! generated Lean, attribute the diagnostics, and fail the run on any card
//! that did not prove.
//!
//! Both tests write `lean/Generated/` and run one `lake` build, so they hold a
//! lock rather than racing each other over the shared workbench directory.

use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;

use xtask::lean_check::LeanCheckArgs;

/// The workbench directory is shared state; one gate run at a time.
static GATE: Mutex<()> = Mutex::new(());

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Whether `lake` can be run at all. The gate is a Lean gate: with no Lean
/// toolchain there is nothing to check, and the test says so rather than
/// passing vacuously.
fn lake_is_available() -> bool {
    std::process::Command::new("lake")
        .arg("--version")
        .output()
        .is_ok()
}

fn skip(test: &str) -> bool {
    if lake_is_available() {
        return false;
    }
    eprintln!("SKIPPED {test}: `lake` is not on PATH, so no Lean toolchain can build the gate");
    true
}

/// Copies a plugin tree so a test can run the gate against a fixture without
/// touching the tracked original.
fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the destination is creatable");
    for entry in std::fs::read_dir(from).expect("the source reads") {
        let entry = entry.expect("the entry reads");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("the entry has a type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("the file copies");
        }
    }
}

/// The fixture plugin the whole workspace is checked against by default.
#[test]
fn the_testing_plugin_proves_every_card() {
    if skip("the_testing_plugin_proves_every_card") {
        return;
    }
    let _guard = GATE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let plugin = workspace_root().join("plugins_v2/testing");
    xtask::lean_check::run(&LeanCheckArgs::new(vec![plugin]))
        .expect("every card in plugins_v2/testing proves `Card.check = []`");
}

/// "Lawless Land" is a land face printed with a mana cost, which `cardCostOk`
/// in `lean/Semantics/Check/Card.lean` refuses with `.cardCost`. The gate must
/// single that card out — its neighbour is an ordinary land and must still
/// prove — and the failure must fail the gate outright, with no way to
/// silence it short of fixing the card.
#[test]
fn a_card_that_breaks_a_card_law_is_singled_out_and_the_gate_fails() {
    if skip("a_card_that_breaks_a_card_law_is_singled_out_and_the_gate_fails") {
        return;
    }
    let _guard = GATE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/plugins_v2_lawless");
    let temp = tempfile::tempdir().expect("a scratch directory");
    let plugin = temp.path().join("plugins_v2_lawless");
    copy_tree(&fixture, &plugin);

    let error = xtask::lean_check::run(&LeanCheckArgs::new(vec![plugin]))
        .expect_err("a card that breaks a Lean law fails the gate");
    let error = format!("{error:#}");
    // Exactly one of the fixture's two cards fails — the neighbouring lawful
    // land still proves — so the count in the message is the "singled out"
    // evidence: a plugin-wide failure would read "2 card(s)".
    assert!(error.contains("1 card(s) did not prove"), "{error}");
}

/// H1: the whole command, run against a `lake` that fails without naming any
/// card — an unknown target, a broken `Semantics/`, a toolchain that never
/// reached the compiler. The gate must stop, not report every card sound. It
/// did the latter until this test existed: `build` discarded `lake`'s exit
/// status and attribution seeded every card `Pass`, so a stub printing
/// `error: unknown target` produced "2/2 cards prove `Card.check = []`" and
/// exit zero.
#[test]
fn a_lake_that_fails_without_naming_a_card_stops_the_gate() {
    let _guard = GATE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let temp = tempfile::tempdir().expect("a scratch directory");
    let stub = temp.path().join("lake");
    std::fs::write(&stub, "#!/bin/sh\necho 'error: unknown target'\nexit 1\n")
        .expect("the stub writes");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&stub, std::fs::Permissions::from_mode(0o755))
            .expect("the stub is executable");
    }

    let plugin = workspace_root().join("plugins_v2/testing");
    let error =
        xtask::lean_check::run(&LeanCheckArgs::new(vec![plugin]).with_lake(stub.to_string_lossy()))
            .expect_err("a lake failure that names no card must stop the gate");
    let error = format!("{error:#}");
    assert!(error.contains("gate defect"), "{error}");
    assert!(error.contains("unknown target"), "{error}");
}

/// A failed run leaves nothing behind that poisons the next one. The stub run
/// above emits a generated tree, clears the build artifacts and then fails; a
/// real run afterwards must re-emit, rebuild and prove the cards again rather
/// than trip over the wreckage.
#[test]
fn a_real_run_recovers_after_a_stubbed_lake_failure() {
    if skip("a_real_run_recovers_after_a_stubbed_lake_failure") {
        return;
    }
    let _guard = GATE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let plugin = workspace_root().join("plugins_v2/testing");
    xtask::lean_check::run(&LeanCheckArgs::new(vec![plugin]))
        .expect("the real gate still runs after a stubbed failure");
}
