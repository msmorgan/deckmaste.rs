//! `EventApply` handlers (and, for the two bare-payload variants, plain
//! dispatch functions) for zone/battlefield-flavored events: zone changes,
//! attachment, tokens, emblems, and shuffling.

use rand::seq::SliceRandom;

use crate::event::Attached;
use crate::event::EmblemCreated;
use crate::event::GameEvent;
use crate::event::TokenCreated;
use crate::event::Unattached;
use crate::event::ZoneChange;
use crate::object::ObjectId;
use crate::player::PlayerId;
use crate::state::GameState;
use crate::step::EventApply;

impl EventApply for ZoneChange {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        match &self.snapshot {
            None => {
                g.apply_zone_will_change(
                    self.object,
                    self.from,
                    self.to,
                    self.enters.clone(),
                    self.position,
                    self.face,
                    self.cause.clone(),
                );
                None
            }
            // [CR#603.6]: the FACT — the move already happened at the
            // will-change apply. A no-op; triggers (a later task) match here.
            Some(_) => None,
        }
    }
}

impl EventApply for Attached {
    // [CR#701.3a,701.3c]: commit the attachment→host relation — a new
    // timestamp is implicit (no remint; the relation edit IS the
    // transition). The verb builder (`Action::Attach`) already filtered
    // the no-ops; this fact is real, so set the link, then record it for
    // "becomes attached / equipped" triggers (breadth is a seam, §9).
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.objects.obj_mut(self.attachment).attached_to = Some(self.host);
        None
    }
}

impl EventApply for Unattached {
    // [CR#701.3d]: commit the unattach — clear the link. The verb
    // builder filtered the not-attached no-op, so this fact is real.
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.objects.obj_mut(self.attachment).attached_to = None;
        None
    }
}

impl EventApply for TokenCreated {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.apply_token_created(self.player, &self.token, self.enters.clone());
        None
    }
}

impl EventApply for EmblemCreated {
    fn apply(&self, g: &mut GameState) -> Option<GameEvent> {
        g.apply_emblem_created(self.player, self.abilities.clone());
        None
    }
}

/// `TokenCeased` carries a bare `ObjectId` — no dedicated payload struct
/// (Tasks 3.1–3.2 structified only multi-field variants) — so it dispatches
/// through a plain function rather than `EventApply`.
///
/// [CR#704.5d,111.7]: a token found in a zone other than the battlefield
/// ceases to exist. Removes the object from its zone and the store
/// outright — no remint, no `ZoneChange` fact (the token doesn't move, it
/// stops existing). Zone-leave triggers already fired at the move that
/// stranded it ([CR#111.7]'s note); anything still pointing at it reads the
/// LKI that rode that fact.
pub(crate) fn handle_token_ceased(g: &mut GameState, id: ObjectId) -> Option<GameEvent> {
    g.apply_token_ceased(id);
    None
}

/// `Shuffled` carries a bare `PlayerId` — see `handle_token_ceased`.
///
/// [CR#701.24a]: randomize so NO player knows the order — the seeded rng
/// (UD-8). Revealed-state reset ([CR#701.20d]) is unbuilt (no reveal
/// windows exist yet).
pub(crate) fn handle_shuffled(g: &mut GameState, player: PlayerId) -> Option<GameEvent> {
    let image = &mut **g;
    image.zones.libraries[player.index()]
        .make_contiguous()
        .shuffle(&mut image.rng);
    None
}
