//! `cargo xtask graduate <plugin>` — rename every `cards/*.ron.todo` that now
//! parses cleanly to `<name>.ron`. Thin wrapper over
//! [`deckmaste_migrations::graduate::graduate_plugin`].

use std::path::Path;
use std::path::PathBuf;

use clap::Args;
use deckmaste_migrations::graduate::GraduateReport;

/// How many top blocking macros to list before truncating.
const TOP_MACROS: usize = 25;

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
    let report = deckmaste_migrations::graduate::graduate_plugin(&args.plugin_dir)?;
    print_report(&args.plugin_dir, &report);
    Ok(())
}

/// Prints a graduation report: the headline counts plus a breakdown of why the
/// remaining cards didn't graduate (unresolved oracle text, the top macros
/// blocking the most cards, and any other failures). Shared by
/// `xtask::generate` and `xtask::graduate` so their output never drifts.
/// Sections with a zero count are omitted.
pub fn print_report(plugin_dir: &Path, report: &GraduateReport) {
    eprintln!(
        "{}: graduated {}, {} still in progress",
        plugin_dir.display(),
        report.graduated.len(),
        report.remaining
    );
    if report.unresolved > 0 {
        eprintln!(
            "  unresolved (still have Unparsed lines): {}",
            report.unresolved
        );
    }
    if !report.blocked_on_macro.is_empty() {
        // Sort by count descending, then name ascending for stable ties.
        let mut by_count: Vec<(&String, &usize)> = report.blocked_on_macro.iter().collect();
        by_count.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        eprintln!("  top macros blocking graduation:");
        for (name, count) in by_count.iter().take(TOP_MACROS) {
            eprintln!("      {name:<24} {count}");
        }
        if by_count.len() > TOP_MACROS {
            eprintln!(
                "      ... ({} more macros not shown)",
                by_count.len() - TOP_MACROS
            );
        }
    }
    if !report.other.is_empty() {
        let sample = report.other.first().map_or("", |(_, line)| line.as_str());
        eprintln!(
            "  other failures: {} (e.g. \"{sample}\")",
            report.other.len()
        );
    }
}
