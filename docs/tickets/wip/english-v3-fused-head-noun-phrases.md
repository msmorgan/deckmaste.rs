---
needs: []
---
# Read *the other* with no following noun: Put one of them into your hand and the other into your graveyard.

## Why

The 2026-10-10 triage (`docs/memory/english-v3-blocker-triage-2026-10-10.md`,
blocker report at trunk `vwlkmvns`) put three shapes under "fused-head NPs",
about 60 sole-blocker faces: bare cardinal *one*, *the other*, and *the
first*. They are three mechanisms (Analysis), so this ticket takes the
largest and lists the other two with counts.

`cargo xtask english-v3 grep '\bthe other( (into|onto|on|to|from|in|is|was|under|at)\b|[.,])'`
finds **68** faces with head *other*; 44 have a single failing line of that
shape. *the others* has 0 faces. *other* lexes only as
`vocab:AttributiveAdjective/Other`, so "Put the other card into your hand."
reads (1) and "Put the other into your hand." reads 0.

## Goal

*the other* reads as a complete NP wherever a definite NP reads (*… and the
other into your graveyard*, *put the other on the top or bottom of your
library*), with byte-exact realisation.

## Analysis

CGEL Ch. 17 §6.2, pp. 1517–1518, [22ii]: "*One of the plates was broken and*
[*the other*] *was chipped*"; pro-nominal *other* "inflects for number, with
*others* as the plural form. This establishes quite clearly that it is a
noun"; "the structure of *the others* in [22i], for example, is simply
determiner + head − there is no syntactic fusion of two functions"; in the
singular it is "common following the determiners *the* and *one*". That backs
a noun lexeme under an ordinary Determiner, and says this is not a fused head.

For the two shapes left out: Ch. 5 §4, p. 356, "*He gave ten copies to me
and* [*six*] *to the others*" is a simple fused head (Determiner-Head);
Ch. 5 §9.3(c), p. 416, [24i] "[*the second*] *did not*" is an ordinal
adjective as fused Modifier-Head.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`DeterminedNounPhrase` (`[determiner: lexical(Determinative), head: Nominal]`)
reads *the rest* and would read *the other* given a noun.
`FusedDeterminerHeadNounPhrase` (`head.FusedHeadUse = Yes`, declared today by
the singular demonstrative and *anywhere*) is the route for the bare cardinal,
not for this ticket. `OrdinalPremodifier` is where *first* reads today.

Orchestrator rulings 2026-10-10:

- FH1. Data first: declare the pro-nominal noun *other* in
  `crates/deckmaste_lexical_source/lexicon/core.ron` and let
  `DeterminedNounPhrase` read it. No new Construction, and no route through
  `FusedDeterminerHeadNounPhrase` (CGEL: no fusion).
- FH2. Singular only: *the others* is unattested (pruning rule); do not
  declare a plural form.
- FH3. The noun is licensed only under a definite Determiner. Express that
  with a declared feature the existing determiner licence reads (the noun's
  determiner selection), never by naming *the* or *other*. If the existing
  licence tables cannot express it, STOP with the probe evidence; do not add
  a guard.
- FH4. One analysis per string; the twins to avoid: (a) *the other card*
  keeps exactly its Readings (the Adjective premodifies; the noun does not
  premodify, coordinate into a compound, or head *other card*); (b) reciprocal
  *each other* (47 faces after a verb or preposition, 0 Readings today) must
  not start reading as *each* + this noun: a reciprocal analysis is a
  different ticket, and a wrong analysis that parses is a defect; (c) *one
  other* / *any other* + noun keep their Readings.
- FH5. Newly covered faces are gains. Any Reading-count increase on a
  previously covered face is reported with its shape before landing; under
  FH4 every one is a defect except the exact shape authorized by FH8.
- FH6. If FH1–FH3 need a grammar edit after all (a feature on a table row),
  that is within the ticket; a new Construction is not.

