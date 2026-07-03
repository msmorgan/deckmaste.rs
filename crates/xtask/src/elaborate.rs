//! The `elaborate` command: the load-time elaboration walk over a plugin's
//! finished cards/tokens (the same gate `Plugin::load` now runs, surfaced
//! standalone for review), the `cards.elab.lock` bless flow, and the
//! `--dump <card>` binding-resolution inspector. xtask owns the CLI; the
//! elaborator and lockfile logic live in `deckmaste_cards`.

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use deckmaste_cards::elaborate;
use deckmaste_cards::lock;
use deckmaste_cards::plugin::Plugin;
use deckmaste_cards::plugin::read;

#[derive(Debug, Args)]
pub struct ElaborateArgs {
    /// Defaults to this workspace's `plugins/builtin`. Ignored by `--lock`,
    /// which always covers every hand-authored plugin
    /// (`lock::HAND_AUTHORED`).
    plugin_dir: Option<PathBuf>,
    /// Recompute and write `cards.elab.lock`: a per-card content hash of the
    /// elaborated IR for every hand-authored plugin. Running it twice with
    /// no table/macro/card change in between is idempotent (no diff) — CI's
    /// "clean" is exactly that: rerun, then `git diff --exit-code`.
    #[arg(long)]
    lock: bool,
    /// Where `--lock` looks for `plugins/{builtin,canon,testing,demo}` and
    /// writes `cards.elab.lock`. Defaults to this workspace. Exists for
    /// drift demonstrations against a throwaway copy — never point CI at
    /// anything but the default.
    #[arg(long)]
    workspace_root: Option<PathBuf>,
    /// Print the computed bindings (which antecedent each `Target`/`That`/
    /// `It`/event-role/`{X}`/counter/designation reference bound to) for one
    /// card by name, for review — even if the card fails elaboration (the
    /// errors print too).
    #[arg(long)]
    dump: Option<String>,
}

/// # Errors
/// If the plugin fails to load, a requested `--dump` card is missing, the
/// hand-authored plugins fail to load/parse for `--lock`, or (outside
/// `--lock`/`--dump`) the plugin's own stage is `Deny` and it has
/// findings — surfaced by `Plugin::load` itself failing first.
pub fn run(args: ElaborateArgs) -> anyhow::Result<()> {
    let ElaborateArgs {
        plugin_dir,
        lock,
        workspace_root,
        dump,
    } = args;

    if lock {
        let workspace_root =
            workspace_root.unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
        return write_lock(&workspace_root);
    }

    let plugin_dir = plugin_dir
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"));

    if let Some(card_name) = dump {
        return dump_card(&plugin_dir, &card_name);
    }

    // No flags: just load the plugin and show its counted report.
    // `Plugin::load*` already ran (and, under `Stage::Deny`, already failed)
    // the gate — this is the `Stage::Warn` "counted report" surfaced
    // standalone, without needing a live engine.
    let plugin = Plugin::load_with_sibling_prelude(&plugin_dir)?;
    println!(
        "{}: stage {:?}, {} checked, {} finding(s)",
        plugin_dir.display(),
        plugin.elab_stage,
        plugin.elab_report.checked,
        plugin.elab_report.findings.len(),
    );
    for finding in &plugin.elab_report.findings {
        println!("  {finding}");
    }
    Ok(())
}

/// (Re)computes and writes `cards.elab.lock` at `workspace_root`.
fn write_lock(workspace_root: &Path) -> anyhow::Result<()> {
    let checksums = lock::compute(workspace_root)?;
    let rendered = lock::render(&checksums);
    let path = workspace_root.join(lock::FILE_NAME);
    std::fs::write(&path, rendered).with_context(|| format!(r#"writing "{}""#, path.display()))?;
    println!(
        "{}: {} card(s) across {} hand-authored plugin(s)",
        path.display(),
        checksums.len(),
        lock::HAND_AUTHORED.len(),
    );
    Ok(())
}

/// Prints `card_name`'s computed bindings (and, if it fails, its
/// elaboration errors) — reads the file directly rather than through
/// `Plugin::card`, so a `Stage::Warn` card the load-time walk already
/// skipped can still be inspected.
fn dump_card(plugin_dir: &Path, card_name: &str) -> anyhow::Result<()> {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)?;
    let path = plugin.card_path(card_name);
    let source = read(&path)?;
    let card: deckmaste_core::Card = plugin
        .macros
        .read_str(&source)
        .with_context(|| format!(r#"parsing "{}""#, path.display()))?;
    let registries = elaborate::Registries {
        subtypes: &plugin.subtypes,
        counters: &plugin.counters,
    };
    let (result, resolutions) = elaborate::elaborate_with_resolutions(&card, &registries);

    println!("{card_name} ({}):", path.display());
    if resolutions.is_empty() {
        println!("  (no bindings resolved)");
    }
    for resolution in &resolutions {
        println!("  {resolution}");
    }

    match result {
        Ok(_) => {
            println!("elaborates clean.");
            Ok(())
        }
        Err(errors) => {
            println!("{} elaboration error(s):", errors.len());
            for error in &errors {
                println!("  {error}");
            }
            anyhow::bail!("{card_name:?} failed elaboration");
        }
    }
}
