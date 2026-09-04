use std::collections::HashMap;
use std::sync::Arc;

use deckmaste_card::Card;
use deckmaste_card::CardFace;
use deckmaste_card::Characteristics;
use deckmaste_core::Ability;
use deckmaste_core::Ident;
use deckmaste_core::Subtype;
use deckmaste_core::Token;
use deckmaste_core::Uint;
use deckmaste_core::Zone;
use slotmap::SlotMap;

use crate::player::PlayerId;

slotmap::new_key_type! {
    /// A transient object identity ([CR#109]): a fresh key is minted on every
    /// zone change ([CR#400.7]). slotmap's generational versioning means a stale
    /// key — the object left its zone, or its slot was later reused — resolves to
    /// `None` on lookup, so identity is self-checking. The backing [`CardId`] is
    /// what persists across moves.
    pub struct ObjectId;
}

#[cfg(test)]
impl ObjectId {
    /// Fabricates a distinct id from a raw value, for in-crate tests that need
    /// opaque ids without a live store (history / combat fixtures).
    /// Engine code never calls this — real ids come only from
    /// [`ObjectStore::mint`].
    pub(crate) fn from_raw(n: u64) -> Self {
        slotmap::KeyData::from_ffi(n).into()
    }
}

/// A continuous-effect ordering stamp ([CR#613.7]). One monotonic clock spans
/// objects (stamped at mint, zone-entry [CR#613.7d]) and floating effects
/// (stamped at creation [CR#613.7b]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Timestamp(pub Uint);

/// A persistent card identity ([CR#108]): an index into the game's card table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CardId(pub Uint);

/// Precomputed base copiable values for one face, shared by `Arc` so a layer
/// rebuild is a bump not a deep clone (see `CardInstance`).
#[derive(Debug, Clone)]
pub(crate) struct FaceCache {
    /// The face's INTRINSIC printed abilities (not type/subtype conferrals,
    /// which the layer-4 fold re-derives per pass), precomputed at setup so the
    /// layer pipeline's base values are an `Arc` bump per rebuild instead of a
    /// deep clone per object.
    pub(crate) printed: Arc<Vec<Ability>>,
    /// The face's subtypes, shared for the same reason (`Subtype` carries its
    /// `confers` payload, so cloning it per rebuild is as heavy as abilities).
    pub(crate) subtypes: Arc<Vec<Subtype>>,
    /// Base colors ([CR#202.2]: cost symbols, else color indicator).
    pub(crate) colors: Arc<Vec<deckmaste_core::Color>>,
    pub(crate) card_types: Arc<Vec<deckmaste_core::TypeDef>>,
    pub(crate) supertypes: Arc<Vec<deckmaste_core::Supertype>>,
}

/// One physical card ([CR#108]) — or a created token's definition: its
/// shared characteristics and its owner, fixed for the whole game
/// ([CR#108.3]; [CR#111.2] for a token's creator).
#[derive(Debug, Clone)]
pub struct CardInstance {
    pub def: Arc<Card>,
    pub owner: PlayerId,
    /// [CR#111.6]: a token isn't a card. Set for entries synthesized by
    /// `TokenCreated`; `is_object_class` reports `Token` (so
    /// `Predicate::Kind(Card)` excludes them) and the ceases-to-exist SBA
    /// ([CR#704.5d]) keys on it.
    pub is_token: bool,
    /// [CR#114.5]: an emblem is neither a card nor a permanent, and "Emblem"
    /// isn't a card type. Set for entries synthesized by `EmblemCreated`; the
    /// def carries only the emblem's abilities ([CR#114.3]), and `is_object_class`
    /// reports `Emblem` so every card/type/permanent filter excludes it.
    pub is_emblem: bool,
    /// Base copiable values for the FRONT face ([CR#712.8d]) — the face every
    /// object presents off the battlefield ([CR#712.8a]) and the default on it.
    pub(crate) front: FaceCache,
    /// Present only for `Card::DoubleFaced` — the back face's base values
    /// ([CR#712.8e]).
    pub(crate) back: Option<FaceCache>,
}

