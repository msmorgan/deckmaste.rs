//! `cargo xtask resolve <plugin>` — rewrite resolvable `Unparsed` ability lines
//! in every `cards/*.ron.todo`. Thin wrapper over
//! [`deckmaste_migrations::resolve::resolve_cards`].

use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct ResolveArgs {
    /// The plugin directory to resolve (e.g. plugins/wizards).
    plugin_dir: PathBuf,
}

/// Rewrite resolvable `Unparsed` ability lines in `cards/*.ron.todo`.
///
/// # Errors
/// If a `.ron.todo` isn't readable/parsable/writable.
#[allow(clippy::needless_pass_by_value)]
pub fn run(args: ResolveArgs) -> anyhow::Result<()> {
    deckmaste_migrations::resolve::resolve_cards(&args.plugin_dir)
}
