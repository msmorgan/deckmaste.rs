use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;

use crate::Expand;
use crate::Expansion;
use crate::PlayerAttr;
use crate::Predicate;
use crate::Reference;
use crate::SupportsMacros;

/// A measurable characteristic of an object, read by `Count::StatOf`
/// ([CR#109.3,208,209,210]). The open part (mana value, loyalty, defense) is
/// finite; new printed stats are rare and get a variant when one arrives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub enum Countable {
    /// Objects matching a filter — the common count domain. Boxed `Predicate`
    /// to break the `Predicate` → `Count` size cycle.
    Objects(Arc<Predicate>),
    /// Players matching a filter — the cross-player fold domain ([CR#119.1]):
    /// `Aggregate(MaxOf, Projection { of: Players(<all players>), by:
    /// PlayerStatOf(It, Life) })` reads "the highest life total among all
    /// players" (Arbiter of Knollridge). The boxed `Predicate` is the SAME
    /// unified type `Objects` boxes — the v1 kind axis carries a `Player`
    /// member ([`crate::ObjectKind::Player`]) even though a player is not an
    /// object ([CR#109.1]) — boxed for the same size reason.
    /// Idris's `Projectable`/`readableOn` gate what a `Players` source can
    /// feed (`Aggregate`/`CountOf`, never `CountDistinct`/`Pick` — those stay
    /// pinned to `Objects`/`Singleton`); the engine mirrors that by fizzling
    /// the ungated combinations rather than enforcing it at the Rust type
    /// level (never-crash on a semantic-input error).
    Players(Arc<Predicate>),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
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

/// A per-element numeric projection over a set ([CR#107.1]) — the Idris
/// `Project`. Each element of `of` binds [`Reference::It`] while `by` is
/// read. Shared by [`Count::Aggregate`] (the value fold) and
/// [`Selection::Pick`](crate::Selection::Pick) (the extremal element).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Expand, Serialize)]
pub struct Projection {
    /// The folded set. `Countable::Objects` (the per-permanent fold) and
    /// `Countable::Players` (the cross-player fold, [CR#119.1]) are
    /// projectable today — Idris's `Projectable` proof.
    pub of: Countable,
    /// The per-element read, over `Reference::It`.
    pub by: Arc<Count>,
}

/// A scalar magnitude an effect computes at resolution: an amount, never an
/// object (objects are `Reference`s, [CR#107.1,107.3]).
///
/// A literal is a bare numeral: `3`, never `Literal(3)` — the `Literal(…)`
/// wrapper is pure noise and never appears in RON. The derive's `literal`
/// marker gives the variant the `StatValue::Number` treatment: it serializes
/// its inner value bare and deserializes a bare numeral back (a tagged
/// `Literal(3)` still reads, for leniency). Full macro interception still
/// applies at and under `Count` positions.
///
/// Both serde impls are generated by `#[derive(SupportsMacros)]`: `Expanded`
/// writes the invocation back, and `Literal` is the bare-numeral variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SupportsMacros)]
pub enum Count {
    /// The value chosen for {X} as the spell or ability was put on the
    /// stack ([CR#107.3]).
    X,
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
    /// Magnitude anaphora, countable spelling: "that many" — the nearest
    /// Amount antecedent on the antecedent stack, fixed by an
    /// earlier instruction or the enclosing event ([CR#107.3]). The primary
    /// spelling; [`ThatMuch`](Count::ThatMuch) is the uncountable alias
    /// ("that much life"), identical in resolution.
    ThatMany,
    /// Magnitude anaphora, uncountable spelling: "that much" — the same
    /// amount anaphor as [`ThatMany`](Count::ThatMany), rendered "that
    /// much" ([CR#107.3]).
    ThatMuch,
    /// The amount allotted to the current element of a divided distribution
    /// ([CR#601.2d] — "N damage/counters divided as you choose"): the
    /// per-element anaphor read inside an
    /// [`OneShotEffect::Distribute`](crate::OneShotEffect::Distribute) body,
    /// where it stands for that element's share of the divided amount.
    Allotment,
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
    /// A noted number read back from a slot ([CR#607.2] linked values).
    Noted(crate::Ident),
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
    /// A remembered `Count` macro invocation.
    #[macro_ron(expanded)]
    Expanded(Expansion<Count>),
    /// A bare integer literal — `3`, with no `Literal(…)` wrapper in RON.
    #[macro_ron(literal)]
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

