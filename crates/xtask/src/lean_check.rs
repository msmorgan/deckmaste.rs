//! `cargo xtask lean-check` — the card soundness gate: re-emit every
//! `plugins_v2` card as a fully expanded Lean term and ask the kernel to prove
//! its refusal list empty.
//!
//! `lean/Semantics` is the semantics of the card language and `Card.check`
//! returns every structural obligation a card breaks, so a card whose
//! `Card.check = []` is proved by `decide` is sound against the modelled laws
//! (`docs/decisions/semantics-v2.md` §13). This command writes the emitted
//! terms as untracked Lean under `lean/Generated/`, builds them with `lake`,
//! attributes each diagnostic back to the card whose block it landed in, and
//! ratchets the verdicts against a per-plugin baseline.
//!
//! There is no gap concept. The v2 mirror is total — every card the reader
//! loads emits — so a card is either proved or a gate failure, never an
//! unmapped shape. That is the substantive difference from the Idris gate this
//! replaces, whose partial emitter needed a third verdict.
//!
//! `Macros.lean` plays no part: the emitted term is post-expansion, so it is
//! raw constructors, and the `spelled` elaborator (which refuses raw
//! constructors by design) is the hand bench's law, not the gate's.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use anyhow::Context;
use clap::Args;
use deckmaste_semantics_v2::card::Card;
use deckmaste_semantics_v2::lean_emit;
use deckmaste_semantics_v2::reader::CARDS_DIR;
use deckmaste_semantics_v2::reader::Plugin;
use serde::Deserialize;
use serde::Serialize;

/// The per-plugin verdict ratchet.
const BASELINE_FILE: &str = "lean-check-baseline.ron";
const BASELINE_VERSION: u32 = 1;

/// The generated library's root module and directory, relative to `lean/`.
const GENERATED_ROOT: &str = "Generated.lean";
const GENERATED_DIR: &str = "Generated";
/// The `lake` target the generated library builds under. It is deliberately
/// absent from `lakefile.toml`'s `defaultTargets`, so `lean/scripts/build`
/// remains the hand workbench's gate and succeeds with no `Generated/`
/// present.
const GENERATED_TARGET: &str = "Generated";

#[derive(Debug, Args)]
pub struct LeanCheckArgs {
    /// The plugin directories to check. Defaults to every plugin under
    /// `plugins_v2/` that has a `cards/` directory.
    plugin_dirs: Vec<PathBuf>,
    /// Rewrite each plugin's baseline from this run's verdicts, after
    /// reviewing them.
    #[arg(long)]
    bless: bool,
}

impl LeanCheckArgs {
    /// The gate's arguments, for callers that drive it directly (its own
    /// integration test).
    #[must_use]
    pub fn new(plugin_dirs: Vec<PathBuf>, bless: bool) -> Self {
        LeanCheckArgs { plugin_dirs, bless }
    }
}

/// What the kernel concluded about one card.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Verdict {
    /// `Card.check` is provably empty.
    Pass,
    /// The emitted term did not build, or `decide` refuted the theorem. The
    /// reason is the head of the Lean diagnostic.
    Fail { reason: String },
}

/// One plugin's ratchet: a verdict per card, keyed by the name the reader
/// loaded it under.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Baseline {
    version: u32,
    cards: Vec<BaselineEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BaselineEntry {
    card: String,
    verdict: Verdict,
}

/// One plugin's run: where it lives and what each of its cards proved.
#[derive(Debug, Clone)]
struct PluginReport {
    dir: PathBuf,
    /// The Lean module the plugin's cards were emitted into.
    module: String,
    verdicts: BTreeMap<String, Verdict>,
    /// The full Lean diagnostics behind every failing verdict, for the console.
    diagnostics: BTreeMap<String, Vec<String>>,
}

/// One parsed Lean diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    /// The path Lean reported, relative to `lean/`.
    path: String,
    line: usize,
    /// The whole diagnostic line, for the console.
    text: String,
    /// The message head — the diagnostic's own first line — which is what the
    /// baseline records.
    head: String,
}

