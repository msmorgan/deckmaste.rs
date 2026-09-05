use std::cmp::Ordering;

use deckmaste_construction_core::macro_def::DeclarationKind;
use deckmaste_construction_core::macro_def::GrammarPosition;
use deckmaste_construction_core::macro_def::GrammarRecipe;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::SurfaceFeature;

use super::diagnostic::Bounded;
use super::diagnostic::ChartItem;
use super::diagnostic::CheckedCompletionRejection;
use super::diagnostic::FamilyIdentity;
use super::diagnostic::FamilyIdentityChild;
use super::diagnostic::ForestChild;
use super::diagnostic::ForestFamily;
use super::diagnostic::ForestNode;
use super::diagnostic::SemanticScannerMatchInventory;
use super::diagnostic::StructuralTrace;
use super::diagnostic::TraceLimits;
use super::diagnostic::grammar_rule_name_v1;
use super::diagnostic::order_bounded_prefix;
use super::engine::ChartFailure;
use super::engine::Child;
use super::engine::CompletionDisposition;
use super::engine::Family;
use super::engine::Forest;
use super::engine::LexicalMatch;
#[cfg(test)]
use super::engine::NodeId;
use super::engine::Observation;
use super::engine::RootRule;
use super::engine::Rule;
use super::engine::StatefulLexicalMatch;
use super::engine::parse_root_observed_with_state;
use super::engine::parse_root_with_state;
use super::materialize::CheckedCompletionState;
use super::materialize::completion_has_checked_build;
use crate::constructions::BuildRejection;
use crate::constructions::CasePosition;
use crate::constructions::CatalogProvider;
use crate::constructions::Category;
use crate::constructions::DeclarationMatcher;
use crate::constructions::FeatureConstraint;
use crate::constructions::GeneratedParseRoot;
use crate::constructions::Leaf;
use crate::constructions::Lexical;
use crate::constructions::LexicalBoundary;
use crate::constructions::LexicalOwner;
use crate::constructions::LexicalTerminal;
use crate::constructions::Number;
use crate::constructions::PrefixPosition;
use crate::constructions::RULES;
use crate::constructions::RuleId;
use crate::constructions::ScanPosition;
use crate::constructions::VerbFrameKey;
use crate::constructions::scan_lexical;
use crate::context::ParseContext;
use crate::environment::DeclarationId;
use crate::environment::ParserEnvironment;
use crate::orthography::initial_surface;

pub(crate) struct SliceGrammar<'a> {
    pub(crate) environment: &'a ParserEnvironment,
    pub(crate) context: &'a ParseContext<'a>,
}

pub(crate) struct ScanInput<'a> {
    pub(crate) text: &'a str,
    pub(crate) position: ScanPosition,
    pub(crate) environment: &'a ParserEnvironment,
    pub(crate) context: &'a ParseContext<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RootRuleId {
    Grammar(RuleId),
    Adapter,
}

impl RootRuleId {
    pub(super) const fn index(self) -> usize {
        match self {
            Self::Grammar(rule) => rule.index(),
            Self::Adapter => RULES.len(),
        }
    }

