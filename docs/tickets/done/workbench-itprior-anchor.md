---
needs: []
---
**Let `Macros.itPrior` take a noun anchor, not the antecedent instruction.**
Residue of `workbench-windowed-pro` (2026-09-04, review D-Q18): `itPrior`
still takes the whole antecedent `Instruction`, because only that term
fixes the clause's outer scope by unification — a noun anchor leaves the
window's `outer` unsolved and a literal `Top n` prints a number the
ruling forbids. Find the shape that derives the window from a noun anchor
(the anchor's own delta length is data the grammar already has), or
record why the instruction is the right anchor; keep every `itPrior`
witness and pin re-spelled, never deleted.

Size: S. Done when: `itPrior`'s parameter is a noun or the ticket records
the ruling; no printed number in any macro body; build at its module count.
Standard constraints apply, including the RON-shaped constraint.

## Ruling 2026-09-04

Keep the antecedent instruction as `itPrior`'s anchor: the instruction is
the antecedent clause, which is what "prior" names. Close by recording this
on the macro's one doc line; no code change.

## As landed

- Ruling recorded on `Macros.itPrior`'s doc line: the antecedent instruction
  stays the anchor. No code change.

## Landing record

- Gates: `cd idris && ./scripts/build` 46/46, 0 warnings; cite check 0 stale.
- Assurance: restored 0, re-spelled 0, ignored 0, added 0, removed 0.
- No deviations; no STOP.
