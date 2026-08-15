use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;

use crate::CatalogKind;
use crate::CatalogSet;

impl CatalogSet {
    /// Loads the complete canonical catalog inventory from plain UTF-8 line files.
    ///
    /// # Errors
    ///
    /// Returns an error when a required file is missing, is not UTF-8, or
    /// contains a blank entry.
    pub fn load(directory: impl AsRef<Path>) -> anyhow::Result<Self> {
        let directory = directory.as_ref();
        let mut entries = BTreeMap::new();

        for kind in CatalogKind::ALL {
            let filename = kind.filename();
            let path = directory.join(filename);
            entries.insert(kind, read_line_catalog(&path)?);
        }

        Self::from_entries(entries)
    }

    /// Writes every catalog deterministically, replacing only `output` on success.
    ///
    /// # Errors
    ///
    /// Returns an error when rendering, staging, or replacing the output directory
    /// fails. An existing output directory is restored if staging cannot replace it.
    pub fn write_to(&self, output: impl AsRef<Path>) -> anyhow::Result<()> {
        write_line_directory(
            output.as_ref(),
            CatalogKind::ALL
                .into_iter()
                .map(|kind| (kind.filename(), self.get(kind))),
        )
    }

    #[cfg(test)]
    fn write_to_with_placement(
        &self,
        output: impl AsRef<Path>,
        place: impl FnOnce(&Path, &Path) -> io::Result<()>,
    ) -> anyhow::Result<()> {
        write_line_directory_with_placement(
            output.as_ref(),
            CatalogKind::ALL
                .into_iter()
                .map(|kind| (kind.filename(), self.get(kind))),
            place,
        )
    }
}

pub(crate) fn read_line_catalog(path: &Path) -> anyhow::Result<BTreeSet<String>> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut catalog = BTreeSet::new();
    for (line_number, line) in contents.split_terminator('\n').enumerate() {
        if line.is_empty() {
            anyhow::bail!(
                "blank entry in {} at line {}",
                path.display(),
                line_number + 1
            );
        }
        catalog.insert(line.to_owned());
    }
    Ok(catalog)
}

pub(crate) fn write_line_directory<'a>(
    output: &Path,
    entries: impl IntoIterator<Item = (&'static str, &'a BTreeSet<String>)>,
) -> anyhow::Result<()> {
    write_line_directory_with_placement(output, entries, |source, destination| {
        fs::rename(source, destination)
    })
}

fn write_line_directory_with_placement<'a>(
    output: &Path,
    entries: impl IntoIterator<Item = (&'static str, &'a BTreeSet<String>)>,
    place: impl FnOnce(&Path, &Path) -> io::Result<()>,
) -> anyhow::Result<()> {
    let output = validate_output(output)?;
    let parent = output
        .parent()
        .context("catalog output must have a parent")?;

    let rendered = entries
        .into_iter()
        .map(|(filename, values)| render(filename, values))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let staging = tempfile::Builder::new()
        .prefix(".catalogs-")
        .tempdir_in(parent)
        .with_context(|| format!("creating catalog staging directory in {}", parent.display()))?;
    for (filename, contents) in rendered {
        fs::write(staging.path().join(filename), contents)
            .with_context(|| format!("staging {filename}"))?;
    }
    let staging_path = staging.keep();

    let backup = if output.exists() { Some(unique_backup_path(parent)?) } else { None };
    if let Some(backup) = &backup {
        fs::rename(&output, backup).with_context(|| {
            format!(
                "moving existing catalog output {} aside to {}",
                output.display(),
                backup.display()
            )
        })?;
    }

    if let Err(error) = place(&staging_path, &output) {
        if let Some(backup) = &backup
            && let Err(restore_error) = fs::rename(backup, &output)
        {
            let _ = fs::remove_dir_all(&staging_path);
            return Err(restore_error).with_context(|| {
                format!(
                    "restoring catalog output {} after replacement failure: {error}",
                    output.display()
                )
            });
        }
        let _ = fs::remove_dir_all(&staging_path);
        return Err(error)
            .with_context(|| format!("moving staged catalog directory into {}", output.display()));
    }

    if let Some(backup) = backup {
        fs::remove_dir_all(&backup)
            .with_context(|| format!("removing catalog backup {}", backup.display()))?;
    }
    Ok(())
}

