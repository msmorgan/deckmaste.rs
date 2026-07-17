//! Activating non-mana activated abilities ([CR#602]): the legality gate and
//! the staged announce (`begin_activate`), which mirrors `cast.rs`
//! ([CR#602.2b]: activation follows the [CR#601.2] steps). Mana abilities
//! never come here: they are stackless ([CR#605.3b]) and keep their fast
//! path.

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Cmp;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PlayerAction;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Stat;
use deckmaste_core::Uint;
use deckmaste_core::UseLimit;
use deckmaste_core::Zone;

use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::stack::Frame;
use crate::stack::PendingStackEntry;
use crate::stack::StackObject;
use crate::state::GameState;
use crate::trigger::TriggerBindings;

/// Look through `Expanded` wrappers to an activated ability, if that is what
/// this is (keyword macros expand to the abilities they grant).
#[must_use]
pub(crate) fn as_activated(ability: &Ability) -> Option<&ActivatedAbility> {
    match ability {
        Ability::Activated(a) => Some(a),
        Ability::Expanded(e) => as_activated(&e.value),
        _ => None,
    }
}

/// True iff `cost` pays with a loyalty-counter verb — `Do(PutCounters(This,
/// LoyaltyCounter, _))` or `Do(RemoveCounters(This, LoyaltyCounter, _))`
/// ([CR#606.4], no dedicated loyalty-cost kind — loyalty costs are plain
/// `PutCounters`/`RemoveCounters` on the source, see `idris/src/Core.idr`'s
/// `Cost` `Do` ruling). This is how [`GameState::can_activate`]'s
/// `UseLimit::LoyaltyOncePerTurn` arm recognizes a permanent's OTHER loyalty
/// abilities ([CR#606.3,306.5d]) among its full ability list. Reuses
/// [`cost_summary`] so this can never diverge from the payment path's own
/// reading of the cost.
#[must_use]
pub(crate) fn is_loyalty_ability(cost: &deckmaste_core::Cost) -> bool {
    let Some(summary) = cost_summary(&cost.0) else {
        return false;
    };
    let loyalty = deckmaste_core::CounterRef::from("LoyaltyCounter");
    summary.verbs.iter().any(|v| {
        matches!(
            v,
            Action::By(
                _,
                PlayerAction::PutCounters(Reference::This, counter, _)
                    | PlayerAction::RemoveCounters(Reference::This, counter, _),
            ) if *counter == loyalty
        )
    })
}

/// One pass over an activation cost ([CR#602.2b,601.2f..601.2h]): the summed
/// mana, the {T}/{Q} components, and cost-eligible verb actions. `None` when a
/// component is not payable (a non-eligible `Do(...)` verb). Loyalty `+N`/`−N`
/// costs are cost-eligible `PutCounters`/`RemoveCounters` and pay through the
/// verb path like any other ([CR#606.4]).
pub(crate) struct CostSummary {
    pub mana: ManaCost,
    pub tap: bool,
    pub untap: bool,
    /// Cost-eligible verb components ([`Action::is_cost_eligible`]): the
    /// player verbs (Sacrifice, Exile, Tap, Untap, `LoseLife`,
    /// `RemoveCounters`, Reveal — as implicit-you `By` actions) and the
    /// discard keyword-action composite ("Discard a card:", [CR#701.9]).
    /// Collected for payment; non-eligible `Do(_)` causes `cost_summary` to
    /// return `None`.
    pub verbs: Vec<Action>,
    /// `ManaCostOf(reference)` components: pay mana equal to the referenced
    /// object's printed mana cost ([CR#202.1]). The reference can only be
    /// resolved against a live frame, so it is collected here and folded into
    /// the mana to pay by [`GameState::resolve_cost_mana`] at the gate and the
    /// payment step.
    pub mana_cost_of: Vec<Reference>,
    /// Aggregate-stat (tap-total) requirements ([CR#702.122a] Crew): each is a
    /// "tap a subset of [filter] whose summed [stat] satisfies [cmp] [count]"
    /// obligation. Like `ManaCostOf`, it can only be checked/paid against a
    /// live frame, so it is collected here and resolved by
    /// [`GameState::tap_total_subset`] at the gate (feasibility) and the
    /// payment step (which subset to tap).
    pub tap_totals: Vec<TapTotalReq>,
    /// Cost-side choose-then-pay steps ([CR#601.2b], `CostComponent::With`):
    /// "sacrifice a creature" = `With(ChooseOne(Creature),
    /// [Do(Sacrifice(That(Permanent)))])`. The binder makes a choice (bound as
    /// `That`/`Those`) the body's verbs pay against — choosing kept OUT of
    /// the verb. Collected verbatim because the choice can only be surfaced
    /// against a live frame: the gate ([`GameState::can_activate`]) checks
    /// the binder's choose-feasibility, and the pay step
    /// ([`GameState::pay_cost`]) runs each as an `OneShotEffect::With`
    /// (which surfaces `ChooseObjects` and binds `frame.those`, exactly like
    /// the effect-side `With`).
    pub withs: Vec<CostComponent>,
}

/// One aggregate-stat (tap-total) cost obligation collected by [`cost_summary`]
/// ([CR#601.2b,702.122a]): tap a chosen subset of `filter`'s untapped matches
/// whose summed `stat` satisfies `cmp` `count`. The owned twin of
/// `CostComponent::TapTotal`, hoisted into the summary so the gate and the pay
/// step share one reading.
pub(crate) struct TapTotalReq {
    pub stat: Stat,
    pub cmp: Cmp,
    pub count: Count,
    pub filter: Predicate,
}

/// Summarize `cost` in one walk (so the `can_activate` gate and the pay step
/// can never diverge). `Expanded` macro wrappers are looked through, and a
/// nested `Cost` (the macro list-splice shape, e.g. cycling — read is faithful,
/// so it arrives lumpy) is recursed into: this walk doubles as the cost's
/// normalization, splicing nested components into the summary rather than
/// requiring a separate `Cost::normalize` clone at the call site.
#[must_use]
pub(crate) fn cost_summary(cost: &[CostComponent]) -> Option<CostSummary> {
    let mut symbols: Vec<ManaSymbol> = Vec::new();
    let mut tap = false;
    let mut untap = false;
    let mut verbs: Vec<Action> = Vec::new();
    let mut mana_cost_of: Vec<Reference> = Vec::new();
    let mut tap_totals: Vec<TapTotalReq> = Vec::new();
    let mut withs: Vec<CostComponent> = Vec::new();
    for component in cost {
        match component {
            CostComponent::Mana(m) => symbols.extend_from_slice(m),
            // Resolved against a live frame at the gate / payment step
            // (`resolve_cost_mana`): the cost walk has no game state, so the
            // reference is collected, not read, here.
            CostComponent::ManaCostOf(reference) => mana_cost_of.push(reference.clone()),
            CostComponent::Tap => tap = true,
            CostComponent::Untap => untap = true,
            // An aggregate-stat cost ([CR#702.122a] Crew): collect the
            // requirement; feasibility and the subset to tap are resolved
            // against a live frame at the gate / pay step.
            CostComponent::TapTotal {
                stat,
                cmp,
                count,
                filter,
            } => tap_totals.push(TapTotalReq {
                stat: *stat,
                cmp: *cmp,
                count: count.clone(),
                filter: (**filter).clone(),
            }),
            CostComponent::Do(action) => {
                if action.is_cost_eligible() {
                    verbs.push(*action.clone());
                } else {
                    // Non-eligible verbs in a cost are malformed.
                    return None;
                }
            }
            // A cost-side choose-then-pay step ([CR#601.2b]) — bind the choice
            // as `That`/`Those`, then pay the body. Collected verbatim: the
            // choice can only be surfaced against a live frame, so the gate
            // checks the binder's choose-feasibility and the pay step runs each
            // as an `OneShotEffect::With` (mirroring `ManaCostOf`/`TapTotal`).
            CostComponent::With { .. } => withs.push(component.clone()),
            // Recurse through macro wrappers.
            CostComponent::Expanded(e) => {
                let inner = cost_summary(std::slice::from_ref(&e.value))?;
                symbols.extend_from_slice(&inner.mana);
                tap |= inner.tap;
                untap |= inner.untap;
                verbs.extend(inner.verbs);
                mana_cost_of.extend(inner.mana_cost_of);
                tap_totals.extend(inner.tap_totals);
                withs.extend(inner.withs);
            }
            // A nested cost (the macro list-splice shape) survives faithful
            // read; recurse to splice it into the summary — this walk is the
            // pay path's `Cost::normalize`, inlined.
            CostComponent::Cost(nested) => {
                let inner = cost_summary(&nested.0)?;
                symbols.extend_from_slice(&inner.mana);
                tap |= inner.tap;
                untap |= inner.untap;
                verbs.extend(inner.verbs);
                mana_cost_of.extend(inner.mana_cost_of);
                tap_totals.extend(inner.tap_totals);
                withs.extend(inner.withs);
            }
        }
    }
    Some(CostSummary {
        mana: ManaCost::from(symbols),
        tap,
        untap,
        verbs,
        mana_cost_of,
        tap_totals,
        withs,
    })
}

