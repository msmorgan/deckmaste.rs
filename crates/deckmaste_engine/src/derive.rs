//! Minimal characteristics derivation: what abilities an object has. The
//! seed of the stage-4 layer system — in the skeleton, a flat concatenation
//! of printed face abilities and subtype-conferred abilities (CR 305.6 falls
//! out of the data; the engine never special-cases land subtypes).

use deckmaste_core::{
    Ability, Action, Card, CardFace, ColorOrColorless, CostComponent, Effect, ManaSpec, Property,
    Quantity, Uint,
};

use crate::object::ObjectId;
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

/// Skeleton-subset mana-ability check (a subset of CR 605.1a): an activated
/// ability with no targets, cost exactly `[Tap]`, producing a fixed amount
/// of specific mana. Full 605.1a admits more (other costs, `AnyColor`,
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
