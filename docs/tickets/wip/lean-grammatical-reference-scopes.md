---
needs: []
---
[design] **Replace author-supplied stack offsets with grammatical reference
scopes, or explicitly separate the expanded representation that owns them.**
The 2026-09-05 semantics_v2 readiness review confirmed through Lean LSP that
ordinary `it` refuses two creature antecedents, while
`.pro .bare .one (.top 1)` accepts the same context. `Window.top` and
`Window.below` in `lean/Semantics/Words.lean` are public syntax.
`Macros.itPrior`, `itCondSubject`, `agentRef`, `itsOther`, and `lookedCards`
calculate offsets from checker bindings, sometimes against `[]`.

The existing contract in `docs/decisions/semantics-v2.md` assigns positional
references to lowering and requires unique compatible antecedents for ordinary
pronouns. Lean is the successor workbench; preserve that distinction unless
the user explicitly revises it. Decide how a clause's own subject, a previous
instruction's mentions, and a library slice are named grammatically. The
representation is not settled by this ticket.

Done when ordinary authorable pronouns cannot bypass ambiguity by choosing a
depth; internal references have an explicit construction/validation boundary;
and the affected macros preserve their referents when unrelated outer bindings
are added. Re-spell the existing cards and pins, preserving their assertions.
Add a negative ambiguity pin and positive scoped-reference twins using Lean
LSP. Audit empty-context offset calculations rather than merely renaming them.

The parked `workbench-segment-indexed-telescope` concerns recovering loop deltas;
this ticket does not reopen its ruling or require that implementation.

## Decision and audit

Keep raw numeric windows as explicitly internal expanded representation, behind
`Spelled`'s macro-authoring boundary. Scope macros name their owning phrase,
condition or preceding instruction. Their expansions now carry ordered binding
kind patterns, validated against the actual context, rather than treating an
empty-context length as a fixed depth. `introducedLetters` makes letter entries
conditional; required non-letter entries must match without searching past an
unrelated binding. Invalid patterns produce no readable scope and cannot update
outer bindings. Ordinary pronouns retain whole-context uniqueness.

The Lean LSP audit produced a concrete counterexample: the second read in a stat
change includes an unrelated artifact when X is already bound, because the old
empty-context count includes a letter binding that is not introduced again.
The new read succeeds and selects the creature; the old numeric expansion is
retained as a negative witness. The audit and each macro's owner are documented
in `lean/CONTRACTS.md`, “Grammatical references and internal windows.”
`itCondSubject` and `dealDamageOwnPower` no longer ask authors for `Bindings`.
The fixed single-subject comparison and chooser windows are retained and
proved independent of arbitrary outer contexts; neither uses an empty-context
length calculation. The chooser macro requires the immediately preceding
chooser mention.

## Landing record

Measured on change `yplqrvlyoxvprnmwkqutmzytlovqwnoz`, lock `covered` 20,254.

### PROVE

- The full `./lean/scripts/build` gate passes all 65 jobs with warnings treated
  as errors after refresh. The Lean LSP reports no diagnostics for the scope
  suite and a completed proof state for the general own-stat identity theorem.
  Axiom inspection reports only `propext` for
  `ownStatStableForEveryOuterContext`, and `propext` plus `Quot.sound` for
  `exactPrefixDoesNotInspectOuter`; source scans report no warnings.
- All 815 existing card identities remain admitted. All 2,173 prior named
  assertions retain their names and expected outcomes. Restored: 0; ignored: 0;
  removed: 0. Five assertions are re-spelled solely to remove the obsolete
  context argument: `ownSurvivesSecondSingular`, `okCondSubjectRead`,
  `badOwnEmptyDelta`, `badOwnTwoInDelta`, and `okThatCreatureAfterDamage`.
  No expected refusal list or binding assertion is weakened or replaced.
