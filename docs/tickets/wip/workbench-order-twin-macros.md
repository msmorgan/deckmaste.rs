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

## As landed

- Removed `Throughout` and `OnlyWhile` and their projection rows, leaving one `Continuously` and one `Conditionally` core row with erased context-threading proofs.
- Added `Macros.throughout` and reimplemented `Macros.onlyWhile`, `Macros.onlyUnless`, and `Macros.onlyIfSo` over the surviving rows.
- Re-spelled Gideon Jura through `Macros.throughout` and Turbulent Fen through `Macros.onlyUnless`.
- Re-spelled `onlyWhileThreadsPrefix`; no failing or `Unspellable` pin named either deleted constructor.

## Landing record

- Core rows: `StaticEffect` 36 → 35; `Effect` 82 → 81. Construction declarations: 384 → 384.
- Coverage: 16,337/16,337 selected cards covered; 0 selected uncovered, 0 unresolved ties, 0 internal failures, 0 round-trip mismatches, 0 ownership failures. Ambiguity: 10,843 unique and 5,494 specificity-resolved.
- Coverage lock: 48,987 lines; SHA-256 `c73055d0af4c46077cc580bbc0185baaba1188184b3391806dfe22be5cc9069b`.
- Build gate: 23/23 modules; 0 errors; 0 warnings.
- Citation gates: 0 non-compliant citation-looking strings; 0 stale citations; diff audit complete.
- Assurance: restored 0; re-spelled 1 (`onlyWhileThreadsPrefix`); ignored 0; added 0; removed 0.
- Deviations and additions: added erased threading proofs so both printed orders share each surviving dependent core row; ordinary direct uses carry explicit normal-order proofs where elaboration needs the base context.
- STOP: none.
