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

Owner refinement (2026-10-10) supersedes NM2's member-list restriction:
the existing quantificational Determinatives *no*, *any*, *many*, *much*,
and *some* share `ComparativeDegreeModifierUse: Yes`. CGEL Ch. 5 §11(d),
p. 432 supports the quantificational Modifier class; *any* is licensed by
the owner ruling. `DegreeUse` cannot express this independently of *many*
and *much* being degree heads. No *far* or *a few* Lexeme exists here;
none is minted. Frequency Complements and their Predicate host refine
existing schemas under NM3.

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

1. **Census delta** (`olomvlyu` before implementation, with claim-parent
   grammar at immutable `nknuqvqu` (original claim `ltmzsouk`), →
   implemented `olomvlyu`): **11,692 → 11,709
   covered faces**, **544,916 → 545,003 line Readings**. **17 gains**;
   lost faces: none; decreases / covered-face increases / digest-only:
   **0 / 0 / 0**. Digest-only line counts are zero in every class; the
   34 changed line entries belong to gains. No wrong-analysis-only gain.
   Selector: supported-corpus grep
   `(?i)\b(more|less|fewer|than|no|any|many|much|some|times|time|once|twice|wrong|yet|simultaneously|exactly|secretly|fully|as|long|then|up|down|also|next|still|rather|otherwise|alone|already|again|so|off|only)\b`
   (**13,080 faces**, covering every Comparative Quantity use, every
   changed Adverb/frequency declaration and the newly licensed words),
   union 300 covered controls from `diff --manifest --random 300
   --seed 20261008`: **13,276 distinct faces**. All 300 controls remained
   covered on the claim parent. Evidence:
   `target/english-v3/english-v3-no-more-than/{manifest,before,after,diff}.json`;
   comparison is `cargo xtask english-v3 diff before.json after.json
   --lines --json diff.json` with those full paths.
   **Refresh attribution** (`otvxplsm`, after `kata refresh`): the official
   BEFORE → `after-refresh.json` comparison reports **11,692 → 11,732**
   covered faces, **544,916 → 545,317** line Readings, **40 gains**,
   **0 lost / 0 decreases / 0 covered-face increases / 633 digest-only**.
   The official `after.json` → `after-refresh.json` comparison isolates
   **23 incoming gains**, **0 lost / 0 decreases / 0 increases**, and
   all **633 digest-only faces**, from trunk implementations `ouykkpns`
   (irrealis scalar difference) and `knlqxlok` (delayed passive shared
   complements). The original 17-gain ticket result is preserved.
   Incoming digest-only classes, attributed to `ouykkpns`'s existing
   scalar Constructions becoming schema instances: **563 measured-cost-only
   lines**, **83 finite-scalar-only lines**, **1 mixed line** (*The End*),
   **647 lines total**; unexplained classes: none. Every changed line was
   parsed and its retained Reading nodes inspected. Evidence:
   `diff-refresh.json`, `diff-incoming.json`, and
   `incoming-digest-classes.json` in the same target directory. The
   remaining incoming changed lines belong to gains; no new covered-face
   Reading shape needs an increase justification.
2. **Witnesses** (`olomvlyu`, parent → implementation probe counts):
   Crawlspace **0 → 2**; Judoon Enforcers **0 → 2**; Pit Imp **0 → 1**;
   Sewer Rats **0 → 2**; Fires of Invention **0 → 4**; Noun Phrase
   *no more than one creature* **0 → 1**. No witness already read.
   Parent controls, all unchanged: *No creatures can attack you each
   combat.* **2**; *More than one creature can attack each combat.* **3**;
   *Activate only twice / twice each turn.* **1 / 1**; *Activate only
   three times / three times each turn.* **1 / 2**; *You can cast more
   than two spells each turn.* **2**; NPs *more than one creature* /
   *no creature* **1 / 1**; *You can't cast more than two spells each
   turn.* **2**; *Activate only once each turn.* **1**. Prerequisite
   controls: *Activate more than twice each turn.* **0 → 1**, *Activate
   more than three times each turn.* **1 → 2**. Sewer Rats' **2** sit
   beside its *three times* control's **2** under W9-A: one frequency
   Adjunct, one nominal Object; both contain the same licensed comparative
   Modifier route. Pit Imp's **1** sits beside its *twice* control's **1**.
