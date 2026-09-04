---
needs: []
---
**Keep only the rules-defined contradiction clashes; delete `Phrase.predEq`
and its two gates.** Ruling 2026-09-04 on contradiction gates (cleanroom
review 3, finding U5 and D-Q3). The ruling is delete, not complete.

- `Phrase.predEq` answers `False` for `p` against itself on eight structural
  predicates — `Targets`, `ManaCostHas`, `CombatRel AttackerOf`, `AttachedTo`,
  `CompareOver`, `Or`, `OtherThan`, `Joined` and `CastBy _ (Just _)` — so
  `noNegatedPair` and `noRepeatedPair` never fire on them and
  `And [AttachedTo (target creature), Not (AttachedTo (target creature))]` is
  admitted. Delete `predEq`, `noNegatedPair` and `noRepeatedPair`.
- `Phrase.contradictionFree` keeps only the clashes the CR makes meaningless:
  token against card, permanent against spell, status, colour, and the emptied
  seed. Its other consumer `DistinctDisjuncts` either gets a replacement read
  stated over those clashes or goes with them; say which in the landing.
- Every pin that refuted through `noNegatedPair` or `noRepeatedPair` is
  re-spelled against the surviving clash it names, or retired together with
  its subject and named in the assurance counts. None is deleted silently.

Size: S. Done when: `grep` finds no `predEq`, `noNegatedPair` or
`noRepeatedPair` in the tree; each surviving clash has a pin and a same-module
twin, probed non-vacuous; the restored, re-spelled and removed counts are
reported; build at its module count. Standard constraints apply, including the
RON-shaped constraint.
