---
needs: [lean-v3-measured-domain-antecedent]
---
# A self-replacement's replacement starts after the replaced action's remainder

Standard constraints apply. Minted from the owner's Q1 ruling on
`lean-v3-measured-domain-antecedent` (2026-10-22).

## Defect

`Action.replace replaced (… (instead replacement))` steps `replaced` (pre-event) and then steps
the replacement from the resulting DRS (`Check/Phrase.lean`, the `.replace` arm of `ops.inst`,
then `stepReplacementWith`). That state still holds what `replaced` introduced: its part ("one of
them") and its "the rest" remainder. So in the replacement, "them" reads the replaced action's
remainder, and "the rest" sees two groups. No existential is involved.

Confirmation trace (2026-10-22), from `lookAt (topSlice 3)` and then stepping the replaced action
"put one of them into your hand and the rest on the bottom of your library in any order":

- Visible referents: 3 (`the`, many, library: the remainder), 2 (`part`, one, library, part of
  1), 1 (the slice, many, library, group key 1), 0.
- `countGroups .object` is 2 and `countParts .object` is 1.
- `resolveCandidates .bare .many` returns `[3]`, the remainder.
- `sequentially [lookAt (topSlice 3), replace (put one … the rest) (instead (put two … the
  rest))]` is refused `theRestFits`. With a condition that introduces nothing, it is refused the
  same way.
- With single moves and no "the rest" (`replace (putInto (someOf (exactly 1) them) hand)
  (instead (putInto (someOf (exactly 2) them) hand))`) it checks.

Affected text: *Flow State*'s printed "instead" form. *Fblthp, the Lost* (`Cards/Trigger.lean`) is
the bench's instance of the same form; it checks today only because its replaced action takes no
part.

## Work

- Fix the replacement's starting state, so the replacement reads what the replaced action read,
  not what the replaced action introduced.
- Re-spell *Flow State* (`Cards/Anaphora.lean`, `flowState`) as the self-replacement its text
  prints. Today it is written `doIf … (some …)`, an if/else workaround that this ticket removes.
  Its pins (`okFlowState`, `flowStateThemAndTheRestReadTheCardsLookedAt`) must hold on the new
  spelling.
- Pin *Fblthp, the Lost*'s self-replacement as a regression guard.

## Open owner question (unruled)

The Q1 ruling ranks an existential's domain below any ordinary earlier mention for plural
resolution and for "the rest" / "the other" (`preferOrdinary` in `Check/Words.lean`). The corpus
has a counterexample: *Juxtapose*, "… Then exchange control of artifacts the same way. If two or
more permanents a player controls are tied for greatest, their controller chooses one of them."
There "them" is the tied permanents. With the demotion, the earlier artifacts win (reviewer's
probe, 2026-10-22: after `get (bare (and [isArtifact, youControl])) …`, a counted existential's
"them" resolves to the artifacts, referent 1). *Juxtapose* is not authored.

Once *Flow State* is a self-replacement, the demotion may be unnecessary, or it may be narrowed to
the replacement case. Which, if either, is the owner's call.

## Note

A property test whose coordinated subject holds two indefinites and no definite member
(`.matches (.and [a creature, a land]) …`) also checks today. It is harmless, and no card writes
it.
