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
//! fails the run if any card did not prove.
//!
//! There is no gap concept and no ratchet: the emitter is total, so a card
//! that fails `Card.check` is never committed, and every card the reader
//! loads either proves or fails the gate outright. That is the substantive
//! difference from the Idris gate this replaces, whose partial emitter needed
//! a third verdict and whose baseline could record a gap.
//!
//! `Macros.lean` plays no part: the emitted term is post-expansion, so it is
//! raw constructors, and the `spelled` elaborator (which refuses raw
//! constructors by design) is the hand bench's law, not the gate's.

use std::collections::BTreeMap;
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

/// The generated library's root module and directory, relative to `lean/`.
const GENERATED_ROOT: &str = "Generated.lean";
const GENERATED_DIR: &str = "Generated";
/// Where `lake` puts the generated library's build output, relative to
/// `lean/`. A run clears both, because a stale `.olean` left by an earlier
/// successful run is exactly the false evidence [`attribute`] must not accept.
const BUILD_DIRS: &[&str] = &[".lake/build/lib/lean", ".lake/build/ir"];
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
    /// The `lake` program to run. Not a command-line option: the gate's own
    /// test points it at a stub that fails, so that "lake could not build the
    /// generated library" can be shown to stop the command rather than leave
    /// every card at its seeded verdict.
    #[arg(skip = String::from("lake"))]
    lake: String,
}

impl LeanCheckArgs {
    /// The gate's arguments, for callers that drive it directly (its own
    /// integration test).
    #[must_use]
    pub fn new(plugin_dirs: Vec<PathBuf>) -> Self {
        LeanCheckArgs {
            plugin_dirs,
            lake: "lake".to_owned(),
        }
    }

    /// The same arguments, run against a different `lake` program.
    #[must_use]
    pub fn with_lake(mut self, lake: impl Into<String>) -> Self {
        self.lake = lake.into();
        self
    }
}

/// What the kernel concluded about one card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// `Card.check` is provably empty.
    Pass,
    /// The emitted term did not build, or `decide` refuted the theorem. The
    /// reason is the head of the Lean diagnostic.
    Fail { reason: String },
}

/// One plugin's run: where it lives and what each of its cards proved.
#[derive(Debug, Clone)]
struct PluginReport {
    dir: PathBuf,
    /// The Lean module the plugin's cards were emitted into.
    module: String,
    verdicts: BTreeMap<String, Verdict>,
    /// Every Lean diagnostic attributed to a card, whole: the `error:`,
    /// `warning:` or `info:` header line and each continuation line of its
    /// message, in the order Lean printed them. The console gets all of it;
    /// a failing verdict's own `reason` is one line chosen from it (see
    /// [`reason_for`]).
    diagnostics: BTreeMap<String, Vec<String>>,
}

/// How seriously Lean meant a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    /// An `error:` or a `warning:` — the build treats both as failures
    /// (`--wfail`), so both refute the card they land on.
    Failing,
    /// An `info:` — the guarded `#eval` naming a card's refusals. It never
    /// decides a verdict; it supplies the reason for one.
    Info,
}

/// One parsed Lean diagnostic, message body included.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Diagnostic {
    severity: Severity,
    /// The path Lean reported, relative to `lean/`.
    path: String,
    line: usize,
    /// The message head — the diagnostic's own first line.
    head: String,
    /// The header line and every continuation line of the message, verbatim.
    lines: Vec<String>,
}

/// # Errors
/// If a plugin fails to load or emit, if `lake` cannot be run, or if any
/// card failed to prove `Card.check = []`.
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
    let (output, lake_succeeded) = build(&lean_dir, &args.lake)?;
    let mut reports = attribute(&modules, &output, lake_succeeded)?;
    reports.sort_by(|a, b| a.dir.cmp(&b.dir));

    let mut failures = 0usize;
    for report in &reports {
        print_report(report);
        failures += failing_count(report);
    }
    println!(
        "\nlean-check: {} plugin(s), {} card(s), {:.1}s",
        reports.len(),
        reports.iter().map(|r| r.verdicts.len()).sum::<usize>(),
        started.elapsed().as_secs_f64()
    );
    anyhow::ensure!(
        failures == 0,
        "{failures} card(s) did not prove `Card.check = []`"
    );
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
    /// The `.olean` `lake` writes when the module elaborates cleanly; its
    /// existence is the positive evidence a plugin's cards need to report
    /// `Pass` when Lean said nothing about them.
    artifact: PathBuf,
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
            artifact: lean_dir
                .join(BUILD_DIRS[0])
                .join(GENERATED_DIR)
                .join(format!("{component}.olean")),
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

