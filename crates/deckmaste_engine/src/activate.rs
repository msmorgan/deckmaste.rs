//! Activating non-mana activated abilities ([CR#602]): the legality gate and
//! the staged announce (`begin_activate`), which mirrors `cast.rs`
//! ([CR#602.2b]: activation follows the [CR#601.2] steps). Mana abilities
//! never come here: they are stackless ([CR#605.3b]) and keep their fast
//! path.

use deckmaste_core::{
    Ability, ActivatedAbility, CostComponent, ManaCost, ManaSymbol, Type, UseLimit, Zone,
};

use crate::cast::can_pay;
use crate::lki::LkiSnapshot;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::stack::{PendingStackEntry, StackObject};
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
/// mana plus the {T}/{Q} components. `None` when a component is beyond the
/// engine today (`Do(...)` verb costs wait for engine-resolve-playeractions;
/// loyalty costs wait for core-loyalty-costs).
pub(crate) struct CostSummary {
    pub mana: ManaCost,
    pub tap: bool,
    pub untap: bool,
}

/// Summarize `cost` in one walk (so the `can_activate` gate and the pay step
/// can never diverge). `Expanded` macro wrappers are looked through.
#[must_use]
pub(crate) fn cost_summary(cost: &[CostComponent]) -> Option<CostSummary> {
    let mut symbols: Vec<ManaSymbol> = Vec::new();
    let mut tap = false;
    let mut untap = false;
    for component in cost {
        match component {
            CostComponent::Mana(m) => symbols.extend_from_slice(m),
            CostComponent::Tap => tap = true,
            CostComponent::Untap => untap = true,
            // Verb costs are not yet handled.
            CostComponent::Do(_) => return None,
            // Recurse through macro wrappers.
            CostComponent::Expanded(e) => {
                let inner = cost_summary(std::slice::from_ref(&e.value))?;
                symbols.extend_from_slice(&inner.mana);
                tap |= inner.tap;
                untap |= inner.untap;
            }
        }
    }
    Some(CostSummary {
        mana: ManaCost::from(symbols),
        tap,
        untap,
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
        // [CR#601.2g,602.2b]: the pool must be able to pay the mana cost.
        let Some(summary) = cost_summary(&ability.cost) else {
            return false;
        };
        if !can_pay(&self.player(player).mana_pool, &summary.mana) {
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

        // [CR#602.5b..602.5e]: activation condition.
        if ability
            .condition
            .as_ref()
            .is_some_and(|c| !self.condition_holds(c, player))
        {
            return false;
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
        ability
            .targets
            .iter()
            .all(|spec| !self.legal_targets(spec).is_empty())
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
        self.announcing = Some(PendingStackEntry {
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
        });
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::{
        Ability, Action, ActivatedAbility, Condition, CostComponent, Effect, ManaCost, ManaSymbol,
        PlayerAction, Reference, Selection, SimpleManaSymbol, UseLimit, Zone,
    };

    use super::*;
    use crate::object::{ObjectId, ObjectSource};
    use crate::player::PlayerId;
    use crate::state::{GameConfig, GameState, PlayerConfig, StartingPlayer};

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
        use deckmaste_core::{Expansion, ExpansionArgs, Ident};
        let act = activated(vec![], noop_effect());
        let expanded = Ability::Expanded(Expansion {
            name: Ident::new("Foo"),
            args: ExpansionArgs::none(),
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
    fn cost_summary_returns_none_on_do_cost() {
        let cost = vec![CostComponent::Do(PlayerAction::Sacrifice(Selection::Ref(
            Reference::This,
        )))];
        assert!(
            cost_summary(&cost).is_none(),
            "Do(...) cost should yield None"
        );
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
        use deckmaste_core::{Expansion, ExpansionArgs};
        let cost = vec![CostComponent::Expanded(Expansion {
            name: "Q".into(),
            args: ExpansionArgs::none(),
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
