use std::cmp::Ordering;

use deckmaste_catalogs::CatalogKind;

use super::diagnostic::Bounded;
use super::diagnostic::ChartItem;
use super::diagnostic::CheckedCompletionRejection;
use super::diagnostic::FamilyIdentity;
use super::diagnostic::FamilyIdentityChild;
use super::diagnostic::ForestChild;
use super::diagnostic::ForestFamily;
use super::diagnostic::ForestNode;
use super::diagnostic::SemanticTokenInventory;
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
use super::lexical::Lexical;
use super::lexical::NounNumber;
use super::materialize::completion_has_checked_build;
use crate::ast::Article;
use crate::ast::CatalogIdentity;
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
use crate::catalogs::ParserCatalogs;
use crate::constructions::Category;
use crate::constructions::RULES;
use crate::constructions::RuleId;
use crate::context::ParseContext;
use crate::features::Agreement;
use crate::features::inflect;

pub(crate) struct SliceGrammar<'a> {
    pub(crate) catalogs: &'a ParserCatalogs,
    pub(crate) context: &'a ParseContext<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Leaf {
    Literal(&'static str),
    EndOfInput,
    TriggerWord(TriggerWord),
    Article(Article),
    Demonstrative(Demonstrative),
    Pronoun(Pronoun),
    Variable(Variable),
    Noun {
        noun: Noun,
        number: NounNumber,
    },
    Verb {
        lexeme: VerbLexeme,
        agreement: Agreement,
    },
    SignedNumber(SignedNumber),
    SelfReference(SelfReferenceSpelling),
}

pub(crate) fn parse_forest(
    grammar: &SliceGrammar<'_>,
    text: &str,
) -> Result<Forest<RuleId, Leaf>, ChartFailure<Category, Lexical>> {
    parse(
        RULES,
        Category::Ability,
        text.len(),
        |lexical, offset| grammar.scan(lexical, text, offset),
        |rule, family, forest| completion_has_checked_build(rule, family, forest, grammar.context),
    )
}

type ObservedForestResult = Result<Forest<RuleId, Leaf>, ChartFailure<Category, Lexical>>;

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
    );
    let trace = observation.finish();
    (result, trace)
}

struct StructuralObservation {
    limit: usize,
    tokens: SemanticTokenInventory<Lexical, Leaf>,
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
            tokens: SemanticTokenInventory::default(),
            chart: Bounded::new(limits.per_collection()),
            forest: Bounded::new(limits.per_collection()),
            roots: Bounded::new(limits.per_collection()),
            rejections: Vec::new(),
        }
    }

    fn finish(mut self) -> StructuralTrace {
        let tokens = self.tokens.into_bounded_by(
            self.limit,
            stable_debug_cmp,
            stable_debug_cmp,
            terminal_name_v1,
            value_label_v1,
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
        StructuralTrace::new(tokens, self.chart, self.forest, self.roots, rejections)
    }

    fn record_token(&mut self, start: usize, end: usize, terminal: Lexical, value: &Leaf) {
        self.tokens.record(start, end, terminal, value.clone());
    }
}

impl Observation<RuleId, Leaf, Lexical> for StructuralObservation {
    fn scanned(&mut self, start: usize, terminal: Lexical, end: usize, value: &Leaf) {
        self.record_token(start, end, terminal, value);
    }

