---
needs: [workbench-copies-are-spells]
---
**Fold `Kind.Ability` into `Object` with an `AbilityP` payload and zone Stack,
and shrink the join family to the object case.** Fresh workbench review
2026-09-03, R2, resolved by ruling.

**Ruling (settled 2026-09-03): the grammar has one object.** An ability on the
stack is in the stack zone [CR#405.1] and is an object [CR#109.1], so
`Words.Kind.Ability` is not a third entity beside `Object` and `Player`. It
becomes `Object` carrying `Words.Payload.AbilityP`, with
`Words.payloadZone (AbilityP _)` returning `Just Stack` rather than `Nothing`.
The review recommended keeping the split and recording the divergence; the
ruling goes the other way, and the divergence disappears instead.

What collapses with it:

- `Phrase.NounWord.JoinW`, `AbilityJoinW`, `CopyJoinW` and `UnionHalf` — four
  join-reading words that exist only because "spell or ability" is a join of
  two kinds. With one kind they are one word over one payload.
- `Phrase.StackActOn` and its `StackAbility` case (`Phrase.idr:2194–2208`,
  with `Counterable` and `Copiable` over it): the ability case is the object
  case with a stack zone, so the zone conjunct already carries it.
- `Words.unionPayload`'s `AbilityP`/`ObjectP` cross-clauses
  (`Words.idr:1461–1467`) and `Phrase.setZone`'s `Ability` no-op clause
  (`Phrase.idr:2613`).
- The `PhAbility` cases in `Phrase.joinHalfPayload`, `Phrase.bindFor` and
  `Phrase.elemPayload`, and `Phrase.IsManaAbility : Predicate bs Ability`,
  which becomes `Predicate bs Object` gated on the payload.

An ability still cannot be moved and has no printed characteristics; those
refusals move from the kind index to the payload, and every pin that names
them must keep refusing for a payload reason rather than a kind reason.

Depends on `workbench-copies-are-spells`: both touch `Words.Payload` origins
and the stack-reading words, and the copy exclusion should already be out
before the join family is rewritten.

Size: L.

Done when: `Kind` has no `Ability` constructor; "counter target spell or
ability" and "copy target activated ability" are typechecking bench witnesses
reading one kind; `JoinW`/`AbilityJoinW`/`CopyJoinW`/`UnionHalf` and
`StackActOn` are gone or reduced to the object case; the pins that refused
moving or animating an ability still refute, re-spelled against the payload
and each probed non-vacuous; the build is 44/44 with 0 errors and 0 warnings.
Standard constraints apply, plus the RON-shaped constraint: a core constructor
is admissible only if the RON re-emitter can produce it from a RON node, and a
macro only if it names a RON macro
(`docs/decisions/workbench-ron-shaped-and-label-rulings.md`).
