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

## Record

- Corpus-wide, before: 1,208 units held Readings with identical Constituents
  (category, span, function, parent) and Productions: 11,579 groups, 14,061
  surplus copies. Per Segment, 1,239 Segments held them, with a Surplus at
  stake of 6,609. Compared with their leaves too, no two Readings of any unit
  are alike (0 groups). Every look-alike pair differs in a leaf's Word Form or
  in its Lexeme, never in features alone.
- The suspected cause was wrong. The probe behind this ticket compared only
  some Constituents. A Banneret's four copies are its present and preterite
  *cast* times its present and preterite *cost*. No Noun Phrase *and*
  coordination leaves a feature unvalued that splits a Reading.
- Two causes were copies, now removed at their source:
  - *more* in a frequency comparison (*more than once*, *no more than twice*)
    read as both the count and the mass Lexeme: 77 of Surplus at stake in 31
    units. Its standard counts occasions (CGEL, Ch. 8 §9, p. 715), so it is
    count *many*'s comparative (Ch. 13 §4.1, p. 1123). `ComparativeAdverb`
    now requires a count quantifier.
  - The two faces of one card shortened to one spelling (*Ral*, *Sorin*,
    *Polukranos*). A face's scope held every face's name, so *Ral* was both
    faces' Lexeme: 198 at stake in 89 units. A face's scope is now its own name
    and its card's (`corpus.rs`; the Shortened Name glossary entry says so).
- What remains keeps a distinction:
  - Syncretic Word Forms, the precedent's case: present and preterite *cast*,
    *put*, *cost* and *bid*; singular and plural *Equipment*, *Myr*,
    *Moonfolk* and *d6*. That is 6,173 at stake in 1,023 units. Wrong
    preterites are ticketed: the matrix Declarative in
    `english-v4-case-solve-condition`, and the others in
    `english-v4-coordinated-clause-tense` and
    `english-v4-temporal-adjunct-tense-harmony`.
  - A card's own Shortened Name that is also another card's full name
    (*Ayesha Tanaka*, *Nicol Bolas*, *Squall*), or an Introduced Name or Anchor
    Word (*Wirefly*, *Hornet*, *Believe*). That is 148 at stake in 57 units.
    The two Lexemes name different objects. Full names are in scope everywhere
    (`english-lexical-analysis.md` §1).
  - *more* before *{E}* (*one or more {E}*): 13 at stake in 8 units. Oracle
    both counts *{E}* (*that many {E}*) and measures it (*the amount of {E}
    paid this way*). Declaring *{E}* count lost Wheel of Potential's only
    Reading, so nothing fixes the Countability, and the Reading keeps the
    Lexeme.
- Every Reading removed is a copy of one that remains. In each of the 92,923
  units parsed, the set of analyses (Constituents and Productions) is the same
  before and after. 122 units lost 730 trees between them. The census Surplus
  fell by 275, the Surplus at stake of the two causes.
- Judgments are in `judgments/look_alike_readings.txt`.

Census delta (measured on nnqtkmqy → mmnlorty): faces read 28,706 → 28,706; total Readings 218,547 → 217,817; Surplus 91,328 → 91,053; cheapest tier singleton in 14,544 of 17,220 → 14,604 of 17,167; no unit rose; lost faces: none.
Judgment set: 3,013 enforced at `COMPLETED_STRATUM` 6, all hold; 200 pending.
