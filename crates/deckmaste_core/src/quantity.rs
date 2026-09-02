use crate::Count;

/// A cardinality range over a scalar [`Count`]: how many objects an effect
/// operates on, never a continuous magnitude (amounts use `Count` directly).
///
/// One canonical primitive — `Range(lo, hi)`, each bound optional (`None` =
/// unbounded that side) — with the readable named forms (`Exactly`, `AtLeast`,
/// `AtMost`, `Between`, `AnyNumber`) provided by the semantic authoring layer.
/// Lowering converts those forms to this explicit range before core sees them.
///
/// Bound ordering and non-zero validity are runtime concerns: an inverted
/// `Range(Some(5), Some(2))` and a degenerate "up to 0" are equally
/// constructible here, exactly as they were under the named variants.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Quantity {
    /// `Range(lo, hi)` — an inclusive cardinality range. `None` on a side is
    /// unbounded ([CR#601.2c] "any number" = `Range(None, None)`).
    Range(Option<Count>, Option<Count>),
}

impl Quantity {
    /// Exactly one object — `Range(Some(1), Some(1))`, the single-target/
    /// single-choice cardinality that recurs across target specs and
    /// selections.
    #[must_use]
    pub fn one() -> Quantity {
        Quantity::Range(Some(Count::Literal(1)), Some(Count::Literal(1)))
    }

    /// The `(lower, upper)` bound pair. `None` on a side = unbounded.
    /// The one read every consumer makes — lower/upper bound is a field read,
    /// not a five-way match.
    #[must_use]
    pub fn bounds(&self) -> (Option<&Count>, Option<&Count>) {
        match self {
            Quantity::Range(lo, hi) => (lo.as_ref(), hi.as_ref()),
        }
    }

    /// Whether this quantity is exactly one (`Range(Some(1), Some(1))`) — the
    /// single-target predicate the announce-list renderer and target machinery
    /// ask.
    #[must_use]
    pub fn is_one(&self) -> bool {
        matches!(
            self.bounds(),
            (Some(Count::Literal(1)), Some(Count::Literal(1)))
        )
    }
}
