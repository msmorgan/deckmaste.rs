//! `cargo xtask graduate <plugin>` — rename every `cards/*.ron.todo` that now
//! parses cleanly to `<name>.ron`. Thin wrapper over
//! [`deckmaste_cards::graduate::graduate_plugin`].

use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct GraduateArgs {
    /// The plugin directory to graduate (e.g. plugins/wizards).
    plugin_dir: PathBuf,
}

/// Graduate every `cards/*.ron.todo` in the plugin that now parses.
///
/// # Errors
/// If the plugin fails to load or a file isn't readable/renamable.
#[allow(clippy::needless_pass_by_value)]
pub fn run(args: GraduateArgs) -> anyhow::Result<()> {
    let report = deckmaste_cards::graduate::graduate_plugin(&args.plugin_dir)?;
    eprintln!(
        "{}: graduated {}, {} still in progress",
        args.plugin_dir.display(),
        report.graduated.len(),
        report.remaining
    );
    Ok(())
}
