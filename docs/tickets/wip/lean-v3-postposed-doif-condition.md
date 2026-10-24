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
- Open question: `doOnlyIf` and `doIf` both build `.doIf`, so Core does not record which side the
  "if" was written on. Decide whether to record it on `.doIf` or to read every condition against
  what its branch announces.

## Tests

- *Joust* through "if it's a Knight" in `Cards/Choice.lean`: the probe above checks `[]`, and the
  condition's "it" reads the first target.
- The existing `doIf`/`doOnlyIf` pins in `Proofs/Description.lean` and `Proofs/Zone.lean` still
  hold.
