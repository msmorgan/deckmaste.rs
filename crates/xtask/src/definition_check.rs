//! `cargo xtask lean definitions` — the registry definition gate: re-emit
//! every Registry Definition a `plugins_v2` plugin declares (counters,
//! subtypes, designations) as a Lean term and ask the kernel to prove its
//! refusal list empty.
//!
//! `lean check` proves cards; a registry declaration's `Definition` node
//! appears in no card, so without this gate `Definition.check`
//! (`lean/Semantics/Check/Rules.lean`) runs only over hand-written pins. This
//! command is `lean check`'s twin over the definitions: it reads every
//! declaration of the three registry kinds (`facts`' readers, so the gate and
//! `Facts.lean` read the same nodes), writes one untracked Lean module per
//! family under `lean/GeneratedDefinitions/`, builds them with `lake`,
//! attributes each diagnostic back to the definition whose block it landed
//! in, and fails the run if any definition did not prove. It shares
//! `lean check`'s build and attribution, so a `lake` failure that names no
//! definition is a gate defect, never a pass.
//!
//! There is no skip list and no family without a rule: a declaration of a
//! registry kind whose body does not read as its family's `Definition`
//! constructor fails the read, `Definition.check` is an exhaustive match (a
//! new constructor without an arm does not build), and [`Family::of`] is an
//! exhaustive match on the Rust side (a new constructor does not compile
//! until it is given a family here).

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context;
use clap::Args;
use deckmaste_semantics_v2::lean_emit;
use deckmaste_semantics_v2::rules::Definition;

use crate::lean_check::BUILD_DIRS;
use crate::lean_check::EmittedModule;
use crate::lean_check::PluginReport;
use crate::lean_check::Verdict;
use crate::lean_check::attribute;
use crate::lean_check::build;
use crate::lean_check::failing_count;
use crate::lean_check::lean_root;
use crate::lean_check::workspace_root;

/// The generated library's root module and directory, relative to `lean/`,
/// beside `lean check`'s `Generated` so the two gates never clear each
/// other's tree. Untracked (`.gitignore`), and outside `lakefile.toml`'s
/// `defaultTargets` for the same reason `Generated` is.
const GENERATED_ROOT: &str = "GeneratedDefinitions.lean";
const GENERATED_DIR: &str = "GeneratedDefinitions";
const GENERATED_TARGET: &str = "GeneratedDefinitions";

#[derive(Debug, Args)]
pub struct DefinitionCheckArgs {
    /// The plugin whose registry declarations to check. Defaults to
    /// `plugins_v2/builtin`, the sole v2 registry.
    plugin_dir: Option<PathBuf>,
    /// The `lake` program to run; the gate's own test points it at a stub
    /// that fails (see `lean_check`'s twin field).
    #[arg(skip = String::from("lake"))]
    lake: String,
}

