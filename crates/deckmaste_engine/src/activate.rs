//! Activating non-mana activated abilities ([CR#602]): the legality gate.
//! The reified announce flow (next task) mirrors `cast.rs` ([CR#602.2b] —
//! activation follows 601.2's steps). Mana abilities never come here: they
//! are stackless ([CR#605.3b]) and keep their fast path.

use deckmaste_core::{
    Ability, ActivatedAbility, CostComponent, ManaCost, ManaSymbol, Type, UseLimit,
};

use crate::cast::can_pay;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;

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

/// The summed mana of an activation cost, or `None` if a component can't be
/// paid yet: `Do(...)` verb costs wait for `engine-resolve-playeractions`;
/// loyalty costs wait for `core-loyalty-costs`.
fn mana_of(cost: &[CostComponent]) -> Option<ManaCost> {
    let mut symbols: Vec<ManaSymbol> = Vec::new();
    for component in cost {
        match component {
            CostComponent::Mana(m) => symbols.extend_from_slice(m),
            // {T} and {Q} contribute no mana but are not unknown.
            CostComponent::Tap | CostComponent::Untap => {}
            // Verb costs are not yet handled.
            CostComponent::Do(_) => return None,
            // Recurse through macro wrappers.
            CostComponent::Expanded(e) => {
                let inner = mana_of(std::slice::from_ref(&e.value))?;
                symbols.extend_from_slice(&inner);
            }
        }
    }
    Some(ManaCost::from(symbols))
}

/// Return whether the cost slice contains a `Tap` component (including inside
/// `Expanded` wrappers).
fn cost_has_tap(cost: &[CostComponent]) -> bool {
    cost.iter().any(|c| match c {
        CostComponent::Tap => true,
        CostComponent::Expanded(e) => cost_has_tap(std::slice::from_ref(&e.value)),
        _ => false,
    })
}

/// Return whether the cost slice contains an `Untap` component (including
/// inside `Expanded` wrappers).
fn cost_has_untap(cost: &[CostComponent]) -> bool {
    cost.iter().any(|c| match c {
        CostComponent::Untap => true,
        CostComponent::Expanded(e) => cost_has_untap(std::slice::from_ref(&e.value)),
        _ => false,
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
        // [CR#601.2g via 602.2b]: the pool must be able to pay the mana cost.
        let Some(mana) = mana_of(&ability.cost) else {
            return false;
        };
        if !can_pay(&self.player(player).mana_pool, &mana) {
            return false;
        }

        let needs_tap = cost_has_tap(&ability.cost);
        let needs_untap = cost_has_untap(&ability.cost);
        let obj = self.objects.obj(object);

        // A tapped object cannot pay {T}; an untapped object cannot pay {Q}.
        if needs_tap && obj.tapped {
            return false;
        }
        if needs_untap && !obj.tapped {
            return false;
        }

        // [CR#602.5a]: summoning sickness prevents {T}/{Q} costs on creatures.
        // Haste exemption is the `kw-haste` seam.
        if (needs_tap || needs_untap)
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
                    if self.activations.this_turn((object, index)) >= 1 {
                        return false;
                    }
                }
                UseLimit::OncePerGame => {
                    if self.activations.this_game((object, index)) >= 1 {
                        return false;
                    }
                }
            }
        }

        // [CR#601.2c via 602.2b]: every target spec must admit at least one
        // legal candidate.
        ability
            .targets
            .iter()
            .all(|spec| !self.legal_targets(spec).is_empty())
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

    // -- mana_of --

    #[test]
    fn mana_of_returns_none_on_do_cost() {
        let cost = vec![CostComponent::Do(PlayerAction::Sacrifice(Selection::Ref(
            Reference::This,
        )))];
        assert!(mana_of(&cost).is_none(), "Do(...) cost should yield None");
    }

    #[test]
    fn mana_of_sums_mana_ignores_tap() {
        let cost = vec![
            CostComponent::Mana(ManaCost::from(vec![ManaSymbol::Simple(
                SimpleManaSymbol::Generic(2),
            )])),
            CostComponent::Tap,
        ];
        let result = mana_of(&cost).expect("mixed [Mana, Tap] should not be None");
        assert_eq!(result.len(), 1, "should have exactly one generic-2 symbol");
    }

    #[test]
    fn mana_of_empty_cost_gives_empty_mana_cost() {
        let result = mana_of(&[]).expect("empty cost should yield Some(empty ManaCost)");
        assert!(result.is_empty());
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
}
