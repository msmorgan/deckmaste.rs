---
needs: []
---
# A fused-head genitive cannot be followed by the nominal it determines

Owner ruling (cost-audit smaller defect S8): mint this as a defect.

## The defect

Two genitive productions share one right-hand side (`Possessor` + `'s`):

- `GenitivePossessor` (CGEL Type I, a subject-determiner);
- `IndependentGenitivePhrase` (CGEL Type III, a fused subject-determiner-head).

Because they share it, a genitive directly followed by its own head nominal
gets a second analysis:

- the genitive is a complete fused-head Noun Phrase;
- the head nominal becomes a separate Temporal Noun Phrase Adjunct.

Examples:

- *that creature doesn't untap during [its controller's] [next untap step]*:
  *during* takes *its controller's* as its Complement, and *next untap step* is
  an Adjunct of *untap* or *don't*. (A Segment of the corpus sample; the
  template recurs across the corpus.)
- *At [this turn's] [next end of combat], tap each creature …*: *At* takes
  *this turn's*, and *next end of combat* becomes a separate Adjunct of *tap*.

## Authority

- CGEL, Ch. 5 §16.3, pp. 467–468:
  - In Type I (*Kim's father*), the genitive is a subject-determiner inside a
    matrix Noun Phrase.
  - In Type III (*Max's attempt wasn't as good as Kim's*), the genitive fuses
    with the head, which is understood.
- CGEL, Ch. 17 §6.1, pp. 1516–1517:
  - The head of a fused-head Noun Phrase is recovered from an antecedent
    nominal or from the context.
  - The antecedent cannot be discontinuous.
  - When the nominal stands immediately after the genitive, it is not an
    antecedent. It is the head, which is the Type I structure.
- `docs/decisions/english-lexical-analysis.md` §2: "a newly admitted wrong
  Reading is a defect".

## Change

- The fused-head genitive (`IndependentGenitivePhrase`) may not end a
  Constituent that is immediately followed by a Nominal that the genitive could
  determine as a Type I subject-determiner. Express this structurally:
  - either through the `unextended` mechanism against a following `Nominal`;
  - or through an equivalent closure the engine derives.
- Never name the words.
- Glossary: CGEL Ch. 5 §16.3 makes the glossary's *Independent Genitive* the
  pronoun forms (*mine*), and the glossary entry agrees. A noun genitive in
  fused-head use is CGEL's "fused subject-determiner-head". If this landing
  touches the production's name, add that term to
  `docs/contexts/oracle-english/CONTEXT.md` (`domain-modeling` skill) and
  rename the production to match.

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Corpus sample:** the 2 Segments above lose 68 Readings, out of a combined
  Surplus of 78.
- **Judgment set:**
  - 2 entries lose 6 Readings, out of a Surplus of 8.
  - No entry loses its endorsed Reading.
- **Corpus reach:**
  - 478 ambiguous units contain a genitive directly before a turn-part or step
    nominal (*'s next untap step*, *this turn's*, *'s upkeep*, …).
  - Their combined count is 3,850 Readings.
  - Expected reduction: most of those units' Surplus that comes from this
    split. The landing measures it.
- **Corpus, `PreposedPrepositionAndTemporal`:** the 15 genitive-split units
  listed below lose their split Readings (157 by product, 99 Surplus, on
  `snxspnqn`).
- **Must keep:** genuine fused-head genitives with an antecedent, for example
  *power less than Mirko's* (the production's own example; its comment cites CGEL Ch. 5 §16.5, p. 471, but p. 471 is §16.4, "Genitive pronouns" — correct the comment if the production is touched).

## Also: the rest of `PreposedPrepositionAndTemporal`

Owner ruling (round 3, item 2): clean this production up alongside S8.

`PreposedPrepositionAndTemporal` (*For one spell this turn, …*) lets a fronted
Preposition Phrase and a Temporal Noun Phrase share one comma.
`english-v4-fronted-temporal-pp-clause` (`snxspnqn`) limited it to a
Preposition Phrase with a Noun Phrase Complement. Measured on `snxspnqn`, 30
units still use it, for 483 Readings by product (159 Surplus):
- **The genitive split above:** 15 units, 157 Readings (99 Surplus). This
  ticket's genitive fix should remove them.
  - With *at*: Triton Tactics (50 Readings), Gaze of the Gorgon, Venomous
    Breath, Glyph of Doom, Faramir, Prince of Ithilien and Suppress.
  - With *during* or *until*: Taunt, Gideon Jura, Rowan Kenrith, Imaginary
    Threats, Oracle en-Vec, Teferi's Reproach, Firestorm Phoenix, Casey & Raph,
    Hotheads and Nuka-Nuke Launcher.
- **A complete trigger or header phrase, then its temporal bound:** 6 units,
  258 Readings.
  - Full Throttle (*At the beginning of each combat this turn,*), Feral
    Encounter, Vivien's Stampede and Pygmy Hippo (*at the beginning of your
    next main phase this turn,*).
  - Paradox Haze (*At the beginning of enchanted player's first upkeep each
    turn,*).
  - World at War (*After the second main phase this turn,*).
  - Each also has the Reading with the temporal inside the phrase.
- ***For each … that entered this turn,*:** 8 units, 63 Readings. *This turn*
  is lifted out of the relative clause. The units are Hurkyl, Master Wizard;
  Renewed Solidarity; Living Laser; Redoubled Stormsinger; Ocelot Pride;
  Murmuration; Giant Albatross; and Tobias, Doomed Conqueror.
- **North Star (*For one spell this turn, you may …*):** 5 Readings. It is the
  only intended use in the corpus.

The ruling says all but North Star are spurious:
- A trigger's or a phase-relative header's temporal bound (*this turn*, *each
  turn*) belongs to the phrase before the comma, not to the matrix Clause
  (`docs/oracle-style-guide.md` checklist item 4 and §8, Triggered
  abilities; [CR#603.1,603.7]).
- In *for each … that entered this turn*, *this turn* modifies *entered*
  inside the relative clause. It does not modify the matrix Clause across the
  relative boundary.

The owner named two approaches. The implementer measures both and picks one:
1. Give North Star a Reading with *this turn* inside the *for* phrase, then
   delete the production.
2. Mark the production's Preposition Phrase child `unextended`, so that it
   cannot end where the following Temporal Noun Phrase could attach inside
   it.
   - North Star keeps its Reading: *this turn* cannot attach inside *for one
     spell*.
   - Today `unextended` covers only a trailing Preposition Phrase that a
     sibling Production would select (`crates/deckmaste_syntax/README.md`).
     Widening it is an engine change, not a grammar feature.
   - `english-v4-two-preposed-preposition-phrases` may need the same closure.

## Done when

- *during its controller's next untap step* and *At this turn's next end of
  combat* have no Reading with the genitive as a complete Noun Phrase.
- Each has a `+` Judgment on real text.
- None of the 29 spurious units above keeps a Reading that strands its
  temporal before the comma.
  - `+` Judgments on Full Throttle and Murmuration witness it with `!`
    assertions.
  - North Star's judgment (`subordinate_clauses.txt`) still holds.
- No face is lost, and no unit gains a Reading.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): this defect comes from a rule that is too broad, and
it is fixed by tightening or deleting that rule. The fix adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Standard constraints apply.

Census delta (measured on `lzzwskzn` → `pwyvozyw`): faces read 28,709 → 28,709, total Readings 295,697 → 294,324, Surplus 149,101 → 148,485; lost faces: none.
Judgment set: `cargo test -p deckmaste_english_v4` passes; 4 entries added (Triton Tactics, Pelt Collector, Full Throttle, Murmuration), none rewritten or retired.
