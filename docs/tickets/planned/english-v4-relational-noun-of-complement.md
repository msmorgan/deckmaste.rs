---
needs: []
---
# A relational noun takes the *of* phrase that follows it as its Complement

Owner ruling (cost-audit defect #2, verbatim in substance): the relational and
partitive nouns *number, amount, top, bottom, beginning, end, rest, choice,
pair, pile, total* must take an immediately following *of* Preposition Phrase
as their Complement. They may not stand bare while that *of* phrase attaches to
a higher host.

## The defect

The grammar today admits Readings in which such a noun keeps its bare frame,
while the *of* phrase right after it becomes an unselected Modifier of an
earlier noun:

- *Each opponent mills cards equal to the number [of cards in their hand]*:
  the *of* phrase is read as a Modifier of *cards*, not as the Complement of
  *number*.
- *it deals damage equal to the number [of time counters on it]* (The War
  Doctor): the *of* phrase becomes a Modifier of *damage*.
- *Reveal cards from the top [of your library]* and *Exile cards from the top
  [of your library] …*: the *of* phrase becomes a Modifier of *cards*.
- *You lose life equal to the number [of cards drawn this way]*: the *of*
  phrase becomes a Modifier of *life*.
- *your devotion to each color and each combination [of colors]*: the *of*
  phrase becomes a Modifier of *devotion*.
- Mockingbird: *the amount [of mana spent …]* has the *of* phrase as a Modifier
  of *copy*, *creature*, *battlefield* or *mana value*.

## Authority

- CGEL, Ch. 5 §3.3, pp. 351–352: in *a number of protesters*, *number* is the
  head and *of protesters* is its Complement. The *of* phrase forms a
  constituent with the oblique, not with *a number*.
- CGEL, Ch. 5 §14.1, pp. 439–441: a head noun licenses its PP Complement, and
  *of* is the default preposition for that Complement.
- CGEL, Ch. 5 §15, p. 454 [12i]: a light post-head Complement precedes the
  post-head Modifiers.
- `docs/decisions/english-lexical-analysis.md` §2:
  - same-head complement priority;
  - "a newly admitted wrong Reading is a defect".
- The ruling cited "CGEL Ch. 5 §10". That section is *Pronouns* (pp. 425–431);
  the correct references are the ones listed above.

## Change

- The decision derives from the nouns' declared
  `Complement(PP(["of"]))` frames in `nouns.ron`, the data the engine already
  reads for same-head complement priority (`CompiledGrammar::extensions`).
  It is never a per-word flag in the grammar.
- Rule: a Nominal headed by one of these nouns that ends at its head (no
  Complement taken) cannot be directly followed by an *of* Preposition Phrase
  attached to any other host.
- `pile` has no *of* frame today, although Oracle uses *a pile of* (8 times)
  and *piles of* (2 times). Declare that frame.
- Applying the rule to every noun with a declared *of* frame (*ability, copy,
  controller, power, value, kind*, …) is the implementer's call. Make it only
  if the census shows no lost face and no unit gaining a Reading, and record
  the outcome in this ticket.
- Not affected: an *of* phrase after a noun that does not select it.
  - *the top three cards of your library* keeps *of your library* on *cards*.
    *top* is not the head immediately before *of*.
  - *the number of colors of mana spent …* (Archaic's Agony) keeps *of mana* as
    a Modifier of *colors*.

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Corpus sample:**
  - 19 ambiguous Segments lose 1,589 Readings, out of a combined Surplus of
    3,068 (about 52%).
  - The four largest sampled units (Mockingbird, Maralen, Spark Double, Sphere
    of Annihilation) are not counted, because their signatures were sampled
    only in part. They carry most of the remaining hits.
- **Judgment set:**
  - 37 entries lose 546 Readings, out of a Surplus of 1,596.
  - No entry loses its endorsed Reading.
  - No entry loses all of its Readings.
- **Corpus reach:** 5,348 ambiguous units contain one of the listed nouns
  followed by *of*. That is the upper bound on the units affected.
- **Expected reduction:** roughly half of those units' Surplus, against a
  corpus total of 175,304. The landing measures it.

## Done when

- None of the examples above has a Reading with the *of* phrase outside the
  relational noun.
- *+* Judgments on real text pin the Complement analysis, for example:
  - The War Doctor;
  - Mockingbird's *the amount of mana spent to cast this creature*;
  - *your devotion to each color and each combination of colors*.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
