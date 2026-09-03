use std::sync::Arc;

use crate::AggregateOp;
use crate::Count;
use crate::Predicate;
use crate::Projection;
use crate::Quantity;
use crate::Reference;

/// A resolution-time group or query over objects — the plural/collective
/// shape a combinator binds ([CR#608.2d]). This is the pure group/query type:
/// effect verbs no longer take a `Selection` patient (they take a single
/// [`Reference`]); plurality and choice live in the
/// [`Each`](crate::Each) / [`Distribute`](crate::Distribute)
/// iterators and explicit choice/search instructions.
///
/// Targeting is NOT here — it lives in [`crate::TargetSpec`], the announce
/// list — because a target has legality recheck and retargeting rules the
/// other choice forms lack ([CR#115]). Announced groups are read through
/// region registers, never resolved as anaphors.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Selection {
    /// The full object group stored in a region register.
    Reg(crate::RefId),
    /// All matching objects as one set ("every creature you control") — the
    /// group a distributor ([`Each`](crate::Each) / `StaticEffect::Each`)
    /// iterates. Mirrors Idris `SelectAll : Predicate -> Selection`.
    SelectAll(Arc<crate::Region<Predicate>>),
    /// Several selections combined as ONE group ("each X and each Y") — the
    /// Idris `Union`. Order-preserving concatenation of the member groups; an
    /// object in more than one member appears once (first position wins).
    Union(Vec<Selection>),
    /// The elements of a selection, sequenced by a player's CHOICE — the
    /// ordered-selection combinator for iterations whose order is
    /// game-visible. The second field names the chooser (not a filter).
    ///
    /// Explicit grammar is needed because order otherwise EMERGES from the
    /// underlying group rather than being chosen: a library window is
    /// top → down, an announce slot is announce order. [CR#603.3b]'s
    /// controller-ordering is trigger-specific, so it does not cover
    /// [CR#707.10d]'s "in the order of their controller's choice". Ordinary
    /// [`Each`](crate::Each)es stay order-invisible — wrap only where the
    /// sequence is observable.
    InChosenOrder(Arc<Selection>, Reference),
    /// A random selection of a quantity of matching objects.
    Random(Quantity, Predicate),
    /// The top `count` cards of a library, top → down (an ORDERED set —
    /// position is the whole point). `whose` names the library's player; the
    /// default `You` writes bare. Feeds the scry `Each` over the peeked
    /// top-N ([CR#701.22a]).
    TopOfLibrary { count: Count, whose: Reference },
    /// The bottom `count` cards of a library, bottom → up (ordered) — the
    /// Idris `BottomOfLibrary`, mirroring
    /// [`TopOfLibrary`](Self::TopOfLibrary). `whose` names the library's
    /// player; the default `You` writes bare.
    BottomOfLibrary { count: Count, whose: Reference },
    /// A WHOLE library as one group, top → bottom (ordered) — the whole-zone
    /// term the slice family [`TopOfLibrary`](Self::TopOfLibrary) /
    /// [`BottomOfLibrary`](Self::BottomOfLibrary) lacks. The [`Reference`]
    /// names the library's player.
    ///
    /// Exists because [CR#701.24a] names a library as one of shuffle's two
    /// own objects — "to shuffle **a library** or a face-down pile of cards"
    /// — and a `count`-bearing slice cannot spell "a library". The pile half
    /// rides a pile-valued [`Reg`](Self::Reg).
    ///
    /// No bare default `whose`, unlike the slice family: a whole-library read
    /// always names whose library it is, so there is no dominant filler to
    /// elide.
    LibraryOf(Reference),
    /// The top `count` cards of a graveyard, top → down (an ORDERED set —
    /// [CR#404.2] a graveyard is a single face-up pile in a fixed order).
    /// `of` names the graveyard's player; the default `You` writes bare, like
    /// [`TopOfLibrary`](Self::TopOfLibrary)'s. Graveyard-topped effects
    /// (Volrath's Shapeshifter, Soldevi Digger) name the player they inspect.
    TopOfGraveyard { count: Count, of: Reference },
    /// Everything legal for EVERY target slot of a stack object at once —
    /// [CR#707.10d]'s same-object rule. That rule copies a spell "for each
    /// player or object it could target", requires that "each of its targets
    /// must be the same player or object", and withholds the copy entirely for
    /// one that "isn't a legal target for each instance of the word 'target'".
    /// So the per-slot legal sets are INTERSECTED, never unioned.
    ///
    /// Players are in scope, not just objects: they ride the same player-proxy
    /// representation every other object-valued term reads.
    ///
    /// The [`Reference`] names the stack object whose slots are read; one that
    /// is not a live stack entry reads as the empty group (never-crash).
    ///
    /// A pure READ of targeting legality: it never announces or re-announces,
    /// and a single-slot spell degenerates to that slot's legal set.
    ///
    /// The [CR#707.10d] for-each-could-target family is COMPOSED from this
    /// rather than built in as a copy mode:
    /// `Each(InChosenOrder(ValidTargetsFor(s), You), CopySpell(You, s,
    /// TargetsThat(It)))`.
    ValidTargetsFor(Reference),
    /// The extremal element(s) of a set, ranked by a per-element
    /// [`Projection`] ([CR#107.1]): "the creature with the greatest power" =
    /// `Pick(op: MaxOf, proj: (of: Objects(Type(Creature)), by: StatOf(It,
    /// Power)))`. `op` is gated to the extremal ops (`MinOf`/`MaxOf`); a
    /// non-extremal `op` fizzles to the empty group. The element-twin of
    /// [`Count::Aggregate`] — shares [`Projection`] with it; the projection's
    /// `by` reads each candidate via [`Reference::It`](crate::Reference::It),
    /// and ties yield the whole group (narrowed by the usual single/choice
    /// path downstream).
    Pick { op: AggregateOp, proj: Projection },
}