- The scope suite adds 35 named assertions (five general, thirty concrete),
  bringing the named total to 2,208, plus two diagnostic tests rejecting raw
  numeric and patterned pronouns at `Spelled`. General laws cover arbitrary
  outer contexts for the own-stat read and selected creature type, arbitrary
  non-letter prefix isolation, and the comparison and chooser slots' local scopes.
  Concrete twins cover every affected macro, fresh versus already-bound X,
  preserved type/carrier facts, scoped movement's full resulting bindings,
  outer ambiguity, and failure to search or mutate through a bad pattern.
- `cargo xtask gate --changed` reports no affected workspace crates. Citation
  checks report zero stale and zero noncompliant sites. No citation was added
  or changed by this ticket.
- English roundtrip, construction/leaf traversal, tie resolution, corpus
  failures, and word-naming checks are outside this Lean-only change. No
  English code, declaration data or corpus identity changed; no English
  structural-law result is inferred from the Lean gate.

### DISCLOSE

- There is no card loss or new corpus coverage claim. The previously unpinned
  X-reuse scope bug was reproduced through the LSP and repaired without
  changing an existing expected assertion. The former erroneous numeric
  expansion remains an explicit negative witness.
- Deviations and additions: two internal Window cases carry introduction
  patterns; `introductionWidth` validates them for reads and updates. No
  additional authorable noun form or numeric-depth API is introduced. The
  Game Model glossary now defines Reference Scope. The added proof names are:
  `agentWithOuter`, `agentWithoutOuter`, `amountLetterReallyChangesThePrefix`, `attachKeepsOuterAmbiguity`, `attachWithOuterLetter`, `attachWithoutOuterLetter`, `blockWithOuterLetter`, `blockWithoutOuterLetter`, `chooserScopeIgnoresOuter`, `comparisonScopeIgnoresOuter`, `conditionSubjectKeepsCreature`, `conditionSubjectWithOuter`, `conditionSubjectWithoutOuter`, `damageOwnPowerWithOuter`, `damageOwnPowerWithoutOuter`, `exactPrefixDoesNotInspectOuter`, `formerDepthReachesOuterArtifact`, `invalidScopeCannotMutateOuter`, `librarySliceKeepsLibraryCarrier`, `librarySliceWithOuter`, `librarySliceWithoutOuter`, `missingScopeCannotSearchPastAnotherKind`, `ordinaryItRemainsAmbiguous`, `ownStatIdentityStableForEveryOuterContext`, `ownStatKeepsCreature`, `ownStatStableForEveryOuterContext`, `ownStatWithOuter`, `ownStatWithoutOuter`, `previousInstructionKeepsCreature`, `previousInstructionWithOuter`, `previousInstructionWithoutOuter`, `scopedMovePreservesTheOuterBindings`, `sharedSubjectKeepsCreature`, `sharedSubjectWithOuter`, `sharedSubjectWithoutOuter`.
- The two diagnostic tests directly attempt raw `.pro` construction with a
  numeric depth and an introduction pattern. Both are rejected by the existing
  authoring gate. No test is deleted, ignored or replaced with a weaker check.
- This implements the ticket's explicitly allowed expanded-representation
  choice. Pattern ownership remains the trusted scope macro's responsibility;
  the checker validates binding kinds and reference uniqueness, not source
  positions or runtime object identity. No lowering/execution proof is claimed.
  The parked segment-indexed-telescope work was not reopened. No unresolved
  ticket-versus-ruling contradiction or glossary gap remains for this change.
- Selection census, specificity share, permitted licensing-checker totals and
  newly selected English analyses were not remeasured: their code and inputs
  are unchanged.

### REPORT

Lock `covered`: 20,254 on `yplqrvlyoxvprnmwkqutmzytlovqwnoz`. No English
construction was added or removed. Construction counts, homograph inventories,
form-literal/vocabulary overlap inventories and coverage-command performance
were not remeasured for this Lean-only change. No corpus wall time, host load,
worker count or per-byte CPU measurement is claimed. No push was performed.
