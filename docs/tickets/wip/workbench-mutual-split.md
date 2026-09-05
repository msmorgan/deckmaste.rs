---
needs: [workbench-profile-table-drift]
---
**Split the whole-module `mutual` blocks in `Phrase` and `Effect`.** Cleanroom
review 3, 2026-09-04, finding G1.

- `Phrase.idr` is one `mutual` block from its first declaration to end of file
  (3 023 lines) and `Effect.idr` one from `SpendPurpose`'s end to the last
  cost predicate. They elaborate in 23.6 s and 12.4 s against 1–3 s for every
  other module, and every full-gate build re-elaborates them.
- The genuine cycle in `Phrase` is `Predicate ↔ Noun ↔ Amount`, with
  `ZoneExpr`, `LookbackClause` and `EventComplement` reaching it through
  `Noun`. `Condition`, `DurationEnd`, `Exposed`, `VisibleThing`, `SearchScope`,
  `TokenPhrase`, `ChoiceClause`, `Ballot`, the `*Ok` predicates and the
  `moveIntro` family are downstream of it and belong in ordinary blocks after
  it.
- In `Effect`, `StaticSpec`, `Instruction`, `Cost` and `AbilityAt` need each
  other, but `deckReadable`…`DeckCondition`, `KeywordParam`, the `cost*`
  predicates and `reflexEncloseUse`/`costActionOk` do not need to be declared
  before the data; `SpendPurpose`…`kindSourceIntro` are already outside.
- Reorder each module into (types, plus only the functions their indices
  mention) followed by plain blocks. Take this after
  `workbench-profile-table-drift`, so the two rounds do not rewrite the same
  `Effect` region.
- Record per-module elaboration times before and after in the landing record.

Size: L. Done when: neither module has a `mutual` block spanning its whole
body; both elaborate in the 2–3 s band that every other module occupies; no
constructor, obligation or macro surface changes; build at its module count.
Standard constraints apply, including the RON-shaped constraint.
