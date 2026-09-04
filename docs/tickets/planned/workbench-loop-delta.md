---
needs: []
---
**Return the binding delta explicitly from `effProfile` instead of recovering
it by length, so a loop body's in-place stack mutations survive the loop.**
Fresh workbench review 2026-09-03, F2 — a soundness hole, not an ergonomics
one.

`Effect.effDelta e = take (length (effIntro e) minus length bs) (effIntro e)`
assumes intros only prepend. They do not: `Phrase.moveIntro`,
`Phrase.setZoneReach`, `Words.groupSpent`, `Words.afterShuffle`,
`Words.dropLetter` and `Words.publicOnly` mutate or remove existing bindings.
So `effIntro (ForEachOf grp body)` and `effIntro (Repeated n body)` rebuild
from the *original* `bs` and the body's zone changes vanish.

Probes:

- P8 `[tap (target creature), ForEachOf (each Opponent) (exile You (It OneOf)), untap (It OneOf)]`
  — **admitted**. The creature is exiled inside the loop and untapped after it.
- P8b the same sentence without the loop — correctly refused (stale zone).

The same length arithmetic is in `Macros.itPrior`, `Macros.dealsDamageOwnPower`,
`Effect.doesEffIntro` and `Effect.distributedDelta`.

Fix: have `Effect.EffProfile` carry the split explicitly — `delta : List
Binding` and `rest : Bindings` — rather than letting callers recover it. The
`ForEachOf` clause then returns `pluralizeDelta delta ++ dropElem rest`: drop
the `elemIntro` binding, keep the mutated tail. `Repeated` and `Enact` take
the same shape, and the four macros read the profile instead of subtracting
lengths.

Size: M.

Done when: P8's sentence is refused for the same reason P8b's is, pinned in
`ProofsZone` (or `ProofsTurn`) and probed non-vacuous; a loop whose body
genuinely leaves the subject readable afterwards is a positive bench witness;
`effDelta`'s `take`/`minus` is gone from the tree and no caller reconstructs a
delta from two lengths; every existing pin that twins a loop still refutes for
its own stated reason; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
