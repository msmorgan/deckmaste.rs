---
needs: []
---
# "That source" has no Noun Word, so Deflecting Palm reads its own spell

Standard constraints apply. **[design]**: the glossary entry **Source** says "Source is not an
Object class", so the fix needs an owner ruling on its shape before any code. Everything under
"Proposed" is a proposal, not a ruling.

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

## Proposed fix

- Governing principle (owner, 2026-10-21): a Core AST distinction exists only where the CR or
  anaphora resolution needs it. Anaphora resolution filters by noun, so "that source" needs a
  noun to filter by. This qualifies.
- Add a `source` Noun Word (or an equivalent the owner prefers) so "that source" resolves as a
  demonstrative to the mention a "source of your choice" / "a source" introduces
  (`dealtBy (aYourChoice isSource)`, a `deals`-event agent), and nothing else.
- Because the glossary says a Source is a contextual relationship and not an Object class, the
  Noun Word must be a reading of that relationship (the damage-dealing Object in scope), not a
  new class. Ruling needed on whether to amend the **Source** entry in
  `docs/contexts/game-model/CONTEXT.md` (with the CR citation) or to name the word differently.
  Rename the identifier before rewording the entry.
- Rewrite *Deflecting Palm* to `controllerOf (that source)` and delete the `it` fallback reading.

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
