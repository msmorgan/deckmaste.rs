use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::PlayerAttr;
use crate::Predicate;
use crate::Reference;

/// A measurable characteristic of an object, read by `Count::StatOf`
/// ([CR#109.3,208,209,210]). The open part (mana value, loyalty, defense) is
/// finite; new printed stats are rare and get a variant when one arrives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Stat {
    /// [CR#208].
    Power,
    /// [CR#208].
    Toughness,
    /// [CR#202.3].
    ManaValue,
    /// [CR#209] (planeswalker).
    Loyalty,
    /// [CR#210] (battle).
    Defense,
}

/// Which way `Count::Half` rounds a non-integer result ([CR#107.1a]). One
/// rounding vocabulary shared by every halving — "half its power, rounded up"
/// vs "rounded down".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum RoundMode {
    /// Round the half up (the common "rounded up" wording).
    RoundUp,
    /// Round the half down.
    RoundDown,
}

/// A characteristic *axis* read off an object ([CR#109.3]) — the dimension
/// `Count::CountDistinct` collapses to its distinct-value count. Distinct from
/// [`Stat`] (a single numeric reader): this names a whole axis (colors, types,
/// subtypes, …) so "the number of distinct land types" (Domain), "creatures
/// with distinct powers" (Coven), and "card types among graveyards"
/// (Tarmogoyf) all share one constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Characteristic {
    /// Colors ([CR#105.1]).
    Colors,
    /// Card types ([CR#205]).
    Types,
    /// Subtypes ([CR#205.3]) — the whole land-type axis.
    Subtypes,
    /// Basic land types only — Forest, Island, Mountain, Plains, Swamp
    /// ([CR#205.3i]): Domain's axis ("the number of basic land types among
    /// lands you control"). A Gate's `Gate` subtype counts on `Subtypes`
    /// but never here.
    BasicLandTypes,
    /// Supertypes ([CR#205.4]).
    Supertypes,
    /// Power ([CR#208]) — Coven's distinct-power axis.
    Power,
    /// Toughness ([CR#208]).
    Toughness,
    /// Defense ([CR#210]).
    Defense,
    /// Mana cost ([CR#202]).
    ManaCost,
    /// Name ([CR#201]).
    Name,
}

/// The five basic land types ([CR#205.3i]) — the membership set of the
/// [`Characteristic::BasicLandTypes`] axis, shared by every consumer
/// (layer/resolve distinct-key evaluation).
pub const BASIC_LAND_TYPES: [&str; 5] = ["Plains", "Island", "Swamp", "Mountain", "Forest"];

/// The domain a count ranges over ([CR#107.3]) — the Idris `Countable`. Wired
/// today: a set of objects (the common case), a set of players (a
/// cross-player fold, [CR#119.1] — Arbiter of Knollridge/Balance), and the
/// mana symbols in an object's mana cost (devotion). Plain `Events`/
/// unfiltered `ManaSpent` are deferred until a card forces them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Countable {
    /// Objects matching a filter — the common count domain. Boxed `Predicate`
    /// to break the `Predicate` → `Count` size cycle.
    Objects(Arc<crate::Region<Predicate>>),
    /// Players matching a filter — the cross-player fold domain ([CR#119.1]):
    /// `Aggregate(MaxOf, Projection { of: Players(<all players>), by:
    /// PlayerStatOf(It, Life) })` reads "the highest life total among all
    /// players" (Arbiter of Knollridge). The boxed `Predicate` is the SAME
    /// unified type `Objects` boxes — both range over Entities, and a player
    /// is an Entity, not an object ([CR#109.1,102.1]) — boxed for the same
    /// size reason.
    /// Idris's `Projectable`/`readableOn` gate what a `Players` source can
    /// feed (`Aggregate`/`CountOf`, never `CountDistinct`/`Pick` — those stay
    /// pinned to `Objects`/`Singleton`); the engine mirrors that by fizzling
    /// the ungated combinations rather than enforcing it at the Rust type
    /// level (never-crash on a semantic-input error).
    Players(Arc<crate::Region<Predicate>>),
    /// The mana symbols in a referenced object's mana cost, filtered by a
    /// [`SymbolPred`](crate::SymbolPred) ([CR#700.5] devotion) —
    /// `CountOf(ManaSymbols(It, CountsAs(Green)))` counts that object's green
    /// pips. `Reference` is boxed (it is 80 bytes, like its `CounterCount`
    /// peer) so this variant doesn't blow up `Count`/`Predicate`'s size.
    ManaSymbols(Arc<Reference>, crate::SymbolPred),
    /// ONE object treated as a singleton set — the per-object twin of
    /// `Objects`, so [`Count::CountDistinct`] can read a characteristic off a
    /// SINGLE object rather than a filtered many ([CR#105.2]): Embiggen's
    /// "number of card types [this creature] has" =
    /// `CountDistinct(Types, Singleton(This))`. `Reference` is boxed (it is
    /// 80 bytes, like its `ManaSymbols` peer) so this variant doesn't blow up
    /// `Count`/`Countable`'s size.
    Singleton(Arc<Reference>),
    /// Mana SPENT to cast/activate a referenced object, filtered by a
    /// [`SymbolPred`](crate::SymbolPred) ([CR#107.4]: what a mana symbol
    /// counts as) — Adamant's "if at least three white [mana symbols were
    /// spent]". Read via [`Count::CountOf`] (cardinality of matching spent
    /// symbols); the FILTERED twin of the plain mana-spent domain (which is
    /// read via `CountDistinct(Colors, ..)` instead). `Reference` boxed, like
    /// its `ManaSymbols` peer.
    ManaSpentMatching(Arc<Reference>, crate::SymbolPred),
}

