//! Casting ([CR#601]): the mana-payment solver plus the reified announce flow
//! (`begin_cast` → `announce_x` → `announce_targets` → `pay_cost`), and the
//! `can_cast` legality gate that `legal::legal_actions` offers from. The
//! announce flow (`announce_targets` / `pay_cost`) is shared with activated
//! abilities ([CR#602.2b]); see `activate.rs` for the activation entry point.

use deckmaste_core::Action;
use deckmaste_core::Agency;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::CostComponent;
use deckmaste_core::Effect;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_core::Window;
use deckmaste_core::Zone;

use crate::agenda::WorkItem;
use crate::decide::PendingDecision;
use crate::event::Cause;
use crate::event::GameEvent;
use crate::event::Occurrence;
use crate::object::ObjectId;
use crate::player::ManaPool;
use crate::player::PlayerId;
use crate::stack::Frame;
use crate::stack::PendingStackEntry;
use crate::stack::StackObject;
use crate::state::GameState;
use crate::target::candidates;

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
            .collect::<Vec<_>>(),
    )
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
            // Hybrid/Phyrexian are concretized at [CR#601.2b] before payment;
            // Variable ({X}) is announced there too (engine-x-costs). A residual
            // one here is an engine bug, not a payable cost.
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
pub fn apply_payment(pool: &mut ManaPool, payment: &Payment) { pool.remove_units(&payment.units); }

