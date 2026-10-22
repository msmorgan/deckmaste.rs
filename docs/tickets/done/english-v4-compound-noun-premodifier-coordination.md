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

## Resolution

**Owner's rulings applied.** Round 6 (2026-10-22): proceed with this ticket,
re-measuring first, and allow rises only with the intended Reading. The
withdrawn `english-v4-premodifier-coordination-split` ruling keeps [*X*] or
[*Y Z*] against [*X or Y*] *Z* as a genuine Ambiguity, so nothing is removed.
Round 5, 7a: a fronted dependent scopes over a whole coordination, never its
first conjunct.

**Re-measured** on `vuurrwws` (before the change): the Noun Phrases have 2 and
3 Readings, all wrong, as stated above. The units have 15 and 42.

**Change** (`english.ron`, no new feature):

- The `NounPremodifier` Coordination takes its conjuncts from a new
  `NounPremodifierConjunct` Category. `NounConjunct` (cost 0, so existing
  Ranking Keys are unchanged) makes any noun premodifier a conjunct.
- A conjunct may take a preposed negative premodifier of its own through an
  `Adjunct` schema: host `NounPremodifierConjunct`, adjunct
  `NegativeNounPremodifier`, side `Before`. That yields [*artifact or
  [non-Aura enchantment]*] *card*.
- The schema is preposed, so the engine's existing first-conjunct rule
  (`Summary::fronted`; `english-lexical-analysis.md` §2) keeps the negative
  premodifier at the start of the first conjunct scoping over the whole
  coordination: *non-Lesson [instant and sorcery] card* (Iroh, Grand Lotus)
  gains no [[*non-Lesson instant*] *and sorcery*] *card*.

**Measured alternative, rejected.** A first prototype let any conjunct be a
compound of two noun premodifiers. 25 units rose; 23 of them gained only
wrong Readings. Examples are [*Pegasus, Unicorn, or [Horse creature]*]
*spell* (Thurid), *creature [[card or [Garruk planeswalker]] card]* (Garruk's
Harbinger, 152 → 344), [[*noncreature artifact*] *or noncreature*]
*enchantment* (Guerrilla Gorilla, Haywire Mite, Hulk's Thunderclap) and
[[*non-Lesson instant*] *and sorcery*] *card* (Iroh). Restricting the
conjunct's own premodifier to a negative one (`docs/oracle-style-guide.md` §7,
Negation with non-), outside the first conjunct, admits the two witnesses and
nothing else.

**The wrong Readings stay.** [*artifact or non-Aura*] *enchantment card* has
the same structure as the endorsed [*instant or sorcery*] *card*, and the
split is the owner's genuine Ambiguity. No tightened structural rule removes
either, so the Ranking Key orders them. Today it ranks the intended Reading
second for Abuelo's Awakening (0/1/9, behind 0/0/9) and third for Excava.
Puppet Crafting keeps its split Reading, pinned by a judgment.

**Units that gain a Reading** (adjudicated):

- Abuelo's Awakening, 15 → 21. The Noun Phrase gains exactly one Reading,
  the intended one; the rest is the unit's other ambiguity multiplied by it.
- Excava, the Risen Past, 42 → 57. Likewise: one Noun Phrase Reading,
  [*artifact, creature, or [non-Aura enchantment]*] *card*.

No other unit changed.

1. **Census delta** (`vuurrwws` → this landing, full corpus): faces read
   28,799 → 28,799; total Readings (product) 214,300 → 214,321; Surplus
   88,560 → 88,581. Only the two witnesses rose, and none fell. Cheapest
   tier: singleton in 14,800 of 16,562 ambiguous units, summed 18,929,
   largest 8, unchanged. Lost faces: none.
2. **Judgment set** (`vuurrwws`): 3,033 enforced, 207 pending; all hold.
