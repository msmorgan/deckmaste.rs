---
needs: [english-structural-recovery-zero]
---
**Replace the hand-maintained `RecoveryWalker` exhaustive AST traversal
(syntax/mod.rs, ~650 lines) with a derived or shared walk.** It powers
`recoveries()`/`lexical_opacity()` and structurally parallels the renderer's
own full traversal; every AST change must be mirrored by hand (each recovery
round adds walker arms for its new variants), and it already carries an
inert placeholder (`predicate_head`, body `let _ = (head, context);` —
verified 2026-07-24). Candidate shapes: a visitor derive, or folding the
recovery queries into one traversal shared with the renderer.

MUST wait for the recovery campaign (the `needs:` gate): the walker is the
campaign's measuring instrument and its census is the campaign's primary
invariant. Once structural recovery reaches zero, `recoveries()` becomes a
pure regression gate and this refactor is safe. Gate then: census
byte-identical (all-zero) + clean roundtrip + exact test-count parity.
Standard constraints apply.