/// Builds the generated library, returning its output and whether `lake`
/// succeeded.
///
/// A nonzero exit is expected whenever a card fails, so the status alone is
/// not the verdict — but it is load-bearing evidence, and dropping it is how a
/// build that failed for a reason naming no card would leave every card at its
/// seeded `Pass` ([`attribute`]).
fn build(lean_dir: &Path, lake: &str) -> anyhow::Result<(String, bool)> {
    let output = Command::new(lake)
        .args(["build", "--wfail", GENERATED_TARGET])
        .current_dir(lean_dir)
        .output()
        .with_context(|| format!("running {lake} (is it on PATH?)"))?;
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok((text, output.status.success()))
}

/// Parses `lake`'s output into diagnostics that name a `.lean` file.
///
/// Lean reports `<severity>: <path>:<line>:<col>: <message>`, the message
/// running on over the lines that follow. Those continuation lines carry the
/// content — the refusal list a guarded `#eval` printed, the proposition
/// `decide` refuted — so they are kept, up to the next diagnostic header or
/// the next line of `lake`'s own progress and summary chatter.
///
/// Lake's pathless lines (`error: build failed`) name no file and are not
/// diagnostics; the command reads lake's EXIT STATUS for that, never the
/// absence of parsed output.
fn diagnostics(output: &str) -> Vec<Diagnostic> {
    let mut out: Vec<Diagnostic> = Vec::new();
    let mut open: Option<usize> = None;
    for line in output.lines() {
        if let Some(diagnostic) = parse_header(line) {
            out.push(diagnostic);
            open = Some(out.len() - 1);
            continue;
        }
        if is_header(line) || is_lake_chatter(line) {
            open = None;
            continue;
        }
        if let Some(index) = open {
            out[index].lines.push(line.to_owned());
        }
    }
    out
}

/// The severity prefix a diagnostic header carries.
fn severity_of(line: &str) -> Option<(Severity, &str)> {
    if let Some(rest) = line.strip_prefix("error: ") {
        return Some((Severity::Failing, rest));
    }
    if let Some(rest) = line.strip_prefix("warning: ") {
        return Some((Severity::Failing, rest));
    }
    line.strip_prefix("info: ")
        .map(|rest| (Severity::Info, rest))
}

fn is_header(line: &str) -> bool {
    severity_of(line).is_some()
}

/// Lake's own progress and summary lines, which close whatever message was
/// running rather than continuing it.
fn is_lake_chatter(line: &str) -> bool {
    line.starts_with("trace: ")
        || line.starts_with("Some required targets")
        || line.starts_with("Build completed")
        || line.starts_with(['\u{2714}', '\u{2716}', '\u{26a0}', '\u{2139}'])
}

/// One header line as a diagnostic, if it names a `.lean` file and a position.
fn parse_header(line: &str) -> Option<Diagnostic> {
    let (severity, rest) = severity_of(line)?;
    let (path, rest) = rest.split_once(".lean:")?;
    let mut parts = rest.splitn(3, ':');
    let number = parts.next()?.parse::<usize>().ok()?;
    parts.next()?.parse::<usize>().ok()?;
    let head = parts.next().unwrap_or_default().trim().to_owned();
    Some(Diagnostic {
        severity,
        path: format!("{path}.lean"),
        line: number,
        head,
        lines: vec![line.to_owned()],
    })
}

/// The reason a refuted card's verdict records, chosen from everything Lean
/// said about that card.
///
/// A refuted card gets three diagnostics: the guarded `#eval`'s `info` line,
/// which is the refusal list itself; the `#guard_msgs` mismatch; and
/// `decide`'s own failure, whose text is the same words for every card
/// ("Tactic decide proved that the proposition … is false"). Recording the
/// last of those would give every refuted card in the repository one
/// identical reason, telling one broken law from another only by re-reading
/// the console output — so the `info` line wins whenever Lean printed one,
/// and the first failing head is the fallback for a card that did not even
/// elaborate.
fn reason_for(diagnostics: &[&Diagnostic]) -> String {
    diagnostics
        .iter()
        .find(|diagnostic| diagnostic.severity == Severity::Info)
        .or_else(|| {
            diagnostics
                .iter()
                .find(|diagnostic| diagnostic.severity == Severity::Failing)
        })
        .map_or_else(|| "no diagnostic".to_owned(), |chosen| chosen.head.clone())
}

