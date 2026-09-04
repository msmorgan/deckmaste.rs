use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Cmp;
use crate::Color;
use crate::Condition;
use crate::Count;
use crate::Ident;
use crate::Normalize;
use crate::PlayerAttr;
use crate::Reference;
use crate::Stat;
use crate::Status;
use crate::Supertype;
use crate::Type;
use crate::Zone;

/// Which Entity a candidate is ([Game Model glossary]). A Player and an
/// Object are both Entities; a Player is NOT an Object — [CR#109.1] lists what
/// an object is and a player is not among them ([CR#102.1]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EntityClass {
    /// One of the people in the game ([CR#102.1]).
    Player,
    /// An object ([CR#109.1]).
    Object,
}

/// One of the overlapping ways [CR#109.1] classifies an object. The classes
/// are independently testable and NONEXCLUSIVE: a card on the stack is both
/// `Card` and `Spell` ([CR#108.2,112.1]); a token on the battlefield is both
/// `Token` and `Permanent` ([CR#111.1,110.1]). `Spell` and `Permanent` are
/// derived from zone and stack state, but they are the CR classes rather than
/// storage tags — nothing outside the engine classifier reads the storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ObjectClass {
    /// An activated or triggered ability on the stack ([CR#602.2a,603.3]) —
    /// on the stack without a card ([CR#405.1]), so never `Card` or `Spell`.
    /// What "counter target activated ability" / "target activated or
    /// triggered ability" (Stifle, Disallow) selects over.
    AbilityOnStack,
    /// A Magic card or an object represented by a Magic card ([CR#108.2]). A
    /// token is not a card ([CR#108.2b]) and an emblem is neither a card nor
    /// a permanent ([CR#114.5]).
    Card,
    /// A copy of a card ([CR#109.1]) with no card of its own ([CR#707.10]):
    /// what a copy effect creating a card copy produces ([CR#707.12]). A copy
    /// of a spell is itself a spell ([CR#112.1a]), so a card copy on the
    /// stack is `Spell` as well as `CopyOfACard`.
    CopyOfACard,
    /// A marker representing an object that has abilities but usually no
    /// other characteristics ([CR#114.1]).
    Emblem,
    /// A card or token on the battlefield ([CR#110.1]).
    Permanent,
    /// A card on the stack ([CR#112.1]); a copy of a spell is itself a spell
    /// even with no card ([CR#112.1a,707.10]).
    Spell,
    /// A marker representing a permanent that is not represented by a card
    /// ([CR#111.1]) — not a card ([CR#108.2b]).
    Token,
}

/// The Entity domain a predicate region's candidate ranges over — the typed
/// provenance rider on [`Provenance::Candidate(Domain::Entity)`](crate::Provenance::Candidate(Domain::Entity))
/// (ADR law 2). It fixes what the engine enumerates before any predicate runs:
/// `Player` enumerates the players, `Object` the objects, `Entity` both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Domain {
    Player,
    Object,
    /// Either — the widest domain, for a candidate a card describes as "any
    /// target" or leaves unrestricted.
    Entity,
}

impl Domain {
    /// Whether a candidate of `class` can appear in this domain.
    #[must_use]
    pub fn admits(self, class: EntityClass) -> bool {
        match self {
            Domain::Entity => true,
            Domain::Player => class == EntityClass::Player,
            Domain::Object => class == EntityClass::Object,
        }
    }

    /// The narrower of two domains, or [`Domain::Entity`] when they disagree
    /// (nothing narrower is sound for the pair).
    #[must_use]
    pub fn meet(self, other: Domain) -> Domain {
        match (self, other) {
            (Domain::Entity, d) | (d, Domain::Entity) => d,
            (a, b) if a == b => a,
            _ => Domain::Entity,
        }
    }
}

