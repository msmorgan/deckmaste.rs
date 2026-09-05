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
use deckmaste_core::Instruction;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PayAct;
use deckmaste_core::PipClass;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::StaticSpec;
use deckmaste_core::TargetSpec;
use deckmaste_core::Timing;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::decide::Action;
use crate::decide::DecisionPointKind;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::event::Tapped;
use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::ManaPool;
use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
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
    // The selected units must perfectly match the pips. Equal cardinality plus
    // a perfect pip-side matching means every selected unit is also used.
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
/// resolution frame whose `controller` is the activator — so the controller
/// parameter resolves to that player and the source parameter (a
/// self-sacrifice) to the source — mirroring the frame
/// any effect node resolves against (no targets, no trigger context: a cost
/// verb names neither). A chooser inside a verb surfaces its own
/// `ChooseObjects` decision and writes its own dest register.
///
/// `x` is the value announced for this activation ([CR#601.2b]) — threaded onto
/// each cost-verb frame so a semantic X operand (a loyalty `−X`'s
/// `RemoveCounters(This, LoyaltyCounter, X)`) pays the announced amount,
/// exactly as the effect-side frame reads X. `None` when no X was announced
/// (the common no-X cost), leaving each X-free verb untouched.
///
/// `payment` ([CR#118.10]) is stamped onto every frame this call
/// mints — the caller mints ONE [`crate::stack::Payment`] per cost payment
/// (shared across every `verb_payment_items` call and cost-`With` step that
/// payment's drain schedules) so every event this payment's verbs perform
/// reads as `Agency::CostPayment` and shares one payment id.
/// The payment window's work for one ordered cost BLOCK ([CR#601.2b,601.2h]).
///
/// Every component runs against the ANNOUNCE activation, so a payment-time
/// decision writes a register the components after it — and the ability body
/// at resolution — read by index. Order is preserved: the choice comes before
/// the verb that spends it.
fn cost_step_items(
    steps: &[CostComponent],
    activation: crate::ActivationId,
    payment: crate::stack::Payment,
) -> Vec<WorkItem> {
    steps
        .iter()
        .map(|step| WorkItem::RunEffect {
            effect: Arc::new(crate::decide::unless_cost_effect(
                step,
                &deckmaste_core::Reference::controller_parameter(),
            )),
            frame: ExecutionFrame {
                activation,
                payment: Some(payment),
            },
        })
        .collect()
}

