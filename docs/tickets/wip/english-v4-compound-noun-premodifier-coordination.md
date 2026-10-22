---
needs: []
---
# *artifact or non-Aura enchantment card* is [*artifact or [non-Aura enchantment]*] *card*

The grammar has no Reading for a premodifier that is itself a premodified noun
when it stands as a conjunct of a coordination of noun Premodifiers. In
*artifact or non-Aura enchantment card*, the carrier *card* distributes over
two coordinates: *artifact* and *non-Aura enchantment*. The intended analysis
is therefore [*artifact or [non-Aura enchantment]*] *card*: an artifact card,
or an enchantment card that isn't an Aura.

A `NounPremodifier` today is a single noun (`AttributiveNoun`), with or without
a prefix (`PrefixedNounPremodifier`). That leaves only two Readings, and both
are wrong:

- [*artifact*] or [*non-Aura enchantment card*]: a bare *artifact* names a
  permanent (`docs/oracle-style-guide.md`, shortest useful checklist, item 6).
  That conjunct would pair a permanent with a graveyard card.
- [*artifact or non-Aura*] *enchantment card*: this is an enchantment card in
  either case, so it leaves out artifact cards that aren't enchantments.

## Witnesses (only wrong Readings today)

- **Abuelo's Awakening:** *Return target artifact or non-Aura enchantment card
  from your graveyard to the battlefield with X additional +1/+1 counters on
  it.* The unit has 15 Readings. `+NounPhrase target artifact or non-Aura
  enchantment card` has 2, the two above.
- **Excava, the Risen Past:** *… return up to one target artifact, creature, or
  non-Aura enchantment card with mana value 3 or less from your graveyard …*
  The unit has 42 Readings. The Noun Phrase has 3: the serial split twice
  (the postmodifier on *card* or on the whole coordination) and [*artifact,
  creature, or non-Aura*] *enchantment card*.

Authority: `docs/oracle-style-guide.md` §7, "Card types are common nouns and
attributive modifiers" (*A final carrier noun may distribute over the whole
coordinated phrase*) and "Negation with non-" (*non-Aura enchantment*). The
withdrawn `english-v4-premodifier-coordination-split` records that the
Nominal split stays as an Ambiguity wherever it is meaning-bearing, as in
Puppet Crafting's *Enchant artifact or non-Aura enchantment*. This ticket adds
the missing analysis. It removes nothing on its own authority.

## Change

- Let a conjunct of a noun Premodifier coordination be a premodified noun (a
  compound nominal premodifier, *non-Aura enchantment*), so that
  [*artifact or [non-Aura enchantment]*] *card* has a Reading.
- Check whether that Reading makes either current wrong Reading removable by a
  tightened rule. If it does not, they stay: the Ranking Key orders them.
- Measure what the new Premodifier shape admits elsewhere on the corpus.
  Adjudicate every unit that gains a Reading. A newly admitted wrong Reading
  is a defect.

## Done when

- Abuelo's Awakening and Excava each have the intended Reading, pinned by a `+`
  Judgment on the real text.
- No face is lost. Every unit that gains a Reading is listed, and its new
  Reading is adjudicated.

## Constraint: no new features in `english.ron`

The fix adds **zero new features to `english.ron`**. It uses existing
features, lexicon or catalog frames, or an engine mechanism, and names no word
in the grammar. If it cannot be done that way, STOP and return to the owner.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
