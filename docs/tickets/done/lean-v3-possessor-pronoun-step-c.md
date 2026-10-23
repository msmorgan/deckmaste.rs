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

## Ruling (owner, 2026-10-22, batch 4): option (2)

**Step `C` becomes a preference, not an exclusion.** Exclude a referent already taken by a
same-clause pronoun only when another candidate remains; otherwise allow coreference. This
preserves *Kalitas* (there is a second creature for "that creature" to take, so `C` still removes
the token) and admits *Saw in Half* (there is none, so the pronoun's referent stays). Both
`demonstrativeRefusesWhenEveryCandidateExcluded` and the ADR §1.5 text change with it.

**Option (1) is rejected**, the possessor exemption ("a possessor does not c-command the other
argument"). Why: *Kalitas*'s "Its power is equal to that creature's power" has "Its" as a
possessor too. It does not c-command "that creature" either, yet the two are disjoint. A
structural possessor exemption would therefore let `R1` pick the token for "that creature" in a
literal spelling such as `powerOf it` beside `powerOf (that creature)`, and Kalitas and Broken
Visage would stay correct only because they are spelled with `getsPt it`. The c-command rationale
predicts coreference in Kalitas as well. What separates the two cards is whether a second
candidate exists, which is exactly what the preference reads.

## Change

- `resolveCandidates` (`Check/Words.lean:1250`): `C` is the `afterC` line of the
  `.word _ | .unionHalf _` arm (`:1273-1276`): `afterA.filter (fun b => !b.pronounResolved)`.
  Replace it with the preference: keep `afterA` when the filter would leave nothing. Rule `A`
  (clause-mates, when `inArgSlot`) is unchanged and applies first; the plural
  `preferOrdinary` step and `R1` (most recently introduced) follow. The `.bare | .atSlot` arm has
  no `C`.
- Add *Saw in Half* to `Cards/Anaphora.lean` with its Oracle wording ("its controller"), and
  amend `docs/decisions/semantics-v3.md` §1.5 Stage 2 ("Anti-coreference with same-clause
  pronoun") to say `C` is a preference.
- Keep the glossary untouched unless it states `C` as an exclusion (grep `pronounResolved` and
  "Anti-coreference" in `docs/contexts/game-model/CONTEXT.md`).

## Tests

- `demonstrativeRefusesWhenEveryCandidateExcluded` (`Proofs/DrtResolution.lean:65`): **rewritten**
  (it asserted `= []`, which pinned the exclusion option (2) replaces). Its candidate 1 is excluded
  only by `C`, and candidate 2 by `A`; it is renamed
  `demonstrativeKeepsPronounReferentWhenNoOtherCandidate` and asserts `= [1]`.
- `demonstrativeExcludesPronounReferentBeforeRecency` (`:60`): unchanged and still passes,
  because another candidate remains.
- `kalitasSelectsTokenThenDestroyedCreature` and `brokenVisageSelectsTokenThenDestroyedCreature`
  (`Proofs/DrtWitnesses.lean:23` and below), `pronoun_and_demonstrative_split` (`Check.lean:152`):
  unchanged, must pass.
- `okSawInHalf` (new): the card spelled in full, with `instruct (controllerOf it) (create 2
  (copyOf (that creature) ex) [])`, checks clean; the three "that creature" mentions all read the
  destroyed creature.
- `kalitasLiteralPossessorStillDisjoint` (new): the literal `powerOf it` beside `powerOf (that
  creature)` in Kalitas's shape still resolves "that creature" to the older creature, not the
  token (the pin that distinguishes option (2) from the rejected option (1)).
