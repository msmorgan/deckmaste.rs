use std::sync::Arc;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::Reference;

    fn read(source: &str) -> Reference {
        crate::ron::options().from_str(source).unwrap()
    }

    #[test]
    fn attach_host_of_round_trips() {
        let v = Reference::AttachHostOf(Arc::new(Reference::This));
        let w = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&w), v);
    }

    #[test]
    fn opponent_round_trips() {
        let v = Reference::Opponent;
        let w = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&w), v);
    }

    /// The event roles read as bare variant names and round-trip — the
    /// object/patient/actor split plus the first-class defending player,
    /// mirroring the Idris `EventObject`/`EventPatient`/`EventActor`
    /// ([CR#603.2e,608.2k,506.2]).
    #[test]
    fn event_roles_round_trip() {
        for (source, value) in [
            ("EventObject", Reference::EventObject),
            ("EventPatient", Reference::EventPatient),
            ("EventActor", Reference::EventActor),
            ("DefendingPlayer", Reference::DefendingPlayer),
        ] {
            assert_eq!(read(source), value, "reads bare: {source}");
            let written = crate::ron::options().to_string(&value).unwrap();
            assert_eq!(read(&written), value, "round-trips: {written}");
        }
    }

    /// `It` — the iteration / projection element ("it") — reads bare and
    /// round-trips (the Idris `Reference.It`).
    #[test]
    fn it_round_trips() {
        assert_eq!(read("It"), Reference::It);
        let written = crate::ron::options().to_string(&Reference::It).unwrap();
        assert_eq!(read(&written), Reference::It);
    }

    /// The sorted anaphor uses the plain-serde enum form of [`Sort`].
    #[test]
    fn sorted_that_round_trips() {
        use crate::Sort;
        use crate::Type;
        assert_eq!(read("That(Card)"), Reference::That(Sort::Card));
        assert_eq!(
            read("That(OfType(Creature))"),
            Reference::That(Sort::OfType(Type::Creature))
        );
        for value in [
            Reference::That(Sort::Card),
            Reference::That(Sort::OfType(Type::Creature)),
            Reference::That(Sort::Player),
        ] {
            let written = crate::ron::options().to_string(&value).unwrap();
            assert_eq!(read(&written), value, "round-trips: {written}");
        }
        assert_eq!(
            crate::ron::options()
                .to_string(&Reference::That(Sort::OfType(Type::Creature)))
                .unwrap(),
            "That(OfType(Creature))",
            "OfType keeps its explicit plain-serde wrapper"
        );
    }

    /// `Target(n)` — the nth announced target — reads and round-trips.
    #[test]
    fn target_index_round_trips() {
        let v = Reference::Target(0);
        assert_eq!(read("Target(0)"), v);
        let written = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&written), v);
    }

    #[test]
    fn coalesce_round_trips() {
        let value = Reference::Coalesce(
            vec![
                Reference::ControllerOf(Arc::new(Reference::Target(0))),
                Reference::Target(0),
            ]
            .into(),
        );
        let written = crate::ron::options().to_string(&value).unwrap();
        assert_eq!(read(&written), value);
    }

    #[test]
    fn single_round_trips() {
        let value = Reference::Single(Arc::new(crate::Selection::SelectAll(
            crate::Predicate::Ref(Reference::You),
        )));
        let written = crate::ron::options().to_string(&value).unwrap();
        assert_eq!(read(&written), value);
    }
}

