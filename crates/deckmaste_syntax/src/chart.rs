//! Earley chart parsing into a Packed Forest.

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::ops::Range;

use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;

use crate::compile::CompiledGrammar;
use crate::hash::ChartHasher;
use crate::reading::glues_after;
use crate::step::ChildView;
use crate::step::LeafFacts;
use crate::step::State;
use crate::step::Summary;
use crate::step::Symbol;

type ChartMap<K> = HashMap<K, usize, BuildHasherDefault<ChartHasher>>;

/// An item's identity among those ending at one position: (Production, dot,
/// start, state, seam bits).
type ItemKey = (usize, usize, usize, State, u64);

/// Sizes of one parse, for measurement only.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Metrics {
    /// Distinct chart items (Production, dot, span, state), predictions
    /// included.
    pub chart_items: usize,
    /// Completed Constituents in the Packed Forest.
    pub completed_nodes: usize,
    /// Alternative Derivations recorded across completed Constituents.
    pub families: usize,
    /// Lexical alternatives offered to the chart.
    pub leaves: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Leaf {
    pub start: usize,
    pub end: usize,
    pub reading: LexicalReading,
    pub facts: LeafFacts,
    /// It was admitted through the scope of the face whose text is parsed:
    /// one of the face's own names, or a card-scoped spelling.
    pub scoped: bool,
}

/// A child of a partial Production in a Packed Forest: a completed
/// Constituent or a lexical alternative (by index), or structural
/// punctuation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChildRef {
    Node(usize),
    Leaf(usize),
    Punct,
}

/// A partial Production: its found children are a chain of families.
#[derive(Debug)]
pub(crate) struct Item {
    pub production: usize,
    pub dot: usize,
    pub start: usize,
    pub end: usize,
    pub state: State,
    /// For each multiword noun whose words `end` falls between, whether this
    /// item's children have a boundary at its start: see [`Seams`].
    pub seam: u64,
    /// (previous item, last child) alternatives.
    pub families: Vec<(Option<usize>, ChildRef)>,
}

/// A completed Constituent with its alternative Derivations.
#[derive(Debug)]
pub(crate) struct Node {
    pub category: usize,
    pub start: usize,
    pub end: usize,
    pub summary: Summary,
    /// (Production, completed item) alternatives.
    pub families: Vec<(usize, usize)>,
}

/// Every Derivation of an input, packed by (Category, span, summary).
#[derive(Debug, Default)]
pub struct PackedForest {
    pub(crate) items: Vec<Item>,
    pub(crate) nodes: Vec<Node>,
    pub(crate) leaves: Vec<Leaf>,
    pub(crate) roots: Vec<usize>,
}

impl PackedForest {
    /// The completed Constituents that span the whole input.
    #[must_use]
    pub fn roots(&self) -> &[usize] {
        &self.roots
    }

    /// The Category index of a completed Constituent.
    #[must_use]
    pub fn node_category(&self, node: usize) -> usize {
        self.nodes[node].category
    }

    /// The extent of a completed Constituent, in chars.
    #[must_use]
    pub fn node_span(&self, node: usize) -> Range<usize> {
        self.nodes[node].start..self.nodes[node].end
    }

    /// The completed items that are a Constituent's alternative Derivations.
    pub fn node_items(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        self.nodes[node].families.iter().map(|&(_, item)| item)
    }

    /// The (previous item, last child) alternatives of a partial Production.
    pub fn item_families(
        &self,
        item: usize,
    ) -> impl Iterator<Item = (Option<usize>, ChildRef)> + '_ {
        self.items[item].families.iter().copied()
    }

    /// The extent of a lexical alternative, in chars.
    #[must_use]
    pub fn leaf_span(&self, leaf: usize) -> Range<usize> {
        self.leaves[leaf].start..self.leaves[leaf].end
    }

    #[must_use]
    pub fn metrics(&self) -> Metrics {
        Metrics {
            chart_items: self.items.len(),
            completed_nodes: self.nodes.len(),
            families: self.nodes.iter().map(|n| n.families.len()).sum(),
            leaves: self.leaves.len(),
        }
    }
}

