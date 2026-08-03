//! `selection` — authored grammar to engine AST.
//!
//! Scaffolded once from the authored grammar's own source, then HAND-OWNED.
//! Arms here are identities while the two grammars mirror each other; any arm
//! that stops being one carries its justification in place (this crate is the
//! divergence ledger — docs/decisions/authoring-spelling-lowering.md §9).

use crate::Lower;

impl Lower for deckmaste_authoring::Selection {
    type Target = deckmaste_core::Selection;
    fn lower(self) -> <Self as Lower>::Target {
        match self {
            Self::SelectAll(f0) => deckmaste_core::Selection::SelectAll(f0.lower()),
            Self::Union(f0) => deckmaste_core::Selection::Union(f0.lower()),
            Self::InChosenOrder(f0, f1) => {
                deckmaste_core::Selection::InChosenOrder(f0.lower(), f1.lower())
            }
            Self::Random(f0, f1) => deckmaste_core::Selection::Random(f0.lower(), f1.lower()),
            Self::AmongNoted(f0, f1) => {
                deckmaste_core::Selection::AmongNoted(f0.lower(), f1.lower())
            }
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
            Self::Targets(f0) => deckmaste_core::Selection::Targets(f0.lower()),
            Self::ValidTargetsFor(f0) => deckmaste_core::Selection::ValidTargetsFor(f0.lower()),
            Self::They => deckmaste_core::Selection::They,
            Self::Them(f0) => deckmaste_core::Selection::Them(f0.lower()),
            Self::PilesOf { note, of } => deckmaste_core::Selection::PilesOf {
                note: note.lower(),
                of: of.lower(),
            },
            Self::Pick { op, proj } => deckmaste_core::Selection::Pick {
                op: op.lower(),
                proj: proj.lower(),
            },
            Self::Expanded(f0) => deckmaste_core::Selection::Expanded(f0.lower()),
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
    use crate::assert_lowers_debug;
    use crate::minimal::*;

    #[test]
    fn lowers_selection_select_all() {
        assert_lowers_debug(deckmaste_authoring::Selection::SelectAll(
            minimal_predicate(),
        ));
        assert_matches!(
            deckmaste_authoring::Selection::SelectAll(minimal_predicate()).lower(),
            deckmaste_core::Selection::SelectAll(..)
        );
    }

    #[test]
    fn lowers_selection_union() {
        assert_lowers_debug(deckmaste_authoring::Selection::Union(Vec::new()));
        assert_matches!(
            deckmaste_authoring::Selection::Union(Vec::new()).lower(),
            deckmaste_core::Selection::Union(..)
        );
    }

    #[test]
    fn lowers_selection_in_chosen_order() {
        assert_lowers_debug(deckmaste_authoring::Selection::InChosenOrder(
            std::sync::Arc::new(minimal_selection()),
            minimal_reference(),
        ));
        assert_matches!(
            deckmaste_authoring::Selection::InChosenOrder(
                std::sync::Arc::new(minimal_selection()),
                minimal_reference()
            )
            .lower(),
            deckmaste_core::Selection::InChosenOrder(..)
        );
    }

    #[test]
    fn lowers_selection_random() {
        assert_lowers_debug(deckmaste_authoring::Selection::Random(
            minimal_quantity(),
            minimal_predicate(),
        ));
        assert_matches!(
            deckmaste_authoring::Selection::Random(minimal_quantity(), minimal_predicate()).lower(),
            deckmaste_core::Selection::Random(..)
        );
    }

    #[test]
    fn lowers_selection_among_noted() {
        assert_lowers_debug(deckmaste_authoring::Selection::AmongNoted(
            "X".into(),
            minimal_quantity(),
        ));
        assert_matches!(
            deckmaste_authoring::Selection::AmongNoted("X".into(), minimal_quantity()).lower(),
            deckmaste_core::Selection::AmongNoted(..)
        );
    }

    #[test]
    fn lowers_selection_top_of_library() {
        assert_lowers_debug(deckmaste_authoring::Selection::TopOfLibrary {
            count: minimal_count(),
            whose: minimal_reference(),
        });
        assert_matches!(
            deckmaste_authoring::Selection::TopOfLibrary {
                count: minimal_count(),
                whose: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::TopOfLibrary { .. }
        );
    }

    #[test]
    fn lowers_selection_bottom_of_library() {
        assert_lowers_debug(deckmaste_authoring::Selection::BottomOfLibrary {
            count: minimal_count(),
            whose: minimal_reference(),
        });
        assert_matches!(
            deckmaste_authoring::Selection::BottomOfLibrary {
                count: minimal_count(),
                whose: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::BottomOfLibrary { .. }
        );
    }

    #[test]
    fn lowers_selection_library_of() {
        assert_lowers_debug(deckmaste_authoring::Selection::LibraryOf(
            minimal_reference(),
        ));
        assert_matches!(
            deckmaste_authoring::Selection::LibraryOf(minimal_reference()).lower(),
            deckmaste_core::Selection::LibraryOf(..)
        );
    }

    #[test]
    fn lowers_selection_top_of_graveyard() {
        assert_lowers_debug(deckmaste_authoring::Selection::TopOfGraveyard {
            count: minimal_count(),
            of: minimal_reference(),
        });
        assert_matches!(
            deckmaste_authoring::Selection::TopOfGraveyard {
                count: minimal_count(),
                of: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::TopOfGraveyard { .. }
        );
    }

    #[test]
    fn lowers_selection_targets() {
        assert_lowers_debug(deckmaste_authoring::Selection::Targets(0));
        assert_matches!(
            deckmaste_authoring::Selection::Targets(0).lower(),
            deckmaste_core::Selection::Targets(..)
        );
    }

    #[test]
    fn lowers_selection_valid_targets_for() {
        assert_lowers_debug(deckmaste_authoring::Selection::ValidTargetsFor(
            minimal_reference(),
        ));
        assert_matches!(
            deckmaste_authoring::Selection::ValidTargetsFor(minimal_reference()).lower(),
            deckmaste_core::Selection::ValidTargetsFor(..)
        );
    }

    #[test]
    fn lowers_selection_they() {
        assert_lowers_debug(deckmaste_authoring::Selection::They);
        assert_matches!(
            deckmaste_authoring::Selection::They.lower(),
            deckmaste_core::Selection::They
        );
    }

    #[test]
    fn lowers_selection_them() {
        assert_lowers_debug(deckmaste_authoring::Selection::Them(minimal_sort()));
        assert_matches!(
            deckmaste_authoring::Selection::Them(minimal_sort()).lower(),
            deckmaste_core::Selection::Them(..)
        );
    }

    #[test]
    fn lowers_selection_piles_of() {
        assert_lowers_debug(deckmaste_authoring::Selection::PilesOf {
            note: "X".into(),
            of: minimal_reference(),
        });
        assert_matches!(
            deckmaste_authoring::Selection::PilesOf {
                note: "X".into(),
                of: minimal_reference()
            }
            .lower(),
            deckmaste_core::Selection::PilesOf { .. }
        );
    }

    #[test]
    fn lowers_selection_pick() {
        assert_lowers_debug(deckmaste_authoring::Selection::Pick {
            op: minimal_aggregate_op(),
            proj: minimal_projection(),
        });
        assert_matches!(
            deckmaste_authoring::Selection::Pick {
                op: minimal_aggregate_op(),
                proj: minimal_projection()
            }
            .lower(),
            deckmaste_core::Selection::Pick { .. }
        );
    }

    #[test]
    fn lowers_selection_expanded() {
        assert_lowers_debug(deckmaste_authoring::Selection::Expanded(
            macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_selection()),
            },
        ));
        assert_matches!(
            deckmaste_authoring::Selection::Expanded(macro_ron::Expansion {
                name: "X".into(),
                args: macro_ron::ExpansionArgs::none(),
                template: None,
                value: Box::new(minimal_selection())
            })
            .lower(),
            deckmaste_core::Selection::Expanded(..)
        );
    }
}