/// Which COLLECTION a group-valued query ranges over — the collection-level
/// twin of [`Domain`]. A [`crate::Selection`] names one, and the two never
/// mix: [CR#700.3b] "each object in a pile is still an individual object. The
/// pile is not an object", so a pile is a labeled group in its own domain and
/// not an Entity group. [`crate::Kind`] is the register shape each answers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum CollectionDomain {
    /// A group of Entities, each an individual member ([CR#109.1,102.1]) —
    /// [`crate::Kind::Entities`].
    Entities,
    /// One temporary pile ([CR#700.3a,700.3b]) — [`crate::Kind::Pile`].
    Pile,
}

impl CollectionDomain {
    /// The register shape a selection in this domain reads.
    #[must_use]
    pub fn register_kind(self) -> crate::Kind {
        match self {
            CollectionDomain::Entities => crate::Kind::Entities,
            CollectionDomain::Pile => crate::Kind::Pile,
        }
    }
}

/// Characteristic atoms ([CR#109.3]): facts printed on or defined for the
/// object. `Type`/`Subtype` carry the RESOLVED def (`Arc<TypeDef>` /
/// `Arc<Subtype>`) supplied by semantic lowering. Open, plugin-declared
/// types/subtypes stay filterable — the def is data, not a closed enum.
/// Matching keys on `.name`.
///
/// Equality is by name: [`TypeDef`](crate::TypeDef)/[`Subtype`](crate::Subtype)
/// compare by their `name` (semantic lowering resolves each name to its plugin
/// declaration), so the derived `PartialEq`/`Hash` here are name-based for
/// `Type`/`Subtype` for free, and a registry-less [`Predicate::r#type`] helper
/// compares equal to a full parse-built filter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum CharacteristicPredicate {
    Type(crate::TypeRef),
    Subtype(crate::SubtypeRef),
    Supertype(Supertype),
    /// The object is the given color ([CR#105.2,202.2]).
    ColorIs(Color),
    /// The object has the given name ([CR#201]).
    Named(Ident),
    /// The object has the card name stored in this region's symbol register.
    NamedReg(crate::RefId),
    /// A printed/defined stat compares as stated, e.g.
    /// `Stat(Power, AtLeast, 3)` — mana value via `Stat(ManaValue, …)`
    /// ([CR#208,202.3]).
    Stat(Stat, Cmp, Count),
    /// Two or more colors ([CR#105.2b]).
    Multicolored,
    /// No colors ([CR#105.2c] — colorless is not a color).
    Colorless,
    /// The object HAS the named keyword ability ([CR#702]) — a bare-ident
    /// keyword reference (`Has(Flying)`), matching by NAME (a parameterized
    /// keyword matches regardless of its arguments — card text says "has
    /// ward", never "has ward {2}").
    Has(crate::KeywordRef),
}

