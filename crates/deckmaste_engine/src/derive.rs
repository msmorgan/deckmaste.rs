//! Minimal characteristics derivation: what abilities an object has. The
//! seed of the stage-4 layer system — in the skeleton, a flat concatenation
//! of printed face abilities and subtype-conferred abilities ([CR#305.6] falls
//! out of the data; the engine never special-cases land subtypes).

use deckmaste_core::{
    Ability, Action, Card, CardFace, ColorOrColorless, CostComponent, Effect, ManaSpec, Property,
    Quantity, Uint,
};

use crate::object::{ObjectId, ObjectSource};
use crate::state::GameState;

/// The face an object presents. Skeleton: the front face.
#[must_use]
pub fn face(card: &Card) -> &CardFace {
    match card {
        Card::Normal(f) | Card::ModalDfc(f, _) => f,
    }
}

/// The object's abilities: printed plus subtype-conferred (the
/// `Property::Ability` arm; other flavors execute elsewhere), in that
/// order. `Action::ActivateAbility` indexes this list.
#[must_use]
pub fn abilities(state: &GameState, id: ObjectId) -> Vec<&Ability> {
    let face = face(state.def(id));
    face.abilities
        .iter()
        .chain(face.subtypes.iter().flat_map(|s| {
            s.confers.iter().filter_map(|p| match p {
                Property::Ability(a) => Some(&**a),
                _ => None,
            })
        }))
        .collect()
}

/// The PRINTED abilities of whatever an `ObjectSource` names — the abilities
/// the trigger scan considers for a watcher. For a card-backed source this is
/// the face's printed list (the same spine that survives reminting and LKI);
/// a player proxy has none. Granted/conferred abilities are a later stage
/// ([CR#603.2] watching abilities; Stage 3 has no continuous effects).
#[must_use]
pub fn abilities_of_source(state: &GameState, source: ObjectSource) -> Vec<Ability> {
    match source {
        ObjectSource::Card(card) => face(&state.cards.get(card).def).abilities.clone(),
        ObjectSource::Player(_) => vec![],
    }
}

/// Skeleton-subset mana-ability check (a subset of [CR#605.1a]): an activated
/// ability with no targets, cost exactly `[Tap]`, producing a fixed amount
/// of specific mana. Full [CR#605.1a] admits more (other costs, `AnyColor`,
/// loyalty exclusion) — not yet needed here. Returns what it produces.
/// Keyword wrappers are looked through.
#[must_use]
pub fn tap_mana_ability(ability: &Ability) -> Option<(ColorOrColorless, Uint)> {
    match ability {
        Ability::Activated(a)
            if a.targets.is_empty() && a.cost.as_slice() == [CostComponent::Tap] =>
        {
            match &a.effect {
                Effect::Act(Action::AddMana(Quantity::Literal(n), ManaSpec::Specific(m))) => {
                    Some((*m, *n))
                }
                _ => None,
            }
        }
        Ability::Expanded(e) => tap_mana_ability(&e.value),
        _ => None,
    }
}
