use std::process::Command;

fn production_expansion() -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-xtask"))
        .args(["english_v2", "expand"])
        .output()
        .expect("run English-v2 production grammar expansion")
}

#[test]
fn production_expansion_is_byte_identical_across_processes() {
    let first = production_expansion();
    assert!(
        first.status.success(),
        "first expansion failed:\n{}",
        String::from_utf8_lossy(&first.stderr),
    );

    let second = production_expansion();
    assert!(
        second.status.success(),
        "second expansion failed:\n{}",
        String::from_utf8_lossy(&second.stderr),
    );

    assert!(!first.stdout.is_empty(), "production expansion was empty");
    assert_eq!(first.stdout, second.stdout);
}
