---
needs: []
---
**Make later constructor fields read the context established by earlier
fields consistently.** The 2026-09-05 review confirmed through Lean LSP:
`Instruction.check [] (.draw (target .opponent) (lifeTotalOf they))` accepts,
but replacing `draw` with `extraTurn` refuses with
`[.anaphor (.word .player) .one 0]`. `extraTurn`, `skipsNext`,
`doesntUntapNext`, and `additionalPart` in `Check/AbilityRules.lean` check their
later amounts against the original context. Audit their corresponding profile
computations in `Check/Abilities.lean` as well.

The existing semantics_v2 contract makes constructor argument order textual
order and threads discourse left to right, with explicit boundaries for
enclosed constructions. Record each field's input context once and make
validation and attribute computation follow the same contract. Choose a
maintainable implementation; this ticket does not require a generic traversal
framework or make sibling predicate modifiers sequential.

Done when the subject/amount examples agree, later fields can read earlier
mentions but cannot read later ones, and an audit of syntax fields against
checker/profile traversals identifies every intentional scope boundary.
Add paired positive and exact-refusal pins, including unrelated outer bindings
and fields that introduce their own mentions. Verify through Lean LSP and
`lean/scripts/build`, preserving existing card and pin assertions.

## Landing record

Change `wszuuxnqlpvvqpqwnxtlwmzskqxtnmyt`; English lock `covered`: 20,254.

**PROVE:** All four scheduling constructors check their amounts after the
subject and compose output through `Amount.intro`. LSP reproduced the missing
subject reads; the new pins also assert complete output bindings for nested
amounts, their order, retained outer bindings, forward-reference refusal, and
later reads of amount-introduced mentions. `lean/CONTRACTS.md` records each
scheduling field's input and the surrounding enclosed traversal rules.
The refreshed `lean/scripts/build` passed all 61 jobs without warnings.
Assurance: 25 pins added; 0 restored, re-spelled, ignored, or removed.
All existing card and pin assertions remain. LSP diagnostics are empty;
the audited exact-output proof uses only `propext`.

**DISCLOSE:** Newly accepted scheduling expressions are the four subject-read
pins in `Proofs/Turn`; no existing card verdict changed. No lexical guards or
new syntax constructors were added. Deviations and additions: the shared
checker-contract document and README link make the required audit durable.
No new glossary terms or unresolved STOPs.

**REPORT:** English coverage, selection and declaration data are unchanged;
English traversal, lexical inventories, licensing counts and performance were
not remeasured for this Lean-only change. No CR citation sites changed.