/// # Errors
/// If a plugin fails to load or emit, if `lake` cannot be run, or if any
/// plugin's verdicts differ from its baseline (in either direction) without
/// `--bless`.
pub fn run(args: &LeanCheckArgs) -> anyhow::Result<()> {
    let started = Instant::now();
    let lean_dir = lean_root()?;
    let plugin_dirs = if args.plugin_dirs.is_empty() {
        default_plugin_dirs(&workspace_root()?)?
    } else {
        args.plugin_dirs.clone()
    };
    anyhow::ensure!(
        !plugin_dirs.is_empty(),
        "no plugin with a `{CARDS_DIR}/` directory to check"
    );

    let modules = emit(&lean_dir, &plugin_dirs)?;
    let output = build(&lean_dir)?;
    let mut reports = attribute(&modules, &output)?;
    reports.sort_by(|a, b| a.dir.cmp(&b.dir));

    let mut failures = 0usize;
    for report in &reports {
        print_report(report);
        if let Err(error) = ratchet(report, args.bless) {
            eprintln!("{error:#}");
            failures += 1;
        }
    }
    println!(
        "\nlean-check: {} plugin(s), {} card(s), {:.1}s",
        reports.len(),
        reports.iter().map(|r| r.verdicts.len()).sum::<usize>(),
        started.elapsed().as_secs_f64()
    );
    anyhow::ensure!(failures == 0, "{failures} plugin(s) off their baseline");
    Ok(())
}

// ---------------------------------------------------------------------------
// Emission
// ---------------------------------------------------------------------------

/// One emitted module: which plugin it came from and where each card sits.
struct EmittedModule {
    dir: PathBuf,
    module: String,
    path: String,
    cards: Vec<lean_emit::GeneratedCard>,
}

/// Writes one Lean module per plugin plus the root that imports them, having
/// first cleared whatever a previous run left. The generated tree is
/// untracked (`.gitignore`), so a run always starts from the plugins as they
/// are now.
fn emit(lean_dir: &Path, plugin_dirs: &[PathBuf]) -> anyhow::Result<Vec<EmittedModule>> {
    let generated = lean_dir.join(GENERATED_DIR);
    if generated.exists() {
        fs::remove_dir_all(&generated)
            .with_context(|| format!("clearing {}", generated.display()))?;
    }
    fs::create_dir_all(&generated).with_context(|| format!("creating {}", generated.display()))?;

    let prelude = prelude_plugin(plugin_dirs)?;
    let mut modules = Vec::new();
    for dir in plugin_dirs {
        let plugin = load_plugin(prelude.as_ref(), dir)?;
        let component = module_component(dir)?;
        let module = format!("{GENERATED_DIR}.{component}");
        let cards: Vec<(String, &Card)> = plugin
            .cards
            .iter()
            .map(|(name, card)| (name.clone(), card))
            .collect();
        let rendered = lean_emit::render_module(&module, &cards)
            .with_context(|| format!("emitting {} as Lean", dir.display()))?;
        let file = generated.join(format!("{component}.lean"));
        fs::write(&file, &rendered.source)
            .with_context(|| format!("writing {}", file.display()))?;
        modules.push(EmittedModule {
            dir: dir.clone(),
            module: module.clone(),
            path: format!("{GENERATED_DIR}/{component}.lean"),
            cards: rendered.cards,
        });
    }

    let mut root = String::new();
    for module in &modules {
        use std::fmt::Write as _;
        let _ = writeln!(root, "import {}", module.module);
    }
    let root_path = lean_dir.join(GENERATED_ROOT);
    fs::write(&root_path, root).with_context(|| format!("writing {}", root_path.display()))?;
    Ok(modules)
}