impl CardInstance {
    /// The base cache for the given face — back only when this is a two-faced
    /// card and `Side::Back` is requested, else front ([CR#712.8d,712.8e]).
    pub(crate) fn face_cache(&self, side: crate::object::Side) -> &FaceCache {
        match (side, &self.back) {
            (crate::object::Side::Back, Some(back)) => back,
            _ => &self.front,
        }
    }
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
        self.push_inner(def, owner, false, false)
    }

    /// Adds a created token's synthesized definition ([CR#111.2]: `owner` is
    /// its creator) and returns its id. The `Token`'s characteristics become a
    /// one-faced card definition, so tokens ride the same derivation / layer /
    /// LKI machinery as cards; only the `is_token` flag tells them apart
    /// ([CR#111.6]) — including a token that's a copy ([CR#707.1]): its
    /// copiable values are already baked into the `Token` passed here, so
    /// no separate copy marker is needed ([CR#109.1] treats "a token" and
    /// "a copy of a card" as distinct kinds; a token copy is the former).
    /// The name honors an explicit `token.name` (set by a copy token per
    /// [CR#707.2]) when present; otherwise it defaults to the subtypes plus
    /// the word "Token" ([CR#111.4]).
    pub(crate) fn push_token(&mut self, token: &Token, owner: PlayerId) -> CardId {
        let name = token.name.clone().unwrap_or_else(|| {
            token
                .subtypes
                .iter()
                .map(|s| s.name.as_str())
                .chain(std::iter::once("Token"))
                .collect::<Vec<_>>()
                .join(" ")
                .into()
        });
        let def = Arc::new(Card::Normal(CardFace::from(Characteristics {
            name,
            mana_cost: deckmaste_core::ManaCost::default(),
            color_indicator: token.color_indicator.to_vec(),
            supertypes: token.supertypes.to_vec(),
            types: token.types.to_vec(),
            subtypes: token.subtypes.to_vec(),
            abilities: token.abilities.to_vec(),
            power: token.power.clone(),
            toughness: token.toughness.clone(),
            loyalty: None,
            defense: None,
        })));
        self.push_inner(def, owner, true, false)
    }

    /// Adds an emblem's synthesized definition ([CR#114.1]: created by an
    /// effect, owned by that effect's controller — [CR#114.2]) and returns its
    /// id. An emblem has NO characteristics other than its abilities
    /// ([CR#114.3]): the def is a genuinely typeless, nameless, cost-less
    /// one-faced `Card` carrying only `abilities`, flagged `is_emblem` so it
    /// presents as neither a card nor a permanent ([CR#114.5]). It rides the
    /// same derivation / layer machinery as a token, so its abilities function
    /// through `abilities_of_source` unchanged.
    pub(crate) fn push_emblem(&mut self, abilities: Vec<Ability>, owner: PlayerId) -> CardId {
        let def = Arc::new(Card::Normal(CardFace::from(Characteristics {
            abilities,
            ..Characteristics::default()
        })));
        self.push_inner(def, owner, false, true)
    }

    /// # Panics
    ///
    /// Panics if the card table exceeds `Uint::MAX` entries.
    fn push_inner(
        &mut self,
        def: Arc<Card>,
        owner: PlayerId,
        is_token: bool,
        is_emblem: bool,
    ) -> CardId {
        let id = CardId(Uint::try_from(self.0.len()).expect("card table fits in Uint"));
        let build = |face: &CardFace| FaceCache {
            printed: Arc::new(crate::derive::printed_of_face(face)),
            subtypes: Arc::new(face.characteristics.subtypes.clone()),
            colors: Arc::new(crate::layer::base_colors(face)),
            card_types: Arc::new(face.characteristics.types.clone()),
            supertypes: Arc::new(face.characteristics.supertypes.clone()),
        };
        let front = build(crate::derive::face(&def));
        let back = match def.as_ref() {
            Card::DoubleFaced { back, .. } => Some(build(back)),
            Card::Normal(_) | Card::Split { .. } | Card::Flip { .. } | Card::Adventurer { .. } => {
                None
            }
        };
        self.0.push(CardInstance {
            def,
            owner,
            is_token,
            is_emblem,
            front,
            back,
        });
        id
    }

    /// The card at `id`.
    ///
    /// # Panics
    ///
    /// Panics on a fabricated `CardId` — engine invariant, not caller input.
    #[must_use]
    pub fn get(&self, id: CardId) -> &CardInstance {
        &self.0[id.0 as usize]
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Where an object's identity comes from ([CR#109]). A created token is
/// `Card`-backed too — `TokenCreated` synthesizes its definition into the
/// card table, flagged `is_token` ([CR#111.6]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectSource {
    Card(CardId),
    Player(PlayerId),
}

