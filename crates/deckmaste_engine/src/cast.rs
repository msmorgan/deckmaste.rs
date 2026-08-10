//! Casting ([CR#601]): the mana-payment solver plus the reified announce flow
//! (`begin_cast` → `announce_x` → `announce_targets` → `pay_cost`), and the
//! `can_cast` legality gate that `legal::legal_actions` offers from. The
//! announce flow (`announce_targets` / `pay_cost`) is shared with activated
//! abilities ([CR#602.2b]); see `activate.rs` for the activation entry point.

use std::sync::Arc;

use deckmaste_core::Action as CoreAction;
use deckmaste_core::Agency;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PayAct;
use deckmaste_core::PipClass;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::StaticEffect;
use deckmaste_core::TargetSpec;
use deckmaste_core::Timing;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::decide::Action;
use crate::decide::PendingDecision;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::Tapped;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::ManaPool;
use crate::player::PlayerId;
use crate::stack::Frame;
use crate::stack::PendingStackEntry;
use crate::stack::StackObject;
use crate::state::GameState;

/// The pool units spent on a cost ([CR#601.2g]): indices into the player's
/// mana pool at payment time. The decision is atomic, so indices into the
/// `PayMana` snapshot equal indices into the live pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    pub units: Vec<usize>,
}

/// [CR#107.3a,107.3i]: substitute each `{X}` (`ManaSymbol::Variable`) in `cost`
/// with `Generic(x)` — all instances of X take the one announced value. A cost
/// with no `Variable` is returned unchanged, so callers may apply this
/// unconditionally.
#[must_use]
pub(crate) fn concretize_x(cost: &ManaCost, x: Uint) -> ManaCost {
    ManaCost::from(
        cost.iter()
            .map(|s| match s {
                ManaSymbol::Variable => ManaSymbol::Simple(SimpleManaSymbol::Generic(x)),
                other => *other,
            })
            .collect::<Arc<[_]>>(),
    )
}

/// Which half of the [CR#601.2f] application order a pass applies: increases
/// (and mandatory additional mana) first, then reductions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChangePhase {
    Raise,
    Lower,
}

/// Fold the MANA components of a cost change into `cost`. A non-mana
/// component on a cost-change row has no consumer yet — loud, per the seam
/// discipline.
fn add_mana_components(cost: &mut Vec<ManaSymbol>, components: &[CostComponent]) {
    for c in components {
        match c {
            CostComponent::Mana(m) => cost.extend(m.iter().copied()),
            other => {
                todo!(
                    "engine seam: non-mana cost-increase component {other:?} ([CR#601.2f]) — fold when a card needs it; owner: engine-cost-modification-residue"
                )
            }
        }
    }
}

/// Remove a reduction's MANA components from `cost` ([CR#601.2f]): `{N}`
/// reduces the summed generic component (flooring at zero — a cost never
/// goes negative, [CR#118.5]); a colored pip removes ONE matching colored
/// pip and nothing else. Other symbol kinds in a reduction are loud.
fn reduce_mana_components(cost: &mut Vec<ManaSymbol>, components: &[CostComponent]) {
    for c in components {
        match c {
            CostComponent::Mana(m) => {
                for sym in m.iter() {
                    reduce_symbol(cost, *sym);
                }
            }
            other => todo!(
                "engine seam: non-mana cost-reduction component {other:?} ([CR#601.2f]) — fold when a card needs it; owner: engine-cost-modification-residue"
            ),
        }
    }
}

/// Remove one reduction symbol from `cost` (see [`reduce_mana_components`]).
fn reduce_symbol(cost: &mut Vec<ManaSymbol>, sym: ManaSymbol) {
    match sym {
        ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => {
            let mut remaining = n;
            for s in cost.iter_mut() {
                if remaining == 0 {
                    break;
                }
                if let ManaSymbol::Simple(SimpleManaSymbol::Generic(g)) = s {
                    let cut = remaining.min(*g);
                    *g -= cut;
                    remaining -= cut;
                }
            }
            // Drop zeroed generic pips ({0} artifacts keep their printed {0}:
            // only pips this reduction emptied vanish, and a cost that was
            // ALL generic keeps one {0} pip rather than becoming "no cost").
            let had_pips = !cost.is_empty();
            cost.retain(|s| !matches!(s, ManaSymbol::Simple(SimpleManaSymbol::Generic(0))));
            if had_pips && cost.is_empty() {
                cost.push(ManaSymbol::Simple(SimpleManaSymbol::Generic(0)));
            }
        }
        colored @ ManaSymbol::Simple(SimpleManaSymbol::Specific(_)) => {
            if let Some(i) = cost.iter().position(|s| *s == colored) {
                cost.remove(i);
            }
        }
        other => todo!(
            "engine seam: reduce_symbol can't handle {other:?} in a cost reduction ([CR#601.2f]) — fold when a card needs it; owner: engine-cost-modification-residue"
        ),
    }
}

/// A cost's payment requirement: its colored pips (per color), its `{S}` (snow,
/// [CR#107.4h]) pip count, and its total generic. Hybrid/Phyrexian/`{X}` are
/// *not* representable here — `requirement` returns `None` for them (they are
/// concretized/announced away before payment; see [`requirement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Requirement {
    colored: Vec<(ColorOrColorless, Uint)>,
    snow: Uint,
    generic: Uint,
}

impl Requirement {
    /// The cost's mana value: one unit per pip ([CR#202.3]; `{S}` counts 1).
    fn mana_value(&self) -> Uint {
        self.colored.iter().map(|(_, n)| *n).sum::<Uint>() + self.snow + self.generic
    }
}

/// The colored / snow / generic requirement of a cost, or `None` if the cost
/// uses an out-of-scope symbol (X, hybrid, Phyrexian).
///
/// Hybrid and Phyrexian symbols are concretized to `Simple` symbols at
/// [CR#601.2b] (the `ChooseCostOptions` step) before payment, and `Variable`
/// (`{X}`) is announced there too (engine-x-costs) — so a *residual* one
/// reaching `requirement` is an engine bug, not a payable cost. `{S}` (snow,
/// [CR#107.4h]) is NOT concretized: it is recognized here and matched against
/// snow-rider units at payment.
fn requirement(cost: &ManaCost) -> Option<Requirement> {
    let mut colored: Vec<(ColorOrColorless, Uint)> = Vec::new();
    let mut snow: Uint = 0;
    let mut generic: Uint = 0;
    for symbol in cost.iter() {
        match symbol {
            ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => generic += n,
            ManaSymbol::Simple(SimpleManaSymbol::Specific(c)) => {
                match colored.iter_mut().find(|(k, _)| k == c) {
                    Some((_, v)) => *v += 1,
                    None => colored.push((*c, 1)),
                }
            }
            // [CR#107.4h]: a snow pip — paid by a unit carrying ManaRider::Snow.
            ManaSymbol::Snow => snow += 1,
            // The hybrid/Phyrexian families are concretized at [CR#601.2b]
            // before payment; Variable ({X}) is announced there too
            // (engine-x-costs). A residual one here is an engine bug, not a
            // payable cost.
            ManaSymbol::Hybrid(..) | ManaSymbol::Phyrexian(..) | ManaSymbol::Variable => {
                return None;
            }
        }
    }
    Some(Requirement {
        colored,
        snow,
        generic,
    })
}

/// One pip's eligibility over a pool unit, for the payment matcher. A colored
/// pip accepts a unit of its color; a snow pip ([CR#107.4h]) accepts any unit
/// carrying `ManaRider::Snow`; a generic pip accepts any unit.
#[derive(Debug, Clone, Copy)]
enum Pip {
    Colored(ColorOrColorless),
    Snow,
    Generic,
}

impl Pip {
    /// Whether `unit` can pay this pip.
    fn accepts(self, unit: &crate::player::ManaUnit) -> bool {
        match self {
            Pip::Colored(c) => unit.kind == c,
            Pip::Snow => unit.riders.contains(&deckmaste_core::ManaRider::Snow),
            Pip::Generic => true,
        }
    }
}

/// The cost's pips as a flat list, in match-difficulty order: colored first
/// (tightest — one color), then snow ([CR#107.4h] — snow units of any color),
/// then generic (any unit). Order only affects auto-pay's *choice* of units,
/// not feasibility (the matcher backtracks); it puts the most-constrained pips
/// first so a greedy seeding lands a maximum matching faster.
fn pips(req: &Requirement) -> Vec<Pip> {
    let mut pips = Vec::with_capacity(req.mana_value() as usize);
    for &(c, n) in &req.colored {
        pips.extend(std::iter::repeat_n(Pip::Colored(c), n as usize));
    }
    pips.extend(std::iter::repeat_n(Pip::Snow, req.snow as usize));
    pips.extend(std::iter::repeat_n(Pip::Generic, req.generic as usize));
    pips
}

/// The index in `symbols` of the first pip a [`PipClass`] alternative may pay
/// ([CR#601.2g]): a `Generic(n>0)` for `PipClass::Generic` (delve / improvise /
/// convoke's generic clause), or a colored pip of the named color for
/// `PipClass::Colored` (convoke's per-color clause, [CR#702.51a]). `None` when
/// no such pip remains. Snow and colorless pips are never matched — those
/// keywords pay only generic / colored mana.
fn pip_index(symbols: &[ManaSymbol], class: PipClass) -> Option<usize> {
    symbols.iter().position(|s| match (class, s) {
        (PipClass::Generic, ManaSymbol::Simple(SimpleManaSymbol::Generic(n))) => *n > 0,
        (
            PipClass::Colored(color),
            ManaSymbol::Simple(SimpleManaSymbol::Specific(ColorOrColorless::Color(c))),
        ) => *c == color,
        _ => false,
    })
}

/// Remove ONE pip at `idx` from the working payment list — the pip an
/// alternative covered, so it "isn't paid with mana" ([CR#702.51a] "rather than
/// pay that mana"). A colored pip is one symbol, dropped outright; a
/// `Generic(n)` bundles n pips, so its count is decremented (the symbol dropped
/// at 0). Edits ONLY the working list: the printed cost and mana value
/// ([CR#202.3]) are never touched — paying this way still counts as paying the
/// original cost ([CR#118.7]).
fn remove_one_pip(symbols: &mut Vec<ManaSymbol>, idx: usize, class: PipClass) {
    match class {
        PipClass::Generic => {
            if let ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) = &mut symbols[idx] {
                *n -= 1;
                if *n == 0 {
                    symbols.remove(idx);
                }
            }
        }
        PipClass::Colored(_) => {
            symbols.remove(idx);
        }
    }
}

/// Maximum bipartite matching (Kuhn's augmenting-path algorithm) of `pips` to
/// the pool `units` restricted to `candidates`, returning, for each pip, the
/// chosen unit index (or `None` if unmatched). Pools are tiny, so the O(V·E)
/// cost is negligible; an exact matching is used (not greedy) so the
/// colored/snow/generic interactions ([CR#107.4h]) are always resolved
/// correctly — e.g. `{G}{S}` over a (plain green, snow green) pool must use the
/// plain green for `{G}` and the snow green for `{S}`.
fn match_pips(
    pips: &[Pip],
    units: &[crate::player::ManaUnit],
    candidates: &[usize],
) -> Vec<Option<usize>> {
    // pip_for[pip] = candidate-slot currently matched to that pip.
    let mut pip_for: Vec<Option<usize>> = vec![None; pips.len()];
    // Reverse map over candidate slots: which pip holds candidate slot `s`.
    let mut held_by: Vec<Option<usize>> = vec![None; candidates.len()];
    for (p, &pip) in pips.iter().enumerate() {
        let mut visited = vec![false; candidates.len()];
        augment(pip, p, pips, units, candidates, &mut held_by, &mut visited);
    }
    // Rebuild pip -> unit-index from the candidate-slot assignment.
    for (slot, holder) in held_by.iter().enumerate() {
        if let Some(p) = holder {
            pip_for[*p] = Some(candidates[slot]);
        }
    }
    pip_for
}

/// Try to match pip `p` (eligibility `pip`) to some candidate slot, displacing
/// already-matched pips along an augmenting path. Returns whether `p` got a
/// slot.
fn augment(
    pip: Pip,
    p: usize,
    pips: &[Pip],
    units: &[crate::player::ManaUnit],
    candidates: &[usize],
    held_by: &mut [Option<usize>],
    visited: &mut [bool],
) -> bool {
    for (slot, &unit_idx) in candidates.iter().enumerate() {
        if visited[slot] || !pip.accepts(&units[unit_idx]) {
            continue;
        }
        visited[slot] = true;
        let free_or_rematched = match held_by[slot] {
            None => true,
            Some(other) => augment(
                pips[other],
                other,
                pips,
                units,
                candidates,
                held_by,
                visited,
            ),
        };
        if free_or_rematched {
            held_by[slot] = Some(p);
            return true;
        }
    }
    false
}

/// Whether `pool` can pay `cost` ([CR#601.2g]). Each colored pip must be
/// covered by a distinct unit of its color, each `{S}` pip ([CR#107.4h]) by a
/// distinct unit carrying `ManaRider::Snow` (of any color), and each generic
/// pip by any remaining unit. Feasibility is decided by an exact bipartite
/// matching, so the colored ↔ snow interaction (a snow unit may pay either, but
/// not both at once) is handled correctly.
///
/// Returns `false` for costs containing out-of-scope symbols (X, hybrid,
/// Phyrexian — concretized away before payment, [CR#601.2b]).
#[must_use]
pub fn can_pay(pool: &ManaPool, cost: &ManaCost) -> bool {
    let Some(req) = requirement(cost) else { return false };
    let pips = pips(&req);
    let units = pool.units();
    let all: Vec<usize> = (0..units.len()).collect();
    // A perfect matching of every pip to a distinct unit means the pool covers
    // the cost.
    match_pips(&pips, units, &all).iter().all(Option::is_some)
}

/// Whether `payment`'s selected pool units legally cover `cost` from `pool`
/// ([CR#601.2g]).
///
/// The selected indices must be distinct and in range; the number of units
/// selected must equal the cost's mana value (colored + `{S}` + generic); and
/// the selected units must admit a perfect matching to the cost's pips — each
/// colored pip ↔ a unit of that color, each `{S}` pip ([CR#107.4h]) ↔ a unit
/// carrying `ManaRider::Snow`, each generic pip ↔ any selected unit.
/// (Spendability/`SpendOnly` is not checked here — see `validate_spendable`.)
#[must_use]
pub fn validate_payment(pool: &ManaPool, cost: &ManaCost, payment: &Payment) -> bool {
    let Some(req) = requirement(cost) else { return false };
    let units = pool.units();
    // Indices must be distinct and in range.
    let mut seen = std::collections::HashSet::with_capacity(payment.units.len());
    for &i in &payment.units {
        if i >= units.len() || !seen.insert(i) {
            return false;
        }
    }
    // Exactly the cost's mana value: no under- or over-spend.
    if payment.units.len() != req.mana_value() as usize {
        return false;
    }
    // The selected units must perfectly match the pips. Equal cardinality plus a
    // perfect pip-side matching means every selected unit is also used.
    let pips = pips(&req);
    match_pips(&pips, units, &payment.units)
        .iter()
        .all(Option::is_some)
}

/// Deducts a validated `payment`'s selected units from `pool`
/// ([CR#601.2g,106.4]). Callers must `validate_payment` first; out-of-range
/// indices are silently ignored, so an unvalidated payment may under-spend.
///
/// Seam: a spent unit's `GrantOnSpend`/`TriggerOnSpend` riders ([CR#106.6]) are
/// dropped here, not fired — on-spend effects need a "mana spent on X" event +
/// delayed triggers (deferred). `SpendOnly`/`Persistent` are already honored
/// (at payment / at emptying), so removal here is correct for them.
pub fn apply_payment(pool: &mut ManaPool, payment: &Payment) {
    pool.remove_units(&payment.units);
}

/// Canonical auto-tap ([CR#601.2g], a test convenience — the engine
/// surfaces the choice, this answers it): pick pool unit indices covering
/// `cost` (colored pips to matching-color units, `{S}` pips to snow-rider units
/// [CR#107.4h], generic pips to any remaining). Caller ensures `can_pay` first.
///
/// Ignores spendability (`SpendOnly`): every unit is eligible. Use
/// [`auto_pay_spendable`] (via [`GameState::auto_pay_pending`]) to honor a
/// subject's spend restrictions.
///
/// # Panics
///
/// Panics if `cost` is out of scope or `pool` cannot cover it (call `can_pay`
/// first).
#[cfg(test)]
#[must_use]
fn auto_pay(pool: &ManaPool, cost: &ManaCost) -> Payment {
    auto_pay_spendable(pool, cost, &vec![true; pool.units().len()])
}

