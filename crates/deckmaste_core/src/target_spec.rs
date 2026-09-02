use std::sync::Arc;

use crate::Predicate;
use crate::Quantity;

/// One entry in an ability's announce list ([CR#601.2c,115]). A `TargetSpec`
/// is the only place "target" lives, and the announce list is an INDEXED
/// channel: the body names this entry BY POSITION — as
/// [`Reference::Target(n)`](crate::Reference::Target) (singular) or
/// [`Selection::Targets(n)`](crate::Selection::Targets) (the slot's whole
/// group), where `n` is this spec's index. Rechecked at resolution
/// ([CR#608.2b]).
///
/// A target pushes NO antecedent: `It`/`That(Sort)`/`They`/`Them` resolve over
/// the antecedent stack (loop elements, binder choices, move/create products)
/// and can never name a target. A target is not something a clause produced
/// and then referred back to — it is announced at index `n` and read at index
/// `n`, so two same-sort slots (the fight family) need no labelling and admit
/// no ambiguity.
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
    Target(Quantity, Predicate),
    /// A co-target set-distinctness constraint ([CR#115.7e], Arc Trail's
    /// "any *other* target"): this spec's final picks must not overlap the
    /// sibling specs at the given indices. Evaluated on the FINAL target
    /// set — retargeting may swap members; only the whole set is checked,
    /// at announce and at the [CR#608.2b] re-check. Never a fixed-binding
    /// exclusion (that is `And([…, Not(Ref(…))])` inside the filter).
    Distinct(Arc<[usize]>, Arc<TargetSpec>),
}