fn verb_payment_items(
    verbs: &[CoreAction],
    activation: crate::ActivationId,
    payment: crate::stack::Payment,
) -> Vec<WorkItem> {
    verbs
        .iter()
        .map(|verb| {
            let frame = ExecutionFrame {
                activation,
                payment: Some(payment),
            };
            WorkItem::RunEffect {
                effect: Arc::new(Instruction::act(verb.clone())),
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
fn verb_mentions_cost_x(verb: &CoreAction, x: deckmaste_core::RefId) -> bool {
    match verb {
        // Pay-X-life ([CR#119.4]) only — a gain/set-life cost never reads X
        // this way, mirroring the former `PlayerAction::LoseLife`-only match.
        CoreAction::ChangeLife(_, deckmaste_core::LifeOp::Down(count))
        | CoreAction::PutCounters(_, _, count)
        | CoreAction::RemoveCounters(_, _, count) => count.mentions_register(x),
        // An X-discard ("discard X cards") — the count rides the body's
        // `With` binder's `Quantity`.
        CoreAction::Composite { name, body } if name.as_str() == "Discard" => {
            deckmaste_core::discard_body_count(body).is_some_and(|count| count.mentions_register(x))
        }
        _ => false,
    }
}

/// Whether a cardinality used by a cost-side choice reads the announced X.
fn quantity_mentions_cost_x(quantity: &deckmaste_core::Quantity, x: deckmaste_core::RefId) -> bool {
    let (lower, upper) = quantity.bounds();
    lower.is_some_and(|count| count.mentions_register(x))
        || upper.is_some_and(|count| count.mentions_register(x))
}

/// Whether a runnable cost component reads the one X value announced for the
/// spell or ability. This follows nested/lowered cost structure so modal and
/// optional additions participate only after they have actually been chosen.
fn cost_component_mentions_x(component: &CostComponent, x: deckmaste_core::RefId) -> bool {
    match component {
        CostComponent::Mana(mana) => mana
            .iter()
            .any(|symbol| matches!(symbol, ManaSymbol::Variable)),
        CostComponent::Act { action, .. } => verb_mentions_cost_x(action, x),
        CostComponent::Cost(nested) => nested.iter().any(|part| cost_component_mentions_x(part, x)),
        CostComponent::TapTotal { count, .. } => count.mentions_register(x),
        // A payment-time choice's cardinality reads X ("sacrifice X
        // creatures", [CR#601.2b]).
        CostComponent::Choose(choice) => quantity_mentions_cost_x(&choice.quantity, x),
        CostComponent::Sample(sample) => quantity_mentions_cost_x(&sample.quantity, x),
        CostComponent::Search(search) => quantity_mentions_cost_x(&search.quantity, x),
        CostComponent::Let(_)
        | CostComponent::ManaCostOf(_)
        | CostComponent::Tap
        | CostComponent::Untap => false,
    }
}

fn cost_components_mention_x(components: &[CostComponent], x: deckmaste_core::RefId) -> bool {
    components
        .iter()
        .any(|component| cost_component_mentions_x(component, x))
}

/// Unwrap the `CostComponent::Do(action)` verbs `concretize` produces for
/// Phyrexian-life picks ([CR#107.4f]) back into the [`Action`]s
/// `verb_payment_items` schedules. `concretize` only ever emits
/// `Do(LoseLife(2))` here, so any other shape is an engine invariant violation.
fn phyrexian_life_verbs(verbs: &[CostComponent]) -> Vec<CoreAction> {
    verbs
        .iter()
        .map(|c| match c {
            CostComponent::Act { action, .. } => (**action).clone(),
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

/// The target declarations contributed by the already-announced modal
/// selection. Each chosen mode owns a fresh target scope, so its specs are
/// appended in choice order; a nonmodal effect contributes its ordinary
/// top-level target wrapper.
pub(crate) fn announced_target_specs(
    effect: &deckmaste_core::Region,
    targets: &[TargetSpec],
    chosen_modes: &[Uint],
) -> Vec<TargetSpec> {
    match effect.body.as_ref() {
        [Instruction::Modal(modal)] => chosen_modes
            .iter()
            .flat_map(|&index| {
                let mode = modal
                    .modes
                    .get(index as usize)
                    .expect("ChooseModes validated every announced index");
                mode.targets.iter().cloned()
            })
            .collect(),
        _ => targets.to_vec(),
    }
}

fn first_mode_selection_matching(
    options: Uint,
    min: Uint,
    max: Uint,
    repeats: bool,
    entwine: bool,
    mut accepts: impl FnMut(&[Uint]) -> bool,
) -> Option<Vec<Uint>> {
    fn choose(
        options: Uint,
        count: usize,
        repeats: bool,
        start: Uint,
        picks: &mut Vec<Uint>,
        accepts: &mut impl FnMut(&[Uint]) -> bool,
    ) -> Option<Vec<Uint>> {
        if picks.len() == count {
            return accepts(picks).then(|| picks.clone());
        }
        for mode in start..options {
            picks.push(mode);
            let next = if repeats { mode } else { mode + 1 };
            if let Some(found) = choose(options, count, repeats, next, picks, accepts) {
                return Some(found);
            }
            picks.pop();
        }
        None
    }

    if min <= max {
        for count in min..=max {
            if let Some(found) = choose(
                options,
                usize::try_from(count).expect("mode count fits usize"),
                repeats,
                0,
                &mut Vec::new(),
                &mut accepts,
            ) {
                return Some(found);
            }
        }
    }
    if entwine {
        let all: Vec<_> = (0..options).collect();
        if accepts(&all) {
            return Some(all);
        }
    }
    None
}

/// Additional cost components contributed by the selected modes and their
/// modal rider. Per-mode costs follow pick order (and repeat with a repeated
/// mode); escalate repeats once per pick beyond the first; entwine contributes
/// once when the all-modes alternative was chosen.
fn announced_mode_cost_components(
    effect: &deckmaste_core::Region,
    chosen_modes: &[Uint],
) -> Vec<CostComponent> {
    let [Instruction::Modal(modal)] = effect.body.as_ref() else {
        return Vec::new();
    };
    let mut components = Vec::new();
    for &index in chosen_modes {
        components.extend(modal.modes[index as usize].cost.iter().cloned());
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
    state: &mut GameState,
    effect: &deckmaste_core::Region,
    frame: &ExecutionFrame,
    chosen_modes: &[Uint],
    targets: &[Vec<ObjectId>],
) -> Vec<WorkItem> {
    let [Instruction::Modal(modal)] = effect.body.as_ref() else {
        let mut region_frame = frame.clone();
        if !matches!(region_frame.activation, crate::ActivationId::Stored(_)) {
            region_frame.activation = state.enter_region(effect, frame);
        }
        return effect
            .body
            .iter()
            .cloned()
            .map(|instruction| WorkItem::RunEffect {
                effect: Arc::new(instruction),
                frame: region_frame.clone(),
            })
            .collect();
    };

    let mut offset = 0;
    let items = chosen_modes
        .iter()
        .flat_map(|&index| {
            let mode = modal
                .modes
                .get(index as usize)
                .expect("ChooseModes validated every announced index");
            let count = mode.targets.len();
            let mut mode_frame = state.fork_frame(frame);
            state.frame_set_targets(&mut mode_frame, &targets[offset..offset + count]);
            mode_frame.activation = state.enter_region(&mode.effect, &mode_frame);
            offset += count;
            mode.effect
                .body
                .iter()
                .cloned()
                .map(move |instruction| WorkItem::RunEffect {
                    effect: Arc::new(instruction),
                    frame: mode_frame.clone(),
                })
        })
        .collect();
    debug_assert_eq!(
        offset,
        targets.len(),
        "announcement stores exactly the selected modes' target slots"
    );
    items
}

struct TargetAnnouncementSearch<'a> {
    state: &'a GameState,
    specs: &'a [TargetSpec],
    targeting_id: ObjectId,
    activation: crate::ActivationId,
}

impl TargetAnnouncementSearch<'_> {
    fn choose_slot(
        &self,
        candidates: &[ObjectId],
        wanted: usize,
        start: usize,
        slot: &mut Vec<ObjectId>,
        chosen: &mut Vec<Vec<ObjectId>>,
    ) -> bool {
        if slot.len() == wanted {
            chosen.push(slot.clone());
            let found = self.prefix_exists(chosen);
            chosen.pop();
            return found;
        }
        let remaining = wanted - slot.len();
        if candidates.len().saturating_sub(start) < remaining {
            return false;
        }
        for index in start..=candidates.len().saturating_sub(remaining) {
            slot.push(candidates[index]);
            if self.choose_slot(candidates, wanted, index + 1, slot, chosen) {
                return true;
            }
            slot.pop();
        }
        false
    }

    fn prefix_exists(&self, chosen: &mut Vec<Vec<ObjectId>>) -> bool {
        if chosen.len() == self.specs.len() {
            return crate::resolve::validate_target_set(self.specs, chosen).is_ok();
        }
        self.state.activation_set_targets(self.activation, chosen);
        let index = chosen.len();
        let candidates =
            self.state
                .legal_targets_for_specs(self.specs, self.targeting_id, self.activation)[index]
                .clone();
        let (min, max) = crate::resolve::slot_count_bounds(&self.specs[index]);
        let min = usize::try_from(min).expect("minimum target count fits usize");
        let max = max.map_or(candidates.len(), |count| {
            usize::try_from(count)
                .expect("maximum target count fits usize")
                .min(candidates.len())
        });
        if min > max {
            return false;
        }
        for wanted in min..=max {
            if self.choose_slot(&candidates, wanted, 0, &mut Vec::new(), chosen) {
                return true;
            }
        }
        false
    }
}

impl GameState {
    pub(crate) fn announcement_effect_satisfiable(
        &self,
        source: ObjectId,
        controller: PlayerId,
        effect: &deckmaste_core::Region,
        targets: &[TargetSpec],
    ) -> bool {
        let carrier = Some(self.objects.obj(source).source);
        let selection_satisfiable = |picks: &[Uint]| {
            let specs = announced_target_specs(effect, targets, picks);
            if crate::resolve::announced_prefix_len(&specs) != 0 {
                return self.with_temporary_region_activation(
                    effect,
                    &self.frame(source, controller),
                    |activation| self.target_announcement_satisfiable(&specs, source, activation),
                );
            }
            let legal: Vec<Vec<ObjectId>> = specs
                .iter()
                .map(|spec| self.legal_targets(spec, carrier, crate::ActivationId::NONE))
                .collect();
            crate::resolve::announce_satisfiable(&specs, &legal)
        };
        let [Instruction::Modal(modal)] = effect.body.as_ref() else {
            return selection_satisfiable(&[]);
        };
        let options = Uint::try_from(modal.modes.len()).expect("mode count fits Uint");
        let frame = self.frame(source, controller);
        let (lo, hi) = modal.choose.count.bounds();
        let lo = lo.map_or(0, |count| self.eval_count(count, &frame));
        let hi = hi.map_or(options, |count| self.eval_count(count, &frame));
        let max = if modal.choose.repeats { hi } else { hi.min(options) };
        let min = if modal.choose.up_to { 0 } else { lo.min(max) };
        first_mode_selection_matching(
            options,
            min,
            max,
            modal.choose.repeats,
            matches!(
                modal.choose.rider,
                Some(deckmaste_core::ModalCostRider::Entwine(_))
            ),
            selection_satisfiable,
        )
        .is_some()
    }

    pub(crate) fn announced_mode_selection_is_legal(&self, picks: &[Uint]) -> bool {
        if !self.payment_mana_modes_legal(picks) {
            return false;
        }
        let pending = self
            .announcing
            .as_ref()
            .expect("an announcement-time mode choice has an announce in flight");
        let view = self.layers();
        let specs = self.stack_object_target_specs(&view, &pending.object, picks);
        self.target_announcement_satisfiable(&specs, pending.id, pending.activation)
    }

    pub(crate) fn first_legal_announced_mode_selection(
        &self,
        choice: &crate::decide::pending::ChooseModes,
    ) -> Option<Vec<Uint>> {
        first_mode_selection_matching(
            choice.options,
            choice.min,
            choice.max,
            choice.repeats,
            choice.entwine,
            |picks| self.announced_mode_selection_is_legal(picks),
        )
    }

    /// [CR#601.3,601.2g]: may `player` cast `object` now? Offered iff the
    /// object is in the holder's hand (the caller iterates the hand), the
    /// object is not a land ([CR#305.9]), timing permits (instant → any
    /// priority; otherwise sorcery-speed), and every target spec has at least
    /// one legal candidate. Resource sufficiency is deliberately deferred to
    /// the explicit payment protocol.
    #[must_use]
    pub(crate) fn can_cast(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
    ) -> bool {
        self.castable_cost_ignoring_mana(view, player, object)
            .is_some()
    }

    /// The concrete cost `player` must cover to cast `object`, IFF every
    /// mana-INDEPENDENT casting legality holds (not a land, correct timing per
    /// [CR#307.1,117.1a,702.8a], a non-empty printed cost per [CR#118.6], and —
    /// per [CR#601.2c] — at least one legal candidate for every target spec);
    /// otherwise `None`. Resource sufficiency is deliberately deferred to the
    /// payment protocol, where a runner may advise on a proposed payment.
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
        if face.characteristics.mana_cost.is_empty() {
            return None;
        }
        let cost = self.mana_cost(object)?;
        // At least one complete mode/target announcement must exist. For a
        // top-level Modal, checking only `top_targets` would see no targets
        // and could offer a spell whose every mode is impossible to announce.
        let effect = self.spell_effect(object);
        let targets = self.spell_targets(object);
        effect
            .as_ref()
            .is_none_or(|effect| {
                self.announcement_effect_satisfiable(object, player, effect, &targets)
            })
            .then_some(cost)
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
        // `usable_abilities` list — see legal.rs).
        struct Src {
            object: ObjectId,
            ability: usize,
            color: ColorOrColorless,
            amount: Uint,
        }
        let view = self.layers();
        // Legal but for the mana? Also yields the concrete cost to cover.
        let cost = self.castable_cost_ignoring_mana(&view, player, object)?;
        // Only fixed Simple pips are auto-tappable; anything needing an
        // announce choice ({X}) or a non-fixed source
        // ({S}/hybrid/Phyrexian) bails.
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
        // pool already floated ([CR#106.6] SpendOnly-filtered for this
        // subject), then colored pips from exact-colour sources, then
        // generic from any.
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
        if self.payment.is_none() {
            self.begin_payment_proposal(controller);
        }
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
        let region = self.spell_effect(object).unwrap_or_else(|| {
            deckmaste_core::Region::new(
                deckmaste_core::announced_region_params(0),
                Instruction::Sequentially(Arc::from([])).into(),
            )
        });
        let activation = self.enter_region(&region, &self.frame(object, controller));
        self.announcing = Some(PendingStackEntry {
            optional_components: Vec::new(),
            paid_costs: Vec::new(),
            // [CR#405]: a spell's stack identity is its own object id.
            id: object,
            activation,
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
    /// played, never cast), a non-empty printed mana cost ([CR#118.6]), and a
    /// legal candidate for every target spec ([CR#601.2c]). Whether its cost
    /// can actually be paid is deliberately discovered by the payment frame,
    /// after the proposal has been announced. This is the gate on the `May`
    /// "yes" branch for `Cast(<ref>)`: offered only when it holds, else the
    /// `if_not` branch runs ([CR#608.2g] — the empty offer defaults to "you
    /// don't").
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
        // permission supplies a payable cost). Otherwise the printed mana cost
        // is the base. Affordability is intentionally not checked until the
        // payment frame is active.
        if alternative_cost.is_none() {
            // [CR#118.6]: an empty mana cost is "no mana cost" — an unpayable base.
            let face = crate::derive::face(self.def(object));
            if face.characteristics.mana_cost.is_empty() {
                return false;
            }
            if self.mana_cost(object).is_none() {
                return false;
            }
        }
        // [CR#601.2b..601.2c]: at least one complete mode/target
        // announcement must exist before offering this resolution cast.
        if self.spell_effect(object).is_some_and(|effect| {
            !self.announcement_effect_satisfiable(
                object,
                caster,
                &effect,
                &self.spell_targets(object),
            )
        }) {
            return false;
        }
        true
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
                resume: self.agenda.iter().cloned().collect::<Vec<_>>().into(),
                if_not: None,
            },
            crate::event::GameEvent::SpellCast(object),
        )
    }

    /// Resolution-time `May(Cast)` announce schedule with outcome branches
    /// attached to the payment boundary. `if_did` follows the becomes-cast
    /// event; `if_not` is retained only for announcement decline.
    #[must_use]
    pub(crate) fn may_cast_as_effect_items(
        &self,
        object: ObjectId,
        caster: PlayerId,
        alternative_cost: Option<deckmaste_core::Cost>,
        if_did: Option<Arc<Instruction>>,
        if_not: Option<Arc<Instruction>>,
        frame: ExecutionFrame,
    ) -> Vec<WorkItem> {
        let origin = self
            .objects
            .obj(object)
            .zone
            .expect("a castable referent is in a zone");
        let decline = if_not.map(|effect| (effect, Box::new(frame.clone())));
        let mut items = Self::announce_schedule_no_priority(
            WorkItem::BeginCastFromResolution {
                object,
                origin,
                caster,
                alternative_cost,
                resume: self.agenda.iter().cloned().collect::<Vec<_>>().into(),
                if_not: decline,
            },
            crate::event::GameEvent::SpellCast(object),
        );
        if let Some(effect) = if_did {
            items.push(WorkItem::RunEffect { effect, frame });
        }
        items
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
        let [Instruction::Modal(modal)] = effect.body.as_ref() else {
            return 0;
        };
        let options = Uint::try_from(modal.modes.len()).expect("mode count fits Uint");
        let frame = self.frame(source, controller);
        let (lo, hi) = modal.choose.count.bounds();
        let lo = lo.map_or(0, |count| self.eval_count(count, &frame));
        let hi = hi.map_or(options, |count| self.eval_count(count, &frame));
        let max = if modal.choose.repeats { hi } else { hi.min(options) };
        let min = if modal.choose.up_to { 0 } else { lo.min(max) };
        let player = self.acting_player(&modal.choose.chooser, &frame);
        self.pending = Some(DecisionPointKind::ChooseModes(
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
        self.choice = Some(crate::state::DecisionContinuation::AnnounceModes);
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
        let activation = pending.activation;
        self.surface_target_choice(controller, specs, spell, activation)
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
                    deckmaste_core::Ability::Spell(spell) => Some(spell.as_ref()),
                    _ => None,
                })
                .map_or_else(Vec::new, |spell| {
                    announced_target_specs(&spell.effect, &spell.targets, chosen_modes)
                }),
            StackObject::Activated { ability, .. } => {
                announced_target_specs(&ability.effect, &ability.targets, chosen_modes)
            }
            StackObject::Triggered {
                source,
                ability,
                created,
                ..
            } => {
                if let Some(t) = created {
                    t.targets.to_vec()
                } else {
                    let abilities = crate::derive::abilities_of_source(self, *source);
                    let other = &abilities[*ability];
                    let t = other.as_triggered().unwrap_or_else(|| {
                        panic!(
                            "a Triggered stack object indexes a Triggered ability, got {other:?}"
                        )
                    });
                    t.targets.to_vec()
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
        activation: crate::ActivationId,
    ) -> Uint {
        let legal = self.legal_targets_for_specs(&specs, targeting_id, activation);
        let count = Uint::try_from(specs.len()).expect("target-spec count fits in Uint");
        self.pending = Some(DecisionPointKind::ChooseTargets(
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
        activation: crate::ActivationId,
    ) -> Vec<Vec<ObjectId>> {
        let view = self.layers();
        let rows = crate::legal::cant_target_rows(self, &view);
        let carrier = Some(self.objects.obj(targeting_id).source);
        let announced = self.announced_targets(activation);
        specs
            .iter()
            .enumerate()
            .map(|(index, _)| {
                self.slot_candidates(specs, index, carrier, activation, &announced)
                    .into_iter()
                    .filter(|&t| {
                        // Forbidden by a Cant(Target) row ([CR#702.11b]
                        // hexproof), UNLESS an AsThough
                        // overlay sees through that specific
                        // obstacle for this agent ([CR#609.4] Glaring
                        // Spotlight).
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

    /// Whether at least one complete target announcement exists when later
    /// slots may read earlier announced-target registers ([CR#601.2c]). The
    /// ordinary independent-slot case keeps the compact set-level gate. A
    /// telescoping target list instead enumerates legal slot subsets in
    /// declaration order, writing each trial prefix into the activation before
    /// deriving the next slot. This is a satisfiability search only; the live
    /// register file is restored before returning.
    #[must_use]
    pub(crate) fn target_announcement_satisfiable(
        &self,
        specs: &[TargetSpec],
        targeting_id: ObjectId,
        activation: crate::ActivationId,
    ) -> bool {
        if crate::resolve::announced_prefix_len(specs) == 0 {
            let legal = self.legal_targets_for_specs(specs, targeting_id, activation);
            return crate::resolve::announce_satisfiable(specs, &legal);
        }

        let restore = self.announced_targets(activation);
        self.activation_set_targets(activation, &[]);
        let found = TargetAnnouncementSearch {
            state: self,
            specs,
            targeting_id,
            activation,
        }
        .prefix_exists(&mut Vec::new());
        self.activation_set_targets(activation, &restore);
        found
    }

    /// Candidate menus for choosing new targets when later slots read earlier
    /// ones. The prompt must contain candidates reachable after ANY fresh
    /// earlier choice, candidates reachable while keeping the current prefix,
    /// and every current target itself ([CR#707.10c,115.7d]). Final-set
    /// legality is re-derived from the submitted prefix by
    /// [`Self::cross_retarget_choice_legal`].
    pub(crate) fn retarget_candidates_for_specs(
        &self,
        specs: &[TargetSpec],
        targeting_id: ObjectId,
        activation: crate::ActivationId,
        current: &[Vec<ObjectId>],
    ) -> Vec<Vec<ObjectId>> {
        let current_legal = self.legal_targets_for_specs(specs, targeting_id, activation);
        if crate::resolve::announced_prefix_len(specs) == 0 {
            return current_legal;
        }
        self.activation_set_targets(activation, &[]);
        let mut widened = self.legal_targets_for_specs(specs, targeting_id, activation);
        self.activation_set_targets(activation, current);
        for (slot, under_current) in widened.iter_mut().zip(current_legal) {
            for candidate in under_current {
                if !slot.contains(&candidate) {
                    slot.push(candidate);
                }
            }
        }
        widened
    }

    /// Validate a retarget proposal whose later slots read earlier target
    /// registers. A newly chosen target must be legal under the PROPOSED
    /// prefix. A retained current target remains keepable when it was already
    /// illegal, but changing an earlier slot may not turn a formerly legal
    /// unchanged target illegal ([CR#115.7d]).
    #[must_use]
    pub(crate) fn cross_retarget_choice_legal(
        &self,
        specs: &[TargetSpec],
        current: &[Vec<ObjectId>],
        chosen: &[Vec<ObjectId>],
        targeting_id: ObjectId,
        activation: crate::ActivationId,
    ) -> bool {
        if specs.len() != current.len()
            || specs.len() != chosen.len()
            || crate::resolve::announced_prefix_len(specs) == 0
        {
            return specs.len() == current.len() && specs.len() == chosen.len();
        }
        self.activation_set_targets(activation, current);
        let current_legal = self.legal_targets_for_specs(specs, targeting_id, activation);
        let mut prefix: Vec<Vec<ObjectId>> = Vec::new();
        let mut legal = true;
        for (index, picks) in chosen.iter().enumerate() {
            self.activation_set_targets(activation, &prefix);
            let proposed_legal = self.legal_targets_for_specs(specs, targeting_id, activation);
            for &pick in picks {
                let retained = current[index].contains(&pick);
                if (!retained && !proposed_legal[index].contains(&pick))
                    || (retained
                        && current_legal[index].contains(&pick)
                        && !proposed_legal[index].contains(&pick))
                {
                    legal = false;
                    break;
                }
            }
            if !legal {
                break;
            }
            prefix.push(picks.clone());
        }
        self.activation_set_targets(activation, current);
        legal
    }

    /// [CR#601.2b]: surface a `ChooseXValue` if the in-flight announce's cost
    /// reads X — either an `{X}` mana symbol (`ManaSymbol::Variable`) or a
    /// non-mana cost verb whose count operand is semantic X (a loyalty `−X`'s
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
        let x = self
            .activation_parameter(pending.activation, &deckmaste_core::Provenance::AnnouncedX)
            .expect("spell and activated-ability regions declare announced X");
        let (base_has_x, effect) = match &pending.object {
            StackObject::Spell(o) => (
                pending.alternative_cost.as_ref().map_or_else(
                    || {
                        self.mana_cost(*o).is_some_and(|cost| {
                            cost.iter()
                                .any(|symbol| matches!(symbol, ManaSymbol::Variable))
                        })
                    },
                    |cost| cost_components_mention_x(cost, x),
                )
                    // [CR#118.8,601.2b]: the printed additional cost is part of
                    // the same announcement, so "sacrifice X creatures"
                    // triggers the X announcement too.
                    || cost_components_mention_x(&self.spell_additional_cost(*o), x),
                self.spell_effect(*o),
            ),
            StackObject::Activated { ability, .. } => {
                let summary = crate::activate::cost_summary(&ability.cost)
                    .expect("can_activate vetted the cost");
                // [CR#601.2b]: an `{X}` mana symbol OR semantic X in any
                // cost-eligible verb (the loyalty `−X` case) triggers the
                // announcement — checked together so a cost carrying both
                // announces X exactly once.
                (
                    summary
                        .mana
                        .iter()
                        .any(|symbol| matches!(symbol, ManaSymbol::Variable))
                        || summary
                            .verbs
                            .iter()
                            .any(|verb| verb_mentions_cost_x(verb, x))
                        || cost_components_mention_x(&ability.cost, x),
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
            || cost_components_mention_x(&pending.optional_components, x)
            || cost_components_mention_x(&mode_components, x);
        if has_x {
            self.pending = Some(DecisionPointKind::ChooseXValue(
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
        // [CR#118.8,118.8a]: a spell's PRINTED ADDITIONAL cost is announced and
        // paid with its mana cost, so its mana joins the concretized demand
        // and its instructions join the payment window.
        if let StackObject::Spell(object) = &pending.object {
            let (extra_mana, extra_steps) =
                partition_alternative_cost(&self.spell_additional_cost(*object));
            let mut symbols: Vec<ManaSymbol> = cost.iter().copied().collect();
            symbols.extend(extra_mana.iter().copied());
            cost = ManaCost::from(Arc::from(symbols));
            alt_verbs.extend(extra_steps);
        }
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
        self.pending = Some(DecisionPointKind::ChooseCostOptions(
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
    /// `StaticSpec::CostOption` rows on the card (they function from the
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
        self.pending = Some(crate::decide::DecisionPointKind::YesNo(
            crate::decide::pending::YesNo { player: controller },
        ));
        self.choice = Some(crate::state::DecisionContinuation::OptionalCost {
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
                deckmaste_core::Ability::Static(s) => Some(&s.body),
                _ => None,
            })
            .filter_map(|e| match e {
                deckmaste_core::StaticSpec::CostOption(oc) => Some(oc.clone()),
                _ => None,
            })
            .collect()
    }

    /// [CR#601.2f,601.2g,601.2h]: open the in-flight cost's payment protocol.
    /// The core never auto-pays.
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
    pub(crate) fn open_payment(&mut self) {
        fn append_nonmana(cost: &deckmaste_core::Cost, out: &mut Vec<CostComponent>) {
            for component in cost {
                match component {
                    CostComponent::Mana(_) | CostComponent::ManaCostOf(_) => {}
                    CostComponent::Cost(inner) => append_nonmana(inner, out),
                    other => out.push(other.clone()),
                }
            }
        }

        fn concretize_component_x(component: &CostComponent, x: Uint) -> CostComponent {
            match component {
                CostComponent::Mana(mana) => CostComponent::Mana(concretize_x(mana, x)),
                CostComponent::Cost(inner) => CostComponent::Cost(deckmaste_core::Cost(
                    inner
                        .iter()
                        .map(|component| concretize_component_x(component, x))
                        .collect::<Arc<[_]>>(),
                )),
                other => other.clone(),
            }
        }

        let pending = self
            .announcing
            .clone()
            .expect("an announce in flight when payment opens");
        let payer = pending.controller;
        let announced_x = pending.x.unwrap_or(0);
        let (mana, extra_components) = pending
            .concretized
            .clone()
            .expect("ChooseCostOptions concretized the cost before OpenPayment");
        let mana = concretize_x(&mana, announced_x);

        let (subject, mut frame, components, pay_pips) = match pending.object {
            StackObject::Spell(object) => {
                let mut components = vec![CostComponent::Mana(mana)];
                components.extend(extra_components);
                components.extend(
                    pending
                        .optional_components
                        .iter()
                        .map(|component| concretize_component_x(component, announced_x)),
                );
                (
                    crate::payment::PaymentSubject::Spell(object),
                    self.frame(object, payer),
                    components,
                    self.payment_pip_alternatives(object),
                )
            }
            StackObject::Activated {
                source,
                ability,
                bindings: _,
            } => {
                let mut components = vec![CostComponent::Mana(mana)];
                append_nonmana(&ability.cost, &mut components);
                components.extend(extra_components);
                (
                    crate::payment::PaymentSubject::Activated {
                        ability: pending.id,
                        source,
                    },
                    self.frame(source, payer),
                    components,
                    Vec::new(),
                )
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability never occupies the announce slot")
            }
        };
        frame.activation = pending.activation;
        frame.payment = Some(self.mint_payment());
        let cost = deckmaste_core::Cost(components.into());
        let locked = crate::payment::lock_cost(self, payer, subject, &frame, &cost, &pay_pips)
            .expect("announce-time choices leave a concrete runnable cost");
        self.open_locked_payment(locked);
    }

    /// Every alternative-payment action currently functioning on `spell`, in
    /// derived card order. Resource choice is deliberately absent: the payer's
    /// later `ManaCoverage` names an exact object and IOU.
    fn payment_pip_alternatives(&self, spell: ObjectId) -> Vec<(PipClass, PayAct)> {
        let view = self.layers();
        let mut alternatives = Vec::new();
        crate::legal::for_each_static(self, &view, spell, |effect| {
            if let StaticSpec::PayPips(class, act) = effect {
                alternatives.push((*class, act.clone()));
            }
        });
        alternatives
    }

    #[expect(
        dead_code,
        reason = "retained temporarily while staged fulfillment replaces each legacy payment arm"
    )]
    pub(crate) fn legacy_pay_cost(&mut self) -> Result<(), &'static str> {
        let (controller, activation, announced_x, concretized, optional_components, object) = {
            let pending = self.announcing.as_ref().expect("an announce in flight");
            (
                pending.controller,
                pending.activation,
                pending.x.unwrap_or(0),
                pending.concretized.clone(),
                pending.optional_components.clone(),
                pending.object.clone(),
            )
        };
        // `announced_x` (defaulted to 0) concretizes `{X}` mana. Cost-verb
        // frames share the announcement activation, whose X parameter already
        // carries the chosen value.
        // [CR#601.2b]: the announced concretization — always set by the
        // preceding `ChooseCostOptions` step.
        let (mana, extra_verbs) =
            concretized.expect("ChooseCostOptions concretized the cost before PayCost");
        // [CR#601.2f]: the announced optional additional components.
        // The Phyrexian-life picks rode as `Do(LoseLife(2))` cost components;
        // unwrap them into payable verbs ([CR#601.2h]).
        let extra_verbs = phyrexian_life_verbs(&extra_verbs);
        match &object {
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
                let mut items = verb_payment_items(&extra_verbs, activation, payment);
                // [CR#601.2b]: apply the announced X to the concretized mana
                // ({X} -> Generic(announced_x); hybrid/Phyrexian already
                // resolved).
                let mana = concretize_x(&mana, announced_x);
                // [CR#601.2f]: announced optional additional costs (kicker,
                // [CR#702.33a]) join the total — mana components into the
                // mana decision, verb components into the payment window.
                let mut mana: Vec<ManaSymbol> = mana.to_vec();
                for component in &optional_components {
                    match component {
                        CostComponent::Mana(m) => mana.extend(m.iter().copied()),
                        CostComponent::Act { .. }
                        | CostComponent::Choose(_)
                        | CostComponent::Sample(_)
                        | CostComponent::Search(_)
                        | CostComponent::Let(_) => {
                            items.extend(cost_step_items(
                                std::slice::from_ref(component),
                                activation,
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
                // (the total cost / mana value are untouched, [CR#702.51b]).
                // The tap/exile items join the same payment
                // window as the verbs.
                let (mana, pip_items) = self.assemble_pip_payments(object, controller, &mana);
                items.extend(pip_items);
                if !items.is_empty() {
                    self.schedule_front(items);
                }
                if !mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(DecisionPointKind::PayMana(
                        crate::decide::pending::PayMana {
                            player: controller,
                            cost: mana,
                            pool,
                            // [CR#106.6]: a spell's stack identity is its own id —
                            // the object SpendOnly riders judge.
                            subject: object,
                        },
                    ));
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
                if !summary.tap_totals.is_empty() {
                    return Err("TapTotal costs require the payment-obligation protocol");
                }
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
                // [CR#601.2b,601.2h]: the cost BLOCK in announcement order,
                // run against the announce activation — a payment-time choice
                // surfaces its `ChooseObjects` decision and writes its
                // register, and the verb after it pays through that register,
                // which the ability body then reads as the paid product.
                items.extend(cost_step_items(&summary.steps, activation, payment));
                items.extend(verb_payment_items(&extra_verbs, activation, payment));
                if !items.is_empty() {
                    self.schedule_front(items);
                }
                // [CR#601.2b]: apply the announced X to the concretized mana
                // (hybrid/Phyrexian already resolved by ChooseCostOptions).
                let mana = concretize_x(&mana, announced_x);
                if !mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(DecisionPointKind::PayMana(
                        crate::decide::pending::PayMana {
                            player: controller,
                            // [CR#601.2b]: the concretized mana (hybrid/Phyrexian
                            // resolved, {X} applied), not the printed cost.
                            cost: mana,
                            pool,
                            // [CR#106.6]: an activated ability's mana is spent on
                            // its source — that is the object SpendOnly judges.
                            subject: source,
                        },
                    ));
                }
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability has no cost and never occupies the announce slot")
            }
        }
        Ok(())
    }

    /// [CR#601.2g..601.2h]: the per-pip alternative-payment hook for a spell
    /// being cast (convoke / delve / improvise) — the EXECUTING consumer of
    /// [`StaticSpec::PayPips`]. Runs the shared [`Self::pip_coverage`] walk
    /// over the locked-in `mana` and turns each covered pip into its payment
    /// work item, returning the mana the player must still pay with real mana
    /// plus those tap/exile items for the payment window.
    ///
    /// The total cost and mana value ([CR#202.3]) are never mutated — this
    /// payment substitution is applied only after the total is locked.
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

    /// The read-only core of the per-pip alternative-payment walk. It gathers
    /// the spell's `PayPips` statics and walks the
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
    /// DEFERRED — interactive picker: the payer is entitled to choose which
    /// permanent to tap or card to exile and which pips to cover.
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
            if let StaticSpec::PayPips(class, act) = e {
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
        let printed = crate::derive::face(self.def(object))
            .characteristics
            .mana_cost
            .clone();
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
    fn cost_modifier_rows(
        &self,
        object: ObjectId,
    ) -> Vec<(ExecutionFrame, deckmaste_core::CostChange)> {
        use std::ops::ControlFlow;
        let mut rows: Vec<(ExecutionFrame, deckmaste_core::CostChange)> = Vec::new();
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
                if let deckmaste_core::StaticSpec::CostModifier { of, change } = e
                    && self.filter_matches_live(of, object, source)
                {
                    rows.push((
                        self.frame(object, self.objects.obj(object).controller),
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
                if let deckmaste_core::StaticSpec::CostModifier { of, change } = e
                    && self.filter_matches_live(of, object, self.objects.obj(id).source)
                {
                    rows.push((
                        self.frame(id, self.objects.obj(id).controller),
                        change.clone(),
                    ));
                }
            });
        }
        // Rows granted by resolved one-shots ([CR#611.2c] instance rows). Each
        // is self-filtered (`of` is a spell predicate); anchor
        // `Ref(This)`/`You` /`Scaled` on the instance controller's
        // player proxy — the row has no battlefield carrier of its own.
        for ce in &self.continuous {
            let carrier = self.player(ce.controller).object;
            let source = self.objects.obj(carrier).source;
            for row in &ce.rows {
                if let deckmaste_core::StaticSpec::CostModifier { of, change } = row
                    && self.filter_matches_live(of, object, source)
                {
                    rows.push((self.frame(carrier, ce.controller), change.clone()));
                }
            }
        }
        rows
    }

    /// Apply one `CostChange` to `cost` for the given phase — increases (and
    /// mandatory additional mana) on `Raise`, reductions on `Lower` — with
    /// `times` scaling from any enclosing `Scaled` ([CR#601.2f]).
    /// The optional kicker-family shape ([CR#118.8b]) is a declared
    /// `StaticSpec::CostOption` — the [CR#601.2b] announce family — and
    /// stays inert here until its announce machinery lands
    /// (core-alt-costs/engine-alt-costs), exactly as before this pipeline.
    fn apply_cost_change(
        &self,
        cost: &mut Vec<ManaSymbol>,
        change: &deckmaste_core::CostChange,
        frame: &ExecutionFrame,
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

    /// The candidates one announce slot offers ([CR#601.2c]) —
    /// [`Self::legal_targets`] for an ordinary slot, and the union over the
    /// earlier slot's own candidates for a slot whose filter READS that
    /// earlier slot's announced register while nothing has been announced yet.
    ///
    /// Announcement fixes the slots in declaration order (core's telescope law
    /// lets slot `k`'s filter read `AnnouncedTarget(0..k)` and nothing later),
    /// so once a real announcement is in the register file this is the plain
    /// enumeration against it. Before then — the menu the player picks from —
    /// the honest offer is every object SOME legal earlier choice would admit;
    /// which of them the announcement actually admits is settled by
    /// [`Self::cross_target_choice_legal`] when the set comes back. Reading an
    /// empty register instead packs to nothing, which decided the slot the
    /// wrong way in both directions: a positive cross-reference (Fiery
    /// Annihilation's "Equipment attached to that creature") offered an empty
    /// menu, a negated one (Run Away Together's second creature) offered
    /// every candidate.
    ///
    /// `announced` is what the register file already holds. A slot reading TWO
    /// earlier slots would need the product of their candidates; no semantic
    /// input has one, so it keeps the plain enumeration.
    #[must_use]
    fn slot_candidates(
        &self,
        specs: &[TargetSpec],
        index: usize,
        carrier: Option<crate::object::ObjectSource>,
        activation: crate::ActivationId,
        announced: &[Vec<ObjectId>],
    ) -> Vec<ObjectId> {
        let spec = &specs[index];
        let reads = crate::resolve::announced_prefix_len(std::slice::from_ref(spec));
        if reads != 1 || !announced.is_empty() || activation == crate::ActivationId::NONE {
            return self.legal_targets(spec, carrier, activation);
        }
        // One announced-target parameter is slot 0, the only shape the
        // telescope admits for a single such parameter.
        let mut admitted: std::collections::HashSet<ObjectId> = std::collections::HashSet::new();
        for candidate in self.legal_targets(&specs[0], carrier, activation) {
            self.activation_set_targets(activation, &[vec![candidate]]);
            admitted.extend(self.legal_targets(spec, carrier, activation));
        }
        self.activation_set_targets(activation, announced);
        self.objects
            .iter()
            .map(|object| object.id)
            .filter(|id| admitted.contains(id))
            .collect()
    }

    /// [CR#601.2c]: is a PROPOSED target set legal once each slot's own
    /// announcement is visible to the slots that read it?
    ///
    /// The menu a cross-referencing slot was offered is a union over the
    /// earlier slot's candidates ([`Self::slot_candidates`]); the
    /// cross-reference itself is enforced here, by walking the slots in
    /// declaration order, writing each announced slot into the register file,
    /// and re-enumerating the next one against it. Core's telescope law makes
    /// the walk total: slot `k` reads only slots before it, so every register
    /// a slot reads is written before that slot is judged.
    ///
    /// The register file is restored to what it held on entry; the caller
    /// writes the whole announced set on success.
    #[must_use]
    pub(crate) fn cross_target_choice_legal(
        &self,
        specs: &[TargetSpec],
        chosen: &[Vec<ObjectId>],
        targeting_id: ObjectId,
        activation: crate::ActivationId,
    ) -> bool {
        if activation == crate::ActivationId::NONE
            || specs.len() != chosen.len()
            || crate::resolve::announced_prefix_len(specs) == 0
        {
            return true;
        }
        let restore = self.announced_targets(activation);
        let mut prefix: Vec<Vec<ObjectId>> = Vec::new();
        let mut legal = true;
        for (index, picks) in chosen.iter().enumerate() {
            self.activation_set_targets(activation, &prefix);
            let refreshed = self.legal_targets_for_specs(specs, targeting_id, activation);
            if picks.iter().any(|pick| !refreshed[index].contains(pick)) {
                legal = false;
                break;
            }
            prefix.push(picks.clone());
        }
        self.activation_set_targets(activation, &restore);
        legal
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
    /// Delegates predicate extraction to `resolve::target_spec_predicate` so that
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
        activation: crate::ActivationId,
    ) -> Vec<ObjectId> {
        let predicate = crate::resolve::target_spec_predicate(spec);
        crate::target::candidates_region_with_activation(self, predicate, carrier, activation)
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
            Some(DecisionPointKind::PayMana(crate::decide::pending::PayMana {
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
        // {X}{R} at X=3 -> {3}{R}; X=0 -> {0}{R}; a cost with no X is
        // unchanged.
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
        // ONE snow green alone canNOT pay {G}{S}: needs two units (one per
        // pip).
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
    use deckmaste_card::Characteristics;
    use deckmaste_core::Ability;
    use deckmaste_core::CostChange;
    use deckmaste_core::Count;
    use deckmaste_core::Predicate;
    use deckmaste_core::Reference;
    use deckmaste_core::StatePredicate;
    use deckmaste_core::StaticSpec;
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

        let target = TargetSpec::Target(
            Quantity::one(),
            Arc::new(deckmaste_core::Region::candidate(Predicate::r#type(
                Type::Creature,
            ))),
        );
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Modal announcement fixture".into(),
            mana_cost: "{0}".parse().unwrap(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: Instruction::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::Reg(deckmaste_core::RefId(1)),
                        rider: None,
                    },
                    modes: vec![
                        deckmaste_core::Mode {
                            targets: [].into(),
                            effect: Instruction::act(CoreAction::ChangeLife(
                                Reference::Reg(deckmaste_core::RefId(1)),
                                deckmaste_core::LifeOp::Up(Count::Literal(1)),
                            ))
                            .into(),
                            cost: deckmaste_core::Cost::default(),
                        },
                        Mode {
                            targets: vec![target.clone()].into(),
                            effect: Instruction::act(CoreAction::destroy(Reference::Reg(
                                deckmaste_core::RefId(6),
                            )))
                            .into(),
                            cost: deckmaste_core::Cost::default(),
                        },
                    ]
                    .into(),
                })
                .into(),
            })],
            ..Characteristics::default()
        }));
        let mut state = cm_game();
        put_synthetic(
            &mut state,
            Card::Normal(CardFace::from(Characteristics {
                name: "Modal target fixture".into(),
                types: vec![Type::Creature.def()],
                ..Characteristics::default()
            })),
            PlayerId(0),
            Zone::Battlefield,
        );
        let spell = put_synthetic(&mut state, card, PlayerId(0), Zone::Hand);

        state.begin_cast(spell);
        assert!(state.announcing.as_ref().unwrap().chosen_modes.is_empty());
        assert_eq!(state.announce_modes(), 2);
        assert!(matches!(
            state.pending,
            Some(DecisionPointKind::ChooseModes(_))
        ));

        state
            .submit_decision(crate::decide::Decision::Modes(vec![1]))
            .unwrap();
        assert_eq!(
            state.announcing.as_ref().unwrap().chosen_modes.as_ref(),
            &[1]
        );

        assert_eq!(state.announce_targets(), 1);
        let Some(DecisionPointKind::ChooseTargets(choice)) = &state.pending else {
            panic!("the selected targeted mode should announce its target");
        };
        assert_eq!(choice.spec, vec![target]);
    }

    #[test]
    fn modal_mode_choices_are_stored_in_printed_order() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;

        let life_mode = |amount| Mode {
            targets: [].into(),
            effect: Instruction::act(CoreAction::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                deckmaste_core::LifeOp::Up(Count::Literal(amount)),
            ))
            .into(),
            cost: deckmaste_core::Cost::default(),
        };
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Ordered modal announcement fixture".into(),
            mana_cost: "{0}".parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: Instruction::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::Reg(deckmaste_core::RefId(1)),
                        rider: None,
                    },
                    modes: vec![life_mode(3), life_mode(5)].into(),
                })
                .into(),
            })],
            ..Characteristics::default()
        }));
        let mut state = cm_game();
        let spell = put_synthetic(&mut state, card, PlayerId(0), Zone::Hand);

        state.begin_cast(spell);
        assert_eq!(state.announce_modes(), 2);
        state
            .submit_decision(crate::decide::Decision::Modes(vec![1, 0]))
            .unwrap();

        assert_eq!(
            state.announcing.as_ref().unwrap().chosen_modes.as_ref(),
            &[0, 1]
        );
    }

    #[test]
    fn resolution_cast_rejects_a_modal_spell_with_no_satisfiable_mode() {
        use deckmaste_core::CharacteristicPredicate;
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;

        let impossible_mode = || Mode {
            targets: vec![TargetSpec::Target(
                Quantity::one(),
                Arc::new(deckmaste_core::Region::candidate(
                    deckmaste_core::Predicate::Characteristic(CharacteristicPredicate::Named(
                        "Missing target".into(),
                    )),
                )),
            )]
            .into(),
            effect: Instruction::Sequentially(Arc::from([])).into(),
            cost: deckmaste_core::Cost::default(),
        };
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Impossible modal spell".into(),
            mana_cost: "{0}".parse().unwrap(),
            types: vec![Type::Instant.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: Instruction::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::Reg(deckmaste_core::RefId(1)),
                        rider: None,
                    },
                    modes: vec![impossible_mode(), impossible_mode()].into(),
                })
                .into(),
            })],
            ..Characteristics::default()
        }));
        let mut state = cm_game();
        let spell = put_synthetic(&mut state, card, PlayerId(0), Zone::Hand);

        assert!(!state.can_cast_as_effect(PlayerId(0), spell, None));
    }

    #[test]
    fn announced_modal_modes_resolve_without_reopening_and_keep_target_scopes() {
        use deckmaste_core::ChooseSpec;
        use deckmaste_core::Modal;
        use deckmaste_core::Mode;
        use deckmaste_core::Quantity;
        use deckmaste_core::SpellAbility;

        let life_mode = |amount| Mode {
            targets: vec![TargetSpec::Target(
                Quantity::one(),
                Arc::new(deckmaste_core::Region::candidate(Predicate::Entity(
                    deckmaste_core::EntityClass::Player,
                ))),
            )]
            .into(),
            effect: deckmaste_core::Region::new(
                vec![
                    deckmaste_core::Param {
                        def: deckmaste_core::DefId(0),
                        kind: deckmaste_core::Kind::Entity,
                        provenance: deckmaste_core::Provenance::Source,
                    },
                    deckmaste_core::Param {
                        def: deckmaste_core::DefId(1),
                        kind: deckmaste_core::Kind::Entity,
                        provenance: deckmaste_core::Provenance::Controller,
                    },
                    deckmaste_core::Param {
                        def: deckmaste_core::DefId(2),
                        kind: deckmaste_core::Kind::Entities,
                        provenance: deckmaste_core::Provenance::AnnouncedTarget(0),
                    },
                    deckmaste_core::Param {
                        def: deckmaste_core::DefId(3),
                        kind: deckmaste_core::Kind::Number,
                        provenance: deckmaste_core::Provenance::AnnouncedX,
                    },
                ]
                .into(),
                Instruction::act(CoreAction::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(2)),
                    deckmaste_core::LifeOp::Up(Count::Literal(amount)),
                ))
                .into(),
            ),
            cost: deckmaste_core::Cost::default(),
        };
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Modal resolution fixture".into(),
            mana_cost: "{0}".parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: Instruction::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::Range(Some(Count::Literal(2)), Some(Count::Literal(2))),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::Reg(deckmaste_core::RefId(1)),
                        rider: None,
                    },
                    modes: vec![life_mode(3), life_mode(5)].into(),
                })
                .into(),
            })],
            ..Characteristics::default()
        }));
        let mut state = cm_game();
        let p0 = PlayerId(0);
        let p1 = PlayerId(1);
        let p0_object = state.player(p0).object;
        let p1_object = state.player(p1).object;
        let p0_life = state.player(p0).life;
        let p1_life = state.player(p1).life;
        let spell = put_synthetic(&mut state, card, p0, Zone::Stack);
        state.stack.push(crate::stack::StackEntry {
            activation: crate::ActivationId::NONE,
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
            targets: [].into(),
            effect: Instruction::act(CoreAction::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                deckmaste_core::LifeOp::Up(Count::Literal(1)),
            ))
            .into(),
            cost: deckmaste_core::Cost(
                vec![CostComponent::Mana(
                    format!("{{{generic}}}").parse().unwrap(),
                )]
                .into(),
            ),
        };
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Modal cost fixture".into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: Instruction::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::Reg(deckmaste_core::RefId(1)),
                        rider: None,
                    },
                    modes: vec![mode(1), mode(2)].into(),
                })
                .into(),
            })],
            ..Characteristics::default()
        }));
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
            targets: [].into(),
            effect: Instruction::act(CoreAction::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                deckmaste_core::LifeOp::Up(Count::Literal(1)),
            ))
            .into(),
            cost: deckmaste_core::Cost::default(),
        };
        let card = Card::Normal(CardFace::from(Characteristics {
            name: "Entwine fixture".into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            abilities: vec![Ability::spell(SpellAbility {
                ability_word: None,
                cost: deckmaste_core::Cost::default(),
                targets: [].into(),
                effect: Instruction::Modal(Modal {
                    choose: ChooseSpec {
                        count: Quantity::one(),
                        up_to: false,
                        repeats: false,
                        chooser: Reference::Reg(deckmaste_core::RefId(1)),
                        rider: Some(ModalCostRider::Entwine(Cost(
                            vec![CostComponent::Mana("{3}".parse().unwrap())].into(),
                        ))),
                    },
                    modes: vec![mode(), mode()].into(),
                })
                .into(),
            })],
            ..Characteristics::default()
        }));
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
            targets: [].into(),
            effect: Instruction::act(CoreAction::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                deckmaste_core::LifeOp::Up(Count::Literal(1)),
            ))
            .into(),
            cost: deckmaste_core::Cost(vec![CostComponent::Mana(mana.parse().unwrap())].into()),
        };
        let card = || {
            Card::Normal(CardFace::from(Characteristics {
                name: "Modal X-cost fixture".into(),
                mana_cost: "{1}".parse().unwrap(),
                types: vec![Type::Sorcery.def()],
                abilities: vec![Ability::spell(SpellAbility {
                    ability_word: None,
                    cost: deckmaste_core::Cost::default(),
                    targets: [].into(),
                    effect: Instruction::Modal(Modal {
                        choose: ChooseSpec {
                            count: Quantity::one(),
                            up_to: false,
                            repeats: false,
                            chooser: Reference::Reg(deckmaste_core::RefId(1)),
                            rider: None,
                        },
                        modes: vec![mode("{X}"), mode("{1}")].into(),
                    })
                    .into(),
                })],
                ..Characteristics::default()
            }))
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
            Some(DecisionPointKind::ChooseXValue(_))
        ));
    }

    fn vanilla_artifact(name: &str) -> Card {
        Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            mana_cost: "{1}".parse().unwrap(),
            types: vec![Type::Artifact.def()],
            ..Characteristics::default()
        }))
    }

    /// An affinity-shaped self-row ([CR#702.41a]): "costs {1} less for each
    /// battlefield artifact".
    fn affinity_card(printed: &str) -> Card {
        Card::Normal(CardFace::from(Characteristics {
            name: "Fromite".into(),
            mana_cost: printed.parse().unwrap(),
            types: vec![Type::Artifact.def()],
            abilities: vec![Ability::r#static(StaticSpec::CostModifier {
                of: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
                change: CostChange::Scaled {
                    change: Arc::new(CostChange::Reduce(
                        vec![CostComponent::Mana("{1}".parse().unwrap())].into(),
                    )),
                    times: Count::CountOf(deckmaste_core::Countable::Objects(Arc::new(
                        deckmaste_core::Region::candidate(Predicate::And(
                            vec![
                                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                                Predicate::r#type(Type::Artifact),
                            ]
                            .into(),
                        )),
                    ))),
                },
            })],
            ..Characteristics::default()
        }))
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
        let bear = Card::Normal(CardFace::from(Characteristics {
            name: "Bear".into(),
            mana_cost: "{1}{G}".parse().unwrap(),
            types: vec![Type::Creature.def()],
            ..Characteristics::default()
        }));
        let spell = put_synthetic(&mut state, bear, PlayerId(0), Zone::Hand);

        let taxer = Card::Normal(CardFace::from(Characteristics {
            name: "Thorn Totem".into(),
            types: vec![Type::Artifact.def()],
            abilities: vec![Ability::r#static(StaticSpec::CostModifier {
                of: Predicate::r#type(Type::Creature),
                change: CostChange::Increase(
                    vec![CostComponent::Mana("{1}".parse().unwrap())].into(),
                ),
            })],
            ..Characteristics::default()
        }));
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
        use deckmaste_core::Cost;
        use deckmaste_core::ManaSpec;
        let ability = Arc::new(ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            cost: Cost(vec![CostComponent::Tap].into()),
            from: None,
            window: None,
            condition: None,
            limits: vec![].into(),
            effect: Instruction::act(CoreAction::AddMana(
                Reference::Reg(deckmaste_core::RefId(1)),
                Count::Literal(1),
                ManaSpec::Specific(color).into(),
            ))
            .into(),
        });
        Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            mana_cost: ManaCost::default(),
            types: vec![Type::Land.def()],
            abilities: vec![Ability::Activated(ability)],
            ..Characteristics::default()
        }))
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
            permanent_type: false,
            confers: vec![deckmaste_core::Property::Ability(Arc::new(
                Ability::r#static(StaticSpec::Deontic(deckmaste_core::Deontic::May(
                    deckmaste_core::DeonticAction::Cast {
                        what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
        Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            mana_cost: mc.parse().unwrap(),
            types: vec![instant_typedef()],
            ..Characteristics::default()
        }))
    }

    /// A sorcery — no casting-window confer, so it is castable only at sorcery
    /// speed. `Type::Sorcery.def()` carries the correct (empty) confers.
    fn sorcery(name: &str, mc: &str) -> Card {
        Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            mana_cost: mc.parse().unwrap(),
            types: vec![Type::Sorcery.def()],
            ..Characteristics::default()
        }))
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
            permanent_type: true,
            confers: vec![deckmaste_core::Property::Ability(Arc::new(
                Ability::r#static(StaticSpec::Deontic(deckmaste_core::Deontic::May(
                    deckmaste_core::DeonticAction::Play {
                        what: Predicate::Ref(Reference::Reg(deckmaste_core::RefId(0))),
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
        Card::Normal(CardFace::from(Characteristics {
            name: name.into(),
            mana_cost: ManaCost::default(),
            types: vec![land_typedef()],
            ..Characteristics::default()
        }))
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
        use crate::agenda::WorkItem;
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::event::GameEvent;
        use crate::payment::IouKind;
        use crate::payment::ManaCoverage;
        use crate::payment::ManaPip;
        use crate::payment::PaymentCommand;
        use crate::step::StepOutcome;

        let mut state = cm_game();
        let land = put_synthetic(
            &mut state,
            mana_land("Mtn", red()),
            PlayerId(0),
            Zone::Battlefield,
        );
        let spell = put_synthetic(&mut state, instant("Bolt", "{R}"), PlayerId(0), Zone::Hand);
        assert!(
            state.can_cast(&state.layers(), PlayerId(0), spell),
            "the structurally legal cast is offered before its resources are supplied"
        );

        state.schedule_front(GameState::announce_schedule(
            WorkItem::BeginCast(spell),
            GameEvent::SpellCast(spell),
        ));
        let prompt = loop {
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => break prompt,
                other => panic!("expected cast payment, got {other:?}"),
            }
        };
        let [iou] = prompt.outstanding.as_slice() else {
            panic!("the {{R}} cast locks exactly one IOU: {prompt:?}")
        };
        assert_eq!(iou.kind, IouKind::ManaPip(ManaPip::Colored(Color::Red)));

        let image_before = format!("{:#?}", state.active());
        let before = prompt;
        assert!(
            state
                .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(
                    ManaCoverage::empty(),
                )))
                .is_err(),
            "empty coverage cannot satisfy the locked red pip"
        );
        let Some(DecisionPointKind::Payment(after)) = state.pending.as_ref() else {
            panic!("the rejected coverage keeps the payment prompt open")
        };
        assert_eq!(
            after, &before,
            "rejection leaves the active prompt unchanged"
        );
        assert_eq!(
            format!("{:#?}", state.active()),
            image_before,
            "rejection leaves the active rules image unchanged"
        );

        state
            .submit_decision(Decision::Payment(PaymentCommand::DeclinePayment))
            .unwrap();
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
