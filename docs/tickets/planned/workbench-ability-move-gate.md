---
needs: []
---
**Refuse moving an ability now that abilities are objects.** Residue of
`workbench-ability-kind-fold` (2026-09-04): with `Kind.Ability` folded into
`Object` + `AbilityP`, `Move (Macros.target (AbilityHead AnyActivated)) exileZ
[]` typechecks. An ability on the stack is an object that cannot change zones
[CR#113.1c] (it ceases to exist when it leaves the stack, it is never moved).
The payload gate on `Move`'s `what` was written and reverted: `what` is
abstract inside the sixteen `Macros` move wrappers, so the obligation must
thread through them to their callers, and the first attempt produced
`Multiple solutions found` in `scry`/`fateseal`/`surveil`.

Fix: a `MovablePayload` gate on `Move` (and `SetStatus`? decide from the CR)
threaded through the move macros as one erased obligation each; a pin
refusing the sentence above, probed non-vacuous; the look macros keep their
single solution.

Size: M. Done when: the pin refutes; every move macro keeps its surface; the
bench typechecks; build at its module count. Standard constraints apply.
