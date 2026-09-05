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

## As landed

- **Whole-module `mutual` split.** `Phrase.idr`'s single block (346
  declarations, first declaration to EOF) is now 5 `mutual` blocks over 191
  declarations plus 155 ordinary top-level declarations; `Effect.idr`'s single
  block (246 declarations) is now 6 `mutual` blocks over 153 declarations plus
  93 ordinary ones. Neither module has a block spanning its whole body.
- **The genuine cycle.** Computed, not guessed: declarations were parsed into
  units, an edge drawn from each unit to every unit whose declared name (data
  type, constructor, record constructor, function, type synonym) it mentions
  outside comments and string literals, and the strongly connected components
  taken. `Phrase`'s cycle is 176 of 346 units, over the types `ZoneScope`,
  `LibPlace`, `ZoneExpr`, `NameSource`, `ChoiceDomain`, `EventSource`,
  `EventComplement`, `ComplementWritten`, `LookbackClause`, `Predicate`,
  `DetPhrase`, `RolesOk`, `Noun`, `Amount`, `Quantity`, `NonZeroQ`,
  `SliceCount`, `ComplementAnchor`, `LinkSource`, `PileMention`, `PaidSubject`
  and the 155 functions and type synonyms their constructor obligations name.
  `Effect`'s is 143 of 246 units, over `TokenChars`, `QualityPayload`,
  `TokenSpec`, `Static.StaticSpec`, `Compulsion`, `PlayPayment`,
  `DeonticRider`, `DamageOp`, `TokenRider`, `Cost`, `ManaRider`, `CopyExcept`,
  `RollRow`, `Instruction`, `Instructions`, `Sim.SimInstructions`,
  `KeywordParam`, `AbilityLost`, `AbilityAt`, `Coord.StaticParts`,
  `Paid.CostSeq`, `AddedPayment` and their obligation functions. The four
  remaining blocks in each module are two-element pairs (`predSays` /
  `predSaysAny`, `predNegFree` / `predNegFreeAll`, `condDelta` /
  `condDeltaAll`, `deckReadable` / `deckReadableAll`, `predRegime` /
  `predRegimeAll`, `instrChoiceDelta` / `instrsChoiceDelta`,
  `staticChoiceDelta` / `partsChoiceDelta`, `costChoiceDelta` /
  `costsChoiceDelta`) plus the 9-element `Condition` group.
- **Everything else moved out, in dependency order.** `Condition` keeps a
  9-element block with its own arity predicates; `DurationEnd`, `Exposed`,
  `VisibleThing`, `SearchScope`, `TokenPhrase`, `ChoiceClause`, `Ballot`, the
  `*Ok` predicates and the `moveIntro` family are ordinary top-level
  declarations after it. In `Effect`, `deckReadable`…`DeckCondition`, the
  `cost*` predicates and `Text.AbilitySeq` are outside the cycle as the ticket
  expected.
- **Three of the ticket's four "not needed before the data" names in `Effect`
  are in fact inside the cycle**, so they stay in the block: `KeywordParam` is
  a constructor argument of `AbilityAt` and itself takes a `Cost`;
  `reflexEncloseUse` matches on `Instruction` and is named by the
  `ReflexEnclosure` obligation on an `Instruction` constructor; `costActionOk`
  matches on `Instruction` and is named by `CostAction`. Only
  `deckReadable`…`DeckCondition` came out. Breaking any of the three needs a
  constructor change, which this ticket forbids.
- **No module was split in two**; no new module, no ipkg change.
- **No semantic change.** Every declaration keeps its name, its text and its
  behaviour: with leading whitespace stripped and lines sorted, the before and
  after files are identical apart from the added `mutual` keywords, and
  `cargo xtask map idris` over each module lists exactly the same types and
  constructors before and after (set-identical; order differs by construction).
  No witness and no pin was added, retired or re-spelled.
- **Undone: the elaboration target.** The ticket's "both elaborate in the
  2–3 s band" is not met and cannot be met by this reordering — see the STOP
  below.

## Landing record

### Numbers before/after

Per-module elaboration, `idris2 --find-ipkg --check` with the module's own
`.ttc`/`.ttm` removed and dependencies warm, three runs, before and after
alternated in one session on the same host:

