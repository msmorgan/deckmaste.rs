//! Left-corner terminal sets, compiled once and tested once per chart position.

use std::collections::BTreeMap;

use deckmaste_lexical::Category;
use deckmaste_lexical::LexemeId;

use crate::compile::CompiledGrammar;
use crate::compile::LexemeFacts;
use crate::step::LeafFacts;
use crate::step::MAX_FEATURES;
use crate::step::Symbol;
use crate::step::Test;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Terminal {
    Leaf(Test),
    Punct(char),
}

#[derive(Debug, Default)]
pub(crate) struct Prediction {
    terminals: Vec<Terminal>,
    lexemes: BTreeMap<LexemeId, Vec<u64>>,
    categories: Vec<Vec<u64>>,
    productions: Vec<Vec<u64>>,
}

impl Prediction {
    pub(crate) fn compile(grammar: &CompiledGrammar) -> Self {
        let mut terminals = Vec::new();
        let first: Vec<_> = grammar
            .productions
            .iter()
            .map(
                |production| match production.rhs.first().expect("nonempty Production") {
                    Symbol::Cat { category, .. } => Err(*category),
                    symbol => {
                        let terminal = match symbol {
                            Symbol::Leaf(test) => Terminal::Leaf(test.clone()),
                            Symbol::Punct(c, _) => Terminal::Punct(*c),
                            Symbol::Cat { .. } => unreachable!(),
                        };
                        let id = terminals
                            .iter()
                            .position(|t| *t == terminal)
                            .unwrap_or_else(|| {
                                terminals.push(terminal);
                                terminals.len() - 1
                            });
                        Ok(id)
                    }
                },
            )
            .collect();
        let words = terminals.len().div_ceil(64);
        let mut categories = vec![vec![0; words]; grammar.categories.len()];
        // Empty Productions are rejected by the compiler. Thus only the first
        // symbol contributes; a fixed point also handles left-recursive cycles.
        loop {
            let mut changed = false;
            for (production, first) in grammar.productions.iter().zip(&first) {
                match first {
                    Ok(terminal) => {
                        let word = &mut categories[production.lhs][terminal / 64];
                        let old = *word;
                        *word |= 1 << (terminal % 64);
                        changed |= *word != old;
                    }
                    Err(category) => {
                        if production.lhs == *category {
                            continue;
                        }
                        let [target, source] = categories
                            .get_disjoint_mut([production.lhs, *category])
                            .expect("distinct category indexes");
                        for (word, inherited) in target.iter_mut().zip(source.iter()) {
                            let before = *word;
                            *word |= inherited;
                            changed |= *word != before;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        let productions = first
            .into_iter()
            .map(|first| match first {
                Err(category) => categories[category].clone(),
                Ok(terminal) => {
                    let mut mask = vec![0; words];
                    mask[terminal / 64] |= 1 << (terminal % 64);
                    mask
                }
            })
            .collect();
        let mut prediction = Self {
            terminals,
            categories,
            productions,
            lexemes: BTreeMap::new(),
        };
        for (&id, facts) in &grammar.lexemes {
            let leaf = LeafFacts {
                binding: facts.binding,
                integer: None,
                notation: None,
                lexeme: Some(id),
                category: facts.category,
                features: [None; MAX_FEATURES],
            };
            let mut mask = vec![0; words];
            prediction.add_tests(grammar, &leaf, Some(facts), &mut mask);
            prediction.lexemes.insert(id, mask);
        }
        prediction
    }

    pub(crate) fn signature(&self) -> Vec<u64> {
        vec![0; self.terminals.len().div_ceil(64)]
    }

    fn add_tests(
        &self,
        grammar: &CompiledGrammar,
        leaf: &LeafFacts,
        facts: Option<&LexemeFacts>,
        active: &mut [u64],
    ) {
        for (id, terminal) in self.terminals.iter().enumerate() {
            if let Terminal::Leaf(test) = terminal
                && grammar.leaf_matches_with_facts(test, leaf, facts)
            {
                active[id / 64] |= 1 << (id % 64);
            }
        }
    }

    pub(crate) fn add_leaf(&self, grammar: &CompiledGrammar, leaf: &LeafFacts, active: &mut [u64]) {
        if let Some(id) = leaf.lexeme {
            for (word, mask) in active.iter_mut().zip(&self.lexemes[&id]) {
                *word |= mask;
            }
        } else {
            debug_assert!(matches!(
                leaf.category,
                Category::Numeral | Category::FlavorWord
            ));
            self.add_tests(grammar, leaf, None, active);
        }
    }

    pub(crate) fn add_punctuation(
        &self,
        raw: Option<char>,
        skipped: Option<char>,
        active: &mut [u64],
    ) {
        for (id, terminal) in self.terminals.iter().enumerate() {
            if let Terminal::Punct(c) = terminal
                && Some(*c) == if c.is_whitespace() { raw } else { skipped }
            {
                active[id / 64] |= 1 << (id % 64);
            }
        }
    }

    pub(crate) fn category(&self, category: usize, active: &[u64]) -> bool {
        intersects(&self.categories[category], active)
    }

    pub(crate) fn production(&self, production: usize, active: &[u64]) -> bool {
        intersects(&self.productions[production], active)
    }
}

fn intersects(mask: &[u64], active: &[u64]) -> bool {
    mask.iter().zip(active).any(|(a, b)| a & b != 0)
}
