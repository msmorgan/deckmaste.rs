---
needs: []
---
**Delete `Throughout` and `OnlyWhile` as core constructors and re-mint them as
macro wrappers over `Continuously` and `Conditionally`.** Cleanroom review
2026-09-03, R5 — ruled, overturning the review's own recommendation (§4 listed
the pair under "what not to change").

`Effect.idr:1185–1191` has `Continuously se span` beside `Throughout span se`,
and `Effect.idr:433–438` has `Conditionally c se` beside `OnlyWhile se c`:
each pair differs only in argument order, which under ADR §2 is textual order.
The bench spells `Throughout` once against 133 `Continuously`, and `OnlyWhile`
once against 2 `Conditionally`.

## The ruling

The semantics layer carries **one core row each** — `Continuously se span` and
`Conditionally c se`. The printed word order is preserved at the **macro
layer**: `Macros.throughout span se = Continuously se span` and
`Macros.onlyWhile se c = Conditionally c se`. Word order that only permutes
existing slots is a macro's job; it does not earn a constructor, and lowering
should not have to read an order flag back out of the semantics layer.

Re-spell the two bench sites through the new macros.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; `Throughout` and
`OnlyWhile` are gone from `Effect` along with their table rows in every
per-constructor projection; `throughout` and `onlyWhile` exist as macros whose
bodies are the reordered core terms; the two bench sites read through them and
typecheck; any pin naming the deleted constructors is re-spelled against the
surviving row and still refutes. Standard constraints apply.
