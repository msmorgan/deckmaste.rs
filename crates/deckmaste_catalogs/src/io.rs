use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
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
            let contents = fs::read_to_string(directory.join(filename))
                .with_context(|| format!("reading {filename}"))?;
            let mut catalog = BTreeSet::new();
            for (line_number, line) in contents.lines().enumerate() {
                if line.is_empty() {
                    anyhow::bail!("blank entry in {filename} at line {}", line_number + 1);
                }
                catalog.insert(line.to_owned());
            }
            entries.insert(kind, catalog);
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
        let output = output.as_ref();
        let parent = output
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let output_name = output
            .file_name()
            .filter(|name| !name.is_empty())
            .context("catalog output must name a directory")?;

        if output.exists()
            && !fs::metadata(output)
                .with_context(|| format!("reading catalog output {}", output.display()))?
                .is_dir()
        {
            anyhow::bail!("catalog output {} is not a directory", output.display());
        }

        let rendered = CatalogKind::ALL
            .into_iter()
            .map(|kind| (kind.filename(), render(self.get(kind))))
            .collect::<Vec<_>>();
        let staging = tempfile::Builder::new()
            .prefix(".catalogs-")
            .tempdir_in(parent)
            .with_context(|| {
                format!("creating catalog staging directory in {}", parent.display())
            })?;
        for (filename, contents) in rendered {
            fs::write(staging.path().join(filename), contents)
                .with_context(|| format!("staging {filename}"))?;
        }
        let staging_path = staging.keep();

        let backup = if output.exists() {
            Some(unique_backup_path(parent, output_name)?)
        } else {
            None
        };
        if let Some(backup) = &backup {
            fs::rename(output, backup).with_context(|| {
                format!(
                    "moving existing catalog output {} aside to {}",
                    output.display(),
                    backup.display()
                )
            })?;
        }

        if let Err(error) = fs::rename(&staging_path, output) {
            if let Some(backup) = &backup {
                fs::rename(backup, output).with_context(|| {
                    format!(
                        "restoring catalog output {} after replacement failure: {error}",
                        output.display()
                    )
                })?;
            }
            let _ = fs::remove_dir_all(&staging_path);
            return Err(error).with_context(|| {
                format!("moving staged catalog directory into {}", output.display())
            });
        }

        if let Some(backup) = backup {
            fs::remove_dir_all(&backup)
                .with_context(|| format!("removing catalog backup {}", backup.display()))?;
        }
        Ok(())
    }
}

fn render(entries: &BTreeSet<String>) -> String {
    let mut rendered = entries.iter().cloned().collect::<Vec<_>>().join("\n");
    if !rendered.is_empty() {
        rendered.push('\n');
    }
    rendered
}

fn unique_backup_path(parent: &Path, output_name: &std::ffi::OsStr) -> anyhow::Result<PathBuf> {
    let backup = tempfile::Builder::new()
        .prefix(".catalogs-backup-")
        .tempdir_in(parent)
        .with_context(|| format!("creating catalog backup directory in {}", parent.display()))?;
    let backup_path = backup.path().to_path_buf();
    backup.close().with_context(|| {
        format!(
            "preparing a backup location for {} in {}",
            output_name.to_string_lossy(),
            parent.display()
        )
    })?;
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

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
    fn loading_rejects_a_missing_catalog_file() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("catalogs");
        complete_catalogs().write_to(&dir).unwrap();
        fs::remove_file(dir.join(CatalogKind::BattleTypes.filename())).unwrap();

        let error = CatalogSet::load(&dir).unwrap_err();

        assert!(error.to_string().contains("battle-types.txt"));
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

    fn complete_catalogs() -> CatalogSet {
        let entries = CatalogKind::ALL
            .into_iter()
            .map(|kind| {
                let values = if kind == CatalogKind::BattleTypes {
                    ["Siege"].into_iter().map(str::to_owned).collect()
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
