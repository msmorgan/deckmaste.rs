//! `cargo xtask idris-check` — the anaphora-soundness gate that replaces the
//! deleted Rust elaborator/twin gate: re-emit each EXPANDED SEMANTIC card as
//! an equivalent raw `idris/src/Semantics.idr` expression (via
//! `deckmaste_plugin::idris_emit`) and typecheck it with `idris2 --check`.
//! Idris's dependent `Normal`/`Reference`/`Selection` proofs make an unsound
//! card (a dangling anaphor, an ambiguous antecedent, …) unrepresentable, so
//! a card that typechecks is sound by construction.
//!
//! Two modes:
//!  - `idris-check <plugin> <card>` — one card, one temp module, one `idris2`
//!    invocation. Prints the idris2 output on failure.
//!  - `idris-check <plugin>` — every card in the plugin, batched (idris2
//!    startup dominates, so many cards share one invocation); compares the
//!    result with `<plugin>/idris-check-baseline.ron`. The baseline is an exact
//!    pass/gap classification: regressions fail, improvements require a
//!    reviewed `--bless`, and Idris proof failures are always fatal.
//!  - `idris-check <plugin> --differential` — the CERTIFIER/RESOLVER
//!    differential (`semantics-spelling-lowering.md` §17, [Core is explicit
//!    regions] law 12). The mirror CERTIFIES that a card's anaphora resolve
//!    uniquely; `deckmaste_lowering` COMPUTES that resolution. Two independent
//!    derivations of one rule, so they must agree card for card: a card the
//!    mirror proves sound must lower, and a card the mirror refutes must not.
//!    Emitter gaps carry NO certifier verdict and are skipped — a shape the
//!    emitter cannot express is not a claim about the card.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use clap::Args;
use deckmaste_plugin::idris_emit;
use deckmaste_plugin::plugin::Plugin;
use deckmaste_semantics::Card;
use serde::Deserialize;

const BASELINE_FILE: &str = "idris-check-baseline.ron";
const BASELINE_VERSION: u32 = 1;

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
    /// Replace the plugin's checked-in pass/gap baseline after reviewing all
    /// emitter gaps. Batch mode only; proof failures can never be blessed.
    #[arg(long, conflicts_with = "card_name")]
    bless: bool,
    /// Pair lowering's computed resolution against the Idris mirror's
    /// certification over the whole plugin and report every disagreement
    /// (`semantics-spelling-lowering.md` §17). Batch mode only.
    #[arg(long, conflicts_with_all = ["card_name", "bless"])]
    differential: bool,
}

/// One card on which the resolver and the certifier disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Disagreement {
    card: String,
    /// What the mirror proved, and what lowering did instead.
    detail: String,
}

/// What the Idris mirror concluded about one card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CertifierVerdict {
    /// The re-emitted term typechecks: its anaphora provably resolve uniquely.
    Sound,
    /// The re-emitted term fails the proof: the card is refuted.
    Unsound,
    /// The emitter cannot express this card's shape, so there is no claim
    /// about it either way.
    NoVerdict,
}

/// How one card's two verdicts pair up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pairing {
    AgreedSound,
    AgreedUnsound,
    /// No certifier verdict to compare against.
    Skipped,
    /// The mirror proved the anaphora resolve; lowering refused.
    CertifiedButRefused,
    /// The mirror refuted the anaphora; lowering resolved them anyway.
    RefutedButResolved,
}

/// Pair one card's two derivations. Split out from the walk so the gate's
/// verdict table is testable without an `idris2` invocation — a differential
/// that cannot be shown to REPORT a disagreement is not evidence of anything.
const fn pair_verdicts(certifier: CertifierVerdict, resolver_resolved: bool) -> Pairing {
    match (certifier, resolver_resolved) {
        (CertifierVerdict::Sound, true) => Pairing::AgreedSound,
        (CertifierVerdict::Sound, false) => Pairing::CertifiedButRefused,
        (CertifierVerdict::Unsound, true) => Pairing::RefutedButResolved,
        (CertifierVerdict::Unsound, false) => Pairing::AgreedUnsound,
        (CertifierVerdict::NoVerdict, _) => Pairing::Skipped,
    }
}