/// Which face of a double-faced card a battlefield permanent currently shows
/// ([CR#712.8d,712.8e]). Meaningful only for a `Card::DoubleFaced { layout:
/// DoubleFacedLayout::Transforming, .. }` on the battlefield; every other
/// object stays `Front`.
/// Distinct from the morph `status::Face { Up, Down }` — that is face-up vs
/// face-down ([CR#708]), this is front vs back ([CR#712]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Front,
    Back,
}

/// One instance of marked damage on a permanent ([CR#120.3]), tagged with the
/// source that dealt it and that source's abilities captured *at deal time*.
/// Provenance is deal-time because damage-source effects — deathtouch most
/// notably ([CR#704.5h]) — are decided when the damage is dealt, not when a
/// later SBA reads the mark: the source may have lost the ability or left the
/// battlefield in between, and it still counts.
/// Replaces the bespoke deal-time `struck_by_deathtouch` bool.
#[derive(Debug, Clone)]
pub struct DamageMark {
    /// The source's identity the instant it dealt the damage, or `None` when
    /// the source had already left (a dies-trigger whose source is a stale,
    /// reminted id — [CR#603.10a]). Kept for future source-relative reads;
    /// deathtouch reads `source_abilities`, not identity.
    pub source: Option<ObjectSource>,
    /// The source's derived abilities captured the instant it dealt the damage
    /// ([CR#704.5h]) — the deal-time snapshot `DealtDamageBy(subject, Has(kw))`
    /// reads.
    /// Empty when the source had already left.
    pub source_abilities: Vec<Ability>,
    /// How much damage this instance marked.
    pub amount: Uint,
}

