---
needs: []
---
[design] **Compose noun-word refinements instead of adding a constructor for
each combination.** `NounWord.typedCard` and `NounWord.abilityCopy` in
`lean/Semantics/Words.lean` combine a noun with a type or copy distinction.
Audit the related `Reach` cases and their matching rules before choosing the
replacement: these reads must preserve carrier, type, origin, and ambiguity
behavior, not merely accept the same broad kind.

The semantics_v2 contract calls for a decomposed noun-word vocabulary, and the
current refinement direction keeps a constructor only where its expansion
cannot carry its distinction. Decide which refinements compose and which
primitives must remain; do not replace the vocabulary with an unrestricted
predicate that discards the existing read discipline.

Done when typed-card and ability-copy reads use the selected compositional
shape, or a retained primitive has an explicit irreducibility justification;
the affected cards and pins are re-spelled with the same asserted outcomes;
and positive/negative twins distinguish a copied ability from its source and a
typed card from an incompatible type or carrier. Verify with Lean LSP and
`lean/scripts/build`. Coordinate representation choices with
`lean-conjunction-type-evidence`; neither design is prescribed here.

## Landing record

Measured tree: `qzwyvrpsnqtoqwskqwmmnqvzlxovunyl`, coverage-lock `covered`
count **20,254**. Results below were rerun after refreshing onto default.

### PROVE

- **No silent loss:** all **2,112** existing named pins remain with their
  exact expected results; the verdict diff reports **0 problems**, without
  expected-expression normalization. All **815** `Spelled` card definitions
  retain their names and compile. No lost card or pin identity.
- `lean/scripts/build` passes **60 jobs**. Lean LSP diagnostics are clean
  for the syntax, word matcher, phrase checker, and new NounWords suite.
  LSP proof-state inspection shows the refinement-order goals closing;
  axiom verification of the direct and verbed commutativity theorems finds
  only standard `propext`, with no warnings.
- The new pins exercise exact selected bindings, carrier/type mismatches,
  copy origin, ambiguity, self exclusion, isolation of joined halves, and
  deed provenance. General theorems prove type/copy refinement commutativity
  in the direct, half-word, and verbed matchers.
- **Structural laws and no word-naming:** no English parser, renderer,
  traversal, registry, licensing checker, or environment loader changes.
  No English coverage identities are lost or gained by this landing. Corpus
  gates are not rerun for this independent semantics change.
- `cargo xtask gate --changed` selects **no workspace crates**. Citation
  checks report **0 noncompliant, 0 stale**; **3** diff citation sites were
  audited against the actual rules text.

### DISCLOSE

The selected representation is `NounWord.ofType word ty` and
`NounWord.copied word`. They replace `typedCard ty` and `abilityCopy` with
`ofType card ty` and `copied ability`, respectively. Carrier, remembered type,
copy origin, and ambiguity remain distinct checks on the same referent.
The [noun-word contract](../../../lean/CONTRACTS.md#noun-words-and-refinements)
records each remaining primitive's independent requirement, all Reach paths,
attachment-profile constraints, and the boundary with
`lean-conjunction-type-evidence`. No unrestricted predicate replaces a word.

**Deviations and additions:**

- **0 restored, 2 respelled, 0 newly ignored, 21 added, 0 removed** pins.
  The respelled pins are `bioplasmTypedCardReadWrites` and
  `okAbilityCopyReadAfterCopy`; their expected results are unchanged.
  Existing Copy-family card spellings and the typed-card Trigger witness
  use the same refinements.
- The **21** additions are the complete
  [NounWords suite](../../../lean/Semantics/Proofs/NounWords.lean). Eighteen
  concrete witnesses provide the required positive/negative coverage and
  exercise the newly composable paths; three general commutativity theorems
  establish that type and copy refinements do not depend on their order.
- `verbedWordOk` receives origin as well as the existing stamp/type/carrier,
  so refinement works in provenance reads. Join-half filtering applies all
  refinements inside one half. Attachment checks reject incompatible
  refinements instead of silently choosing a single type by their order.
  These additions make the selected composition coherent across consumers.
- Some CONTRACTS references to instruction names had survived the preceding
  naming migration. They are updated here while editing that document;
  current helper names such as `thisWayCtx` remain unchanged.
- No new domain concept or glossary gap. The Game Model already separates
  Referent Sort, Object Class, Card Type, and Copy. This landing documents
  their representation contract without changing the glossary.
- No English construction/test changes or new-analysis identities. English
  selection census and permitted licensing-checker totals are not
  remeasured for this scope. **STOPs: none.**

### REPORT

All counts above are stamped with `qzwyvrpsnqtoqwskqwmmnqvzlxovunyl` and
lock count **20,254**. The lock is unchanged. English construction count,
homograph and form-literal/vocabulary inventories, coverage wall time, and
per-byte thread CPU telemetry are not measured for this semantics-only
landing. No corpus workers ran and no claim is made against the English
quiet-host performance ceiling. No branch was pushed and no hosted CI result
is claimed.
