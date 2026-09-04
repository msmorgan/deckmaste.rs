//! Activating non-mana activated abilities ([CR#602]): the legality gate and
//! the staged announce (`begin_activate`), which mirrors `cast.rs`
//! ([CR#602.2b]: activation follows the [CR#601.2] steps). Mana abilities
//! never come here: they are stackless ([CR#605.3b]) and keep their fast
//! path.

use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::Action;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::Cmp;
use deckmaste_core::CostComponent;
use deckmaste_core::Count;
use deckmaste_core::LifeOp;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Stat;
use deckmaste_core::Uint;
use deckmaste_core::UseLimit;
use deckmaste_core::Zone;

#[cfg(test)]
use crate::event::AbilityActivated;
#[cfg(test)]
use crate::event::AbilityUsed;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::stack::ExecutionFrame;
use crate::stack::PendingStackEntry;
use crate::stack::StackObject;
use crate::state::GameState;
use crate::trigger::TriggerBindings;

/// Look through `Expanded` wrappers to an activated ability, if that is what
/// this is (keyword macros expand to the abilities they grant).
#[must_use]
pub(crate) fn as_activated(ability: &Ability) -> Option<&ActivatedAbility> {
    ability.as_activated()
}

/// True iff `cost` pays with a loyalty-counter verb — `Do(PutCounters(This,
/// LoyaltyCounter, _))` or `Do(RemoveCounters(This, LoyaltyCounter, _))`
/// ([CR#606.4], no dedicated loyalty-cost kind — loyalty costs are plain
/// `PutCounters`/`RemoveCounters` on the source, see
/// `idris/src/Semantics.idr`'s `Cost` `Do` ruling). This is how
/// [`GameState::can_activate`]'s `UseLimit::LoyaltyOncePerTurn` arm recognizes
/// a permanent's OTHER loyalty abilities ([CR#606.3,306.5d]) among its full
/// ability list. Reuses [`cost_summary`] so this can never diverge from the
/// payment path's own reading of the cost.
#[must_use]
pub(crate) fn is_loyalty_ability(cost: &deckmaste_core::Cost) -> bool {
    let Some(summary) = cost_summary(&cost.0) else {
        return false;
    };
    let loyalty = deckmaste_core::CounterRef::from("LoyaltyCounter");
    summary.verbs.iter().any(|v| match v {
        Action::PutCounters(reference, counter, _)
        | Action::RemoveCounters(reference, counter, _) => {
            *reference == Reference::source_parameter() && *counter == loyalty
        }
        _ => false,
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
    /// player verbs (Sacrifice, Exile, Tap, Untap, `ChangeLife`,
    /// `RemoveCounters`, Reveal — agent slots spelled `You` in cost context)
    /// and the discard keyword-action composite ("Discard a card:",
    /// [CR#701.9]). Collected for payment; non-eligible `Do(_)` causes
    /// `cost_summary` to return `None`.
    pub verbs: Vec<Action>,
    /// `ManaCostOf(reference)` components: pay mana equal to the referenced
    /// object's printed mana cost ([CR#202.1]). The reference can only be
    /// resolved against a live frame, so it is collected here and folded into
    /// the mana to pay by [`GameState::resolve_cost_mana`] at the gate and the
    /// payment step.
    pub mana_cost_of: Vec<Reference>,
    /// Aggregate-stat (tap-total) requirements ([CR#702.122a] Crew): each is a
    /// "tap a subset of [filter] whose summed [stat] satisfies [cmp] [count]"
    /// obligation. Like `ManaCostOf`, it can only be locked against a live
    /// frame; the payment protocol exposes every legal complete subset and the
    /// runner alone applies a preference among them.
    pub tap_totals: Vec<TapTotalReq>,
    /// The cost block's INSTRUCTIONS in announcement order
    /// ([CR#601.2b,601.2h]): every payment-time subject instruction and every
    /// paying action, kept as one ordered run because an instruction may write
    /// the register the action after it pays through. "Sacrifice a creature" is
    /// `[Choose(dest: 2, Creature), Act(Sacrifice(Reg(1), Reg(2)))]`.
    /// Collected verbatim because these instructions can only run against a
    /// live frame: the gate ([`GameState::can_activate`]) checks each one's
    /// feasibility and the pay step ([`GameState::pay_cost`]) runs the whole
    /// block against the ANNOUNCE activation, so the paid product is a register
    /// the ability body reads.
    pub steps: Vec<CostComponent>,
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
    let mut steps: Vec<CostComponent> = Vec::new();
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
            CostComponent::Act { action, .. } => {
                verbs.push(action.as_action().clone());
                steps.push(component.clone());
            }
            // A payment-time subject instruction — collected verbatim in place,
            // because the action after it pays through the register it writes.
            // Feasibility is decided against a live frame at the gate; the pay
            // step runs the whole ordered block.
            CostComponent::Choose(_)
            | CostComponent::Sample(_)
            | CostComponent::Search(_)
            | CostComponent::Let(_) => {
                steps.push(component.clone());
            }
            // Provenance is erased at `lower` (`deckmaste_lowering`), so no
            // loaded value reaches here wrapped. The arm survives only because
            // the variant does; `core-demacro` deletes both.
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
                steps.extend(inner.steps);
            }
        }
    }
    Some(CostSummary {
        mana: ManaCost::from(Arc::<[ManaSymbol]>::from(symbols)),
        tap,
        untap,
        verbs,
        mana_cost_of,
        tap_totals,
        steps,
    })
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
        let frame = self.frame(source, controller);
        let mut symbols: Vec<ManaSymbol> = summary.mana.iter().copied().collect();
        for reference in &summary.mana_cost_of {
            let object = self.eval_reference(reference, &frame);
            if let Some(printed) = self.mana_cost(object) {
                symbols.extend_from_slice(&printed);
            }
        }
        ManaCost::from(Arc::<[ManaSymbol]>::from(symbols))
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
        self.can_activate_with_blanket(view, player, object, index, ability, true)
    }

    /// The ordinary activation gate with blanket `Cant(Activate)` effects
    /// disabled. Cost-scoped prohibitions such as summoning sickness still
    /// apply to mana abilities ([CR#602.5a,702.61b]).
    #[must_use]
    pub(crate) fn can_activate_mana(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
        index: usize,
        ability: &ActivatedAbility,
    ) -> bool {
        self.can_activate_with_blanket(view, player, object, index, ability, false)
    }

    fn can_activate_with_blanket(
        &self,
        view: &crate::layer::LayeredView,
        player: PlayerId,
        object: ObjectId,
        index: usize,
        ability: &ActivatedAbility,
        blanket_applies: bool,
    ) -> bool {
        // Structural cost validation remains a proposal gate; whether its
        // resources can actually be supplied is discovered only by the
        // explicit payment protocol.
        let Some(summary) = cost_summary(&ability.cost) else {
            return false;
        };

        // [CR#602.5a,702.61a,702.61b]: a conferred/stack `Cant(Activate)` row
        // forbids this activation — a cost-scoped `Cant(Activate(cost:
        // IncludesTapSymbol))` (the summoning-sickness tap gate a `Creature`
        // type confers, haste-exempt) as well as a BLANKET row (split
        // second, Linvala): this is the full [CR#602.5] non-mana gate, so
        // `blanket_applies: true` — mana abilities never reach this method
        // (they take the stackless arm), so a blanket row is safe to apply
        // here.
        if crate::legal::cant_activate(
            self,
            view,
            object,
            player,
            summary.tap || summary.untap,
            blanket_applies,
        ) {
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
            let frame = self.frame(object, player);
            if !self.condition_holds(c, &frame) {
                return false;
            }
        }

        // [CR#602.5b]: use limits — gate via the turn/game history window.
        let index_u = deckmaste_core::Uint::try_from(index).expect("ability index fits in Uint");
        for limit in ability.limits.iter() {
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

        // [CR#601.2b..601.2c,602.2b]: at least one complete mode/target
        // announcement must exist. A top-level Modal itself has no target
        // wrapper, so every permitted mode must be considered here.
        if !self.announcement_effect_satisfiable(object, player, &ability.effect, &ability.targets)
        {
            return false;
        }
        true
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
    ///
    /// The candidate filter is watched by `source` ([CR#113.7] the ability's
    /// own source): "sacrifice ANOTHER creature" is `And([Creature,
    /// Not(Ref(This))])`, which needs a carrier to exclude `source` itself —
    /// engine-frameless-carrier-threading.
    #[cfg(test)]
    fn cost_step_feasible(
        &self,
        step: &CostComponent,
        source: ObjectId,
        controller: PlayerId,
    ) -> bool {
        let watcher = Some(self.objects.obj(source).source);
        match step {
            // >= 1 candidate to choose, and >= the quantity's lower bound
            // when several are demanded (no partial payment, [CR#601.2b]).
            CostComponent::Choose(choice) => {
                let candidates = crate::target::candidates_region_with_activation(
                    self,
                    &choice.filter,
                    watcher,
                    crate::ActivationId::NONE,
                );
                let frame = self.frame(source, controller);
                let (lo, _hi) = choice.quantity.bounds();
                let need = lo.map_or(0, |c| self.eval_count(c, &frame));
                Uint::try_from(candidates.len()).unwrap_or(Uint::MAX) >= need
            }
            CostComponent::Sample(sample) => {
                let candidates = crate::target::candidates_region_with_activation(
                    self,
                    &sample.filter,
                    watcher,
                    crate::ActivationId::NONE,
                );
                let frame = self.frame(source, controller);
                let (lo, _) = sample.quantity.bounds();
                let need = lo.map_or(0, |count| self.eval_count(count, &frame));
                Uint::try_from(candidates.len()).unwrap_or(Uint::MAX) >= need
            }
            // A search ([CR#701.23b..701.23d]) is ALWAYS payable, unlike a
            // choice: even the compulsory bare-quantity case explicitly
            // settles for "as many as exist" ([CR#701.23d]) rather than
            // failing outright, and a stated-quality search never compels a
            // find at all ([CR#701.23b]). Searching a zone — even an empty
            // one — is a legal outcome. A `Let` pins an existing read and a
            // paying `Act` is gated by `can_pay_verbs`, not here.
            CostComponent::Search(_)
            | CostComponent::Let(_)
            | CostComponent::Act { .. }
            | CostComponent::Mana(_)
            | CostComponent::ManaCostOf(_)
            | CostComponent::Tap
            | CostComponent::Untap
            | CostComponent::Cost(_)
            | CostComponent::TapTotal { .. } => true,
        }
    }

    /// The derived numeric value of `stat` for the card-backed object `id`,
    /// clamped to a non-negative [`Uint`] ([CR#107.1b]) — the aggregate-stat
    /// cost summand. `None` for a non-card object (a player proxy has no stat)
    /// or a stat axis whose engine machinery is unbuilt (loyalty/defense). The
    /// view is built once by the caller and threaded in.
    pub(crate) fn cost_stat_value(
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
                crate::derive::face(self.def(id))
                    .characteristics
                    .mana_cost
                    .mana_value(),
            )
            .ok(),
            // [CR#209.1,306.5a]: loyalty is the PRINTED loyalty characteristic
            // off the card face — never the live counter count (current loyalty
            // is `CounterCount(This, LoyaltyCounter)`). `base_stat` maps
            // `Number(n)→n`, `DefinedByAbility`/`Variable`/absent → 0.
            Stat::Loyalty => crate::layer::base_stat(
                crate::derive::face(self.def(id))
                    .characteristics
                    .loyalty
                    .as_ref(),
            ),
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
    /// satisfiable. The test helper's frame uses the activation's object as its
    /// source, the controller as payer, and no targets.
    #[must_use]
    #[cfg(test)]
    pub(crate) fn can_pay_verbs(
        &self,
        player: PlayerId,
        verbs: &[Action],
        subject: ObjectId,
    ) -> bool {
        // The payer is the controller, and `~`/`This` is the live source.
        // [CR#601.2b]: this test-only helper has no announced X, so it uses an
        // X binding of 0. A semantic-X cost verb (a loyalty `−X`) therefore
        // reads as removing zero counters. The real payment protocol binds and
        // pays the announced X. Without this binding, evaluating X
        // on an X-less frame would panic.
        let mut frame = self.frame(subject, player);
        self.frame_set_x(&mut frame, Some(0));
        // TODO(engine-cost-payment / deontics): [CR#119.8] "can't pay life" is
        // NOT YET ENFORCED. Under a continuous effect saying a player can't
        // lose life, a cost that involves having that player pay life
        // can't be paid — so a `Do(LoseLife(..))` cost (and a
        // Phyrexian-life reading, which concretizes to
        // `Do(LoseLife(2))`) should be UNPAYABLE for that player
        // while the mana reading stays available. The deontic layer has no
        // pay-life / lose-life `DeonticAction` variant today (it models only
        // attack/block/target/attach/cast/play/activate), so there is nothing
        // cheap to query here. See `cant_pay_life_lock_is_a_documented_seam`.
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
    #[expect(
        clippy::match_same_arms,
        reason = "the always-payable verb groups are kept separate to carry their distinct scope/TODO comments (Sacrifice/Move/Tap/Untap vs the loyalty-`+N` PutCounters arm vs the out-of-scope Reveal seam)"
    )]
    pub(crate) fn verb_cost_payable(
        &self,
        verb: &Action,
        player: PlayerId,
        frame: &ExecutionFrame,
    ) -> bool {
        match verb {
            // [CR#701.21a]: "a player can't sacrifice something that isn't a
            // permanent, or something that's a permanent they don't control."
            // The cost's own `Choose` filter may already restrict control, but
            // it need not, so the verb enforces the rule itself — a witness
            // naming a permanent the payer does not control is unpayable, and
            // partial payment is forbidden ([CR#601.2h]).
            Action::Sacrifice(agent, what) => {
                let sacrificer = self.eval_reference(agent, frame);
                let subjects = self.eval_reference_set(what, frame);
                !subjects.is_empty()
                    && subjects.iter().all(|&id| {
                        self.objects.get(id).is_some_and(|object| {
                            object.zone == Some(deckmaste_core::Zone::Battlefield)
                                && self.player(object.controller).object == sacrificer
                        })
                    })
            }
            // Move (exile is `Move(_, Exile)`)/Tap/Untap take a single
            // `Reference` (an agent slot spelled as the controller register in
            // cost context on the verbs that carry one) — it always names its
            // one object, so it is payable. The choose-feasibility of "sacrifice
            // a creature" lives in the cost's own `Choose` instruction, not the
            // verb.
            Action::Move(..) | Action::Tap(_) | Action::Untap(_) => true,
            // [CR#119.4]: pay-life needs life ≥ the amount; [CR#119.4b]: paying
            // 0 is always allowed (and `life >= 0` holds trivially).
            Action::ChangeLife(_, LifeOp::Down(count)) => {
                let amount = self.eval_count(count, frame);
                // [CR#119.4,119.4b]: compare in Uint space — negative life can
                // never be ≥ a non-negative amount, so clamp to 0 before
                // converting. `unwrap_or(Uint::MAX)` keeps this panic-free.
                let life = deckmaste_core::Uint::try_from(self.player(player).life.max(0))
                    .unwrap_or(deckmaste_core::Uint::MAX);
                life >= amount
            }
            // Gaining/setting life as a cost ([CR#119.7]; Invigorate,
            // Skyshroud Cutter) needs no prior resource — always payable.
            Action::ChangeLife(_, LifeOp::Up(_) | LifeOp::Set(_)) => true,
            // `PutCounters` as a cost (a loyalty `+N` ability adds that many
            // loyalty counters to its source, [CR#606.4]) is always
            // payable — adding counters needs no prior resource.
            Action::PutCounters(..) => true,
            // [CR#601.2h,107.14]: removing counters as a cost needs at least
            // that many present on the carrier — the loyalty `−N` ability
            // ([CR#606.6]) and "pay {E}" ([CR#107.14]: paying {E} removes an
            // energy counter from the player). The carrier is the resolved
            // object OR player proxy ([CR#122.1] — a counter is a marker on an
            // object OR player, so energy/poison sit on the player), so
            // `RemoveCounters(You, Energy, N)` reads the payer's
            // proxy counter map. An absent kind reads zero, so an unfunded
            // "pay {E}" is unpayable (partial payment forbidden, [CR#601.2h]).
            Action::RemoveCounters(sel, kind, count) => {
                let need = self.eval_count(count, frame);
                let carriers = self.eval_reference_set(sel, frame);
                !carriers.is_empty()
                    && carriers.iter().all(|&id| {
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
            Action::Reveal { .. } => true,
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
        let compiled = abilities
            .get(index)
            .expect("ability index from the legal list is in bounds");
        let captures = crate::derive::usable_ability_captures(self, object, index, compiled);
        let mut ability = as_activated(compiled)
            .expect("BeginActivate names an activated ability")
            .clone();
        if ability.effect.params.is_empty() {
            ability.effect.params = deckmaste_core::announced_region_params(ability.targets.len());
        }
        let controller = self.objects.obj(object).controller;
        if self.payment.is_none() {
            self.begin_payment_proposal(controller);
        }
        // The source's announce-time snapshot: `~` reads it at resolution even
        // if the source is gone ([CR#608.2]). The other bindings stay empty,
        // as for a fresh trigger outside any event context.
        let bindings = TriggerBindings {
            this: Some(LkiSnapshot::capture(self, object)),
            captures: captures.clone(),
            ..Default::default()
        };
        // [CR#602.2a]: the ability is created on the stack as the FIRST step
        // of announcing — mint its stack identity now, so announce-time
        // deontic `by` rows (hexproof's controller anchor, stack-zone-keyed
        // shapes) evaluate against the real id. `AbilityActivated` promotes
        // this same id into the committed entry.
        let src = self.objects.obj(object).source;
        let id = self.objects.mint(src, controller, Some(Zone::Stack));
        let mut frame = self.frame(object, controller);
        self.frame_set_source_lki(&mut frame, bindings.this.clone());
        let activation = self.enter_created_region(&ability.effect, &frame, &captures);
        self.announcing = Some(PendingStackEntry {
            optional_components: Vec::new(),
            paid_costs: Vec::new(),
            id,
            activation,
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
            chosen_modes: Arc::from([]),
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
    use std::sync::Arc;

    use deckmaste_core::Ability;
    use deckmaste_core::Action;
    use deckmaste_core::ActivatedAbility;
    use deckmaste_core::BeginningStep;
    use deckmaste_core::Condition;
    use deckmaste_core::CostComponent;
    use deckmaste_core::Instruction;
    use deckmaste_core::ManaCost;
    use deckmaste_core::ManaSymbol;
    use deckmaste_core::PhaseStep;
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

    fn creature_card(power: i32, toughness: i32) -> Arc<deckmaste_card::Card> {
        Arc::new(deckmaste_card::Card::Normal(
            deckmaste_card::CardFace::from(deckmaste_card::Characteristics {
                name: "Activation fixture".into(),
                types: vec![deckmaste_core::Type::Creature.def()],
                power: Some(deckmaste_core::StatValue::Number(power)),
                toughness: Some(deckmaste_core::StatValue::Number(toughness)),
                ..deckmaste_card::Characteristics::default()
            }),
        ))
    }

    fn announce_activation_to_payment(
        state: &mut GameState,
        object: ObjectId,
        ability: usize,
    ) -> crate::payment::PaymentPrompt {
        use crate::agenda::WorkItem;
        use crate::decide::DecisionPointKind;
        use crate::step::StepOutcome;

        state.schedule_front(GameState::announce_schedule(
            WorkItem::BeginActivate { object, ability },
            crate::event::GameEvent::AbilityActivated(AbilityActivated {
                source: object,
                ability,
            }),
        ));
        for _ in 0..20 {
            match state.step() {
                StepOutcome::Progress(_) => {}
                StepOutcome::NeedsDecision(DecisionPointKind::Payment(prompt)) => return prompt,
                other => panic!("expected activation payment, got {other:?}"),
            }
        }
        panic!("activation did not reach payment")
    }

    /// Build an `ActivatedAbility` with the given cost and no
    /// condition/limits/targets.
    fn activated(cost: Vec<CostComponent>, effect: Instruction) -> ActivatedAbility {
        ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            from: None,
            window: None,
            cost: Arc::<[CostComponent]>::from(cost).into(),
            condition: None,
            limits: vec![].into(),
            effect: effect.into(),
        }
    }

    fn noop_effect() -> Instruction {
        // A no-target effect: Sacrifice(You, This) — available in core.
        Instruction::act(Action::Sacrifice(
            Reference::Reg(deckmaste_core::RefId(1)),
            Reference::Reg(deckmaste_core::RefId(0)),
        ))
    }

    // -- as_activated --

    #[test]
    fn as_activated_returns_inner_for_plain() {
        let act = activated(vec![], noop_effect());
        let ability = Ability::activated(act);
        assert!(as_activated(&ability).is_some());
    }

    #[test]
    fn as_activated_returns_none_for_non_activated() {
        assert!(
            // The effect's content is immaterial here — only the
            // `Ability::Static` shell (vs. `Activated`) matters, so a
            // no-op `Several([])` stands in for "any static ability".
            as_activated(&Ability::r#static(deckmaste_core::StaticSpec::Modify(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Modification::Several(vec![].into()),
            )))
            .is_none()
        );
    }

    // -- cost_summary --

    #[test]
    fn non_eligible_do_cost_cannot_cross_the_core_boundary() {
        assert_eq!(
            CostComponent::try_do_action(Action::DrawCard(Reference::Reg(deckmaste_core::RefId(
                1
            )))),
            Err(deckmaste_core::RunnableCostActionError::Ineligible),
        );
    }

    #[test]
    fn cost_summary_collects_verb_components() {
        let cost = vec![
            CostComponent::Mana("{1}".parse().unwrap()),
            CostComponent::Tap,
            CostComponent::do_action(Action::Sacrifice(
                Reference::Reg(deckmaste_core::RefId(1)),
                Reference::Reg(deckmaste_core::RefId(0)),
            )),
        ];
        let summary = cost_summary(&cost).expect("verb costs no longer abort the summary");
        assert_eq!(summary.mana, "{1}".parse().unwrap());
        assert!(summary.tap);
        assert_eq!(summary.verbs.len(), 1);
    }

    /// A cycling-shaped semantic cost lowers to the runnable compositional
    /// grammar: {2} mana followed by the already-bound discard-self action.
    /// This is the cycling cost paying end-to-end at the level the engine
    /// supports (from-hand activation is a separate seam).
    #[test]
    fn cost_summary_pays_lumpy_cycling_cost() {
        use deckmaste_core::KeywordAbility;
        use deckmaste_lowering::Lower;

        let plugin = deckmaste_plugin::plugin::Plugin::load(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin"),
        )
        .unwrap();
        let semantic: deckmaste_semantics::KeywordAbility = plugin
            .macros
            .read_str("Cycling([Mana([Generic(2)])])")
            .unwrap();
        let KeywordAbility::Composite { abilities, .. } = semantic.lower() else {
            panic!("Cycling lowers to a composite keyword")
        };
        let cycling = abilities
            .iter()
            .find_map(Ability::as_activated)
            .expect("Cycling grants an activated ability");

        let summary = cost_summary(&cycling.cost).expect("cycling cost is payable");
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
        assert!(
            cycling.cost.iter().any(|component| matches!(
                component,
                CostComponent::Act { action, .. }
                    if matches!(
                        action.as_action(),
                        Action::Composite { name, body }
                            if name.as_str() == "Discard"
                                && matches!(
                                    body.as_ref(),
                                    deckmaste_core::Instruction::Act { action: Action::Move(
                                        Reference::Reg(deckmaste_core::RefId(0)),
                                        deckmaste_core::Destination::Zone(Zone::Graveyard),
                                        _,
                                        _,
                                    ), .. }
                                )
                    )
            )),
            "lowering binds the runnable discard action's subject to This"
        );
        assert!(
            cycling.cost.iter().all(|component| !matches!(
                component,
                CostComponent::Choose(_) | CostComponent::Sample(_) | CostComponent::Search(_)
            )),
            "cycling's bound discard needs no payment-time decision ([CR#702.29a])"
        );
    }

    #[test]
    fn cost_summary_sums_mana_and_notes_tap() {
        let cost = vec![
            CostComponent::Mana(ManaCost::from(Arc::from(vec![ManaSymbol::Simple(
                SimpleManaSymbol::Generic(2),
            )]))),
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
    fn cost_summary_sees_untap() {
        let cost = vec![CostComponent::Untap];
        let summary = cost_summary(&cost).expect("a {Q} cost should summarize");
        assert!(summary.untap, "{{Q}} is seen");
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
                filter: Arc::new(Predicate::creature()),
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

    // -- can_activate gate --

    fn make_object_on_battlefield(state: &mut GameState, player: PlayerId) -> ObjectId {
        // A minimal Card-backed permanent (empty types ⇒ no confers, so this
        // stays a neutral fixture for the activation gate). Being Card-backed
        // is what a real battlefield object always is: `base_map`
        // builds a `LayeredView` entry only for objects with a
        // `card_id`, skipping card-less player proxies — which never
        // sit on the battlefield in real play. A prior
        // `ObjectSource::Player` synthetic was absent from the view, so
        // the battlefield-wide `Cant(Activate)` collector's `view.get`
        // could not resolve it.
        let card = Arc::new(deckmaste_card::Card::Normal(
            deckmaste_card::CardFace::from(deckmaste_card::Characteristics {
                name: "Gate Fixture".into(),
                ..deckmaste_card::Characteristics::default()
            }),
        ));
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
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            window: None,
            condition: Some(Condition::YourTurn),
            limits: vec![].into(),
            effect: noop_effect().into(),
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
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            condition: Some(Condition::YourTurn),
            window: None,
            limits: vec![].into(),
            effect: noop_effect().into(),
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
            crate::event::GameEvent::AbilityUsed(AbilityUsed {
                object: obj,
                ability: 0,
            }),
        );

        let ability = ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            condition: None,
            limits: vec![UseLimit::OncePerTurn].into(),
            window: None,
            effect: noop_effect().into(),
        };
        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &ability),
            "OncePerTurn should block after one activation"
        );
        // Confirm the gate passes after advancing to a new turn (ThisTurn
        // window excludes prior-turn entries).
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
            crate::event::GameEvent::AbilityUsed(AbilityUsed {
                object: obj,
                ability: 0,
            }),
        );
        // Advance to a new turn — the ThisGame window still sees the prior
        // entry.
        state.turn.turn_number += 1;

        let ability = ActivatedAbility {
            ability_word: None,
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            condition: None,
            limits: vec![UseLimit::OncePerGame].into(),
            window: None,
            effect: noop_effect().into(),
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
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            window: Some(Timing::DuringTurn(WhoseTurn::Your)),
            condition: None,
            limits: vec![].into(),
            effect: noop_effect().into(),
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
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            window: Some(Timing::DuringTurn(WhoseTurn::AnOpponents)),
            condition: None,
            limits: vec![].into(),
            effect: noop_effect().into(),
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
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            window: Some(Timing::DuringTurn(WhoseTurn::EachPlayers)),
            condition: None,
            limits: vec![].into(),
            effect: noop_effect().into(),
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
            targets: [].into(),
            from: None,
            cost: Arc::<[CostComponent]>::from(vec![]).into(),
            window: Some(Timing::DuringStep(
                PhaseStep::Beginning(BeginningStep::Upkeep),
                WhoseTurn::Your,
            )),
            condition: None,
            limits: vec![].into(),
            effect: noop_effect().into(),
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

    /// `LoseLife(2)` is structurally legal at 1 life, but its locked payment
    /// IOU cannot be fulfilled. Rejection leaves the active prompt unchanged.
    #[test]
    fn gate_rejects_pay_life_cost_when_life_too_low() {
        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::payment::FulfillmentWitness;
        use crate::payment::IouKind;
        use crate::payment::PaymentCommand;

        let mut state = game();
        let player = PlayerId(0);
        let ability = activated(
            vec![CostComponent::do_action(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Down(deckmaste_core::Count::Literal(2)),
            ))],
            noop_effect(),
        );
        let card_id = state
            .cards
            .push(card_with_activated(ability.clone()), player);
        let obj = state
            .objects
            .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(obj);

        state.player_mut(player).life = 1;
        let view = state.layers();
        assert!(
            state.can_activate(&view, player, obj, 0, &ability),
            "resource insufficiency does not suppress a structurally legal activation"
        );

        let prompt = announce_activation_to_payment(&mut state, obj, 0);
        let [iou] = prompt.outstanding.as_slice() else {
            panic!("the activation locks exactly one life IOU: {prompt:?}")
        };
        assert_eq!(iou.kind, IouKind::PayLife(2));

        let iou = iou.id;
        let image_before = format!("{:#?}", state.active());
        let before = prompt;
        assert!(
            state
                .submit_decision(Decision::Payment(PaymentCommand::Fulfill {
                    iou,
                    witness: FulfillmentWitness::PayLife,
                }))
                .is_err(),
            "the explicit fulfillment rejects paying 2 life from a total of 1"
        );
        let Some(DecisionPointKind::Payment(after)) = state.pending.as_ref() else {
            panic!("the rejected fulfillment keeps the payment prompt open")
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
    }

    #[test]
    fn gate_allows_pay_life_cost_when_life_sufficient() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let ability = activated(
            vec![CostComponent::do_action(Action::ChangeLife(
                Reference::Reg(deckmaste_core::RefId(1)),
                LifeOp::Down(deckmaste_core::Count::Literal(2)),
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
                &[Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(deckmaste_core::Count::Literal(0)),
                )],
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
                &[Action::ChangeLife(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    LifeOp::Down(deckmaste_core::Count::Literal(2)),
                )],
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
            Reference::Reg(deckmaste_core::RefId(1)),
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

    /// [CR#601.2h]: a payment-time choice filtered by `Not(Ref(This))`
    /// ("sacrifice another creature") must exclude the ability's own source
    /// from its candidates — an unpayable cost can't be paid, and without the
    /// exclusion the source would wrongly count as its own "another" —
    /// engine-frameless-carrier-threading. Without a threaded carrier,
    /// `Ref(This)` panics in the frameless matcher; `cost_step_feasible` must
    /// supply the source as watcher.
    #[test]
    fn choose_cost_filter_excludes_source_via_not_ref_this() {
        let mut state = game();
        let player = PlayerId(0);
        let card_id = state.cards.push(creature_card(2, 2), player);
        let source =
            state
                .objects
                .mint(ObjectSource::Card(card_id), player, Some(Zone::Battlefield));
        state.zones.battlefield.push(source);

        // A predicate region: register 0 is the CANDIDATE under test, and the
        // enclosing source/controller follow as captured parameters — the
        // shape `deckmaste_lowering::region::candidate_region` builds. "Another
        // creature" is therefore `Not(Ref(Reg(1)))`, the source.
        let filter = deckmaste_core::Region::new(
            Arc::from([
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(0),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Candidate(
                        deckmaste_core::Domain::Entity,
                    ),
                },
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(1),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Source,
                },
                deckmaste_core::Param {
                    def: deckmaste_core::DefId(2),
                    kind: deckmaste_core::Kind::Entity,
                    provenance: deckmaste_core::Provenance::Controller,
                },
            ]),
            Predicate::And(Arc::from(vec![
                Predicate::creature(),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::Reg(
                    deckmaste_core::RefId(1),
                )))),
            ])),
        );
        let choose = CostComponent::Choose(deckmaste_core::Choose {
            dest: deckmaste_core::DefId(2),
            by: Reference::Reg(deckmaste_core::RefId(1)),
            quantity: deckmaste_core::Quantity::one(),
            filter: Arc::new(filter),
        });

        // Only the source itself is a creature: "sacrifice another creature"
        // has no legal candidate — unpayable.
        assert!(
            !state.cost_step_feasible(&choose, source, player),
            "Not(Ref(This)) excludes the source; no OTHER creature exists to sacrifice"
        );

        // A second creature makes "another creature" payable.
        let other_id = state.cards.push(creature_card(1, 1), player);
        let other = state.objects.mint(
            ObjectSource::Card(other_id),
            player,
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(other);
        assert!(
            state.cost_step_feasible(&choose, source, player),
            "a second creature satisfies 'sacrifice another creature'"
        );
    }

    /// [CR#701.23b..701.23d]: a search cost instruction is always payable —
    /// even the compulsory bare-quantity case settles for "as many as exist"
    /// ([CR#701.23d]) rather than failing outright, unlike a choice's
    /// no-partial-payment rule. An EMPTY library (the hardest case) still
    /// doesn't block payment.
    #[test]
    fn search_cost_step_is_always_payable_even_over_an_empty_library() {
        use deckmaste_core::ObjectClass;
        use deckmaste_core::Quantity;

        let mut state = game();
        let player = PlayerId(0);
        let source = make_object_on_battlefield(&mut state, player);

        // Both cardinalities the retired binder pair spelled: exactly one
        // (`SearchOne`) and a stated quantity (`Search`).
        for quantity in [
            Quantity::one(),
            Quantity::Range(
                Some(deckmaste_core::Count::Literal(2)),
                Some(deckmaste_core::Count::Literal(2)),
            ),
        ] {
            let search = CostComponent::Search(deckmaste_core::Search {
                dest: deckmaste_core::DefId(2),
                by: Reference::Reg(deckmaste_core::RefId(1)),
                whose: Reference::Reg(deckmaste_core::RefId(1)),
                from: vec![Zone::Library].into(),
                quantity: quantity.clone(),
                filter: Arc::new(deckmaste_core::Region::candidate(Predicate::Class(
                    ObjectClass::Card,
                ))),
                if_none: deckmaste_core::Block::default(),
            });
            assert!(
                state.cost_step_feasible(&search, source, player),
                "a search over an empty library is still payable ({quantity:?})"
            );
        }
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
                &[Action::Sacrifice(
                    Reference::Reg(deckmaste_core::RefId(1)),
                    Reference::Reg(deckmaste_core::RefId(0))
                )],
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
        let verbs = [Action::RemoveCounters(
            Reference::Reg(deckmaste_core::RefId(1)),
            deckmaste_core::CounterRef::from("Energy"),
            deckmaste_core::Count::Literal(2),
        )];

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
        let verbs = [Action::RemoveCounters(
            Reference::Reg(deckmaste_core::RefId(0)),
            deckmaste_core::CounterRef::from("LoyaltyCounter"),
            deckmaste_core::Count::Literal(3),
        )];

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
    fn card_with_activated(act: ActivatedAbility) -> Arc<deckmaste_card::Card> {
        Arc::new(deckmaste_card::Card::Normal(
            deckmaste_card::CardFace::from(deckmaste_card::Characteristics {
                name: "Activated Fixture".into(),
                mana_cost: ManaCost::from(Arc::from(vec![])),
                color_indicator: vec![],
                supertypes: vec![],
                types: vec![deckmaste_core::Type::Artifact.def()],
                subtypes: vec![],
                abilities: vec![Ability::activated(act)],
                power: None,
                toughness: None,
                loyalty: None,
                defense: None,
            }),
        ))
    }

    /// A card with the given printed mana cost whose only ability is `act`
    /// (an artifact, so {T}/{Q}-free activation faces no summoning sickness).
    fn card_with_cost_and_activated(
        mana_cost: ManaCost,
        act: ActivatedAbility,
    ) -> Arc<deckmaste_card::Card> {
        Arc::new(deckmaste_card::Card::Normal(
            deckmaste_card::CardFace::from(deckmaste_card::Characteristics {
                name: "ManaCostOf Fixture".into(),
                mana_cost,
                color_indicator: vec![],
                supertypes: vec![],
                types: vec![deckmaste_core::Type::Artifact.def()],
                subtypes: vec![],
                abilities: vec![Ability::activated(act)],
                power: None,
                toughness: None,
                loyalty: None,
                defense: None,
            }),
        ))
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
            vec![CostComponent::ManaCostOf(Reference::Reg(
                deckmaste_core::RefId(0),
            ))],
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
        assert_eq!(
            summary.mana_cost_of,
            vec![Reference::Reg(deckmaste_core::RefId(0))]
        );

        let resolved = state.resolve_cost_mana(&summary, obj, player);
        assert_eq!(
            resolved, printed,
            "ManaCostOf(This) pays the source's printed {{1}}{{U}}"
        );
    }

    /// `ManaCostOf(This)` remains structurally legal with an empty pool. The
    /// payment boundary resolves it to locked {1}{U} IOUs, then rejects empty
    /// coverage without mutating the active prompt.
    #[test]
    fn can_activate_gates_on_resolved_mana_cost_of() {
        use deckmaste_core::Color;

        use crate::decide::Decision;
        use crate::decide::DecisionPointKind;
        use crate::payment::IouKind;
        use crate::payment::ManaCoverage;
        use crate::payment::ManaPip;
        use crate::payment::PaymentCommand;

        let mut state = game();
        let player = PlayerId(0);
        let printed: ManaCost = "{1}{U}".parse().unwrap();
        let act = activated(
            vec![CostComponent::ManaCostOf(Reference::Reg(
                deckmaste_core::RefId(0),
            ))],
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
            state.can_activate(&view, player, obj, 0, &act),
            "an empty pool does not suppress a structurally legal activation"
        );

        let prompt = announce_activation_to_payment(&mut state, obj, 0);
        assert!(
            prompt
                .outstanding
                .iter()
                .any(|iou| matches!(iou.kind, IouKind::ManaPip(ManaPip::Generic))),
            "ManaCostOf locks its generic pip"
        );
        assert!(
            prompt
                .outstanding
                .iter()
                .any(|iou| matches!(iou.kind, IouKind::ManaPip(ManaPip::Colored(Color::Blue)))),
            "ManaCostOf locks its blue pip"
        );

        let image_before = format!("{:#?}", state.active());
        let before = prompt;
        assert!(
            state
                .submit_decision(Decision::Payment(PaymentCommand::BeginPayment(
                    ManaCoverage::empty(),
                )))
                .is_err(),
            "empty coverage cannot satisfy the locked {{1}}{{U}} IOUs"
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
        let Ability::Activated(expected) = Ability::activated(act) else {
            unreachable!("the constructor preserves the ability kind")
        };
        assert_eq!(**ability, *expected, "the ability VALUE rides, cloned");
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
            vec![CostComponent::Mana(ManaCost::from(Arc::from(vec![])))],
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
            crate::event::GameEvent::AbilityActivated(AbilityActivated {
                source: obj,
                ability: 0,
            }),
        );
        state.schedule_front(items);

        // Step until the `AbilityActivated` apply completes (at most 20 steps).
        // A {0} cost with no targets/X surfaces only the payment protocol here.
        let mut activated = false;
        for _ in 0..20 {
            match state.step() {
                StepOutcome::Progress(Progress::Applied(Occurrence::Single(
                    crate::event::GameEvent::AbilityActivated(AbilityActivated { .. }),
                ))) => {
                    activated = true;
                    break;
                }
                StepOutcome::NeedsDecision(crate::decide::DecisionPointKind::Payment(_)) => {
                    let decision = state
                        .auto_payment_pending()
                        .expect("automatic payment decision");
                    state
                        .submit_decision(decision)
                        .expect("automatic payment succeeds");
                }
                StepOutcome::NeedsDecision(
                    crate::decide::DecisionPointKind::ChooseManaReversals(prompt),
                ) => {
                    let maximal = prompt
                        .legal
                        .iter()
                        .max_by_key(|set| set.len())
                        .cloned()
                        .expect("a reversal prompt offers a legal set");
                    state
                        .submit_decision(crate::decide::Decision::ManaReversals(maximal))
                        .expect("automatic reversal succeeds");
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
                crate::event::GameEvent::AbilityUsed(AbilityUsed { object, ability })
                    if *object == obj && *ability == 0
            )
        });
        assert!(
            found,
            "AbilityActivated apply must record GameEvent::AbilityUsed(AbilityUsed {{ object: {obj:?}, ability: 0 }}) in history",
        );
    }
}
