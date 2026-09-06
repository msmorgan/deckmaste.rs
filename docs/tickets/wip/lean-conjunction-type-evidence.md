---
needs: []
---
[design] **Stop silently choosing a referent's remembered type by the order of
type conjuncts.** In `lean/Semantics/Check/Phrase.lean`, `Predicate.seedTyAll`
keeps the first type found; `Payload.object` stores one `Option CardType`.
Lean LSP confirmed that after `target (.and [artifact, creature])`,
`that (.type .creature)` refuses with zero antecedents, but after
`target (.and [creature, artifact])` it accepts. This is inherited from Idris,
not solely a port regression.

The semantics_v2 contract describes predicates as flat sibling modifier sets.
Decide whether bindings preserve all relevant written type facts or whether a
distinguished grammatical head must be explicit in syntax. Do not introduce an
undocumented first-conjunct convention. The selected representation must also
account for disjunctions and the current carrier after movement.

Done when the two pure type conjunctions support the intended same reads, or
an explicit head distinction explains their difference; the choice is recorded
in the contract; and positive/negative anaphora pins cover both type words,
movement, and joined kinds. Prove order invariance for pure type modifiers if
the set interpretation is retained. Do not assume arbitrary modifiers with
binding effects commute. Preserve the existing card and pin assertions.

## Landing record

Measured tree: `wzkzlnyutzyylxysolkrknmvtkussvvr`, coverage-lock `covered`
count **20,254**. Checks were rerun after refreshing onto the latest default.

### PROVE

- **No silent loss:** all **2,133** existing named pins remain. The verdict
  comparison finds **0 problems** after the mechanical optional-type-to-list
  conversion in nine expected expressions. Every refusal list stays exact.
  All **815** existing `Spelled` card identities still compile; no card or
  Lean file was deleted, and no covered identity was lost.
- `lean/scripts/build` passes **61 jobs**, including the new TypeEvidence
  module. Lean LSP diagnostics are clean for the changed checker modules
  and new proofs. LSP verifies `pureTypePermutation` with only standard
  `propext` and no source warnings; proof-state inspection shows the final
  membership goal closing. Direct LSP evaluations also accept both target
  orders and the corresponding typed anaphoric reads with empty refusals.
- The new suite contains **35** proofs: five general facts about membership,
  canonicalization, pure written/remembered types, and permutation; thirty
  concrete witnesses for conjunctions, disjunctions, joined halves, type
  refinements, damage, and movement. Positive and negative reads distinguish
  remembered facts from the current carrier.
- **Structural laws and no word-naming:** no English parser, renderer,
  traversal, registry, licensing checker, or environment loader changes.
  No English coverage identities are gained or lost. Guards inspect declared
  card types, kinds, zones, and origins; no new lexeme or card-name guards.
  English corpus gates are not rerun for this separate semantics project.
- `cargo xtask gate --changed` selects **no workspace crates**. Citation
  checks report **0 noncompliant, 0 stale**; **4** changed citation sites
  were audited against the rules text.

### DISCLOSE

The [type-evidence contract](../../../lean/CONTRACTS.md#remembered-type-evidence)
selects canonical lists of facts, without a distinguished grammatical head.
Conjunction collects written card types, same-kind disjunction intersects
facts, and joined kinds retain evidence within their respective halves.
Movement preserves evidence while changing the current carrier. The contract
also records subtype fallback, role alternatives, and the boundary with
presupposition inference and continuous characteristic changes.

**Deviations and additions:**

- **0 restored, 20 respelled, 0 newly ignored, 35 added, 0 removed** pins.
  Nine expected expressions are respelled: `bioplasmExiledCardHasNoType`,
  `bioplasmTestRemarksType`, `describedSliceReadsAsCreature`,
  `bareSliceReadsUntyped`, `joinedCreatureTy`, `okExtraTurnNestedAmountOrder`,
  `okSkipNextNestedAmountOrder`, `okAdditionalPartNestedAmountOrder`, and
  `okUntapNestedAmountOrder`. They retain the same absent/singleton facts.
- Ten further statements only update payload fixtures: `okOtherAnchored`,
  `badDoubleOther`, `okDiscardHandCard`, `badDiscardIt`, `badOtherInOr`,
  `oneTokenIsOneSpec`, `manyTokensAreOneSpec`, `oneNonTokenIsNoSpec`,
  `okVesuvanShapeshifterCopyDuration`, and `okAnotherDisjunctPhrase`.
  `verbedTypeAndCopyRefinementsCommute` now quantifies over a list of
  remembered types. Supporting fixture definitions receive the same schema
  conversion. All asserted outcomes are preserved.
- The **35** additions are the complete
  [TypeEvidence suite](../../../lean/Semantics/Proofs/TypeEvidence.lean).
  Nested conjunction/disjunction alternatives, damage and attachment roles,
  choice-arm intersection, group narrowing, and type remarks are updated
  so consumers use the new evidence representation consistently.
- The preceding noun-refinement landing's single-type attachment restriction
  was reported as a limitation: multiple card types can coexist. This ticket
  replaces it with collected facts, including attachment-host seeding and
  movement; new positive/negative pins cover those paths.
- **STOPs and resolutions:** the first proof run exposed kernel reduction
  stuck in the new mutual recursion; the wrapper was moved outside the
  recursive block so ordinary `decide` works. The preserved
  `badFightPermanent` pin then exposed changed empty-evidence behavior, and
  the subsequent run exposed the same distinction in `drachNyen`'s stat
  read. Both failures were reported; code restores the existing distinction
  and both original assertions pass unchanged. No test was weakened to
  obtain green results and no ruling contradiction was resolved by fiat.
- The Game Model glossary now defines **Type Evidence**. No remaining
  glossary gap. No English construction/test changes or new-analysis
  identities; selection census and licensing totals are not remeasured.

### REPORT

All counts above are stamped with `wzkzlnyutzyylxysolkrknmvtkussvvr` and lock
count **20,254**. The lock is unchanged. English construction count, homograph
and form-literal/vocabulary inventories, coverage wall time, and per-byte
thread CPU telemetry are not measured for this semantics-only landing.
No corpus workers ran and no comparison with the English quiet-host ceiling
is claimed. No branch was pushed and no hosted CI result is claimed.
