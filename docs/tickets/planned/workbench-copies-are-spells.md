---
needs: []
---
**Drop the copy exclusion from `wordReaches SpellW`: a copy of a spell is
itself a spell [CR#707.10,112.1a].** Fresh workbench review 2026-09-03, F4.

`Words.wordReaches SpellW` carries a `not (isCopyOrigin og)` conjunct, so
`That SpellW` skips copies. Probe P5: after `Copy FromStack …`, `That SpellW
OneOf` and `That CopyW OneOf` are both admitted, but `That SpellW ManyOf` is
**refused** — no plural spell read can ever include a copy, which contradicts
[CR#707.10] ("A copy of a spell is itself a spell") and [CR#112.1a].

The exclusion exists for resolution, not for rules content: it makes `That
SpellW OneOf` after a `Copy` resolve to the original. Three bench sites depend
on it (`Cards/Copy.idr:46,226,256`), plus whichever pins twin them.

Fix: delete the conjunct. Where a card reads the *original* after a copy,
spell it with a stamped read the grammar already has — `ItVerbed "Copy"` for
the copy, an `Other`-style exclusion or `TheVerbed` for the original. This is
the review's own advice: stop growing `Reach`, and let provenance stamps carry
disambiguation while the kind word carries rules content.

Size: S–M.

Done when: `That SpellW ManyOf` after a `Copy` reads both the original and the
copy as a typechecking bench witness; the three `Cards/Copy.idr` sites read
their original through a stamp and still typecheck; `isCopyOrigin` no longer
appears in `wordReaches`; a pin records that the copy is reachable as a spell,
probed non-vacuous; the build is 44/44 with 0 errors and 0 warnings.

The landing record names this as a **deliberate CR alignment**: the grammar
previously diverged from [CR#707.10] and no longer does, so any future
reintroduction of a copy exclusion in a kind word is a regression, not a
convenience. Standard constraints apply, plus the RON-shaped constraint: a
core constructor is admissible only if the RON re-emitter can produce it from
a RON node, and a macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
