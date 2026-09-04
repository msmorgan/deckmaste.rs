use std::path::Path;
use std::process::Command;

#[test]
fn checked_in_flavor_word_nursery_passes_the_generator_check() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let output = Command::new(env!("CARGO_BIN_EXE_cargo-xtask"))
        .current_dir(workspace_root)
        .args(["english_v2", "flavor-words", "--check"])
        .output()
        .expect("run the flavor-word generator check");

    assert!(
        output.status.success(),
        "flavor-word generator check failed:\n{}",
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.starts_with("flavor-word stubs are up to date ("));
    assert!(stdout.ends_with(" files)\n"));
}
