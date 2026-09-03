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

## As landed

- Removed `Modal`'s `distinctModes` obligation and the orphaned `effEq`,
  `anyEffEq`, and `distinctModes` machinery.
- Removed the retired obligation from all five modal macros while retaining
  their mode-count and quantity-fit gates.
- Re-spelled `badDuplicateModes` as the synthetic positive witness
  `identicalModesAllowed`: choosing two identical draw modes typechecks.
- Kept the independent modal refusal pins `badModalOneMode`,
  `badModalOverreach`, `badModalReadsAcrossModes`, and `badReadsAfterModal`.

## Landing record

Measured on change `oykprnmv` with 16,337 covered lock identities.

- `Effect` constructors remained 82 -> 82; per-constructor effect tables went
  9 -> 8; `Modal` auto gates went 5 -> 4; each modal macro's auto gates went
  3 -> 2.
- Coverage and construction count were unchanged: 16,337 -> 16,337 selected
  and covered units; 384 -> 384 construction declarations. The coverage lock
  remained current at 48,987 lines with SHA-256
  `c73055d0af4c46077cc580bbc0185baaba1188184b3391806dfe22be5cc9069b`.
- Selection census was unchanged: 10,843 unique and 5,494
  specificity-resolved selections, with 0 unresolved ties and 0 internal
  failures.
- `idris/scripts/build`: 23/23 modules, 0 errors, 0 warnings. The positive
  witness also passed a package-aware single-module check; changing it to
  choose three from two modes produced the expected `ModesFit` failure.
- Assurance: restored 0; re-spelled 1 (`badDuplicateModes` ->
  `identicalModesAllowed`); ignored with blockers 0; added 0; removed 0.
- Deviations and additions: deleted the now-orphaned `anyEffEq` helper with
  the named table and gate; no other additions. No STOP was taken.
