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
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    path: Option<PathBuf>,
}

/// # Errors
/// If `jj diff`, `cargo metadata`, source scanning, or an elected gate command
/// fails.
pub fn run(args: &GateArgs) -> anyhow::Result<()> {
    anyhow::ensure!(args.changed, "`gate` currently requires `--changed`");

    let workspace_root = workspace_root();
    let paths = changed_paths(&workspace_root, &args.from)?;
    let metadata = metadata(&workspace_root)?;
    let readers = builtin_v2_readers(&metadata)?;
    let packages = closure_for_paths(&metadata, &paths, &readers);

    if packages.is_empty() {
        println!("No workspace crates are affected by the changed paths.");
        return Ok(());
    }

    let test = cargo_test_line(&packages);
    println!("{test}");
    if args.clippy {
        println!("{}", cargo_clippy_line(&packages));
    }

    if args.run {
        run_cargo(&["test"], &packages)?;
        if args.clippy {
            run_cargo(&["clippy"], &packages)?;
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
            if source_root.is_dir() && source_root_mentions(&source_root, "builtin_v2")? {
                readers.insert(package.name.clone());
                break;
            }
        }
    }
    Ok(readers)
}

fn source_root_mentions(root: &Path, needle: &str) -> anyhow::Result<bool> {
    for entry in fs::read_dir(root).with_context(|| format!("reading {}", root.display()))? {
        let entry = entry.with_context(|| format!("reading {}", root.display()))?;
        let path = entry.path();
        if path.is_dir() {
            if source_root_mentions(&path, needle)? {
                return Ok(true);
            }
        } else if path.extension().is_some_and(|extension| extension == "rs")
            && fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?
                .contains(needle)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn paths_from_summary(summary: &str) -> Vec<PathBuf> {
    summary
        .lines()
        .flat_map(|line| {
            let Some((status, paths)) = line.split_once(char::is_whitespace) else {
                return Vec::new();
            };
            let paths = paths.trim();
            if status == "R" {
                paths
                    .split(" => ")
                    .map(|path| PathBuf::from(path.trim().trim_matches(['{', '}', ' '])))
                    .collect()
            } else {
                vec![PathBuf::from(paths)]
            }
        })
        .collect()
}

fn closure_for_paths(
    metadata: &Metadata,
    paths: &[PathBuf],
    builtin_v2_readers: &BTreeSet<String>,
) -> Vec<String> {
    let mut roots = BTreeSet::new();
    for path in paths {
        if path.starts_with("plugins/builtin_v2") {
            roots.extend(builtin_v2_readers.iter().cloned());
        } else if let Some(package) = owner_for_path(metadata, path) {
            roots.insert(package.name.clone());
        }
    }
    reverse_dependency_closure(metadata, &roots)
}

fn owner_for_path<'a>(metadata: &'a Metadata, path: &Path) -> Option<&'a Package> {
    let path = metadata.workspace_root.join(path);
    if path.parent() == Some(metadata.workspace_root.as_path())
        && path != metadata.workspace_root.join("Cargo.toml")
    {
        return None;
    }
    metadata
        .packages
        .iter()
        .filter(|package| path.starts_with(package_root(package)))
        .max_by_key(|package| package_root(package).components().count())
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

fn cargo_test_line(packages: &[String]) -> String {
    cargo_line("test", packages, false)
}

fn cargo_clippy_line(packages: &[String]) -> String {
    cargo_line("clippy", packages, true)
}

fn cargo_line(subcommand: &str, packages: &[String], clippy: bool) -> String {
    let mut words = vec!["cargo".to_owned(), subcommand.to_owned()];
    for package in packages {
        words.extend(["-p".to_owned(), package.clone()]);
    }
    if clippy {
        words.extend([
            "--all-targets".to_owned(),
            "--".to_owned(),
            "-D".to_owned(),
            "warnings".to_owned(),
        ]);
    }
    words.join(" ")
}

fn run_cargo(subcommand: &[&str], packages: &[String]) -> anyhow::Result<()> {
    let mut command = Command::new("cargo");
    command.args(subcommand);
    for package in packages {
        command.args(["-p", package]);
    }
    if subcommand == ["clippy"] {
        command.args(["--all-targets", "--", "-D", "warnings"]);
    }
    let status = command.status().context("running selected cargo gate")?;
    anyhow::ensure!(status.success(), "selected cargo gate failed: {status}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const METADATA: &str = r#"
    {
      "workspace_root": "/workspace",
      "packages": [
        {"name":"deckmaste_construction_core","manifest_path":"/workspace/crates/deckmaste_construction_core/Cargo.toml","dependencies":[]},
        {"name":"deckmaste_construction","manifest_path":"/workspace/crates/deckmaste_construction/Cargo.toml","dependencies":[{"path":"/workspace/crates/deckmaste_construction_core"}]},
        {"name":"deckmaste_english_v2","manifest_path":"/workspace/crates/deckmaste_english_v2/Cargo.toml","dependencies":[{"path":"/workspace/crates/deckmaste_construction_core"}]},
        {"name":"xtask","manifest_path":"/workspace/crates/xtask/Cargo.toml","dependencies":[{"path":"/workspace/crates/deckmaste_english_v2"}]}
      ]
    }
    "#;

    fn metadata() -> Metadata {
        serde_json::from_str(METADATA).expect("metadata snapshot parses")
    }

    #[test]
    fn construction_core_path_gets_its_complete_reverse_dependency_closure() {
        let packages = closure_for_paths(
            &metadata(),
            &[PathBuf::from(
                "crates/deckmaste_construction_core/src/emit/build.rs",
            )],
            &BTreeSet::new(),
        );
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
            cargo_test_line(&packages),
            "cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask"
        );
    }

    #[test]
    fn english_v2_path_gets_only_its_reverse_dependency_closure() {
        let packages = closure_for_paths(
            &metadata(),
            &[PathBuf::from(
                "crates/deckmaste_english_v2/src/environment.rs",
            )],
            &BTreeSet::new(),
        );
        assert_eq!(packages, ["deckmaste_english_v2", "xtask"]);
    }

    #[test]
    fn builtin_declaration_path_starts_from_each_reader() {
        let packages = closure_for_paths(
            &metadata(),
            &[PathBuf::from(
                "plugins/builtin_v2/macros/stubs/types/Foo.ron",
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
                "xtask"
            ]
        );
    }

    #[test]
    fn docs_only_paths_produce_no_gate() {
        assert!(
            closure_for_paths(
                &metadata(),
                &[
                    PathBuf::from("docs/tickets/wip/example.md"),
                    PathBuf::from("CLAUDE.md"),
                    PathBuf::from("Cargo.lock"),
                ],
                &BTreeSet::new(),
            )
            .is_empty()
        );
    }

    #[test]
    fn summary_parser_keeps_each_renamed_path_in_scope() {
        assert_eq!(
            paths_from_summary("M crates/xtask/src/gate.rs\nR crates/a.rs => crates/b.rs\n"),
            [
                PathBuf::from("crates/xtask/src/gate.rs"),
                PathBuf::from("crates/a.rs"),
                PathBuf::from("crates/b.rs"),
            ]
        );
    }
}
