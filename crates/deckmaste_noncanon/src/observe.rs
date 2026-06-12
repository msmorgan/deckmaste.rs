//! The honest view: everything a player may legitimately act on, and nothing
//! else. Pilots receive `&Observation`, never `&GameState`, so information
//! honesty holds by construction — the opponent's hand and both library
//! orders appear here only as counts.

use deckmaste_core::Ability;
use deckmaste_core::Action as CoreAction;
use deckmaste_core::Card;
use deckmaste_core::Effect;
use deckmaste_core::Int;
use deckmaste_core::ManaSymbol;
use deckmaste_core::Phase;
use deckmaste_core::PlayerAction;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::StatValue;
use deckmaste_core::Type;
use deckmaste_core::Uint;
use deckmaste_engine::GameState;
use deckmaste_engine::ManaPool;
use deckmaste_engine::ObjectId;
use deckmaste_engine::PlayerId;

/// What's knowable about one visible object.
#[derive(Debug, Clone)]
pub struct ObjView {
    pub id: ObjectId,
    pub name: String,
    pub types: Vec<Type>,
    /// Layered values for battlefield objects; printed values for hand cards.
    pub power: Option<Int>,
    pub toughness: Option<Int>,
    pub controller: PlayerId,
    pub tapped: bool,
    pub mana_value: Uint,
    /// The PRINTED abilities (public info). Indexable by the engine's
    /// `ActivateAbility { ability }` while nothing in the matchup grants or
    /// rewrites abilities — printed and derived lists coincide for these
    /// cards. Revisit when an ability-granting effect joins a deck.
    pub abilities: Vec<Ability>,
}

impl ObjView {
    #[must_use]
    pub fn is_creature(&self) -> bool {
        self.types.contains(&Type::Creature)
    }

    #[must_use]
    pub fn is_land(&self) -> bool {
        self.types.contains(&Type::Land)
    }
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub seat: PlayerId,
    pub active: PlayerId,
    pub phase: Phase,
    pub turn_number: Uint,
    pub my_life: Int,
    pub opp_life: Int,
    pub my_hand: Vec<ObjView>,
    pub battlefield: Vec<ObjView>,
    pub my_graveyard: Vec<ObjView>,
    pub opp_graveyard: Vec<ObjView>,
    /// Current attackers (public combat state; empty outside combat).
    pub attackers: Vec<ObjectId>,
    pub stack_size: usize,
    pub opp_hand_count: usize,
    pub my_library_count: usize,
    pub opp_library_count: usize,
    pub my_pool: ManaPool,
    /// The player proxies — damage/targeting handles for the faces.
    pub my_proxy: ObjectId,
    pub opp_proxy: ObjectId,
}

/// The other seat. Two-player games only: `p` must be seat 0 or 1.
#[must_use]
pub fn opponent(p: PlayerId) -> PlayerId {
    PlayerId(1 - p.0)
}

#[must_use]
pub fn face(card: &Card) -> &deckmaste_core::CardFace {
    match card {
        Card::Normal(f) | Card::ModalDfc(f, _) => f,
    }
}

#[must_use]
pub fn mana_cost_value(cost: &deckmaste_core::ManaCost) -> Uint {
    let mut mv = 0;
    for sym in cost.iter() {
        match sym {
            ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => mv += *n,
            ManaSymbol::Simple(SimpleManaSymbol::Specific(_)) => mv += 1,
            // X / hybrid / phyrexian / snow don't occur in this matchup's costs;
            // revisit when a deck plays them.
            _ => {}
        }
    }
    mv
}

/// A mana ability by the engine's stackless rule: activated, no targets,
/// and the effect is a plain add-mana. Compound effects with riders (e.g.
/// Ancient Tomb's damage) deliberately classify as NOT mana here — for float
/// purposes the `is_land` shortcut covers them when they're allowlisted.
#[must_use]
pub fn is_mana_ability(a: &Ability) -> bool {
    matches!(a, Ability::Activated(act)
        if act.targets.is_empty()
            && matches!(&act.effect, Effect::Act(CoreAction::By(_, PlayerAction::AddMana(..)))))
}

fn printed_stat(stat: Option<&StatValue>) -> Option<Int> {
    match stat {
        Some(StatValue::Number(n)) => Some(*n),
        _ => None,
    }
}

