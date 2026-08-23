use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use tempfile::NamedTempFile;

use super::coverage::CoverageLockMode;
use super::coverage::CoverageReport;

const SCHEMA_VERSION_V1: u32 = 1;
const SCHEMA_VERSION_V2: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CoverageLockV1 {
    schema_version: u32,
    source_fingerprint: String,
    accepted: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CoverageLockV2 {
    schema_version: u32,
    source_fingerprint: String,
    covered: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoadedCoverageLock {
    V1(CoverageLockV1),
    V2(CoverageLockV2),
}

#[derive(Debug, serde::Deserialize)]
struct CoverageLockSchema {
    schema_version: u32,
}

impl CoverageLockV2 {
    fn new(source_fingerprint: String, covered: Vec<String>) -> anyhow::Result<Self> {
        let lock = Self {
            schema_version: SCHEMA_VERSION_V2,
            source_fingerprint,
            covered,
        };
        lock.validate(Path::new("<new coverage lock>"))?;
        Ok(lock)
    }

    fn validate(&self, path: &Path) -> anyhow::Result<()> {
        if self.schema_version != SCHEMA_VERSION_V2 {
            bail!(
                "invalid English-v2 coverage lock {}: unsupported schema version {}",
                path.display(),
                self.schema_version,
            );
        }
        validate_identity(&self.source_fingerprint, "source fingerprint", path)?;
        validate_vector(&self.covered, "covered corpus identity", path)
    }
}

#[cfg(test)]
impl LoadedCoverageLock {
    fn source_fingerprint(&self) -> &str {
        match self {
            Self::V1(lock) => &lock.source_fingerprint,
            Self::V2(lock) => &lock.source_fingerprint,
        }
    }

    fn covered(&self) -> &[String] {
        match self {
            Self::V1(lock) => &lock.accepted,
            Self::V2(lock) => &lock.covered,
        }
    }
}

pub(super) fn read_lock(path: &Path) -> anyhow::Result<LoadedCoverageLock> {
    let bytes = fs::read(path)
        .with_context(|| format!("reading English-v2 coverage lock {}", path.display()))?;
    let schema = serde_json::from_slice::<CoverageLockSchema>(&bytes)
        .with_context(|| format!("parsing English-v2 coverage lock {}", path.display()))?;
    match schema.schema_version {
        SCHEMA_VERSION_V1 => {
            let lock = serde_json::from_slice::<CoverageLockV1>(&bytes).with_context(|| {
                format!(
                    "parsing schema-1 English-v2 coverage lock {}",
                    path.display()
                )
            })?;
            validate_identity(&lock.source_fingerprint, "source fingerprint", path)?;
            validate_vector(&lock.accepted, "accepted corpus identity", path)?;
            Ok(LoadedCoverageLock::V1(lock))
        }
        SCHEMA_VERSION_V2 => {
            let lock = serde_json::from_slice::<CoverageLockV2>(&bytes).with_context(|| {
                format!(
                    "parsing schema-2 English-v2 coverage lock {}",
                    path.display()
                )
            })?;
            lock.validate(path)?;
            Ok(LoadedCoverageLock::V2(lock))
        }
        version => bail!(
            "invalid English-v2 coverage lock {}: unsupported schema version {version}",
            path.display(),
        ),
    }
}

fn validate_vector(values: &[String], kind: &str, path: &Path) -> anyhow::Result<()> {
    for value in values {
        validate_identity(value, kind, path)?;
    }
    for pair in values.windows(2) {
        match pair[0].cmp(&pair[1]) {
            std::cmp::Ordering::Less => {}
            std::cmp::Ordering::Equal => bail!(
                "invalid English-v2 coverage lock {}: duplicate {kind} {}",
                path.display(),
                pair[0],
            ),
            std::cmp::Ordering::Greater => bail!(
                "invalid English-v2 coverage lock {}: {kind} vector must be strictly sorted",
                path.display(),
            ),
        }
    }
    Ok(())
}

fn validate_identity(value: &str, kind: &str, path: &Path) -> anyhow::Result<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Ok(());
    }
    bail!(
        "invalid English-v2 coverage lock {}: {kind} must be lowercase 64-hex",
        path.display(),
    );
}

