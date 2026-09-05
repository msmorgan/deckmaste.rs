---
needs: []
---
**Validate the expressions inside printed stat boxes, not only their
presence.** `CardFace.check` in `lean/Semantics/Check/Card.lean` checks box/type
compatibility without checking the `Amount` values. The 2026-09-05 Lean LSP
review compiled this acceptance:

```lean
import Semantics.Check
open Semantics
example : Card.check (.singleFaced { characteristics :=
    { name := "Review witness", types := [.creature],
      power := some .thatMuch, toughness := some (.lit 1) } }) = [] := by decide
```

`Amount.check [] .thatMuch` itself refuses with
`[.quantOutcomeInScope 0]`. The README says printed card stats are literals,
while dynamically described objects can carry Amount expressions. Enforce that
consumer-specific contract across faces, shared-line boxes, level bands, and
prototype frames. Audit power, toughness, loyalty, and defense. Reusing
`Characteristics` does not authorize unchecked expressions in printed slots.

Done when the malformed face is refused, permitted printed literals (including
the existing negative-number cases) and characteristic-defining absent slots
retain their verdicts, and token/effect expressions retain their context-aware
checks. Add same-slot valid twins and exact-refusal pins, verify them through
Lean LSP, and run `lean/scripts/build` without deleting existing assertions.

## Landing record

Change `rpulxnmuqwvqylxqpnlzrtqupqulrtqp`; English lock `covered`: 20,254.

**PROVE:** `cardBoxOk` now requires literals in all present printed stat slots,
through the common path used by faces, shared-line halves, level bands, and
prototype frames. The guard reads only closed Amount constructors. Lean LSP
reproduced 14 missing-refusal cases before the fix; all now pass alongside
their literal twins. `lean/scripts/build` passed all 60 jobs without warnings,
preserving every existing card and pin assertion, including negative printed
stats, absent characteristic-defined slots, and token expressions. Assurance:
37 pins added; 0 restored, re-spelled, ignored, or removed. LSP diagnostics
are empty; the audited `badPrintedPowerLiteral` proof uses only `propext`.

**DISCLOSE:** No existing accepted card was lost and no new card acceptance is
claimed. Nonliteral printed stats report `cardBox`; shared-line cards retain
the existing per-half box checks and therefore report it twice. Dynamic
bundles still accept expressions in context and refuse missing magnitude
bindings. Deviations and additions: README contract clarification added;
no additional syntax constructions, glossary terms, or unresolved STOPs.

**REPORT:** English declarations and coverage lock are untouched. English
selection, traversal, lexical inventories, licensing counts, and coverage
performance were not measured for this Lean-only change. No CR citation sites
changed; the diff citation audit selected none.