/// Like [`auto_pay`], but only units `i` with `spendable[i] == true` are
/// eligible ([CR#106.6]). `spendable` must index the same `pool` (length
/// `== pool.units().len()`).
///
/// Snow-rider units are RESERVED for `{S}` pips ([CR#107.4h]): the matcher
/// considers non-snow units first, so colored/generic pips spend a plain unit
/// when one is available and the scarcer snow units stay open for `{S}` (e.g.
/// auto-paying `{G}{S}` over a plain-green + snow-green pool pairs plain →
/// `{G}` and snow → `{S}`). Correctness — a covering selection whenever one
/// exists — is the matching's; the ordering only steers *which* covering
/// selection.
///
/// # Panics
///
/// Panics if `cost` is out of scope, or if the spendable units cannot cover it
/// (call `can_pay` over the spendable sub-pool first).
#[must_use]
pub fn auto_pay_spendable(pool: &ManaPool, cost: &ManaCost, spendable: &[bool]) -> Payment {
    let req =
        requirement(cost).expect("auto_pay_spendable on a cost the spendable units can cover");
    let units = pool.units();
    // Candidate units: spendable only, non-snow first so colored/generic pips
    // prefer a plain unit and reserve snow units for {S} pips ([CR#107.4h]).
    let mut candidates: Vec<usize> = (0..units.len()).filter(|&i| spendable[i]).collect();
    candidates.sort_by_key(|&i| units[i].riders.contains(&deckmaste_core::ManaRider::Snow));
    let pips = pips(&req);
    let matched = match_pips(&pips, units, &candidates);
    let chosen: Vec<usize> = matched
        .into_iter()
        .map(|m| m.expect("can_pay over the spendable sub-pool guarantees a covering unit"))
        .collect();
    Payment { units: chosen }
}

/// [CR#601.2h]: one `RunEffect` per cost-eligible verb, each performed by the
/// activating `player` against the ability's `source`, over a fresh
/// resolution frame whose `controller` is the activator — so `Reference::You`
/// (the verb's own agent slot, spelled) resolves to that player and
/// `Reference::This` (a self-sacrifice) to the source — mirroring the frame
/// any effect node resolves against (`targets`/`bindings`/`chosen` empty: a
/// cost verb names no targets and carries no trigger context). A
/// `With(ChooseOne/Choose)` binder inside a verb surfaces its own
/// `ChooseObjects` decision via `run_effect`'s `chosen.is_none()` path.
///
/// `x` is the value announced for this activation ([CR#601.2b]) — threaded onto
/// each cost-verb frame so a `Count::X` operand (a loyalty `−X`'s
/// `RemoveCounters(This, LoyaltyCounter, X)`) pays the announced amount,
/// exactly as the effect-side frame reads X. `None` when no X was announced
/// (the common no-X cost), leaving each `Count::X`-free verb untouched.
///
/// `payment` ([CR#118.10]) is stamped onto every frame this call
/// mints — the caller mints ONE [`crate::stack::Payment`] per cost payment
/// (shared across every `verb_payment_items` call and cost-`With` step that
/// payment's drain schedules) so every event this payment's verbs perform
/// reads as `Agency::CostPayment` and shares one payment id.
fn verb_payment_items(
    verbs: &[CoreAction],
    source: ObjectId,
    player: PlayerId,
    x: Option<Uint>,
    payment: crate::stack::Payment,
) -> Vec<WorkItem> {
    verbs
        .iter()
        .map(|verb| {
            // A cost verb names no targets; it may read the announced X.
            let mut frame = Frame::bare(source, player);
            frame.anaphora.x = x;
            frame.payment = Some(payment);
            WorkItem::RunEffect {
                effect: Arc::new(OneShotEffect::Act(verb.clone())),
                frame,
            }
        })
        .collect()
}

/// Whether a cost-eligible verb reads the announced X ([CR#107.3a]) in a count
/// operand — a loyalty `−X`/`+X` (`RemoveCounters`/`PutCounters`), a
/// pay-`X`-life (`LoseLife`), or an X-discard. Drives the non-mana X-announce
/// trigger: an activation whose cost carries such a verb must announce X even
/// without an `{X}` mana symbol. Looks through `Expanded` macro wrappers,
/// mirroring `verb_cost_payable`.
fn verb_mentions_cost_x(verb: &CoreAction) -> bool {
    match verb {
        // Pay-X-life ([CR#119.4]) only — a gain/set-life cost never reads X
        // this way, mirroring the former `PlayerAction::LoseLife`-only match.
        CoreAction::ChangeLife(_, deckmaste_core::LifeOp::Down(count))
        | CoreAction::PutCounters(_, _, count)
        | CoreAction::RemoveCounters(_, _, count) => count.mentions_x(),
        // An X-discard ("discard X cards") — the count rides the body's
        // `With` binder's `Quantity`.
        CoreAction::Composite { name, body } if name.as_str() == "Discard" => {
            deckmaste_core::discard_body_count(body).is_some_and(deckmaste_core::Count::mentions_x)
        }
        CoreAction::Expanded(expansion) => verb_mentions_cost_x(&expansion.value),
        _ => false,
    }
}

/// Whether a cardinality used by a cost-side choice reads the announced X.
fn quantity_mentions_cost_x(quantity: &deckmaste_core::Quantity) -> bool {
    let (lower, upper) = quantity.bounds();
    lower.is_some_and(deckmaste_core::Count::mentions_x)
        || upper.is_some_and(deckmaste_core::Count::mentions_x)
}

/// Whether a cost-side binder's choice cardinality reads the announced X.
fn binder_mentions_cost_x(binder: &deckmaste_core::Binder) -> bool {
    match binder {
        deckmaste_core::Binder::Choose { quantity, .. }
        | deckmaste_core::Binder::Search { quantity, .. } => quantity_mentions_cost_x(quantity),
        deckmaste_core::Binder::Produce(action) => verb_mentions_cost_x(action),
        deckmaste_core::Binder::Expanded(expansion) => binder_mentions_cost_x(&expansion.value),
        deckmaste_core::Binder::TheRef(_)
        | deckmaste_core::Binder::ChooseOne { .. }
        | deckmaste_core::Binder::SearchOne { .. }
        | deckmaste_core::Binder::Existing(_) => false,
    }
}

/// Whether a runnable cost component reads the one X value announced for the
/// spell or ability. This follows nested/lowered cost structure so modal and
/// optional additions participate only after they have actually been chosen.
fn cost_component_mentions_x(component: &CostComponent) -> bool {
    match component {
        CostComponent::Mana(mana) => mana
            .iter()
            .any(|symbol| matches!(symbol, ManaSymbol::Variable)),
        CostComponent::Act(action) => verb_mentions_cost_x(action),
        CostComponent::Cost(nested) => nested.iter().any(cost_component_mentions_x),
        CostComponent::TapTotal { count, .. } => count.mentions_x(),
        CostComponent::ChooseAndPay { binder, body } => {
            binder_mentions_cost_x(binder) || body.iter().any(cost_component_mentions_x)
        }
        CostComponent::Expanded(expansion) => cost_component_mentions_x(&expansion.value),
        CostComponent::ManaCostOf(_) | CostComponent::Tap | CostComponent::Untap => false,
    }
}

fn cost_components_mention_x(components: &[CostComponent]) -> bool {
    components.iter().any(cost_component_mentions_x)
}

/// Unwrap the `CostComponent::Do(action)` verbs `concretize` produces for
/// Phyrexian-life picks ([CR#107.4f]) back into the [`Action`]s
/// `verb_payment_items` schedules. `concretize` only ever emits
/// `Do(LoseLife(2))` here, so any other shape is an engine invariant violation.
fn phyrexian_life_verbs(verbs: &[CostComponent]) -> Vec<CoreAction> {
    verbs
        .iter()
        .map(|c| match c {
            CostComponent::Act(action) => (**action).clone(),
            other => unreachable!("concretize emits only Act(_) verb costs, got {other:?}"),
        })
        .collect()
}

/// [CR#118.9,702.35a]: split an ALTERNATIVE base cost (madness's madness cost,
/// threaded from `Cast(what, [cost])`) into its MANA part (the merged `Mana`
/// components — what `ChooseCostOptions`/`PayCost` demand as the base mana) and
/// its remaining PAYMENT components (`Do(...)` etc., paid alongside via the
/// same verb window `PayCost` already uses). A macro-spliced nested `Cost(...)`
/// flattens by recursion. Madness's `[Mana([1,R])]` yields just the mana.
fn partition_alternative_cost(cost: &deckmaste_core::Cost) -> (ManaCost, Vec<CostComponent>) {
    let mut symbols: Vec<ManaSymbol> = Vec::new();
    let mut verbs: Vec<CostComponent> = Vec::new();
    for component in cost {
        match component {
            CostComponent::Mana(m) => symbols.extend(m.iter().copied()),
            CostComponent::Cost(nested) => {
                let (m, v) = partition_alternative_cost(nested);
                symbols.extend(m.iter().copied());
                verbs.extend(v);
            }
            other => verbs.push(other.clone()),
        }
    }
    (ManaCost::from(Arc::from(symbols)), verbs)
}

/// [CR#118.9]: just the MANA portion of an alternative base cost — the
/// affordability-gate view ([`GameState::can_cast_as_effect`]).
fn alternative_cost_mana(cost: &deckmaste_core::Cost) -> ManaCost {
    partition_alternative_cost(cost).0
}

/// The target declarations contributed by the already-announced modal
/// selection. Each chosen mode owns a fresh target scope, so its specs are
/// appended in choice order; a nonmodal effect contributes its ordinary
/// top-level target wrapper.
pub(crate) fn announced_target_specs(
    effect: &OneShotEffect,
    chosen_modes: &[Uint],
) -> Vec<TargetSpec> {
    match effect {
        OneShotEffect::Modal(modal) => chosen_modes
            .iter()
            .flat_map(|&index| {
                let mode = modal
                    .modes
                    .get(index as usize)
                    .expect("ChooseModes validated every announced index");
                crate::resolve::top_targets(&mode.effect).iter().cloned()
            })
            .collect(),
        other => crate::resolve::top_targets(other).to_vec(),
    }
}

/// Additional cost components contributed by the selected modes and their
/// modal rider. Per-mode costs follow pick order (and repeat with a repeated
/// mode); escalate repeats once per pick beyond the first; entwine contributes
/// once when the all-modes alternative was chosen.
fn announced_mode_cost_components(
    effect: &OneShotEffect,
    chosen_modes: &[Uint],
) -> Vec<CostComponent> {
    let OneShotEffect::Modal(modal) = effect else {
        return Vec::new();
    };
    let mut components = Vec::new();
    for &index in chosen_modes {
        if let Some(cost) = &modal.modes[index as usize].cost {
            components.extend(cost.iter().cloned());
        }
    }
    match &modal.choose.rider {
        Some(deckmaste_core::ModalCostRider::Escalate(cost)) => {
            for _ in 1..chosen_modes.len() {
                components.extend(cost.iter().cloned());
            }
        }
        Some(deckmaste_core::ModalCostRider::Entwine(cost))
            if chosen_modes.len() == modal.modes.len()
                && chosen_modes
                    .iter()
                    .copied()
                    .eq(0..Uint::try_from(modal.modes.len()).expect("mode count fits Uint")) =>
        {
            components.extend(cost.iter().cloned());
        }
        Some(deckmaste_core::ModalCostRider::Entwine(_)) | None => {}
    }
    components
}

/// Reify a spell or activated ability's already-announced modal effects.
/// Every mode owns its own target scope: the flat stack-entry target list is
/// partitioned by the selected modes' target-spec counts, then each effect is
/// run with a frame containing only its segment. This keeps each mode's
/// `Target(0)` local even when several modes were chosen.
pub(crate) fn announced_effect_items(
    effect: &OneShotEffect,
    frame: &Frame,
    chosen_modes: &[Uint],
    targets: &[Vec<ObjectId>],
) -> Vec<WorkItem> {
    let OneShotEffect::Modal(modal) = effect else {
        return vec![WorkItem::RunEffect {
            effect: Arc::new(effect.clone()),
            frame: frame.clone(),
        }];
    };

    let mut offset = 0;
    let items = chosen_modes
        .iter()
        .map(|&index| {
            let mode = modal
                .modes
                .get(index as usize)
                .expect("ChooseModes validated every announced index");
            let count = crate::resolve::top_targets(&mode.effect).len();
            let mut mode_frame = frame.clone();
            mode_frame.anaphora.targets = targets[offset..offset + count].to_vec();
            offset += count;
            WorkItem::RunEffect {
                effect: Arc::new(mode.effect.clone()),
                frame: mode_frame,
            }
        })
        .collect();
    debug_assert_eq!(
        offset,
        targets.len(),
        "announcement stores exactly the selected modes' target slots"
    );
    items
}

impl GameState {
    /// [CR#601.2b,601.2g,107.4e,107.4f]: is SOME legal reading of `cost`'s
    /// hybrid/Phyrexian symbols fully payable by `player` for `subject`? The
    /// affordability gate (`can_cast`/`can_activate`) calls this when the cost
    /// has choosable symbols — a plain or `{S}`-only cost keeps the direct
    /// `can_pay` path.
    ///
    /// Hybrid/Phyrexian are concretized at announce ([CR#601.2b]); the gate
    /// must already know a payable reading EXISTS so the action is offered. A
    /// reading is a `CostOptionChoices` — one pick per choosable symbol — that
    /// `concretize` resolves to a concrete `(mana, verbs)`; it is payable iff
    /// the spendable pool covers the mana AND the Phyrexian-life picks are
    /// jointly affordable.
    ///
    /// `{X}` never blocks (its floor is X=0, [CR#107.3a]): callers pass an
    /// already-X-reduced cost (`concretize_x(.., 0)`), so a residual `Variable`
    /// is impossible and `choosable` (which ignores it anyway) sees only the
    /// hybrid/Phyrexian symbols. A cost with both `{X}` and a hybrid composes:
    /// X is reduced to `{0}` first, the hybrid drives this search.
    ///
    /// ## Search and shared-resource correctness
    ///
    /// The readings are searched by a bounded recursion over the per-symbol
    /// options (costs carry few choosable symbols; the product is tiny). The
    /// full concretization is assembled and checked at each leaf — never
    /// per-symbol greedily — because mana and life are resources SHARED across
    /// symbols: two `{W/P}` can't both be paid by 2 life (each costs 2; 4
    /// total), and two hybrids competing for one colored unit can't both take
    /// it. `can_pay` decides the mana side by an exact matching (joint), and
    /// the life side is checked against the COMBINED Phyrexian-life total here
    /// (`can_pay_verbs` alone judges each `LoseLife` against full life, so it
    /// can't see two life payments competing — this method sums them).
    #[must_use]
    pub(crate) fn affordable_concretization(
        &self,
        player: PlayerId,
        cost: &ManaCost,
        subject: ObjectId,
        pip_spell: Option<ObjectId>,
    ) -> bool {
        let options = crate::cost_options::choosable(cost);
        // Recurse over the per-symbol option lists, building one pick per
        // symbol; at a complete pick set, test the assembled concretization.
        self.any_reading_payable(
            player,
            cost,
            subject,
            &options.options,
            &mut Vec::new(),
            pip_spell,
        )
    }

    /// [CR#601.2b,601.2g]: whether `player` can afford `cost`'s mana for
    /// `subject` under SOME legal reading — reduces `{X}` to its 0 floor
    /// ([CR#107.3a]), then either the plain `can_pay` fast path (no choosable
    /// symbol) or the hybrid/Phyrexian reading search
    /// ([`affordable_concretization`]). The single entry point both `can_cast`
    /// and `can_activate` gate on, so a new caster can't forget the
    /// concretize/choosable step.
    pub(crate) fn gate_mana_affordable(
        &self,
        player: PlayerId,
        cost: &ManaCost,
        subject: ObjectId,
        pip_spell: Option<ObjectId>,
    ) -> bool {
        let reduced = concretize_x(cost, 0);
        if crate::cost_options::choosable(&reduced).options.is_empty() {
            // [CR#601.2g..601.2h]: pips a spell's `PayPips` statics can cover
            // (convoke / delve / improvise) drop out of the mana that must be
            // paid with real mana, so a cast is affordable when mana plus
            // available pip-payment resources together cover the cost.
            let payable = self.mana_after_pips(&reduced, pip_spell);
            can_pay(&self.spendable_pool(player, subject), &payable)
        } else {
            self.affordable_concretization(player, &reduced, subject, pip_spell)
        }
    }

    /// `mana` with the pips a spell's `PayPips` statics can currently cover
    /// removed ([CR#601.2g..601.2h]) when `pip_spell` names the spell being
    /// cast, else `mana` unchanged. The single point where the affordability
    /// gate consults the same [`Self::pip_coverage`] walk the payment window
    /// uses, so a cast the gate judges payable is actually payable. `None`
    /// (activated abilities, which have no `PayPips`) is a plain pass-through.
    fn mana_after_pips(&self, mana: &ManaCost, pip_spell: Option<ObjectId>) -> ManaCost {
        match pip_spell {
            Some(spell) => self.pip_coverage(spell, mana).0,
            None => mana.clone(),
        }
    }

