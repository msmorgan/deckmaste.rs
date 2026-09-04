//! Transform legality ([CR#701.27]).

use deckmaste_card::Card;
use deckmaste_card::DoubleFacedLayout;
use deckmaste_core::Type;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::Side;
use crate::state::GameState;

/// Whether an instruction to transform `object` actually does anything
/// ([CR#701.27a]). Only a battlefield permanent whose CARD is a Transforming
/// two-faced card, and whose destination face is not an instant/sorcery
/// ([CR#701.27d]), transforms; every other case is a silent no-op (fizzle,
/// never panic). The Transforming-DFC test is on the backing CARD, not on the
/// object's (possibly copied) characteristics ([CR#701.27c,712.9] — a Clone of
/// a DFC's face is not itself a DFC and can't transform).
#[must_use]
pub(crate) fn transform_legal(state: &GameState, object: ObjectId) -> bool {
    let Some(obj) = state.objects.get(object) else {
        return false;
    };
    // [CR#701.27a]: only a permanent (on the battlefield) transforms.
    if obj.zone != Some(Zone::Battlefield) {
        return false;
    }
    // A player proxy has no backing card.
    let Some(card_id) = obj.card_id() else {
        return false;
    };
    // [CR#701.27c,712.9]: only a transforming two-faced CARD transforms.
    let Card::DoubleFaced {
        layout: DoubleFacedLayout::Transforming,
        front,
        back,
    } = state.cards.get(card_id).def.as_ref()
    else {
        return false;
    };
    // [CR#701.27d]: nothing happens if the destination face is instant/sorcery.
    let dest = match obj.side {
        Side::Front => back,
        Side::Back => front,
    };
    !dest
        .characteristics
        .types
        .iter()
        .any(|t| t.name == Type::Instant.name() || t.name == Type::Sorcery.name())
}