/// `plugins_v2/builtin` as the scope every other plugin's declarations load
/// over; it is the sole v2 registry (`docs/decisions/semantics-v2.md` §15).
fn prelude_plugin(plugin_dirs: &[PathBuf]) -> anyhow::Result<Option<Plugin>> {
    let builtin = workspace_root()?.join("plugins_v2").join("builtin");
    if !builtin.is_dir() || plugin_dirs.iter().any(|dir| same_dir(dir, &builtin)) {
        return Ok(None);
    }
    Ok(Some(Plugin::load(&builtin).with_context(|| {
        format!("loading the prelude {}", builtin.display())
    })?))
}

fn load_plugin(prelude: Option<&Plugin>, dir: &Path) -> anyhow::Result<Plugin> {
    let loaded = match prelude {
        Some(prelude) => Plugin::load_with_prelude(prelude, dir),
        None => Plugin::load(dir),
    };
    loaded.with_context(|| format!("loading plugin {}", dir.display()))
}

fn same_dir(left: &Path, right: &Path) -> bool {
    match (left.canonicalize(), right.canonicalize()) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}

/// A plugin directory's name as one Lean module component: `testing` becomes
/// `Testing`, `plugins_v2_lawless` becomes `PluginsV2Lawless`.
fn module_component(dir: &Path) -> anyhow::Result<String> {
    let name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .with_context(|| format!("{} has no directory name", dir.display()))?;
    let mut out = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            if capitalize {
                out.extend(ch.to_uppercase());
                capitalize = false;
            } else {
                out.push(ch);
            }
        } else {
            capitalize = true;
        }
    }
    anyhow::ensure!(
        !out.is_empty() && !out.starts_with(|ch: char| ch.is_ascii_digit()),
        "{} does not name a Lean module component",
        dir.display()
    );
    Ok(out)
}

// ---------------------------------------------------------------------------
// The Lean build
// ---------------------------------------------------------------------------

/// Builds the generated library. A nonzero exit is expected whenever a card
/// fails, so the output — not the status — is the verdict; only a failure to
/// RUN `lake` is an error here.
fn build(lean_dir: &Path) -> anyhow::Result<String> {
    let output = Command::new("lake")
        .args(["build", "--wfail", GENERATED_TARGET])
        .current_dir(lean_dir)
        .output()
        .context("running lake (is it on PATH?)")?;
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(text)
}

/// Parses `lake`'s output into diagnostics that name a generated file.
///
/// Lean reports `error: <path>:<line>:<col>: <message>`, the message running
/// on over following lines; lake adds its own pathless lines (`error: build
/// failed`) which name no card and are dropped here — an unbuildable module is
/// caught by [`attribute`], which refuses a diagnostic it cannot place.
fn diagnostics(output: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for line in output.lines() {
        let Some(rest) = line
            .strip_prefix("error: ")
            .or_else(|| line.strip_prefix("warning: "))
        else {
            continue;
        };
        let Some((path, rest)) = rest.split_once(".lean:") else {
            continue;
        };
        let path = format!("{path}.lean");
        let mut parts = rest.splitn(3, ':');
        let Some(Ok(number)) = parts.next().map(str::parse::<usize>) else {
            continue;
        };
        if parts
            .next()
            .and_then(|col| col.parse::<usize>().ok())
            .is_none()
        {
            continue;
        }
        let head = parts.next().unwrap_or_default().trim().to_owned();
        out.push(Diagnostic {
            path,
            line: number,
            text: line.to_owned(),
            head,
        });
    }
    out
}