/// The paired verdicts over one plugin slice.
#[derive(Debug, Clone, Default)]
struct DifferentialReport {
    /// Cards where both derivations accept.
    agreed_sound: usize,
    /// Cards where both derivations reject.
    agreed_unsound: usize,
    /// Cards the emitter cannot express, so the certifier has no verdict.
    skipped_gaps: usize,
    disagreements: Vec<Disagreement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Baseline {
    version: u32,
    passes: Vec<String>,
    gaps: Vec<BaselineGap>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Ord, PartialOrd)]
#[serde(deny_unknown_fields)]
struct BaselineGap {
    card: String,
    reason: String,
}

#[derive(Debug, Clone)]
struct ProofFailure {
    card: String,
    output: String,
}

#[derive(Debug, Clone)]
struct BatchReport {
    total: usize,
    passes: Vec<String>,
    gaps: Vec<BaselineGap>,
    proof_failures: Vec<ProofFailure>,
}

#[derive(Debug, PartialEq, Eq)]
struct BaselineComparison {
    lost_passes: Vec<String>,
    new_gaps: Vec<BaselineGap>,
    proof_failures: Vec<String>,
    missing_cards: Vec<String>,
    resolved_gaps: Vec<String>,
    new_passes: Vec<String>,
}

impl BaselineComparison {
    fn regression_count(&self) -> usize {
        self.lost_passes.len()
            + self.new_gaps.len()
            + self.proof_failures.len()
            + self.missing_cards.len()
    }

    fn baseline_update_count(&self) -> usize {
        self.resolved_gaps.len() + self.new_passes.len()
    }
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
        None if args.differential => {
            run_differential(&plugin, &args.plugin_dir, &idris_dir, args.batch_size)
        }
        None => run_batch(
            &plugin,
            &args.plugin_dir,
            &idris_dir,
            args.batch_size,
            args.bless,
        ),
    }
}