/// The result of parsing one input.
#[derive(Debug)]
pub struct Parse<'g> {
    pub(crate) grammar: &'g CompiledGrammar,
    /// The Category parsed for.
    pub(crate) root: usize,
    /// The analyzed text, as chars, in logical order: a period or comma
    /// written inside a closing quotation mark follows it ([`quotations`]).
    pub(crate) tokens: Vec<char>,
    /// The analyzed text as written.
    pub(crate) source: String,
    pub forest: PackedForest,
    /// Words no declared lexeme spells, as char ranges.
    pub vocabulary_gaps: Vec<Range<usize>>,
}

impl<'g> Parse<'g> {
    /// The analyzed text, as chars, in logical order (a period or comma
    /// written inside a closing quotation mark follows it).
    #[must_use]
    pub fn tokens(&self) -> &[char] {
        &self.tokens
    }

    /// Parses another analyzed text as the Category this Parse was made for.
    /// Unlike [`CompiledGrammar::parse_category`] with [`Parse::category`],
    /// this keeps a Category that Readings report under another Category's
    /// name.
    #[must_use]
    pub fn reparse(&self, analyzed: &AnalyzedText) -> Parse<'g> {
        self.grammar.run(analyzed, self.root)
    }
}

/// The quotation mark a `Quoted` child is compiled with
/// ([`crate::grammar::SymbolDecl::Quoted`]): it stands for whichever mark its
/// nesting depth takes ([`quotation_mark`]).
pub(crate) const QUOTATION_MARK: char = '"';

/// Whether a char is a quotation mark, double or single.
pub(crate) fn is_quotation_mark(c: char) -> bool {
    matches!(c, '"' | '\'')
}

/// The quotation mark at a nesting depth: double at even depth, single at odd
/// ("If quoted rules text itself quotes an ability, use straight single
/// quotation marks for the inner quotation"; `docs/oracle-style-guide.md`
/// §3, Quotation marks and apostrophes).
pub(crate) fn quotation_mark(depth: usize) -> char {
    if depth.is_multiple_of(2) { '"' } else { '\'' }
}

/// A char's part in quotation: a mark that opens or closes one, or a period
/// or comma written after a closing mark, where Oracle never writes one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Quote {
    Open,
    Close,
    Outside,
}

/// The quotation marks of a text, paired, and the text in logical order.
///
/// A quotation mark that no word of two or more chars covers (an apostrophe
/// inside *can't*) opens a quotation where it is the mark of its
/// depth, follows a space, a line start or an opening mark, and precedes no
/// space; it closes the innermost open quotation of the same mark where it
/// follows no space and precedes no letter or digit. A mark left unpaired is
/// no quotation mark.
///
/// Oracle writes a period or comma that follows a quotation inside its
/// closing mark (`docs/oracle-style-guide.md` §3, Periods; Quotation marks
/// and apostrophes): *has "… a card."*, *"A," "B," and "C."*. In logical
/// order it follows the closing marks, where the Production that punctuates
/// the quotation finds it, and the quotation's own final period is not
/// written at all: a `.` matches no char just before a closing mark, and no
/// written period or comma does. One written after a closing mark is
/// `Outside`, and matches nothing.
pub(crate) fn quotations(tokens: &[char], covered: &[bool]) -> (Vec<char>, Vec<Option<Quote>>) {
    let n = tokens.len();
    let mut marks = vec![None; n];
    let mut open: Vec<usize> = Vec::new();
    for at in 0..n {
        let c = tokens[at];
        if covered[at] || !is_quotation_mark(c) {
            continue;
        }
        let before = at.checked_sub(1).map(|b| tokens[b]);
        let after = tokens.get(at + 1).copied();
        if open.last().is_some_and(|&o| tokens[o] == c)
            && before.is_some_and(|b| !b.is_whitespace())
            && after.is_none_or(|a| !a.is_alphanumeric())
        {
            let o = open.pop().expect("an open quotation");
            marks[o] = Some(Quote::Open);
            marks[at] = Some(Quote::Close);
        } else if c == quotation_mark(open.len())
            && before.is_none_or(|b| b.is_whitespace() || open.last() == Some(&(at - 1)))
            && after.is_some_and(|a| !a.is_whitespace())
        {
            open.push(at);
        }
    }
    let mut logical = tokens.to_vec();
    let mut at = 0;
    while at < n {
        if marks[at] != Some(Quote::Close) {
            at += 1;
            continue;
        }
        let run = at;
        while at < n && marks[at] == Some(Quote::Close) {
            at += 1;
        }
        if at < n && matches!(logical[at], '.' | ',') && !covered[at] {
            marks[at] = Some(Quote::Outside);
        }
        if run > 0 && matches!(logical[run - 1], '.' | ',') && !covered[run - 1] {
            logical[run - 1..at].rotate_left(1);
            marks[run - 1..at].rotate_left(1);
        }
    }
    (logical, marks)
}

