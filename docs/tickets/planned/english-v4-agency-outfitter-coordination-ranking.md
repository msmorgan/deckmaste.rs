---
needs: []
---
# Agency Outfitter: the intended *may [search … and put …]* ranks third

## Owner's ruling (round 7, item 7)

"Ticket investigating why Agency Outfitter's wrong clause-coordination
Readings rank ahead of the intended `may [search … and put …]` (fix via clean
structure/ranking, no hax)."

## Evidence

The corrected line (`corrections.txt`) is *When this creature enters, you may
search your graveyard, hand, and/or library for a card named Magnifying Glass
and/or a card named Thinking Cap and put them onto the battlefield.* Measured
on the round-7 base, it has 17 Readings:

- **Readings 1–2 (keys 0/4/43 and 0/11/44): wrong.** They are a Clause
  coordination, *[you may search … for a card named Magnifying Glass] and/or
  [a card [named __ Thinking Cap and put them onto] the battlefield]*.
  - The second conjunct is a preterite declarative.
  - Its Verb Phrase coordination shares *the battlefield* by Right Node
    Raising.
  - In *named* (`ComplexTransitive(NP,Pred)/gap0`), the shared phrase fills
    the gapped Object, which is followed by the predicative *Thinking Cap*.
    The Gap is not at the conjunct's right edge.
- **Reading 3 (0/15/43): intended.** *you may [[search …] and [put them onto
  the battlefield]]*.
- **Readings 4–17: other wrong scopes.** Among them is *[may search …] and
  [put …]*.

## Leads

1. Right Node Raising needs the shared constituent at the right edge of every
   conjunct. *named __ Thinking Cap* has its Gap before the predicative, so
   it should not be a raising conjunct. Find why the `gap0` derivation
   counts as right-edge here (`Summary::gap_right_edge`), and close it
   structurally.
2. The Clause coordination joins a present modal Clause and a preterite one
   under *and/or*. `english-v4-coordinated-clause-tense` would have removed
   such coordinations at admission, but the owner's round-8 treatment plan
   retired it: a mixed-tense coordination is grammatical English
   (`docs/decisions/english-v4.md` §4.4). Tense will not remove Readings 1–2.
   Only lead 1, a Gap that is not at the right edge, can do that.
3. Only if structure leaves wrong Readings ahead: examine the Ranking Key's
   Right Association, which charges the long VP coordination under *may*
   more than the short Clause conjuncts. Any change must be purely structural
   (`docs/decisions/english-v4.md` §4.3), with no policing feature and no
   word.

## Done when

- The intended Reading is cheapest, or every Reading ranked ahead of it is
  removed by a structural fix that removes only wrong Readings corpus-wide.
- The judgment in `judgments/coordination.txt` still holds.
- No face is lost. Every unit that changes is listed and adjudicated.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