/// Canonical auto-tap ([CR#601.2g], a runner/test convenience — the engine
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
#[must_use]
#[allow(
    dead_code,
    reason = "public subject-free auto-tap helper; engine paths now route through auto_pay_pending (spendability-aware), but this stays the canonical pure form for runners/tests"
)]
pub fn auto_pay(pool: &ManaPool, cost: &ManaCost) -> Payment {
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
/// activating `player` against the ability's `source`. The verb rides
/// `Action::By(You, …)` over a fresh resolution frame whose `controller` is
/// the activator — so `Reference::You` resolves to that player and
/// `Reference::This` (a self-sacrifice) to the source — mirroring the frame
/// any effect node resolves against (`targets`/`bindings`/`chosen` empty: a
/// cost verb names no targets and carries no trigger context). A
/// `Selection::Choose` inside a verb surfaces its own `ChooseObjects` decision
/// via `run_effect`'s `chosen.is_none()` path.
fn verb_payment_items(verbs: &[PlayerAction], source: ObjectId, player: PlayerId) -> Vec<WorkItem> {
    verbs
        .iter()
        .map(|verb| WorkItem::RunEffect {
            effect: Box::new(Effect::Act(Action::By(Reference::You, verb.clone()))),
            frame: Frame {
                source,
                controller: player,
                targets: vec![],
                bindings: None,
                chosen: None,
                // A cost verb reads no announced X.
                x: None,
            },
        })
        .collect()
}

/// Unwrap the `CostComponent::Do(action)` verbs `concretize` produces for
/// Phyrexian-life picks ([CR#107.4f]) back into the `PlayerAction`s
/// `verb_payment_items` schedules. `concretize` only ever emits
/// `Do(LoseLife(2))` here, so any other shape is an engine invariant violation.
fn phyrexian_life_verbs(verbs: &[CostComponent]) -> Vec<PlayerAction> {
    verbs
        .iter()
        .map(|c| match c {
            CostComponent::Do(action) => action.clone(),
            other => unreachable!("concretize emits only Do(_) verb costs, got {other:?}"),
        })
        .collect()
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
    ) -> bool {
        let options = crate::cost_options::choosable(cost);
        // Recurse over the per-symbol option lists, building one pick per
        // symbol; at a complete pick set, test the assembled concretization.
        self.any_reading_payable(player, cost, subject, &options.options, &mut Vec::new())
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
    ) -> bool {
        let reduced = concretize_x(cost, 0);
        if crate::cost_options::choosable(&reduced).options.is_empty() {
            can_pay(&self.spendable_pool(player, subject), &reduced)
        } else {
            self.affordable_concretization(player, &reduced, subject)
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
    ) -> bool {
        if picks.len() == options.len() {
            return self.reading_payable(player, cost, subject, picks);
        }
        for &choice in &options[picks.len()].choices {
            picks.push(choice);
            let payable = self.any_reading_payable(player, cost, subject, options, picks);
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
    ) -> bool {
        let choices = crate::cost_options::CostOptionChoices {
            picks: picks.to_vec(),
        };
        // A complete, legal pick set always concretizes — `picks` is built from
        // `choosable`'s own options, so the count and legality are guaranteed.
        let Ok((mana, verbs)) = crate::cost_options::concretize(cost, &choices) else {
            return false;
        };
        if !can_pay(&self.spendable_pool(player, subject), &mana) {
            return false;
        }
        let verb_actions = phyrexian_life_verbs(&verbs);
        // Structural per-verb payability (here: each LoseLife is non-negative
        // and life ≥ that ONE amount). `concretize` emits only Do(LoseLife(2)),
        // so this is the [CR#119.4] floor; the joint check below adds the
        // shared-life constraint `can_pay_verbs` can't express.
        if !self.can_pay_verbs(player, &verb_actions, subject) {
            return false;
        }
        // [CR#107.4f]: the COMBINED life of all Phyrexian-life picks must be
        // affordable — two {W/P} paid with life cost 4, not 2. `can_pay_verbs`
        // judges each LoseLife against full life independently, so sum them.
        // The frame mirrors the one `can_pay_verbs`/`verb_payment_items` use: a
        // cost verb names no targets and `~`/`This` is the live source.
        let frame = Frame {
            source: subject,
            controller: player,
            targets: vec![],
            bindings: None,
            chosen: None,
            x: None,
        };
        let life_required: Uint = verb_actions
            .iter()
            .map(|v| self.life_cost_of(v, &frame))
            .sum::<Uint>();
        let life = Uint::try_from(self.player(player).life.max(0)).unwrap_or(Uint::MAX);
        life >= life_required
    }

    /// The life a single concretized Phyrexian-life verb costs. `concretize`
    /// emits only `Do(LoseLife(n))` for life picks ([CR#107.4f]); any other
    /// shape contributes 0 (its own structural check in `can_pay_verbs` covers
    /// it — this sum is purely the shared-life constraint).
    fn life_cost_of(&self, verb: &PlayerAction, frame: &Frame) -> Uint {
        match verb {
            PlayerAction::LoseLife(count) => self.eval_count(count, frame),
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
        let face = crate::derive::face(self.def(object));
        // Lands are never cast as spells — playing a land is a special action
        // ([CR#305.9,116.2a]).
        if face.types.contains(&Type::Land) {
            return false;
        }
        let instant = face.types.contains(&Type::Instant);
        // Sorcery speed for non-instants ([CR#307.1,117.1a]), unless a
        // May(Cast(window: InstantSpeed)) row lifts the default
        // ([CR#702.8a] flash — the card's own row functions from the
        // hand; an Orrery-style battlefield grant rides the same shape).
        // Rows carrying `from`/`cost` slots are different unlocks
        // (cast-from-zones, alternative costs) and never lift timing.
        let proxy = self.player(player).object;
        let timing_ok = instant
            || self.sorcery_speed_ok(player)
            || crate::legal::may_cast_rows(self, view, object)
                .iter()
                .any(|r| {
                    r.window == Some(Window::InstantSpeed)
                        && r.from.is_none()
                        && r.cost.is_none()
                        && self.filter_matches_live(&r.what, object, r.carrier)
                        && self.filter_matches_live(&r.by, proxy, r.carrier)
                });
        if !timing_ok {
            return false;
        }
        // [CR#118.6]: an EMPTY mana cost is "no mana cost" — an unpayable
        // base. Attempting the cast is legal in the CR but pointless to
        // offer; an alternative cost ([CR#118.6a], May(Cast(cost: …)) rows)
        // is the future unlock. {0} is spelled [Generic(0)] and payable
        // ([CR#118.5]).
        if face.mana_cost.is_empty() {
            return false;
        }
        let Some(cost) = self.mana_cost(object) else {
            return false;
        };
        // [CR#601.2b,601.2g,107.3a]: gate mana affordability under all legal
        // readings (concretizes {X} to 0, then plain or hybrid/Phyrexian path).
        if !self.gate_mana_affordable(player, &cost, object) {
            return false;
        }
        // If the spell targets, every spec must admit at least one candidate.
        crate::resolve::spell_targets(view, object)
            .iter()
            .all(|spec| !self.legal_targets(spec).is_empty())
    }

    /// [CR#601.2a,601.2b]: move the spell from its controller's hand to the stack and
    /// open the announce slot. Procedural — not an event.
    ///
    /// # Panics
    ///
    /// Panics if `object` is not in its controller's hand — engine invariant.
    pub(crate) fn begin_cast(&mut self, object: ObjectId) {
        let controller = self.objects.obj(object).controller;
        self.remove_from_hand(controller, object);
        self.objects.obj_mut(object).zone = Some(Zone::Stack);
        self.announcing = Some(PendingStackEntry {
            // [CR#405]: a spell's stack identity is its own object id.
            id: object,
            object: StackObject::Spell(object),
            controller,
            origin: Zone::Hand,
            targets: vec![],
            // [CR#601.2b]: filled by the `ChooseXValue` step before `PayCost`.
            x: None,
            // [CR#601.2b]: filled by the `ChooseCostOptions` step before `PayCost`.
            concretized: None,
        });
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
        let specs: Vec<TargetSpec> = match &pending.object {
            StackObject::Spell(o) => crate::resolve::spell_targets(&self.layers(), *o),
            // The carried ability text is authoritative — never re-derive
            // from the (possibly changed) source.
            StackObject::Activated { ability, .. } => ability.targets.clone(),
            StackObject::Triggered { .. } => {
                unreachable!("triggers announce targets at placement, not in the announce slot")
            }
        };
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
        let view = self.layers();
        let rows = crate::legal::cant_target_rows(self, &view);
        let legal: Vec<Vec<ObjectId>> = specs
            .iter()
            .map(|s| {
                self.legal_targets(s)
                    .into_iter()
                    .filter(|&t| {
                        crate::legal::target_forbidden_by(self, &rows, targeting_id, t).is_none()
                    })
                    .collect()
            })
            .collect();
        let count = Uint::try_from(specs.len()).expect("target-spec count fits in Uint");
        self.pending = Some(PendingDecision::ChooseTargets {
            player,
            spec: specs,
            legal,
        });
        count
    }

    /// [CR#601.2b]: surface a `ChooseXValue` if the in-flight announce's cost has
    /// an `{X}` (`ManaSymbol::Variable`). Runs before `announce_targets`
    /// ([CR#601.2c]). No-op for an X-free cost, so the step is uniform.
    ///
    /// # Panics
    /// Panics if no announce is in flight, or a `Triggered` object occupies the
    /// slot — engine invariants.
    pub(crate) fn announce_x(&mut self) {
        let pending = self.announcing.as_ref().expect("an announce in flight");
        let controller = pending.controller;
        let has_x = match &pending.object {
            StackObject::Spell(o) => self
                .mana_cost(*o)
                .is_some_and(|c| c.iter().any(|s| matches!(s, ManaSymbol::Variable))),
            StackObject::Activated { ability, .. } => crate::activate::cost_summary(&ability.cost)
                .expect("can_activate vetted the cost")
                .mana
                .iter()
                .any(|s| matches!(s, ManaSymbol::Variable)),
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability never occupies the announce slot")
            }
        };
        if has_x {
            self.pending = Some(PendingDecision::ChooseXValue { player: controller });
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
        let cost = match &pending.object {
            StackObject::Spell(o) => self
                .mana_cost(*o)
                .expect("a castable spell has a printed cost"),
            StackObject::Activated { ability, .. } => {
                crate::activate::cost_summary(&ability.cost)
                    .expect("can_activate vetted the cost")
                    .mana
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability has no cost and never occupies the announce slot")
            }
        };
        let options = crate::cost_options::choosable(&cost);
        if options.options.is_empty() {
            // [CR#601.2b]: no multi-way symbol — the cost is already concrete.
            // Stash it (with no Phyrexian-life verbs) so `PayCost` reads the
            // stash uniformly; surface nothing.
            let concrete = crate::cost_options::concretize(
                &cost,
                &crate::cost_options::CostOptionChoices { picks: vec![] },
            )
            .expect("a cost with no choosable symbols needs no picks");
            self.announcing
                .as_mut()
                .expect("an announce in flight")
                .concretized = Some(concrete);
            return false;
        }
        // [CR#601.2b]: the player announces each reading; the submission handler
        // concretizes and stashes.
        self.pending = Some(PendingDecision::ChooseCostOptions {
            player: controller,
            cost,
            options,
        });
        true
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
        let announced_x = pending.x.unwrap_or(0);
        // [CR#601.2b]: the announced concretization — always set by the
        // preceding `ChooseCostOptions` step.
        let (mana, extra_verbs) = pending
            .concretized
            .clone()
            .expect("ChooseCostOptions concretized the cost before PayCost");
        // The Phyrexian-life picks rode as `Do(LoseLife(2))` cost components;
        // unwrap them into payable verbs ([CR#601.2h]).
        let extra_verbs = phyrexian_life_verbs(&extra_verbs);
        match &pending.object {
            StackObject::Spell(o) => {
                let object = *o;
                // [CR#601.2h]: pay the Phyrexian-life verb costs in the payment
                // window — front-scheduled so they sit behind the pending mana
                // decision (if any) and ahead of the `SpellCast` becomes-cast
                // step. The source is the spell object; the payer its
                // controller.
                let items = verb_payment_items(&extra_verbs, object, controller);
                if !items.is_empty() {
                    self.schedule_front(items);
                }
                // [CR#601.2b]: apply the announced X to the concretized mana
                // ({X} -> Generic(announced_x); hybrid/Phyrexian already resolved).
                let mana = concretize_x(&mana, announced_x);
                if !mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(PendingDecision::PayMana {
                        player: controller,
                        cost: mana,
                        pool,
                        // [CR#106.6]: a spell's stack identity is its own id —
                        // the object SpendOnly riders judge.
                        subject: object,
                    });
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
                    items.push(WorkItem::Emit(Occurrence::single(GameEvent::Tapped {
                        object: source,
                        cause: Some(Cause::tap(Agency::CostPayment, Some((source, controller)))),
                    })));
                }
                if summary.untap {
                    items.push(WorkItem::Emit(Occurrence::single(GameEvent::Untapped(
                        source,
                    ))));
                }
                // [CR#601.2h]: every cost-eligible verb (Sacrifice, LoseLife,
                // Discard, …) is performed now, by the activating player,
                // against the ability's source. One `RunEffect` per verb, after
                // the {T}/{Q} events and before `AbilityActivated`. The
                // ability's own verb costs come first, then the
                // concretization's Phyrexian-life verbs.
                items.extend(verb_payment_items(&summary.verbs, source, controller));
                items.extend(verb_payment_items(&extra_verbs, source, controller));
                if !items.is_empty() {
                    self.schedule_front(items);
                }
                // [CR#601.2b]: apply the announced X to the concretized mana
                // (hybrid/Phyrexian already resolved by ChooseCostOptions).
                let mana = concretize_x(&mana, announced_x);
                if !mana.is_empty() {
                    let pool = self.player(controller).mana_pool.clone();
                    self.pending = Some(PendingDecision::PayMana {
                        player: controller,
                        // [CR#601.2b]: the concretized mana (hybrid/Phyrexian
                        // resolved, {X} applied), not the printed cost.
                        cost: mana,
                        pool,
                        // [CR#106.6]: an activated ability's mana is spent on
                        // its source — that is the object SpendOnly judges.
                        subject: source,
                    });
                }
            }
            StackObject::Triggered { .. } => {
                unreachable!("a triggered ability has no cost and never occupies the announce slot")
            }
        }
    }

    /// The card face's printed mana cost ([CR#202]). `None` would mark an
    /// uncastable object; every card face carries a (possibly empty) cost, so
    /// this is always `Some` today — the option leaves room for future
    /// faces/zones that have no castable cost (and lets `can_cast`/`pay_cost`
    /// share the `let Some(cost) = …` gate).
    #[must_use]
    #[allow(
        clippy::unnecessary_wraps,
        reason = "the Option is the cast-legality seam; future no-cost faces return None (now a pub API, so clippy may not fire — keep the seam documented)"
    )]
    pub fn mana_cost(&self, object: ObjectId) -> Option<ManaCost> {
        Some(crate::derive::face(self.def(object)).mana_cost.clone())
    }

    /// [CR#115]: the legal candidates for a single `TargetSpec` (its filter's
    /// matching objects, in id order).
    ///
    /// Delegates filter extraction to `resolve::target_spec_filter` so that
    /// announce-time and resolution-time `TargetSpec` handling stay in sync.
    ///
    /// # Panics
    ///
    /// Panics on `TargetSpec` variants other than `Target` or `Expanded` —
    /// only those are wired for Stage 2.
    #[must_use]
    pub(crate) fn legal_targets(&self, spec: &TargetSpec) -> Vec<ObjectId> {
        let filter = crate::resolve::target_spec_filter(spec);
        candidates(self, filter)
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
            Some(PendingDecision::PayMana {
                cost,
                pool,
                subject,
                ..
            }) => {
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
    fn unit_spendable_on(&self, unit: &crate::player::ManaUnit, subject: ObjectId) -> bool {
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
            p.add(m, n);
        }
        p
    }
    /// A pool whose units each carry `ManaRider::Snow` ([CR#107.4h]) — produced
    /// by a snow source, so eligible to pay `{S}` (and, being otherwise normal
    /// mana, colored/generic too).
    fn snow_pool(pairs: &[(ColorOrColorless, Uint)]) -> ManaPool {
        let units = pairs
            .iter()
            .flat_map(|&(m, n)| {
                (0..n).map(move |_| crate::player::ManaUnit {
                    kind: m,
                    riders: vec![deckmaste_core::ManaRider::Snow],
                })
            })
            .collect();
        ManaPool::from_units(units)
    }
    fn cost(s: &str) -> ManaCost { s.parse().unwrap() }
    fn red() -> ColorOrColorless { Color::Red.into() }
    fn green() -> ColorOrColorless { Color::Green.into() }

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
            crate::player::ManaUnit {
                kind: red(),
                riders: vec![deckmaste_core::ManaRider::Snow],
            },
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![],
            },
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
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![],
            },
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![deckmaste_core::ManaRider::Snow],
            },
        ];
        assert!(can_pay(&ManaPool::from_units(units), &cost("{G}{S}")));
        // ONE snow green alone canNOT pay {G}{S}: needs two units (one per pip).
        assert!(!can_pay(&snow_pool(&[(green(), 1)]), &cost("{G}{S}")));
    }

    #[test]
    fn validate_payment_snow_round_trips() {
        // Pool [snow-red (0), plain green (1)] against {1}{S}.
        let p = ManaPool::from_units(vec![
            crate::player::ManaUnit {
                kind: red(),
                riders: vec![deckmaste_core::ManaRider::Snow],
            },
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![],
            },
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
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![],
            },
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![deckmaste_core::ManaRider::Snow],
            },
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
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![],
            },
            crate::player::ManaUnit {
                kind: green(),
                riders: vec![deckmaste_core::ManaRider::Snow],
            },
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
}
