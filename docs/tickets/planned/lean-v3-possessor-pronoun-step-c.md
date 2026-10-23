---
needs: [lean-v3-that-noun-after-move-historical]
---
# A possessor pronoun takes its referent away from a same-clause "that <noun>" (Saw in Half)

Standard constraints apply. Found by
[lean-v3-that-noun-after-move-historical](../wip/lean-v3-that-noun-after-move-historical.md).

## Problem

*Saw in Half*: "Destroy target creature. If that creature dies this way, its controller creates
two tokens that are copies of that creature, except their power is half that creature's power and
their toughness is half that creature's toughness. Round up each time."

All three later mentions of "that creature" mean the destroyed creature. The checker refuses all
three with `anaphor (word creature) one 0`. The cause is step `C` of rule `AC>R1`
(`docs/decisions/semantics-v3.md` §1.5, Stage 2, "Anti-coreference with same-clause pronoun";
`resolveCandidates` in `lean/Semantics/Check/Words.lean`, `afterC`). The possessor "its" in "its
controller" resolves the creature by `D′` and marks it `pronounResolved`. `C` then removes it
from every "that creature" in the same clause, and no other creature is left.

Probe evidence. The probes were scratch `lake env lean` runs after
lean-v3-that-noun-after-move-historical, starting from `DRS.ofList []`:

- `[destroy (target isCreature), doIf (did (that creature) die thisWay) (instruct (controllerOf
  it) (create 2 (copyOf (that creature) ex) []))]`, with `ex` the half power/toughness
  `copyCharacteristics`, gives three `anaphor (word creature)` refusals.
- `[destroy (target isCreature), instruct (controllerOf it) (create 2 (copyOf (that creature) [])
  [])]` gives one refusal. The lookback is not involved.
- The same body with `controllerOf (that creature)` in place of `controllerOf it` checks clean.
  So does `create 2 (copyOf (that creature) ex) []` with no possessor pronoun.

## What step C is for

§1.5 justifies `C` with *Kalitas, Bloodchief of Ghet*: "Its power is equal to that creature's
power". "Its" is the new token, and "that creature" takes the remaining older creature. *Broken
Visage* is the same case. `docs/decisions/english-lexical-analysis.md` governs syntax and Readings,
not anaphor resolution, and says nothing that bears on `C`.

## Existing step-C pins

Found by grepping for `pronounResolved` and the §1.5 witnesses:

- `Semantics/Check.lean`, `pronoun_and_demonstrative_split`: the pronoun is the clause's subject
  (`instruct it`), an argument.
- `Semantics/Proofs/DrtResolution.lean`, `demonstrativeExcludesPronounReferentBeforeRecency` and
  `demonstrativeRefusesWhenEveryCandidateExcluded`: these set `pronounResolved` directly on a
  hand-built DRS, with no pronoun form.
- `Semantics/Proofs/DrtWitnesses.lean`, `kalitasSelectsTokenThenDestroyedCreature` and
  `brokenVisageSelectsTokenThenDestroyedCreature`: both are spelled `getsPt it (.set (powerOf
  (that creature))) …`, so the pronoun is the argument of `gets`.

## Recommendation (proposed, not ruled)

Step `C` should not apply when the pronoun is a possessor inside a different argument from the
one holding "that <noun>": "its controller creates copies of that creature". A possessor does
not c-command the other argument, as in "His mother loves John", where "his" can be John. In the
checker, `C` would then ignore a `pronounResolved` mark set from a possessor slot
(`possessorOf`, `statOf`, `inArgSlot = false`). That needs the mark to record how the referent
was resolved.

Whether it keeps the pins:

- **The Lean pins all survive.** None resolves its pronoun from a possessor slot. In the two
  DrtResolution pins the mark is set by hand, and it would have to default to the argument
  case.
- **The English motivation does not.** In the Oracle text of *Kalitas* and *Broken Visage*,
  "Its power is equal to that creature's power" has "Its" as a possessor too. It does not
  c-command "that creature" either, yet the two are disjoint. A structural possessor exemption
  would let `R1` pick the token for "that creature" in a literal spelling such as `powerOf it`
  beside `powerOf (that creature)`.
- **So the c-command rationale predicts coreference in Kalitas as well.** What separates the two
  cards is that Kalitas has a second creature for "that creature" to take, and Saw in Half has
  none.

The owner should rule between:

1. **The possessor exemption, as stated above.** Kalitas and Broken Visage then stay correct only
   because they are spelled with `getsPt it`.
2. **`C` as a preference rather than an exclusion.** If `C` would leave no candidate, keep the
   pronoun's referent. This keeps Kalitas and fixes Saw in Half. It rewrites
   `demonstrativeRefusesWhenEveryCandidateExcluded`, whose candidate 1 is excluded only by `C`,
   to assert resolution to 1.

The landing adds *Saw in Half* to `lean/Semantics/Cards/Anaphora.lean` with its Oracle wording
("its controller"), and adds the ADR §1.5 step-`C` amendment.