- FH7. Orchestrator ruling (2026-10-10), resolving the FH3 STOP: a noun may
  declare its selected Determiner class, read by the determined-NP licence.
  Reuse `DeterminerRequirement` and the existing Definite dependent class;
  add a feature only if these cannot carry selection, explaining why. Guards
  read declared features on both sides. Nouns without selection retain their
  behaviour; every covered bounded face without *the other* must retain its
  count and digest. No new Construction. NounPhrase negatives *each other*,
  *any other*, *no other*, *an other*, *some other* must retain their existing
  Readings and gain none through the new noun.


- FH8. Orchestrator ruling (2026-10-10), resolving the Reading-count STOP:
  retain Flickerform 33 → 49, Invasion of Alara 36 → 54, and Plargg and
  Nassari 432 → 684. `[the [other [cards exiled ___ this way]]]` is a
  grammatical analysis: noun *other* heads a bare object relative. The
  retained-Reading contract preserves this even when game-semantically odd;
  no relative-head restriction is wanted. FH5 is amended to accept increases
  of exactly this shape; any increase of another shape remains a defect.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary (`--readings 3 --as-card`; (D) = `--category document`,
whole line): witness 0; control = the same unit with *the rest* (3 means at
least 3). Trunk has since taken `nvotwtqp`; re-probe on the claim parent
first.

- Nagging Thoughts (D): "Look at the top two cards of your library. Put one of
  them into your hand and the other into your graveyard." (control 3)
- Sibsig Appraiser (D): "When this creature enters, look at the top two cards
  of your library. Put one of them into your hand and the other into your
  graveyard." (control 3)
- Talas Lookout (D): "When this creature dies, look at the top two cards of
  your library. Put one of them into your hand and the other into your
  graveyard." (control 3)
- Tower Geist (D): same second sentence after "When this creature enters, look
  at the top two cards of your library." (control as Sibsig Appraiser)
- Write into Being: "Manifest one of those cards, then put the other on the
  top or bottom of your library." (control 3)

Controls that must not change: "Put the other card into your hand." 1; "Put
the rest into your hand." 1; "Put one of them into your hand and the rest
into your graveyard." 1; "Those creatures fight each other." 0 (FH4b: stays
0).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \bthe other\b|\beach other\b