/// The certifier/resolver differential over one plugin slice.
///
/// The mirror's verdict comes from the same batch typecheck the baseline gate
/// runs; the resolver's comes from `Plugin::card_resolution_from_str`, which
/// returns lowering's refusal as data rather than raising it. Both derive the
/// same rule (a card's anaphora resolve uniquely — R1/R2), so any card they
/// classify differently is a real finding in one of them.
///
/// # Errors
/// If cards cannot be read, or if the two derivations disagree anywhere.
fn run_differential(
    plugin: &Plugin,
    plugin_dir: &Path,
    idris_dir: &Path,
    batch_size: usize,
) -> anyhow::Result<()> {
    let certifier = collect_batch_report(plugin, plugin_dir, idris_dir, batch_size)?;
    let certified: HashSet<&str> = certifier.passes.iter().map(String::as_str).collect();
    let refuted: HashSet<&str> = certifier
        .proof_failures
        .iter()
        .map(|failure| failure.card.as_str())
        .collect();
    let gaps: HashSet<&str> = certifier.gaps.iter().map(|gap| gap.card.as_str()).collect();

    let mut report = DifferentialReport::default();
    for path in crate::idris_check::card_sources(plugin_dir)? {
        let source =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        if deckmaste_core::plugin::is_todo_source(&source) {
            continue;
        }
        let resolution = plugin
            .card_resolution_from_str(&source)
            .with_context(|| format!("parsing {}", path.display()))?;
        // Name the card the way the certifier does, so the two verdicts key on
        // one identity rather than on a file stem that may differ.
        let semantic = plugin
            .rendering_card_from_str(&source)
            .with_context(|| format!("parsing {}", path.display()))?;
        let name = idris_emit::card_display_name(&semantic).to_string();

        let certifier = if certified.contains(name.as_str()) {
            CertifierVerdict::Sound
        } else if refuted.contains(name.as_str()) {
            CertifierVerdict::Unsound
        } else {
            // An emitter gap: a shape the mirror cannot express is not a claim
            // about the card. Anything NOT in the gap list either has been
            // silently dropped by the batch or is named differently by the two
            // halves — a hole in the gate itself, so say so rather than
            // counting it as agreement.
            anyhow::ensure!(
                gaps.contains(name.as_str()),
                "{name} reached no certifier verdict and is not a listed emitter gap — \
                 the differential is not covering it"
            );
            CertifierVerdict::NoVerdict
        };
        match pair_verdicts(certifier, resolution.is_ok()) {
            Pairing::AgreedSound => report.agreed_sound += 1,
            Pairing::AgreedUnsound => report.agreed_unsound += 1,
            Pairing::Skipped => report.skipped_gaps += 1,
            Pairing::CertifiedButRefused => {
                let detail = resolution.as_ref().err().map_or_else(
                    || "lowering refused".to_owned(),
                    |diagnostic| {
                        format!(
                            "the mirror PROVES its anaphora resolve uniquely, but \
                             lowering refused: {diagnostic}"
                        )
                    },
                );
                report
                    .disagreements
                    .push(Disagreement { card: name, detail });
            }
            Pairing::RefutedButResolved => report.disagreements.push(Disagreement {
                card: name,
                detail: "the mirror REFUTES this card's anaphora, but lowering \
                         resolved it without complaint"
                    .to_owned(),
            }),
        }
    }

    println!(
        "{}: certifier/resolver differential — {} agreed sound, {} agreed unsound, \
         {} skipped (no certifier verdict)",
        plugin_dir.display(),
        report.agreed_sound,
        report.agreed_unsound,
        report.skipped_gaps,
    );
    if report.disagreements.is_empty() {
        println!("differential OK: 0 disagreements");
        return Ok(());
    }
    for disagreement in &report.disagreements {
        println!("  {}: {}", disagreement.card, disagreement.detail);
    }
    anyhow::bail!(
        "{} card(s) on which lowering's resolution and the Idris mirror's certification disagree",
        report.disagreements.len()
    )
}

/// Every finished card source file in a plugin, in a stable order.
fn card_sources(plugin_dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    deckmaste_plugin::plugin::ron_files_recursive(
        &plugin_dir.join(deckmaste_core::plugin::CARDS_DIR),
    )
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
/// `module IdrisCheckDeps\nimport Semantics\n\n`, no card defs.
///
/// Without this, a `Semantics.idr` that doesn't compile makes *every* batch
/// and the per-card isolation fallback (meant for one genuinely-unsound card)
/// re-checks *every* card, each failing identically for the same dependency
/// error — ~72 × 101 ≈ 7200 `idris2` invocations for the wizards corpus, all
/// burying the one real cause. A broken dependency is O(1) to detect up front;
/// left to the failure path it is *rediscovered* N times.
///
/// It also warms `Semantics.ttc` (built once here) so the real batches — and
/// every isolation re-check — reuse it instead of rebuilding the mirror from
/// source.
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
             (Semantics.idr and whatever the probe module imports). No card is at fault — \
             fix the dependency build first.\n\nidris2 said:\n{output}"
        ),
    }
}

