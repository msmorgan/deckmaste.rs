---
needs: []
---
**Two residuals from the bounce-to-hand consolidation in [[macro-second-wave]].**
Neither blocks any committed card today; both were deferred rather than guessed.

## 1. Render "its owner's X" vs "your X" disambiguation — Hand AND Library

Bounce consolidates onto `Move(<ref>, Hand)` and (as of the bounce-to-library
production) `Move(<ref>, Library(FromTop(0)/FromBottom(0)))`. The single render
arm for each shape prints "**your** hand" / "**your** library" for every
reference. That is faithful when the object is provably yours (a self-bounce, or
a card scoped to *your* graveyard — e.g. Soulshift), but a *targeted* bounce of a
permanent that could be an opponent's should read "to **its owner's** hand" /
"on top of **its owner's** library." The render context can't distinguish the two
from `Move(_, Hand)` / `Move(_, Library(anchor))` alone — the destination RON
carries no owner qualifier.

**Priority note (library):** unlike the hand case, this is NOT ahead-of-need. A
corpus grep shows most of the ~118-card bounce-to-library family (Excommunicate,
Temporal Spring, Forced Retreat, Swirling Torrent, Jeskai Charm, and the graveyard-
hate "put on the bottom of its owner's library" variants) uses "its owner's
library" verbatim on a *targeted* (usually opponent-owned) creature. The byte-exact
fidelity gate blocks every one of them from graduating until this render lands, so
the bounce-to-library production graduates only the rare "your library" phrasings
today. Closing this is the single highest-leverage unblock for that family.

- Decide the disambiguation signal: infer from the reference (a `Target(...)`/
  `That(...)`/`It` of a possibly-foreign object → "its owner's X"; a self/`This` or
  a your-graveyard-scoped ref → "your X"), or carry an explicit owner marker on the
  destination. Apply the SAME signal to both Hand and Library arms.
- Add/replace the render arms so the round-trip is faithful for both destinations,
  with tests covering a self-bounce, a your-graveyard bounce, and a targeted
  (possibly-foreign) bounce, for each of Hand and Library (top and bottom).
- The hand case is still fidelity hardening (no committed card exercises the
  foreign-owned targeted hand-bounce yet); the library case is an active graduation
  unblock per the note above.

## 2. Non-target chosen-subject bounce coverage

The parser covers self, anaphor ("it"/"that card"), and `target <subject>` bounce.
It does NOT yet cover the *chosen-subject* form — "Return **a land you control** to
its owner's hand." / "return **another creature you control** to its owner's hand."
(corpus: ~25 + ~16 + ~10 occurrences). These choose an object among a filtered set
rather than targeting one, which needs chosen-object (not `target`) selection
semantics — a design decision beyond the consolidation pass.

- Design how a chosen-subject bounce is represented (a chosen `Selection` the
  controller picks, distinct from `TargetOne`), then add the parser production
  emitting `Move(<chosen ref>, Hand)`.
- Acceptance: a "return a land you control to its owner's hand" card graduates and
  round-trips.

## Verification
- `cargo test --workspace` green; `cargo clippy --all-targets -- -D warnings` clean.
- Wipe-first wizards regen; graduation count rises once the chosen-subject form lands.