/// A minimal subset of `(object, stat)` candidates whose summed stat satisfies
/// `cmp` `need`, or `None` when no subset does ([CR#702.122a] Crew payment).
///
/// Greedy from the highest stat (a stable sort, so equal stats keep id order):
/// for the lower-bound comparators a card actually uses (`AtLeast`/`Greater` —
/// "total power N or greater"), adding the largest contributors reaches the
/// bound with the fewest taps, and once it holds it stays held. The empty
/// subset is returned when `cmp 0 need` already holds (e.g. `AtMost`, or a zero
/// bound), since tapping nothing is then a legal payment.
fn greedy_tap_subset(
    mut candidates: Vec<(ObjectId, Uint)>,
    cmp: Cmp,
    need: Uint,
) -> Option<Vec<ObjectId>> {
    let mut sum: Uint = 0;
    let mut chosen: Vec<ObjectId> = Vec::new();
    if cmp.apply(sum, need) {
        return Some(chosen);
    }
    // Highest stat first; stable, so equal-stat ties keep the id order
    // `candidates_with` produced.
    candidates.sort_by_key(|&(_, stat)| std::cmp::Reverse(stat));
    for (id, stat) in candidates {
        chosen.push(id);
        sum = sum.saturating_add(stat);
        if cmp.apply(sum, need) {
            return Some(chosen);
        }
    }
    None
}

impl GameState {
    /// The mana a cost summary requires, with every `ManaCostOf(reference)`
    /// resolved ([CR#202.1]): the literal `Mana(...)` symbols plus, for each
    /// referenced object, that object's printed mana cost — the colored
    /// cost-language twin of `ManaValueOf`. `source`/`controller` anchor the
    /// references (`This` is the cost's source, `You` its payer), mirroring the
    /// announce-gate frame. Cheap-paths to the summary's own mana when there is
    /// no `ManaCostOf` component, so a plain cost is untouched.
    #[must_use]
    pub(crate) fn resolve_cost_mana(
        &self,
        summary: &CostSummary,
        source: ObjectId,
        controller: PlayerId,
    ) -> ManaCost {
        if summary.mana_cost_of.is_empty() {
            return summary.mana.clone();
        }
        let frame = Frame::bare(source, controller);
        let mut symbols: Vec<ManaSymbol> = summary.mana.iter().copied().collect();
        for reference in &summary.mana_cost_of {
            let object = self.eval_reference(reference, &frame);
            if let Some(printed) = self.mana_cost(object) {
                symbols.extend_from_slice(&printed);
            }
        }
        ManaCost::from(symbols)
    }

    /// [CR#602.1,602.5]: may `player` activate this non-mana activated
    /// ability of `object` right now? `index` is the position in the derived
    /// ability list (the ledger key).
    #[must_use]
    pub(crate) fn can_activate(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
        index: usize,
        ability: &ActivatedAbility,
    ) -> bool {
        // [CR#601.2g,602.2b,106.6]: the pool must be able to pay the mana cost.
        // Only mana spendable on this ability's source can fund it — restrict
        // the affordability check to the spendable sub-pool.
        let Some(summary) = cost_summary(&ability.cost) else {
            return false;
        };
        // [CR#601.2b,601.2g,107.3a]: gate mana affordability under all legal
        // readings (concretizes {X} to 0, then plain or hybrid/Phyrexian path).
        // `ManaCostOf` components ([CR#202.1]) are resolved against the live
        // source here so "pay mana equal to its mana cost" gates on the real
        // amount, not a free read.
        let mana = self.resolve_cost_mana(&summary, object, player);
        // No `PayPips` — convoke / delve / improvise are cast-time spell
        // statics, never activated-ability costs ([CR#702.51a]) — so pip
        // payment does not enter the affordability gate here.
        if !self.gate_mana_affordable(player, &mana, object, None) {
            return false;
        }

        let obj = self.objects.obj(object);

        // A tapped object cannot pay {T}; an untapped object cannot pay {Q}.
        if summary.tap && obj.tapped {
            return false;
        }
        if summary.untap && !obj.tapped {
            return false;
        }

        // [CR#602.5a]: a conferred `Cant(Activate(cost: IncludesTapSymbol))`
        // forbids paying a {T}/{Q} cost — the summoning-sickness tap gate a
        // `Creature` type confers (haste-exempt). Keyed on the capability, not a
        // `Type::Creature` literal; a non-tap cost is never gated here.
        if crate::legal::cant_activate(self, view, object, player, summary.tap || summary.untap) {
            return false;
        }

        // [CR#602.5d..602.5e]: activation window ("Activate only as a
        // sorcery" — the Only refinement on the activation permission).
        if let Some(window) = &ability.window {
            let in_window = match window {
                deckmaste_core::Timing::InstantSpeed => true,
                deckmaste_core::Timing::SorcerySpeed => self.sorcery_speed_ok(player),
                // [CR#500.1]: "Activate only during [relation]'s turn" — a
                // pure predicate over the active player and the named
                // `WhoseTurn` relation, mirroring the `WhoseTurn` reading in
                // `EventFilter::StepBegins` (eval.rs).
                //
                // DIVERGENCE (documented, not fixed): forecast's CR text
                // anchors this window to the card's OWNER ([CR#702.57b]:
                // "...may be activated only during the upkeep step of the
                // card's owner..."), but `WhoseTurn` only expresses a
                // controller/activator-relative relation — `player` here is
                // the activating player, i.e. the object's CURRENT
                // controller, and there is no owner-relative `WhoseTurn`
                // variant. So a stolen forecast-style card's window follows
                // its controller's turn, not its owner's, until `WhoseTurn`
                // grows an owner-relative arm.
                deckmaste_core::Timing::DuringTurn(whose) => {
                    self.whose_turn_matches(*whose, player)
                }
                // [CR#602.5d..602.5e,500.1]: as `DuringTurn`, plus the
                // current step must be the named one exactly
                // (forecast-style, [CR#702.57b] — see the DIVERGENCE note
                // above, which applies here too).
                deckmaste_core::Timing::DuringStep(step, whose) => {
                    self.turn.current == *step && self.whose_turn_matches(*whose, player)
                }
            };
            if !in_window {
                return false;
            }
        }

        // [CR#602.5b..602.5e]: activation condition ("Activate only if …").
        // The gate runs before targets are chosen, so the frame carries none;
        // `Ref(This)`/`Is(This, …)` anchors to the live source.
        if let Some(c) = &ability.condition {
            let frame = Frame::bare(object, player);
            if !self.condition_holds(c, &frame) {
                return false;
            }
        }

        // [CR#602.5b]: use limits — gate via the turn/game history window.
        let index_u = deckmaste_core::Uint::try_from(index).expect("ability index fits in Uint");
        for limit in &ability.limits {
            match limit {
                UseLimit::OncePerTurn => {
                    if self.ability_used_count(object, index_u, deckmaste_core::Lookback::ThisTurn)
                        >= 1
                    {
                        return false;
                    }
                }
                UseLimit::OncePerGame => {
                    if self.ability_used_count(object, index_u, deckmaste_core::Lookback::ThisGame)
                        >= 1
                    {
                        return false;
                    }
                }
                // [CR#606.3,306.5d]: shared across every loyalty ability of
                // this permanent — not the one at `index` alone. Enumerate
                // the object's OTHER derived activated abilities (the same
                // `derive::usable_abilities` list `index`/`ability_used_count`
                // are keyed against, [CR#602.5b]) and block if any loyalty
                // one among them already fired this turn.
                UseLimit::LoyaltyOncePerTurn => {
                    let siblings = crate::derive::usable_abilities(self, object);
                    let any_loyalty_used = siblings.iter().enumerate().any(|(i, a)| {
                        as_activated(a).is_some_and(|act| {
                            is_loyalty_ability(&act.cost)
                                && self.ability_used_count(
                                    object,
                                    Uint::try_from(i).expect("ability index fits in Uint"),
                                    deckmaste_core::Lookback::ThisTurn,
                                ) >= 1
                        })
                    });
                    if any_loyalty_used {
                        return false;
                    }
                }
            }
        }

        // [CR#601.2c,602.2b,115.7e]: the target specs must be jointly
        // satisfiable — each slot offering at least its minimum count, the
        // Distinct slots admitting distinct representatives. The carrier is the
        // activation object's source — anchors a target filter's carrier-
        // relative self-reference (`Ref(This)`, `StatOf(This, …)`).
        let carrier = Some(self.objects.obj(object).source);
        let specs = crate::resolve::top_targets(&ability.effect);
        let legal: Vec<Vec<ObjectId>> = specs
            .iter()
            .map(|spec| self.legal_targets(spec, carrier))
            .collect();
        if !crate::resolve::announce_satisfiable(specs, &legal) {
            return false;
        }

        // [CR#601.2h,118.3]: the non-mana verb/life costs must be fully
        // payable too — partial payment is forbidden.
        if !self.can_pay_verbs(player, &summary.verbs, object) {
            return false;
        }

        // [CR#601.2b,601.2h]: every cost-side `With` choose-then-pay step must
        // have a legal choice — the choose-feasibility that used to live in the
        // verb ("sacrifice a creature" needs a creature to choose). A directly-
        // resolved binder (`TheRef`/`Existing`) is always feasible.
        if !summary
            .withs
            .iter()
            .all(|w| self.with_cost_feasible(w, object, player))
        {
            return false;
        }

        // [CR#601.2h,702.122a]: every aggregate-stat (tap-total) cost must have
        // a qualifying untapped subset to tap (Crew: enough total power) —
        // partial payment is forbidden, so an infeasible requirement bars
        // activation.
        summary
            .tap_totals
            .iter()
            .all(|req| self.tap_total_subset(req, object, player).is_some())
    }

