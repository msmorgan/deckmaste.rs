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
    /// The full ENTITY group stored in a region register — a
    /// [`Kind::Entities`](crate::Kind::Entities) read. A pile-valued register
    /// is a different domain and reads through [`Pile`](Self::Pile).
    Reg(crate::RefId),
    /// The temporary PILE stored in a region register ([CR#700.3a]) — a
    /// [`Kind::Pile`](crate::Kind::Pile) read, the collection-domain twin of
    /// [`Reg`](Self::Reg).
    ///
    /// Separate from `Reg` because a pile is not an Entity group:
    /// [CR#700.3b] — "each object in a pile is still an individual object.
    /// The pile is not an object". A pile is written by a partition
    /// instruction and named as a whole ("those piles", "a pile of your
    /// choice"); its MEMBERS are objects, which is what
    /// [`element_domain`](Self::element_domain) reports.
    Pile(crate::RefId),
    /// All matching objects as one set ("every creature you control") — the
    /// group a distributor ([`Each`](crate::Each) / `StaticSpec::Each`)
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
    /// A random selection of a quantity of matching candidates. The
    /// predicate rides a candidate region like
    /// [`SelectAll`](Self::SelectAll)'s, so the domain it enumerates over is
    /// declared rather than inferred at evaluation ([CR#109.1,102.1] — ADR
    /// law 2).
    Random(Quantity, Arc<crate::Region<Predicate>>),
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
    /// Reg(candidate)))`, where the nested body declares `candidate` as its
    /// loop-element parameter.
    ValidTargetsFor(Reference),
    /// The extremal element(s) of a set, ranked by a per-element
    /// [`Projection`] ([CR#107.1]): "the creature with the greatest power" =
    /// `Pick(op: MaxOf, proj: (of: Objects(Type(Creature)), by: StatOf(It,
    /// Power)))`. `op` is gated to the extremal ops (`MinOf`/`MaxOf`); a
    /// non-extremal `op` fizzles to the empty group. The element-twin of
    /// [`Count::Aggregate`] — shares [`Projection`] with it; the projection's
    /// `by` reads each candidate through its region's declared
    /// [`Provenance::Candidate`](crate::Provenance::Candidate) parameter, and
    /// ties yield the whole group (narrowed by the usual single/choice path
    /// downstream).
    Pick { op: AggregateOp, proj: Projection },
}

impl Selection {
    /// The collection domain this selection ranges over — which register
    /// shape it reads and what kind of group it denotes ([CR#700.3b]).
    #[must_use]
    pub fn collection_domain(&self) -> crate::CollectionDomain {
        match self {
            Selection::Pile(_) => crate::CollectionDomain::Pile,
            _ => crate::CollectionDomain::Entities,
        }
    }

    /// The Entity domain this selection's MEMBERS inhabit
    /// ([CR#109.1,102.1]) — the group twin of
    /// [`Reference::referent_domain`](crate::Reference::referent_domain).
    /// Every constructor names one, so a Player-only query never enumerates
    /// objects and the reverse. `Entity` means "either".
    #[must_use]
    pub fn element_domain(&self) -> crate::Domain {
        match self {
            // The declared candidate domain of the query's own region.
            Selection::SelectAll(region) | Selection::Random(_, region) => {
                region.candidate_domain()
            }
            // Zone slices read cards, and a pile holds objects ([CR#700.3b]).
            Selection::TopOfLibrary { .. }
            | Selection::BottomOfLibrary { .. }
            | Selection::LibraryOf(_)
            | Selection::TopOfGraveyard { .. }
            | Selection::Pile(_) => crate::Domain::Object,
            // Both Entity classes are in scope: [CR#707.10d] reads "each
            // player or object it could target", and a register's members
            // are whatever its declared parameter kind holds, which the
            // selection alone cannot see.
            Selection::ValidTargetsFor(_) | Selection::Reg(_) => crate::Domain::Entity,
            Selection::InChosenOrder(inner, _) => inner.element_domain(),
            Selection::Union(parts) => parts
                .iter()
                .map(Selection::element_domain)
                .reduce(crate::Domain::meet)
                .unwrap_or(crate::Domain::Entity),
            Selection::Pick { proj, .. } => proj.of.element_domain(),
        }
    }
}