/// A bound variable: a value fixed earlier (at announce, by the rules of
/// the position, or by a binder) and referenced later. References name
/// *objects*; amounts live in [`crate::Quantity`].
///
/// Players are objects — `You`, `ControllerOf`, `OwnerOf` resolve to
/// player objects.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Reference {
    /// The object this ability is printed on / the resolving spell.
    This,
    /// Demote a selection to its sole element. Empty and ambiguous
    /// selections fail closed. Rust counterpart of Idris `Reference.Single`.
    Single(Arc<crate::Selection>),
    /// The controller of this ability ([CR#109.5]).
    You,
    /// An opponent of `You` ([CR#102.1]); in a two-player game, the other
    /// player. Multiplayer "an opponent" that requires a choice is a future
    /// edge — single-opponent assumption for now.
    Opponent,
    /// The wildcard singular anaphor — "it". Inside a binder it is the
    /// innermost bound element, deterministically: the loop variable of
    /// [`Each`](crate::Each) / [`Distribute`](crate::Distribute), a
    /// [`With`](crate::With) binder's choice, the per-subject candidate a
    /// continuous modifier reads, the candidate a per-object filter
    /// ([`Predicate::Where`](crate::Predicate::Where)) or extremal projection
    /// ([`Selection::Pick`](crate::Selection::Pick)) is currently testing —
    /// subsuming the old `Subject` role (candidate-relative predicates spell
    /// as `SharesColor(It, This)`, "with the same name as ~"). OUTSIDE every
    /// binder it resolves over the antecedent stack — the nearest singular
    /// antecedent of ANY sort (R1), refused when a second compatible
    /// antecedent makes it a guess (the R2 uniqueness gate; both are
    /// soundness invariants proven by the Idris re-emit gate).
    ///
    /// NEVER an announced target: a target is not an anaphor but an indexed
    /// entry in the announce list, read only as
    /// [`Target(n)`](Reference::Target)
    /// / [`Selection::Targets`](crate::Selection::Targets). Lightning Bolt is
    /// `DealDamage(This, 3, Target(0))`. A bare `It` in a targeted body with no
    /// loop, binder, or product antecedent is an unbound read, not a target.
    It,
    /// The nth announced target SLOT read as a single object
    /// ([CR#115.3,601.2c]) — the announce list is an INDEXED channel, and this
    /// plus [`Selection::Targets`](crate::Selection::Targets) are its only
    /// readers. Two same-sort slots (the fight family) are `Target(0)` and
    /// `Target(1)`; no labelling apparatus and no anaphor resolution are
    /// involved, so no ambiguity can arise.
    ///
    /// Reads the slot's first still-live member (a quantity-one slot has
    /// exactly one; a departed member is skipped — partial fizzle,
    /// [CR#608.2b]); out-of-range degrades to the null id (never-crash).
    ///
    /// Does NOT chase [CR#400.7j] moves: an object this resolution moved is a
    /// NEW object and has its own channel — the move product, read as
    /// [`That(Sort)`](Reference::That). "Exile target creature, then return
    /// that card" is `Target(0)` then `That(Card)`.
    Target(usize),
    /// The triggering event's OBJECT — the doer/source ("that card"): the
    /// moving object of a zone change, the source of damage
    /// ([CR#603.2e,608.2k]). Mirrors the Idris `Reference.EventObject`
    /// (`hasObject`); distinct from the
    /// [`EventPatient`](Reference::EventPatient) it acts upon and the
    /// [`EventActor`](Reference::EventActor) player.
    EventObject,
    /// The triggering event's PATIENT — the acted-upon thing (a damage
    /// recipient, a destroyed/countered object), kind-polymorphic: a player OR
    /// an object, fixed by the event ([CR#120.3,608.2k]). Distinct from the
    /// doer ([`EventObject`](Reference::EventObject)), so a two-object event
    /// ("whenever a creature deals damage to another creature") can name source
    /// and recipient separately. For the combat defender prefer
    /// [`DefendingPlayer`](Reference::DefendingPlayer) (always a player).
    EventPatient,
    /// The triggering event's responsible player ("that player") — the event's
    /// ACTOR ([CR#603.2e]). Mirrors the Idris `Reference.EventActor`
    /// (`hasActor`).
    EventActor,
    /// The defending player of an attack/combat — ALWAYS a player, even versus
    /// a planeswalker or battle ([CR#506.2,508.5]). Reachable today otherwise
    /// only as a [`DeciderSpec::DefendingPlayer`](crate::DeciderSpec) — this is
    /// the first-class reference for bodies that name "the defending player"
    /// (landwalk, Annihilator, Afflict).
    DefendingPlayer,
    /// The SORTED singular anaphor — "that card", "that creature", "that
    /// player": the nearest singular antecedent of this [`Sort`](crate::Sort)
    /// on the antecedent stack (R1 nearest-compatible, R2 uniqueness gate).
    /// Antecedents are pushed by producing clauses (the moved/created object,
    /// [CR#400.7]), event bodies ([CR#603.2e]), and binders ([CR#608.2d]) —
    /// NOT by target slots, which are read positionally as
    /// [`Target(n)`](Reference::Target). This is the read for a move's PRODUCT
    /// ("exile target creature … return that **card**"): the returned card is
    /// a new object [CR#400.7], so `Target(0)` cannot name it and `That(Card)`
    /// does. Resolution is dynamic at engine eval time (over
    /// the frame), and its soundness is proven by the Idris re-emit gate. A
    /// many-antecedent is read instead as
    /// [`Selection::They`](crate::Selection::They) /
    /// [`Selection::Them`](crate::Selection::Them).
    That(crate::Sort),
    /// A named role bound by an event pattern or instruction (e.g. the
    /// attacker vs. the blocker).
    Bound(crate::Ident),
    /// Information remembered by a linked ability ([CR#607]): the object
    /// exiled with this, the chosen value, the cost paid.
    Linked(crate::Ident),
    /// The controller of a referenced object ([CR#109.5]).
    ControllerOf(Arc<Reference>),
    /// The first reference in order that resolves to a non-null value.
    Coalesce(Arc<[Reference]>),
    /// The owner of a referenced object ([CR#108.3]).
    OwnerOf(Arc<Reference>),
    /// The permanent that attachment R is attached to — attachment→host
    /// direction ([CR#301.5,303.4]).  Covers Equipment hosts, Aura
    /// enchantees, and Fortification hosts alike.
    AttachHostOf(Arc<Reference>),
    /// The sources of marked damage on the object under evaluation (the
    /// frame's [`This`](Reference::This) — the creature an SBA rule's scope
    /// binds) — a SET-valued binding over each source *as it was when it dealt
    /// the damage* ([CR#120.3,702.2c]): its deal-time abilities are captured
    /// with the mark, since the source may since have lost the ability or left
    /// the battlefield. Meaningful only inside [`Is(Source, …)`](
    /// crate::Condition::Matches), which reads existentially — "was dealt
    /// damage by a source matching F" tests F against each mark's captured
    /// abilities. The lethal-damage SBA's deathtouch clause ([CR#704.5h])
    /// is `Is(Source, Has(Deathtouch))`. No live-object resolution exists
    /// (the binding is over stored deal-time records), so it has no Idris
    /// counterpart.
    Source,
}