    /// [CR#500.1]: does the current active player stand in the named
    /// `WhoseTurn` relation to `activator`? The `Timing::DuringTurn`/
    /// `DuringStep` window-gate predicate — mirrors the `WhoseTurn` reading
    /// in `EventFilter::StepBegins` (eval.rs): `Your` compares equal,
    /// `AnOpponents` compares unequal (no team modeling yet, so "opponent"
    /// is exactly "a different player", same as `same_team`'s seam),
    /// `EachPlayers` is unconditional.
    #[must_use]
    fn whose_turn_matches(&self, whose: deckmaste_core::WhoseTurn, activator: PlayerId) -> bool {
        let active = self.turn.active_player;
        match whose {
            deckmaste_core::WhoseTurn::Your => active == activator,
            deckmaste_core::WhoseTurn::AnOpponents => active != activator,
            deckmaste_core::WhoseTurn::EachPlayers => true,
        }
    }

    /// [CR#601.2b,601.2h]: is the cost-side `With`'s binder a payable choice for
    /// `source`'s controller? A chooser binder (`ChooseOne`/`Choose`) needs
    /// enough legal candidates matching its filter — ≥ 1 for `ChooseOne`, ≥ the
    /// quantity's lower bound for `Choose` (partial payment is forbidden, so
    /// "sacrifice two creatures" with one creature is unpayable). A directly-
    /// resolved binder (`TheRef`/`Existing`) names its object(s) and is always
    /// feasible. `with` is a `CostComponent::With` (the only shape
    /// [`cost_summary`] collects into `withs`).
    fn with_cost_feasible(
        &self,
        with: &CostComponent,
        source: ObjectId,
        controller: PlayerId,
    ) -> bool {
        use deckmaste_core::Binder;
        let CostComponent::With { binder, .. } = with else {
            unreachable!("cost_summary collects only With components into `withs`");
        };
        match crate::resolve::peel_binder(binder) {
            // A captured reference / existing selection resolves directly.
            Binder::TheRef(_) | Binder::Existing(_) => true,
            // ≥ 1 candidate to choose ([CR#601.2b]).
            Binder::ChooseOne { filter, .. } => !crate::target::candidates(self, filter).is_empty(),
            // ≥ the quantity's lower bound of candidates (no partial payment).
            Binder::Choose {
                quantity, filter, ..
            } => {
                let candidates = crate::target::candidates(self, filter);
                let frame = Frame::bare(source, controller);
                let (lo, _hi) = quantity.bounds();
                let need = lo.map_or(0, |c| self.eval_count(c, &frame));
                Uint::try_from(candidates.len()).unwrap_or(Uint::MAX) >= need
            }
            // SEAM: producer/search cost binders — payability needs the same
            // produce-and-capture / library-search primitive the resolution
            // spine lacks, so it can't be decided here. No corpus card uses
            // these in a cost; an explicit labeled arm keeps a future use a
            // loud seam rather than a silent `true`/`false`.
            Binder::Produce(_) | Binder::Search { .. } | Binder::SearchOne { .. } => {
                unimplemented!(
                    "Binder::{{Produce,Search,SearchOne}} as a cost binder: producer/search binders \
                 are not yet wired (no runtime produce-and-capture / library-search primitive)"
                )
            }
            Binder::Expanded(_) => unreachable!("peeled above"),
        }
    }

    /// The subset of untapped permanents `player` would tap to pay one
    /// aggregate-stat cost `req` for `source` ([CR#601.2h,702.122a] Crew), or
    /// `None` when no qualifying subset exists (the cost is unpayable). The
    /// candidates are `req.filter`'s untapped matches (anchored on `source` so
    /// `Ref(You)`/`Ref(This)` resolve), each contributing its derived
    /// `req.stat`; the greedy reading taps the fewest (highest-stat first)
    /// that meet `req.cmp` `req.count`.
    ///
    /// The payer is entitled to *choose* which qualifying permanents to tap
    /// ([CR#601.2h]); this picks a deterministic minimal subset. The
    /// interactive choice is a follow-up seam (it needs a new payment-time
    /// decision point) — the chosen subset is always a legal payment.
    pub(crate) fn tap_total_subset(
        &self,
        req: &TapTotalReq,
        source: ObjectId,
        controller: PlayerId,
    ) -> Option<Vec<ObjectId>> {
        let frame = Frame::bare(source, controller);
        let need = self.eval_count(&req.count, &frame);
        let watcher = self.objects.obj(source).source;
        let view = self.layers();
        let candidates: Vec<(ObjectId, Uint)> = crate::target::candidates_with(self, &req.filter, Some(watcher))
                .into_iter()
                // Only a *permanent* (an object on the battlefield, [CR#110.1])
                // may be tapped to pay: `candidates_with` is zone-agnostic and
                // `ControlledBy(You)` matches the `controller` field on a
                // library/hand object too, so without this guard a controlled
                // non-battlefield creature would wrongly qualify to crew
                // ([CR#702.122a]). Mirrors the `PayPips` zone guard in cast.rs.
                .filter(|&id| self.objects.obj(id).zone == Some(Zone::Battlefield))
                // Only an *untapped* permanent can be tapped to pay ([CR#107.5]).
                .filter(|&id| !self.objects.obj(id).tapped)
                .filter_map(|id| self.cost_stat_value(&view, id, req.stat).map(|v| (id, v)))
                .collect();
        greedy_tap_subset(candidates, req.cmp, need)
    }

    /// The derived numeric value of `stat` for the card-backed object `id`,
    /// clamped to a non-negative [`Uint`] ([CR#107.1b]) — the aggregate-stat
    /// cost summand. `None` for a non-card object (a player proxy has no stat)
    /// or a stat axis whose engine machinery is unbuilt (loyalty/defense). The
    /// view is built once by the caller and threaded in.
    fn cost_stat_value(
        &self,
        view: &crate::layer::LayeredView,
        id: ObjectId,
        stat: Stat,
    ) -> Option<Uint> {
        // A non-card object (a player proxy) has no stat.
        self.objects.obj(id).card_id()?;
        let raw: Option<deckmaste_core::Int> = match stat {
            Stat::Power => view.power(id),
            Stat::Toughness => view.toughness(id),
            Stat::ManaValue => deckmaste_core::Int::try_from(
                crate::derive::face(self.def(id)).mana_cost.mana_value(),
            )
            .ok(),
            // [CR#209.1,306.5a]: loyalty is the PRINTED loyalty characteristic
            // off the card face — never the live counter count (current loyalty
            // is `CounterCount(This, LoyaltyCounter)`). `base_stat` maps
            // `Number(n)→n`, `DefinedByAbility`/`Variable`/absent → 0.
            Stat::Loyalty => {
                crate::layer::base_stat(crate::derive::face(self.def(id)).loyalty.as_ref())
            }
            Stat::Defense => deckmaste_core::Int::try_from(
                self.objects
                    .obj(id)
                    .counters
                    .get("DefenseCounter")
                    .copied()
                    .unwrap_or(0),
            )
            .ok(),
        };
        raw.map(|v| Uint::try_from(v.max(0)).unwrap_or(0))
    }

