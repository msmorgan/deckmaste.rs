---
needs: []
---
# A noun that may stand bare must not shed the Complement that follows it

One right-edge-closure mechanism, in the engine, covers two defects:
- the idiom noun that may omit its Complement (*top*);
- the relational and partitive nouns (cost-audit defect #2: *number, amount,
  top, bottom, beginning, end, rest, choice, pair, pile, total*).

Owner's ruling (round 2, caveat 7): #2 is the same mechanism as this ticket.
It is merged here, and `english-v4-relational-noun-of-complement` is
retired.

*Addition* is no longer a witness. `english-v4-in-addition-to-predicate-host`
(`mnpnoqsp`) removed the noun: *in addition* is now a compound preposition in
`prepositions.ron`. One engine check on the right-edge closure of a bare noun that has a
PP Complement frame fixes both, with zero grammar features.

## The defect

A noun that declares `Complement(PP([p]))` in `nouns.ron` can still head a
Nominal through the bare valence (`Valence:*/Bare`). `compile.rs` admits the
noun there as follows:
- `Test::FreeOptional(Noun)` where the phrase is obligatory, for nouns that
  also declare `Intransitive` (*top*);
- `Test::FreeLexical(Noun)` elsewhere, which is how *number* and *amount*
  reach it.

The *p*-phrase right after it then attaches to another host:

- **Idiom noun** (CGEL, Ch. 7 §3.1, p. 620): Starving Revenant (*for each card
  you put on top of your library*) has 8 Readings, 4 of them *on [top]* with
  *of your library* elsewhere. Candidates: Nael, Avizoa Aeronaut; Cream of the
  Crop (an `Interleaved` complement-cluster PP).
- **Relational nouns:**
  - *mills cards equal to the number [of cards in their hand]*: *of …* is a
    Modifier of *cards*;
  - The War Doctor's *the number [of time counters on it]*: a Modifier of
    *damage*;
  - *from the top [of your library]*: a Modifier of *cards*;
  - *equal to the number [of cards drawn this way]*: a Modifier of *life*;
  - *each combination [of colors]*: a Modifier of *devotion*;
  - Mockingbird's *the amount [of mana spent …]*: a Modifier of *copy*,
    *creature*, *battlefield* or *mana value*.

These Readings predate
[`english-v4-amount-of-excess-bare-nominal`](../done/english-v4-amount-of-excess-bare-nominal.md),
which removed the same split for nouns that may not stand bare (*the amount of
[excess] damage dealt this way*).

## Authority

- CGEL, Ch. 5 §3.3, pp. 351–352: in *a number of protesters*, *number* is the
  head and *of protesters* its Complement.
- CGEL, Ch. 5 §14.1, pp. 439–441: a head noun licenses its PP Complement;
  *of* is the default preposition.
- CGEL, Ch. 5 §15, p. 454 [12i]: a light post-head Complement precedes
  post-head Modifiers.
- CGEL, Ch. 7 §3.1, p. 620: *on top*.
- `docs/decisions/english-lexical-analysis.md` §2: same-head complement
  priority, and "a newly admitted wrong Reading is a defect".
- The round-1 ruling cited "CGEL Ch. 5 §10". That section is *Pronouns*
  (pp. 425–431); the references above are the right ones.

## Change

- In the engine, record the preposition(s) of the noun's declared PP
  Complement frame on its bare valence.
  - Today that is the omitted-complement preposition mask from
    `Test::FreeOptional(Noun)`. Extend it to `Test::FreeLexical` for a noun
    that declares a PP Complement frame.
  - Propagate it along the right edge of ancestor constituents, as the
    `final_preposition`/`unextended` summary does.
  - Then no PP headed by that preposition can attach to any other host
    immediately after the bare noun. This covers:
    - an outer `Nominal+PrepositionPhrase` adjunct (Starving Revenant, the
      relational nouns);
    - an `Interleaved` complement-cluster PP (Cream of the Crop).
- Amend `english-lexical-analysis.md` §2 with this as an extension of
  same-head complement priority to nouns.
- **No new features in `english.ron`**, and no word named in the grammar. If
  the fix cannot be done that way, STOP and return to the owner.
- `pile` has no *of* frame today, though Oracle uses *a pile of* (8 times) and
  *piles of* (2 times). Declare it in `nouns.ron`.
- Extending the closure to every noun with a declared *of* frame (*ability,
  copy, controller, power, value, kind*, …) is the implementer's call. Do it
  only if the census shows no lost face and no unit gaining a Reading, and
  record the outcome here.
- Not affected:
  - *the top three cards of your library*: *of …* stays on *cards*;
  - *the number of colors of mana spent …*: *of mana* stays a Modifier of
    *colors*.

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Relational nouns, corpus sample:**
  - 19 ambiguous Segments lose 1,589 Readings of 3,068 Surplus (about 52%).
  - The four largest units were sampled only in part: Mockingbird, Maralen,
    Spark Double, Sphere of Annihilation.
- **Judgment set:**
  - 37 entries lose 546 Readings of 1,596 Surplus.
  - No entry loses its endorsed Reading, and none loses all its Readings.
- **Reach:** 5,348 ambiguous units contain a listed noun followed by *of*.
  Expect roughly half of their Surplus to go; the landing measures it against
  the corpus total of 175,304.

## Done when

- None of the examples above has a Reading with the Complement outside its
  noun.
- `+` judgments on real text pin the Complement analysis:
  - Starving Revenant;
  - The War Doctor;
  - Mockingbird's *the amount of mana spent to cast this creature*;
  - *your devotion to each color and each combination of colors*.
- *Put that card on top.* (Mwonvuli Beast Tracker) still reads.
- No new feature in `english.ron`.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.

1. **Census delta**: measured at landing.
2. **Judgment set**: measured at landing.
