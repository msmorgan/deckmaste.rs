use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::Context;
use anyhow::ensure;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_data::mtgjson::AtomicCards;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::Expectation;
use deckmaste_english_v2::parser::ParseError;
use rayon::prelude::*;
use sha2::Digest;
use sha2::Sha256;

const ID_DOMAIN: &[u8] = b"deckmaste:english-v2:corpus-unit:v1";
const SOURCE_ID_DOMAIN: &[u8] = b"deckmaste:english-v2:source-unit:v1";
const NORMALIZATION_DIGEST_DOMAIN: &[u8] = b"deckmaste:english-v2:normalization:v2";
const LEGACY_NORMALIZATION_DIGEST_DOMAIN: &[u8] = b"deckmaste:english-v2:normalization:v1";
const CORPUS_PATCH_LEDGER: &str = include_str!("corpus_patches.ron");
// Governed by docs/decisions/english-v2-rewrite.md, "Ruling: corpus timing
// ceiling and acceptance-cost telemetry" (2026-09-02).
const CORPUS_WALL_CEILING_SECONDS: f64 = 16.26;
const RULES_BEARING_PARENTHETICALS: &[&str] = &[
    "(as long as this creature is on the battlefield)",
    "(even if this card isn't on the battlefield)",
    "(front face up)",
    "(if it's still on the battlefield)",
    "(or {1})",
];
const REMINDER_FOLLOWED_BY_TEXT_PARENTHETICALS: &[&str] = &[
    "(For example, you may change \"black creatures can't attack\" to \"blue creatures can't attack.\")",
    "(a ticket counter)",
    "(an energy counter)",
    "(energy counter)",
    "(energy counters)",
    "(four energy counters)",
    "(the Fridge)",
    "(three energy counters)",
    "(two energy counters)",
];

/// Snapshot provenance for the two parenthetical classes used by normalization.
/// The classification remains the invariant; these attestation counts are not
/// acceptance targets.
#[derive(Debug, serde::Serialize)]
pub(super) struct ParentheticalInventory {
    pub(super) rules_bearing: Vec<ParentheticalOccurrence>,
    pub(super) reminder_followed_by_text: Vec<ParentheticalOccurrence>,
}

#[derive(Debug, serde::Serialize)]
pub(super) struct ParentheticalOccurrence {
    pub(super) surface: String,
    pub(super) occurrences: usize,
}

pub(super) fn parenthetical_inventory() -> anyhow::Result<ParentheticalInventory> {
    let bytes = deckmaste_data::mtgjson::atomic_cards_bytes()
        .context("reading AtomicCards snapshot for parenthetical provenance")?;
    let cards = AtomicCards::parse(&bytes)
        .context("parsing AtomicCards snapshot for parenthetical provenance")?;
    Ok(parenthetical_inventory_from_texts(
        cards
            .data
            .values()
            .flatten()
            .filter(|card| card.vintage_playable())
            .filter_map(|card| card.text.as_deref()),
    ))
}

fn parenthetical_inventory_from_texts<'a>(
    texts: impl IntoIterator<Item = &'a str>,
) -> ParentheticalInventory {
    let texts = texts.into_iter().collect::<Vec<_>>();
    let count = |surface: &str| texts.iter().filter(|text| text.contains(surface)).count();
    let rows = |surfaces: &[&str]| {
        surfaces
            .iter()
            .map(|surface| ParentheticalOccurrence {
                surface: (*surface).to_owned(),
                occurrences: count(surface),
            })
            .collect()
    };
    ParentheticalInventory {
        rules_bearing: rows(RULES_BEARING_PARENTHETICALS),
        reminder_followed_by_text: rows(REMINDER_FOLLOWED_BY_TEXT_PARENTHETICALS),
    }
}

/// Quarantines corrections to irregular source-of-truth corpus text as
/// reviewed data. A patch corrects raw `AtomicCards` text before structural
/// normalization; grammatical Oracle English that the construction grammar
/// does not yet analyze instead remains `CoverageStatus::SelectedUncovered`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusPatchLedger {
    patches: Vec<CorpusTextPatch>,
}

impl CorpusPatchLedger {
    fn embedded() -> anyhow::Result<Self> {
        Self::parse(CORPUS_PATCH_LEDGER).context("loading English-v2 corpus patch ledger")
    }

    fn parse(source: &str) -> anyhow::Result<Self> {
        let ledger = ron::from_str::<Self>(source).context("parsing corpus patch ledger RON")?;
        ledger.validate()?;
        Ok(ledger)
    }

    fn validate(&self) -> anyhow::Result<()> {
        for (index, patch) in self.patches.iter().enumerate() {
            ensure!(
                !patch.reason.trim().is_empty(),
                "corpus patch {index} has an empty reason"
            );
            ensure!(
                !patch.card_or_face_names.is_empty(),
                "corpus patch {index} has no card or face names"
            );
            ensure!(
                patch
                    .card_or_face_names
                    .iter()
                    .all(|name| !name.trim().is_empty()),
                "corpus patch {index} has an empty card or face name"
            );
            ensure!(
                !patch.find.is_empty(),
                "corpus patch {index} has empty find text"
            );
            ensure!(
                patch.find != patch.replace,
                "corpus patch {index} has identical find and replace text"
            );
        }
        Ok(())
    }

    fn unused_marks(&self) -> Vec<bool> {
        vec![false; self.patches.len()]
    }

    fn apply(
        &self,
        card_name: &str,
        face_name: Option<&str>,
        source_text: &str,
        used: &mut [bool],
    ) -> anyhow::Result<String> {
        let mut text = source_text.to_owned();
        for (patch, used) in self
            .patches
            .iter()
            .zip(used)
            .filter(|(patch, _)| patch.applies_to(card_name, face_name))
        {
            let occurrences = text.matches(&patch.find).count();
            ensure!(
                occurrences == 1,
                "corpus patch for card {card_name:?}, face {face_name:?} expected find text {:?} exactly once, found {occurrences}; reason: {}",
                patch.find,
                patch.reason,
            );
            text = text.replacen(&patch.find, &patch.replace, 1);
            *used = true;
        }
        Ok(text)
    }

