use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::ByteMismatchScope as RuntimeByteMismatchScope;
use deckmaste_english_v2::parser::InternalFailureKind as RuntimeInternalFailureKind;
use deckmaste_english_v2::parser::InvalidSpanKind as RuntimeInvalidSpanKind;
use deckmaste_english_v2::parser::OwnershipFailure as RuntimeOwnershipFailure;
use deckmaste_english_v2::parser::OwnershipSummary as RuntimeOwnershipSummary;
use deckmaste_english_v2::parser::ParseAnalysis;
use deckmaste_english_v2::parser::ParseAnalysisOutcome;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectedOwnership as RuntimeSelectedOwnership;

use super::CoverageArgs;
use super::corpus::Corpus;
use super::corpus::CorpusUnit;

const REPORT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CoverageLockMode {
    None,
    Check,
    Bless,
}

impl CoverageArgs {
    const fn lock_mode(&self) -> CoverageLockMode {
        if self.check {
            CoverageLockMode::Check
        } else if self.bless {
            CoverageLockMode::Bless
        } else {
            CoverageLockMode::None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CoverageStatus {
    SelectedCovered,
    SelectedUncovered,
    ParseFailure,
    UnresolvedAmbiguity,
    InternalFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CoverageInternalFailureKind {
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
    OwnershipInspection,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(super) struct RoundtripFailure {
    expected: String,
    actual: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum CoverageInvalidSpanKind {
    OutOfBounds,
    NonUtf8Boundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(super) enum CoverageByteMismatchScope {
    WholeRender,
    ClaimSlice { index: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(super) enum CoverageOwnershipFailure {
    Gap {
        start: usize,
        end: usize,
    },
    Overlap {
        left_start: usize,
        left_end: usize,
        right_start: usize,
        right_end: usize,
        overlap_start: usize,
        overlap_end: usize,
    },
    InvalidSpan {
        start: usize,
        end: usize,
        span_kind: CoverageInvalidSpanKind,
    },
    Synthetic {
        start: usize,
        end: usize,
    },
    ProvenancePlanMismatch {
        index: usize,
        parsed: String,
        rendered: String,
    },
    ByteMismatch {
        scope: CoverageByteMismatchScope,
        expected: String,
        actual: String,
    },
}

pub(super) trait OwnershipSummarySource {
    fn covered(&self) -> bool;
    fn claims(&self) -> usize;
    fn claimed_bytes(&self) -> usize;
    fn form_literal_claims(&self) -> usize;
    fn form_literal_bytes(&self) -> usize;
    fn vocab_claims(&self) -> usize;
    fn vocab_bytes(&self) -> usize;
    fn lexeme_claims(&self) -> usize;
    fn lexeme_bytes(&self) -> usize;
    fn codec_claims(&self) -> usize;
    fn codec_bytes(&self) -> usize;
    fn identity_claims(&self) -> usize;
    fn identity_bytes(&self) -> usize;
    fn gap_spans(&self) -> usize;
    fn gap_bytes(&self) -> usize;
    fn overlap_spans(&self) -> usize;
    fn overlap_bytes(&self) -> usize;
    fn synthetic_claims(&self) -> usize;
    fn provenance_plan_mismatches(&self) -> usize;
}

impl OwnershipSummarySource for RuntimeOwnershipSummary {
    fn covered(&self) -> bool {
        self.covered()
    }
    fn claims(&self) -> usize {
        self.claims()
    }
    fn claimed_bytes(&self) -> usize {
        self.claimed_bytes()
    }
    fn form_literal_claims(&self) -> usize {
        self.form_literal_claims()
    }
    fn form_literal_bytes(&self) -> usize {
        self.form_literal_bytes()
    }
    fn vocab_claims(&self) -> usize {
        self.vocab_claims()
    }
    fn vocab_bytes(&self) -> usize {
        self.vocab_bytes()
    }
    fn lexeme_claims(&self) -> usize {
        self.lexeme_claims()
    }
    fn lexeme_bytes(&self) -> usize {
        self.lexeme_bytes()
    }
    fn codec_claims(&self) -> usize {
        self.codec_claims()
    }
    fn codec_bytes(&self) -> usize {
        self.codec_bytes()
    }
    fn identity_claims(&self) -> usize {
        self.identity_claims()
    }
    fn identity_bytes(&self) -> usize {
        self.identity_bytes()
    }
    fn gap_spans(&self) -> usize {
        self.gap_spans()
    }
    fn gap_bytes(&self) -> usize {
        self.gap_bytes()
    }
    fn overlap_spans(&self) -> usize {
        self.overlap_spans()
    }
    fn overlap_bytes(&self) -> usize {
        self.overlap_bytes()
    }
    fn synthetic_claims(&self) -> usize {
        self.synthetic_claims()
    }
    fn provenance_plan_mismatches(&self) -> usize {
        self.provenance_plan_mismatches()
    }
}

pub(super) trait SelectedOwnershipSource {
    type Summary: OwnershipSummarySource;

    fn rendered_text(&self) -> &str;
    fn summary(&self) -> &Self::Summary;
    fn failures(&self) -> &[RuntimeOwnershipFailure];
}

impl SelectedOwnershipSource for RuntimeSelectedOwnership {
    type Summary = RuntimeOwnershipSummary;

    fn rendered_text(&self) -> &str {
        self.rendered_text()
    }
    fn summary(&self) -> &Self::Summary {
        self.summary()
    }
    fn failures(&self) -> &[RuntimeOwnershipFailure] {
        self.failures()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(super) struct CoverageOwnership {
    covered: bool,
    claims: usize,
    claimed_bytes: usize,
    form_literal_claims: usize,
    form_literal_bytes: usize,
    vocab_claims: usize,
    vocab_bytes: usize,
    lexeme_claims: usize,
    lexeme_bytes: usize,
    codec_claims: usize,
    codec_bytes: usize,
    identity_claims: usize,
    identity_bytes: usize,
    gap_spans: usize,
    gap_bytes: usize,
    overlap_spans: usize,
    overlap_bytes: usize,
    synthetic_claims: usize,
    provenance_plan_mismatches: usize,
    failures: Vec<CoverageOwnershipFailure>,
}

impl CoverageOwnership {
    fn from_source(
        summary: &impl OwnershipSummarySource,
        failures: &[RuntimeOwnershipFailure],
    ) -> Self {
        Self {
            covered: summary.covered(),
            claims: summary.claims(),
            claimed_bytes: summary.claimed_bytes(),
            form_literal_claims: summary.form_literal_claims(),
            form_literal_bytes: summary.form_literal_bytes(),
            vocab_claims: summary.vocab_claims(),
            vocab_bytes: summary.vocab_bytes(),
            lexeme_claims: summary.lexeme_claims(),
            lexeme_bytes: summary.lexeme_bytes(),
            codec_claims: summary.codec_claims(),
            codec_bytes: summary.codec_bytes(),
            identity_claims: summary.identity_claims(),
            identity_bytes: summary.identity_bytes(),
            gap_spans: summary.gap_spans(),
            gap_bytes: summary.gap_bytes(),
            overlap_spans: summary.overlap_spans(),
            overlap_bytes: summary.overlap_bytes(),
            synthetic_claims: summary.synthetic_claims(),
            provenance_plan_mismatches: summary.provenance_plan_mismatches(),
            failures: failures.iter().map(coverage_failure).collect(),
        }
    }

    fn validate(&self, id: &str) -> Result<(), CoverageValidationError> {
        let kind_claims = checked_sum(
            "row claim-kind sum",
            [
                self.form_literal_claims,
                self.vocab_claims,
                self.lexeme_claims,
                self.codec_claims,
                self.identity_claims,
            ],
        )?;
        if self.claims != kind_claims {
            return Err(CoverageValidationError::RowClaimKindEquation {
                id: id.to_owned(),
                claims: self.claims,
                kind_claims,
            });
        }
        let kind_bytes = checked_sum(
            "row byte-kind sum",
            [
                self.form_literal_bytes,
                self.vocab_bytes,
                self.lexeme_bytes,
                self.codec_bytes,
                self.identity_bytes,
            ],
        )?;
        if self.claimed_bytes != kind_bytes {
            return Err(CoverageValidationError::RowByteKindEquation {
                id: id.to_owned(),
                claimed_bytes: self.claimed_bytes,
                kind_bytes,
            });
        }
        Ok(())
    }

    #[cfg(test)]
    fn failures(&self) -> &[CoverageOwnershipFailure] {
        &self.failures
    }
}

fn coverage_failure(failure: &RuntimeOwnershipFailure) -> CoverageOwnershipFailure {
    match failure {
        RuntimeOwnershipFailure::Gap { span } => CoverageOwnershipFailure::Gap {
            start: span.start,
            end: span.end,
        },
        RuntimeOwnershipFailure::Overlap {
            left,
            right,
            overlap,
        } => CoverageOwnershipFailure::Overlap {
            left_start: left.start,
            left_end: left.end,
            right_start: right.start,
            right_end: right.end,
            overlap_start: overlap.start,
            overlap_end: overlap.end,
        },
        RuntimeOwnershipFailure::InvalidSpan { span, kind } => {
            CoverageOwnershipFailure::InvalidSpan {
                start: span.start,
                end: span.end,
                span_kind: match kind {
                    RuntimeInvalidSpanKind::OutOfBounds => CoverageInvalidSpanKind::OutOfBounds,
                    RuntimeInvalidSpanKind::NonUtf8Boundary => {
                        CoverageInvalidSpanKind::NonUtf8Boundary
                    }
                },
            }
        }
        RuntimeOwnershipFailure::Synthetic { span } => CoverageOwnershipFailure::Synthetic {
            start: span.start,
            end: span.end,
        },
        RuntimeOwnershipFailure::ProvenancePlanMismatch {
            index,
            parsed,
            rendered,
        } => CoverageOwnershipFailure::ProvenancePlanMismatch {
            index: *index,
            parsed: parsed.clone(),
            rendered: rendered.clone(),
        },
        RuntimeOwnershipFailure::ByteMismatch {
            scope,
            expected,
            actual,
        } => CoverageOwnershipFailure::ByteMismatch {
            scope: match scope {
                RuntimeByteMismatchScope::WholeRender => CoverageByteMismatchScope::WholeRender,
                RuntimeByteMismatchScope::ClaimSlice { index } => {
                    CoverageByteMismatchScope::ClaimSlice { index: *index }
                }
            },
            expected: expected.clone(),
            actual: actual.clone(),
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(super) struct SelectedCoverage {
    rendered_text: String,
    ownership: CoverageOwnership,
    roundtrip_failure: Option<RoundtripFailure>,
}

impl SelectedCoverage {
    #[cfg(test)]
    fn rendered_text(&self) -> &str {
        &self.rendered_text
    }
    #[cfg(test)]
    const fn ownership(&self) -> &CoverageOwnership {
        &self.ownership
    }
    #[cfg(test)]
    const fn roundtrip_failure(&self) -> Option<&RoundtripFailure> {
        self.roundtrip_failure.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(super) struct CoverageRow {
    id: String,
    card_name: String,
    face_name: Option<String>,
    side: Option<String>,
    context_name: String,
    text: String,
    status: CoverageStatus,
    selected: Option<SelectedCoverage>,
    internal_failure_kind: Option<CoverageInternalFailureKind>,
    message: Option<String>,
}

impl CoverageRow {
    #[cfg(test)]
    const fn status(&self) -> CoverageStatus {
        self.status
    }
    #[cfg(test)]
    const fn selected(&self) -> Option<&SelectedCoverage> {
        self.selected.as_ref()
    }
    #[cfg(test)]
    const fn internal_failure_kind(&self) -> Option<CoverageInternalFailureKind> {
        self.internal_failure_kind
    }
    #[cfg(test)]
    fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    fn validate(&self) -> Result<(), CoverageValidationError> {
        match (self.status, &self.selected) {
            (
                CoverageStatus::SelectedCovered | CoverageStatus::SelectedUncovered,
                Some(selected),
            ) => {
                selected.ownership.validate(&self.id)?;
                let is_covered = selected.ownership.covered
                    && selected.ownership.failures.is_empty()
                    && selected.roundtrip_failure.is_none();
                let status_is_covered = self.status == CoverageStatus::SelectedCovered;
                if is_covered != status_is_covered {
                    return Err(CoverageValidationError::RowStatusEvidence {
                        id: self.id.clone(),
                    });
                }
            }
            (CoverageStatus::SelectedCovered | CoverageStatus::SelectedUncovered, None)
            | (
                CoverageStatus::ParseFailure
                | CoverageStatus::UnresolvedAmbiguity
                | CoverageStatus::InternalFailure,
                Some(_),
            ) => {
                return Err(CoverageValidationError::RowShape {
                    id: self.id.clone(),
                });
            }
            _ => {}
        }
        Ok(())
    }
}

pub(super) fn analysis_row<Ownership: SelectedOwnershipSource>(
    unit: &CorpusUnit,
    outcome: ParseAnalysisOutcome,
    ownership: Option<&Ownership>,
    message: Option<&str>,
) -> CoverageRow {
    let mut row = CoverageRow {
        id: unit.id().to_owned(),
        card_name: unit.card_name().to_owned(),
        face_name: unit.face_name().map(str::to_owned),
        side: unit.side().map(str::to_owned),
        context_name: unit.context_name().to_owned(),
        text: unit.text().to_owned(),
        status: CoverageStatus::InternalFailure,
        selected: None,
        internal_failure_kind: None,
        message: message.map(str::to_owned),
    };
    match outcome {
        ParseAnalysisOutcome::Selected => {
            let Some(ownership) = ownership else {
                row.internal_failure_kind = Some(CoverageInternalFailureKind::OwnershipInspection);
                row.message = Some("selected analysis has no ownership inspection".to_owned());
                return row;
            };
            let rendered_text = ownership.rendered_text().to_owned();
            let roundtrip_failure = (rendered_text != unit.text()).then(|| RoundtripFailure {
                expected: unit.text().to_owned(),
                actual: rendered_text.clone(),
            });
            let mapped = CoverageOwnership::from_source(ownership.summary(), ownership.failures());
            let covered =
                mapped.covered && mapped.failures.is_empty() && roundtrip_failure.is_none();
            row.status = if covered {
                CoverageStatus::SelectedCovered
            } else {
                CoverageStatus::SelectedUncovered
            };
            row.selected = Some(SelectedCoverage {
                rendered_text,
                ownership: mapped,
                roundtrip_failure,
            });
        }
        ParseAnalysisOutcome::ParseFailure => row.status = CoverageStatus::ParseFailure,
        ParseAnalysisOutcome::UnresolvedAmbiguity => {
            row.status = CoverageStatus::UnresolvedAmbiguity;
        }
        ParseAnalysisOutcome::InternalFailure(kind) => {
            row.internal_failure_kind = Some(match kind {
                RuntimeInternalFailureKind::ValidatedRootDidNotMaterialize => {
                    CoverageInternalFailureKind::ValidatedRootDidNotMaterialize
                }
                RuntimeInternalFailureKind::SelectionConfiguration => {
                    CoverageInternalFailureKind::SelectionConfiguration
                }
                RuntimeInternalFailureKind::OwnershipInspection => {
                    CoverageInternalFailureKind::OwnershipInspection
                }
            });
        }
    }
    row
}

fn runtime_analysis_row<V: Clone>(unit: &CorpusUnit, analysis: &ParseAnalysis<V>) -> CoverageRow {
    let outcome = analysis.outcome();
    let error = analysis
        .clone()
        .into_parse_result()
        .err()
        .map(|error| error.to_string());
    if outcome == ParseAnalysisOutcome::Selected && analysis.selected().is_none() {
        return CoverageRow {
            id: unit.id().to_owned(),
            card_name: unit.card_name().to_owned(),
            face_name: unit.face_name().map(str::to_owned),
            side: unit.side().map(str::to_owned),
            context_name: unit.context_name().to_owned(),
            text: unit.text().to_owned(),
            status: CoverageStatus::InternalFailure,
            selected: None,
            internal_failure_kind: Some(
                CoverageInternalFailureKind::ValidatedRootDidNotMaterialize,
            ),
            message: Some("selected analysis has no selected value".to_owned()),
        };
    }
    analysis_row(unit, outcome, analysis.ownership(), error.as_deref())
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(super) struct CoverageSummary {
    total_units: usize,
    selected_units: usize,
    covered_units: usize,
    selected_uncovered_units: usize,
    parse_failures: usize,
    unresolved_ties: usize,
    internal_failures: usize,
    roundtrip_mismatch_units: usize,
    ownership_failure_units: usize,
    claims: usize,
    claimed_bytes: usize,
    form_literal_claims: usize,
    form_literal_bytes: usize,
    vocab_claims: usize,
    vocab_bytes: usize,
    lexeme_claims: usize,
    lexeme_bytes: usize,
    codec_claims: usize,
    codec_bytes: usize,
    identity_claims: usize,
    identity_bytes: usize,
    gap_spans: usize,
    gap_bytes: usize,
    overlap_spans: usize,
    overlap_bytes: usize,
    synthetic_claims: usize,
    provenance_plan_mismatches: usize,
}

#[cfg(test)]
macro_rules! summary_getters {
    ($($field:ident),+ $(,)?) => {
        impl CoverageSummary {
            $(pub(super) const fn $field(&self) -> usize { self.$field })+
        }
    };
}

#[cfg(test)]
summary_getters!(
    total_units,
    selected_units,
    covered_units,
    selected_uncovered_units,
    parse_failures,
    unresolved_ties,
    internal_failures,
    roundtrip_mismatch_units,
    ownership_failure_units,
    claims,
    claimed_bytes,
);

impl CoverageSummary {
    fn from_rows(rows: &[CoverageRow]) -> Result<Self, CoverageValidationError> {
        let mut summary = Self::default();
        for row in rows {
            row.validate()?;
            checked_increment(&mut summary.total_units, "total_units")?;
            match row.status {
                CoverageStatus::SelectedCovered => {
                    checked_increment(&mut summary.selected_units, "selected_units")?;
                    checked_increment(&mut summary.covered_units, "covered_units")?;
                }
                CoverageStatus::SelectedUncovered => {
                    checked_increment(&mut summary.selected_units, "selected_units")?;
                    checked_increment(
                        &mut summary.selected_uncovered_units,
                        "selected_uncovered_units",
                    )?;
                }
                CoverageStatus::ParseFailure => {
                    checked_increment(&mut summary.parse_failures, "parse_failures")?;
                }
                CoverageStatus::UnresolvedAmbiguity => {
                    checked_increment(&mut summary.unresolved_ties, "unresolved_ties")?;
                }
                CoverageStatus::InternalFailure => {
                    checked_increment(&mut summary.internal_failures, "internal_failures")?;
                }
            }
            let Some(selected) = &row.selected else { continue };
            let ownership = &selected.ownership;
            if selected.roundtrip_failure.is_some() {
                checked_increment(
                    &mut summary.roundtrip_mismatch_units,
                    "roundtrip_mismatch_units",
                )?;
            }
            if !ownership.failures.is_empty() {
                checked_increment(
                    &mut summary.ownership_failure_units,
                    "ownership_failure_units",
                )?;
            }
            add_field(&mut summary.claims, ownership.claims, "claims")?;
            add_field(
                &mut summary.claimed_bytes,
                ownership.claimed_bytes,
                "claimed_bytes",
            )?;
            add_field(
                &mut summary.form_literal_claims,
                ownership.form_literal_claims,
                "form_literal_claims",
            )?;
            add_field(
                &mut summary.form_literal_bytes,
                ownership.form_literal_bytes,
                "form_literal_bytes",
            )?;
            add_field(
                &mut summary.vocab_claims,
                ownership.vocab_claims,
                "vocab_claims",
            )?;
            add_field(
                &mut summary.vocab_bytes,
                ownership.vocab_bytes,
                "vocab_bytes",
            )?;
            add_field(
                &mut summary.lexeme_claims,
                ownership.lexeme_claims,
                "lexeme_claims",
            )?;
            add_field(
                &mut summary.lexeme_bytes,
                ownership.lexeme_bytes,
                "lexeme_bytes",
            )?;
            add_field(
                &mut summary.codec_claims,
                ownership.codec_claims,
                "codec_claims",
            )?;
            add_field(
                &mut summary.codec_bytes,
                ownership.codec_bytes,
                "codec_bytes",
            )?;
            add_field(
                &mut summary.identity_claims,
                ownership.identity_claims,
                "identity_claims",
            )?;
            add_field(
                &mut summary.identity_bytes,
                ownership.identity_bytes,
                "identity_bytes",
            )?;
            add_field(&mut summary.gap_spans, ownership.gap_spans, "gap_spans")?;
            add_field(&mut summary.gap_bytes, ownership.gap_bytes, "gap_bytes")?;
            add_field(
                &mut summary.overlap_spans,
                ownership.overlap_spans,
                "overlap_spans",
            )?;
            add_field(
                &mut summary.overlap_bytes,
                ownership.overlap_bytes,
                "overlap_bytes",
            )?;
            add_field(
                &mut summary.synthetic_claims,
                ownership.synthetic_claims,
                "synthetic_claims",
            )?;
            add_field(
                &mut summary.provenance_plan_mismatches,
                ownership.provenance_plan_mismatches,
                "provenance_plan_mismatches",
            )?;
        }
        summary.validate()?;
        Ok(summary)
    }

    fn validate(&self) -> Result<(), CoverageValidationError> {
        let outcomes = checked_sum(
            "summary outcome partition",
            [
                self.selected_units,
                self.parse_failures,
                self.unresolved_ties,
                self.internal_failures,
            ],
        )?;
        if self.total_units != outcomes {
            return Err(CoverageValidationError::SummaryOutcomeEquation {
                total_units: self.total_units,
                outcomes,
            });
        }
        let selected = checked_sum(
            "summary selected partition",
            [self.covered_units, self.selected_uncovered_units],
        )?;
        if self.selected_units != selected {
            return Err(CoverageValidationError::SummarySelectedEquation {
                selected_units: self.selected_units,
                statuses: selected,
            });
        }
        let claims = checked_sum(
            "summary claim-kind sum",
            [
                self.form_literal_claims,
                self.vocab_claims,
                self.lexeme_claims,
                self.codec_claims,
                self.identity_claims,
            ],
        )?;
        if self.claims != claims {
            return Err(CoverageValidationError::SummaryClaimKindEquation {
                claims: self.claims,
                kind_claims: claims,
            });
        }
        let bytes = checked_sum(
            "summary byte-kind sum",
            [
                self.form_literal_bytes,
                self.vocab_bytes,
                self.lexeme_bytes,
                self.codec_bytes,
                self.identity_bytes,
            ],
        )?;
        if self.claimed_bytes != bytes {
            return Err(CoverageValidationError::SummaryByteKindEquation {
                claimed_bytes: self.claimed_bytes,
                kind_bytes: bytes,
            });
        }
        Ok(())
    }
}

fn checked_increment(
    value: &mut usize,
    field: &'static str,
) -> Result<(), CoverageValidationError> {
    add_field(value, 1, field)
}

fn add_field(
    value: &mut usize,
    added: usize,
    field: &'static str,
) -> Result<(), CoverageValidationError> {
    *value = value
        .checked_add(added)
        .ok_or(CoverageValidationError::ArithmeticOverflow { field })?;
    Ok(())
}

fn checked_sum<const N: usize>(
    field: &'static str,
    values: [usize; N],
) -> Result<usize, CoverageValidationError> {
    values.into_iter().try_fold(0usize, |sum, value| {
        sum.checked_add(value)
            .ok_or(CoverageValidationError::ArithmeticOverflow { field })
    })
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(super) struct CoverageReport {
    schema_version: u32,
    source_fingerprint: String,
    rows: Vec<CoverageRow>,
    summary: CoverageSummary,
}

impl CoverageReport {
    pub(super) fn try_new(
        source_fingerprint: String,
        rows: Vec<CoverageRow>,
    ) -> Result<Self, CoverageValidationError> {
        let summary = CoverageSummary::from_rows(&rows)?;
        Ok(Self {
            schema_version: REPORT_SCHEMA_VERSION,
            source_fingerprint,
            rows,
            summary,
        })
    }

    pub(super) fn source_fingerprint(&self) -> &str {
        &self.source_fingerprint
    }

    pub(super) const fn gate_failure_counts(&self) -> (usize, usize, usize) {
        (
            self.summary.selected_uncovered_units,
            self.summary.unresolved_ties,
            self.summary.internal_failures,
        )
    }

    pub(super) fn selected_covered_ids(&self) -> Result<Vec<String>, CoverageValidationError> {
        let mut ids = self
            .rows
            .iter()
            .filter(|row| row.status == CoverageStatus::SelectedCovered)
            .map(|row| row.id.clone())
            .collect::<Vec<_>>();
        ids.sort_unstable();
        if let Some(pair) = ids.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(CoverageValidationError::DuplicateRowIdentity {
                id: pair[0].clone(),
            });
        }
        Ok(ids)
    }

    #[cfg(test)]
    pub(super) fn rows(&self) -> &[CoverageRow] {
        &self.rows
    }

    #[cfg(test)]
    pub(super) const fn summary(&self) -> &CoverageSummary {
        &self.summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(super) enum CoverageValidationError {
    #[error("coverage report contains duplicate corpus identity {id}")]
    DuplicateRowIdentity { id: String },
    #[error("coverage row {id} does not have fields matching its status")]
    RowShape { id: String },
    #[error("coverage row {id} status contradicts its selected evidence")]
    RowStatusEvidence { id: String },
    #[error("coverage row {id} claim equation failed: {claims} != {kind_claims}")]
    RowClaimKindEquation {
        id: String,
        claims: usize,
        kind_claims: usize,
    },
    #[error("coverage row {id} byte equation failed: {claimed_bytes} != {kind_bytes}")]
    RowByteKindEquation {
        id: String,
        claimed_bytes: usize,
        kind_bytes: usize,
    },
    #[error("coverage summary outcome equation failed: {total_units} != {outcomes}")]
    SummaryOutcomeEquation { total_units: usize, outcomes: usize },
    #[error("coverage summary selected equation failed: {selected_units} != {statuses}")]
    SummarySelectedEquation {
        selected_units: usize,
        statuses: usize,
    },
    #[error("coverage summary claim equation failed: {claims} != {kind_claims}")]
    SummaryClaimKindEquation { claims: usize, kind_claims: usize },
    #[error("coverage summary byte equation failed: {claimed_bytes} != {kind_bytes}")]
    SummaryByteKindEquation {
        claimed_bytes: usize,
        kind_bytes: usize,
    },
    #[error("coverage arithmetic overflow while calculating {field}")]
    ArithmeticOverflow { field: &'static str },
}

pub(super) trait CoverageObserver {
    fn record(&mut self, event: String);
}

pub(super) struct NoopObserver;

impl CoverageObserver for NoopObserver {
    fn record(&mut self, _event: String) {}
}

pub(super) fn run(args: &CoverageArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let stderr = std::io::stderr();
    let mut diagnostics = stderr.lock();
    run_with_components(
        args,
        output,
        &mut diagnostics,
        &mut NoopObserver,
        || {
            Corpus::load(&args.corpus.data)
                .with_context(|| format!("loading corpus from {}", args.corpus.data.display()))
        },
        crate::english_v2::parser_from_builtin_v2,
        crate::english_v2::coverage_lock::apply,
    )
}

fn run_with_components<LoadCorpus, LoadParser, Gate>(
    args: &CoverageArgs,
    output: &mut dyn Write,
    diagnostics: &mut dyn Write,
    observer: &mut dyn CoverageObserver,
    load_corpus: LoadCorpus,
    load_parser: LoadParser,
    mut gate: Gate,
) -> anyhow::Result<()>
where
    LoadCorpus: FnOnce() -> anyhow::Result<Corpus>,
    LoadParser: FnOnce() -> anyhow::Result<Parser>,
    Gate: FnMut(&CoverageReport, &Path, CoverageLockMode, &mut dyn Write) -> anyhow::Result<()>,
{
    observer.record("load_corpus".to_owned());
    let corpus = load_corpus()?;
    observer.record("load_environment".to_owned());
    let parser = load_parser()?;
    let mut rows = Vec::with_capacity(corpus.units().len());
    for unit in corpus.units() {
        observer.record(format!("analyze_oracle_text:{}", unit.id()));
        let context = ParseContext::new(
            unit.context_name(),
            unit.is_legendary(),
            unit.context_onset(),
        )
        .expect("Corpus validates every stored parse context");
        let analysis = parser.analyze_oracle_text(unit.text(), &context);
        observer.record(format!("map_row:{}", unit.id()));
        rows.push(runtime_analysis_row(unit, &analysis));
    }
    observer.record("validate".to_owned());
    let report = CoverageReport::try_new(corpus.source_fingerprint().to_owned(), rows)?;
    observer.record("render".to_owned());
    render_report(&report, args.json, output)?;
    observer.record("flush".to_owned());
    output
        .flush()
        .context("flushing English-v2 coverage report")?;
    reject_internal_failures(&report)?;
    let mode = args.lock_mode();
    if mode != CoverageLockMode::None {
        observer.record("gate".to_owned());
        gate(&report, &args.lock, mode, diagnostics)?;
    }
    Ok(())
}

fn render_report(
    report: &CoverageReport,
    json: bool,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    if json {
        serde_json::to_writer_pretty(&mut *output, report)
            .context("writing English-v2 coverage report JSON")?;
        writeln!(output).context("writing English-v2 coverage report JSON terminator")?;
        return Ok(());
    }

    writeln!(
        output,
        "English v2 lexical coverage schema {REPORT_SCHEMA_VERSION}"
    )
    .context("writing English-v2 coverage report header")?;
    for row in &report.rows {
        let rendered = serde_json::to_string(row).context("serializing English-v2 coverage row")?;
        writeln!(output, "row {rendered}").context("writing English-v2 coverage report row")?;
    }
    let summary = serde_json::to_string(&report.summary)
        .context("serializing English-v2 coverage summary")?;
    writeln!(output, "summary {summary}").context("writing English-v2 coverage summary")?;
    Ok(())
}

fn reject_internal_failures(report: &CoverageReport) -> anyhow::Result<()> {
    let count = report.summary.internal_failures;
    if count == 0 {
        return Ok(());
    }
    bail!(
        "English-v2 coverage report contains {count} internal failure{}",
        if count == 1 { "" } else { "s" },
    )
}

#[cfg(test)]
impl CoverageOwnership {
    fn scalar_values_for_test(&self) -> [usize; 19] {
        [
            usize::from(self.covered),
            self.claims,
            self.claimed_bytes,
            self.form_literal_claims,
            self.form_literal_bytes,
            self.vocab_claims,
            self.vocab_bytes,
            self.lexeme_claims,
            self.lexeme_bytes,
            self.codec_claims,
            self.codec_bytes,
            self.identity_claims,
            self.identity_bytes,
            self.gap_spans,
            self.gap_bytes,
            self.overlap_spans,
            self.overlap_bytes,
            self.synthetic_claims,
            self.provenance_plan_mismatches,
        ]
    }

    fn set_claims_for_test(&mut self, value: usize) {
        self.claims = value;
    }
    fn set_claimed_bytes_for_test(&mut self, value: usize) {
        self.claimed_bytes = value;
    }
    fn set_form_literal_claims_for_test(&mut self, value: usize) {
        self.form_literal_claims = value;
    }
}

#[cfg(test)]
impl CoverageRow {
    fn selected_for_test(
        id: String,
        status: CoverageStatus,
        summary: &impl OwnershipSummarySource,
        roundtrip_failure: bool,
        ownership_failure: bool,
    ) -> Self {
        let failures = ownership_failure
            .then(all_test_failures)
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let ownership = CoverageOwnership::from_source(summary, &failures);
        Self {
            id,
            card_name: "Fixture".to_owned(),
            face_name: None,
            side: None,
            context_name: "Fixture".to_owned(),
            text: "expected".to_owned(),
            status,
            selected: Some(SelectedCoverage {
                rendered_text: if roundtrip_failure { "actual" } else { "expected" }.to_owned(),
                ownership,
                roundtrip_failure: roundtrip_failure.then(|| RoundtripFailure {
                    expected: "expected".to_owned(),
                    actual: "actual".to_owned(),
                }),
            }),
            internal_failure_kind: None,
            message: None,
        }
    }

    fn outcome_for_test(id: String, status: CoverageStatus) -> Self {
        Self {
            id,
            card_name: "Fixture".to_owned(),
            face_name: None,
            side: None,
            context_name: "Fixture".to_owned(),
            text: "fixture".to_owned(),
            status,
            selected: None,
            internal_failure_kind: (status == CoverageStatus::InternalFailure)
                .then_some(CoverageInternalFailureKind::OwnershipInspection),
            message: Some("fixture outcome".to_owned()),
        }
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "test fixture pins all stored corpus metadata"
    )]
    fn selected_with_metadata_for_test(
        id: String,
        card_name: &str,
        face_name: Option<&str>,
        side: Option<&str>,
        context_name: &str,
        text: &str,
        status: CoverageStatus,
        summary: &impl OwnershipSummarySource,
        failures: &[RuntimeOwnershipFailure],
        rendered: &str,
    ) -> Self {
        let roundtrip_failure = (text != rendered).then(|| RoundtripFailure {
            expected: text.to_owned(),
            actual: rendered.to_owned(),
        });
        Self {
            id,
            card_name: card_name.to_owned(),
            face_name: face_name.map(str::to_owned),
            side: side.map(str::to_owned),
            context_name: context_name.to_owned(),
            text: text.to_owned(),
            status,
            selected: Some(SelectedCoverage {
                rendered_text: rendered.to_owned(),
                ownership: CoverageOwnership::from_source(summary, failures),
                roundtrip_failure,
            }),
            internal_failure_kind: None,
            message: None,
        }
    }

    fn selected_mut_for_test(&mut self) -> &mut SelectedCoverage {
        self.selected.as_mut().expect("selected fixture")
    }
}

#[cfg(test)]
impl SelectedCoverage {
    fn ownership_mut_for_test(&mut self) -> &mut CoverageOwnership {
        &mut self.ownership
    }
}

#[cfg(test)]
impl CoverageSummary {
    fn from_rows_for_test(rows: &[CoverageRow]) -> Result<Self, CoverageValidationError> {
        Self::from_rows(rows)
    }
    fn validate_for_test(&self) -> Result<(), CoverageValidationError> {
        self.validate()
    }
    fn set_total_units_for_test(&mut self, value: usize) {
        self.total_units = value;
    }
    fn set_selected_units_for_test(&mut self, value: usize) {
        self.selected_units = value;
    }
}

#[cfg(test)]
impl CoverageReport {
    pub(super) fn for_gate_test(
        source_fingerprint: String,
        covered: Vec<String>,
        selected_uncovered: usize,
        parse_failures: usize,
        unresolved: usize,
        internal: usize,
    ) -> Self {
        let empty_ownership = || CoverageOwnership {
            covered: true,
            claims: 0,
            claimed_bytes: 0,
            form_literal_claims: 0,
            form_literal_bytes: 0,
            vocab_claims: 0,
            vocab_bytes: 0,
            lexeme_claims: 0,
            lexeme_bytes: 0,
            codec_claims: 0,
            codec_bytes: 0,
            identity_claims: 0,
            identity_bytes: 0,
            gap_spans: 0,
            gap_bytes: 0,
            overlap_spans: 0,
            overlap_bytes: 0,
            synthetic_claims: 0,
            provenance_plan_mismatches: 0,
            failures: Vec::new(),
        };
        let mut rows = covered
            .into_iter()
            .map(|id| CoverageRow {
                id,
                card_name: "Covered".to_owned(),
                face_name: None,
                side: None,
                context_name: "Covered".to_owned(),
                text: String::new(),
                status: CoverageStatus::SelectedCovered,
                selected: Some(SelectedCoverage {
                    rendered_text: String::new(),
                    ownership: empty_ownership(),
                    roundtrip_failure: None,
                }),
                internal_failure_kind: None,
                message: None,
            })
            .collect::<Vec<_>>();
        let mut next_id = 0x1000usize;
        for _ in 0..selected_uncovered {
            let mut ownership = empty_ownership();
            ownership.covered = false;
            ownership
                .failures
                .push(CoverageOwnershipFailure::Synthetic { start: 0, end: 0 });
            rows.push(CoverageRow {
                id: format!("{next_id:064x}"),
                card_name: "Uncovered".to_owned(),
                face_name: None,
                side: None,
                context_name: "Uncovered".to_owned(),
                text: String::new(),
                status: CoverageStatus::SelectedUncovered,
                selected: Some(SelectedCoverage {
                    rendered_text: String::new(),
                    ownership,
                    roundtrip_failure: None,
                }),
                internal_failure_kind: None,
                message: None,
            });
            next_id += 1;
        }
        for _ in 0..parse_failures {
            rows.push(CoverageRow::outcome_for_test(
                format!("{next_id:064x}"),
                CoverageStatus::ParseFailure,
            ));
            next_id += 1;
        }
        for _ in 0..unresolved {
            rows.push(CoverageRow::outcome_for_test(
                format!("{next_id:064x}"),
                CoverageStatus::UnresolvedAmbiguity,
            ));
            next_id += 1;
        }
        for _ in 0..internal {
            rows.push(CoverageRow::outcome_for_test(
                format!("{next_id:064x}"),
                CoverageStatus::InternalFailure,
            ));
            next_id += 1;
        }
        Self::try_new(source_fingerprint, rows).unwrap()
    }
}

#[cfg(test)]
fn all_test_failures() -> Vec<RuntimeOwnershipFailure> {
    vec![RuntimeOwnershipFailure::Synthetic {
        span: deckmaste_english_v2::parser::TextSpan { start: 0, end: 0 },
    }]
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::io;
    use std::io::Write;
    use std::path::Path;
    use std::rc::Rc;

    use deckmaste_english_v2::context::ParseContext;
    use deckmaste_english_v2::parser::ByteMismatchScope;
    use deckmaste_english_v2::parser::InternalFailureKind;
    use deckmaste_english_v2::parser::InvalidSpanKind;
    use deckmaste_english_v2::parser::OwnershipFailure;
    use deckmaste_english_v2::parser::ParseAnalysisOutcome;
    use deckmaste_english_v2::parser::ParserEntryPoint;
    use deckmaste_english_v2::parser::TextSpan;
    use deckmaste_english_v2::parser::reset_parser_entry_calls_for_test;
    use deckmaste_english_v2::parser::take_parser_entry_calls_for_test;
    use deckmaste_english_v2::parser::with_forced_ownership_inspection_failure_for_test;
    use macro_ron::v2::Onset;

    use super::CoverageByteMismatchScope;
    use super::CoverageInternalFailureKind;
    use super::CoverageInvalidSpanKind;
    use super::CoverageLockMode;
    use super::CoverageObserver;
    use super::CoverageOwnership;
    use super::CoverageOwnershipFailure;
    use super::CoverageReport;
    use super::CoverageRow;
    use super::CoverageStatus;
    use super::CoverageSummary;
    use super::CoverageValidationError;
    use super::NoopObserver;
    use super::OwnershipSummarySource;
    use super::SelectedOwnershipSource;
    use super::analysis_row;
    use super::render_report;
    use super::run_with_components;
    use crate::english_v2::CorpusArgs;
    use crate::english_v2::CoverageArgs;
    use crate::english_v2::corpus::Corpus;
    use crate::english_v2::corpus::CorpusUnit;

    const PLAN06_COVERED_IDS: &str = include_str!("plan06_covered_ids.txt");
    const PLAN07_TARGETS: &str = include_str!("plan07_targets.tsv");
    const PLAN08_TARGETS: &str = include_str!("plan08_targets.tsv");

    fn sha256_hex(bytes: &[u8]) -> String {
        use std::fmt::Write as _;

        use sha2::Digest as _;

        sha2::Sha256::digest(bytes)
            .iter()
            .fold(String::new(), |mut hexadecimal, byte| {
                write!(&mut hexadecimal, "{byte:02x}").expect("writing to String cannot fail");
                hexadecimal
            })
    }

    fn id(digit: char) -> String {
        digit.to_string().repeat(64)
    }

    fn assert_full_production_selected_covered_baseline(
        report: &CoverageReport,
        baseline_ids: &BTreeSet<&str>,
        target_ids: &BTreeSet<&str>,
    ) {
        let lock: serde_json::Value =
            serde_json::from_str(include_str!("../../../../english-v2-coverage.lock"))
                .expect("production coverage lock parses");
        let covered_ids = lock["covered"]
            .as_array()
            .expect("schema-2 lock has covered IDs")
            .iter()
            .map(|id| id.as_str().expect("covered ID is a string"))
            .collect::<BTreeSet<_>>();
        let production_selected_covered_ids = report
            .selected_covered_ids()
            .expect("full production selected-covered IDs are unique");
        assert_eq!(covered_ids.len(), 735);
        let production_selected_covered_ids = production_selected_covered_ids
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        assert_eq!(production_selected_covered_ids.len(), 3_413);
        assert!(covered_ids.is_subset(&production_selected_covered_ids));
        assert!(baseline_ids.is_subset(&covered_ids));
        assert!(target_ids.is_subset(&covered_ids));
    }

    #[test]
    fn plan07_frozen_manifest_and_full_lock_are_exact_on_production_data() {
        const EXPECTED_COUNTS: [(&str, usize); 13] = [
            ("damage.any-target", 25),
            ("damage.binary-target-coordination", 8),
            ("damage.each-bare-head", 7),
            ("damage.one-prenominal-modifier", 2),
            ("destroy.all-bare-plural", 16),
            ("destroy.all-binary-coordination", 4),
            ("destroy.all-one-modifier", 12),
            ("destroy.binary-target-coordination", 17),
            ("destroy.fixed-or-x-target-count", 7),
            ("destroy.one-prenominal-modifier", 31),
            ("destroy.oxford-target-list", 5),
            ("destroy.simple-power-toughness-comparison", 9),
            ("destroy.target-bare-carrier", 2),
        ];

        assert_eq!(
            sha256_hex(PLAN07_TARGETS.as_bytes()),
            "3535a10fd5bcecc86dee14c1df28d4f66478f724b0dc465945063a6efb01723d",
        );
        let manifest_rows = PLAN07_TARGETS
            .lines()
            .skip(7)
            .filter(|line| !line.is_empty())
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(manifest_rows.len(), 145);

        let mut category_counts = BTreeMap::new();
        let target_ids = manifest_rows
            .iter()
            .map(|fields| {
                *category_counts.entry(fields[0]).or_insert(0usize) += 1;
                fields[1]
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(target_ids.len(), 145);
        assert_eq!(
            category_counts.into_iter().collect::<Vec<_>>(),
            EXPECTED_COUNTS,
        );

        let baseline_ids = PLAN06_COVERED_IDS
            .lines()
            .skip(6)
            .filter(|line| !line.is_empty())
            .collect::<BTreeSet<_>>();
        assert_eq!(baseline_ids.len(), 412);
        assert!(baseline_ids.is_disjoint(&target_ids));

        let data =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/mtgjson/AtomicCards.json");
        let corpus = Corpus::load(&data).expect("production MTGJSON corpus loads");
        assert_eq!(
            corpus.source_fingerprint(),
            "e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd",
        );
        let plus_two_mace = corpus
            .units()
            .iter()
            .find(|unit| unit.context_name() == "+2 Mace")
            .expect("production corpus contains +2 Mace");
        assert!(!plus_two_mace.is_legendary());
        assert_eq!(
            plus_two_mace.context_onset(),
            macro_ron::v2::Onset::Consonant
        );
        let aang = corpus
            .units()
            .iter()
            .find(|unit| unit.context_name() == "Aang, A Lot to Learn")
            .expect("production corpus contains the legendary Aang witness");
        assert!(aang.is_legendary());
        assert_eq!(aang.context_onset(), macro_ron::v2::Onset::Vowel);
        let parser = crate::english_v2::parser_from_builtin_v2()
            .expect("production English-v2 parser environment loads");
        let coverage_rows = corpus
            .units()
            .iter()
            .map(|unit| {
                let context = ParseContext::new(
                    unit.context_name(),
                    unit.is_legendary(),
                    unit.context_onset(),
                )
                .unwrap_or_else(|| panic!("production context is valid for {}", unit.id()));
                let analysis = parser.analyze_oracle_text(unit.text(), &context);
                super::runtime_analysis_row(unit, &analysis)
            })
            .collect::<Vec<_>>();
        let report = CoverageReport::try_new(corpus.source_fingerprint().to_owned(), coverage_rows)
            .expect("full production coverage evidence is internally consistent");
        let rows_by_id = report
            .rows()
            .iter()
            .map(|row| (row.id.as_str(), row))
            .collect::<BTreeMap<_, _>>();

        for fields in manifest_rows {
            let [_, id, card_name, face_name, side, context_name, text] = fields.as_slice() else {
                panic!("frozen manifest row has seven columns: {fields:?}");
            };
            let unit = corpus
                .units()
                .iter()
                .find(|unit| unit.id() == *id)
                .unwrap_or_else(|| panic!("production corpus contains frozen ID {id}"));
            assert_eq!(unit.card_name(), *card_name, "{id}");
            assert_eq!(unit.face_name().unwrap_or_default(), *face_name, "{id}");
            assert_eq!(unit.side().unwrap_or_default(), *side, "{id}");
            assert_eq!(unit.context_name(), *context_name, "{id}");
            assert_eq!(unit.text(), *text, "{id}");

            let row = rows_by_id
                .get(id)
                .copied()
                .unwrap_or_else(|| panic!("full production report contains {id}"));
            assert_eq!(row.status(), CoverageStatus::SelectedCovered, "{id}");
            let selected = row
                .selected()
                .unwrap_or_else(|| panic!("selected evidence exists for {id}"));
            assert_eq!(selected.rendered_text(), unit.text(), "{id}");
            assert!(selected.ownership().covered, "{id}");
            assert!(selected.ownership().failures().is_empty(), "{id}");
            assert!(selected.roundtrip_failure().is_none(), "{id}");
        }

        assert_eq!(report.rows().len(), 32_641);
        assert_eq!(report.summary().total_units(), 32_641);
        assert_eq!(report.summary().selected_units(), 3_413);
        assert_eq!(report.summary().covered_units(), 3_413);
        assert_eq!(report.summary().selected_uncovered_units(), 0);
        assert_eq!(report.summary().parse_failures(), 29_228);
        assert_eq!(report.summary().unresolved_ties(), 0);
        assert_eq!(report.summary().internal_failures(), 0);
        assert_eq!(report.summary().roundtrip_mismatch_units(), 0);
        assert_eq!(report.summary().ownership_failure_units(), 0);

        assert_full_production_selected_covered_baseline(&report, &baseline_ids, &target_ids);
    }

    #[allow(
        clippy::too_many_lines,
        reason = "the frozen Plan 08 target and complete-lock verifier is deliberately literal"
    )]
    #[test]
    fn plan08_frozen_manifest_and_full_lock_are_exact_on_production_data() {
        const EXPECTED_HEADERS: [&str; 10] = [
            "# English v2 Plan 08 frozen corpus target manifest",
            "# manifest_schema=1",
            "# source_fingerprint=e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd",
            "# candidate_results_sha256=918d7eaf1d0d66342b308b7290a2ab19621325749c59de9a4341a6a56bfbeac4",
            "# baseline_covered=608",
            "# baseline_ids_sha256=35ea73406725742d51b92df60bb07a261ed7570d6767caf59306a88785de87c2",
            "# baseline_status=parse_failure",
            "# targets=127",
            "# family_counts=ability.activated:56,ability.plain-modal:28,ability.triggered:12,clause.coordination:23,finite.auxiliary:1,nominal.demonstrative-possessive:7",
            r"# columns=family\tid\tcard_name_json\tface_name_json\tside_json\tcontext_name_json\tis_legendary\tcontext_onset\toracle_text_json",
        ];
        const EXPECTED_FAMILIES: [(&str, usize); 6] = [
            ("ability.activated", 56),
            ("ability.plain-modal", 28),
            ("ability.triggered", 12),
            ("clause.coordination", 23),
            ("finite.auxiliary", 1),
            ("nominal.demonstrative-possessive", 7),
        ];
        const EXPECTED_OUT_OF_POOL: [(&str, usize); 5] = [
            ("ability.plain-modal", 28),
            ("ability.triggered", 1),
            ("clause.coordination", 23),
            ("finite.auxiliary", 1),
            ("nominal.demonstrative-possessive", 7),
        ];

        fn decode(value: &str) -> String {
            serde_json::from_str(value).expect("manifest JSON string is valid")
        }

        assert_eq!(
            sha256_hex(PLAN08_TARGETS.as_bytes()),
            "4f6f749bcb93acb534b830a5393672a41a31ccc9c7a2deb4df7a8fb28ea3275e",
        );
        assert_eq!(
            PLAN08_TARGETS
                .lines()
                .take(EXPECTED_HEADERS.len())
                .collect::<Vec<_>>(),
            EXPECTED_HEADERS,
        );
        assert!(PLAN08_TARGETS.ends_with('\n'));
        let manifest_rows = PLAN08_TARGETS
            .lines()
            .skip(EXPECTED_HEADERS.len())
            .map(|line| line.split('\t').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(manifest_rows.len(), 127);

        let mut previous_id = None;
        let mut family_counts = BTreeMap::new();
        let target_ids = manifest_rows
            .iter()
            .map(|fields| {
                assert_eq!(fields.len(), 9, "target row has complete metadata");
                let id = fields[1];
                assert!(previous_id.is_none_or(|previous| previous < id));
                previous_id = Some(id);
                *family_counts.entry(fields[0]).or_insert(0usize) += 1;
                id
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(target_ids.len(), 127);
        assert_eq!(
            family_counts.into_iter().collect::<Vec<_>>(),
            EXPECTED_FAMILIES,
        );

        let lock: serde_json::Value =
            serde_json::from_str(include_str!("../../../../english-v2-coverage.lock"))
                .expect("production lock parses");
        let covered_ids = lock["covered"]
            .as_array()
            .expect("schema-2 lock has covered IDs")
            .iter()
            .map(|id| id.as_str().expect("covered ID is a string"))
            .collect::<BTreeSet<_>>();
        let baseline_ids = covered_ids
            .difference(&target_ids)
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(baseline_ids.len(), 608);
        let baseline_membership = baseline_ids
            .iter()
            .fold(String::new(), |mut membership, id| {
                membership.push_str(id);
                membership.push('\n');
                membership
            });
        assert_eq!(
            sha256_hex(baseline_membership.as_bytes()),
            "35ea73406725742d51b92df60bb07a261ed7570d6767caf59306a88785de87c2",
        );
        assert!(baseline_ids.is_disjoint(&target_ids));
        assert_eq!(
            baseline_ids
                .union(&target_ids)
                .copied()
                .collect::<BTreeSet<_>>(),
            covered_ids,
        );

        let candidate_pool_ids = include_str!("plan08_candidate_pool.tsv")
            .lines()
            .skip(9)
            .map(|line| line.split('\t').nth(1).expect("candidate ID"))
            .collect::<BTreeSet<_>>();
        let candidate_selected_ids = include_str!("plan08_candidate_results.tsv")
            .lines()
            .skip(10)
            .filter_map(|line| {
                let fields = line.split('\t').collect::<Vec<_>>();
                (fields[0] == "plan08-selected").then_some(fields[3])
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(candidate_selected_ids.len(), 67);
        assert_eq!(
            target_ids
                .intersection(&candidate_pool_ids)
                .copied()
                .collect::<BTreeSet<_>>(),
            candidate_selected_ids,
        );
        let out_of_pool_ids = target_ids
            .difference(&candidate_pool_ids)
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(out_of_pool_ids.len(), 60);
        let mut out_of_pool_counts = BTreeMap::new();
        for fields in &manifest_rows {
            if out_of_pool_ids.contains(fields[1]) {
                *out_of_pool_counts.entry(fields[0]).or_insert(0usize) += 1;
            }
        }
        assert_eq!(
            out_of_pool_counts.into_iter().collect::<Vec<_>>(),
            EXPECTED_OUT_OF_POOL,
        );

        let data =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/mtgjson/AtomicCards.json");
        let corpus = Corpus::load(&data).expect("production MTGJSON corpus loads");
        assert_eq!(
            corpus.source_fingerprint(),
            "e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd",
        );
        let units = corpus
            .units()
            .iter()
            .map(|unit| (unit.id(), unit))
            .collect::<BTreeMap<_, _>>();
        let parser = crate::english_v2::parser_from_builtin_v2()
            .expect("production English-v2 parser environment loads");
        let coverage_rows = corpus
            .units()
            .iter()
            .map(|unit| {
                let context = ParseContext::new(
                    unit.context_name(),
                    unit.is_legendary(),
                    unit.context_onset(),
                )
                .unwrap_or_else(|| panic!("production context is valid for {}", unit.id()));
                let analysis = parser.analyze_oracle_text(unit.text(), &context);
                super::runtime_analysis_row(unit, &analysis)
            })
            .collect::<Vec<_>>();
        let report = CoverageReport::try_new(corpus.source_fingerprint().to_owned(), coverage_rows)
            .expect("full production coverage evidence is internally consistent");
        let rows_by_id = report
            .rows()
            .iter()
            .map(|row| (row.id.as_str(), row))
            .collect::<BTreeMap<_, _>>();

        for fields in manifest_rows {
            let [
                family,
                id,
                card_name,
                face_name,
                side,
                context_name,
                legendary,
                onset,
                text,
            ] = fields.as_slice()
            else {
                unreachable!("nine-column assertion already passed")
            };
            let unit = units
                .get(id)
                .copied()
                .unwrap_or_else(|| panic!("production corpus contains target {id}"));
            assert_eq!(unit.card_name(), decode(card_name), "{id}");
            assert_eq!(
                unit.face_name().unwrap_or_default(),
                decode(face_name),
                "{id}"
            );
            assert_eq!(unit.side().unwrap_or_default(), decode(side), "{id}");
            assert_eq!(unit.context_name(), decode(context_name), "{id}");
            assert_eq!(unit.is_legendary().to_string(), *legendary, "{id}");
            assert_eq!(
                match unit.context_onset() {
                    Onset::Vowel => "vowel",
                    Onset::Consonant => "consonant",
                },
                *onset,
                "{id}",
            );
            assert_eq!(unit.text(), decode(text), "{id}");
            let row = rows_by_id
                .get(id)
                .copied()
                .unwrap_or_else(|| panic!("coverage report contains target {id}"));
            assert_eq!(row.status(), CoverageStatus::SelectedCovered, "{id}");
            let selected = row.selected().expect("selected target evidence");
            assert_eq!(selected.rendered_text(), unit.text(), "{id}");
            assert!(selected.ownership().covered, "{id}");
            assert!(selected.ownership().failures().is_empty(), "{id}");
            assert!(selected.roundtrip_failure().is_none(), "{id}");

            let context = ParseContext::new(
                unit.context_name(),
                unit.is_legendary(),
                unit.context_onset(),
            )
            .expect("target context is valid");
            let analysis = parser.analyze_oracle_text(unit.text(), &context);
            let decision = analysis.decision().expect("target selection decision");
            assert_eq!(
                decision.resolution(),
                deckmaste_english_v2::parser::SelectionResolution::Unique,
                "{id}",
            );
            assert!(decision.exception_uses().is_empty(), "{id}");
            let ordinal = decision.selected().expect("selected target ordinal");
            let path = decision
                .candidates()
                .iter()
                .find(|candidate| candidate.ordinal() == ordinal)
                .expect("selected target candidate")
                .construction_path();
            let explained = match *family {
                "ability.activated" => path.iter().any(|name| name == "AbilityActivated"),
                "ability.triggered" => path.iter().any(|name| name == "AbilityTriggered"),
                "ability.plain-modal" => path.iter().any(|name| name == "AbilityBodyPlainModal"),
                "clause.coordination" => path
                    .iter()
                    .any(|name| name.starts_with("ClauseCoordination")),
                "finite.auxiliary" => path
                    .iter()
                    .any(|name| name == "FiniteClauseAuxiliaryFiniteClause"),
                "nominal.demonstrative-possessive" => path
                    .iter()
                    .any(|name| name == "UnqualifiedReferenceDemonstrativePossessiveReference"),
                other => panic!("unexplained Plan 08 family {other}: {id}"),
            };
            assert!(explained, "target has its named Plan 08 family: {id}");
        }

        assert_eq!(report.summary().total_units(), 32_641);
        assert_eq!(report.summary().selected_units(), 3_413);
        assert_eq!(report.summary().covered_units(), 3_413);
        assert_eq!(report.summary().selected_uncovered_units(), 0);
        assert_eq!(report.summary().parse_failures(), 29_228);
        assert_eq!(report.summary().unresolved_ties(), 0);
        assert_eq!(report.summary().internal_failures(), 0);
        assert_eq!(report.summary().roundtrip_mismatch_units(), 0);
        assert_eq!(report.summary().ownership_failure_units(), 0);
        assert_full_production_selected_covered_baseline(&report, &baseline_ids, &target_ids);
    }

    #[derive(Clone)]
    struct FixtureSummary {
        covered: bool,
        values: [usize; 18],
    }

    impl OwnershipSummarySource for FixtureSummary {
        fn covered(&self) -> bool {
            self.covered
        }
        fn claims(&self) -> usize {
            self.values[0]
        }
        fn claimed_bytes(&self) -> usize {
            self.values[1]
        }
        fn form_literal_claims(&self) -> usize {
            self.values[2]
        }
        fn form_literal_bytes(&self) -> usize {
            self.values[3]
        }
        fn vocab_claims(&self) -> usize {
            self.values[4]
        }
        fn vocab_bytes(&self) -> usize {
            self.values[5]
        }
        fn lexeme_claims(&self) -> usize {
            self.values[6]
        }
        fn lexeme_bytes(&self) -> usize {
            self.values[7]
        }
        fn codec_claims(&self) -> usize {
            self.values[8]
        }
        fn codec_bytes(&self) -> usize {
            self.values[9]
        }
        fn identity_claims(&self) -> usize {
            self.values[10]
        }
        fn identity_bytes(&self) -> usize {
            self.values[11]
        }
        fn gap_spans(&self) -> usize {
            self.values[12]
        }
        fn gap_bytes(&self) -> usize {
            self.values[13]
        }
        fn overlap_spans(&self) -> usize {
            self.values[14]
        }
        fn overlap_bytes(&self) -> usize {
            self.values[15]
        }
        fn synthetic_claims(&self) -> usize {
            self.values[16]
        }
        fn provenance_plan_mismatches(&self) -> usize {
            self.values[17]
        }
    }

    #[derive(Clone)]
    struct FixtureOwnership {
        rendered: String,
        summary: FixtureSummary,
        failures: Vec<OwnershipFailure>,
    }

    impl SelectedOwnershipSource for FixtureOwnership {
        type Summary = FixtureSummary;

        fn rendered_text(&self) -> &str {
            &self.rendered
        }
        fn summary(&self) -> &Self::Summary {
            &self.summary
        }
        fn failures(&self) -> &[OwnershipFailure] {
            &self.failures
        }
    }

    fn valid_summary(covered: bool) -> FixtureSummary {
        FixtureSummary {
            covered,
            values: [
                15, 150, 1, 10, 2, 20, 3, 30, 4, 40, 5, 50, 6, 60, 7, 70, 8, 9,
            ],
        }
    }

    fn all_failures() -> Vec<OwnershipFailure> {
        vec![
            OwnershipFailure::Gap {
                span: TextSpan { start: 1, end: 2 },
            },
            OwnershipFailure::Overlap {
                left: TextSpan { start: 3, end: 8 },
                right: TextSpan { start: 6, end: 9 },
                overlap: TextSpan { start: 6, end: 8 },
            },
            OwnershipFailure::InvalidSpan {
                span: TextSpan { start: 10, end: 9 },
                kind: InvalidSpanKind::OutOfBounds,
            },
            OwnershipFailure::InvalidSpan {
                span: TextSpan { start: 1, end: 2 },
                kind: InvalidSpanKind::NonUtf8Boundary,
            },
            OwnershipFailure::Synthetic {
                span: TextSpan { start: 4, end: 4 },
            },
            OwnershipFailure::ProvenancePlanMismatch {
                index: 5,
                parsed: "parsed\n\t\\\"é".to_owned(),
                rendered: "rendered\r\\\"ß".to_owned(),
            },
            OwnershipFailure::ByteMismatch {
                scope: ByteMismatchScope::WholeRender,
                expected: "expected\nwhole".to_owned(),
                actual: "actual\rwhole".to_owned(),
            },
            OwnershipFailure::ByteMismatch {
                scope: ByteMismatchScope::ClaimSlice { index: 7 },
                expected: "expected\tclaim".to_owned(),
                actual: "actual\\claim".to_owned(),
            },
        ]
    }

    fn unit(name: &str, text: &str) -> CorpusUnit {
        CorpusUnit::for_test(name, text)
    }

    #[test]
    fn row_mapping_keeps_all_statuses_internal_kinds_and_typed_failure_fields() {
        let selected = unit("Selected", "stored\ntext");
        let uncovered = analysis_row(
            &selected,
            ParseAnalysisOutcome::Selected,
            Some(&FixtureOwnership {
                rendered: "rendered\rtext".to_owned(),
                summary: valid_summary(false),
                failures: all_failures(),
            }),
            None,
        );
        assert_eq!(uncovered.status(), CoverageStatus::SelectedUncovered);
        assert_eq!(
            uncovered.selected().unwrap().rendered_text(),
            "rendered\rtext"
        );
        assert_eq!(
            uncovered.selected().unwrap().ownership().failures().len(),
            8
        );
        assert_eq!(
            uncovered.selected().unwrap().roundtrip_failure().unwrap(),
            &super::RoundtripFailure {
                expected: "stored\ntext".to_owned(),
                actual: "rendered\rtext".to_owned(),
            }
        );
        assert_eq!(
            uncovered.selected().unwrap().ownership().failures(),
            &[
                CoverageOwnershipFailure::Gap { start: 1, end: 2 },
                CoverageOwnershipFailure::Overlap {
                    left_start: 3,
                    left_end: 8,
                    right_start: 6,
                    right_end: 9,
                    overlap_start: 6,
                    overlap_end: 8,
                },
                CoverageOwnershipFailure::InvalidSpan {
                    start: 10,
                    end: 9,
                    span_kind: CoverageInvalidSpanKind::OutOfBounds,
                },
                CoverageOwnershipFailure::InvalidSpan {
                    start: 1,
                    end: 2,
                    span_kind: CoverageInvalidSpanKind::NonUtf8Boundary,
                },
                CoverageOwnershipFailure::Synthetic { start: 4, end: 4 },
                CoverageOwnershipFailure::ProvenancePlanMismatch {
                    index: 5,
                    parsed: "parsed\n\t\\\"é".to_owned(),
                    rendered: "rendered\r\\\"ß".to_owned(),
                },
                CoverageOwnershipFailure::ByteMismatch {
                    scope: CoverageByteMismatchScope::WholeRender,
                    expected: "expected\nwhole".to_owned(),
                    actual: "actual\rwhole".to_owned(),
                },
                CoverageOwnershipFailure::ByteMismatch {
                    scope: CoverageByteMismatchScope::ClaimSlice { index: 7 },
                    expected: "expected\tclaim".to_owned(),
                    actual: "actual\\claim".to_owned(),
                },
            ]
        );

        let covered = analysis_row(
            &unit("Covered", "same"),
            ParseAnalysisOutcome::Selected,
            Some(&FixtureOwnership {
                rendered: "same".to_owned(),
                summary: valid_summary(true),
                failures: vec![],
            }),
            None,
        );
        assert_eq!(covered.status(), CoverageStatus::SelectedCovered);

        for (outcome, status, kind, message) in [
            (
                ParseAnalysisOutcome::ParseFailure,
                CoverageStatus::ParseFailure,
                None,
                "parse\nfailed",
            ),
            (
                ParseAnalysisOutcome::UnresolvedAmbiguity,
                CoverageStatus::UnresolvedAmbiguity,
                None,
                "tie\rremains",
            ),
            (
                ParseAnalysisOutcome::InternalFailure(
                    InternalFailureKind::ValidatedRootDidNotMaterialize,
                ),
                CoverageStatus::InternalFailure,
                Some(CoverageInternalFailureKind::ValidatedRootDidNotMaterialize),
                "root missing",
            ),
            (
                ParseAnalysisOutcome::InternalFailure(InternalFailureKind::SelectionConfiguration),
                CoverageStatus::InternalFailure,
                Some(CoverageInternalFailureKind::SelectionConfiguration),
                "selection bad",
            ),
            (
                ParseAnalysisOutcome::InternalFailure(InternalFailureKind::OwnershipInspection),
                CoverageStatus::InternalFailure,
                Some(CoverageInternalFailureKind::OwnershipInspection),
                "ownership bad",
            ),
        ] {
            let row = analysis_row::<FixtureOwnership>(
                &unit("Failure", "text"),
                outcome,
                None,
                Some(message),
            );
            assert_eq!(row.status(), status);
            assert_eq!(row.internal_failure_kind(), kind);
            assert_eq!(row.message(), Some(message));
        }

        let missing = analysis_row::<FixtureOwnership>(
            &unit("Missing ownership", "text"),
            ParseAnalysisOutcome::Selected,
            None,
            None,
        );
        assert_eq!(missing.status(), CoverageStatus::InternalFailure);
        assert_eq!(
            missing.internal_failure_kind(),
            Some(CoverageInternalFailureKind::OwnershipInspection)
        );
    }

    #[test]
    fn json_wire_uses_literal_status_and_internal_kind_spellings() {
        let selected_covered = CoverageRow::selected_for_test(
            id('1'),
            CoverageStatus::SelectedCovered,
            &valid_summary(true),
            false,
            false,
        );
        let selected_uncovered = CoverageRow::selected_for_test(
            id('2'),
            CoverageStatus::SelectedUncovered,
            &valid_summary(false),
            true,
            true,
        );
        let status_rows = [
            (selected_covered, "selected_covered"),
            (selected_uncovered, "selected_uncovered"),
            (
                CoverageRow::outcome_for_test(id('3'), CoverageStatus::ParseFailure),
                "parse_failure",
            ),
            (
                CoverageRow::outcome_for_test(id('4'), CoverageStatus::UnresolvedAmbiguity),
                "unresolved_ambiguity",
            ),
            (
                CoverageRow::outcome_for_test(id('5'), CoverageStatus::InternalFailure),
                "internal_failure",
            ),
        ];
        for (row, expected) in status_rows {
            assert_eq!(serde_json::to_value(row).unwrap()["status"], expected);
        }

        for (outcome, expected) in [
            (
                ParseAnalysisOutcome::InternalFailure(
                    InternalFailureKind::ValidatedRootDidNotMaterialize,
                ),
                "validated_root_did_not_materialize",
            ),
            (
                ParseAnalysisOutcome::InternalFailure(InternalFailureKind::SelectionConfiguration),
                "selection_configuration",
            ),
            (
                ParseAnalysisOutcome::InternalFailure(InternalFailureKind::OwnershipInspection),
                "ownership_inspection",
            ),
        ] {
            let row = analysis_row::<FixtureOwnership>(
                &unit("Internal", "text"),
                outcome,
                None,
                Some("message"),
            );
            assert_eq!(
                serde_json::to_value(row).unwrap()["internal_failure_kind"],
                expected
            );
        }
    }

    #[test]
    fn json_wire_uses_literal_ownership_failure_and_invalid_span_tags() {
        let ownership = CoverageOwnership::from_source(&valid_summary(false), &all_failures());
        let json = serde_json::to_value(ownership).unwrap();
        let failures = json["failures"].as_array().unwrap();
        assert_eq!(
            failures
                .iter()
                .map(|failure| failure["kind"].as_str().unwrap())
                .collect::<Vec<_>>(),
            [
                "gap",
                "overlap",
                "invalid_span",
                "invalid_span",
                "synthetic",
                "provenance_plan_mismatch",
                "byte_mismatch",
                "byte_mismatch",
            ]
        );
        assert_eq!(failures[2]["span_kind"], "out_of_bounds");
        assert_eq!(failures[3]["span_kind"], "non_utf8_boundary");
        assert_eq!(failures[5]["parsed"], "parsed\n\t\\\"é");
        assert_eq!(failures[5]["rendered"], "rendered\r\\\"ß");
    }

    #[test]
    fn json_wire_uses_literal_byte_mismatch_scope_tags_and_claim_index() {
        let ownership = CoverageOwnership::from_source(&valid_summary(false), &all_failures());
        let json = serde_json::to_value(ownership).unwrap();
        let failures = json["failures"].as_array().unwrap();
        assert_eq!(failures[6]["scope"]["kind"], "whole_render");
        assert!(failures[6]["scope"].get("index").is_none());
        assert_eq!(failures[6]["expected"], "expected\nwhole");
        assert_eq!(failures[6]["actual"], "actual\rwhole");
        assert_eq!(failures[7]["scope"]["kind"], "claim_slice");
        assert_eq!(failures[7]["scope"]["index"], 7);
        assert_eq!(failures[7]["expected"], "expected\tclaim");
        assert_eq!(failures[7]["actual"], "actual\\claim");
    }

    fn independently_derived_report() -> CoverageReport {
        let distinct_uncovered = FixtureSummary {
            covered: false,
            values: [
                40, 400, 2, 20, 4, 40, 6, 60, 8, 80, 20, 200, 10, 100, 11, 110, 12, 13,
            ],
        };
        let rows = vec![
            CoverageRow::selected_for_test(
                id('1'),
                CoverageStatus::SelectedCovered,
                &valid_summary(true),
                false,
                false,
            ),
            CoverageRow::selected_for_test(
                id('2'),
                CoverageStatus::SelectedUncovered,
                &distinct_uncovered,
                true,
                true,
            ),
            CoverageRow::outcome_for_test(id('3'), CoverageStatus::ParseFailure),
            CoverageRow::outcome_for_test(id('4'), CoverageStatus::UnresolvedAmbiguity),
            CoverageRow::outcome_for_test(id('5'), CoverageStatus::InternalFailure),
        ];
        CoverageReport::try_new(id('a'), rows).unwrap()
    }

    #[test]
    fn summary_matches_all_twenty_seven_independently_derived_fields() {
        let report = independently_derived_report();
        let summary = report.summary();
        assert_eq!(summary.total_units(), 5);
        assert_eq!(summary.selected_units(), 2);
        assert_eq!(summary.covered_units(), 1);
        assert_eq!(summary.selected_uncovered_units(), 1);
        assert_eq!(summary.parse_failures(), 1);
        assert_eq!(summary.unresolved_ties(), 1);
        assert_eq!(summary.internal_failures(), 1);
        assert_eq!(summary.roundtrip_mismatch_units(), 1);
        assert_eq!(summary.ownership_failure_units(), 1);
        assert_eq!(summary.claims(), 55);
        assert_eq!(summary.claimed_bytes(), 550);

        assert_eq!(
            serde_json::to_value(summary).unwrap(),
            serde_json::json!({
                "total_units": 5,
                "selected_units": 2,
                "covered_units": 1,
                "selected_uncovered_units": 1,
                "parse_failures": 1,
                "unresolved_ties": 1,
                "internal_failures": 1,
                "roundtrip_mismatch_units": 1,
                "ownership_failure_units": 1,
                "claims": 55,
                "claimed_bytes": 550,
                "form_literal_claims": 3,
                "form_literal_bytes": 30,
                "vocab_claims": 6,
                "vocab_bytes": 60,
                "lexeme_claims": 9,
                "lexeme_bytes": 90,
                "codec_claims": 12,
                "codec_bytes": 120,
                "identity_claims": 25,
                "identity_bytes": 250,
                "gap_spans": 16,
                "gap_bytes": 160,
                "overlap_spans": 18,
                "overlap_bytes": 180,
                "synthetic_claims": 20,
                "provenance_plan_mismatches": 22
            })
        );
    }

    #[test]
    fn report_and_summary_json_have_the_exact_reviewed_fields() {
        let report = independently_derived_report();
        let json = serde_json::to_value(&report).unwrap();
        let summary_keys = json["summary"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            summary_keys,
            [
                "total_units",
                "selected_units",
                "covered_units",
                "selected_uncovered_units",
                "parse_failures",
                "unresolved_ties",
                "internal_failures",
                "roundtrip_mismatch_units",
                "ownership_failure_units",
                "claims",
                "claimed_bytes",
                "form_literal_claims",
                "form_literal_bytes",
                "vocab_claims",
                "vocab_bytes",
                "lexeme_claims",
                "lexeme_bytes",
                "codec_claims",
                "codec_bytes",
                "identity_claims",
                "identity_bytes",
                "gap_spans",
                "gap_bytes",
                "overlap_spans",
                "overlap_bytes",
                "synthetic_claims",
                "provenance_plan_mismatches",
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(
            json.as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect::<std::collections::BTreeSet<_>>(),
            ["rows", "schema_version", "source_fingerprint", "summary"]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn claim_totals_overflow_only_on_the_second_summary_accumulation() {
        let summary = FixtureSummary {
            covered: true,
            values: [
                usize::MAX,
                1,
                usize::MAX,
                1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        let first = CoverageRow::selected_for_test(
            id('6'),
            CoverageStatus::SelectedCovered,
            &summary,
            false,
            false,
        );
        let second = CoverageRow::selected_for_test(
            id('7'),
            CoverageStatus::SelectedCovered,
            &summary,
            false,
            false,
        );

        assert!(CoverageReport::try_new(id('a'), vec![first.clone()]).is_ok());
        assert!(CoverageReport::try_new(id('a'), vec![second.clone()]).is_ok());
        assert_eq!(
            CoverageReport::try_new(id('a'), vec![first, second]),
            Err(CoverageValidationError::ArithmeticOverflow { field: "claims" })
        );
    }

    #[test]
    fn byte_totals_overflow_only_on_the_second_summary_accumulation() {
        let summary = FixtureSummary {
            covered: true,
            values: [
                1,
                usize::MAX,
                1,
                usize::MAX,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        };
        let first = CoverageRow::selected_for_test(
            id('8'),
            CoverageStatus::SelectedCovered,
            &summary,
            false,
            false,
        );
        let second = CoverageRow::selected_for_test(
            id('9'),
            CoverageStatus::SelectedCovered,
            &summary,
            false,
            false,
        );

        assert!(CoverageReport::try_new(id('a'), vec![first.clone()]).is_ok());
        assert!(CoverageReport::try_new(id('a'), vec![second.clone()]).is_ok());
        assert_eq!(
            CoverageReport::try_new(id('a'), vec![first, second]),
            Err(CoverageValidationError::ArithmeticOverflow {
                field: "claimed_bytes"
            })
        );
    }

    #[test]
    fn row_and_summary_equation_corruptions_return_typed_errors() {
        let report = independently_derived_report();
        let mut bad_row = CoverageRow::selected_for_test(
            id('6'),
            CoverageStatus::SelectedCovered,
            &valid_summary(true),
            false,
            false,
        );
        bad_row
            .selected_mut_for_test()
            .ownership_mut_for_test()
            .set_claims_for_test(16);
        assert!(matches!(
            CoverageReport::try_new(id('a'), vec![bad_row]),
            Err(CoverageValidationError::RowClaimKindEquation { .. })
        ));

        let mut bad_bytes = CoverageRow::selected_for_test(
            id('7'),
            CoverageStatus::SelectedCovered,
            &valid_summary(true),
            false,
            false,
        );
        bad_bytes
            .selected_mut_for_test()
            .ownership_mut_for_test()
            .set_claimed_bytes_for_test(151);
        assert!(matches!(
            CoverageReport::try_new(id('a'), vec![bad_bytes]),
            Err(CoverageValidationError::RowByteKindEquation { .. })
        ));

        let mut overflow = CoverageRow::selected_for_test(
            id('8'),
            CoverageStatus::SelectedCovered,
            &valid_summary(true),
            false,
            false,
        );
        overflow
            .selected_mut_for_test()
            .ownership_mut_for_test()
            .set_claims_for_test(usize::MAX);
        overflow
            .selected_mut_for_test()
            .ownership_mut_for_test()
            .set_form_literal_claims_for_test(usize::MAX);
        assert!(matches!(
            CoverageReport::try_new(id('a'), vec![overflow]),
            Err(CoverageValidationError::ArithmeticOverflow { .. })
        ));

        let mut counterfeit = CoverageSummary::from_rows_for_test(report.rows()).unwrap();
        counterfeit.set_total_units_for_test(6);
        assert!(matches!(
            counterfeit.validate_for_test(),
            Err(CoverageValidationError::SummaryOutcomeEquation { .. })
        ));
        let mut counterfeit = CoverageSummary::from_rows_for_test(report.rows()).unwrap();
        counterfeit.set_selected_units_for_test(3);
        counterfeit.set_total_units_for_test(6);
        assert!(matches!(
            counterfeit.validate_for_test(),
            Err(CoverageValidationError::SummarySelectedEquation { .. })
        ));
        let mut counterfeit = CoverageSummary::from_rows_for_test(report.rows()).unwrap();
        counterfeit.claims += 1;
        assert!(matches!(
            counterfeit.validate_for_test(),
            Err(CoverageValidationError::SummaryClaimKindEquation { .. })
        ));
        let mut counterfeit = CoverageSummary::from_rows_for_test(report.rows()).unwrap();
        counterfeit.claimed_bytes += 1;
        assert!(matches!(
            counterfeit.validate_for_test(),
            Err(CoverageValidationError::SummaryByteKindEquation { .. })
        ));
    }

    #[derive(Default)]
    struct Recorder {
        events: Vec<String>,
    }

    impl CoverageObserver for Recorder {
        fn record(&mut self, event: String) {
            self.events.push(event);
        }
    }

    #[derive(Default)]
    struct RecordingWriter {
        bytes: Vec<u8>,
        events: Rc<RefCell<Vec<String>>>,
        fail_write: bool,
        fail_flush: bool,
    }

    impl Write for RecordingWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_write {
                return Err(io::Error::other("write sentinel"));
            }
            self.events.borrow_mut().push("writer.write".to_owned());
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.events.borrow_mut().push("writer.flush".to_owned());
            if self.fail_flush {
                return Err(io::Error::other("flush sentinel"));
            }
            Ok(())
        }
    }

    fn args(json: bool, mode: CoverageLockMode) -> CoverageArgs {
        CoverageArgs {
            corpus: CorpusArgs {
                data: Path::new("fixture.json").to_owned(),
            },
            lock: Path::new("fixture.lock").to_owned(),
            json,
            check: mode == CoverageLockMode::Check,
            bless: mode == CoverageLockMode::Bless,
        }
    }

    #[test]
    fn runner_orders_orchestration_and_flush_before_gate() {
        let corpus = Corpus::from_units_for_test(vec![
            unit("Zulu", "You frobnitz a card."),
            unit("Alpha", "Whenever a player connives, you gain X life."),
        ]);
        let ordered_ids = corpus
            .units()
            .iter()
            .map(|unit| unit.id().to_owned())
            .collect::<Vec<_>>();
        let shared = Rc::new(RefCell::new(Vec::new()));
        let mut output = RecordingWriter {
            events: Rc::clone(&shared),
            ..RecordingWriter::default()
        };
        let mut diagnostics = Vec::new();
        let mut observer = Recorder::default();
        let mut gate_calls = 0;

        run_with_components(
            &args(true, CoverageLockMode::Check),
            &mut output,
            &mut diagnostics,
            &mut observer,
            || Ok(corpus),
            crate::english_v2::parser_from_builtin_v2,
            |_, _, _, diagnostics| {
                gate_calls += 1;
                shared.borrow_mut().push("gate".to_owned());
                writeln!(diagnostics, "gate diagnostic")?;
                Ok(())
            },
        )
        .unwrap();

        assert_eq!(gate_calls, 1);
        assert_eq!(
            observer.events,
            [
                "load_corpus".to_owned(),
                "load_environment".to_owned(),
                format!("analyze_oracle_text:{}", ordered_ids[0]),
                format!("map_row:{}", ordered_ids[0]),
                format!("analyze_oracle_text:{}", ordered_ids[1]),
                format!("map_row:{}", ordered_ids[1]),
                "validate".to_owned(),
                "render".to_owned(),
                "flush".to_owned(),
                "gate".to_owned(),
            ]
        );
        assert_eq!(shared.borrow().last().map(String::as_str), Some("gate"));
        assert_eq!(diagnostics, b"gate diagnostic\n");
        let json: serde_json::Value = serde_json::from_slice(&output.bytes).unwrap();
        assert_eq!(json["rows"].as_array().unwrap().len(), 2);
        assert_eq!(json["rows"][0]["id"], ordered_ids[0]);
        assert_eq!(json["rows"][1]["id"], ordered_ids[1]);
        assert!(output.bytes.ends_with(b"\n"));
        assert!(!output.bytes.ends_with(b"\n\n"));
    }

    #[test]
    fn runner_calls_oracle_text_once_per_ordered_unit_without_ability_fallback() {
        let corpus = Corpus::from_units_for_test(vec![
            unit("Zulu", "Destroy target Spirit.\nYou gain 2 life."),
            unit("Alpha", "Whenever a player connives, you gain X life."),
        ]);
        let expected_ids = corpus
            .units()
            .iter()
            .map(|unit| unit.id().to_owned())
            .collect::<Vec<_>>();
        reset_parser_entry_calls_for_test();
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();
        let mut observer = Recorder::default();

        run_with_components(
            &args(true, CoverageLockMode::None),
            &mut output,
            &mut diagnostics,
            &mut observer,
            || Ok(corpus),
            crate::english_v2::parser_from_builtin_v2,
            |_, _, _, _| unreachable!("report-only coverage has no gate"),
        )
        .unwrap();

        assert_eq!(
            take_parser_entry_calls_for_test(),
            [
                (
                    ParserEntryPoint::AnalyzeOracleText,
                    "Whenever a player connives, you gain X life.".to_owned(),
                    "Alpha".to_owned(),
                ),
                (
                    ParserEntryPoint::AnalyzeOracleText,
                    "Destroy target Spirit.\nYou gain 2 life.".to_owned(),
                    "Zulu".to_owned(),
                ),
            ],
            "coverage must enter only OracleText, exactly once per ordered corpus unit"
        );
        let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(
            json["rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| row["id"].as_str().unwrap())
                .collect::<Vec<_>>(),
            expected_ids.iter().map(String::as_str).collect::<Vec<_>>()
        );
        assert_eq!(json["rows"][0]["status"], "selected_covered");
    }

    #[test]
    fn real_runner_preserves_every_stored_metadata_field_and_corpus_order() {
        let metadata = CorpusUnit::for_test_with_metadata(
            "Zulu Stored Card",
            Some("Distinct Face"),
            Some("b"),
            "Distinct Parser Context",
            "Destroy target Spirit.",
        );
        let metadata_id = metadata.id().to_owned();
        let first = unit(
            "Alpha Other Card",
            "Whenever a player connives, you gain X life.",
        );
        let first_id = first.id().to_owned();
        let corpus = Corpus::from_units_for_test(vec![metadata, first]);
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();
        let mut observer = NoopObserver;

        run_with_components(
            &args(true, CoverageLockMode::None),
            &mut output,
            &mut diagnostics,
            &mut observer,
            || Ok(corpus),
            crate::english_v2::parser_from_builtin_v2,
            |_, _, _, _| unreachable!("report-only coverage has no gate"),
        )
        .unwrap();

        let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(json["rows"][0]["id"], first_id);
        assert_eq!(json["rows"][1]["id"], metadata_id);
        assert_eq!(json["rows"][1]["card_name"], "Zulu Stored Card");
        assert_eq!(json["rows"][1]["face_name"], "Distinct Face");
        assert_eq!(json["rows"][1]["side"], "b");
        assert_eq!(json["rows"][1]["context_name"], "Distinct Parser Context");
        assert_eq!(json["rows"][1]["text"], "Destroy target Spirit.");
    }

    #[test]
    fn real_runner_parsing_uses_legendary_face_context_metadata() {
        let snapshot = br#"{"data":{"Composite Wrong Name":[{
            "name":"Composite Wrong Name",
            "faceName":"Aang, A Lot to Learn",
            "side":"a",
            "layout":"modal_dfc",
            "types":["Creature"],
            "supertypes":["Legendary"],
            "subtypes":[],
            "legalities":{"vintage":"Legal"},
            "text":"Aang gains 2 life."
        }]}}"#;
        let onsets = BTreeMap::from([("Aang, A Lot to Learn".to_owned(), Onset::Vowel)]);
        let corpus = Corpus::from_bytes_with_context_onsets(snapshot, &onsets)
            .expect("authoritative legendary face fixture loads");
        assert_eq!(corpus.units()[0].card_name(), "Composite Wrong Name");
        assert_eq!(corpus.units()[0].context_name(), "Aang, A Lot to Learn");
        assert!(corpus.units()[0].is_legendary());

        let mut output = Vec::new();
        run_with_components(
            &args(true, CoverageLockMode::None),
            &mut output,
            &mut Vec::new(),
            &mut NoopObserver,
            || Ok(corpus),
            crate::english_v2::parser_from_builtin_v2,
            |_, _, _, _| unreachable!("report-only coverage has no gate"),
        )
        .expect("real coverage runner selects the abbreviated face self-reference");

        let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(json["rows"][0]["status"], "selected_covered");
        assert_eq!(
            json["rows"][0]["selected"]["rendered_text"],
            "Aang gains 2 life."
        );
    }

    #[test]
    fn render_is_complete_deterministic_and_line_safe_for_hostile_metadata() {
        let row = CoverageRow::selected_with_metadata_for_test(
            id('1'),
            "Card\n\r\t\\\"é",
            Some("Face\nß"),
            Some("a\r"),
            "Context\t",
            "Text\nline",
            CoverageStatus::SelectedUncovered,
            &valid_summary(false),
            &all_failures(),
            "Rendered\rline",
        );
        let report = CoverageReport::try_new(id('f'), vec![row]).unwrap();
        let mut first = Vec::new();
        let mut second = Vec::new();
        render_report(&report, true, &mut first).unwrap();
        render_report(&report, true, &mut second).unwrap();
        assert_eq!(first, second);
        let decoded: CoverageReport = serde_json::from_slice(&first).unwrap();
        assert_eq!(decoded, report);
        assert!(first.ends_with(b"\n"));
        assert!(!first.ends_with(b"\n\n"));

        let mut human = Vec::new();
        render_report(&report, false, &mut human).unwrap();
        let human = String::from_utf8(human).unwrap();
        assert_eq!(human.lines().count(), 3);
        assert!(
            human
                .lines()
                .all(|line| !line.contains('\r') && !line.contains('\t'))
        );
        assert!(human.contains("Card\\n\\r\\t"));
        assert!(!human.contains("..."));
    }

    #[test]
    fn write_and_flush_failures_win_and_prevent_gate_or_diagnostics() {
        for (fail_write, fail_flush, sentinel) in [
            (true, false, "write sentinel"),
            (false, true, "flush sentinel"),
        ] {
            let corpus = Corpus::from_units_for_test(vec![unit(
                "Clean",
                "Whenever a player connives, you gain X life.",
            )]);
            let mut output = RecordingWriter {
                fail_write,
                fail_flush,
                ..RecordingWriter::default()
            };
            let mut diagnostics = Vec::new();
            let mut observer = NoopObserver;
            let mut gate_calls = 0;
            let error = run_with_components(
                &args(true, CoverageLockMode::Check),
                &mut output,
                &mut diagnostics,
                &mut observer,
                || Ok(corpus),
                crate::english_v2::parser_from_builtin_v2,
                |_, _, _, _| {
                    gate_calls += 1;
                    anyhow::bail!("gate sentinel")
                },
            )
            .unwrap_err();
            let error = format!("{error:#}");
            assert!(error.contains(sentinel), "{error}");
            assert_eq!(gate_calls, 0);
            assert!(diagnostics.is_empty());
        }
    }

    #[test]
    fn report_only_diagnostics_do_not_fail_or_invoke_a_gate() {
        let corpus = Corpus::from_units_for_test(vec![unit("Failed", "You frobnitz a card.")]);
        let mut output = Vec::new();
        let mut diagnostics = Vec::new();
        let mut observer = NoopObserver;
        let mut gate_calls = 0;
        run_with_components(
            &args(true, CoverageLockMode::None),
            &mut output,
            &mut diagnostics,
            &mut observer,
            || Ok(corpus),
            crate::english_v2::parser_from_builtin_v2,
            |_, _, _, _| {
                gate_calls += 1;
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(gate_calls, 0);
        assert!(diagnostics.is_empty());
        let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(json["summary"]["parse_failures"], 1);
    }

    #[test]
    fn real_internal_analysis_is_rendered_and_flushed_before_the_command_fails() {
        let corpus = Corpus::from_units_for_test(vec![unit(
            "Internal",
            "Whenever a player connives, you gain X life.",
        )]);
        let shared = Rc::new(RefCell::new(Vec::new()));
        let mut output = RecordingWriter {
            events: Rc::clone(&shared),
            ..RecordingWriter::default()
        };
        let mut diagnostics = Vec::new();
        let mut observer = Recorder::default();
        let error = with_forced_ownership_inspection_failure_for_test(|| {
            run_with_components(
                &args(true, CoverageLockMode::Check),
                &mut output,
                &mut diagnostics,
                &mut observer,
                || Ok(corpus),
                crate::english_v2::parser_from_builtin_v2,
                |_, _, _, _| unreachable!("internal coverage must not invoke the gate"),
            )
        })
        .unwrap_err()
        .to_string();

        assert!(error.contains("1 internal failure"));
        assert_eq!(observer.events.last().map(String::as_str), Some("flush"));
        assert_eq!(
            shared.borrow().last().map(String::as_str),
            Some("writer.flush")
        );
        assert!(diagnostics.is_empty());
        let json: serde_json::Value = serde_json::from_slice(&output.bytes).unwrap();
        assert_eq!(json["rows"][0]["status"], "internal_failure");
        assert_eq!(
            json["rows"][0]["internal_failure_kind"],
            "ownership_inspection"
        );
        assert_eq!(json["summary"]["internal_failures"], 1);
        assert!(output.bytes.ends_with(b"\n"));
    }

    #[test]
    fn coverage_ownership_copies_every_summary_scalar_without_recounting() {
        let summary = valid_summary(false);
        let mapped = CoverageOwnership::from_source(&summary, &all_failures());
        assert_eq!(
            mapped.scalar_values_for_test(),
            [
                usize::from(false),
                15,
                150,
                1,
                10,
                2,
                20,
                3,
                30,
                4,
                40,
                5,
                50,
                6,
                60,
                7,
                70,
                8,
                9
            ]
        );
    }
}
