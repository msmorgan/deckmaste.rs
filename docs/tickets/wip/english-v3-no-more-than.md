---
needs: []
---
# Read *no* modifying comparative *more*: No more than one creature can attack each combat.

## Why

*more than one creature* reads as a Noun Phrase and *no creature* reads;
*no more than one creature* does not, and neither does *no more than twice*.
Blocker report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`):
key `at:no more` 15 sole-blocker faces. `cargo xtask english-v3 grep -i
'\bno more than\b'` finds **18** faces (21 lines), all unread; **14** faces
have that line as their only failing line. Two hosts: a Noun Phrase (*No
more than one / two creature(s) can attack …*, *cast no more than two
spells*, *No more than one mana of each color may be spent …*; 10 faces) and
a frequency Adjunct (*Activate no more than twice / three times each turn*,
8 faces). Dueling Grounds, Silent Arbiter and Caverns of Despair have two
failing lines, both this construction (in scope, outside the sole count). `\bno (fewer|less|greater) than\b` has 0 faces.

## Goal

*no* reads as a pre-head Modifier of comparative *more* inside the
comparative quantity phrase, so *no more than one creature* is a Noun Phrase
and *no more than twice* / *no more than three times* a frequency phrase,
with byte-exact realisation.

## Analysis

CGEL Ch. 5 §11(d), p. 432: "The comparative forms of the degree
determinatives, *more*, *less*, and *fewer* take PP complements with the form
*than* + quantifier (usually a numeral), and pre-head modifiers realised by a
quantificational determinative (numeral, *many*, *much*, *some*, *a few*,
*no*, etc.)"; [5i] "[*More* / *Less* / *Fewer than twenty*] *people came to
the meeting*", [5iii] "*You can have* [*no* / *a little more*] *money*"; [6i]
"[*More than one*] *glass was broken*", with "The presence of *one* selects a
singular head noun". N. 49 (pp. 432–433): "*no more applications* … argues that
*more* is head of the construction". Those back *no* as Modifier of the head
*more* in a Determinative Phrase with a *than* Complement, and singular
agreement after *than one*. The frequency use (*no more than twice*) is not
in the passage: project ruling.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): the
Comparative Quantity Determiner route landed by
`english-v3-comparative-quantity-determiners` and
`english-v3-comparative-quantity-than` (category `ComparativeQuantity`;
`vocab:ComparativeQuantifier/More` with `ComparativeDeterminerUse`,
`ComparativeQuantityUse`, `MeasureModifierUse`); the `CardinalModifier`
slot; the frequency phrase route (`FrequencyPhrase`, *Activate only twice
each turn.* 1). *no* is `vocab:Determinative/No` (`DeterminerUse:
Unrestricted`). Glossary: **Comparative Quantity**, **Degree Modifier**.

Orchestrator rulings 2026-10-10:

- NM1. Refine the existing Comparative Quantity Construction with one
  optional pre-head Modifier slot filled by a Determinative; no new
  Construction. *no* declares the use (data), through an existing degree /
  modifier feature (`DegreeUse` has `Modifier`) if one fits, else one new
  declared value.
- NM2. The licence is that declared feature on the Modifier and the head's
  declared comparative use. No test on *no* or *more*. Do not declare
  *many*, *a few*, *some* as Modifiers here (0 faces), and do not add a
  check to block *no fewer than* (0 faces): it reads or not by what the
  features give.
- NM3. Frequency host: *no more than twice* / *no more than three times* is
  the same comparative quantity phrase where *more than twice* / *more than
  three times* would stand. If *more than twice* is not itself a frequency
  phrase today (probe it on the claim parent), supply it by letting the
  frequency route take a Comparative Quantity over a frequency Adverb or
  *N times*, again as a refinement; if that needs a new Construction, STOP
  with the count (8 faces) rather than adding it.
- NM4. One analysis per string. The twins to avoid: (a) *no* as Determiner
  of an NP *more than one creature* (Determiner over a determined NP);
  (b) *No creatures can attack you each combat.* (2 today) and *more than
  one creature* (1 as `noun-phrase`) keep exactly their Readings;
  (c) negative polarity: *no more than* does not make the Clause negative
  for *can't*-style agreement or central-Adverb placement beyond what the NP
  Subject already gives.
- NM5. Number agreement follows the Complement of *than* as it does today
  for *more than one creature* (singular) and *more than two creatures*
  (plural).
- NM6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains *no more*. An increase on a
  covered *more than* or *no* + noun face is a NM4 defect; report it with
  its shape before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--readings 3`): witness 0. Re-probe on the claim parent first.

- Crawlspace: "No more than two creatures can attack you each combat."
  (controls "No creatures can attack you each combat." 2; "More than one
  creature can attack each combat." 3+)
- Judoon Enforcers: "No more than one creature can attack you each combat."
- Pit Imp: "{B}: This creature gets +1/+0 until end of turn. Activate no
  more than twice each turn." (controls "Activate only twice each turn." 1;
  "Activate twice each turn." 1)
- Sewer Rats: "{B}, Pay 1 life: This creature gets +1/+0 until end of turn.
  Activate no more than three times each turn." (controls "Activate only
  three times each turn." 1; "Activate three times each turn." 2)
- Fires of Invention: "You can cast spells only during your turn and you can
  cast no more than two spells each turn." (control clause "You can cast
  more than two spells each turn." 2)

Also 0 today: "no more than one creature" (`noun-phrase`). Controls that
must not change: "more than one creature" (`noun-phrase`) 1; "no creature"
(`noun-phrase`) 1; "You can't cast more than two spells each turn." 2;
"Activate only once each turn." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bmore than\b|\bno more\b` (130 faces) and, because *no*
gains a declared use, every face matching `(?i)\bno\b` (360 faces), plus 300
random covered controls (`--random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; NM6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` and
`deckmaste_lexical_source`, once, at the end: a rebuild after a grammar edit
takes about 2–3 minutes and the six test binaries about 75 seconds, so batch
edits; lexicon data loads at runtime. Tests assert the role (*no* as
Modifier of *more*, the *than* phrase as its Complement, the whole as
Determiner or as frequency phrase, the NM4 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- *If there are no more scream counters on it* (All Hallow's Eve, 1 face):
  *no more* + noun with no *than* (CGEL's "further, additional" sense).
- *Activate no more times each turn than the number of snow Swamps you
  control* (Withering Wisps, 1 face): discontinuous comparative.
- The Eternal Wanderer's other failing line; Emblazoned Golem's *Spend only
  colored mana on X.*

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
