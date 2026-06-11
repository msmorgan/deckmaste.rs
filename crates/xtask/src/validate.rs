//! The `validate` command: check every finished card in a plugin against the
//! macro-aware reader and canon. xtask owns the CLI; the parsing and checking
//! live in `deckmaste_cards`.

use std::path::Path;
use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Defaults to this workspace's `plugins/builtin`.
    plugin_dir: Option<PathBuf>,
}

/// Report every non-todo card that doesn't parse, and every finished card that
/// disagrees with canon's reference version of the same name.
///
/// # Errors
/// If the plugin (or its prelude) fails to load, or a card file isn't readable.
/// Invalid cards are printed to stderr and turned into one nonzero-exit error
/// after the summary.
pub fn run(args: ValidateArgs) -> anyhow::Result<()> {
    let plugin_dir = args
        .plugin_dir
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"));

    let validation = deckmaste_cards::validate::validate_plugin(&plugin_dir)?;
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }

    // canon is the authority: a finished card a plugin shares with canon must
    // expand to canon's value (see `check_against_canon`).
    let mismatches = deckmaste_cards::validate::check_against_canon(&plugin_dir)?;
    for mismatch in &mismatches {
        eprintln!(
            "{}: differs from canon reference {}",
            mismatch.plugin_path.display(),
            mismatch.canon_path.display()
        );
    }

    println!(
        "{}: {} valid, {} todos skipped, {} invalid, {} canon mismatch(es)",
        plugin_dir.display(),
        validation.valid,
        validation.todos,
        validation.failures.len(),
        mismatches.len(),
    );
    if !validation.failures.is_empty() || !mismatches.is_empty() {
        anyhow::bail!(
            "{}: {} invalid card(s), {} canon mismatch(es)",
            plugin_dir.display(),
            validation.failures.len(),
            mismatches.len(),
        );
    }
    Ok(())
}
