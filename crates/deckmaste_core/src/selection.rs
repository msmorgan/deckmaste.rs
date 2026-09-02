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
/// iterators and the [`With`](crate::With) [`Binder`](crate::Binder).
///
/// Targeting is NOT here — it lives in [`crate::TargetSpec`], the announce
/// list — because a target has legality recheck and retargeting rules the
/// other choice forms lack ([CR#115]). [`Targets`](Selection::Targets) is the
/// one member that touches it, and only to READ it: an announced slot is
/// named positionally, never resolved as an anaphor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Selection {
    /// All matching objects as one set ("every creature you control") — the
    /// group a distributor ([`Each`](crate::Each) / `StaticEffect::Each`)
    /// iterates. Mirrors Idris `SelectAll : Predicate -> Selection`.
    SelectAll(Predicate),
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
    /// A choice from among a PREVIOUSLY COMPUTED set ("exile two of them",
    /// "…from among them" — the among-restriction, queries.md §2): the
    /// domain is whatever `OneShotEffect::Noting{key, …}` recorded under the
    /// key, not a re-evaluated filter — re-evaluation would be wrong for
    /// "this way" anaphora ([CR#607.2a] linkage).
    AmongNoted(crate::Ident, Quantity),
    /// The top `count` cards of a library, top → down (an ORDERED set —
    /// position is the whole point). `whose` names the library's player; the
    /// default `You` writes bare. Feeds the scry `Each` over the peeked
    /// top-N ([CR#701.22a]).
    TopOfLibrary {
        count: Count,
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        whose: Reference,
    },
    /// The bottom `count` cards of a library, bottom → up (ordered) — the
    /// Idris `BottomOfLibrary`, mirroring
    /// [`TopOfLibrary`](Self::TopOfLibrary). `whose` names the library's
    /// player; the default `You` writes bare.
    BottomOfLibrary {
        count: Count,
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        whose: Reference,
    },
    /// A WHOLE library as one group, top → bottom (ordered) — the whole-zone
    /// term the slice family [`TopOfLibrary`](Self::TopOfLibrary) /
    /// [`BottomOfLibrary`](Self::BottomOfLibrary) lacks. The [`Reference`]
    /// names the library's player.
    ///
    /// Exists because [CR#701.24a] names a library as one of shuffle's two
    /// own objects — "to shuffle **a library** or a face-down pile of cards"
    /// — and a `count`-bearing slice cannot spell "a library". The pile half
    /// rides [`PilesOf`](Self::PilesOf).
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
    TopOfGraveyard {
        count: Count,
        #[serde(default = "ref_you", skip_serializing_if = "ref_is_you")]
        of: Reference,
    },
    /// The nth announced target SLOT read as its full GROUP ([CR#115.3,601.2c])
    /// — the plural twin of [`Reference::Target`](crate::Reference::Target),
    /// and the ONLY plural read of the announce list. Arc Lightning's 1–3
    /// targets are `Targets(0)`. Order-preserved as announced; a member
    /// that has left its announced zone is dropped (partial fizzle,
    /// [CR#608.2b]), so a wholly-departed slot reads as the empty group and
    /// its verb no-ops. Out-of-range degrades to the empty group
    /// (never-crash).
    ///
    /// A target is an INDEXED entry in the announce list, not an anaphor over
    /// the antecedent stack: it is read here and by `Target(n)`, and nowhere
    /// else. `They`/`Them` never reach it.
    ///
    /// Kind-disambiguated from [`Predicate::Targets`](crate::Predicate) and
    /// `Count::TargetsOf` by position — no parse ambiguity.
    Targets(usize),
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
    /// A pure READ of targeting legality, like
    /// [`Targets`](Selection::Targets) — it never announces or re-announces,
    /// and a single-slot spell degenerates to that slot's legal set.
    ///
    /// The [CR#707.10d] for-each-could-target family is COMPOSED from this
    /// rather than built in as a copy mode:
    /// `Each(InChosenOrder(ValidTargetsFor(s), You), CopySpell(You, s,
    /// TargetsThat(It)))`.
    ValidTargetsFor(Reference),
    /// The PLURAL anaphor — "they"/"them": the nearest Many antecedent on
    /// the antecedent stack, any sort (R1 nearest-compatible,
    /// R2 uniqueness gate). Pushed by a many-binder
    /// ([`OneShotEffect::With`](crate::With), [CR#608.2d]) or a
    /// group-producing clause ("create two tokens — **they** gain haste",
    /// [CR#111.2]); order preserved; iterated with [`Each`](crate::Each)
    /// (per-element [`Reference::It`]).
    ///
    /// NOT pushed by a target slot: a plural announced slot is read
    /// positionally as [`Targets(n)`](Selection::Targets). An anaphor names
    /// what a clause produced or bound; a target is named by its index.
    They,
    /// The SORTED plural anaphor — "those tokens", "those cards": the
    /// nearest Many antecedent of this [`Sort`](crate::Sort) (R1/R2, like
    /// [`They`](Selection::They) with the sort constraint of
    /// [`Reference::That`](crate::Reference::That)).
    Them(crate::Sort),
    /// Piles noted earlier by a
    /// [`SeparatePiles`](crate::OneShotEffect::SeparatePiles) with a `note:
    /// ` key, keyed by their divider: `of` names the player whose piles
    /// these are ([CR#700.3a]; the Whims-of-the-Fates per-player nesting).
    PilesOf { note: crate::Ident, of: Reference },
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

/// serde default for [`TopOfLibrary.whose`] — the library belongs to "you"
/// unless the text names another player.
fn ref_you() -> Reference {
    Reference::You
}

/// `skip_serializing_if` predicate for [`TopOfLibrary.whose`]: the default
/// `You` is omitted from RON.
fn ref_is_you(r: &Reference) -> bool {
    matches!(r, Reference::You)
}