impl Observation {
    /// Builds the seat's view. Computes `state.layers()` once.
    ///
    /// # Panics
    ///
    /// Panics if a zone holds a stale `ObjectId` or an id that is not
    /// card-backed (player proxies have no card face). Callers must only
    /// pass a `GameState` with a live, consistent object store (i.e. not a
    /// snapshot from a previous game tick).
    #[must_use]
    pub fn of(state: &GameState, seat: PlayerId) -> Self {
        let view = state.layers();
        let opp = opponent(seat);
        let layered = |id: ObjectId| {
            let f = face(state.def(id));
            ObjView {
                id,
                name: f.name.clone(),
                types: f.types.clone(),
                power: view.power(id),
                toughness: view.toughness(id),
                controller: state.objects.obj(id).controller,
                tapped: state.objects.obj(id).tapped,
                mana_value: mana_cost_value(&f.mana_cost),
                abilities: f.abilities.clone(),
            }
        };
        let printed = |id: ObjectId| {
            let f = face(state.def(id));
            ObjView {
                id,
                name: f.name.clone(),
                types: f.types.clone(),
                power: printed_stat(f.power.as_ref()),
                toughness: printed_stat(f.toughness.as_ref()),
                controller: state.objects.obj(id).controller,
                tapped: false,
                mana_value: mana_cost_value(&f.mana_cost),
                abilities: f.abilities.clone(),
            }
        };

        Self {
            seat,
            active: state.turn.active_player,
            phase: state.turn.current,
            turn_number: state.turn.turn_number,
            my_life: state.players[seat.index()].life,
            opp_life: state.players[opp.index()].life,
            my_hand: state.zones.hands[seat.index()]
                .iter()
                .map(|&o| printed(o))
                .collect(),
            battlefield: state
                .zones
                .battlefield
                .iter()
                .map(|&o| layered(o))
                .collect(),
            my_graveyard: state.zones.graveyards[seat.index()]
                .iter()
                .map(|&o| printed(o))
                .collect(),
            opp_graveyard: state.zones.graveyards[opp.index()]
                .iter()
                .map(|&o| printed(o))
                .collect(),
            attackers: state.combat.attackers().to_vec(),
            stack_size: state.stack.len(),
            opp_hand_count: state.zones.hands[opp.index()].len(),
            my_library_count: state.zones.libraries[seat.index()].len(),
            opp_library_count: state.zones.libraries[opp.index()].len(),
            my_pool: state.players[seat.index()].mana_pool.clone(),
            my_proxy: state.players[seat.index()].object,
            opp_proxy: state.players[opp.index()].object,
        }
    }

    /// Returns all battlefield objects controlled by this player.
    pub fn my_battlefield(&self) -> impl Iterator<Item = &ObjView> {
        self.battlefield
            .iter()
            .filter(|v| v.controller == self.seat)
    }

    /// Returns all battlefield objects controlled by the opponent.
    pub fn opp_battlefield(&self) -> impl Iterator<Item = &ObjView> {
        let opp = opponent(self.seat);
        self.battlefield.iter().filter(move |v| v.controller == opp)
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_engine::GameConfig;
    use deckmaste_engine::PlayerConfig;
    use deckmaste_engine::StartingPlayer;
    use deckmaste_engine::StepOutcome;

    use super::*;
    use crate::deck;
    use crate::source::CardSource;
    use crate::wc99;

    fn fresh_game() -> GameState {
        let src = CardSource::load();
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: deck::build_subset(&wc99::SPED_RED, wc99::SPED_RED_ALLOWLIST, &src),
                },
                PlayerConfig {
                    deck: deck::build_subset(&wc99::STOMPY, wc99::STOMPY_ALLOWLIST, &src),
                },
            ],
            seed: 7,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
        });
        // Step to the first decision so zones are dealt.
        while let StepOutcome::Progress(_) = state.step() {}
        state
    }

    #[test]
    fn observation_hides_hidden_zones() {
        let state = fresh_game();
        let obs = Observation::of(&state, PlayerId(0));
        // Opponent hand and both libraries appear as counts only.
        assert_eq!(obs.opp_hand_count, state.zones.hands[1].len());
        assert_eq!(obs.my_library_count, state.zones.libraries[0].len());
        // No view object belongs to a hidden zone.
        let visible: Vec<ObjectId> = obs
            .my_hand
            .iter()
            .chain(&obs.battlefield)
            .chain(&obs.my_graveyard)
            .chain(&obs.opp_graveyard)
            .map(|v| v.id)
            .collect();
        for id in &visible {
            assert!(
                !state.zones.hands[1].contains(id),
                "opponent hand leaked into the view"
            );
            assert!(
                !state.zones.libraries[0].contains(id) && !state.zones.libraries[1].contains(id),
                "library contents leaked into the view"
            );
        }
        assert_eq!(obs.my_hand.len(), state.zones.hands[0].len());
    }
}
