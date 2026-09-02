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
        let value = Reference::Single(Arc::new(crate::Selection::SelectAll(Arc::new(
            crate::Region::new(
                Arc::from([]),
                crate::Predicate::Ref(Reference::Reg(crate::RefId(0))),
            ),
        ))));
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
    /// A named role bound by an event pattern or instruction (for example,
    /// the attacker versus the blocker). Announced targets and lexical
    /// instruction results use indexed region parameters instead.
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

impl Reference {
    /// Canonical source parameter used by root ability regions.
    #[must_use]
    pub const fn source_parameter() -> Self {
        Self::Reg(crate::RefId(0))
    }

    /// Canonical controller parameter used by root ability regions.
    #[must_use]
    pub const fn controller_parameter() -> Self {
        Self::Reg(crate::RefId(1))
    }
}