| module | before (ms) | after (ms) | median before → after |
| --- | --- | --- | --- |
| `Experimental.Phrase` | 30 668 / 29 540 / 28 416 | 34 805 / 28 581 / 28 848 | 29 540 → 28 848 |
| `Experimental.Effect` | 4 214 / 5 320 / 6 069 | 4 325 / 4 275 / 5 547 | 5 320 → 4 325 |

Reference band on the same host, measured the same way: `Experimental.Words`
2 748 / 2 647 / 2 671 ms, `Experimental.Triggers` 1 556 / 1 585 / 1 564 ms.
An earlier, noisier pass before the A/B measured `Phrase` at 35 286 / 38 004 /
28 535 ms and `Effect` at 4 930 / 5 118 / 5 160 ms; the review's `Effect`
figure of 12 370 ms is stale — the profile-table round already took that out.

Clean full gate, `rm -rf build && ./scripts/build`: 64.75 s before, 65.42 s
after.

Structure: `Phrase` 1 whole-module block → 5 blocks (176, 9, 2, 2, 2) plus 155
top-level declarations; `Effect` 1 whole-module block → 6 blocks (143, 2, 2, 2,
2, 2) plus 93 top-level declarations. File lengths 2 860 → 2 861 and 2 576 →
2 572 lines.

### Gate lines

- `cd idris && ./scripts/build` (after `rm -rf build`): `46/46: Building Cards
  (src/Cards.idr)`, exit 0, 0 `Error` lines, 0 `Warning` lines.
- `cargo xtask cite check --list-noncompliant`: `0 non-compliant
  citation-looking string(s)`.
- `cargo xtask cite check`: `checked 14399 citations against cr.txt (eff.
  2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff`: `audited 23
  citation site(s) — read each rule text against its claim`. All 23 are
  pre-existing docstrings that changed line number only; the whitespace- and
  order-insensitive file comparison above proves no citation text or claim
  changed. `cargo xtask cite bless` was not run: no new rule is cited.

### Assurance counts

Restored 0, re-spelled 0, ignored 0, added 0, removed 0. No test, witness or
pin has a subject that this ticket retires; the whole `Proofs*` and `Cards/*`
surface is untouched (the diff is two files).

### Deviations and additions

None beyond the ticket's letter: no constructor, obligation, macro, witness or
pin added, deleted or renamed; no new module; no ipkg change; no comment added.
The only edits are the reordering of existing declarations and the `mutual`
keywords that group them.

### STOP — the ticket's performance premise is wrong

The ticket's diagnosis is that the whole-module `mutual` block is what makes
these modules slow. It is not, and the reorder is time-neutral: `Phrase` sits
at 28.8 s after the split against 29.5 s before, and the clean full gate is
unchanged.

`idris2 --timing 5 --check` locates the cost. Before the split, `Phrase`
spends 28.006 s elaborating, of which `Totality check overall` is 25.865 s and
`Processing decls` — the elaboration a `mutual` block actually reshapes — is
1.616 s. After the split it spends 27.517 s, of which totality is 25.820 s.
The per-name lines are the same functions in both layouts and in the same
band: `TestSubject` 2.562 s → 2.545 s, `OtherAnchored` 1.792 s → 1.874 s,
`EventAgent` 1.958 s → 1.858 s, `Nontarget` 1.683 s → 1.855 s, `HostedRead`
1.604 s → 1.774 s. Several of them, `TestSubject` and `Nontarget` among them,
are outside every `mutual` block after the split and cost the same as before.
Termination checking walks the call graph, not the syntactic block, so moving
a declaration out of `mutual` does not shrink the graph it walks.

I did not act on this beyond finishing the ticket's own instruction. The
reorder still stands on its own terms — the whole-module blocks are gone, and
the cycle that costs the time is now explicit and measurable in the source
rather than hidden in a 3 000-line block. But the 2–3 s band is unreachable
while `Predicate ↔ Noun ↔ Amount` and `Cost ↔ Instruction ↔ AbilityAt` remain
176- and 143-unit strongly connected components, because every constructor in
them carries an erased obligation naming a function that matches on the same
data. Cutting either component is a constructor change and so a separate,
design-bearing ticket; a follow-up should target the obligations, not the
block structure.
