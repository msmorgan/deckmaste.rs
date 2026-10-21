---
needs: []
---
# Find and remove the source of look-alike duplicate Readings

This follows
[`english-v4-byte-identical-duplicate-readings`](../done/english-v4-byte-identical-duplicate-readings.md).
That ticket removed Readings that printed identically, and fixed each one at
its source rather than by a dedup pass. A second kind of copy remains: Readings
with identical Constituents and identical Productions that differ only in
feature values the Reading does not project. Each such group is one tree, yet
the census counts every member. Identical trees should not be emitted as
separate Readings.

## Evidence

A scratch probe was run during `english-v4-wrong-judgments-and-np-and-dup`. Its
evidence file is `w1.txt`, in the `english-v4-wrong-judgments-and-np-and-dup/`
folder of the session scratchpad; the relevant lines are those marked
`STRICT`. The probe covered 3,942 Corpus Units that contain a Noun Phrase
*and*-coordination candidate, each with at most 4,096 Derivations. It grouped
each unit's Readings by Constituents (category, span, function and parent) and
by Productions. In that grouping, the numbered Noun Phrase *and*-coordination
Schemas (`NounPhrase#And`, `#And3`, `#AndOr3`) count as one.

- **304 groups** of two or more Readings. They hold **360 surplus copies**
  across **36 units**.
- Those 36 units hold 3,297 Readings and a Surplus of 624 on the current line.
- Almost all groups run through a Noun Phrase *and*-coordination: 258 through
  the binary `Coordination:NounPhrase#And`. The rest are serial and *and/or*
  shapes.
- Examples:
  - Brighthearth Banneret, *Elemental spells and Warrior spells you cast cost
    {1} less to cast.*: one group of 4.
  - Ballyrush Banneret, *Kithkin spells and Soldier spells you cast …*: one
    group of 4.
  - The same holds for Stonybrook Banneret, Herald of War and Cid, Freeflier
    Pilot.
  - The largest are Intellect Devourer (65 copies), Wild Magic Surge (60) and
    Radiant Performer (36).

The likely cause is that two or more of `english.ron`'s Noun Phrase *and*
coordinations match the same conjunct pair, leaving features the Reading does
not project unvalued in different ways: `Targeting`,
`DeterminativeDistribution` and the resolved `Number`. That is not yet
confirmed. `english-v4-wrong-judgments-and-np-and-dup` item 3 covers one
suspected overlap, under *each*. Start from its record if it has landed.

## Change

- **Measure the whole corpus.** Count units whose Readings include two or more
  with identical Constituents and Productions, and the surplus copies.
  - Report the Surplus at stake per Segment. The probe above measured whole
    units, and only *and*-coordination candidates.
- **Find each cause and remove it at the source.** Examples are coordination
  Schemas whose equations overlap, or a feature left unvalued where it should
  be valued or not carried. A dedup pass over finished Readings does not count
  as a fix, and neither does a Reading Normal Form comparison.
- **Keep genuine distinctions.** Where the differing feature values are a real
  grammatical distinction, keep both, as the precedent kept syncretic Word
  Forms. Make the Reading project the distinction so the two no longer look
  alike, and record which case each cause was.

## Done when

- No unit has two Readings with identical Constituents and Productions unless
  the record names the distinction they keep.
- No face is lost, and every Reading removed is a copy of one that remains.
- Brighthearth Banneret and Ballyrush Banneret each have one Reading per
  distinct analysis, pinned by `+` Judgments on their real text.

## Constraint: no new features in `english.ron`

The fix adds **zero new features to `english.ron`**. It uses existing
features, lexicon or catalog frames, or an engine mechanism, and names no word
in the grammar. If it cannot be done that way, STOP and return to the owner.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
