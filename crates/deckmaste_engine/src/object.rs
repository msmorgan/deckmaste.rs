use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use deckmaste_core::Ability;
use deckmaste_core::Card;
use deckmaste_core::CardFace;
use deckmaste_core::Ident;
use deckmaste_core::Subtype;
use deckmaste_core::Token;
use deckmaste_core::Uint;
use deckmaste_core::Zone;

use crate::player::PlayerId;

/// A transient object identity ([CR#109]): a fresh id is minted on every zone
/// change ([CR#400.7]). The backing [`CardId`] is what persists across moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId(pub Uint);

/// A continuous-effect ordering stamp ([CR#613.7]). One monotonic clock spans
/// objects (stamped at mint, zone-entry [CR#613.7d]) and floating effects
/// (stamped at creation [CR#613.7b]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub Uint);

/// A persistent card identity ([CR#108]): an index into the game's card table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardId(pub Uint);

/// One physical card ([CR#108]) — or a created token's definition: its
/// shared characteristics and its owner, fixed for the whole game
/// ([CR#108.3]; [CR#111.2] for a token's creator).
#[derive(Debug, Clone)]
pub struct CardInstance {
    pub def: Arc<Card>,
    pub owner: PlayerId,
    /// [CR#111.6]: a token isn't a card. Set for entries synthesized by
    /// `TokenCreated`; `object_kind` reports `Token` (so `Filter::Kind(Card)`
    /// excludes them) and the ceases-to-exist SBA ([CR#704.5d]) keys on it.
    pub is_token: bool,
    /// The face's printed + subtype-conferred abilities, precomputed at setup
    /// so the layer pipeline's base values are an `Arc` bump per rebuild
    /// instead of a deep clone per object.
    pub(crate) printed: Arc<Vec<Ability>>,
    /// The face's subtypes, shared for the same reason (`Subtype` carries its
    /// `confers` payload, so cloning it per rebuild is as heavy as abilities).
    pub(crate) subtypes: Arc<Vec<Subtype>>,
    /// Base colors ([CR#202.2]: cost symbols, else color indicator).
    pub(crate) colors: Arc<Vec<deckmaste_core::Color>>,
    pub(crate) card_types: Arc<Vec<deckmaste_core::Type>>,
    pub(crate) supertypes: Arc<Vec<deckmaste_core::Supertype>>,
}

/// The game's card table: the cards the decklists brought, built at game
/// start, plus one synthesized entry per created token ([CR#111.3] — its
/// characteristics are exactly what the creating effect defined). Entries are
/// never removed or mutated; a ceased token's entry stays as inert history.
#[derive(Debug, Clone, Default)]
pub struct Cards(Vec<CardInstance>);

impl Cards {
    /// Adds a card at game setup and returns its id.
    ///
    /// # Panics
    ///
    /// Panics if the card table exceeds `Uint::MAX` entries.
    pub(crate) fn push(&mut self, def: Arc<Card>, owner: PlayerId) -> CardId {
        self.push_inner(def, owner, false)
    }

    /// Adds a created token's synthesized definition ([CR#111.2]: `owner` is
    /// its creator) and returns its id. The `Token`'s characteristics become a
    /// one-faced card definition, so tokens ride the same derivation / layer /
    /// LKI machinery as cards; only the `is_token` flag tells them apart
    /// ([CR#111.6]). The name defaults to the subtypes plus the word "Token"
    /// ([CR#111.4] — `Token` carries no name of its own yet).
    pub(crate) fn push_token(&mut self, token: &Token, owner: PlayerId) -> CardId {
        let name = token
            .subtypes
            .iter()
            .map(|s| s.name.as_str())
            .chain(std::iter::once("Token"))
            .collect::<Vec<_>>()
            .join(" ");
        let def = Arc::new(Card::Normal(CardFace {
            name,
            mana_cost: deckmaste_core::ManaCost::default(),
            color_indicator: vec![],
            supertypes: token.supertypes.clone(),
            types: token.types.clone(),
            subtypes: token.subtypes.clone(),
            abilities: token.abilities.clone(),
            power: None,
            toughness: None,
            loyalty: None,
            defense: None,
        }));
        self.push_inner(def, owner, true)
    }

    /// # Panics
    ///
    /// Panics if the card table exceeds `Uint::MAX` entries.
    fn push_inner(&mut self, def: Arc<Card>, owner: PlayerId, is_token: bool) -> CardId {
        let id = CardId(Uint::try_from(self.0.len()).expect("card table fits in Uint"));
        let face = crate::derive::face(&def);
        let printed = Arc::new(crate::derive::printed_of_face(face));
        let subtypes = Arc::new(face.subtypes.clone());
        let colors = Arc::new(crate::layer::base_colors(face));
        let card_types = Arc::new(face.types.clone());
        let supertypes = Arc::new(face.supertypes.clone());
        self.0.push(CardInstance {
            def,
            owner,
            is_token,
            printed,
            subtypes,
            colors,
            card_types,
            supertypes,
        });
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

/// Where an object's identity comes from ([CR#109]). A created token is
/// `Card`-backed too — `TokenCreated` synthesizes its definition into the
/// card table, flagged `is_token` ([CR#111.6]).
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
    /// Layer-system ordering stamp ([CR#613.7]): assigned at mint / zone-entry
    /// ([CR#613.7d]) from the shared monotonic clock in [`ObjectStore`].
    pub timestamp: Timestamp,
    /// Meaningful only on the battlefield.
    pub tapped: bool,
    /// [CR#302.6]: set when the object enters the battlefield, cleared at the
    /// controller's turn start — a creature controlled continuously since the
    /// turn began is not summoning-sick. Meaningful only on the battlefield.
    pub summoning_sick: bool,
    /// Marked damage ([CR#120.3,704.5g]) — meaningful only on the battlefield.
    pub damage: Uint,
    /// Set when this object has been dealt damage by a deathtouch source
    /// ([CR#702.2]). Any nonzero damage from such a source destroys a creature
    /// with toughness > 0; the SBA checks this flag alongside lethal marked
    /// damage ([CR#704.5h]).
    /// Meaningful only on the battlefield; cleared at Cleanup ([CR#514.2]).
    pub struck_by_deathtouch: bool,
    /// Counters on this object, keyed by counter name ([CR#122]).
    /// `"+1/+1"` and `"-1/-1"` modify P/T in layer 7c ([CR#613.4c]).
    pub counters: HashMap<Ident, Uint>,
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
    /// Shared monotonic clock for both object timestamps ([CR#613.7d]) and
    /// floating-effect timestamps ([CR#613.7b]): one total order over all.
    clock: Uint,
}

impl ObjectStore {
    /// Draws the next timestamp from the shared monotonic clock and advances
    /// it. Floating effects (a later task) use this same clock so that object
    /// and effect timestamps are totally ordered ([CR#613.7]).
    pub(crate) fn next_timestamp(&mut self) -> Timestamp {
        let t = Timestamp(self.clock);
        self.clock += 1;
        t
    }

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
        let timestamp = self.next_timestamp();
        self.objects.insert(
            id,
            GameObject {
                id,
                source,
                controller,
                timestamp,
                tapped: false,
                summoning_sick: false,
                damage: 0,
                struck_by_deathtouch: false,
                counters: HashMap::new(),
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

    /// Removes an object — its identity is gone ([CR#400.7] reminting; no LKI
    /// retention).
    ///
    /// # Panics
    ///
    /// Panics if the id was not present — engine invariant, not caller input.
    pub fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id).expect("removing a live ObjectId");
    }
}
