use deckmaste_english_v3::Leaf;
use deckmaste_english_v3::Materializer;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Error;
use deckmaste_english_v3::grammar::FeatureValue;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::Summary;
use deckmaste_english_v3::grammar::Value;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Lexicon;
use serde::Serialize;

use crate::raw_corpus::digest;

/// Fingerprints include the complete grammatical value, not just its variant.
/// Debug encoding is diagnostic and versioned by the report's tree identity.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize)]
pub(super) struct NodeIdentity {
    pub construction: &'static str,
    pub sha256: String,
}

impl NodeIdentity {
    fn new(reading: &Reading) -> Self {
        Self {
            construction: reading.construction(),
            sha256: digest(format!("{reading:?}").as_bytes()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub(super) struct TracedValue {
    pub value: Value,
    pub nodes: Vec<NodeIdentity>,
    pub words: Vec<Word>,
}

/// Record what the forest materializes independently of generated visitors.
pub(super) struct Tracing<'a>(pub &'a Grammar);

impl Materializer<Summary, Vec<Option<FeatureValue>>> for Tracing<'_> {
    type Reading = TracedValue;
    type Error = Error;

    fn leaf(&mut self, leaf: &Leaf, summary: Option<&Summary>) -> Result<TracedValue, Error> {
        let value = self.0.leaf(leaf, summary)?;
        let words = match &value {
            Value::Word(word) => vec![word.clone()],
            _ => Vec::new(),
        };
        Ok(TracedValue {
            value,
            nodes: Vec::new(),
            words,
        })
    }

    fn build(
        &mut self,
        production: usize,
        summary: &Summary,
        state: &Vec<Option<FeatureValue>>,
        children: Vec<TracedValue>,
    ) -> Result<TracedValue, Error> {
        let mut nodes = Vec::new();
        let mut words = Vec::new();
        let mut values = Vec::with_capacity(children.len());
        for child in children {
            nodes.extend(child.nodes);
            words.extend(child.words);
            values.push(child.value);
        }
        let value = self.0.build(production, summary, state, values)?;
        if let Value::Reading(reading) = &value {
            nodes.insert(0, NodeIdentity::new(reading));
        }
        Ok(TracedValue {
            value,
            nodes,
            words,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub(super) enum Issue {
    Parser(String),
    Materialization(String),
    RootCategory,
    Admission(String),
    Roundtrip { realized: String },
    ConstructionTraversal,
    LexicalTraversal,
    DuplicateReading,
}

pub(super) fn validate<'a>(
    traced: &'a TracedValue,
    raw: &str,
    lexicon: &Lexicon,
    category: Category,
) -> Result<&'a Reading, Issue> {
    let Value::Reading(reading) = &traced.value else {
        return Err(Issue::RootCategory);
    };
    if reading.category() != category {
        return Err(Issue::RootCategory);
    }
    let realized = reading
        .realize(lexicon)
        .map_err(|error| Issue::Admission(error.to_string()))?;
    if realized != raw {
        return Err(Issue::Roundtrip { realized });
    }
    let mut nodes = Vec::new();
    reading
        .visit(&mut |node| nodes.push(NodeIdentity::new(node)))
        .map_err(|error| Issue::Admission(error.to_string()))?;
    if nodes != traced.nodes {
        return Err(Issue::ConstructionTraversal);
    }
    let mut words = Vec::new();
    reading
        .visit_words(&mut |word| words.push(word.clone()))
        .map_err(|error| Issue::Admission(error.to_string()))?;
    if words != traced.words {
        return Err(Issue::LexicalTraversal);
    }
    Ok(reading)
}