/// Places each diagnostic on the card whose block it landed in.
///
/// # Errors
/// If a diagnostic names a generated file but sits above its first card — the
/// module header itself did not elaborate, which is a defect in the gate
/// rather than a verdict about any card.
fn attribute(modules: &[EmittedModule], output: &str) -> anyhow::Result<Vec<PluginReport>> {
    let parsed = diagnostics(output);
    let mut reports = Vec::new();
    for module in modules {
        let mut verdicts: BTreeMap<String, Verdict> = module
            .cards
            .iter()
            .map(|card| (card.name.clone(), Verdict::Pass))
            .collect();
        let mut collected: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for diagnostic in parsed.iter().filter(|d| d.path.ends_with(&module.path)) {
            let owner = module
                .cards
                .iter()
                .rfind(|card| card.start_line <= diagnostic.line)
                .with_context(|| {
                    format!(
                        "{}:{}: {} — the diagnostic is above the module's first card, so the \
                         generated header itself did not elaborate; this is a gate defect, not a \
                         card verdict",
                        diagnostic.path, diagnostic.line, diagnostic.head
                    )
                })?;
            verdicts.insert(
                owner.name.clone(),
                Verdict::Fail {
                    reason: diagnostic.head.clone(),
                },
            );
            collected
                .entry(owner.name.clone())
                .or_default()
                .push(diagnostic.text.clone());
        }
        reports.push(PluginReport {
            dir: module.dir.clone(),
            module: module.module.clone(),
            verdicts,
            diagnostics: collected,
        });
    }
    Ok(reports)
}

// ---------------------------------------------------------------------------
// The ratchet
// ---------------------------------------------------------------------------

fn print_report(report: &PluginReport) {
    let passes = report
        .verdicts
        .values()
        .filter(|verdict| **verdict == Verdict::Pass)
        .count();
    println!(
        "{}: {passes}/{} card(s) prove `Card.check = []` ({})",
        report.dir.display(),
        report.verdicts.len(),
        report.module
    );
    for (card, lines) in &report.diagnostics {
        println!("  {card}:");
        for line in lines {
            println!("    {line}");
        }
    }
}

/// Compares a plugin's verdicts with its baseline and, with `bless`, rewrites
/// it. The ratchet is add-only in both directions: a lost pass and a resolved
/// failure both stop the command, one as a regression and one as an
/// improvement that wants review.
fn ratchet(report: &PluginReport, bless: bool) -> anyhow::Result<()> {
    let path = report.dir.join(BASELINE_FILE);
    let current = Baseline {
        version: BASELINE_VERSION,
        cards: report
            .verdicts
            .iter()
            .map(|(card, verdict)| BaselineEntry {
                card: card.clone(),
                verdict: verdict.clone(),
            })
            .collect(),
    };
    if bless {
        let text = render_baseline(&current)?;
        fs::write(&path, text).with_context(|| format!("writing {}", path.display()))?;
        let failures = current
            .cards
            .iter()
            .filter(|entry| entry.verdict != Verdict::Pass)
            .count();
        println!(
            "blessed {}: {} card(s), {failures} recorded failure(s)",
            path.display(),
            current.cards.len()
        );
        return Ok(());
    }

    let baseline = read_baseline(&path)?;
    let recorded: BTreeMap<&str, &Verdict> = baseline
        .cards
        .iter()
        .map(|entry| (entry.card.as_str(), &entry.verdict))
        .collect();
    let mut regressions = Vec::new();
    let mut improvements = Vec::new();
    for (card, verdict) in &report.verdicts {
        match (recorded.get(card.as_str()), verdict) {
            (Some(Verdict::Pass), Verdict::Fail { reason }) => {
                regressions.push(format!("lost pass: {card}: {reason}"));
            }
            (Some(Verdict::Fail { .. }), Verdict::Pass) => {
                improvements.push(format!("recorded failure now passes: {card}"));
            }
            (Some(Verdict::Fail { reason: was }), Verdict::Fail { reason: now }) if was != now => {
                regressions.push(format!("failure changed: {card}: {was} -> {now}"));
            }
            (Some(Verdict::Pass), Verdict::Pass)
            | (Some(Verdict::Fail { .. }), Verdict::Fail { .. }) => {}
            (None, Verdict::Pass) => improvements.push(format!("card not in baseline: {card}")),
            (None, Verdict::Fail { reason }) => {
                regressions.push(format!(
                    "card not in baseline, and failing: {card}: {reason}"
                ));
            }
        }
    }
    let present: BTreeSet<&str> = report.verdicts.keys().map(String::as_str).collect();
    for entry in &baseline.cards {
        if !present.contains(entry.card.as_str()) {
            regressions.push(format!("card disappeared: {}", entry.card));
        }
    }

    for line in &regressions {
        eprintln!("{}: REGRESSION: {line}", report.dir.display());
    }
    for line in &improvements {
        println!("{}: improvement: {line}", report.dir.display());
    }
    anyhow::ensure!(
        regressions.is_empty() && improvements.is_empty(),
        "{}: {} regression(s) and {} improvement(s) against {}; fix the failures or review the \
         improvements and re-run with --bless",
        report.dir.display(),
        regressions.len(),
        improvements.len(),
        path.display()
    );
    println!(
        "{}: baseline OK ({} card(s))",
        report.dir.display(),
        baseline.cards.len()
    );
    Ok(())
}

