use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use deckmaste_english_v3::Metrics;
use deckmaste_english_v3::ReadingMetrics;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Source;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::english_v3::EnglishV3Args;
use crate::english_v3::SourceField;
use crate::english_v3::sum_metrics;
use crate::english_v3::validation::Issue;
use crate::english_v3::validation::MaterializationMetrics;
use crate::english_v3::validation::NodeIdentity;
use crate::english_v3::validation::TracedValue;
use crate::raw_corpus::SUPPORT_FILTER;
use crate::raw_corpus::SelectedCorpus;
use crate::raw_corpus::SelectedFace;
use crate::raw_corpus::SelectionRequest;
use crate::raw_corpus::SelectionStatus;
use crate::raw_corpus::digest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Census {
    No,
    One,
    Multiple,
    Undetermined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Enumeration {
    Complete,
    Limited,
    Failed,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct LineReport {
    /// Half-open byte offsets in the face's analyzed source.
    pub span: (usize, usize),
    pub category: String,
    pub retained_reason: Option<String>,
    pub proposed_parts: Vec<(String, (usize, usize))>,
    pub text_sha256: String,
    pub checked_readings: usize,
    pub readings_sha256: Option<String>,
    pub enumeration: Enumeration,
    #[serde(default)]
    pub nominal_normal_forms: Option<BTreeSet<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct UnknownWord {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct WordSample {
    value: LexicalReading,
    frame: Option<usize>,
    countability: Option<bool>,
    provenance: Option<Source>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Sample {
    total_cost: u64,
    id: String,
    tree: String,
    nodes: Vec<NodeIdentity>,
    words: Vec<WordSample>,
}

impl Sample {
    pub fn new(
        reading: &Reading,
        traced: &TracedValue,
        lexicon: &Lexicon,
        total_cost: u64,
    ) -> Self {
        let words = traced
            .words
            .iter()
            .map(|word| WordSample {
                value: word.value.clone(),
                frame: word.frame,
                countability: word.countability,
                provenance: match &word.value {
                    LexicalReading::Word(value) => lexicon
                        .lexemes()
                        .get(&value.lexeme)
                        .map(|lexeme| lexeme.source.clone()),
                    LexicalReading::Numeral { .. } | LexicalReading::FlavorWord { .. } => None,
                },
            })
            .collect();
        Self {
            total_cost,
            id: NodeIdentity::new(reading).sha256,
            tree: format!("{reading:#?}"),
            nodes: traced
                .nodes
                .iter()
                .filter_map(|value| {
                    if let deckmaste_english_v3::grammar::Value::Reading(node) = &**value {
                        Some(NodeIdentity::new(node))
                    } else {
                        None
                    }
                })
                .collect(),
            words,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct FaceReport {
    pub id: String,
    pub oracle_id: String,
    pub printing_id: String,
    pub group_name: String,
    pub face_index: Option<usize>,
    pub name: String,
    pub face_name: Option<String>,
    pub side: Option<String>,
    pub type_line: Option<String>,
    pub raw_text: Option<String>,
    pub raw_text_sha256: String,
    pub source_sha256: String,
    pub analyzed_source: String,
    pub analyzed_source_sha256: String,
    pub unknown_words: Vec<UnknownWord>,
    pub enumeration: Enumeration,
    pub census: Census,
    pub checked_readings: usize,
    #[serde(default)]
    pub ambiguous: Option<bool>,
    #[serde(default)]
    pub partial_checked_readings: Option<usize>,
    pub exact_readings: Option<usize>,
    pub readings_sha256: Option<String>,
    pub lines: Vec<LineReport>,
    #[serde(default)]
    pub ability_spans: Vec<(usize, usize)>,
    #[serde(default)]
    pub helper_fallbacks: usize,
    #[serde(default)]
    pub surroundings_sha256: Option<String>,
    #[serde(default)]
    pub migration: Vec<super::decomposition::Bridge>,
    pub chart: BTreeMap<String, usize>,
    pub readings: BTreeMap<String, usize>,
    pub materializations: MaterializationMetrics,
    /// Construction occurrences summed over line Readings, not Document
    /// combinations.
    pub constructions: BTreeMap<String, usize>,
    pub issues: Vec<Issue>,
    pub samples: Vec<Sample>,
    pub lexical_wall_ns: u128,
    pub chart_wall_ns: u128,
    pub validation_wall_ns: u128,
    pub thread_cpu_ns: Option<u128>,
}

/// The parser input the census derives from a face's selected field: rules
/// text with balanced reminder text removed, or the Type Line unchanged.
pub(super) fn analyzed_source(source: &str, field: SourceField) -> String {
    match field {
        SourceField::Text => deckmaste_data::oracle_text::strip_reminder_text(source),
        SourceField::TypeLine => source.to_owned(),
    }
}

impl FaceReport {
    pub fn new(face: &SelectedFace, field: SourceField) -> Self {
        let raw = face.text.as_deref();
        let source = field.source(face).unwrap_or("");
        let analyzed_source = analyzed_source(source, field);
        Self {
            id: face.identity.clone(),
            oracle_id: face.oracle_id.clone(),
            printing_id: face.printing_id.clone(),
            group_name: face.group_name.clone(),
            face_index: face.face_index,
            name: face.name.clone(),
            face_name: face.face_name.clone(),
            side: face.side.clone(),
            type_line: face.type_line.clone(),
            raw_text: raw.map(str::to_owned),
            raw_text_sha256: digest(raw.unwrap_or("").as_bytes()),
            source_sha256: digest(field.source(face).unwrap_or("").as_bytes()),
            analyzed_source_sha256: digest(analyzed_source.as_bytes()),
            analyzed_source,
            unknown_words: Vec::new(),
            enumeration: Enumeration::Failed,
            census: Census::Undetermined,
            checked_readings: 0,
            ambiguous: None,
            partial_checked_readings: None,
            exact_readings: None,
            readings_sha256: None,
            lines: Vec::new(),
            ability_spans: Vec::new(),
            helper_fallbacks: 0,
            surroundings_sha256: None,
            migration: Vec::new(),
            chart: chart_metrics(Metrics::default()),
            readings: reading_metrics(ReadingMetrics::default()),
            materializations: MaterializationMetrics::default(),
            constructions: BTreeMap::new(),
            issues: Vec::new(),
            samples: Vec::new(),
            lexical_wall_ns: 0,
            chart_wall_ns: 0,
            validation_wall_ns: 0,
            thread_cpu_ns: None,
        }
    }

    pub fn finish_readings(&mut self) {
        self.enumeration = if !self.issues.is_empty()
            || self
                .lines
                .iter()
                .any(|line| line.enumeration == Enumeration::Failed)
        {
            Enumeration::Failed
        } else if self
            .lines
            .iter()
            .any(|line| line.enumeration == Enumeration::Limited)
        {
            Enumeration::Limited
        } else {
            Enumeration::Complete
        };
        // A sum must never conceal an unread line. No root derivation has no
        // line records and is likewise uncovered. EmptyDocument is represented
        // by one empty segment to preserve its existing single Reading.
        self.checked_readings =
            if self.lines.is_empty() || self.lines.iter().any(|line| line.checked_readings == 0) {
                0
            } else {
                self.lines.iter().map(|line| line.checked_readings).sum()
            };
        let observed_ambiguity = self.lines.iter().any(|unit| unit.checked_readings > 1);
        self.ambiguous = (observed_ambiguity || self.enumeration == Enumeration::Complete)
            .then_some(observed_ambiguity);
        self.partial_checked_readings = (self.enumeration != Enumeration::Complete)
            .then(|| self.lines.iter().map(|unit| unit.checked_readings).sum());
        self.exact_readings =
            (self.enumeration == Enumeration::Complete).then_some(self.checked_readings);
        self.readings_sha256 = (self.enumeration == Enumeration::Complete)
            .then(|| face_digest(&self.lines, self.surroundings_sha256.as_deref()))
            .flatten();
        self.census = if self.enumeration != Enumeration::Complete {
            Census::Undetermined
        } else if self.checked_readings == 0 {
            Census::No
        } else if observed_ambiguity {
            Census::Multiple
        } else {
            Census::One
        };
    }
}

/// Hash ordered unit descriptors with their tree-set digests. Document order
/// is independent of Reading enumeration order within each unit.
pub(super) fn lines_digest(lines: &[LineReport]) -> Option<String> {
    let descriptors = lines
        .iter()
        .map(|line| {
            Some((
                &line.category,
                line.span,
                &line.text_sha256,
                line.readings_sha256.as_ref()?,
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    Some(digest(
        &serde_json::to_vec(&(READINGS_DIGEST_VERSION, descriptors)).ok()?,
    ))
}

fn face_digest(lines: &[LineReport], surroundings: Option<&str>) -> Option<String> {
    Some(digest(
        &serde_json::to_vec(&(DECOMPOSITION_VERSION, lines_digest(lines)?, surroundings)).ok()?,
    ))
}

#[derive(Debug, Default, Serialize)]
pub(super) struct Totals {
    pub no: usize,
    pub one: usize,
    pub multiple: usize,
    pub undetermined: usize,
    pub complete: usize,
    pub limited: usize,
    pub failed: usize,
    pub issues: usize,
    supported_faces_without_text: usize,
    supported_faces_without_source: usize,
    source_bytes: usize,
    checked_text_bytes: usize,
    pub checked_readings: usize,
    exact_readings: Option<usize>,
    thread_cpu_ns: Option<u128>,
    pub checked_text_cpu_ns_per_byte: Option<u128>,
    chart: BTreeMap<String, usize>,
    readings: BTreeMap<String, usize>,
    constructions: BTreeMap<String, usize>,
}

impl Totals {
    fn new(faces: &[FaceReport], field: SourceField) -> Self {
        let mut totals = Self::default();
        let mut checked_cpu = Some(0_u128);
        let mut cpu = Some(0_u128);
        for face in faces {
            match face.census {
                Census::No => totals.no += 1,
                Census::One => totals.one += 1,
                Census::Multiple => totals.multiple += 1,
                Census::Undetermined => totals.undetermined += 1,
            }
            match face.enumeration {
                Enumeration::Complete => totals.complete += 1,
                Enumeration::Limited => totals.limited += 1,
                Enumeration::Failed => totals.failed += 1,
            }
            totals.issues += face.issues.len();
            totals.supported_faces_without_text += usize::from(face.raw_text.is_none());
            let source = match field {
                SourceField::Text => &face.raw_text,
                SourceField::TypeLine => &face.type_line,
            };
            totals.supported_faces_without_source += usize::from(source.is_none());
            let bytes = source.as_ref().map_or(0, String::len);
            totals.source_bytes += bytes;
            totals.checked_readings += face.checked_readings;
            cpu = cpu.zip(face.thread_cpu_ns).map(|(sum, n)| sum + n);
            if face.checked_readings > 0 {
                totals.checked_text_bytes += face.analyzed_source.len();
                checked_cpu = checked_cpu.zip(face.thread_cpu_ns).map(|(sum, n)| sum + n);
            }
        }
        totals.thread_cpu_ns = cpu;
        totals.checked_text_cpu_ns_per_byte =
            checked_cpu.and_then(|n| n.checked_div(totals.checked_text_bytes as u128));
        totals.exact_readings = (totals.complete == faces.len()).then_some(totals.checked_readings);
        totals.chart = sum_metrics(faces.iter().map(|face| &face.chart));
        totals.readings = sum_metrics(faces.iter().map(|face| &face.readings));
        totals.constructions = sum_metrics(faces.iter().map(|face| &face.constructions));
        totals
    }
}

#[derive(Debug, Serialize)]
pub(super) struct Measurements {
    pub host_load: Option<[f64; 3]>,
    pub setup_wall_ns: u128,
    pub corpus_wall_ns: u128,
}

#[derive(Debug, Serialize)]
pub(super) struct Report {
    schema_version: usize,
    field: SourceField,
    input: PathBuf,
    input_sha256: String,
    selection: SelectionRequest,
    selection_status: SelectionStatus,
    analysis_status: &'static str,
    records_scanned: usize,
    supported_faces_examined: usize,
    lexical_inventory_sha256: String,
    change_id: Option<String>,
    support_filter: &'static str,
    input_policy: &'static str,
    validation_scope: &'static str,
    fingerprint_encoding: &'static str,
    readings_digest_version: usize,
    decomposition_version: usize,
    context_version: usize,
    nominal_normal_form_version: usize,
    nominal_normal_form_source_sha256: String,
    count_semantics: &'static str,
    readings_digest_encoding: &'static str,
    reading_limit: Option<usize>,
    samples_per_face: usize,
    unmapped_sources: Vec<String>,
    pub workers: usize,
    pub incremental_snapshot: Option<super::incremental::Snapshot>,
    #[serde(flatten)]
    pub incremental: super::incremental::Incremental,
    measurement_scope: &'static str,
    #[serde(flatten)]
    pub measurements: Measurements,
    pub totals: Totals,
    /// Initial evidence groups; grammatical causes require inspection of the
    /// connected declarations and witnesses, not guessing from failed text.
    residual_groups: BTreeMap<&'static str, Vec<String>>,
    unknown_word_faces: BTreeMap<String, BTreeSet<String>>,
    pub faces: Vec<FaceReport>,
}

impl Report {
    pub fn new(
        args: &EnglishV3Args,
        corpus: &SelectedCorpus,
        inventory_sha256: String,
        unmapped_sources: Vec<String>,
        faces: Vec<FaceReport>,
        measurements: Measurements,
    ) -> Self {
        let mut residual_groups = BTreeMap::<_, Vec<_>>::new();
        let mut unknown_word_faces = BTreeMap::<_, BTreeSet<_>>::new();
        for face in &faces {
            let group = match (face.census, face.unknown_words.is_empty()) {
                (Census::No, false) => Some("no_reading_with_lexical_gap"),
                (Census::No, true) => Some("no_reading_unresolved_grammatical_or_lexical_cause"),
                (Census::Undetermined, _) => Some("undetermined"),
                _ => None,
            };
            if let Some(group) = group {
                residual_groups
                    .entry(group)
                    .or_default()
                    .push(face.id.clone());
            }
            if !face.issues.is_empty() {
                residual_groups
                    .entry("validation_issue")
                    .or_default()
                    .push(face.id.clone());
            }
            for word in &face.unknown_words {
                unknown_word_faces
                    .entry(word.text.clone())
                    .or_default()
                    .insert(face.id.clone());
            }
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let change_id = std::process::Command::new("jj")
            .args([
                "--no-pager",
                "log",
                "-r",
                "@",
                "--no-graph",
                "-T",
                "change_id",
            ])
            .current_dir(root)
            .output()
            .ok()
            .filter(|out| out.status.success())
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|text| text.trim().to_owned());
        Self {
            schema_version: SCHEMA_VERSION,
            field: args.field,
            input: args.data.clone(),
            input_sha256: corpus.snapshot_sha256.clone(),
            selection: corpus.selection.clone(),
            selection_status: corpus.selection_status,
            analysis_status: if args.reading_limit.is_some() {
                "limited"
            } else {
                match corpus.selection_status {
                    SelectionStatus::Complete => "complete",
                    SelectionStatus::Subset => "subset",
                }
            },
            records_scanned: corpus.records_scanned,
            supported_faces_examined: corpus.supported_faces_examined,
            lexical_inventory_sha256: inventory_sha256,
            change_id,
            support_filter: SUPPORT_FILTER,
            input_policy: "Strip balanced parenthesized reminder text before analyzing rules text as a Document; Type Lines are unchanged. Reminder contents are not parsed. Missing fields are analyzed as empty with null retained in metadata. raw_text, type_line and source_sha256 retain the original selected field. analyzed_source and analyzed_source_sha256 identify the parser input; lexical offsets and roundtrip validation refer to analyzed_source. source_bytes counts original bytes; checked_text_bytes counts analyzed bytes.",
            validation_scope: "Every counted Reading passes declaration admission, lexical ownership/context, byte-exact realization, and node/word traversal comparison against materialization traces. Independent linguistic correctness and the independently constructed-value roundtrip law are NOT checked by this corpus command.",
            fingerprint_encoding: "SHA-256 of generated Reading Debug; diagnostic identity within this source tree, not a stable serialization contract. Node fingerprints include complete subtrees.",
            readings_digest_version: READINGS_DIGEST_VERSION,
            decomposition_version: DECOMPOSITION_VERSION,
            context_version: CONTEXT_VERSION,
            nominal_normal_form_version: super::nominal_normal_form::VERSION,
            nominal_normal_form_source_sha256: super::nominal_normal_form::source_sha256(),
            count_semantics: "enumerated unit Readings; face counts are sums, not Document-tree counts; read means syntactically admitted, not a correctness claim",
            readings_digest_encoding: "Version 2 retains version-1 validated tree keys and sorted fixed-width tree-set hashing within each unit. Ordered unit descriptors (category, half-open byte span, source hash, tree-set digest) are encoded as a JSON array with digest version 2. The face hashes a JSON array of decomposition version 2, that descriptor digest, and the validated surrounding-structure/root-constraint digest. Surroundings retain constructor structure and the root contract, but omit derived root-summary storage and its feature-slot layout. Complete, issue-free enumeration alone has an exact sum or complete digest; partial checked counts are separate. EmptyDocument is one whole unit. Several unambiguous units do not imply ambiguity. Debug identities are diagnostic within the recorded source tree.",
            reading_limit: args.reading_limit.map(std::num::NonZeroUsize::get),
            samples_per_face: args.samples_per_face,
            unmapped_sources,
            workers: args.workers.get(),
            incremental_snapshot: None,
            incremental: super::incremental::Incremental::default(),
            measurement_scope: "setup_wall_ns and corpus_wall_ns measure this invocation. In incremental output, per-face chart/enumeration/materialization counters and timing/CPU telemetry for reused_faces are copied from the base census; aggregate face counters and CPU totals combine those historical observations with rechecked_faces. They are not measurements of this invocation's work.",
            measurements,
            totals: Totals::new(&faces, args.field),
            residual_groups,
            unknown_word_faces,
            faces,
        }
    }
}

pub(super) const DECOMPOSITION_VERSION: usize = 2;
pub(super) const CONTEXT_VERSION: usize = 1;

pub(super) const SCHEMA_VERSION: usize = 10;
pub(super) const READINGS_DIGEST_VERSION: usize = 2;

pub(super) fn chart_metrics(m: Metrics) -> BTreeMap<String, usize> {
    [
        ("lexical_alternatives", m.lexical_alternatives),
        ("lexical_projections", m.lexical_projections),
        ("items", m.items),
        ("intermediate_nodes", m.intermediate_nodes),
        ("completed_nodes", m.completed_nodes),
        ("intermediate_edges", m.intermediate_edges),
        ("completed_families", m.completed_families),
        ("families", m.families),
        ("completion_work", m.completion_work),
        ("scan_work", m.scan_work),
    ]
    .into_iter()
    .map(|(key, count)| (key.to_owned(), count))
    .collect()
}

pub(super) fn reading_metrics(m: ReadingMetrics) -> BTreeMap<String, usize> {
    [
        ("requests", m.requests),
        ("derivations", m.derivations),
        ("readings", m.readings),
        ("duplicates", m.duplicates),
        ("cyclic_derivations", m.cyclic_derivations),
        ("internal_failures", m.internal_failures),
        ("builds", m.builds),
    ]
    .into_iter()
    .map(|(key, count)| (key.to_owned(), count))
    .collect()
}

pub(super) fn host_load() -> Option<[f64; 3]> {
    let mut load = [0.0; 3];
    // SAFETY: load supplies three writable doubles for the requested samples.
    let samples = unsafe { libc::getloadavg(load.as_mut_ptr(), 3) };
    (samples == 3).then_some(load)
}

pub(super) fn thread_cpu_ns() -> Option<u128> {
    let mut time = std::mem::MaybeUninit::<libc::timespec>::uninit();
    // SAFETY: the pointer is writable; the value is read only on success.
    if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, time.as_mut_ptr()) } != 0 {
        return None;
    }
    // SAFETY: successful clock_gettime initialized both fields.
    let time = unsafe { time.assume_init() };
    Some(u128::try_from(time.tv_sec).ok()? * 1_000_000_000 + u128::try_from(time.tv_nsec).ok()?)
}

/// Skip fingerprints when even the candidate's cost cannot enter the sample.
pub(super) fn sample_cost_would_be_retained(samples: &[Sample], cost: u64, limit: usize) -> bool {
    limit != 0
        && (samples.len() < limit || samples.last().is_some_and(|last| cost <= last.total_cost))
}

/// Decide before building a diagnostic payload; this never limits Reading
/// validation.
pub(super) fn sample_would_be_retained(
    samples: &[Sample],
    total_cost: u64,
    id: &str,
    limit: usize,
) -> bool {
    limit != 0
        && (samples.len() < limit
            || samples
                .last()
                .is_some_and(|last| (total_cost, id) < (last.total_cost, last.id.as_str())))
}

/// Keep the cheapest samples among readings actually enumerated, including late
/// arrivals.
pub(super) fn retain_sample(samples: &mut Vec<Sample>, sample: Sample, limit: usize) {
    samples.push(sample);
    samples.sort_by(|left, right| (left.total_cost, &left.id).cmp(&(right.total_cost, &right.id)));
    samples.truncate(limit);
}

#[cfg(test)]
mod sample_tests {
    use super::Sample;
    use super::retain_sample;
    use super::sample_would_be_retained;

    fn sample(cost: u64, id: &str) -> Sample {
        Sample {
            total_cost: cost,
            id: id.into(),
            tree: String::new(),
            nodes: vec![],
            words: vec![],
        }
    }

    #[test]
    fn payload_preflight_preserves_cost_and_identity_order_without_zero_limit_work() {
        let mut samples = vec![];
        let mut built = 0;
        for (cost, id) in [
            (100, "fallback"),
            (2, "b"),
            (1, "cheap"),
            (2, "z"),
            (2, "a"),
            (50, "unused"),
        ] {
            if sample_would_be_retained(&samples, cost, id, 2) {
                built += 1;
                retain_sample(&mut samples, sample(cost, id), 2);
            }
        }
        assert_eq!(built, 4);
        assert_eq!(
            samples
                .iter()
                .map(|s| (s.total_cost, s.id.as_str()))
                .collect::<Vec<_>>(),
            [(1, "cheap"), (2, "a")]
        );
        assert!(!sample_would_be_retained(&samples, 0, "unused", 0));
    }

    #[test]
    fn late_cheaper_samples_replace_earlier_samples_and_ties_are_deterministic() {
        let mut samples = vec![];
        for (cost, id) in [(100, "fallback"), (2, "b"), (1, "cheap"), (2, "a")] {
            retain_sample(&mut samples, sample(cost, id), 2);
        }
        assert_eq!(
            samples
                .iter()
                .map(|s| (s.total_cost, s.id.as_str()))
                .collect::<Vec<_>>(),
            [(1, "cheap"), (2, "a")]
        );
        retain_sample(&mut samples, sample(0, "discard"), 0);
        assert_eq!(samples.len(), 0);
    }
}

/// Fixed-width keys make concatenation unambiguous, including the empty set.
pub(super) fn readings_digest(keys: &BTreeSet<[u8; 32]>) -> String {
    use std::fmt::Write as _;
    let mut hasher = sha2::Sha256::new();
    for key in keys {
        hasher.update(key);
    }
    let mut digest = String::with_capacity(64);
    for byte in hasher.finalize() {
        write!(digest, "{byte:02x}").expect("writing to a String is infallible");
    }
    digest
}

#[cfg(test)]
mod digest_tests {
    use std::collections::BTreeSet;

    use super::readings_digest;

    #[test]
    fn line_digest_is_order_independent_and_distinguishes_equal_sized_sets() {
        let left = BTreeSet::from([[1; 32], [2; 32]]);
        let reordered = BTreeSet::from([[2; 32], [1; 32]]);
        let changed = BTreeSet::from([[1; 32], [3; 32]]);
        assert_eq!(readings_digest(&left), readings_digest(&reordered));
        assert_ne!(readings_digest(&left), readings_digest(&changed));
        assert_eq!(
            readings_digest(&BTreeSet::new()),
            crate::raw_corpus::digest(b"")
        );
        let bytes: Vec<_> = left.iter().flatten().copied().collect();
        assert_eq!(readings_digest(&left), crate::raw_corpus::digest(&bytes));
    }
}

#[cfg(test)]
mod line_tests {
    use super::*;

    fn line(count: usize, key: u8) -> LineReport {
        LineReport {
            span: (0, 1),
            category: "Sentence".into(),
            retained_reason: None,
            proposed_parts: vec![],
            text_sha256: digest(b"a"),
            checked_readings: count,
            readings_sha256: Some(readings_digest(&BTreeSet::from([[key; 32]]))),
            enumeration: Enumeration::Complete,
            nominal_normal_forms: None,
        }
    }

    fn face() -> FaceReport {
        serde_json::from_value(serde_json::json!({
            "id":"test", "oracle_id":"test", "printing_id":"test", "group_name":"test",
            "name":"test", "face_index":null, "face_name":null, "side":null,
            "type_line":null, "raw_text":"a", "raw_text_sha256":"", "source_sha256":"",
            "analyzed_source":"a", "analyzed_source_sha256":"", "unknown_words":[],
            "enumeration":"complete", "census":"one", "checked_readings":1,
            "exact_readings":1, "readings_sha256":null, "lines":[], "chart":{}, "readings":{},
            "materializations":{"leaf_requests":0,"unique_leaves":0,"build_requests":0,"unique_builds":0},
            "constructions":{}, "issues":[], "samples":[], "lexical_wall_ns":0,
            "chart_wall_ns":0, "validation_wall_ns":0, "thread_cpu_ns":null
        })).unwrap()
    }

    #[test]
    fn face_counts_are_sums_and_digests_preserve_line_order() {
        let mut result = face();
        result.lines = vec![line(2, 1), line(3, 2)];
        result.finish_readings();
        assert_eq!(result.checked_readings, 5);
        assert_eq!(result.exact_readings, Some(5));
        assert_eq!(result.census, Census::Multiple);
        let original = result.readings_sha256.clone();
        let descriptors = result
            .lines
            .iter()
            .map(|line| {
                (
                    &line.category,
                    line.span,
                    &line.text_sha256,
                    &line.readings_sha256,
                )
            })
            .collect::<Vec<_>>();
        let descriptor_digest =
            digest(&serde_json::to_vec(&(READINGS_DIGEST_VERSION, descriptors)).unwrap());
        assert_eq!(
            original,
            Some(digest(
                &serde_json::to_vec(&(DECOMPOSITION_VERSION, descriptor_digest, None::<String>))
                    .unwrap()
            ))
        );
        result.lines.reverse();
        result.finish_readings();
        assert_ne!(result.readings_sha256, original);
    }

    #[test]
    fn an_unread_line_keeps_a_face_uncovered_and_preserves_other_line_counts() {
        let mut result = face();
        result.lines = vec![line(3, 1), line(0, 2)];
        result.finish_readings();
        assert_eq!(result.checked_readings, 0);
        assert_eq!(result.exact_readings, Some(0));
        assert_eq!(result.census, Census::No);
        assert_eq!(result.lines[0].checked_readings, 3);
        assert_eq!(result.lines[1].checked_readings, 0);
    }

    #[test]
    fn limited_and_failed_lines_prevent_complete_face_digests() {
        let mut result = face();
        result.lines = vec![line(1, 1), line(1, 2)];
        result.lines[1].enumeration = Enumeration::Limited;
        result.lines[1].readings_sha256 = None;
        result.finish_readings();
        assert_eq!(result.enumeration, Enumeration::Limited);
        assert_eq!(result.census, Census::Undetermined);
        assert_eq!(result.exact_readings, None);
        assert_eq!(result.readings_sha256, None);
        result.issues.push(Issue::AmbiguousSegmentation);
        result.finish_readings();
        assert_eq!(result.enumeration, Enumeration::Failed);
        assert_eq!(result.readings_sha256, None);
    }
}

#[cfg(test)]
mod descriptor_tests {
    use super::*;

    #[test]
    fn identical_unit_trees_do_not_hide_category_span_or_surrounding_changes() {
        let mut units = vec![LineReport {
            span: (0, 1),
            category: "Sentence".into(),
            retained_reason: None,
            proposed_parts: vec![],
            text_sha256: digest(b"x"),
            checked_readings: 1,
            readings_sha256: Some(readings_digest(&BTreeSet::from([[7; 32]]))),
            enumeration: Enumeration::Complete,
            nominal_normal_forms: None,
        }];
        let original = face_digest(&units, Some("original-wrapper"));
        assert_ne!(original, face_digest(&units, Some("different-label")));
        units[0].span = (1, 2);
        assert_ne!(original, face_digest(&units, Some("original-wrapper")));
        units[0].span = (0, 1);
        units[0].category = "Clause".into();
        assert_ne!(original, face_digest(&units, Some("original-wrapper")));
    }
}
