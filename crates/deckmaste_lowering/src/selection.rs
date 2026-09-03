//! `selection` — semantics grammar to engine AST.
//!
//! Scaffolded once, then hand-owned. See the crate docs.

use crate::Lower;

impl Lower for deckmaste_semantics::Selection {
    type Target = deckmaste_core::Selection;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SelectAll(f0) => deckmaste_core::Selection::SelectAll(std::sync::Arc::new(
                crate::region::candidate_region(|| f0.lower()),
            )),
            Self::Union(f0) => deckmaste_core::Selection::Union(f0.lower()),
            Self::InChosenOrder(f0, f1) => {
                deckmaste_core::Selection::InChosenOrder(f0.lower(), f1.lower())
            }
            Self::Random(f0, f1) => deckmaste_core::Selection::Random(f0.lower(), f1.lower()),
            // [CR#607.1]: "them" over a noted group — a register of this
            // region, or the card's memory cell read through the ability's
            // declared `Provenance::Linked` parameter (ADR law 8).
            Self::AmongNoted(name, _) => deckmaste_core::Selection::Reg(
                crate::region::named(&name)
                    .or_else(|| crate::region::cell_read(&name))
                    .unwrap_or_else(|| {
                        crate::region::refuse(&format!(
                            "noted selection `{name}` has no declared cell — no ability on this \
                             card writes it ([CR#607.1])"
                        ))
                    }),
            ),
            Self::TopOfLibrary { count, whose } => deckmaste_core::Selection::TopOfLibrary {
                count: count.lower(),
                whose: whose.lower(),
            },
            Self::BottomOfLibrary { count, whose } => deckmaste_core::Selection::BottomOfLibrary {
                count: count.lower(),
                whose: whose.lower(),
            },
            Self::LibraryOf(f0) => deckmaste_core::Selection::LibraryOf(f0.lower()),
            Self::TopOfGraveyard { count, of } => deckmaste_core::Selection::TopOfGraveyard {
                count: count.lower(),
                of: of.lower(),
            },
            Self::Targets(index) => crate::region::target(index).map_or(
                deckmaste_core::Selection::Reg(deckmaste_core::RefId(
                    6 + u32::try_from(index).expect("target index fits u32"),
                )),
                deckmaste_core::Selection::Reg,
            ),
            Self::ValidTargetsFor(f0) => deckmaste_core::Selection::ValidTargetsFor(f0.lower()),
            Self::They => deckmaste_core::Selection::Reg(
                crate::region::they(None).expect("unbound `They` during semantic lowering"),
            ),
            Self::Them(sort) => deckmaste_core::Selection::Reg(
                crate::region::they(Some(sort)).expect("unbound sorted plural during lowering"),
            ),
            Self::PilesOf { note, of } => deckmaste_core::Selection::PilesOf {
                note: note.lower(),
                of: of.lower(),
            },
            Self::Pick { op, proj } => deckmaste_core::Selection::Pick {
                op: op.lower(),
                proj: proj.lower(),
            },
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
    fn lowers_selection_select_all() {
        assert_matches!(
            deckmaste_semantics::Selection::SelectAll(minimal_predicate()).lower(),
            deckmaste_core::Selection::SelectAll(_)
        );
    }

    #[test]
    fn lowers_selection_union() {
        assert_matches!(
            deckmaste_semantics::Selection::Union(Vec::new()).lower(),
            deckmaste_core::Selection::Union(_)
        );
    }

    #[test]
    fn lowers_selection_in_chosen_order() {
        assert_matches!(
            deckmaste_semantics::Selection::InChosenOrder(
                std::sync::Arc::new(minimal_selection()),
                minimal_reference()
            )
            .lower(),
            deckmaste_core::Selection::InChosenOrder(
                _,
                deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            )
        );
    }

    #[test]
    fn lowers_selection_random() {
        assert_matches!(
            deckmaste_semantics::Selection::Random(minimal_quantity(), minimal_predicate()).lower(),
            deckmaste_core::Selection::Random(
                deckmaste_core::Quantity::Range(None, None),
                deckmaste_core::Predicate::Kind(deckmaste_core::ObjectKind::Ability)
            )
        );
    }

