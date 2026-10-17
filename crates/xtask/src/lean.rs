//! `cargo xtask lean check`: build the native Lean workbench.

use std::path::Path;
use std::process::Command;

use anyhow::Context as _;
use clap::Args;
use clap::Subcommand;

#[derive(Debug, Args)]
pub struct LeanArgs {
    #[command(subcommand)]
    command: LeanCmd,
}

#[derive(Debug, Subcommand)]
enum LeanCmd {
    /// Build the syntax, checker, macros, card bench and proofs; warnings fail.
    Check,
}

pub fn run(args: &LeanArgs) -> anyhow::Result<()> {
    match args.command {
        LeanCmd::Check => check(
            &mut Command::new("lake"),
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../lean"),
        ),
    }
}

fn check(lake: &mut Command, workbench: &Path) -> anyhow::Result<()> {
    let status = lake
        .current_dir(workbench)
        .args(["build", "--wfail"])
        .status()
        .context("starting the Lean workbench build")?;
    anyhow::ensure!(status.success(), "Lean workbench build failed: {status}");
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn stub(body: &str) -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let script = dir.path().join("lake.sh");
        std::fs::write(&script, body)?;
        let mut command = Command::new("sh");
        command.arg(&script).env("EXPECTED_WORKBENCH", dir.path());
        check(&mut command, dir.path())
    }

    #[test]
    fn check_builds_the_workbench_with_warnings_as_errors() {
        stub("test \"$PWD\" = \"$EXPECTED_WORKBENCH\" && test \"$#\" = 2 && test \"$1\" = build && test \"$2\" = --wfail").unwrap();
    }

    #[test]
    fn check_propagates_build_failure() {
        let error = stub("exit 7").unwrap_err();
        assert!(error.to_string().contains("Lean workbench build failed"));
    }

    #[test]
    fn check_propagates_failure_to_start_lake() {
        let dir = tempfile::tempdir().unwrap();
        let error = check(
            &mut Command::new(dir.path().join("missing-lake")),
            dir.path(),
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("starting the Lean workbench build")
        );
    }
}
