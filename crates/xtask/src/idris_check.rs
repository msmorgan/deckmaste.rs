//! `cargo xtask idris-check` — the anaphora-soundness gate that replaces the
//! deleted Rust elaborator/twin gate: re-emit each EXPANDED card as an
//! equivalent raw `idris/src/Core.idr` expression (via
//! `deckmaste_cards::idris_emit`) and typecheck it with `idris2 --check`.
//! Idris's dependent `Normal`/`Reference`/`Selection` proofs make an unsound
//! card (a dangling anaphor, an ambiguous antecedent, …) unrepresentable, so
//! a card that typechecks is sound by construction.
//!
//! Two modes:
//!  - `idris-check <plugin> <card>` — one card, one temp module, one `idris2`
//!    invocation. Prints the idris2 output on failure.
//!  - `idris-check <plugin>` — every card in the plugin, batched (idris2
//!    startup dominates, so many cards share one invocation); reports how many
//!    typecheck, and for the rest whether it's an emitter gap (no Idris text
//!    produced at all) or an Idris proof failure (emitted, but rejected — the
//!    interesting case: either a genuinely unsound card or an over-strict Idris
//!    proof).

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use clap::Args;
use deckmaste_cards::idris_emit;
use deckmaste_cards::plugin::Plugin;
use deckmaste_core::Card;
use deckmaste_core::Expand;

#[derive(Debug, Args)]
pub struct IdrisCheckArgs {
    /// The plugin directory (e.g. `plugins/canon`).
    plugin_dir: PathBuf,
    /// A single card name to check (e.g. "Grizzly Bears"). Omit to
    /// batch-check every finished card in the plugin.
    card_name: Option<String>,
    /// How many cards share one `idris2 --check` invocation in batch mode.
    #[arg(long, default_value_t = 100)]
    batch_size: usize,
}

/// # Errors
/// If the plugin fails to load, the named card is missing/invalid, or (batch
/// mode) any card fails to typecheck.
pub fn run(args: &IdrisCheckArgs) -> anyhow::Result<()> {
    let idris_dir = idris_root()?;
    // Guard BOTH modes with a one-shot dependency typecheck: if the shared
    // imports don't compile, fail fast here instead of rediscovering the same
    // build error once per card (see `preflight_deps`).
    preflight_deps(&idris_dir)?;
    let plugin = Plugin::load_with_sibling_prelude(&args.plugin_dir)
        .with_context(|| format!("loading plugin {}", args.plugin_dir.display()))?;
    match &args.card_name {
        Some(name) => run_single(&plugin, name, &idris_dir),
        None => run_batch(&plugin, &args.plugin_dir, &idris_dir, args.batch_size),
    }
}

/// The workspace's `idris/` directory — `idris2 --find-ipkg --check` is run
/// from here so it picks up `mtg.ipkg`'s `sourcedir = "src"`.
fn idris_root() -> anyhow::Result<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../idris");
    anyhow::ensure!(dir.is_dir(), "expected an idris/ dir at {}", dir.display());
    Ok(dir)
}

/// One-shot pre-flight typecheck of ONLY the shared dependency surface that
/// every batch imports — `render_module("IdrisCheckDeps", &[])` is exactly
/// `module IdrisCheckDeps\nimport Core\n\n`, no card defs.
///
/// Without this, a `Core.idr` that doesn't compile makes *every* batch fail,
/// and the per-card isolation fallback (meant for one genuinely-unsound card)
/// re-checks *every* card, each failing identically for the same dependency
/// error — ~72 × 101 ≈ 7200 `idris2` invocations for the wizards corpus, all
/// burying the one real cause. A broken dependency is O(1) to detect up front;
/// left to the failure path it is *rediscovered* N times.
///
/// It also warms `Core.ttc` (built once here) so the real batches — and every
/// isolation re-check — reuse it instead of rebuilding Core from source.
///
/// On failure this bails framed as a dependency/build error, deterministically
/// (no parsing of idris2 diagnostics): a broken import surface is never any
/// card's fault.
fn preflight_deps(idris_dir: &Path) -> anyhow::Result<()> {
    let source = render_module("IdrisCheckDeps", &[]);
    match typecheck_module(idris_dir, "IdrisCheckDeps", &source)? {
        TypecheckOutcome::Pass => Ok(()),
        TypecheckOutcome::Fail(output) => anyhow::bail!(
            "idris-check pre-flight: the shared Idris dependencies do not compile \
             (Core.idr and whatever the probe module imports). No card is at fault — \
             fix the dependency build first.\n\nidris2 said:\n{output}"
        ),
    }
}

