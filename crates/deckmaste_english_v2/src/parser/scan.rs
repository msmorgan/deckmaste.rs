use std::cmp::Ordering;

use macro_ron::v2::DeclarationKind;
use macro_ron::v2::GrammarPosition;
use macro_ron::v2::SurfaceFeature;

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
use super::diagnostic::order_bounded_prefix;
use super::engine::ChartFailure;
use super::engine::Child;
use super::engine::Family;
use super::engine::Forest;
use super::engine::LexicalMatch;
use super::engine::Observation;
use super::engine::parse;
use super::engine::parse_observed;
use super::materialize::completion_has_checked_build;
use crate::ast::VerbLexeme;
use crate::constructions::Agreement;
use crate::constructions::CasePosition;
use crate::constructions::Category;
use crate::constructions::DeclarationMatcher;
use crate::constructions::FeatureConstraint;
use crate::constructions::Leaf;
use crate::constructions::Lexical;
use crate::constructions::LexicalOwner;
use crate::constructions::LexicalTerminal;
use crate::constructions::Number;
use crate::constructions::RULES;
use crate::constructions::RuleId;
use crate::constructions::ScanPosition;
use crate::constructions::scan_lexical;
use crate::context::ParseContext;
use crate::environment::DeclarationId;
use crate::environment::ParserEnvironment;
use crate::features::inflect;
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

pub(crate) fn parse_forest(
    grammar: &SliceGrammar<'_>,
    text: &str,
) -> Result<Forest<RuleId, Leaf, LexicalOwner>, ChartFailure<Category, Lexical>> {
    parse(
        RULES,
        Category::Ability,
        text.len(),
        |lexical, offset| grammar.scan(lexical, text, offset),
        |rule, family, forest| completion_has_checked_build(rule, family, forest, grammar.context),
    )
    .map_err(project_failure)
}

type ObservedForestResult =
    Result<Forest<RuleId, Leaf, LexicalOwner>, ChartFailure<Category, Lexical>>;

pub(crate) fn parse_forest_observed(
    grammar: &SliceGrammar<'_>,
    text: &str,
    limits: TraceLimits,
) -> (ObservedForestResult, StructuralTrace) {
    let mut observation = StructuralObservation::new(limits);
    let result = parse_observed(
        RULES,
        Category::Ability,
        text.len(),
        |lexical, offset| grammar.scan(lexical, text, offset),
        |rule, family, forest| completion_has_checked_build(rule, family, forest, grammar.context),
        &mut observation,
    )
    .map_err(project_failure);
    let trace = observation.finish();
    (result, trace)
}

struct StructuralObservation {
    limit: usize,
    scanner_matches: SemanticScannerMatchInventory<Lexical, (Leaf, Option<LexicalOwner>)>,
    chart: Bounded<ChartItem>,
    forest: Bounded<ForestNode>,
    roots: Bounded<usize>,
    rejections: Vec<CheckedRejectionIdentity>,
}

