use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use crate::constructions::RULES;

#[derive(Debug, Clone, Copy)]
pub(crate) enum MetricEvent {
    Prediction,
    Completion,
    Materialization,
    MemoMiss,
    CloneHeavy,
}

#[derive(Default)]
struct Counters {
    predictions: AtomicU64,
    completions: AtomicU64,
    materializations: AtomicU64,
    memo_misses: AtomicU64,
    clone_heavy: AtomicU64,
}

static COUNTERS: OnceLock<Vec<Counters>> = OnceLock::new();

fn counters() -> &'static [Counters] {
    COUNTERS.get_or_init(|| (0..=RULES.len()).map(|_| Counters::default()).collect())
}

pub(crate) fn record(rule_index: usize, event: MetricEvent) {
    let counters = &counters()[rule_index];
    let counter = match event {
        MetricEvent::Prediction => &counters.predictions,
        MetricEvent::Completion => &counters.completions,
        MetricEvent::Materialization => &counters.materializations,
        MetricEvent::MemoMiss => &counters.memo_misses,
        MetricEvent::CloneHeavy => &counters.clone_heavy,
    };
    counter.fetch_add(1, Ordering::Relaxed);
}

/// Aggregated parser work attributed to one generated construction owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructionMetrics {
    name: &'static str,
    predictions: u64,
    completions: u64,
    materializations: u64,
    memo_misses: u64,
    clone_heavy: u64,
}

impl ConstructionMetrics {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
    #[must_use]
    pub const fn predictions(&self) -> u64 {
        self.predictions
    }
    #[must_use]
    pub const fn completions(&self) -> u64 {
        self.completions
    }
    #[must_use]
    pub const fn materializations(&self) -> u64 {
        self.materializations
    }
    #[must_use]
    pub const fn memo_misses(&self) -> u64 {
        self.memo_misses
    }
    #[must_use]
    pub const fn clone_heavy(&self) -> u64 {
        self.clone_heavy
    }
}

/// Returns a stable name-sorted snapshot of the feature-gated counters.
#[must_use]
pub fn parser_metrics() -> Vec<ConstructionMetrics> {
    let mut by_name = BTreeMap::<&'static str, ConstructionMetrics>::new();
    for (index, rule) in RULES.iter().enumerate() {
        merge(&mut by_name, rule.id.metric_name(), &counters()[index]);
    }
    merge(&mut by_name, "RootAdapter", &counters()[RULES.len()]);
    by_name.into_values().collect()
}

fn merge(
    by_name: &mut BTreeMap<&'static str, ConstructionMetrics>,
    name: &'static str,
    counters: &Counters,
) {
    let row = by_name.entry(name).or_insert(ConstructionMetrics {
        name,
        predictions: 0,
        completions: 0,
        materializations: 0,
        memo_misses: 0,
        clone_heavy: 0,
    });
    row.predictions += counters.predictions.load(Ordering::Relaxed);
    row.completions += counters.completions.load(Ordering::Relaxed);
    row.materializations += counters.materializations.load(Ordering::Relaxed);
    row.memo_misses += counters.memo_misses.load(Ordering::Relaxed);
    row.clone_heavy += counters.clone_heavy.load(Ordering::Relaxed);
}