fn run_single(plugin: &Plugin, card_name: &str, idris_dir: &Path) -> anyhow::Result<()> {
    let card = plugin
        .card(card_name)
        .with_context(|| format!("loading card {card_name:?}"))?
        .expand_all();
    let ident = idris_emit::sanitize_ident(card_name);
    let module_name = format!("IdrisCheckSingle_{ident}");

    let expr = match idris_emit::emit_card_expr(&card, plugin) {
        Ok(expr) => expr,
        Err(gap) => {
            println!("FAIL (emitter gap): {card_name}");
            println!("  {gap}");
            anyhow::bail!("{card_name}: emitter gap: {gap}");
        }
    };

    let source = render_module(&module_name, &[(ident.clone(), expr)]);
    let outcome = typecheck_module(idris_dir, &module_name, &source)?;

    match outcome {
        TypecheckOutcome::Pass => {
            println!("PASS: {card_name}");
            Ok(())
        }
        TypecheckOutcome::Fail(output) => {
            println!("FAIL (Idris proof failure): {card_name}");
            println!("{output}");
            anyhow::bail!("{card_name}: failed idris2 --check");
        }
    }
}

fn run_batch(
    plugin: &Plugin,
    plugin_dir: &Path,
    idris_dir: &Path,
    batch_size: usize,
) -> anyhow::Result<()> {
    let cards = idris_emit::load_all_cards(plugin_dir, plugin)
        .with_context(|| format!("loading cards from {}", plugin_dir.display()))?;
    let total = cards.len();

    // One (name, ident) assignment per card, computed ONCE so every later
    // step (emission, reporting proof failures by name) agrees on it.
    let mut used_idents: HashSet<String> = HashSet::new();
    let named_cards: Vec<(String, String, Card)> = cards
        .iter()
        .map(|card| {
            let card = card.clone().expand_all();
            let name = idris_emit::card_display_name(&card).to_string();
            let mut ident = idris_emit::sanitize_ident(&name);
            while !used_idents.insert(ident.clone()) {
                ident.push('_');
            }
            (name, ident, card)
        })
        .collect();
    let mut names: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    // Emission is pure Rust — do it all up front, splitting into what can be
    // typechecked (emitted fine) and what can't (gaps), so the batch loop
    // below only ever hands idris2 already-emitted, valid Idris source.
    let mut emitted: Vec<(String, String)> = Vec::new(); // (ident, expr)
    let mut gaps: Vec<(String, String)> = Vec::new(); // (card name, gap message)
    for (name, ident, card) in named_cards {
        names.insert(ident.clone(), name.clone());
        match idris_emit::emit_card_expr(&card, plugin) {
            Ok(expr) => emitted.push((ident, expr)),
            Err(gap) => gaps.push((name, gap.to_string())),
        }
    }

    let mut passed = 0usize;
    let mut proof_failures: Vec<(String, String)> = Vec::new(); // (card name, idris2 output)

    for (batch_idx, chunk) in emitted.chunks(batch_size.max(1)).enumerate() {
        let module_name = format!("IdrisCheckBatch_{batch_idx}");
        let source = render_module(&module_name, chunk);
        let outcome = typecheck_module(idris_dir, &module_name, &source)?;
        match outcome {
            TypecheckOutcome::Pass => passed += chunk.len(),
            TypecheckOutcome::Fail(output) => {
                // Isolate which card(s) in this chunk actually fail: re-check
                // each individually (only on the failure path, so the common
                // "all sound" case stays one invocation per ~batch_size).
                eprintln!(
                    "batch {batch_idx} ({} cards) failed as a whole; isolating per-card (idris2 said):\n{output}",
                    chunk.len()
                );
                for (ident, expr) in chunk {
                    let single_module = format!("IdrisCheckIsolate_{ident}");
                    let single_source = render_module(
                        &single_module,
                        std::slice::from_ref(&(ident.clone(), expr.clone())),
                    );
                    let single_outcome =
                        typecheck_module(idris_dir, &single_module, &single_source)?;
                    match single_outcome {
                        TypecheckOutcome::Pass => passed += 1,
                        TypecheckOutcome::Fail(single_output) => {
                            let name = names.get(ident).cloned().unwrap_or_else(|| ident.clone());
                            proof_failures.push((name, single_output));
                        }
                    }
                }
            }
        }
    }

    println!(
        "{}: {}/{total} cards emit and typecheck",
        plugin_dir.display(),
        passed
    );

    if !gaps.is_empty() {
        println!(
            "\n{} card(s) fail to EMIT (emitter gap — a Rust grammar shape not yet mapped):",
            gaps.len()
        );
        for (name, msg) in &gaps {
            println!("  {name}: {msg}");
        }
    }

    if !proof_failures.is_empty() {
        println!(
            "\n{} card(s) emit but FAIL the Idris proof (interesting: either genuinely unsound or an over-strict Idris proof):",
            proof_failures.len()
        );
        for (name, output) in &proof_failures {
            println!("  {name}:");
            for line in output.lines() {
                println!("    {line}");
            }
        }
    }

    Ok(())
}

