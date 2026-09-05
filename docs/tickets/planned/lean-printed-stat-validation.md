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
