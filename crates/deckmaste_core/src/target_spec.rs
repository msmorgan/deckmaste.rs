use std::sync::Arc;

use crate::Predicate;
use crate::Quantity;

/// One entry in an ability's announce list ([CR#601.2c,115]). A `TargetSpec`
/// is the only place "target" lives, and the announce list is an INDEXED
/// channel: lowering assigns each entry a region parameter, and the body reads
/// that parameter by position. Rechecked at resolution ([CR#608.2b]).
///
/// Each target entry declares an
/// [`AnnouncedTarget`](crate::Provenance::AnnouncedTarget) parameter. The body
/// reads that indexed register directly; it does not share the candidate,
/// capture, or instruction-product channels. Two same-kind slots (the fight
/// family) therefore need no labels and admit no ambiguity.
///
/// Separated from [`crate::Selection`] so that resolution-time choices
/// (`Each`, `Choose`, …) and announce-time targets never share a position —
/// targeting has legality recheck and retargeting rules that the other
/// choice forms don't.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum TargetSpec {
    /// A quantity of targets matching the filter ([CR#115.1,115.6,601.2c]).
    /// Use `Quantity::one()` for a single target, `Quantity::Range(None,
    /// Some(n))` for "up to N", and `Quantity::Range(None, None)` for "any
    /// number of targets" (or the `Exactly`/`AtMost`/`AnyNumber` macros at the
    /// RON surface).
    Target(Quantity, Arc<crate::Region<Predicate>>),
    /// A co-target set-distinctness constraint ([CR#115.7e], Arc Trail's
    /// "any *other* target"): this spec's final picks must not overlap the
    /// sibling specs at the given indices. Evaluated on the FINAL target
    /// set — retargeting may swap members; only the whole set is checked,
    /// at announce and at the [CR#608.2b] re-check. Never a fixed-binding
    /// exclusion (that is `And([…, Not(Ref(…))])` inside the filter).
    Distinct(Arc<[usize]>, Arc<TargetSpec>),
}