/// State atoms: where the object is and what's on it — not
/// characteristics ([CR#110.5a,122.1]).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum StatePredicate {
    InZone(Zone),
    /// The object's status ([CR#110.5]).
    Status(Status),
    /// [CR#302.6]: the object is summoning-sick — it (or its controller) has not
    /// controlled it continuously since their most recent turn began. The
    /// engine reads `GameObject.summoning_sick`; the combatant grant
    /// references it via `Matches(This, SummoningSick)`.
    SummoningSick,
    /// The object has at least one counter of the named kind ([CR#122.1]).
    /// The kind is a bare `CounterRef` (`HasCounter(P1P1Counter)`), not a
    /// string.
    HasCounter(crate::CounterRef),
    /// The object carries the named designation (taxonomy §8) — derived
    /// designations (`Designated(Modified)`) work with no special casing.
    Designated(Ident),
    /// The object is related to a matching object by a named, declared
    /// relation ([CR#607] family). Box because the inner predicate is a
    /// Predicate.
    RelatedBy(Ident, Arc<Predicate>),
    /// Declared as an attacker, still in combat ([CR#508.1a]).
    Attacking,
    /// Declared as a blocker, still in combat ([CR#509.1a]).
    Blocking,
    /// Attacking and unblocked once blockers are declared ([CR#509.1h]).
    Unblocked,
    /// "that targets [desc]" ([CR#115.9b]): a stack object one of whose
    /// chosen targets CURRENTLY matches — departed targets are ignored,
    /// never read through LKI (the one value read with no LKI fallback).
    /// Box breaks the `Predicate` size cycle.
    Targets(Arc<Predicate>),
    /// "with [N] target(s)" ([CR#115.9a]): counts the target instances
    /// chosen at stack-put; "targets only …" counts distinct chosen
    /// targets, then checks current state ([CR#115.9c]).
    TargetCount(crate::CountBound),
    /// The object's tagged optional cost
    /// ([`OptionalCost`](crate::OptionalCost)) was paid — the filter-language
    /// channel of the paid-cost linkage ([CR#702.33d..702.33e,607.2]): "a
    /// kicked spell" = `And([Kind(Spell), WasPaidWith(Kicker)])`.
    WasPaidWith(crate::CostTag),
    /// The object was cast using the named alternative base cost
    /// ([CR#118.9,702.34a]) — the filter-language channel of the alt-cost
    /// linkage; the twin of `WasPaidWith` (optional/additional costs). Idris
    /// `WasCastWith`.
    WasCastWith(crate::CostTag),
    /// The object was PUT into its current zone directly from this zone
    /// (move provenance) — the move-provenance twin of `WasCastFrom`;
    /// engine-held, turn-scoped (like `DamagedBy`). "milled" =
    /// `And([InZone(Graveyard), WasPutFrom(Library)])` ([CR#701.17a]);
    /// "discarded" = `WasPutFrom(Hand)` ([CR#701.9a]).
    WasPutFrom(Zone),
    /// The candidate was dealt damage by a source matching the inner
    /// predicate — an EXISTENTIAL over the candidate's marked damage
    /// ([CR#120.3]), read against each mark's DEAL-TIME abilities: the source
    /// may since have lost the ability or left the battlefield, so the mark
    /// carries what it had when it dealt the damage ([CR#702.2c]).
    ///
    /// [CR#120.1] settles why this is a predicate of the DAMAGED object and
    /// not a reference: "an object that deals damage is the source of that
    /// damage" — being a source is a contextual relation between two objects,
    /// so a set of them is never one Entity a [`Reference`] can denote. The
    /// lethal-damage SBA's deathtouch clause ([CR#704.5h]) is
    /// `Matches(This, WasDealtDamageBy(Has(Deathtouch)))`.
    ///
    /// The relatum is an object ([CR#120.1] — only objects deal damage), so
    /// the inner predicate runs in the Object domain. Boxed like its
    /// `Targets`/`RelatedBy` peers to break the `Predicate` size cycle.
    WasDealtDamageBy(Arc<Predicate>),
}

/// Structural relations the engine owns. Relations are
/// implicitly existential: `ControlledBy(IsOpponent-shaped)` means "whose
/// controller matches".
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum RelationPredicate {
    /// The object's controller matches ([CR#109.5]).
    ControlledBy(Arc<Predicate>),
    /// The object is a player who controls a matching object — the inverse
    /// of [`ControlledBy`](RelationPredicate::ControlledBy) ([CR#109.5]).
    /// Zone-agnostic: control spans the battlefield, the stack (spells and
    /// abilities), and the command zone, so the inner filter carries any
    /// zone restriction it needs (e.g. `Controls(And([Permanent, …]))`
    /// for "controls a permanent").
    Controls(Arc<Predicate>),
    /// The object's owner matches ([CR#108.3]).
    Owner(Arc<Predicate>),
    /// The object is an opponent of a matching player ([CR#102.2,102.3]) — a
    /// player NOT on the matching player's team.
    OpponentOf(Arc<Predicate>),
    /// The object is a teammate of a matching player ([CR#102.3,810.1]) —
    /// ANOTHER player on the matching player's team (never that player itself).
    /// PRIMITIVE, not `Not(OpponentOf …)`: in Two-Headed Giant a teammate is
    /// neither you nor an opponent ([CR#810]). "your team" ([CR#102.4]) is
    /// `Or([Ref(You), TeammateOf(Ref(You))])`.
    TeammateOf(Arc<Predicate>),
    /// The object is attached to a matching object ([CR#301.5,303.4]).
    AttachedTo(Arc<Predicate>),
    /// The object has a matching attachment ([CR#301.5,303.4]).
    Attachment(Arc<Predicate>),
}

