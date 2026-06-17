//! The `cite` command: a thin shim over the shared fish checker that ships
//! with the mtg-rules skill (`scripts/cite`). All the machinery — site
//! scanning, range expansion, the JSON lockfile with text-hash staleness —
//! lives there, configured for this repo by the root `cite-config.json`.
//! The subcommand surface is unchanged: `check [--list-noncompliant]`,
//! `bless`, `list`, `show <citation> [--plain]`, `diff <rule>`; bare
//! `cargo xtask cite` runs `check`.

use std::path::PathBuf;
use std::process::Command;

use clap::Args;

/// Arguments passed through to the shared `cite` script verbatim.
#[derive(Debug, Args)]
pub struct CiteArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[must_use]
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// `$MTG_RULES_CITE` overrides the script location (e.g. a plugin install or
/// a development checkout elsewhere).
fn script_path() -> PathBuf {
    if let Ok(p) = std::env::var("MTG_RULES_CITE") {
        return PathBuf::from(p);
    }
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home).join(".claude/skills/mtg-rules/scripts/cite")
}

/// Run the shared checker with this repo's config; propagate its exit status.
///
/// # Errors
/// Fails when the script is missing, cannot be spawned, or exits non-zero
/// (stale citations, non-compliant strings, …).
#[allow(clippy::needless_pass_by_value)]
pub fn dispatch(args: CiteArgs) -> anyhow::Result<()> {
    let root = repo_root();
    let script = script_path();
    anyhow::ensure!(
        script.is_file(),
        "shared cite script not found at {} — install the mtg-rules skill or set MTG_RULES_CITE",
        script.display(),
    );
    let mut cmd = Command::new(&script);
    cmd.arg("--config").arg(root.join("cite-config.json"));
    cmd.args(&args.args);
    // The script resolves its CR snapshot via MTG_RULES_DATA; default it to
    // this repo's data/ so `cargo xtask cite` is self-contained anywhere.
    if std::env::var_os("MTG_RULES_DATA").is_none() {
        cmd.env("MTG_RULES_DATA", root.join("data"));
    }
    let status = cmd.status()?;
    anyhow::ensure!(status.success(), "cite exited with {status}");
    Ok(())
}