fn render_module(module_name: &str, defs: &[(String, String)]) -> String {
    use std::fmt::Write as _;

    let mut out = format!("module {module_name}\nimport Core\n\n");
    for (ident, expr) in defs {
        let _ = write!(out, "card_{ident} : Card\ncard_{ident} = {expr}\n\n");
    }
    out
}

enum TypecheckOutcome {
    Pass,
    Fail(String),
}

/// RAII owner of a scratch module's on-disk footprint: dropping it removes the
/// `idris/src/<module>.idr` source and its compiled `.ttc`/`.ttm` artifacts.
///
/// Cleanup used to be an explicit `cleanup_module` call *after* the typecheck,
/// which any interruption skipped — a `?`-early-return, a panic, or a kill mid
/// batch left `IdrisCheck*.idr` litter in the *tracked* `idris/src/` tree (a
/// killed run really did leave an added `IdrisCheckIsolate_*.idr`). A `Drop`
/// guard removes the file on every one of those paths, so cleanup no longer
/// leans on the `.gitignore` stopgap to keep the leak out of a commit.
struct ScratchModule<'a> {
    idris_dir: &'a Path,
    module_name: String,
}

impl Drop for ScratchModule<'_> {
    fn drop(&mut self) {
        cleanup_module(self.idris_dir, &self.module_name);
    }
}

fn typecheck_module(
    idris_dir: &Path,
    module_name: &str,
    source: &str,
) -> anyhow::Result<TypecheckOutcome> {
    // Own the scratch footprint up front so it is cleaned up on EVERY exit from
    // this function — the `?` below, a panic, or normal return alike.
    let _scratch = ScratchModule {
        idris_dir,
        module_name: module_name.to_owned(),
    };

    let rel_path = format!("src/{module_name}.idr");
    let abs_path = idris_dir.join(&rel_path);
    fs::write(&abs_path, source).with_context(|| format!("writing {}", abs_path.display()))?;

    let output = Command::new("idris2")
        .args(["--find-ipkg", "--check", &rel_path])
        .current_dir(idris_dir)
        .output()
        .with_context(|| "running idris2 (is it on PATH?)")?;

    if output.status.success() {
        Ok(TypecheckOutcome::Pass)
    } else {
        let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&output.stderr));
        Ok(TypecheckOutcome::Fail(text))
    }
}

/// Removes the temp module's source file and its compiled `.ttc`/`.ttm`
/// artifacts (`build/ttc/<version>/<module>.tt?`) so repeated runs don't
/// accumulate scratch files in the tracked `idris/` tree.
fn cleanup_module(idris_dir: &Path, module_name: &str) {
    let _ = fs::remove_file(idris_dir.join(format!("src/{module_name}.idr")));
    let ttc_dir = idris_dir.join("build/ttc");
    let Ok(versions) = fs::read_dir(&ttc_dir) else {
        return;
    };
    for entry in versions.flatten() {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        let _ = fs::remove_file(dir.join(format!("{module_name}.ttc")));
        let _ = fs::remove_file(dir.join(format!("{module_name}.ttm")));
    }
}
