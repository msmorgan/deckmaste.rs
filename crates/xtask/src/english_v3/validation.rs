use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::OnceLock;

use deckmaste_english_v3::Leaf;
use deckmaste_english_v3::Materializer;
use deckmaste_english_v3::grammar::Category;
use deckmaste_english_v3::grammar::Error;
use deckmaste_english_v3::grammar::Grammar;
use deckmaste_english_v3::grammar::Reading;
use deckmaste_english_v3::grammar::RealizationContext;
use deckmaste_english_v3::grammar::State;
use deckmaste_english_v3::grammar::Summary;
use deckmaste_english_v3::grammar::Value;
use deckmaste_english_v3::grammar::Word;
use deckmaste_lexical::Lexicon;
use serde::Deserialize;
use serde::Serialize;

use crate::raw_corpus::digest;

/// Fingerprints include the complete grammatical value, not just its variant.
/// Debug encoding is diagnostic and versioned by the report's tree identity.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize)]
pub(super) struct NodeIdentity {
    pub construction: String,
    pub sha256: String,
}

impl NodeIdentity {
    pub(super) fn new(reading: &Reading) -> Self {
        Self {
            construction: reading.construction().to_owned(),
            sha256: digest(format!("{reading:?}").as_bytes()),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct TracedValue {
    pub value: Arc<Value>,
    pub nodes: Arc<Vec<Arc<Value>>>,
    pub words: Arc<Vec<Word>>,
    id: usize,
    fingerprint: Arc<OnceLock<String>>,
}

impl TracedValue {
    /// SHA-256 over the value identity and both independent traces, so a
    /// Reading whose trace differs is never merged with one that matches.
    pub(super) fn dedup_key(&self) -> [u8; 32] {
        use std::fmt::Write as _;

        use sha2::Digest as _;
        struct HashWriter(sha2::Sha256);
        impl std::fmt::Write for HashWriter {
            fn write_str(&mut self, text: &str) -> std::fmt::Result {
                self.0.update(text.as_bytes());
                Ok(())
            }
        }
        let mut writer = HashWriter(sha2::Sha256::new());
        writer
            .write_str(self.identity())
            .expect("hashing cannot fail");
        // A matching independent trace is exactly reconstructible from the
        // complete root value. Hash a canonical marker only after comparing
        // every value; retain raw Debug for any mismatch, so a malformed trace
        // can never disappear behind a valid one during deduplication.
        let (canonical_nodes, canonical_words) = if let Value::Reading(reading) = &*self.value {
            (
                matches_nodes(reading, &self.nodes).unwrap_or(false),
                matches_words(reading, &self.words).unwrap_or(false),
            )
        } else {
            (false, false)
        };
        writer.write_str("|n=").expect("hashing cannot fail");
        if canonical_nodes {
            writer.write_str("canonical").expect("hashing cannot fail");
        } else {
            write!(writer, "raw{:?}", self.nodes).expect("hashing Debug cannot fail");
        }
        writer.write_str("|w=").expect("hashing cannot fail");
        if canonical_words {
            writer.write_str("canonical").expect("hashing cannot fail");
        } else {
            write!(writer, "raw{:?}", self.words).expect("hashing Debug cannot fail");
        }
        writer.0.finalize().into()
    }

    pub(super) fn identity(&self) -> &str {
        self.fingerprint.get_or_init(|| match &*self.value {
            Value::Reading(reading) => NodeIdentity::new(reading).sha256,
            value => digest(format!("{value:?}").as_bytes()),
        })
    }
}

// Cache coordinates are not Reading identity. Include the independent traces
// so a different materialization trace cannot disappear during deduplication.
impl Ord for TracedValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // The digest orders the set cheaply; equal digests still compare full
        // values and traces. A collision can never erase a distinct Reading.
        let ordering = self.identity().cmp(other.identity());
        if !ordering.is_eq() {
            return ordering;
        }
        (&self.value, &self.nodes, &self.words).cmp(&(&other.value, &other.nodes, &other.words))
    }
}
impl PartialOrd for TracedValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for TracedValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for TracedValue {}

type BuildKey = (usize, Vec<usize>);
const MEMO_CAP: usize = 100_000;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub(super) struct MaterializationMetrics {
    pub leaf_requests: usize,
    pub unique_leaves: usize,
    pub build_requests: usize,
    pub unique_builds: usize,
}

/// Record forest materializations independently of generated visitors. Reuse
/// a pure build only for the identical production and ordered child tuple;
/// neither chart alternatives nor complete Reading requests are pruned.
pub(super) struct Tracing<'a> {
    grammar: &'a Grammar,
    leaves: BTreeMap<Value, TracedValue>,
    builds: BTreeMap<BuildKey, TracedValue>,
    next_id: usize,
    metrics: MaterializationMetrics,
    memo_cap: usize,
}

impl<'a> Tracing<'a> {
    pub(super) fn new(grammar: &'a Grammar) -> Self {
        Self {
            grammar,
            leaves: BTreeMap::new(),
            builds: BTreeMap::new(),
            next_id: 0,
            metrics: MaterializationMetrics::default(),
            memo_cap: MEMO_CAP,
        }
    }