fn validate_output(output: &Path) -> anyhow::Result<PathBuf> {
    let parent = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let name = output
        .file_name()
        .filter(|name| !name.is_empty())
        .context("catalog output must name a directory")?;
    let parent = canonicalize_parent_allowing_missing(parent)?;
    let normalized = parent.join(name);
    let filesystem_root = normalized
        .ancestors()
        .last()
        .context("catalog output must not be empty")?;
    if normalized == filesystem_root {
        anyhow::bail!(
            "catalog output {} must not be a filesystem root",
            normalized.display()
        );
    }

    let current = fs::canonicalize(env::current_dir().context("reading current directory")?)
        .context("canonicalizing current directory")?;
    if current.starts_with(&normalized) {
        anyhow::bail!(
            "catalog output {} must not be the current directory or its ancestor",
            normalized.display()
        );
    }

    let workspace_root = fs::canonicalize(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .context("finding project workspace root")?,
    )
    .context("canonicalizing project workspace root")?;
    if workspace_root.starts_with(&normalized) {
        anyhow::bail!(
            "catalog output {} must not be the project workspace root or its ancestor",
            normalized.display()
        );
    }

    match fs::symlink_metadata(&normalized) {
        Ok(metadata) if !metadata.file_type().is_dir() => {
            anyhow::bail!(
                "catalog output {} must be a real directory when it already exists",
                normalized.display()
            );
        }
        Ok(_) => {
            for entry in fs::read_dir(&normalized)
                .with_context(|| format!("reading catalog output {}", normalized.display()))?
            {
                let entry = entry.with_context(|| {
                    format!("reading entry in catalog output {}", normalized.display())
                })?;
                let path = entry.path();
                if !fs::symlink_metadata(&path)
                    .with_context(|| format!("reading catalog output entry {}", path.display()))?
                    .file_type()
                    .is_file()
                {
                    anyhow::bail!(
                        "catalog output contains non-regular entry {}",
                        path.display()
                    );
                }
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error)
                .with_context(|| format!("reading catalog output {}", normalized.display()));
        }
    }

    fs::create_dir_all(&parent)
        .with_context(|| format!("creating catalog output parent {}", parent.display()))?;

    Ok(normalized)
}

fn canonicalize_parent_allowing_missing(parent: &Path) -> anyhow::Result<PathBuf> {
    let original = parent;
    let mut ancestor = parent;
    let mut missing = Vec::<OsString>::new();

    loop {
        match fs::canonicalize(ancestor) {
            Ok(mut canonical) => {
                for component in missing.iter().rev() {
                    canonical.push(component);
                }
                return Ok(canonical);
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let component = ancestor.file_name().with_context(|| {
                    format!(
                        "catalog output parent {} contains an unresolved path component",
                        original.display()
                    )
                })?;
                missing.push(component.to_owned());
                ancestor = ancestor
                    .parent()
                    .filter(|parent| !parent.as_os_str().is_empty())
                    .unwrap_or_else(|| Path::new("."));
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!(
                        "canonicalizing catalog output parent {}",
                        original.display()
                    )
                });
            }
        }
    }
}

fn render(
    filename: &'static str,
    entries: &BTreeSet<String>,
) -> anyhow::Result<(&'static str, String)> {
    if entries.is_empty() {
        anyhow::bail!("catalog {filename} must contain at least one entry");
    }
    if entries
        .iter()
        .any(|entry| entry.is_empty() || entry.contains('\n'))
    {
        anyhow::bail!("catalog {filename} contains an empty or multiline entry");
    }

    Ok((
        filename,
        format!(
            "{}\n",
            entries.iter().cloned().collect::<Vec<_>>().join("\n")
        ),
    ))
}

