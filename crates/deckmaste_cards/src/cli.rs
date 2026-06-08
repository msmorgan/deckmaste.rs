//! CLI entry points, shared by this crate's `card` bin and `cargo xtask`.
//! Each takes full argv (program name included) so both callers parse
//! identically.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use clap::Parser;

use crate::plugin::Plugin;

/// Shows a card as parsed from a plugin, with its macro references
/// expanded.
#[derive(Debug, Parser)]
struct CardArgs {
    plugin_dir: PathBuf,
    card_name: String,
}

/// The `card` entry point: parse one card (builtin sibling prelude in
/// scope) and print its expansion.
///
/// # Errors
/// If the plugin fails to load or the card is missing or invalid.
pub fn card<I: IntoIterator<Item = OsString>>(args: I) -> anyhow::Result<()> {
    let args = CardArgs::parse_from(args);
    let plugin = Plugin::load_with_sibling_prelude(&args.plugin_dir)?;
    let card = plugin.card(&args.card_name)?;

    println!(
        "{} expands to:\n",
        plugin.card_path(&args.card_name).display()
    );
    println!("{card:#?}");

    Ok(())
}

/// Validates every finished card in a plugin through the macro-aware
/// reader.
#[derive(Debug, Parser)]
struct ValidateArgs {
    /// Defaults to this workspace's `plugins/builtin`.
    plugin_dir: Option<PathBuf>,
}

/// The `validate` entry point: report every non-todo card that doesn't
/// parse, and every finished card that disagrees with canon's reference
/// version of the same name.
///
/// # Errors
/// If the plugin (or its prelude) fails to load, or a card file isn't
/// readable. Invalid cards are printed to stderr and turned into one
/// nonzero-exit error after the summary.
pub fn validate<I: IntoIterator<Item = OsString>>(args: I) -> anyhow::Result<()> {
    let args = ValidateArgs::parse_from(args);
    let plugin_dir = args
        .plugin_dir
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"));

    let validation = crate::validate::validate_plugin(&plugin_dir)?;
    for failure in &validation.failures {
        eprintln!("{}: {}", failure.path.display(), failure.error);
    }

    // canon is the authority: a finished card a plugin shares with canon must
    // expand to canon's value (see `check_against_canon`).
    let mismatches = crate::validate::check_against_canon(&plugin_dir)?;
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