    /// Depth-first walk of the choosable symbols' readings: `picks` holds the
    /// readings chosen for symbols `0..picks.len()`; `options[picks.len()..]`
    /// remain. At a full pick set (`picks.len() == options.len()`) the
    /// assembled concretization is tested for full payability. Returns true
    /// as soon as one payable reading is found (short-circuits).
    fn any_reading_payable(
        &self,
        player: PlayerId,
        cost: &ManaCost,
        subject: ObjectId,
        options: &[crate::cost_options::SymbolOptions],
        picks: &mut Vec<crate::cost_options::SymbolChoice>,
        pip_spell: Option<ObjectId>,
    ) -> bool {
        if picks.len() == options.len() {
            return self.reading_payable(player, cost, subject, picks, pip_spell);
        }
        for &choice in &options[picks.len()].choices {
            picks.push(choice);
            let payable =
                self.any_reading_payable(player, cost, subject, options, picks, pip_spell);
            picks.pop();
            if payable {
                return true;
            }
        }
        false
    }

    /// Whether one complete reading (`picks`) of `cost` concretizes to a fully
    /// payable `(mana, verbs)` for `player`/`subject` ([CR#601.2g,601.2h]). The
    /// mana is matched against the spendable pool (joint, via `can_pay`); the
    /// verbs are checked structurally by `can_pay_verbs` AND — for the
    /// Phyrexian-life picks, the one resource shared across symbols here — by
    /// their COMBINED life requirement against the player's life.
    fn reading_payable(
        &self,
        player: PlayerId,
        cost: &ManaCost,
        subject: ObjectId,
        picks: &[crate::cost_options::SymbolChoice],
        pip_spell: Option<ObjectId>,
    ) -> bool {
        let choices = crate::cost_options::CostOptionChoices {
            picks: picks.to_vec(),
        };
        // A complete, legal pick set always concretizes — `picks` is built from
        // `choosable`'s own options, so the count and legality are guaranteed.
        let Ok((mana, verbs)) = crate::cost_options::concretize(cost, &choices) else {
            return false;
        };
        // [CR#601.2g..601.2h]: pip-payment covers pips of this concretized
        // reading before the pool is asked to fund the remainder — matching the
        // payment window, which runs the same walk over the concretized mana.
        let mana = self.mana_after_pips(&mana, pip_spell);
        if !can_pay(&self.spendable_pool(player, subject), &mana) {
            return false;
        }
        let verb_actions = phyrexian_life_verbs(&verbs);
        // Structural per-verb payability (here: each ChangeLife(Down) is
        // non-negative and life ≥ that ONE amount). `concretize` emits only
        // Do(ChangeLife(You, Down(2))), so this is the [CR#119.4] floor; the
        // joint check below adds the shared-life constraint `can_pay_verbs`
        // can't express.
        if !self.can_pay_verbs(player, &verb_actions, subject) {
            return false;
        }
        // [CR#107.4f]: the COMBINED life of all Phyrexian-life picks must be
        // affordable — two {W/P} paid with life cost 4, not 2. `can_pay_verbs`
        // judges each ChangeLife(Down) against full life independently, so sum them.
        // The frame mirrors the one `can_pay_verbs`/`verb_payment_items` use: a
        // cost verb names no targets and `~`/`This` is the live source.
        let frame = Frame::bare(subject, player);
        let life_required: Uint = verb_actions
            .iter()
            .map(|v| self.life_cost_of(v, &frame))
            .sum::<Uint>();
        let life = Uint::try_from(self.player(player).life.max(0)).unwrap_or(Uint::MAX);
        life >= life_required
    }

    /// The life a single concretized Phyrexian-life verb costs. `concretize`
    /// emits only `Do(ChangeLife(You, Down(n)))` for life picks ([CR#107.4f];
    /// paying life IS losing life, [CR#119.4]); any other shape contributes 0
    /// (its own structural check in `can_pay_verbs` covers it — this sum is
    /// purely the shared-life constraint).
    fn life_cost_of(&self, verb: &CoreAction, frame: &Frame) -> Uint {
        match verb {
            CoreAction::ChangeLife(_, deckmaste_core::LifeOp::Down(count)) => {
                self.eval_count(count, frame)
            }
            _ => 0,
        }
    }

    /// [CR#601.3,601.2g]: may `player` cast `object` now? Offered iff the
    /// object is in the holder's hand (the caller iterates the hand), the
    /// object is not a land ([CR#305.9]), timing permits (instant → any
    /// priority; otherwise sorcery-speed), the pool can pay the cost, and
    /// every target spec has at least one legal candidate.
    #[must_use]
    pub(crate) fn can_cast(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
    ) -> bool {
        // Split into the mana-independent legality (timing, non-empty cost, a
        // legal target per spec) and the affordability gate. The former yields
        // the concrete cost to price; a runner-side autotapper reuses it (see
        // `autotap_for_cast`) to tell "blocked only by unfloated mana" apart
        // from "illegal regardless of mana".
        let Some(cost) = self.castable_cost_ignoring_mana(view, player, object) else {
            return false;
        };
        // [CR#601.2b,601.2g,107.3a]: gate mana affordability under all legal
        // readings (concretizes {X} to 0, then plain or hybrid/Phyrexian path).
        // The spell's own `PayPips` statics (convoke / delve / improvise) may
        // cover pips the pool can't, so pass the spell for the pip-payment walk.
        self.gate_mana_affordable(player, &cost, object, Some(object))
    }

    /// The concrete cost `player` must cover to cast `object`, IFF every
    /// mana-INDEPENDENT casting legality holds (not a land, correct timing per
    /// [CR#307.1,117.1a,702.8a], a non-empty printed cost per [CR#118.6], and —
    /// per [CR#601.2c] — at least one legal candidate for every target spec);
    /// otherwise `None`. The mana-affordability gate is deliberately omitted so
    /// a runner autotapper can decide whether an unaffordable-looking cast is
    /// blocked ONLY by unfloated mana. `can_cast` = this AND affordability.
    pub(crate) fn castable_cost_ignoring_mana(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
    ) -> Option<ManaCost> {
        // Lands are never cast as spells — playing a land is a special action
        // ([CR#305.9,116.2a,701.18]). Keyed on the conferred `May(Play)`
        // capability (a card's Land type confers it), not a `Type::Land`
        // literal — per-face correct for an MDFC land//spell.
        if crate::legal::confers_may_play(self, view, object) {
            return None;
        }
        let face = crate::derive::face(self.def(object));
        // Sorcery speed by default ([CR#307.1,117.1a]), unless a
        // May(Cast(window: InstantSpeed)) row lifts it ([CR#702.8a] flash).
        // An Instant's own type CONFERS exactly this row, so the casting
        // window is data — no `Type::Instant` literal. The card's own row
        // functions from the hand; an Orrery-style battlefield grant rides
        // the same collector. Rows carrying `from`/`cost` slots are
        // different unlocks (cast-from-zones, alternative costs) and never
        // lift timing.
        let proxy = self.player(player).object;
        let timing_ok = self.sorcery_speed_ok(player)
            || crate::legal::may_cast_rows(self, view, object)
                .iter()
                .any(|r| {
                    r.window == Some(Timing::InstantSpeed)
                        && r.from.is_none()
                        && r.cost.is_none()
                        && self.filter_matches_live(&r.what, object, r.carrier)
                        && self.filter_matches_live(&r.by, proxy, r.carrier)
                });
        if !timing_ok {
            return None;
        }
        // [CR#702.61a,101.2]: a Cant(Cast) row (split second on the stack, or
        // a battlefield "can't cast" grant) forbids this cast — Cant beats
        // the flash May, so this is checked after the timing lift, not
        // folded into it.
        if crate::legal::cant_cast(self, view, object, player) {
            return None;
        }
        // [CR#118.6]: an EMPTY mana cost is "no mana cost" — an unpayable
        // base. Attempting the cast is legal in the CR but pointless to
        // offer; an alternative cost ([CR#118.6a], May(Cast(cost: …)) rows)
        // is the future unlock. {0} is spelled [Generic(0)] and payable
        // ([CR#118.5]).
        if face.mana_cost.is_empty() {
            return None;
        }
        let cost = self.mana_cost(object)?;
        // If the spell targets, its specs must be jointly satisfiable — each
        // slot offering at least its minimum count, the Distinct slots admitting
        // distinct representatives ([CR#601.2c,115.7e]). The carrier is the
        // spell's own object source — anchors a target filter's `Ref(This)`.
        let carrier = Some(self.objects.obj(object).source);
        let specs = crate::resolve::spell_targets(view, object);
        let legal: Vec<Vec<ObjectId>> = specs
            .iter()
            .map(|spec| self.legal_targets(spec, carrier))
            .collect();
        crate::resolve::announce_satisfiable(&specs, &legal).then_some(cost)
    }

    /// Runner autotap ([CR#106.4,605.1a]): the ordered mana-ability activations
    /// that float exactly enough mana for `player` to cast `object`, or `None`
    /// when the cast is blocked by something floating mana can't fix (wrong
    /// timing, no legal target, an empty/`{X}`/`{S}`/hybrid/Phyrexian cost) or
    /// the player's untapped lands can't cover the cost. Pure and read-only:
    /// the engine mutates nothing — auto-tapping is a runner POLICY, so a
    /// consumer submits the returned `ActivateAbility` actions (then the
    /// `CastSpell`) itself, exactly as if the player had tapped by hand.
    ///
    /// Minimal by design: it covers `{C}`/colored/generic pips paid by
    /// tap-for-fixed-specific-mana sources (basics and mono lands), matching
    /// colored pips to exact-colour sources first, then generic pips to any
    /// leftover. Costs with `{X}`/`{S}`/hybrid/Phyrexian pips, and
    /// `AnyColor`/`OneOf`/other non-fixed sources, return `None` (the player
    /// taps those by hand). Colour-OPTIMAL selection (sparing a colored source
    /// for a generic pip) is a deferred refinement; the greedy here is correct
    /// for covering THIS cost.
    #[must_use]
    pub fn autotap_for_cast(&self, player: PlayerId, object: ObjectId) -> Option<Vec<Action>> {
        // The player's untapped fixed-mana land sources, each recorded with the
        // ability index `legal_actions`/`ActivateAbility` uses (into the
        // Innate-peeled `usable_abilities` list — see legal.rs).
        struct Src {
            object: ObjectId,
            ability: usize,
            color: ColorOrColorless,
            amount: Uint,
        }
        let view = self.layers();
        // Legal but for the mana? Also yields the concrete cost to cover.
        let cost = self.castable_cost_ignoring_mana(&view, player, object)?;
        // Only fixed Simple pips are auto-tappable; anything needing an announce
        // choice ({X}) or a non-fixed source ({S}/hybrid/Phyrexian) bails.
        let mut colored: Vec<ColorOrColorless> = Vec::new();
        let mut generic: Uint = 0;
        for sym in cost.iter() {
            match sym {
                ManaSymbol::Simple(SimpleManaSymbol::Specific(c)) => colored.push(*c),
                ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => {
                    generic = generic.checked_add(*n)?;
                }
                _ => return None,
            }
        }
        let mut sources: Vec<Src> = Vec::new();
        for &land in &self.zones.battlefield {
            let obj = self.objects.obj(land);
            if view.controller(land) != player || obj.tapped {
                continue;
            }
            // Mirror the legal_actions guard: a conferred
            // `Cant(Activate(cost: IncludesTapSymbol))` forbids the {T} mana
            // ability — the summoning-sickness tap gate a `Creature` type
            // confers (lands aren't creatures, so this rarely bites).
            // `blanket_applies: false` — this is the mana-ability tap arm,
            // exempt from a blanket split-second-style row ([CR#702.61b]).
            if crate::legal::cant_activate(self, &view, land, player, true, false) {
                continue;
            }
            for (ability, a) in crate::derive::usable_abilities(self, land)
                .iter()
                .enumerate()
            {
                if let Some((color, amount)) = crate::derive::tap_mana_ability(a) {
                    sources.push(Src {
                        object: land,
                        ability,
                        color,
                        amount,
                    });
                    break; // one mana ability per source suffices here
                }
            }
        }
        // Greedy coverage over a working bag of available units: the spendable
        // pool already floated ([CR#106.6] SpendOnly-filtered for this subject),
        // then colored pips from exact-colour sources, then generic from any.
        let mut available: Vec<ColorOrColorless> = self
            .spendable_pool(player, object)
            .units()
            .iter()
            .map(|u| u.kind)
            .collect();
        let take = |bag: &mut Vec<ColorOrColorless>, want: Option<ColorOrColorless>| -> bool {
            let pos = match want {
                Some(c) => bag.iter().position(|&x| x == c),
                None => (!bag.is_empty()).then_some(0),
            };
            match pos {
                Some(i) => {
                    bag.remove(i);
                    true
                }
                None => false,
            }
        };
        let mut plan: Vec<Action> = Vec::new();
        for &need in &colored {
            if take(&mut available, Some(need)) {
                continue;
            }
            let i = sources.iter().position(|s| s.color == need)?;
            let s = sources.remove(i);
            plan.push(Action::ActivateAbility {
                object: s.object,
                ability: s.ability,
            });
            for _ in 0..s.amount {
                available.push(s.color);
            }
            take(&mut available, Some(need)); // now guaranteed present
        }
        let mut g = generic;
        while g > 0 {
            if take(&mut available, None) {
                g -= 1;
                continue;
            }
            let s = sources.pop()?;
            plan.push(Action::ActivateAbility {
                object: s.object,
                ability: s.ability,
            });
            for _ in 0..s.amount {
                available.push(s.color);
            }
        }
        // Defence in depth: reuse the engine's exact pip matcher over the pool
        // the plan would produce, so a planning slip fails closed (no taps)
        // rather than tapping lands for a cast that can't actually be paid.
        let mut projected = self.spendable_pool(player, object);
        for act in &plan {
            if let Action::ActivateAbility {
                object: land,
                ability,
            } = act
                && let Some((c, n)) = crate::derive::tap_mana_ability(
                    &crate::derive::usable_abilities(self, *land)[*ability],
                )
            {
                projected.add(
                    c,
                    n,
                    crate::player::ManaProvenance {
                        source: Some(*land),
                        action: None,
                    },
                );
            }
        }
        can_pay(&projected, &cost).then_some(plan)
    }

    /// [CR#601.2a,601.2b]: move the spell from its controller's hand to the stack and
    /// open the announce slot. Procedural — not an event.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not in its controller's hand — engine invariant.
    pub(crate) fn begin_cast(&mut self, object: ObjectId) {
        let controller = self.objects.obj(object).controller;
        self.begin_cast_from(object, Zone::Hand, controller, None);
    }

    /// [CR#601.2a,601.2b]: the general open-announce, from any `origin` zone
    /// under `controller`'s control — the hand cast ([`begin_cast`]) and the
    /// cast-as-effect ([CR#608.2g] — casting the referenced card from the zone
    /// it's in, e.g. Chandra's just-exiled card) share this body. Removes
    /// `object` from `origin`, moves it to the stack, and opens the announce
    /// slot recording `origin` so a rewound/countered cast returns there.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not in `origin` — the caller validated the zone
    /// (engine invariant, not caller input).
    ///
    /// `alternative_cost` ([CR#118.9,702.35a]) is the base cost this cast pays
    /// RATHER THAN the card's mana cost (madness's madness cost, threaded from
    /// the resolution-time `Cast(what, [cost])`); `None` for a normal cast.
    ///
    /// [`begin_cast`]: GameState::begin_cast
    pub(crate) fn begin_cast_from(
        &mut self,
        object: ObjectId,
        origin: Zone,
        controller: PlayerId,
        alternative_cost: Option<deckmaste_core::Cost>,
    ) {
        match origin {
            Zone::Hand => self.remove_from_hand(self.objects.obj(object).controller, object),
            Zone::Exile => self.remove_from_exile(object),
            Zone::Graveyard => self.remove_from_graveyard(self.owner_of(object), object),
            Zone::Library => self.remove_from_library(self.owner_of(object), object),
            Zone::Battlefield => self.remove_from_battlefield(object),
            other => unreachable!("cannot begin a cast from {other:?}"),
        }
        let obj = self.objects.obj_mut(object);
        obj.zone = Some(Zone::Stack);
        // [CR#608.2g,601.2a]: the caster controls the spell it casts.
        obj.controller = controller;
        self.announcing = Some(PendingStackEntry {
            optional_components: Vec::new(),
            paid_costs: Vec::new(),
            // [CR#405]: a spell's stack identity is its own object id.
            id: object,
            object: StackObject::Spell(object),
            controller,
            origin,
            targets: vec![],
            chosen_modes: Arc::from([]),
            // [CR#601.2b]: filled by the `ChooseXValue` step before `PayCost`.
            x: None,
            // [CR#601.2b]: filled by the `ChooseCostOptions` step before `PayCost`.
            concretized: None,
            // [CR#118.9,702.35a]: the resolution-time alternative base cost, if any.
            alternative_cost,
        });
    }

