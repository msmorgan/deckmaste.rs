//! Activating non-mana activated abilities ([CR#602]): the legality gate and
//! the staged announce (`begin_activate`), which mirrors `cast.rs`
//! ([CR#602.2b]: activation follows the [CR#601.2] steps). Mana abilities
//! never come here: they are stackless ([CR#605.3b]) and keep their fast
//! path.

use deckmaste_core::Ability;
use deckmaste_core::ActivatedAbility;
use deckmaste_core::CostComponent;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::PlayerAction;
use deckmaste_core::Selection;
use deckmaste_core::Type;
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
}

/// Summarize `cost` in one walk (so the `can_activate` gate and the pay step
/// can never diverge). `Expanded` macro wrappers are looked through.
#[must_use]
pub(crate) fn cost_summary(cost: &[CostComponent]) -> Option<CostSummary> {
    let mut symbols: Vec<ManaSymbol> = Vec::new();
    let mut tap = false;
    let mut untap = false;
    let mut verbs: Vec<PlayerAction> = Vec::new();
    for component in cost {
        match component {
            CostComponent::Mana(m) => symbols.extend_from_slice(m),
            CostComponent::Tap => tap = true,
            CostComponent::Untap => untap = true,
            CostComponent::Do(action) => {
                if action.is_cost_eligible() {
                    verbs.push(action.clone());
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
            }
        }
    }
    Some(CostSummary {
        mana: ManaCost::from(symbols),
        tap,
        untap,
        verbs,
    })
}

impl GameState {
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
        if !self.gate_mana_affordable(player, &summary.mana, object) {
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
            };
            if !self.condition_holds(c, &frame) {
                return false;
            }
        }

        // [CR#602.5b]: use limits.
        for limit in &ability.limits {
            match limit {
                UseLimit::OncePerTurn => {
                    if self.activations.turn_count((object, index)) >= 1 {
                        return false;
                    }
                }
                UseLimit::OncePerGame => {
                    if self.activations.game_count((object, index)) >= 1 {
                        return false;
                    }
                }
            }
        }

        // [CR#601.2c,602.2b]: every target spec must admit at least one
        // legal candidate.
        if !ability
            .targets
            .iter()
            .all(|spec| !self.legal_targets(spec).is_empty())
        {
            return false;
        }

        // [CR#601.2h,118.3]: the non-mana verb/life costs must be fully
        // payable too — partial payment is forbidden.
        self.can_pay_verbs(player, &summary.verbs, object)
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
        };
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
            // (partial payment is forbidden).
            PlayerAction::Discard(count) => {
                let need = self.eval_count(count, frame) as usize;
                self.zones.hands[player.index()].len() >= need
            }
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
        })
    }

    /// Build an `ActivatedAbility` with the given cost and no
    /// condition/limits/targets.
    fn activated(cost: Vec<CostComponent>, effect: Effect) -> ActivatedAbility {
        ActivatedAbility {
            window: None,
            cost,
            condition: None,
            limits: vec![],
            targets: vec![],
            effect,
        }
    }

    fn noop_effect() -> Effect {
        // A no-target effect: By(You, Sacrifice(This)) — available in core.
        Effect::Act(Action::By(
            Reference::You,
            PlayerAction::Sacrifice(Selection::Ref(Reference::This)),
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
        let cost = vec![CostComponent::Do(PlayerAction::Draw(
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
            CostComponent::Do(PlayerAction::Sacrifice(Selection::Ref(Reference::This))),
        ];
        let summary = cost_summary(&cost).expect("verb costs no longer abort the summary");
        assert_eq!(summary.mana, "{1}".parse().unwrap());
        assert!(summary.tap);
        assert_eq!(summary.verbs.len(), 1);
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
            cost: vec![],
            window: None,
            condition: Some(Condition::YourTurn),
            limits: vec![],
            targets: vec![],
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
            cost: vec![],
            condition: Some(Condition::YourTurn),
            window: None,
            limits: vec![],
            targets: vec![],
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
        let key = (obj, 0);

        // Bump the ledger to simulate a previous activation this turn.
        state.activations.bump(key);

        let ability = ActivatedAbility {
            cost: vec![],
            condition: None,
            limits: vec![UseLimit::OncePerTurn],
            window: None,
            targets: vec![],
            effect: noop_effect(),
        };
        let view = state.layers();
        assert!(
            !state.can_activate(&view, player, obj, 0, &ability),
            "OncePerTurn should block after one activation"
        );
    }

    #[test]
    fn gate_rejects_when_once_per_game_exhausted() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let key = (obj, 0);

        state.activations.bump(key);
        // Simulate a new turn (reset_turn clears per-turn, not per-game).
        state.activations.reset_turn();

        let ability = ActivatedAbility {
            cost: vec![],
            condition: None,
            limits: vec![UseLimit::OncePerGame],
            targets: vec![],
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
            vec![CostComponent::Do(PlayerAction::LoseLife(
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
            vec![CostComponent::Do(PlayerAction::LoseLife(
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

    /// `Discard(1)` cost: the actor needs at least one card in hand.
    #[test]
    fn discard_cost_needs_a_card_in_hand() {
        let mut state = game();
        let player = PlayerId(0);
        let obj = make_object_on_battlefield(&mut state, player);
        let verbs = [PlayerAction::Discard(deckmaste_core::Count::Literal(1))];

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
                &[PlayerAction::Sacrifice(Selection::Ref(Reference::This))],
                obj,
            ),
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
}
