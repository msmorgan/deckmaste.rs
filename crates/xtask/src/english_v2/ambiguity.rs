use std::cmp::Ordering;
use std::io::Write;

use anyhow::Context;
use anyhow::bail;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::parser::InternalFailureKind;
use deckmaste_english_v2::parser::ParseAnalysisOutcome;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionDecision as ParserSelectionDecision;
use deckmaste_english_v2::parser::SelectionDecisive as ParserSelectionDecisive;
use deckmaste_english_v2::parser::SelectionResolution as ParserSelectionResolution;
use deckmaste_english_v2::parser::SpecificityTier as ParserSpecificityTier;
use serde::Serialize;

use super::AmbiguityArgs;
use super::corpus::Corpus;
use super::corpus::CorpusPerformance;
use super::corpus::CorpusUnit;
use super::corpus::map_corpus_units;
use super::packed;
use super::packed::PackedSite;

pub(super) fn run(args: &AmbiguityArgs, output: &mut dyn Write) -> anyhow::Result<()> {
    let started = std::time::Instant::now();
    let workers = args.corpus.workers()?;
    let corpus = Corpus::load(&args.corpus.data)
        .with_context(|| format!("loading corpus from {}", args.corpus.data.display()))?;
    let parser = crate::english_v2::parser_from_builtin_v2()?;
    let report = AmbiguityReport::run(&corpus, &parser, workers)?;

    render_report(&report, args.json, output)?;
    output
        .flush()
        .context("flushing English-v2 ambiguity census")?;
    super::corpus::write_corpus_performance("ambiguity", started.elapsed(), report.performance)?;
    apply_exit_predicates(&report, args.require_resolved)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum AmbiguityStatus {
    Selected,
    ParseFailure,
    UnresolvedTie,
    InternalFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum InternalKind {
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
    OwnershipInspection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SelectionResolution {
    Unique,
    Specificity,
    Exception,
    UnresolvedTie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SpecificityTier {
    Nonterminal,
    TypedLexical,
    Identity,
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SelectionDecisive {
    Position { index: usize },
    VectorExhaustion { index: usize },
    Tie,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SelectionCandidate {
    ordinal: usize,
    construction_path: Vec<String>,
    specificity: Vec<SpecificityTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SelectionComparison {
    left_ordinal: usize,
    right_ordinal: usize,
    ordering: OrderingKind,
    decisive: SelectionDecisive,
    exception_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum OrderingKind {
    Less,
    Equal,
    Greater,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SelectionDecision {
    candidates: Vec<SelectionCandidate>,
    comparisons: Vec<SelectionComparison>,
    survivors: Vec<usize>,
    selected: Option<usize>,
    resolution: SelectionResolution,
    exception_uses: Vec<String>,
}

impl SelectionDecision {
    fn from_parser(decision: &ParserSelectionDecision) -> Self {
        Self {
            candidates: decision
                .candidates()
                .iter()
                .map(|candidate| SelectionCandidate {
                    ordinal: candidate.ordinal(),
                    construction_path: candidate.construction_path().to_vec(),
                    specificity: candidate
                        .specificity()
                        .iter()
                        .copied()
                        .map(specificity_tier)
                        .collect(),
                })
                .collect(),
            comparisons: decision
                .comparisons()
                .iter()
                .map(|comparison| SelectionComparison {
                    left_ordinal: comparison.left_ordinal(),
                    right_ordinal: comparison.right_ordinal(),
                    ordering: ordering(comparison.ordering()),
                    decisive: decisive(comparison.decisive()),
                    exception_id: comparison.exception_id().map(str::to_owned),
                })
                .collect(),
            survivors: decision.survivors().to_vec(),
            selected: decision.selected(),
            resolution: resolution(decision.resolution()),
            exception_uses: decision.exception_uses().to_vec(),
        }
    }

    #[cfg(test)]
    fn resolution(&self) -> SelectionResolution {
        self.resolution
    }

    #[cfg(test)]
    fn survivors(&self) -> &[usize] {
        &self.survivors
    }

    #[cfg(test)]
    fn comparisons(&self) -> &[SelectionComparison] {
        &self.comparisons
    }
}

fn ordering(ordering: Ordering) -> OrderingKind {
    match ordering {
        Ordering::Less => OrderingKind::Less,
        Ordering::Equal => OrderingKind::Equal,
        Ordering::Greater => OrderingKind::Greater,
    }
}

fn decisive(decisive: ParserSelectionDecisive) -> SelectionDecisive {
    match decisive {
        ParserSelectionDecisive::Position(index) => SelectionDecisive::Position { index },
        ParserSelectionDecisive::VectorExhaustion(index) => {
            SelectionDecisive::VectorExhaustion { index }
        }
        ParserSelectionDecisive::Tie => SelectionDecisive::Tie,
    }
}

fn resolution(resolution: ParserSelectionResolution) -> SelectionResolution {
    match resolution {
        ParserSelectionResolution::Unique => SelectionResolution::Unique,
        ParserSelectionResolution::Specificity => SelectionResolution::Specificity,
        ParserSelectionResolution::Exception => SelectionResolution::Exception,
        ParserSelectionResolution::UnresolvedTie => SelectionResolution::UnresolvedTie,
    }
}

fn specificity_tier(tier: ParserSpecificityTier) -> SpecificityTier {
    match tier {
        ParserSpecificityTier::Nonterminal => SpecificityTier::Nonterminal,
        ParserSpecificityTier::TypedLexical => SpecificityTier::TypedLexical,
        ParserSpecificityTier::Identity => SpecificityTier::Identity,
        ParserSpecificityTier::Literal => SpecificityTier::Literal,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct AmbiguityRow {
    id: String,
    card_name: String,
    face_name: Option<String>,
    side: Option<String>,
    context_name: String,
    text: String,
    status: AmbiguityStatus,
    message: Option<String>,
    internal_kind: Option<InternalKind>,
    decision: Option<SelectionDecision>,
    packed_sites: Vec<PackedSite>,
}

impl AmbiguityRow {
    fn from_unit(unit: &CorpusUnit, parser: &Parser) -> Self {
        let mut row = Self::base(unit);
        let context = ParseContext::new(
            unit.context_name(),
            unit.is_legendary(),
            unit.context_onset(),
        )
        .expect("corpus admission validates every parser context");

        let analysis = parser.analyze_oracle_text(unit.text(), &context);
        row.decision = analysis.decision().map(SelectionDecision::from_parser);
        row.packed_sites = analysis
            .selected()
            .map_or_else(Vec::new, packed::oracle_text);
        match analysis.outcome() {
            ParseAnalysisOutcome::Selected => {
                row.status = AmbiguityStatus::Selected;
            }
            ParseAnalysisOutcome::ParseFailure => {
                row.status = AmbiguityStatus::ParseFailure;
            }
            ParseAnalysisOutcome::UnresolvedAmbiguity => {
                row.status = AmbiguityStatus::UnresolvedTie;
            }
            ParseAnalysisOutcome::InternalFailure(kind) => {
                row.status = AmbiguityStatus::InternalFailure;
                row.internal_kind = Some(internal_kind(kind));
            }
        }
        row.message = analysis.error().map(super::corpus::corpus_error_message);
        row
    }

    fn base(unit: &CorpusUnit) -> Self {
        Self {
            id: unit.id().to_owned(),
            card_name: unit.card_name().to_owned(),
            face_name: unit.face_name().map(str::to_owned),
            side: unit.side().map(str::to_owned),
            context_name: unit.context_name().to_owned(),
            text: unit.text().to_owned(),
            status: AmbiguityStatus::InternalFailure,
            message: None,
            internal_kind: None,
            decision: None,
            packed_sites: Vec::new(),
        }
    }

    #[cfg(test)]
    fn id(&self) -> &str {
        &self.id
    }

    #[cfg(test)]
    fn status(&self) -> AmbiguityStatus {
        self.status
    }

    #[cfg(test)]
    fn decision(&self) -> Option<&SelectionDecision> {
        self.decision.as_ref()
    }

    #[cfg(test)]
    fn internal_kind(&self) -> Option<InternalKind> {
        self.internal_kind
    }

    #[cfg(test)]
    fn set_message(&mut self, message: Option<String>) {
        self.message = message;
    }

    #[cfg(test)]
    fn set_text(&mut self, text: String) {
        self.text = text;
    }
}

fn internal_kind(kind: InternalFailureKind) -> InternalKind {
    match kind {
        InternalFailureKind::ValidatedRootDidNotMaterialize => {
            InternalKind::ValidatedRootDidNotMaterialize
        }
        InternalFailureKind::SelectionConfiguration => InternalKind::SelectionConfiguration,
        InternalFailureKind::OwnershipInspection => InternalKind::OwnershipInspection,
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
struct AmbiguitySummary {
    total: usize,
    selected: usize,
    unique: usize,
    specificity_resolved: usize,
    exception_resolved: usize,
    unresolved_ties: usize,
    parse_failures: usize,
    internal_failures: usize,
    exception_uses: usize,
    #[serde(skip)]
    packed_units: usize,
    #[serde(skip)]
    packed_unit_ids: Vec<String>,
}

impl AmbiguitySummary {
    fn from_rows(rows: &[AmbiguityRow]) -> anyhow::Result<Self> {
        let mut summary = Self::default();
        for row in rows {
            summary.total += 1;
            match row.status {
                AmbiguityStatus::Selected => {
                    summary.selected += 1;
                    let decision = row
                        .decision
                        .as_ref()
                        .context("selected ambiguity row lacks decision")?;
                    match decision.resolution {
                        SelectionResolution::Unique => summary.unique += 1,
                        SelectionResolution::Specificity => summary.specificity_resolved += 1,
                        SelectionResolution::Exception => summary.exception_resolved += 1,
                        SelectionResolution::UnresolvedTie => {
                            bail!("selected ambiguity row has unresolved-tie decision")
                        }
                    }
                    summary.exception_uses += decision.exception_uses.len();
                    if !row.packed_sites.is_empty() {
                        summary.packed_units += 1;
                        summary.packed_unit_ids.push(row.id.clone());
                    }
                }
                AmbiguityStatus::ParseFailure => summary.parse_failures += 1,
                AmbiguityStatus::UnresolvedTie => {
                    summary.unresolved_ties += 1;
                    let decision = row
                        .decision
                        .as_ref()
                        .context("unresolved ambiguity row lacks decision")?;
                    if decision.resolution != SelectionResolution::UnresolvedTie {
                        bail!("unresolved ambiguity row has non-tie decision")
                    }
                    summary.exception_uses += decision.exception_uses.len();
                }
                AmbiguityStatus::InternalFailure => summary.internal_failures += 1,
            }
        }
        Ok(summary)
    }

    fn validate(&self) -> anyhow::Result<()> {
        if self.total
            != self.selected + self.unresolved_ties + self.parse_failures + self.internal_failures
        {
            bail!("ambiguity summary total does not equal its row-status counts")
        }
        if self.selected != self.unique + self.specificity_resolved + self.exception_resolved {
            bail!("ambiguity summary selected does not equal its resolution counts")
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct AmbiguityReport {
    schema_version: u32,
    source_fingerprint: String,
    rows: Vec<AmbiguityRow>,
    summary: AmbiguitySummary,
    #[serde(skip)]
    performance: CorpusPerformance,
}

impl AmbiguityReport {
    fn run(corpus: &Corpus, parser: &Parser, workers: usize) -> anyhow::Result<Self> {
        let (rows, performance) = map_corpus_units(
            corpus.units(),
            workers,
            |_, unit| AmbiguityRow::from_unit(unit, parser),
            |row| row.status == AmbiguityStatus::Selected,
        );
        let mut report = Self::new(corpus.source_fingerprint().to_owned(), rows)?;
        report.performance = performance;
        report.validate_against_corpus(corpus)?;
        Ok(report)
    }

    fn new(source_fingerprint: String, rows: Vec<AmbiguityRow>) -> anyhow::Result<Self> {
        if !is_lower_hex_64(&source_fingerprint) {
            bail!("ambiguity source fingerprint must be lowercase 64-hex")
        }
        for row in &rows {
            if !is_lower_hex_64(&row.id) {
                bail!("ambiguity row ID `{}` must be lowercase 64-hex", row.id)
            }
        }
        let summary = AmbiguitySummary::from_rows(&rows)?;
        summary.validate()?;
        Ok(Self {
            schema_version: 1,
            source_fingerprint,
            rows,
            summary,
            performance: CorpusPerformance::default(),
        })
    }

    fn validate_against_corpus(&self, corpus: &Corpus) -> anyhow::Result<()> {
        if self.rows.len() != corpus.units().len() {
            bail!("ambiguity report row count does not match corpus order")
        }
        for (index, (row, unit)) in self.rows.iter().zip(corpus.units()).enumerate() {
            if row.id != unit.id()
                || row.card_name != unit.card_name()
                || row.face_name.as_deref() != unit.face_name()
                || row.side.as_deref() != unit.side()
                || row.context_name != unit.context_name()
                || row.text != unit.text()
            {
                bail!("ambiguity report row {index} does not match corpus order")
            }
        }
        Ok(())
    }

    #[cfg(test)]
    fn fixture_with_every_status() -> Self {
        fn row(number: usize, status: AmbiguityStatus) -> AmbiguityRow {
            AmbiguityRow {
                id: format!("{number:064x}"),
                card_name: format!("Fixture {number}"),
                face_name: (number == 2).then(|| "Front".to_owned()),
                side: (number == 2).then(|| "a".to_owned()),
                context_name: format!("Context {number}"),
                text: format!("Fixture {number} text."),
                status,
                message: (status != AmbiguityStatus::Selected)
                    .then(|| format!("Fixture {number} failed.")),
                internal_kind: None,
                decision: None,
                packed_sites: Vec::new(),
            }
        }
        fn candidate(ordinal: usize, tier: SpecificityTier) -> SelectionCandidate {
            SelectionCandidate {
                ordinal,
                construction_path: vec![format!("Candidate{ordinal}")],
                specificity: vec![tier],
            }
        }
        fn decision(resolution: SelectionResolution) -> SelectionDecision {
            SelectionDecision {
                candidates: vec![candidate(0, SpecificityTier::Literal)],
                comparisons: vec![],
                survivors: vec![0],
                selected: Some(0),
                resolution,
                exception_uses: vec![],
            }
        }

        let mut rows = vec![
            row(1, AmbiguityStatus::Selected),
            row(2, AmbiguityStatus::Selected),
            row(3, AmbiguityStatus::Selected),
            row(4, AmbiguityStatus::ParseFailure),
            row(5, AmbiguityStatus::UnresolvedTie),
            row(6, AmbiguityStatus::InternalFailure),
        ];
        rows[0].decision = Some(decision(SelectionResolution::Unique));
        rows[1].decision = Some(SelectionDecision {
            candidates: vec![
                candidate(0, SpecificityTier::TypedLexical),
                candidate(1, SpecificityTier::Literal),
            ],
            comparisons: vec![SelectionComparison {
                left_ordinal: 0,
                right_ordinal: 1,
                ordering: OrderingKind::Less,
                decisive: SelectionDecisive::Position { index: 0 },
                exception_id: None,
            }],
            survivors: vec![1],
            selected: Some(1),
            resolution: SelectionResolution::Specificity,
            exception_uses: vec![],
        });
        rows[2].decision = Some(SelectionDecision {
            candidates: vec![
                candidate(0, SpecificityTier::Literal),
                candidate(1, SpecificityTier::Literal),
            ],
            comparisons: vec![SelectionComparison {
                left_ordinal: 0,
                right_ordinal: 1,
                ordering: OrderingKind::Less,
                decisive: SelectionDecisive::Tie,
                exception_id: Some("fixture-right-wins".to_owned()),
            }],
            survivors: vec![1],
            selected: Some(1),
            resolution: SelectionResolution::Exception,
            exception_uses: vec!["fixture-right-wins".to_owned()],
        });
        rows[1].packed_sites = vec![packed::fixture()];
        rows[4].decision = Some(SelectionDecision {
            candidates: (0..3)
                .map(|ordinal| SelectionCandidate {
                    ordinal,
                    construction_path: vec![format!("Candidate{ordinal}")],
                    specificity: vec![SpecificityTier::TypedLexical],
                })
                .collect(),
            comparisons: vec![
                SelectionComparison {
                    left_ordinal: 0,
                    right_ordinal: 1,
                    ordering: OrderingKind::Equal,
                    decisive: SelectionDecisive::Tie,
                    exception_id: None,
                },
                SelectionComparison {
                    left_ordinal: 0,
                    right_ordinal: 2,
                    ordering: OrderingKind::Equal,
                    decisive: SelectionDecisive::Tie,
                    exception_id: None,
                },
                SelectionComparison {
                    left_ordinal: 1,
                    right_ordinal: 2,
                    ordering: OrderingKind::Equal,
                    decisive: SelectionDecisive::Tie,
                    exception_id: None,
                },
            ],
            survivors: vec![0, 1, 2],
            selected: None,
            resolution: SelectionResolution::UnresolvedTie,
            exception_uses: vec![],
        });
        rows[5].internal_kind = Some(InternalKind::SelectionConfiguration);
        Self::new("f".repeat(64), rows).expect("fixture report validates")
    }

    #[cfg(test)]
    fn rows(&self) -> &[AmbiguityRow] {
        &self.rows
    }

    #[cfg(test)]
    fn rows_mut(&mut self) -> &mut [AmbiguityRow] {
        &mut self.rows
    }

    #[cfg(test)]
    fn schema_version(&self) -> u32 {
        self.schema_version
    }

    #[cfg(test)]
    fn source_fingerprint(&self) -> &str {
        &self.source_fingerprint
    }

    #[cfg(test)]
    fn summary(&self) -> &AmbiguitySummary {
        &self.summary
    }

    #[cfg(test)]
    fn with_statuses(&self, statuses: &[AmbiguityStatus]) -> Self {
        let rows = self
            .rows
            .iter()
            .filter(|row| statuses.contains(&row.status))
            .cloned()
            .collect();
        Self::new(self.source_fingerprint.clone(), rows).expect("fixture statuses validate")
    }

    #[cfg(test)]
    fn with_internal_kind(&self, kind: InternalKind) -> Self {
        let mut row = self.rows[0].clone();
        row.status = AmbiguityStatus::InternalFailure;
        row.message = Some("fixture internal failure".to_owned());
        row.internal_kind = Some(kind);
        row.decision = None;
        Self::new(self.source_fingerprint.clone(), vec![row])
            .expect("fixture internal row validates")
    }
}

fn is_lower_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

#[cfg(test)]
fn render_then_apply(
    report: &AmbiguityReport,
    json: bool,
    require_resolved: bool,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    render_report(report, json, output)?;
    output
        .flush()
        .context("flushing English-v2 ambiguity census")?;
    apply_exit_predicates(report, require_resolved)
}

fn render_report(
    report: &AmbiguityReport,
    json: bool,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    if json {
        serde_json::to_writer_pretty(&mut *output, report)
            .context("writing English-v2 ambiguity census JSON")?;
        writeln!(output).context("writing English-v2 ambiguity census JSON terminator")?;
        return Ok(());
    }

    writeln!(output, "English v2 selection ambiguity census")
        .context("writing English-v2 ambiguity census")?;
    for row in &report.rows {
        write_human_row(row, output).context("writing English-v2 ambiguity census row")?;
    }
    let summary = &report.summary;
    writeln!(output, "summary total={}", summary.total)
        .context("writing English-v2 ambiguity census summary")?;
    writeln!(output, "summary selected={}", summary.selected)
        .context("writing English-v2 ambiguity census summary")?;
    writeln!(output, "summary unique={}", summary.unique)
        .context("writing English-v2 ambiguity census summary")?;
    writeln!(
        output,
        "summary specificity_resolved={}",
        summary.specificity_resolved
    )
    .context("writing English-v2 ambiguity census summary")?;
    writeln!(
        output,
        "summary exception_resolved={}",
        summary.exception_resolved
    )
    .context("writing English-v2 ambiguity census summary")?;
    writeln!(
        output,
        "summary unresolved_ties={}",
        summary.unresolved_ties
    )
    .context("writing English-v2 ambiguity census summary")?;
    writeln!(output, "summary parse_failures={}", summary.parse_failures)
        .context("writing English-v2 ambiguity census summary")?;
    writeln!(
        output,
        "summary internal_failures={}",
        summary.internal_failures
    )
    .context("writing English-v2 ambiguity census summary")?;
    writeln!(output, "summary exception_uses={}", summary.exception_uses)
        .context("writing English-v2 ambiguity census summary")?;
    writeln!(
        output,
        "summary packed_units={} packed_unit_ids={}",
        summary.packed_units,
        serde_json::to_string(&summary.packed_unit_ids)
            .context("serializing packed-unit identities")?
    )
    .context("writing English-v2 ambiguity packed-unit census")?;
    Ok(())
}

fn write_human_row(row: &AmbiguityRow, output: &mut dyn Write) -> anyhow::Result<()> {
    output.write_all(b"row id=")?;
    serde_json::to_writer(&mut *output, &row.id)?;
    output.write_all(b" card_name=")?;
    serde_json::to_writer(&mut *output, &row.card_name)?;
    output.write_all(b" face_name=")?;
    serde_json::to_writer(&mut *output, &row.face_name)?;
    output.write_all(b" side=")?;
    serde_json::to_writer(&mut *output, &row.side)?;
    output.write_all(b" context_name=")?;
    serde_json::to_writer(&mut *output, &row.context_name)?;
    output.write_all(b" text=")?;
    serde_json::to_writer(&mut *output, &row.text)?;
    write!(output, " status={} message=", status_name(row.status))?;
    serde_json::to_writer(&mut *output, &row.message)?;
    output.write_all(b" internal_kind=")?;
    serde_json::to_writer(&mut *output, &row.internal_kind)?;
    output.write_all(b" decision=")?;
    serde_json::to_writer(&mut *output, &row.decision)?;
    writeln!(output)?;
    Ok(())
}

fn status_name(status: AmbiguityStatus) -> &'static str {
    match status {
        AmbiguityStatus::Selected => "selected",
        AmbiguityStatus::ParseFailure => "parse_failure",
        AmbiguityStatus::UnresolvedTie => "unresolved_tie",
        AmbiguityStatus::InternalFailure => "internal_failure",
    }
}

fn apply_exit_predicates(report: &AmbiguityReport, require_resolved: bool) -> anyhow::Result<()> {
    let summary = &report.summary;
    if summary.internal_failures > 0 {
        bail!(
            "English-v2 ambiguity census found {}",
            counted(
                summary.internal_failures,
                "internal failure",
                "internal failures"
            )
        )
    }
    if require_resolved && summary.unresolved_ties > 0 {
        bail!(
            "English-v2 ambiguity census has {}",
            counted(summary.unresolved_ties, "unresolved tie", "unresolved ties")
        )
    }
    Ok(())
}

fn counted(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
}

#[cfg(test)]
mod tests {
    use super::AmbiguityReport;
    use super::AmbiguityRow;
    use super::AmbiguityStatus;
    use super::InternalKind;
    use super::SelectionResolution;
    use super::apply_exit_predicates;
    use super::render_report;
    use crate::english_v2::corpus::Corpus;
    use crate::english_v2::corpus::CorpusUnit;

    fn complete_fixture_report() -> AmbiguityReport {
        AmbiguityReport::fixture_with_every_status()
    }

    #[test]
    fn corpus_runner_preserves_each_oracle_text_in_source_order() {
        let corpus = Corpus::from_units_for_test(vec![
            CorpusUnit::for_test("Two Blocks", "Destroy target creature.\nYou gain 2 life."),
            CorpusUnit::for_test("Failed", "You frobnitz a card."),
        ]);
        let parser = crate::english_v2::parser_from_builtin_v2().unwrap();
        let report = AmbiguityReport::run(&corpus, &parser, 1).unwrap();

        assert_eq!(report.rows()[0].card_name, corpus.units()[0].card_name());
        assert_eq!(report.rows()[1].card_name, corpus.units()[1].card_name());
        assert_eq!(report.rows()[0].status(), AmbiguityStatus::ParseFailure);
        assert_eq!(report.rows()[1].status(), AmbiguityStatus::Selected);
    }

    #[test]
    fn complete_rows_preserve_every_identity_status_and_decision_in_corpus_order() {
        let report = complete_fixture_report();
        let rows = report.rows();

        assert_eq!(report.schema_version(), 1);
        assert_eq!(report.source_fingerprint(), "f".repeat(64));
        assert_eq!(
            rows.iter().map(AmbiguityRow::status).collect::<Vec<_>>(),
            [
                AmbiguityStatus::Selected,
                AmbiguityStatus::Selected,
                AmbiguityStatus::Selected,
                AmbiguityStatus::ParseFailure,
                AmbiguityStatus::UnresolvedTie,
                AmbiguityStatus::InternalFailure,
            ]
        );
        assert_eq!(
            rows[0].decision().unwrap().resolution(),
            SelectionResolution::Unique
        );
        assert_eq!(
            rows[1].decision().unwrap().resolution(),
            SelectionResolution::Specificity
        );
        assert_eq!(
            rows[2].decision().unwrap().resolution(),
            SelectionResolution::Exception
        );
        assert_eq!(rows[4].decision().unwrap().survivors(), [0, 1, 2]);
        assert_eq!(rows[4].decision().unwrap().comparisons().len(), 3);
        assert_eq!(
            rows[5].internal_kind(),
            Some(InternalKind::SelectionConfiguration)
        );
        assert!(rows.iter().all(|row| row.id().len() == 64));
    }

    #[test]
    fn json_keeps_all_rows_and_complete_pair_comparisons() {
        let report = complete_fixture_report();
        let mut output = Vec::new();

        render_report(&report, true, &mut output).unwrap();

        let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(json["schema_version"], 1);
        assert_eq!(json["rows"].as_array().unwrap().len(), 6);
        assert_eq!(
            json["rows"][1]["decision"]["comparisons"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            json["rows"][1]["decision"]["comparisons"][0]["decisive"]["position"]["index"],
            0
        );
        assert_eq!(
            json["rows"][2]["decision"]["comparisons"][0]["exception_id"],
            "fixture-right-wins"
        );
        assert_eq!(
            json["rows"][2]["decision"]["exception_uses"],
            serde_json::json!(["fixture-right-wins"])
        );
        assert_eq!(
            json["rows"][1]["packed_sites"],
            serde_json::json!([{
                "construction_path": ["Fixture"],
                "role": "mobile",
                "paths": [[
                    {"role": {"name": "host"}},
                    {"conjunct": {"name": "members", "ordinal": 1}}
                ]]
            }])
        );
        assert_eq!(
            json["rows"][4]["decision"]["survivors"],
            serde_json::json!([0, 1, 2])
        );
        assert_eq!(
            json["rows"][4]["decision"]["comparisons"]
                .as_array()
                .unwrap()
                .len(),
            3
        );
        assert_eq!(json["rows"][5]["internal_kind"], "selection_configuration");
        assert_eq!(
            json["summary"],
            serde_json::json!({
                "total": 6,
                "selected": 3,
                "unique": 1,
                "specificity_resolved": 1,
                "exception_resolved": 1,
                "unresolved_ties": 1,
                "parse_failures": 1,
                "internal_failures": 1,
                "exception_uses": 1,
            })
        );
    }

    #[test]
    fn human_rendering_prints_every_row_with_json_escaped_variable_fields() {
        let mut report = complete_fixture_report();
        report.rows_mut()[3].set_message(Some("failed\nwith\ttab".to_owned()));
        report.rows_mut()[3].set_text("first\nsecond\"\\".to_owned());
        let mut output = Vec::new();

        render_report(&report, false, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert_eq!(
            output
                .lines()
                .filter(|line| line.starts_with("row "))
                .count(),
            6
        );
        assert!(output.contains("status=parse_failure"));
        assert!(output.contains("text=\"first\\nsecond\\\"\\\\\""));
        assert!(output.contains("message=\"failed\\nwith\\ttab\""));
        assert!(!output.contains("failed\nwith"));
    }

    #[test]
    fn summary_arithmetic_and_exception_uses_are_exact() {
        let report = complete_fixture_report();
        let summary = report.summary();

        assert_eq!(
            summary.total,
            summary.selected
                + summary.unresolved_ties
                + summary.parse_failures
                + summary.internal_failures
        );
        assert_eq!(
            summary.selected,
            summary.unique + summary.specificity_resolved + summary.exception_resolved
        );
        assert_eq!(summary.exception_uses, 1);
        assert_eq!(summary.packed_units, 1);
        assert_eq!(
            summary.packed_unit_ids,
            ["0000000000000000000000000000000000000000000000000000000000000002"]
        );
    }

    #[test]
    fn resolved_gate_ignores_parse_failures_but_not_ties_or_internal_failures() {
        let report = complete_fixture_report();
        let parse_only = report.with_statuses(&[AmbiguityStatus::ParseFailure]);
        assert!(apply_exit_predicates(&parse_only, true).is_ok());

        let tie_error = apply_exit_predicates(
            &report.with_statuses(&[AmbiguityStatus::UnresolvedTie]),
            true,
        )
        .unwrap_err()
        .to_string();
        assert!(tie_error.contains("unresolved tie"));

        let internal_error = apply_exit_predicates(
            &report.with_statuses(&[AmbiguityStatus::InternalFailure]),
            false,
        )
        .unwrap_err()
        .to_string();
        assert!(internal_error.contains("internal failure"));
    }

    #[test]
    fn strict_exit_happens_after_the_complete_report() {
        let report = complete_fixture_report().with_statuses(&[AmbiguityStatus::UnresolvedTie]);
        let mut output = Vec::new();

        let error = super::render_then_apply(&report, true, true, &mut output)
            .unwrap_err()
            .to_string();

        assert!(error.contains("unresolved tie"));
        let json: serde_json::Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(json["rows"].as_array().unwrap().len(), 1);
        assert_eq!(json["summary"]["unresolved_ties"], 1);
    }

    #[test]
    fn typed_internal_kinds_remain_distinct() {
        let report = complete_fixture_report();
        let materialization =
            report.with_internal_kind(InternalKind::ValidatedRootDidNotMaterialize);
        let configuration = report.with_internal_kind(InternalKind::SelectionConfiguration);
        let ownership = report.with_internal_kind(InternalKind::OwnershipInspection);

        assert_eq!(
            materialization.rows()[0].internal_kind(),
            Some(InternalKind::ValidatedRootDidNotMaterialize)
        );
        assert_eq!(
            configuration.rows()[0].internal_kind(),
            Some(InternalKind::SelectionConfiguration)
        );
        assert_eq!(
            ownership.rows()[0].internal_kind(),
            Some(InternalKind::OwnershipInspection)
        );
    }
}