    /// [CR#608.2g]: may `caster` cast `object` as a resolution-time
    /// effect grants? The effect SUPPLIES the permission, so this skips the
    /// timing ([CR#307.1]) and zone/hand gates `can_cast` enforces — but the
    /// card must still be castable at all: not a land ([CR#305.9] — lands are
    /// played, never cast), a non-empty printed mana cost ([CR#118.6]), a legal
    /// candidate for every target spec ([CR#601.2c]), and a cost the caster's
    /// pool can cover ([CR#601.2g]). This is the gate on the `May` "yes" branch
    /// for `Cast(<ref>)`: offered only when it holds, else the `if_not` branch
    /// runs ([CR#608.2g] — the empty offer defaults to "you don't").
    #[must_use]
    pub(crate) fn can_cast_as_effect(
        &self,
        caster: PlayerId,
        object: ObjectId,
        alternative_cost: Option<&deckmaste_core::Cost>,
    ) -> bool {
        // Never-crash: a stale/absent referent (the object left its zone, or
        // the anaphor resolved to nothing) is simply not castable — guard
        // before `layers()`/`confers_may_play`, both of which panic on a dead
        // id. An object already on the stack (mid-cast) is not re-castable.
        let Some(obj) = self.objects.get(object) else {
            return false;
        };
        match obj.zone {
            None | Some(Zone::Stack) => return false,
            Some(_) => {}
        }
        let view = self.layers();
        // [CR#305.9]: lands are never cast — keyed on the conferred May(Play),
        // per-face correct for an MDFC land//spell (never a `Type::Land` test).
        if crate::legal::confers_may_play(self, &view, object) {
            return false;
        }
        // [CR#118.9,702.35a]: with an alternative base cost (madness), the
        // caster pays THAT rather than the printed mana cost — the "empty
        // printed cost is unpayable" gate ([CR#118.6]) is bypassed (the
        // permission supplies a payable cost) and affordability is checked
        // against the alternative's mana. Otherwise the printed mana cost is
        // the base.
        let cost = if let Some(alt) = alternative_cost {
            alternative_cost_mana(alt)
        } else {
            // [CR#118.6]: an empty mana cost is "no mana cost" — an unpayable base.
            let face = crate::derive::face(self.def(object));
            if face.mana_cost.is_empty() {
                return false;
            }
            let Some(cost) = self.mana_cost(object) else {
                return false;
            };
            cost
        };
        // [CR#601.2c]: every target spec must have a legal candidate.
        let carrier = Some(self.objects.obj(object).source);
        let specs = crate::resolve::spell_targets(&view, object);
        let legal: Vec<Vec<ObjectId>> = specs
            .iter()
            .map(|spec| self.legal_targets(spec, carrier))
            .collect();
        if !crate::resolve::announce_satisfiable(&specs, &legal) {
            return false;
        }
        // [CR#601.2g]: the caster's pool (with the spell's own PayPips statics)
        // must be able to cover the cost.
        self.gate_mana_affordable(caster, &cost, object, Some(object))
    }

    /// [CR#608.2g]: the work-item chain that casts `object` from resolution,
    /// under `caster`'s control, from the zone `object` is currently in. Reuses
    /// the shared [CR#601.2a..601.2i] announce schedule (`BeginCast` → optional
    /// costs → X → targets → cost options → pay → the `SpellCast` becomes-cast)
    /// but WITHOUT the priority tail — "no player receives priority after it's
    /// cast" ([CR#608.2g]), so the currently-resolving ability's remaining work
    /// continues once the spell is on the stack. The opening shell is a
    /// dedicated `BeginCastFromResolution` (which records the object's live
    /// zone as the cast origin and re-controls it) rather than the
    /// hand-only `BeginCast`.
    #[must_use]
    pub(crate) fn cast_as_effect_items(
        &self,
        object: ObjectId,
        caster: PlayerId,
        alternative_cost: Option<deckmaste_core::Cost>,
    ) -> Vec<WorkItem> {
        let origin = self
            .objects
            .obj(object)
            .zone
            .expect("a castable referent is in a zone");
        Self::announce_schedule_no_priority(
            WorkItem::BeginCastFromResolution {
                object,
                origin,
                caster,
                alternative_cost,
            },
            crate::event::GameEvent::SpellCast(object),
        )
    }

    /// [CR#601.2b,602.2b,700.2]: surface the mode choice for the spell or
    /// activated ability currently being announced. This runs before X,
    /// targets, and payment. Nonmodal objects are a uniform no-op.
    ///
    /// Returns the number of authored modes (zero when no decision surfaced).
    #[must_use]
    pub(crate) fn announce_modes(&mut self) -> Uint {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        let (source, effect) = match &pending.object {
            StackObject::Spell(object) => (*object, self.spell_effect(*object)),
            StackObject::Activated {
                source, ability, ..
            } => (*source, Some(ability.effect.clone())),
            StackObject::Triggered { .. } => {
                unreachable!("triggers do not occupy the announce slot")
            }
        };
        let Some(effect) = effect else {
            // Permanent spells need no spell-effect payload to be modal.
            return 0;
        };
        let OneShotEffect::Modal(modal) = effect else {
            return 0;
        };
        let options = Uint::try_from(modal.modes.len()).expect("mode count fits Uint");
        let frame = Frame::bare(source, controller);
        let (lo, hi) = modal.choose.count.bounds();
        let lo = lo.map_or(0, |count| self.eval_count(count, &frame));
        let hi = hi.map_or(options, |count| self.eval_count(count, &frame));
        let max = if modal.choose.repeats { hi } else { hi.min(options) };
        let min = if modal.choose.up_to { 0 } else { lo.min(max) };
        let player = self.acting_player(&modal.choose.chooser, &frame);
        self.pending = Some(PendingDecision::ChooseModes(
            crate::decide::pending::ChooseModes {
                player,
                options,
                min,
                max,
                repeats: modal.choose.repeats,
                entwine: matches!(
                    modal.choose.rider,
                    Some(deckmaste_core::ModalCostRider::Entwine(_))
                ),
            },
        ));
        self.choice = Some(crate::state::ChoiceContinuation::AnnounceModes);
        options
    }

    /// [CR#601.2c]: surface a `ChooseTargets` decision if the in-flight
    /// announce targets. A spell's specs derive from its `Spell` ability; an
    /// activated ability's ride the carried text ([CR#602.2b]). Returns the
    /// number of target specs (0 = no decision surfaced).
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight, if a `Triggered` object occupies
    /// the slot (triggers announce targets at placement, [CR#603.3d]), or if
    /// the spec count overflows `Uint` — engine invariants.
    #[must_use]
    pub(crate) fn announce_targets(&mut self) -> Uint {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        if matches!(pending.object, StackObject::Triggered { .. }) {
            unreachable!("triggers announce targets at placement, not in the announce slot");
        }
        // The per-kind spec derivation is shared with re-targeting a
        // COMMITTED entry ([`Self::stack_object_target_specs`],
        // `Retarget`, [CR#707.10c]).
        let view = self.layers();
        let specs =
            self.stack_object_target_specs(&view, &pending.object, pending.chosen_modes.as_ref());
        if specs.is_empty() {
            return 0;
        }
        // The Cant(Target) filtering (hexproof, protection) and the
        // `ChooseTargets` construction live in `surface_target_choice`, shared
        // with trigger placement ([CR#603.3d]). `by` evaluates against the
        // announce's stack identity — a spell's own id, or the ability
        // identity minted when the announce opened ([CR#602.2a]) — so
        // stack-zone-keyed rows read the real object.
        let spell = pending.id;
        self.surface_target_choice(controller, specs, spell)
    }

    /// The target specs a stack object's ability carries
    /// ([CR#601.2c,603.3d]) — the per-kind derivation shared by the announce
    /// slot ([`Self::announce_targets`]) and re-targeting a COMMITTED entry
    /// (`Retarget`, [CR#707.10c]). A spell's specs are read fresh off
    /// `view` (its `Spell` ability may have changed since the object hit the
    /// stack, and a copy's controller can differ from the caster,
    /// [CR#707.10]); an activated/triggered ability's ride the text carried
    /// at promote/placement ([CR#602.2a,603.3d]) — never re-derived from a
    /// (possibly gone, possibly changed) source.
    #[must_use]
    pub(crate) fn stack_object_target_specs(
        &self,
        view: &crate::layer::LayeredView,
        object: &StackObject,
        chosen_modes: &[Uint],
    ) -> Vec<TargetSpec> {
        match object {
            StackObject::Spell(o) => view
                .get(*o)
                .abilities
                .iter()
                .find_map(|ability| match ability {
                    deckmaste_core::Ability::Spell(spell) => Some(&spell.effect),
                    _ => None,
                })
                .map_or_else(Vec::new, |effect| {
                    announced_target_specs(effect, chosen_modes)
                }),
            StackObject::Activated { ability, .. } => {
                announced_target_specs(&ability.effect, chosen_modes)
            }
            StackObject::Triggered {
                source,
                ability,
                created,
                ..
            } => {
                match created {
                    Some(t) => crate::resolve::top_targets(&t.effect).to_vec(),
                    None => {
                        let abilities = crate::derive::abilities_of_source(self, *source);
                        let other = &abilities[*ability];
                        let t = other.as_triggered().unwrap_or_else(|| {
                        panic!("a Triggered stack object indexes a Triggered ability, got {other:?}")
                    });
                        crate::resolve::top_targets(&t.effect).to_vec()
                    }
                }
            }
        }
    }

    /// [CR#601.2c]: surface a `ChooseTargets` decision for `player` over
    /// `specs`, computing each spec's legal candidates with the `Cant(Target)`
    /// carriers ([CR#702.11b] hexproof, [CR#702.16b] protection's targeted
    /// clause) excluded. `targeting_id` is the live stack identity each
    /// forbidding row's `by` filter evaluates against — a spell's own id / an
    /// ability announce's minted id ([CR#602.2a]), or a placing trigger's
    /// freshly minted stack id ([CR#603.3d]); it must be a real object, since
    /// `by` reads the targeting object's controller (hexproof's "abilities
    /// your opponents control").
    ///
    /// Returns the spec count. The surfaced decision carries the per-spec
    /// legal sets; a caller that must drop on an empty set (a targeting
    /// trigger, [CR#603.3c]) inspects them off `self.pending`.
    ///
    /// # Panics
    ///
    /// Panics if the spec count overflows `Uint` — an engine invariant.
    #[must_use]
    pub(crate) fn surface_target_choice(
        &mut self,
        player: PlayerId,
        specs: Vec<TargetSpec>,
        targeting_id: ObjectId,
    ) -> Uint {
        let legal = self.legal_targets_for_specs(&specs, targeting_id);
        let count = Uint::try_from(specs.len()).expect("target-spec count fits in Uint");
        self.pending = Some(PendingDecision::ChooseTargets(
            crate::decide::pending::ChooseTargets {
                player,
                spec: specs,
                legal,
            },
        ));
        count
    }

    /// The per-spec legal-target computation ([CR#601.2c]) shared by
    /// [`Self::surface_target_choice`] (announce / trigger placement) and
    /// re-targeting a COMMITTED entry (`Retarget`, [CR#707.10c]) —
    /// `Cant(Target)` carriers ([CR#702.11b] hexproof, [CR#702.16b]
    /// protection's targeted clause) excluded per spec. `targeting_id` is the
    /// live stack identity each forbidding row's `by` filter evaluates
    /// against; it anchors a target filter's `Ref(This)`/`StatOf(This, …)`
    /// too (Mentor's lesser-power clause, [CR#702.134a]).
    pub(crate) fn legal_targets_for_specs(
        &self,
        specs: &[TargetSpec],
        targeting_id: ObjectId,
    ) -> Vec<Vec<ObjectId>> {
        let view = self.layers();
        let rows = crate::legal::cant_target_rows(self, &view);
        let carrier = Some(self.objects.obj(targeting_id).source);
        specs
            .iter()
            .map(|s| {
                self.legal_targets(s, carrier)
                    .into_iter()
                    .filter(|&t| {
                        // Forbidden by a Cant(Target) row ([CR#702.11b] hexproof),
                        // UNLESS an AsThough overlay sees through that specific
                        // obstacle for this agent ([CR#609.4] Glaring Spotlight).
                        crate::legal::target_forbidden_by(self, &rows, targeting_id, t).is_none()
                            || crate::legal::asthough_sees_through_target(
                                self,
                                &view,
                                &rows,
                                targeting_id,
                                t,
                            )
                    })
                    .collect()
            })
            .collect()
    }

    /// [CR#601.2b]: surface a `ChooseXValue` if the in-flight announce's cost
    /// reads X — either an `{X}` mana symbol (`ManaSymbol::Variable`) or a
    /// non-mana cost verb whose count operand is `Count::X` (a loyalty `−X`'s
    /// `RemoveCounters(This, LoyaltyCounter, X)`, which carries no `{X}` mana).
    /// One announcement per activation covers both — the same chosen value
    /// binds the mana and every X-cost verb. Runs before `announce_targets`
    /// ([CR#601.2c]). No-op for an X-free cost, so the step is uniform.
    ///
    /// # Panics
    /// Panics if no announce is in flight, or a `Triggered` object occupies the
    /// slot — engine invariants.
    pub(crate) fn announce_x(&mut self) {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        let (base_has_x, effect) = match &pending.object {
            StackObject::Spell(o) => (
                pending.alternative_cost.as_ref().map_or_else(
                    || {
                        self.mana_cost(*o).is_some_and(|cost| {
                            cost.iter()
                                .any(|symbol| matches!(symbol, ManaSymbol::Variable))
                        })
                    },
                    |cost| cost_components_mention_x(cost),
                ),
                self.spell_effect(*o),
            ),
            StackObject::Activated { ability, .. } => {
                let summary = crate::activate::cost_summary(&ability.cost)
                    .expect("can_activate vetted the cost");
                // [CR#601.2b]: an `{X}` mana symbol OR a `Count::X` in any
                // cost-eligible verb (the loyalty `−X` case) triggers the
                // announcement — checked together so a cost carrying both
                // announces X exactly once.
                (
                    summary
                        .mana
                        .iter()
                        .any(|symbol| matches!(symbol, ManaSymbol::Variable))
                        || summary.verbs.iter().any(verb_mentions_cost_x)
                        || cost_components_mention_x(&ability.cost),
                    Some(ability.effect.clone()),
                )
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability never occupies the announce slot")
            }
        };
        let mode_components = effect.map_or_else(Vec::new, |effect| {
            announced_mode_cost_components(&effect, pending.chosen_modes.as_ref())
        });
        let has_x = base_has_x
            || cost_components_mention_x(&pending.optional_components)
            || cost_components_mention_x(&mode_components);
        if has_x {
            self.pending = Some(PendingDecision::ChooseXValue(
                crate::decide::pending::ChooseXValue { player: controller },
            ));
        }
    }

    /// [CR#601.2b]: concretize the in-flight cost's hybrid/Phyrexian symbols
    /// ([CR#107.4e,107.4f]). Reads the announce slot's printed mana cost — a
    /// spell's via `mana_cost`, an activated ability's via
    /// `cost_summary(&ability.cost).mana` (which already aggregates the
    /// ability's mana symbols, hybrid/Phyrexian included). Then:
    ///
    /// - No choosable symbol → stash the cost unchanged (`concretize` with no
    ///   picks is infallible here) and surface NO decision, so every plain-cost
    ///   subject behaves exactly as before with `PayCost` reading a populated
    ///   stash uniformly. Returns `false`.
    /// - Otherwise → surface `ChooseCostOptions` for the controller to announce
    ///   each nonhybrid equivalent / color-or-2-life; the submission handler
    ///   concretizes and stashes. Returns `true`.
    ///
    /// `Variable`/`{X}` is announced at [CR#601.2b] too, but X is out of scope
    /// here (engine-x-costs); `choosable` ignores it, so an X cost takes the
    /// no-decision path and its `Variable` symbol passes through unchanged.
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight, or a `Triggered` object occupies the
    /// slot — engine invariants.
    #[must_use]
    pub(crate) fn choose_cost_options(&mut self) -> bool {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        // The base mana cost, plus any extra PAYMENT components an alternative
        // base cost carries ([CR#118.9,702.35a] — madness's non-mana toll, if
        // any; empty for a mana-only madness cost).
        let (mut cost, mut alt_verbs, effect) = match &pending.object {
            StackObject::Spell(o) => match &pending.alternative_cost {
                // [CR#118.9,702.35a]: this cast pays the alternative cost RATHER
                // THAN the printed mana cost (madness's madness cost).
                Some(alt) => {
                    let (mana, components) = partition_alternative_cost(alt);
                    (mana, components, self.spell_effect(*o))
                }
                None => (
                    self.mana_cost(*o)
                        .expect("a castable spell has a printed cost"),
                    vec![],
                    self.spell_effect(*o),
                ),
            },
            StackObject::Activated {
                source, ability, ..
            } => {
                // [CR#202.1]: resolve any `ManaCostOf(reference)` against the
                // live source so the concretized cost the stash carries — and
                // thus what `pay_cost` makes the player pay — includes the
                // referenced object's mana cost ("equal to its mana cost").
                let summary = crate::activate::cost_summary(&ability.cost)
                    .expect("can_activate vetted the cost");
                let effect = ability.effect.clone();
                (
                    self.resolve_cost_mana(&summary, *source, controller),
                    vec![],
                    Some(effect),
                )
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability has no cost and never occupies the announce slot")
            }
        };
        let mode_components = effect.map_or_else(Vec::new, |effect| {
            announced_mode_cost_components(&effect, pending.chosen_modes.as_ref())
        });
        if !mode_components.is_empty() {
            let (mode_mana, mode_nonmana) =
                partition_alternative_cost(&deckmaste_core::Cost(mode_components.into()));
            let mut symbols: Vec<ManaSymbol> = cost.iter().copied().collect();
            symbols.extend(mode_mana.iter().copied());
            cost = ManaCost::from(Arc::from(symbols));
            alt_verbs.extend(mode_nonmana);
        }
        let options = crate::cost_options::choosable(&cost);
        if options.options.is_empty() {
            // [CR#601.2b]: no multi-way symbol — the cost is already concrete.
            // Stash it (plus any alternative-cost verb toll) so `PayCost` reads
            // the stash uniformly; surface nothing.
            let (mana, mut verbs) = crate::cost_options::concretize(
                &cost,
                &crate::cost_options::CostOptionChoices { picks: vec![] },
            )
            .expect("a cost with no choosable symbols needs no picks");
            verbs.extend(alt_verbs);
            self.announcing
                .as_mut()
                .expect("an announce in flight")
                .concretized = Some((mana, verbs));
            return false;
        }
        // [CR#601.2b]: the player announces each reading; the submission handler
        // concretizes and stashes it together with every already-concrete
        // nonmana component.
        self.pending = Some(PendingDecision::ChooseCostOptions(
            crate::decide::pending::ChooseCostOptions {
                player: controller,
                cost,
                additional: alt_verbs,
                options,
            },
        ));
        true
    }

