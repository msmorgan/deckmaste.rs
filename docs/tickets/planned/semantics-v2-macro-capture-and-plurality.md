---
needs: []
---
**What the RON macro language can say: capture parameters and
amount-derived plurality.** Design-bearing (sol or Opus); standard
constraints apply.

Lean's
`semantic_macro` bodies use two devices with no positional-RON spelling:
`capture` parameters that re-read an argument after a binding is introduced
(Fight, Regenerate) and plurality computed from an `Amount` argument
(`Amount.plur (.lit 1) = .one`: Scry, Surveil, Fateseal, Connive). Decide
whether the RON macro language grows the device (`macros-are-declarative.md`
forbids control flow; a typed capture is not control flow) or the Lean macro
is re-spelled without it. `lean-macros-from-ron` needs the same answer from
the other side, so record it in `semantics-v2.md` §12. `target-sugar-elaboration`
names Fight as its fixture for a different mechanism; related, not overlapping.


Then give Fight, Regenerate, Scry, Surveil, Fateseal, and Connive their
bodies under the chosen device, with a canon card each proving through
`lean-check`.