fn run_single(plugin: &Plugin, card_name: &str, idris_dir: &Path) -> anyhow::Result<()> {
    // The SEMANTIC term, expanded to the kernel's normal form — the mirror
    // certifies semantic input, not the engine image
    // (`docs/decisions/semantics-spelling-lowering.md` §10).
    let card = plugin
        .card(card_name)
        .with_context(|| format!("loading card {card_name:?}"))?
        .semantic;
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
    bless: bool,
) -> anyhow::Result<()> {
    let baseline_path = plugin_dir.join(BASELINE_FILE);
    let baseline = if bless { None } else { Some(read_baseline(&baseline_path)?) };
    let report = collect_batch_report(plugin, plugin_dir, idris_dir, batch_size)?;
    print_batch_report(plugin_dir, &report);

    if bless {
        anyhow::ensure!(
            report.proof_failures.is_empty(),
            "refusing to bless {} Idris proof failure(s)",
            report.proof_failures.len()
        );
        let baseline = baseline_from_report(&report);
        let text = render_baseline(&baseline)?;
        fs::write(&baseline_path, text)
            .with_context(|| format!("writing {}", baseline_path.display()))?;
        println!(
            "\nblessed {}: {} required pass(es), {} known emitter gap(s)",
            baseline_path.display(),
            baseline.passes.len(),
            baseline.gaps.len()
        );
        return Ok(());
    }

    let baseline = baseline.context("internal error: batch baseline was not loaded")?;
    let comparison = compare_baseline(&report, &baseline);
    report_comparison(&comparison, &baseline_path)?;
    println!(
        "\nIdris baseline OK: {} required pass(es), {} known emitter gap(s)",
        baseline.passes.len(),
        baseline.gaps.len()
    );
    Ok(())
}

fn collect_batch_report(
    plugin: &Plugin,
    plugin_dir: &Path,
    idris_dir: &Path,
    batch_size: usize,
) -> anyhow::Result<BatchReport> {
    let cards = idris_emit::load_all_cards(plugin_dir, plugin)
        .with_context(|| format!("loading cards from {}", plugin_dir.display()))?;
    let total = cards.len();

    // One (name, ident) assignment per card, computed ONCE so every later
    // step (emission, reporting proof failures by name) agrees on it.
    let mut used_idents: HashSet<String> = HashSet::new();
    let named_cards: Vec<(String, String, Card)> = cards
        .iter()
        .map(|card| {
            let card = card.semantic.clone();
            let name = idris_emit::card_display_name(&card).to_string();
            let mut ident = idris_emit::sanitize_ident(&name);
            while !used_idents.insert(ident.clone()) {
                ident.push('_');
            }
            (name, ident, card)
        })
        .collect();
    let mut names: HashMap<String, String> = HashMap::new();

    // Emission is pure Rust — do it all up front, splitting into what can be
    // typechecked (emitted fine) and what can't (gaps), so the batch loop
    // below only ever hands idris2 already-emitted, valid Idris source.
    let mut emitted: Vec<(String, String)> = Vec::new(); // (ident, expr)
    let mut gaps: Vec<BaselineGap> = Vec::new();
    for (name, ident, card) in named_cards {
        names.insert(ident.clone(), name.clone());
        match idris_emit::emit_card_expr(&card, plugin) {
            Ok(expr) => emitted.push((ident, expr)),
            Err(gap) => gaps.push(BaselineGap {
                card: name,
                reason: gap.to_string(),
            }),
        }
    }

    let mut passes = Vec::new();
    let mut proof_failures = Vec::new();

    for (batch_idx, chunk) in emitted.chunks(batch_size.max(1)).enumerate() {
        let module_name = format!("IdrisCheckBatch_{batch_idx}");
        let source = render_module(&module_name, chunk);
        let outcome = typecheck_module(idris_dir, &module_name, &source)?;
        match outcome {
            TypecheckOutcome::Pass => {
                for (ident, _) in chunk {
                    passes.push(
                        names.get(ident).cloned().with_context(|| {
                            format!("missing card name for Idris ident {ident}")
                        })?,
                    );
                }
            }
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
                        TypecheckOutcome::Pass => {
                            passes.push(names.get(ident).cloned().with_context(|| {
                                format!("missing card name for Idris ident {ident}")
                            })?);
                        }
                        TypecheckOutcome::Fail(single_output) => {
                            let name = names.get(ident).cloned().unwrap_or_else(|| ident.clone());
                            proof_failures.push(ProofFailure {
                                card: name,
                                output: single_output,
                            });
                        }
                    }
                }
            }
        }
    }

    passes.sort();
    gaps.sort();
    proof_failures.sort_by(|a, b| a.card.cmp(&b.card));
    Ok(BatchReport {
        total,
        passes,
        gaps,
        proof_failures,
    })
}

