use crate::CatalogError;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::error::IoContext;

use crate::CatalogKind;
use crate::CatalogSet;
use crate::OutputProblem;

impl CatalogSet {
    /// Loads the complete canonical catalog inventory from plain UTF-8 line files.
    ///
    /// # Errors
    ///
    /// Returns an error when a required file is missing, is not UTF-8, or
    /// contains a blank entry.
    pub fn load(directory: impl AsRef<Path>) -> Result<Self, CatalogError> {
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
    pub fn write_to(&self, output: impl AsRef<Path>) -> Result<(), CatalogError> {
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
    ) -> Result<(), CatalogError> {
        write_line_directory_with_placement(
            output.as_ref(),
            CatalogKind::ALL
                .into_iter()
                .map(|kind| (kind.filename(), self.get(kind))),
            place,
        )
    }
}

pub(crate) fn read_line_catalog(path: &Path) -> Result<BTreeSet<String>, CatalogError> {
    let contents = fs::read_to_string(path).at("reading", path)?;
    let mut catalog = BTreeSet::new();
    for (line_number, line) in contents.split_terminator('\n').enumerate() {
        if line.is_empty() {
            return Err(CatalogError::BlankEntry {
                path: path.into(),
                line: line_number + 1,
            });
        }
        catalog.insert(line.to_owned());
    }
    Ok(catalog)
}

pub(crate) fn write_line_directory<'a>(
    output: &Path,
    entries: impl IntoIterator<Item = (&'static str, &'a BTreeSet<String>)>,
) -> Result<(), CatalogError> {
    write_line_directory_with_placement(output, entries, |source, destination| {
        fs::rename(source, destination)
    })
}

fn write_line_directory_with_placement<'a>(
    output: &Path,
    entries: impl IntoIterator<Item = (&'static str, &'a BTreeSet<String>)>,
    place: impl FnOnce(&Path, &Path) -> io::Result<()>,
) -> Result<(), CatalogError> {
    let output = validate_output(output)?;
    let parent = output
        .parent()
        .ok_or_else(|| invalid_output(&output, OutputProblem::MissingParent))?;

    let rendered = entries
        .into_iter()
        .map(|(filename, values)| render(filename, values))
        .collect::<Result<Vec<_>, CatalogError>>()?;
    let staging = tempfile::Builder::new()
        .prefix(".catalogs-")
        .tempdir_in(parent)
        .at("creating catalog staging directory", parent)?;
    for (filename, contents) in rendered {
        fs::write(staging.path().join(filename), contents)
            .at("staging", &staging.path().join(filename))?;
    }
    let staging_path = staging.keep();

    let backup = if output.exists() { Some(unique_backup_path(parent)?) } else { None };
    if let Some(backup) = &backup {
        fs::rename(&output, backup).at("moving existing catalog output aside", &output)?;
    }

    if let Err(error) = place(&staging_path, &output) {
        if let Some(backup) = &backup
            && let Err(restore_error) = fs::rename(backup, &output)
        {
            let _ = fs::remove_dir_all(&staging_path);
            return Err(CatalogError::Restore {
                output,
                backup: backup.clone(),
                staging: staging_path,
                replacement: error,
                source: restore_error,
            });
        }
        let _ = fs::remove_dir_all(&staging_path);
        return Err(error).at("moving staged catalog directory into", &output);
    }

    if let Some(backup) = backup {
        fs::remove_dir_all(&backup).at("removing catalog backup", &backup)?;
    }
    Ok(())
}

fn invalid_output(path: &Path, problem: OutputProblem) -> CatalogError {
    CatalogError::InvalidOutput {
        path: path.into(),
        problem,
    }
}

