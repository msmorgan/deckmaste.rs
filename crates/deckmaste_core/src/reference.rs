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
        let v = Reference::AttachHostOf(Arc::new(Reference::Reg(crate::RefId(0))));
        let w = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&w), v);
    }

    #[test]
    fn opponent_of_round_trips() {
        let v = Reference::OpponentOf(Arc::new(Reference::Reg(crate::RefId(1))));
        let w = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&w), v);
    }

    /// Event roles are ordinary fixed-prefix register reads.
    #[test]
    fn event_roles_round_trip() {
        for (source, value) in [
            ("Reg(2)", Reference::Reg(crate::RefId(2))),
            ("Reg(3)", Reference::Reg(crate::RefId(3))),
            ("Reg(4)", Reference::Reg(crate::RefId(4))),
            ("Reg(5)", Reference::Reg(crate::RefId(5))),
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

    /// Announced targets serialize as their fixed-prefix register reads.
    #[test]
    fn target_index_round_trips() {
        let v = Reference::Reg(crate::RefId(6));
        assert_eq!(read("Reg(6)"), v);
        let written = crate::ron::options().to_string(&v).unwrap();
        assert_eq!(read(&written), v);
    }

    #[test]
    fn coalesce_round_trips() {
        let value = Reference::Coalesce(
            vec![
                Reference::ControllerOf(Arc::new(Reference::Reg(crate::RefId(6)))),
                Reference::Reg(crate::RefId(6)),
            ]
            .into(),
        );
        let written = crate::ron::options().to_string(&value).unwrap();
        assert_eq!(read(&written), value);
    }

    #[test]
    fn single_round_trips() {
        let value = Reference::Single(Arc::new(crate::Selection::SelectAll(
            crate::Predicate::Ref(Reference::Reg(crate::RefId(1))),
        )));
        let written = crate::ron::options().to_string(&value).unwrap();
        assert_eq!(read(&written), value);
    }
}

/// A bound variable: a value fixed earlier (at announce, by the rules of
/// the position, or by a binder) and referenced later. References name
/// *objects*; amounts live in [`crate::Quantity`].
///
/// Players are objects: the controller region parameter and the results of
/// `ControllerOf` and `OwnerOf` resolve to player objects.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Reference {
    /// An indexed read from the current region's activation record.
    Reg(crate::RefId),
    /// Demote a selection to its sole element. Empty and ambiguous
    /// selections fail closed. Rust counterpart of Idris `Reference.Single`.
    Single(Arc<crate::Selection>),
    /// An opponent of the player produced by the inner expression.
    OpponentOf(Arc<Reference>),
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
    /// NEVER an announced target: target slots are indexed region parameters.
    /// A bare `It` in a targeted body with no loop, binder, or product
    /// antecedent is an unbound read, not a target.
    It,
    /// The SORTED singular anaphor — "that card", "that creature", "that
    /// player": the nearest singular antecedent of this [`Sort`](crate::Sort)
    /// on the antecedent stack (R1 nearest-compatible, R2 uniqueness gate).
    /// Antecedents are pushed by producing clauses (the moved/created object,
    /// [CR#400.7]), event bodies ([CR#603.2e]), and binders ([CR#608.2d]) —
    /// NOT by target slots, which are region parameters. This is the read for
    /// a move's PRODUCT ("exile target creature … return that **card**"): the
    /// returned card is a new object [CR#400.7], so its original target
    /// register cannot name it and `That(Card)` does. Resolution is dynamic at
    /// engine eval time, and its soundness is proven by the Idris re-emit gate. A
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
    /// frame's source register — the creature an SBA rule's scope binds) — a
    /// SET-valued binding over each source *as it was when it dealt
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
