---
needs: [lean-v3-anaphoric-definite-description]
---
# A spell's later spell abilities read its earlier targets

Standard constraints apply.

## Problem (probed 2026-10-24)

Each paragraph break in a card's text marks a separate ability [CR#113.2c], so *Beastie
Beatdown*'s three paragraphs are three spell abilities. They are spell abilities of one spell,
though. The spell's targets are announced as it is cast, for the spell as a whole [CR#601.2c].
Its controller then follows its instructions in the order written, reading the whole text by the
rules of English [CR#608.2c]. So a later spell ability may read back the targets an earlier one
announced ("Tap target creature. / Metalcraft — … exile that creature.").

The checker gives a later paragraph none of them. The `.spell` arm of `Ability.textContextWith`
(`Check/Phrase.lean`) passes on only the spell's text choices (`publishTextChoices`), so no
Reference in a later spell ability reaches an earlier one's target:

```lean
Ability.checkText (DRS.ofList []) [spell (tap (target isCreature)), spell (untap (that creature))]
  -- [anaphor (.word (.type creature)) .one 0]
Ability.checkText (DRS.ofList []) [spell (tap (target isCreature)), delirium (spell (untap it))]
  -- [anaphor .bare .one 0]
```

*Beastie Beatdown*, spelled as three spell abilities, is refused with `[uniquifying, uniquifying,
uniquifying]`. The same instructions as one `sequentially` action check `[]`, but that erases
the ability boundaries [CR#113.2c] and the ability word on the second paragraph.

## Witnesses

- *Beastie Beatdown*, in full: "Choose target creature you control and target creature an
  opponent controls. / Delirium — If there are four or more card types among cards in your
  graveyard, put two +1/+1 counters on the creature you control. / The creature you control deals
  damage equal to its power to the creature an opponent controls."
- "that creature" after an ability word: *Dispatch* (Metalcraft), *Fatal Push* (Revolt),
  *Efflorescence* (Infusion), *Hunger of the Howlpack* (Morbid), *Kamahl's Sledge* (Threshold).
- "that creature's controller": *Ravaging Blaze* (Spell mastery), *Searing Barrage* (Adamant),
  *Foolish Fate* (Infusion).
- "that spell": *Stubborn Denial* (Ferocious), *Bring the Ending* (Corrupted).
- "the creature you control": *Savage Punch* (Ferocious; its "if" is also postposed,
  `lean-v3-postposed-doif-condition`).

## Design

There is no owner ruling. The design follows from the CR text: [CR#113.2c] makes each paragraph
its own ability, [CR#601.2c] announces the targets for the spell as a whole, and [CR#608.2c] has
the spell's instructions followed in the order written, read as one text.

- The `.spell` arm of `Ability.textContextWith` passes the spell's announced targets on to the
  later spell abilities of the same spell, as ordinary mentions in their Reference Scope. Every
  Reference kind reads them as it reads any earlier mention, not only Participial Descriptions.
  They are passed on as the earlier ability's instructions leave them, as they would stand in one
  run of instructions.
- Only targets pass on. [CR#601.2c] is what makes a target the spell's rather than one
  paragraph's. [CR#608.2c] says how the text is read, but it does not plainly say that a
  paragraph's other mentions (an amount, "a creature" chosen on resolution) reach a later
  ability. Its examples are all within one paragraph. The existing pin
  `cardTextDoesNotExportOrdinaryObjectMentions` keeps those mentions inside their own ability.
- The targets reach only later spell abilities. A triggered, activated or static ability of the
  same text is not part of the spell's resolution [CR#113.3a], so it does not read them.
- Non-spell abilities are unchanged: an activated or triggered ability still publishes to later
  abilities only what a Participial Description reads [CR#607.1,607.2a].

## Tests

- *Beastie Beatdown* spelled in full in `Cards/Choice.lean`.
- *Dispatch* spelled in full. *Efflorescence* and *Foolish Fate* are spelled beside it.
- A pin that a triggered or activated ability's target is still not an ordinary antecedent in a
  later ability.

## Witnesses moved

Two witnesses need constructs this ticket does not add, so they moved to new tickets:

- *Fatal Push*, together with *Stubborn Denial*, *Hunger of the Howlpack*, *Kamahl's Sledge* and
  *Bring the Ending*, moved to `lean-v3-cross-ability-self-replacement`. In each, the later
  paragraph's "instead" replaces the earlier paragraph's effect.
- *Searing Barrage* moved to `lean-v3-colored-mana-spent-count`, because its Adamant condition
  counts the red mana spent.