    /// [CR#601.2h,118.3]: can `player` fully pay every cost-eligible verb in
    /// `verbs`, with `subject` as the cost's source (`~`/`This`)? Each verb's
    /// payment is all-or-nothing, so this is `true` only when *every* verb is
    /// satisfiable. The frame mirrors the condition gate's: the source is the
    /// activation's object, the controller is the payer, and no targets are
    /// chosen yet.
    #[must_use]
    pub(crate) fn can_pay_verbs(
        &self,
        player: PlayerId,
        verbs: &[Action],
        subject: ObjectId,
    ) -> bool {
        // Same anchoring as the condition gate (`can_activate` above): the
        // payer is the controller, `~`/`This` is the live source.
        // [CR#601.2b]: X has not been announced yet at the gate, so read it at
        // its floor of 0 — the cheapest reading, mirroring how the mana gate
        // "concretizes {X} to 0". A `Count::X` cost verb (a loyalty `−X`) is
        // then payable for X=0 (remove 0 counters), so the ability is offered;
        // the actual announced X is bound and paid at `pay_cost`. Without this,
        // `eval_count(Count::X, …)` on an X-less frame would panic.
        let mut frame = Frame::bare(subject, player);
        frame.anaphora.x = Some(0);
        // TODO(engine-cost-payment / deontics): [CR#119.8] "can't pay life" is
        // NOT YET ENFORCED. Under a continuous effect saying a player can't lose
        // life, a cost that involves having that player pay life can't be paid —
        // so a `Do(LoseLife(..))` cost (and a Phyrexian-life reading, which
        // concretizes to `Do(LoseLife(2))`) should be UNPAYABLE for that player
        // while the mana reading stays available. The deontic layer has no
        // pay-life / lose-life `DeonticAction` variant today (it models only
        // attack/block/target/attach/cast/play/activate), so there is nothing
        // cheap to query here. When that lock is built, gate the `LoseLife` arm
        // of `verb_cost_payable` (and the Phyrexian-life sum in
        // `reading_payable`) on it. See `cant_pay_life_lock_is_a_documented_seam`.
        verbs
            .iter()
            .all(|verb| self.verb_cost_payable(verb, player, &frame))
    }

    /// Whether one cost-eligible action can be paid in full ([CR#601.2h]).
    /// A player verb (`By(You, …)`) defers to the verb match below; the
    /// discard keyword-action composite ([CR#701.9,601.2h]) needs at least
    /// `count` cards in hand for the chosen form (partial payment is
    /// forbidden), while the bound "discard this card" form
    /// ([CR#702.29a]) always names its one card, so it is payable like the
    /// other single-`Reference` verbs.
    fn verb_cost_payable(&self, verb: &Action, player: PlayerId, frame: &Frame) -> bool {
        match verb {
            Action::By(_, pa) => self.player_verb_cost_payable(pa, player, frame),
            // The direct-variant relocation (`Do(Move(This, Exile))`,
            // Scavenge) — a single `Reference` always names its one object,
            // so it is payable, like the `By`-wrapped verbs.
            Action::Move(..) => true,
            Action::Composite { name, body } if name.as_str() == "Discard" => {
                if deckmaste_core::discard_body_what(body).is_some() {
                    // The bound form names its one card ([CR#702.29a]) —
                    // payable; choose-feasibility isn't its concern.
                    true
                } else {
                    // [CR#601.2h]: the chosen form needs the full count (read
                    // off the body's `With` binder's `Quantity`).
                    let need = deckmaste_core::discard_body_count(body)
                        .map_or(0, |count| self.eval_count(count, frame))
                        as usize;
                    self.zones.hands[player.index()].len() >= need
                }
            }
            // `cost_summary` only collects cost-eligible actions, so nothing
            // else reaches here.
            other => unreachable!("non-cost-eligible action in a cost summary: {other:?}"),
        }
    }

