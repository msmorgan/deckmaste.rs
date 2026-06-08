//! Targeting (CR 115): which objects a `Filter` admits. Stage 2 wires only the
//! arms the corpus's `AnyTarget` reaches; the rest are `todo!`.

use deckmaste_core::{CharacteristicFilter, Filter, ObjectKind, StateFilter, Type, Zone};

use crate::object::{ObjectId, ObjectSource};
use crate::state::GameState;

/// The object's kind (CR 109.1) as the corpus needs it: a player proxy is a
/// `Player`; a card on the stack is a `Spell`; otherwise a `Card`.
#[must_use]
pub fn object_kind(state: &GameState, id: ObjectId) -> ObjectKind {
    let obj = state.objects.obj(id);
    match obj.source {
        ObjectSource::Player(_) => ObjectKind::Player,
        ObjectSource::Card(_) if obj.zone == Some(Zone::Stack) => ObjectKind::Spell,
        ObjectSource::Card(_) => ObjectKind::Card,
    }
}

/// Whether `id` matches `filter`. Only the corpus-reachable arms are wired.
#[must_use]
pub fn matches(state: &GameState, id: ObjectId, filter: &Filter) -> bool {
    match filter {
        Filter::Kind(k) => object_kind(state, id) == *k,
        Filter::Characteristic(CharacteristicFilter::Type(t)) => has_type(state, id, *t),
        // CR 110.5a: state, not characteristic — card/token objects only, so a
        // player proxy (zone None) never matches InZone.
        Filter::State(StateFilter::InZone(z)) => state.objects.obj(id).zone == Some(*z),
        Filter::AllOf(fs) => fs.iter().all(|f| matches(state, id, f)),
        Filter::OneOf(fs) => fs.iter().any(|f| matches(state, id, f)),
        Filter::Not(f) => !matches(state, id, f),
        Filter::Any => true,
        // An `Expanded` macro invocation is transparent: delegate to the
        // expanded value so callers can pass a `Filter::Expanded(AnyTarget)`
        // directly without pre-unwrapping.
        Filter::Expanded(e) => matches(state, id, &e.value),
        other => todo!("stage 2 does not evaluate filter {other:?}"),
    }
}

/// A card object's printed types; a player proxy has none.
fn has_type(state: &GameState, id: ObjectId, ty: Type) -> bool {
    state
        .objects
        .obj(id)
        .card_id()
        .is_some_and(|_| crate::derive::face(state.def(id)).types.contains(&ty))
}

/// Every object (card objects in their zones + player proxies) matching
/// `filter`, in deterministic id order.
#[must_use]
pub fn candidates(state: &GameState, filter: &Filter) -> Vec<ObjectId> {
    state
        .objects
        .iter()
        .map(|o| o.id)
        .filter(|&id| matches(state, id, filter))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;
    use deckmaste_core::{Filter, Zone};

    use super::*;
    use crate::player::PlayerId;
    use crate::state::{GameConfig, GameState, PlayerConfig, StartingPlayer};

    fn builtin() -> Plugin {
        Plugin::load(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin")).unwrap()
    }

    /// A two-player game; player 0's deck is Grizzly Bears, player 1's is
    /// Forest. Returns the state plus a Bears object moved onto the
    /// battlefield.
    fn game_with_a_bear_on_the_field() -> (GameState, ObjectId) {
        let plugin = builtin();
        let bears = Arc::new(plugin.card("Grizzly Bears").unwrap());
        let forest = Arc::new(plugin.card("Forest").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 10],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&forest); 10],
                },
            ],
            seed: 1,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        // Force a Bears from player 0's hand onto the battlefield.
        let bear = *state.zones.hands[0]
            .iter()
            .find(|&&o| {
                matches(
                    &state,
                    o,
                    &Filter::Characteristic(deckmaste_core::CharacteristicFilter::Type(
                        deckmaste_core::Type::Creature,
                    )),
                )
            })
            .expect("a Bears in the opening hand (10-card mono deck)");
        state.remove_from_hand(PlayerId(0), bear);
        state.objects.obj_mut(bear).zone = Some(Zone::Battlefield);
        state.zones.battlefield.push(bear);
        (state, bear)
    }

    #[test]
    fn any_target_is_creatures_and_players_not_lands() {
        // `plugin.macros.read_str` returns `Filter::Expanded(AnyTarget { .. })`
        // whose value is the OneOf body. `matches` delegates through Expanded
        // transparently, so the wrapped form works directly.
        let any_target: Filter = builtin().macros.read_str("AnyTarget").unwrap();
        let (state, bear) = game_with_a_bear_on_the_field();
        let targets = candidates(&state, &any_target);
        // Both player proxies + the lone battlefield creature; no lands (in
        // hand/library), no spells (stack empty).
        assert!(targets.contains(&bear));
        assert!(targets.contains(&state.players[0].object));
        assert!(targets.contains(&state.players[1].object));
        assert_eq!(targets.len(), 3);
    }
}