/// An addressable Entity in the game — an object ([CR#109]) or, through a
/// proxy, a player ([CR#102.1]). An object whose `zone == Some(Battlefield)` is
/// a permanent ([CR#110.1]). A player proxy has `source = Player(..)` and
/// `zone == None`: a player is not in a Zone ([CR#400.1]), and the shared
/// `ObjectId` store is storage, not a claim about the CR classification.
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
    /// [CR#502.3,701.43a]: the one-shot "doesn't untap during your next untap
    /// step" rider — exert, and the temple/painland mana riders. Set by an
    /// effect; consumed at this permanent's controller's next untap step (the
    /// untap turn-based action clears it and suppresses that one untap),
    /// mirroring `summoning_sick`'s set-by-arrival / cleared-by-turn-start
    /// life-cycle. The one-shot twin of the continuous
    /// `Cant(Untap)` restriction. Meaningful only on the battlefield; a zone
    /// change remints a fresh object, so it never rides a leaving permanent.
    pub skip_next_untap: bool,
    /// Marked damage ([CR#120.3,704.5g]) as a list of source-tagged instances
    /// — meaningful only on the battlefield. Each [`DamageMark`] carries the
    /// dealing source's deal-time abilities, so the lethal-damage SBA can read
    /// deathtouch provenance ([CR#704.5h]) generically. The scalar total is
    /// [`total_damage`](GameObject::total_damage).
    pub damage: Vec<DamageMark>,
    /// Counters on this object, keyed by counter name ([CR#122]).
    /// `"+1/+1"` and `"-1/-1"` modify P/T in layer 7c ([CR#613.4c]).
    pub counters: HashMap<Ident, Uint>,
    /// The host this attachment is attached to ([CR#301.5,303.4]). The
    /// attachment→host direction is the single source of truth; the inverse
    /// is derived by scanning. Cleared on remint / zone change so a leaving
    /// attachment becomes unattached ([CR#701.3d]) — `mint` always sets it
    /// `None`, and a zone change remints a fresh object.
    pub attached_to: Option<ObjectId>,
    /// The face this permanent currently shows ([CR#712.18]: a transform does
    /// not remint, so this persists across a flip; a zone change remints and it
    /// resets to `Front`, [CR#712.14]). Meaningful only on the battlefield.
    pub side: Side,
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

    /// Total marked damage ([CR#120.3]) — the sum over every source-tagged
    /// [`DamageMark`]. The scalar the lethal-marked-damage SBA ([CR#704.5g])
    /// and LKI compare against toughness.
    #[must_use]
    pub fn total_damage(&self) -> Uint {
        self.damage.iter().map(|m| m.amount).sum()
    }

    /// Record one instance of marked damage from a source, capturing the
    /// source's deal-time abilities ([CR#120.3,702.2c]). `amount == 0` still
    /// records a (provenance-only) mark so a 0-damage deathtouch strike is not
    /// silently dropped — but the caller (`step.rs`) only marks nonzero damage.
    pub fn mark_damage(
        &mut self,
        source: Option<ObjectSource>,
        source_abilities: Vec<Ability>,
        amount: Uint,
    ) {
        self.damage.push(DamageMark {
            source,
            source_abilities,
            amount,
        });
    }

    /// Remove all marked damage ([CR#514.2] cleanup, [CR#701.19a]
    /// regeneration) — clears the deal-time deathtouch provenance with it, so a
    /// healed creature is no longer "dealt damage by a deathtouch source".
    pub fn clear_damage(&mut self) {
        self.damage.clear();
    }

    /// Set the total marked damage to a single anonymous instance of `amount`
    /// (no source identity, no deal-time abilities) — a scenario-setup shortcut
    /// for tests and tools that only care about the lethal-marked-damage
    /// total, not provenance. Deathtouch scenarios use
    /// [`mark_damage`](GameObject::mark_damage) with a deathtouch ability.
    pub fn set_marked_damage(&mut self, amount: Uint) {
        self.damage.clear();
        if amount > 0 {
            self.mark_damage(None, Vec::new(), amount);
        }
    }
}

/// All live objects, keyed by a generational [`ObjectId`]. The slotmap mints
/// the keys (so ids come for free) and bumps a slot's generation when it's
/// reused — exactly the transient-identity remint of [CR#400.7], with the bonus
/// that a key can never resolve to a different object than the one it was
/// minted for.
#[derive(Debug, Clone, Default)]
pub struct ObjectStore {
    objects: SlotMap<ObjectId, GameObject>,
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

    /// Creates an object and returns its freshly minted id.
    ///
    /// # Panics
    ///
    /// Panics if the live-object count reaches the slotmap's capacity
    /// (`2^32 - 2`) — engine invariant, not caller input.
    #[must_use]
    pub fn mint(
        &mut self,
        source: ObjectSource,
        controller: PlayerId,
        zone: Option<Zone>,
    ) -> ObjectId {
        let timestamp = self.next_timestamp();
        self.objects.insert_with_key(|id| GameObject {
            id,
            source,
            controller,
            timestamp,
            tapped: false,
            summoning_sick: false,
            skip_next_untap: false,
            damage: Vec::new(),
            counters: HashMap::new(),
            attached_to: None,
            side: Side::Front,
            zone,
        })
    }

    #[must_use]
    pub fn get(&self, id: ObjectId) -> Option<&GameObject> {
        self.objects.get(id)
    }

    /// Panics if the id is stale — engine invariant, not caller input.
    ///
    /// # Panics
    ///
    /// Panics if the id does not exist in the object store.
    #[must_use]
    pub fn obj(&self, id: ObjectId) -> &GameObject {
        self.objects.get(id).expect("live ObjectId")
    }

    /// Panics if the id is stale — engine invariant, not caller input.
    ///
    /// # Panics
    ///
    /// Panics if the id does not exist in the object store.
    pub fn obj_mut(&mut self, id: ObjectId) -> &mut GameObject {
        self.objects.get_mut(id).expect("live ObjectId")
    }