    /// Whether one cost-eligible PLAYER verb can be paid in full
    /// ([CR#601.2h]). Looks through `Expanded` macro wrappers.
    fn player_verb_cost_payable(
        &self,
        verb: &PlayerAction,
        player: PlayerId,
        frame: &Frame,
    ) -> bool {
        #[expect(
            clippy::match_same_arms,
            reason = "the always-payable verb groups are kept separate to carry their distinct scope/TODO comments (Sacrifice/Move/Tap/Untap vs the loyalty-`+N` PutCounters arm vs the out-of-scope Reveal seam)"
        )]
        match verb {
            // [CR#119.4]: pay-life needs life ≥ the amount; [CR#119.4b]: paying
            // 0 is always allowed (and `life >= 0` holds trivially).
            PlayerAction::LoseLife(count) => {
                let amount = self.eval_count(count, frame);
                // [CR#119.4,119.4b]: compare in Uint space — negative life can
                // never be ≥ a non-negative amount, so clamp to 0 before
                // converting. `unwrap_or(Uint::MAX)` keeps this panic-free.
                let life = deckmaste_core::Uint::try_from(self.player(player).life.max(0))
                    .unwrap_or(deckmaste_core::Uint::MAX);
                life >= amount
            }
            // Sacrifice/Move (exile is `Move(_, Exile)`)/Tap/Untap take a single
            // `Reference` — it always names its one object, so it is payable.
            // The choose-feasibility of "sacrifice a creature" lives in the
            // cost `With(ChooseOne(filter), …)` binder, not the verb.
            PlayerAction::Sacrifice(_)
            | PlayerAction::Move(..)
            | PlayerAction::Tap(_)
            | PlayerAction::Untap(_) => true,
            // `PutCounters` as a cost (a loyalty `+N` ability adds that many
            // loyalty counters to its source, [CR#606.4]) is always
            // payable — adding counters needs no prior resource.
            PlayerAction::PutCounters(..) => true,
            // [CR#601.2h,107.14]: removing counters as a cost needs at least
            // that many present on the carrier — the loyalty `−N` ability
            // ([CR#606.6]) and "pay {E}" ([CR#107.14]: paying {E} removes an
            // energy counter from the player). The carrier is the resolved
            // object OR player proxy ([CR#122.1] — a counter is a marker on an
            // object OR player, so energy/poison sit on the player), so
            // `RemoveCounters(You, Energy, N)` reads the payer's
            // proxy counter map. An absent kind reads zero, so an unfunded
            // "pay {E}" is unpayable (partial payment forbidden, [CR#601.2h]).
            PlayerAction::RemoveCounters(sel, kind, count) => {
                let need = self.eval_count(count, frame);
                self.eval_reference_set(sel, frame).iter().all(|&id| {
                    self.objects
                        .get(id)
                        .and_then(|o| o.counters.get(kind.as_str()).copied())
                        .unwrap_or(0)
                        >= need
                })
            }
            // Out of this ticket's listed scope — the reveal window is unbuilt,
            // so treat as payable for now.
            // TODO(engine-cost-payment follow-up): payability for Reveal (needs
            // the reveal window).
            PlayerAction::Reveal { .. } => true,
            // Look through a remembered macro invocation.
            PlayerAction::Expanded(e) => self.player_verb_cost_payable(&e.value, player, frame),
            // `cost_summary` only collects cost-eligible verbs, so nothing else
            // reaches here.
            other => unreachable!("non-cost-eligible verb in a cost summary: {other:?}"),
        }
    }

    /// [CR#602.2a,602.2b]: stage a non-mana activated ability — snapshot the
    /// ability text and the source's LKI into the announce slot. The shared
    /// `AnnounceTargets`/`PayCost` items follow; `AbilityActivated` promotes
    /// it onto the stack.
    ///
    /// The source must be a battlefield permanent — `legal_actions` only offers
    /// battlefield activations; `origin` and the LKI capture assume a zoned
    /// object. Activating from other zones (flashback-style) is a later seam.
    ///
    /// # Panics
    ///
    /// Panics if `index` does not name an activated ability in `object`'s
    /// derived list — `legal_actions` offered it and the pending decision
    /// froze the state.
    pub(crate) fn begin_activate(&mut self, object: ObjectId, index: usize) {
        debug_assert_eq!(
            self.objects.obj(object).zone,
            Some(Zone::Battlefield),
            "begin_activate only handles battlefield sources"
        );
        let abilities = crate::derive::usable_abilities(self, object);
        let ability = as_activated(
            abilities
                .get(index)
                .expect("ability index from the legal list is in bounds"),
        )
        .expect("BeginActivate names an activated ability")
        .clone();
        let controller = self.objects.obj(object).controller;
        // The source's announce-time snapshot: `~` reads it at resolution even
        // if the source is gone ([CR#608.2]). The other bindings stay empty,
        // as for a fresh trigger outside any event context.
        let bindings = TriggerBindings {
            this: Some(LkiSnapshot::capture(self, object)),
            ..Default::default()
        };
        // [CR#602.2a]: the ability is created on the stack as the FIRST step
        // of announcing — mint its stack identity now, so announce-time
        // deontic `by` rows (hexproof's controller anchor, stack-zone-keyed
        // shapes) evaluate against the real id. `AbilityActivated` promotes
        // this same id into the committed entry.
        let src = self.objects.obj(object).source;
        let id = self.objects.mint(src, controller, Some(Zone::Stack));
        self.announcing = Some(PendingStackEntry {
            optional_components: Vec::new(),
            paid_costs: Vec::new(),
            id,
            object: StackObject::Activated {
                source: object,
                ability: Box::new(ability),
                bindings,
            },
            controller,
            // Origin is a cast-from-zone concept; an ability has no zone of
            // origin — record the source's zone for symmetry.
            origin: Zone::Battlefield,
            targets: vec![],
            x: None,
            // [CR#601.2b]: filled by the `ChooseCostOptions` step before `PayCost`.
            concretized: None,
            // An activated ability has no alternative cast cost ([CR#118.9]).
            alternative_cost: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::ActivatedAbility;
    use deckmaste_core::BeginningStep;
    use deckmaste_core::Condition;
    use deckmaste_core::CostComponent;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaSymbol;
    use deckmaste_core::OneShotEffect;
    use deckmaste_core::PhaseStep;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::SimpleManaSymbol;
    use deckmaste_core::Timing;
    use deckmaste_core::UseLimit;
    use deckmaste_core::WhoseTurn;
    use deckmaste_core::Zone;

    use super::*;
    use crate::object::ObjectId;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn game() -> GameState {
        GameState::new(GameConfig {
            players: vec![PlayerConfig { deck: vec![] }, PlayerConfig { deck: vec![] }],
            seed: 0,
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

    /// Build an `ActivatedAbility` with the given cost and no
    /// condition/limits/targets.
    fn activated(cost: Vec<CostComponent>, effect: OneShotEffect) -> ActivatedAbility {
        ActivatedAbility {
            ability_word: None,
            from: None,
            window: None,
            cost: cost.into(),
            condition: None,
            limits: vec![],
            effect,
        }
    }

    fn noop_effect() -> OneShotEffect {
        // A no-target effect: By(You, Sacrifice(This)) — available in core.
        OneShotEffect::Act(Action::By(
            Reference::You,
            PlayerAction::Sacrifice(Reference::This),
        ))
    }

    // -- as_activated --

    #[test]
    fn as_activated_returns_inner_for_plain() {
        let act = activated(vec![], noop_effect());
        let ability = Ability::Activated(act);
        assert!(as_activated(&ability).is_some());
    }

    #[test]
    fn as_activated_looks_through_expanded() {
        use deckmaste_core::Expansion;
        use deckmaste_core::ExpansionArgs;
        use deckmaste_core::Ident;
        let act = activated(vec![], noop_effect());
        let expanded = Ability::Expanded(Expansion {
            name: Ident::new("Foo"),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(Ability::Activated(act)),
        });
        assert!(
            as_activated(&expanded).is_some(),
            "as_activated must look through Expanded"
        );
    }

    #[test]
    fn as_activated_returns_none_for_non_activated() {
        assert!(
            // The effect's content is immaterial here — only the
            // `Ability::Static` shell (vs. `Activated`) matters, so a
            // no-op `Several([])` stands in for "any static ability".
            as_activated(&Ability::Static(deckmaste_core::StaticEffect::Modify(
                deckmaste_core::Reference::This,
                deckmaste_core::Modification::Several(vec![]),
            )))
            .is_none()
        );
    }

    // -- cost_summary --

    #[test]
    fn cost_summary_returns_none_on_non_eligible_do_cost() {
        let cost = vec![CostComponent::do_(PlayerAction::GainLife(
            deckmaste_core::Count::Literal(1),
        ))];
        assert!(
            cost_summary(&cost).is_none(),
            "Do(...) with a non-cost-eligible action should yield None"
        );
    }

    #[test]
    fn cost_summary_collects_verb_components() {
        let cost = vec![
            CostComponent::Mana("{1}".parse().unwrap()),
            CostComponent::Tap,
            CostComponent::do_(PlayerAction::Sacrifice(Reference::This)),
        ];
        let summary = cost_summary(&cost).expect("verb costs no longer abort the summary");
        assert_eq!(summary.mana, "{1}".parse().unwrap());
        assert!(summary.tap);
        assert_eq!(summary.verbs.len(), 1);
    }

    /// A cycling-shaped cost reads LUMPY (faithful read keeps the macro's
    /// nested `Cost([Mana(2)])` splice), and the pay path summarizes it
    /// correctly: {2} mana plus the discard-self verb. This is the cycling
    /// cost paying end-to-end at the level the engine supports (from-hand
    /// activation is a separate, unbuilt seam) — `cost_summary` doubles as the
    /// cost's normalization, so the nested `Cost` never derails payment.
    #[test]
    fn cost_summary_pays_lumpy_cycling_cost() {
        use deckmaste_core::Cost;
        use deckmaste_core::Normalize;

        // The exact shape a `Cycling([Mana([Generic(2)])])` expansion produces
        // under faithful read: the printed cost rides in a nested `Cost`.
        let lumpy: Cost = deckmaste_core::ron::options()
            .from_str(
                "[Cost([Mana([Generic(2)])]), \
                 Do(Composite(name: Discard, body: Move(This, Graveyard)))]",
            )
            .unwrap();
        // Pre-condition: read really is lumpy (a nested Cost survives).
        assert!(
            matches!(lumpy.0.first(), Some(CostComponent::Cost(_))),
            "cycling cost reads lumpy, got {:?}",
            lumpy.0,
        );

        // The pay path summarizes the lumpy cost correctly.
        let summary = cost_summary(&lumpy.0).expect("cycling cost is payable");
        assert_eq!(summary.mana, "{2}".parse().unwrap(), "pays {{2}}");
        assert!(!summary.tap && !summary.untap);
        assert_eq!(summary.verbs.len(), 1, "the discard-self verb is collected");
        assert!(
            matches!(
                &summary.verbs[0],
                Action::Composite { name, .. } if name.as_str() == "Discard"
            ),
            "the verb is the discard-self composite, got {:?}",
            summary.verbs[0],
        );

        // And it summarizes identically to the normalized (flat) cost — the
        // walk-as-normalize equivalence the boundary relies on.
        let flat = lumpy.normalize();
        let flat_summary = cost_summary(&flat.0).expect("flat cost is payable");
        assert_eq!(summary.mana, flat_summary.mana);
        assert_eq!(summary.verbs.len(), flat_summary.verbs.len());
    }

    #[test]
    fn cost_summary_sums_mana_and_notes_tap() {
        let cost = vec![
            CostComponent::Mana(ManaCost::from(vec![ManaSymbol::Simple(
                SimpleManaSymbol::Generic(2),
            )])),
            CostComponent::Tap,
        ];
        let summary = cost_summary(&cost).expect("mixed [Mana, Tap] should not be None");
        assert_eq!(
            summary.mana.len(),
            1,
            "should have exactly one generic-2 symbol"
        );
        assert!(summary.tap, "the {{T}} component is seen");
        assert!(!summary.untap, "no {{Q}} component");
    }

    #[test]
    fn cost_summary_empty_cost_is_all_empty() {
        let summary = cost_summary(&[]).expect("empty cost should summarize");
        assert!(summary.mana.is_empty());
        assert!(!summary.tap);
        assert!(!summary.untap);
    }

    #[test]
    fn cost_summary_sees_untap_through_expanded() {
        use deckmaste_core::Expansion;
        use deckmaste_core::ExpansionArgs;
        let cost = vec![CostComponent::Expanded(Expansion {
            name: "Q".into(),
            args: ExpansionArgs::none(),
            template: None,
            value: Box::new(CostComponent::Untap),
        })];
        let summary = cost_summary(&cost).expect("a wrapped {Q} should summarize");
        assert!(summary.untap, "{{Q}} is seen through the macro wrapper");
        assert!(!summary.tap);
    }

    /// A `TapTotal` component is well-formed (it never aborts the summary) and
    /// is hoisted into `tap_totals` for the gate / pay step ([CR#702.122a]).
    #[test]
    fn cost_summary_collects_tap_total() {
        let cost = vec![
            CostComponent::Mana("{1}".parse().unwrap()),
            CostComponent::TapTotal {
                stat: Stat::Power,
                cmp: Cmp::AtLeast,
                count: Count::Literal(3),
                filter: Box::new(Predicate::creature()),
            },
        ];
        let summary = cost_summary(&cost).expect("a TapTotal cost summarizes");
        assert_eq!(summary.tap_totals.len(), 1);
        assert_eq!(summary.tap_totals[0].stat, Stat::Power);
        assert_eq!(summary.tap_totals[0].cmp, Cmp::AtLeast);
        assert_eq!(summary.tap_totals[0].count, Count::Literal(3));
        // The plain {1} still rides the mana lane.
        assert_eq!(summary.mana, "{1}".parse().unwrap());
    }

    // -- greedy_tap_subset (the aggregate-stat payment reading) --

    #[test]
    fn greedy_tap_subset_meets_lower_bound_with_fewest_taps() {
        let a = ObjectId::from_raw(1);
        let b = ObjectId::from_raw(2);
        let c = ObjectId::from_raw(3);
        // Highest-stat first: a single power-3 covers "total power 3 or greater".
        let chosen = greedy_tap_subset(vec![(a, 1), (b, 3), (c, 2)], Cmp::AtLeast, 3)
            .expect("3+2+1 = 6 can reach 3");
        assert_eq!(chosen, vec![b], "tap only the power-3 permanent");

        // Two power-2 bears sum to 4 >= 3 (one is not enough).
        let two =
            greedy_tap_subset(vec![(a, 2), (b, 2)], Cmp::AtLeast, 3).expect("2+2 = 4 reaches 3");
        assert_eq!(two.len(), 2, "needs both bears to clear 3");
    }

    #[test]
    fn greedy_tap_subset_none_when_total_falls_short() {
        let a = ObjectId::from_raw(1);
        let b = ObjectId::from_raw(2);
        // Total power 4 can never reach 5 ([CR#601.2h] no partial payment).
        assert!(greedy_tap_subset(vec![(a, 2), (b, 2)], Cmp::AtLeast, 5).is_none());
        // No candidates and a positive bound is unpayable.
        assert!(greedy_tap_subset(vec![], Cmp::AtLeast, 1).is_none());
    }

    #[test]
    fn greedy_tap_subset_empty_satisfies_trivial_bound() {
        let a = ObjectId::from_raw(1);
        // "total power 0 or greater" holds by tapping nothing.
        assert_eq!(
            greedy_tap_subset(vec![(a, 2)], Cmp::AtLeast, 0),
            Some(vec![])
        );
        // An at-most bound is met by the empty subset (sum 0 <= N).
        assert_eq!(
            greedy_tap_subset(vec![(a, 2)], Cmp::AtMost, 3),
            Some(vec![])
        );
    }

    // -- tap_total_subset (the crew candidate set) --

    /// A vanilla creature card with the given printed power/toughness.
    fn creature_card(power: i32, toughness: i32) -> std::sync::Arc<deckmaste_core::Card> {
        std::sync::Arc::new(deckmaste_core::Card::Normal(deckmaste_core::CardFace {
            name: "Crew Fixture".into(),
            mana_cost: ManaCost::from(vec![]),
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![deckmaste_core::Type::Creature.def()],
            subtypes: vec![],
            abilities: vec![],
            power: Some(deckmaste_core::StatValue::Number(power)),
            toughness: Some(deckmaste_core::StatValue::Number(toughness)),
            loyalty: None,
            defense: None,
        }))
    }

    /// [CR#702.122a,110.1]: only a creature ON THE BATTLEFIELD may be tapped to
    /// crew a Vehicle. `candidates_with` scans every object regardless of zone
    /// and `ControlledBy` matches the `controller` field a library card carries
    /// too — so without the battlefield zone guard a controlled library
    /// creature would wrongly count toward (and be tapped for) the
    /// aggregate power. Here a power-2 creature is on the battlefield and
    /// an identical one sits in the library: a Crew 4 (total power ≥ 4) is
    /// UNPAYABLE (the lone battlefield 2 falls short — the library creature
    /// must not contribute), while a Crew 2 taps exactly the battlefield
    /// creature, never the library one.
    #[test]
    fn tap_total_subset_excludes_a_library_creature() {
        let mut state = game();
        let player = PlayerId(0);
        let card_id = state.cards.push(creature_card(2, 2), player);

        let on_field =
            state
                .objects
                .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(on_field);
        // An identical creature controlled by the same player, but in the library.
        let in_library =
            state
                .objects
                .mint(ObjectSource::Card(card_id), player, Some(Zone::Library));

        // Sanity: the library creature really is a zone-agnostic creature
        // candidate with a readable power — so the *only* thing that can keep it
        // out of the crew set is the battlefield zone guard, not an incidental
        // filter/stat mismatch (this is what makes the assertions below a real
        // regression test of the guard).
        let raw = crate::target::candidates_with(&state, &Predicate::creature(), None);
        assert!(
            raw.contains(&in_library) && raw.contains(&on_field),
            "both creatures match the zone-agnostic creature filter"
        );
        let view = state.layers();
        assert_eq!(
            state.cost_stat_value(&view, in_library, Stat::Power),
            Some(2),
            "the library creature has a readable power 2"
        );

        let crew_4 = TapTotalReq {
            stat: Stat::Power,
            cmp: Cmp::AtLeast,
            count: Count::Literal(4),
            filter: Predicate::creature(),
        };
        let crew_2 = TapTotalReq {
            stat: Stat::Power,
            cmp: Cmp::AtLeast,
            count: Count::Literal(2),
            filter: Predicate::creature(),
        };

        // Crew 4: the lone battlefield power-2 can't reach 4, and the library
        // creature must not be borrowed to make up the difference.
        assert!(
            state.tap_total_subset(&crew_4, on_field, player).is_none(),
            "a library creature must not contribute to crew (Crew 4 is unpayable)"
        );

        // Crew 2: payable by tapping ONLY the battlefield creature.
        let chosen = state
            .tap_total_subset(&crew_2, on_field, player)
            .expect("the battlefield power-2 crews a Crew 2");
        assert_eq!(
            chosen,
            vec![on_field],
            "only the battlefield creature is tapped — never the library one"
        );
    }

    // -- can_activate gate --

    fn make_object_on_battlefield(state: &mut GameState, player: PlayerId) -> ObjectId {
        // A minimal Card-backed permanent (empty types ⇒ no confers, so this
        // stays a neutral fixture for the activation gate). Being Card-backed is
        // what a real battlefield object always is: `base_map` builds a
        // `LayeredView` entry only for objects with a `card_id`, skipping
        // card-less player proxies — which never sit on the battlefield in real
        // play. A prior `ObjectSource::Player` synthetic was absent from the
        // view, so the battlefield-wide `Cant(Activate)` collector's `view.get`
        // could not resolve it.
        let card = std::sync::Arc::new(deckmaste_core::Card::Normal(deckmaste_core::CardFace {
            name: "Gate Fixture".into(),
            ..deckmaste_core::CardFace::default()
        }));
        let card_id = state.cards.push(card, player);
        let id = state
            .objects
            .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(id);
        id
    }

    #[test]
    fn gate_rejects_when_condition_wrong_player() {
        let mut state = game();
        // Active player is PlayerId(0); checking PlayerId(1) for YourTurn.
        let player = PlayerId(1);
        let obj = make_object_on_battlefield(&mut state, player);

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            window: None,
            condition: Some(Condition::YourTurn),
            limits: vec![],
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &ability),
            "condition YourTurn should block non-active player"
        );
    }

    #[test]
    fn gate_allows_when_condition_correct_player() {
        let mut state = game();
        let player = PlayerId(0); // active player
        let obj = make_object_on_battlefield(&mut state, player);

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            condition: Some(Condition::YourTurn),
            window: None,
            limits: vec![],
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            state.can_activate(&view, player, obj, 0, &ability),
            "condition YourTurn should allow active player"
        );
    }

    #[test]
    fn gate_rejects_when_once_per_turn_exhausted() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);

        // Record an AbilityUsed fact in history to simulate a previous
        // activation this turn (replaces the deleted ledger bump).
        state.record_history_fact(
            state.turn.turn_number,
            None,
            crate::event::GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            condition: None,
            limits: vec![UseLimit::OncePerTurn],
            window: None,
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &ability),
            "OncePerTurn should block after one activation"
        );
        // Confirm the gate passes after advancing to a new turn (ThisTurn window
        // excludes prior-turn entries).
        state.turn.turn_number += 1;
        let view = state.layers();
        assert!(
            state.can_activate(&view, player, obj, 0, &ability),
            "OncePerTurn should allow again on the next turn"
        );
        drop(view);
    }

    #[test]
    fn gate_rejects_when_once_per_game_exhausted() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);

        // Record an AbilityUsed fact in history to simulate a previous
        // activation (replaces the deleted ledger bump).
        state.record_history_fact(
            state.turn.turn_number,
            None,
            crate::event::GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        // Advance to a new turn — the ThisGame window still sees the prior entry.
        state.turn.turn_number += 1;

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            condition: None,
            limits: vec![UseLimit::OncePerGame],
            window: None,
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &ability),
            "OncePerGame should block even after turn reset"
        );
    }

    #[test]
    fn gate_allows_zero_cost_no_limits() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let ability = activated(vec![], noop_effect());
        let view = state.layers();
        assert!(
            state.can_activate(&view, player, obj, 0, &ability),
            "zero-cost, no-limits ability should always be activatable"
        );
    }

    // -- can_activate gate: activation window ([CR#602.5d..602.5e,500.1]) --

    /// `Timing::DuringTurn(WhoseTurn::Your)` allows only the active player.
    #[test]
    fn gate_during_turn_your_allows_active_blocks_non_active() {
        let mut state = game();
        // Active player is PlayerId(0) (game()'s Fixed starting player).
        let active = PlayerId(0);
        let non_active = PlayerId(1);
        let obj_active = make_object_on_battlefield(&mut state, active);
        let obj_non_active = make_object_on_battlefield(&mut state, non_active);

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            window: Some(Timing::DuringTurn(WhoseTurn::Your)),
            condition: None,
            limits: vec![],
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            state.can_activate(&view, active, obj_active, 0, &ability),
            "DuringTurn(Your) should allow the active player during their own turn"
        );
        assert!(
            !state.can_activate(&view, non_active, obj_non_active, 0, &ability),
            "DuringTurn(Your) should block a non-active player"
        );
    }

    /// `Timing::DuringTurn(WhoseTurn::AnOpponents)` allows only a player who
    /// is not the active player.
    #[test]
    fn gate_during_turn_an_opponents_allows_non_active_blocks_active() {
        let mut state = game();
        let active = PlayerId(0);
        let non_active = PlayerId(1);
        let obj_active = make_object_on_battlefield(&mut state, active);
        let obj_non_active = make_object_on_battlefield(&mut state, non_active);

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            window: Some(Timing::DuringTurn(WhoseTurn::AnOpponents)),
            condition: None,
            limits: vec![],
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            !state.can_activate(&view, active, obj_active, 0, &ability),
            "DuringTurn(AnOpponents) should block the active player"
        );
        assert!(
            state.can_activate(&view, non_active, obj_non_active, 0, &ability),
            "DuringTurn(AnOpponents) should allow a non-active player"
        );
    }

    /// `Timing::DuringTurn(WhoseTurn::EachPlayers)` allows any player,
    /// regardless of whose turn it is.
    #[test]
    fn gate_during_turn_each_players_always_allows() {
        let mut state = game();
        let active = PlayerId(0);
        let non_active = PlayerId(1);
        let obj_active = make_object_on_battlefield(&mut state, active);
        let obj_non_active = make_object_on_battlefield(&mut state, non_active);

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            window: Some(Timing::DuringTurn(WhoseTurn::EachPlayers)),
            condition: None,
            limits: vec![],
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            state.can_activate(&view, active, obj_active, 0, &ability),
            "DuringTurn(EachPlayers) should allow the active player"
        );
        assert!(
            state.can_activate(&view, non_active, obj_non_active, 0, &ability),
            "DuringTurn(EachPlayers) should allow a non-active player too"
        );
    }

    /// `Timing::DuringStep(step, whose)` gates on BOTH the named step and the
    /// `WhoseTurn` relation — forecast-style ("Activate only during the
    /// upkeep step", [CR#702.57b]).
    #[test]
    fn gate_during_step_blocks_wrong_step_allows_named_step() {
        let mut state = game();
        let player = PlayerId(0); // active player
        let obj = make_object_on_battlefield(&mut state, player);

        let ability = ActivatedAbility {
            ability_word: None,
            from: None,
            cost: vec![].into(),
            window: Some(Timing::DuringStep(
                PhaseStep::Beginning(BeginningStep::Upkeep),
                WhoseTurn::Your,
            )),
            condition: None,
            limits: vec![],
            effect: noop_effect(),
        };

        // Wrong step: main phase.
        state.turn.current = PhaseStep::PrecombatMain;
        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &ability),
            "DuringStep(Upkeep, Your) should block outside the upkeep step"
        );

        // Right step: upkeep.
        state.turn.current = PhaseStep::Beginning(BeginningStep::Upkeep);
        let view = state.layers();
        assert!(
            state.can_activate(&view, player, obj, 0, &ability),
            "DuringStep(Upkeep, Your) should allow during the upkeep step"
        );
    }

    // -- can_pay_verbs gate ([CR#601.2h,118.3,119.4]) --

    /// `LoseLife(2)` cost: the controller must have ≥ 2 life to activate. Goes
    /// through the real `can_activate` gate so the wiring is exercised end to
    /// end. The other gate inputs (no mana, no condition/limits/targets) are
    /// inert, isolating the verb-payability check.
    #[test]
    fn gate_rejects_pay_life_cost_when_life_too_low() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let ability = activated(
            vec![CostComponent::do_(PlayerAction::LoseLife(
                deckmaste_core::Count::Literal(2),
            ))],
            noop_effect(),
        );

        // 1 life < 2: cannot pay the life cost.
        state.player_mut(player).life = 1;
        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &ability),
            "LoseLife(2) cost must block activation at 1 life"
        );
    }

    #[test]
    fn gate_allows_pay_life_cost_when_life_sufficient() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let ability = activated(
            vec![CostComponent::do_(PlayerAction::LoseLife(
                deckmaste_core::Count::Literal(2),
            ))],
            noop_effect(),
        );

        // Exactly 2 life ≥ 2: the cost is payable ([CR#119.4]).
        state.player_mut(player).life = 2;
        let view = state.layers();
        assert!(
            state.can_activate(&view, player, obj, 0, &ability),
            "LoseLife(2) cost must be payable at 2 life"
        );
    }

    /// [CR#119.4b]: paying 0 life is always allowed, even at 0 life.
    #[test]
    fn pay_life_of_zero_is_always_payable() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        state.player_mut(player).life = 0;
        assert!(
            state.can_pay_verbs(
                player,
                &[Action::by_you(PlayerAction::LoseLife(
                    deckmaste_core::Count::Literal(0),
                ))],
                obj,
            ),
            "paying 0 life is always allowed [CR#119.4b]"
        );
    }

    /// [CR#119.8] SEAM: under an effect that says a player can't lose life, a
    /// cost involving paying life can't be paid. That lock is NOT YET ENFORCED
    /// (the deontic layer has no lose-life `DeonticAction` variant, so there is
    /// nothing to query — see the seam comment in `can_pay_verbs`). This test
    /// pins the CURRENT behavior so the seam is visible: with sufficient life
    /// and no such effect in play (none is constructible today), a
    /// `LoseLife(2)` cost IS payable. When the lock lands, extend this to
    /// assert that a can't-lose-life effect makes the life cost UNPAYABLE
    /// while a sibling mana reading stays available.
    ///
    /// Lives inline (in `src/`, not `tests/`) because it calls the
    /// `pub(crate)` `can_pay_verbs` directly.
    #[test]
    fn cant_pay_life_lock_is_a_documented_seam() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        state.player_mut(player).life = 20;
        // No "can't lose life" effect exists (unrepresentable today), so the
        // life cost is payable — the [CR#119.8] lock is a documented seam.
        assert!(
            state.can_pay_verbs(
                player,
                &[Action::by_you(PlayerAction::LoseLife(
                    deckmaste_core::Count::Literal(2),
                ))],
                obj,
            ),
            "without an (unbuilt) can't-lose-life lock, a LoseLife(2) cost is payable at 20 life"
        );
    }

    /// `Discard(1)` cost: the actor needs at least one card in hand.
    #[test]
    fn discard_cost_needs_a_card_in_hand() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let verbs = [Action::discard(
            Reference::You,
            deckmaste_core::Count::Literal(1),
            false,
        )];

        // Empty hand: not payable.
        assert!(
            !state.can_pay_verbs(player, &verbs, obj),
            "Discard(1) is not payable with an empty hand"
        );

        // One object in hand: payable.
        let card = state
            .objects
            .mint(ObjectSource::Player(player), player, Some(Zone::Hand));
        state.zones.hands[player.index()].push(card);
        assert!(
            state.can_pay_verbs(player, &verbs, obj),
            "Discard(1) is payable with a card in hand"
        );
    }

    /// A `This` self-sacrifice always has its one object — payable.
    #[test]
    fn self_sacrifice_cost_is_payable() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        assert!(
            state.can_pay_verbs(
                player,
                &[Action::by_you(PlayerAction::Sacrifice(Reference::This))],
                obj,
            ),
            "a self-sacrifice always has its one object to pay with"
        );
    }

    /// [CR#107.14,601.2h]: "pay {E}" is `RemoveCounters(You, Energy, N)` — a
    /// player-borne counter cost. It is payable only when the payer's proxy
    /// holds at least N energy counters ([CR#122.1] energy, a counter, sits on
    /// the player); an unfunded pay is unpayable (partial payment forbidden).
    #[test]
    fn pay_energy_cost_needs_enough_energy() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let proxy = state.player(player).object;
        let verbs = [Action::by_you(PlayerAction::RemoveCounters(
            Reference::You,
            deckmaste_core::CounterRef::from("Energy"),
            deckmaste_core::Count::Literal(2),
        ))];

        // Zero energy: pay {E}{E} is unpayable.
        assert!(
            !state.can_pay_verbs(player, &verbs, obj),
            "pay {{E}}{{E}} is unpayable with no energy [CR#601.2h]"
        );

        // One energy < two needed: still unpayable (no partial payment).
        state
            .objects
            .obj_mut(proxy)
            .counters
            .insert("Energy".into(), 1);
        assert!(
            !state.can_pay_verbs(player, &verbs, obj),
            "pay {{E}}{{E}} is unpayable with only one energy [CR#601.2h]"
        );

        // Exactly two energy: payable.
        state
            .objects
            .obj_mut(proxy)
            .counters
            .insert("Energy".into(), 2);
        assert!(
            state.can_pay_verbs(player, &verbs, obj),
            "pay {{E}}{{E}} is payable with two energy [CR#107.14]"
        );
    }

    /// A loyalty `−N` cost on a permanent (`RemoveCounters(This, …)`) now
    /// reads the object's counter map too ([CR#606.6]): it is payable only
    /// when the source holds at least N loyalty counters. This is the same
    /// player/object-agnostic payability check the energy cost exercises,
    /// pinned on the `This` carrier.
    #[test]
    fn remove_loyalty_cost_needs_enough_counters_on_source() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let verbs = [Action::by_you(PlayerAction::RemoveCounters(
            Reference::This,
            deckmaste_core::CounterRef::from("LoyaltyCounter"),
            deckmaste_core::Count::Literal(3),
        ))];

        // No loyalty counters: −3 is unpayable.
        assert!(
            !state.can_pay_verbs(player, &verbs, obj),
            "a −3 loyalty cost is unpayable with no loyalty counters"
        );

        // Three loyalty counters: payable.
        state
            .objects
            .obj_mut(obj)
            .counters
            .insert("LoyaltyCounter".into(), 3);
        assert!(
            state.can_pay_verbs(player, &verbs, obj),
            "a −3 loyalty cost is payable at three loyalty [CR#606.6]"
        );
    }

    // -- begin_activate --

    /// A card whose only ability is the given activated ability.
    // In-module fixture: no macro/serde path exercised, so no plugin round-trip
    // needed.
    fn card_with_activated(act: ActivatedAbility) -> std::sync::Arc<deckmaste_core::Card> {
        std::sync::Arc::new(deckmaste_core::Card::Normal(deckmaste_core::CardFace {
            name: "Activated Fixture".into(),
            mana_cost: ManaCost::from(vec![]),
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![deckmaste_core::Type::Artifact.def()],
            subtypes: vec![],
            abilities: vec![Ability::Activated(act)],
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
        }))
    }

    /// A card with the given printed mana cost whose only ability is `act`
    /// (an artifact, so {T}/{Q}-free activation faces no summoning sickness).
    fn card_with_cost_and_activated(
        mana_cost: ManaCost,
        act: ActivatedAbility,
    ) -> std::sync::Arc<deckmaste_core::Card> {
        std::sync::Arc::new(deckmaste_core::Card::Normal(deckmaste_core::CardFace {
            name: "ManaCostOf Fixture".into(),
            mana_cost,
            color_indicator: vec![],
            supertypes: vec![],
            types: vec![deckmaste_core::Type::Artifact.def()],
            subtypes: vec![],
            abilities: vec![Ability::Activated(act)],
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
        }))
    }

    /// `ManaCostOf(This)` resolves to the source object's printed mana cost
    /// ([CR#202.1]) — the cost-language "pay mana equal to its mana cost". The
    /// summary collects the reference (mana stays empty), and
    /// `resolve_cost_mana` reads the live source's {1}{U}.
    #[test]
    fn resolve_cost_mana_reads_sources_printed_cost() {
        let mut state = game();
        let player = PlayerId(0);
        let printed: ManaCost = "{1}{U}".parse().unwrap();
        let act = activated(
            vec![CostComponent::ManaCostOf(Reference::This)],
            noop_effect(),
        );
        let card_id = state.cards.push(
            card_with_cost_and_activated(printed.clone(), act.clone()),
            player,
        );
        let obj = state
            .objects
            .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(obj);

        let summary = cost_summary(&act.cost).expect("ManaCostOf cost summarizes");
        assert!(summary.mana.is_empty(), "no literal mana, only ManaCostOf");
        assert_eq!(summary.mana_cost_of, vec![Reference::This]);

        let resolved = state.resolve_cost_mana(&summary, obj, player);
        assert_eq!(
            resolved, printed,
            "ManaCostOf(This) pays the source's printed {{1}}{{U}}"
        );
    }

    /// `can_activate` gates "pay mana equal to its mana cost" on the RESOLVED
    /// amount ([CR#202.1,601.2g]), not a free read: an empty pool can't afford
    /// the source's {1}{U}, a matching pool can.
    #[test]
    fn can_activate_gates_on_resolved_mana_cost_of() {
        let mut state = game();
        let player = PlayerId(0);
        let printed: ManaCost = "{1}{U}".parse().unwrap();
        let act = activated(
            vec![CostComponent::ManaCostOf(Reference::This)],
            noop_effect(),
        );
        let card_id = state
            .cards
            .push(card_with_cost_and_activated(printed, act.clone()), player);
        let obj = state
            .objects
            .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(obj);

        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &act),
            "an empty pool can't pay the resolved {{1}}{{U}}"
        );

        // Fund exactly the resolved cost: {1} generic + {U}.
        let pool = &mut state.player_mut(player).mana_pool;
        pool.add(deckmaste_core::ColorOrColorless::Colorless, 1);
        pool.add(
            deckmaste_core::ColorOrColorless::from(deckmaste_core::Color::Blue),
            1,
        );
        let view = state.layers();
        assert!(
            state.can_activate(&view, player, obj, 0, &act),
            "a {{1}}{{U}} pool affords the resolved ManaCostOf cost"
        );
    }

    #[test]
    fn begin_activate_stages_cloned_ability_and_lki() {
        let mut state = game();
        let player = PlayerId(0);
        let act = activated(vec![CostComponent::Tap], noop_effect());
        let card_id = state.cards.push(card_with_activated(act.clone()), player);
        let obj = state
            .objects
            .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(obj);

        state.begin_activate(obj, 0);

        let pending = state.announcing.as_ref().expect("the announce slot opens");
        assert_eq!(pending.controller, player);
        assert_eq!(pending.origin, Zone::Battlefield);
        assert!(pending.targets.is_empty(), "targets fill at announce");
        let StackObject::Activated {
            source,
            ability,
            bindings,
        } = &pending.object
        else {
            panic!(
                "expected an Activated stack object, got {:?}",
                pending.object
            );
        };
        assert_eq!(*source, obj);
        assert_eq!(**ability, act, "the ability VALUE rides, cloned");
        let this = bindings.this.as_ref().expect("the source's LKI snapshot");
        assert_eq!(this.object, obj, "LKI names the announce-time source");
        assert_eq!(this.left, Zone::Battlefield);
        assert!(bindings.that_object.is_none(), "no event context");
        assert_eq!(bindings.that_player, None);
    }

    /// When an activated ability is committed (`AbilityActivated` event
    /// applies), a `GameEvent::AbilityUsed` fact must be recorded in history
    /// for the same (source, ability-index) pair ([CR#602.2a,608.2i]).
    ///
    /// Drive through the full announce schedule for a {0}-cost no-target
    /// ability so the `Emit(AbilityActivated)` item fires without surfacing
    /// any `PayMana` or `ChooseTargets` decisions.
    #[test]
    fn activation_records_ability_used() {
        use deckmaste_core::Lookback;

        use crate::agenda::WorkItem;
        use crate::event::Occurrence;
        use crate::step::Progress;
        use crate::step::StepOutcome;

        let mut state = game();
        let player = PlayerId(0);

        // Build a free ({0}) no-op artifact activated ability.
        let act = activated(
            vec![CostComponent::Mana(ManaCost::from(vec![]))],
            noop_effect(),
        );
        let card_id = state.cards.push(card_with_activated(act), player);
        let obj = state
            .objects
            .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(obj);

        // Schedule the full announce+commit pipeline as the engine would for
        // an `ActivateAbility` action (mirrors `GameState::act` in decide.rs).
        let items = crate::state::GameState::announce_schedule(
            WorkItem::BeginActivate {
                object: obj,
                ability: 0,
            },
            crate::event::GameEvent::AbilityActivated {
                source: obj,
                ability: 0,
            },
        );
        state.schedule_front(items);

        // Step until the `AbilityActivated` apply completes (at most 20 steps).
        // A {0} cost with no targets/X surfaces no decisions in this window.
        let mut activated = false;
        for _ in 0..20 {
            match state.step() {
                StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                    crate::event::GameEvent::AbilityActivated { .. },
                ))) => {
                    activated = true;
                    break;
                }
                StepOutcome::NeedsDecision(d) => {
                    panic!("unexpected decision while stepping activation: {d:?}");
                }
                StepOutcome::GameOver(_) => {
                    panic!("game ended while stepping activation");
                }
                StepOutcome::Progress(_) => {}
            }
        }
        assert!(
            activated,
            "AbilityActivated must have applied within 20 steps"
        );

        // History must contain an AbilityUsed for (obj, 0).
        // Use-limits are object-scoped: record the per-instance ObjectId,
        // not the persistent CardId/ObjectSource ([CR#400.7]).
        let turn = state.turn.turn_number;
        let found = state.history.scan(Lookback::ThisGame, turn).any(|e| {
            matches!(
                e,
                crate::event::GameEvent::AbilityUsed { object, ability }
                    if *object == obj && *ability == 0
            )
        });
        assert!(
            found,
            "AbilityActivated apply must record GameEvent::AbilityUsed {{ object: {obj:?}, ability: 0 }} in history",
        );
    }
}