3. **Spot checks** (`olomvlyu`): Caverns of Despair, Crawlspace, Dueling
   Grounds, Fires of Invention and Judoon Enforcers use the Comparative
   Quantity as Determiner with *no* internal to it and *than* as its
   Complement. Fire-Belly Changeling, Phyrexian Battleflies and Pit Imp
   use a Frequency Predicate over a Comparative Quantity with *twice*
   in the Complement; Keen-Eared Sentry uses the same frequency route
   with *once*. Manaforge Cinder retains the frequency/nominal alternatives
   for *three times*. These are ten newly covered faces; all 17 gained
   faces' relevant lines were probed and inspected in
   `target/english-v3/english-v3-no-more-than/spot-checks.json`.
4. **Deviations and STOPs** (`olomvlyu`): no Construction, Verb Frame or
   Lexeme added or retired. NM3 uses two category instances of the existing
   `QuantitativePrepositionPhrase` schema (counted frequency / frequency
   Adverb), plus a form of the existing `FrequencyPredicate` schema.
   `FrequencyUse` passes through Adverb focus and Coordination; numerical
   determination rejects it. The Modifier class follows the owner
   refinement above, rather than NM2's superseded member list. Glossary
   additions: Comparative Degree Modifier Use and Frequency Use, with
   CGEL and project-ruling attribution kept distinct. **STOP, ruling
   pending:** Emblazoned Golem's *no more than one mana of each color*
   and its unmodified control *more than one mana of each color* both
   have **0** Readings. The existing measured NP admits only Coordinated
   quantities under BQ1; a ruling was requested on admitting Headed
   quantities there or leaving this inherited host gap to a follow-up.
   That route has not been broadened. **Refresh STOP, scope ruling pending:**
   the full English suite fails
   `selected_complement_clusters::all_nine_inherited_recipient_witnesses_have_exact_independent_document_sets`
   at the unchanged chart-item ceiling: **12,503 > 12,500**;
   **12,884** families remain within **13,000**. This ticket's full suite
   passed before refresh. The exact two-line artifact fixture's direct
   packed probe has **1** Reading, complete enumeration, byte-exact
   realization and zero issues. The earlier item guard prevents the test
   reaching its exact Reading-set assertion. No test ceiling or assertion
   was weakened; parser-performance work would extend this ticket's scope,
   and the orchestrator was asked whether to add it or review this STOP.
   Evidence: `refresh-test-english.log` and `resource-failure-probe.json`
   under the ticket's target directory. The Eternal Wanderer's independent
   failing line and the ticket's discontinuous/no-*than* exclusions remain
   outside this implementation.
5. **Tests** (`olomvlyu`): `cargo test -p deckmaste_english_v3` PASS
   (**1,084 passed**, one pre-existing ignored test:
   `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`,
   blocked by `english-v3-mana-symbol-verbless-subject` MS4's governed Mana
   Phrase Subject route); `cargo test -p deckmaste_lexical_source` PASS
   (**88 passed**); `cargo clippy -p deckmaste_english_v3 --tests --
   -D warnings` PASS; `cargo clippy -p deckmaste_lexical_source --tests --
   -D warnings` PASS. Restored / re-spelled / newly ignored / added /
   removed: **0 / 3 / 0 / 5 / 0**. Re-spelled
   `independent_comparative_quantities_preserve_structure_and_lexical_identity`,
   `comparative_determiners_preserve_finite_agreement` and
   `comparative_quantity_selection_rejects_invalid_independent_values`:
   their independently constructed comparative values now include
   `modifier: None`, retaining every existing card and expected outcome.
   No removal to justify. Citation checks: zero noncompliant, zero stale,
   no new citation sites in the piped diff audit; `cite bless` unchanged.
   After refresh (`otvxplsm`, code-equivalent to refreshed `olomvlyu`):
   `cargo build -p xtask` PASS; the same full English test command **FAIL**
   (**1,092 passed / 1 failed / 1 pre-existing ignored**), with the failure
   named in item 4; the same full lexical-source test command PASS
   (**88 passed**); both exact clippy commands above PASS. All five new
   tests passed again. No additional test re-spelling or removal.
6. **Timing** (`olomvlyu`): the bounded AFTER census took **549 seconds, 100 milliseconds**
   under `/usr/bin/time -v`, **6 workers**, `--samples-per-face 0`.
   The same bounded census after refresh took **383 seconds, 580 milliseconds**, under the
   same timing command, **6 workers**, `--samples-per-face 0`, with the
   original **13,276-face** identity manifest; no full corpus run.
