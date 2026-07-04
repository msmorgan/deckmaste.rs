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
    /// The corpus dry-run harness ([[cards-corpus-dry-run]]): elaborate every
    /// encodable face across `plugins/{builtin,canon,testing,wizards}` and
    /// print the calibration metrics — (a) the R2 ambiguity-gate fire rate,
    /// (b) the `Label`/`The`/`TheGroup` fallback rate, (c) a pointer to the
    /// archived hand-audit sample. Ignores the positional plugin dir;
    /// honors `--workspace-root`.
    #[arg(long)]
    metrics: bool,
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
        metrics,
    } = args;

    if lock || metrics {
        let workspace_root =
            workspace_root.unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."));
        return if lock { write_lock(&workspace_root) } else { corpus_metrics(&workspace_root) };
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

/// The plugins the dry-run measures: every hand-authored plugin with cards
/// plus the extracted `wizards` corpus (`demo` has no cards).
const CORPUS: [&str; 4] = ["builtin", "canon", "testing", "wizards"];

/// The corpus dry-run harness ([[cards-corpus-dry-run]]): elaborates every
/// encodable (finished, parsing) face across [`CORPUS`] via the same traced
/// walk `validate` runs, and prints the R2 calibration metrics. Deterministic:
/// two runs print identical numbers.
#[expect(
    clippy::cast_precision_loss,
    reason = "percentage display over face counts (~thousands) — exact in f64"
)]
fn corpus_metrics(workspace_root: &Path) -> anyhow::Result<()> {
    let mut faces = 0usize;
    let mut ambiguous_errors = 0usize;
    let mut ambiguous_files = std::collections::BTreeSet::new();
    let mut fallback_reads = 0usize;
    let mut fallback_files = 0usize;
    let mut other_findings = 0usize;

    for plugin in CORPUS {
        let dir = workspace_root.join("plugins").join(plugin);
        let v = deckmaste_cards::validate::validate_plugin(&dir)?;
        anyhow::ensure!(
            v.failures.is_empty(),
            "{plugin}: {} unparseable card(s) — not an encodable-face question, fix them first",
            v.failures.len(),
        );
        println!(
            "plugins/{plugin}: {} encodable face(s), {} elaboration finding(s)",
            v.valid,
            v.elab_failures.len(),
        );
        faces += v.valid;
        for (path, error) in &v.elab_failures {
            if error.code == elaborate::Code::BindAmbiguous {
                ambiguous_errors += 1;
                ambiguous_files.insert(path.clone());
            } else {
                other_findings += 1;
            }
        }
        fallback_reads += v.fallback_reads;
        fallback_files += v.fallback_files;
    }

    let pct = |n: usize| 100.0 * n as f64 / faces as f64;
    println!("corpus: {faces} encodable faces");
    println!(
        "(a) R2 ambiguity-gate fires (E-BIND-AMBIGUOUS): {} face(s), {} error(s) — {:.2}% of \
         faces (bar: <=3%)",
        ambiguous_files.len(),
        ambiguous_errors,
        pct(ambiguous_files.len()),
    );
    for path in &ambiguous_files {
        println!("    {}", path.display());
    }
    println!(
        "(b) Label/The/TheGroup fallback reads: {fallback_reads} read(s) across \
         {fallback_files} face(s) — {:.2}% of faces",
        pct(fallback_files),
    );
    println!(
        "(c) mis-binding hand audit: crates/deckmaste_cards/tests/r2_audit/ (sample + pinned \
         resolution tables)",
    );
    println!("    context: {other_findings} non-gate elaboration finding(s)");
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
    let registries = plugin.registries();
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