#[derive(Clone, PartialEq, Eq)]
struct CheckedRejectionIdentity {
    rule: RuleId,
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
        StructuralTrace::new(
            scanner_matches,
            self.chart,
            self.forest,
            self.roots,
            rejections,
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

impl Observation<RuleId, Leaf, LexicalTerminal, LexicalOwner> for StructuralObservation {
    fn scanned(&mut self, start: usize, terminal: LexicalTerminal, end: usize, value: &Leaf) {
        self.record_scanned_token(start, end, terminal, value);
    }

    fn checked_completion(
        &mut self,
        rule: RuleId,
        start: usize,
        end: usize,
        family: &Family<Leaf, LexicalOwner>,
        accepted: bool,
    ) {
        let key = CheckedRejectionIdentity {
            rule,
            start,
            end,
            family: raw_family_identity(family),
        };
        if accepted {
            self.rejections.retain(|rejection| rejection != &key);
        } else if !self.rejections.contains(&key) {
            self.rejections.push(key);
        }
    }

    fn chart_item(
        &mut self,
        column: usize,
        rule: RuleId,
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

    fn final_forest(&mut self, forest: &Forest<RuleId, Leaf, LexicalOwner>) {
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

fn rule_name_v1(rule: RuleId) -> String {
    #[cfg(test)]
    TRACE_LABEL_COUNTS.with(|counts| {
        let mut current = counts.get();
        current.rules += 1;
        counts.set(current);
    });
    format!("{rule:?}")
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
    format!("{value:?}")
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
    pub(super) fn scan(
        &self,
        terminal: LexicalTerminal,
        text: &str,
        offset: usize,
    ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        let position = ScanPosition {
            byte_offset: offset,
            case: if offset == 0 {
                CasePosition::DocumentInitial
            } else {
                CasePosition::Continuation
            },
        };
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

impl ScanInput<'_> {
    pub(crate) fn word_end(&self, running_text: &str) -> Option<usize> {
        let offset = self.position.byte_offset;
        debug_assert_eq!(
            self.position.case,
            if offset == 0 {
                CasePosition::DocumentInitial
            } else {
                CasePosition::Continuation
            }
        );
        let prefix = usize::from(self.position.case == CasePosition::Continuation);
        let remainder = self.text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let word = if self.position.case == CasePosition::DocumentInitial {
            initial_surface(running_text)
        } else {
            running_text.to_owned()
        };
        let end = offset + prefix + word.len();
        (remainder.starts_with(&word) && has_lexical_boundary(self.text, end)).then_some(end)
    }

    pub(crate) fn identity_end(&self, exact_text: &str) -> Option<usize> {
        let offset = self.position.byte_offset;
        let prefix = usize::from(self.position.case == CasePosition::Continuation);
        let remainder = self.text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let end = offset + prefix + exact_text.len();
        (!exact_text.is_empty()
            && end > offset
            && remainder.starts_with(exact_text)
            && has_lexical_boundary(self.text, end))
        .then_some(end)
    }

    pub(crate) fn punctuation_end(&self, punctuation: &str) -> Option<usize> {
        let offset = self.position.byte_offset;
        self.text
            .get(offset..)?
            .starts_with(punctuation)
            .then_some(offset + punctuation.len())
    }

    pub(crate) fn declaration_readings(
        &self,
        matcher: DeclarationMatcher,
    ) -> Vec<(usize, DeclarationId, SurfaceFeature)> {
        lookup_declaration_readings(
            self.text,
            self.position.byte_offset,
            self.position.case == CasePosition::DocumentInitial,
            self.environment,
            matcher.kind,
            matcher.name,
            matcher.position,
            |feature| matches_feature(matcher.feature, feature),
        )
    }

    pub(crate) fn declaration_noun_readings(
        &self,
        position: GrammarPosition,
        wanted: FeatureConstraint<Number>,
    ) -> Vec<(usize, DeclarationId, SurfaceFeature)> {
        let offset = self.position.byte_offset;
        let document_initial = self.position.case == CasePosition::DocumentInitial;
        let prefix = usize::from(!document_initial);
        let Some(remainder) = self.text.get(offset..) else {
            return Vec::new();
        };
        let Some(surface_text) = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))
        else {
            return Vec::new();
        };
        let surface_byte_limit = if document_initial {
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
            if !has_lexical_boundary(self.text, end) {
                continue;
            }
            let candidate = &surface_text[..relative_end];
            let readings = if document_initial {
                self.environment.initial_readings(position, candidate)
            } else {
                self.environment.readings(position, candidate)
            };
            for reading in readings {
                let number = match reading.feature() {
                    SurfaceFeature::Singular => Number::Singular,
                    SurfaceFeature::Plural => Number::Plural,
                    SurfaceFeature::Bare
                    | SurfaceFeature::ThirdPersonSingular
                    | SurfaceFeature::Fixed => continue,
                };
                if matches!(wanted, FeatureConstraint::Any)
                    || matches!(wanted, FeatureConstraint::Exact(expected) if expected == number)
                {
                    results.push((end, reading.id().clone(), reading.feature()));
                }
            }
        }
        results.sort();
        results.dedup();
        results
    }

    fn scan_verb(&self, lexeme: VerbLexeme) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
        [Agreement::Bare, Agreement::ThirdPersonSingular]
            .into_iter()
            .filter_map(|agreement| {
                self.word_end(inflect(lexeme, agreement))
                    .map(|end| LexicalMatch {
                        end,
                        value: Leaf::Verb { lexeme, agreement },
                        owner: None,
                    })
            })
            .collect()
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "the shared lookup seam keeps synthetic generated grammars on the production scanner algorithm"
)]
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
    let prefix = usize::from(!document_initial);
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
    let surface_byte_limit = if document_initial {
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
        if !has_lexical_boundary(text, end) {
            continue;
        }
        let candidate = &surface_text[..relative_end];
        let readings = if document_initial {
            environment.initial_readings(position, candidate)
        } else {
            environment.readings(position, candidate)
        };
        for reading in readings {
            if reading.id().kind() == kind
                && reading.id().name() == name
                && matches_feature(reading.feature())
            {
                results.push((end, reading.id().clone(), reading.feature()));
            }
        }
    }
    results.sort();
    results.dedup();
    results
}

pub(crate) fn scan_bound_terminal(
    input: &ScanInput<'_>,
    terminal: LexicalTerminal,
) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
    match terminal.matcher {
        Lexical::Verb(lexeme) => input.scan_verb(lexeme),
        Lexical::Literal(_)
        | Lexical::EndOfInput
        | Lexical::TriggerWord
        | Lexical::Article
        | Lexical::Demonstrative
        | Lexical::Pronoun
        | Lexical::Variable
        | Lexical::SelfReference
        | Lexical::SignedNumber
        | Lexical::Declaration(_)
        | Lexical::Noun(_) => {
            unreachable!("generated scanner delegated a terminal it owns")
        }
    }
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
                super::engine::RulePosition::Lexical(terminal) => {
                    super::engine::RulePosition::Lexical(terminal.matcher)
                }
            })
            .collect(),
    }
}

