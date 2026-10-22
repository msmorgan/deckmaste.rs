---
needs: []
---
# [design] *the excess damage dealt …* is split into *the excess* and a Bare Relative Clause

Owner ruling (cost-audit smaller defect S11): mint this as a defect.

## The defect

Archaic's Agony: *Exile cards from the top of your library equal to the excess
damage dealt to that creature this way.* This Segment has 132 Readings, and
88 of them read *the excess* as a complete Noun Phrase (the noun *excess*).
They then read *damage dealt to that creature* as a Bare Relative Clause with
*damage* as its Subject. That Relative Clause postmodifies *top* (or *cards*):

- In one family, the Object of *deal* is the Gap: "the top that damage dealt
  to that creature".
- In the other family (44 Readings), *to* is stranded: "the top that damage
  dealt to __ that creature", with *deal* in its two-object frame.

Both families make *damage* the Subject of active *deal*.

## Authority

- [CR#120.1]: an object deals damage and is its source. Damage never deals
  anything.
  - In Oracle text, *damage dealt* is always the passive participle: 152
    occurrences in the snapshot.
  - Active *damage deals* never occurs in rules text.
- The noun *excess* occurs in Oracle text in only two ways (snapshot counts):
  - *the excess*, ending its Noun Phrase: Toralf, God of Fury, *deals damage
    equal to the excess to any target* (Judgment `excess_count_noun.txt:5`);
  - the fossilised *in excess of*.

  Before a noun, *excess* is the attributive Adjective (*excess damage*, 42
  occurrences). `nouns.ron` already records this in the comment on its
  `excess` entry.
- `docs/decisions/english-lexical-analysis.md` §2: "a newly admitted wrong
  Reading is a defect".

## Design decision to make at claim time

The syntax layer has no Game Model semantics (`docs/decisions/english-v4.md`).
So the fix cannot read [CR#120.1] directly. The candidate levers are:

1. **Lexical.** Close the Noun Phrase *the excess*. The noun *excess* takes its
   *of* Complement or nothing, and admits no unselected postmodifier, so a
   Relative Clause cannot follow a phrase that ends in it. This is a
   `distribution` value in `nouns.ron`, like the existing `HeadOnly`.
2. **Valence.** The frames of *deal* in the catalog declare which kinds of
   Noun Phrase can be the Subject of the active.
3. **Structural.** A Bare Relative Clause cannot postmodify a head across an
   intervening postpositive Adjective Phrase whose last word could premodify
   the Relative Clause's Subject.

Lever 3 is the most general and the riskiest, and it approaches a policing
rule. Lever 1 is the most local. Confirm the choice with the owner. If the
chosen lever counts as a policing flag under `docs/decisions/english-v4.md`
§4.2, STOP.

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Corpus sample:** Archaic's Agony loses 88 of its 132 Readings (Surplus 131).
- **Corpus reach:** 4 units contain *the excess damage* (Archaic's Agony,
  Nahiri's Warcrafting and two more), with 2,166 Readings between them.
- **Must keep:**
  - Toralf, God of Fury's *equal to the excess* (Judgment
    `excess_count_noun.txt:5`);
  - every *in excess of* unit.

## Done when

- Archaic's Agony and Nahiri's Warcrafting have no Reading with a Bare Relative
  Clause *damage dealt …*.
- Each has a `+` Judgment pinning *the excess damage dealt …* as one Noun
  Phrase.
- No face is lost, and no unit gains a Reading.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): this defect comes from a rule that is too broad, and
it is fixed by tightening or deleting that rule. The fix adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Caveat 5 (owner-approved): this ticket needs a structural fix or goes back to
the owner. Lever 1 adds a `NounDistribution` value, which is declared in
`english.ron`. Confirm with the owner whether a new value of an existing
feature is allowed before choosing it; if it is not, and no lever avoids it,
STOP.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