fn print_batch_report(plugin_dir: &Path, report: &BatchReport) {
    println!(
        "{}: {}/{} cards emit and typecheck",
        plugin_dir.display(),
        report.passes.len(),
        report.total
    );

    if !report.gaps.is_empty() {
        println!(
            "\n{} card(s) fail to EMIT (emitter gap — a Rust grammar shape not yet mapped):",
            report.gaps.len()
        );
        for gap in &report.gaps {
            println!("  {}: {}", gap.card, gap.reason);
        }
    }

    if !report.proof_failures.is_empty() {
        println!(
            "\n{} card(s) emit but FAIL the Idris proof (interesting: either genuinely unsound or an over-strict Idris proof):",
            report.proof_failures.len()
        );
        for failure in &report.proof_failures {
            println!("  {}:", failure.card);
            for line in failure.output.lines() {
                println!("    {line}");
            }
        }
    }
}

fn baseline_from_report(report: &BatchReport) -> Baseline {
    Baseline {
        version: BASELINE_VERSION,
        passes: report.passes.clone(),
        gaps: report.gaps.clone(),
    }
}

fn compare_baseline(report: &BatchReport, baseline: &Baseline) -> BaselineComparison {
    let current_passes: BTreeSet<&str> = report.passes.iter().map(String::as_str).collect();
    let current_cards: BTreeSet<&str> = report
        .passes
        .iter()
        .map(String::as_str)
        .chain(report.gaps.iter().map(|gap| gap.card.as_str()))
        .chain(
            report
                .proof_failures
                .iter()
                .map(|failure| failure.card.as_str()),
        )
        .collect();
    let baseline_passes: BTreeSet<&str> = baseline.passes.iter().map(String::as_str).collect();
    let baseline_gap_cards: BTreeSet<&str> =
        baseline.gaps.iter().map(|gap| gap.card.as_str()).collect();
    let baseline_gaps: BTreeSet<(&str, &str)> = baseline
        .gaps
        .iter()
        .map(|gap| (gap.card.as_str(), gap.reason.as_str()))
        .collect();

    let lost_passes = baseline
        .passes
        .iter()
        .filter(|card| {
            current_cards.contains(card.as_str()) && !current_passes.contains(card.as_str())
        })
        .cloned()
        .collect();
    let new_gaps = report
        .gaps
        .iter()
        .filter(|gap| !baseline_gaps.contains(&(gap.card.as_str(), gap.reason.as_str())))
        .cloned()
        .collect();
    let proof_failures = report
        .proof_failures
        .iter()
        .map(|failure| failure.card.clone())
        .collect();
    let missing_cards = baseline
        .passes
        .iter()
        .map(String::as_str)
        .chain(baseline.gaps.iter().map(|gap| gap.card.as_str()))
        .filter(|card| !current_cards.contains(card))
        .map(str::to_owned)
        .collect();
    let resolved_gaps = baseline
        .gaps
        .iter()
        .filter(|gap| current_passes.contains(gap.card.as_str()))
        .map(|gap| gap.card.clone())
        .collect();
    let new_passes = report
        .passes
        .iter()
        .filter(|card| {
            !baseline_passes.contains(card.as_str()) && !baseline_gap_cards.contains(card.as_str())
        })
        .cloned()
        .collect();

    BaselineComparison {
        lost_passes,
        new_gaps,
        proof_failures,
        missing_cards,
        resolved_gaps,
        new_passes,
    }
}