    pub(super) const fn public_construction(self) -> Option<crate::constructions::Construction> {
        match self {
            Self::Grammar(rule) => rule.public_construction(),
            Self::Adapter => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootForest {
    forest: Forest<RootRuleId, Leaf, LexicalOwner>,
}

impl RootForest {
    pub(crate) fn forest(&self) -> &Forest<RootRuleId, Leaf, LexicalOwner> {
        &self.forest
    }

    #[cfg(test)]
    pub(crate) fn from_test_forest(forest: Forest<RootRuleId, Leaf, LexicalOwner>) -> Self {
        Self { forest }
    }
}

impl std::ops::Deref for RootForest {
    type Target = Forest<RootRuleId, Leaf, LexicalOwner>;

    fn deref(&self) -> &Self::Target {
        &self.forest
    }
}

pub(crate) fn parse_forest<R: GeneratedParseRoot>(
    grammar: &SliceGrammar<'_>,
    text: &str,
) -> Result<RootForest, ChartFailure<Category, Lexical>> {
    #[cfg(test)]
    super::count_pipeline_stage(super::PipelineStage::Parse);
    let rules = rules_for_root::<R>();
    let mut completion_state = CheckedCompletionState::default();
    let forest = parse_root_with_state(
        &rules,
        RootRule::family_reachable(RootRuleId::Adapter),
        text.len(),
        &initial_scan_position(),
        |lexical, offset, position, suppress_right_boundary| {
            grammar.scan_stateful(lexical, text, offset, *position, suppress_right_boundary)
        },
        |rule, family, forest| {
            completion_has_checked_build(
                &rules,
                rule,
                family,
                forest,
                grammar.context,
                &mut completion_state,
            )
        },
    )
    .map_err(project_failure)?;
    Ok(RootForest { forest })
}

type ObservedForestResult = Result<RootForest, ChartFailure<Category, Lexical>>;

pub(crate) fn parse_forest_observed<R: GeneratedParseRoot>(
    grammar: &SliceGrammar<'_>,
    text: &str,
    limits: TraceLimits,
) -> (ObservedForestResult, StructuralTrace) {
    #[cfg(test)]
    super::count_pipeline_stage(super::PipelineStage::Parse);
    let mut observation = StructuralObservation::new(limits);
    let rules = rules_for_root::<R>();
    let mut completion_state = CheckedCompletionState::default();
    let result = parse_root_observed_with_state(
        &rules,
        RootRule::family_reachable(RootRuleId::Adapter),
        text.len(),
        &initial_scan_position(),
        |lexical, offset, position, suppress_right_boundary| {
            grammar.scan_stateful(lexical, text, offset, *position, suppress_right_boundary)
        },
        |rule, family, forest| {
            completion_has_checked_build(
                &rules,
                rule,
                family,
                forest,
                grammar.context,
                &mut completion_state,
            )
        },
        &mut observation,
    )
    .map(|forest| RootForest { forest })
    .map_err(project_failure);
    #[cfg(test)]
    inject_checked_completion_rejection_for_test(&mut observation);
    let trace = observation.finish();
    (result, trace)
}

#[cfg(test)]
thread_local! {
    static CHECKED_COMPLETION_REJECTION_FOR_TEST: std::cell::Cell<Option<BuildRejection>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(super) fn with_checked_completion_rejection_for_test<T>(
    rejection: BuildRejection,
    run: impl FnOnce() -> T,
) -> T {
    struct Restore(Option<BuildRejection>);

    impl Drop for Restore {
        fn drop(&mut self) {
            CHECKED_COMPLETION_REJECTION_FOR_TEST.with(|slot| slot.set(self.0));
        }
    }

    let previous = CHECKED_COMPLETION_REJECTION_FOR_TEST.with(|slot| slot.replace(Some(rejection)));
    let _restore = Restore(previous);
    run()
}

#[cfg(test)]
fn inject_checked_completion_rejection_for_test(observation: &mut StructuralObservation) {
    CHECKED_COMPLETION_REJECTION_FOR_TEST.with(|slot| {
        let Some(rejection) = slot.get() else {
            return;
        };
        let family = Family {
            children: vec![Child::Node(NodeId(usize::MAX))],
        };
        observation.checked_completion(
            RootRuleId::Adapter,
            0,
            0,
            &family,
            &CompletionDisposition::DeferredBuildRejection(rejection),
        );
    });
}

pub(super) fn rules_for_root<R: GeneratedParseRoot>()
-> Vec<Rule<Category, LexicalTerminal, RootRuleId>> {
    let mut rules = RULES
        .iter()
        .map(|rule| Rule {
            id: RootRuleId::Grammar(rule.id),
            lhs: rule.lhs,
            rhs: rule.rhs,
        })
        .collect::<Vec<_>>();
    let rhs = R::CATEGORY.root_rule_rhs(R::EOI);
    debug_assert!(!rhs.is_empty(), "generated root has one adapter rule");
    rules.push(Rule {
        id: RootRuleId::Adapter,
        lhs: R::CATEGORY,
        rhs,
    });
    rules
}

struct StructuralObservation {
    limit: usize,
    scanner_matches: SemanticScannerMatchInventory<Lexical, (Leaf, Option<LexicalOwner>)>,
    chart: Bounded<ChartItem>,
    forest: Bounded<ForestNode>,
    roots: Bounded<usize>,
    rejections: Vec<CheckedRejectionIdentity>,
    deferred_build_rejections: Vec<(CheckedRejectionIdentity, BuildRejection)>,
}

#[derive(Clone, PartialEq, Eq)]
struct CheckedRejectionIdentity {
    rule: RootRuleId,
    start: usize,
    end: usize,
    family: RawFamilyIdentity,
}

#[derive(Clone, PartialEq, Eq)]
struct RawFamilyIdentity(Vec<RawFamilyIdentityChild>);

#[derive(Clone, PartialEq, Eq)]
enum RawFamilyIdentityChild {
    Node(usize),
    Lexical(Leaf),
}

impl StructuralObservation {
    fn new(limits: TraceLimits) -> Self {
        Self {
            limit: limits.per_collection(),
            scanner_matches: SemanticScannerMatchInventory::default(),
            chart: Bounded::new(limits.per_collection()),
            forest: Bounded::new(limits.per_collection()),
            roots: Bounded::new(limits.per_collection()),
            rejections: Vec::new(),
            deferred_build_rejections: Vec::new(),
        }
    }

    fn finish(mut self) -> StructuralTrace {
        let scanner_matches = self.scanner_matches.into_bounded_by(
            self.limit,
            stable_debug_cmp,
            stable_debug_cmp,
            terminal_name_v1,
            |(value, _owner)| value_label_v1(value),
        );
        order_bounded_prefix(&mut self.rejections, self.limit, rejection_identity_cmp);
        let mut rejections = Bounded::new(self.limit);
        for rejection in self.rejections {
            rejections.push_with(|| CheckedCompletionRejection {
                rule_name_v1: rule_name_v1(rejection.rule),
                start: rejection.start,
                end: rejection.end,
                family_identity_v1: family_identity_v1(&rejection.family),
            });
        }
        self.deferred_build_rejections.sort_by(|left, right| {
            rejection_identity_cmp(&left.0, &right.0).then_with(|| left.1.cmp(&right.1))
        });
        let first_build_rejection = self
            .deferred_build_rejections
            .first()
            .map(|(_, rejection)| *rejection);
        StructuralTrace::new(
            scanner_matches,
            self.chart,
            self.forest,
            self.roots,
            rejections,
            first_build_rejection,
        )
    }

    #[cfg(test)]
    fn record_token(&mut self, start: usize, end: usize, terminal: Lexical, value: &Leaf) {
        self.scanner_matches
            .record(start, end, terminal, (value.clone(), None));
    }

    fn record_scanned_token(
        &mut self,
        start: usize,
        end: usize,
        terminal: LexicalTerminal,
        value: &Leaf,
    ) {
        self.scanner_matches.record_projected(
            start,
            end,
            terminal,
            value,
            |terminal| terminal.matcher,
            |terminal, value| (value.clone(), terminal.owner.instantiate(value)),
        );
    }
}

impl Observation<RootRuleId, Leaf, LexicalTerminal, LexicalOwner, BuildRejection>
    for StructuralObservation
{
    fn scanned(&mut self, start: usize, terminal: LexicalTerminal, end: usize, value: &Leaf) {
        self.record_scanned_token(start, end, terminal, value);
    }

    fn checked_completion(
        &mut self,
        rule: RootRuleId,
        start: usize,
        end: usize,
        family: &Family<Leaf, LexicalOwner>,
        disposition: &CompletionDisposition<BuildRejection>,
    ) {
        let key = CheckedRejectionIdentity {
            rule,
            start,
            end,
            family: raw_family_identity(family),
        };
        match disposition {
            CompletionDisposition::Accepted => {
                self.rejections.retain(|rejection| rejection != &key);
                self.deferred_build_rejections
                    .retain(|(rejection, _)| rejection != &key);
            }
            CompletionDisposition::Rejected => {
                self.deferred_build_rejections
                    .retain(|(rejection, _)| rejection != &key);
                if !self.rejections.contains(&key) {
                    self.rejections.push(key);
                }
            }
            CompletionDisposition::DeferredBuildRejection(rejection) => {
                self.rejections.retain(|rejection| rejection != &key);
                self.deferred_build_rejections
                    .retain(|(seen, _)| seen != &key);
                self.deferred_build_rejections.push((key, *rejection));
            }
        }
    }

    fn chart_item(
        &mut self,
        column: usize,
        rule: RootRuleId,
        dot: usize,
        origin: usize,
        family_count: usize,
    ) {
        self.chart.push_with(|| ChartItem {
            column,
            rule_name_v1: rule_name_v1(rule),
            dot,
            origin,
            family_count,
        });
    }

    fn final_forest(&mut self, forest: &Forest<RootRuleId, Leaf, LexicalOwner>) {
        for root in forest.accepted_root_ids() {
            self.roots.push_with(|| root.0);
        }
        for (id, node) in forest.nodes() {
            self.forest.push_with(|| {
                let mut families = Bounded::new(self.limit);
                for family in &node.families {
                    families.push_with(|| {
                        let mut children = Bounded::new(self.limit);
                        for child in &family.children {
                            children.push_with(|| match child {
                                Child::Node(id) => ForestChild {
                                    node_id: Some(id.0),
                                    value_label_v1: None,
                                },
                                Child::Lexical(lexical) => ForestChild {
                                    node_id: None,
                                    value_label_v1: Some(value_label_v1(&lexical.value)),
                                },
                            });
                        }
                        ForestFamily { children }
                    });
                }
                ForestNode {
                    id: id.0,
                    rule_name_v1: rule_name_v1(node.rule),
                    start: node.start,
                    end: node.end,
                    families,
                }
            });
        }
    }
}

fn rule_name_v1(rule: RootRuleId) -> String {
    #[cfg(test)]
    TRACE_LABEL_COUNTS.with(|counts| {
        let mut current = counts.get();
        current.rules += 1;
        counts.set(current);
    });
    match rule {
        RootRuleId::Grammar(rule) => grammar_rule_name_v1(rule),
        RootRuleId::Adapter => "RootAdapter".to_owned(),
    }
}
fn terminal_name_v1(terminal: Lexical) -> String {
    #[cfg(test)]
    TRACE_LABEL_COUNTS.with(|counts| {
        let mut current = counts.get();
        current.terminals += 1;
        counts.set(current);
    });
    format!("{terminal:?}")
}
fn value_label_v1(value: &Leaf) -> String {
    #[cfg(test)]
    TRACE_LABEL_COUNTS.with(|counts| {
        let mut current = counts.get();
        current.values += 1;
        counts.set(current);
    });
    match value {
        Leaf::Noun {
            noun: crate::constructions::Noun::Lexeme(noun),
            number,
            onset,
            possessive_ending,
        } => format!(
            "Noun {{ noun: {noun:?}, number: {number:?}, onset: {onset:?}, possessive_ending: {possessive_ending:?} }}"
        ),
        _ => format!("{value:?}"),
    }
}
fn raw_family_identity(family: &Family<Leaf, LexicalOwner>) -> RawFamilyIdentity {
    RawFamilyIdentity(
        family
            .children
            .iter()
            .map(|child| match child {
                Child::Node(id) => RawFamilyIdentityChild::Node(id.0),
                Child::Lexical(lexical) => RawFamilyIdentityChild::Lexical(lexical.value.clone()),
            })
            .collect(),
    )
}

fn family_identity_v1(family: &RawFamilyIdentity) -> FamilyIdentity {
    #[cfg(test)]
    TRACE_LABEL_COUNTS.with(|counts| {
        let mut current = counts.get();
        current.families += 1;
        counts.set(current);
    });
    FamilyIdentity(
        family
            .0
            .iter()
            .map(|child| match child {
                RawFamilyIdentityChild::Node(id) => FamilyIdentityChild::Node(*id),
                RawFamilyIdentityChild::Lexical(value) => {
                    FamilyIdentityChild::Lexical(value_label_v1(value))
                }
            })
            .collect(),
    )
}

fn rejection_identity_cmp(
    left: &CheckedRejectionIdentity,
    right: &CheckedRejectionIdentity,
) -> Ordering {
    stable_debug_cmp(&left.rule, &right.rule)
        .then_with(|| left.start.cmp(&right.start))
        .then_with(|| left.end.cmp(&right.end))
        .then_with(|| raw_family_identity_cmp(&left.family, &right.family))
}

fn raw_family_identity_cmp(left: &RawFamilyIdentity, right: &RawFamilyIdentity) -> Ordering {
    for (left, right) in left.0.iter().zip(&right.0) {
        let ordering = match (left, right) {
            (RawFamilyIdentityChild::Node(left), RawFamilyIdentityChild::Node(right)) => {
                left.cmp(right)
            }
            (RawFamilyIdentityChild::Node(_), RawFamilyIdentityChild::Lexical(_)) => Ordering::Less,
            (RawFamilyIdentityChild::Lexical(_), RawFamilyIdentityChild::Node(_)) => {
                Ordering::Greater
            }
            (RawFamilyIdentityChild::Lexical(left), RawFamilyIdentityChild::Lexical(right)) => {
                stable_debug_cmp(left, right)
            }
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    left.0.len().cmp(&right.0.len())
}

fn stable_debug_cmp<T: std::fmt::Debug>(left: &T, right: &T) -> Ordering {
    // Schema-v1 ordering is the lexical order of the generated Debug labels.
    // Compare fixed-size stack chunks so ordering never requires an owned
    // label for an identity that the bounded projection will discard.
    const CHUNK_SIZE: usize = 128;
    let mut offset = 0;
    loop {
        let mut left_bytes = [0; CHUNK_SIZE];
        let mut right_bytes = [0; CHUNK_SIZE];
        let (left_len, left_complete) = debug_chunk(left, offset, &mut left_bytes);
        let (right_len, right_complete) = debug_chunk(right, offset, &mut right_bytes);
        let ordering = left_bytes[..left_len].cmp(&right_bytes[..right_len]);
        if ordering != Ordering::Equal {
            return ordering;
        }
        match (left_complete, right_complete) {
            (true, true) => return Ordering::Equal,
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            (false, false) => offset += CHUNK_SIZE,
        }
    }
}

fn debug_chunk<T: std::fmt::Debug>(value: &T, skip: usize, buffer: &mut [u8]) -> (usize, bool) {
    struct ChunkWriter<'a> {
        skip: usize,
        buffer: &'a mut [u8],
        len: usize,
    }

    impl std::fmt::Write for ChunkWriter<'_> {
        fn write_str(&mut self, rendered: &str) -> std::fmt::Result {
            let mut rendered = rendered.as_bytes();
            if self.skip >= rendered.len() {
                self.skip -= rendered.len();
                return Ok(());
            }
            rendered = &rendered[self.skip..];
            self.skip = 0;
            let available = self.buffer.len() - self.len;
            let copied = available.min(rendered.len());
            self.buffer[self.len..self.len + copied].copy_from_slice(&rendered[..copied]);
            self.len += copied;
            (copied == rendered.len())
                .then_some(())
                .ok_or(std::fmt::Error)
        }
    }

    let mut writer = ChunkWriter {
        skip,
        buffer,
        len: 0,
    };
    let complete = std::fmt::write(&mut writer, format_args!("{value:?}")).is_ok();
    (writer.len, complete)
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct TraceLabelCounts {
    rules: usize,
    terminals: usize,
    values: usize,
    families: usize,
}

#[cfg(test)]
thread_local! {
    static TRACE_LABEL_COUNTS: std::cell::Cell<TraceLabelCounts> =
        const { std::cell::Cell::new(TraceLabelCounts {
            rules: 0,
            terminals: 0,
            values: 0,
            families: 0,
        }) };
}

#[cfg(test)]
fn reset_trace_label_counts() {
    TRACE_LABEL_COUNTS.set(TraceLabelCounts::default());
}

#[cfg(test)]
fn trace_label_counts() -> TraceLabelCounts {
    TRACE_LABEL_COUNTS.get()
}

impl SliceGrammar<'_> {
    #[cfg(test)]
    pub(super) fn scan(
        &self,
        terminal: LexicalTerminal,
        text: &str,
        offset: usize,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        let position = if offset == 0 {
            initial_scan_position()
        } else {
            ScanPosition {
                byte_offset: offset,
                case: CasePosition::Continuation,
                prefix: PrefixPosition::WordOwnedSpace,
            }
        };
        self.scan_at(terminal, text, position)
    }

    fn scan_stateful(
        &self,
        terminal: LexicalTerminal,
        text: &str,
        offset: usize,
        position: ScanPosition,
        suppress_right_boundary: bool,
    ) -> Vec<StatefulLexicalMatch<Leaf, LexicalOwner, ScanPosition>> {
        debug_assert_eq!(offset, position.byte_offset);
        let terminal = if suppress_right_boundary {
            terminal.suppress_right_boundary()
        } else {
            terminal
        };
        self.scan_at(terminal, text, position)
            .into_iter()
            .map(|lexical| StatefulLexicalMatch {
                state: terminal.position_after(position, lexical.end),
                lexical,
            })
            .collect()
    }

    fn scan_at(
        &self,
        terminal: LexicalTerminal,
        text: &str,
        position: ScanPosition,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        let position = terminal.position_before(position);
        scan_lexical(
            &ScanInput {
                text,
                position,
                environment: self.environment,
                context: self.context,
            },
            terminal,
        )
    }
}

const fn initial_scan_position() -> ScanPosition {
    ScanPosition {
        byte_offset: 0,
        case: CasePosition::DocumentInitial,
        prefix: PrefixPosition::None,
    }
}

impl ScanInput<'_> {
    pub(crate) fn word_end(
        &self,
        running_text: &str,
        right_boundary: LexicalBoundary,
    ) -> Option<usize> {
        let offset = self.position.byte_offset;
        debug_assert!(
            self.position.case != CasePosition::DocumentInitial
                || self.position.prefix == PrefixPosition::None,
        );
        debug_assert!(self.position.prefix != PrefixPosition::None || offset == 0,);
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let remainder = self.text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let word = if matches!(
            self.position.case,
            CasePosition::DocumentInitial | CasePosition::SentenceInitial
        ) {
            initial_surface(running_text)
        } else {
            running_text.to_owned()
        };
        let end = offset + prefix + word.len();
        (remainder.starts_with(&word) && permits_right_boundary(self.text, end, right_boundary))
            .then_some(end)
    }

    pub(crate) fn identity_end(
        &self,
        exact_text: &str,
        right_boundary: LexicalBoundary,
    ) -> Option<usize> {
        let offset = self.position.byte_offset;
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let remainder = self.text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let end = offset + prefix + exact_text.len();
        (!exact_text.is_empty()
            && end > offset
            && remainder.starts_with(exact_text)
            && permits_right_boundary(self.text, end, right_boundary))
        .then_some(end)
    }

    pub(crate) fn catalog_identity_reading(
        &self,
        provider: CatalogProvider,
        right_boundary: LexicalBoundary,
    ) -> Option<(usize, std::sync::Arc<str>, Onset)> {
        let offset = self.position.byte_offset;
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let remainder = self.text.get(offset..)?;
        let surface_text = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let byte_limit = self.environment.catalog_surface_byte_limit(provider);
        let mut longest = None;
        for relative_end in surface_text
            .char_indices()
            .skip(1)
            .map(|(end, _)| end)
            .chain(std::iter::once(surface_text.len()))
            .take_while(|&end| end <= byte_limit)
        {
            let end = offset + prefix + relative_end;
            if !permits_right_boundary(self.text, end, right_boundary) {
                continue;
            }
            let Some(row) = self
                .environment
                .catalog_row_for_surface(provider, &surface_text[..relative_end])
            else {
                continue;
            };
            longest = Some((
                end,
                std::sync::Arc::from(row.canonical_identity()),
                row.onset(),
            ));
        }
        longest
    }

    pub(crate) fn punctuation_end(&self, punctuation: &str) -> Option<usize> {
        let offset = self.position.byte_offset;
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let remainder = self.text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        remainder
            .starts_with(punctuation)
            .then_some(offset + prefix + punctuation.len())
    }

    pub(crate) fn structural_surface_end(&self, surface: &str) -> Option<usize> {
        let offset = self.position.byte_offset;
        self.text
            .get(offset..)?
            .starts_with(surface)
            .then_some(offset + surface.len())
    }

    pub(crate) fn declaration_readings(
        &self,
        matcher: DeclarationMatcher,
        right_boundary: LexicalBoundary,
    ) -> Vec<(usize, DeclarationId, SurfaceFeature, Onset)> {
        lookup_declaration_readings_with_prefix(
            self.text,
            self.position.byte_offset,
            matches!(
                self.position.case,
                CasePosition::DocumentInitial | CasePosition::SentenceInitial
            ),
            self.position.prefix,
            self.environment,
            matcher.kind,
            matcher.name,
            matcher.position,
            right_boundary,
            |feature| matches_feature(matcher.feature, feature),
        )
    }

    pub(crate) fn declaration_noun_readings(
        &self,
        position: GrammarPosition,
        wanted: FeatureConstraint<Number>,
        right_boundary: LexicalBoundary,
    ) -> Vec<(usize, DeclarationId, SurfaceFeature, Onset)> {
        let offset = self.position.byte_offset;
        let initial = matches!(
            self.position.case,
            CasePosition::DocumentInitial | CasePosition::SentenceInitial
        );
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let Some(remainder) = self.text.get(offset..) else {
            return Vec::new();
        };
        let Some(surface_text) = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))
        else {
            return Vec::new();
        };
        let surface_byte_limit = if initial {
            self.environment.initial_surface_byte_limit(position)
        } else {
            self.environment.running_surface_byte_limit(position)
        };
        let mut results = Vec::new();
        for relative_end in surface_text
            .char_indices()
            .skip(1)
            .map(|(end, _)| end)
            .chain(std::iter::once(surface_text.len()))
            .take_while(|&end| end <= surface_byte_limit)
        {
            let end = offset + prefix + relative_end;
            if !permits_right_boundary(self.text, end, right_boundary) {
                continue;
            }
            let candidate = &surface_text[..relative_end];
            let readings = if initial {
                self.environment.initial_readings(position, candidate)
            } else {
                self.environment.readings(position, candidate)
            };
            for reading in readings {
                let number = match reading.feature() {
                    SurfaceFeature::Singular => Number::Singular,
                    SurfaceFeature::Plural => Number::Plural,
                    SurfaceFeature::Inflectional(_)
                    | SurfaceFeature::Fixed
                    | SurfaceFeature::BoundSuffix
                    | SurfaceFeature::BlockLabel => continue,
                };
                if matches!(wanted, FeatureConstraint::Any)
                    || matches!(wanted, FeatureConstraint::Exact(expected) if expected == number)
                {
                    results.push((
                        end,
                        reading.id().clone(),
                        reading.feature(),
                        reading.onset(),
                    ));
                }
            }
        }
        results.sort();
        results.dedup();
        results
    }

    pub(crate) fn declaration_term_readings(
        &self,
        position: GrammarPosition,
        kinds: &[DeclarationKind],
        params: Option<&[&str]>,
        feature: SurfaceFeature,
        right_boundary: LexicalBoundary,
    ) -> Vec<(
        usize,
        DeclarationId,
        Option<deckmaste_construction_core::macro_def::FixedKeywordParameterGrammar>,
        Onset,
    )> {
        let offset = self.position.byte_offset;
        let initial = matches!(
            self.position.case,
            CasePosition::DocumentInitial | CasePosition::SentenceInitial
        );
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let Some(remainder) = self.text.get(offset..) else {
            return Vec::new();
        };
        let Some(surface_text) = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))
        else {
            return Vec::new();
        };
        let surface_byte_limit = if initial {
            self.environment.initial_surface_byte_limit(position)
        } else {
            self.environment.running_surface_byte_limit(position)
        };
        let mut results = Vec::new();
        for relative_end in surface_text
            .char_indices()
            .skip(1)
            .map(|(end, _)| end)
            .chain(std::iter::once(surface_text.len()))
            .take_while(|&end| end <= surface_byte_limit)
        {
            let end = offset + prefix + relative_end;
            if !permits_right_boundary(self.text, end, right_boundary) {
                continue;
            }
            let candidate = &surface_text[..relative_end];
            let readings = if initial {
                self.environment.initial_readings(position, candidate)
            } else {
                self.environment.readings(position, candidate)
            };
            results.extend(readings.iter().filter_map(|reading| {
                let record = self
                    .environment
                    .declaration(reading.id().kind(), reading.id().name());
                (reading.feature() == feature
                    && kinds.contains(&reading.id().kind())
                    && record.is_some_and(|record| {
                        params.is_none_or(|params| {
                            record
                                .params()
                                .iter()
                                .map(AsRef::as_ref)
                                .eq(params.iter().copied())
                        })
                    }))
                .then(|| {
                    let parameter = record.and_then(|record| match record.recipe() {
                        Some(GrammarRecipe::FixedKeyword { parameter }) => parameter.clone(),
                        _ => None,
                    });
                    (end, reading.id().clone(), parameter, reading.onset())
                })
            }));
        }
        results.sort();
        results.dedup();
        results
    }

    pub(crate) fn declaration_verb_readings(
        &self,
        start: usize,
        frame: &VerbFrameKey,
        feature: SurfaceFeature,
    ) -> Vec<(usize, crate::environment::VerbInventoryReading)> {
        if start != self.position.byte_offset {
            return Vec::new();
        }
        let offset = self.position.byte_offset;
        let initial = matches!(
            self.position.case,
            CasePosition::DocumentInitial | CasePosition::SentenceInitial
        );
        let prefix = usize::from(self.position.prefix == PrefixPosition::WordOwnedSpace);
        let Some(remainder) = self.text.get(offset..) else {
            return Vec::new();
        };
        let Some(surface_text) = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))
        else {
            return Vec::new();
        };
        let surface_byte_limit = if initial {
            self.environment
                .initial_surface_byte_limit(GrammarPosition::Verb)
        } else {
            self.environment
                .running_surface_byte_limit(GrammarPosition::Verb)
        };
        let mut results = Vec::new();
        for relative_end in surface_text
            .char_indices()
            .skip(1)
            .map(|(end, _)| end)
            .chain(std::iter::once(surface_text.len()))
            .take_while(|&end| end <= surface_byte_limit)
        {
            let end = offset + prefix + relative_end;
            if !has_lexical_boundary(self.text, end) {
                continue;
            }
            let candidate = &surface_text[..relative_end];
            let readings = if initial {
                self.environment
                    .initial_verb_inventory_readings(candidate, feature, *frame)
            } else {
                self.environment
                    .verb_inventory_readings(candidate, feature, *frame)
            };
            results.extend(readings.into_iter().map(|reading| (end, reading)));
        }
        results.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.reference().cmp(right.1.reference()))
                .then_with(|| {
                    left.1
                        .frame_complement_pair_preposition()
                        .cmp(&right.1.frame_complement_pair_preposition())
                })
        });
        results.dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
        results
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "the shared lookup seam keeps synthetic generated grammars on the production scanner algorithm"
)]
#[cfg(test)]
pub(super) fn lookup_declaration_readings(
    text: &str,
    offset: usize,
    document_initial: bool,
    environment: &ParserEnvironment,
    kind: DeclarationKind,
    name: &str,
    position: GrammarPosition,
    matches_feature: impl Fn(SurfaceFeature) -> bool,
) -> Vec<(usize, DeclarationId, SurfaceFeature)> {
    lookup_declaration_readings_with_prefix(
        text,
        offset,
        document_initial,
        if document_initial {
            PrefixPosition::None
        } else {
            PrefixPosition::WordOwnedSpace
        },
        environment,
        kind,
        name,
        position,
        LexicalBoundary::Separated,
        matches_feature,
    )
    .into_iter()
    .map(|(end, id, feature, _)| (end, id, feature))
    .collect()
}

