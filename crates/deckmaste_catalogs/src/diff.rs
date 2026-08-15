use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;

/// Differences between immediate entries in two catalog directories.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct DirectoryDiff {
    pub missing: Vec<PathBuf>,
    pub unexpected: Vec<PathBuf>,
    pub changed: Vec<PathBuf>,
}

impl DirectoryDiff {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.missing.is_empty() && self.unexpected.is_empty() && self.changed.is_empty()
    }
}

impl fmt::Display for DirectoryDiff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (label, paths) in [
            ("missing", &self.missing),
            ("unexpected", &self.unexpected),
            ("changed", &self.changed),
        ] {
            for path in paths {
                if !first {
                    writeln!(formatter)?;
                }
                write!(formatter, "{label}: {}", path.display())?;
                first = false;
            }
        }
        Ok(())
    }
}

/// Compares immediate file entries in `expected` and `actual` without mutation.
///
/// # Errors
///
/// Returns an error when a shared entry is a non-file in both directories, or
/// when either directory cannot be read.
pub fn compare_directories(
    expected: impl AsRef<Path>,
    actual: impl AsRef<Path>,
) -> anyhow::Result<DirectoryDiff> {
    let expected = expected.as_ref();
    let actual = actual.as_ref();
    let expected_names = directory_names(expected)?;
    let actual_names = directory_names(actual)?;
    let mut diff = DirectoryDiff {
        missing: expected_names
            .difference(&actual_names)
            .cloned()
            .map(PathBuf::from)
            .collect(),
        unexpected: actual_names
            .difference(&expected_names)
            .cloned()
            .map(PathBuf::from)
            .collect(),
        changed: Vec::new(),
    };

    for name in expected_names.intersection(&actual_names) {
        let expected_path = expected.join(name);
        let actual_path = actual.join(name);
        let expected_is_file = fs::symlink_metadata(&expected_path)
            .with_context(|| format!("reading expected entry {}", expected_path.display()))?
            .file_type()
            .is_file();
        let actual_is_file = fs::symlink_metadata(&actual_path)
            .with_context(|| format!("reading actual entry {}", actual_path.display()))?
            .file_type()
            .is_file();

        match (expected_is_file, actual_is_file) {
            (true, true) => {
                if fs::read(&expected_path)
                    .with_context(|| format!("reading expected file {}", expected_path.display()))?
                    != fs::read(&actual_path)
                        .with_context(|| format!("reading actual file {}", actual_path.display()))?
                {
                    diff.changed.push(PathBuf::from(name));
                }
            }
            (false, false) => anyhow::bail!("shared non-file entry {}", Path::new(name).display()),
            (true, false) => diff.unexpected.push(PathBuf::from(name)),
            (false, true) => anyhow::bail!("expected non-file entry {}", Path::new(name).display()),
        }
    }

    diff.missing.sort_unstable();
    diff.unexpected.sort_unstable();
    diff.changed.sort_unstable();
    Ok(diff)
}

fn directory_names(directory: &Path) -> anyhow::Result<BTreeSet<OsString>> {
    fs::read_dir(directory)
        .with_context(|| format!("reading catalog directory {}", directory.display()))?
        .map(|entry| {
            entry
                .map(|entry| entry.file_name())
                .with_context(|| format!("reading entry in {}", directory.display()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::compare_directories;

    #[test]
    fn classifies_directory_differences_without_mutating_the_actual_directory() {
        let root = tempfile::tempdir().unwrap();
        let expected = root.path().join("expected");
        let actual = root.path().join("actual");
        fs::create_dir(&expected).unwrap();
        fs::create_dir(&actual).unwrap();
        fs::write(expected.join("a-missing.txt"), b"missing").unwrap();
        fs::write(expected.join("z-missing.txt"), b"missing").unwrap();
        fs::write(actual.join("a-unexpected.txt"), b"unexpected").unwrap();
        fs::write(actual.join("z-unexpected.txt"), b"unexpected").unwrap();
        fs::write(expected.join("a-changed.txt"), b"expected").unwrap();
        fs::write(actual.join("a-changed.txt"), b"actual").unwrap();
        fs::write(expected.join("z-changed.txt"), b"expected").unwrap();
        fs::write(actual.join("z-changed.txt"), b"actual").unwrap();
        fs::write(expected.join("same.txt"), b"same").unwrap();
        fs::write(actual.join("same.txt"), b"same").unwrap();
        let actual_before = [
            (
                "a-unexpected.txt",
                fs::read(actual.join("a-unexpected.txt")).unwrap(),
            ),
            (
                "z-unexpected.txt",
                fs::read(actual.join("z-unexpected.txt")).unwrap(),
            ),
            (
                "a-changed.txt",
                fs::read(actual.join("a-changed.txt")).unwrap(),
            ),
            (
                "z-changed.txt",
                fs::read(actual.join("z-changed.txt")).unwrap(),
            ),
            ("same.txt", fs::read(actual.join("same.txt")).unwrap()),
        ];

        let diff = compare_directories(&expected, &actual).unwrap();

        assert_eq!(
            diff.missing,
            [
                PathBuf::from("a-missing.txt"),
                PathBuf::from("z-missing.txt")
            ]
        );
        assert_eq!(
            diff.unexpected,
            [
                PathBuf::from("a-unexpected.txt"),
                PathBuf::from("z-unexpected.txt")
            ]
        );
        assert_eq!(
            diff.changed,
            [
                PathBuf::from("a-changed.txt"),
                PathBuf::from("z-changed.txt")
            ]
        );
        assert!(!diff.is_empty());
        assert_eq!(
            diff.to_string(),
            "missing: a-missing.txt\nmissing: z-missing.txt\nunexpected: a-unexpected.txt\nunexpected: z-unexpected.txt\nchanged: a-changed.txt\nchanged: z-changed.txt"
        );
        for (name, bytes) in actual_before {
            assert_eq!(fs::read(actual.join(name)).unwrap(), bytes);
        }
    }

    #[test]
    fn classifies_actual_non_files_as_unexpected_and_rejects_shared_non_files() {
        let root = tempfile::tempdir().unwrap();
        let expected = root.path().join("expected");
        let actual = root.path().join("actual");
        fs::create_dir(&expected).unwrap();
        fs::create_dir(&actual).unwrap();
        fs::create_dir(actual.join("only-actual-directory")).unwrap();

        let diff = compare_directories(&expected, &actual).unwrap();

        assert_eq!(diff.unexpected, [PathBuf::from("only-actual-directory")]);
        fs::create_dir(expected.join("shared-directory")).unwrap();
        fs::create_dir(actual.join("shared-directory")).unwrap();
        let error = compare_directories(&expected, &actual).unwrap_err();
        assert!(error.to_string().contains("shared-directory"));
    }

    #[test]
    fn reports_an_empty_diff_for_identical_files() {
        let root = tempfile::tempdir().unwrap();
        let expected = root.path().join("expected");
        let actual = root.path().join("actual");
        fs::create_dir(&expected).unwrap();
        fs::create_dir(&actual).unwrap();
        fs::write(expected.join("same.txt"), b"same").unwrap();
        fs::write(actual.join("same.txt"), b"same").unwrap();

        let diff = compare_directories(&expected, &actual).unwrap();

        assert!(diff.is_empty());
        assert_eq!(diff.to_string(), "");
    }
}
