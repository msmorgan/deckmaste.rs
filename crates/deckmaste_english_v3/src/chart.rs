use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use deckmaste_lexical::AnalyzedText;
use deckmaste_lexical::Category;
use deckmaste_lexical::LexicalReading;
use deckmaste_lexical::Lexicon;

use crate::CompletedId;
use crate::Forest;
use crate::Grammar;
use crate::GrammarForest;
use crate::Leaf;
use crate::LexicalFeatures;
use crate::Symbol;
use crate::forest::Child;
use crate::forest::Completed;
use crate::forest::Edge;
use crate::forest::Family;
use crate::forest::Intermediate;
use crate::forest::IntermediateId;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("the supplied token sequence differs from the analyzed text")]
    InvalidTokens,
    #[error("lexical occurrence {0} has invalid positions")]
    InvalidOccurrence(usize),
    #[error("lexical occurrence {0} is not declared in this environment")]
    UndeclaredOccurrence(usize),
}

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Item<S> {
    production: usize,
    dot: usize,
    start: usize,
    end: usize,
    state: S,
}

struct Occurrence {
    end: usize,
    leaf: usize,
}

type CompletedKey<C, S> = (C, usize, usize, S);

struct Chart<'a, G: Grammar> {
    grammar: &'a G,
    input: &'a AnalyzedText,
    forest: Forest<G::Category, G::Summary, G::State>,
    items: Vec<Item<G::State>>,
    item_index: BTreeMap<Item<G::State>, IntermediateId>,
    completed_index: BTreeMap<CompletedKey<G::Category, G::Summary>, CompletedId>,
    waiting: BTreeMap<(usize, G::Category), Vec<IntermediateId>>,
    available: BTreeMap<(usize, G::Category), Vec<CompletedId>>,
    predicted: BTreeSet<(usize, G::Category)>,
    productions: BTreeMap<G::Category, Vec<usize>>,
    lexical: BTreeMap<(usize, Category), Vec<Occurrence>>,
    literals: BTreeMap<(usize, String), usize>,
    agenda: VecDeque<IntermediateId>,
}

/// Recognize all admitted summaries over the supplied lexical occurrences.
/// No tokenization, lexical lookup by expected word, or AST construction occurs
/// in the agenda. A grammatical failure is a forest with no roots.
///
/// # Errors
/// Rejects inconsistent tokens, malformed positions or lexical values absent
/// from the environment.
pub fn parse<G: Grammar>(
    grammar: &G,
    lexicon: &Lexicon,
    input: &AnalyzedText,
    start: &G::Category,
) -> Result<GrammarForest<G>, ParseError> {
    if !input.text().chars().eq(input.tokens.iter().copied()) {
        return Err(ParseError::InvalidTokens);
    }
    let mut chart = Chart {
        grammar,
        input,
        forest: Forest::new(grammar),
        items: Vec::new(),
        item_index: BTreeMap::new(),
        completed_index: BTreeMap::new(),
        waiting: BTreeMap::new(),
        available: BTreeMap::new(),
        predicted: BTreeSet::new(),
        productions: BTreeMap::new(),
        lexical: BTreeMap::new(),
        literals: BTreeMap::new(),
        agenda: VecDeque::new(),
    };
    chart.index_lexical(lexicon)?;
    for (index, production) in grammar.productions().iter().enumerate() {
        chart
            .productions
            .entry(production.category.clone())
            .or_default()
            .push(index);
    }
    chart.predict(0, start);
    while let Some(id) = chart.agenda.pop_front() {
        chart.process(id);
    }
    for (index, node) in chart.forest.completed.iter().enumerate() {
        if &node.category == start
            && node.start == 0
            && node.end == input.tokens.len()
            && grammar.root(start, &node.summary)
        {
            chart.forest.roots.push(CompletedId(index));
        }
    }
    Ok(chart.forest)
}

