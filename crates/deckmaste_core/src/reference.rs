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

    /// The discourse anaphors have no core spelling: `It` and `That(Sort)`
    /// resolve to register reads at lowering, so core RON never parses them.
    /// Re-spelled from `it_round_trips` and `sorted_that_round_trips`, whose
    /// variants this stage deletes.
    #[test]
    fn discourse_anaphors_have_no_core_spelling() {
        for source in ["It", "That(Card)", "That(Player)", "That(OfType(Creature))"] {
            assert!(
                crate::ron::options().from_str::<Reference>(source).is_err(),
                "core has no anaphor spelling for {source}"
            );
        }
    }

    /// An instruction product's register — past the fixed parameter prefix —
    /// reads bare and round-trips. This is the successor spelling of every
    /// retired anaphor: a definition's ordinal, whatever the English that
    /// introduced it.
    #[test]
    fn instruction_product_registers_round_trip() {
        for index in [7_u32, 12, 30] {
            let value = Reference::Reg(crate::RefId(index));
            assert_eq!(
                read(&format!("Reg({index})")),
                value,
                "reads bare: Reg({index})"
            );
            let written = crate::ron::options().to_string(&value).unwrap();
            assert_eq!(read(&written), value, "round-trips: {written}");
        }
    }
}

/// A bound variable: a value fixed earlier (at announce, by the rules of
/// the position, or by a binder) and referenced later.
///
/// A reference is SINGULAR and Entity-valued: every constructor denotes one
/// Object or one Player ([CR#109.1,102.1]) — the controller region parameter
/// and the results of `ControllerOf` and `OwnerOf` resolve to players.
/// Nothing else lives here: a group is a [`crate::Selection`], an amount a
/// [`crate::Count`], a pile a pile-valued register, and a contextual
/// relation over a set (the sources of an object's marked damage,
/// [CR#120.1]) a [`crate::Predicate`] atom. [`Reference::referent_domain`]
/// states each constructor's domain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum Reference {
    /// An indexed read from the current region's activation record.
    Reg(crate::RefId),
    /// Demote a selection to its sole element. Empty and ambiguous
    /// selections fail closed. Rust counterpart of Idris `Reference.Single`.
    Single(Arc<crate::Selection>),
    /// An opponent of the player produced by the inner expression.
    OpponentOf(Arc<Reference>),
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

    /// The Entity domain this reference's referent can inhabit
    /// ([CR#109.1,102.1]). Every constructor names one: the possessor
    /// relations produce a player ([CR#109.5,108.3,102.2]), an Aura's host
    /// either Entity ([CR#303.4b]), a register read whatever its declared
    /// [`Kind`](crate::Kind) holds, and a demoted selection its members'
    /// domain. `Entity` means "either" — the constructor constrains nothing.
    #[must_use]
    #[allow(
        clippy::match_same_arms,
        reason = "an Aura host is Entity-wide by [CR#303.4b]; a register is \
                  Entity only because the reference cannot see its declared \
                  kind — merging the arms would erase that distinction"
    )]
    pub fn referent_domain(&self) -> crate::Domain {
        match self {
            // [CR#109.5,108.3,102.2]: a controller, an owner, and an opponent
            // are all players.
            Reference::ControllerOf(_) | Reference::OwnerOf(_) | Reference::OpponentOf(_) => {
                crate::Domain::Player
            }
            // [CR#303.4b]: "the object OR PLAYER an Aura is attached to" —
            // an Equipment host is a creature ([CR#301.5]), but an Aura
            // enchants either, so the host is Entity-wide.
            Reference::AttachHostOf(_) => crate::Domain::Entity,
            Reference::Single(selection) => selection.element_domain(),
            // The first arm that resolves wins, so the reference can produce
            // any domain its arms can.
            Reference::Coalesce(parts) => parts
                .iter()
                .map(Reference::referent_domain)
                .reduce(crate::Domain::meet)
                .unwrap_or(crate::Domain::Entity),
            // A register's domain is its declared parameter kind's, which the
            // reference alone cannot see.
            Reference::Reg(_) => crate::Domain::Entity,
        }
    }
}
