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
use serde::Serialize;

use crate::english_v3::EnglishV3Args;
use crate::english_v3::SourceField;
use crate::english_v3::sum_metrics;
use crate::english_v3::validation::Issue;
use crate::english_v3::validation::NodeIdentity;
use crate::english_v3::validation::TracedValue;
use crate::raw_corpus::SUPPORT_FILTER;
use crate::raw_corpus::SelectedCorpus;
use crate::raw_corpus::SelectedFace;
use crate::raw_corpus::SelectionRequest;
use crate::raw_corpus::SelectionStatus;
use crate::raw_corpus::digest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Census {
    No,
    One,
    Multiple,
    Undetermined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Enumeration {
    Complete,
    Limited,
    Failed,
}

#[derive(Debug, Serialize)]
pub(super) struct UnknownWord {
    pub text: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Serialize)]
struct WordSample {
    value: LexicalReading,
    frame: Option<usize>,
    countability: Option<bool>,
    provenance: Option<Source>,
}

#[derive(Debug, Serialize)]
pub(super) struct Sample {
    id: String,
    tree: String,
    nodes: Vec<NodeIdentity>,
    words: Vec<WordSample>,
}

impl Sample {
    pub fn new(reading: &Reading, traced: &TracedValue, lexicon: &Lexicon) -> Self {
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
                    LexicalReading::Numeral { .. } => None,
                },
            })
            .collect();
        Self {
            id: digest(format!("{reading:?}").as_bytes()),
            tree: format!("{reading:#?}"),
            nodes: traced.nodes.clone(),
            words,
        }
    }
}

#[derive(Debug, Serialize)]
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
    pub unknown_words: Vec<UnknownWord>,
    pub enumeration: Enumeration,
    pub census: Census,
    pub checked_readings: usize,
    pub exact_readings: Option<usize>,
    pub chart: BTreeMap<&'static str, usize>,
    pub readings: BTreeMap<&'static str, usize>,
    pub constructions: BTreeMap<&'static str, usize>,
    pub issues: Vec<Issue>,
    pub samples: Vec<Sample>,
    pub lexical_wall_ns: u128,
    pub chart_wall_ns: u128,
    pub validation_wall_ns: u128,
    pub thread_cpu_ns: Option<u128>,
}

impl FaceReport {
    pub fn new(face: &SelectedFace, field: SourceField) -> Self {
        let raw = face.text.as_deref();
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
            unknown_words: Vec::new(),
            enumeration: Enumeration::Failed,
            census: Census::Undetermined,
            checked_readings: 0,
            exact_readings: None,
            chart: chart_metrics(Metrics::default()),
            readings: reading_metrics(ReadingMetrics::default()),
            constructions: BTreeMap::new(),
            issues: Vec::new(),
            samples: Vec::new(),
            lexical_wall_ns: 0,
            chart_wall_ns: 0,
            validation_wall_ns: 0,
            thread_cpu_ns: None,
        }
    }
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
    checked_readings: usize,
    exact_readings: Option<usize>,
    thread_cpu_ns: Option<u128>,
    pub checked_text_cpu_ns_per_byte: Option<u128>,
    chart: BTreeMap<&'static str, usize>,
    readings: BTreeMap<&'static str, usize>,
    constructions: BTreeMap<&'static str, usize>,
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
                totals.checked_text_bytes += bytes;
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
    reading_limit: Option<usize>,
    samples_per_face: usize,
    unmapped_sources: Vec<String>,
    pub workers: usize,
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
            schema_version: 3,
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
            input_policy: "Analyze the selected raw face field with its declared root Category, without normalization or removal of reminders. Missing fields are analyzed as empty with null retained in metadata. source_sha256 and all analysis metrics refer to the selected field; raw_text and type_line always retain their original meanings.",
            validation_scope: "Every counted Reading passes declaration admission, lexical ownership/context, byte-exact realization, and node/word traversal comparison against materialization traces. Independent linguistic correctness and the independently constructed-value roundtrip law are NOT checked by this corpus command.",
            fingerprint_encoding: "SHA-256 of generated Reading Debug; diagnostic identity within this source tree, not a stable serialization contract. Node fingerprints include complete subtrees.",
            reading_limit: args.reading_limit.map(std::num::NonZeroUsize::get),
            samples_per_face: args.samples_per_face,
            unmapped_sources,
            workers: args.workers.get(),
            measurements,
            totals: Totals::new(&faces, args.field),
            residual_groups,
            unknown_word_faces,
            faces,
        }
    }
}

pub(super) fn chart_metrics(m: Metrics) -> BTreeMap<&'static str, usize> {
    BTreeMap::from([
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
    ])
}

pub(super) fn reading_metrics(m: ReadingMetrics) -> BTreeMap<&'static str, usize> {
    BTreeMap::from([
        ("requests", m.requests),
        ("derivations", m.derivations),
        ("readings", m.readings),
        ("duplicates", m.duplicates),
        ("cyclic_derivations", m.cyclic_derivations),
        ("internal_failures", m.internal_failures),
        ("builds", m.builds),
    ])
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