fn read_baseline(path: &Path) -> anyhow::Result<Baseline> {
    let text = fs::read_to_string(path).with_context(|| {
        format!(
            "reading {} — run `cargo xtask lean-check <plugin> --bless` first",
            path.display()
        )
    })?;
    parse_baseline_text(&text).with_context(|| format!("reading {}", path.display()))
}

fn parse_baseline_text(text: &str) -> anyhow::Result<Baseline> {
    let baseline: Baseline = ron::from_str(text).context("parsing baseline RON")?;
    anyhow::ensure!(
        baseline.version == BASELINE_VERSION,
        "unsupported baseline version {}; expected {BASELINE_VERSION}",
        baseline.version
    );
    let mut seen = BTreeSet::new();
    for entry in &baseline.cards {
        anyhow::ensure!(
            !entry.card.trim().is_empty(),
            "a baseline card name must not be empty"
        );
        anyhow::ensure!(
            seen.insert(entry.card.as_str()),
            "duplicate baseline card: {}",
            entry.card
        );
        if let Verdict::Fail { reason } = &entry.verdict {
            anyhow::ensure!(
                !reason.trim().is_empty(),
                "the recorded failure for {} must name a reason",
                entry.card
            );
        }
    }
    anyhow::ensure!(
        render_baseline(&baseline)? == text,
        "baseline is not in canonical deterministic format; regenerate it with --bless"
    );
    Ok(baseline)
}

fn render_baseline(baseline: &Baseline) -> anyhow::Result<String> {
    use std::fmt::Write as _;

    let mut cards = baseline.cards.clone();
    cards.sort_by(|a, b| a.card.cmp(&b.card));
    let mut out = String::from(
        "// Generated by `cargo xtask lean-check <plugin> --bless`; do not edit.\n(\n",
    );
    let _ = writeln!(out, "    version: {},", baseline.version);
    out.push_str("    cards: [\n");
    for entry in cards {
        let card = ron::to_string(&entry.card)?;
        match entry.verdict {
            Verdict::Pass => {
                let _ = writeln!(out, "        (card: {card}, verdict: Pass),");
            }
            Verdict::Fail { reason } => {
                let reason = ron::to_string(&reason)?;
                out.push_str("        (\n");
                let _ = writeln!(out, "            card: {card},");
                let _ = writeln!(out, "            verdict: Fail(reason: {reason}),");
                out.push_str("        ),\n");
            }
        }
    }
    out.push_str("    ],\n)\n");
    Ok(out)
}

// ---------------------------------------------------------------------------
// Paths
// ---------------------------------------------------------------------------

fn workspace_root() -> anyhow::Result<PathBuf> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .context("xtask sits two directories below the workspace root")?
        .to_path_buf();
    anyhow::ensure!(
        dir.is_dir(),
        "expected a workspace root at {}",
        dir.display()
    );
    Ok(dir)
}

/// The workspace's `lean/` directory: `lake` runs from here so it reads
/// `lakefile.toml`.
fn lean_root() -> anyhow::Result<PathBuf> {
    let dir = workspace_root()?.join("lean");
    anyhow::ensure!(dir.is_dir(), "expected a lean/ dir at {}", dir.display());
    Ok(dir)
}

