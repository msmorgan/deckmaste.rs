//! `reference` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Reference {
    type Target = deckmaste_core::Reference;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::This => crate::region::source().map_or(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0)),
                deckmaste_core::Reference::Reg,
            ),
            Self::Single(f0) => deckmaste_core::Reference::Single(f0.lower()),
            Self::You => crate::region::controller().map_or(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
                deckmaste_core::Reference::Reg,
            ),
            Self::Opponent => crate::region::controller().map_or(
                deckmaste_core::Reference::OpponentOf(std::sync::Arc::new(
                    deckmaste_core::Reference::Reg(deckmaste_core::RefId(1)),
                )),
                |controller| {
                    deckmaste_core::Reference::OpponentOf(std::sync::Arc::new(
                        deckmaste_core::Reference::Reg(controller),
                    ))
                },
            ),
            Self::It => deckmaste_core::Reference::Reg(
                crate::region::it().expect("unbound `It` during semantic lowering"),
            ),
            Self::Target(index) => crate::region::target(index).map_or(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(
                    6 + u32::try_from(index).expect("target index fits u32"),
                )),
                deckmaste_core::Reference::Reg,
            ),
            Self::EventObject => crate::region::event_object().map_or(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(2)),
                deckmaste_core::Reference::Reg,
            ),
            Self::EventPatient => crate::region::event_patient().map_or(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(3)),
                deckmaste_core::Reference::Reg,
            ),
            Self::EventActor => crate::region::event_actor().map_or(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(4)),
                deckmaste_core::Reference::Reg,
            ),
            Self::DefendingPlayer => crate::region::defending_player().map_or(
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(5)),
                deckmaste_core::Reference::Reg,
            ),
            Self::That(sort) => deckmaste_core::Reference::Reg(
                crate::region::that(sort).expect("unbound sorted anaphor during semantic lowering"),
            ),
            // [CR#607]: a named role and a linked-memory read are register
            // reads or nothing — core has no name-keyed store, so an
            // unresolved name is a lowering error, not a runtime fizzle.
            Self::Bound(name) => deckmaste_core::Reference::Reg(
                crate::region::named(&name)
                    .unwrap_or_else(|| panic!("unbound role `{name}` during semantic lowering")),
            ),
            // [CR#607.1]: a linked read resolves to a register of THIS region —
            // either one the same ability bound, or the card's memory cell
            // declared as a `Provenance::Linked` parameter (ADR law 8). A name
            // no ability on the card writes has no cell to read, and refusing
            // here names the card.
            Self::Linked(name) => deckmaste_core::Reference::Reg(
                crate::region::named(&name)
                    .or_else(|| crate::region::cell_read(&name))
                    .unwrap_or_else(|| {
                        crate::region::refuse(&format!(
                            "linked-memory read `{name}` has no declared cell — no ability on \
                             this card writes it ([CR#607.1])"
                        ));
                        deckmaste_core::RefId(0)
                    }),
            ),
            Self::ControllerOf(f0) => deckmaste_core::Reference::ControllerOf(f0.lower()),
            Self::Coalesce(f0) => deckmaste_core::Reference::Coalesce(f0.lower()),
            Self::OwnerOf(f0) => deckmaste_core::Reference::OwnerOf(f0.lower()),
            Self::AttachHostOf(f0) => deckmaste_core::Reference::AttachHostOf(f0.lower()),
            // [CR#120.1]: "an object that deals damage is the source of
            // that damage" — being a source is a contextual relation over a
            // SET of objects, never one Entity a core `Reference` denotes.
            // Its only meaningful position is the damage-history query, which
            // `Condition`'s lowering rewrites whole (both the explicit
            // `DealtDamageBy(subject, F)` and the legacy `Matches(Source, F)`
            // spelling); a `Source` anywhere else names nothing resolvable.
            Self::Source => {
                crate::region::refuse(
                    "`Source` is the set of an object's marked-damage sources ([CR#120.1]), not \
                     a reference to one object; damage-source history is only meaningful in a \
                     DealtDamageBy condition",
                );
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
            // Invocation provenance does not cross `lower`: the core grammar is
            // a compiled artifact and carries no record of the semantic
            // spelling (spec §12). Prose recovers the semantic term through the
            // provenance index instead. This is the divergence ledger's first
            // non-identity arm family.
            Self::Expanded(f0) => *f0.value.lower(),
        }
    }
}
#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        reason = "a module may need only one assertion, or no helper"
    )]

    use std::assert_matches;

    use crate::Lower;
    use crate::assert_lowers;
    use crate::minimal::*;

    #[test]
    fn lowers_reference_this() {
        assert_matches!(
            deckmaste_semantics::Reference::This.lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
        );
    }

    #[test]
    fn lowers_reference_single() {
        assert_matches!(
            deckmaste_semantics::Reference::Single(std::sync::Arc::new(minimal_selection()))
                .lower(),
            deckmaste_core::Reference::Single(_)
        );
    }

    #[test]
    fn lowers_reference_you() {
        assert_matches!(
            deckmaste_semantics::Reference::You.lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(1))
        );
    }

    #[test]
    fn lowers_reference_opponent() {
        assert_matches!(
            deckmaste_semantics::Reference::Opponent.lower(),
            deckmaste_core::Reference::OpponentOf(_)
        );
    }

    #[test]
    fn lowers_reference_target() {
        assert_matches!(
            deckmaste_semantics::Reference::Target(0).lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(6))
        );
    }

    #[test]
    fn lowers_reference_event_object() {
        assert_matches!(
            deckmaste_semantics::Reference::EventObject.lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(2))
        );
    }

    #[test]
    fn lowers_reference_event_patient() {
        assert_matches!(
            deckmaste_semantics::Reference::EventPatient.lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(3))
        );
    }

    #[test]
    fn lowers_reference_event_actor() {
        assert_matches!(
            deckmaste_semantics::Reference::EventActor.lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(4))
        );
    }

    #[test]
    fn lowers_reference_defending_player() {
        assert_matches!(
            deckmaste_semantics::Reference::DefendingPlayer.lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(5))
        );
    }

    /// A named role is a REGISTER, not a name-keyed store lookup: core has no
    /// `Bound`/`Linked` variant to fall back to, so a role bound earlier in
    /// the region lowers to its register read.
    #[test]
    fn lowers_reference_bound_to_its_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            crate::region::bind_named("X".into(), deckmaste_core::RefId(1));
            deckmaste_semantics::Reference::Bound("X".into()).lower()
        });
        assert_matches!(
            lowered,
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(1))
        );
    }

    /// An unbound role is a LOWERING error, never a runtime fizzle
    /// ([CR#607] — a linked read names a declared cell or nothing).
    #[test]
    #[should_panic(expected = "unbound role")]
    fn unbound_role_is_a_lowering_error() {
        let _ = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            deckmaste_semantics::Reference::Bound("X".into()).lower()
        });
    }

    /// A linked-memory read ([CR#607]) is a register read too — core has no
    /// `Linked` variant and no name-keyed store, so a read with no declared
    /// cell in scope is a lowering error rather than a value that fizzles at
    /// runtime.
    ///
    /// Re-spelled from `lowers_reference_linked`, whose target variant this
    /// stage deletes.
    #[test]
    fn linked_read_without_a_declared_cell_is_a_lowering_error() {
        let error = crate::lower_for_test(|| {
            crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
                deckmaste_semantics::Reference::Linked("X".into()).lower()
            })
        })
        .expect_err("a linked read without a card cell is refused");
        assert_eq!(&*error.card, "Lowering Test");
        assert!(
            error.message.contains("has no declared cell"),
            "the returned diagnostic carries the refusal, got {:?}",
            error.message
        );
    }

    #[test]
    fn lowers_reference_controller_of() {
        assert_matches!(
            deckmaste_semantics::Reference::ControllerOf(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Reference::ControllerOf(_)
        );
    }

    #[test]
    fn lowers_reference_coalesce() {
        assert_matches!(
            deckmaste_semantics::Reference::Coalesce([].into()).lower(),
            deckmaste_core::Reference::Coalesce(_)
        );
    }

    #[test]
    fn lowers_reference_owner_of() {
        assert_matches!(
            deckmaste_semantics::Reference::OwnerOf(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Reference::OwnerOf(_)
        );
    }

    #[test]
    fn lowers_reference_attach_host_of() {
        assert_matches!(
            deckmaste_semantics::Reference::AttachHostOf(std::sync::Arc::new(minimal_reference()))
                .lower(),
            deckmaste_core::Reference::AttachHostOf(_)
        );
    }

    /// Re-spelled from `lowers_reference_source`, whose target variant this
    /// stage deletes: `Source` is a damage relation ([CR#120.1]), so it has
    /// no core `Reference` spelling and lowering refuses it outside
    /// `Matches`. Same subject, new outcome shape.
    #[test]
    fn source_is_not_a_core_reference() {
        let outcome = crate::region::in_card("Source probe", || {
            deckmaste_semantics::Reference::Source.lower()
        });
        let message = outcome.expect_err("a bare `Source` is refused");
        assert!(
            message.contains("[CR#120.1]"),
            "the refusal names the damage-source rule: {message}"
        );
    }

    #[test]
    fn lowers_reference_expanded() {
        assert_matches!(
            deckmaste_semantics::Reference::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_reference())
            })
            .lower(),
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
        );
    }

    /// "It" names the nearest compatible antecedent as a register read.
    /// Re-spelled from `lowers_reference_it`: `Reference::It` left core with
    /// the discourse channel, so the anaphor's image is the register the
    /// antecedent occupies.
    #[test]
    fn lowers_reference_it_to_the_nearest_antecedents_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let product = crate::region::define(deckmaste_core::Kind::Entity);
            crate::region::push_antecedent(
                product.into(),
                deckmaste_core::Kind::Entity,
                crate::region::Cardinality::One,
                None,
                crate::region::Site::ExecutionFrame,
            );
            deckmaste_semantics::Reference::It.lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(3))
        );
    }

    /// A sorted anaphor ("that player") reads the register of the nearest
    /// antecedent whose sort it is compatible with. Re-spelled from
    /// `lowers_reference_that`.
    #[test]
    fn lowers_reference_that_to_the_nearest_sorted_antecedents_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let card = crate::region::define(deckmaste_core::Kind::Entity);
            crate::region::push_antecedent(
                card.into(),
                deckmaste_core::Kind::Entity,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Card),
                crate::region::Site::ExecutionFrame,
            );
            let player = crate::region::define(deckmaste_core::Kind::Entity);
            crate::region::push_antecedent(
                player.into(),
                deckmaste_core::Kind::Entity,
                crate::region::Cardinality::One,
                Some(deckmaste_semantics::Sort::Player),
                crate::region::Site::ExecutionFrame,
            );
            deckmaste_semantics::Reference::That(deckmaste_semantics::Sort::Player).lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Reference::Reg(deckmaste_core::RefId(4)),
            "the sort picks the player register, not the nearer card one"
        );
    }
}