/// A predicate over game objects, players included. Core RON keeps the
/// `Characteristic`, `State`, and `Relation` compartments explicit.
///
/// Relative position between two objects in an ORDERED zone ([CR#404.2] — a
/// graveyard is kept in a single face-up pile with a fixed order): gates
/// [`Predicate::Adjacent`] (Death Spark's "directly above").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Adjacency {
    Above,
    Below,
}

/// Conjunction is explicit (`And`) — an enum position never carries a
/// bare list. Canonical filters are context-free-correct: state the whole
/// predicate even where engine context would make parts redundant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Predicate {
    /// Which Entity the candidate is ([CR#109.1,102.1]) — the Entity-level
    /// half of the classification. `Entity(Player)` is the "a player" filter;
    /// `Entity(Object)` is "an object". Nonexclusive with nothing: exactly one
    /// of the two holds of any candidate.
    Entity(EntityClass),
    /// The candidate is in the named CR object class ([CR#109.1]) — the
    /// Object-level half, one independently testable class per atom.
    /// Conjunction is `And`: a card on the stack is
    /// `And([Class(Card), Class(Spell)])`, a battlefield token is
    /// `And([Class(Token), Class(Permanent)])`.
    Class(ObjectClass),
    Characteristic(CharacteristicPredicate),
    State(StatePredicate),
    Relation(RelationPredicate),
    Ref(Reference),
    /// The candidate is directly Above/Below `r` in an ORDERED zone, nothing
    /// between ([CR#404.2]) — Death Spark's "a creature card directly above
    /// it" (a graveyard). Object-relative, unlike an end-relative `Anchor`:
    /// both objects must occupy the SAME ordered zone (implicit — nothing
    /// requires it explicitly since the position comparison only holds
    /// within one zone's own order).
    Adjacent(Adjacency, Reference),
    /// A player whose attribute compares to the bound ([CR#107.1] numbers) —
    /// the player-scope twin of `Characteristic(Stat(..))`: "a player has 13
    /// or less life" = `PlayerStatCmp(Life, AtMost, 13)`. Mirrors the Idris
    /// `PlayerStatCmp : PlayerAttr -> Cmp -> Count -> Predicate APlayer`.
    PlayerStatCmp(PlayerAttr, Cmp, Count),
    /// Lifts a quality to a stack ABILITY's source ([CR#702.11d] "abilities
    /// … from [quality] sources"): matches an activated/triggered ability on
    /// the stack whose SOURCE — the object that generated it ([CR#113.7]) —
    /// matches the inner filter. Strict to abilities by construction: a
    /// spell carries its qualities itself, so "red spells or abilities from
    /// red sources" is `Or([And([Class(Spell), ColorIs(Red)]),
    /// FromSource(ColorIs(Red))])`. Boxed like the other one-child atoms.
    FromSource(Arc<Predicate>),
    And(Arc<[Predicate]>),
    Or(Arc<[Predicate]>),
    Not(Arc<Predicate>),
    /// The candidate matches iff a [`Condition`] holds with `It` bound to
    /// it — the bridge that lets a per-object filter slot reach the whole
    /// condition language ([CR#603.4] predicates) against the object being
    /// matched. The region's candidate register inside the condition reads
    /// that candidate; `Ref(This)`/`Ref(You)` still anchor to the carrier. Boxed to break the `Predicate` → `Condition` → `Predicate` size
    /// cycle. The one candidate-relative escape hatch: "shares a color with ~",
    /// "has the same name as ~", etc., expressed as
    /// `Where(SharesColor(It, This))` and kin.
    Where(Arc<crate::Region<Condition>>),
    /// Matches every object — the bare-Predicate default for event participant
    /// slots (an `EventFilter` master form's `who`/`what`).
    Any,
}