/// Places each diagnostic on the card whose block it landed in, and refuses to
/// report a verdict the build does not actually evidence.
///
/// A card is `Pass` only on POSITIVE evidence that Lean elaborated it: either
/// its module produced a build artifact, or Lean reported a diagnostic
/// somewhere in that module, which is proof it elaborated the module and
/// reported per-declaration failures. Seeding `Pass` and downgrading on parsed
/// output alone would report every card sound whenever lake failed for a
/// reason that names no card — an unknown target, a broken `Semantics/`, an
/// error in the generated root — which is the one way a soundness gate must
/// never fail.
///
/// # Errors
/// If lake failed with nothing attributable to a card; if a module was neither
/// built nor diagnosed; or if a diagnostic names a `.lean` file that is not one
/// of the emitted modules, or sits above its module's first card. Each of
/// those is a defect in the gate or the workbench, never a verdict about a
/// card.
fn attribute(
    modules: &[EmittedModule],
    output: &str,
    lake_succeeded: bool,
) -> anyhow::Result<Vec<PluginReport>> {
    let parsed = diagnostics(output);
    let tail = || output.lines().rev().take(20).collect::<Vec<_>>().join("\n");

    // Every diagnostic has to land on a card. One that does not is the gate or
    // the workbench breaking, and swallowing it is exactly how a seeded `Pass`
    // survives a failed build.
    let mut owners: Vec<(usize, usize)> = Vec::with_capacity(parsed.len());
    for diagnostic in &parsed {
        let module = modules
            .iter()
            .position(|module| diagnostic.path.ends_with(&module.path))
            .with_context(|| {
                format!(
                    "{}:{}: {} — the diagnostic is in a file this run did not emit as a card \
                     module (the generated root, or the workbench itself), so no card is at \
                     fault; this is a gate defect",
                    diagnostic.path, diagnostic.line, diagnostic.head
                )
            })?;
        let card = modules[module]
            .cards
            .iter()
            .rposition(|card| card.start_line <= diagnostic.line)
            .with_context(|| {
                format!(
                    "{}:{}: {} — the diagnostic is above the module's first card, so the \
                     generated header itself did not elaborate; this is a gate defect, not a \
                     card verdict",
                    diagnostic.path, diagnostic.line, diagnostic.head
                )
            })?;
        owners.push((module, card));
    }

    // Every parsed diagnostic is attributed by now, so this asks exactly the
    // reviewer's question: did lake's failure reach a card at all?
    anyhow::ensure!(
        lake_succeeded
            || parsed
                .iter()
                .any(|diagnostic| diagnostic.severity == Severity::Failing),
        "gate defect: `lake` exited nonzero but reported no error against any emitted card, so \
         no card has a verdict. lake said:\n{}",
        tail()
    );

    let mut reports = Vec::new();
    for (index, module) in modules.iter().enumerate() {
        let mut per_card: BTreeMap<usize, Vec<&Diagnostic>> = BTreeMap::new();
        for (position, &(owning_module, card)) in owners.iter().enumerate() {
            if owning_module == index {
                per_card.entry(card).or_default().push(&parsed[position]);
            }
        }
        let built = module.artifact.is_file();
        anyhow::ensure!(
            built || !per_card.is_empty(),
            "gate defect: {} produced neither a build artifact ({}) nor a diagnostic, so nothing \
             evidences that Lean elaborated its cards. lake said:\n{}",
            module.module,
            module.artifact.display(),
            tail()
        );

        let mut verdicts: BTreeMap<String, Verdict> = module
            .cards
            .iter()
            .map(|card| (card.name.clone(), Verdict::Pass))
            .collect();
        let mut collected: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (card, diagnostics) in &per_card {
            let name = module.cards[*card].name.clone();
            collected.insert(
                name.clone(),
                diagnostics
                    .iter()
                    .flat_map(|diagnostic| diagnostic.lines.clone())
                    .collect(),
            );
            if diagnostics
                .iter()
                .any(|diagnostic| diagnostic.severity == Severity::Failing)
            {
                verdicts.insert(
                    name,
                    Verdict::Fail {
                        reason: reason_for(diagnostics),
                    },
                );
            }
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
// Reporting
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

/// How many of a plugin's cards did not prove `Card.check = []`. The gate
/// fails iff this is nonzero for any plugin.
fn failing_count(report: &PluginReport) -> usize {
    report
        .verdicts
        .values()
        .filter(|verdict| **verdict != Verdict::Pass)
        .count()
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
/// declarations (`plugins_v2/builtin`) has nothing for the gate to prove.
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

    /// Everything `lake` prints for one refuted card, verbatim from a real
    /// run: the guarded `#eval`'s refusal list, the `#guard_msgs` mismatch and
    /// its indented body, `decide`'s failure and its two continuation lines,
    /// and lake's own pathless chatter around them.
    const REFUTED: &str = "\u{2716} [23/25] Building Generated.Testing (519ms)\n\
         trace: .> LEAN_PATH=… lean Generated/Testing.lean\n\
         info: Generated/Testing.lean:17:0: [Semantics.Refusal.cardCost]\n\
         error: Generated/Testing.lean:16:0: \u{274c}\u{fe0f} Docstring on `#guard_msgs` does not \
         match generated message:\n\n- info: []\n+ info: [Semantics.Refusal.cardCost]\n\
         error: Generated/Testing.lean:18:81: Tactic `decide` proved that the proposition\n  \
         card_lawless_land.check = []\nis false\n\
         error: Lean exited with code 1\n\
         Some required targets logged failures:\n- Generated.Testing\nerror: build failed\n";

    #[test]
    fn a_lean_diagnostic_keeps_its_position_severity_and_whole_message() {
        let parsed = diagnostics(REFUTED);
        assert_eq!(parsed.len(), 3, "{parsed:?}");

        assert_eq!(parsed[0].severity, Severity::Info);
        assert_eq!(parsed[0].path, "Generated/Testing.lean");
        assert_eq!(parsed[0].line, 17);
        assert_eq!(parsed[0].head, "[Semantics.Refusal.cardCost]");

        // The `#guard_msgs` body — a blank line and the diff — is message, not
        // chatter, and the report keeps it.
        assert_eq!(parsed[1].severity, Severity::Failing);
        assert_eq!(
            parsed[1].lines[1..],
            ["", "- info: []", "+ info: [Semantics.Refusal.cardCost]"]
        );

        // `decide`'s proposition and verdict lines survive the same way.
        assert_eq!(parsed[2].line, 18);
        assert_eq!(
            parsed[2].lines[1..],
            ["  card_lawless_land.check = []", "is false"]
        );

        // `error: Lean exited with code 1` and `error: build failed` name no
        // file, so they are lake's status, not diagnostics.
        assert!(
            parsed
                .iter()
                .all(|d| Path::new(&d.path).extension() == Some("lean".as_ref())),
            "{parsed:?}"
        );
    }

    #[test]
    fn a_warning_is_a_diagnostic_too_because_the_build_treats_it_as_one() {
        let parsed = diagnostics("warning: Generated/Testing.lean:9:0: unused variable\n");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].severity, Severity::Failing);
        assert_eq!(parsed[0].line, 9);
        assert_eq!(parsed[0].head, "unused variable");
    }

    /// A module with an `artifact` that exists, so its cards may be reported
    /// `Pass` on the strength of it.
    fn module(built: &Path, cards: &[(&str, usize)]) -> EmittedModule {
        EmittedModule {
            dir: PathBuf::from("plugins_v2/testing"),
            module: "Generated.Testing".to_owned(),
            path: "Generated/Testing.lean".to_owned(),
            artifact: built.to_path_buf(),
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

    /// A stand-in for the `.olean` lake writes when a module elaborates.
    fn built_artifact(dir: &tempfile::TempDir) -> PathBuf {
        let path = dir.path().join("Testing.olean");
        fs::write(&path, b"olean").unwrap();
        path
    }

    #[test]
    fn a_diagnostic_lands_on_the_card_whose_block_it_sits_in() {
        let dir = tempfile::tempdir().unwrap();
        let modules = vec![module(&built_artifact(&dir), &[("Alpha", 6), ("Beta", 14)])];
        let reports = attribute(&modules, REFUTED, false).unwrap();
        assert_eq!(reports[0].verdicts["Alpha"], Verdict::Pass);
        assert_eq!(
            reports[0].verdicts["Beta"],
            Verdict::Fail {
                // The refusal list, not `decide`'s one-size-fits-all preamble.
                reason: "[Semantics.Refusal.cardCost]".to_owned()
            }
        );
        assert_eq!(
            reports[0].diagnostics["Beta"].len(),
            8,
            "the report keeps every line of all three diagnostics: {:?}",
            reports[0].diagnostics["Beta"]
        );
    }

    /// A card that does not even elaborate has no refusal list to record, so
    /// the failing head is the reason.
    #[test]
    fn a_card_that_fails_to_elaborate_records_the_failing_head() {
        let dir = tempfile::tempdir().unwrap();
        let modules = vec![module(&built_artifact(&dir), &[("Alpha", 6)])];
        let reports = attribute(
            &modules,
            "error: Generated/Testing.lean:8:4: unknown identifier 'Semantics.Card.nope'\n",
            false,
        )
        .unwrap();
        assert_eq!(
            reports[0].verdicts["Alpha"],
            Verdict::Fail {
                reason: "unknown identifier 'Semantics.Card.nope'".to_owned()
            }
        );
    }

    /// A header that does not elaborate is the gate's own defect: reporting it
    /// as some card's failure would bless a broken emitter.
    #[test]
    fn a_diagnostic_above_the_first_card_is_a_gate_defect() {
        let dir = tempfile::tempdir().unwrap();
        let modules = vec![module(&built_artifact(&dir), &[("Alpha", 6)])];
        let error = attribute(
            &modules,
            "error: Generated/Testing.lean:2:0: unknown import\n",
            false,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("gate defect"), "{error}");
    }

    /// The generated root, and the workbench itself, are files no card owns.
    /// Filtering them out silently is how a broken build reports every card
    /// sound.
    #[test]
    fn a_diagnostic_in_a_file_no_card_owns_is_a_gate_defect() {
        let dir = tempfile::tempdir().unwrap();
        let modules = vec![module(&built_artifact(&dir), &[("Alpha", 6)])];
        for output in [
            "error: Generated.lean:1:0: unknown module prefix 'Generated'\n",
            "error: Semantics/Check/Card.lean:20:2: unknown identifier\n",
        ] {
            let error = attribute(&modules, output, false).unwrap_err().to_string();
            assert!(error.contains("gate defect"), "{output} gave {error}");
        }
    }

    /// H1: lake failing for a reason that names no card must stop the command.
    /// Seeding `Pass` and downgrading only on parsed diagnostics reported every
    /// card sound on an unknown target, a broken `Semantics/`, or a lake that
    /// never ran the compiler at all.
    #[test]
    fn a_lake_failure_that_names_no_card_is_a_gate_defect() {
        let dir = tempfile::tempdir().unwrap();
        let modules = vec![module(&built_artifact(&dir), &[("Alpha", 6)])];
        let error = attribute(&modules, "error: unknown target\n", false)
            .unwrap_err()
            .to_string();
        assert!(error.contains("gate defect"), "{error}");
        assert!(error.contains("unknown target"), "{error}");

        // The same output with lake succeeding is not a defect: nothing was
        // reported because nothing was wrong.
        let reports = attribute(&modules, "", true).unwrap();
        assert_eq!(reports[0].verdicts["Alpha"], Verdict::Pass);
    }

    /// The other half of the same law: a module with no build artifact and no
    /// diagnostic has no evidence behind it either way, even if lake claims
    /// success.
    #[test]
    fn a_module_that_was_neither_built_nor_diagnosed_is_a_gate_defect() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("never-written.olean");
        let modules = vec![module(&missing, &[("Alpha", 6)])];
        let error = attribute(&modules, "", true).unwrap_err().to_string();
        assert!(error.contains("gate defect"), "{error}");

        // A module Lean did diagnose was elaborated, so its quiet cards pass
        // even though the failing one cost the module its artifact.
        let modules = vec![module(&missing, &[("Alpha", 6), ("Beta", 14)])];
        let reports = attribute(&modules, REFUTED, false).unwrap();
        assert_eq!(reports[0].verdicts["Alpha"], Verdict::Pass);
        assert!(matches!(reports[0].verdicts["Beta"], Verdict::Fail { .. }));
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

    /// The new contract has no ratchet: a report is clean iff every card
    /// proved, and any failing card — no matter how many, or whether it
    /// failed before — fails the gate.
    #[test]
    fn a_report_fails_iff_any_card_did_not_prove() {
        let all_passing = report(&[("Alpha", Verdict::Pass), ("Beta", Verdict::Pass)]);
        assert_eq!(
            failing_count(&all_passing),
            0,
            "an all-passing report is clean"
        );

        let one_failing = report(&[("Alpha", Verdict::Pass), ("Beta", failing("known"))]);
        assert_eq!(
            failing_count(&one_failing),
            1,
            "a report with a failing card is not clean, regardless of any prior run"
        );

        let all_failing = report(&[("Alpha", failing("a")), ("Beta", failing("b"))]);
        assert_eq!(failing_count(&all_failing), 2);
    }
}
