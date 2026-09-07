//! Changed-path test gates.
//!
//! `cargo xtask gate --changed` derives its package list from the changed
//! paths, rather than relying on an author-maintained list. The resolver is
//! deliberately independent of process execution so its boundary cases are
//! tested from a small `cargo metadata` snapshot.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context as _;
use clap::Args;
use serde::Deserialize;

/// Derive the reverse-dependency test gate for the current changed paths.
#[derive(Debug, Args)]
pub struct GateArgs {
    /// Derive the test gate from paths changed since this jj revset.
    #[arg(long)]
    changed: bool,
    /// Revision or revset supplied to `jj diff --from`.
    #[arg(long, default_value = "default@")]
    from: String,
    /// Execute the printed command or commands.
    #[arg(long)]
    run: bool,
    /// Include the strict clippy command for the same package closure.
    #[arg(long)]
    clippy: bool,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    workspace_root: PathBuf,
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    manifest_path: PathBuf,
    targets: Vec<Target>,
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
struct Target {
    src_path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    path: Option<PathBuf>,
}

/// # Errors
/// If `jj diff`, `cargo metadata`, source scanning, or an elected gate command
/// fails.
pub fn run(args: &GateArgs) -> anyhow::Result<()> {
    anyhow::ensure!(args.changed, "`gate` requires `--changed`");

    let workspace_root = workspace_root();
    let paths = changed_paths(&workspace_root, &args.from)?;
    let metadata = metadata(&workspace_root)?;
    let readers = if paths.iter().any(|path| is_builtin_v2_path(path)) {
        builtin_v2_readers(&metadata)?
    } else {
        BTreeSet::new()
    };
    let packages = closure_for_paths(&metadata, &paths, &readers);

    if packages.is_empty() {
        println!("No workspace crates are affected by the changed paths.");
        return Ok(());
    }

    let mut commands = vec![test_command(&packages)];
    if args.clippy {
        commands.push(clippy_command(&packages));
    }
    for command in &commands {
        println!("{}", rendered(command));
    }

    if args.run {
        for command in &commands {
            run_cargo(command)?;
        }
    }

    Ok(())
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn changed_paths(workspace_root: &Path, from: &str) -> anyhow::Result<Vec<PathBuf>> {
    let output = Command::new("jj")
        .args(["--no-pager", "diff", "--from", from, "--summary"])
        .current_dir(workspace_root)
        .output()
        .context("running `jj diff --from … --summary`")?;
    anyhow::ensure!(
        output.status.success(),
        "`jj diff --from {from} --summary` failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    let summary = String::from_utf8(output.stdout).context("reading jj diff summary as UTF-8")?;
    Ok(paths_from_summary(&summary))
}

fn metadata(workspace_root: &Path) -> anyhow::Result<Metadata> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .current_dir(workspace_root)
        .output()
        .context("running `cargo metadata --no-deps --format-version 1`")?;
    anyhow::ensure!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    serde_json::from_slice(&output.stdout).context("parsing cargo metadata JSON")
}

fn builtin_v2_readers(metadata: &Metadata) -> anyhow::Result<BTreeSet<String>> {
    let mut readers = BTreeSet::new();
    for package in &metadata.packages {
        let crate_root = package_root(package);
        if !crate_root.starts_with(metadata.workspace_root.join("crates")) {
            continue;
        }
        for source_root in [crate_root.join("src"), crate_root.join("tests")] {
            if source_root.is_dir() && source_root_mentions(&source_root, BUILTIN_V2_NEEDLES)? {
                readers.insert(package.name.clone());
                break;
            }
        }
    }
    Ok(readers)
}

fn source_root_mentions(root: &Path, needles: &[&str]) -> anyhow::Result<bool> {
    for entry in fs::read_dir(root).with_context(|| format!("reading {}", root.display()))? {
        let entry = entry.with_context(|| format!("reading {}", root.display()))?;
        let path = entry.path();
        if path.is_dir() {
            if source_root_mentions(&path, needles)? {
                return Ok(true);
            }
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let source =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            if needles.iter().any(|needle| source.contains(needle)) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn paths_from_summary(summary: &str) -> Vec<PathBuf> {
    summary
        .lines()
        .flat_map(|line| {
            let Some((status, rendered)) = line.split_once(char::is_whitespace) else {
                return Vec::new();
            };
            let rendered = rendered.trim();
            if status == "R" || status == "C" {
                moved_paths(rendered)
            } else {
                vec![PathBuf::from(rendered)]
            }
        })
        .collect()
}

/// Both endpoints of a rename or copy. jj factors the shared prefix and suffix
/// out of the two paths and braces what differs, so a ticket move renders as
/// `docs/tickets/{wip => done}/slug.md` and an unrelated pair as
/// `{crates/xtask/src/gate.rs => gate.rs}`.
fn moved_paths(rendered: &str) -> Vec<PathBuf> {
    let Some((prefix, rest)) = rendered.split_once('{') else {
        return rendered
            .split(" => ")
            .map(|path| PathBuf::from(path.trim()))
            .collect();
    };
    let Some((endpoints, suffix)) = rest.split_once('}') else {
        return vec![PathBuf::from(rendered)];
    };
    endpoints
        .split(" => ")
        .map(|endpoint| PathBuf::from(format!("{prefix}{}{suffix}", endpoint.trim())))
        .collect()
}

fn closure_for_paths(
    metadata: &Metadata,
    paths: &[PathBuf],
    builtin_v2_readers: &BTreeSet<String>,
) -> Vec<String> {
    let mut roots = BTreeSet::new();
    for path in paths {
        if is_plugins_v2_path(path) {
            // Every `plugins_v2` tree is data `deckmaste_semantics_v2`'s reader
            // consumes — `builtin` through the declaration tests, `testing`
            // through the reader tests, `canon` when it lands — and no crate
            // OWNS the directory, so without this the closure would be empty
            // for a fixture change.
            roots.insert(PLUGINS_V2_READER.to_owned());
            // The builtin declarations are the file `deckmaste_english_v2` and
            // `deckmaste_construction_core` read too, under their own metadata.
            if is_builtin_v2_path(path) {
                roots.extend(builtin_v2_readers.iter().cloned());
            }
        } else if let Some(package) = owner_for_path(metadata, path) {
            roots.insert(package.name.clone());
        }
    }
    reverse_dependency_closure(metadata, &roots)
}

/// The v2 plugin format: every tree under `plugins_v2/`.
fn is_plugins_v2_path(path: &Path) -> bool {
    path.starts_with("plugins_v2")
}

/// The crate whose reader consumes any `plugins_v2` tree.
const PLUGINS_V2_READER: &str = "deckmaste_semantics_v2";

/// The builtin declaration tree, at its `plugins_v2/builtin` home.
fn is_builtin_v2_path(path: &Path) -> bool {
    path.starts_with("plugins_v2/builtin")
}

/// What a crate's sources say when they read the builtin declarations.
const BUILTIN_V2_NEEDLES: &[&str] = &["plugins_v2/builtin"];

fn owner_for_path<'a>(metadata: &'a Metadata, path: &Path) -> Option<&'a Package> {
    let path = metadata.workspace_root.join(path);
    metadata
        .packages
        .iter()
        .filter_map(|package| {
            owned_prefixes(metadata, package)
                .iter()
                .filter(|prefix| path.starts_with(prefix))
                .map(|prefix| prefix.components().count())
                .max()
                .map(|depth| (depth, package))
        })
        .max_by_key(|(depth, _)| *depth)
        .map(|(_, package)| package)
}

/// The paths a package owns. A crate owns its whole directory, but the
/// workspace-root package's directory contains every other file in the
/// repository, so that one owns only its manifest and the top-level
/// directories its targets are built from.
fn owned_prefixes(metadata: &Metadata, package: &Package) -> Vec<PathBuf> {
    let root = package_root(package);
    if root != metadata.workspace_root {
        return vec![root.to_path_buf()];
    }
    let mut prefixes = vec![package.manifest_path.clone()];
    prefixes.extend(package.targets.iter().filter_map(|target| {
        let relative = target.src_path.strip_prefix(root).ok()?;
        Some(root.join(relative.components().next()?))
    }));
    prefixes
}

fn package_root(package: &Package) -> &Path {
    package
        .manifest_path
        .parent()
        .expect("cargo metadata manifest paths always have parents")
}

fn reverse_dependency_closure(metadata: &Metadata, roots: &BTreeSet<String>) -> Vec<String> {
    let roots_by_path: BTreeMap<_, _> = metadata
        .packages
        .iter()
        .map(|package| (package_root(package).to_path_buf(), package.name.as_str()))
        .collect();
    let mut reverse: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for package in &metadata.packages {
        for dependency in &package.dependencies {
            if let Some(name) = dependency
                .path
                .as_ref()
                .and_then(|path| roots_by_path.get(path))
            {
                reverse.entry(name).or_default().push(&package.name);
            }
        }
    }
    for dependents in reverse.values_mut() {
        dependents.sort_unstable();
        dependents.dedup();
    }

    let mut queue = roots.iter().map(String::as_str).collect::<VecDeque<_>>();
    let mut selected = BTreeSet::new();
    while let Some(package) = queue.pop_front() {
        if !selected.insert(package) {
            continue;
        }
        if let Some(dependents) = reverse.get(package) {
            queue.extend(dependents);
        }
    }

    let mut remaining = selected;
    let mut ordered = Vec::new();
    while !remaining.is_empty() {
        let next = remaining
            .iter()
            .copied()
            .find(|package| {
                let mut dependencies = metadata
                    .packages
                    .iter()
                    .find(|candidate| candidate.name == **package)
                    .expect("reverse-closure package comes from metadata")
                    .dependencies
                    .iter()
                    .filter_map(|dependency| {
                        dependency
                            .path
                            .as_ref()
                            .and_then(|path| roots_by_path.get(path))
                    });
                dependencies.all(|dependency| !remaining.contains(dependency))
            })
            // Cargo package dependencies cannot cycle. Keep the output useful
            // if malformed metadata ever violates that invariant.
            .or_else(|| remaining.iter().next().copied())
            .expect("the nonempty remaining set has a first item");
        remaining.remove(next);
        ordered.push(next.to_owned());
    }
    ordered
}

fn test_command(packages: &[String]) -> Vec<String> {
    cargo_arguments("test", packages)
}

fn clippy_command(packages: &[String]) -> Vec<String> {
    let mut arguments = cargo_arguments("clippy", packages);
    arguments.extend([
        "--all-targets".to_owned(),
        "--".to_owned(),
        "-D".to_owned(),
        "warnings".to_owned(),
    ]);
    arguments
}

fn cargo_arguments(subcommand: &str, packages: &[String]) -> Vec<String> {
    let mut arguments = vec![subcommand.to_owned()];
    for package in packages {
        arguments.extend(["-p".to_owned(), package.clone()]);
    }
    arguments
}

fn rendered(arguments: &[String]) -> String {
    format!("cargo {}", arguments.join(" "))
}

fn run_cargo(arguments: &[String]) -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .args(arguments)
        .status()
        .context("running the selected cargo gate")?;
    anyhow::ensure!(status.success(), "selected cargo gate failed: {status}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Shaped like the real workspace: a root package whose directory contains
    /// every other package, plus the `deckmaste_english_v2` and
    /// `deckmaste_semantics_v2` dependency chains.
    const METADATA: &str = r#"
    {
      "workspace_root": "/workspace",
      "packages": [
        {"name":"deckmaste","manifest_path":"/workspace/Cargo.toml","targets":[{"src_path":"/workspace/src/main.rs"}],"dependencies":[]},
        {"name":"deckmaste_construction_core","manifest_path":"/workspace/crates/deckmaste_construction_core/Cargo.toml","targets":[{"src_path":"/workspace/crates/deckmaste_construction_core/src/lib.rs"}],"dependencies":[]},
        {"name":"deckmaste_construction","manifest_path":"/workspace/crates/deckmaste_construction/Cargo.toml","targets":[{"src_path":"/workspace/crates/deckmaste_construction/src/lib.rs"}],"dependencies":[{"path":"/workspace/crates/deckmaste_construction_core"}]},
        {"name":"deckmaste_english_v2","manifest_path":"/workspace/crates/deckmaste_english_v2/Cargo.toml","targets":[{"src_path":"/workspace/crates/deckmaste_english_v2/src/lib.rs"}],"dependencies":[{"path":"/workspace/crates/deckmaste_construction_core"}]},
        {"name":"deckmaste_semantics_v2","manifest_path":"/workspace/crates/deckmaste_semantics_v2/Cargo.toml","targets":[{"src_path":"/workspace/crates/deckmaste_semantics_v2/src/lib.rs"}],"dependencies":[]},
        {"name":"xtask","manifest_path":"/workspace/crates/xtask/Cargo.toml","targets":[{"src_path":"/workspace/crates/xtask/src/lib.rs"}],"dependencies":[{"path":"/workspace/crates/deckmaste_english_v2"},{"path":"/workspace/crates/deckmaste_semantics_v2"}]}
      ]
    }
    "#;

    fn metadata() -> Metadata {
        serde_json::from_str(METADATA).expect("metadata snapshot parses")
    }

    fn closure(paths: &[&str]) -> Vec<String> {
        closure_for_paths(
            &metadata(),
            &paths.iter().map(PathBuf::from).collect::<Vec<_>>(),
            &BTreeSet::new(),
        )
    }

    #[test]
    fn construction_core_path_gets_its_complete_reverse_dependency_closure() {
        let packages = closure(&["crates/deckmaste_construction_core/src/emit/build.rs"]);
        assert_eq!(
            packages,
            [
                "deckmaste_construction_core",
                "deckmaste_construction",
                "deckmaste_english_v2",
                "xtask"
            ]
        );
        assert_eq!(
            rendered(&test_command(&packages)),
            "cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask"
        );
    }

    #[test]
    fn english_v2_path_gets_only_its_reverse_dependency_closure() {
        assert_eq!(
            closure(&["crates/deckmaste_english_v2/src/environment.rs"]),
            ["deckmaste_english_v2", "xtask"]
        );
    }

    #[test]
    fn core_verbs_declarations_belong_to_the_crate_that_holds_them() {
        assert_eq!(
            closure(&["crates/deckmaste_english_v2/src/core_verbs.ron"]),
            ["deckmaste_english_v2", "xtask"]
        );
    }

    /// A declaration file is read by every crate that reads the shared
    /// contract — the two english-v2 readers under their typed metadata, and
    /// `deckmaste_semantics_v2` under an opaque one.
    #[test]
    fn builtin_declaration_path_starts_from_each_reader() {
        let packages = closure_for_paths(
            &metadata(),
            &[PathBuf::from(
                "plugins_v2/builtin/macros/stubs/types/Foo.ron",
            )],
            &BTreeSet::from([
                "deckmaste_construction_core".to_owned(),
                "deckmaste_english_v2".to_owned(),
                "xtask".to_owned(),
            ]),
        );
        assert_eq!(
            packages,
            [
                "deckmaste_construction_core",
                "deckmaste_construction",
                "deckmaste_english_v2",
                "deckmaste_semantics_v2",
                "xtask"
            ]
        );
    }

    /// A fixture plugin is read by `deckmaste_semantics_v2` alone: it is not a
    /// declaration file, so the english-v2 readers stay out of the closure.
    #[test]
    fn a_plugins_v2_fixture_starts_from_the_semantics_reader() {
        let packages = closure_for_paths(
            &metadata(),
            &[PathBuf::from("plugins_v2/testing/cards/Lightning Bolt.ron")],
            &BTreeSet::from([
                "deckmaste_construction_core".to_owned(),
                "deckmaste_english_v2".to_owned(),
                "xtask".to_owned(),
            ]),
        );
        assert_eq!(packages, ["deckmaste_semantics_v2", "xtask"]);
    }

    #[test]
    fn docs_only_paths_produce_no_gate() {
        assert!(
            closure(&[
                "docs/tickets/wip/example.md",
                "docs/decisions/english-v2-rewrite.md",
                "CLAUDE.md",
                "Cargo.lock",
                ".github/workflows/ci.yml",
            ])
            .is_empty()
        );
    }

    #[test]
    fn the_root_package_owns_only_its_manifest_and_its_target_sources() {
        assert_eq!(closure(&["src/main.rs"]), ["deckmaste"]);
        assert_eq!(closure(&["Cargo.toml"]), ["deckmaste"]);
    }

    #[test]
    fn summary_parser_keeps_both_endpoints_of_a_move_in_scope() {
        assert_eq!(
            paths_from_summary(concat!(
                "M crates/xtask/src/gate.rs\n",
                "R docs/tickets/{wip => done}/example.md\n",
                "R {crates/deckmaste_english/src/tail.rs => crates/deckmaste_english_v2/src/tail.rs}\n",
                "C crates/deckmaste_english_v2/src/{tail.rs => head.rs}\n",
            )),
            [
                PathBuf::from("crates/xtask/src/gate.rs"),
                PathBuf::from("docs/tickets/wip/example.md"),
                PathBuf::from("docs/tickets/done/example.md"),
                PathBuf::from("crates/deckmaste_english/src/tail.rs"),
                PathBuf::from("crates/deckmaste_english_v2/src/tail.rs"),
                PathBuf::from("crates/deckmaste_english_v2/src/tail.rs"),
                PathBuf::from("crates/deckmaste_english_v2/src/head.rs"),
            ]
        );
    }

    #[test]
    fn a_move_between_crates_gates_both_crates() {
        assert_eq!(
            closure(
                &paths_from_summary(
                    "R crates/{deckmaste_construction => deckmaste_english_v2}/src/tail.rs\n"
                )
                .iter()
                .map(|path| path.to_str().expect("test paths are UTF-8"))
                .collect::<Vec<_>>()
            ),
            ["deckmaste_construction", "deckmaste_english_v2", "xtask"]
        );
    }

    #[test]
    fn the_clippy_command_gates_every_target_strictly() {
        assert_eq!(
            rendered(&clippy_command(&["xtask".to_owned()])),
            "cargo clippy -p xtask --all-targets -- -D warnings"
        );
    }
}
