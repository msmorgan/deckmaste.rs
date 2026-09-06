---
needs: []
---
[design] **Define what makes an enacted deed's tag agree with its instruction
body.** The 2026-09-05 Lean LSP review compiled `by decide` witnesses that
`Instruction.check []` accepts both of these terms (with the Semantics and
Macros namespaces open):

```lean
.enact none (.action "Destroy") (.draw .you (.lit 1))
.enact none (.action "Destroy")
  (.move (target creature) (.zone .hand .bare) [])
```

`Check/AbilityRules.lean` validates the body and selected deed facts separately.
`Instruction.enactPatient` in `Check/Abilities.lean` recognizes only a directly
nested `move`: enclosing that move in `sequentially` also bypasses the enacted
patient-zone check. The tag is significant to deontics and provenance under the
existing semantics_v2 contract, so its relationship to the body matters.

Decide whether `enact` is trusted macro expansion output or independently
validated authorable syntax. For trusted output, specify and enforce the
expansion boundary that prevents arbitrary tag/body combinations. For authorable
syntax, establish the required agreement through declared facts and structural
checks. Do not accumulate checks naming individual lexemes or shallow body
shapes; a general equivalence checker is not a predetermined solution.

Done when the malformed examples are refused at the chosen boundary, a
singleton sequence cannot evade that boundary, genuine expansions retain their
profiles and provenance, and the contract states exactly what `Card.check`
guarantees about expansions. Use Lean LSP for the new pins and preserve the
existing card and pin assertions.

## Accepted direction

User ruling (2026-09-05): enforce a uniform macro boundary for card definitions,
so English spellings can attach uniformly to the macro declarations. This
supersedes the former macro-only authoring policy without a gate. The scope is
all semantic expressions throughout card bodies, including nested macro
arguments; printed-data records and leaf values remain data. Add automatic
authoring evidence at `Spelled` alongside the existing `Card.check` proof.

The boundary must inspect or retain authoring evidence: a macro call and its
expanded value are definitionally equal, so checking the resulting `Card`
alone cannot distinguish how it was authored. `Card.check` still validates the
body and its contextual use; it does not prove the trusted macro definition
matches its rules meaning. Existing internal diagnostic pins retain their
exact checker assertions.

## Landing record

Measured on change `xmwotnqsztnoxntkqmqqynnmllqmsxou`, with lock `covered`
20,254. The Semantics and English Lake projects remain independent.

### PROVE

- `./lean/scripts/build` passes all 64 jobs with warnings treated as errors.
  Lean LSP reports no diagnostics in the new authoring suite. Axiom inspection
  of `expandedValueIsDefinitionallyIdentical` and
  `macroDefaultAgentRetainsAuthorship` reports only `propext`; the source scan's
  `opaque` flag is the defined negative fixture `hiddenCard`, not an axiom.
- All 815 existing card definitions and 779 supporting definitions are retained.
  A temporary independent namespace compiled from the saved pre-migration
  sources verifies all 1,594 expanded-value equalities by `rfl`. No card identity,
  helper meaning, profile, or provenance was lost. Temporary comparison sources
  stay outside tracked code.
- All 2,168 existing named assertions retain their statements and outcomes.
  Restored: 0; re-spelled assertions: 0; ignored: 0; removed: 0. Card/helper
  expressions were mechanically re-spelled through named primitives; the five
  Thriving-land calls now use the `spelled <|` front door. The Write into Being
  placement fragment uses `manifestPlacement` with its exact previous value.
- The new suite adds five named assertions: `printedDataNeedsNoExpressionMacro`,
  `keywordRecordsMacroAuthorship`, `macroDefaultAgentRetainsAuthorship`,
  `authoredCardStillPassesSemanticCheck`, and
  `expandedValueIsDefinitionallyIdentical`. Total named assertions: 2,173.
  Eleven diagnostic tests additionally reject raw helper aliases, explicit raw
  macro arguments, Destroy/draw and Destroy/move-to-hand bodies, the singleton
  sequence bypass, record update, direct constructor notation, opaque cards,
  variable cards, unused raw let bindings, and raw syntax hidden in projections.
- `cargo xtask gate --changed` reports no affected workspace crates after refresh.
  Citation checking reports zero stale and zero noncompliant sites. Both new
  manifest citation sites were audited against the rule text; the lock adds
  only the verified manifest rule.
- English roundtrip, selection, traversal, licensing and corpus checks are
  outside this Lean-only change. No English code, declaration data, or coverage
  identity changed; no English structural-law or no-word-naming result is
  inferred from the Lean build.

### DISCLOSE

- The user broadened this ticket from enacted-deed trust to every semantic
  expression in a card, with records and leaves remaining data. This explicitly
  resolves the former policy-only boundary. `Spelled` now carries retained
  authoring evidence and two automatically synthesized proofs; its constructor
  is private. Primitive wrappers provide a named attachment point for future
  spellings. Internal enact/pronoun/window carriers have no primitive wrapper.
- Macro definitions remain trusted. The kernel checks the retained tree and
  card validity; the elaborator establishes the tree's correspondence to its
  input. This is neither rules-fidelity evidence for arbitrary macro definitions
  nor a proof of rendering, lowering, or execution.
- Deviations and additions: the uniform boundary, generated primitive interface,
  authoring evidence tree, five assertions, eleven rejection tests, and three
  synthetic Spelled witnesses implement the accepted broader scope. The new
  `manifestPlacement` macro preserves a pre-existing placement-only bench
  fragment; face-down characteristics and the turn-up special action remain
  outside that fragment's claim. The Game Model glossary now defines Semantic
  Macro; `lean/CONTRACTS.md` states the trust boundary.
- STOP resolutions: the earlier policy-versus-gate contradiction was resolved by
  the explicit user ruling above. Strict projection traversal temporarily hit
  Lean's existing heartbeat limit in `odricLunarchMarshal`, `bleedingEffect`,
  and `urborgScavengers`. Data-only projections now avoid unnecessary unfolding;
  semantic-containing projections retain inspection. All three original card
  assertions and the new bypass tests pass without increasing the limit.
- No new corpus identity or selected English analysis is claimed. Selection
  census, specificity share, and permitted licensing-checker totals were not
  remeasured because their code and inputs did not change.

### REPORT

Lock `covered`: 20,254 on `xmwotnqsztnoxntkqmqqynnmllqmsxou`. No English
construction was added or removed. Construction counts, homograph inventories,
form-literal/vocabulary overlaps, and coverage-command performance were not
remeasured for this Lean-only change. No corpus timing, host-load, worker-count,
or per-byte CPU result is claimed. No push was performed.