fn unique_backup_path(parent: &Path) -> anyhow::Result<PathBuf> {
    let backup = tempfile::Builder::new()
        .prefix(".catalogs-backup-")
        .tempdir_in(parent)
        .with_context(|| format!("creating catalog backup directory in {}", parent.display()))?;
    let backup_path = backup.path().to_path_buf();
    backup
        .close()
        .with_context(|| format!("preparing a backup location in {}", parent.display()))?;
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::env;
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;

    use crate::CatalogKind;
    use crate::CatalogSet;

    #[test]
    fn writes_a_complete_sorted_catalog_directory_that_loads_back_exactly() {
        let catalogs = complete_catalogs();
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("catalogs");

        catalogs.write_to(&dir).unwrap();

        assert_eq!(
            fs::read_to_string(dir.join("battle-types.txt")).unwrap(),
            "Siege\n"
        );
        assert_eq!(
            sorted_directory_names(&dir),
            CatalogKind::ALL
                .into_iter()
                .map(|kind| kind.filename().to_owned())
                .collect::<Vec<_>>()
        );
        for kind in CatalogKind::ALL {
            let bytes = fs::read(dir.join(kind.filename())).unwrap();
            assert!(bytes.ends_with(b"\n"));
            assert!(!bytes.ends_with(b"\n\n"));

            let contents = String::from_utf8(bytes).unwrap();
            let lines = contents.lines().collect::<Vec<_>>();
            let mut sorted_deduplicated = lines.clone();
            sorted_deduplicated.sort_unstable();
            sorted_deduplicated.dedup();
            assert_eq!(lines, sorted_deduplicated);
        }
        assert_eq!(CatalogSet::load(&dir).unwrap(), catalogs);
    }

    #[test]
    fn write_creates_a_missing_output_parent() {
        let root = tempfile::tempdir().unwrap();
        let output = root.path().join("missing/gen/catalogs");

        complete_catalogs().write_to(&output).unwrap();

        assert_eq!(CatalogSet::load(&output).unwrap(), complete_catalogs());
    }

    #[test]
    fn loading_rejects_a_missing_catalog_file() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("catalogs");
        complete_catalogs().write_to(&dir).unwrap();
        fs::remove_file(dir.join(CatalogKind::BattleTypes.filename())).unwrap();

        let error = CatalogSet::load(&dir).unwrap_err();

        assert!(error.to_string().contains("battle-types.txt"));
        assert!(
            error
                .to_string()
                .contains(dir.join("battle-types.txt").to_string_lossy().as_ref())
        );
    }

    #[test]
    fn loading_rejects_non_utf8_catalog_contents() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("catalogs");
        complete_catalogs().write_to(&dir).unwrap();
        fs::write(
            dir.join(CatalogKind::BattleTypes.filename()),
            b"valid\n\xff",
        )
        .unwrap();

        let error = CatalogSet::load(&dir).unwrap_err();

        assert!(error.to_string().contains("battle-types.txt"));
        assert!(
            error
                .to_string()
                .contains(dir.join("battle-types.txt").to_string_lossy().as_ref())
        );
    }

    #[test]
    fn loading_rejects_blank_entries_with_their_file_and_line() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("catalogs");
        complete_catalogs().write_to(&dir).unwrap();
        fs::write(
            dir.join(CatalogKind::BattleTypes.filename()),
            "Alpha\n\nBeta\n",
        )
        .unwrap();

        let error = CatalogSet::load(&dir).unwrap_err();

        assert!(error.to_string().contains("battle-types.txt"));
        assert!(error.to_string().contains("line 2"));
        assert!(
            error
                .to_string()
                .contains(dir.join("battle-types.txt").to_string_lossy().as_ref())
        );
    }

    #[test]
    fn loading_preserves_carriage_returns_before_newline_delimiters() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("catalogs");
        complete_catalogs().write_to(&dir).unwrap();
        fs::write(
            dir.join(CatalogKind::BattleTypes.filename()),
            b"Alpha\r\nBeta\n",
        )
        .unwrap();

        let catalogs = CatalogSet::load(&dir).unwrap();

        assert_eq!(
            catalogs
                .get(CatalogKind::BattleTypes)
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["Alpha\r", "Beta"]
        );
    }

    #[test]
    fn successful_write_replaces_an_unexpected_existing_file() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("catalogs");
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("unexpected.txt"), "stale\n").unwrap();

        complete_catalogs().write_to(&dir).unwrap();

        assert!(!dir.join("unexpected.txt").exists());
    }

    #[test]
    fn rejects_workspace_roots_and_current_directory_ancestors_before_writing() {
        let current = fs::canonicalize(env::current_dir().unwrap()).unwrap();
        let workspace_root = fs::canonicalize(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .parent()
                .unwrap(),
        )
        .unwrap();

        let current_manifest = current.join("Cargo.toml");
        let workspace_manifest = workspace_root.join("Cargo.toml");
        let current_before = fs::read(&current_manifest).unwrap();
        let workspace_before = fs::read(&workspace_manifest).unwrap();

        let current_error = complete_catalogs().write_to(&current).unwrap_err();
        let workspace_error = complete_catalogs().write_to(&workspace_root).unwrap_err();

        assert!(
            current_error
                .to_string()
                .contains(current.to_string_lossy().as_ref())
        );
        assert!(
            workspace_error
                .to_string()
                .contains(workspace_root.to_string_lossy().as_ref())
        );
        assert_eq!(fs::read(current_manifest).unwrap(), current_before);
        assert_eq!(fs::read(workspace_manifest).unwrap(), workspace_before);
    }

    #[test]
    fn rejects_nonregular_output_entries_before_mutating_the_destination() {
        let root = tempfile::tempdir().unwrap();
        let output = root.path().join("catalogs");
        fs::create_dir(&output).unwrap();
        fs::write(output.join("keep.txt"), b"keep").unwrap();
        fs::create_dir(output.join("nested")).unwrap();

        let error = complete_catalogs().write_to(&output).unwrap_err();

        assert!(error.to_string().contains("nested"));
        assert_eq!(fs::read(output.join("keep.txt")).unwrap(), b"keep");
        assert!(output.join("nested").is_dir());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_output_entries_before_mutating_the_destination() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let output = root.path().join("catalogs");
        fs::create_dir(&output).unwrap();
        fs::write(output.join("keep.txt"), b"keep").unwrap();
        symlink(output.join("keep.txt"), output.join("linked.txt")).unwrap();

        let error = complete_catalogs().write_to(&output).unwrap_err();

        assert!(error.to_string().contains("linked.txt"));
        assert_eq!(fs::read(output.join("keep.txt")).unwrap(), b"keep");
        assert!(fs::symlink_metadata(output.join("linked.txt")).is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_a_symlink_destination_before_mutating_its_target() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let target = root.path().join("target");
        let output = root.path().join("catalogs");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("keep.txt"), b"keep").unwrap();
        symlink(&target, &output).unwrap();

        let error = complete_catalogs().write_to(&output).unwrap_err();

        assert!(
            error
                .to_string()
                .contains(output.to_string_lossy().as_ref())
        );
        assert_eq!(fs::read(target.join("keep.txt")).unwrap(), b"keep");
    }

    #[test]
    fn rejects_empty_and_multiline_catalog_entries_before_creating_output() {
        let root = tempfile::tempdir().unwrap();
        for entries in [
            BTreeSet::new(),
            BTreeSet::from([String::new()]),
            BTreeSet::from(["Alpha\nBeta".to_owned()]),
        ] {
            let output = root.path().join(format!("catalogs-{}", entries.len()));

            let error = catalogs_with_battle_entries(&entries)
                .write_to(&output)
                .unwrap_err();

            assert!(error.to_string().contains("battle-types.txt"));
            assert!(!output.exists());
        }
    }

    #[test]
    fn restores_the_previous_output_when_final_placement_fails() {
        let root = tempfile::tempdir().unwrap();
        let output = root.path().join("catalogs");
        complete_catalogs().write_to(&output).unwrap();
        let previous_battle_types = fs::read(output.join("battle-types.txt")).unwrap();
        fs::write(output.join("unexpected.txt"), b"previous").unwrap();

        let error = complete_catalogs()
            .write_to_with_placement(&output, |_, _| Err(io::Error::other("forced failure")))
            .unwrap_err();

        assert!(format!("{error:#}").contains("forced failure"));
        assert_eq!(
            fs::read(output.join("battle-types.txt")).unwrap(),
            previous_battle_types
        );
        assert_eq!(
            fs::read(output.join("unexpected.txt")).unwrap(),
            b"previous"
        );
    }

    fn complete_catalogs() -> CatalogSet {
        catalogs_with_battle_entries(&BTreeSet::from(["Siege".to_owned()]))
    }

    fn catalogs_with_battle_entries(battle_entries: &BTreeSet<String>) -> CatalogSet {
        let entries = CatalogKind::ALL
            .into_iter()
            .map(|kind| {
                let values = if kind == CatalogKind::BattleTypes {
                    battle_entries.clone()
                } else {
                    ["Zulu", "Alpha", "Alpha"]
                        .into_iter()
                        .map(str::to_owned)
                        .collect()
                };
                (kind, values)
            })
            .collect::<BTreeMap<CatalogKind, BTreeSet<String>>>();
        CatalogSet::from_entries(entries).unwrap()
    }

    fn sorted_directory_names(dir: &Path) -> Vec<String> {
        let mut names = fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();
        names.sort_unstable();
        names
    }
}