/// Every plugin under `plugins_v2/` that has cards. A plugin with only
/// declarations (`plugins_v2/builtin`) has nothing for the gate to prove and
/// owns no baseline.
fn default_plugin_dirs(workspace: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let root = workspace.join("plugins_v2");
    anyhow::ensure!(root.is_dir(), "expected {}", root.display());
    let mut out = Vec::new();
    for entry in fs::read_dir(&root).with_context(|| format!("reading {}", root.display()))? {
        let path = entry
            .with_context(|| format!("reading {}", root.display()))?
            .path();
        if path.is_dir() && path.join(CARDS_DIR).is_dir() {
            out.push(path);
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_plugin_directory_names_a_lean_module_component() {
        assert_eq!(
            module_component(Path::new("plugins_v2/testing")).unwrap(),
            "Testing"
        );
        assert_eq!(
            module_component(Path::new("a/plugins_v2_lawless")).unwrap(),
            "PluginsV2Lawless"
        );
        assert!(module_component(Path::new("a/9lives")).is_err());
    }

    /// The shape Lean reports a refuted `decide` in, taken from a real run.
    #[test]
    fn a_lean_diagnostic_is_parsed_to_its_file_line_and_head() {
        let output = "\u{2716} [23/25] Building Generated.Testing (522ms)\n\
             trace: .> LEAN_PATH=… lean Generated/Testing.lean\n\
             error: Generated/Testing.lean:14:81: Tactic `decide` proved that the proposition\n  \
             card_lawless_land.check = []\nis false\n\
             error: Lean exited with code 1\nerror: build failed\n";
        let parsed = diagnostics(output);
        assert_eq!(parsed.len(), 1, "{parsed:?}");
        assert_eq!(parsed[0].path, "Generated/Testing.lean");
        assert_eq!(parsed[0].line, 14);
        assert_eq!(
            parsed[0].head,
            "Tactic `decide` proved that the proposition"
        );
    }

    #[test]
    fn a_warning_is_a_diagnostic_too_because_the_build_treats_it_as_one() {
        let parsed = diagnostics("warning: Generated/Testing.lean:9:0: unused variable\n");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].line, 9);
        assert_eq!(parsed[0].head, "unused variable");
    }

    fn module(cards: &[(&str, usize)]) -> EmittedModule {
        EmittedModule {
            dir: PathBuf::from("plugins_v2/testing"),
            module: "Generated.Testing".to_owned(),
            path: "Generated/Testing.lean".to_owned(),
            cards: cards
                .iter()
                .map(|(name, start_line)| lean_emit::GeneratedCard {
                    name: (*name).to_owned(),
                    ident: lean_emit::card_ident(name),
                    start_line: *start_line,
                })
                .collect(),
        }
    }

    #[test]
    fn a_diagnostic_lands_on_the_card_whose_block_it_sits_in() {
        let modules = vec![module(&[("Alpha", 6), ("Beta", 11)])];
        let reports = attribute(
            &modules,
            "error: Generated/Testing.lean:13:4: Tactic `decide` proved that the proposition\n",
        )
        .unwrap();
        assert_eq!(reports[0].verdicts["Alpha"], Verdict::Pass);
        assert_eq!(
            reports[0].verdicts["Beta"],
            Verdict::Fail {
                reason: "Tactic `decide` proved that the proposition".to_owned()
            }
        );
    }

    /// A header that does not elaborate is the gate's own defect: reporting it
    /// as some card's failure would bless a broken emitter.
    #[test]
    fn a_diagnostic_above_the_first_card_is_a_gate_defect() {
        let modules = vec![module(&[("Alpha", 6)])];
        let error = attribute(
            &modules,
            "error: Generated/Testing.lean:2:0: unknown import\n",
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("gate defect"), "{error}");
    }

    fn report(verdicts: &[(&str, Verdict)]) -> PluginReport {
        PluginReport {
            dir: PathBuf::from("plugins_v2/testing"),
            module: "Generated.Testing".to_owned(),
            verdicts: verdicts
                .iter()
                .map(|(name, verdict)| ((*name).to_owned(), verdict.clone()))
                .collect(),
            diagnostics: BTreeMap::new(),
        }
    }

    fn failing(reason: &str) -> Verdict {
        Verdict::Fail {
            reason: reason.to_owned(),
        }
    }

    #[test]
    fn the_baseline_format_is_sorted_canonical_ron_and_round_trips() {
        let baseline = Baseline {
            version: BASELINE_VERSION,
            cards: vec![
                BaselineEntry {
                    card: "Beta".into(),
                    verdict: failing("decide refuted it"),
                },
                BaselineEntry {
                    card: "Alpha".into(),
                    verdict: Verdict::Pass,
                },
            ],
        };
        let text = render_baseline(&baseline).unwrap();
        assert_eq!(
            text,
            r#"// Generated by `cargo xtask lean-check <plugin> --bless`; do not edit.
(
    version: 1,
    cards: [
        (card: "Alpha", verdict: Pass),
        (
            card: "Beta",
            verdict: Fail(reason: "decide refuted it"),
        ),
    ],
)
"#
        );
        let parsed = parse_baseline_text(&text).unwrap();
        assert_eq!(parsed.cards.len(), 2);
        assert_eq!(parsed.cards[0].card, "Alpha");
    }

    #[test]
    fn a_noncanonical_baseline_is_refused() {
        let text = "(version: 1, cards: [(card: \"A\", verdict: Pass)])";
        let error = parse_baseline_text(text).unwrap_err().to_string();
        assert!(error.contains("canonical"), "{error}");
    }

    /// The ratchet is add-only in both directions, so every kind of difference
    /// stops the command; only an unchanged plugin is silent.
    #[test]
    fn the_ratchet_refuses_every_difference_and_accepts_an_unchanged_plugin() {
        let baseline = render_baseline(&Baseline {
            version: BASELINE_VERSION,
            cards: vec![
                BaselineEntry {
                    card: "Alpha".into(),
                    verdict: Verdict::Pass,
                },
                BaselineEntry {
                    card: "Beta".into(),
                    verdict: failing("known"),
                },
            ],
        })
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(BASELINE_FILE), &baseline).unwrap();

        let mut unchanged = report(&[("Alpha", Verdict::Pass), ("Beta", failing("known"))]);
        unchanged.dir = dir.path().to_path_buf();
        ratchet(&unchanged, false).expect("an unchanged plugin passes");

        for (name, verdicts) in [
            (
                "a lost pass",
                vec![("Alpha", failing("new")), ("Beta", failing("known"))],
            ),
            (
                "a resolved failure",
                vec![("Alpha", Verdict::Pass), ("Beta", Verdict::Pass)],
            ),
            (
                "a changed reason",
                vec![("Alpha", Verdict::Pass), ("Beta", failing("different"))],
            ),
            (
                "a card missing from the baseline",
                vec![
                    ("Alpha", Verdict::Pass),
                    ("Beta", failing("known")),
                    ("Gamma", Verdict::Pass),
                ],
            ),
            ("a card that disappeared", vec![("Alpha", Verdict::Pass)]),
        ] {
            let mut changed = report(&verdicts);
            changed.dir = dir.path().to_path_buf();
            assert!(
                ratchet(&changed, false).is_err(),
                "the ratchet accepted {name}"
            );
        }
    }

    #[test]
    fn blessing_writes_the_runs_verdicts() {
        let dir = tempfile::tempdir().unwrap();
        let mut blessed = report(&[("Alpha", Verdict::Pass), ("Beta", failing("known"))]);
        blessed.dir = dir.path().to_path_buf();
        ratchet(&blessed, true).expect("blessing writes a fresh baseline");
        ratchet(&blessed, false).expect("the blessed baseline then matches");
    }
}