    /// A patch naming no corpus unit is a stale ledger entry, so it fails the
    /// load rather than passing as a silent no-op.
    fn ensure_every_patch_used(&self, used: &[bool]) -> anyhow::Result<()> {
        for (index, patch) in self.patches.iter().enumerate() {
            ensure!(
                used[index],
                "corpus patch {index} matched no corpus unit; card or face names {:?}; reason: {}",
                patch.card_or_face_names,
                patch.reason,
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpusTextPatch {
    reason: String,
    card_or_face_names: Vec<String>,
    find: String,
    replace: String,
}

impl CorpusTextPatch {
    fn applies_to(&self, card_name: &str, face_name: Option<&str>) -> bool {
        self.card_or_face_names
            .iter()
            .any(|name| name == card_name || face_name.is_some_and(|face| name == face))
    }
}

pub(super) fn corpus_error_message(error: &ParseError) -> String {
    const MAX_EXPECTATIONS: usize = 8;
    let ParseError::Failure { span, expectations } = error else {
        return error.to_string();
    };
    let mut ordered = Vec::new();
    for rank in (1..=3).rev() {
        for expectation in expectations
            .iter()
            .filter(|expectation| corpus_expectation_rank(expectation) == rank)
        {
            ordered.push(expectation.to_string());
        }
    }
    let (ranked, omitted) = bounded_unique_names(ordered, MAX_EXPECTATIONS);
    let mut message = format!(
        "parse failed at bytes {}..{}; expected ",
        span.start, span.end
    );
    for (index, name) in ranked.into_iter().take(MAX_EXPECTATIONS).enumerate() {
        if index > 0 {
            message.push_str(", ");
        }
        message.push_str(&name);
    }
    if omitted > 0 {
        write!(message, ", and {omitted} more").expect("writing to String cannot fail");
    }
    message
}

fn bounded_unique_names(
    names: impl IntoIterator<Item = String>,
    limit: usize,
) -> (Vec<String>, usize) {
    let mut unique = Vec::new();
    for name in names {
        if !unique.contains(&name) {
            unique.push(name);
        }
    }
    let omitted = unique.len().saturating_sub(limit);
    unique.truncate(limit);
    (unique, omitted)
}

const fn corpus_expectation_rank(expectation: &Expectation) -> u8 {
    match expectation {
        Expectation::Literal(_) => 3,
        Expectation::Terminal(_) => 2,
        Expectation::Nonterminal(_) => 1,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(super) struct CorpusUnit {
    source_id: String,
    id: String,
    card_name: String,
    face_name: Option<String>,
    side: Option<String>,
    context_name: String,
    is_legendary: bool,
    context_onset: Onset,
    text: String,
}

impl CorpusUnit {
    pub(super) fn source_id(&self) -> &str {
        &self.source_id
    }

    pub(super) fn id(&self) -> &str {
        &self.id
    }

    pub(super) fn card_name(&self) -> &str {
        &self.card_name
    }

    pub(super) fn face_name(&self) -> Option<&str> {
        self.face_name.as_deref()
    }

    pub(super) fn side(&self) -> Option<&str> {
        self.side.as_deref()
    }

    pub(super) fn context_name(&self) -> &str {
        &self.context_name
    }

    pub(super) const fn is_legendary(&self) -> bool {
        self.is_legendary
    }

    pub(super) const fn context_onset(&self) -> Onset {
        self.context_onset
    }

    pub(super) fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ValidatedCorpusId(String);

impl ValidatedCorpusId {
    pub(super) fn parse(id: &str) -> anyhow::Result<Self> {
        ensure!(
            id.len() == 64
                && id
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
            "invalid corpus ID {}; expected exactly 64 lowercase hexadecimal bytes",
            quoted(id),
        );
        Ok(Self(id.to_owned()))
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Corpus {
    source_fingerprint: String,
    units: Vec<CorpusUnit>,
}

impl Corpus {
    pub(super) fn from_bytes_with_context_onsets(
        bytes: &[u8],
        context_onsets: &BTreeMap<String, Onset>,
    ) -> anyhow::Result<Self> {
        let patches = CorpusPatchLedger::embedded()?;
        Self::from_bytes_with_context_onsets_and_patches(bytes, context_onsets, &patches)
    }

    fn from_bytes_with_context_onsets_and_patches(
        bytes: &[u8],
        context_onsets: &BTreeMap<String, Onset>,
        patches: &CorpusPatchLedger,
    ) -> anyhow::Result<Self> {
        let cards = AtomicCards::parse(bytes).context("parsing MTGJSON atomic-card snapshot")?;
        let mut used_patches = patches.unused_marks();
        let mut units = cards
            .data
            .values()
            .flat_map(|cards| cards.iter())
            .filter(|card| card.vintage_playable())
            .map(|card| -> anyhow::Result<_> {
                let card_name = card.name.to_string();
                let face_name = card.face_name.as_deref().map(str::to_owned);
                let side = card.side.as_deref().map(str::to_owned);
                let context_name = face_name.clone().unwrap_or_else(|| card_name.clone());
                let is_legendary = card
                    .supertypes
                    .iter()
                    .any(|supertype| supertype.as_str() == "Legendary");
                ensure!(
                    !context_name.is_empty(),
                    "invalid parser context: context name is empty; card name {card_name:?}, face name {face_name:?}, side {side:?}",
                );
                let context_onset = context_onsets.get(&context_name).copied().with_context(|| {
                    format!(
                        "missing explicit card-name onset metadata for opaque context {context_name:?}"
                    )
                })?;
                let source_text = card.text.as_deref().unwrap_or_default();
                let patched_text = patches.apply(
                    &card_name,
                    face_name.as_deref(),
                    source_text,
                    &mut used_patches,
                )?;
                let text = normalize_oracle_text(&card_name, &patched_text)?;
                Ok(corpus_unit(
                    &card_name,
                    face_name.as_deref(),
                    side.as_deref(),
                    &context_name,
                    is_legendary,
                    context_onset,
                    source_text,
                    &text,
                ))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        patches.ensure_every_patch_used(&used_patches)?;
        units.sort_by(|left, right| corpus_sort_key(left).cmp(&corpus_sort_key(right)));
        validate_contexts(&units)?;

        Ok(Self {
            source_fingerprint: sha256_hex(&Sha256::digest(bytes)),
            units,
        })
    }

    pub(super) fn load(path: &Path) -> anyhow::Result<Self> {
        let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
        let catalog_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        let adapted = super::adapt_card_name_catalog_provider(&catalog_root)?;
        Self::from_bytes_with_context_onsets(&bytes, &adapted.context_onsets)
    }

    pub(super) fn source_fingerprint(&self) -> &str {
        &self.source_fingerprint
    }

    pub(super) fn normalization_digest(&self) -> String {
        normalization_digest(
            self.units
                .iter()
                .map(|unit| (unit.text(), unit.context_onset())),
        )
    }

    pub(super) fn units(&self) -> &[CorpusUnit] {
        &self.units
    }

    pub(super) fn resolve_exact(&self, id: &ValidatedCorpusId) -> anyhow::Result<&CorpusUnit> {
        let mut matches = self.units.iter().filter(|unit| unit.id() == id.as_str());
        let Some(unit) = matches.next() else {
            anyhow::bail!("no corpus unit has exact ID {}", quoted(id.as_str()));
        };
        ensure!(
            matches.next().is_none(),
            "multiple corpus units have exact ID {}",
            quoted(id.as_str()),
        );
        Ok(unit)
    }
}

pub(super) fn normalization_digest<'a>(
    inputs: impl IntoIterator<Item = (&'a str, Onset)>,
) -> String {
    let mut digest = Sha256::new();
    digest.update(NORMALIZATION_DIGEST_DOMAIN);
    for (text, onset) in inputs {
        let length = u64::try_from(text.len()).expect("normalized text length fits in u64");
        digest.update(length.to_be_bytes());
        digest.update(text.as_bytes());
        digest.update([match onset {
            Onset::Consonant => 0,
            Onset::Vowel => 1,
        }]);
    }
    sha256_hex(&digest.finalize())
}

pub(super) fn legacy_normalization_digest<'a>(texts: impl IntoIterator<Item = &'a str>) -> String {
    let mut digest = Sha256::new();
    digest.update(LEGACY_NORMALIZATION_DIGEST_DOMAIN);
    for text in texts {
        let length = u64::try_from(text.len()).expect("normalized text length fits in u64");
        digest.update(length.to_be_bytes());
        digest.update(text.as_bytes());
    }
    sha256_hex(&digest.finalize())
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct CorpusPerformance {
    workers: usize,
    units: usize,
    bytes: usize,
    cpu_nanos: u128,
}

impl CorpusPerformance {
    fn record<T>(
        &mut self,
        unit: &CorpusUnit,
        elapsed: Duration,
        value: &T,
        accepted: &impl Fn(&T) -> bool,
    ) {
        if accepted(value) {
            self.units += 1;
            self.bytes += unit.text().len();
            self.cpu_nanos += elapsed.as_nanos();
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "corpus-scale timing ratios do not need integer precision beyond f64"
    )]
    pub(super) fn accepted_cpu_micros_per_byte(self) -> f64 {
        if self.bytes == 0 {
            return 0.0;
        }
        self.cpu_nanos as f64 / 1_000.0 / self.bytes as f64
    }

    pub(super) fn for_unit_bytes(bytes: usize, cpu_elapsed: Duration, accepted: bool) -> Self {
        Self {
            workers: 1,
            units: usize::from(accepted),
            bytes: bytes * usize::from(accepted),
            cpu_nanos: cpu_elapsed.as_nanos() * u128::from(accepted),
        }
    }
}

pub(super) fn write_corpus_performance(
    gate: &str,
    elapsed: Duration,
    performance: CorpusPerformance,
) -> anyhow::Result<()> {
    let mut diagnostics = std::io::stderr().lock();
    write_corpus_performance_to(
        gate,
        elapsed,
        performance,
        std::thread::available_parallelism().map_or(1, usize::from),
        &mut diagnostics,
    )
}

fn write_corpus_performance_to(
    gate: &str,
    elapsed: Duration,
    performance: CorpusPerformance,
    available_workers: usize,
    diagnostics: &mut dyn std::io::Write,
) -> anyhow::Result<()> {
    let load = host_load_average();
    let capped_workers = performance.workers < available_workers;
    let criterion = if capped_workers { "capped_workers" } else { "quiet_host" };
    writeln!(
        diagnostics,
        "PERFORMANCE english-v2 gate={gate} workers={} elapsed_seconds={:.9} accepted_cpu_micros_per_byte={:.3} accepted_units={} accepted_bytes={} ceiling_seconds={CORPUS_WALL_CEILING_SECONDS:.3} criterion={criterion} host_load_1m={} host_load_5m={} host_load_15m={}",
        performance.workers,
        elapsed.as_secs_f64(),
        performance.accepted_cpu_micros_per_byte(),
        performance.units,
        performance.bytes,
        load.map_or_else(
            || "unavailable".to_owned(),
            |value| format!("{:.2}", value[0])
        ),
        load.map_or_else(
            || "unavailable".to_owned(),
            |value| format!("{:.2}", value[1])
        ),
        load.map_or_else(
            || "unavailable".to_owned(),
            |value| format!("{:.2}", value[2])
        ),
    )?;
    if elapsed.as_secs_f64() > CORPUS_WALL_CEILING_SECONDS && !capped_workers {
        writeln!(
            diagnostics,
            "WARNING english-v2-common-path-performance-regression gate={gate} workers={} elapsed_seconds={:.9} ceiling_seconds={CORPUS_WALL_CEILING_SECONDS:.3} criterion=quiet_host host_load_1m={} host_load_5m={} host_load_15m={}",
            performance.workers,
            elapsed.as_secs_f64(),
            load.map_or_else(
                || "unavailable".to_owned(),
                |value| format!("{:.2}", value[0])
            ),
            load.map_or_else(
                || "unavailable".to_owned(),
                |value| format!("{:.2}", value[1])
            ),
            load.map_or_else(
                || "unavailable".to_owned(),
                |value| format!("{:.2}", value[2])
            ),
        )?;
    }
    #[cfg(feature = "parser-metrics")]
    write_parser_metrics(&mut diagnostics)?;
    Ok(())
}

fn host_load_average() -> Option<[f64; 3]> {
    let mut load = [0.0; 3];
    // SAFETY: `load` provides space for the three samples requested from the
    // POSIX `getloadavg` API, and remains live for the duration of the call.
    let samples = unsafe { libc::getloadavg(load.as_mut_ptr(), load.len().try_into().ok()?) };
    (samples == 3).then_some(load)
}

pub(super) fn thread_cpu_time() -> Duration {
    let mut sample = std::mem::MaybeUninit::<libc::timespec>::uninit();
    // SAFETY: `sample` is a valid out pointer and `CLOCK_THREAD_CPUTIME_ID`
    // asks the POSIX clock for the calling worker thread, not process wall time.
    let status = unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, sample.as_mut_ptr()) };
    assert_eq!(status, 0, "CLOCK_THREAD_CPUTIME_ID must be available");
    // SAFETY: successful `clock_gettime` initialized the complete timespec.
    let sample = unsafe { sample.assume_init() };
    Duration::new(
        sample
            .tv_sec
            .try_into()
            .expect("thread CPU seconds are nonnegative"),
        sample
            .tv_nsec
            .try_into()
            .expect("thread CPU nanoseconds fit in u32"),
    )
}

#[cfg(feature = "parser-metrics")]
fn write_parser_metrics(output: &mut dyn std::io::Write) -> anyhow::Result<()> {
    let mut rows = deckmaste_english_v2::parser::parser_metrics();
    let work = deckmaste_english_v2::parser::parser_work_metrics();
    let score = |row: &deckmaste_english_v2::parser::ConstructionMetrics| {
        row.predictions()
            + row.completions()
            + row.materializations()
            + row.memo_misses()
            + row.clone_heavy()
    };
    rows.sort_by(|left, right| {
        score(right)
            .cmp(&score(left))
            .then_with(|| left.name().cmp(right.name()))
    });
    let totals = rows.iter().fold([0_u64; 5], |mut totals, row| {
        totals[0] += row.predictions();
        totals[1] += row.completions();
        totals[2] += row.materializations();
        totals[3] += row.memo_misses();
        totals[4] += row.clone_heavy();
        totals
    });
    writeln!(
        output,
        "PARSER_METRICS totals predictions={} completions={} materializations={} memo_misses={} clone_heavy={}",
        totals[0], totals[1], totals[2], totals[3], totals[4],
    )?;
    writeln!(
        output,
        "PARSER_METRICS work chart_columns_visited={} predictions_per_column={:.3} scan_attempts={}",
        work.chart_columns_visited(),
        work.predictions_per_column(totals[0]),
        work.scan_attempts(),
    )?;
    for (rank, row) in rows
        .into_iter()
        .filter(|row| score(row) > 0)
        .take(20)
        .enumerate()
    {
        writeln!(
            output,
            "PARSER_METRICS rank={} construction={} predictions={} completions={} materializations={} memo_misses={} clone_heavy={}",
            rank + 1,
            row.name(),
            row.predictions(),
            row.completions(),
            row.materializations(),
            row.memo_misses(),
            row.clone_heavy(),
        )?;
    }
    Ok(())
}

pub(super) fn map_corpus_units<T: Send>(
    units: &[CorpusUnit],
    requested_workers: usize,
    map: impl Fn(usize, &CorpusUnit) -> T + Send + Sync,
    accepted: impl Fn(&T) -> bool + Send + Sync,
) -> (Vec<T>, CorpusPerformance) {
    map_corpus_units_with_workers(units, requested_workers, map, accepted)
}

fn map_corpus_units_with_workers<T: Send>(
    units: &[CorpusUnit],
    requested_workers: usize,
    map: impl Fn(usize, &CorpusUnit) -> T + Send + Sync,
    accepted: impl Fn(&T) -> bool + Send + Sync,
) -> (Vec<T>, CorpusPerformance) {
    if units.is_empty() {
        return (Vec::new(), CorpusPerformance::default());
    }
    let jobs = corpus_unit_jobs(requested_workers, units.len());
    let mut scheduled = units.iter().enumerate().collect::<Vec<_>>();
    scheduled.sort_by(|(left_index, left), (right_index, right)| {
        right
            .text()
            .len()
            .cmp(&left.text().len())
            .then_with(|| left_index.cmp(right_index))
    });
    let run = |(index, unit): &(usize, &CorpusUnit)| {
        let started = thread_cpu_time();
        let value = map(*index, unit);
        let elapsed = thread_cpu_time()
            .checked_sub(started)
            .expect("thread CPU clock is monotonic");
        (*index, elapsed, value)
    };
    let mut completed = if jobs == 1 {
        scheduled.iter().map(run).collect::<Vec<_>>()
    } else {
        rayon::ThreadPoolBuilder::new()
            .num_threads(jobs)
            .thread_name(|index| format!("english-v2-corpus-{index}"))
            .build()
            .expect("bounded English-v2 corpus pool must build")
            .install(|| scheduled.par_iter().map(run).collect::<Vec<_>>())
    };
    completed.sort_by_key(|(index, _, _)| *index);
    let mut performance = CorpusPerformance {
        workers: jobs,
        ..CorpusPerformance::default()
    };
    let values = completed
        .into_iter()
        .map(|(index, elapsed, value)| {
            performance.record(&units[index], elapsed, &value, &accepted);
            value
        })
        .collect();
    (values, performance)
}

fn corpus_unit_jobs(requested_workers: usize, units: usize) -> usize {
    requested_workers.min(units).max(1)
}

fn quoted(value: &str) -> String {
    serde_json::to_string(value).expect("serializing a string cannot fail")
}

fn validate_contexts(units: &[CorpusUnit]) -> anyhow::Result<()> {
    for unit in units {
        if ParseContext::new(
            unit.context_name(),
            unit.is_legendary(),
            unit.context_onset(),
        )
        .is_some()
        {
            continue;
        }
        anyhow::bail!(
            "invalid parser context: context name is empty; card name {:?}, face name {:?}, side {:?}",
            unit.card_name(),
            unit.face_name(),
            unit.side(),
        );
    }
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "a corpus unit identity pins every source and parser-context field"
)]
fn corpus_unit(
    card_name: &str,
    face_name: Option<&str>,
    side: Option<&str>,
    context_name: &str,
    is_legendary: bool,
    context_onset: Onset,
    source_text: &str,
    text: &str,
) -> CorpusUnit {
    let mut source_hasher = Sha256::new();
    source_hasher.update(SOURCE_ID_DOMAIN);
    for field in [
        card_name,
        face_name.unwrap_or_default(),
        side.unwrap_or_default(),
        context_name,
        source_text,
    ] {
        source_hasher.update((field.len() as u64).to_be_bytes());
        source_hasher.update(field.as_bytes());
    }
    source_hasher.update([u8::from(is_legendary)]);

    let mut hasher = Sha256::new();
    hasher.update(ID_DOMAIN);
    for field in [
        card_name,
        face_name.unwrap_or_default(),
        side.unwrap_or_default(),
        context_name,
        text,
    ] {
        hasher.update((field.len() as u64).to_be_bytes());
        hasher.update(field.as_bytes());
    }

    CorpusUnit {
        source_id: sha256_hex(&source_hasher.finalize()),
        id: sha256_hex(&hasher.finalize()),
        card_name: card_name.to_owned(),
        face_name: face_name.map(str::to_owned),
        side: side.map(str::to_owned),
        context_name: context_name.to_owned(),
        is_legendary,
        context_onset,
        text: text.to_owned(),
    }
}

fn sha256_hex(digest: &[u8]) -> String {
    let mut hexadecimal = String::with_capacity(digest.len() * 2);
    for byte in digest {
        write!(&mut hexadecimal, "{byte:02x}").expect("writing to String cannot fail");
    }
    hexadecimal
}

fn corpus_sort_key(unit: &CorpusUnit) -> (&str, Option<&str>, Option<&str>, &str, &str, &str) {
    (
        &unit.card_name,
        unit.face_name.as_deref(),
        unit.side.as_deref(),
        &unit.context_name,
        &unit.text,
        &unit.id,
    )
}

fn normalize_oracle_text(card_name: &str, text: &str) -> anyhow::Result<String> {
    let typography =
        normalize_roll_row_dashes(&deckmaste_data::academyruins::normalize_quotes(text));
    strip_reminder_text(card_name, &typography)
}

/// Removes nonempty, single-line reminder parentheticals while retaining at
/// most one surrounding space. A line containing only reminder text
/// disappears. Every parenthetical followed by non-whitespace text on its
/// line must be explicitly classified; authored rules-bearing parentheticals
/// survive byte-exactly. An empty line-final parenthetical is not reminder
/// text and passes through byte-exactly.
fn strip_reminder_text(card_name: &str, text: &str) -> anyhow::Result<String> {
    text.split('\n')
        .map(|line| {
            strip_reminder_text_line(card_name, line).map(|stripped| {
                (!stripped.trim().is_empty() || line.is_empty()).then_some(stripped)
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .map(|lines| lines.into_iter().flatten().collect::<Vec<_>>().join("\n"))
}

fn strip_reminder_text_line(card_name: &str, line: &str) -> anyhow::Result<String> {
    assert_no_nested_parentheses(line);

    let mut stripped = String::with_capacity(line.len());
    let mut remainder = line;
    while let Some(open) = remainder.find('(') {
        stripped.push_str(&remainder[..open]);
        let Some(relative_close) = remainder[open + 1..].find(')') else {
            stripped.push_str(&remainder[open..]);
            return Ok(stripped);
        };
        let close = open + 1 + relative_close;
        let parenthetical = &remainder[open..=close];
        let after = &remainder[close + 1..];

        if RULES_BEARING_PARENTHETICALS.contains(&parenthetical) {
            stripped.push_str(parenthetical);
            remainder = after;
            continue;
        }
        let is_followed_by_text = !after.trim().is_empty();
        ensure!(
            !is_followed_by_text
                || REMINDER_FOLLOWED_BY_TEXT_PARENTHETICALS.contains(&parenthetical),
            "unknown parenthetical followed by text {parenthetical:?} while normalizing card {card_name:?}",
        );

        if relative_close == 0 {
            stripped.push_str(parenthetical);
            remainder = after;
            continue;
        }

        if stripped.ends_with(' ') {
            stripped.pop();
        }
        let (after, had_space_after) = after
            .strip_prefix(' ')
            .map_or((after, false), |after| (after, true));
        if !stripped.is_empty() && !after.is_empty() && had_space_after {
            stripped.push(' ');
        }
        remainder = after;
    }
    stripped.push_str(remainder);
    Ok(stripped)
}

fn assert_no_nested_parentheses(line: &str) {
    let mut open = false;
    for character in line.chars() {
        match character {
            '(' => {
                assert!(!open, "nested parenthetical in Oracle text line {line:?}");
                open = true;
            }
            ')' => open = false,
            _ => {}
        }
    }
}

fn normalize_roll_row_dashes(text: &str) -> String {
    text.split('\n')
        .map(normalize_roll_row_line)
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_roll_row_line(line: &str) -> String {
    let low_end = line
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len());
    if low_end == 0 {
        return line.to_owned();
    }
    let Some(dash) = line[low_end..].chars().next() else {
        return line.to_owned();
    };
    if !matches!(dash, '-' | '—') {
        return line.to_owned();
    }
    let high_start = low_end + dash.len_utf8();
    let high_len = line[high_start..]
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(line.len() - high_start);
    if high_len == 0 || !line[high_start + high_len..].starts_with(" |") {
        return line.to_owned();
    }
    format!("{}–{}", &line[..low_end], &line[high_start..])
}

#[cfg(test)]
impl CorpusUnit {
    pub(super) fn for_test(card_name: &str, text: &str) -> Self {
        let context_onset = if card_name.is_empty() {
            // The test-only invalid-context constructor rejects this unit
            // before the realization fact can be observed.
            Onset::Consonant
        } else {
            super::catalog_surface_onset(card_name)
                .expect("test corpus fixture must use a name with known onset")
        };
        corpus_unit(
            card_name,
            None,
            None,
            card_name,
            false,
            context_onset,
            text,
            &normalize_oracle_text(card_name, text)
                .expect("test corpus fixture must satisfy normalization invariants"),
        )
    }

    pub(super) fn for_test_with_metadata(
        card_name: &str,
        face_name: Option<&str>,
        side: Option<&str>,
        context_name: &str,
        text: &str,
    ) -> Self {
        corpus_unit(
            card_name,
            face_name,
            side,
            context_name,
            false,
            super::catalog_surface_onset(context_name)
                .expect("test corpus fixture must use a context name with known onset"),
            text,
            &normalize_oracle_text(card_name, text)
                .expect("test corpus fixture must satisfy normalization invariants"),
        )
    }
}

#[cfg(test)]
impl Corpus {
    pub(super) fn from_units_for_test(mut units: Vec<CorpusUnit>) -> Self {
        units.sort_by(|left, right| corpus_sort_key(left).cmp(&corpus_sort_key(right)));
        validate_contexts(&units).expect("test corpus must contain valid parser contexts");
        Self {
            source_fingerprint: "0".repeat(64),
            units,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::sync::Barrier;
    use std::sync::Mutex;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::sync::mpsc::sync_channel;
    use std::time::Duration;

    use super::*;

    #[test]
    fn omitted_expectation_count_uses_unique_display_names() {
        let names = ["a", "b", "a", "c", "b", "d"]
            .into_iter()
            .map(str::to_owned);
        let (shown, omitted) = bounded_unique_names(names, 2);

        assert_eq!(shown, ["a", "b"]);
        assert_eq!(omitted, 2);
    }

    #[test]
    fn thread_cpu_clock_excludes_sleeping_wall_time() {
        let started = thread_cpu_time();
        std::thread::sleep(Duration::from_millis(20));
        let cpu_elapsed = thread_cpu_time()
            .checked_sub(started)
            .expect("thread CPU clock is monotonic");

        assert!(cpu_elapsed < Duration::from_millis(10), "{cpu_elapsed:?}");
    }

    #[test]
    fn capped_workers_report_the_advisory_criterion_without_a_regression_warning() {
        let performance = CorpusPerformance {
            workers: 2,
            ..CorpusPerformance::default()
        };
        let mut diagnostics = Vec::new();

        write_corpus_performance_to(
            "coverage",
            Duration::from_secs(17),
            performance,
            4,
            &mut diagnostics,
        )
        .unwrap();

        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(diagnostics.contains("workers=2"));
        assert!(diagnostics.contains("criterion=capped_workers"));
        assert!(!diagnostics.contains("WARNING english-v2-common-path-performance-regression"));
    }

    #[test]
    fn full_parallelism_reports_the_quiet_host_criterion_with_the_regression_warning() {
        let performance = CorpusPerformance {
            workers: 4,
            ..CorpusPerformance::default()
        };
        let mut diagnostics = Vec::new();

        write_corpus_performance_to(
            "coverage",
            Duration::from_secs(17),
            performance,
            4,
            &mut diagnostics,
        )
        .unwrap();

        let diagnostics = String::from_utf8(diagnostics).unwrap();
        assert!(diagnostics.contains("criterion=quiet_host"));
        assert!(diagnostics.contains("WARNING english-v2-common-path-performance-regression"));
    }

    fn explicit_onsets(
        rows: impl IntoIterator<Item = (&'static str, Onset)>,
    ) -> BTreeMap<String, Onset> {
        rows.into_iter()
            .map(|(name, onset)| (name.to_owned(), onset))
            .collect()
    }

    #[test]
    fn corpus_unit_map_overlaps_work_and_preserves_source_order() {
        let units = [
            CorpusUnit::for_test("First", ""),
            CorpusUnit::for_test("Second", ""),
        ];
        let (sender, receiver) = sync_channel(1);
        let receiver = Mutex::new(receiver);

        let (results, _) = map_corpus_units_with_workers(
            &units,
            2,
            |index, unit| match unit.card_name() {
                "First" => format!(
                    "{index}:{}",
                    receiver
                        .lock()
                        .unwrap()
                        .recv_timeout(Duration::from_secs(1))
                        .unwrap()
                ),
                "Second" => {
                    sender.send("released").unwrap();
                    format!("{index}:second")
                }
                name => panic!("unexpected mapped corpus unit {name}"),
            },
            |_| false,
        );

        assert_eq!(results, ["0:released", "1:second"]);
    }

    #[test]
    fn corpus_unit_map_honors_requested_concurrency() {
        let units = (0..12)
            .map(|index| CorpusUnit::for_test(&format!("Unit {index:02}"), ""))
            .collect::<Vec<_>>();
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let cohort = Arc::new(Barrier::new(units.len()));

        let (results, _) = map_corpus_units_with_workers(
            &units,
            64,
            {
                let active = Arc::clone(&active);
                let peak = Arc::clone(&peak);
                let cohort = Arc::clone(&cohort);
                move |index, _unit| {
                    let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                    peak.fetch_max(now, Ordering::SeqCst);
                    cohort.wait();
                    active.fetch_sub(1, Ordering::SeqCst);
                    index
                }
            },
            |_| false,
        );

        assert_eq!(results, (0..units.len()).collect::<Vec<_>>());
        assert_eq!(peak.load(Ordering::SeqCst), units.len());
    }

    fn snapshot_onsets() -> BTreeMap<String, Onset> {
        explicit_onsets([
            ("Alpha", Onset::Vowel),
            ("Empty", Onset::Vowel),
            ("Front", Onset::Consonant),
            ("Restricted", Onset::Consonant),
        ])
    }

    const SNAPSHOT_A: &[u8] = br#"{
        "data": {
            "Restricted": [{
                "name": "Restricted", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Restricted"},
                "text": "Restricted text."
            }],
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Front", "side": "a",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "1\u20142 | Choose one."
            }, {
                "name": "Front // Back", "faceName": "Back", "side": "b",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned back."
            }],
            "Alpha": [{
                "name": "Alpha", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "\u2018Alpha\u2019"
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned text."
            }],
            "Empty": [{
                "name": "Empty", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"}
            }]
        }
    }"#;

    const SNAPSHOT_B: &[u8] = br#"{
        "data": {
            "Empty": [{
                "name": "Empty", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"}
            }],
            "Banned": [{
                "name": "Banned", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned text."
            }],
            "Alpha": [{
                "name": "Alpha", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "\u2018Alpha\u2019"
            }],
            "Front // Back": [{
                "name": "Front // Back", "faceName": "Back", "side": "b",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Banned"},
                "text": "Banned back."
            }, {
                "name": "Front // Back", "faceName": "Front", "side": "a",
                "layout": "modal_dfc", "types": ["Creature"], "supertypes": [],
                "subtypes": [], "legalities": {"vintage": "Legal"},
                "text": "1\u20142 | Choose one."
            }],
            "Restricted": [{
                "name": "Restricted", "layout": "normal", "types": ["Creature"],
                "supertypes": [], "subtypes": [], "legalities": {"vintage": "Restricted"},
                "text": "Restricted text."
            }]
        }
    }"#;

    #[test]
    fn corpus_patch_changes_raw_text_before_normalization_and_keeps_its_reason() {
        let snapshot = br#"{"data":{"Patch Fixture":[{
            "name":"Patch Fixture", "layout":"normal", "types":["Creature"],
            "supertypes":[], "subtypes":[], "legalities":{"vintage":"Legal"},
            "text":"\u2018Creatue\u2019 enters tapped."
        }]}}"#;
        let ledger = CorpusPatchLedger::parse(
            r#"CorpusPatchLedger(
                patches: [
                    CorpusTextPatch(
                        reason: "The source snapshot misspells creature.",
                        card_or_face_names: ["Patch Fixture"],
                        find: "‘Creatue’",
                        replace: "‘Creature’",
                    ),
                ],
            )"#,
        )
        .expect("fixture patch ledger loads");
        let onsets = explicit_onsets([("Patch Fixture", Onset::Consonant)]);
        let cards = AtomicCards::parse(snapshot).expect("fixture snapshot loads");
        let raw_text = cards.data["Patch Fixture"][0]
            .text
            .as_deref()
            .expect("fixture has raw text");

        let corpus = Corpus::from_bytes_with_context_onsets_and_patches(snapshot, &onsets, &ledger)
            .expect("patched fixture corpus loads");
        let unit = &corpus.units()[0];

        assert_eq!(raw_text, "‘Creatue’ enters tapped.");
        assert_ne!(unit.text(), raw_text);
        assert_eq!(unit.text(), "'Creature' enters tapped.");
        assert_eq!(
            ledger.patches[0].reason,
            "The source snapshot misspells creature."
        );
    }

    #[test]
    fn corpus_patch_matching_no_corpus_unit_fails_the_load() {
        let snapshot = br#"{"data":{"Patch Fixture":[{
            "name":"Patch Fixture", "layout":"normal", "types":["Creature"],
            "supertypes":[], "subtypes":[], "legalities":{"vintage":"Legal"},
            "text":"Patch Fixture enters tapped."
        }]}}"#;
        let ledger = CorpusPatchLedger::parse(
            r#"CorpusPatchLedger(
                patches: [
                    CorpusTextPatch(
                        reason: "The source snapshot drops a comma.",
                        card_or_face_names: ["Absent Fixture"],
                        find: "enters tapped",
                        replace: "enters the battlefield tapped",
                    ),
                ],
            )"#,
        )
        .expect("fixture patch ledger loads");
        let onsets = explicit_onsets([("Patch Fixture", Onset::Consonant)]);

        let error = Corpus::from_bytes_with_context_onsets_and_patches(snapshot, &onsets, &ledger)
            .expect_err("a patch naming no corpus unit fails the load");

        assert!(
            error.to_string().contains("matched no corpus unit"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn normalization_straightens_typography_and_strips_reminder_text() {
        let input = "‘Choose’.\n1—9 | Draw a card.\n2-10 | Get {E}{E} (two energy counters), then don’t strip.\nDeal 3-4 damage.\n[-2]: Act.";
        assert_eq!(
            normalize_oracle_text("Fixture", input).unwrap(),
            "'Choose'.\n1–9 | Draw a card.\n2–10 | Get {E}{E}, then don't strip.\nDeal 3-4 damage.\n[-2]: Act."
        );
    }

    #[test]
    fn normalization_preserves_non_reminder_document_structure() {
        let input = "Choose one —\n• Draw a card.\n• Create a token.\n(Fixed reminder.)";
        assert_eq!(
            normalize_oracle_text("Fixture", input).unwrap(),
            "Choose one —\n• Draw a card.\n• Create a token."
        );
    }

    #[test]
    fn reminder_stripping_pins_surrounding_space_and_malformed_input_contracts() {
        assert_eq!(
            strip_reminder_text(
                "Fixture",
                "Flying (This creature can't be blocked except by...)"
            )
            .unwrap(),
            "Flying"
        );
        assert_eq!(
            strip_reminder_text("Fixture", "(the Fridge) Foo").unwrap(),
            "Foo"
        );
        assert_eq!(
            strip_reminder_text(
                "Fixture",
                "Get {E} (an energy counter), then {E}{E} (two energy counters)."
            )
            .unwrap(),
            "Get {E}, then {E}{E}."
        );
        assert_eq!(
            strip_reminder_text("Fixture", "Choose (perhaps").unwrap(),
            "Choose (perhaps"
        );
        assert_eq!(
            strip_reminder_text(
                "Fixture",
                "({R/P} can be paid with {R} or 2 life.)\nGain control."
            )
            .unwrap(),
            "Gain control."
        );
        assert_eq!(
            strip_reminder_text("Fixture", "(Reminder only.) ").unwrap(),
            ""
        );
        assert_eq!(
            strip_reminder_text("Fixture", "A (an energy counter) (two energy counters) d")
                .unwrap(),
            "A d"
        );
    }

    #[test]
    fn unknown_parenthetical_followed_by_text_is_a_card_named_error() {
        let error = strip_reminder_text("Tripwire Card", "Before (unknown spelling) after")
            .unwrap_err()
            .to_string();

        assert!(error.contains("Tripwire Card"), "{error}");
        assert!(error.contains("(unknown spelling)"), "{error}");
    }

    #[test]
    fn unknown_line_initial_parenthetical_is_a_card_named_error() {
        let error = strip_reminder_text("Tripwire Card", "(a wholly novel gloss) trample.")
            .unwrap_err()
            .to_string();

        assert!(error.contains("Tripwire Card"), "{error}");
        assert!(error.contains("(a wholly novel gloss)"), "{error}");
    }

    #[test]
    fn unknown_parenthetical_after_a_leading_reminder_is_an_error() {
        let error = strip_reminder_text(
            "Tripwire Card",
            "(the Fridge) (a wholly novel gloss) trample.",
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("Tripwire Card"), "{error}");
        assert!(error.contains("(a wholly novel gloss)"), "{error}");
    }

    #[test]
    fn empty_parenthetical_followed_by_text_is_a_card_named_error() {
        let error = strip_reminder_text("Empty Group", "Before () after")
            .unwrap_err()
            .to_string();

        assert!(error.contains("Empty Group"), "{error}");
        assert!(error.contains("()"), "{error}");
    }

    #[test]
    fn empty_line_final_parenthetical_is_preserved() {
        assert_eq!(
            strip_reminder_text("Empty Group", "Before ()").unwrap(),
            "Before ()"
        );
        assert_eq!(strip_reminder_text("Empty Group", "()").unwrap(), "()");
    }

    #[test]
    fn rules_bearing_parentheticals_survive_byte_exactly() {
        for parenthetical in RULES_BEARING_PARENTHETICALS {
            for input in [
                format!("{parenthetical} after"),
                format!("Before {parenthetical} after"),
                format!("Before {parenthetical}"),
            ] {
                assert_eq!(strip_reminder_text("Fixture", &input).unwrap(), input);
            }
        }
    }

    #[test]
    fn classified_parenthetical_inventory_is_reported_without_replacing_normalization_errors() {
        let bytes =
            deckmaste_data::mtgjson::atomic_cards_bytes().expect("reading AtomicCards snapshot");
        let cards = AtomicCards::parse(&bytes).expect("parsing AtomicCards snapshot");
        let vintage_cards = cards
            .data
            .values()
            .flatten()
            .filter(|card| card.vintage_playable())
            .collect::<Vec<_>>();

        for card in &vintage_cards {
            strip_reminder_text(card.name.as_str(), card.text.as_deref().unwrap_or_default())
                .unwrap_or_else(|error| panic!("{error}"));
        }

        let inventory = parenthetical_inventory_from_texts(
            vintage_cards.iter().filter_map(|card| card.text.as_deref()),
        );
        assert_eq!(
            inventory.rules_bearing.len(),
            RULES_BEARING_PARENTHETICALS.len()
        );
        assert_eq!(
            inventory.reminder_followed_by_text.len(),
            REMINDER_FOLLOWED_BY_TEXT_PARENTHETICALS.len()
        );
        assert!(
            inventory
                .rules_bearing
                .iter()
                .all(|row| !row.surface.is_empty())
        );
    }

    #[test]
    fn vintage_snapshot_has_no_line_initial_parenthetical_followed_by_text() {
        let bytes =
            deckmaste_data::mtgjson::atomic_cards_bytes().expect("reading AtomicCards snapshot");
        let cards = AtomicCards::parse(&bytes).expect("parsing AtomicCards snapshot");
        let mut line_initial_parentheticals_followed_by_text = Vec::new();

        for card in cards
            .data
            .values()
            .flatten()
            .filter(|card| card.vintage_playable())
        {
            for line in card.text.as_deref().unwrap_or_default().split('\n') {
                let Some(remainder) = line.strip_prefix('(') else {
                    continue;
                };
                let Some(relative_close) = remainder.find(')') else {
                    continue;
                };
                let after = &remainder[relative_close + 1..];
                if !after.trim().is_empty() {
                    line_initial_parentheticals_followed_by_text.push((card.name.as_str(), line));
                }
            }
        }

        assert!(
            line_initial_parentheticals_followed_by_text.is_empty(),
            "line-initial parentheticals followed by text changed normalization identity: {line_initial_parentheticals_followed_by_text:?}",
        );
    }

    #[test]
    #[should_panic(expected = "nested parenthetical")]
    fn nested_parentheticals_are_rejected() {
        let _ = strip_reminder_text("Fixture", "Choose (an outer (nested) phrase).");
    }

    #[test]
    fn whole_text_reminder_normalizes_to_an_empty_document() {
        assert_eq!(
            normalize_oracle_text("Fixture", "(Basic land reminder text.)").unwrap(),
            ""
        );
    }

    #[test]
    fn normalized_basic_land_and_french_vanilla_probes_select() {
        let snapshot = br#"{"data":{
            "A.I.M. Bot":[{
                "name":"A.I.M. Bot", "layout":"normal", "types":["Creature"],
                "supertypes":[], "subtypes":[],
                "legalities":{"vintage":"Legal"},
                "text":"Flying (This creature can't be blocked except by creatures with flying or reach.)"
            }],
            "Plains":[{
                "name":"Plains", "layout":"normal", "types":["Land"],
                "supertypes":["Basic"], "subtypes":["Plains"],
                "legalities":{"vintage":"Legal"}, "text":"({T}: Add {W}.)"
            }]
        }}"#;
        let onsets = explicit_onsets([("A.I.M. Bot", Onset::Vowel), ("Plains", Onset::Consonant)]);
        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets).unwrap();
        let parser =
            crate::english_v2::parser_from_builtin_v2().expect("probe grammar initializes");

        for (name, text) in [("A.I.M. Bot", "Flying"), ("Plains", "")] {
            let unit = corpus
                .units()
                .iter()
                .find(|unit| unit.card_name() == name)
                .expect("probe unit exists");
            assert_eq!(unit.text(), text);
            let context = ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .expect("probe context is valid");
            let analysis = parser.analyze_oracle_text(unit.text(), &context);
            assert!(analysis.selected().is_some(), "{name} selects");
            assert!(
                analysis
                    .ownership()
                    .expect("selected probe has ownership")
                    .summary()
                    .covered(),
                "{name} is totally owned",
            );
        }
    }

    #[test]
    fn corpus_filters_normalizes_and_sorts_equivalent_snapshots() {
        let onsets = snapshot_onsets();
        let left = Corpus::from_bytes_with_context_onsets(SNAPSHOT_A, &onsets).unwrap();
        let right = Corpus::from_bytes_with_context_onsets(SNAPSHOT_B, &onsets).unwrap();
        assert_eq!(left.units(), right.units());
        assert_eq!(left.normalization_digest(), right.normalization_digest());
        assert_eq!(left.units().len(), 4);
        assert_eq!(
            left.units()
                .iter()
                .map(CorpusUnit::context_name)
                .collect::<Vec<_>>(),
            ["Alpha", "Empty", "Front", "Restricted"]
        );
        assert!(left.units().iter().all(|unit| unit.id().len() == 64));
        assert_eq!(left.units()[1].text(), "");

        let front = &left.units()[2];
        assert_eq!(front.card_name(), "Front // Back");
        assert_eq!(front.face_name(), Some("Front"));
        assert_eq!(front.side.as_deref(), Some("a"));
        assert_eq!(front.text(), "1–2 | Choose one.");
    }

    #[test]
    fn normalization_digest_frames_texts_onsets_and_preserves_unit_order() {
        assert_ne!(
            normalization_digest([("ab", Onset::Consonant), ("c", Onset::Consonant)]),
            normalization_digest([("a", Onset::Consonant), ("bc", Onset::Consonant)])
        );
        assert_ne!(
            normalization_digest([("first", Onset::Consonant), ("second", Onset::Vowel),]),
            normalization_digest([("second", Onset::Vowel), ("first", Onset::Consonant),])
        );
        assert_ne!(
            normalization_digest([("same", Onset::Consonant)]),
            normalization_digest([("same", Onset::Vowel)])
        );
        assert_ne!(
            normalization_digest([("", Onset::Consonant)]),
            normalization_digest([])
        );
    }

    #[test]
    fn corpus_threads_authoritative_legendary_face_metadata() {
        let snapshot = br#"{"data":{
            "Aang, A Lot to Learn":[{
                "name":"Aang, A Lot to Learn", "layout":"normal",
                "types":["Creature"], "supertypes":["Legendary"], "subtypes":[],
                "legalities":{"vintage":"Legal"}, "text":"Aang gains 2 life."
            }],
            "Fear, Fire, Foes!":[{
                "name":"Fear, Fire, Foes!", "layout":"normal",
                "types":["Sorcery"], "supertypes":[], "subtypes":[],
                "legalities":{"vintage":"Legal"}, "text":"Fear, Fire, Foes! deals 1 damage to any target."
            }]
        }}"#;
        let onsets = explicit_onsets([
            ("Aang, A Lot to Learn", Onset::Vowel),
            ("Fear, Fire, Foes!", Onset::Consonant),
        ]);
        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets)
            .expect("metadata fixture loads");
        let legendary = corpus
            .units()
            .iter()
            .find(|unit| unit.context_name() == "Aang, A Lot to Learn")
            .expect("legendary row exists");
        let ordinary = corpus
            .units()
            .iter()
            .find(|unit| unit.context_name() == "Fear, Fire, Foes!")
            .expect("nonlegendary row exists");
        assert!(legendary.is_legendary());
        assert!(!ordinary.is_legendary());
    }

    #[test]
    fn corpus_retains_an_explicit_empty_text_face() {
        let snapshot = br#"{"data":{"Explicit Empty":[{"name":"Explicit Empty","layout":"normal","types":["Creature"],"supertypes":[],"subtypes":[],"legalities":{"vintage":"Legal"},"text":""}]}}"#;

        let onsets = explicit_onsets([("Explicit Empty", Onset::Vowel)]);
        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets).unwrap();
        assert_eq!(corpus.units().len(), 1);
        assert_eq!(corpus.units()[0].context_name(), "Explicit Empty");
        assert_eq!(corpus.units()[0].text(), "");
    }

    #[test]
    fn corpus_rejects_only_empty_parse_contexts() {
        let snapshot = br#"{"data":{"Fixture":[{"name":"","layout":"normal","types":["Creature"],"supertypes":[],"subtypes":[],"legalities":{"vintage":"Legal"},"text":"Fixture text."}]}}"#;

        let error = Corpus::from_bytes_with_context_onsets(snapshot, &BTreeMap::new())
            .expect_err("empty parser context must reject the MTGJSON source")
            .to_string();

        assert!(error.contains("invalid parser context"));
        assert!(error.contains("context name is empty"));
        assert!(error.contains("card name \"\""));
    }

    #[test]
    fn corpus_admits_opaque_nonempty_names_with_explicit_onset_metadata() {
        let snapshot = br#"{"data":{
            ", Invalid":[{
                "name":", Invalid", "layout":"normal", "types":["Creature"],
                "supertypes":[], "subtypes":[],
                "legalities":{"vintage":"Legal"}, "text":", Invalid deals 1 damage to any target."
            }],
            "+2 Mace":[{
                "name":"+2 Mace", "layout":"normal", "types":["Artifact"],
                "supertypes":[], "subtypes":["Equipment"],
                "legalities":{"vintage":"Legal"}, "text":"Equipped creature gets +2/+2."
            }]
        }}"#;
        let onsets = explicit_onsets([(", Invalid", Onset::Vowel), ("+2 Mace", Onset::Consonant)]);

        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets)
            .expect("nonempty opaque names with explicit realization metadata load");

        assert_eq!(corpus.units().len(), 2);
        assert_eq!(corpus.units()[0].context_name(), "+2 Mace");
        assert_eq!(corpus.units()[0].context_onset(), Onset::Consonant);
        assert_eq!(corpus.units()[1].context_name(), ", Invalid");
        assert_eq!(corpus.units()[1].context_onset(), Onset::Vowel);
    }

    #[test]
    fn corpus_reports_missing_realization_metadata_without_parsing_the_name() {
        let snapshot = br#"{"data":{"! Unknown":[{
            "name":"! Unknown", "layout":"normal", "types":["Creature"],
            "supertypes":[], "subtypes":[], "legalities":{"vintage":"Legal"},
            "text":"! Unknown deals 1 damage to any target."
        }]}}"#;

        let error = Corpus::from_bytes_with_context_onsets(snapshot, &BTreeMap::new())
            .expect_err("the separate generated onset fact is required")
            .to_string();

        assert!(error.contains("missing explicit card-name onset metadata"));
        assert!(error.contains("! Unknown"));
        assert!(!error.contains("normalizing"));
    }

    #[test]
    #[should_panic(expected = "invalid parser context")]
    fn test_corpus_constructor_rejects_invalid_contexts() {
        let _ = Corpus::from_units_for_test(vec![CorpusUnit::for_test("", "Fixture text.")]);
    }
}
