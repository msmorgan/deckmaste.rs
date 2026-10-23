---
needs: []
---
# Stratum 7: Quantifiers, reflexives, joint coordination and targeting scope

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Read:
- floating *each* and *both* (Kutzil, Malamet Exemplar; Angelic Aberration; Leonardo's Technique; Dance of the Manse; Ballroom Brawlers; Keen Duelist; Storyweave);
- reflexives and reciprocals (*with the same name as one another*, Chrome Replicator);
- singular joint coordination under *a* (*is also a Cleric, Rogue, Warrior, and Wizard*: Stonework Packbeast, Burakos, Tajuru Paragon, Veteran Adventurer; under a singular determiner Lush Growth and Wild Shape). The same rule also admits *becomes a [Human Soldier creature with power] and [toughness each equal to ...]* on about 100 units. That is a grammatical attachment alternative beside the endorsed Reading and stays admitted; semantics disambiguates (principle 1);
- *whichever comes first / is greater* (Thought Dissector, Willowdusk, Triumphant Chomp, Prophet of the Scarab, Squee's Revenge, where a frequency NP is coordinated with an *until* PP);
- the unlike coordination of an Amount and a Noun Phrase under a head that selects both (*equal to [2] or [the greatest power among Dinosaurs you control]*), each conjunct able to fill the slot alone (CGEL, Ch. 15 §3.1, p. 1323, rule [1]), Triumphant Chomp. The *[2 or the] greatest power* Reading is not this ticket's concern; semantics rejects it;
- *total power and toughness 5 or less* (Cut Down, Meek Attack): *total* modifies [power and toughness] as a whole;
- a determined measure nominal as a nominal conjunct (*your [choice of 5/1] or [1/5]*, Aquamorph Entity). Admit it beside the endorsed *choice of [5/1 or 1/5]*;
- the Targeting Marker over a coordination when the marked Noun Phrase sits inside a larger first conjunct (*[control of X target creatures] and/or [planeswalkers]*, *[each of up to two target creatures] and/or [planeswalkers]*): add the wide-scope reading (mark a conjunct whose right edge is a targeted Noun Phrase) alongside the narrow-scope reading, which stays admitted. Twelve units: Meteor Swarm; Nahiri's Wrath; Saheeli, the Sun's Brilliance; Mass Manipulation; Shatterskull Smashing; Ignite Disorder; Volcanic Salvo; Zimone, Paradox Sculptor; Captain's Maneuver; Spinning Wheel Kick; Etched Slith; Cloudspire Skycycle;
- the Targeting Marker determining a nominal that needs an overt determiner of its own: Cloud, Planet's Champion (*that target Cloud cost {2} less*): *target* as a determiner (`det:target`) of a proper Name. Check CGEL for whether that is ungrammatical; if it is clearly so, reject it by a general rule (no consumer exclusion), and if it is not clearly so, leave it admitted. Frodo, Determined Hero (*target [Equipment ... mana value 2] or [3]*) and Eternal Flame (*[opponent or planeswalker and half] X damage*): make sure the endorsed Reading is admitted. Suppress no grammatical alternative.

## Grounding

- CGEL, Ch. 5 §9, p. 413 (quantificational adjuncts).
- CGEL, Ch. 17 §4, p. 1499 (reciprocals).
- CGEL, Ch. 5 §18.4, p. 508 (joint coordination).
- CGEL, Ch. 5 §7.14, p. 398 (*whichever*).
- CGEL, Ch. 15 §3.1, p. 1323 [1].

## Witness cards

- Kutzil, Malamet Exemplar
- Chrome Replicator
- Stonework Packbeast
- Lush Growth
- Triumphant Chomp
- Cut Down
- Aquamorph Entity
- Meteor Swarm
- Cloud, Planet's Champion

## Estimated faces

About 24 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar only.

## Pending judgments this slice makes hold

`trailing_adjunct_height.txt:12,14`; `up_to_that_many.txt:4`; `property_pair_each_equal_to.txt:4`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

## Findings

- Kutzil, Malamet Exemplar stays deferred: its Oracle text and printed card (LCI 0232) both read *one or more creatures … deals*, a subject–verb agreement error no English grammar admits. With *deal* it reads.
- The wide-scope targeting reading, *[your choice of 5/1] or [1/5]* and the *target Cloud* reading needed no change: the twelve units already carry both bracketings, Aquamorph Entity already has the nominal-conjunct Reading, and Cloud has no `det:target` Reading. Frodo and Eternal Flame keep their endorsed Readings.
- Engine: a preposition selecting both an Amount and a Noun Phrase derives an `AmountOrNP` frame (Triumphant Chomp), and a Production may require a `coordinated` child (a trailing Adjunct of a Clause coordination, `trailing_adjunct_height.txt:12,14`). That one costs 4,201 Readings on 281 units.
- Plural *you* under a floating quantifier (Keen Duelist, Juxtapose) also admits floated *both* in *you both own and control*, which doubles Remove Enchantments (9,384 → 18,768).

1. **Census delta** (measured on `zuqmzkks`): faces read 32,667 → 32,687; total Readings 314,400 → 333,708; lost faces: none.
2. **Judgment set**: 3,153 enforced at `COMPLETED_STRATUM` 6, all hold; 285 pending.