#[allow(
    clippy::too_many_arguments,
    reason = "case and prefix are independent generated scanner state"
)]
fn lookup_declaration_readings_with_prefix(
    text: &str,
    offset: usize,
    initial: bool,
    prefix_position: PrefixPosition,
    environment: &ParserEnvironment,
    kind: DeclarationKind,
    name: &str,
    position: GrammarPosition,
    right_boundary: LexicalBoundary,
    matches_feature: impl Fn(SurfaceFeature) -> bool,
) -> Vec<(usize, DeclarationId, SurfaceFeature, Onset)> {
    let prefix = usize::from(prefix_position == PrefixPosition::WordOwnedSpace);
    let Some(remainder) = text.get(offset..) else {
        return Vec::new();
    };
    let Some(surface_text) = (prefix == 0)
        .then_some(remainder)
        .or_else(|| remainder.strip_prefix(' '))
    else {
        return Vec::new();
    };

    let mut results = Vec::new();
    let surface_byte_limit = if initial {
        environment.initial_surface_byte_limit(position)
    } else {
        environment.running_surface_byte_limit(position)
    };
    let candidate_ends = surface_text
        .char_indices()
        .skip(1)
        .map(|(end, _)| end)
        .chain(std::iter::once(surface_text.len()))
        .take_while(|&end| end <= surface_byte_limit);
    for relative_end in candidate_ends {
        let end = offset + prefix + relative_end;
        if !permits_right_boundary(text, end, right_boundary) {
            continue;
        }
        let candidate = &surface_text[..relative_end];
        let readings = if initial {
            environment.initial_readings(position, candidate)
        } else {
            environment.readings(position, candidate)
        };
        for reading in readings {
            if reading.id().kind() == kind
                && reading.id().name() == name
                && matches_feature(reading.feature())
            {
                results.push((
                    end,
                    reading.id().clone(),
                    reading.feature(),
                    reading.onset(),
                ));
            }
        }
    }
    results.sort();
    results.dedup();
    results
}

fn permits_right_boundary(text: &str, end: usize, boundary: LexicalBoundary) -> bool {
    matches!(
        boundary,
        LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent
    ) || has_lexical_boundary(text, end)
}

fn project_failure(
    failure: ChartFailure<Category, LexicalTerminal>,
) -> ChartFailure<Category, Lexical> {
    ChartFailure {
        offset: failure.offset,
        live: failure
            .live
            .into_iter()
            .map(|position| match position {
                super::engine::RulePosition::Nonterminal(category) => {
                    super::engine::RulePosition::Nonterminal(category)
                }
                super::engine::RulePosition::AdjacentNonterminal(category) => {
                    super::engine::RulePosition::AdjacentNonterminal(category)
                }
                super::engine::RulePosition::Lexical(terminal) => {
                    super::engine::RulePosition::Lexical(terminal.matcher)
                }
            })
            .collect(),
    }
}

fn has_lexical_boundary(text: &str, end: usize) -> bool {
    let Some(remainder) = text.get(end..) else {
        return false;
    };
    remainder.is_empty()
        || matches!(
            remainder.chars().next(),
            Some(' ' | '\n' | ',' | '.' | ':' | ']' | '}' | '—')
        )
}

