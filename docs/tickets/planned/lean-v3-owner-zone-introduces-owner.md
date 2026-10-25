---
needs: [lean-v3-checker-and-drt-fixes]
---
# "Its owner's hand" introduces the owner

**Design: proposed.**

A move to "its owner's hand" (`returnToHand`, `hand` = `.zone .hand none`) introduces no player:
the destination is scoped to nobody and a card goes to its owner's zone by rule [CR#400.3]. So a
later "that player" that reads the owner has no antecedent, and this family is unwritable:

- *Recoil*: "Return target permanent to its owner's hand. Then that player discards a card."
- *Dinrova Horror*: "… return target permanent to its owner's hand, then that player discards a
  card."
- *Ashiok, Nightmare Muse* −3: "Return target nonland permanent to its owner's hand, then that
  player exiles a card from their hand."
- *Compelling Deterrence*: "Return target nonland permanent to its owner's hand. Then that player
  discards a card if you control a Zombie."
- *Cut the Tethers*: "For each Spirit, return it to its owner's hand unless that player pays {3}."
- *Firestorm Phoenix*: "If this creature would die, return it to its owner's hand instead. Until
  that player's next turn, that player plays with that card revealed in their hand and can't play
  it."

Review probe (2026-10-24), *Recoil*:

```lean
#eval Action.check (DRS.ofList [])
  (sequentially [returnToHand (target isPermanent), instruct (that .player) (discard (a isCard))])
-- [Refusal.anaphor (Reach.word NounWord.player) Plurality.one 0]
```

Proposal: the owner's zones written "its owner's <zone>" introduce the owner, by stepping the
destination as `handOf (ownerOf it)` (or giving `returnToHand` and its siblings an owner
possessor read after the subject), so "that player" reads the owner and each card above checks.
The move already steps its subject before its destination, so a "that player" inside the subject
still reads an earlier player: *Noetic Scales*' pin
(`Proofs/CorpusRecency.lean` `noeticScalesRawDemonstrativeSelectsTheUpkeepPlayer`) already
checks the owner-written spelling. Settle, before implementing: whether the owner is introduced
for every move to an owned zone or only where the text writes "its owner's", and how a plural
("return all creatures to their owners' hands") introduces the owners.