/// A fold operator over a projected set ([CR#107.1]) — the Idris `AggregateOp`.
/// The extremal ops (`MinOf`/`MaxOf`) are also the only ones `Selection::Pick`
/// admits (an extremal ELEMENT is well-defined; a sum/average element is not).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum AggregateOp {
    /// Sum of the projected values (devotion, total power).
    SumOf,
    /// Least projected value (0 over the empty set).
    MinOf,
    /// Greatest projected value (0 over the empty set).
    MaxOf,
    /// Mean of the projected values, rounded per [`RoundMode`] (0 over ∅).
    AverageOf(RoundMode),
}

impl Countable {
    /// The Entity domain the counted candidates inhabit
    /// ([CR#109.1,102.1]) — the count-side twin of
    /// [`Selection::element_domain`](crate::Selection::element_domain).
    #[must_use]
    pub fn element_domain(&self) -> crate::Domain {
        match self {
            Countable::Objects(region) => region.candidate_domain(),
            // [CR#102.1]: the cross-player fold ranges over the people in
            // the game and nothing else.
            Countable::Players(_) => crate::Domain::Player,
            Countable::Singleton(reference) => reference.referent_domain(),
            // Mana symbols are not Entities; the domain question does not
            // arise, so the widest answer is the honest one.
            Countable::ManaSymbols(..) | Countable::ManaSpentMatching(..) => crate::Domain::Entity,
        }
    }
}

/// A per-element numeric projection over a set ([CR#107.1]) — the Idris
/// `Project`. Each element of `of` binds the projection region's candidate register
/// while `by` is read. Shared by [`Count::Aggregate`] (the value fold) and
/// [`Selection::Pick`](crate::Selection::Pick) (the extremal element).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Projection {
    /// The folded set. `Countable::Objects` (the per-permanent fold) and
    /// `Countable::Players` (the cross-player fold, [CR#119.1]) are
    /// projectable today — Idris's `Projectable` proof.
    pub of: Countable,
    /// The per-element read, over the projection region's candidate register.
    pub by: Arc<crate::Region<Count>>,
}

