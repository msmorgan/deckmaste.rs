//! Last-known information ([CR#603.10a], [CR#608.2]). A value snapshot of an
//! object captured at a zone change, so triggers and effects can read it after
//! the object itself is gone. The engine never retains dead objects.

use std::collections::HashMap;

use deckmaste_core::Ident;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::object::ObjectId;
use crate::object::ObjectSource;
use crate::player::PlayerId;
use crate::state::GameState;

/// An object's last-known state, captured the instant it changes zones.
/// Characteristics (types/P-T/name) derive from `source` via the card
/// definition — correct in Stage 3 (no continuous effects yet to have modified
/// them).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LkiSnapshot {
    /// The now-stale id — a label/reference token, not a live object.
    pub object: ObjectId,
    /// `CardId` spine: derives printed characteristics and the owner.
    pub source: ObjectSource,
    pub controller: PlayerId,
    pub tapped: bool,
    pub damage: Uint,
    /// Counters on the object the instant it left ([CR#122.1,603.10a]) — keyed
    /// by counter name, mirroring `GameObject::counters`. A dies-trigger
    /// reads these once the live object is gone (its id is stale):
    /// Modular's "for each +1/+1 counter on this permanent" count, and
    /// Undying/Persist's "if it had no +1/+1 (resp. -1/-1) counters on it"
    /// intervening-if ([CR#702.93a,702.79a]).
    pub counters: HashMap<Ident, Uint>,
    /// The zone it left.
    pub left: Zone,
}

impl LkiSnapshot {
    /// Snapshot a currently-live object.
    ///
    /// # Panics
    ///
    /// Panics on a player proxy (no zone) or a stale `ObjectId`.
    #[must_use]
    pub fn capture(state: &GameState, id: ObjectId) -> Self {
        let o = state.objects.obj(id);
        LkiSnapshot {
            object: id,
            source: o.source,
            controller: o.controller,
            tapped: o.tapped,
            damage: o.damage,
            counters: o.counters.clone(),
            left: o.zone.expect("a zoned object has a zone to leave"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use deckmaste_cards::plugin::Plugin;

    use super::*;
    use crate::object::ObjectSource;
    use crate::player::PlayerId;
    use crate::state::GameConfig;
    use crate::state::GameState;
    use crate::state::PlayerConfig;
    use crate::state::StartingPlayer;

    fn canon() -> Plugin {
        Plugin::load_with_sibling_prelude(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/canon"),
        )
        .unwrap()
    }

    /// [CR#603.10a]: `LkiSnapshot::capture` records the object's counters at the
    /// instant it leaves — the data Undying/Persist's intervening-if reads off
    /// the dying object. Two +1/+1 counters on a battlefield creature are
    /// captured; the snapshot is decoupled from the live object (clearing the
    /// live map after capture leaves the snapshot untouched).
    #[test]
    fn capture_records_object_counters() {
        let bears = Arc::new(canon().card("Grizzly Bears").unwrap());
        let mut state = GameState::new(GameConfig {
            players: vec![
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 4],
                },
                PlayerConfig {
                    deck: vec![Arc::clone(&bears); 4],
                },
            ],
            seed: 5,
            starting_life: 20,
            starting_player: StartingPlayer::Fixed(PlayerId(0)),
            sba_rules: vec![],
            counter_decls: std::collections::HashMap::new(),
            subtypes: std::collections::HashMap::new(),
        });
        let bear_card = state.cards.push(Arc::clone(&bears), PlayerId(0));
        let bear = state.objects.mint(
            ObjectSource::Card(bear_card),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        state.zones.battlefield.push(bear);
        state
            .objects
            .obj_mut(bear)
            .counters
            .insert("P1P1Counter".into(), 2);

        let snapshot = LkiSnapshot::capture(&state, bear);
        assert_eq!(
            snapshot.counters.get("P1P1Counter").copied(),
            Some(2),
            "the +1/+1 counters are captured on the snapshot"
        );
        assert_eq!(
            snapshot.counters.get("M1M1Counter").copied(),
            None,
            "an absent counter kind is absent on the snapshot"
        );

        // The snapshot is a value copy, independent of the live object.
        state.objects.obj_mut(bear).counters.clear();
        assert_eq!(
            snapshot.counters.get("P1P1Counter").copied(),
            Some(2),
            "the snapshot survives the live counters being cleared"
        );
    }
}