fn report_comparison(comparison: &BaselineComparison, baseline_path: &Path) -> anyhow::Result<()> {
    for card in &comparison.resolved_gaps {
        println!("Idris baseline improvement: known gap now passes: {card}");
    }
    for card in &comparison.new_passes {
        println!("Idris baseline improvement: new card passes: {card}");
    }
    if !comparison.resolved_gaps.is_empty() || !comparison.new_passes.is_empty() {
        println!(
            "review the improvements, then run with --bless to ratchet {}",
            baseline_path.display()
        );
    }

    for card in &comparison.lost_passes {
        eprintln!("Idris baseline REGRESSION: lost pass: {card}");
    }
    for gap in &comparison.new_gaps {
        eprintln!(
            "Idris baseline REGRESSION: new emitter gap: {}: {}",
            gap.card, gap.reason
        );
    }
    for card in &comparison.proof_failures {
        eprintln!("Idris baseline REGRESSION: proof failure: {card}");
    }
    for card in &comparison.missing_cards {
        eprintln!("Idris baseline REGRESSION: card disappeared: {card}");
    }

    let regression_count = comparison.regression_count();
    anyhow::ensure!(
        regression_count == 0,
        "{regression_count} Idris mirror regression(s); restore the cards/passes, fix the failures, or review emitter gaps and re-run with --bless"
    );

    let update_count = comparison.baseline_update_count();
    anyhow::ensure!(
        update_count == 0,
        "{update_count} Idris mirror improvement(s) require a baseline refresh; review them and re-run with --bless"
    );
    Ok(())
}

fn read_baseline(path: &Path) -> anyhow::Result<Baseline> {
    let text = fs::read_to_string(path).with_context(|| {
        format!(
            "reading {} — run `cargo xtask idris-check <plugin> --bless` first",
            path.display()
        )
    })?;
    parse_baseline_text(&text).with_context(|| format!("reading {}", path.display()))
}

fn parse_baseline_text(text: &str) -> anyhow::Result<Baseline> {
    let baseline: Baseline = ron::from_str(text).context("parsing baseline RON")?;
    validate_baseline(&baseline)?;
    let canonical = render_baseline(&baseline)?;
    anyhow::ensure!(
        canonical == text,
        "baseline is not in canonical deterministic format; regenerate it with --bless"
    );
    Ok(baseline)
}

fn validate_baseline(baseline: &Baseline) -> anyhow::Result<()> {
    anyhow::ensure!(
        baseline.version == BASELINE_VERSION,
        "unsupported baseline version {}; expected {BASELINE_VERSION}",
        baseline.version
    );

    let mut cards = BTreeSet::new();
    for card in &baseline.passes {
        anyhow::ensure!(
            !card.trim().is_empty(),
            "baseline pass name must not be empty"
        );
        anyhow::ensure!(
            cards.insert(card.as_str()),
            "duplicate baseline card: {card}"
        );
    }
    for gap in &baseline.gaps {
        anyhow::ensure!(
            !gap.card.trim().is_empty(),
            "baseline gap card must not be empty"
        );
        anyhow::ensure!(
            !gap.reason.trim().is_empty(),
            "baseline gap reason for {} must not be empty",
            gap.card
        );
        anyhow::ensure!(
            cards.insert(gap.card.as_str()),
            "duplicate baseline card: {}",
            gap.card
        );
    }
    Ok(())
}

fn render_baseline(baseline: &Baseline) -> anyhow::Result<String> {
    use std::fmt::Write as _;

    let mut passes = baseline.passes.clone();
    let mut gaps = baseline.gaps.clone();
    passes.sort();
    gaps.sort();

    let mut out = String::from(
        "// Generated by `cargo xtask idris-check <plugin> --bless`; do not edit.\n(\n",
    );
    let _ = writeln!(out, "    version: {},", baseline.version);
    out.push_str("    passes: [\n");
    for card in passes {
        let card = ron::to_string(&card)?;
        let _ = writeln!(out, "        {card},");
    }
    out.push_str("    ],\n    gaps: [\n");
    for gap in gaps {
        let card = ron::to_string(&gap.card)?;
        let reason = ron::to_string(&gap.reason)?;
        out.push_str("        (\n");
        let _ = writeln!(out, "            card: {card},");
        let _ = writeln!(out, "            reason: {reason},");
        out.push_str("        ),\n");
    }
    out.push_str("    ],\n)\n");
    Ok(out)
}