/// A scalar magnitude an effect computes at resolution: an amount, never an
/// object (objects are `Reference`s, [CR#107.1,107.3]).
///
/// Core RON uses the explicit `Literal(3)` form; bare numerals are semantic
/// authoring syntax.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Count {
    /// A numeric value stored in the current region's activation record.
    Reg(crate::RefId),
    /// How many objects/mana symbols match a [`Countable`] source
    /// ([CR#107.3], "for each"; [CR#700.5] devotion).
    CountOf(Countable),
    /// The size of the DISTINCT union of a characteristic across a
    /// [`Countable`] source ([CR#107.3]): Domain = `CountDistinct(Subtypes,
    /// Objects(<your lands>))` (distinct land subtypes), Coven =
    /// `CountDistinct(Power, Objects(<your creatures>))` (distinct powers),
    /// Tarmogoyf = `CountDistinct(Types, Objects(<cards in graveyards>))`.
    /// The granularity comes from the source; the axis from the
    /// [`Characteristic`].
    CountDistinct(Characteristic, Countable),
    /// A referenced object's stat ([CR#107.3], "equal to its power").
    StatOf(Reference, Stat),
    /// A referenced player's numeric attribute ([CR#119.1] life,
    /// [CR#402.2] hand size) — the player-side twin of
    /// [`StatOf`](Count::StatOf), mirroring the Idris `PlayerStatOf`. "a
    /// player's life total" = `PlayerStatOf(<player>, Life)`. The
    /// `Reference` resolves to a player proxy; a non-player reference
    /// fizzles to 0 (never-crash). Held unboxed, like its `StatOf` peer
    /// (`Reference` + a `Copy` [`PlayerAttr`]).
    PlayerStatOf(Reference, PlayerAttr),
    /// How many opponents a referenced player has ([CR#102.1]) — "the number
    /// of opponents you have" = `Opponents(You)`. The count of the live
    /// players NOT on the referenced player's team ([CR#102.4,810.1]); the
    /// Idris twin is `CountOf (Players OpponentOf)`. The `Reference` resolves
    /// to a player proxy; a non-player reference fizzles to 0 (never-crash).
    Opponents(Reference),
    /// How many counters of the named kind sit on a referenced object or
    /// player proxy ([CR#122.1]) — "for each +1/+1 counter on ~", and the
    /// magnitude a counter's own conferred effect scales by (a +1/+1 counter
    /// confers `Power(Up(CounterCount(This, P1P1Counter)))`). The kind is a
    /// bare `CounterRef`, not a string. The `Reference` is boxed (it is 80
    /// bytes) so this variant doesn't make `Count` larger than its `StatOf`
    /// peer.
    CounterCount(Arc<Reference>, crate::CounterRef),
    /// The greatest threshold a referenced object's own abilities watch a
    /// crossing of ([`Condition::Crossed`](crate::Condition::Crossed)) — a
    /// Saga's final chapter number ([CR#714.2d], "the greatest value among
    /// chapter abilities it has"), read generically: a chapter ability's
    /// intervening-if IS a `Crossed` gate ([CR#714.2b]), so the greatest
    /// entry across the object's `Crossed` thresholds is that number. Read
    /// off the DERIVED abilities, so a permanent that lost them ([CR#613.1f])
    /// reads 0 — [CR#714.2d]'s "if a Saga somehow has no chapter abilities,
    /// its final chapter number is 0". The `Reference` is boxed, like its
    /// `CounterCount` peer.
    GreatestWatchedThreshold(Arc<Reference>),
    /// The smaller of two counts — the +1/+1 vs -1/-1 annihilation removes
    /// `Min(CounterCount(+1/+1), CounterCount(-1/-1))` of each ([CR#704.5q]),
    /// and "the lesser of X and Y" appears across the card base. Boxed to keep
    /// `Count` small.
    Min(Arc<Count>, Arc<Count>),
    /// The greater of two counts ([CR#107.1], "the greater of X and Y") — the
    /// twin of `Min`. Boxed to keep `Count` small.
    Max(Arc<Count>, Arc<Count>),
    /// The sum of two counts ([CR#107.1], "X plus N"). Boxed.
    Plus(Arc<Count>, Arc<Count>),
    /// The difference of two counts, floored at 0 ([CR#107.1b] — a count never
    /// goes negative). Boxed.
    Minus(Arc<Count>, Arc<Count>),
    /// The product of two counts ([CR#107.1], "twice X" = `Times(2, X)`).
    /// Boxed.
    Times(Arc<Count>, Arc<Count>),
    /// Half a count, rounded per [`RoundMode`] ([CR#107.1a], "half its power
    /// rounded up"). Boxed.
    Half(RoundMode, Arc<Count>),
    /// Divide the first count by the second, rounded per [`RoundMode`]
    /// ([CR#107.1a]) — the general twin of `Half`'s dedicated /2 constructor
    /// ("divided by X, rounded down"). Boxed.
    Divide(RoundMode, Arc<Count>, Arc<Count>),
    /// The remainder of the first count divided by the second ([CR#107.1] —
    /// parity checks: "if X is even" reads `Compare(Mod(X, 2), Eq, 0)`).
    /// Boxed.
    Mod(Arc<Count>, Arc<Count>),
    /// The first count raised to the second's power ([CR#107.1] —
    /// exponential-growth effects: "double ~'s power X times" builds a
    /// `Times`/`Minus` delta from `Pow(2, X)`; Mathemagics). Boxed.
    Pow(Arc<Count>, Arc<Count>),
    /// How many times a referenced object was chosen as a target when it was
    /// put on the stack ([CR#115.9a]) — Strive's "for each target beyond the
    /// first" reads `Minus(TargetsOf(This), 1)`. Unboxed, like its `Damage`/
    /// `Opponents`/`ManaAvailable` peers (a single `Reference` field).
    TargetsOf(Reference),
    /// How many times an event matching the pattern occurred within the
    /// [`Lookback`](crate::Lookback) ([CR#608.2i] history reads) — the
    /// count-valued twin of `Condition::Happened`. The window is a required
    /// field: history counting never gets a silent default. The pattern is
    /// boxed (it is large) to keep `Count` from growing, mirroring
    /// `CountOf(Arc<Predicate>)`.
    EventCount(Arc<crate::EventFilter>, crate::Lookback),
    /// The summed AMOUNT of history facts matching the pattern within the
    /// [`Lookback`](crate::Lookback) ([CR#608.2i,119.3]) — e.g. total life
    /// lost this turn. The match is by the same pattern as `EventCount`;
    /// the magnitude is each matched fact's carried amount. `EventCount`
    /// counts; `EventSum` sums.
    EventSum(Arc<crate::EventFilter>, crate::Lookback),
    /// How many times the tagged optional cost
    /// ([`OptionalCost`](crate::OptionalCost)) was paid for this object —
    /// multikicker's "for each time it was kicked" ([CR#702.33c,702.33d]).
    /// The count-valued twin of `Condition::PaidCost` (a `repeatable`
    /// optional cost may be paid any number of times).
    TimesPaid(crate::CostTag),
    /// Marked damage on a referenced object ([CR#120.3]) — "damage marked on
    /// ~". Used by the lethal-damage SBA: `Compare(Damage(This), AtLeast,
    /// StatOf(This, Toughness))`.
    Damage(Reference),
    /// A referenced player's total unspent (floated) mana — the size of their
    /// mana pool ([CR#106.4]). Not a printed characteristic, so it is a
    /// player/pool source of its own, not a [`StatOf`](Count::StatOf) stat:
    /// it reads the mana currently floated in the pool, the quantity a
    /// data-driven strategy's ramp gate senses ("tap until I can afford the
    /// spell") — `Compare(ManaAvailable(You), Less, <cost>)`. The `Reference`
    /// resolves to a player proxy; a non-player reference fizzles to 0
    /// (never-crash), like its `Opponents`/`PlayerStatOf` peers.
    ManaAvailable(Reference),
    /// The unspent units of exactly one mana kind in a referenced player's
    /// pool. Riders do not change a unit's kind; this is the color-sensitive
    /// pool read needed by mana-production effects whose output depends on
    /// currently floating mana.
    ManaAvailableKind(Reference, crate::ColorOrColorless),
    /// Fold a [`Projection`] to one value per [`AggregateOp`] ([CR#107.1]):
    /// "the total power of creatures you control" = `Aggregate(SumOf, (of:
    /// Objects(<your creatures>), by: StatOf(It, Power)))`; devotion to green
    /// = `Aggregate(SumOf, (of: Objects(<your permanents>), by: CountOf(
    /// ManaSymbols(It, CountsAs(Green)))))` ([CR#700.5]). Poly over the
    /// projected element kind, so a cross-player fold works too ([CR#119.1]):
    /// "the highest life total among all players" (Arbiter of Knollridge) =
    /// `Aggregate(MaxOf, (of: Players(<all players>), by: PlayerStatOf(It,
    /// Life)))`.
    Aggregate(AggregateOp, Projection),
    /// An integer literal, spelled explicitly as `Literal(3)` in core RON.
    Literal(crate::Uint),
}

impl Count {
    /// The literal magnitude if this count is a bare integer, else `None` —
    /// the one type-level question call sites ask of a `Count` (pluralization,
    /// "is this exactly one"). A dynamic count (`X`, `CountOf`, …) has no
    /// static value.
    #[must_use]
    pub fn literal_value(&self) -> Option<crate::Uint> {
        match self {
            Count::Literal(n) => Some(*n),
            _ => None,
        }
    }

    /// Whether this count reads `reference` anywhere, including inside an
    /// arithmetic combinator.
    #[must_use]
    pub fn mentions_register(&self, reference: crate::RefId) -> bool {
        match self {
            Count::Reg(found) => *found == reference,
            Count::Min(a, b)
            | Count::Max(a, b)
            | Count::Plus(a, b)
            | Count::Minus(a, b)
            | Count::Times(a, b)
            | Count::Divide(_, a, b)
            | Count::Mod(a, b)
            | Count::Pow(a, b) => a.mentions_register(reference) || b.mentions_register(reference),
            Count::Half(_, c) => c.mentions_register(reference),
            _ => false,
        }
    }
}