/// The spans of the multiword nouns of one input whose words also reach the
/// chart as leaves of their own.
///
/// The separate words stand apart only where they straddle a constituent
/// boundary: in the smallest Constituent spanning the term, the child holding
/// its first word begins before the term and the child holding its last word
/// ends after it (*the [first time] [counters have been put ...]*). Where
/// either child begins or ends with the term, its words rebuild the term's
/// own analysis, alone (*[first] [strike]*) or with a dependent of one part
/// (*mana [cost for ...]*, *[red or haste] counter*), beside the term itself.
///
/// The rule is checked where a Production that began at or before the term
/// takes the child that reaches the term's end from inside it: an item
/// records, for each term whose words its end falls between, whether one of
/// its children begins where that term begins.
struct Seams {
    spans: Vec<Range<usize>>,
    /// For each position, the spans strictly containing it.
    inside: Vec<Vec<usize>>,
}

impl Seams {
    fn new(mut spans: Vec<Range<usize>>, n: usize) -> Self {
        spans.sort_by_key(|span| (span.start, span.end));
        spans.dedup();
        let mut inside = vec![Vec::new(); n + 1];
        for (index, span) in spans.iter().enumerate() {
            for at in &mut inside[span.start + 1..span.end] {
                at.push(index);
            }
        }
        Self { spans, inside }
    }

    /// Whether an item ending at `end` has a boundary at the start of span
    /// `span`, given its seam bits.
    fn bit(&self, end: usize, seam: u64, span: usize) -> bool {
        self.inside[end]
            .iter()
            .position(|&s| s == span)
            .is_some_and(|bit| seam >> bit & 1 == 1)
    }

    /// The seam bits of an item whose first word is at `first` and that ends
    /// at `end`, after a child whose first word is at `child`, following
    /// `previous` (its end and seam bits); or `None` when that child leaves
    /// a term's words not straddling a constituent boundary.
    fn after(
        &self,
        first: usize,
        previous: Option<(usize, u64)>,
        child: usize,
        end: usize,
    ) -> Option<u64> {
        if self.spans.is_empty() {
            return Some(0);
        }
        let had_start = |span: usize| {
            let start = self.spans[span].start;
            child == start
                || (child > start && previous.is_some_and(|(at, seam)| self.bit(at, seam, span)))
        };
        let rebuilt = self.spans.iter().enumerate().any(|(index, span)| {
            first <= span.start
                && span.start < child
                && child < span.end
                && span.end <= end
                && (span.end == end || had_start(index))
        });
        if rebuilt {
            return None;
        }
        Some(
            self.inside[end]
                .iter()
                .enumerate()
                .filter(|&(_, &span)| had_start(span))
                .fold(0, |seam, (bit, _)| seam | 1 << bit),
        )
    }
}

