---
needs: [lean-grammatical-reference-scopes]
---
[design] **Express the relational subject and sacrifice deed currently fused
in `Instruction.controllerSacrifices`.** The constructor in
`lean/Semantics/Abilities.lean` has dedicated validation and profile arms in
`Check/AbilityRules.lean` and `Check/Abilities.lean`. Its profile introduces a
player, moves the subject, and stamps the sacrifice provenance. The existing
`Cards/Anaphora.lean` witness is `arcumDagssonSacrifice` with
`okArcumDagssonSacrifice`.

Use the grammatical-reference decision to represent the controller of the
named object as the subject of the deed and read the same object in its body.
Do not duplicate the target declaration when expanding the phrase. Determine
whether the existing noun/clause machinery suffices or which general mechanism
is missing; the exact replacement is not settled. The established direction
removes constructors whose meaning is a composition of other constructs.

Done when the existing witness is re-spelled and still passes, the later
player and moved-object reads retain their identities and provenance, and
`controllerSacrifices` plus its dedicated checker/profile arms are removed.
Pin target multiplicity and reference behavior through Lean LSP, and preserve
the existing card and pin assertions in `lean/scripts/build`.

## Decision

The constructor was named `haveControllerSacrifice` in the claimed tree. It
and its dedicated validation, profile, cost, reflexive-enclosure and number-slot
arms are removed. `Macros.controllerSacrifices` composes `controllerOf` and
`sacrifice`, with an internal scoped read of the same object.

The missing general distinction is whether a noun phrase introduces its own
referent before mentions nested in its arguments. The structural property
`NounPhrase.introducesOwnReferent` supplies it. The relational subject announces
the controller and the object's head once; the deed re-reads that head.
Referential inputs with no introductions are reused. Coordinated references
have no single introduced head. No guard names a card or lexeme.

## Landing record

Measured on change `ruwwzmryzsonnpylkxnukvulupyyxptt`, lock `covered` 20,254.

### PROVE

- `./lean/scripts/build` passes all 66 jobs with warnings as errors. Lean LSP
  reports no diagnostics for `Proofs/ControllerSacrifice.lean`. Axiom inspection
  of the arbitrary-outer-context profile, authoring-boundary assertion and
  nested-description movement theorem reports only `propext`, with no source
  warnings.
- All 815 existing card definitions and 2,208 prior named assertion statements
  are retained. Restored: 0; re-spelled assertions: 0; ignored: 0; removed: 0;
  added: 27, bringing the named total to 2,235. The source fragment
  `arcumDagssonSacrifice` changes spelling; `okArcumDagssonSacrifice` is unchanged.
- The full profile theorem covers arbitrary outer contexts. Tests pin one
  target declaration, controller identity, artifact and creature types,
  graveyard carrier, sacrifice stamp, rider context, subsequent reads, existing
  references, nested descriptions and the Spelled macro boundary. The nested
  movement theorem preserves the complete binding list for arbitrary outer
  contexts, including the unmoved nested artifact.
- `cargo xtask gate --changed` reports no affected workspace crates. Citation
  checks report zero stale and zero noncompliant sites; the diff audit's two
  sacrifice citations were read against the rule text.
- English roundtrip, construction/leaf traversal, ties, internal failures and
  word-naming checks are outside this Lean-only change. No English code or
  corpus identity changed; no English structural-law result is inferred from
  the Lean gate.

### DISCLOSE

- No existing card identity is lost and no new corpus coverage is claimed.
  The synthetic Spelled witness tests macro admission, not a newly covered card.
- STOP and resolution: the initial whole-owned-scope composition admitted the
  existing fragment but made a valid nested-description input ambiguous.
  Integration was stopped and the regression reported. The referent distinction
  fixes it; `wholeOwnedScopeWouldBeAmbiguous` retains the failing expansion as
  an exact negative witness, with positive admission and movement assertions.
- Deviations and additions: the generic noun-introduction property and proof
  suite supply the general mechanism and regression evidence requested by the
  ticket. No new semantic AST constructor is introduced. Added assertion names:
  `expansionIntroducesTheTargetOnce`, `exactProfileForEveryOuterContext`, `targetMultiplicity`,
  `duplicatingThePatientWouldIntroduceTwoTargets`, `instructionIsAdmitted`,
  `laterPlayerIsTheController`, `laterCardIsTheSacrificedObject`,
  `sacrificeStampSelectsTheSameObject`, `laterPlayerReadIsAdmitted`, `laterCardReadIsAdmitted`,
  `formerPermanentReadIsAdmitted`, `laterCardRetainsBothTypes`,
  `laterCardRetainsGraveyardCarrier`, `riderReadsTheStampedPatient`,
  `reflexiveFollowUpRemainsAvailable`, `existingReferenceIsNotIntroducedAgain`,
  `existingReferenceIsAdmitted`, `outerArtifactDoesNotStealThePatient`,
  `thatPlayerCanTakeTheFollowingAction`, `nestedDescriptionIsAdmitted`,
  `wholeOwnedScopeWouldBeAmbiguous`, `nestedDescriptionMovesOnlyItsOwnReferent`,
  `nestedDescriptionDeclaresExactlyItsTwoTargets`,
  `coordinatedReferencesHaveNoSingleIntroducedReferent`,
  `typedDescriptionRetainsItsOwnReferent`, `wrappedSelfDoesNotInventAnIntroduction`,
  `compositionPassesTheAuthoringBoundary`.
- Standard deed machinery supplies an explicit stamped rider context; the old
  constructor had none and fell back to its unstamped input. This intentional
  difference is pinned separately. Ordinary sacrifice checks replace the fused
  constructor's bespoke singular check; no equivalence claim is made for every
  malformed raw constructor input. Existing refusal assertions are unchanged.
- Macro authors remain responsible for expansion meaning and scope ownership.
  These proofs establish checker/reference behavior, not engine execution.
  No unresolved ticket/ruling contradiction or new glossary gap remains;
  Reference and Reference Scope use the existing Game Model definitions.
- English selection census, specificity share, permitted licensing-checker
  totals and selected analyses were not remeasured; their code and inputs are
  unchanged.

### REPORT

Lock `covered`: 20,254 on `ruwwzmryzsonnpylkxnukvulupyyxptt`. No English
construction was added or removed. Construction counts, homograph inventories,
form-literal/vocabulary overlap inventories and coverage-command performance
were not remeasured for this Lean-only change. No corpus wall time, host load,
worker count or per-byte CPU measurement is claimed. No push was performed.