    #[test]
    fn lowers_selection_top_of_library() {
        assert_matches!(
            deckmaste_semantics::Selection::TopOfLibrary {
                count: minimal_count(),
                whose: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::TopOfLibrary {
                count: deckmaste_core::Count::X,
                whose: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_selection_bottom_of_library() {
        assert_matches!(
            deckmaste_semantics::Selection::BottomOfLibrary {
                count: minimal_count(),
                whose: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::BottomOfLibrary {
                count: deckmaste_core::Count::X,
                whose: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_selection_library_of() {
        assert_matches!(
            deckmaste_semantics::Selection::LibraryOf(minimal_reference()).lower(),
            deckmaste_core::Selection::LibraryOf(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_selection_top_of_graveyard() {
        assert_matches!(
            deckmaste_semantics::Selection::TopOfGraveyard {
                count: minimal_count(),
                of: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::TopOfGraveyard {
                count: deckmaste_core::Count::X,
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_selection_targets() {
        assert_matches!(
            deckmaste_semantics::Selection::Targets(0).lower(),
            deckmaste_core::Selection::Reg(deckmaste_core::RefId(6))
        );
    }

    #[test]
    fn lowers_selection_valid_targets_for() {
        assert_matches!(
            deckmaste_semantics::Selection::ValidTargetsFor(minimal_reference()).lower(),
            deckmaste_core::Selection::ValidTargetsFor(deckmaste_core::Reference::Reg(
                deckmaste_core::RefId(0)
            ))
        );
    }

    #[test]
    fn lowers_selection_piles_of() {
        assert_matches!(
            deckmaste_semantics::Selection::PilesOf {
                note: "X".into(),
                of: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::PilesOf {
                note: _,
                of: deckmaste_core::Reference::Reg(deckmaste_core::RefId(0))
            }
        );
    }

    #[test]
    fn lowers_selection_pick() {
        assert_matches!(
            deckmaste_semantics::Selection::Pick {
                op: minimal_aggregate_op(),
                proj: minimal_projection()
            }
            .lower(),
            deckmaste_core::Selection::Pick {
                op: deckmaste_core::AggregateOp::SumOf,
                proj: deckmaste_core::Projection {
                    of: deckmaste_core::Countable::Objects(_),
                    by: _
                }
            }
        );
    }

    #[test]
    fn lowers_selection_expanded() {
        assert_matches!(
            deckmaste_semantics::Selection::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_selection())
            })
            .lower(),
            deckmaste_core::Selection::SelectAll(_)
        );
    }

    /// A plural anaphor ("they") reads the register holding the group.
    /// Re-spelled from `lowers_selection_they`: `Selection::They` left core
    /// with the discourse channel.
    #[test]
    fn lowers_selection_they_to_the_group_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let group = crate::region::define(deckmaste_core::Kind::Objects);
            crate::region::push_antecedent(
                group.into(),
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                None,
                crate::region::Site::Frame,
            );
            deckmaste_semantics::Selection::They.lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Selection::Reg(deckmaste_core::RefId(3))
        );
    }

    /// A sorted plural anaphor ("those players") picks the group register
    /// whose sort it matches. Re-spelled from `lowers_selection_them`.
    #[test]
    fn lowers_selection_them_to_the_sorted_group_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            let cards = crate::region::define(deckmaste_core::Kind::Objects);
            crate::region::push_antecedent(
                cards.into(),
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                Some(deckmaste_semantics::Sort::Card),
                crate::region::Site::Frame,
            );
            let players = crate::region::define(deckmaste_core::Kind::Objects);
            crate::region::push_antecedent(
                players.into(),
                deckmaste_core::Kind::Objects,
                crate::region::Cardinality::Many,
                Some(deckmaste_semantics::Sort::Player),
                crate::region::Site::Frame,
            );
            deckmaste_semantics::Selection::Them(deckmaste_semantics::Sort::Player).lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Selection::Reg(deckmaste_core::RefId(4)),
            "the sort picks the player group, not the nearer card one"
        );
    }

    /// "Among the cards noted this way" reads the register the noting
    /// instruction defined ([CR#607.2a]) — core keeps no name-keyed store.
    /// Re-spelled from `lowers_selection_among_noted`.
    #[test]
    fn lowers_selection_among_noted_to_the_noted_register() {
        let (_, lowered) = crate::region::in_region(crate::region::RegionKind::Spell, 0, || {
            crate::region::bind_named("X".into(), deckmaste_core::RefId(2));
            deckmaste_semantics::Selection::AmongNoted(
                "X".into(),
                deckmaste_semantics::Quantity::one(),
            )
            .lower()
        });
        assert_eq!(
            lowered,
            deckmaste_core::Selection::Reg(deckmaste_core::RefId(2))
        );
    }
}
