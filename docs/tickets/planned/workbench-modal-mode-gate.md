---
needs: []
---
**Delete the `distinctModes` gate and the `effEq` table it is the only
consumer of.** Cleanroom review 2026-09-03, F15 / R3, ruled.

`Effect.idr:1746–1867` — `effEq`, roughly 120 lines, mostly `_ _ = False` —
has exactly one consumer: the `Modal` gate `{auto 0 dm : So (distinctModes
modes)}` (`Effect.idr:1286`).

## The ruling

[CR#700.2d] is about *choosing* the same mode, not about the modes being
textually distinct. Identical modes are rules-meaningful: "choose two — draw a
card; draw a card" is legal and means draw two. The gate refuses a
rules-meaningful shape on printing history, which is exactly what a pin may
not do. Delete the gate and `effEq` with it.

This is one of the nine per-constructor `Effect` tables counted in F6;
removing it is independent of the `effProfile` fold in
`workbench-table-machinery` and can land before it.

Size: S.

Done when: the build is 23/23 with 0 errors and 0 warnings; `distinctModes`
and `effEq` are gone from `Effect` and nothing else referenced them; a `Modal`
with two identical modes is a typechecking bench witness or, absent a printed
card, a synthetic witness with a note; any pin that refuted through
`distinctModes` is deleted or re-spelled against a gate that still exists, and
the landing record names it. Standard constraints apply.