trait LockWriter {
    fn create(&mut self, parent: &Path) -> anyhow::Result<NamedTempFile>;
    fn write_all(&mut self, temporary: &mut NamedTempFile, bytes: &[u8]) -> anyhow::Result<()>;
    fn flush(&mut self, temporary: &mut NamedTempFile) -> anyhow::Result<()>;
    fn persist(&mut self, temporary: NamedTempFile, path: &Path) -> anyhow::Result<()>;
}

pub(super) struct FilesystemLockWriter;

impl LockWriter for FilesystemLockWriter {
    fn create(&mut self, parent: &Path) -> anyhow::Result<NamedTempFile> {
        NamedTempFile::new_in(parent).map_err(anyhow::Error::new)
    }

    fn write_all(&mut self, temporary: &mut NamedTempFile, bytes: &[u8]) -> anyhow::Result<()> {
        temporary.write_all(bytes).map_err(anyhow::Error::new)
    }

    fn flush(&mut self, temporary: &mut NamedTempFile) -> anyhow::Result<()> {
        temporary.flush().map_err(anyhow::Error::new)
    }

    fn persist(&mut self, temporary: NamedTempFile, path: &Path) -> anyhow::Result<()> {
        temporary
            .persist(path)
            .map(|_| ())
            .map_err(|error| anyhow::Error::new(error.error))
    }
}

fn write_v2_with(
    lock: &CoverageLockV2,
    path: &Path,
    writer: &mut impl LockWriter,
) -> anyhow::Result<()> {
    lock.validate(path)?;
    let mut serialized =
        serde_json::to_vec_pretty(lock).context("serializing English-v2 coverage lock")?;
    serialized.push(b'\n');
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let mut temporary = writer.create(parent).with_context(|| {
        format!(
            "creating temporary English-v2 coverage lock beside {}",
            path.display()
        )
    })?;
    writer
        .write_all(&mut temporary, &serialized)
        .context("writing temporary English-v2 coverage lock")?;
    writer
        .flush(&mut temporary)
        .context("flushing temporary English-v2 coverage lock")?;
    writer
        .persist(temporary, path)
        .with_context(|| format!("persisting English-v2 coverage lock {}", path.display()))?;
    Ok(())
}

pub(super) fn apply(
    report: &CoverageReport,
    path: &Path,
    mode: CoverageLockMode,
    diagnostics: &mut dyn Write,
) -> anyhow::Result<()> {
    apply_with_writer(report, path, mode, diagnostics, &mut FilesystemLockWriter)
}