    #[cfg(test)]
    pub(super) fn with_memo_cap(mut self, cap: usize) -> Self {
        self.memo_cap = cap;
        self
    }

    pub(super) const fn metrics(&self) -> MaterializationMetrics {
        self.metrics
    }

    fn traced(
        &mut self,
        value: Value,
        mut nodes: Vec<Arc<Value>>,
        words: Vec<Word>,
    ) -> TracedValue {
        let value = Arc::new(value);
        if matches!(&*value, Value::Reading(_)) {
            nodes.insert(0, Arc::clone(&value));
        }
        let result = TracedValue {
            value,
            nodes: Arc::new(nodes),
            words: Arc::new(words),
            id: self.next_id,
            fingerprint: Arc::new(OnceLock::new()),
        };
        self.next_id += 1;
        result
    }
}

impl Materializer<Summary, State> for Tracing<'_> {
    type Reading = TracedValue;
    type Error = Error;

    fn leaf(&mut self, leaf: &Leaf, summary: Option<&Summary>) -> Result<TracedValue, Error> {
        self.metrics.leaf_requests += 1;
        let value = self.grammar.leaf(leaf, summary)?;
        if let Some(traced) = self.leaves.get(&value) {
            return Ok(traced.clone());
        }
        let words = match &value {
            Value::Word(word) => vec![word.clone()],
            _ => Vec::new(),
        };
        let traced = self.traced(value.clone(), Vec::new(), words);
        self.metrics.unique_leaves += 1;
        if self.leaves.len() >= self.memo_cap {
            self.leaves.clear();
        }
        self.leaves.insert(value, traced.clone());
        Ok(traced)
    }

    fn build(
        &mut self,
        production: usize,
        _: &Summary,
        _: &State,
        children: Vec<TracedValue>,
    ) -> Result<TracedValue, Error> {
        self.metrics.build_requests += 1;
        let key = (production, children.iter().map(|child| child.id).collect());
        if let Some(traced) = self.builds.get(&key) {
            return Ok(traced.clone());
        }
        if self.builds.len() >= self.memo_cap {
            // Bounded memo: forgetting entries only costs rebuilds; ids keep
            // increasing, so no stale key can alias a new value.
            self.builds.clear();
        }
        let mut nodes = Vec::new();
        let mut words = Vec::new();
        let mut values = Vec::with_capacity(children.len());
        for child in children {
            nodes.extend(child.nodes.iter().cloned());
            words.extend(child.words.iter().cloned());
            values.push((*child.value).clone());
        }
        let value = self.grammar.materialize(production, values)?;
        let traced = self.traced(value, nodes, words);
        self.metrics.unique_builds += 1;
        self.builds.insert(key, traced.clone());
        Ok(traced)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
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
    AmbiguousSegmentation,
}

pub(super) fn validate<'a>(
    traced: &'a TracedValue,
    raw: &str,
    lexicon: &Lexicon,
    category: Category,
    grammar: &Grammar,
) -> Result<&'a Reading, Issue> {
    validate_using(traced, raw, category, |reading| {
        grammar.realize(reading, lexicon)
    })
}

pub(super) fn validate_in_context<'a>(
    traced: &'a TracedValue,
    raw: &str,
    category: Category,
    context: &RealizationContext<'_>,
) -> Result<&'a Reading, Issue> {
    validate_using(traced, raw, category, |reading| context.realize(reading))
}

fn validate_using<'a>(
    traced: &'a TracedValue,
    raw: &str,
    category: Category,
    realize: impl FnOnce(&Reading) -> Result<String, Error>,
) -> Result<&'a Reading, Issue> {
    let Value::Reading(reading) = &*traced.value else {
        return Err(Issue::RootCategory);
    };
    if reading.category() != category {
        return Err(Issue::RootCategory);
    }
    let realized = realize(reading).map_err(|error| Issue::Admission(error.to_string()))?;
    if realized != raw {
        return Err(Issue::Roundtrip { realized });
    }
    if !matches_nodes(reading, &traced.nodes)
        .map_err(|error| Issue::Admission(error.to_string()))?
    {
        return Err(Issue::ConstructionTraversal);
    }
    if !matches_words(reading, &traced.words)
        .map_err(|error| Issue::Admission(error.to_string()))?
    {
        return Err(Issue::LexicalTraversal);
    }
    Ok(reading)
}

fn matches_nodes(reading: &Reading, expected: &[Arc<Value>]) -> Result<bool, Error> {
    let mut nodes = expected.iter();
    let mut same = true;
    reading.visit(&mut |node| {
        same &= nodes
            .next()
            .is_some_and(|expected| matches!(&**expected, Value::Reading(value) if node == value));
    })?;
    Ok(same && nodes.next().is_none())
}

fn matches_words(reading: &Reading, expected: &[Word]) -> Result<bool, Error> {
    let mut words = expected.iter();
    let mut same = true;
    reading.visit_words(&mut |word| same &= words.next() == Some(word))?;
    Ok(same && words.next().is_none())
}