fn matches_feature(constraint: FeatureConstraint<SurfaceFeature>, feature: SurfaceFeature) -> bool {
    match constraint {
        FeatureConstraint::Exact(expected) => expected == feature,
        FeatureConstraint::Any => true,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use deckmaste_construction_core::macro_def::DeclarationKind;
    use deckmaste_construction_core::macro_def::GrammarPosition;
    use deckmaste_construction_core::macro_def::SubtypeCategory;
    use deckmaste_construction_core::macro_def::SurfaceFeature;
    use deckmaste_construction_core::macro_def::read_builtin_v2;
    use deckmaste_construction_core::macro_def::read_str;

    use super::super::engine::Child;
    use super::super::engine::CompletionDisposition;
    use super::super::engine::Family;
    use super::super::engine::NodeId;
    use super::super::engine::Observation;
    use super::super::engine::RulePosition;
    use super::super::engine::SpannedLexical;
    use super::Category;
    use super::ChartFailure;
    use super::Leaf;
    use super::Lexical;
    use super::LexicalBoundary;
    use super::LexicalMatch;
    use super::RootRuleId;
    use super::RuleId;
    use super::ScanInput;
    use super::SliceGrammar;
    use super::StructuralObservation;
    use super::TraceLimits;
    use super::initial_surface;
    use super::parse_forest;
    use super::reset_trace_label_counts;
    use super::rule_name_v1;
    use super::terminal_name_v1;
    use super::trace_label_counts;
    use super::value_label_v1;
    use crate::ast::CommonNoun;
    use crate::ast::DeclarationNoun;
    use crate::ast::Noun;
    use crate::ast::ObjectPronoun;
    use crate::ast::ScalarNumber;
    use crate::ast::SelfReferenceSpelling;
    use crate::ast::SubjectPronoun;
    use crate::ast::TriggerMarker;
    use crate::ast::Variable;
    use crate::constructions::CasePosition;
    use crate::constructions::ConcordClass;
    use crate::constructions::DeclarationLeaf;
    use crate::constructions::DeclarationMatcher;
    use crate::constructions::FeatureConstraint;
    use crate::constructions::LexicalOwnerTemplate;
    use crate::constructions::LexicalProvenanceKind;
    use crate::constructions::LexicalTerminal;
    use crate::constructions::Number;
    use crate::constructions::Onset;
    use crate::constructions::PossessiveEnding;
    use crate::constructions::PrefixPosition;
    use crate::constructions::RULES;
    use crate::constructions::ScanPosition;
    use crate::context::ParseContext;
    use crate::environment::DeclarationId;
    use crate::environment::ParserEnvironment;
    use crate::environment::canonical_test_catalog_provider;
    use crate::environment::canonical_test_environment;
    use crate::environment::reading_lookup_count;
    use crate::environment::reset_reading_lookup_count;
    use crate::parser::Parser;

    fn lexical(value: Leaf) -> Child<Leaf, crate::constructions::LexicalOwner> {
        Child::Lexical(SpannedLexical {
            span: crate::parser::TextSpan { start: 0, end: 1 },
            value,
            owner: None,
        })
    }

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<super::RootForest, ChartFailure<Category, Lexical>> {
        let environment = canonical_test_environment();
        let context = context(card_name);
        let grammar = SliceGrammar {
            environment: &environment,
            context: &context,
        };

        parse_forest::<crate::ast::Ability>(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(
            card_name,
            card_name == "Zacama, Primal Calamity",
            deckmaste_construction_core::macro_def::Onset::Consonant,
        )
        .expect("test card names are valid parse contexts")
    }

    #[test]
    fn sentence_initial_case_and_surface_owned_prefix_are_orthogonal() {
        let environment = canonical_test_environment();
        let context = context("Context Card");
        let word_end = |text, byte_offset, case, prefix, word| {
            ScanInput {
                text,
                position: ScanPosition {
                    byte_offset,
                    case,
                    prefix,
                },
                environment: &environment,
                context: &context,
            }
            .word_end(word, LexicalBoundary::Separated)
        };

        assert_eq!(
            word_end(
                "Destroy",
                0,
                CasePosition::DocumentInitial,
                PrefixPosition::None,
                "destroy",
            ),
            Some(7),
        );
        assert_eq!(
            word_end(
                "Destroy target",
                7,
                CasePosition::Continuation,
                PrefixPosition::WordOwnedSpace,
                "target",
            ),
            Some(14),
        );
        assert_eq!(
            word_end(
                "Destroy target creature. You",
                25,
                CasePosition::SentenceInitial,
                PrefixPosition::SurfaceOwned,
                "you",
            ),
            Some(28),
            "the following word starts after the separator-owned byte",
        );
        assert_eq!(
            word_end(
                "alpha, beta",
                7,
                CasePosition::Continuation,
                PrefixPosition::SurfaceOwned,
                "beta",
            ),
            Some(11),
            "a non-sentence separator suppresses a word-owned prefix without capitalizing",
        );

        assert_eq!(
            word_end(
                "Destroy target creature. You",
                25,
                CasePosition::Continuation,
                PrefixPosition::SurfaceOwned,
                "you",
            ),
            None,
            "capitalization depends on case, not surface ownership",
        );
        assert_eq!(
            word_end(
                "Destroy target creature. You",
                25,
                CasePosition::SentenceInitial,
                PrefixPosition::WordOwnedSpace,
                "you",
            ),
            None,
            "prefix consumption depends on ownership, not sentence case",
        );
    }

    #[test]
    fn punctuation_consumes_only_its_owned_leading_space() {
        let environment = canonical_test_environment();
        let context = context("Context Card");
        let punctuation_end = |text, byte_offset, prefix| {
            ScanInput {
                text,
                position: ScanPosition {
                    byte_offset,
                    case: CasePosition::Continuation,
                    prefix,
                },
                environment: &environment,
                context: &context,
            }
            .punctuation_end("{")
        };

        assert_eq!(
            punctuation_end("Pay {2}", 3, PrefixPosition::WordOwnedSpace),
            Some(5),
            "word-owned punctuation consumes exactly one leading ASCII space",
        );
        assert_eq!(
            punctuation_end("Pay {2}", 4, PrefixPosition::SurfaceOwned),
            Some(5),
            "surface-owned punctuation remains adjacent",
        );
        assert_eq!(
            punctuation_end("Pay {2}", 3, PrefixPosition::SurfaceOwned),
            None,
            "surface-owned punctuation cannot consume a separator",
        );
        assert_eq!(
            punctuation_end("Pay{2}", 3, PrefixPosition::WordOwnedSpace),
            None,
            "word-owned punctuation requires its separator",
        );
        assert_eq!(
            punctuation_end("Pay  {2}", 3, PrefixPosition::WordOwnedSpace),
            None,
            "word-owned punctuation consumes no more than one separator",
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn document_initial_rejects_a_nonempty_prefix_contract() {
        let environment = canonical_test_environment();
        let context = context("Context Card");
        let input = ScanInput {
            text: " Destroy",
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::WordOwnedSpace,
            },
            environment: &environment,
            context: &context,
        };
        let _ = input.word_end("destroy", LexicalBoundary::Separated);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn noninitial_offset_rejects_an_absent_prefix_contract() {
        let environment = canonical_test_environment();
        let context = context("Context Card");
        let input = ScanInput {
            text: " alpha",
            position: ScanPosition {
                byte_offset: 1,
                case: CasePosition::Continuation,
                prefix: PrefixPosition::None,
            },
            environment: &environment,
            context: &context,
        };
        let _ = input.word_end("alpha", LexicalBoundary::Separated);
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "the authority enumerates every closed morphology boundary and owner"
    )]
    fn generated_declaration_verb_scans_exact_surfaces_boundaries_and_owners() {
        let environment = canonical_test_environment();
        let context = context("Context Card");
        let scan = |text, matcher, owner| {
            super::scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition {
                        byte_offset: 0,
                        case: CasePosition::DocumentInitial,
                        prefix: PrefixPosition::None,
                    },
                    environment: &environment,
                    context: &context,
                },
                LexicalTerminal {
                    matcher,
                    owner,
                    right_boundary: LexicalBoundary::Separated,
                },
            )
        };
        let intransitive_codec = RULES
            .iter()
            .find(|rule| rule.id == RuleId::IntransitiveLexicalVerbPhraseIntransitivePredicate)
            .and_then(|rule| {
                rule.rhs.iter().find_map(|position| match position {
                    RulePosition::Lexical(terminal) => match terminal.matcher {
                        Lexical::DeclarationVerb(codec, _) => Some(codec),
                        _ => None,
                    },
                    _ => None,
                })
            })
            .expect("the intransitive frame owns one declaration-verb terminal");
        for (text, concord_class, owner) in [
            ("Die.", ConcordClass::Other, "core-verb:Die"),
            ("Dies.", ConcordClass::ThirdPersonSingular, "core-verb:Die"),
        ] {
            let matches = scan(
                text,
                Lexical::DeclarationVerb(
                    intransitive_codec,
                    FeatureConstraint::Exact(concord_class),
                ),
                LexicalOwnerTemplate::DeclarationVerb(intransitive_codec),
            );
            assert!(matches!(
                matches.as_slice(),
                [LexicalMatch {
                    value: Leaf::IntransitiveVerb {
                        verb,
                        concord_class: actual_concord_class,
                        ..
                    },
                    ..
                }] if *actual_concord_class == concord_class
                    && verb.reference()
                        == &crate::environment::VerbInventoryRef::Core(
                            crate::environment::CoreVerbIdentity::Die,
                        )
            ));
            assert_eq!(
                LexicalOwnerTemplate::DeclarationVerb(intransitive_codec)
                    .instantiate(&matches[0].value)
                    .expect("declaration-verb leaf has its lexical owner")
                    .stable_id(),
                owner,
            );
        }
        for rejected in ["Diex.", "Diesx."] {
            assert!(
                scan(
                    rejected,
                    Lexical::DeclarationVerb(intransitive_codec, FeatureConstraint::Any),
                    LexicalOwnerTemplate::DeclarationVerb(intransitive_codec),
                )
                .is_empty(),
                "unexpected closed verb reading for {rejected:?}"
            );
        }

        let noun_codec = RULES
            .iter()
            .find(|rule| rule.id == RuleId::HeadNounSingularHead)
            .and_then(|rule| {
                rule.rhs.iter().find_map(|position| match position {
                    RulePosition::Lexical(terminal) => match terminal.matcher {
                        Lexical::DeclarationNoun(codec, _) => Some(codec),
                        _ => None,
                    },
                    _ => None,
                })
            })
            .expect("the noun inventory head owns its aggregate terminal");
        for (text, number, owner) in [
            (
                "Player.",
                Number::Singular,
                "lexeme:CommonNoun/Player/singular",
            ),
            (
                "Players.",
                Number::Plural,
                "lexeme:CommonNoun/Player/plural",
            ),
        ] {
            let matches = scan(
                text,
                Lexical::DeclarationNoun(noun_codec, FeatureConstraint::Exact(number)),
                LexicalOwnerTemplate::DeclarationNoun(noun_codec),
            );
            let closed = matches
                .iter()
                .find(|matched| {
                    matches!(
                        matched.value,
                        Leaf::Noun {
                            noun: Noun::Lexeme(CommonNoun::Player),
                            number: actual,
                            ..
                        } if actual == number
                    )
                })
                .expect("closed noun reading survives any open declaration collision");
            assert_eq!(closed.owner.as_ref().unwrap().stable_id(), owner);
        }
        assert!(
            scan(
                "Playersx.",
                Lexical::DeclarationNoun(noun_codec, FeatureConstraint::Any),
                LexicalOwnerTemplate::DeclarationNoun(noun_codec),
            )
            .is_empty()
        );
    }

    fn declaration_environment(
        name: &str,
        bare: &str,
        third_person: Option<&str>,
    ) -> ParserEnvironment {
        let override_field = third_person
            .map(|surface| format!(r#",third_person:"{surface}""#))
            .unwrap_or_default();
        let source = format!(
            r#"KeywordAction(name:"{name}",spelling:"{bare}",grammar:Verb(bare:"{bare}"{override_field},frame_set:MeasureComplement))"#
        );
        let declaration = read_str(format!("/synthetic/{name}.ron"), &source)
            .expect("synthetic keyword action is valid");
        ParserEnvironment::try_from_declarations([declaration])
            .expect("synthetic parser environment freezes")
    }

    fn declaration_terminal(name: &'static str) -> LexicalTerminal {
        let kind = DeclarationKind::KeywordAction;
        LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name,
                position: GrammarPosition::Verb,
                feature: FeatureConstraint::Any,
            }),
            owner: LexicalOwnerTemplate::Declaration { kind, name },
            right_boundary: LexicalBoundary::Separated,
        }
    }

    #[test]
    fn owned_declaration_owner_ids_outlive_the_source_and_include_feature() {
        let source = String::from(
            r#"KeywordAction(name:"Novel",spelling:"novel",grammar:Verb(bare:"novel",frame_set:Intransitive))"#,
        );
        let declaration = read_str("/synthetic/Novel.ron", &source).unwrap();
        let environment = ParserEnvironment::try_from_declarations([declaration]).unwrap();
        drop(source);
        let context = context("Context Card");
        let matches = super::scan_lexical(
            &ScanInput {
                text: "Novel.",
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                environment: &environment,
                context: &context,
            },
            declaration_terminal("Novel"),
        );
        let owner = matches[0].owner.as_ref().expect("non-EOI has owner");
        assert_eq!(owner.kind(), LexicalProvenanceKind::Lexeme);
        assert_eq!(owner.stable_id(), "lexeme:keyword_action/Novel/bare");
    }

    #[test]
    fn declaration_owner_projection_is_deterministic_under_shuffled_input() {
        let sources = [
            (
                "/synthetic/Novel.ron",
                r#"KeywordAction(name:"Novel",spelling:"novel",grammar:Verb(bare:"novel",frame_set:Intransitive))"#,
            ),
            (
                "/synthetic/Other.ron",
                r#"KeywordAbility(name:"Other",spelling:"other",grammar:FixedTerm(surface:"other"))"#,
            ),
        ];
        let project = |order: [usize; 2]| {
            let declarations = order.map(|index| {
                let (path, source) = sources[index];
                read_str(path, source).unwrap()
            });
            let environment = ParserEnvironment::try_from_declarations(declarations).unwrap();
            let context = context("Context Card");
            super::scan_lexical(
                &ScanInput {
                    text: "Novel.",
                    position: ScanPosition {
                        byte_offset: 0,
                        case: CasePosition::DocumentInitial,
                        prefix: PrefixPosition::None,
                    },
                    environment: &environment,
                    context: &context,
                },
                declaration_terminal("Novel"),
            )
            .into_iter()
            .map(|matched| matched.owner.unwrap().stable_id().to_owned())
            .collect::<Vec<_>>()
        };

        assert_eq!(project([0, 1]), project([1, 0]));
        assert_eq!(project([0, 1]), ["lexeme:keyword_action/Novel/bare"]);
    }

    fn parser_declaration_environment(
        name: &str,
        bare: &str,
        third_person: Option<&str>,
    ) -> ParserEnvironment {
        let mut declarations = read_builtin_v2(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
        )
        .expect("integrated builtin-v2 rows load")
        .into_iter()
        .filter(|declaration| {
            declaration.identity().kind() == DeclarationKind::KeywordAction
                && matches!(declaration.identity().name(), "Destroy" | "Connive")
        })
        .collect::<Vec<_>>();
        if !declarations.iter().any(|declaration| {
            declaration.identity().kind() == DeclarationKind::KeywordAction
                && declaration.identity().name() == name
        }) {
            let override_field = third_person
                .map(|surface| format!(r#",third_person:"{surface}""#))
                .unwrap_or_default();
            let source = format!(
                r#"KeywordAction(name:"{name}",spelling:"{bare}",grammar:Verb(bare:"{bare}"{override_field},frame_set:MeasureComplement))"#
            );
            declarations.push(
                read_str(format!("/synthetic/{name}.ron"), &source)
                    .expect("synthetic keyword action is valid"),
            );
        }
        ParserEnvironment::try_from_parts(declarations, [canonical_test_catalog_provider()])
            .expect("parser declaration environment freezes")
    }

    #[test]
    fn generated_vocab_scan_uses_exact_tables_boundaries_and_owners() {
        let environment = ParserEnvironment::try_from_declarations([])
            .expect("an empty declaration environment is valid");
        let context = context("Context Card");
        let cases = [
            (
                Lexical::TriggerMarker,
                Leaf::TriggerMarker(TriggerMarker::Whenever),
                "whenever",
                "vocab:TriggerMarker/Whenever",
            ),
            (
                Lexical::SubjectPronoun,
                Leaf::SubjectPronoun(SubjectPronoun::They),
                "they",
                "vocab:SubjectPronoun/They",
            ),
            (
                Lexical::ObjectPronoun,
                Leaf::ObjectPronoun(ObjectPronoun::Them),
                "them",
                "vocab:ObjectPronoun/Them",
            ),
            (
                Lexical::Variable,
                Leaf::Variable(Variable::X),
                "X",
                "vocab:Variable/X",
            ),
        ];

        for (matcher, expected_leaf, running, expected_owner) in cases {
            let owner = match matcher {
                Lexical::TriggerMarker => LexicalOwnerTemplate::Vocab {
                    declaration: "TriggerMarker",
                },
                Lexical::SubjectPronoun => LexicalOwnerTemplate::Vocab {
                    declaration: "SubjectPronoun",
                },
                Lexical::ObjectPronoun => LexicalOwnerTemplate::Vocab {
                    declaration: "ObjectPronoun",
                },
                Lexical::Variable => LexicalOwnerTemplate::Vocab {
                    declaration: "Variable",
                },
                _ => unreachable!("the fixture contains only finite vocab terminals"),
            };
            let terminal = LexicalTerminal {
                matcher,
                owner,
                right_boundary: LexicalBoundary::Separated,
            };

            let initial =
                if running == "X" { running.to_owned() } else { initial_surface(running) };
            let initial_input = ScanInput {
                text: &initial,
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                environment: &environment,
                context: &context,
            };
            let initial_matches = crate::constructions::scan_lexical(&initial_input, terminal);
            assert_eq!(initial_matches.len(), 1, "initial {running}");
            assert_eq!(initial_matches[0].end, initial.len());
            assert_eq!(initial_matches[0].value, expected_leaf);
            assert_eq!(
                terminal
                    .owner
                    .instantiate(&initial_matches[0].value)
                    .expect("vocab match owns its declaration member")
                    .stable_id(),
                expected_owner
            );

            for suffix in ["", " ", ",", "."] {
                let text = format!("Prefix {running}{suffix}");
                let input = ScanInput {
                    text: &text,
                    position: ScanPosition {
                        byte_offset: "Prefix".len(),
                        case: CasePosition::Continuation,
                        prefix: PrefixPosition::WordOwnedSpace,
                    },
                    environment: &environment,
                    context: &context,
                };
                let matches = crate::constructions::scan_lexical(&input, terminal);
                assert_eq!(matches.len(), 1, "continuation {running}{suffix}");
                assert_eq!(matches[0].end, "Prefix ".len() + running.len());
                assert_eq!(matches[0].value, expected_leaf);
            }

            for rejected in [format!("Prefix{running}"), format!("Prefix {running}x")] {
                let input = ScanInput {
                    text: &rejected,
                    position: ScanPosition {
                        byte_offset: "Prefix".len(),
                        case: CasePosition::Continuation,
                        prefix: PrefixPosition::WordOwnedSpace,
                    },
                    environment: &environment,
                    context: &context,
                };
                assert!(
                    crate::constructions::scan_lexical(&input, terminal).is_empty(),
                    "accepted non-rendered boundary {rejected:?}"
                );
            }
        }
    }

    #[test]
    fn generated_vocab_scan_capitalizes_only_the_first_unicode_scalar() {
        let environment = ParserEnvironment::try_from_declarations([])
            .expect("empty declaration environment freezes");
        let context = context("Context Card");
        let input = ScanInput {
            text: "Élan vital",
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
            },
            environment: &environment,
            context: &context,
        };

        assert_eq!(
            input.word_end("élan", LexicalBoundary::Separated),
            Some("Élan".len())
        );
        assert_eq!(
            input.word_end("élan vital", LexicalBoundary::Separated),
            Some("Élan vital".len())
        );
        assert_eq!(input.word_end("éLan", LexicalBoundary::Separated), None);
    }

    #[test]
    fn generated_declaration_scan_isolated_to_active_environment() {
        let scry = declaration_environment("Scry", "scry", Some("scries"));
        let connive = declaration_environment("Connive", "connive", None);
        let context = context("Context Card");
        let terminal = declaration_terminal("Scry");

        let scan = |environment: &ParserEnvironment, text: &str| {
            let input = ScanInput {
                text,
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                environment,
                context: &context,
            };
            crate::constructions::scan_lexical(&input, terminal)
        };

        let scry_matches = scan(&scry, "Scry");
        assert_eq!(scry_matches.len(), 1);
        assert_eq!(scry_matches[0].end, 4);
        assert_eq!(
            scry_matches[0].value,
            Leaf::Declaration(DeclarationLeaf {
                id: DeclarationId::new(DeclarationKind::KeywordAction, "Scry"),
                feature: SurfaceFeature::PLAIN,
                onset: Onset::Consonant,
            })
        );
        assert!(scan(&connive, "Scry").is_empty());
        assert!(scan(&scry, "Connive").is_empty());

        // Repeated engine retries are pure and deterministic.
        let project =
            |matches: Vec<super::LexicalMatch<Leaf, crate::constructions::LexicalOwner>>| {
                matches
                    .into_iter()
                    .map(|matched| (matched.end, matched.value))
                    .collect::<Vec<_>>()
            };
        assert_eq!(project(scan(&scry, "Scry")), project(scan(&scry, "Scry")));
    }

    #[test]
    fn parser_instances_route_declaration_scans_through_their_own_environment() {
        let scry = Parser::new(parser_declaration_environment(
            "Scry",
            "scry",
            Some("scries"),
        ))
        .expect("required declarations are present");
        let connive = Parser::new(parser_declaration_environment("Connive", "connive", None))
            .expect("required declarations are present");
        let context = context("Context Card");
        let terminal = declaration_terminal("Scry");

        let scry_matches = scry.test_only_scan_terminal("Scry", &context, terminal, 0);
        assert_eq!(scry_matches.len(), 1);
        assert_eq!(scry_matches[0].end, "Scry".len());
        assert!(
            connive
                .test_only_scan_terminal("Scry", &context, terminal, 0)
                .is_empty()
        );
    }

    #[test]
    fn lexical_boundary_is_exactly_eoi_authored_spacing_transition_or_closing_circumfix() {
        for text in [
            "word",
            "word ",
            "word\n",
            "word,",
            "word.",
            "word:",
            "word]",
            "word}",
            "word—Next",
        ] {
            assert!(super::has_lexical_boundary(text, "word".len()), "{text:?}");
        }
        for text in [
            "word/", "word?", "word‽", "word_", "word!", "wordx", "wordé",
        ] {
            assert!(!super::has_lexical_boundary(text, "word".len()), "{text:?}");
        }
        assert!(!super::has_lexical_boundary("é", 1));
    }

    #[test]
    fn generated_declaration_scan_accepts_bounded_multiword_surfaces() {
        let declaration = read_str(
            "/synthetic/PartnerWith.ron",
            r#"KeywordAbility(name:"PartnerWith",spelling:"partner with",grammar:FixedTerm(surface:"partner with"))"#,
        )
        .expect("synthetic fixed term is valid");
        let environment = ParserEnvironment::try_from_declarations([declaration])
            .expect("synthetic environment freezes");
        let context = context("Context Card");
        let kind = DeclarationKind::KeywordAbility;
        let terminal = LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name: "PartnerWith",
                position: GrammarPosition::FixedTerm,
                feature: FeatureConstraint::Exact(SurfaceFeature::Fixed),
            }),
            owner: LexicalOwnerTemplate::Declaration {
                kind,
                name: "PartnerWith",
            },
            right_boundary: LexicalBoundary::Separated,
        };

        for (text, offset, expected_end) in [
            ("Partner with.", 0, "Partner with".len()),
            (
                "Prefix partner with, suffix",
                "Prefix".len(),
                "Prefix partner with".len(),
            ),
        ] {
            let input = ScanInput {
                text,
                position: ScanPosition {
                    byte_offset: offset,
                    case: if offset == 0 {
                        CasePosition::DocumentInitial
                    } else {
                        CasePosition::Continuation
                    },
                    prefix: if offset == 0 {
                        PrefixPosition::None
                    } else {
                        PrefixPosition::WordOwnedSpace
                    },
                },
                environment: &environment,
                context: &context,
            };
            let matches = crate::constructions::scan_lexical(&input, terminal);
            assert_eq!(matches.len(), 1, "{text:?}");
            assert_eq!(matches[0].end, expected_end);
            assert!(matches!(
                &matches[0].value,
                Leaf::Declaration(leaf)
                    if leaf.id
                        == DeclarationId::new(DeclarationKind::KeywordAbility, "PartnerWith")
                        && leaf.feature == SurfaceFeature::Fixed
            ));
        }
    }

    #[test]
    fn generated_declaration_scan_handles_expanding_unicode_initial_case() {
        let declaration = read_str(
            "/synthetic/SharpS.ron",
            r#"CounterKind(name:"SharpS",spelling:"ßeta",grammar:FixedTerm(surface:"ßeta",onset:Consonant))"#,
        )
        .expect("synthetic Unicode fixed term is valid");
        let environment = ParserEnvironment::try_from_declarations([declaration])
            .expect("synthetic environment freezes");
        let context = context("Context Card");
        let kind = DeclarationKind::CounterKind;
        let terminal = LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name: "SharpS",
                position: GrammarPosition::FixedTerm,
                feature: FeatureConstraint::Exact(SurfaceFeature::Fixed),
            }),
            owner: LexicalOwnerTemplate::Declaration {
                kind,
                name: "SharpS",
            },
            right_boundary: LexicalBoundary::Separated,
        };
        let input = ScanInput {
            text: "SSeta.",
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
            },
            environment: &environment,
            context: &context,
        };

        let matches = crate::constructions::scan_lexical(&input, terminal);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].end, "SSeta".len());
    }

    #[test]
    fn generated_declaration_scan_bounds_prefix_lookups_by_the_environment_index() {
        let declaration = read_str(
            "/synthetic/ScryWord.ron",
            r#"KeywordAbility(name:"ScryWord",spelling:"scry",grammar:FixedTerm(surface:"scry"))"#,
        )
        .expect("synthetic fixed term is valid");
        let environment = ParserEnvironment::try_from_declarations([declaration])
            .expect("synthetic environment freezes");
        let context = context("Context Card");
        let kind = DeclarationKind::KeywordAbility;
        let terminal = LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name: "ScryWord",
                position: GrammarPosition::FixedTerm,
                feature: FeatureConstraint::Exact(SurfaceFeature::Fixed),
            }),
            owner: LexicalOwnerTemplate::Declaration {
                kind,
                name: "ScryWord",
            },
            right_boundary: LexicalBoundary::Separated,
        };
        let text = format!("Scry{}.", " x".repeat(4_096));
        let input = ScanInput {
            text: &text,
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
            },
            environment: &environment,
            context: &context,
        };

        reset_reading_lookup_count();
        let matches = crate::constructions::scan_lexical(&input, terminal);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].end, "Scry".len());
        assert_eq!(
            reading_lookup_count(),
            1,
            "lookup work must be bounded by indexed surface length, not trailing input"
        );
    }

    #[test]
    fn generated_declaration_scan_filters_collisions_and_orders_features_exactly() {
        let declarations = [
            (
                "/synthetic/Alpha.ron",
                r#"KeywordAction(name:"Alpha",spelling:"echo",grammar:Verb(bare:"echo",third_person:"echo",frame_set:Intransitive))"#,
            ),
            (
                "/synthetic/Zeta.ron",
                r#"KeywordAction(name:"Zeta",spelling:"echo",grammar:Verb(bare:"echo",frame_set:Intransitive))"#,
            ),
        ]
        .into_iter()
        .map(|(path, source)| read_str(path, source).expect("collision fixture is valid"));
        let mut environment = ParserEnvironment::try_from_declarations(declarations)
            .expect("collision environment freezes");
        environment.test_only_duplicate_initial_readings(GrammarPosition::Verb, "Echo");
        let context = context("Context Card");
        let input = ScanInput {
            text: "Echo.",
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
                prefix: PrefixPosition::None,
            },
            environment: &environment,
            context: &context,
        };
        let terminal = |kind, name, feature| LexicalTerminal {
            matcher: Lexical::Declaration(DeclarationMatcher {
                kind,
                name,
                position: GrammarPosition::Verb,
                feature,
            }),
            owner: LexicalOwnerTemplate::Declaration { kind, name },
            right_boundary: LexicalBoundary::Separated,
        };
        let scan = |terminal| crate::constructions::scan_lexical(&input, terminal);
        let project =
            |matches: &[super::LexicalMatch<Leaf, crate::constructions::LexicalOwner>]| {
                matches
                    .iter()
                    .map(|matched| match &matched.value {
                        Leaf::Declaration(leaf) => (matched.end, leaf.id.clone(), leaf.feature),
                        _ => panic!("declaration terminal returned a non-declaration leaf"),
                    })
                    .collect::<Vec<_>>()
            };

        let alpha_any = scan(terminal(
            DeclarationKind::KeywordAction,
            "Alpha",
            FeatureConstraint::Any,
        ));
        assert_eq!(
            project(&alpha_any),
            [
                (
                    "Echo".len(),
                    DeclarationId::new(DeclarationKind::KeywordAction, "Alpha"),
                    SurfaceFeature::PLAIN,
                ),
                (
                    "Echo".len(),
                    DeclarationId::new(DeclarationKind::KeywordAction, "Alpha"),
                    SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                ),
            ]
        );
        let alpha_third = scan(terminal(
            DeclarationKind::KeywordAction,
            "Alpha",
            FeatureConstraint::Exact(SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT),
        ));
        assert_eq!(
            project(&alpha_third),
            [(
                "Echo".len(),
                DeclarationId::new(DeclarationKind::KeywordAction, "Alpha"),
                SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
            )]
        );
        assert!(
            scan(terminal(
                DeclarationKind::KeywordAction,
                "Missing",
                FeatureConstraint::Any,
            ))
            .is_empty()
        );
        assert!(
            scan(terminal(
                DeclarationKind::KeywordAbility,
                "Alpha",
                FeatureConstraint::Any,
            ))
            .is_empty()
        );
        let zeta = scan(terminal(
            DeclarationKind::KeywordAction,
            "Zeta",
            FeatureConstraint::Exact(SurfaceFeature::PLAIN),
        ));
        assert!(matches!(
            zeta.as_slice(),
            [matched]
                if matches!(
                    &matched.value,
                    Leaf::Declaration(leaf)
                        if leaf.id
                            == DeclarationId::new(DeclarationKind::KeywordAction, "Zeta")
                            && leaf.feature == SurfaceFeature::PLAIN
                )
        ));
        assert_eq!(
            project(&alpha_any),
            project(&scan(terminal(
                DeclarationKind::KeywordAction,
                "Alpha",
                FeatureConstraint::Any,
            ))),
            "retries preserve exact stable order"
        );
    }

    #[test]
    fn scanner_accepts_multi_token_context_identity_and_declaration_nouns() {
        assert!(
            slice_candidates(
                "Zacama deals 3 damage to target creature.",
                "Zacama, Primal Calamity"
            )
            .is_ok()
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "the aggregate scanner test authenticates every contributor, feature, and provenance boundary"
    )]
    fn aggregate_noun_scanner_keeps_core_and_declaration_provenance_in_one_terminal() {
        let environment = ParserEnvironment::try_from_declarations([
            read_str(
                "/synthetic/types/Elf.ron",
                r#"Type(name:"Elf",spelling:"elf",grammar:Noun(singular:"elf",plural:"elves"))"#,
            )
            .unwrap(),
            read_str(
                "/synthetic/subtypes/creature/Elf.ron",
                r#"Subtype(category:Creature,name:"Elf",spelling:"elf",grammar:Noun(singular:"elf",plural:"elves"))"#,
            )
            .unwrap(),
            read_str(
                "/synthetic/turn_parts/Upkeep.ron",
                r#"TurnPart(name:"Upkeep",spelling:"upkeep",grammar:Noun(singular:"upkeep"))"#,
            )
            .unwrap(),
            read_str(
                "/synthetic/types/Player.ron",
                r#"Type(name:"Player",spelling:"player",grammar:Noun(singular:"player"))"#,
            )
            .unwrap(),
            read_str(
                "/synthetic/abilities/Flying.ron",
                r#"KeywordAbility(name:"Flying",spelling:"flying",grammar:Noun(singular:"flying"))"#,
            )
            .unwrap(),
        ])
        .unwrap();
        let codec = RULES
            .iter()
            .find(|rule| rule.id == RuleId::HeadNounSingularHead)
            .and_then(|rule| {
                rule.rhs.iter().find_map(|position| match position {
                    RulePosition::Lexical(terminal) => match terminal.matcher {
                        Lexical::DeclarationNoun(codec, _) => Some(codec),
                        _ => None,
                    },
                    _ => None,
                })
            })
            .expect("the noun head owns the single aggregate terminal");
        let context = context("Context Card");
        let scan = |text, byte_offset, case, wanted| {
            super::scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition {
                        byte_offset,
                        case,
                        prefix: if byte_offset == 0 {
                            PrefixPosition::None
                        } else {
                            PrefixPosition::WordOwnedSpace
                        },
                    },
                    environment: &environment,
                    context: &context,
                },
                LexicalTerminal {
                    matcher: Lexical::DeclarationNoun(codec, wanted),
                    owner: LexicalOwnerTemplate::DeclarationNoun(codec),
                    right_boundary: LexicalBoundary::Separated,
                },
            )
        };
        let elf_kinds = scan(
            "Elves.",
            0,
            CasePosition::DocumentInitial,
            FeatureConstraint::Exact(Number::Plural),
        )
        .into_iter()
        .filter_map(|matched| match matched.value {
            Leaf::Noun {
                noun: Noun::Declaration(noun),
                number: Number::Plural,
                ..
            } => Some(noun.id().kind()),
            _ => None,
        })
        .collect::<Vec<_>>();
        assert_eq!(
            elf_kinds,
            [
                DeclarationKind::Subtype(SubtypeCategory::Creature),
                DeclarationKind::Type,
            ]
        );
        let continued_elf_kinds = scan(
            "prefix elf.",
            6,
            CasePosition::Continuation,
            FeatureConstraint::Exact(Number::Singular),
        )
        .into_iter()
        .filter_map(|matched| match matched.value {
            Leaf::Noun {
                noun: Noun::Declaration(noun),
                number: Number::Singular,
                ..
            } => Some((matched.end, noun.id().kind())),
            _ => None,
        })
        .collect::<Vec<_>>();
        assert_eq!(
            continued_elf_kinds,
            [
                (10, DeclarationKind::Subtype(SubtypeCategory::Creature),),
                (10, DeclarationKind::Type),
            ]
        );

        let player = scan(
            "Player.",
            0,
            CasePosition::DocumentInitial,
            FeatureConstraint::Exact(Number::Singular),
        );
        assert_eq!(
            player.len(),
            2,
            "core and declaration readings both survive"
        );
        assert!(player.iter().all(|matched| matched.end == 6));
        assert!(player.iter().any(|matched| matches!(
            matched.value,
            Leaf::Noun {
                noun: Noun::Lexeme(CommonNoun::Player),
                ..
            }
        )));
        assert!(player.iter().any(|matched| matches!(
            &matched.value,
            Leaf::Noun {
                noun: Noun::Declaration(noun),
                ..
            } if noun.id() == &DeclarationId::new(DeclarationKind::Type, "Player")
        )));
        let owners = player
            .iter()
            .map(|matched| matched.owner.as_ref().unwrap().stable_id())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            owners.len(),
            2,
            "provenance keeps colliding readings distinct"
        );

        assert!(matches!(
            scan(
                "Upkeeps.",
                0,
                CasePosition::DocumentInitial,
                FeatureConstraint::Exact(Number::Plural),
            )
            .as_slice(),
            [super::LexicalMatch {
                value: Leaf::Noun {
                    noun: Noun::Declaration(noun),
                    number: Number::Plural,
                    ..
                },
                ..
            }] if noun.id().kind() == DeclarationKind::TurnPart
        ));
        assert!(
            scan(
                "Flying.",
                0,
                CasePosition::DocumentInitial,
                FeatureConstraint::Any,
            )
            .is_empty()
        );
    }

    #[test]
    fn parser_trace_lexical_ownership_scanner_matches_dedupe_retries_and_keep_overlaps() {
        for (limit, shown) in [(0, 0), (1, 1), (8, 3)] {
            let mut observed = StructuralObservation::new(TraceLimits::new(limit));
            observed.record_token(4, 9, Lexical::Literal("z"), &Leaf::Literal("z"));
            observed.record_token(0, 5, Lexical::Literal("a"), &Leaf::Literal("a"));
            observed.record_token(0, 5, Lexical::Literal("a"), &Leaf::Literal("a"));
            observed.record_token(0, 7, Lexical::Literal("b"), &Leaf::Literal("b"));
            let scanner_matches = observed.finish().scanner_matches().clone();
            assert_eq!(
                (
                    scanner_matches.total(),
                    scanner_matches.shown(),
                    scanner_matches.omitted(),
                ),
                (3, shown, 3 - shown)
            );
            assert_eq!(
                scanner_matches
                    .items()
                    .iter()
                    .map(|scanner_match| (scanner_match.start, scanner_match.end))
                    .collect::<Vec<_>>(),
                [(0, 5), (0, 7), (4, 9)][..shown]
            );
        }
    }

    #[test]
    fn parser_trace_lexical_ownership_scanner_labels_are_built_only_for_retained_matches() {
        for (limit, expected_labels) in [(0, 0), (1, 1)] {
            reset_trace_label_counts();
            let mut observed = StructuralObservation::new(TraceLimits::new(limit));
            observed.record_token(0, 1, Lexical::Literal("z"), &Leaf::Literal("z"));
            observed.record_token(0, 1, Lexical::Literal("a"), &Leaf::Literal("a"));

            let scanner_matches = observed.finish().scanner_matches().clone();
            assert_eq!(
                (
                    scanner_matches.total(),
                    scanner_matches.shown(),
                    scanner_matches.omitted(),
                ),
                (2, expected_labels, 2 - expected_labels)
            );
            if let Some(scanner_match) = scanner_matches.items().first() {
                assert_eq!(scanner_match.value_label_v1(), "Literal(\"a\")");
            }
            let counts = trace_label_counts();
            assert_eq!(counts.terminals, expected_labels, "limit {limit}");
            assert_eq!(counts.values, expected_labels, "limit {limit}");
            assert_eq!(counts.rules, 0, "limit {limit}");
            assert_eq!(counts.families, 0, "limit {limit}");
        }
    }

    #[test]
    fn structural_trace_noop_observer_builds_no_diagnostic_labels() {
        reset_trace_label_counts();
        assert!(slice_candidates("Destroy target creature.", "Context Card").is_ok());
        assert_eq!(trace_label_counts(), super::TraceLabelCounts::default());
    }

    #[test]
    fn structural_trace_transient_checked_rejection_disappears() {
        let family = Family {
            children: vec![Child::Node(NodeId(7)), lexical(Leaf::Literal("where"))],
        };
        let mut observed = StructuralObservation::new(TraceLimits::new(1));
        observed.checked_completion(
            RootRuleId::Grammar(RuleId::AmountNumber),
            1,
            4,
            &family,
            &CompletionDisposition::Rejected,
        );
        observed.checked_completion(
            RootRuleId::Grammar(RuleId::AmountNumber),
            1,
            4,
            &family,
            &CompletionDisposition::Accepted,
        );
        let rejections = observed.finish().checked_completion_rejections().clone();
        assert_eq!(
            (rejections.total(), rejections.shown(), rejections.omitted()),
            (0, 0, 0)
        );
    }

    #[test]
    fn structural_trace_transient_typed_rejection_disappears_after_later_success() {
        let family = Family {
            children: vec![Child::Node(NodeId(7)), lexical(Leaf::Literal("where"))],
        };
        let rejection = crate::constructions::BuildRejection::new(
            "Sentence",
            "with_where",
            crate::constructions::BuildViolation::Invariant {
                identity: "clause is Where",
            },
        );
        let mut observed = StructuralObservation::new(TraceLimits::new(1));
        observed.checked_completion(
            RootRuleId::Grammar(RuleId::AmountNumber),
            1,
            4,
            &family,
            &CompletionDisposition::DeferredBuildRejection(rejection),
        );
        observed.checked_completion(
            RootRuleId::Grammar(RuleId::AmountNumber),
            1,
            4,
            &family,
            &CompletionDisposition::Accepted,
        );

        let trace = observed.finish();
        assert!(trace.first_build_rejection().is_none());
        assert_eq!(trace.checked_completion_rejections().total(), 0);
    }

    #[test]
    fn structural_trace_replaces_a_stale_typed_cause_for_the_same_family_only() {
        let family = Family {
            children: vec![Child::Node(NodeId(7)), lexical(Leaf::Literal("where"))],
        };
        let unrelated_family = Family {
            children: vec![Child::Node(NodeId(6)), lexical(Leaf::Literal("where"))],
        };
        let stale_child = crate::constructions::BuildRejection::new(
            "AChild",
            "guarded",
            crate::constructions::BuildViolation::Invariant {
                identity: "child is Allowed",
            },
        );
        let current_parent = crate::constructions::BuildRejection::new(
            "CParent",
            "guarded",
            crate::constructions::BuildViolation::Invariant {
                identity: "child is Built",
            },
        );
        let unrelated = crate::constructions::BuildRejection::new(
            "BUnrelated",
            "guarded",
            crate::constructions::BuildViolation::Invariant {
                identity: "other is Allowed",
            },
        );
        let mut observed = StructuralObservation::new(TraceLimits::new(3));
        for (family, rejection) in [
            (&family, stale_child),
            (&unrelated_family, unrelated),
            (&family, current_parent),
        ] {
            observed.checked_completion(
                RootRuleId::Grammar(RuleId::AmountNumber),
                1,
                4,
                family,
                &CompletionDisposition::DeferredBuildRejection(rejection),
            );
        }

        let family_key = super::CheckedRejectionIdentity {
            rule: RootRuleId::Grammar(RuleId::AmountNumber),
            start: 1,
            end: 4,
            family: super::raw_family_identity(&family),
        };
        let retained_for_family = observed
            .deferred_build_rejections
            .iter()
            .filter_map(|(key, rejection)| (key == &family_key).then_some(*rejection))
            .collect::<Vec<_>>();
        assert_eq!(retained_for_family, [current_parent]);
        assert_eq!(observed.deferred_build_rejections.len(), 2);
        assert!(
            observed
                .deferred_build_rejections
                .iter()
                .any(|(_, rejection)| rejection == &unrelated)
        );
        assert_eq!(observed.finish().first_build_rejection(), Some(&unrelated));
    }

    #[test]
    fn structural_trace_final_checked_rejection_is_structured_and_bounded() {
        let family = Family {
            children: vec![Child::Node(NodeId(7)), lexical(Leaf::Literal("where"))],
        };
        for (limit, shown) in [(0, 0), (1, 1)] {
            let mut observed = StructuralObservation::new(TraceLimits::new(limit));
            observed.checked_completion(
                RootRuleId::Grammar(RuleId::AmountNumber),
                1,
                4,
                &family,
                &CompletionDisposition::Rejected,
            );
            let rejections = observed.finish().checked_completion_rejections().clone();
            assert_eq!(
                (rejections.total(), rejections.shown(), rejections.omitted()),
                (1, shown, 1 - shown)
            );
            if let Some(rejection) = rejections.items().first() {
                assert_eq!(rejection.rule_name_v1, "AmountNumber");
                assert_eq!((rejection.start, rejection.end), (1, 4));
                assert_eq!(rejection.family_identity_v1.0.len(), 2);
            }
        }
    }

    #[test]
    fn structural_trace_rejection_labels_are_built_only_for_retained_entries() {
        let family = Family {
            children: vec![lexical(Leaf::Literal("family"))],
        };
        for (limit, expected_labels) in [(0, 0), (1, 1)] {
            reset_trace_label_counts();
            let mut observed = StructuralObservation::new(TraceLimits::new(limit));
            observed.checked_completion(
                RootRuleId::Grammar(RuleId::VerbPhraseBaseVerbPhrase),
                1,
                2,
                &family,
                &CompletionDisposition::Rejected,
            );
            observed.checked_completion(
                RootRuleId::Grammar(RuleId::AmountNumber),
                1,
                2,
                &family,
                &CompletionDisposition::Rejected,
            );

            let rejections = observed.finish().checked_completion_rejections().clone();
            assert_eq!(
                (rejections.total(), rejections.shown(), rejections.omitted()),
                (2, expected_labels, 2 - expected_labels)
            );
            if let Some(rejection) = rejections.items().first() {
                assert_eq!(rejection.rule_name_v1(), "AmountNumber");
            }
            let counts = trace_label_counts();
            assert_eq!(counts.rules, expected_labels, "limit {limit}");
            assert_eq!(counts.values, expected_labels, "limit {limit}");
            assert_eq!(counts.terminals, 0, "limit {limit}");
            assert_eq!(counts.families, expected_labels, "limit {limit}");
        }
    }

    #[test]
    fn structural_trace_generated_rule_and_terminal_names_are_intrinsic() {
        let rule_names = RULES
            .iter()
            .map(|rule| rule_name_v1(RootRuleId::Grammar(rule.id)))
            .collect::<Vec<_>>();
        assert!(!rule_names.is_empty());
        assert!(
            rule_names
                .iter()
                .all(|name| !name.is_empty() && !name.contains(['\n', '\r']))
        );
        let unique_rule_names = rule_names
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for stable_name in [
            "AbilityPlain",
            "SentenceImperative",
            "FiniteClausePlainFiniteClause",
            "WhereClauseCategoryWhere",
            "VerbPhraseBaseVerbPhrase",
            "IntransitiveLexicalVerbPhraseIntransitivePredicate",
        ] {
            assert!(
                unique_rule_names.contains(stable_name),
                "missing stable public diagnostic rule name {stable_name:?}",
            );
        }

        let terminal_names = RULES
            .iter()
            .flat_map(|rule| rule.rhs)
            .filter_map(|position| match position {
                RulePosition::Lexical(terminal) => Some(terminal_name_v1(terminal.matcher)),
                RulePosition::Nonterminal(_) | RulePosition::AdjacentNonterminal(_) => None,
            })
            .collect::<BTreeSet<_>>();
        assert!(!terminal_names.is_empty());
        assert!(
            terminal_names
                .iter()
                .all(|name| !name.is_empty() && !name.contains(['\n', '\r']))
        );
        for stable_name in [
            "TriggerMarker",
            "SubjectPronoun",
            "FiniteCopula",
            "Literal(\"where\")",
        ] {
            assert!(
                terminal_names.contains(stable_name),
                "missing stable public diagnostic terminal name {stable_name:?}",
            );
        }
    }

    #[test]
    fn structural_trace_other_value_labels_are_pinned() {
        let values = [
            (Leaf::Literal("where"), "Literal(\"where\")"),
            (Leaf::EndOfInput, "EndOfInput"),
            (
                Leaf::TriggerMarker(TriggerMarker::Whenever),
                "TriggerMarker(Whenever)",
            ),
            (
                Leaf::SubjectPronoun(SubjectPronoun::They),
                "SubjectPronoun(They)",
            ),
            (
                Leaf::ObjectPronoun(ObjectPronoun::Them),
                "ObjectPronoun(Them)",
            ),
            (
                Leaf::SubjectPronoun(SubjectPronoun::It),
                "SubjectPronoun(It)",
            ),
            (
                Leaf::ObjectPronoun(ObjectPronoun::You),
                "ObjectPronoun(You)",
            ),
            (Leaf::Variable(Variable::X), "Variable(X)"),
            (
                Leaf::Noun {
                    noun: Noun::Lexeme(CommonNoun::Player),
                    number: super::Number::Singular,
                    onset: Onset::Consonant,
                    possessive_ending: PossessiveEnding::Other,
                },
                "Noun { noun: Player, number: Singular, onset: Consonant, possessive_ending: Other }",
            ),
            (
                Leaf::Noun {
                    noun: Noun::Lexeme(CommonNoun::Player),
                    number: super::Number::Plural,
                    onset: Onset::Consonant,
                    possessive_ending: PossessiveEnding::EndsInS,
                },
                "Noun { noun: Player, number: Plural, onset: Consonant, possessive_ending: EndsInS }",
            ),
            (
                Leaf::Declaration(DeclarationLeaf {
                    id: DeclarationId::new(DeclarationKind::KeywordAction, "Destroy"),
                    feature: SurfaceFeature::PLAIN,
                    onset: Onset::Consonant,
                }),
                "Declaration(DeclarationLeaf { id: DeclarationIdentity { kind: KeywordAction, name: \"Destroy\" }, feature: Inflectional(Plain), onset: Consonant })",
            ),
            (
                Leaf::IntransitiveVerb {
                    verb: crate::ast::DeclarationIntransitiveVerb::new(
                        &canonical_test_environment(),
                        crate::environment::VerbInventoryRef::Declaration(DeclarationId::new(
                            DeclarationKind::KeywordAction,
                            "Connive",
                        )),
                    )
                    .expect("the canonical environment declares intransitive Connive"),
                    concord_class: ConcordClass::ThirdPersonSingular,
                    onset: Onset::Consonant,
                },
                "IntransitiveVerb { verb: DeclarationIntransitiveVerb { reference: Declaration(DeclarationIdentity { kind: KeywordAction, name: \"Connive\" }) }, concord_class: ThirdPersonSingular, onset: Consonant }",
            ),
            (
                Leaf::ScalarNumber(ScalarNumber { magnitude: 2 }),
                "ScalarNumber(ScalarNumber { magnitude: 2 })",
            ),
            (
                Leaf::SelfReference(SelfReferenceSpelling::Full),
                "SelfReference(Full)",
            ),
            (
                Leaf::SelfReference(SelfReferenceSpelling::Abbreviated),
                "SelfReference(Abbreviated)",
            ),
        ];
        for (value, expected) in values {
            assert_eq!(value_label_v1(&value), expected);
        }
    }

    #[test]
    fn unsigned_decimal_scanner_is_canonical_at_both_scan_positions() {
        let environment = canonical_test_environment();
        let context = context("Test Card");
        let terminal = LexicalTerminal {
            matcher: Lexical::ScalarNumber,
            owner: LexicalOwnerTemplate::Static {
                kind: crate::constructions::LexicalProvenanceKind::Codec,
                stable_id: "codec:ScalarNumber",
            },
            right_boundary: LexicalBoundary::Separated,
        };
        let accepted = [("0", 0), ("1", 1), ("4,294,967,295", u32::MAX)];

        for (number, magnitude) in accepted {
            for (text, byte_offset, case, expected_end) in [
                (
                    number.to_owned(),
                    0,
                    CasePosition::DocumentInitial,
                    number.len(),
                ),
                (
                    format!("x {number}"),
                    1,
                    CasePosition::Continuation,
                    number.len() + 2,
                ),
            ] {
                let matches = super::scan_lexical(
                    &ScanInput {
                        text: &text,
                        position: ScanPosition {
                            byte_offset,
                            case,
                            prefix: if byte_offset == 0 {
                                PrefixPosition::None
                            } else {
                                PrefixPosition::WordOwnedSpace
                            },
                        },
                        environment: &environment,
                        context: &context,
                    },
                    terminal,
                );
                assert_eq!(matches.len(), 1, "{text:?} at {case:?}");
                assert_eq!(matches[0].end, expected_end, "{text:?} at {case:?}");
                assert_eq!(
                    matches[0].value,
                    Leaf::ScalarNumber(ScalarNumber { magnitude }),
                    "{text:?} at {case:?}"
                );
                let owner = terminal
                    .owner
                    .instantiate(&matches[0].value)
                    .expect("unsigned decimals own their bytes");
                assert_eq!(owner.stable_id(), "codec:ScalarNumber");
            }
        }

        for number in [
            "",
            "-",
            "+1",
            "-1",
            "00",
            "01",
            "-00",
            "1000",
            "4294967296",
            "-4294967296",
            "١",
            ".",
            ",",
            "1x",
            "1_",
            "1!",
            "-1_",
            "-1!",
        ] {
            for (text, byte_offset, case) in [
                (number.to_owned(), 0, CasePosition::DocumentInitial),
                (format!("x {number}"), 1, CasePosition::Continuation),
            ] {
                let matches = super::scan_lexical(
                    &ScanInput {
                        text: &text,
                        position: ScanPosition {
                            byte_offset,
                            case,
                            prefix: if byte_offset == 0 {
                                PrefixPosition::None
                            } else {
                                PrefixPosition::WordOwnedSpace
                            },
                        },
                        environment: &environment,
                        context: &context,
                    },
                    terminal,
                );
                assert!(
                    matches.is_empty(),
                    "accepted noncanonical decimal {number:?} at {case:?}"
                );
            }
        }

        for text in ["1.", "1,"] {
            let matches = super::scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition {
                        byte_offset: 0,
                        case: CasePosition::DocumentInitial,
                        prefix: PrefixPosition::None,
                    },
                    environment: &environment,
                    context: &context,
                },
                terminal,
            );
            assert_eq!(matches.len(), 1, "punctuation is a lexical boundary");
        }
    }

    #[test]
    fn context_identity_scanner_is_canonical_at_both_scan_positions_and_owner_typed() {
        let environment = canonical_test_environment();
        let terminal = LexicalTerminal {
            matcher: Lexical::SelfReference,
            owner: LexicalOwnerTemplate::Identity {
                declaration: "SelfReferenceSpelling",
            },
            right_boundary: LexicalBoundary::Separated,
        };
        let cases = [
            (
                "Context Card",
                vec![(
                    "Context Card",
                    SelfReferenceSpelling::Full,
                    "identity:SelfReferenceSpelling/Full",
                )],
            ),
            (
                "Zacama, Primal Calamity",
                vec![
                    (
                        "Zacama, Primal Calamity",
                        SelfReferenceSpelling::Full,
                        "identity:SelfReferenceSpelling/Full",
                    ),
                    (
                        "Zacama",
                        SelfReferenceSpelling::Abbreviated,
                        "identity:SelfReferenceSpelling/Abbreviated",
                    ),
                ],
            ),
            (
                "Fear, Fire, Foes!",
                vec![(
                    "Fear, Fire, Foes!",
                    SelfReferenceSpelling::Full,
                    "identity:SelfReferenceSpelling/Full",
                )],
            ),
        ];

        for (card_name, spellings) in cases {
            let context = context(card_name);
            for (surface, spelling, stable_id) in spellings {
                for (text, byte_offset, case, expected_end) in [
                    (
                        surface.to_owned(),
                        0,
                        CasePosition::DocumentInitial,
                        surface.len(),
                    ),
                    (
                        format!("x {surface}"),
                        1,
                        CasePosition::Continuation,
                        surface.len() + 2,
                    ),
                ] {
                    let matches = super::scan_lexical(
                        &ScanInput {
                            text: &text,
                            position: ScanPosition {
                                byte_offset,
                                case,
                                prefix: if byte_offset == 0 {
                                    PrefixPosition::None
                                } else {
                                    PrefixPosition::WordOwnedSpace
                                },
                            },
                            environment: &environment,
                            context: &context,
                        },
                        terminal,
                    );
                    let expected_count = 1 + usize::from(
                        context.abbreviated_card_name() != context.card_name()
                            && spelling == SelfReferenceSpelling::Full,
                    );
                    assert_eq!(matches.len(), expected_count, "{text:?} at {case:?}");
                    let selected_match = matches
                        .iter()
                        .find(|matched| matched.value == Leaf::SelfReference(spelling))
                        .expect("the requested context-identity arm is among all valid matches");
                    assert_eq!(selected_match.end, expected_end, "{text:?} at {case:?}");
                    assert_eq!(
                        selected_match.value,
                        Leaf::SelfReference(spelling),
                        "{text:?} at {case:?}"
                    );
                    let owner = terminal
                        .owner
                        .instantiate(&selected_match.value)
                        .expect("context identities own their bytes");
                    assert_eq!(owner.kind(), LexicalProvenanceKind::Identity);
                    assert_eq!(owner.stable_id(), stable_id);
                }
            }
        }

        let equal = context("Context Card");
        let matches = super::scan_lexical(
            &ScanInput {
                text: "Context Card.",
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
                    prefix: PrefixPosition::None,
                },
                environment: &environment,
                context: &equal,
            },
            terminal,
        );
        assert_eq!(matches.len(), 1, "equal spellings emit one token");
        assert_eq!(matches[0].end, "Context Card".len());
        assert_eq!(
            matches[0].value,
            Leaf::SelfReference(SelfReferenceSpelling::Full),
            "equal spellings emit only the declared canonical arm"
        );

        for text in [
            "Context Cardx",
            "Context Card_",
            "Context Card!",
            "Context Card‽",
        ] {
            assert!(
                super::scan_lexical(
                    &ScanInput {
                        text,
                        position: ScanPosition {
                            byte_offset: 0,
                            case: CasePosition::DocumentInitial,
                            prefix: PrefixPosition::None,
                        },
                        environment: &environment,
                        context: &equal,
                    },
                    terminal,
                )
                .is_empty(),
                "identity accepted a non-boundary suffix in {text:?}"
            );
        }
    }

    #[test]
    fn structural_trace_declaration_noun_value_label_is_pinned() {
        let environment = canonical_test_environment();
        let id = DeclarationId::new(DeclarationKind::Type, "Creature");
        assert_eq!(
            environment.surface(&id, SurfaceFeature::Plural),
            Some("creatures")
        );
        let identity = DeclarationNoun::from_reading(
            id,
            deckmaste_construction_core::macro_def::NounLocativeTemporalLicense::ObjectAttachmentLicensed,
            deckmaste_construction_core::macro_def::NounRelationality::QualifiedRelational,
            false,
        )
        .expect("Type is an allowed declaration noun kind");
        let value = Leaf::Noun {
            noun: Noun::Declaration(identity),
            number: super::Number::Plural,
            onset: Onset::Consonant,
            possessive_ending: PossessiveEnding::EndsInS,
        };
        assert_eq!(
            value_label_v1(&value),
            "Noun { noun: Declaration(DeclarationNoun { id: DeclarationIdentity { kind: Type, name: \"Creature\" }, locative_temporal_license: ObjectAttachmentLicensed, relationality: QualifiedRelational, number_invariant: false }), number: Plural, onset: Consonant, possessive_ending: EndsInS }"
        );
    }

    #[test]
    fn scanner_rejects_case_and_spacing_that_render_would_not_emit() {
        for text in [
            "destroy target creature.",
            "Destroy Target creature.",
            "Destroy  target creature.",
            "Destroytarget creature.",
            "Whenever a player connives, you Gain X life.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "accepted {text:?}"
            );
        }
    }

    #[test]
    fn scanner_accepts_lowercase_you_after_the_trigger_comma() {
        assert!(
            slice_candidates(
                "Whenever a player connives, you gain X life.",
                "Context Card"
            )
            .is_ok()
        );
    }
    #[test]
    fn chart_completion_rejects_invalid_concord_class_and_count_facts() {
        for text in [
            "You gains X life.",
            "Creatures you control with power 2 or less gains X life.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "completed invalid chart family for {text:?}"
            );
        }
    }

    #[test]
    fn controller_qualification_requires_you_and_bare_control() {
        assert!(
            slice_candidates(
                "You gain X life, where X is the number of creatures it controls with power 2 or less.",
                "Context Card"
            )
            .is_ok(),
            "a grammatical non-you subject is admitted without a game-semantic controller guard"
        );
        let parser = Parser::new(canonical_test_environment())
            .expect("canonical environment satisfies the grammar");
        let context = context("Context Card");
        assert!(
            parser
                .analyze(
                    "You gain X life, where X is the number of creatures you controls with power 2 or less.",
                    &context,
                )
                .decision()
                .is_none(),
            "derived concord_class leaves the invalid subject with no reading at all"
        );

        assert!(
            slice_candidates(
                "You gain X life, where X is the number of creatures you control with power 2 or less.",
                "Context Card"
            )
            .is_ok()
        );
    }

    #[test]
    fn chart_rejects_a_finite_clause_in_the_typed_where_clause_slot() {
        let text = "Whenever where X is the number of creatures you control with power 2 or less, you gain X life.";
        assert!(
            slice_candidates(text, "Context Card").is_err(),
            "the non-left-recursive staging rejects a Where clause where FiniteClause is required",
        );
    }
}
