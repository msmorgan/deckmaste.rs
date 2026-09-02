---
needs: [core-regions-piles]
---
**Engine: grouped piles have no resolution or read path.**

`Selection::PilesOf { note, of }` reads back the labeled piles a
`SeparatePiles.note:` binder produced ([CR#700.3a] — each affected object goes
into exactly one pile; [CR#700.3b] — a pile is not an object, each card in it
stays individual). `resolve/query.rs` panics at read time, and the seam has
been naming an `engine-piles` owner that did not exist as a ticket. This is
that ticket.

The whole family is unbuilt, not just the read:

- `resolve/effect.rs` has no arm for `OneShotEffect::SeparatePiles` or
  `ChoosePile` — both fall into the catch-all choice seam.
- `PileSource::Noted` has no store. `engine-noted-slots` (done) deliberately
  left this as a boundary, naming "(pile store)" as the missing piece.
- `Action::Shuffle` over a `PilesOf`-shaped face-down pile degrades to a
  silent no-op in `resolve/player_action.rs`.

**This is the engine-resolution half that `core-do-or-die-divide-and-choose`
asked for.** That ticket (planned) scopes the two-pile,
opponent-chooses-and-consequence-applies shape onto a new `DivideAndChoose`
core primitive, and defers here twice over: it carves the general N-pile case
back out for "genuinely N-way cards (Whims of the Fates)", and its blocker 2
recommends "splitting into a `core:` emit/render/RON slice (cleanly doable) vs
a separate engine-resolution ticket". This is that separate ticket. Its
blocker 2 also records the one reuse dead end worth knowing up front:
`ChoiceContinuation::ArrangePiles` is scry-style ordering *within* a known
pile, not partition-and-choose, so it is not a starting point.

Scope: partition `group` into `into.len()` labeled piles per `by`'s divider,
binding each label as a Many antecedent and persisting to a pile store keyed
by `(note, label, divider)` when `note:` is set; a pile-pick decision over
`from` binding the chosen pile and running `then`; the `PilesOf` read; and the
shuffle wiring once a pile has a real object-group representation.

No canon card exercises this today — Whims of the Fates is the design target
once authored, so check the corpus at pickup before sizing.

Effort: **M**.
