use std::collections::BTreeMap;
use std::sync::Arc;

use deckmaste_core::{Card, Uint, Zone};

use crate::player::PlayerId;

/// A transient object identity ([CR#109]). Reminted on zone change in a later
/// stage; in the skeleton an object keeps its id for the whole game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId(pub Uint);

/// A persistent card identity ([CR#108]): an index into the game's card table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardId(pub Uint);

/// One physical card ([CR#108]): its shared definition and its owner, fixed
/// for the whole game ([CR#108.3]).
#[derive(Debug, Clone)]
pub struct CardInstance {
    pub def: Arc<Card>,
    pub owner: PlayerId,
}

/// The game's card table: exactly the cards the decklists brought, built at
/// game start and never mutated after.
#[derive(Debug, Clone, Default)]
pub struct Cards(Vec<CardInstance>);

impl Cards {
    /// Adds a card at game setup and returns its id.
    ///
    /// # Panics
    ///
    /// Panics if the card table exceeds `Uint::MAX` entries.
    pub(crate) fn push(&mut self, def: Arc<Card>, owner: PlayerId) -> CardId {
        let id = CardId(Uint::try_from(self.0.len()).expect("card table fits in Uint"));
        self.0.push(CardInstance { def, owner });
        id
    }

    /// The card at `id`.
    ///
    /// # Panics
    ///
    /// Panics on a fabricated `CardId` — engine invariant, not caller input.
    #[must_use]
    pub fn get(&self, id: CardId) -> &CardInstance { &self.0[id.0 as usize] }

    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}

/// Where an object's identity comes from ([CR#109]). Tokens are deferred — no
/// fixture creates them yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSource {
    Card(CardId),
    Player(PlayerId),
}

/// An object in the game ([CR#109]). An object whose `zone ==
/// Some(Battlefield)` is a permanent ([CR#110.1]). A player proxy has `source =
/// Player(..)` and `zone == None` (players are objects here, but in no zone).
#[derive(Debug, Clone)]
pub struct GameObject {
    pub id: ObjectId,
    pub source: ObjectSource,
    pub controller: PlayerId,
    /// Meaningful only on the battlefield.
    pub tapped: bool,
    /// Marked damage ([CR#120.3,704.5g]) — meaningful only on the battlefield.
    pub damage: Uint,
    /// `None` for a player proxy.
    pub zone: Option<Zone>,
}

impl GameObject {
    /// The backing card, or `None` for a player proxy.
    #[must_use]
    pub fn card_id(&self) -> Option<CardId> {
        match self.source {
            ObjectSource::Card(c) => Some(c),
            ObjectSource::Player(_) => None,
        }
    }
}

/// All live objects, keyed by id. `BTreeMap` for deterministic iteration.
#[derive(Debug, Clone, Default)]
pub struct ObjectStore {
    objects: BTreeMap<ObjectId, GameObject>,
    next: Uint,
}

impl ObjectStore {
    /// Creates an object and returns its id.
    ///
    /// # Panics
    ///
    /// Panics if the object count exceeds `Uint::MAX` — engine invariant, not
    /// caller input.
    #[must_use]
    pub fn mint(
        &mut self,
        source: ObjectSource,
        controller: PlayerId,
        zone: Option<Zone>,
    ) -> ObjectId {
        let id = ObjectId(self.next);
        self.next += 1;
        self.objects.insert(
            id,
            GameObject {
                id,
                source,
                controller,
                tapped: false,
                damage: 0,
                zone,
            },
        );
        id
    }

    #[must_use]
    pub fn get(&self, id: ObjectId) -> Option<&GameObject> { self.objects.get(&id) }

    /// Panics if the id is stale — engine invariant, not caller input.
    ///
    /// # Panics
    ///
    /// Panics if the id does not exist in the object store.
    #[must_use]
    pub fn obj(&self, id: ObjectId) -> &GameObject { self.objects.get(&id).expect("live ObjectId") }

    /// Panics if the id is stale — engine invariant, not caller input.
    ///
    /// # Panics
    ///
    /// Panics if the id does not exist in the object store.
    pub fn obj_mut(&mut self, id: ObjectId) -> &mut GameObject {
        self.objects.get_mut(&id).expect("live ObjectId")
    }

    pub fn iter(&self) -> impl Iterator<Item = &GameObject> { self.objects.values() }
}
