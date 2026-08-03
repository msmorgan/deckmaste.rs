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