    /// Whether this count reads the announced X ([CR#107.3a]) anywhere —
    /// directly (the `X` variant) or nested inside an arithmetic combinator
    /// (`Minus(TargetsOf(This), X)`, "half X", …) or a macro invocation. The
    /// activate-time X-announce trigger asks this of a non-mana cost verb's
    /// count operand to decide whether X must be announced for the activation
    /// even without an `{X}` mana symbol (a loyalty `−X`).
    #[must_use]
    pub fn mentions_x(&self) -> bool {
        match self {
            Self::X => true,
            Count::Min(a, b)
            | Count::Max(a, b)
            | Count::Plus(a, b)
            | Count::Minus(a, b)
            | Count::Times(a, b)
            | Count::Divide(_, a, b)
            | Count::Mod(a, b)
            | Count::Pow(a, b) => a.mentions_x() || b.mentions_x(),
            Count::Half(_, c) => c.mentions_x(),
            Count::Expanded(e) => e.value.mentions_x(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reference::Reference;

    type SemValue = Count;

    fn read(source: &str) -> Count {
        crate::ron::options().from_str(source).unwrap()
    }
    fn write(count: &Count) -> String {
        crate::ron::options().to_string(count).unwrap()
    }

    /// A bare numeral IS core grammar: `3` reads as `Literal(3)`, and
    /// `Literal(3)` writes back bare. `Literal(N)` is read/write noise the
    /// grammar elides — the literal wrapper has no tagged spelling to carry.
    #[test]
    fn literal_reads_and_writes_bare() {
        assert_eq!(read("3"), Count::Literal(3));
        assert_eq!(write(&Count::Literal(3)), "3");
    }

    /// `mentions_x` sees the `X` variant directly and nested inside an arithmetic
    /// combinator ([CR#107.3a]), and says no for an X-free count — the check
    /// the non-mana X-cost announce trigger relies on.
    #[test]
    fn mentions_x_detects_direct_and_nested_x() {
        assert!(SemValue::X.mentions_x(), "bare X");
        assert!(
            read("Minus(TargetsOf(This), X)").mentions_x(),
            "X nested in a Minus operand"
        );
        assert!(
            read("Half(RoundDown, X)").mentions_x(),
            "X nested under a Half combinator"
        );
        assert!(!Count::Literal(3).mentions_x(), "a literal reads no X");
        assert!(
            !read("StatOf(This, Power)").mentions_x(),
            "an X-free count reads no X"
        );
    }

    #[test]
    fn constructors_read_named() {
        assert_eq!(read("X"), SemValue::X);
        assert_eq!(read("ThatMuch"), Count::ThatMuch);
        assert_eq!(read("Allotment"), Count::Allotment);
        assert_eq!(write(&Count::Allotment), "Allotment");
        assert_eq!(
            read("StatOf(This, Power)"),
            Count::StatOf(Reference::This, Stat::Power),
        );
    }

    /// The 2-field tuple variant pins the derived `Serialize` round-trip —
    /// serialize → read is identity (the highest-risk arm).
    #[test]
    fn stat_of_round_trips() {
        let value = Count::StatOf(Reference::This, Stat::Power);
        let written = crate::ron::options().to_string(&value).unwrap();
        assert_eq!(read(&written), value);
    }

    /// `CountOf(Countable)` — the `Objects` wrapper is spelled explicitly, and
    /// a `ManaSymbols` source round-trips ([CR#700.5] devotion).
    #[test]
    fn count_of_objects_and_mana_symbols_round_trip() {
        use crate::Color;
        let mk = Count::CountOf(Countable::ManaSymbols(
            Arc::new(Reference::It),
            crate::SymbolPred::CountsAs(Color::Green),
        ));
        assert_eq!(read(&write(&mk)), mk);
        // Objects wrapper is spelled explicitly.
        let obj = read(r"CountOf(Objects(And([Supertype(Basic), ControlledBy(Ref(You))])))");
        assert!(matches!(obj, Count::CountOf(Countable::Objects(_))));
        assert_eq!(read(&write(&obj)), obj);
    }

    /// `Countable::Singleton` — the per-object twin of `Objects` — and
    /// `Countable::ManaSpentMatching` — the filtered mana-spent domain
    /// ([CR#107.4]) — round-trip under `CountDistinct`/`CountOf`.
    #[test]
    fn singleton_and_mana_spent_matching_round_trip() {
        use crate::Color;

        let singleton = Count::CountDistinct(
            Characteristic::Types,
            Countable::Singleton(Arc::new(Reference::This)),
        );
        assert_eq!(read("CountDistinct(Types, Singleton(This))"), singleton,);
        assert_eq!(read(&write(&singleton)), singleton);

        let mana_spent = Count::CountOf(Countable::ManaSpentMatching(
            Arc::new(Reference::This),
            crate::SymbolPred::CountsAs(Color::White),
        ));
        assert_eq!(
            read("CountOf(ManaSpentMatching(This, CountsAs(White)))"),
            mana_spent,
        );
        assert_eq!(read(&write(&mana_spent)), mana_spent);
    }

    /// `TargetsOf(Reference)` — the announced-as-target count ([CR#115.9a])
    /// — parses named and round-trips.
    #[test]
    fn targets_of_reads_and_round_trips() {
        assert_eq!(read("TargetsOf(This)"), Count::TargetsOf(Reference::This));
        let value = Count::TargetsOf(Reference::This);
        assert_eq!(read(&write(&value)), value);
    }

    /// `EventCount(EventFilter, Lookback)` parses and round-trips — the
    /// count-valued twin of `Condition::Happened` ([CR#608.2i]).
    #[test]
    fn event_count_round_trips() {
        use crate::EventFilter;
        use crate::Lookback;
        use crate::Predicate;

        let event = EventFilter::Cast {
            who: Predicate::Ref(crate::Reference::You),
            what: Predicate::Any,
        };
        // Parse from RON — `what` defaults to Any.
        let parsed = read("EventCount(Cast(who: Ref(You)), ThisTurn)");
        assert_eq!(
            parsed,
            Count::EventCount(Arc::new(event.clone()), Lookback::ThisTurn),
        );
        // Serialize → read round-trip.
        let written = write(&parsed);
        assert_eq!(read(&written), parsed);
    }

    /// The arithmetic constructors parse from named RON and round-trip — the
    /// value-language parity headline ([CR#107.1]).
    #[test]
    fn arithmetic_reads_and_round_trips() {
        let cases = [
            (
                "Plus(X, 1)",
                Count::Plus(Arc::new(SemValue::X), Arc::new(Count::Literal(1))),
            ),
            (
                "Minus(3, X)",
                Count::Minus(Arc::new(Count::Literal(3)), Arc::new(SemValue::X)),
            ),
            (
                "Times(2, X)",
                Count::Times(Arc::new(Count::Literal(2)), Arc::new(SemValue::X)),
            ),
            (
                "Max(X, 1)",
                Count::Max(Arc::new(SemValue::X), Arc::new(Count::Literal(1))),
            ),
            (
                "Half(RoundUp, StatOf(This, Power))",
                Count::Half(
                    RoundMode::RoundUp,
                    Arc::new(Count::StatOf(Reference::This, Stat::Power)),
                ),
            ),
            (
                "Divide(RoundDown, X, 2)",
                Count::Divide(
                    RoundMode::RoundDown,
                    Arc::new(SemValue::X),
                    Arc::new(Count::Literal(2)),
                ),
            ),
            (
                "Mod(X, 2)",
                Count::Mod(Arc::new(SemValue::X), Arc::new(Count::Literal(2))),
            ),
            (
                "Pow(2, X)",
                Count::Pow(Arc::new(Count::Literal(2)), Arc::new(SemValue::X)),
            ),
        ];
        for (src, want) in cases {
            assert_eq!(read(src), want, "parse failed for {src}");
            assert_eq!(read(&write(&want)), want, "round-trip failed for {src}");
        }
    }

    /// `CountDistinct(Characteristic, Countable)` — the distinct-union count
    /// (Domain / Coven / Tarmogoyf) parses and round-trips.
    #[test]
    fn count_distinct_round_trips() {
        let value = Count::CountDistinct(
            Characteristic::Subtypes,
            Countable::Objects(Arc::new(Predicate::Characteristic(
                crate::CharacteristicPredicate::Supertype(crate::Supertype::Basic),
            ))),
        );
        assert_eq!(read(&write(&value)), value);
        assert!(matches!(
            read("CountDistinct(Power, Objects(Type(name:\"Creature\",permanent:true)))"),
            Count::CountDistinct(Characteristic::Power, Countable::Objects(_))
        ));
    }

    /// `TimesPaid(tag)` — multikicker's per-payment count
    /// ([CR#702.33c,702.33d]) — reads a bare-ident tag and round-trips.
    #[test]
    fn times_paid_reads_and_round_trips() {
        let v = read("TimesPaid(Kicker)");
        assert_eq!(v, Count::TimesPaid(crate::CostTag::from("Kicker")));
        assert_eq!(write(&v), "TimesPaid(Kicker)");
        assert_eq!(read(&write(&v)), v);
    }

    /// `PlayerStatOf(Reference, PlayerAttr)` — a player's numeric attribute
    /// (the player-side twin of `StatOf`) — reads named and round-trips.
    #[test]
    fn player_stat_of_reads_and_round_trips() {
        use crate::PlayerAttr;
        assert_eq!(
            read("PlayerStatOf(You, Life)"),
            Count::PlayerStatOf(Reference::You, PlayerAttr::Life),
        );
        let value = Count::PlayerStatOf(Reference::You, PlayerAttr::Life);
        assert_eq!(read(&write(&value)), value);
    }

    /// `Opponents(Reference)` — the opponent-count read — parses and
    /// round-trips ([CR#102.1]).
    #[test]
    fn opponents_reads_and_round_trips() {
        assert_eq!(read("Opponents(You)"), Count::Opponents(Reference::You));
        let value = Count::Opponents(Reference::You);
        assert_eq!(read(&write(&value)), value);
    }

    /// `ManaAvailable(Reference)` — a player's total floated mana
    /// ([CR#106.4]) — parses named and round-trips.
    #[test]
    fn mana_available_reads_and_round_trips() {
        assert_eq!(
            read("ManaAvailable(You)"),
            Count::ManaAvailable(Reference::You),
        );
        let value = Count::ManaAvailable(Reference::You);
        assert_eq!(read(&write(&value)), value);
    }

    #[test]
    fn mana_available_kind_reads_and_round_trips() {
        use crate::Color;
        assert_eq!(
            read("ManaAvailableKind(You, Green)"),
            Count::ManaAvailableKind(Reference::You, Color::Green.into()),
        );
        let value = Count::ManaAvailableKind(Reference::You, Color::Green.into());
        assert_eq!(read(&write(&value)), value);
    }

    /// `Aggregate(AggregateOp, Projection)` — the fold — round-trips for
    /// devotion (an `Objects` source over `CountOf(ManaSymbols(..))`) and for
    /// every `AggregateOp` over a `StatOf` projection.
    #[test]
    fn aggregate_round_trips() {
        use crate::Color;
        use crate::RelationPredicate;
        use crate::StatePredicate;
        use crate::Zone;

        let devotion_green = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(Arc::new(Predicate::And(
                    vec![
                        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                        Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                            Predicate::Ref(Reference::You),
                        ))),
                    ]
                    .into(),
                ))),
                by: Arc::new(Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::It),
                    crate::SymbolPred::CountsAs(Color::Green),
                ))),
            },
        );
        assert_eq!(read(&write(&devotion_green)), devotion_green);
        for op in ["SumOf", "MinOf", "MaxOf", "AverageOf(RoundUp)"] {
            let src =
                format!("Aggregate({op}, (of: Objects(Supertype(Basic)), by: StatOf(It, Power)))");
            assert_eq!(read(&write(&read(&src))), read(&src), "round-trip {op}");
        }
    }

    /// `EventSum(EventFilter, Lookback)` parses and round-trips — the
    /// sum-valued twin of `EventCount` for amount-carrying facts
    /// ([CR#608.2i,119.3]).
    #[test]
    fn event_sum_round_trips() {
        use crate::EventFilter;
        use crate::Lookback;
        use crate::Predicate;

        let event = EventFilter::LifeLost {
            who: Predicate::Any,
            amount: None,
        };
        // Parse from RON — `who` defaults to Any.
        let parsed = read("EventSum(LifeLost(), ThisTurn)");
        assert_eq!(
            parsed,
            Count::EventSum(Arc::new(event.clone()), Lookback::ThisTurn),
        );
        // Serialize → read round-trip.
        let written = write(&parsed);
        assert_eq!(read(&written), parsed);
    }
}
