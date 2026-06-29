//! Activating non-mana activated abilities ([CR#602]): the legality gate and
//! the staged announce (`begin_activate`), which mirrors `cast.rs`
//! ([CR#602.2b]: activation follows the [CR#601.2] steps). Mana abilities
//! never come here: they are stackless ([CR#605.3b]) and keep their fast
//! path.

use deckmaste_core::Ability;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Cmp;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::Filter;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PlayerAction;
use deckmaste_core::Reference;
use deckmaste_core::Selection;
use deckmaste_core::Stat;
use deckmaste_core::Type;
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

/// One pass over an activation cost ([CR#602.2b,601.2f..601.2h]): the summed
/// mana, the {T}/{Q} components, and cost-eligible verb actions. `None` when a
/// component is not payable (a non-eligible `Do(...)` verb; loyalty costs wait
/// for core-loyalty-costs).
pub(crate) struct CostSummary {
    pub mana: ManaCost,
    pub tap: bool,
    pub untap: bool,
    /// Cost-eligible verb components ([`PlayerAction::is_cost_eligible`]):
    /// Sacrifice, Exile, Tap, Untap, Discard, `LoseLife`, `RemoveCounters`,
    /// Reveal. Collected for payment; non-eligible `Do(_)` causes
    /// `cost_summary` to return `None`.
    pub verbs: Vec<PlayerAction>,
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
    pub filter: Filter,
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
    let mut verbs: Vec<PlayerAction> = Vec::new();
    let mut mana_cost_of: Vec<Reference> = Vec::new();
    let mut tap_totals: Vec<TapTotalReq> = Vec::new();
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
            // Recurse through macro wrappers.
            CostComponent::Expanded(e) => {
                let inner = cost_summary(std::slice::from_ref(&e.value))?;
                symbols.extend_from_slice(&inner.mana);
                tap |= inner.tap;
                untap |= inner.untap;
                verbs.extend(inner.verbs);
                mana_cost_of.extend(inner.mana_cost_of);
                tap_totals.extend(inner.tap_totals);
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
        let frame = Frame {
            source,
            controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        };
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
        if !self.gate_mana_affordable(player, &mana, object) {
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

        // [CR#602.5a]: summoning sickness prevents {T}/{Q} costs on creatures.
        // Haste exemption is the `kw-haste` seam.
        if (summary.tap || summary.untap)
            && obj.summoning_sick
            && view.get(object).card_types.contains(&Type::Creature)
        {
            return false;
        }

        // [CR#602.5d..602.5e]: activation window ("Activate only as a
        // sorcery" — the Only refinement on the activation permission).
        if let Some(window) = &ability.window {
            let in_window = match window {
                deckmaste_core::Window::InstantSpeed => true,
                deckmaste_core::Window::SorcerySpeed => self.sorcery_speed_ok(player),
                other => todo!("P0.W1: activation window {other:?}"),
            };
            if !in_window {
                return false;
            }
        }

        // [CR#602.5b..602.5e]: activation condition ("Activate only if …").
        // The gate runs before targets are chosen, so the frame carries none;
        // `Ref(This)`/`Is(This, …)` anchors to the live source.
        if let Some(c) = &ability.condition {
            let frame = Frame {
                source: object,
                controller: player,
                targets: Vec::new(),
                bindings: None,
                chosen: None,
                x: None,
                subject: None,
                those: None,
            };
            if !self.condition_holds(c, &frame) {
                return false;
            }
        }

        // [CR#602.5b]: use limits — gate via the turn/game history window.
        let index_u = deckmaste_core::Uint::try_from(index).expect("ability index fits in Uint");
        for limit in &ability.limits {
            match limit {
                UseLimit::OncePerTurn => {
                    if self.ability_used_count(object, index_u, deckmaste_core::Window::ThisTurn)
                        >= 1
                    {
                        return false;
                    }
                }
                UseLimit::OncePerGame => {
                    if self.ability_used_count(object, index_u, deckmaste_core::Window::ThisGame)
                        >= 1
                    {
                        return false;
                    }
                }
            }
        }

        // [CR#601.2c,602.2b]: every target spec must admit at least one
        // legal candidate. The carrier is the activation object's source —
        // anchors a target filter's carrier-relative self-reference (`Ref(This)`,
        // `StatOf(This, …)`).
        let carrier = Some(self.objects.obj(object).source);
        if !crate::resolve::top_targets(&ability.effect)
            .iter()
            .all(|spec| !self.legal_targets(spec, carrier).is_empty())
        {
            return false;
        }

        // [CR#601.2h,118.3]: the non-mana verb/life costs must be fully
        // payable too — partial payment is forbidden.
        if !self.can_pay_verbs(player, &summary.verbs, object) {
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
        let frame = Frame {
            source,
            controller,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            x: None,
            subject: None,
            those: None,
        };
        let need = self.eval_count(&req.count, &frame);
        let watcher = self.objects.obj(source).source;
        let view = self.layers();
        let candidates: Vec<(ObjectId, Uint)> = crate::target::candidates_with(self, &req.filter, Some(watcher))
                .into_iter()
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
            // Aggregate-stat costs over loyalty/defense have no canon card and
            // ride the same unbuilt counter machinery as `eval_count`.
            Stat::Loyalty | Stat::Defense => None,
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
        verbs: &[PlayerAction],
        subject: ObjectId,
    ) -> bool {
        // Same anchoring as the condition gate (`can_activate` above): the
        // payer is the controller, `~`/`This` is the live source.
        let frame = Frame {
            source: subject,
            controller: player,
            targets: Vec::new(),
            bindings: None,
            chosen: None,
            // A cost-payability gate reads no announced X.
            x: None,
            subject: None,
            those: None,
        };
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

    /// Whether one cost-eligible verb can be paid in full ([CR#601.2h]). Looks
    /// through `Expanded` macro wrappers.
    fn verb_cost_payable(&self, verb: &PlayerAction, player: PlayerId, frame: &Frame) -> bool {
        match verb {
            // [CR#119.4]: pay-life needs life ≥ the amount; [CR#119.4b]: paying
            // 0 is always allowed (and `life >= 0` holds trivially).
            PlayerAction::LoseLife(count) => {
                let amount = self.eval_count(count, frame);
                // [CR#119.4,119.4b]: compare in Uint space — negative life can
                // never be ≥ a non-negative amount, so clamp to 0 before
                // converting. `unwrap_or(Uint::MAX)` mirrors the idiom used in
                // `selection_cost_payable` to keep this panic-free.
                let life = deckmaste_core::Uint::try_from(self.player(player).life.max(0))
                    .unwrap_or(deckmaste_core::Uint::MAX);
                life >= amount
            }
            // [CR#601.2h]: discard needs at least that many cards in hand
            // (partial payment is forbidden). A named `what` (cycling's
            // "discard this card", [CR#702.29a]) is payable when those specific
            // cards resolve, same as the other selection-cost verbs.
            PlayerAction::Discard { count, what } => match what {
                None => {
                    let need = self.eval_count(count, frame) as usize;
                    self.zones.hands[player.index()].len() >= need
                }
                Some(sel) => self.selection_cost_payable(sel, frame),
            },
            // Sacrifice/Exile/Tap/Untap: enough legal candidates for the
            // selection's required count ([CR#601.2h]).
            PlayerAction::Sacrifice(sel)
            | PlayerAction::Exile(sel)
            | PlayerAction::Tap(sel)
            | PlayerAction::Untap(sel) => self.selection_cost_payable(sel, frame),
            // Out of this ticket's listed scope — counter storage and the
            // reveal window are unbuilt, so treat as payable for now.
            // TODO(engine-cost-payment follow-up): payability for RemoveCounters
            // (needs counter storage) and Reveal (needs the reveal window).
            PlayerAction::RemoveCounters(..) | PlayerAction::Reveal { .. } => true,
            // Look through a remembered macro invocation.
            PlayerAction::Expanded(e) => self.verb_cost_payable(&e.value, player, frame),
            // `cost_summary` only collects cost-eligible verbs, so nothing else
            // reaches here.
            other => unreachable!("non-cost-eligible verb in a cost summary: {other:?}"),
        }
    }

    /// The minimum number of objects a `quantity` requires, independent of how
    /// many candidates exist — `choice_bounds`'s floor read with an unbounded
    /// pool (`Uint::MAX as usize` round-trips cleanly, never clamping the
    /// floor).
    fn required_minimum(
        &self,
        quantity: &deckmaste_core::Quantity,
        frame: &Frame,
    ) -> deckmaste_core::Uint {
        self.choice_bounds(quantity, deckmaste_core::Uint::MAX as usize, frame)
            .0
    }

    /// Whether a selection-bearing verb cost (sacrifice/exile/tap/untap) has
    /// enough legal candidates to pay it ([CR#601.2h]). For `Choose` the
    /// required count is the quantity's lower bound (via `choice_bounds`); the
    /// fixed forms (`This`/`Each`/`Filter`) succeed when non-empty — a `This`
    /// self-cost always names its one object.
    fn selection_cost_payable(&self, sel: &Selection, frame: &Frame) -> bool {
        match sel {
            Selection::Choose(quantity, filter) => {
                let available = crate::target::candidates(self, filter).len();
                let required = self.required_minimum(quantity, frame);
                deckmaste_core::Uint::try_from(available).unwrap_or(deckmaste_core::Uint::MAX)
                    >= required
            }
            Selection::Each(filter) | Selection::Filter(filter) => {
                !crate::target::candidates(self, filter).is_empty()
            }
            Selection::Expanded(e) => self.selection_cost_payable(&e.value, frame),
            // `Ref` (a bound `This`/`Target`/… ) always names its one object, so
            // it is payable. `AmongNoted`/`Random` are out of this ticket's scope
            // (no noted-slot store yet; random isn't a cost form) — treat as
            // payable for now.
            // TODO(engine-cost-payment follow-up): payability for AmongNoted/Random selections.
            Selection::Ref(_) | Selection::AmongNoted(..) | Selection::Random(..) => true,
            // Those/TopOfLibrary are not valid cost-selection forms.
            Selection::TopOfLibrary { .. } | Selection::Those => {
                todo!(
                    "engine-cost-payment: TopOfLibrary/Those as a cost selection is not supported"
                )
            }
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
        let abilities = crate::derive::abilities(self, object);
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
            that_object: None,
            that_player: None,
        };
        // [CR#602.2a]: the ability is created on the stack as the FIRST step
        // of announcing — mint its stack identity now, so announce-time
        // deontic `by` rows (hexproof's controller anchor, stack-zone-keyed
        // shapes) evaluate against the real id. `AbilityActivated` promotes
        // this same id into the committed entry.
        let src = self.objects.obj(object).source;
        let id = self.objects.mint(src, controller, Some(Zone::Stack));
        self.announcing = Some(PendingStackEntry {
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
        });
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::ActivatedAbility;
    use deckmaste_core::Condition;
    use deckmaste_core::CostComponent;
    use deckmaste_core::Effect;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaSymbol;
    use deckmaste_core::PlayerAction;
    use deckmaste_core::Reference;
    use deckmaste_core::Selection;
    use deckmaste_core::SimpleManaSymbol;
    use deckmaste_core::UseLimit;
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
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        })
    }

    /// Build an `ActivatedAbility` with the given cost and no
    /// condition/limits/targets.
    fn activated(cost: Vec<CostComponent>, effect: Effect) -> ActivatedAbility {
        ActivatedAbility {
            from: None,
            window: None,
            cost: cost.into(),
            condition: None,
            limits: vec![],
            effect,
        }
    }

    fn noop_effect() -> Effect {
        // A no-target effect: By(You, Sacrifice(This)) — available in core.
        Effect::Act(Action::By(
            Reference::You,
            PlayerAction::Sacrifice(Selection::this()),
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
            as_activated(&Ability::Static(deckmaste_core::StaticAbility {
                condition: None,
                effects: vec![],
                characteristic_defining: false,
            }))
            .is_none()
        );
    }

    // -- cost_summary --

    #[test]
    fn cost_summary_returns_none_on_non_eligible_do_cost() {
        let cost = vec![CostComponent::do_(PlayerAction::Draw(
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
            CostComponent::do_(PlayerAction::Sacrifice(Selection::Ref(Reference::This))),
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
            .from_str("[Cost([Mana([Generic(2)])]), Do(Discard(count: Literal(1), what: This))]")
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
            matches!(summary.verbs[0], PlayerAction::Discard { .. }),
            "the verb is the discard-self, got {:?}",
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
                filter: Box::new(Filter::creature()),
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

    // -- can_activate gate --

    fn make_object_on_battlefield(state: &mut GameState, player: PlayerId) -> ObjectId {
        let id = state.objects.mint(
            ObjectSource::Player(player),
            player,
            Some(Zone::Battlefield),
        );
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
        state.history.record(
            state.turn.turn_number,
            crate::event::GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );

        let ability = ActivatedAbility {
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
        state.history.record(
            state.turn.turn_number,
            crate::event::GameEvent::AbilityUsed {
                object: obj,
                ability: 0,
            },
        );
        // Advance to a new turn — the ThisGame window still sees the prior entry.
        state.turn.turn_number += 1;

        let ability = ActivatedAbility {
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
                &[PlayerAction::LoseLife(deckmaste_core::Count::Literal(0))],
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
                &[PlayerAction::LoseLife(deckmaste_core::Count::Literal(2))],
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
        let verbs = [PlayerAction::Discard {
            count: deckmaste_core::Count::Literal(1),
            what: None,
        }];

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
            state.can_pay_verbs(player, &[PlayerAction::Sacrifice(Selection::this())], obj,),
            "a self-sacrifice always has its one object to pay with"
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
            types: vec![deckmaste_core::Type::Artifact],
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
            types: vec![deckmaste_core::Type::Artifact],
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
        use deckmaste_core::Window;

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
        let found = state.history.scan(Window::ThisGame, turn).any(|e| {
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