fn validate_output(output: &Path) -> Result<PathBuf, CatalogError> {
    let parent = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let name = output
        .file_name()
        .filter(|name| !name.is_empty())
        .ok_or_else(|| invalid_output(output, OutputProblem::MissingName))?;
    let parent = canonicalize_parent_allowing_missing(parent)?;
    let normalized = parent.join(name);
    let filesystem_root = normalized
        .ancestors()
        .last()
        .ok_or_else(|| invalid_output(output, OutputProblem::Empty))?;
    if normalized == filesystem_root {
        return Err(invalid_output(&normalized, OutputProblem::FilesystemRoot));
    }

    let current =
        fs::canonicalize(env::current_dir().at("reading current directory", Path::new("."))?)
            .at("canonicalizing current directory", Path::new("."))?;
    if current.starts_with(&normalized) {
        return Err(invalid_output(&normalized, OutputProblem::CurrentDirectory));
    }

    let workspace_root = fs::canonicalize(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .ok_or_else(|| invalid_output(output, OutputProblem::WorkspaceRoot))?,
    )
    .at(
        "canonicalizing project workspace root",
        Path::new(env!("CARGO_MANIFEST_DIR")),
    )?;
    if workspace_root.starts_with(&normalized) {
        return Err(invalid_output(&normalized, OutputProblem::WorkspaceRoot));
    }

    match fs::symlink_metadata(&normalized) {
        Ok(metadata) if !metadata.file_type().is_dir() => {
            return Err(invalid_output(&normalized, OutputProblem::NotDirectory));
        }
        Ok(_) => {
            for entry in fs::read_dir(&normalized).at("reading catalog output", &normalized)? {
                let entry = entry.at("reading entry in catalog output", &normalized)?;
                let path = entry.path();
                if !fs::symlink_metadata(&path)
                    .at("reading catalog output entry", &path)?
                    .file_type()
                    .is_file()
                {
                    return Err(invalid_output(&path, OutputProblem::NonRegularEntry));
                }
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).at("reading catalog output", &normalized);
        }
    }

    fs::create_dir_all(&parent).at("creating catalog output parent", &parent)?;

    Ok(normalized)
}

fn canonicalize_parent_allowing_missing(parent: &Path) -> Result<PathBuf, CatalogError> {
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
                let component = ancestor
                    .file_name()
                    .ok_or_else(|| invalid_output(original, OutputProblem::UnresolvedComponent))?;
                missing.push(component.to_owned());
                ancestor = ancestor
                    .parent()
                    .filter(|parent| !parent.as_os_str().is_empty())
                    .unwrap_or_else(|| Path::new("."));
            }
            Err(error) => {
                return Err(error).at("canonicalizing catalog output parent", original);
            }
        }
    }
}

fn render(
    filename: &'static str,
    entries: &BTreeSet<String>,
) -> Result<(&'static str, String), CatalogError> {
    if entries.is_empty() {
        return Err(CatalogError::EmptyCatalog {
            catalog: filename.into(),
        });
    }
    if let Some(entry) = entries
        .iter()
        .find(|entry| entry.is_empty() || entry.contains('\n'))
    {
        return Err(CatalogError::InvalidEntry {
            catalog: filename.into(),
            entry: entry.clone(),
        });
    }

    Ok((
        filename,
        format!(
            "{}\n",
            entries.iter().cloned().collect::<Vec<_>>().join("\n")
        ),
    ))
}

fn unique_backup_path(parent: &Path) -> Result<PathBuf, CatalogError> {
    let backup = tempfile::Builder::new()
        .prefix(".catalogs-backup-")
        .tempdir_in(parent)
        .at("creating catalog backup directory", parent)?;
    let backup_path = backup.path().to_path_buf();
    backup.close().at("preparing a backup location", parent)?;
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

        match &error {
            crate::CatalogError::Io { path, source, .. } => {
                assert_eq!(path, &dir.join("battle-types.txt"));
                assert!(
                    std::error::Error::source(&error)
                        .unwrap()
                        .is::<std::io::Error>()
                );
                assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
            }
            other => panic!("{other:?}"),
        }
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

        match &error {
            crate::CatalogError::Io { path, source, .. } => {
                assert_eq!(path, &dir.join("battle-types.txt"));
                assert!(
                    std::error::Error::source(&error)
                        .unwrap()
                        .is::<std::io::Error>()
                );
                assert_eq!(source.kind(), std::io::ErrorKind::InvalidData);
            }
            other => panic!("{other:?}"),
        }
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

        match &error {
            crate::CatalogError::BlankEntry { path, line } => {
                assert_eq!(path, &dir.join("battle-types.txt"));
                assert_eq!(*line, 2);
            }
            other => panic!("{other:?}"),
        }
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

            match &error {
                crate::CatalogError::EmptyCatalog { catalog } => {
                    assert!(entries.is_empty());
                    assert_eq!(catalog, "battle-types.txt");
                }
                crate::CatalogError::InvalidEntry { catalog, entry } => {
                    assert_eq!(catalog, "battle-types.txt");
                    assert!(entries.contains(entry));
                }
                other => panic!("{other:?}"),
            }
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
