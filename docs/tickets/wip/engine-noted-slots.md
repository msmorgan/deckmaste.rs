---
needs: []
design: true
---
Build the P0.W5 noted-slot store and close the two P0.W4 seams that read it:
`PlayerAction::ChooseAndNote(..)` (`resolve.rs` `todo!("P0.W4: choose-and-note
(slot store is P0.W5)")`) and `Count::Noted(key)` (`resolve.rs` `todo!("P0.W4:
noted read …")`).

The noted surface is wider than the two seams: `ChooseAndNote(Ident,
NotedKind)` (action.rs), `Count::Noted(Ident)` (count.rs),
`Selection::AmongNoted(Ident, Quantity)` (selection.rs), and the
Whims-of-the-Fates pile source `ChoosePile(from: Noted(note, of))`
(effect.rs). The store must serve whichever readers this ticket wires; the
rest stay loud per-site.

Design boundary to settle before building: the relation to
`engine-linked-abilities` (`Reference::Linked(Ident)`, the [CR#607]
linked-ability information store). Notes are choice anaphora ("the chosen
color"); linked info is what an earlier linked ability did or affected
(exile-then-return). One store or two, and which ticket owns the shared
machinery, is the design dialogue.

Scope of note lifetime matters: per-resolution ("choose a color" read later
in the same effect), per-object ("as ~ enters, choose a color" read by its
other abilities while it remains), and game-scope designations are already
separate machinery (`state.designations`).
