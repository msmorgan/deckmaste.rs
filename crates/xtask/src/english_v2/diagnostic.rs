use std::cmp::Ordering;
use std::io::Write;

use anyhow::Context;
use deckmaste_english_v2::parser::Bounded;
use deckmaste_english_v2::parser::BoundedParseOutcome;
use deckmaste_english_v2::parser::BoundedSelectionCandidate as RuntimeSelectionCandidate;
use deckmaste_english_v2::parser::BoundedSelectionComparison as RuntimeSelectionComparison;
use deckmaste_english_v2::parser::BoundedSelectionDecision as RuntimeSelectionDecision;
use deckmaste_english_v2::parser::ExpectationInfo as RuntimeExpectation;
use deckmaste_english_v2::parser::FamilyIdentity as RuntimeFamilyIdentity;
use deckmaste_english_v2::parser::FamilyIdentityChild as RuntimeFamilyIdentityChild;
use deckmaste_english_v2::parser::InternalFailureKind as RuntimeInternalFailureKind;
use deckmaste_english_v2::parser::NonterminalCategory;
use deckmaste_english_v2::parser::ParserTrace;
use deckmaste_english_v2::parser::SelectionDecisive as RuntimeSelectionDecisive;
use deckmaste_english_v2::parser::SelectionResolution as RuntimeSelectionResolution;
use deckmaste_english_v2::parser::SpecificityTier as RuntimeSpecificityTier;
use deckmaste_english_v2::parser::TerminalClass;
use deckmaste_english_v2::parser::UnselectedCandidate as RuntimeUnselectedCandidate;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct DiagnosticReport {
    schema_version: u32,
    source: DiagnosticSource,
    trace: DiagnosticTrace,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct DiagnosticSource {
    kind: SourceKind,
    text: String,
    context: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SourceKind {
    Probe,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct DiagnosticTrace {
    tokens: Counted<ScannedToken>,
    final_chart: Counted<ChartItem>,
    forest_nodes: Counted<ForestNode>,
    forest_roots: Counted<usize>,
    checked_completion_rejections: Counted<CheckedCompletionRejection>,
    materialized_candidates: Counted<MaterializedCandidate>,
    materialization_cycles: Counted<MaterializationCycle>,
    outcome: DiagnosticOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct Counted<T> {
    total: usize,
    shown: usize,
    omitted: usize,
    items: Vec<T>,
}

impl<T> Counted<T> {
    fn from_runtime<U>(bounded: &Bounded<U>, project: impl Fn(&U) -> T) -> Self {
        Self {
            total: bounded.total(),
            shown: bounded.shown(),
            omitted: bounded.omitted(),
            items: bounded.items().iter().map(project).collect(),
        }
    }

    #[cfg(test)]
    fn fixture(total: usize, items: Vec<T>) -> Self {
        let shown = items.len();
        Self {
            total,
            shown,
            omitted: total - shown,
            items,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ScannedToken {
    start: usize,
    end: usize,
    terminal_name_v1: String,
    value_label_v1: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ChartItem {
    column: usize,
    rule_name_v1: String,
    dot: usize,
    origin: usize,
    family_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ForestNode {
    id: usize,
    rule_name_v1: String,
    start: usize,
    end: usize,
    families: Counted<ForestFamily>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ForestFamily {
    children: Counted<ForestChild>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ForestChild {
    node_id: Option<usize>,
    value_label_v1: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct CheckedCompletionRejection {
    rule_name_v1: String,
    start: usize,
    end: usize,
    family_identity_v1: FamilyIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct FamilyIdentity {
    children: Vec<FamilyIdentityChild>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum FamilyIdentityChild {
    Node(usize),
    Lexical(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct MaterializedCandidate {
    ordinal: usize,
    rendered: String,
    ast_debug_v1: String,
    construction_path: Counted<String>,
    specificity: Counted<SpecificityTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct MaterializationCycle {
    node_ordinal: usize,
    construction_path: Counted<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SpecificityTier {
    Nonterminal,
    TypedLexical,
    Literal,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum DiagnosticOutcome {
    Selected(SelectedOutcome),
    ParseFailure(ParseFailureOutcome),
    UnresolvedAmbiguity(UnresolvedOutcome),
    InternalFailure(InternalFailureOutcome),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectedOutcome {
    rendered: String,
    selection: SelectionDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct ParseFailureOutcome {
    span: TextSpan,
    expectations: Counted<Expectation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct TextSpan {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
enum Expectation {
    Nonterminal(NonterminalKind),
    Terminal(TerminalKind),
    Literal(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum NonterminalKind {
    Ability,
    Sentence,
    Clause,
    NounPhrase,
    VerbPhrase,
    Amount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum TerminalKind {
    Article,
    Demonstrative,
    EndOfInput,
    Noun,
    Pronoun,
    SelfReference,
    SignedNumber,
    TriggerWord,
    Variable,
    VerbLexeme,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct UnresolvedOutcome {
    selection: SelectionDecision,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct InternalFailureOutcome {
    kind: InternalFailureKind,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum InternalFailureKind {
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
}

impl InternalFailureKind {
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::ValidatedRootDidNotMaterialize => "validated_root_did_not_materialize",
            Self::SelectionConfiguration => "selection_configuration",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectionDecision {
    candidates: Counted<SelectionCandidate>,
    comparisons: Counted<SelectionComparison>,
    survivors: Counted<usize>,
    exception_uses: Counted<String>,
    selected: Option<SelectionCandidate>,
    resolution: SelectionResolution,
    unselected_candidates: Counted<UnselectedCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectionCandidate {
    ordinal: usize,
    construction_path: Counted<String>,
    specificity: Counted<SpecificityTier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct SelectionComparison {
    left_ordinal: usize,
    right_ordinal: usize,
    ordering: OrderingKind,
    decisive: SelectionDecisive,
    exception_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum OrderingKind {
    Less,
    Equal,
    Greater,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", content = "index", rename_all = "snake_case")]
enum SelectionDecisive {
    Position(usize),
    VectorExhaustion(usize),
    Tie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SelectionResolution {
    Unique,
    Specificity,
    Exception,
    UnresolvedTie,
}

impl SelectionResolution {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Unique => "unique",
            Self::Specificity => "specificity",
            Self::Exception => "exception",
            Self::UnresolvedTie => "unresolved_tie",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct UnselectedCandidate {
    candidate: SelectionCandidate,
    reason: SelectionLoserReason,
    evidence: Counted<SelectionComparison>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SelectionLoserReason {
    LessSpecific,
    ExceptionLoser,
    UnresolvedSurvivor,
}

impl DiagnosticReport {
    pub(super) fn from_probe(text: &str, context: &str, trace: &ParserTrace) -> Self {
        Self {
            schema_version: 1,
            source: DiagnosticSource {
                kind: SourceKind::Probe,
                text: text.to_owned(),
                context: context.to_owned(),
            },
            trace: DiagnosticTrace::from_runtime(trace),
        }
    }

    pub(super) fn internal_failure(&self) -> Option<(InternalFailureKind, &str)> {
        match &self.trace.outcome {
            DiagnosticOutcome::InternalFailure(failure) => {
                Some((failure.kind, failure.message.as_str()))
            }
            DiagnosticOutcome::Selected(_)
            | DiagnosticOutcome::ParseFailure(_)
            | DiagnosticOutcome::UnresolvedAmbiguity(_) => None,
        }
    }
}

impl DiagnosticTrace {
    fn from_runtime(trace: &ParserTrace) -> Self {
        Self {
            tokens: Counted::from_runtime(trace.tokens(), |token| ScannedToken {
                start: token.start(),
                end: token.end(),
                terminal_name_v1: token.terminal_name_v1().to_owned(),
                value_label_v1: token.value_label_v1().to_owned(),
            }),
            final_chart: Counted::from_runtime(trace.chart(), |item| ChartItem {
                column: item.column(),
                rule_name_v1: item.rule_name_v1().to_owned(),
                dot: item.dot(),
                origin: item.origin(),
                family_count: item.family_count(),
            }),
            forest_nodes: Counted::from_runtime(trace.forest(), |node| ForestNode {
                id: node.id(),
                rule_name_v1: node.rule_name_v1().to_owned(),
                start: node.start(),
                end: node.end(),
                families: Counted::from_runtime(node.families(), |family| ForestFamily {
                    children: Counted::from_runtime(family.children(), |child| ForestChild {
                        node_id: child.node_id(),
                        value_label_v1: child.value_label_v1().map(str::to_owned),
                    }),
                }),
            }),
            forest_roots: Counted::from_runtime(trace.accepted_roots(), |root| *root),
            checked_completion_rejections: Counted::from_runtime(
                trace.checked_completion_rejections(),
                |rejection| CheckedCompletionRejection {
                    rule_name_v1: rejection.rule_name_v1().to_owned(),
                    start: rejection.start(),
                    end: rejection.end(),
                    family_identity_v1: family_identity(rejection.family_identity_v1()),
                },
            ),
            materialized_candidates: Counted::from_runtime(
                trace.materialized_candidates(),
                |candidate| MaterializedCandidate {
                    ordinal: candidate.ordinal(),
                    rendered: candidate.rendered().to_owned(),
                    ast_debug_v1: candidate.ast_debug_v1().to_owned(),
                    construction_path: Counted::from_runtime(
                        candidate.construction_path(),
                        Clone::clone,
                    ),
                    specificity: Counted::from_runtime(candidate.specificity(), |tier| {
                        specificity(*tier)
                    }),
                },
            ),
            materialization_cycles: Counted::from_runtime(
                trace.materialization_cycles(),
                |cycle| MaterializationCycle {
                    node_ordinal: cycle.node_ordinal(),
                    construction_path: Counted::from_runtime(
                        cycle.construction_path(),
                        Clone::clone,
                    ),
                },
            ),
            outcome: outcome(trace.outcome()),
        }
    }
}

fn family_identity(identity: &RuntimeFamilyIdentity) -> FamilyIdentity {
    FamilyIdentity {
        children: identity
            .children()
            .iter()
            .map(|child| match child {
                RuntimeFamilyIdentityChild::Node(node) => FamilyIdentityChild::Node(*node),
                RuntimeFamilyIdentityChild::Lexical(value) => {
                    FamilyIdentityChild::Lexical(value.clone())
                }
            })
            .collect(),
    }
}

fn outcome(outcome: &BoundedParseOutcome) -> DiagnosticOutcome {
    match outcome {
        BoundedParseOutcome::Selected(selected) => DiagnosticOutcome::Selected(SelectedOutcome {
            rendered: selected.rendered().to_owned(),
            selection: selection(selected.selection()),
        }),
        BoundedParseOutcome::ParseFailure(failure) => {
            DiagnosticOutcome::ParseFailure(ParseFailureOutcome {
                span: TextSpan {
                    start: failure.span().start,
                    end: failure.span().end,
                },
                expectations: Counted::from_runtime(failure.expectations(), expectation),
            })
        }
        BoundedParseOutcome::UnresolvedAmbiguity(unresolved) => {
            DiagnosticOutcome::UnresolvedAmbiguity(UnresolvedOutcome {
                selection: selection(unresolved.selection()),
            })
        }
        BoundedParseOutcome::InternalFailure(failure) => {
            DiagnosticOutcome::InternalFailure(InternalFailureOutcome {
                kind: internal_kind(failure.kind()),
                message: failure.message().to_owned(),
            })
        }
    }
}

fn expectation(expectation: &RuntimeExpectation) -> Expectation {
    match expectation {
        RuntimeExpectation::Nonterminal(kind) => Expectation::Nonterminal(nonterminal(*kind)),
        RuntimeExpectation::Terminal(kind) => Expectation::Terminal(terminal(*kind)),
        RuntimeExpectation::Literal(value) => Expectation::Literal((*value).to_owned()),
    }
}

fn nonterminal(kind: NonterminalCategory) -> NonterminalKind {
    match kind {
        NonterminalCategory::Ability => NonterminalKind::Ability,
        NonterminalCategory::Sentence => NonterminalKind::Sentence,
        NonterminalCategory::Clause => NonterminalKind::Clause,
        NonterminalCategory::NounPhrase => NonterminalKind::NounPhrase,
        NonterminalCategory::VerbPhrase => NonterminalKind::VerbPhrase,
        NonterminalCategory::Amount => NonterminalKind::Amount,
    }
}

fn terminal(kind: TerminalClass) -> TerminalKind {
    match kind {
        TerminalClass::Article => TerminalKind::Article,
        TerminalClass::Demonstrative => TerminalKind::Demonstrative,
        TerminalClass::EndOfInput => TerminalKind::EndOfInput,
        TerminalClass::Noun => TerminalKind::Noun,
        TerminalClass::Pronoun => TerminalKind::Pronoun,
        TerminalClass::SelfReference => TerminalKind::SelfReference,
        TerminalClass::SignedNumber => TerminalKind::SignedNumber,
        TerminalClass::TriggerWord => TerminalKind::TriggerWord,
        TerminalClass::Variable => TerminalKind::Variable,
        TerminalClass::VerbLexeme => TerminalKind::VerbLexeme,
    }
}

fn selection(decision: &RuntimeSelectionDecision) -> SelectionDecision {
    SelectionDecision {
        candidates: Counted::from_runtime(decision.candidates(), selection_candidate),
        comparisons: Counted::from_runtime(decision.comparisons(), selection_comparison),
        survivors: Counted::from_runtime(decision.survivors(), |ordinal| *ordinal),
        exception_uses: Counted::from_runtime(decision.exception_uses(), Clone::clone),
        selected: decision.selected().map(selection_candidate),
        resolution: selection_resolution(decision.resolution()),
        unselected_candidates: Counted::from_runtime(
            decision.unselected_candidates(),
            unselected_candidate,
        ),
    }
}

fn selection_candidate(candidate: &RuntimeSelectionCandidate) -> SelectionCandidate {
    SelectionCandidate {
        ordinal: candidate.ordinal(),
        construction_path: Counted::from_runtime(candidate.construction_path(), Clone::clone),
        specificity: Counted::from_runtime(candidate.specificity(), |tier| specificity(*tier)),
    }
}

fn selection_comparison(comparison: &RuntimeSelectionComparison) -> SelectionComparison {
    SelectionComparison {
        left_ordinal: comparison.left_ordinal(),
        right_ordinal: comparison.right_ordinal(),
        ordering: ordering(comparison.ordering()),
        decisive: decisive(comparison.decisive()),
        exception_id: comparison.exception_id().map(str::to_owned),
    }
}

fn unselected_candidate(candidate: &RuntimeUnselectedCandidate) -> UnselectedCandidate {
    UnselectedCandidate {
        candidate: selection_candidate(candidate.candidate()),
        reason: match candidate.reason() {
            deckmaste_english_v2::parser::SelectionLoserReason::LessSpecific => {
                SelectionLoserReason::LessSpecific
            }
            deckmaste_english_v2::parser::SelectionLoserReason::ExceptionLoser => {
                SelectionLoserReason::ExceptionLoser
            }
            deckmaste_english_v2::parser::SelectionLoserReason::UnresolvedSurvivor => {
                SelectionLoserReason::UnresolvedSurvivor
            }
        },
        evidence: Counted::from_runtime(candidate.evidence(), selection_comparison),
    }
}

fn ordering(value: Ordering) -> OrderingKind {
    match value {
        Ordering::Less => OrderingKind::Less,
        Ordering::Equal => OrderingKind::Equal,
        Ordering::Greater => OrderingKind::Greater,
    }
}

fn decisive(value: RuntimeSelectionDecisive) -> SelectionDecisive {
    match value {
        RuntimeSelectionDecisive::Position(index) => SelectionDecisive::Position(index),
        RuntimeSelectionDecisive::VectorExhaustion(index) => {
            SelectionDecisive::VectorExhaustion(index)
        }
        RuntimeSelectionDecisive::Tie => SelectionDecisive::Tie,
    }
}

fn specificity(value: RuntimeSpecificityTier) -> SpecificityTier {
    match value {
        RuntimeSpecificityTier::Nonterminal => SpecificityTier::Nonterminal,
        RuntimeSpecificityTier::TypedLexical => SpecificityTier::TypedLexical,
        RuntimeSpecificityTier::Literal => SpecificityTier::Literal,
    }
}

fn selection_resolution(value: RuntimeSelectionResolution) -> SelectionResolution {
    match value {
        RuntimeSelectionResolution::Unique => SelectionResolution::Unique,
        RuntimeSelectionResolution::Specificity => SelectionResolution::Specificity,
        RuntimeSelectionResolution::Exception => SelectionResolution::Exception,
        RuntimeSelectionResolution::UnresolvedTie => SelectionResolution::UnresolvedTie,
    }
}

fn internal_kind(value: RuntimeInternalFailureKind) -> InternalFailureKind {
    match value {
        RuntimeInternalFailureKind::ValidatedRootDidNotMaterialize => {
            InternalFailureKind::ValidatedRootDidNotMaterialize
        }
        RuntimeInternalFailureKind::SelectionConfiguration => {
            InternalFailureKind::SelectionConfiguration
        }
    }
}

pub(super) fn render(
    report: &DiagnosticReport,
    json: bool,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    if json {
        serde_json::to_writer_pretty(&mut *output, report)
            .context("write English-v2 probe JSON")?;
        writeln!(output).context("writing English-v2 probe JSON terminator")?;
    } else {
        render_human(report, output)?;
    }
    Ok(())
}

fn render_human(report: &DiagnosticReport, output: &mut dyn Write) -> anyhow::Result<()> {
    writeln!(
        output,
        "English v2 diagnostic schema_version={} source_kind=probe text={} context={}",
        report.schema_version,
        json_value(&report.source.text)?,
        json_value(&report.source.context)?,
    )
    .context("writing English-v2 probe human header")?;
    let trace = &report.trace;
    write_counted("tokens", &trace.tokens, output)?;
    write_items("token", &trace.tokens.items, output)?;
    write_counted("final_chart", &trace.final_chart, output)?;
    write_items("chart_item", &trace.final_chart.items, output)?;
    write_counted("forest_nodes", &trace.forest_nodes, output)?;
    for (node_index, node) in trace.forest_nodes.items.iter().enumerate() {
        writeln!(
            output,
            "forest_node index={node_index} id={} rule_name_v1={} start={} end={}",
            node.id,
            json_value(&node.rule_name_v1)?,
            node.start,
            node.end,
        )
        .context("writing English-v2 probe forest node")?;
        write_counted(
            &format!("forest_nodes[{node_index}].families"),
            &node.families,
            output,
        )?;
        for (family_index, family) in node.families.items.iter().enumerate() {
            let name = format!("forest_nodes[{node_index}].families[{family_index}].children");
            write_counted(&name, &family.children, output)?;
            write_items(&format!("{name}.child"), &family.children.items, output)?;
        }
    }
    write_counted("forest_roots", &trace.forest_roots, output)?;
    write_items("forest_root", &trace.forest_roots.items, output)?;
    write_counted(
        "checked_completion_rejections",
        &trace.checked_completion_rejections,
        output,
    )?;
    write_items(
        "checked_completion_rejection",
        &trace.checked_completion_rejections.items,
        output,
    )?;
    write_counted(
        "materialized_candidates",
        &trace.materialized_candidates,
        output,
    )?;
    for (index, candidate) in trace.materialized_candidates.items.iter().enumerate() {
        writeln!(
            output,
            "materialized_candidate index={index} ordinal={} rendered={} ast_debug_v1={}",
            candidate.ordinal,
            json_value(&candidate.rendered)?,
            json_value(&candidate.ast_debug_v1)?,
        )
        .context("writing English-v2 probe materialized candidate")?;
        render_candidate_nested("materialized_candidates", index, candidate, output)?;
    }
    write_counted(
        "materialization_cycles",
        &trace.materialization_cycles,
        output,
    )?;
    for (index, cycle) in trace.materialization_cycles.items.iter().enumerate() {
        writeln!(
            output,
            "materialization_cycle index={index} node_ordinal={}",
            cycle.node_ordinal
        )
        .context("writing English-v2 probe materialization cycle")?;
        let name = format!("materialization_cycles[{index}].construction_path");
        write_counted(&name, &cycle.construction_path, output)?;
        write_items(
            &format!("{name}.item"),
            &cycle.construction_path.items,
            output,
        )?;
    }
    render_outcome(&trace.outcome, output)
}

fn render_candidate_nested(
    section: &str,
    index: usize,
    candidate: &MaterializedCandidate,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    let path = format!("{section}[{index}].construction_path");
    write_counted(&path, &candidate.construction_path, output)?;
    write_items(
        &format!("{path}.item"),
        &candidate.construction_path.items,
        output,
    )?;
    let specificity = format!("{section}[{index}].specificity");
    write_counted(&specificity, &candidate.specificity, output)?;
    write_items(
        &format!("{specificity}.item"),
        &candidate.specificity.items,
        output,
    )
}

fn render_outcome(outcome: &DiagnosticOutcome, output: &mut dyn Write) -> anyhow::Result<()> {
    match outcome {
        DiagnosticOutcome::Selected(selected) => {
            writeln!(
                output,
                "outcome status=selected rendered={}",
                json_value(&selected.rendered)?
            )
            .context("writing English-v2 probe selected outcome")?;
            render_selection(&selected.selection, output)
        }
        DiagnosticOutcome::ParseFailure(failure) => {
            writeln!(
                output,
                "outcome status=parse_failure span_start={} span_end={}",
                failure.span.start, failure.span.end
            )
            .context("writing English-v2 probe parse-failure outcome")?;
            write_counted("outcome.expectations", &failure.expectations, output)?;
            write_items("outcome.expectation", &failure.expectations.items, output)
        }
        DiagnosticOutcome::UnresolvedAmbiguity(unresolved) => {
            writeln!(output, "outcome status=unresolved_ambiguity")
                .context("writing English-v2 probe unresolved outcome")?;
            render_selection(&unresolved.selection, output)
        }
        DiagnosticOutcome::InternalFailure(failure) => writeln!(
            output,
            "outcome status=internal_failure kind={} message={}",
            failure.kind.as_str(),
            json_value(&failure.message)?
        )
        .context("writing English-v2 probe internal outcome"),
    }
}

fn render_selection(selection: &SelectionDecision, output: &mut dyn Write) -> anyhow::Result<()> {
    writeln!(
        output,
        "selection resolution={} selected={}",
        selection.resolution.as_str(),
        json_value(&selection.selected)?,
    )
    .context("writing English-v2 probe selection")?;
    write_counted("selection.candidates", &selection.candidates, output)?;
    for (index, candidate) in selection.candidates.items.iter().enumerate() {
        writeln!(
            output,
            "selection_candidate index={index} ordinal={}",
            candidate.ordinal
        )
        .context("writing English-v2 probe selection candidate")?;
        render_selection_candidate_nested("selection.candidates", index, candidate, output)?;
    }
    write_counted("selection.comparisons", &selection.comparisons, output)?;
    write_items("selection_comparison", &selection.comparisons.items, output)?;
    write_counted("selection.survivors", &selection.survivors, output)?;
    write_items("selection_survivor", &selection.survivors.items, output)?;
    write_counted(
        "selection.exception_uses",
        &selection.exception_uses,
        output,
    )?;
    write_items(
        "selection_exception_use",
        &selection.exception_uses.items,
        output,
    )?;
    if let Some(selected) = &selection.selected {
        render_selection_candidate_nested("selection.selected", 0, selected, output)?;
    }
    write_counted(
        "selection.unselected_candidates",
        &selection.unselected_candidates,
        output,
    )?;
    for (index, unselected) in selection.unselected_candidates.items.iter().enumerate() {
        writeln!(
            output,
            "selection_unselected_candidate index={index} item={}",
            json_value(unselected)?
        )
        .context("writing English-v2 probe unselected candidate")?;
        render_selection_candidate_nested(
            "selection.unselected_candidates.candidate",
            index,
            &unselected.candidate,
            output,
        )?;
        let evidence = format!("selection.unselected_candidates[{index}].evidence");
        write_counted(&evidence, &unselected.evidence, output)?;
        write_items(
            &format!("{evidence}.comparison"),
            &unselected.evidence.items,
            output,
        )?;
    }
    Ok(())
}

fn render_selection_candidate_nested(
    section: &str,
    index: usize,
    candidate: &SelectionCandidate,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    let path = format!("{section}[{index}].construction_path");
    write_counted(&path, &candidate.construction_path, output)?;
    write_items(
        &format!("{path}.item"),
        &candidate.construction_path.items,
        output,
    )?;
    let specificity = format!("{section}[{index}].specificity");
    write_counted(&specificity, &candidate.specificity, output)?;
    write_items(
        &format!("{specificity}.item"),
        &candidate.specificity.items,
        output,
    )
}

fn write_counted<T>(
    name: &str,
    counted: &Counted<T>,
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    writeln!(
        output,
        "{name} shown={} total={} omitted={}",
        counted.shown, counted.total, counted.omitted
    )
    .with_context(|| format!("writing English-v2 probe {name} counts"))
}

fn write_items<T: Serialize>(
    name: &str,
    items: &[T],
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    for (index, item) in items.iter().enumerate() {
        writeln!(output, "{name} index={index} item={}", json_value(item)?)
            .with_context(|| format!("writing English-v2 probe {name}"))?;
    }
    Ok(())
}

fn json_value<T: Serialize>(value: &T) -> anyhow::Result<String> {
    serde_json::to_string(value).context("encoding English-v2 probe human field")
}

#[cfg(test)]
fn render_to_vec(report: &DiagnosticReport, json: bool) -> anyhow::Result<Vec<u8>> {
    let mut output = Vec::new();
    render(report, json, &mut output)?;
    Ok(output)
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, Default)]
pub(super) enum FixtureOutcome {
    #[default]
    Selected,
    ParseFailure,
    UnresolvedAmbiguity,
    ValidatedRootDidNotMaterialize,
    SelectionConfiguration,
}

#[cfg(test)]
#[expect(
    clippy::too_many_lines,
    reason = "one complete schema fixture keeps every nested bounded section visible"
)]
pub(super) fn fixture_report(outcome: FixtureOutcome) -> DiagnosticReport {
    fn candidate(ordinal: usize) -> SelectionCandidate {
        SelectionCandidate {
            ordinal,
            construction_path: Counted::fixture(2, vec![format!("path {ordinal}\nline")]),
            specificity: Counted::fixture(2, vec![SpecificityTier::Literal]),
        }
    }
    fn comparison(
        left: usize,
        right: usize,
        ordering: OrderingKind,
        exception_id: Option<&str>,
    ) -> SelectionComparison {
        SelectionComparison {
            left_ordinal: left,
            right_ordinal: right,
            ordering,
            decisive: if exception_id.is_some() {
                SelectionDecisive::Tie
            } else {
                SelectionDecisive::Position(0)
            },
            exception_id: exception_id.map(str::to_owned),
        }
    }
    fn selection_fixture() -> SelectionDecision {
        let less = comparison(1, 0, OrderingKind::Less, None);
        let exception = comparison(2, 0, OrderingKind::Less, Some("fixture\nexception"));
        let tie = comparison(3, 0, OrderingKind::Equal, None);
        SelectionDecision {
            candidates: Counted::fixture(
                4,
                vec![candidate(0), candidate(1), candidate(2), candidate(3)],
            ),
            comparisons: Counted::fixture(3, vec![less.clone(), exception.clone(), tie.clone()]),
            survivors: Counted::fixture(2, vec![0]),
            exception_uses: Counted::fixture(2, vec!["fixture\nexception".to_owned()]),
            selected: Some(candidate(0)),
            resolution: SelectionResolution::Exception,
            unselected_candidates: Counted::fixture(
                3,
                vec![
                    UnselectedCandidate {
                        candidate: candidate(1),
                        reason: SelectionLoserReason::LessSpecific,
                        evidence: Counted::fixture(1, vec![less]),
                    },
                    UnselectedCandidate {
                        candidate: candidate(2),
                        reason: SelectionLoserReason::ExceptionLoser,
                        evidence: Counted::fixture(1, vec![exception]),
                    },
                    UnselectedCandidate {
                        candidate: candidate(3),
                        reason: SelectionLoserReason::UnresolvedSurvivor,
                        evidence: Counted::fixture(1, vec![tie]),
                    },
                ],
            ),
        }
    }

    let selection = selection_fixture();
    let outcome = match outcome {
        FixtureOutcome::Selected => DiagnosticOutcome::Selected(SelectedOutcome {
            rendered: "selected\nrender".to_owned(),
            selection,
        }),
        FixtureOutcome::ParseFailure => DiagnosticOutcome::ParseFailure(ParseFailureOutcome {
            span: TextSpan { start: 4, end: 5 },
            expectations: Counted::fixture(
                3,
                vec![
                    Expectation::Nonterminal(NonterminalKind::Ability),
                    Expectation::Terminal(TerminalKind::Noun),
                    Expectation::Literal("literal\nvalue".to_owned()),
                ],
            ),
        }),
        FixtureOutcome::UnresolvedAmbiguity => {
            DiagnosticOutcome::UnresolvedAmbiguity(UnresolvedOutcome { selection })
        }
        FixtureOutcome::ValidatedRootDidNotMaterialize => {
            DiagnosticOutcome::InternalFailure(InternalFailureOutcome {
                kind: InternalFailureKind::ValidatedRootDidNotMaterialize,
                message: "validated\nroot failure".to_owned(),
            })
        }
        FixtureOutcome::SelectionConfiguration => {
            DiagnosticOutcome::InternalFailure(InternalFailureOutcome {
                kind: InternalFailureKind::SelectionConfiguration,
                message: "selection\nconfiguration failure".to_owned(),
            })
        }
    };
    DiagnosticReport {
        schema_version: 1,
        source: DiagnosticSource {
            kind: SourceKind::Probe,
            text: "line\nbreak".to_owned(),
            context: "context\rbreak".to_owned(),
        },
        trace: DiagnosticTrace {
            tokens: Counted::fixture(
                2,
                vec![ScannedToken {
                    start: 0,
                    end: 4,
                    terminal_name_v1: "terminal\nname".to_owned(),
                    value_label_v1: "value\rlabel".to_owned(),
                }],
            ),
            final_chart: Counted::fixture(
                1,
                vec![ChartItem {
                    column: 1,
                    rule_name_v1: "rule\nname".to_owned(),
                    dot: 2,
                    origin: 0,
                    family_count: 2,
                }],
            ),
            forest_nodes: Counted::fixture(
                2,
                vec![ForestNode {
                    id: 7,
                    rule_name_v1: "forest\nrule".to_owned(),
                    start: 0,
                    end: 4,
                    families: Counted::fixture(
                        2,
                        vec![ForestFamily {
                            children: Counted::fixture(
                                2,
                                vec![ForestChild {
                                    node_id: None,
                                    value_label_v1: Some("child\nvalue".to_owned()),
                                }],
                            ),
                        }],
                    ),
                }],
            ),
            forest_roots: Counted::fixture(0, vec![]),
            checked_completion_rejections: Counted::fixture(
                1,
                vec![CheckedCompletionRejection {
                    rule_name_v1: "rejected\nrule".to_owned(),
                    start: 1,
                    end: 3,
                    family_identity_v1: FamilyIdentity {
                        children: vec![
                            FamilyIdentityChild::Node(7),
                            FamilyIdentityChild::Lexical("lexical\nvalue".to_owned()),
                        ],
                    },
                }],
            ),
            materialized_candidates: Counted::fixture(
                2,
                vec![MaterializedCandidate {
                    ordinal: 0,
                    rendered: "candidate\nrender".to_owned(),
                    ast_debug_v1: "candidate\ndebug".to_owned(),
                    construction_path: Counted::fixture(2, vec!["candidate\npath".to_owned()]),
                    specificity: Counted::fixture(2, vec![SpecificityTier::TypedLexical]),
                }],
            ),
            materialization_cycles: Counted::fixture(
                1,
                vec![MaterializationCycle {
                    node_ordinal: 9,
                    construction_path: Counted::fixture(2, vec!["cycle\npath".to_owned()]),
                }],
            ),
            outcome,
        },
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use deckmaste_english_v2::catalogs::ParserCatalogs;
    use deckmaste_english_v2::context::ParseContext;
    use deckmaste_english_v2::parser::Bounded;
    use deckmaste_english_v2::parser::BoundedParseOutcome;
    use deckmaste_english_v2::parser::Parser;
    use deckmaste_english_v2::parser::ParserTrace;
    use deckmaste_english_v2::parser::TraceLimits;
    use serde_json::Value;

    use super::*;

    fn parser() -> Parser {
        let catalogs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs");
        Parser::new(ParserCatalogs::load(&catalogs).expect("canonical catalogs load"))
    }

    fn trace(text: &str, context: &str, limit: usize) -> ParserTrace {
        let context = ParseContext::new(context).expect("fixture context is valid");
        parser().trace(text, &context, TraceLimits::new(limit))
    }

    fn assert_counts<T, U>(runtime: &Bounded<T>, projected: &Counted<U>) {
        assert_eq!(projected.total, runtime.total());
        assert_eq!(projected.shown, runtime.shown());
        assert_eq!(projected.omitted, runtime.omitted());
        assert_eq!(projected.items.len(), runtime.items().len());
    }

    fn assert_selection_candidate(
        runtime: &deckmaste_english_v2::parser::BoundedSelectionCandidate,
        projected: &SelectionCandidate,
    ) {
        assert_eq!(projected.ordinal, runtime.ordinal());
        assert_counts(runtime.construction_path(), &projected.construction_path);
        assert_eq!(
            projected.construction_path.items,
            runtime.construction_path().items()
        );
        assert_counts(runtime.specificity(), &projected.specificity);
        for (actual, dto) in runtime
            .specificity()
            .items()
            .iter()
            .zip(&projected.specificity.items)
        {
            assert_eq!(*dto, specificity(*actual));
        }
    }

    fn assert_selection(
        runtime: &deckmaste_english_v2::parser::BoundedSelectionDecision,
        projected: &SelectionDecision,
    ) {
        assert_counts(runtime.candidates(), &projected.candidates);
        for (actual, dto) in runtime
            .candidates()
            .items()
            .iter()
            .zip(&projected.candidates.items)
        {
            assert_selection_candidate(actual, dto);
        }
        assert_counts(runtime.comparisons(), &projected.comparisons);
        for (actual, dto) in runtime
            .comparisons()
            .items()
            .iter()
            .zip(&projected.comparisons.items)
        {
            assert_eq!(dto.left_ordinal, actual.left_ordinal());
            assert_eq!(dto.right_ordinal, actual.right_ordinal());
            assert_eq!(dto.ordering, ordering(actual.ordering()));
            assert_eq!(dto.decisive, decisive(actual.decisive()));
            assert_eq!(dto.exception_id.as_deref(), actual.exception_id());
        }
        assert_counts(runtime.survivors(), &projected.survivors);
        assert_eq!(projected.survivors.items, runtime.survivors().items());
        assert_counts(runtime.exception_uses(), &projected.exception_uses);
        assert_eq!(
            projected.exception_uses.items,
            runtime.exception_uses().items()
        );
        assert_eq!(
            projected.resolution,
            selection_resolution(runtime.resolution())
        );
        match (runtime.selected(), projected.selected.as_ref()) {
            (Some(actual), Some(dto)) => assert_selection_candidate(actual, dto),
            (None, None) => {}
            _ => panic!("selected metadata presence changed during projection"),
        }
        assert_counts(
            runtime.unselected_candidates(),
            &projected.unselected_candidates,
        );
        for (actual, dto) in runtime
            .unselected_candidates()
            .items()
            .iter()
            .zip(&projected.unselected_candidates.items)
        {
            assert_selection_candidate(actual.candidate(), &dto.candidate);
            assert_eq!(dto.reason, loser_reason(actual.reason()));
            assert_counts(actual.evidence(), &dto.evidence);
            for (actual_evidence, dto_evidence) in
                actual.evidence().items().iter().zip(&dto.evidence.items)
            {
                assert_eq!(dto_evidence.left_ordinal, actual_evidence.left_ordinal());
                assert_eq!(dto_evidence.right_ordinal, actual_evidence.right_ordinal());
                assert_eq!(dto_evidence.ordering, ordering(actual_evidence.ordering()));
                assert_eq!(dto_evidence.decisive, decisive(actual_evidence.decisive()));
                assert_eq!(
                    dto_evidence.exception_id.as_deref(),
                    actual_evidence.exception_id()
                );
            }
        }
    }

    fn loser_reason(
        reason: deckmaste_english_v2::parser::SelectionLoserReason,
    ) -> SelectionLoserReason {
        match reason {
            deckmaste_english_v2::parser::SelectionLoserReason::LessSpecific => {
                SelectionLoserReason::LessSpecific
            }
            deckmaste_english_v2::parser::SelectionLoserReason::ExceptionLoser => {
                SelectionLoserReason::ExceptionLoser
            }
            deckmaste_english_v2::parser::SelectionLoserReason::UnresolvedSurvivor => {
                SelectionLoserReason::UnresolvedSurvivor
            }
        }
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "one field-by-field assertion authenticates the complete public trace projection"
    )]
    fn projection_authenticates_every_public_runtime_counter_at_caps_zero_one_and_above_total() {
        let text = "Whenever a player connives, you gain X life.";
        let context = "Probe Card";

        for limit in [0, 1, usize::MAX] {
            let runtime = trace(text, context, limit);
            let projected = DiagnosticReport::from_probe(text, context, &runtime);
            assert_eq!(projected.schema_version, 1);
            assert_eq!(projected.source.kind, SourceKind::Probe);
            assert_eq!(projected.source.text, text);
            assert_eq!(projected.source.context, context);
            assert_counts(runtime.tokens(), &projected.trace.tokens);
            assert_counts(runtime.chart(), &projected.trace.final_chart);
            assert_counts(runtime.forest(), &projected.trace.forest_nodes);
            assert_counts(runtime.accepted_roots(), &projected.trace.forest_roots);
            assert_counts(
                runtime.checked_completion_rejections(),
                &projected.trace.checked_completion_rejections,
            );
            assert_counts(
                runtime.materialized_candidates(),
                &projected.trace.materialized_candidates,
            );
            assert_counts(
                runtime.materialization_cycles(),
                &projected.trace.materialization_cycles,
            );

            for (actual, dto) in runtime
                .tokens()
                .items()
                .iter()
                .zip(&projected.trace.tokens.items)
            {
                assert_eq!(dto.start, actual.start());
                assert_eq!(dto.end, actual.end());
                assert_eq!(dto.terminal_name_v1, actual.terminal_name_v1());
                assert_eq!(dto.value_label_v1, actual.value_label_v1());
            }
            for (actual, dto) in runtime
                .chart()
                .items()
                .iter()
                .zip(&projected.trace.final_chart.items)
            {
                assert_eq!(dto.column, actual.column());
                assert_eq!(dto.rule_name_v1, actual.rule_name_v1());
                assert_eq!(dto.dot, actual.dot());
                assert_eq!(dto.origin, actual.origin());
                assert_eq!(dto.family_count, actual.family_count());
            }
            for (actual, dto) in runtime
                .forest()
                .items()
                .iter()
                .zip(&projected.trace.forest_nodes.items)
            {
                assert_eq!(dto.id, actual.id());
                assert_eq!(dto.rule_name_v1, actual.rule_name_v1());
                assert_eq!(dto.start, actual.start());
                assert_eq!(dto.end, actual.end());
                assert_counts(actual.families(), &dto.families);
                for (actual_family, dto_family) in
                    actual.families().items().iter().zip(&dto.families.items)
                {
                    assert_counts(actual_family.children(), &dto_family.children);
                    for (actual_child, dto_child) in actual_family
                        .children()
                        .items()
                        .iter()
                        .zip(&dto_family.children.items)
                    {
                        assert_eq!(dto_child.node_id, actual_child.node_id());
                        assert_eq!(
                            dto_child.value_label_v1.as_deref(),
                            actual_child.value_label_v1()
                        );
                    }
                }
            }
            assert_eq!(
                projected.trace.forest_roots.items,
                runtime.accepted_roots().items()
            );
            for (actual, dto) in runtime
                .checked_completion_rejections()
                .items()
                .iter()
                .zip(&projected.trace.checked_completion_rejections.items)
            {
                assert_eq!(dto.rule_name_v1, actual.rule_name_v1());
                assert_eq!(dto.start, actual.start());
                assert_eq!(dto.end, actual.end());
                for (actual_child, dto_child) in actual
                    .family_identity_v1()
                    .children()
                    .iter()
                    .zip(&dto.family_identity_v1.children)
                {
                    match (actual_child, dto_child) {
                        (
                            deckmaste_english_v2::parser::FamilyIdentityChild::Node(actual),
                            FamilyIdentityChild::Node(dto),
                        ) => assert_eq!(actual, dto),
                        (
                            deckmaste_english_v2::parser::FamilyIdentityChild::Lexical(actual),
                            FamilyIdentityChild::Lexical(dto),
                        ) => assert_eq!(actual, dto),
                        _ => panic!("family identity child kind changed during projection"),
                    }
                }
            }
            for (actual, dto) in runtime
                .materialized_candidates()
                .items()
                .iter()
                .zip(&projected.trace.materialized_candidates.items)
            {
                assert_eq!(dto.ordinal, actual.ordinal());
                assert_eq!(dto.rendered, actual.rendered());
                assert_eq!(dto.ast_debug_v1, actual.ast_debug_v1());
                assert_counts(actual.construction_path(), &dto.construction_path);
                assert_eq!(
                    dto.construction_path.items,
                    actual.construction_path().items()
                );
                assert_counts(actual.specificity(), &dto.specificity);
            }
            for (actual, dto) in runtime
                .materialization_cycles()
                .items()
                .iter()
                .zip(&projected.trace.materialization_cycles.items)
            {
                assert_eq!(dto.node_ordinal, actual.node_ordinal());
                assert_counts(actual.construction_path(), &dto.construction_path);
                assert_eq!(
                    dto.construction_path.items,
                    actual.construction_path().items()
                );
            }
            let (
                BoundedParseOutcome::Selected(actual),
                DiagnosticOutcome::Selected(projected_outcome),
            ) = (runtime.outcome(), &projected.trace.outcome)
            else {
                panic!("fixture must remain selected");
            };
            assert_eq!(projected_outcome.rendered, actual.rendered());
            assert_selection(actual.selection(), &projected_outcome.selection);
            assert_eq!(
                DiagnosticReport::from_probe(text, context, &runtime),
                projected
            );
        }
    }

    #[test]
    fn projection_keeps_overlapping_spans_nested_omissions_rejections_and_failure_expectations() {
        let overlap = trace("You gain X life.", "You", usize::MAX);
        let overlap = DiagnosticReport::from_probe("You gain X life.", "You", &overlap);
        let spans: Vec<_> = overlap
            .trace
            .tokens
            .items
            .iter()
            .map(|token| (token.start, token.end))
            .collect();
        assert!(
            spans
                .iter()
                .any(|span| spans.iter().filter(|other| *other == span).count() > 1),
            "scanner tokens must retain overlapping successful spans: {spans:?}"
        );

        let bounded = trace(
            "Whenever a player connives, you gain X life.",
            "Probe Card",
            1,
        );
        let bounded = DiagnosticReport::from_probe(
            "Whenever a player connives, you gain X life.",
            "Probe Card",
            &bounded,
        );
        assert!(bounded.trace.forest_nodes.omitted > 0);
        assert!(bounded.trace.forest_nodes.items.iter().any(|node| {
            node.families
                .items
                .iter()
                .any(|family| family.children.omitted > 0)
                || node.families.omitted > 0
        }));

        let rejected_runtime = trace("You gains X life.", "Probe Card", usize::MAX);
        let rejected =
            DiagnosticReport::from_probe("You gains X life.", "Probe Card", &rejected_runtime);
        assert!(rejected.trace.checked_completion_rejections.total > 0);
        let DiagnosticOutcome::ParseFailure(failure) = rejected.trace.outcome else {
            panic!("fixture must remain an ordinary parse failure");
        };
        let BoundedParseOutcome::ParseFailure(actual) = rejected_runtime.outcome() else {
            panic!("runtime fixture must remain a parse failure");
        };
        assert_eq!(failure.span.start, actual.span().start);
        assert_eq!(failure.span.end, actual.span().end);
        assert_counts(actual.expectations(), &failure.expectations);
    }

    #[test]
    fn schema_v1_json_and_human_render_every_outcome_and_nested_bounded_section_deterministically()
    {
        for outcome in [
            FixtureOutcome::Selected,
            FixtureOutcome::ParseFailure,
            FixtureOutcome::UnresolvedAmbiguity,
            FixtureOutcome::ValidatedRootDidNotMaterialize,
            FixtureOutcome::SelectionConfiguration,
        ] {
            let report = fixture_report(outcome);
            let first = render_to_vec(&report, true).unwrap();
            let second = render_to_vec(&report, true).unwrap();
            assert_eq!(first, second);
            let value: Value = serde_json::from_slice(&first).unwrap();
            assert_eq!(value["schema_version"], 1);
            let round_trip: DiagnosticReport = serde_json::from_slice(&first).unwrap();
            assert_eq!(round_trip, report);

            let human = String::from_utf8(render_to_vec(&report, false).unwrap()).unwrap();
            assert!(!human.contains("..."));
            assert!(!human.contains("line\nbreak"));
            for section in [
                "tokens",
                "final_chart",
                "forest_nodes",
                "forest_roots",
                "checked_completion_rejections",
                "materialized_candidates",
                "materialization_cycles",
            ] {
                let row = human
                    .lines()
                    .find(|line| line.starts_with(section))
                    .unwrap();
                assert!(row.contains("shown="));
                assert!(row.contains("total="));
                assert!(row.contains("omitted="));
            }
        }
    }

    #[test]
    fn schema_v1_pins_status_reason_and_internal_kind_spellings() {
        let selected: Value = serde_json::from_slice(
            &render_to_vec(&fixture_report(FixtureOutcome::Selected), true).unwrap(),
        )
        .unwrap();
        assert_eq!(selected["trace"]["outcome"]["status"], "selected");
        let reasons: BTreeSet<_> =
            selected["trace"]["outcome"]["selection"]["unselected_candidates"]["items"]
                .as_array()
                .unwrap()
                .iter()
                .map(|item| item["reason"].as_str().unwrap())
                .collect();
        assert_eq!(
            reasons,
            BTreeSet::from(["exception_loser", "less_specific", "unresolved_survivor"])
        );
        for (fixture, spelling) in [
            (
                FixtureOutcome::ValidatedRootDidNotMaterialize,
                "validated_root_did_not_materialize",
            ),
            (
                FixtureOutcome::SelectionConfiguration,
                "selection_configuration",
            ),
        ] {
            let value: Value =
                serde_json::from_slice(&render_to_vec(&fixture_report(fixture), true).unwrap())
                    .unwrap();
            assert_eq!(value["trace"]["outcome"]["status"], "internal_failure");
            assert_eq!(value["trace"]["outcome"]["kind"], spelling);
        }
    }
}
