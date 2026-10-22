---
needs: []
---
# "That source" has no way to be written, so Deflecting Palm reads its own spell

Standard constraints apply. Owner notes (2026-10-21):
- A Noun Word is not an Object class. `player` and `join` are Noun Words too, and players are not
  Objects, so nothing here conflicts with the glossary's **Source** entry ("Source is not an
  Object class").
- Owner's suggested direction, not yet a ruling: "source" may just be a Template over a generic
  Noun Word (`any`, `object` or similar).

## Defect (verified 2026-10-21)

*Deflecting Palm* (`lean/Semantics/Cards/Damage.lean:492-501`): "The next time a source of your
choice would deal damage to you this turn, prevent that damage. If damage is prevented this way,
Deflecting Palm deals that much damage to **that source's controller**." The card writes
`controllerOf it`. With the chosen source never named as a mention, `it` falls back to the
Source, which is the spell: `Action.check (DRS.ofList []) (instruct this_ (dealDamage (.lit 1)
(controllerOf it)))` is clean with nothing in scope. So the card checks clean and reads "the
spell's controller", which is the caster, who is the damage's recipient, not the chosen source's
controller. Probed alternatives, all refused:

| Written | Refusal |
|---|---|
| `controllerOf (that permanent)` | `anaphor (word permanent) one` |
| `controllerOf (that card)` | `anaphor (word card) one` |
| `controllerOf (that creature)` | `anaphor (word creature) one` |
| `controllerOf (that player)` | `possessorKind controller player` |

No `NounWord` (`Words.lean:310`) means "source". The chosen source can be a permanent, a spell
on the stack, an object an effect refers to, or a face-up object in the command zone
([CR#609.7a]), so `permanent`, `card` and `spell` each name only some of what "that source"
denotes.

## Cards affected

Authored: *Deflecting Palm*. Not yet authored, same blocker (each prints "that source" or
"that source's controller" back to a chosen or event-named source [CR#609.7]): *Jade Monolith*,
*Reflect Damage*, *Eye for an Eye*, *Desperate Gambit*, *New Way Forward*, and the "whenever a
source deals damage to this creature, that source's controller …" family (*Phyrexian
Obliterator*, *Reaper of Sheoldred*, *Belltower Sphinx*, *Archfiend of Spite*, *Crag Saurian*,
*Rona, Herald of Invasion*, *Elesh Norn // The Argent Etchings*, *Flameblade Angel*).

## Proposed fix (direction from the owner; shape to confirm)

- Governing principle (owner, 2026-10-21): printed wording lives on Templates, and a Core AST
  distinction exists only where the CR or anaphora resolution needs it.
- "Source" is wording. Add a `thatSource` Template (spelling "that source") that expands to a
  demonstrative over a generic Noun Word. That word filters only by kind (object), not by card
  type or zone, because a source can be a permanent, a spell, an object an effect refers to, or
  a face-up command-zone object [CR#609.7a].
- The one Core addition is that generic Noun Word, e.g. `NounWord.object`. `lean/CONTRACTS.md`
  (the `copy` row) says "There is no generic object word", so the landing amends that line.
  Check first whether an existing head (`join`, or a kind-only demonstrative) already does
  this job.
- Check the resolution: "that source" in *Deflecting Palm* must reach the chosen source
  (`dealtBy (aYourChoice isSource)`), not the spell's handed-off `this_` or "you". If plain
  demonstrative recency doesn't give that, STOP rather than adding a source-specific rule.
- Rewrite *Deflecting Palm* to `controllerOf thatSource` and drop the `it` fallback reading.

## Tests to add

- *Deflecting Palm*: the Contrast-Anchor-style pin that the referent `controllerOf (that source)`
  reads is the chosen source's mention, not the Source (the spell).
- A refusal pin that `that source` with no source mentioned in scope is refused, not read as the
  spell.
- A positive pin for the event-agent form ("Whenever a source deals damage to this creature,
  that source's controller …") on one of the unauthored cards, authored as a bench item.
- Retire nothing; if an existing pin asserts `controllerOf it` on *Deflecting Palm*, rewrite it to
  assert the chosen source and say so.

Gates: `./lean/scripts/build`, `cargo xtask lean check`, and the CR citation gates.