    /// [CR#601.2b,702.33a]: announce the in-flight SPELL's tagged optional
    /// additional costs — the kicker family, declared as
    /// `StaticEffect::CostOption` rows on the card (they function from the
    /// stack while the spell is cast). Surfaces one `YesNo` for the
    /// `index`-th declared row (with the `OptionalCost` continuation
    /// recording the answer); returns whether a decision surfaced. `false` =
    /// past the last row / an activation / no rows — the announce walks on.
    pub(crate) fn announce_optional_costs(&mut self, index: usize) -> bool {
        let Some(pending) = self.announcing.as_ref() else {
            return false;
        };
        // Kicker is a spell-cast announcement ([CR#702.33a] "as you cast
        // this spell"); activations have no CostOption rows to walk.
        let StackObject::Spell(object) = pending.object else {
            return false;
        };
        let controller = pending.controller;
        let rows = self.cost_option_rows(object);
        let Some(option) = rows.get(index) else {
            return false;
        };
        self.pending = Some(crate::decide::PendingDecision::YesNo(
            crate::decide::pending::YesNo { player: controller },
        ));
        self.choice = Some(crate::state::ChoiceContinuation::OptionalCost {
            tag: option.tag,
            components: option.components.to_vec(),
            repeatable: option.repeatable,
            index,
        });
        true
    }

    /// The `CostOption` rows declared on `object`'s statics, in card order
    /// ([CR#702.33a,118.8b] — the kicker-family declarations).
    fn cost_option_rows(&self, object: ObjectId) -> Vec<deckmaste_core::OptionalCost> {
        crate::derive::abilities_of_source(self, self.objects.obj(object).source)
            .iter()
            .filter_map(|a| match a {
                deckmaste_core::Ability::Static(s) => Some(s.as_ref()),
                _ => None,
            })
            .filter_map(|e| match e {
                deckmaste_core::StaticEffect::CostOption(oc) => Some(oc.clone()),
                _ => None,
            })
            .collect()
    }