fn apply_with_writer(
    report: &CoverageReport,
    path: &Path,
    mode: CoverageLockMode,
    diagnostics: &mut dyn Write,
    writer: &mut impl LockWriter,
) -> anyhow::Result<()> {
    let (selected_uncovered, unresolved, internal) = report.gate_failure_counts();
    if selected_uncovered != 0 {
        bail!(
            "coverage gate rejected {selected_uncovered} selected-uncovered unit{}",
            if selected_uncovered == 1 { "" } else { "s" },
        );
    }
    if unresolved != 0 {
        bail!(
            "coverage gate rejected {unresolved} unresolved ambiguit{}",
            if unresolved == 1 { "y" } else { "ies" },
        );
    }
    if internal != 0 {
        bail!(
            "coverage gate rejected {internal} internal failure{}",
            if internal == 1 { "" } else { "s" },
        );
    }
    let current = report.selected_covered_ids()?;
    let baseline = if path.exists() { Some(read_lock(path)?) } else { None };
    match (mode, baseline) {
        (CoverageLockMode::None, _) => Ok(()),
        (CoverageLockMode::Check, None) => bail!(
            "English-v2 coverage lock {} is missing; review the coverage report and rerun with --bless",
            path.display(),
        ),
        (CoverageLockMode::Bless, None) => {
            let replacement = CoverageLockV2::new(report.source_fingerprint().to_owned(), current)?;
            write_v2_with(&replacement, path, writer)
        }
        (CoverageLockMode::Check, Some(LoadedCoverageLock::V1(_))) => bail!(
            "English-v2 coverage lock {} uses schema 1; migrate it with coverage --bless",
            path.display(),
        ),
        (CoverageLockMode::Bless, Some(LoadedCoverageLock::V1(baseline))) => {
            if current != baseline.accepted {
                bail!(
                    "coverage schema-1 migration requires the current covered vector to exactly equal all {} accepted identities",
                    baseline.accepted.len(),
                );
            }
            let replacement = CoverageLockV2::new(report.source_fingerprint().to_owned(), current)?;
            write_v2_with(&replacement, path, writer)
        }
        (mode, Some(LoadedCoverageLock::V2(baseline))) => {
            let lost = baseline
                .covered
                .iter()
                .filter(|identity| current.binary_search(identity).is_err())
                .collect::<Vec<_>>();
            if !lost.is_empty() {
                bail!(
                    "lost {} previously covered corpus identit{}:\n{}",
                    lost.len(),
                    if lost.len() == 1 { "y" } else { "ies" },
                    lost.into_iter()
                        .map(String::as_str)
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
            }
            if mode == CoverageLockMode::Check {
                if baseline.source_fingerprint != report.source_fingerprint() {
                    writeln!(
                        diagnostics,
                        "coverage lock source fingerprint changed: old {} new {}",
                        baseline.source_fingerprint,
                        report.source_fingerprint(),
                    )
                    .context("writing English-v2 coverage lock diagnostic")?;
                }
                for identity in current
                    .iter()
                    .filter(|identity| baseline.covered.binary_search(identity).is_err())
                {
                    writeln!(diagnostics, "newly covered\t{identity}")
                        .context("writing English-v2 coverage lock diagnostic")?;
                }
                return Ok(());
            }
            let replacement = CoverageLockV2::new(report.source_fingerprint().to_owned(), current)?;
            write_v2_with(&replacement, path, writer)
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailureStage {
    Create,
    Write,
    Flush,
    Persist,
}

#[cfg(test)]
impl FailureStage {
    const fn name(self) -> &'static str {
        match self {
            Self::Create => "creating",
            Self::Write => "writing",
            Self::Flush => "flushing",
            Self::Persist => "persisting",
        }
    }

    const fn expected_events(self) -> &'static [&'static str] {
        match self {
            Self::Create => &["create"],
            Self::Write => &["create", "write"],
            Self::Flush => &["create", "write", "flush"],
            Self::Persist => &["create", "write", "flush", "persist"],
        }
    }
}

#[cfg(test)]
struct FailingLockWriter {
    failure: FailureStage,
    events: Vec<&'static str>,
}

#[cfg(test)]
impl FailingLockWriter {
    fn new(failure: FailureStage) -> Self {
        Self {
            failure,
            events: Vec::new(),
        }
    }
    fn events(&self) -> &[&'static str] {
        &self.events
    }
}

#[cfg(test)]
impl LockWriter for FailingLockWriter {
    fn create(&mut self, parent: &Path) -> anyhow::Result<NamedTempFile> {
        self.events.push("create");
        if self.failure == FailureStage::Create {
            bail!("injected create failure");
        }
        NamedTempFile::new_in(parent).map_err(anyhow::Error::new)
    }

    fn write_all(&mut self, temporary: &mut NamedTempFile, bytes: &[u8]) -> anyhow::Result<()> {
        self.events.push("write");
        if self.failure == FailureStage::Write {
            bail!("injected write failure");
        }
        temporary.write_all(bytes).map_err(anyhow::Error::new)
    }

    fn flush(&mut self, temporary: &mut NamedTempFile) -> anyhow::Result<()> {
        self.events.push("flush");
        if self.failure == FailureStage::Flush {
            bail!("injected flush failure");
        }
        temporary.flush().map_err(anyhow::Error::new)
    }

    fn persist(&mut self, _temporary: NamedTempFile, _path: &Path) -> anyhow::Result<()> {
        self.events.push("persist");
        bail!("injected persist failure")
    }
}

#[cfg(test)]
impl CoverageLockV2 {
    fn new_for_test(source_fingerprint: String, covered: Vec<String>) -> Self {
        Self::new(source_fingerprint, covered).unwrap()
    }
}

#[cfg(test)]
impl LoadedCoverageLock {
    fn covered_for_test(&self) -> &[String] {
        self.covered()
    }
    fn source_fingerprint_for_test(&self) -> &str {
        self.source_fingerprint()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;
    use std::fs;
    use std::path::Path;

    use sha2::Digest;
    use sha2::Sha256;

    use super::CoverageLockV2;
    use super::FailureStage;
    use super::LoadedCoverageLock;
    use super::apply_with_writer;
    use super::read_lock;
    use super::write_v2_with;
    use crate::english_v2::coverage::CoverageLockMode;
    use crate::english_v2::coverage::CoverageReport;

    fn id(digit: char) -> String {
        digit.to_string().repeat(64)
    }

    fn json_v1(source: &str, accepted: &[String]) -> Vec<u8> {
        let accepted = serde_json::to_string(accepted).unwrap();
        format!(
            "{{\n  \"schema_version\": 1,\n  \"source_fingerprint\": \"{source}\",\n  \"accepted\": {accepted}\n}}\n"
        )
        .into_bytes()
    }

    fn json_v2(source: &str, covered: &[String]) -> Vec<u8> {
        let covered = serde_json::to_string(covered).unwrap();
        format!(
            "{{\n  \"schema_version\": 2,\n  \"source_fingerprint\": \"{source}\",\n  \"covered\": {covered}\n}}\n"
        )
        .into_bytes()
    }

    fn report(
        source: &str,
        covered: Vec<String>,
        selected_uncovered: usize,
        parse_failures: usize,
        unresolved: usize,
        internal: usize,
    ) -> CoverageReport {
        CoverageReport::for_gate_test(
            source.to_owned(),
            covered,
            selected_uncovered,
            parse_failures,
            unresolved,
            internal,
        )
    }

    #[test]
    fn strict_reader_rejects_malformed_unknown_missing_mixed_and_extra_schemas() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        for (bytes, needle) in [
            (b"not json".to_vec(), "parsing"),
            (
                format!(
                    "{{\"schema_version\":3,\"source_fingerprint\":\"{}\",\"covered\":[]}}",
                    id('1')
                )
                .into_bytes(),
                "unsupported schema version 3",
            ),
            (
                format!(
                    "{{\"schema_version\":1,\"source_fingerprint\":\"{}\"}}",
                    id('1')
                )
                .into_bytes(),
                "missing field `accepted`",
            ),
            (
                format!(
                    "{{\"schema_version\":2,\"source_fingerprint\":\"{}\"}}",
                    id('1')
                )
                .into_bytes(),
                "missing field `covered`",
            ),
            (
                format!(
                    "{{\"schema_version\":1,\"source_fingerprint\":\"{}\",\"accepted\":[],\"covered\":[]}}",
                    id('1')
                )
                .into_bytes(),
                "unknown field `covered`",
            ),
            (
                format!(
                    "{{\"schema_version\":2,\"source_fingerprint\":\"{}\",\"covered\":[],\"accepted\":[]}}",
                    id('1')
                )
                .into_bytes(),
                "unknown field `accepted`",
            ),
            (
                format!(
                    "{{\"schema_version\":1,\"source_fingerprint\":\"{}\",\"accepted\":[],\"covered\":null}}",
                    id('1')
                )
                .into_bytes(),
                "unknown field",
            ),
            (
                format!(
                    "{{\"schema_version\":2,\"source_fingerprint\":\"{}\",\"covered\":[],\"accepted\":null}}",
                    id('1')
                )
                .into_bytes(),
                "unknown field",
            ),
            (
                format!(
                    "{{\"schema_version\":2,\"source_fingerprint\":\"{}\",\"covered\":[],\"covered\":[]}}",
                    id('1')
                )
                .into_bytes(),
                "duplicate field `covered`",
            ),
            (
                format!(
                    "{{\"schema_version\":2,\"schema_version\":2,\"source_fingerprint\":\"{}\",\"covered\":[]}}",
                    id('1')
                )
                .into_bytes(),
                "duplicate field `schema_version`",
            ),
            (
                format!(
                    "{{\"schema_version\":2,\"source_fingerprint\":\"{}\",\"covered\":[],\"extra\":true}}",
                    id('1')
                )
                .into_bytes(),
                "unknown field",
            ),
        ] {
            fs::write(&path, bytes).unwrap();
            let before = fs::read(&path).unwrap();
            let error = format!("{:#}", read_lock(&path).unwrap_err());
            assert!(error.contains(needle), "expected {needle:?} in {error:?}");
            assert!(error.contains(&path.display().to_string()));
            assert_eq!(fs::read(&path).unwrap(), before);
        }
    }

    #[test]
    fn strict_reader_rejects_bad_fingerprints_ids_duplicates_and_order() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        for (bytes, needle) in [
            (json_v1(&"A".repeat(64), &[id('a')]), "source fingerprint"),
            (
                json_v2(&id('1'), &["a".repeat(63)]),
                "covered corpus identity",
            ),
            (json_v1(&id('1'), &[id('a'), id('a')]), "duplicate"),
            (json_v2(&id('1'), &[id('b'), id('a')]), "strictly sorted"),
        ] {
            fs::write(&path, bytes).unwrap();
            let before = fs::read(&path).unwrap();
            let error = format!("{:#}", read_lock(&path).unwrap_err());
            assert!(error.contains(needle), "expected {needle:?} in {error:?}");
            assert_eq!(fs::read(&path).unwrap(), before);
        }
    }

    #[test]
    fn schema_one_check_is_rejected_and_exact_bless_alone_migrates_without_id_changes() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let accepted = vec![id('a'), id('b')];
        let baseline = json_v1(&id('1'), &accepted);
        fs::write(&path, &baseline).unwrap();
        let current = report(&id('1'), accepted.clone(), 0, 7, 0, 0);
        let mut diagnostics = Vec::new();

        let error = apply_with_writer(
            &current,
            &path,
            CoverageLockMode::Check,
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("schema 1"));
        assert_eq!(fs::read(&path).unwrap(), baseline);

        apply_with_writer(
            &current,
            &path,
            CoverageLockMode::Bless,
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
        )
        .unwrap();
        let migrated = fs::read(&path).unwrap();
        let loaded = read_lock(&path).unwrap();
        assert_eq!(
            loaded,
            LoadedCoverageLock::V2(CoverageLockV2::new_for_test(id('1'), accepted.clone()))
        );
        assert_eq!(loaded.covered_for_test(), accepted.as_slice());
        let value: serde_json::Value = serde_json::from_slice(&migrated).unwrap();
        assert!(value.get("accepted").is_none());
        assert_eq!(value["covered"], serde_json::json!(accepted));
        assert!(migrated.ends_with(b"\n"));
        assert!(!migrated.ends_with(b"\n\n"));
    }

    #[test]
    fn schema_one_migration_rejects_both_growth_and_loss_without_mutation() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let baseline = json_v1(&id('1'), &[id('a'), id('b')]);
        for current in [vec![id('a')], vec![id('a'), id('b'), id('c')]] {
            fs::write(&path, &baseline).unwrap();
            let error = apply_with_writer(
                &report(&id('1'), current, 0, 0, 0, 0),
                &path,
                CoverageLockMode::Bless,
                &mut Vec::new(),
                &mut super::FilesystemLockWriter,
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains("exact"), "{error}");
            assert_eq!(fs::read(&path).unwrap(), baseline);
        }
    }

    #[test]
    fn schema_two_check_allows_new_coverage_but_rejects_loss_and_bless_is_add_only() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let baseline = json_v2(&id('1'), &[id('a'), id('b')]);
        fs::write(&path, &baseline).unwrap();
        let growth = report(&id('2'), vec![id('a'), id('b'), id('c')], 0, 3, 0, 0);
        let mut diagnostics = Vec::new();
        apply_with_writer(
            &growth,
            &path,
            CoverageLockMode::Check,
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
        )
        .unwrap();
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(diagnostics.contains("source fingerprint changed"));
        assert!(diagnostics.contains(&format!("newly covered\t{}", id('c'))));
        assert_eq!(fs::read(&path).unwrap(), baseline);

        let loss = report(&id('2'), vec![id('b'), id('c')], 0, 0, 0, 0);
        let error = apply_with_writer(
            &loss,
            &path,
            CoverageLockMode::Bless,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("lost 1 previously covered corpus identity"));
        assert!(error.contains(&id('a')));
        assert_eq!(fs::read(&path).unwrap(), baseline);

        apply_with_writer(
            &growth,
            &path,
            CoverageLockMode::Bless,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap();
        assert_eq!(
            read_lock(&path).unwrap().covered_for_test(),
            &[id('a'), id('b'), id('c')]
        );
        assert_eq!(
            read_lock(&path).unwrap().source_fingerprint_for_test(),
            id('2')
        );
    }

    #[test]
    fn missing_lock_is_check_error_but_bless_creates_schema_two() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let current = report(&id('1'), vec![id('a')], 0, 1, 0, 0);
        assert!(
            apply_with_writer(
                &current,
                &path,
                CoverageLockMode::Check,
                &mut Vec::new(),
                &mut super::FilesystemLockWriter,
            )
            .unwrap_err()
            .to_string()
            .contains("missing")
        );
        assert!(!path.exists());

        apply_with_writer(
            &current,
            &path,
            CoverageLockMode::Bless,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap();
        assert!(matches!(
            read_lock(&path).unwrap(),
            LoadedCoverageLock::V2(_)
        ));
    }

    #[test]
    fn gate_rejects_uncovered_unresolved_and_internal_but_ignores_ordinary_parse_failures() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let baseline = json_v2(&id('1'), &[id('a')]);
        fs::write(&path, &baseline).unwrap();

        for (uncovered, unresolved, internal, needle) in [
            (1, 0, 0, "selected-uncovered"),
            (0, 1, 0, "unresolved"),
            (0, 0, 1, "internal"),
        ] {
            let error = apply_with_writer(
                &report(&id('1'), vec![id('a')], uncovered, 9, unresolved, internal),
                &path,
                CoverageLockMode::Check,
                &mut Vec::new(),
                &mut super::FilesystemLockWriter,
            )
            .unwrap_err()
            .to_string();
            assert!(error.contains(needle), "expected {needle:?} in {error:?}");
            assert_eq!(fs::read(&path).unwrap(), baseline);
        }

        apply_with_writer(
            &report(&id('1'), vec![id('a')], 0, 99, 0, 0),
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap();
    }

    #[test]
    fn every_atomic_write_failure_preserves_original_bytes_and_leaves_no_temporary() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let baseline = json_v2(&id('1'), &[id('a')]);
        let replacement = CoverageLockV2::new_for_test(id('2'), vec![id('a'), id('b')]);

        for stage in [
            FailureStage::Create,
            FailureStage::Write,
            FailureStage::Flush,
            FailureStage::Persist,
        ] {
            fs::write(&path, &baseline).unwrap();
            let mut writer = super::FailingLockWriter::new(stage);
            let error = write_v2_with(&replacement, &path, &mut writer)
                .unwrap_err()
                .to_string();
            assert!(error.contains(stage.name()), "{error}");
            assert_eq!(fs::read(&path).unwrap(), baseline);
            assert_eq!(fs::read_dir(directory.path()).unwrap().count(), 1);
            assert_eq!(writer.events(), stage.expected_events());
        }
    }

    #[test]
    fn validation_and_gate_failures_never_create_or_mutate_the_lock() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let malformed = b"malformed baseline\n".to_vec();
        fs::write(&path, &malformed).unwrap();
        let error = apply_with_writer(
            &report(&id('1'), vec![id('a')], 0, 0, 0, 0),
            &path,
            CoverageLockMode::Bless,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err();
        assert!(format!("{error:#}").contains("parsing"));
        assert_eq!(fs::read(&path).unwrap(), malformed);
    }

    #[test]
    fn production_schema_two_lock_is_the_exact_reviewed_ratchet_result() {
        const REND_SPIRIT: &str =
            "5a0bd9563d2e05ca394ee7bedc5e55f386f82ee16f4227c410565066c6585660";
        const EXPECTED_FILE_SHA256: &str =
            "755c90d4a0c7736ca11c6c94716d84a3e783c4bd685790266feab4d7ad06888d";
        const EXPECTED_SOURCE: &str =
            "e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd";

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../english-v2-coverage.lock");
        let bytes = fs::read(&path).unwrap();
        let mut digest = String::new();
        for byte in Sha256::digest(&bytes) {
            write!(&mut digest, "{byte:02x}").unwrap();
        }
        assert_eq!(digest, EXPECTED_FILE_SHA256);
        let loaded = read_lock(&path).expect("production coverage lock is strict schema 2");
        assert!(matches!(loaded, LoadedCoverageLock::V2(_)));
        assert_eq!(loaded.source_fingerprint_for_test(), EXPECTED_SOURCE);
        assert_eq!(loaded.covered_for_test().len(), 608);
        assert!(
            loaded
                .covered_for_test()
                .iter()
                .any(|candidate| candidate == REND_SPIRIT)
        );
    }
}
