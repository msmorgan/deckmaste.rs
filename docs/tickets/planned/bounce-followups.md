---
needs: []
---
**Two residuals from the bounce-to-hand consolidation in [[macro-second-wave]].**
Neither blocks any committed card today; both were deferred rather than guessed.

## 1. Render "its owner's hand" vs "your hand" disambiguation

Bounce now consolidates onto `Move(<ref>, Hand)`, and the single render arm for
that shape prints "Return X to **your** hand." for every reference. That is
faithful when the object is provably yours (a self-bounce, or a card scoped to
*your* graveyard — e.g. Soulshift), but a *targeted* bounce of a permanent that
could be an opponent's should read "to **its owner's** hand." The render context
can't currently distinguish the two from `Move(_, Hand)` alone.

- Decide the disambiguation signal: infer from the reference (a `Target(...)`/
  `That(...)` of a possibly-foreign object → "its owner's hand"; a self/`This` or
  a your-graveyard-scoped ref → "your hand"), or carry an explicit owner marker.
- Add the render arm so the round-trip is faithful for both, with tests covering
  a self-bounce, a your-graveyard bounce, and a targeted (possibly-foreign) bounce.
- No committed card exercises the foreign-owned targeted-bounce case yet, so this
  is fidelity hardening ahead of such a card, not a live regression.

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