impl Predicate {
    /// The match-anything filter, as a serde `default` for fields like an
    /// `EventFilter` master form's `who`/`what`.
    #[must_use]
    pub fn any() -> Predicate {
        Predicate::Any
    }

    /// Matches objects of a single card type ([CR#109.3]) — the flat
    /// `Type(t)` atom, spelled without its `Characteristic` compartment.
    #[must_use]
    pub fn r#type(t: Type) -> Predicate {
        Predicate::Characteristic(CharacteristicPredicate::Type(crate::TypeRef(Arc::new(
            t.def(),
        ))))
    }

    /// Matches creatures — [`Predicate::r#type`] for [`Type::Creature`], the
    /// most common typed filter across the card base.
    #[must_use]
    pub fn creature() -> Predicate {
        Predicate::r#type(Type::Creature)
    }

    /// Matches a player ([CR#102.1]) — the Entity-level class, spelled
    /// without its compartment.
    #[must_use]
    pub fn player() -> Predicate {
        Predicate::Entity(EntityClass::Player)
    }

    /// The narrowest Entity domain this predicate's own subject can inhabit.
    /// `Entity` means "either" — the atom constrains nothing at the Entity
    /// boundary. Lowering declares a candidate region's domain from this, and
    /// [`crate::validate`] refuses a predicate whose subject cannot inhabit
    /// the declared domain.
    #[must_use]
    pub fn subject_domain(&self) -> Domain {
        match self {
            Predicate::Entity(EntityClass::Player) | Predicate::PlayerStatCmp(..) => Domain::Player,
            Predicate::Entity(EntityClass::Object)
            | Predicate::Class(_)
            | Predicate::Characteristic(_)
            | Predicate::State(_)
            | Predicate::Adjacent(..)
            | Predicate::FromSource(_) => Domain::Object,
            Predicate::Relation(relation) => relation.subject_domain(),
            Predicate::And(parts) => parts
                .iter()
                .fold(Domain::Entity, |acc, part| acc.meet(part.subject_domain())),
            // A disjunct that admits either side widens the whole disjunction.
            Predicate::Or(parts) => {
                let mut parts = parts.iter().map(Predicate::subject_domain);
                match parts.next() {
                    None => Domain::Entity,
                    Some(first) => {
                        if parts.all(|d| d == first) {
                            first
                        } else {
                            Domain::Entity
                        }
                    }
                }
            }
            // "the candidate IS r", so the candidate's domain is r's own.
            Predicate::Ref(reference) => reference.referent_domain(),
            Predicate::Not(_) | Predicate::Where(_) | Predicate::Any => Domain::Entity,
        }
    }

    /// Whether this filter is exactly the self-reference (`Ref(This)`) — the
    /// "~ itself" predicate rendering and the replacement layer test for.
    #[must_use]
    pub fn is_this(&self) -> bool {
        matches!(self, Predicate::Ref(Reference::Reg(crate::RefId(0))))
    }
}

impl RelationPredicate {
    /// The Entity domain of the relation's SUBJECT — the candidate the
    /// relation is asserted of ([CR#109.5,108.3,301.5]).
    #[must_use]
    pub fn subject_domain(&self) -> Domain {
        match self {
            RelationPredicate::ControlledBy(_)
            | RelationPredicate::Owner(_)
            | RelationPredicate::AttachedTo(_) => Domain::Object,
            // [CR#303.4b]: an Aura enchants "that object or player", so the
            // HOST side of the attachment relation admits either Entity.
            RelationPredicate::Attachment(_) => Domain::Entity,
            RelationPredicate::Controls(_)
            | RelationPredicate::OpponentOf(_)
            | RelationPredicate::TeammateOf(_) => Domain::Player,
        }
    }