    /// [CR#601.2f,601.2g,601.2h]: pay the in-flight cost. Always surfaces a `PayMana`
    /// decision for any non-empty mana cost; the core never auto-pays.
    /// Auto-resolution (an Arena-style autotapper) is a future runner concern.
    /// For an activated ability ([CR#602.2b]) the cost's {T}/{Q} components
    /// are scheduled as events alongside the mana decision.
    ///
    /// The mana paid is the CONCRETIZED cost the preceding `ChooseCostOptions`
    /// step ([CR#601.2b]) stashed on the announce slot — its hybrid/Phyrexian
    /// symbols resolved to `Simple` symbols ([CR#107.4e,107.4f]). Any
    /// Phyrexian-life picks contributed `Do(LoseLife(2))` verb costs to the
    /// same stash; those are paid here (the spell branch's whole verb set;
    /// folded after the ability branch's own verb costs), via the shared
    /// `verb_payment_items`. When the concretized mana is empty but verbs
    /// remain, the verbs are still scheduled (a fully-Phyrexian-life cost).
    ///
    /// # Panics
    ///
    /// Panics if no announce is in flight, the announce slot was not
    /// concretized (the `ChooseCostOptions` step always populates it), or a
    /// `Triggered` object occupies the slot.
    pub(crate) fn pay_cost(&mut self) {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        // `announced_x` (defaulted to 0) concretizes `{X}` mana; `x_binding`
        // (the raw `Option`) is threaded onto cost-verb frames so a `Count::X`
        // verb operand reads the same announced value — `None` leaves an X-free
        // cost's verb frames exactly as before.
        let announced_x = pending.x.unwrap_or(0);
        let x_binding = pending.x;
        // [CR#601.2b]: the announced concretization — always set by the
        // preceding `ChooseCostOptions` step.
        let (mana, extra_verbs) = pending
            .concretized
            .clone()
            .expect("ChooseCostOptions concretized the cost before PayCost");
        // [CR#601.2f]: the announced optional additional components.
        let optional_components = pending.optional_components.clone();
        // The Phyrexian-life picks rode as `Do(LoseLife(2))` cost components;
        // unwrap them into payable verbs ([CR#601.2h]).
        let extra_verbs = phyrexian_life_verbs(&extra_verbs);
        match &pending.object {
            StackObject::Spell(o) => {
                let object = *o;
                // [CR#118.10]: one fresh payment id for this spell's WHOLE
                // cost payment — shared by every verb this payment schedules
                // below, so no event belongs to two payments.
                let payment = self.mint_payment();
                // [CR#601.2h]: pay the Phyrexian-life verb costs in the payment
                // window — front-scheduled so they sit behind the pending mana
                // decision (if any) and ahead of the `SpellCast` becomes-cast
                // step. The source is the spell object; the payer its
                // controller.
                let mut items =
                    verb_payment_items(&extra_verbs, object, controller, x_binding, payment);
                // [CR#601.2b]: apply the announced X to the concretized mana
                // ({X} -> Generic(announced_x); hybrid/Phyrexian already resolved).
                let mana = concretize_x(&mana, announced_x);
                // [CR#601.2f]: announced optional additional costs (kicker,
                // [CR#702.33a]) join the total — mana components into the
                // mana decision, verb components into the payment window.
                let mut mana: Vec<ManaSymbol> = mana.to_vec();
                for component in &optional_components {
                    match component {
                        CostComponent::Mana(m) => mana.extend(m.iter().copied()),
                        CostComponent::Act(pa) => {
                            items.extend(verb_payment_items(
                                &[(**pa).clone()],
                                object,
                                controller,
                                x_binding,
                                payment,
                            ));
                        }
                        other => todo!(
                            "engine-alt-costs seam: an optional-cost component \
                             beyond Mana/Do ({other:?}); owner: engine-alt-costs"
                        ),
                    }
                }
                let mana = ManaCost::from(Arc::from(mana));
                // [CR#601.2g..601.2h]: convoke/delve/improvise — offer each
                // eligible pip of the now-locked-in cost its `PayPips`
                // alternative; pips paid that way drop out of the mana decision
                // (the total cost / mana value are untouched, [CR#702.51b]). The
                // tap/exile items join the same payment window as the verbs.
                let (mana, pip_items) = self.assemble_pip_payments(object, controller, &mana);
                items.extend(pip_items);
                if !items.is_empty() {
                    self.schedule_front(items);
                }
                if !mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending =
                        Some(PendingDecision::PayMana(crate::decide::pending::PayMana {
                            player: controller,
                            cost: mana,
                            pool,
                            // [CR#106.6]: a spell's stack identity is its own id —
                            // the object SpendOnly riders judge.
                            subject: object,
                        }));
                }
                // Empty cost (no mana required): no decision surfaces, cast
                // continues (the verbs above already front-scheduled).
            }
            StackObject::Activated {
                source, ability, ..
            } => {
                let source = *source;
                let summary = crate::activate::cost_summary(&ability.cost)
                    .expect("can_activate vetted the cost");
                // [CR#118.10]: one fresh payment id for this activation's
                // WHOLE cost payment — shared by every verb/`With` step this
                // payment schedules below, so no event belongs to two
                // payments.
                let payment = self.mint_payment();
                // Costs are paid at [CR#601.2h,602.2b]: schedule the {T}/{Q}
                // events and the verb costs at the agenda FRONT — they sit
                // behind the pending mana decision (if any) and ahead of the
                // `AbilityActivated` "becomes activated" step ([CR#601.2i])
                // that `take_priority_action` queued after this `PayCost`.
                // FRONT-scheduling lands them in exactly that window: paying
                // the mana decision schedules nothing, so its continuation is
                // these items, then `AbilityActivated`.
                let mut items: Vec<WorkItem> = Vec::new();
                if summary.tap {
                    items.push(WorkItem::Emit(Occurrence::single(GameEvent::Tapped(
                        Tapped {
                            object: source,
                            cause: Some(Cause::tap(
                                Agency::CostPayment,
                                Some((source, controller)),
                            )),
                        },
                    ))));
                }
                if summary.untap {
                    items.push(WorkItem::Emit(Occurrence::single(GameEvent::Untapped(
                        source,
                        Some(Cause::untap(
                            Agency::CostPayment,
                            Some((source, controller)),
                        )),
                    ))));
                }
                // [CR#601.2h]: every cost-eligible verb (Sacrifice, LoseLife,
                // Discard, …) is performed now, by the activating player,
                // against the ability's source. One `RunEffect` per verb, after
                // the {T}/{Q} events and before `AbilityActivated`. The
                // ability's own verb costs come first, then the
                // concretization's Phyrexian-life verbs.
                items.extend(verb_payment_items(
                    &summary.verbs,
                    source,
                    controller,
                    x_binding,
                    payment,
                ));
                items.extend(verb_payment_items(
                    &extra_verbs,
                    source,
                    controller,
                    x_binding,
                    payment,
                ));
                // [CR#601.2b,601.2h]: pay each cost-side `With` choose-then-pay
                // step. Rendered as an `OneShotEffect::With` (choosing kept OUT of the
                // verb) and run over a fresh frame whose controller is the
                // activator — so the binder surfaces a `ChooseObjects` decision,
                // binds `That`/`Those`, then the body's verb pays against it,
                // exactly like the effect-side `With`. One `RunEffect` per step,
                // in the same payment window as the verb costs.
                for with in &summary.withs {
                    let effect =
                        crate::decide::unless_cost_effect(with, &deckmaste_core::Reference::You);
                    let mut frame = Frame::bare(source, controller);
                    frame.payment = Some(payment);
                    items.push(WorkItem::RunEffect {
                        effect: Arc::new(effect),
                        frame,
                    });
                }
                // [CR#601.2h,702.122a]: pay each aggregate-stat (tap-total) cost
                // by tapping a qualifying subset (Crew taps creatures with the
                // required total power). One `Tapped` event per chosen
                // permanent, in the same payment window as the {T}/verb costs.
                for req in &summary.tap_totals {
                    if let Some(subset) = self.tap_total_subset(req, source, controller) {
                        for tapped in subset {
                            items.push(WorkItem::Emit(Occurrence::single(GameEvent::Tapped(
                                Tapped {
                                    object: tapped,
                                    cause: Some(Cause::tap(
                                        Agency::CostPayment,
                                        Some((source, controller)),
                                    )),
                                },
                            ))));
                        }
                    }
                }
                if !items.is_empty() {
                    self.schedule_front(items);
                }
                // [CR#601.2b]: apply the announced X to the concretized mana
                // (hybrid/Phyrexian already resolved by ChooseCostOptions).
                let mana = concretize_x(&mana, announced_x);
                if !mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending =
                        Some(PendingDecision::PayMana(crate::decide::pending::PayMana {
                            player: controller,
                            // [CR#601.2b]: the concretized mana (hybrid/Phyrexian
                            // resolved, {X} applied), not the printed cost.
                            cost: mana,
                            pool,
                            // [CR#106.6]: an activated ability's mana is spent on
                            // its source — that is the object SpendOnly judges.
                            subject: source,
                        }));
                }
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability has no cost and never occupies the announce slot")
            }
        }
    }

    /// [CR#601.2g..601.2h]: the per-pip alternative-payment hook for a spell
    /// being cast (convoke / delve / improvise) — the EXECUTING consumer of
    /// [`StaticEffect::PayPips`]. Runs the shared [`Self::pip_coverage`] walk
    /// over the locked-in `mana` and turns each covered pip into its payment
    /// work item, returning the mana the player must still pay with real mana
    /// plus those tap/exile items for the payment window.
    ///
    /// `PayPips` is read in exactly two read-only places, both through
    /// `pip_coverage`: here (to PAY, in the casting's payment window) and the
    /// castability affordability gate ([`Self::gate_mana_affordable`], to JUDGE
    /// a cast payable before it is offered). Both are exercised only during a
    /// casting's cost handling, so the static's "functions while the spell is
    /// on the stack" lifetime ([CR#702.51a]) still needs no stack-lifetime
    /// "is it active?" predicate. The total cost and mana value ([CR#202.3])
    /// are never mutated — it "isn't an additional or alternative cost"
    /// ([CR#702.51b]) and paying this way still counts as paying the original
    /// ([CR#118.7]); only HOW each pip is paid changes.
    fn assemble_pip_payments(
        &self,
        spell: ObjectId,
        controller: PlayerId,
        mana: &ManaCost,
    ) -> (ManaCost, Vec<WorkItem>) {
        let (mana, covered) = self.pip_coverage(spell, mana);
        // Each covered pip's chosen resource becomes its payment work item —
        // a `Tapped` event or a move to exile ([CR#601.2h]).
        let items = covered
            .into_iter()
            .map(|(act, resource)| self.pip_payment_item(&act, resource, spell, controller))
            .collect();
        (mana, items)
    }

    /// The read-only core of the per-pip alternative-payment walk, shared by
    /// the payment window ([`assemble_pip_payments`]) and the castability
    /// affordability gate ([`gate_mana_affordable`]) so both agree, by the SAME
    /// logic, on which pips of `mana` a spell's `PayPips` statics cover
    /// ([CR#601.2g..601.2h]). Gathers the spell's `PayPips` statics, walks the
    /// locked-in `mana`'s pips, and for each eligible pip with an available
    /// resource covers it that way ([CR#702.51a] "rather than pay that mana"):
    /// tapping a permanent ([CR#107.5]) or exiling a graveyard card
    /// ([CR#702.66a]). Returns the mana that must still be paid with real mana
    /// (covered pips removed) plus the `(action, resource)` pair covering each
    /// pip — each resource spent at most once. The gate discards the pairs and
    /// prices the remaining mana; the payment window turns each pair into a
    /// work item. The total cost and mana value ([CR#202.3]) are never
    /// mutated — it "isn't an additional or alternative cost"
    /// ([CR#702.51b]) and paying this way still counts as paying the
    /// original ([CR#118.7]); only HOW each pip is paid changes.
    ///
    /// DEFERRED — interactive picker: the payer is entitled to CHOOSE which
    /// permanent to tap / card to exile and WHICH pips to cover ([CR#601.2g]);
    /// this takes a deterministic first-eligible subset (each resource spent at
    /// most once), exactly as [`GameState::tap_total_subset`] does for Crew.
    /// Both the gate and the payment take the same subset, so a cast the
    /// gate judges affordable is always actually payable.
    fn pip_coverage(
        &self,
        spell: ObjectId,
        mana: &ManaCost,
    ) -> (ManaCost, Vec<(PayAct, ObjectId)>) {
        // The static functions while the spell is on the stack ([CR#702.51a]),
        // so it rides the spell object's own derived ability list.
        let view = self.layers();
        let mut acts: Vec<(PipClass, PayAct)> = Vec::new();
        crate::legal::for_each_static(self, &view, spell, |e| {
            if let StaticEffect::PayPips(class, act) = e {
                acts.push((*class, act.clone()));
            }
        });
        if acts.is_empty() {
            return (mana.clone(), Vec::new());
        }
        // The filter's `Ref(This)`/`Ref(You)` anchor on the spell's source.
        let watcher = self.objects.obj(spell).source;
        let mut symbols: Vec<ManaSymbol> = mana.iter().copied().collect();
        let mut used: std::collections::HashSet<ObjectId> = std::collections::HashSet::new();
        let mut covered: Vec<(PayAct, ObjectId)> = Vec::new();
        for (class, act) in &acts {
            // One static may pay several matching pips ([CR#702.51a] "for each
            // ... mana"); loop until pips or eligible resources run out.
            while let Some(idx) = pip_index(&symbols, *class) {
                let Some(resource) = self.first_pip_resource(act, watcher, &used) else {
                    break;
                };
                used.insert(resource);
                remove_one_pip(&mut symbols, idx, *class);
                covered.push((act.clone(), resource));
            }
        }
        (ManaCost::from(Arc::from(symbols)), covered)
    }

    /// The first eligible object for a [`PayAct`] alternative not already spent
    /// ([CR#601.2g]), in deterministic id order: an untapped battlefield
    /// permanent matching the filter for `TapToPay` ([CR#107.5]), or a card in
    /// a graveyard for `ExileToPay` ([CR#702.66a]). The zone guard is explicit
    /// (not left to the filter), so a permissive plugin filter still can't tap
    /// a graveyard card or exile a battlefield permanent. `watcher` anchors the
    /// filter's self-references.
    fn first_pip_resource(
        &self,
        act: &PayAct,
        watcher: ObjectSource,
        used: &std::collections::HashSet<ObjectId>,
    ) -> Option<ObjectId> {
        let (filter, zone, untapped_only) = match act {
            PayAct::TapToPay(f) => (f, Zone::Battlefield, true),
            PayAct::ExileToPay(f) => (f, Zone::Graveyard, false),
        };
        crate::target::candidates_with(self, filter, Some(watcher))
            .into_iter()
            .find(|&id| {
                !used.contains(&id)
                    && self.objects.obj(id).zone == Some(zone)
                    && (!untapped_only || !self.objects.obj(id).tapped)
            })
    }

    /// The payment work item for one pip satisfied by an alternative
    /// ([CR#601.2h]): a `Tapped` event with the cost-payment cause for
    /// `TapToPay` (convoke / improvise, [CR#702.51a,702.126a]), or a move to
    /// exile for `ExileToPay` (delve, [CR#702.66a]). Front-scheduled into the
    /// same payment window as the other cost payments.
    fn pip_payment_item(
        &self,
        act: &PayAct,
        resource: ObjectId,
        spell: ObjectId,
        controller: PlayerId,
    ) -> WorkItem {
        match act {
            PayAct::TapToPay(_) => WorkItem::Emit(Occurrence::single(GameEvent::Tapped(Tapped {
                object: resource,
                cause: Some(Cause::tap(Agency::CostPayment, Some((spell, controller)))),
            }))),
            PayAct::ExileToPay(_) => WorkItem::Emit(Occurrence::single(
                self.relocate_from_current(resource, Zone::Exile, None),
            )),
        }
    }

    /// The card face's mana cost ([CR#202]) with the [CR#601.2f]
    /// cost-modification pipeline applied (see [`Self::modified_mana_cost`]).
    /// `None` would mark an uncastable object; every card face carries a
    /// (possibly empty) cost, so this is always `Some` today — the option
    /// leaves room for future faces/zones that have no castable cost (and
    /// lets `can_cast`/`pay_cost` share the `let Some(cost) = …` gate).
    #[must_use]
    pub fn mana_cost(&self, object: ObjectId) -> Option<ManaCost> {
        Some(self.modified_mana_cost(object))
    }

    /// The [CR#601.2f] cost-modification pipeline: the printed mana cost with
    /// every applicable `CostModifier` row applied — the spell's OWN rows
    /// (affinity's `of: Ref(This)`, [CR#702.41a]) plus battlefield statics
    /// whose `of` admits this spell (sphere taxers and reducers). Increases
    /// apply before reductions ([CR#601.2f]); the generic component floors at
    /// zero. Modifying the cost never changes the mana cost itself
    /// ([CR#118.7]): mana-value reads keep going to the printed face.
    fn modified_mana_cost(&self, object: ObjectId) -> ManaCost {
        let printed = crate::derive::face(self.def(object)).mana_cost.clone();
        let rows = self.cost_modifier_rows(object);
        if rows.is_empty() {
            return printed;
        }
        let mut cost: Vec<ManaSymbol> = printed.to_vec();
        for (frame, change) in &rows {
            self.apply_cost_change(&mut cost, change, frame, ChangePhase::Raise, 1);
        }
        for (frame, change) in &rows {
            self.apply_cost_change(&mut cost, change, frame, ChangePhase::Lower, 1);
        }
        ManaCost::from(Arc::from(cost))
    }

    /// Every applicable `CostModifier` row for casting `object`, each with a
    /// frame anchored to the ROW's carrier — `Ref(This)`, `You`, and a
    /// `Scaled` count all read relative to the ability's own carrier, whether
    /// that is the spell itself (affinity) or a battlefield permanent (a
    /// sphere taxer).
    fn cost_modifier_rows(&self, object: ObjectId) -> Vec<(Frame, deckmaste_core::CostChange)> {
        use std::ops::ControlFlow;
        let mut rows: Vec<(Frame, deckmaste_core::CostChange)> = Vec::new();
        // The spell's own rows (the card being cast is not on the battlefield,
        // so the statics walk below never sees it).
        let source = self.objects.obj(object).source;
        let abilities = crate::derive::abilities_of_source(self, source);
        let _ = crate::legal::walk_abilities(
            &abilities,
            // Presence scan of the spell's own rows: look through `Conditionally`
            // unconditionally (a cost-modifier read, not a live-condition gate).
            &mut |_: &deckmaste_core::Condition| true,
            &mut |e| {
                if let deckmaste_core::StaticEffect::CostModifier { of, change } = e
                    && self.filter_matches_live(of, object, source)
                {
                    rows.push((
                        Frame::bare(object, self.objects.obj(object).controller),
                        change.clone(),
                    ));
                }
                ControlFlow::<()>::Continue(())
            },
        );
        // Battlefield rows.
        let view = self.layers();
        for &id in &self.zones.battlefield {
            crate::legal::for_each_static(self, &view, id, |e| {
                if let deckmaste_core::StaticEffect::CostModifier { of, change } = e
                    && self.filter_matches_live(of, object, self.objects.obj(id).source)
                {
                    rows.push((
                        Frame::bare(id, self.objects.obj(id).controller),
                        change.clone(),
                    ));
                }
            });
        }
        // Rows granted by resolved one-shots ([CR#611.2c] instance rows). Each
        // is self-filtered (`of` is a spell predicate); anchor `Ref(This)`/`You`
        // /`Scaled` on the instance controller's player proxy — the row has no
        // battlefield carrier of its own.
        for ce in &self.continuous {
            let carrier = self.player(ce.controller).object;
            let source = self.objects.obj(carrier).source;
            for row in &ce.rows {
                if let deckmaste_core::StaticEffect::CostModifier { of, change } = row
                    && self.filter_matches_live(of, object, source)
                {
                    rows.push((Frame::bare(carrier, ce.controller), change.clone()));
                }
            }
        }
        rows
    }

    /// Apply one `CostChange` to `cost` for the given phase — increases (and
    /// mandatory additional mana) on `Raise`, reductions on `Lower` — with
    /// `times` scaling from any enclosing `Scaled` ([CR#601.2f]).
    /// The optional kicker-family shape ([CR#118.8b]) is a declared
    /// `StaticEffect::CostOption` — the [CR#601.2b] announce family — and
    /// stays inert here until its announce machinery lands
    /// (core-alt-costs/engine-alt-costs), exactly as before this pipeline.
    fn apply_cost_change(
        &self,
        cost: &mut Vec<ManaSymbol>,
        change: &deckmaste_core::CostChange,
        frame: &Frame,
        phase: ChangePhase,
        times: Uint,
    ) {
        use deckmaste_core::CostChange;
        match change {
            CostChange::Increase(components) if phase == ChangePhase::Raise => {
                for _ in 0..times {
                    add_mana_components(cost, components);
                }
            }
            CostChange::Reduce(components) if phase == ChangePhase::Lower => {
                for _ in 0..times {
                    reduce_mana_components(cost, components);
                }
            }
            CostChange::Additional { components } if phase == ChangePhase::Raise => {
                for _ in 0..times {
                    add_mana_components(cost, components);
                }
            }
            CostChange::Scaled {
                change,
                times: count,
            } => {
                let n = self.eval_count(count, frame);
                if n > 0 {
                    self.apply_cost_change(cost, change, frame, phase, times.saturating_mul(n));
                }
            }
            // The other phase's changes.
            _ => {}
        }
    }

    /// [CR#115]: the legal candidates for a single `TargetSpec` (its filter's
    /// matching objects, in id order). `carrier` is the targeting object's
    /// `ObjectSource` (the spell, or the source of an activated/triggered
    /// ability), anchoring a target filter's carrier-relative self-references
    /// (`Ref(This)`, and the `StatOf(This, …)` a `Predicate::Where` reaches) —
    /// e.g. Mentor's "attacking creature with power less than this
    /// creature's power" ([CR#702.134a]). Filters that never reference the
    /// carrier ignore it.
    ///
    /// Delegates filter extraction to `resolve::target_spec_filter` so that
    /// announce-time and resolution-time `TargetSpec` handling stay in sync.
    /// That helper is a TOTAL match over all three `TargetSpec` variants
    /// (`Distinct` peels to its inner `Target`'s predicate, `Expanded` to the
    /// remembered invocation's), so this function is panic-free — the former
    /// "panics on variants other than Target or Expanded" note described a
    /// partial match that no longer exists.
    #[must_use]
    pub(crate) fn legal_targets(
        &self,
        spec: &TargetSpec,
        carrier: Option<crate::object::ObjectSource>,
    ) -> Vec<ObjectId> {
        let filter = crate::resolve::target_spec_filter(spec);
        crate::target::candidates_with(self, filter, carrier)
    }

    /// Auto-tap the in-flight `PayMana` decision ([CR#601.2g,106.6]), honoring
    /// the subject's spend restrictions — only units spendable on the `PayMana`
    /// subject are eligible.
    ///
    /// # Panics
    ///
    /// Panics if the pending decision is not `PayMana`.
    #[must_use]
    pub fn auto_pay_pending(&self) -> Payment {
        match &self.pending {
            Some(PendingDecision::PayMana(crate::decide::pending::PayMana {
                cost,
                pool,
                subject,
                ..
            })) => {
                let mask: Vec<bool> = pool
                    .units()
                    .iter()
                    .map(|u| self.unit_spendable_on(u, *subject))
                    .collect();
                auto_pay_spendable(pool, cost, &mask)
            }
            other => panic!("auto_pay_pending called without a PayMana decision: {other:?}"),
        }
    }

    /// [CR#106.6]: may `unit` pay for `subject`? True unless a `SpendOnly`
    /// rider's filter rejects the object being paid for. Other rider kinds
    /// (`GrantOnSpend`/`TriggerOnSpend`/`Persistent`/`Expanded`) don't restrict
    /// spending.
    ///
    /// The watcher anchor is the subject's own `ObjectSource`: a `SpendOnly`
    /// filter today is object-shaped ("creature spell", "noncreature spell"),
    /// so it never reads the rider's grantor. A *relative* `SpendOnly` (a
    /// "your" reference back to the mana's producer — "spend only on a spell
    /// YOU cast") would need the producing source threaded onto the unit; that
    /// is a seam (riders carry no grantor today).
    pub(crate) fn unit_spendable_on(
        &self,
        unit: &crate::player::ManaUnit,
        subject: ObjectId,
    ) -> bool {
        let watcher = self.objects.obj(subject).source;
        unit.riders.iter().all(|r| match r {
            deckmaste_core::ManaRider::SpendOnly(f) => {
                self.filter_matches_live(f, subject, watcher)
            }
            _ => true,
        })
    }

    /// [CR#601.2g,106.6]: full payment validity — the structural coverage check
    /// ([`validate_payment`]) layered with spendability: every selected unit
    /// must be spendable on `subject`.
    #[must_use]
    pub(crate) fn validate_spendable(
        &self,
        player: PlayerId,
        cost: &ManaCost,
        payment: &Payment,
        subject: ObjectId,
    ) -> bool {
        let pool = &self.player(player).mana_pool;
        validate_payment(pool, cost, payment)
            && payment.units.iter().all(|&i| {
                pool.units()
                    .get(i)
                    .is_some_and(|u| self.unit_spendable_on(u, subject))
            })
    }

    /// [CR#106.6]: a clone of `player`'s pool holding only the units spendable
    /// on `subject` — the sub-pool an affordability check (`can_pay`) runs over
    /// so a spend-restricted unit can't fund an object it forbids.
    #[must_use]
    pub(crate) fn spendable_pool(&self, player: PlayerId, subject: ObjectId) -> ManaPool {
        let units = self
            .player(player)
            .mana_pool
            .units()
            .iter()
            .filter(|u| self.unit_spendable_on(u, subject))
            .cloned()
            .collect();
        ManaPool::from_units(units)
    }

    /// [CR#733.1,733.2]: reverse an in-flight announce whose announced cost can't
    /// be paid. A spell returns to its origin zone; an activated ability's
    /// minted stack identity is discarded (the source is untouched). No
    /// triggers fire (none were queued — targets are chosen after X), and
    /// the caster keeps priority. Drains this cast's continuation, still
    /// contiguous at the agenda front (`take_priority_action` pushed the
    /// whole block onto an empty agenda; no priority is held mid-announce),
    /// then reopens priority.
    ///
    /// # Panics
    /// Panics if no announce is in flight.
    pub(crate) fn rewind_announce(&mut self) {
        let pending = self.announcing.take().expect("an announce to rewind");
        match &pending.object {
            StackObject::Spell(o) => {
                let object = *o;
                self.objects.obj_mut(object).zone = Some(pending.origin);
                self.zones.hands[pending.controller.index()].push(object);
            }
            StackObject::Activated { .. } => {
                // The id begin_activate minted was never committed to the stack.
                self.objects.remove(pending.id);
            }
            StackObject::Triggered { .. } => {
                unreachable!("triggers never occupy the announce slot")
            }
        }
        while let Some(item) = self.agenda.pop_front() {
            debug_assert!(
                matches!(
                    item,
                    WorkItem::AnnounceTargets
                        | WorkItem::ChooseCostOptions
                        | WorkItem::PayCost
                        | WorkItem::Emit(_)
                        | WorkItem::CheckSbas
                        | WorkItem::PlaceTriggers
                        | WorkItem::OpenPriority
                ),
                "rewind drained an unexpected agenda item: {item:?}"
            );
        }
        self.schedule_front(vec![WorkItem::OpenPriority]);
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Color;

    use super::*;

    fn pool(pairs: &[(ColorOrColorless, Uint)]) -> ManaPool {
        let mut p = ManaPool::default();
        for &(m, n) in pairs {
            p.add(m, n, crate::player::ManaProvenance::default());
        }
        p
    }
    /// A pool whose units each carry `ManaRider::Snow` ([CR#107.4h]) — produced
    /// by a snow source, so eligible to pay `{S}` (and, being otherwise normal
    /// mana, colored/generic too).
    fn snow_pool(pairs: &[(ColorOrColorless, Uint)]) -> ManaPool {
        let mut pool = ManaPool::default();
        for &(mana, amount) in pairs {
            pool.add_riders(
                mana,
                amount,
                &[deckmaste_core::ManaRider::Snow],
                crate::player::ManaProvenance::default(),
            );
        }
        pool
    }
    fn unit(
        id: u64,
        kind: ColorOrColorless,
        riders: Vec<deckmaste_core::ManaRider>,
    ) -> crate::player::ManaUnit {
        crate::player::ManaUnit {
            id: crate::player::FloatingManaId(id),
            kind,
            riders,
            provenance: crate::player::ManaProvenance::default(),
        }
    }
    fn cost(s: &str) -> ManaCost {
        s.parse().unwrap()
    }
    fn red() -> ColorOrColorless {
        Color::Red.into()
    }
    fn green() -> ColorOrColorless {
        Color::Green.into()
    }

    #[test]
    fn colored_pip_needs_its_color() {
        assert!(can_pay(&pool(&[(red(), 1)]), &cost("{R}")));
        assert!(!can_pay(&pool(&[(green(), 1)]), &cost("{R}")));
        assert!(!can_pay(&ManaPool::default(), &cost("{R}")));
    }

    #[test]
    fn generic_pays_from_any_leftover() {
        assert!(can_pay(&pool(&[(green(), 2)]), &cost("{1}{G}"))); // G pays {G}, G pays {1}
        assert!(!can_pay(&pool(&[(green(), 1)]), &cost("{1}{G}"))); // nothing left for {1}
        assert!(can_pay(&pool(&[(green(), 1), (red(), 1)]), &cost("{1}{G}")));
    }

    #[test]
    fn validate_payment_selects_units() {
        // Pool [G, R] (indices 0, 1) against {1}{G}: covering selection valid.
        let p = pool(&[(green(), 1), (red(), 1)]);
        assert!(validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0, 1] }
        ));
        // Too few units (mana value is 2, only one selected).
        assert!(!validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0] }
        ));
        // Too many units (over-spend).
        assert!(!validate_payment(
            &pool(&[(green(), 1), (red(), 2)]),
            &cost("{1}{G}"),
            &Payment {
                units: vec![0, 1, 2]
            }
        ));
        // Out-of-range index.
        assert!(!validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0, 9] }
        ));
        // Duplicate index (would select the same unit twice).
        assert!(!validate_payment(
            &p,
            &cost("{1}{G}"),
            &Payment { units: vec![0, 0] }
        ));
        // The colored {G} need is unmet: selecting two reds for {1}{G}.
        assert!(!validate_payment(
            &pool(&[(red(), 2)]),
            &cost("{1}{G}"),
            &Payment { units: vec![0, 1] }
        ));
    }

    #[test]
    fn validate_and_apply_round_trip() {
        let mut p = pool(&[(green(), 1), (red(), 1)]); // 0=G, 1=R
        let pay = Payment { units: vec![1, 0] }; // {1}<-R(1), {G}<-G(0)
        assert!(validate_payment(&p, &cost("{1}{G}"), &pay));
        apply_payment(&mut p, &pay);
        assert!(p.is_empty());
    }

    #[test]
    fn auto_pay_covers_colored_then_generic() {
        // Pool [G, G, R] (0,1,2), cost {1}{G}: both pips are covered by the two
        // greens, leaving the red unused. The exact bipartite matcher pairs the
        // {G} pip with unit 1 and the {1} pip with unit 0 (an augmenting-path
        // assignment) — a valid covering selection of the SET {0,1}; the red
        // (index 2) is never chosen. (`pay.units` is in pip order: [{G}, {1}].)
        let p = pool(&[(green(), 2), (red(), 1)]);
        let pay = auto_pay(&p, &cost("{1}{G}"));
        assert_eq!(pay.units, vec![1, 0]);
        assert!(!pay.units.contains(&2)); // the red is never spent
        assert!(validate_payment(&p, &cost("{1}{G}"), &pay));
    }

    #[test]
    fn concretize_x_substitutes_variable_with_generic() {
        // {X}{R} at X=3 -> {3}{R}; X=0 -> {0}{R}; a cost with no X is unchanged.
        assert_eq!(concretize_x(&cost("{X}{R}"), 3), cost("{3}{R}"));
        assert_eq!(concretize_x(&cost("{X}{R}"), 0), cost("{0}{R}"));
        assert_eq!(concretize_x(&cost("{1}{G}"), 5), cost("{1}{G}"));
    }

    #[test]
    fn snow_pip_needs_a_snow_source() {
        // {S} ([CR#107.4h]): one mana of any type, but from a snow source.
        assert!(can_pay(&snow_pool(&[(red(), 1)]), &cost("{S}")));
        // Plain (non-snow) mana cannot pay {S}.
        assert!(!can_pay(&pool(&[(red(), 1)]), &cost("{S}")));
        assert!(!can_pay(&ManaPool::default(), &cost("{S}")));
    }

    #[test]
    fn snow_and_generic_mix() {
        // One snow + one plain unit pays {1}{S}: snow -> {S}, plain -> {1}.
        let units = vec![
            unit(0, red(), vec![deckmaste_core::ManaRider::Snow]),
            unit(1, green(), vec![]),
        ];
        assert!(can_pay(&ManaPool::from_units(units), &cost("{1}{S}")));
        // A single plain unit can't pay {1}{S}: no snow source for {S}.
        assert!(!can_pay(&pool(&[(green(), 1)]), &cost("{1}{S}")));
        // A single snow unit can't pay {1}{S}: only one unit, mana value is 2.
        assert!(!can_pay(&snow_pool(&[(red(), 1)]), &cost("{1}{S}")));
    }

    #[test]
    fn snow_does_not_help_a_second_snow_pip() {
        // ONE snow unit cannot pay {S}{S}: each {S} needs its own snow unit.
        assert!(!can_pay(&snow_pool(&[(red(), 1)]), &cost("{S}{S}")));
        // Two snow units do.
        assert!(can_pay(
            &snow_pool(&[(red(), 1), (green(), 1)]),
            &cost("{S}{S}")
        ));
    }

    #[test]
    fn colored_and_snow_interaction() {
        // [plain green, snow green] pays {G}{S}: plain-green -> {G}, snow-green
        // -> {S}. The snow unit is the ONLY one that can cover {S}, so {G} must
        // take the plain green — a correct matcher finds this.
        let units = vec![
            unit(0, green(), vec![]),
            unit(1, green(), vec![deckmaste_core::ManaRider::Snow]),
        ];
        assert!(can_pay(&ManaPool::from_units(units), &cost("{G}{S}")));
        // ONE snow green alone canNOT pay {G}{S}: needs two units (one per pip).
        assert!(!can_pay(&snow_pool(&[(green(), 1)]), &cost("{G}{S}")));
    }

    #[test]
    fn validate_payment_snow_round_trips() {
        // Pool [snow-red (0), plain green (1)] against {1}{S}.
        let p = ManaPool::from_units(vec![
            unit(0, red(), vec![deckmaste_core::ManaRider::Snow]),
            unit(1, green(), vec![]),
        ]);
        // snow-red -> {S}, plain green -> {1}: a correct selection.
        assert!(validate_payment(
            &p,
            &cost("{1}{S}"),
            &Payment { units: vec![0, 1] }
        ));
        // A plain (non-snow) unit can never pay an {S} pip: pool [plain green]
        // for {S} is rejected.
        let plain = pool(&[(green(), 1)]);
        assert!(!validate_payment(
            &plain,
            &cost("{S}"),
            &Payment { units: vec![0] }
        ));
        // Pool [plain green (0), snow green (1)] for {G}{S}: selecting {0,1} is
        // valid (plain->{G}, snow->{S}); but if the only snow unit is index 1
        // and we instead try to pay {S} with index 0 alone it fails on count
        // anyway — the discriminating case is that a payment naming only the
        // plain unit for an {S}-bearing cost is rejected.
        let mixed = ManaPool::from_units(vec![
            unit(0, green(), vec![]),
            unit(1, green(), vec![deckmaste_core::ManaRider::Snow]),
        ]);
        assert!(validate_payment(
            &mixed,
            &cost("{G}{S}"),
            &Payment { units: vec![0, 1] }
        ));
    }

    #[test]
    fn auto_pay_prefers_non_snow_for_generic() {
        // Pool [plain green (0), snow green (1)] for {1}{S}: the snow unit must
        // go to {S}, so {1} takes the plain green. Auto-pay reserves snow for
        // {S}.
        let p = ManaPool::from_units(vec![
            unit(0, green(), vec![]),
            unit(1, green(), vec![deckmaste_core::ManaRider::Snow]),
        ]);
        let pay = auto_pay(&p, &cost("{1}{S}"));
        assert!(validate_payment(&p, &cost("{1}{S}"), &pay));
        // The snow unit (index 1) must be among the chosen, paired with {S};
        // and the plain green (index 0) must be chosen for {1}.
        assert!(pay.units.contains(&1));
        assert!(pay.units.contains(&0));
        // Same for {G}{S}: plain green -> {G}, snow green -> {S}.
        let pay = auto_pay(&p, &cost("{G}{S}"));
        assert!(validate_payment(&p, &cost("{G}{S}"), &pay));
        assert!(pay.units.contains(&0) && pay.units.contains(&1));
    }

    // ===== The [CR#601.2f] cost-modification pipeline =====

    use std::sync::Arc;

    use deckmaste_card::Card;
    use deckmaste_card::CardFace;
    use deckmaste_core::Ability;
    use deckmaste_core::CostChange;
    use deckmaste_core::Count;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::StaticEffect;
    use deckmaste_core::Type;
    use deckmaste_core::Zone;
    use deckmaste_plugin::plugin::Plugin;

    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    /// A two-player game with no permanents (builtin Forests as decks).
    fn cm_game() -> GameState {
        let builtin = Plugin::load(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"),
        )
        .unwrap();
        let forest = Arc::new(builtin.card("Forest").unwrap().core);
        GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            conferral_rules: vec![],
            damage_result_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        })
    }

    /// Mint `card` for `controller` straight into `zone`.
    fn put_synthetic(
        state: &mut GameState,
        card: Card,
        controller: PlayerId,
        zone: Zone,
    ) -> crate::object::ObjectId {
        let card_id = state.cards.push(Arc::new(card), controller);
        let id = state
            .objects
            .mint(ObjectSource::Card(card_id), controller, Some(zone));
        match zone {
            Zone::Battlefield => state.zones.battlefield.push(id),
            Zone::Hand => state.zones.hands[controller.index()].push(id),
            Zone::Stack => {}
            other => panic!("unsupported test zone {other:?}"),
        }
        id
    }

    #[test]
    fn modal_modes_lock_before_x_targets_and_payment() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;
        use deckmaste_core::Targeted;

        let target = TargetSpec::Target(Quantity::one(), Predicate::r#type(Type::Creature));
        let card = Card::Normal(CardFace {
            name: "Modal announcement fixture".into(),
            mana_cost: "{0}".parse().unwrap(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                effect: OneShotEffect::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::You,
                        rider: None,
                    },
                    modes: vec![
                        deckmaste_core::Mode {
                            effect: OneShotEffect::Act(CoreAction::ChangeLife(
                                Reference::You,
                                deckmaste_core::LifeOp::Up(Count::Literal(1)),
                            )),
                            cost: None,
                        },
                        Mode {
                            effect: OneShotEffect::Targeted(Targeted::new(
                                vec![target.clone()].into(),
                                OneShotEffect::Act(CoreAction::destroy(Reference::Target(0))),
                            )),
                            cost: None,
                        },
                    ]
                    .into(),
                }),
            })],
            ..CardFace::default()
        });
        let mut state = cm_game();
        let spell = put_synthetic(&mut state, card, PlayerId(0), Zone::Hand);

        state.begin_cast(spell);
        assert!(state.announcing.as_ref().unwrap().chosen_modes.is_empty());
        assert_eq!(state.announce_modes(), 2);
        assert!(matches!(
            state.pending,
            Some(PendingDecision::ChooseModes(_))
        ));

        state
            .submit_decision(crate::decide::Decision::Modes(vec![1]))
            .unwrap();
        assert_eq!(
            state.announcing.as_ref().unwrap().chosen_modes.as_ref(),
            &[1]
        );

        assert_eq!(state.announce_targets(), 1);
        let Some(PendingDecision::ChooseTargets(choice)) = &state.pending else {
            panic!("the selected targeted mode should announce its target");
        };
        assert_eq!(choice.spec, vec![target]);
    }

    #[test]
    fn announced_modal_modes_resolve_without_reopening_and_keep_target_scopes() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;
        use deckmaste_core::Targeted;

        let life_mode = |amount| Mode {
            effect: OneShotEffect::Targeted(Targeted::new(
                vec![TargetSpec::Target(
                    Quantity::one(),
                    Predicate::Kind(deckmaste_core::ObjectKind::Player),
                )]
                .into(),
                OneShotEffect::Act(CoreAction::ChangeLife(
                    Reference::Target(0),
                    deckmaste_core::LifeOp::Up(Count::Literal(amount)),
                )),
            )),
            cost: None,
        };
        let card = Card::Normal(CardFace {
            name: "Modal resolution fixture".into(),
            mana_cost: "{0}".parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                effect: OneShotEffect::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::You,
                        rider: None,
                    },
                    modes: vec![life_mode(3), life_mode(5)].into(),
                }),
            })],
            ..CardFace::default()
        });
        let mut state = cm_game();
        let p0 = PlayerId(0);
        let p1 = PlayerId(1);
        let p0_object = state.player(p0).object;
        let p1_object = state.player(p1).object;
        let p0_life = state.player(p0).life;
        let p1_life = state.player(p1).life;
        let spell = put_synthetic(&mut state, card, p0, Zone::Stack);
        state.stack.push(crate::stack::StackEntry {
            id: spell,
            object: StackObject::Spell(spell),
            controller: p0,
            targets: vec![vec![p0_object], vec![p1_object]],
            chosen_modes: Arc::from([0, 1]),
            x: None,
            paid_costs: Vec::new(),
            copy: false,
        });
        assert!(
            state.targets_still_legal(&state.stack[0]),
            "both announced player targets begin legal"
        );

        state.resolve_object(spell);
        for _ in 0..5 {
            let outcome = state.step();
            assert!(
                !matches!(outcome, crate::step::StepOutcome::NeedsDecision(_)),
                "announced modes must not reopen at resolution: {outcome:?}"
            );
        }
        assert_eq!(state.player(p0).life, p0_life + 3);
        assert_eq!(state.player(p1).life, p1_life + 5);
    }

    #[test]
    fn only_selected_modes_contribute_to_the_locked_cost() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;

        let mode = |generic| Mode {
            effect: OneShotEffect::Act(CoreAction::ChangeLife(
                Reference::You,
                deckmaste_core::LifeOp::Up(Count::Literal(1)),
            )),
            cost: Some(
                vec![CostComponent::Mana(
                    format!("{{{generic}}}").parse().unwrap(),
                )]
                .into(),
            ),
        };
        let card = Card::Normal(CardFace {
            name: "Modal cost fixture".into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                effect: OneShotEffect::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::You,
                        rider: None,
                    },
                    modes: vec![mode(1), mode(2)].into(),
                }),
            })],
            ..CardFace::default()
        });
        let mut state = cm_game();
        let spell = put_synthetic(&mut state, card, PlayerId(0), Zone::Hand);

        state.begin_cast(spell);
        let _ = state.announce_modes();
        state
            .submit_decision(crate::decide::Decision::Modes(vec![1]))
            .unwrap();
        assert!(!state.choose_cost_options());
        let (mana, _) = state
            .announcing
            .as_ref()
            .unwrap()
            .concretized
            .as_ref()
            .unwrap();
        assert_eq!(mana, &"{1}{2}".parse().unwrap());
    }

    #[test]
    fn entwine_all_modes_is_announced_and_added_to_the_cost() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Cost;
        use deckmaste_core::Modal;
        use deckmaste_core::ModalCostRider;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;

        let mode = || Mode {
            effect: OneShotEffect::Act(CoreAction::ChangeLife(
                Reference::You,
                deckmaste_core::LifeOp::Up(Count::Literal(1)),
            )),
            cost: None,
        };
        let card = Card::Normal(CardFace {
            name: "Entwine fixture".into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                effect: OneShotEffect::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::You,
                        rider: Some(ModalCostRider::Entwine(Cost(
                            vec![CostComponent::Mana("{3}".parse().unwrap())].into(),
                        ))),
                    },
                    modes: vec![mode(), mode()].into(),
                }),
            })],
            ..CardFace::default()
        });
        let mut state = cm_game();
        let spell = put_synthetic(&mut state, card, PlayerId(0), Zone::Hand);

        state.begin_cast(spell);
        let _ = state.announce_modes();
        state
            .submit_decision(crate::decide::Decision::Modes(vec![0, 1]))
            .unwrap();
        assert!(!state.choose_cost_options());
        let (mana, _) = state
            .announcing
            .as_ref()
            .unwrap()
            .concretized
            .as_ref()
            .unwrap();
        assert_eq!(mana, &"{1}{3}".parse().unwrap());
    }

    #[test]
    fn selected_mode_cost_controls_whether_x_is_announced() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;

        let mode = |mana: &str| Mode {
            effect: OneShotEffect::Act(CoreAction::ChangeLife(
                Reference::You,
                deckmaste_core::LifeOp::Up(Count::Literal(1)),
            )),
            cost: Some(vec![CostComponent::Mana(mana.parse().unwrap())].into()),
        };
        let card = || {
            Card::Normal(CardFace {
                name: "Modal X-cost fixture".into(),
                mana_cost: "{1}".parse().unwrap(),
                types: vec![Type::Sorcery.def()],
                abilities: vec![Ability::spell(SpellAbility {
                    ability_word: None,
                    effect: OneShotEffect::Modal(Modal {
                        choose: ChooseSpec {
                            count: Quantity::one(),
                            up_to: false,
                            repeats: false,
                            chooser: Reference::You,
                            rider: None,
                        },
                        modes: vec![mode("{X}"), mode("{1}")].into(),
                    }),
                })],
                ..CardFace::default()
            })
        };

        let mut without_x = cm_game();
        let spell = put_synthetic(&mut without_x, card(), PlayerId(0), Zone::Hand);
        without_x.begin_cast(spell);
        let _ = without_x.announce_modes();
        without_x
            .submit_decision(crate::decide::Decision::Modes(vec![1]))
            .unwrap();
        without_x.announce_x();
        assert!(
            without_x.pending.is_none(),
            "the unselected X cost is ignored"
        );

        let mut with_x = cm_game();
        let spell = put_synthetic(&mut with_x, card(), PlayerId(0), Zone::Hand);
        with_x.begin_cast(spell);
        let _ = with_x.announce_modes();
        with_x
            .submit_decision(crate::decide::Decision::Modes(vec![0]))
            .unwrap();
        with_x.announce_x();
        assert!(matches!(
            with_x.pending,
            Some(PendingDecision::ChooseXValue(_))
        ));
    }

    fn vanilla_artifact(name: &str) -> Card {
        Card::Normal(CardFace {
            name: name.into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Artifact.def()],
            ..CardFace::default()
        })
    }

    /// An affinity-shaped self-row ([CR#702.41a]): "costs {1} less for each
    /// battlefield artifact".
    fn affinity_card(printed: &str) -> Card {
        Card::Normal(CardFace {
            name: "Fromite".into(),
            mana_cost: printed.parse().unwrap(),
            types: vec![Type::Artifact.def()],
            abilities: vec![Ability::r#static(StaticEffect::CostModifier {
                of: Predicate::Ref(Reference::This),
                change: CostChange::Scaled {
                    change: Arc::new(CostChange::Reduce(
                        vec![CostComponent::Mana("{1}".parse().unwrap())].into(),
                    )),
                    times: Count::CountOf(deckmaste_core::Countable::Objects(Arc::new(
                        Predicate::And(
                            vec![
                                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                                Predicate::r#type(Type::Artifact),
                            ]
                            .into(),
                        ),
                    ))),
                },
            })],
            ..CardFace::default()
        })
    }

    /// Affinity's `Scaled(Reduce)` self-row lowers the generic component by
    /// the live count, flooring at `{0}` ([CR#601.2f,702.41a]).
    #[test]
    fn affinity_scaled_reduce_lowers_generic_with_floor() {
        let mut state = cm_game();
        let spell = put_synthetic(&mut state, affinity_card("{3}"), PlayerId(0), Zone::Hand);
        assert_eq!(
            state.mana_cost(spell).unwrap(),
            cost("{3}"),
            "no artifacts: printed cost"
        );
        put_synthetic(
            &mut state,
            vanilla_artifact("Trinket A"),
            PlayerId(0),
            Zone::Battlefield,
        );
        put_synthetic(
            &mut state,
            vanilla_artifact("Trinket B"),
            PlayerId(1),
            Zone::Battlefield,
        );
        assert_eq!(
            state.mana_cost(spell).unwrap(),
            cost("{1}"),
            "two artifacts: {{3}} - {{2}}"
        );
        // Floor: more reduction than pips leaves {0}, never "no cost".
        put_synthetic(
            &mut state,
            vanilla_artifact("Trinket C"),
            PlayerId(0),
            Zone::Battlefield,
        );
        put_synthetic(
            &mut state,
            vanilla_artifact("Trinket D"),
            PlayerId(0),
            Zone::Battlefield,
        );
        assert_eq!(state.mana_cost(spell).unwrap(), cost("{0}"));
    }

    /// A battlefield taxer row ("creature spells cost {1} more") raises the
    /// cast cost of a matching card in hand; a colored reduction removes only
    /// its matching pip ([CR#601.2f]).
    #[test]
    fn battlefield_taxer_raises_and_colored_reduce_removes_pip() {
        let mut state = cm_game();
        let bear = Card::Normal(CardFace {
            name: "Bear".into(),
            mana_cost: "{1}{G}".parse().unwrap(),
            types: vec![Type::Creature.def()],
            ..CardFace::default()
        });
        let spell = put_synthetic(&mut state, bear, PlayerId(0), Zone::Hand);

        let taxer = Card::Normal(CardFace {
            name: "Thorn Totem".into(),
            types: vec![Type::Artifact.def()],
            abilities: vec![Ability::r#static(StaticEffect::CostModifier {
                of: Predicate::r#type(Type::Creature),
                change: CostChange::Increase(
                    vec![CostComponent::Mana("{1}".parse().unwrap())].into(),
                ),
            })],
            ..CardFace::default()
        });
        put_synthetic(&mut state, taxer, PlayerId(1), Zone::Battlefield);
        // Increase appends: [{1}, {G}] + {1} -> [{1}, {G}, {1}].
        let expected: ManaCost = Arc::<[ManaSymbol]>::from(vec![
            ManaSymbol::Simple(SimpleManaSymbol::Generic(1)),
            deckmaste_core::Color::Green.into(),
            ManaSymbol::Simple(SimpleManaSymbol::Generic(1)),
        ])
        .into();
        assert_eq!(state.mana_cost(spell).unwrap(), expected);
    }

    /// The pure reduction arithmetic: a colored pip removes one matching pip
    /// and nothing else; generic reduction spreads across generic pips.
    #[test]
    fn reduce_symbol_arithmetic() {
        let mut c: Vec<ManaSymbol> = Vec::from(&*cost("{2}{G}{G}"));
        reduce_symbol(&mut c, deckmaste_core::Color::Green.into());
        assert_eq!(ManaCost::from(Arc::from(c.clone())), {
            let v: Vec<ManaSymbol> = vec![
                ManaSymbol::Simple(SimpleManaSymbol::Generic(2)),
                deckmaste_core::Color::Green.into(),
            ];
            ManaCost::from(Arc::from(v))
        });
        reduce_symbol(&mut c, ManaSymbol::Simple(SimpleManaSymbol::Generic(1)));
        let v: Vec<ManaSymbol> = vec![
            ManaSymbol::Simple(SimpleManaSymbol::Generic(1)),
            deckmaste_core::Color::Green.into(),
        ];
        assert_eq!(ManaCost::from(Arc::from(c)), ManaCost::from(Arc::from(v)));
    }

    // ---- autotap_for_cast (runner autotap planner) ----

    /// A land whose only ability is `{T}: Add one <color>` — the fixed-specific
    /// mana shape `tap_mana_ability` recognizes, at ability index 0.
    fn mana_land(name: &str, color: ColorOrColorless) -> Card {
        use deckmaste_core::ActivatedAbility;
        use deckmaste_core::ActivatedManaProfile;
        use deckmaste_core::Cost;
        use deckmaste_core::ManaAbility;
        use deckmaste_core::ManaSpec;
        let ability = Arc::new(ActivatedAbility {
            ability_word: None,
            cost: Cost(vec![CostComponent::Tap].into()),
            from: None,
            window: None,
            condition: None,
            limits: vec![].into(),
            effect: OneShotEffect::Act(CoreAction::AddMana(
                Reference::You,
                Count::Literal(1),
                ManaSpec::Specific(color).into(),
            )),
        });
        Card::Normal(CardFace {
            name: name.into(),
            mana_cost: ManaCost::default(),
            types: vec![Type::Land.def()],
            abilities: vec![Ability::Mana(ManaAbility::Activated {
                ability,
                profile: ActivatedManaProfile::Always,
            })],
            ..CardFace::default()
        })
    }

    /// The plugin-loaded Instant `TypeDef`: its conferred
    /// `May(Cast(window: InstantSpeed))` row ([CR#307.1,117.1a,702.8a]) is what
    /// lifts casting timing now that the `Type::Instant` literal is gone.
    /// Fixtures build it inline because `cm_game` mints synthetic cards that
    /// never pass through the plugin macro expansion that would attach the
    /// confer — a bare `Type::Instant.def()` carries EMPTY confers.
    fn instant_typedef() -> deckmaste_core::TypeDef {
        deckmaste_core::TypeDef {
            name: "Instant".into(),
            permanent: false,
            confers: vec![deckmaste_core::Property::Ability(Arc::new(
                Ability::r#static(StaticEffect::Deontic(deckmaste_core::Deontic::May(
                    deckmaste_core::DeonticAction::Cast {
                        what: Predicate::Ref(Reference::This),
                        by: Predicate::Any,
                        from: None,
                        window: Some(Timing::InstantSpeed),
                        cost: None,
                        tag: None,
                    },
                ))),
            ))]
            .into(),
        }
    }

    /// An instant carrying its conferred instant-speed casting window (see
    /// `instant_typedef`), with no targets — so `castable_cost_ignoring_mana`
    /// turns purely on cost + mana.
    fn instant(name: &str, mc: &str) -> Card {
        Card::Normal(CardFace {
            name: name.into(),
            mana_cost: mc.parse().unwrap(),
            types: vec![instant_typedef()],
            ..CardFace::default()
        })
    }

    /// A sorcery — no casting-window confer, so it is castable only at sorcery
    /// speed. `Type::Sorcery.def()` carries the correct (empty) confers.
    fn sorcery(name: &str, mc: &str) -> Card {
        Card::Normal(CardFace {
            name: name.into(),
            mana_cost: mc.parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            ..CardFace::default()
        })
    }

    /// [CR#307.1,117.1a,702.8a] casting-window brick: with the `Type::Instant`
    /// literal gone from `castable_cost_ignoring_mana`, instant-speed timing is
    /// driven ENTIRELY by the conferred `May(Cast(window: InstantSpeed))` row.
    /// `cm_game` starts in `Ending(Cleanup)` — NOT a main phase, so
    /// `sorcery_speed_ok` is false; the only way timing can pass is the confer.
    /// An Instant carrying it is castable at instant speed; a Sorcery (no
    /// confer) is not — proof the conferred DATA, not an enum literal, lifts
    /// timing.
    #[test]
    fn casting_window_reads_the_conferred_may_cast_row() {
        let mut state = cm_game();
        assert!(
            !state.sorcery_speed_ok(PlayerId(0)),
            "a fresh cm_game is at Cleanup — not sorcery speed"
        );
        let inst = put_synthetic(&mut state, instant("Bolt", "{R}"), PlayerId(0), Zone::Hand);
        let sorc = put_synthetic(
            &mut state,
            sorcery("Lava Axe", "{R}"),
            PlayerId(0),
            Zone::Hand,
        );
        let view = state.layers();
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), inst)
                .is_some(),
            "the Instant's conferred May(Cast(InstantSpeed)) row lifts timing outside sorcery speed"
        );
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), sorc)
                .is_none(),
            "a Sorcery has no instant-speed confer, so it is not castable at Cleanup"
        );
    }

    /// The plugin-loaded Land `TypeDef`: its conferred `May(Play(what:
    /// Ref(This)))` marker ([CR#305.9,116.2a,701.18]) is what makes a land
    /// "playable as a land, not castable as a spell" now that the
    /// `Type::Land` literal is gone from `castable_cost_ignoring_mana`.
    /// Built inline because synthetic cards never pass through the plugin
    /// macro expansion that would attach the confer — a bare
    /// `Type::Land.def()` carries EMPTY confers (decision 6).
    fn land_typedef() -> deckmaste_core::TypeDef {
        deckmaste_core::TypeDef {
            name: "Land".into(),
            permanent: true,
            confers: vec![deckmaste_core::Property::Ability(Arc::new(
                Ability::r#static(StaticEffect::Deontic(deckmaste_core::Deontic::May(
                    deckmaste_core::DeonticAction::Play {
                        what: Predicate::Ref(Reference::This),
                        by: Predicate::Any,
                        from: None,
                    },
                ))),
            ))]
            .into(),
        }
    }

    /// A land carrying its conferred `May(Play)` marker (see `land_typedef`),
    /// with the empty mana cost every real land has.
    fn conferred_land(name: &str) -> Card {
        Card::Normal(CardFace {
            name: name.into(),
            mana_cost: ManaCost::default(),
            types: vec![land_typedef()],
            ..CardFace::default()
        })
    }

    /// [CR#305.9,116.2a,701.18] land-play brick: with the `Type::Land` literal
    /// gone from `castable_cost_ignoring_mana`, a land's "not castable as a
    /// spell" nature is driven by the conferred `May(Play)` marker. A land
    /// carrying it is seen by `confers_may_play` and is not castable; a genuine
    /// spell face (an Instant, no `May(Play)`) is NOT caught by the land-play
    /// capability and stays castable — the MDFC land//spell split, per face.
    #[test]
    fn land_play_reads_the_conferred_may_play_marker() {
        let mut state = cm_game();
        let land = put_synthetic(
            &mut state,
            conferred_land("Mountain"),
            PlayerId(0),
            Zone::Hand,
        );
        let inst = put_synthetic(&mut state, instant("Bolt", "{R}"), PlayerId(0), Zone::Hand);
        let view = state.layers();
        // The land confers May(Play) and is never castable as a spell.
        assert!(
            crate::legal::confers_may_play(&state, &view, land),
            "a Land's conferred May(Play) marker is seen ([CR#701.18])"
        );
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), land)
                .is_none(),
            "a land confers May(Play) → not castable as a spell ([CR#305.9])"
        );
        // A spell face carries no May(Play) → not land-playable, and still
        // castable (its own May(Cast) confer lifts timing at Cleanup).
        assert!(
            !crate::legal::confers_may_play(&state, &view, inst),
            "an Instant face has no May(Play) — the land-play gate never catches it"
        );
        assert!(
            state
                .castable_cost_ignoring_mana(&view, PlayerId(0), inst)
                .is_some(),
            "an Instant face stays castable — land-play does not block a spell face"
        );
    }

    /// The land objects a plan taps, in order.
    fn tapped(plan: &[Action]) -> Vec<crate::object::ObjectId> {
        plan.iter()
            .map(|a| match a {
                Action::ActivateAbility { object, .. } => *object,
                other => panic!("autotap planned a non-activation: {other:?}"),
            })
            .collect()
    }

    #[test]
    fn autotap_covers_a_single_colored_pip() {
        let mut state = cm_game();
        let land = put_synthetic(
            &mut state,
            mana_land("Mtn", red()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let spell = put_synthetic(&mut state, instant("Bolt", "{R}"), PlayerId(0), Zone::Hand);
        // No mana floated → the engine does not offer the cast yet.
        assert!(!state.can_cast(&state.layers(), PlayerId(0), spell));
        let plan = state
            .autotap_for_cast(PlayerId(0), spell)
            .expect("plannable");
        assert_eq!(tapped(&plan), vec![land]);
        // The plan's activations all target index 0 (the land's mana ability).
        assert!(matches!(
            plan[0],
            Action::ActivateAbility { ability: 0, .. }
        ));
    }

    #[test]
    fn autotap_covers_generic_and_colored_with_two_taps() {
        let mut state = cm_game();
        let l1 = put_synthetic(
            &mut state,
            mana_land("M1", red()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let l2 = put_synthetic(
            &mut state,
            mana_land("M2", red()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let spell = put_synthetic(
            &mut state,
            instant("Shock2", "{1}{R}"),
            PlayerId(0),
            Zone::Hand,
        );
        let plan = state
            .autotap_for_cast(PlayerId(0), spell)
            .expect("plannable");
        let mut taps = tapped(&plan);
        taps.sort();
        let mut want = vec![l1, l2];
        want.sort();
        assert_eq!(taps, want, "both red sources tapped for {{1}}{{R}}");
    }

    #[test]
    fn autotap_prefers_an_exact_colour_source_over_an_off_colour_one() {
        let mut state = cm_game();
        let _green = put_synthetic(
            &mut state,
            mana_land("Frst", green()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let mtn = put_synthetic(
            &mut state,
            mana_land("Mtn", red()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let spell = put_synthetic(&mut state, instant("Bolt", "{R}"), PlayerId(0), Zone::Hand);
        let plan = state
            .autotap_for_cast(PlayerId(0), spell)
            .expect("plannable");
        assert_eq!(
            tapped(&plan),
            vec![mtn],
            "the {{R}} pip taps the Mountain, not the Forest"
        );
    }

    #[test]
    fn autotap_gives_up_when_untapped_lands_cannot_cover_the_cost() {
        let mut state = cm_game();
        put_synthetic(
            &mut state,
            mana_land("Mtn", red()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let spell = put_synthetic(
            &mut state,
            instant("Shock2", "{1}{R}"),
            PlayerId(0),
            Zone::Hand,
        );
        // One red source can't pay {1}{R} (two mana) → no plan, no wasted taps.
        assert_eq!(state.autotap_for_cast(PlayerId(0), spell), None);
    }

    #[test]
    fn autotap_gives_up_with_no_mana_sources() {
        let mut state = cm_game();
        let spell = put_synthetic(&mut state, instant("Bolt", "{R}"), PlayerId(0), Zone::Hand);
        assert_eq!(state.autotap_for_cast(PlayerId(0), spell), None);
    }

    #[test]
    fn autotap_declines_a_variable_cost() {
        // {X}{R}: the {X} announce can't be autotapped → planner bails even
        // though a red source is present.
        let mut state = cm_game();
        put_synthetic(
            &mut state,
            mana_land("Mtn", red()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let spell = put_synthetic(
            &mut state,
            instant("XBurn", "{X}{R}"),
            PlayerId(0),
            Zone::Hand,
        );
        assert_eq!(state.autotap_for_cast(PlayerId(0), spell), None);
    }
}
