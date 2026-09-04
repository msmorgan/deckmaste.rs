---
needs: []
---
**Publish `ForEachOf`'s group binding after the loop.** Ruling 2026-09-04
(cleanroom review 3, D-Q7).

- `Effect.instrIntro (ForEachOf grp body) = pluralizeDelta (instrDelta body)
  ++ bs` discards the group's own bindings, so after "for each creature you
  control, …" neither the group nor anything the element clause described
  about it is readable. Publish `nounDelta grp`, pluralised, alongside the
  body delta.
- `Effect.ForEachKindOf` carries the identical `KeepsOuter` obligation and was
  left at `instrIntro … = bs` by `workbench-choice-scope-residues`, which
  recorded it as a loop-delta gap rather than a separate shape. Apply the same
  rule there and unblock Celestial Judgment.
- Bench a printed card that reads the group after the loop, and pin a read of
  a body-local binding the loop does not publish.

Size: S. Done when: both loop rows publish the group and the body delta;
Celestial Judgment is benched; the pin probes non-vacuous; build at its module
count. Standard constraints apply, including the RON-shaped constraint.