    /// The related filter and the Entity domain ITS subject ranges over.
    #[must_use]
    pub fn relatum(&self) -> (&Predicate, Domain) {
        match self {
            RelationPredicate::ControlledBy(p)
            | RelationPredicate::Owner(p)
            | RelationPredicate::OpponentOf(p)
            | RelationPredicate::TeammateOf(p) => (p, Domain::Player),
            // The host an attachment names is an object or a player
            // ([CR#303.4b]); an attachment itself is always an object
            // ([CR#301.5,303.4]).
            RelationPredicate::AttachedTo(p) => (p, Domain::Entity),
            RelationPredicate::Controls(p) | RelationPredicate::Attachment(p) => {
                (p, Domain::Object)
            }
        }
    }
}

impl Normalize for RelationPredicate {
    /// Recurse into the related-object filter each relation carries.
    fn normalize(self) -> Self {
        use RelationPredicate as Rp;
        match self {
            Rp::ControlledBy(f) => Rp::ControlledBy(f.normalize()),
            Rp::Controls(f) => Rp::Controls(f.normalize()),
            Rp::Owner(f) => Rp::Owner(f.normalize()),
            Rp::OpponentOf(f) => Rp::OpponentOf(f.normalize()),
            Rp::TeammateOf(f) => Rp::TeammateOf(f.normalize()),
            Rp::AttachedTo(f) => Rp::AttachedTo(f.normalize()),
            Rp::Attachment(f) => Rp::Attachment(f.normalize()),
        }
    }
}

impl Normalize for StatePredicate {
    /// Recurse into the inner filter the relation/target state atoms carry;
    /// every other state atom is a leaf for normalization.
    fn normalize(self) -> Self {
        use StatePredicate as Sp;
        match self {
            Sp::RelatedBy(rel, f) => Sp::RelatedBy(rel, f.normalize()),
            Sp::Targets(f) => Sp::Targets(f.normalize()),
            Sp::WasDealtDamageBy(f) => Sp::WasDealtDamageBy(f.normalize()),
            other => other,
        }
    }
}

impl Normalize for Predicate {
    /// Normalize a predicate (bottom-up): recurse into child filters, then
    /// collapse the boolean combinators. `And`/`Or` are flattened by
    /// associativity (a nested `And` inside an `And` splices in — same for
    /// `Or`) and a singleton `And([x])`/`Or([x])` collapses to `x`.
    /// Both rewrites preserve meaning: conjunction/disjunction are associative,
    /// and a one-element conjunction/disjunction is its element.
    ///
    /// Out of scope for this pass (demo identities only): `Not(Not x) → x`,
    /// `Any`-absorption, dedup/sort. `Where`'s inner [`Condition`] is left as
    /// authored (no `Condition` normalization yet).
    fn normalize(self) -> Self {
        match self {
            // Recurse into compartments that hold child filters.
            Predicate::Relation(r) => Predicate::Relation(r.normalize()),
            Predicate::State(s) => Predicate::State(s.normalize()),
            Predicate::Not(inner) => Predicate::Not(inner.normalize()),
            Predicate::FromSource(inner) => Predicate::FromSource(inner.normalize()),

            Predicate::And(children) => {
                let mut flat: Vec<Predicate> = Vec::with_capacity(children.len());
                for child in children.iter().cloned() {
                    match child.normalize() {
                        // Associativity: splice a nested And in.
                        Predicate::And(inner) => flat.extend(inner.iter().cloned()),
                        other => flat.push(other),
                    }
                }
                // Singleton collapse: And([x]) → x.
                if flat.len() == 1 {
                    flat.pop().expect("len checked")
                } else {
                    Predicate::And(flat.into())
                }
            }
            Predicate::Or(children) => {
                let mut flat: Vec<Predicate> = Vec::with_capacity(children.len());
                for child in children.iter().cloned() {
                    match child.normalize() {
                        Predicate::Or(inner) => flat.extend(inner.iter().cloned()),
                        other => flat.push(other),
                    }
                }
                if flat.len() == 1 {
                    flat.pop().expect("len checked")
                } else {
                    Predicate::Or(flat.into())
                }
            }

            // Leaves (no in-scope child filter to recurse / no redundancy).
            // `Where` carries a Condition, not normalized in this pass.
            other => other,
        }
    }
}
