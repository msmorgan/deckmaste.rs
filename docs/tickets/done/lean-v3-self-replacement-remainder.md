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

## Change

- Fix the replacement's starting state, so the replacement reads what the replaced action read,
  not what the replaced action introduced. The site is the `.replace repld repl` arm of
  `ops.inst` (`Check/Phrase.lean:3852`): it steps `repld`, then `stepReplacementWith`
  (`Check/Phrase.lean:2005`) from `drs1`; the replacement must start from the state before
  `repld` introduced its part and remainder.
- Re-spell *Flow State* (`Cards/Anaphora.lean:89`, `flowState`) as the self-replacement its text
  prints. Today it is the if/else workaround `doIf (thereAre …) A B` that this ticket removes.
  Its pins (`okFlowState` `:102`, `flowStateThemAndTheRestReadTheCardsLookedAt` `:107`) must hold
  on the new spelling.
- Pin *Fblthp, the Lost*'s self-replacement (`Cards/Trigger.lean:587`) as a regression guard.
- Apply the batch-4 ruling below to `preferOrdinary` (`Check/Words.lean:847`; read at `:855`,
  `:898`, `:1264`).

## Ruling (owner, 2026-10-22, batch 4): Juxtapose, Option A

Once the self-replacement fix lands and *Flow State* is written as printed (a self-replacement
"instead"), **drop the existential demotion if *Flow State* still resolves cleanly**. Otherwise
**narrow the demotion strictly to replacement conditions**. Pin *Juxtapose* either way.

Context kept from the open question: the Q1 ruling ranks an existential's domain below any
ordinary earlier mention for plural resolution and for "the rest" / "the other" (`preferOrdinary`
in `Check/Words.lean:847`). The corpus counterexample is *Juxtapose*, "… Then exchange control of
artifacts the same way. If two or more permanents a player controls are tied for greatest, their
controller chooses one of them." There "them" is the tied permanents; with the demotion the
earlier artifacts win (reviewer's probe, 2026-10-22: after `get (bare (and [isArtifact,
youControl])) …`, a counted existential's "them" resolves to the artifacts, referent 1).
*Juxtapose* is not authored.

Decision procedure for the implementer:

1. Land the fix and re-spell *Flow State* with the demotion still in place; run its pins.
2. Remove the demotion (`preferOrdinary` calls). If `okFlowState` and
   `flowStateThemAndTheRestReadTheCardsLookedAt` still pass, leave it removed. *Juxtapose* then
   resolves "them" to the tied permanents, because the existential is the nearer mention.
3. If *Flow State* fails, restore the demotion only for an existential published by a
   replacement's condition (the `instead` condition of a self-replacement), not for any
   existential. Say in the landing which of the two outcomes happened and why.

## Tests

- `okFlowState` and `flowStateThemAndTheRestReadTheCardsLookedAt`: kept, re-pointed at the
  self-replacement spelling (rewritten; say so in the report).
- `okFblthpSelfReplacement` (new): *Fblthp, the Lost* checks clean, as the regression guard.
- `selfReplacementReadsWhatTheReplacedActionRead` (new): the Defect's confirmation trace,
  `sequentially [lookAt (topSlice 3), replace (put one … the rest …) (instead (put two … the
  rest …))]`, no longer refused `theRestFits`, and "them" in the replacement resolves to the
  slice (referent 1), not the remainder.
- `okJuxtaposeThemReadsTheTiedPermanents` (new): *Juxtapose*'s tied-permanents sentence spelled,
  "them" resolves to the tied permanents and not the earlier artifacts. Pinned in either outcome
  of the decision procedure above.
- Any pin that asserts the demotion (grep `existentialDomain`, `preferOrdinary` in `Proofs/`) is
  kept if the demotion survives for replacement conditions, and retired by name with the reason
  "the demotion it pinned no longer exists" if it is dropped.

## Note

A property test whose coordinated subject holds two indefinites and no definite member
(`.matches (.and [a creature, a land]) …`) also checks today. It is harmless, and no card writes
it.