fn render_module(module_name: &str, defs: &[(String, String)]) -> String {
    use std::fmt::Write as _;

    let mut out = format!("module {module_name}\nimport Semantics\n\n");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_format_is_sorted_canonical_ron_and_round_trips() {
        let baseline = Baseline {
            version: BASELINE_VERSION,
            passes: vec!["Beta".into(), "Alpha".into()],
            gaps: vec![BaselineGap {
                card: "Gap Card".into(),
                reason: "unsupported shape".into(),
            }],
        };

        let text = render_baseline(&baseline).unwrap();
        assert_eq!(
            text,
            r#"// Generated by `cargo xtask idris-check <plugin> --bless`; do not edit.
(
    version: 1,
    passes: [
        "Alpha",
        "Beta",
    ],
    gaps: [
        (
            card: "Gap Card",
            reason: "unsupported shape",
        ),
    ],
)
"#
        );
        assert_eq!(
            parse_baseline_text(&text).unwrap(),
            Baseline {
                version: BASELINE_VERSION,
                passes: vec!["Alpha".into(), "Beta".into()],
                gaps: vec![BaselineGap {
                    card: "Gap Card".into(),
                    reason: "unsupported shape".into(),
                }],
            }
        );
    }

    #[test]
    fn baseline_format_rejects_noncanonical_order() {
        let text = "// Generated by `cargo xtask idris-check <plugin> --bless`; do not edit.\n\
(\n\
    version: 1,\n\
    passes: [\"Beta\", \"Alpha\"],\n\
    gaps: [],\n\
)\n";
        let error = parse_baseline_text(text).unwrap_err().to_string();
        assert!(error.contains("canonical deterministic format"));
    }

    #[test]
    fn comparison_reports_improvements_that_require_refresh() {
        let baseline = Baseline {
            version: BASELINE_VERSION,
            passes: vec!["Always Passed".into()],
            gaps: vec![
                BaselineGap {
                    card: "Known Gap".into(),
                    reason: "known shape".into(),
                },
                BaselineGap {
                    card: "Resolved Gap".into(),
                    reason: "old shape".into(),
                },
            ],
        };
        let report = BatchReport {
            total: 4,
            passes: vec![
                "Always Passed".into(),
                "New Pass".into(),
                "Resolved Gap".into(),
            ],
            gaps: vec![BaselineGap {
                card: "Known Gap".into(),
                reason: "known shape".into(),
            }],
            proof_failures: vec![],
        };

        let comparison = compare_baseline(&report, &baseline);
        assert_eq!(
            comparison,
            BaselineComparison {
                lost_passes: vec![],
                new_gaps: vec![],
                proof_failures: vec![],
                missing_cards: vec![],
                resolved_gaps: vec!["Resolved Gap".into()],
                new_passes: vec!["New Pass".into()],
            }
        );
        assert!(report_comparison(&comparison, Path::new("baseline.ron")).is_err());
    }

    #[test]
    fn comparison_accepts_exact_current_classification() {
        let baseline = Baseline {
            version: BASELINE_VERSION,
            passes: vec!["Pass".into()],
            gaps: vec![BaselineGap {
                card: "Known Gap".into(),
                reason: "known shape".into(),
            }],
        };
        let report = BatchReport {
            total: 2,
            passes: vec!["Pass".into()],
            gaps: baseline.gaps.clone(),
            proof_failures: vec![],
        };

        let comparison = compare_baseline(&report, &baseline);
        assert_eq!(comparison.regression_count(), 0);
        assert_eq!(comparison.baseline_update_count(), 0);
        report_comparison(&comparison, Path::new("baseline.ron")).unwrap();
    }

    #[test]
    fn comparison_rejects_lost_passes_new_gaps_and_proof_failures() {
        let baseline = Baseline {
            version: BASELINE_VERSION,
            passes: vec!["Lost Pass".into()],
            gaps: vec![BaselineGap {
                card: "Known Gap".into(),
                reason: "known shape".into(),
            }],
        };
        let report = BatchReport {
            total: 4,
            passes: vec![],
            gaps: vec![
                BaselineGap {
                    card: "Lost Pass".into(),
                    reason: "newly unsupported".into(),
                },
                BaselineGap {
                    card: "Known Gap".into(),
                    reason: "known shape".into(),
                },
                BaselineGap {
                    card: "New Gap".into(),
                    reason: "new shape".into(),
                },
            ],
            proof_failures: vec![ProofFailure {
                card: "Proof Failure".into(),
                output: "type error".into(),
            }],
        };

        assert_eq!(
            compare_baseline(&report, &baseline),
            BaselineComparison {
                lost_passes: vec!["Lost Pass".into()],
                new_gaps: vec![
                    BaselineGap {
                        card: "Lost Pass".into(),
                        reason: "newly unsupported".into(),
                    },
                    BaselineGap {
                        card: "New Gap".into(),
                        reason: "new shape".into(),
                    },
                ],
                proof_failures: vec!["Proof Failure".into()],
                missing_cards: vec![],
                resolved_gaps: vec![],
                new_passes: vec![],
            }
        );
    }

    #[test]
    fn missing_known_gap_is_rejected() {
        let baseline = Baseline {
            version: BASELINE_VERSION,
            passes: vec![],
            gaps: vec![BaselineGap {
                card: "Deleted Gap".into(),
                reason: "known shape".into(),
            }],
        };
        let report = BatchReport {
            total: 0,
            passes: vec![],
            gaps: vec![],
            proof_failures: vec![],
        };

        let comparison = compare_baseline(&report, &baseline);
        assert_eq!(comparison.missing_cards, vec!["Deleted Gap"]);
        assert!(report_comparison(&comparison, Path::new("baseline.ron")).is_err());
    }

    #[test]
    fn changed_reason_is_a_new_gap_not_a_known_one() {
        let baseline = Baseline {
            version: BASELINE_VERSION,
            passes: vec![],
            gaps: vec![BaselineGap {
                card: "Gap Card".into(),
                reason: "old shape".into(),
            }],
        };
        let report = BatchReport {
            total: 1,
            passes: vec![],
            gaps: vec![BaselineGap {
                card: "Gap Card".into(),
                reason: "different unsupported shape".into(),
            }],
            proof_failures: vec![],
        };

        assert_eq!(
            compare_baseline(&report, &baseline).new_gaps,
            vec![BaselineGap {
                card: "Gap Card".into(),
                reason: "different unsupported shape".into(),
            }]
        );
    }
}

