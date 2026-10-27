---
needs: [lean-v3-spell-abilities-share-targets, plugins-v3-builtin-primitives-and-templates]
---
# A later spell ability's "instead" replaces an earlier spell ability's effect

Standard constraints apply.

## Problem

A self-replacement effect replaces part or all of its own spell's effect [CR#614.15]. "The text
creating a self-replacement effect is usually part of the ability whose effect is being replaced,
but the text can be a separate ability, particularly when preceded by an ability word"
[CR#614.15]. The word "instead" marks it as a replacement effect [CR#614.1a].

The checker cannot spell that separate-ability form. `Action.replace replaced replacement`
(`lean/Semantics/Abilities.lean`, "the spell's or ability's own `replaced` action") holds the
replaced action inside itself. So the replaced action and the replacement have to be one
`Action`, inside one spell ability. Each paragraph is its own ability [CR#113.2c], and a
`Replacement` (`instead`, `Replacement.doIf`) is not an `Action`, so a later spell ability has no
way to name the earlier paragraph's effect as the event it replaces. Folding both paragraphs into
one `Action.replace` would erase the ability boundary and the ability word. Writing the later
paragraph as a second action without "instead" would change what the card does.

`lean-v3-spell-abilities-share-targets` already lets the later paragraph read the earlier one's
targets ("that creature", "that spell"). What is missing is the replacement itself.

## Witnesses

- *Fatal Push*: "Destroy target creature if it has mana value 2 or less. / Revolt — Destroy that
  creature if it has mana value 4 or less instead if a permanent left the battlefield under your
  control this turn."
- *Stubborn Denial*: "Counter target noncreature spell unless its controller pays {1}. /
  Ferocious — If you control a creature with power 4 or greater, counter that spell instead."
- *Hunger of the Howlpack*: "Put a +1/+1 counter on target creature. / Morbid — Put three +1/+1
  counters on that creature instead if a creature died this turn."
- *Kamahl's Sledge*: "Kamahl's Sledge deals 4 damage to target creature. / Threshold — If there are
  seven or more cards in your graveyard, instead Kamahl's Sledge deals 4 damage to that creature and
  4 damage to that creature's controller."
- *Bring the Ending*: "Counter target spell unless its controller pays {2}. / Corrupted — Counter
  that spell instead if its controller has three or more poison counters."

## Ruling (owner, 2026-10-24)

Approved. Per [CR#113.2c,614.15], a self-replacement printed as its own paragraph (typically
after an ability word) is a separate spell ability whose body is a `Replacement`.

## Change

- Add `Ability.spellReplacement (replacement : Replacement)` with template `spellReplacement`
  (beside `spell`, `Templates.lean:2101`). It replaces the effect of the immediately preceding
  spell ability of the same spell.
- The checker steps it as the `.replace` arm of `ops.inst` does (`Check/Phrase.lean`, the
  `stepReplacementWith` path): from the state the replaced spell ability read, per
  `lean-v3-self-replacement-remainder`, plus the targets that earlier spell abilities of the same
  spell published.
- It refuses a `spellReplacement` that has no preceding spell ability to replace. "Preceding" is
  immediate, so a witness's later paragraph replaces exactly the paragraph before it.

## Tests

- Each witness spelled in full, with the later paragraph's "instead" as a self-replacement of the
  earlier paragraph's effect.
- A refusal pin: a `spellReplacement` with no preceding spell ability to replace (first ability of
  the card, or preceded only by a non-spell ability).
- A pin that the replacement reads the earlier paragraph's published targets ("that creature",
  "that spell").