fn has_lexical_boundary(text: &str, end: usize) -> bool {
    matches!(text.as_bytes().get(end), None | Some(b' ' | b',' | b'.'))
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

    use macro_ron::v2::DeclarationKind;
    use macro_ron::v2::GrammarPosition;
    use macro_ron::v2::SubtypeCategory;
    use macro_ron::v2::SurfaceFeature;
    use macro_ron::v2::read_builtin_v2;
    use macro_ron::v2::read_str;

    use super::super::engine::Child;
    use super::super::engine::Family;
    use super::super::engine::NodeId;
    use super::super::engine::Observation;
    use super::super::engine::SpannedLexical;
    use super::Category;
    use super::ChartFailure;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
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
    use crate::ast::Article;
    use crate::ast::DeclarationNoun;
    use crate::ast::Demonstrative;
    use crate::ast::Noun;
    use crate::ast::NounLexeme;
    use crate::ast::Pronoun;
    use crate::ast::SelfReferenceSpelling;
    use crate::ast::Sign;
    use crate::ast::SignedNumber;
    use crate::ast::TriggerWord;
    use crate::ast::Variable;
    use crate::ast::VerbLexeme;
    use crate::constructions::Agreement;
    use crate::constructions::CasePosition;
    use crate::constructions::DeclarationLeaf;
    use crate::constructions::DeclarationMatcher;
    use crate::constructions::FeatureConstraint;
    use crate::constructions::LexicalOwnerTemplate;
    use crate::constructions::LexicalProvenanceKind;
    use crate::constructions::LexicalTerminal;
    use crate::constructions::Number;
    use crate::constructions::RULES;
    use crate::constructions::ScanPosition;
    use crate::context::ParseContext;
    use crate::environment::DeclarationId;
    use crate::environment::ParserEnvironment;
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
    ) -> Result<
        Forest<RuleId, Leaf, crate::constructions::LexicalOwner>,
        ChartFailure<Category, Lexical>,
    > {
        let environment = canonical_test_environment();
        let context = context(card_name);
        let grammar = SliceGrammar {
            environment: &environment,
            context: &context,
        };

        parse_forest(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(card_name).expect("test card names are valid parse contexts")
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
            r#"KeywordAction(name:"{name}",spelling:"{bare}",grammar:Verb(bare:"{bare}"{override_field},valence:Numerative))"#
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
        }
    }

    #[test]
    fn owned_declaration_owner_ids_outlive_the_source_and_include_feature() {
        let source = String::from(
            r#"KeywordAction(name:"Novel",spelling:"novel",grammar:Verb(bare:"novel",valence:Intransitive))"#,
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
                r#"KeywordAction(name:"Novel",spelling:"novel",grammar:Verb(bare:"novel",valence:Intransitive))"#,
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
                r#"KeywordAction(name:"{name}",spelling:"{bare}",grammar:Verb(bare:"{bare}"{override_field},valence:Numerative))"#
            );
            declarations.push(
                read_str(format!("/synthetic/{name}.ron"), &source)
                    .expect("synthetic keyword action is valid"),
            );
        }
        ParserEnvironment::try_from_declarations(declarations)
            .expect("parser declaration environment freezes")
    }

    #[test]
    fn generated_vocab_scan_uses_exact_tables_boundaries_and_owners() {
        let environment = ParserEnvironment::try_from_declarations([])
            .expect("an empty declaration environment is valid");
        let context = context("Context Card");
        let cases = [
            (
                Lexical::TriggerWord,
                Leaf::TriggerWord(TriggerWord::Whenever),
                "whenever",
                "vocab:TriggerWord/Whenever",
            ),
            (
                Lexical::Article,
                Leaf::Article(Article::An),
                "an",
                "vocab:Article/An",
            ),
            (
                Lexical::Demonstrative,
                Leaf::Demonstrative(Demonstrative::Those),
                "those",
                "vocab:Demonstrative/Those",
            ),
            (
                Lexical::Pronoun,
                Leaf::Pronoun(Pronoun::You),
                "you",
                "vocab:Pronoun/You",
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
                Lexical::TriggerWord => LexicalOwnerTemplate::Vocab {
                    declaration: "TriggerWord",
                },
                Lexical::Article => LexicalOwnerTemplate::Vocab {
                    declaration: "Article",
                },
                Lexical::Demonstrative => LexicalOwnerTemplate::Vocab {
                    declaration: "Demonstrative",
                },
                Lexical::Pronoun => LexicalOwnerTemplate::Vocab {
                    declaration: "Pronoun",
                },
                Lexical::Variable => LexicalOwnerTemplate::Vocab {
                    declaration: "Variable",
                },
                _ => unreachable!("the fixture contains only finite vocab terminals"),
            };
            let terminal = LexicalTerminal { matcher, owner };

            let initial =
                if running == "X" { running.to_owned() } else { initial_surface(running) };
            let initial_input = ScanInput {
                text: &initial,
                position: ScanPosition {
                    byte_offset: 0,
                    case: CasePosition::DocumentInitial,
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
            },
            environment: &environment,
            context: &context,
        };

        assert_eq!(input.word_end("élan"), Some("Élan".len()));
        assert_eq!(input.word_end("élan vital"), Some("Élan vital".len()));
        assert_eq!(input.word_end("éLan"), None);
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
                feature: SurfaceFeature::Bare,
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
    fn lexical_boundary_is_exactly_eoi_ascii_space_comma_or_period() {
        for text in ["word", "word ", "word,", "word."] {
            assert!(super::has_lexical_boundary(text, "word".len()), "{text:?}");
        }
        for text in ["word?", "word‽", "word_", "word!", "wordx", "wordé"] {
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
            r#"CounterKind(name:"SharpS",spelling:"ßeta",grammar:FixedTerm(surface:"ßeta"))"#,
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
        };
        let input = ScanInput {
            text: "SSeta.",
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
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
        };
        let text = format!("Scry{}.", " x".repeat(4_096));
        let input = ScanInput {
            text: &text,
            position: ScanPosition {
                byte_offset: 0,
                case: CasePosition::DocumentInitial,
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
                r#"KeywordAction(name:"Alpha",spelling:"echo",grammar:Verb(bare:"echo",third_person:"echo",valence:Intransitive))"#,
            ),
            (
                "/synthetic/Zeta.ron",
                r#"KeywordAction(name:"Zeta",spelling:"echo",grammar:Verb(bare:"echo",valence:Intransitive))"#,
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
                    SurfaceFeature::Bare,
                ),
                (
                    "Echo".len(),
                    DeclarationId::new(DeclarationKind::KeywordAction, "Alpha"),
                    SurfaceFeature::ThirdPersonSingular,
                ),
            ]
        );
        let alpha_third = scan(terminal(
            DeclarationKind::KeywordAction,
            "Alpha",
            FeatureConstraint::Exact(SurfaceFeature::ThirdPersonSingular),
        ));
        assert_eq!(
            project(&alpha_third),
            [(
                "Echo".len(),
                DeclarationId::new(DeclarationKind::KeywordAction, "Alpha"),
                SurfaceFeature::ThirdPersonSingular,
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
            FeatureConstraint::Exact(SurfaceFeature::Bare),
        ));
        assert!(matches!(
            zeta.as_slice(),
            [matched]
                if matches!(
                    &matched.value,
                    Leaf::Declaration(leaf)
                        if leaf.id
                            == DeclarationId::new(DeclarationKind::KeywordAction, "Zeta")
                            && leaf.feature == SurfaceFeature::Bare
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
    fn declaration_noun_scanner_retains_collisions_features_and_exact_offsets() {
        let declarations = [
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
                "/synthetic/types/Player.ron",
                r#"Type(name:"Player",spelling:"player",grammar:Noun(singular:"player"))"#,
            )
            .unwrap(),
            read_str(
                "/synthetic/abilities/Flying.ron",
                r#"KeywordAbility(name:"Flying",spelling:"flying",grammar:Noun(singular:"flying"))"#,
            )
            .unwrap(),
        ];
        let environment = ParserEnvironment::try_from_declarations(declarations).unwrap();
        let context = context("Context Card");
        let scan = |text, byte_offset, case, wanted| {
            super::scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition { byte_offset, case },
                    environment: &environment,
                    context: &context,
                },
                LexicalTerminal {
                    matcher: Lexical::Noun(wanted),
                    owner: LexicalOwnerTemplate::DeclarationNoun,
                },
            )
        };
        let declarations =
            |matches: Vec<super::LexicalMatch<Leaf, crate::constructions::LexicalOwner>>| {
                matches
                    .into_iter()
                    .filter_map(|matched| match matched.value {
                        Leaf::Noun {
                            noun: Noun::Declaration(noun),
                            number,
                        } => Some((
                            matched.end,
                            noun.id().kind(),
                            noun.id().name().to_owned(),
                            noun.feature(),
                            number,
                        )),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            };

        assert_eq!(
            declarations(scan(
                "Elves.",
                0,
                CasePosition::DocumentInitial,
                FeatureConstraint::Exact(Number::Plural),
            )),
            [
                (
                    5,
                    DeclarationKind::Subtype(SubtypeCategory::Creature),
                    "Elf".to_owned(),
                    SurfaceFeature::Plural,
                    Number::Plural,
                ),
                (
                    5,
                    DeclarationKind::Type,
                    "Elf".to_owned(),
                    SurfaceFeature::Plural,
                    Number::Plural,
                ),
            ],
            "same-spelling Type/Subtype readings remain distinct in identity order",
        );
        assert_eq!(
            declarations(scan(
                "prefix elf.",
                6,
                CasePosition::Continuation,
                FeatureConstraint::Exact(Number::Singular),
            )),
            [
                (
                    10,
                    DeclarationKind::Subtype(SubtypeCategory::Creature),
                    "Elf".to_owned(),
                    SurfaceFeature::Singular,
                    Number::Singular,
                ),
                (
                    10,
                    DeclarationKind::Type,
                    "Elf".to_owned(),
                    SurfaceFeature::Singular,
                    Number::Singular,
                ),
            ],
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
            "core/declaration collision retains both branches"
        );
        assert!(player.iter().all(|matched| matched.end == 6));
        assert!(player.iter().any(|matched| matches!(
            matched.value,
            Leaf::Noun {
                noun: Noun::Lexeme(NounLexeme::Player),
                number: Number::Singular
            }
        )));
        assert!(player.iter().any(|matched| matches!(
            &matched.value,
            Leaf::Noun { noun: Noun::Declaration(noun), number: Number::Singular }
                if noun.id() == &DeclarationId::new(DeclarationKind::Type, "Player")
        )));
        assert_noun_collision_owners(&player);
        assert!(
            scan(
                "Flying.",
                0,
                CasePosition::DocumentInitial,
                FeatureConstraint::Any,
            )
            .is_empty(),
            "a noun-position reading of a disallowed declaration kind is rejected",
        );
    }

    fn assert_noun_collision_owners(
        player: &[super::LexicalMatch<Leaf, crate::constructions::LexicalOwner>],
    ) {
        let owners = player
            .iter()
            .map(|matched| {
                LexicalOwnerTemplate::DeclarationNoun
                    .instantiate(&matched.value)
                    .expect("each noun branch has an exact owner")
            })
            .collect::<Vec<_>>();
        assert!(owners.iter().any(|owner| {
            owner.kind() == LexicalProvenanceKind::Lexeme
                && owner.stable_id() == "lexeme:NounLexeme/Player"
        }));
        assert!(owners.iter().any(|owner| {
            owner.kind() == LexicalProvenanceKind::Lexeme
                && owner.stable_id() == "lexeme:type/Player/singular"
        }));
        assert!(owners.iter().all(|owner| owner.stable_id() != "codec:Noun"));
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
        observed.checked_completion(RuleId::AmountNumber, 1, 4, &family, false);
        observed.checked_completion(RuleId::AmountNumber, 1, 4, &family, true);
        let rejections = observed.finish().checked_completion_rejections().clone();
        assert_eq!(
            (rejections.total(), rejections.shown(), rejections.omitted()),
            (0, 0, 0)
        );
    }

    #[test]
    fn structural_trace_final_checked_rejection_is_structured_and_bounded() {
        let family = Family {
            children: vec![Child::Node(NodeId(7)), lexical(Leaf::Literal("where"))],
        };
        for (limit, shown) in [(0, 0), (1, 1)] {
            let mut observed = StructuralObservation::new(TraceLimits::new(limit));
            observed.checked_completion(RuleId::AmountNumber, 1, 4, &family, false);
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
            observed.checked_completion(RuleId::VerbPhraseGainLife, 1, 2, &family, false);
            observed.checked_completion(RuleId::AmountNumber, 1, 2, &family, false);

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
    fn structural_trace_generated_rule_and_terminal_names_are_pinned() {
        let mut seen_rules = BTreeSet::new();
        let rules = RULES
            .iter()
            .filter_map(|rule| seen_rules.insert(rule.id).then_some(rule_name_v1(rule.id)))
            .collect::<Vec<_>>();
        assert_eq!(
            rules,
            [
                "AbilitySpell",
                "AbilityTriggered",
                "SentenceImperative",
                "SentenceDeclarative",
                "SentenceWithWhere",
                "ClauseEvent",
                "ClauseWhere",
                "NounPhrasePronoun",
                "NounPhraseCommon",
                "NounPhraseDemonstrative",
                "NounPhraseTarget",
                "NounPhraseSelfReference",
                "NounPhraseCount",
                "VerbPhraseDestroy",
                "VerbPhraseConnive",
                "VerbPhraseDealDamage",
                "VerbPhraseGainLife",
                "AmountNumber",
                "AmountVariable"
            ]
        );
        let mut seen_lexical = BTreeSet::new();
        let terminals = RULES
            .iter()
            .flat_map(|rule| rule.rhs)
            .filter_map(|position| match position {
                super::super::engine::RulePosition::Lexical(lexical) => seen_lexical
                    .insert(lexical.matcher)
                    .then_some(terminal_name_v1(lexical.matcher)),
                super::super::engine::RulePosition::Nonterminal(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            terminals,
            [
                "Literal(\".\")",
                "EndOfInput",
                "TriggerWord",
                "Literal(\",\")",
                "Literal(\"where\")",
                "Variable",
                "Verb(Be)",
                "Literal(\"the\")",
                "Literal(\"number\")",
                "Literal(\"of\")",
                "Pronoun",
                "Article",
                "Noun(Exact(Singular))",
                "Demonstrative",
                "Noun(Any)",
                "Literal(\"target\")",
                "SelfReference",
                "Noun(Exact(Plural))",
                "Verb(Control)",
                "Literal(\"with\")",
                "Literal(\"power\")",
                "SignedNumber",
                "Literal(\"or\")",
                "Literal(\"less\")",
                "Declaration(DeclarationMatcher { kind: KeywordAction, name: \"Destroy\", position: Verb, feature: Any })",
                "Declaration(DeclarationMatcher { kind: KeywordAction, name: \"Connive\", position: Verb, feature: Any })",
                "Verb(Deal)",
                "Literal(\"damage\")",
                "Literal(\"to\")",
                "Verb(Gain)",
                "Literal(\"life\")"
            ]
        );
    }

    #[test]
    fn structural_trace_other_value_labels_are_pinned() {
        let values = [
            (Leaf::Literal("where"), "Literal(\"where\")"),
            (Leaf::EndOfInput, "EndOfInput"),
            (
                Leaf::TriggerWord(TriggerWord::Whenever),
                "TriggerWord(Whenever)",
            ),
            (Leaf::Article(Article::A), "Article(A)"),
            (Leaf::Article(Article::An), "Article(An)"),
            (
                Leaf::Demonstrative(Demonstrative::That),
                "Demonstrative(That)",
            ),
            (
                Leaf::Demonstrative(Demonstrative::Those),
                "Demonstrative(Those)",
            ),
            (Leaf::Pronoun(Pronoun::It), "Pronoun(It)"),
            (Leaf::Pronoun(Pronoun::You), "Pronoun(You)"),
            (Leaf::Variable(Variable::X), "Variable(X)"),
            (
                Leaf::Noun {
                    noun: Noun::Lexeme(NounLexeme::Player),
                    number: super::Number::Singular,
                },
                "Noun { noun: Lexeme(Player), number: Singular }",
            ),
            (
                Leaf::Noun {
                    noun: Noun::Lexeme(NounLexeme::Player),
                    number: super::Number::Plural,
                },
                "Noun { noun: Lexeme(Player), number: Plural }",
            ),
            (
                Leaf::Declaration(DeclarationLeaf {
                    id: DeclarationId::new(DeclarationKind::KeywordAction, "Destroy"),
                    feature: SurfaceFeature::Bare,
                }),
                "Declaration(DeclarationLeaf { id: DeclarationIdentity { kind: KeywordAction, name: \"Destroy\" }, feature: Bare })",
            ),
            (
                Leaf::Verb {
                    lexeme: VerbLexeme::Deal,
                    agreement: Agreement::ThirdPersonSingular,
                },
                "Verb { lexeme: Deal, agreement: ThirdPersonSingular }",
            ),
            (
                Leaf::SignedNumber(SignedNumber {
                    sign: Sign::Positive,
                    magnitude: 2,
                }),
                "SignedNumber(SignedNumber { sign: Positive, magnitude: 2 })",
            ),
            (
                Leaf::SignedNumber(SignedNumber {
                    sign: Sign::Negative,
                    magnitude: 2,
                }),
                "SignedNumber(SignedNumber { sign: Negative, magnitude: 2 })",
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
    fn signed_decimal_scanner_is_canonical_at_both_scan_positions() {
        let environment = canonical_test_environment();
        let context = context("Test Card");
        let terminal = LexicalTerminal {
            matcher: Lexical::SignedNumber,
            owner: LexicalOwnerTemplate::Static {
                kind: crate::constructions::LexicalProvenanceKind::Codec,
                stable_id: "codec:SignedNumber",
            },
        };
        let accepted = [
            ("0", Sign::Positive, 0),
            ("-0", Sign::Negative, 0),
            ("1", Sign::Positive, 1),
            ("-1", Sign::Negative, 1),
            ("4294967295", Sign::Positive, u32::MAX),
            ("-4294967295", Sign::Negative, u32::MAX),
        ];

        for (number, sign, magnitude) in accepted {
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
                        position: ScanPosition { byte_offset, case },
                        environment: &environment,
                        context: &context,
                    },
                    terminal,
                );
                assert_eq!(matches.len(), 1, "{text:?} at {case:?}");
                assert_eq!(matches[0].end, expected_end, "{text:?} at {case:?}");
                assert_eq!(
                    matches[0].value,
                    Leaf::SignedNumber(SignedNumber { sign, magnitude }),
                    "{text:?} at {case:?}"
                );
                let owner = terminal
                    .owner
                    .instantiate(&matches[0].value)
                    .expect("signed decimals own their bytes");
                assert_eq!(owner.stable_id(), "codec:SignedNumber");
            }
        }

        for number in [
            "",
            "-",
            "00",
            "01",
            "-00",
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
                        position: ScanPosition { byte_offset, case },
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

        for text in ["1.", "-1,"] {
            let matches = super::scan_lexical(
                &ScanInput {
                    text,
                    position: ScanPosition {
                        byte_offset: 0,
                        case: CasePosition::DocumentInitial,
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
                            position: ScanPosition { byte_offset, case },
                            environment: &environment,
                            context: &context,
                        },
                        terminal,
                    );
                    let expected_count = 1 + usize::from(
                        card_name.contains(',') && spelling == SelfReferenceSpelling::Full,
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
        let identity = DeclarationNoun::from_reading(id, SurfaceFeature::Plural)
            .expect("Type is an allowed declaration noun kind");
        let value = Leaf::Noun {
            noun: Noun::Declaration(identity),
            number: super::Number::Plural,
        };
        assert_eq!(
            value_label_v1(&value),
            "Noun { noun: Declaration(DeclarationNoun { id: DeclarationIdentity { kind: Type, name: \"Creature\" }, feature: Plural }), number: Plural }"
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
    fn chart_completion_rejects_invalid_agreement_and_count_facts() {
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
    fn count_np_requires_you_and_bare_control() {
        for text in [
            "You gain X life, where X is the number of creatures it control with power 2 or less.",
            "You gain X life, where X is the number of creatures you controls with power 2 or less.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "completed invalid count noun phrase for {text:?}"
            );
        }

        assert!(
            slice_candidates(
                "You gain X life, where X is the number of creatures you control with power 2 or less.",
                "Context Card"
            )
            .is_ok()
        );
    }

    #[test]
    fn chart_completion_rejects_a_construction_category_mismatch() {
        let text = "Whenever where X is the number of creatures you control with power 2 or less, you gain X life.";
        let Err(failure) = slice_candidates(text, "Context Card") else {
            panic!("a where clause cannot satisfy an event-clause construction role");
        };

        assert!(!failure.live.is_empty());
    }
}
