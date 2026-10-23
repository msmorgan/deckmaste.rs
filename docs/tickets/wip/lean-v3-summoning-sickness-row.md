---
needs: [lean-v3-card-type-conferral-rows]
---
# The summoning-sickness rule goes on the creature row, and haste gets its body

Standard constraints apply. Lean first (`lean/Semantics/`). Split out of
`lean-v3-card-type-conferral-rows`, which wrote the rest of the creature row.

## Problem

[CR#302.6]: "A creature's activated ability with the tap symbol or the untap symbol in its
activation cost can't be activated unless the creature has been under its controller's control
continuously since their most recent turn began. A creature can't attack unless it has been under
its controller's control continuously since their most recent turn began." Haste's exemption is
stated against the same reading [CR#702.10b,702.10c]. None of it can be written today:

- `TurnRef` (`Abilities.lean:161`) is `current | previous | next | each | later | that`. None
  names "player P's most recent turn", which is the current turn only while it is P's and
  otherwise P's previous one. `.beginningOf .turn (some controller) .current` would be a
  known-wrong reading.
- No condition or predicate reads "has been under its controller's control continuously since"
  an event.
- Haste is a bodiless keyword (`Definitions/Keywords.lean`, `haste`), so its exemption
  [CR#702.10b,702.10c] has no home.

## Design (owner, 2026-10-23: option (a))

1. Add a `TurnRef` member for "`player`'s most recent turn" (the current turn if it is theirs,
   else their last), with its citation [CR#302.6], and thread it through `stepTurnRefWith`
   (`Check/Phrase.lean` ~929) and every other exhaustive read.
2. Add a reading for "has been under its controller's control continuously since <event>",
   composed from existing primitives where it can be (ADR §0.1 item 5(b)); the event here is the
   beginning of the controller's most recent turn.
3. Add the [CR#302.6] conferral to `creatureType` (`Definitions/CardTypeRules.lean`): the attack
   restriction and the {T}/{Q} activation restriction, each unless that reading holds.
4. Give `haste` a body that exempts its creature from both restrictions [CR#702.10b,702.10c].

**Rejected: option (b)**, re-citing the `combatant` conferral as carrying [CR#302.6] (its
docstring and the glossary's Combatant say the rule "applies to" a Combatant). It leaves a
decomposable game rule inside the engine, against ADR `semantics-v3.md` §0.1 item 4 (every
decomposable game rule is data in the DSL).

Touches `Abilities.lean` and `Check/Phrase.lean` (~929), checker territory: coordinate with any
landing that owns the checker files.

## Tests

- The creature row's summoning-sickness conferral is on the row and checks, cited [CR#302.6].
- Haste's body checks, cited [CR#702.10b,702.10c].
- The new `TurnRef` checks with a player and is refused without one. Which turn it denotes at
  runtime is the engine's (ADR §0.1 item 4).