impl<G: Grammar> Chart<'_, G> {
    fn index_lexical(&mut self, lexicon: &Lexicon) -> Result<(), ParseError> {
        self.forest.metrics.lexical_alternatives = self.input.matches.len();
        for (index, occurrence) in self.input.matches.iter().enumerate() {
            if occurrence.start >= occurrence.end || occurrence.end > self.input.tokens.len() {
                return Err(ParseError::InvalidOccurrence(index));
            }
            let surface = lexicon
                .surface_features(&occurrence.reading)
                .map_err(|_| ParseError::UndeclaredOccurrence(index))?;
            let (category, features, provenance) = match &occurrence.reading {
                LexicalReading::Word(value) => {
                    let lexeme = lexicon
                        .lexemes()
                        .get(&value.lexeme)
                        .ok_or(ParseError::UndeclaredOccurrence(index))?;
                    (
                        lexeme.category,
                        LexicalFeatures::Word {
                            category: lexeme.category,
                            form: value.form,
                            features: &value.features,
                            properties: &lexeme.properties,
                            surface,
                        },
                        Some(lexeme.source.clone()),
                    )
                }
                LexicalReading::Numeral {
                    value, notation, ..
                } => (
                    Category::Numeral,
                    LexicalFeatures::Numeral {
                        value: *value,
                        notation: *notation,
                        surface,
                    },
                    None,
                ),
            };
            // Validate public occurrence input without reanalyzing it. Realize
            // uses the same immutable index and checks declared variants.
            let spelling = lexicon
                .realize(&occurrence.reading)
                .map_err(|_| ParseError::UndeclaredOccurrence(index))?;
            if !spelling
                .chars()
                .eq(self.input.tokens[occurrence.start..occurrence.end]
                    .iter()
                    .copied())
            {
                return Err(ParseError::UndeclaredOccurrence(index));
            }
            for summary in self
                .grammar
                .lexical(features)
                .into_iter()
                .collect::<BTreeSet<_>>()
            {
                let leaf = self.forest.leaves.len();
                self.forest.leaves.push(Leaf::Lexical {
                    occurrence: occurrence.clone(),
                    provenance: provenance.clone(),
                });
                self.forest.leaf_summaries.push(Some(summary));
                self.forest.metrics.lexical_projections += 1;
                self.lexical
                    .entry((occurrence.start, category))
                    .or_default()
                    .push(Occurrence {
                        end: occurrence.end,
                        leaf,
                    });
            }
        }
        Ok(())
    }

    fn insert(&mut self, item: Item<G::State>, edge: Edge) {
        let id = if let Some(id) = self.item_index.get(&item) {
            *id
        } else {
            let id = IntermediateId(self.items.len());
            self.item_index.insert(item.clone(), id);
            self.forest.intermediate.push(Intermediate {
                state: item.state.clone(),
                edges: Vec::new(),
                edge_set: BTreeSet::new(),
            });
            self.items.push(item);
            self.agenda.push_back(id);
            self.forest.metrics.items += 1;
            self.forest.metrics.intermediate_nodes += 1;
            id
        };
        let node = &mut self.forest.intermediate[id.0];
        if node.edge_set.insert(edge) {
            node.edges.push(edge);
            self.forest.metrics.intermediate_edges += 1;
            self.forest.metrics.families += 1;
        }
    }

    fn predict(&mut self, position: usize, category: &G::Category) {
        if !self.predicted.insert((position, category.clone())) {
            return;
        }
        for production in self.productions.get(category).cloned().unwrap_or_default() {
            for state in self.grammar.begin(production) {
                self.insert(
                    Item {
                        production,
                        dot: 0,
                        start: position,
                        end: position,
                        state,
                    },
                    Edge::Empty,
                );
            }
        }
    }

    fn advance(
        &mut self,
        prefix: IntermediateId,
        child: Child,
        end: usize,
        summary: Option<&G::Summary>,
    ) {
        let item = self.items[prefix.0].clone();
        for state in self
            .grammar
            .advance(item.production, item.dot, &item.state, summary)
        {
            self.insert(
                Item {
                    dot: item.dot + 1,
                    end,
                    state,
                    ..item.clone()
                },
                Edge::Append { prefix, child },
            );
        }
    }

    fn join(&mut self, prefix: IntermediateId, child: CompletedId) {
        self.forest.metrics.completion_work += 1;
        let node = &self.forest.completed[child.0];
        let end = node.end;
        let summary = node.summary.clone();
        self.advance(prefix, Child::Completed(child), end, Some(&summary));
    }

    fn process(&mut self, id: IntermediateId) {
        let item = self.items[id.0].clone();
        let production = &self.grammar.productions()[item.production];
        match production.symbols.get(item.dot) {
            None => self.complete(id),
            Some(Symbol::Nonterminal(category)) => {
                let category = category.clone();
                let key = (item.end, category.clone());
                self.waiting.entry(key.clone()).or_default().push(id);
                for child in self.available.get(&key).cloned().unwrap_or_default() {
                    self.join(id, child);
                }
                self.predict(item.end, &category);
            }
            Some(Symbol::Lexical(category)) => {
                // Move the bucket temporarily to avoid cloning its summaries
                // for each waiter. Advancing items does not scan recursively.
                let key = (item.end, *category);
                if let Some(occurrences) = self.lexical.remove(&key) {
                    for occurrence in &occurrences {
                        let summary = self.forest.leaf_summaries[occurrence.leaf].clone();
                        self.forest.metrics.scan_work += 1;
                        self.advance(
                            id,
                            Child::Leaf(occurrence.leaf),
                            occurrence.end,
                            summary.as_ref(),
                        );
                    }
                    self.lexical.insert(key, occurrences);
                }
            }
            Some(Symbol::Literal(text)) => {
                let text = text.clone();
                let length = text.chars().count();
                let end = item.end + length;
                if end <= self.input.tokens.len()
                    && text
                        .chars()
                        .eq(self.input.tokens[item.end..end].iter().copied())
                {
                    let leaf = *self
                        .literals
                        .entry((item.end, text.clone()))
                        .or_insert_with(|| {
                            let leaf = self.forest.leaves.len();
                            self.forest.leaves.push(Leaf::Literal {
                                start: item.end,
                                end,
                                text,
                            });
                            self.forest.leaf_summaries.push(None);
                            leaf
                        });
                    self.forest.metrics.scan_work += 1;
                    self.advance(id, Child::Leaf(leaf), end, None);
                }
            }
        }
    }

    fn complete(&mut self, prefix: IntermediateId) {
        let item = &self.items[prefix.0];
        let Some(summary) = self.grammar.complete(item.production, &item.state) else {
            return;
        };
        let category = self.grammar.productions()[item.production].category.clone();
        let key = (category.clone(), item.start, item.end, summary.clone());
        let family = Family {
            production: item.production,
            prefix,
        };
        let (id, fresh) = if let Some(&id) = self.completed_index.get(&key) {
            (id, false)
        } else {
            let id = CompletedId(self.forest.completed.len());
            self.completed_index.insert(key, id);
            self.forest.completed.push(Completed {
                category: category.clone(),
                start: item.start,
                end: item.end,
                summary,
                families: Vec::new(),
                family_set: BTreeSet::new(),
            });
            self.forest.metrics.completed_nodes += 1;
            (id, true)
        };
        let node = &mut self.forest.completed[id.0];
        if node.family_set.insert(family) {
            node.families.push(family);
            self.forest.metrics.completed_families += 1;
            self.forest.metrics.families += 1;
        }
        if fresh {
            let key = (item.start, category);
            self.available.entry(key.clone()).or_default().push(id);
            for waiter in self.waiting.get(&key).cloned().unwrap_or_default() {
                self.join(waiter, id);
            }
        }
    }
}