(393 faces) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; FH5 for increases. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`,
once, at the end: a rebuild after a grammar edit takes about 2–3 minutes and
the test binary about 7, so batch edits. Tests assert the role (*other* as the
noun head of a definite NP in each witness, and the FH4 negatives by
structure), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only.

## Out of scope

- Bare cardinal as fused Determiner-Head (*Put one into your hand.*,
  *exile one face down*, *discard one*): 33 faces match
  `\b(one|two|three|four) (into|onto|on|from|to|at|under|face) `, 21 single
  failing lines; 13 of those also need this ticket. Own ticket, refining
  `FusedDeterminerHeadNounPhrase`. "Choose one." already reads.
- Ordinal as fused Modifier-Head (*for each target beyond the first*): 24
  faces match `\bbeyond the first\b`, 23 single failing lines. Own ticket.
- *… and the other on the bottom of your library* (4 lines) also needs
  `english-v3-gapped-coordination-shared-adjunct`.

## Claim-parent probes and FH3 STOP (2026-10-10)

Measured on claim parent `qtnyyyvx` (feature working copy `vkqxprpx`), before
any implementation. `cargo xtask english-v3 probe --readings 3 --json`,
`--category document` and each witness's `--as-card`:

| Witness | Original | With *the rest* |
|---|---:|---:|
| Nagging Thoughts | 0 | ≥3 |
| Sibsig Appraiser | 0 | ≥3 |
| Talas Lookout | 0 | ≥3 |
| Tower Geist | 0 | ≥3 |
| Write into Being | 0 | ≥3 |

All five witnesses remain in scope. Fixed document controls: *Put the other
card into your hand.* 1; *Put the rest into your hand.* 1; *Put one of them
into your hand and the rest into your graveyard.* 1; *Those creatures fight
each other.* 0. Noun-phrase controls: *one other card* 2; *any other card* 1.
Every probe reports zero issues. JSON evidence is under
`target/english-v3/english-v3-fused-head-noun-phrases/*-before.json`.
`cargo xtask english-v3 grep '\bthe other\b|\beach other\b' --count`
reports 393 supported faces and 400 matches on `qtnyyyvx`.

**STOP under FH3:** the existing `determiner_license` table reads only
`DeterminerUse`, `number`, and `countability`. *The*, *any*, *no*, and *some*
all declare `Unrestricted` and `Ordinary`; no choice of noun Number or
Countability can admit this noun under *the* while excluding the other
three. A singular Count noun would additionally admit *each other* through
`SingularCount`. `DeterminerRequirement` is the existing measured-nominal
outer-Determiner requirement, not a noun's definite-only selection. There
is no declared noun-side determiner selection read by this licence.

STOP resolved by orchestrator ruling FH7 (2026-10-10), above. Original proposal: add a declared
noun-side determiner selection and definiteness input to the licence,
propagate selection through Nominal composition, and keep the existing
`DeterminedNounPhrase` route. That note records the state at the STOP; implementation proceeds under FH7.

## Reading-count STOP (2026-10-10)

The bounded comparison on `vkqxprpx` finds three increases on covered faces:
Flickerform 33 → 49, Invasion of Alara 36 → 54, and Plargg and Nassari
432 → 684. `cargo xtask english-v3 diff --lines` attributes each increase to
its line containing *the other cards exiled this way*. Probing that NP finds
three complete Readings. The new one is
`[the [other [cards exiled ___ this way]]]`: `DeterminedNounPhrase` heads an
`ObjectRelativeNominal` whose noun is *other*; the zero object relative has
plural *cards* as Subject and finite preterite *exiled* as Predicate.

This is grammatical but game-semantically odd. FH5 rejects every covered-face
increase, whereas `docs/decisions/english-lexical-analysis.md` (2026-10-04
amendment, Source and roundtripping) preserves grammatical game-semantic
nonsense. The ticket-vs-ruling STOP was resolved by orchestrator ruling FH8
(2026-10-10), above: retain these three increases of this exact grammatical
shape, with no relative-head restriction. Any other shape remains a defect. Evidence:
`target/english-v3/english-v3-fused-head-noun-phrases/increase-shape.json` and
`diff.json`. This STOP is resolved; the standing STOP before integration remains.

## Landing record

1. **Census delta (`vkqxprpx`, preimplementation code `qtnyyyvx` → implemented):**
   bounded 689 faces, 527 → 565 covered, 11,749 → 15,064 Readings;
   38 gains, lost none, decreases none, digest-only changes none, 648 unchanged.
   Selector `\bthe other\b|\beach other\b` reaches 393 faces; add 300 random
   covered faces (seed 20261008), with 4 overlapping identities. Every covered
   face without *the other* has unchanged count and digest. Increases:
   Flickerform 33 → 49, Invasion of Alara 36 → 54, Plargg and Nassari
   432 → 684; each adds the grammatical zero-object-relative shape
   `[the [other [cards exiled ___ this way]]]`. These exact increases are accepted by
   orchestrator ruling FH8 (2026-10-10), above; no other increased shape was found. Official `diff --lines` evidence is
   `target/english-v3/english-v3-fused-head-noun-phrases/diff.json`.
   After refresh (`vkqxprpx` on refreshed `qtnyyyvx`), the same 689-face
   subset has 566 covered and 15,088 Readings: one trunk gain, Lita,
   Mechanical Engineer 0 → 24, from vocabulary batch 2 (`nqnztuzooxpw`,
   declaration of the opaque name Zeppelin); refresh adds no losses,
   decreases, covered-face increases or digest-only changes. Combined delta
   is 39 gains, 3 FH8-authorized increases, 647 unchanged
   (`final-diff.json`; isolated refresh delta `refresh-diff.json`). A second
   grammar-bearing refresh incorporated multiplier predeterminers
   (`qoswqlnsyrsm`); all 689 faces retained their counts and digests
   (`refresh2-diff.json`), so these final totals are unchanged.
2. **Witnesses (`qtnyyyvx` → `vkqxprpx`):** Nagging Thoughts 0 → 3,
   Sibsig Appraiser 0 → 3, Talas Lookout 0 → 3, Tower Geist 0 → 3,
   Write into Being 0 → 12 (counts for the ticket's witness line;
   whole-face after counts respectively 4, 3, 4, 4, 12). All use *other*
   as the noun head of `DeterminedNounPhrase`. The five FH7 negative NPs
   are 0 before and after; existing *the/one/any other card* modifier
   analyses are preserved.
3. **Spot checks (`vkqxprpx`):** A Little Chat, Chrome Courier, Dark Bargain,
   Fact or Fiction, Grim Contest, Nagging Thoughts, Sea Gate Oracle,
   Steam Augury, Tower Geist, and Write into Being each newly read with
   `DeterminedNounPhrase` headed by `Noun Other`. Ten full-source probes
   report exact roundtrips and zero issues (`spot-1.json` through
   `spot-10.json` under the evidence directory).
4. **Deviations and STOPs:** FH3 resolved by orchestrator ruling FH7
   (2026-10-10), recorded in Analysis. Reused `DeterminerRequirement` with
   one new value `Definite`, and the existing Determinative class
   `NominalAdjunctDependent`; licence reads both declared features. No new
   feature, Construction or frame; no superseded route to retire. Existing
   Nominal composition propagates selection, and alternate determiner routes
   respect it. The glossary's Determiner Requirement definition now covers
   noun selection, cited to FH7 separately from its original CGEL claim.
   Reading-count STOP resolved by orchestrator ruling FH8 (2026-10-10):
   retain the three exact grammatical relative-clause increases, amending FH5.
   The multiplier refresh's overlapping quantitative-determiner declarations
   were combined: trunk's Definiteness and this ticket's existing
   NominalAdjunctDependent input coexist; both test registrations were kept. One new test fixture
   was corrected to include the adjective's declared frame, then strengthened
   to compare the complete modifier analysis; no existing test was weakened.
5. **Tests (`vkqxprpx`):** `cargo test -p deckmaste_english_v3` passed
   (753 tests); `cargo test -p deckmaste_lexical_source` passed (76 tests).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   The first English run found the new fixture error described above; its
   corrected full-suite rerun passed. Restored 0 / re-spelled 0 / ignored 0 /
   added 5 / removed 0. Citation checks: zero noncompliant, zero stale;
   diff audit selected zero citation sites. After the grammar/data-bearing
   refresh, both full suites passed again (`deckmaste_english_v3`: 754;
   `deckmaste_lexical_source`: 80), and both exact clippy commands above
   passed again. `cargo test -p xtask english_v3` passed (56 tests),
   required because refresh changed construction-core code. The refresh's pure-add insertion conflicts in `core.ron`
   and `tests/main.rs` were resolved by retaining both trunk and feature
   additions; no feature test was changed or removed during refresh.
   After the second grammar-bearing refresh, both full suites passed again
   (`deckmaste_english_v3`: 764; `deckmaste_lexical_source`: 80) and both
   exact clippy commands passed again. No construction-core or plugin-data
   change arrived in that second refresh, so the xtask result was retained.
6. **Timing (`vkqxprpx`):** bounded after census 7.42 s wall time,
   6 workers, `--samples-per-face 0`, prebuilt binary under `/usr/bin/time -v`.
   Refreshed bounded census (`vkqxprpx`) 20.06 s wall time, same 6 workers
   and manifest; second refreshed bounded census 10.45 s, also 6 workers.
   User's binding bounded-run override applies; no full census
   was run.