#[cfg(test)]
mod differential_tests {
    use super::CertifierVerdict;
    use super::Pairing;
    use super::pair_verdicts;

    /// The differential AGREES when both derivations reach the same verdict —
    /// the shape the canon slice is in today.
    #[test]
    fn matching_verdicts_agree() {
        assert_eq!(
            pair_verdicts(CertifierVerdict::Sound, true),
            Pairing::AgreedSound
        );
        assert_eq!(
            pair_verdicts(CertifierVerdict::Unsound, false),
            Pairing::AgreedUnsound
        );
    }

    /// The gate REPORTS a disagreement in both directions. Without this the
    /// "0 disagreements" line would be indistinguishable from a gate that
    /// cannot fail.
    #[test]
    fn opposed_verdicts_are_disagreements() {
        assert_eq!(
            pair_verdicts(CertifierVerdict::Sound, false),
            Pairing::CertifiedButRefused,
            "the mirror proved the anaphora resolve; lowering must not refuse"
        );
        assert_eq!(
            pair_verdicts(CertifierVerdict::Unsound, true),
            Pairing::RefutedButResolved,
            "the mirror refuted the anaphora; lowering must not resolve them"
        );
    }

    /// An emitter gap is not a claim about the card, so it can never be a
    /// disagreement — in either resolver direction.
    #[test]
    fn an_emitter_gap_is_never_a_disagreement() {
        for resolved in [true, false] {
            assert_eq!(
                pair_verdicts(CertifierVerdict::NoVerdict, resolved),
                Pairing::Skipped
            );
        }
    }
}
