---
needs: [lean-v3-anaphoric-definite-description]
---
# A postposed "if" reads its clause's subject

Standard constraints apply.

## Problem (probed 2026-10-24)

*Joust*: "The creature you control gets +2/+1 until end of turn if it's a Knight." The "if"
follows its clause in English, so its "it" reads that clause's subject, "the creature you
control". It is evaluated first, though. `doIf` (`Check/Phrase.lean`) checks its condition
against the context from before the branch, plus only the branch's new targets
(`det == .target`). A branch that reads back an earlier mention announces no new target, so the
condition sees both earlier targets equally and "it" is ambiguous:

```lean
Action.check (DRS.ofList [])
  (sequentially
    [ choose (both (target (and [isCreature, youControl]))
                   (target (and [isCreature, youDontControl]))),
      doOnlyIf (get (the (and [isCreature, youControl])) (up 2) (up 1) untilEndOfTurn)
        (is it (hasSubtype knight)) ])
  -- [anaphor .bare .one 2]
```

## Witnesses

- *Joust* ("… if it's a Knight"), once `lean-v3-reciprocal-each-other` also lands for "Then those
  creatures fight each other".
- *Savage Punch* ("The creature you control gets +2/+2 until end of turn before it fights if you
  control a creature with power 4 or greater"), which also needs
  `lean-v3-spell-abilities-share-targets`.

## Design (proposed)

- A postposed condition is read after its clause: the branch's mentions are in its Reference
  Scope and ranked as the clause left them. This covers readbacks of earlier mentions and the
  subject's grammatical role, not only the targets the branch announces.
- The branch keeps the ids it takes now, so later lines read the same referents.
- A preposed "if" keeps its present reading.
- `doOnlyIf` and `doIf` both built `.doIf`, so Core did not record which side the "if" was
  written on. Settled by the owner ruling below.

## Owner ruling (2026-10-24)

Option A: add `postposed : Bool` (default `false`) to `Action.doIf`, set by the `doOnlyIf`
template. `Action.check` runs on the reduced Core term, so the order that affects anaphora must be
in Core.

- Even when postposed, the condition is checked at resolution, before the branch acts
  [CR#608.2c] (read with [CR#601.2c]).
- In the `.doIf` arm of the action stepper (`Check/Phrase.lean`), when `postposed = true`, the
  context the condition is stepped in also includes:
  1. new targets the branch announces (`det == .target`), as before;
  2. the branch's pre-event re-mentions of existing referents, with their updated roles, so that
     in *Joust* "the creature you control" ranks as the clause subject for "if it's a Knight".
- It must not expose fresh non-target referents (`det == .a`, created tokens, drawn cards) that
  the branch would only choose or create as it acts: `conditionReadsOnlyTheBranchsTargets`
  (`Proofs/DrtCards.lean`) keeps holding.
- A preposed "If it's a Knight, the creature you control …" is unchanged: the forward reference
  is still refused.

## Tests

- *Joust* through "if it's a Knight" in `Cards/Choice.lean`: the probe above checks `[]`, and the
  condition's "it" reads the first target.
- The preposed "If it's a Knight, the creature you control …" is still refused.
- The existing `doIf`/`doOnlyIf` pins in `Proofs/Description.lean` and `Proofs/Zone.lean` still
  hold.
