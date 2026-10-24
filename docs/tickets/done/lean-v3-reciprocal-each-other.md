---
needs: [lean-v3-choose-over-coordination]
---
# Reciprocal "each other" and "the other"

Standard constraints apply.

## Problem (probed 2026-10-23)

[CR#701.14a]: "A spell or ability may instruct a creature to fight another creature or it may
instruct two creatures to fight each other. Each of those creatures deals damage equal to its
power to the other creature." The `fight` Definition (`Definitions/Actions.lean`) has only the
first form: its actor fights `other`. The model has no reciprocal: the only reflexive noun
phrase is `themselves` (`.refl`). The done `workbench-coordination-family` ticket already lists
"the reciprocal fight node" as an open gap ("eleven printed lines"), and no open ticket owns it.

The distributive form does not check either. After choosing two creatures,
`instruct (eachOf (those creature)) (dealDamage (toughnessOf it) (theOther .object))` is refused
with `theRestFits .object`.

## Witnesses

- "Then those creatures fight each other": *Blizzard Brawl*, *Joust*, *Tail Swipe*, *Malamet
  Battle Glyph*. *Duel for Dominance* writes "the chosen creatures fight each other" (also
  `lean-v3-chosen-participial-anchor`).
- "Each of those creatures deals damage equal to its toughness to the other": *Grim Contest*.

## Design (proposed)

- A reciprocal argument read against a plural subject of exactly two members: "those creatures
  fight each other" means the two-creature form of [CR#701.14a]. It is not each creature
  fighting the other in turn, which would deal the damage twice.
- "the other", inside the scope of "each of those …", reads the group member that is not the
  current one.

Keep the self-fight rule [CR#701.14c] in view: two targets can be one object only when they come
from different instances of "target" [CR#601.2c]. "Target creature you control and target
creature you don't control" can never pick one creature twice.

## Owner ruling (2026-10-24)

- No new Core AST constructor.
- Option B: "the other" is the existing `theOther .object` (`.theRest .object .one`), as the
  printed "to the other" has no head noun. `theOtherOk` (`Check/Words.lean`) accepts it when the
  clause's distributive subject (`distributiveSubject`) is the current member of a group of
  exactly two, and it reads that member's counterpart with the group's Card Types and zone
  (`DRS.distributivePair?`, read through `restGroup` by `zoneOfGroup`/`tyOfGroup`). The `eachOf`
  step records the group its member ranges over (`Referent.memberOf`).
- The anaphoric definite description's rule is untouched: `the (and [isCreature, isOther])`
  admits no mention by its "other", and stays refused `uniquifying`.
- "Fight each other" is an Action Definition that expands to [CR#701.14a]'s own sentence,
  `instruct (eachOf <pair>) (dealDamage (powerOf it) (theOther .object))`, under fight's
  `ownScope`/`enact (.action "fight" true)` header and its [CR#701.14b] guard.
- No separate [CR#701.14c] branch: the pair has one member per target mention, so if both slots
  name one creature, each slot deals its power to "the other" and the creature is dealt twice its
  power.

## Landed

- `fightEachOther (pair : NounPhrase)` in `Definitions/Actions.lean`, a Definition of its own
  rather than an optional argument on `fight`: the two forms take different arguments (an actor
  and another creature, against a pair with no actor), so one optional parameter would make
  `fight` read differently by arity, and `fight`'s binding identities and pins would change. Both
  Definitions generate the same deed (`fightEachOther.deed = fight.deed`), so a fight lookback or
  trigger reads either.
- Grim Contest, Blizzard Brawl, Tail Swipe and Malamet Battle Glyph are spelled whole in
  `Cards/Choice.lean` (each a `Spelled` card).
- *Duel for Dominance*: only its first sentence reaches (`duelForDominanceOpening`). "Put a +1/+1
  counter on the chosen creature you control" and "the chosen creatures fight each other" wait on
  `lean-v3-chosen-participial-anchor`. Once that ticket lands, the fight is
  `fightEachOther (<the chosen creatures>)`.
- *Joust* waits on `lean-v3-postposed-doif-condition` ("The creature you control gets +2/+1
  until end of turn if it's a Knight"); its "Then those creatures fight each other" is
  `fightEachOther (those creature)`.
- A counted pair ("Choose two target creatures that share no creature types. Those creatures
  fight each other": *Rivals' Duel*) is one referent with no recorded count, so it is no pair
  yet: `lean-v3-counted-pair-each-other`.

## Tests

- *Joust* and *Blizzard Brawl* bench items through "those creatures fight each other", and
  *Grim Contest* whole.
- A refusal pin: "each other" with a singular subject.