    pub fn iter(&self) -> impl Iterator<Item = &GameObject> {
        self.objects.values()
    }

    /// Removes an object — its identity is gone ([CR#400.7] reminting; no LKI
    /// retention).
    ///
    /// # Panics
    ///
    /// Panics if the id was not present — engine invariant, not caller input.
    pub fn remove(&mut self, id: ObjectId) {
        self.objects.remove(id).expect("removing a live ObjectId");
    }
}

#[cfg(test)]
mod tests {
    use deckmaste_core::Color;
    use deckmaste_core::StatValue;
    use deckmaste_core::Type;

    use super::*;

    /// A synthesized token reflects the creating effect's color [CR#202.2e]
    /// and P/T [CR#111.3], not a colorless statless default.
    #[test]
    fn push_token_carries_color_and_pt() {
        let mut cards = Cards::default();
        let token = Token {
            name: None,
            color_indicator: vec![Color::Red].into(),
            supertypes: vec![].into(),
            types: vec![Type::Creature.def()].into(),
            subtypes: vec![].into(),
            abilities: vec![].into(),
            power: Some(StatValue::Number(1)),
            toughness: Some(StatValue::Number(1)),
        };
        let id = cards.push_token(&token, PlayerId(0));
        let inst = cards.get(id);
        assert_eq!(*inst.front.colors, vec![Color::Red]);
        let Card::Normal(face) = inst.def.as_ref() else {
            panic!("a token synthesizes a one-faced Normal card");
        };
        assert_eq!(face.characteristics.power, Some(StatValue::Number(1)));
        assert_eq!(face.characteristics.toughness, Some(StatValue::Number(1)));
    }

    /// An unnamed token still synthesizes subtypes + "Token" [CR#111.4] — the
    /// `name: None` default from the test above, spelled out explicitly here
    /// as the control case for the next test.
    #[test]
    fn push_token_with_no_name_synthesizes_subtypes_plus_token() {
        let mut cards = Cards::default();
        let token = Token {
            name: None,
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Creature.def()].into(),
            subtypes: vec![deckmaste_core::Subtype {
                name: "Bear".into(),
                types: vec![Type::Creature].into(),
                confers: vec![].into(),
            }]
            .into(),
            abilities: vec![].into(),
            power: None,
            toughness: None,
        };
        let id = cards.push_token(&token, PlayerId(0));
        let Card::Normal(face) = cards.get(id).def.as_ref() else {
            panic!("a token synthesizes a one-faced Normal card");
        };
        assert_eq!(
            &*face.characteristics.name, "Bear Token",
            "[CR#111.4]: no explicit name -> subtypes + \"Token\""
        );
    }

    /// An explicit `token.name` (a copy token, [CR#707.2]) is used as-is —
    /// it does NOT resynthesize from subtypes the way an unnamed token does.
    #[test]
    fn push_token_with_explicit_name_uses_it_verbatim() {
        let mut cards = Cards::default();
        let token = Token {
            name: Some("Grizzly Bears".into()),
            color_indicator: vec![].into(),
            supertypes: vec![].into(),
            types: vec![Type::Creature.def()].into(),
            subtypes: vec![deckmaste_core::Subtype {
                name: "Bear".into(),
                types: vec![Type::Creature].into(),
                confers: vec![].into(),
            }]
            .into(),
            abilities: vec![].into(),
            power: None,
            toughness: None,
        };
        let id = cards.push_token(&token, PlayerId(0));
        let Card::Normal(face) = cards.get(id).def.as_ref() else {
            panic!("a token synthesizes a one-faced Normal card");
        };
        assert_eq!(
            &*face.characteristics.name, "Grizzly Bears",
            "[CR#707.2]: an explicit name is carried verbatim, not resynthesized"
        );
    }

    #[test]
    fn mint_defaults_side_front() {
        let mut store = ObjectStore::default();
        let id = store.mint(
            ObjectSource::Player(PlayerId(0)),
            PlayerId(0),
            Some(Zone::Battlefield),
        );
        assert_eq!(
            store.obj(id).side,
            Side::Front,
            "objects enter Front-face-up [CR#712.14]"
        );
    }
}
