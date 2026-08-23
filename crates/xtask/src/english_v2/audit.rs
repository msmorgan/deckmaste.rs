use std::collections::BTreeSet;

use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::render::Render;

use super::corpus::Corpus;
use super::corpus::CorpusUnit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum AuditStatus {
    Clean,
    Mismatch,
    ParseFailure,
    Ambiguous,
    InternalFailure,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(super) struct AuditRow {
    id: String,
    card_name: String,
    face_name: Option<String>,
    text: String,
    status: AuditStatus,
    rendered: Option<String>,
    message: Option<String>,
}

impl AuditRow {
    pub(super) const fn status(&self) -> AuditStatus {
        self.status
    }

    pub(super) fn id(&self) -> &str {
        &self.id
    }

    pub(super) fn printed_face(&self) -> &str {
        self.face_name.as_deref().unwrap_or(&self.card_name)
    }

    pub(super) fn text(&self) -> &str {
        &self.text
    }

    pub(super) fn rendered(&self) -> Option<&str> {
        self.rendered.as_deref()
    }

    pub(super) fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(super) struct AuditReport {
    schema_version: u32,
    source_fingerprint: String,
    rows: Vec<AuditRow>,
}

impl AuditReport {
    pub(super) fn run(corpus: &Corpus, parser: &Parser) -> Self {
        Self {
            schema_version: 1,
            source_fingerprint: corpus.source_fingerprint().to_owned(),
            rows: corpus
                .units()
                .iter()
                .map(|unit| audit_unit(unit, parser))
                .collect(),
        }
    }

    pub(super) fn rows(&self) -> &[AuditRow] {
        &self.rows
    }

    pub(super) const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub(super) fn source_fingerprint(&self) -> &str {
        &self.source_fingerprint
    }

    pub(super) fn summary(&self) -> AuditSummary {
        AuditSummary::from_rows(&self.rows)
    }

    pub(super) fn accepted_ids(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .filter(|row| matches!(row.status, AuditStatus::Clean | AuditStatus::Mismatch))
            .map(|row| row.id.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
pub(super) struct AuditSummary {
    pub(super) total: usize,
    pub(super) clean: usize,
    pub(super) mismatched: usize,
    pub(super) parse_failures: usize,
    pub(super) ambiguous: usize,
    pub(super) internal_failures: usize,
}

impl AuditSummary {
    fn from_rows(rows: &[AuditRow]) -> Self {
        let mut summary = Self::default();
        for row in rows {
            summary.total += 1;
            match row.status {
                AuditStatus::Clean => summary.clean += 1,
                AuditStatus::Mismatch => summary.mismatched += 1,
                AuditStatus::ParseFailure => summary.parse_failures += 1,
                AuditStatus::Ambiguous => summary.ambiguous += 1,
                AuditStatus::InternalFailure => summary.internal_failures += 1,
            }
        }
        summary
    }
}

fn audit_unit(unit: &CorpusUnit, parser: &Parser) -> AuditRow {
    let mut row = AuditRow {
        id: unit.id().to_owned(),
        card_name: unit.card_name().to_owned(),
        face_name: unit.face_name().map(str::to_owned),
        text: unit.text().to_owned(),
        status: AuditStatus::InternalFailure,
        rendered: None,
        message: None,
    };

    let Some(context) = ParseContext::new(
        unit.context_name(),
        unit.is_legendary(),
        unit.context_onset(),
    ) else {
        row.message = Some("parse context did not materialize".to_owned());
        return row;
    };

    match parser
        .analyze_oracle_text(unit.text(), &context)
        .into_parse_result()
    {
        Ok(oracle_text) => {
            let rendered = oracle_text.render(&context, parser.environment());
            row.status =
                if rendered == unit.text() { AuditStatus::Clean } else { AuditStatus::Mismatch };
            row.message =
                (row.status == AuditStatus::Mismatch).then(|| "rendered bytes differ".to_owned());
            row.rendered = Some(rendered);
        }
        Err(error) => {
            apply_parse_error(&mut row, &error);
        }
    }

    row
}

fn apply_parse_error(row: &mut AuditRow, error: &ParseError) {
    row.status = audit_status_for_error(error);
    row.rendered = None;
    row.message = Some(error.to_string());
}

fn audit_status_for_error(error: &ParseError) -> AuditStatus {
    match error {
        ParseError::Failure { .. } | ParseError::BuildRejected { .. } => AuditStatus::ParseFailure,
        ParseError::Ambiguous { .. } => AuditStatus::Ambiguous,
        ParseError::InvalidSelectionExceptionConfiguration(_)
        | ParseError::ValidatedRootDidNotMaterialize
        | ParseError::OwnershipInspection => AuditStatus::InternalFailure,
    }
}

#[cfg(test)]
impl AuditReport {
    pub(super) fn from_rows_for_test(rows: &[(AuditStatus, &str, &str, Option<&str>)]) -> Self {
        let rows = rows
            .iter()
            .enumerate()
            .map(|(index, (status, face, text, rendered))| {
                let number = index + 1;
                AuditRow {
                    id: format!("{number:064x}"),
                    card_name: (*face).to_owned(),
                    face_name: None,
                    text: (*text).to_owned(),
                    status: *status,
                    rendered: rendered.map(str::to_owned),
                    message: None,
                }
            })
            .collect();
        Self {
            schema_version: 1,
            source_fingerprint: "0".repeat(64),
            rows,
        }
    }

    pub(super) fn from_statuses_for_test(statuses: &[AuditStatus]) -> Self {
        let rows = statuses
            .iter()
            .copied()
            .enumerate()
            .map(|(index, status)| {
                let number = index + 1;
                let text = format!("Fixture {number} text.");
                let (rendered, message) = match status {
                    AuditStatus::Clean => (Some(text.clone()), None),
                    AuditStatus::Mismatch => (
                        Some(format!("Different fixture {number} text.")),
                        Some("rendered bytes differ".to_owned()),
                    ),
                    AuditStatus::ParseFailure => {
                        (None, Some("parse failed at bytes 0..7".to_owned()))
                    }
                    AuditStatus::Ambiguous => (
                        None,
                        Some("ambiguous parse between first and second".to_owned()),
                    ),
                    AuditStatus::InternalFailure => (
                        None,
                        Some("validated chart root did not materialize".to_owned()),
                    ),
                };
                AuditRow {
                    id: format!("{number:064x}"),
                    card_name: format!("Fixture {number}"),
                    face_name: None,
                    text,
                    status,
                    rendered,
                    message,
                }
            })
            .collect();
        Self {
            schema_version: 1,
            source_fingerprint: "0".repeat(64),
            rows,
        }
    }
}

#[cfg(test)]
impl AuditSummary {
    pub(super) fn from_statuses(statuses: impl IntoIterator<Item = AuditStatus>) -> Self {
        let statuses = statuses.into_iter().collect::<Vec<_>>();
        AuditReport::from_statuses_for_test(&statuses).summary()
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_english_v2::context::ParseContext;
    use deckmaste_english_v2::parser::ParseError;
    use deckmaste_english_v2::parser::Parser;
    use deckmaste_english_v2::parser::ParserEntryPoint;
    use deckmaste_english_v2::parser::SelectionExceptionInventoryError;
    use deckmaste_english_v2::parser::reset_parser_entry_calls_for_test;
    use deckmaste_english_v2::parser::take_parser_entry_calls_for_test;

    use super::AuditReport;
    use super::AuditRow;
    use super::AuditStatus;
    use super::AuditSummary;
    use crate::english_v2::corpus::Corpus;
    use crate::english_v2::corpus::CorpusUnit;

    fn unit(card_name: &str, text: &str) -> CorpusUnit {
        CorpusUnit::for_test(card_name, text)
    }

    fn parser() -> Parser {
        crate::english_v2::parser_from_builtin_v2().unwrap()
    }

    #[test]
    fn audit_keeps_parse_failure_distinct_from_exact_round_trip() {
        let corpus = Corpus::from_units_for_test(vec![
            unit("Clean", "Whenever a player connives, you gain X life."),
            unit("Failed", "You frobnitz a card."),
        ]);
        let report = AuditReport::run(&corpus, &parser());
        assert_eq!(report.rows()[0].status(), AuditStatus::Clean);
        assert_eq!(
            report.rows()[0].rendered.as_deref(),
            Some(corpus.units()[0].text())
        );
        assert_eq!(report.rows()[1].status(), AuditStatus::ParseFailure);
        assert!(
            report.rows()[1]
                .message()
                .unwrap()
                .contains("parse failed at bytes")
        );
    }

    #[test]
    fn authored_build_rejection_counts_as_parse_failure_not_internal_failure() {
        let context = ParseContext::new("Context Card", false, macro_ron::v2::Onset::Consonant)
            .expect("test context is valid");
        let error = parser()
            .analyze_oracle_text("You gain X life, a player connives.", &context)
            .into_parse_result()
            .expect_err("the WithWhere invariant rejects an Event clause");

        assert!(matches!(error, ParseError::BuildRejected { .. }));
        assert_eq!(
            super::audit_status_for_error(&error),
            AuditStatus::ParseFailure
        );
        assert_ne!(
            super::audit_status_for_error(&error),
            AuditStatus::InternalFailure
        );
    }

    #[test]
    fn corpus_runner_analyzes_each_complete_document_once_without_ability_fallback() {
        let corpus = Corpus::from_units_for_test(vec![
            unit("Two Blocks", "Destroy target creature.\nYou gain 2 life."),
            unit("Failed", "You frobnitz a card."),
        ]);
        reset_parser_entry_calls_for_test();

        let report = AuditReport::run(&corpus, &parser());

        assert_eq!(
            take_parser_entry_calls_for_test(),
            [
                (
                    ParserEntryPoint::AnalyzeOracleText,
                    "You frobnitz a card.".to_owned(),
                    "Failed".to_owned(),
                ),
                (
                    ParserEntryPoint::AnalyzeOracleText,
                    "Destroy target creature.\nYou gain 2 life.".to_owned(),
                    "Two Blocks".to_owned(),
                ),
            ],
            "the shared parse/roundtrip audit must enter only OracleText, exactly once per ordered corpus unit"
        );
        assert_eq!(report.rows().len(), corpus.units().len());
        assert_eq!(
            report.rows()[0].printed_face(),
            corpus.units()[0].card_name()
        );
        assert_eq!(
            report.rows()[1].printed_face(),
            corpus.units()[1].card_name()
        );
        assert_eq!(report.rows()[0].status(), AuditStatus::ParseFailure);
        assert_eq!(report.rows()[1].status(), AuditStatus::Clean);
    }

    #[test]
    fn summary_counts_every_status_without_collapsing_failures() {
        let summary = AuditSummary::from_statuses([
            AuditStatus::Clean,
            AuditStatus::Mismatch,
            AuditStatus::ParseFailure,
            AuditStatus::Ambiguous,
            AuditStatus::InternalFailure,
        ]);
        assert_eq!(summary.clean, 1);
        assert_eq!(summary.mismatched, 1);
        assert_eq!(summary.parse_failures, 1);
        assert_eq!(summary.ambiguous, 1);
        assert_eq!(summary.internal_failures, 1);
    }

    #[test]
    fn accepted_ids_include_clean_and_mismatch_rows_only() {
        let report = AuditReport::from_statuses_for_test(&[
            AuditStatus::Clean,
            AuditStatus::Mismatch,
            AuditStatus::ParseFailure,
            AuditStatus::Ambiguous,
            AuditStatus::InternalFailure,
        ]);

        assert_eq!(
            report.accepted_ids(),
            [format!("{:064x}", 1), format!("{:064x}", 2)]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn invalid_selection_configuration_is_an_internal_failure_with_full_message() {
        let error = ParseError::InvalidSelectionExceptionConfiguration(
            SelectionExceptionInventoryError::BlankId,
        );

        assert_eq!(
            super::audit_status_for_error(&error),
            AuditStatus::InternalFailure
        );
        assert_eq!(
            error.to_string(),
            "invalid selection exception configuration: entry id is empty or whitespace-only"
        );
    }

    #[test]
    fn invalid_selection_configuration_writes_its_complete_message_to_the_audit_row() {
        let error = ParseError::InvalidSelectionExceptionConfiguration(
            SelectionExceptionInventoryError::BlankId,
        );
        let mut row = AuditRow {
            id: "fixture-id".to_owned(),
            card_name: "Fixture".to_owned(),
            face_name: None,
            text: "Fixture text.".to_owned(),
            status: AuditStatus::Clean,
            rendered: Some("Fixture text.".to_owned()),
            message: None,
        };

        super::apply_parse_error(&mut row, &error);

        assert_eq!(row.status(), AuditStatus::InternalFailure);
        assert_eq!(row.rendered(), None);
        assert_eq!(
            row.message(),
            Some("invalid selection exception configuration: entry id is empty or whitespace-only")
        );
    }
}