    fn checked_completion(
        &mut self,
        rule: RuleId,
        start: usize,
        end: usize,
        family: &Family<Leaf>,
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

    fn final_forest(&mut self, forest: &Forest<RuleId, Leaf>) {
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
                                Child::Lexical(value) => ForestChild {
                                    node_id: None,
                                    value_label_v1: Some(value_label_v1(value)),
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
fn raw_family_identity(family: &Family<Leaf>) -> RawFamilyIdentity {
    RawFamilyIdentity(
        family
            .children
            .iter()
            .map(|child| match child {
                Child::Node(id) => RawFamilyIdentityChild::Node(id.0),
                Child::Lexical(value) => RawFamilyIdentityChild::Lexical(value.clone()),
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
    fn scan(&self, lexical: Lexical, text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
        match lexical {
            Lexical::EndOfInput => (offset == text.len())
                .then_some(LexicalMatch {
                    end: offset,
                    value: Leaf::EndOfInput,
                })
                .into_iter()
                .collect(),
            Lexical::Literal(literal @ ("." | ",")) => text[offset..]
                .starts_with(literal)
                .then_some(LexicalMatch {
                    end: offset + literal.len(),
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::Literal(literal) => Self::word(text, offset, literal)
                .map(|end| LexicalMatch {
                    end,
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::TriggerWord => Self::closed_word(
                text,
                offset,
                "whenever",
                Leaf::TriggerWord(TriggerWord::Whenever),
            ),
            Lexical::Article => [
                ("a", Leaf::Article(Article::A)),
                ("an", Leaf::Article(Article::An)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Demonstrative => [
                ("that", Leaf::Demonstrative(Demonstrative::That)),
                ("those", Leaf::Demonstrative(Demonstrative::Those)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Pronoun => [
                ("it", Leaf::Pronoun(Pronoun::It)),
                ("you", Leaf::Pronoun(Pronoun::You)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Variable => Self::closed_word(text, offset, "X", Leaf::Variable(Variable::X)),
            Lexical::Noun(number) => self.scan_noun(text, offset, number),
            Lexical::Verb(lexeme) => Self::scan_verb(text, offset, lexeme),
            Lexical::SignedNumber => Self::scan_signed_number(text, offset),
            Lexical::SelfReference => std::iter::once((
                self.context.card_name(),
                Leaf::SelfReference(SelfReferenceSpelling::Full),
            ))
            .chain(
                (self.context.abbreviated_card_name() != self.context.card_name()).then_some((
                    self.context.abbreviated_card_name(),
                    Leaf::SelfReference(SelfReferenceSpelling::Abbreviated),
                )),
            )
            .filter_map(|(word, value)| {
                Self::identity(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
        }
    }

    fn closed_word(text: &str, offset: usize, word: &str, value: Leaf) -> Vec<LexicalMatch<Leaf>> {
        Self::word(text, offset, word)
            .map(|end| LexicalMatch { end, value })
            .into_iter()
            .collect()
    }

    fn word(text: &str, offset: usize, word: &str) -> Option<usize> {
        let prefix = usize::from(offset != 0);
        let remainder = text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let word = if offset == 0 { capitalize(word) } else { word.to_owned() };
        let end = offset + prefix + word.len();
        (remainder.starts_with(&word) && has_lexical_boundary(text, end)).then_some(end)
    }

    fn identity(text: &str, offset: usize, identity: &str) -> Option<usize> {
        let prefix = usize::from(offset != 0);
        let remainder = text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let end = offset + prefix + identity.len();
        (!identity.is_empty()
            && end > offset
            && remainder.starts_with(identity)
            && has_lexical_boundary(text, end))
        .then_some(end)
    }

    fn scan_noun(&self, text: &str, offset: usize, wanted: NounNumber) -> Vec<LexicalMatch<Leaf>> {
        let mut matches = Vec::new();
        for (noun, singular) in std::iter::once((
            Noun::Lexeme(NounLexeme::Player),
            "player".to_owned(),
        ))
        .chain(
            self.catalogs
                .set()
                .get(CatalogKind::CardTypes)
                .iter()
                .filter_map(|spelling| {
                    CatalogIdentity::new(self.catalogs, CatalogKind::CardTypes, spelling.clone())
                        .map(|identity| {
                            (
                                Noun::Catalog(identity),
                                rendered_catalog(CatalogKind::CardTypes, spelling),
                            )
                        })
                }),
        ) {
            for (number, word) in noun_forms(&singular, wanted) {
                if let Some(end) = Self::word(text, offset, &word) {
                    matches.push(LexicalMatch {
                        end,
                        value: Leaf::Noun {
                            noun: noun.clone(),
                            number,
                        },
                    });
                }
            }
        }
        matches
    }

    fn scan_verb(text: &str, offset: usize, lexeme: VerbLexeme) -> Vec<LexicalMatch<Leaf>> {
        [Agreement::Bare, Agreement::ThirdPersonSingular]
            .into_iter()
            .filter_map(|agreement| {
                Self::word(text, offset, inflect(lexeme, agreement)).map(|end| LexicalMatch {
                    end,
                    value: Leaf::Verb { lexeme, agreement },
                })
            })
            .collect()
    }

    fn scan_signed_number(text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
        let prefix = usize::from(offset != 0);
        let Some(remainder) = text.get(offset..) else {
            return Vec::new();
        };
        let Some(number) = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))
        else {
            return Vec::new();
        };
        let (sign, digits) = number
            .strip_prefix('-')
            .map_or((Sign::Positive, number), |digits| (Sign::Negative, digits));
        let digit_length = digits.bytes().take_while(u8::is_ascii_digit).count();
        let digits = &digits[..digit_length];
        let Some(magnitude) = (!digits.is_empty())
            .then(|| digits.parse::<u32>().ok())
            .flatten()
        else {
            return Vec::new();
        };
        let end = offset + prefix + usize::from(sign == Sign::Negative) + digit_length;
        (magnitude.to_string() == digits && has_lexical_boundary(text, end))
            .then_some(LexicalMatch {
                end,
                value: Leaf::SignedNumber(SignedNumber { sign, magnitude }),
            })
            .into_iter()
            .collect()
    }
}

fn has_lexical_boundary(text: &str, end: usize) -> bool {
    matches!(text.as_bytes().get(end), None | Some(b' ' | b',' | b'.'))
}

fn capitalize(word: &str) -> String {
    let Some(first) = word.chars().next() else {
        return String::new();
    };
    first.to_uppercase().chain(word.chars().skip(1)).collect()
}

fn rendered_catalog(kind: CatalogKind, spelling: &str) -> String {
    match kind {
        CatalogKind::CardTypes | CatalogKind::Supertypes => spelling.to_lowercase(),
        CatalogKind::AbilityWords
        | CatalogKind::ArtifactTypes
        | CatalogKind::BattleTypes
        | CatalogKind::CardNames
        | CatalogKind::CounterKindPhrases
        | CatalogKind::CreatureTypes
        | CatalogKind::EnchantmentTypes
        | CatalogKind::KeywordAbilities
        | CatalogKind::KeywordActions
        | CatalogKind::LandTypes
        | CatalogKind::PlaneswalkerTypes
        | CatalogKind::SpellTypes => spelling.to_owned(),
    }
}

fn noun_forms(singular: &str, wanted: NounNumber) -> Vec<(NounNumber, String)> {
    match wanted {
        NounNumber::Singular => vec![(NounNumber::Singular, singular.to_owned())],
        NounNumber::Plural => vec![(NounNumber::Plural, format!("{singular}s"))],
        NounNumber::Either => vec![
            (NounNumber::Singular, singular.to_owned()),
            (NounNumber::Plural, format!("{singular}s")),
        ],
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use super::super::engine::Child;
    use super::super::engine::Family;
    use super::super::engine::NodeId;
    use super::super::engine::Observation;
    use super::CatalogIdentity;
    use super::CatalogKind;
    use super::Category;
    use super::ChartFailure;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
    use super::RuleId;
    use super::SliceGrammar;
    use super::StructuralObservation;
    use super::TraceLimits;
    use super::parse_forest;
    use super::reset_trace_label_counts;
    use super::rule_name_v1;
    use super::terminal_name_v1;
    use super::trace_label_counts;
    use super::value_label_v1;
    use crate::ast::Article;
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
    use crate::catalogs::ParserCatalogs;
    use crate::constructions::RULES;
    use crate::context::ParseContext;
    use crate::features::Agreement;

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<Forest<RuleId, Leaf>, ChartFailure<Category, Lexical>> {
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let context = context(card_name);
        let grammar = SliceGrammar {
            catalogs: &catalogs,
            context: &context,
        };

        parse_forest(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(card_name).expect("test card names are valid parse contexts")
    }
    #[test]
    fn scanner_accepts_multi_token_context_identity_and_catalog_nouns() {
        assert!(
            slice_candidates(
                "Zacama deals 3 damage to target creature.",
                "Zacama, Primal Calamity"
            )
            .is_ok()
        );
    }

    #[test]
    fn structural_trace_scanner_inventory_dedupes_retries_sorts_and_keeps_overlaps() {
        for (limit, shown) in [(0, 0), (1, 1), (8, 3)] {
            let mut observed = StructuralObservation::new(TraceLimits::new(limit));
            observed.record_token(4, 9, Lexical::Literal("z"), &Leaf::Literal("z"));
            observed.record_token(0, 5, Lexical::Literal("a"), &Leaf::Literal("a"));
            observed.record_token(0, 5, Lexical::Literal("a"), &Leaf::Literal("a"));
            observed.record_token(0, 7, Lexical::Literal("b"), &Leaf::Literal("b"));
            let tokens = observed.finish().tokens().clone();
            assert_eq!(
                (tokens.total(), tokens.shown(), tokens.omitted()),
                (3, shown, 3 - shown)
            );
            assert_eq!(
                tokens
                    .items()
                    .iter()
                    .map(|token| (token.start, token.end))
                    .collect::<Vec<_>>(),
                [(0, 5), (0, 7), (4, 9)][..shown]
            );
        }
    }

    #[test]
    fn structural_trace_token_labels_are_built_only_for_retained_entries() {
        for (limit, expected_labels) in [(0, 0), (1, 1)] {
            reset_trace_label_counts();
            let mut observed = StructuralObservation::new(TraceLimits::new(limit));
            observed.record_token(0, 1, Lexical::Literal("z"), &Leaf::Literal("z"));
            observed.record_token(0, 1, Lexical::Literal("a"), &Leaf::Literal("a"));

            let tokens = observed.finish().tokens().clone();
            assert_eq!(
                (tokens.total(), tokens.shown(), tokens.omitted()),
                (2, expected_labels, 2 - expected_labels)
            );
            if let Some(token) = tokens.items().first() {
                assert_eq!(token.value_label_v1(), "Literal(\"a\")");
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
            children: vec![
                Child::Node(NodeId(7)),
                Child::Lexical(Leaf::Literal("where")),
            ],
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
            children: vec![
                Child::Node(NodeId(7)),
                Child::Lexical(Leaf::Literal("where")),
            ],
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
            children: vec![Child::Lexical(Leaf::Literal("family"))],
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
                    .insert(*lexical)
                    .then_some(terminal_name_v1(*lexical)),
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
                "Noun(Singular)",
                "Demonstrative",
                "Noun(Either)",
                "Literal(\"target\")",
                "SelfReference",
                "Noun(Plural)",
                "Verb(Control)",
                "Literal(\"with\")",
                "Literal(\"power\")",
                "SignedNumber",
                "Literal(\"or\")",
                "Literal(\"less\")",
                "Verb(Destroy)",
                "Verb(Connive)",
                "Verb(Deal)",
                "Literal(\"damage\")",
                "Literal(\"to\")",
                "Verb(Gain)",
                "Literal(\"life\")"
            ]
        );
    }

    #[test]
    fn structural_trace_non_catalog_value_labels_are_pinned() {
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
                    number: super::NounNumber::Singular,
                },
                "Noun { noun: Lexeme(Player), number: Singular }",
            ),
            (
                Leaf::Noun {
                    noun: Noun::Lexeme(NounLexeme::Player),
                    number: super::NounNumber::Plural,
                },
                "Noun { noun: Lexeme(Player), number: Plural }",
            ),
            (
                Leaf::Verb {
                    lexeme: VerbLexeme::Destroy,
                    agreement: Agreement::Bare,
                },
                "Verb { lexeme: Destroy, agreement: Bare }",
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
    fn structural_trace_catalog_value_label_is_pinned() {
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let identity =
            CatalogIdentity::new(&catalogs, CatalogKind::CardTypes, "Creature".to_owned())
                .expect("canonical Creature catalog identity");
        let value = Leaf::Noun {
            noun: Noun::Catalog(identity),
            number: super::NounNumber::Plural,
        };
        assert_eq!(
            value_label_v1(&value),
            "Noun { noun: Catalog(CatalogIdentity { kind: CardTypes, spelling: \"Creature\" }), number: Plural }"
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
