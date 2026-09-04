use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Context;
use anyhow::bail;
use deckmaste_construction_core::macro_def::Onset;
use tempfile::NamedTempFile;

use super::coverage::CoverageLockMode;
use super::coverage::CoverageLockPolicy;
use super::coverage::CoverageReport;

const SCHEMA_VERSION_V3: u32 = 3;
const SCHEMA_VERSION_V4: u32 = 4;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CoverageLockV3 {
    schema_version: u32,
    source_fingerprint: String,
    normalization_digest: String,
    covered: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct NormalizationUnit {
    source_id: String,
    id: String,
    card_name: String,
    context_onset: Onset,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CoverageLockV4 {
    schema_version: u32,
    source_fingerprint: String,
    normalization_digest: String,
    normalization_units: Vec<NormalizationUnit>,
    covered: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LoadedCoverageLock {
    V3(CoverageLockV3),
    V4(CoverageLockV4),
}

#[derive(Debug, serde::Deserialize)]
struct CoverageLockSchema {
    schema_version: u32,
}

impl CoverageLockV3 {
    fn validate(&self, path: &Path) -> anyhow::Result<()> {
        if self.schema_version != SCHEMA_VERSION_V3 {
            bail!(
                "invalid English-v2 coverage lock {}: unsupported schema version {}",
                path.display(),
                self.schema_version,
            );
        }
        validate_identity(&self.source_fingerprint, "source fingerprint", path)?;
        validate_identity(&self.normalization_digest, "normalization digest", path)?;
        validate_vector(&self.covered, "covered corpus identity", path)
    }
}

impl CoverageLockV4 {
    fn new(report: &CoverageReport, covered: Vec<String>) -> anyhow::Result<Self> {
        let mut normalization_units = report
            .normalization_inputs()
            .map(|input| NormalizationUnit {
                source_id: input.source_id.to_owned(),
                id: input.id.to_owned(),
                card_name: input.card_name.to_owned(),
                context_onset: input.context_onset,
            })
            .collect::<Vec<_>>();
        normalization_units.sort_by(|left, right| left.source_id.cmp(&right.source_id));
        let lock = Self {
            schema_version: SCHEMA_VERSION_V4,
            source_fingerprint: report.source_fingerprint().to_owned(),
            normalization_digest: report.normalization_digest(),
            normalization_units,
            covered,
        };
        lock.validate(Path::new("<new coverage lock>"))?;
        Ok(lock)
    }

    fn validate(&self, path: &Path) -> anyhow::Result<()> {
        if self.schema_version != SCHEMA_VERSION_V4 {
            bail!(
                "invalid English-v2 coverage lock {}: unsupported schema version {}",
                path.display(),
                self.schema_version,
            );
        }
        validate_identity(&self.source_fingerprint, "source fingerprint", path)?;
        validate_identity(&self.normalization_digest, "normalization digest", path)?;
        validate_vector(&self.covered, "covered corpus identity", path)?;
        for unit in &self.normalization_units {
            validate_identity(&unit.source_id, "normalization source identity", path)?;
            validate_identity(&unit.id, "normalized corpus identity", path)?;
        }
        for pair in self.normalization_units.windows(2) {
            match pair[0].source_id.cmp(&pair[1].source_id) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => bail!(
                    "invalid English-v2 coverage lock {}: duplicate normalization source identity {}",
                    path.display(),
                    pair[0].source_id,
                ),
                std::cmp::Ordering::Greater => bail!(
                    "invalid English-v2 coverage lock {}: normalization units must be strictly sorted by source identity",
                    path.display(),
                ),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl LoadedCoverageLock {
    fn source_fingerprint(&self) -> &str {
        match self {
            Self::V3(lock) => &lock.source_fingerprint,
            Self::V4(lock) => &lock.source_fingerprint,
        }
    }

    fn covered(&self) -> &[String] {
        match self {
            Self::V3(lock) => &lock.covered,
            Self::V4(lock) => &lock.covered,
        }
    }
}

pub(super) fn read_lock(path: &Path) -> anyhow::Result<LoadedCoverageLock> {
    let bytes = fs::read(path)
        .with_context(|| format!("reading English-v2 coverage lock {}", path.display()))?;
    let schema = serde_json::from_slice::<CoverageLockSchema>(&bytes)
        .with_context(|| format!("parsing English-v2 coverage lock {}", path.display()))?;
    match schema.schema_version {
        SCHEMA_VERSION_V3 => {
            let lock = serde_json::from_slice::<CoverageLockV3>(&bytes).with_context(|| {
                format!(
                    "parsing schema-3 English-v2 coverage lock {}",
                    path.display()
                )
            })?;
            lock.validate(path)?;
            Ok(LoadedCoverageLock::V3(lock))
        }
        SCHEMA_VERSION_V4 => {
            let lock = serde_json::from_slice::<CoverageLockV4>(&bytes).with_context(|| {
                format!(
                    "parsing schema-4 English-v2 coverage lock {}",
                    path.display()
                )
            })?;
            lock.validate(path)?;
            Ok(LoadedCoverageLock::V4(lock))
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

#[derive(Clone, Copy)]
struct CoverageLockRequest<'a> {
    mode: CoverageLockMode,
    lock_policy: CoverageLockPolicy,
    retirement_path: Option<&'a Path>,
}

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

fn write_v4_with(
    lock: &CoverageLockV4,
    path: &Path,
    writer: &mut impl LockWriter,
) -> anyhow::Result<()> {
    lock.validate(path)?;
    let serialized = serialize_v4(lock)?;
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

fn serialize_v4(lock: &CoverageLockV4) -> anyhow::Result<Vec<u8>> {
    let mut output = Vec::new();
    writeln!(output, "{{").context("serializing English-v2 coverage lock")?;
    writeln!(output, "  \"schema_version\": {},", lock.schema_version)
        .context("serializing English-v2 coverage lock")?;
    writeln!(
        output,
        "  \"source_fingerprint\": {},",
        serde_json::to_string(&lock.source_fingerprint)?
    )
    .context("serializing English-v2 coverage lock")?;
    writeln!(
        output,
        "  \"normalization_digest\": {},",
        serde_json::to_string(&lock.normalization_digest)?
    )
    .context("serializing English-v2 coverage lock")?;
    writeln!(output, "  \"normalization_units\": [")
        .context("serializing English-v2 coverage lock")?;
    for (index, unit) in lock.normalization_units.iter().enumerate() {
        writeln!(
            output,
            "    {}{}",
            serde_json::to_string(unit)?,
            if index + 1 == lock.normalization_units.len() { "" } else { "," },
        )
        .context("serializing English-v2 coverage lock")?;
    }
    writeln!(output, "  ],").context("serializing English-v2 coverage lock")?;
    writeln!(output, "  \"covered\": [").context("serializing English-v2 coverage lock")?;
    for (index, identity) in lock.covered.iter().enumerate() {
        writeln!(
            output,
            "    {}{}",
            serde_json::to_string(identity)?,
            if index + 1 == lock.covered.len() { "" } else { "," },
        )
        .context("serializing English-v2 coverage lock")?;
    }
    writeln!(output, "  ]").context("serializing English-v2 coverage lock")?;
    writeln!(output, "}}").context("serializing English-v2 coverage lock")?;
    Ok(output)
}

pub(super) fn apply_with_retirement(
    report: &CoverageReport,
    path: &Path,
    mode: CoverageLockMode,
    lock_policy: CoverageLockPolicy,
    retirement_path: Option<&Path>,
    diagnostics: &mut dyn Write,
) -> anyhow::Result<()> {
    let mut authenticate = authenticate_retirement;
    apply_with_writer_and_retirement(
        report,
        path,
        CoverageLockRequest {
            mode,
            lock_policy,
            retirement_path,
        },
        diagnostics,
        &mut FilesystemLockWriter,
        &mut authenticate,
    )
}

#[cfg(test)]
fn apply_with_writer(
    report: &CoverageReport,
    path: &Path,
    mode: CoverageLockMode,
    diagnostics: &mut dyn Write,
    writer: &mut impl LockWriter,
) -> anyhow::Result<()> {
    apply_with_writer_and_retirement(
        report,
        path,
        CoverageLockRequest {
            mode,
            lock_policy: CoverageLockPolicy::Ratchet,
            retirement_path: None,
        },
        diagnostics,
        writer,
        &mut |_, _| Ok(()),
    )
}

#[cfg(test)]
fn apply_with_test_retirement(
    report: &CoverageReport,
    path: &Path,
    mode: CoverageLockMode,
    retirement_path: Option<&Path>,
    diagnostics: &mut dyn Write,
) -> anyhow::Result<()> {
    apply_with_writer_and_retirement(
        report,
        path,
        CoverageLockRequest {
            mode,
            lock_policy: CoverageLockPolicy::Ratchet,
            retirement_path,
        },
        diagnostics,
        &mut FilesystemLockWriter,
        &mut |_, _| Ok(()),
    )
}

fn write_v4_coverage_drift(
    report: &CoverageReport,
    baseline: &CoverageLockV4,
    current_normalization: &[NormalizationUnit],
    lost: &[&String],
    newly_covered: &[&String],
    lock_policy: CoverageLockPolicy,
    diagnostics: &mut dyn Write,
) -> anyhow::Result<()> {
    if baseline.source_fingerprint != report.source_fingerprint() {
        writeln!(
            diagnostics,
            "coverage lock source fingerprint changed: old {} new {}",
            baseline.source_fingerprint,
            report.source_fingerprint(),
        )
        .context("writing English-v2 coverage lock diagnostic")?;
    }
    let normalization_digest = report.normalization_digest();
    if baseline.normalization_digest != normalization_digest {
        writeln!(
            diagnostics,
            "coverage lock normalization digest changed: old {} new {normalization_digest}",
            baseline.normalization_digest,
        )
        .context("writing English-v2 coverage lock diagnostic")?;
        write_normalization_unit_drift(
            &baseline.normalization_units,
            current_normalization,
            diagnostics,
        )?;
    }
    write_report_delta(
        report,
        lost,
        newly_covered,
        Some(&baseline.normalization_units),
        lock_policy,
        diagnostics,
    )?;
    Ok(())
}

/// Renders the lock delta. Losses and gains are named from the current corpus;
/// an identity the corpus no longer holds at all falls back to the name the
/// baseline lock recorded for it.
fn write_report_delta(
    report: &CoverageReport,
    lost: &[&String],
    newly_covered: &[&String],
    baseline_units: Option<&[NormalizationUnit]>,
    lock_policy: CoverageLockPolicy,
    diagnostics: &mut dyn Write,
) -> anyhow::Result<()> {
    let details = (lock_policy == CoverageLockPolicy::Report
        && !(lost.is_empty() && newly_covered.is_empty()))
    .then(|| report.lock_delta_details());
    if lock_policy == CoverageLockPolicy::Report && !lost.is_empty() {
        writeln!(
            diagnostics,
            "no longer covered {} corpus identit{}",
            lost.len(),
            if lost.len() == 1 { "y" } else { "ies" },
        )
        .context("writing English-v2 coverage lock diagnostic")?;
        for identity in lost {
            let card_name = details
                .as_ref()
                .and_then(|details| details.get(identity.as_str()))
                .map(|detail| detail.card.clone())
                .or_else(|| {
                    baseline_units.and_then(|units| {
                        units
                            .iter()
                            .find(|unit| &unit.id == *identity)
                            .map(|unit| unit.card_name.clone())
                    })
                })
                .unwrap_or_else(|| "<unknown>".to_owned());
            writeln!(
                diagnostics,
                "no longer covered\t{identity}\tcard {}",
                serde_json::to_string(&card_name)?
            )
            .context("writing English-v2 coverage lock diagnostic")?;
        }
    }
    if !newly_covered.is_empty() {
        writeln!(
            diagnostics,
            "newly covered {} corpus identit{}",
            newly_covered.len(),
            if newly_covered.len() == 1 { "y" } else { "ies" },
        )
        .context("writing English-v2 coverage lock diagnostic")?;
    }
    for identity in newly_covered {
        let detail = if lock_policy == CoverageLockPolicy::Report {
            let evidence = details
                .as_ref()
                .and_then(|details| details.get(identity.as_str()))
                .filter(|detail| detail.selected_analysis.is_some())
                .context("newly covered identity lacks selected coverage evidence")?;
            format!(
                "\tcard {}\tselected_analysis {}",
                serde_json::to_string(&evidence.card)?,
                serde_json::to_string(&evidence.selected_analysis)?,
            )
        } else {
            String::new()
        };
        writeln!(diagnostics, "newly covered\t{identity}{detail}")
            .context("writing English-v2 coverage lock diagnostic")?;
    }
    Ok(())
}

fn write_normalization_unit_drift(
    baseline: &[NormalizationUnit],
    current: &[NormalizationUnit],
    diagnostics: &mut dyn Write,
) -> anyhow::Result<()> {
    let mut old = baseline.iter().peekable();
    let mut new = current.iter().peekable();
    while let (Some(old_unit), Some(new_unit)) = (old.peek(), new.peek()) {
        match old_unit.source_id.cmp(&new_unit.source_id) {
            std::cmp::Ordering::Less => {
                old.next();
            }
            std::cmp::Ordering::Greater => {
                new.next();
            }
            std::cmp::Ordering::Equal => {
                let card_name = serde_json::to_string(&new_unit.card_name)
                    .context("serializing normalization-drift card name")?;
                if old_unit.id != new_unit.id {
                    writeln!(
                        diagnostics,
                        "normalized text changed\told {}\tnew {}\tcard {card_name}",
                        old_unit.id, new_unit.id,
                    )
                    .context("writing English-v2 per-unit normalization diagnostic")?;
                }
                if old_unit.context_onset != new_unit.context_onset {
                    writeln!(
                        diagnostics,
                        "context onset changed\t{}\tcard {card_name}\told {}\tnew {}",
                        new_unit.id,
                        onset_name(old_unit.context_onset),
                        onset_name(new_unit.context_onset),
                    )
                    .context("writing English-v2 per-unit onset diagnostic")?;
                }
                old.next();
                new.next();
            }
        }
    }
    Ok(())
}

const fn onset_name(onset: Onset) -> &'static str {
    match onset {
        Onset::Consonant => "consonant",
        Onset::Vowel => "vowel",
    }
}

fn migrate_v3(
    report: &CoverageReport,
    baseline: &CoverageLockV3,
    current: Vec<String>,
    retirement_path: Option<&Path>,
    path: &Path,
    writer: &mut impl LockWriter,
) -> anyhow::Result<()> {
    if retirement_path.is_some() {
        bail!("a coverage retirement manifest requires an existing schema-4 lock");
    }
    if baseline.source_fingerprint != report.source_fingerprint()
        || baseline.normalization_digest != report.legacy_normalization_digest()
        || baseline.covered != current
    {
        bail!(
            "coverage schema-3 migration requires the current source fingerprint, legacy normalization digest, and covered vector to exactly equal the baseline",
        );
    }
    let replacement = CoverageLockV4::new(report, current)?;
    write_v4_with(&replacement, path, writer)
}

fn validate_gate_preconditions(
    report: &CoverageReport,
    mode: CoverageLockMode,
    lock_policy: CoverageLockPolicy,
    retirement_path: Option<&Path>,
) -> anyhow::Result<()> {
    if retirement_path.is_some() && mode != CoverageLockMode::Bless {
        bail!("a coverage retirement manifest is valid only with --bless");
    }
    if retirement_path.is_some() && lock_policy == CoverageLockPolicy::Report {
        bail!("coverage retirement manifests are unavailable in report mode");
    }
    let (selected_uncovered, unresolved, internal, exception_resolved, exception_uses) =
        report.gate_failure_counts();
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
    if exception_resolved != 0 || exception_uses != 0 {
        bail!(
            "coverage gate rejected {exception_resolved} exception-resolved unit{} and {exception_uses} exception use{}",
            if exception_resolved == 1 { "" } else { "s" },
            if exception_uses == 1 { "" } else { "s" },
        );
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "the lock transition keeps its validation, drift diagnostics, and atomic write ordering auditable together"
)]
fn apply_with_writer_and_retirement<Authenticate>(
    report: &CoverageReport,
    path: &Path,
    request: CoverageLockRequest<'_>,
    diagnostics: &mut dyn Write,
    writer: &mut impl LockWriter,
    authenticate: &mut Authenticate,
) -> anyhow::Result<()>
where
    Authenticate: FnMut(&Path, &[String]) -> anyhow::Result<()>,
{
    validate_gate_preconditions(
        report,
        request.mode,
        request.lock_policy,
        request.retirement_path,
    )?;
    let current = report.selected_covered_ids()?;
    let baseline = if path.exists() { Some(read_lock(path)?) } else { None };
    match (request.mode, baseline) {
        (CoverageLockMode::None, _) => Ok(()),
        (CoverageLockMode::Check, None) => bail!(
            "English-v2 coverage lock {} is missing; review the coverage report and rerun with --bless",
            path.display(),
        ),
        (CoverageLockMode::Bless, None) => {
            if request.retirement_path.is_some() {
                bail!("a coverage retirement manifest requires an existing schema-4 lock");
            }
            let replacement = CoverageLockV4::new(report, current)?;
            write_v4_with(&replacement, path, writer)
        }
        (CoverageLockMode::Check, Some(LoadedCoverageLock::V3(_)))
            if request.lock_policy == CoverageLockPolicy::Ratchet =>
        {
            bail!(
                "English-v2 coverage lock {} uses schema 3 without per-unit normalization inputs; migrate it with coverage --bless",
                path.display(),
            )
        }
        (CoverageLockMode::Bless, Some(LoadedCoverageLock::V3(baseline)))
            if request.lock_policy == CoverageLockPolicy::Ratchet =>
        {
            migrate_v3(
                report,
                &baseline,
                current,
                request.retirement_path,
                path,
                writer,
            )
        }
        (CoverageLockMode::Check, Some(LoadedCoverageLock::V3(baseline))) => {
            let lost = baseline
                .covered
                .iter()
                .filter(|identity| current.binary_search(identity).is_err())
                .collect::<Vec<_>>();
            let newly_covered = current
                .iter()
                .filter(|identity| baseline.covered.binary_search(identity).is_err())
                .collect::<Vec<_>>();
            write_report_delta(
                report,
                &lost,
                &newly_covered,
                None,
                request.lock_policy,
                diagnostics,
            )
        }
        (CoverageLockMode::Bless, Some(LoadedCoverageLock::V3(_))) => {
            let replacement = CoverageLockV4::new(report, current)?;
            write_v4_with(&replacement, path, writer)
        }
        (_, Some(LoadedCoverageLock::V4(baseline))) => {
            let replacement = CoverageLockV4::new(report, current.clone())?;
            let lost = baseline
                .covered
                .iter()
                .filter(|identity| current.binary_search(identity).is_err())
                .collect::<Vec<_>>();
            let newly_covered = current
                .iter()
                .filter(|identity| baseline.covered.binary_search(identity).is_err())
                .collect::<Vec<_>>();
            write_v4_coverage_drift(
                report,
                &baseline,
                &replacement.normalization_units,
                &lost,
                &newly_covered,
                request.lock_policy,
                diagnostics,
            )?;
            if request.lock_policy == CoverageLockPolicy::Ratchet && !lost.is_empty() {
                let Some(retirement_path) = request.retirement_path else {
                    bail!(
                        "lost {} previously covered corpus identit{}:\n{}",
                        lost.len(),
                        if lost.len() == 1 { "y" } else { "ies" },
                        lost.into_iter()
                            .map(String::as_str)
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );
                };
                let retired = read_retirement_manifest(retirement_path)?;
                let lost = lost.into_iter().cloned().collect::<Vec<_>>();
                if retired != lost {
                    bail!(
                        "coverage retirement manifest {} must exactly match the {} currently lost identit{}",
                        retirement_path.display(),
                        lost.len(),
                        if lost.len() == 1 { "y" } else { "ies" },
                    );
                }
                authenticate(retirement_path, &retired)?;
                writeln!(
                    diagnostics,
                    "retired {} previously covered corpus identit{}",
                    retired.len(),
                    if retired.len() == 1 { "y" } else { "ies" },
                )
                .context("writing English-v2 coverage retirement diagnostic")?;
                writeln!(
                    diagnostics,
                    "WARNING: coverage retirement is a coordinator ruling; a ticket-vs-purpose contradiction requires a STOP, and the landing record must contain a re-coverage or retirement obligation line",
                )
                .context("writing English-v2 coverage retirement authority diagnostic")?;
            } else if let Some(retirement_path) = request.retirement_path {
                bail!(
                    "coverage retirement manifest {} was supplied but no identities are currently lost",
                    retirement_path.display(),
                );
            }
            if request.mode == CoverageLockMode::Check
                && request.lock_policy == CoverageLockPolicy::Ratchet
            {
                if !newly_covered.is_empty() {
                    bail!(
                        "coverage lock has {} newly covered corpus identit{}; review the coverage report and rerun with --bless",
                        newly_covered.len(),
                        if newly_covered.len() == 1 { "y" } else { "ies" },
                    );
                }
                if baseline.source_fingerprint != report.source_fingerprint() {
                    bail!(
                        "coverage lock source fingerprint changed; review the coverage report and rerun with --bless",
                    );
                }
                if baseline.normalization_digest != replacement.normalization_digest {
                    bail!(
                        "coverage lock normalization digest changed; review the normalized corpus and rerun with --bless",
                    );
                }
                if baseline.normalization_units != replacement.normalization_units {
                    let index = baseline
                        .normalization_units
                        .iter()
                        .zip(&replacement.normalization_units)
                        .position(|(locked, current)| locked != current)
                        .unwrap_or_else(|| {
                            baseline
                                .normalization_units
                                .len()
                                .min(replacement.normalization_units.len())
                        });
                    bail!(
                        "coverage lock normalization unit records differ; first differing unit at index {index}: lock {:?}, current {:?}; review the normalized corpus and rerun with --bless",
                        baseline.normalization_units.get(index),
                        replacement.normalization_units.get(index),
                    );
                }
                return Ok(());
            }
            if request.mode == CoverageLockMode::Bless {
                write_v4_with(&replacement, path, writer)?;
            }
            Ok(())
        }
    }
}

fn authenticate_retirement(path: &Path, retired: &[String]) -> anyhow::Result<()> {
    let current_directory = std::env::current_dir().context("reading the current directory")?;
    let workspace_root_output = run_jj(&["workspace", "root"])?;
    let workspace_root = PathBuf::from(workspace_root_output.trim());
    let workspace_root = workspace_root
        .canonicalize()
        .with_context(|| format!("resolving jj workspace root {}", workspace_root.display()))?;
    let tracked_output = run_jj(&["file", "list", "-T", "path ++ \"\\n\""])?;
    let tracked = tracked_output
        .lines()
        .map(|relative| workspace_root.join(relative))
        .collect::<Vec<_>>();
    let manifest = absolute_existing_path(path, &current_directory)?;
    validate_retirement_manifest(path, &manifest, &tracked)?;

    let workspace_name = workspace_root
        .file_name()
        .and_then(|name| name.to_str())
        .context("the jj workspace root has no UTF-8 workspace name")?;
    if workspace_name == "default" {
        bail!(
            "coverage retirement requires a claimed feature ticket; current workspace is default"
        );
    }
    let ticket = workspace_root
        .join("docs/tickets/wip")
        .join(format!("{workspace_name}.md"));
    validate_claimed_ticket(&ticket, &tracked)?;
    let ticket_source = fs::read_to_string(&ticket)
        .with_context(|| format!("reading claimed retirement ticket {}", ticket.display()))?;
    validate_retirement_obligation(&ticket, &ticket_source, retired)
}

fn run_jj(arguments: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("jj")
        .arg("--no-pager")
        .arg("--ignore-working-copy")
        .args(arguments)
        .output()
        .with_context(|| format!("running jj {}", arguments.join(" ")))?;
    if !output.status.success() {
        bail!(
            "jj {} failed while authenticating coverage retirement: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr).trim(),
        );
    }
    String::from_utf8(output.stdout).context("jj output was not UTF-8")
}

fn absolute_existing_path(path: &Path, current_directory: &Path) -> anyhow::Result<PathBuf> {
    let absolute = if path.is_absolute() { path.to_owned() } else { current_directory.join(path) };
    absolute
        .canonicalize()
        .with_context(|| format!("resolving coverage retirement manifest {}", path.display()))
}

fn path_is_tracked(path: &Path, tracked: &[PathBuf]) -> bool {
    tracked.iter().any(|candidate| {
        candidate
            .canonicalize()
            .is_ok_and(|candidate| candidate == path)
    })
}

#[cfg(test)]
fn validate_retirement_authentication(
    manifest_argument: &Path,
    manifest: &Path,
    tracked: &[PathBuf],
    ticket: &Path,
    ticket_source: &str,
    retired: &[String],
) -> anyhow::Result<()> {
    validate_retirement_manifest(manifest_argument, manifest, tracked)?;
    validate_claimed_ticket(ticket, tracked)?;
    validate_retirement_obligation(ticket, ticket_source, retired)
}

fn validate_retirement_manifest(
    manifest_argument: &Path,
    manifest: &Path,
    tracked: &[PathBuf],
) -> anyhow::Result<()> {
    if path_is_tracked(manifest, tracked) {
        bail!(
            "coverage retirement manifest {} must be untracked; jj file list reports it as tracked",
            manifest_argument.display(),
        );
    }
    Ok(())
}

fn validate_claimed_ticket(ticket: &Path, tracked: &[PathBuf]) -> anyhow::Result<()> {
    if !path_is_tracked(ticket, tracked) {
        bail!(
            "coverage retirement requires tracked claimed ticket {}",
            ticket.display(),
        );
    }
    Ok(())
}

fn validate_retirement_obligation(
    ticket: &Path,
    ticket_source: &str,
    retired: &[String],
) -> anyhow::Result<()> {
    let mut landing_record = false;
    for line in ticket_source.lines() {
        let trimmed = line.trim();
        if trimmed == "## Landing record" {
            landing_record = true;
            continue;
        }
        if landing_record && trimmed.starts_with("## ") {
            break;
        }
        if landing_record {
            let obligation = trimmed
                .strip_prefix("- ")
                .unwrap_or(trimmed)
                .to_ascii_lowercase();
            let names_obligation = [
                "retirement/re-coverage obligation:",
                "re-coverage/retirement obligation:",
                "re-coverage or retirement obligation:",
                "re-coverage obligation:",
            ]
            .iter()
            .any(|label| obligation.starts_with(label));
            if names_obligation && retired.iter().all(|identity| line.contains(identity)) {
                return Ok(());
            }
        }
    }
    bail!(
        "claimed ticket {} must have a ## Landing record containing one obligation line that names every retired identity; accepted labels are `retirement/re-coverage obligation:`, `re-coverage/retirement obligation:`, `re-coverage or retirement obligation:`, and `re-coverage obligation:`",
        ticket.display(),
    )
}

fn read_retirement_manifest(path: &Path) -> anyhow::Result<Vec<String>> {
    let source = fs::read_to_string(path)
        .with_context(|| format!("reading English-v2 coverage retirement {}", path.display()))?;
    let retired = source.lines().map(str::to_owned).collect::<Vec<_>>();
    if retired.is_empty() {
        bail!(
            "invalid English-v2 coverage retirement {}: identity vector must not be empty",
            path.display(),
        );
    }
    validate_vector(&retired, "retired corpus identity", path)?;
    Ok(retired)
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
impl LoadedCoverageLock {
    fn covered_for_test(&self) -> &[String] {
        self.covered()
    }
    fn source_fingerprint_for_test(&self) -> &str {
        self.source_fingerprint()
    }
    fn normalization_digest_for_test(&self) -> &str {
        match self {
            Self::V3(lock) => &lock.normalization_digest,
            Self::V4(lock) => &lock.normalization_digest,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use std::process::Command;

    use deckmaste_construction_core::macro_def::Onset;

    use super::CoverageLockRequest;
    use super::CoverageLockV4;
    use super::FailureStage;
    use super::LoadedCoverageLock;
    use super::apply_with_retirement;
    use super::apply_with_test_retirement;
    use super::apply_with_writer;
    use super::apply_with_writer_and_retirement;
    use super::read_lock;
    use super::validate_retirement_authentication;
    use super::write_v4_with;
    use crate::english_v2::coverage::CoverageLockMode;
    use crate::english_v2::coverage::CoverageLockPolicy;
    use crate::english_v2::coverage::CoverageReport;

    fn id(digit: char) -> String {
        digit.to_string().repeat(64)
    }

    fn json_v3(source: &str, normalization: &str, covered: &[String]) -> Vec<u8> {
        let covered = serde_json::to_string(covered).unwrap();
        format!(
            "{{\n  \"schema_version\": 3,\n  \"source_fingerprint\": \"{source}\",\n  \"normalization_digest\": \"{normalization}\",\n  \"covered\": {covered}\n}}\n"
        )
        .into_bytes()
    }

    fn json_v4(report: &CoverageReport, covered: Vec<String>) -> Vec<u8> {
        let lock = CoverageLockV4::new(report, covered).unwrap();
        let mut bytes = serde_json::to_vec_pretty(&lock).unwrap();
        bytes.push(b'\n');
        bytes
    }

    fn json_v4_with_source(
        report: &CoverageReport,
        source_fingerprint: String,
        covered: Vec<String>,
    ) -> Vec<u8> {
        let mut lock = CoverageLockV4::new(report, covered).unwrap();
        lock.source_fingerprint = source_fingerprint;
        let mut bytes = serde_json::to_vec_pretty(&lock).unwrap();
        bytes.push(b'\n');
        bytes
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

    const AUTHENTICATOR_CHILD_SCENARIO: &str = "DECKMASTE_TEST_RETIREMENT_AUTHENTICATOR_SCENARIO";

    fn run_jj_in(directory: &Path, arguments: &[&str]) -> String {
        let output = Command::new("jj")
            .arg("--no-pager")
            .args(arguments)
            .current_dir(directory)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "jj {} failed:\n{}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr),
        );
        String::from_utf8(output.stdout).unwrap()
    }

    fn scratch_jj_workspace(
        workspace_name: &str,
        ticket_source: &str,
        manifest_is_tracked: bool,
    ) -> (tempfile::TempDir, PathBuf) {
        let directory = tempfile::tempdir().unwrap();
        let workspace = directory.path().join(workspace_name);
        let init = Command::new("jj")
            .arg("--no-pager")
            .args(["git", "init", "--no-colocate"])
            .arg(&workspace)
            .output()
            .unwrap();
        assert!(
            init.status.success(),
            "jj git init failed:\n{}",
            String::from_utf8_lossy(&init.stderr),
        );

        if !manifest_is_tracked {
            fs::write(workspace.join(".gitignore"), "retired.ids\n").unwrap();
        }
        let ticket = workspace
            .join("docs/tickets/wip")
            .join(format!("{workspace_name}.md"));
        fs::create_dir_all(ticket.parent().unwrap()).unwrap();
        fs::write(ticket, ticket_source).unwrap();
        let current = report(&id('1'), vec![id('b'), id('c')], 0, 0, 0, 0);
        fs::write(
            workspace.join("coverage.lock"),
            json_v4(&current, vec![id('a'), id('b')]),
        )
        .unwrap();
        fs::write(workspace.join("retired.ids"), format!("{}\n", id('a'))).unwrap();
        run_jj_in(&workspace, &["st"]);
        (directory, workspace)
    }

    fn run_authenticator_child(workspace: &Path, scenario: &str) {
        let output = Command::new(std::env::current_exe().unwrap())
            .arg("production_retirement_authenticator_uses_scratch_jj_workspaces")
            .arg("--nocapture")
            .env(AUTHENTICATOR_CHILD_SCENARIO, scenario)
            .current_dir(workspace)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "authenticator child {scenario:?} failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("1 passed"),
            "authenticator child filter did not run exactly one test:\n{}",
            String::from_utf8_lossy(&output.stdout),
        );
    }

    #[test]
    fn production_retirement_authenticator_uses_scratch_jj_workspaces() {
        if let Ok(scenario) = std::env::var(AUTHENTICATOR_CHILD_SCENARIO) {
            let current = report(&id('1'), vec![id('b'), id('c')], 0, 0, 0, 0);
            let mut diagnostics = Vec::new();
            let result = apply_with_retirement(
                &current,
                Path::new("coverage.lock"),
                CoverageLockMode::Bless,
                CoverageLockPolicy::Ratchet,
                Some(Path::new("retired.ids")),
                &mut diagnostics,
            );
            match scenario.as_str() {
                "success" => {
                    result.unwrap();
                    let diagnostics = String::from_utf8(diagnostics).unwrap();
                    assert!(
                        diagnostics.contains("newly covered 1 corpus identity"),
                        "{diagnostics}",
                    );
                    assert!(diagnostics.contains(&format!("newly covered\t{}", id('c'))));
                }
                "tracked" => {
                    let error = result.unwrap_err().to_string();
                    assert!(error.contains("must be untracked"), "{error}");
                    assert!(
                        error.contains("jj file list reports it as tracked"),
                        "{error}"
                    );
                }
                "default" => {
                    let error = result.unwrap_err().to_string();
                    assert!(error.contains("current workspace is default"), "{error}");
                }
                "obligation" => {
                    let error = result.unwrap_err().to_string();
                    for label in [
                        "`retirement/re-coverage obligation:`",
                        "`re-coverage/retirement obligation:`",
                        "`re-coverage or retirement obligation:`",
                        "`re-coverage obligation:`",
                    ] {
                        assert!(error.contains(label), "missing {label:?} from {error:?}");
                    }
                }
                other => panic!("unknown authenticator child scenario {other:?}"),
            }
            return;
        }

        let valid_ticket = format!(
            "## Landing record\n\n- Re-coverage obligation: {}\n",
            id('a'),
        );
        let (_success_directory, success_workspace) =
            scratch_jj_workspace("derived-retirement-ticket", &valid_ticket, false);
        run_authenticator_child(&success_workspace, "success");
        assert_eq!(
            read_lock(&success_workspace.join("coverage.lock"))
                .unwrap()
                .covered_for_test(),
            &[id('b'), id('c')],
        );

        let (_tracked_directory, tracked_workspace) =
            scratch_jj_workspace("tracked-retirement", &valid_ticket, true);
        let tracked_baseline = fs::read(tracked_workspace.join("coverage.lock")).unwrap();
        run_authenticator_child(&tracked_workspace, "tracked");
        assert_eq!(
            fs::read(tracked_workspace.join("coverage.lock")).unwrap(),
            tracked_baseline,
        );

        let (_default_directory, default_workspace) =
            scratch_jj_workspace("default", &valid_ticket, false);
        fs::write(
            default_workspace.join("pending.txt"),
            "must remain unsnapshotted\n",
        )
        .unwrap();
        run_authenticator_child(&default_workspace, "default");
        let tracked = run_jj_in(
            &default_workspace,
            &["--ignore-working-copy", "file", "list"],
        );
        assert!(
            !tracked.lines().any(|path| path == "pending.txt"),
            "{tracked}"
        );

        let invalid_ticket = "## Landing record\n\n- no retirement authority here\n";
        let (_obligation_directory, obligation_workspace) =
            scratch_jj_workspace("missing-obligation", invalid_ticket, false);
        run_authenticator_child(&obligation_workspace, "obligation");
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
                "missing field `normalization_digest`",
            ),
            (
                format!(
                    "{{\"schema_version\":4,\"source_fingerprint\":\"{}\",\"normalization_digest\":\"{}\",\"covered\":[]}}",
                    id('1'),
                    id('1')
                )
                .into_bytes(),
                "missing field `normalization_units`",
            ),
            (
                b"{\"schema_version\":1}".to_vec(),
                "unsupported schema version 1",
            ),
            (
                b"{\"schema_version\":2}".to_vec(),
                "unsupported schema version 2",
            ),
            (
                format!(
                    "{{\"schema_version\":3,\"source_fingerprint\":\"{}\",\"normalization_digest\":\"{}\",\"covered\":[],\"extra\":true}}",
                    id('1'),
                    id('2')
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
            (
                json_v3(&"A".repeat(64), &id('1'), &[id('a')]),
                "source fingerprint",
            ),
            (
                json_v3(&id('1'), &id('2'), &["a".repeat(63)]),
                "covered corpus identity",
            ),
            (
                json_v3(&id('1'), &id('2'), &[id('a'), id('a')]),
                "duplicate",
            ),
            (
                json_v3(&id('1'), &id('2'), &[id('b'), id('a')]),
                "strictly sorted",
            ),
            (
                json_v3(&id('1'), &"A".repeat(64), &[id('a')]),
                "normalization digest",
            ),
        ] {
            fs::write(&path, bytes).unwrap();
            let before = fs::read(&path).unwrap();
            let error = format!("{:#}", read_lock(&path).unwrap_err());
            assert!(error.contains(needle), "expected {needle:?} in {error:?}");
            assert_eq!(fs::read(&path).unwrap(), before);
        }
    }

    #[test]
    fn schema_four_rejects_bad_duplicate_and_unsorted_normalization_units() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let current = report(&id('1'), vec![id('a'), id('b')], 0, 0, 0, 0);
        let valid: serde_json::Value =
            serde_json::from_slice(&json_v4(&current, vec![id('a'), id('b')])).unwrap();

        let mut bad_identity = valid.clone();
        bad_identity["normalization_units"][0]["source_id"] = serde_json::json!("A".repeat(64));
        let mut duplicate = valid.clone();
        duplicate["normalization_units"][1]["source_id"] =
            duplicate["normalization_units"][0]["source_id"].clone();
        let mut unsorted = valid.clone();
        unsorted["normalization_units"]
            .as_array_mut()
            .unwrap()
            .reverse();
        let mut bad_onset = valid;
        bad_onset["normalization_units"][0]["context_onset"] = serde_json::json!("Unknown");

        for (value, needle) in [
            (bad_identity, "normalization source identity"),
            (duplicate, "duplicate normalization source identity"),
            (unsorted, "strictly sorted by source identity"),
            (bad_onset, "unknown variant"),
        ] {
            let bytes = serde_json::to_vec_pretty(&value).unwrap();
            fs::write(&path, &bytes).unwrap();
            let error = format!("{:#}", read_lock(&path).unwrap_err());
            assert!(error.contains(needle), "expected {needle:?} in {error:?}");
            assert_eq!(fs::read(&path).unwrap(), bytes);
        }
    }

    #[test]
    fn schema_three_requires_an_exact_bless_to_add_per_unit_inputs() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let covered = vec![id('a'), id('b')];
        let current = report(&id('1'), covered.clone(), 0, 3, 0, 0);
        let baseline = json_v3(&id('1'), &current.legacy_normalization_digest(), &covered);
        fs::write(&path, &baseline).unwrap();

        let error = apply_with_writer(
            &current,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("schema 3"), "{error}");
        assert!(error.contains("per-unit normalization inputs"), "{error}");
        assert_eq!(fs::read(&path).unwrap(), baseline);

        apply_with_writer(
            &current,
            &path,
            CoverageLockMode::Bless,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap();
        let migrated = fs::read(&path).unwrap();
        let loaded = read_lock(&path).unwrap();
        assert!(matches!(loaded, LoadedCoverageLock::V4(_)));
        assert_eq!(loaded.covered_for_test(), covered);
        assert_eq!(
            loaded.normalization_digest_for_test(),
            current.normalization_digest()
        );
        let value: serde_json::Value = serde_json::from_slice(&migrated).unwrap();
        assert_eq!(value["schema_version"], 4);
        assert_eq!(value["normalization_units"].as_array().unwrap().len(), 5);
        assert!(migrated.ends_with(b"\n"));
        assert!(!migrated.ends_with(b"\n\n"));

        let mismatched = report(&id('1'), vec![id('a')], 0, 3, 0, 0);
        fs::write(&path, &baseline).unwrap();
        let error = apply_with_writer(
            &mismatched,
            &path,
            CoverageLockMode::Bless,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("exactly equal"), "{error}");
        assert_eq!(fs::read(&path).unwrap(), baseline);
    }

    #[test]
    fn schema_four_check_localizes_drift_and_rejects_loss_while_bless_is_add_only() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let growth = report(&id('2'), vec![id('a'), id('b'), id('c')], 0, 3, 0, 0);
        let baseline = json_v4_with_source(&growth, id('1'), vec![id('a'), id('b')]);
        fs::write(&path, &baseline).unwrap();
        let mut diagnostics = Vec::new();
        let error = apply_with_writer(
            &growth,
            &path,
            CoverageLockMode::Check,
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("1 newly covered corpus identity"), "{error}");
        assert!(error.contains("--bless"), "{error}");
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(diagnostics.contains("source fingerprint changed"));
        assert!(diagnostics.contains(&format!("newly covered\t{}", id('c'))));
        assert_eq!(fs::read(&path).unwrap(), baseline);

        let loss = report(&id('2'), vec![id('b'), id('c')], 0, 0, 0, 0);
        let mut loss_diagnostics = Vec::new();
        let error = apply_with_writer(
            &loss,
            &path,
            CoverageLockMode::Bless,
            &mut loss_diagnostics,
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("lost 1 previously covered corpus identity"));
        assert!(error.contains(&id('a')));
        assert!(
            String::from_utf8(loss_diagnostics)
                .unwrap()
                .contains(&format!("newly covered\t{}", id('c')))
        );
        assert_eq!(fs::read(&path).unwrap(), baseline);

        let fingerprint_only = report(&id('2'), vec![id('a'), id('b')], 0, 0, 0, 0);
        let error = apply_with_writer(
            &fingerprint_only,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("source fingerprint changed"), "{error}");
        assert!(error.contains("--bless"), "{error}");
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
        assert_eq!(
            read_lock(&path).unwrap().normalization_digest_for_test(),
            growth.normalization_digest()
        );

        let blessed = fs::read(&path).unwrap();
        let mut normalization_drift = growth.clone();
        let old_id = format!("{:064x}", 0x1000);
        let changed_id = normalization_drift
            .set_first_parse_failure_text_for_test("changed only on a parse failure");
        let mut diagnostics = Vec::new();
        let error = apply_with_writer(
            &normalization_drift,
            &path,
            CoverageLockMode::Check,
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("normalization digest changed"), "{error}");
        assert!(error.contains("--bless"), "{error}");
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(
            diagnostics.contains("coverage lock normalization digest changed"),
            "{diagnostics}"
        );
        assert!(
            diagnostics.contains(&format!(
                "normalized text changed\told {old_id}\tnew {changed_id}\tcard \"Fixture\""
            )),
            "{diagnostics}"
        );
        assert_eq!(fs::read(&path).unwrap(), blessed);

        apply_with_writer(
            &normalization_drift,
            &path,
            CoverageLockMode::Bless,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap();
        assert_eq!(
            read_lock(&path).unwrap().normalization_digest_for_test(),
            normalization_drift.normalization_digest()
        );

        let text_blessed = fs::read(&path).unwrap();
        let mut onset_drift = normalization_drift.clone();
        onset_drift.set_first_parse_failure_onset_for_test(Onset::Vowel);
        let mut diagnostics = Vec::new();
        let error = apply_with_writer(
            &onset_drift,
            &path,
            CoverageLockMode::Check,
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("normalization digest changed"), "{error}");
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(
            diagnostics.contains(&format!(
                "context onset changed\t{changed_id}\tcard \"Fixture\"\told consonant\tnew vowel"
            )),
            "{diagnostics}"
        );
        assert_eq!(fs::read(&path).unwrap(), text_blessed);
    }

    fn lock_delta_fixture() -> (Vec<u8>, CoverageReport) {
        let baseline_report = report(&id('1'), vec![id('a'), id('b'), id('d')], 0, 0, 0, 0);
        let baseline = json_v4(&baseline_report, vec![id('a'), id('b'), id('d')]);
        // `a` is still in the corpus but no longer parses, and its face makes the
        // corpus label differ from the name the lock recorded; `d` left the corpus
        // entirely, so only the lock can name it; `c` is the gain.
        let changed = CoverageReport::for_lock_delta_test(
            id('2'),
            &[(id('b'), None), (id('c'), None)],
            &[(id('a'), Some("Alternate Face"))],
        );
        (baseline, changed)
    }

    #[test]
    fn report_policy_prints_drop_and_gain_delta_and_blesses_without_retirement() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let (baseline, changed) = lock_delta_fixture();
        fs::write(&path, &baseline).unwrap();

        let mut diagnostics = Vec::new();
        apply_with_writer_and_retirement(
            &changed,
            &path,
            CoverageLockRequest {
                mode: CoverageLockMode::Check,
                lock_policy: CoverageLockPolicy::Report,
                retirement_path: None,
            },
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
            &mut |_, _| panic!("report mode must not authenticate retirement"),
        )
        .unwrap();
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(
            diagnostics.contains("no longer covered 2 corpus identities"),
            "{diagnostics}"
        );
        assert!(
            diagnostics.contains(&format!(
                "no longer covered\t{}\tcard \"Card aaaaaaaa (Alternate Face)\"",
                id('a')
            )),
            "{diagnostics}"
        );
        assert!(
            diagnostics.contains(&format!(
                "no longer covered\t{}\tcard \"Card dddddddd\"",
                id('d')
            )),
            "{diagnostics}"
        );
        assert!(
            diagnostics.contains("newly covered 1 corpus identity"),
            "{diagnostics}"
        );
        assert!(
            diagnostics.contains(&format!(
                "newly covered\t{}\tcard \"Card cccccccc\"\tselected_analysis \"analysis cccccccc\"",
                id('c')
            )),
            "{diagnostics}"
        );
        assert_eq!(fs::read(&path).unwrap(), baseline);

        let error = apply_with_writer(
            &changed,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(
            error.contains("lost 2 previously covered corpus identities"),
            "{error}"
        );

        apply_with_writer_and_retirement(
            &changed,
            &path,
            CoverageLockRequest {
                mode: CoverageLockMode::Bless,
                lock_policy: CoverageLockPolicy::Report,
                retirement_path: None,
            },
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
            &mut |_, _| panic!("report mode must not authenticate retirement"),
        )
        .unwrap();
        assert_eq!(
            read_lock(&path).unwrap().covered_for_test(),
            &[id('b'), id('c')]
        );
    }

    #[test]
    fn report_policy_rejects_a_retirement_manifest() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let retirement = directory.path().join("retired.ids");
        let (baseline, changed) = lock_delta_fixture();
        fs::write(&path, &baseline).unwrap();
        fs::write(&retirement, format!("{}\n", id('a'))).unwrap();

        let error = apply_with_writer_and_retirement(
            &changed,
            &path,
            CoverageLockRequest {
                mode: CoverageLockMode::Bless,
                lock_policy: CoverageLockPolicy::Report,
                retirement_path: Some(&retirement),
            },
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
            &mut |_, _| panic!("report mode must not authenticate retirement"),
        )
        .unwrap_err()
        .to_string();
        assert!(
            error.contains("retirement manifests are unavailable in report mode"),
            "{error}"
        );
    }

    #[test]
    fn report_policy_reports_a_schema_three_lock_instead_of_demanding_migration() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let changed = CoverageReport::for_lock_delta_test(
            id('2'),
            &[(id('b'), None), (id('c'), None)],
            &[(id('a'), Some("Alternate Face"))],
        );
        fs::write(
            &path,
            json_v3(&id('1'), &id('9'), &[id('a'), id('b'), id('d')]),
        )
        .unwrap();

        let mut diagnostics = Vec::new();
        apply_with_writer_and_retirement(
            &changed,
            &path,
            CoverageLockRequest {
                mode: CoverageLockMode::Check,
                lock_policy: CoverageLockPolicy::Report,
                retirement_path: None,
            },
            &mut diagnostics,
            &mut super::FilesystemLockWriter,
            &mut |_, _| panic!("report mode must not authenticate retirement"),
        )
        .unwrap();
        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(
            diagnostics.contains(&format!(
                "no longer covered\t{}\tcard \"Card aaaaaaaa (Alternate Face)\"",
                id('a')
            )),
            "{diagnostics}"
        );
        // A schema-3 lock records no per-unit names, so an identity the corpus has
        // also dropped can only be listed by its identity.
        assert!(
            diagnostics.contains(&format!(
                "no longer covered\t{}\tcard \"<unknown>\"",
                id('d')
            )),
            "{diagnostics}"
        );

        let error = apply_with_writer(
            &changed,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("uses schema 3"), "{error}");

        apply_with_writer_and_retirement(
            &changed,
            &path,
            CoverageLockRequest {
                mode: CoverageLockMode::Bless,
                lock_policy: CoverageLockPolicy::Report,
                retirement_path: None,
            },
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
            &mut |_, _| panic!("report mode must not authenticate retirement"),
        )
        .unwrap();
        assert_eq!(
            read_lock(&path).unwrap().covered_for_test(),
            &[id('b'), id('c')]
        );
    }

    #[test]
    fn schema_four_check_rejects_a_deleted_normalization_unit_vector() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let current = report(&id('1'), vec![id('a')], 0, 0, 0, 0);
        let mut baseline: serde_json::Value =
            serde_json::from_slice(&json_v4(&current, vec![id('a')])).unwrap();
        baseline["normalization_units"] = serde_json::json!([]);
        let baseline = serde_json::to_vec_pretty(&baseline).unwrap();
        fs::write(&path, &baseline).unwrap();

        let error = apply_with_writer(
            &current,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();

        assert!(
            error.contains("normalization unit records differ"),
            "{error}"
        );
        assert!(error.contains("first differing unit at index 0"), "{error}");
        assert!(error.contains(&id('a')), "{error}");
        assert!(error.contains("--bless"), "{error}");
        assert_eq!(fs::read(&path).unwrap(), baseline);
    }

    #[test]
    fn schema_four_check_rejects_each_falsified_normalization_unit_field() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let current = report(&id('1'), vec![id('a')], 0, 0, 0, 0);
        let valid: serde_json::Value =
            serde_json::from_slice(&json_v4(&current, vec![id('a')])).unwrap();

        for (field, falsified) in [
            ("card_name", serde_json::json!("Falsified")),
            ("context_onset", serde_json::json!("Vowel")),
        ] {
            let mut baseline = valid.clone();
            baseline["normalization_units"][0][field] = falsified;
            let baseline = serde_json::to_vec_pretty(&baseline).unwrap();
            fs::write(&path, &baseline).unwrap();

            let error = apply_with_writer(
                &current,
                &path,
                CoverageLockMode::Check,
                &mut Vec::new(),
                &mut super::FilesystemLockWriter,
            )
            .unwrap_err()
            .to_string();

            assert!(
                error.contains("normalization unit records differ"),
                "{error}"
            );
            assert!(error.contains("first differing unit at index 0"), "{error}");
            assert!(error.contains(&id('a')), "{error}");
            assert!(error.contains("--bless"), "{error}");
            assert_eq!(fs::read(&path).unwrap(), baseline);
        }
    }

    #[test]
    fn schema_four_bless_accepts_only_an_exact_one_time_retirement_manifest() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let retirement = directory.path().join("retired.ids");
        let current = report(&id('1'), vec![id('b'), id('c')], 0, 3, 0, 0);
        let baseline = json_v4(&current, vec![id('a'), id('b')]);

        fs::write(&path, &baseline).unwrap();
        fs::write(&retirement, format!("{}\n", id('b'))).unwrap();
        let error = apply_with_test_retirement(
            &current,
            &path,
            CoverageLockMode::Bless,
            Some(&retirement),
            &mut Vec::new(),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("exactly match"), "{error}");
        assert_eq!(fs::read(&path).unwrap(), baseline);

        fs::write(&retirement, format!("{}\n", id('a'))).unwrap();
        let mut diagnostics = Vec::new();
        apply_with_test_retirement(
            &current,
            &path,
            CoverageLockMode::Bless,
            Some(&retirement),
            &mut diagnostics,
        )
        .unwrap();
        assert_eq!(
            read_lock(&path).unwrap().covered_for_test(),
            &[id('b'), id('c')]
        );
        assert_eq!(
            String::from_utf8(diagnostics).unwrap(),
            "newly covered 1 corpus identity\n\
newly covered\tcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\n\
retired 1 previously covered corpus identity\n\
WARNING: coverage retirement is a coordinator ruling; a ticket-vs-purpose contradiction requires a STOP, and the landing record must contain a re-coverage or retirement obligation line\n"
        );

        apply_with_test_retirement(
            &current,
            &path,
            CoverageLockMode::Check,
            None,
            &mut Vec::new(),
        )
        .unwrap();

        let next_loss = report(&id('1'), vec![id('c')], 0, 4, 0, 0);
        let migrated = fs::read(&path).unwrap();
        let error = apply_with_test_retirement(
            &next_loss,
            &path,
            CoverageLockMode::Bless,
            Some(&retirement),
            &mut Vec::new(),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("exactly match"), "{error}");
        assert_eq!(fs::read(&path).unwrap(), migrated);
    }

    #[test]
    fn tracked_retirement_manifest_is_refused_before_the_lock_write() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let retirement = directory.path().join("retired.ids");
        let ticket = directory.path().join("docs/tickets/wip/fixture.md");
        fs::create_dir_all(ticket.parent().unwrap()).unwrap();
        let current = report(&id('1'), vec![id('b')], 0, 0, 0, 0);
        let baseline = json_v4(&current, vec![id('a'), id('b')]);
        fs::write(&path, &baseline).unwrap();
        fs::write(&retirement, format!("{}\n", id('a'))).unwrap();
        fs::write(
            &ticket,
            format!(
                "## Landing record\n\n- Retirement/re-coverage obligation: {}\n",
                id('a')
            ),
        )
        .unwrap();
        let tracked = vec![retirement.clone(), ticket.clone()];
        let ticket_source = fs::read_to_string(&ticket).unwrap();
        let mut authenticate = |manifest: &Path, retired: &[String]| {
            validate_retirement_authentication(
                manifest,
                &manifest.canonicalize().unwrap(),
                &tracked,
                &ticket,
                &ticket_source,
                retired,
            )
        };

        let error = apply_with_writer_and_retirement(
            &current,
            &path,
            CoverageLockRequest {
                mode: CoverageLockMode::Bless,
                lock_policy: CoverageLockPolicy::Ratchet,
                retirement_path: Some(&retirement),
            },
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
            &mut authenticate,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("must be untracked"), "{error}");
        assert!(
            error.contains("jj file list reports it as tracked"),
            "{error}"
        );
        assert_eq!(fs::read(&path).unwrap(), baseline);
    }

    #[test]
    fn retirement_obligation_must_name_every_lost_identity_on_one_line() {
        let directory = tempfile::tempdir().unwrap();
        let manifest = directory.path().join("retired.ids");
        let ticket = directory.path().join("docs/tickets/wip/fixture.md");
        fs::create_dir_all(ticket.parent().unwrap()).unwrap();
        fs::write(&manifest, format!("{}\n{}\n", id('a'), id('b'))).unwrap();
        fs::write(&ticket, "ticket fixture\n").unwrap();
        let tracked = vec![ticket.clone()];
        let retired = vec![id('a'), id('b')];

        let error = validate_retirement_authentication(
            &manifest,
            &manifest.canonicalize().unwrap(),
            &tracked,
            &ticket,
            "## Landing record\n- Retirement/re-coverage obligation: a later ticket\n",
            &retired,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("names every retired identity"), "{error}");

        validate_retirement_authentication(
            &manifest,
            &manifest.canonicalize().unwrap(),
            &tracked,
            &ticket,
            &format!(
                "## Landing record\n- Re-coverage/retirement obligation: {} {}\n",
                id('a'),
                id('b')
            ),
            &retired,
        )
        .unwrap();
    }

    #[test]
    fn missing_lock_is_check_error_but_bless_creates_schema_four() {
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
            LoadedCoverageLock::V4(_)
        ));
    }

    #[test]
    fn gate_rejects_uncovered_unresolved_and_internal_but_ignores_ordinary_parse_failures() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let ordinary_failures = report(&id('1'), vec![id('a')], 0, 99, 0, 0);
        let baseline = json_v4(&ordinary_failures, vec![id('a')]);
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
            &ordinary_failures,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap();

        let visible_collision =
            CoverageReport::for_collision_metric_test(id('1'), vec![id('a')], 9, 10);
        fs::write(&path, json_v4(&visible_collision, vec![id('a')])).unwrap();
        apply_with_writer(
            &visible_collision,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .expect("collision census is visible but not ratcheted");

        let decision = deckmaste_english_v2::parser::exception_decision_for_test();
        let exception = CoverageReport::for_exception_gate_test(id('1'), vec![id('a')], &decision);
        assert_eq!(exception.exception_counts_for_test(), (1, 1));
        let error = apply_with_writer(
            &exception,
            &path,
            CoverageLockMode::Check,
            &mut Vec::new(),
            &mut super::FilesystemLockWriter,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("1 exception-resolved unit"), "{error}");
        assert!(error.contains("1 exception use"), "{error}");
    }

    #[test]
    fn every_atomic_write_failure_preserves_original_bytes_and_leaves_no_temporary() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let baseline = json_v3(&id('1'), &id('9'), &[id('a')]);
        let replacement_report = report(&id('2'), vec![id('a'), id('b')], 0, 0, 0, 0);
        let replacement = CoverageLockV4::new(&replacement_report, vec![id('a'), id('b')]).unwrap();

        for stage in [
            FailureStage::Create,
            FailureStage::Write,
            FailureStage::Flush,
            FailureStage::Persist,
        ] {
            fs::write(&path, &baseline).unwrap();
            let mut writer = super::FailingLockWriter::new(stage);
            let error = write_v4_with(&replacement, &path, &mut writer)
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
    fn production_lock_passes_intrinsic_schema_and_identity_validation() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../english-v2-coverage.lock");
        let loaded = read_lock(&path).expect("production coverage lock is strict schema 4");
        assert!(matches!(loaded, LoadedCoverageLock::V4(_)));
    }
}