impl DefinitionCheckArgs {
    /// The gate's arguments, for callers that drive it directly.
    #[must_use]
    pub fn new(plugin_dir: Option<PathBuf>) -> Self {
        DefinitionCheckArgs {
            plugin_dir,
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

/// The three families of Registry Definition, one generated module each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Counter,
    Subtype,
    Designation,
}

impl Family {
    /// The family a definition node belongs to. Exhaustive on purpose: a new
    /// `Definition` constructor must be given a family before the gate
    /// compiles.
    fn of(definition: &Definition) -> Family {
        match definition {
            Definition::Counter { .. } => Family::Counter,
            Definition::Subtype { .. } => Family::Subtype,
            Definition::Designation { .. } => Family::Designation,
        }
    }

    /// The module component and the `macros/` directory the family is
    /// declared in.
    fn module_and_dir(self) -> (&'static str, &'static str) {
        match self {
            Family::Counter => ("Counters", "counter_kinds"),
            Family::Subtype => ("Subtypes", "subtypes"),
            Family::Designation => ("Designations", "designations"),
        }
    }

    fn plural(self) -> &'static str {
        match self {
            Family::Counter => "counters",
            Family::Subtype => "subtypes",
            Family::Designation => "designations",
        }
    }
}

/// # Errors
/// If the declarations fail to read or emit, if `lake` cannot be run, or if
/// any definition failed to prove `Definition.check = []`.
pub fn run(args: &DefinitionCheckArgs) -> anyhow::Result<()> {
    let started = Instant::now();
    let lean_dir = lean_root()?;
    let plugin_dir = match &args.plugin_dir {
        Some(dir) => dir.clone(),
        None => workspace_root()?.join("plugins_v2").join("builtin"),
    };

    let read = crate::facts::registry_definitions(&plugin_dir)?;
    let families = [
        (Family::Counter, read.counters),
        (Family::Subtype, read.subtypes),
        (Family::Designation, read.designations),
    ];
    let total: usize = families.iter().map(|(_, items)| items.len()).sum();
    anyhow::ensure!(
        total > 0,
        "{} declares no registry definition to check",
        plugin_dir.display()
    );

    let modules = emit(&lean_dir, &plugin_dir, &families)?;
    let (output, lake_succeeded) = build(&lean_dir, &args.lake, GENERATED_TARGET)?;
    let reports = attribute(&modules, &output, lake_succeeded)?;

    let mut failures = 0usize;
    for ((family, _), report) in families.iter().zip(&reports) {
        print_report(*family, report);
        failures += failing_count(report);
    }
    println!(
        "\nlean definitions: {total} definition(s) ({}), {:.1}s",
        families
            .iter()
            .map(|(family, items)| format!("{} {}", items.len(), family.plural()))
            .collect::<Vec<_>>()
            .join(", "),
        started.elapsed().as_secs_f64()
    );
    anyhow::ensure!(
        failures == 0,
        "{failures} definition(s) did not prove `Definition.check = []`"
    );
    Ok(())
}

/// Writes one Lean module per family plus the root that imports them, having
/// first cleared whatever a previous run left — the generated sources and
/// their build outputs, so a stale `.olean` can never stand as the evidence
/// [`attribute`] reads.
fn emit(
    lean_dir: &Path,
    plugin_dir: &Path,
    families: &[(Family, Vec<(String, Definition)>)],
) -> anyhow::Result<Vec<EmittedModule>> {
    let generated = lean_dir.join(GENERATED_DIR);
    if generated.exists() {
        fs::remove_dir_all(&generated)
            .with_context(|| format!("clearing {}", generated.display()))?;
    }
    for build_dir in BUILD_DIRS {
        let stale = lean_dir.join(build_dir).join(GENERATED_DIR);
        if stale.exists() {
            fs::remove_dir_all(&stale).with_context(|| format!("clearing {}", stale.display()))?;
        }
    }
    fs::create_dir_all(&generated).with_context(|| format!("creating {}", generated.display()))?;

    let mut modules = Vec::new();
    for (family, items) in families {
        for (name, definition) in items {
            anyhow::ensure!(
                Family::of(definition) == *family,
                "{name}: declared as one of the {} but defines {:?}",
                family.plural(),
                Family::of(definition)
            );
        }
        let (component, macros_dir) = family.module_and_dir();
        let module = format!("{GENERATED_DIR}.{component}");
        let borrowed: Vec<(String, &Definition)> = items
            .iter()
            .map(|(name, definition)| (name.clone(), definition))
            .collect();
        let rendered = lean_emit::render_definitions_module(&module, &borrowed)
            .with_context(|| format!("emitting the {} as Lean", family.plural()))?;
        let file = generated.join(format!("{component}.lean"));
        fs::write(&file, &rendered.source)
            .with_context(|| format!("writing {}", file.display()))?;
        modules.push(EmittedModule {
            dir: plugin_dir.join("macros").join(macros_dir),
            module,
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

/// One family's count, then every refused definition by name with its
/// refusal list (the guarded `#eval`'s line), then Lean's whole message for
/// each.
fn print_report(family: Family, report: &PluginReport) {
    let passes = report
        .verdicts
        .values()
        .filter(|verdict| **verdict == Verdict::Pass)
        .count();
    println!(
        "{}: {passes}/{} {} prove `Definition.check = []` ({}, {})",
        family.plural(),
        report.verdicts.len(),
        family.plural(),
        report.module,
        report.dir.display()
    );
    for (name, verdict) in &report.verdicts {
        if let Verdict::Fail { reason } = verdict {
            println!("  REFUSED {name}: {reason}");
        }
    }
    for (name, lines) in &report.diagnostics {
        println!("  {name}:");
        for line in lines {
            println!("    {line}");
        }
    }
}