impl CompiledGrammar {
    /// Parses `text` as the grammar's root Category.
    #[must_use]
    pub fn parse<'g>(&'g self, lexicon: &Lexicon, text: &str) -> Parse<'g> {
        self.parse_analyzed(&lexicon.analyze(text))
    }

    /// Parses an already analyzed text, such as one analyzed with a card's
    /// names in scope, as the grammar's root Category.
    #[must_use]
    pub fn parse_analyzed<'g>(&'g self, analyzed: &AnalyzedText) -> Parse<'g> {
        self.run(analyzed, self.root)
    }

    /// Parses an analyzed text as the named Category, or `None` when the
    /// grammar has no such Category.
    #[must_use]
    pub fn parse_category<'g>(
        &'g self,
        analyzed: &AnalyzedText,
        category: &str,
    ) -> Option<Parse<'g>> {
        let root = self.category_id(category)?;
        Some(self.run(analyzed, root))
    }

    /// The spans of the multiword nouns of `analyzed`, each with whether its
    /// words may stand apart where they straddle a constituent boundary:
    /// those of a term declared outside the core lexicon may
    /// (`docs/decisions/english-lexical-analysis.md` §2). A noun whose first
    /// word is also a preposition is none: its words form a Preposition
    /// Phrase, never the noun's own analysis (*cards you own [outside the
    /// game]* beside the noun *outside the game*).
    pub(crate) fn atomic_spans(&self, analyzed: &AnalyzedText) -> Vec<(Range<usize>, bool)> {
        let prepositional = |start: usize| {
            analyzed.matches.iter().any(|p| {
                p.start == start
                    && matches!(&p.reading, LexicalReading::Word(w)
                    if self.lexemes.get(&w.lexeme).is_some_and(|f| {
                        f.category == deckmaste_lexical::Category::Preposition
                    }))
            })
        };
        analyzed
            .matches
            .iter()
            .filter_map(|m| {
                let LexicalReading::Word(word) = &m.reading else { return None };
                let facts = self.lexemes.get(&word.lexeme)?;
                (m.start == 0 || !analyzed.tokens[m.start - 1].is_alphanumeric()).then_some(())?;
                (facts.category == deckmaste_lexical::Category::Noun
                    && analyzed.tokens[m.start..m.end].contains(&' ')
                    && !prepositional(m.start))
                .then_some((
                    m.start..m.end,
                    facts.source != deckmaste_lexical::SourceKind::Core,
                ))
            })
            .collect()
    }

    #[expect(clippy::too_many_lines, reason = "one Earley loop reads best whole")]
    pub(crate) fn run<'g>(&'g self, analyzed: &AnalyzedText, root: usize) -> Parse<'g> {
        let n = analyzed.tokens.len();
        let mut covered = vec![false; n];
        // A word spelled by a quotation mark alone (the genitive apostrophe
        // of *owners'*) leaves it a candidate mark, and so does a Flavor
        // Word's label, which spans any text before an em dash.
        for found in analyzed
            .matches
            .iter()
            .filter(|m| matches!(m.reading, LexicalReading::Word(_)) && m.end - m.start > 1)
        {
            covered[found.start..found.end].fill(true);
        }
        let (logical, marks) = quotations(&analyzed.tokens, &covered);
        let tokens = &logical;
        let skip = |mut p: usize| {
            // A structural line break consumes its own boundary. Indentation
            // is not an interword space and has no atom to realize it.
            if p > 0 && matches!(tokens[p - 1], '\n' | '\r') {
                return p;
            }
            while p < n && tokens[p].is_whitespace() && tokens[p] != '\n' && tokens[p] != '\r' {
                p += 1;
            }
            p
        };
        let mut forest = PackedForest::default();
        let mut by_start: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let atomic = self.atomic_spans(analyzed);
        for found in &analyzed.matches {
            // Inside a multiword noun, only an ordinary word of a term whose
            // words may stand apart reaches the chart.
            let word = matches!(found.reading, LexicalReading::Word(_));
            if atomic.iter().any(|(span, splits)| {
                ((span.start < found.start && found.start < span.end)
                    || (span.start < found.end && found.end < span.end))
                    && !(*splits && word)
            }) {
                continue;
            }
            if let Some(facts) = self.leaf_facts(&found.reading) {
                by_start[found.start].push(forest.leaves.len());
                forest.leaves.push(Leaf {
                    start: found.start,
                    end: found.end,
                    reading: found.reading.clone(),
                    facts,
                    scoped: self.scoped(&found.reading, &analyzed.scope),
                });
            }
        }
        // A word whose spelling ends in terminal punctuation (*Blood for the
        // Blood God!*, *Agents of S.H.I.E.L.D.*) ends a sentence with it: the
        // sentence's period is not written after it, as it is not after a
        // quotation's own (CGEL, Ch. 20 §8.4, p. 1764, and §6, p. 1757, n. 21).
        let mut terminal = vec![false; n + 1];
        for leaf in &forest.leaves {
            if leaf.end > leaf.start + 1 && matches!(tokens[leaf.end - 1], '.' | '!' | '?') {
                terminal[leaf.end] = true;
            }
        }
        let seams = Seams::new(
            atomic
                .into_iter()
                .filter_map(|(span, splits)| splits.then_some(span))
                .collect(),
            n,
        );
        let mut sets: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        let mut item_index: Vec<ChartMap<ItemKey>> = (0..=n).map(|_| ChartMap::default()).collect();
        let mut node_index: ChartMap<(usize, usize, usize, Summary)> = ChartMap::default();
        let categories = self.categories.len();
        let mut waiting: Vec<Vec<usize>> = vec![Vec::new(); (n + 1) * categories];
        let mut predicted_at = vec![usize::MAX; categories];

        let mut add = |forest: &mut PackedForest,
                       sets: &mut Vec<Vec<usize>>,
                       production: usize,
                       dot: usize,
                       start: usize,
                       end: usize,
                       state: State,
                       seam: u64,
                       family: Option<(Option<usize>, ChildRef)>| {
            let key = (production, dot, start, state, seam);
            let item_index = &mut item_index[end];
            let index = if let Some(&i) = item_index.get(&key) {
                i
            } else {
                let i = forest.items.len();
                forest.items.push(Item {
                    production,
                    dot,
                    start,
                    end,
                    state: key.3,
                    seam,
                    families: Vec::new(),
                });
                item_index.insert(key, i);
                sets[end].push(i);
                i
            };
            if let Some(family) = family {
                forest.items[index].families.push(family);
            }
        };

        let mut active = self.prediction.signature();
        for position in 0..=n {
            if position != 0 && sets[position].is_empty() {
                continue;
            }
            active.fill(0);
            let at = skip(position);
            for &leaf in &by_start[at] {
                self.prediction
                    .add_leaf(self, &forest.leaves[leaf].facts, &mut active);
            }
            self.prediction.add_punctuation(
                tokens.get(position).copied(),
                if marks.get(at).is_some_and(Option::is_some) {
                    Some(QUOTATION_MARK)
                } else {
                    tokens.get(at).copied()
                },
                &mut active,
            );
            if position == 0 {
                predicted_at[root] = position;
                for &p in &self.by_lhs[root] {
                    if self.prediction.production(p, &active) {
                        add(
                            &mut forest,
                            &mut sets,
                            p,
                            0,
                            0,
                            0,
                            self.productions[p].start(),
                            0,
                            None,
                        );
                    }
                }
            }
            let mut cursor = 0;
            while cursor < sets[position].len() {
                let id = sets[position][cursor];
                cursor += 1;
                let (production_id, dot, start) = {
                    let item = &forest.items[id];
                    (item.production, item.dot, item.start)
                };
                let production = &self.productions[production_id];
                let previous = (dot > 0).then_some(id);
                if dot == production.rhs.len() {
                    let Some(summary) = production.finish(self, &forest.items[id].state) else {
                        continue;
                    };
                    let key = (production.lhs, start, position, summary);
                    if let Some(&node) = node_index.get(&key) {
                        forest.nodes[node].families.push((production_id, id));
                        continue;
                    }
                    let node = forest.nodes.len();
                    forest.nodes.push(Node {
                        category: production.lhs,
                        start,
                        end: position,
                        summary: key.3,
                        families: vec![(production_id, id)],
                    });
                    node_index.insert(key, node);
                    for &parent in &waiting[start * categories + production.lhs] {
                        let item = &forest.items[parent];
                        let parent_production = &self.productions[item.production];
                        let view = ChildView::Node(&forest.nodes[node].summary);
                        if !parent_production.spacing_matches(
                            &item.state,
                            item.dot,
                            &view,
                            &tokens[item.end..skip(item.end)],
                        ) {
                            continue;
                        }
                        if let Some(state) =
                            parent_production.accept(self, &item.state, item.dot, &view)
                        {
                            let (p, d, s) = (item.production, item.dot, item.start);
                            let prev = (d > 0).then_some(parent);
                            let Some(seam) = seams.after(
                                skip(s),
                                prev.map(|_| (item.end, item.seam)),
                                skip(start),
                                position,
                            ) else {
                                continue;
                            };
                            add(
                                &mut forest,
                                &mut sets,
                                p,
                                d + 1,
                                s,
                                position,
                                state,
                                seam,
                                Some((prev, ChildRef::Node(node))),
                            );
                        }
                    }
                    continue;
                }
                match &production.rhs[dot] {
                    Symbol::Cat { category, .. } => {
                        waiting[position * categories + category].push(id);
                        if predicted_at[*category] == position {
                            continue;
                        }
                        predicted_at[*category] = position;
                        if !self.prediction.category(*category, &active) {
                            continue;
                        }
                        for &p in &self.by_lhs[*category] {
                            if self.prediction.production(p, &active) {
                                add(
                                    &mut forest,
                                    &mut sets,
                                    p,
                                    0,
                                    position,
                                    position,
                                    self.productions[p].start(),
                                    0,
                                    None,
                                );
                            }
                        }
                    }
                    Symbol::Leaf(leaf_test) => {
                        let at = skip(position);
                        for &leaf in &by_start[at] {
                            let facts = &forest.leaves[leaf].facts;
                            if !self.leaf_matches(leaf_test, facts) {
                                continue;
                            }
                            let view = ChildView::Leaf(facts);
                            if !production.spacing_matches(
                                &forest.items[id].state,
                                dot,
                                &view,
                                &tokens[position..at],
                            ) {
                                continue;
                            }
                            if let Some(state) =
                                production.accept(self, &forest.items[id].state, dot, &view)
                            {
                                let end = forest.leaves[leaf].end;
                                let item = &forest.items[id];
                                let Some(seam) = seams.after(
                                    skip(start),
                                    previous.map(|_| (item.end, item.seam)),
                                    at,
                                    end,
                                ) else {
                                    continue;
                                };
                                add(
                                    &mut forest,
                                    &mut sets,
                                    production_id,
                                    dot + 1,
                                    start,
                                    end,
                                    state,
                                    seam,
                                    Some((previous, ChildRef::Leaf(leaf))),
                                );
                            }
                        }
                    }
                    Symbol::Punct(c, binding) => {
                        let mut at = if c.is_whitespace() { position } else { skip(position) };
                        let mut end = at + 1;
                        let found = if *c == QUOTATION_MARK {
                            let role =
                                if glues_after(*binding) { Quote::Open } else { Quote::Close };
                            marks.get(at) == Some(&Some(role))
                        } else if at < n && tokens[at] == *c {
                            // Written punctuation is no quotation's, and
                            // never precedes a closing mark.
                            marks[at].is_none()
                                && !(matches!(c, '.' | ',')
                                    && marks.get(at + 1) == Some(&Some(Quote::Close)))
                                && !(*c == '.' && at == position && terminal[at])
                        } else if *c == '.'
                            && (marks.get(position) == Some(&Some(Quote::Close))
                                || terminal[position])
                            && !forest.items[id].state.unwritten_period
                        {
                            // A quotation's own final period is not written,
                            // nor is one after a word ending in terminal
                            // punctuation, and it is the one period there.
                            (at, end) = (position, position);
                            true
                        } else {
                            false
                        };
                        if found
                            && production.spacing_matches(
                                &forest.items[id].state,
                                dot,
                                &ChildView::Punct,
                                &tokens[position..at],
                            )
                            && let Some(mut state) = production.accept(
                                self,
                                &forest.items[id].state,
                                dot,
                                &ChildView::Punct,
                            )
                            && let Some(seam) = seams.after(
                                skip(start),
                                previous.map(|_| (forest.items[id].end, forest.items[id].seam)),
                                at,
                                end,
                            )
                        {
                            state.unwritten_period = at == end;
                            add(
                                &mut forest,
                                &mut sets,
                                production_id,
                                dot + 1,
                                start,
                                end,
                                state,
                                seam,
                                Some((previous, ChildRef::Punct)),
                            );
                        }
                    }
                }
            }
        }
        forest.roots = (0..forest.nodes.len())
            .filter(|&i| {
                let node = &forest.nodes[i];
                !node.families.is_empty()
                    && node.category == root
                    && node.start == 0
                    && skip(node.end) == n
                    && node.summary.gap == crate::Gap::None
            })
            .collect();
        Parse {
            grammar: self,
            root,
            source: analyzed.tokens.iter().collect(),
            tokens: logical,
            forest,
            vocabulary_gaps: analyzed.unknown_words(),
        }
    }
}
