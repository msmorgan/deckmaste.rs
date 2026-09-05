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
