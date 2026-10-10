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
  FH4 every one is a defect.
- FH6. If FH1–FH3 need a grammar edit after all (a feature on a table row),
  that is within the ticket; a new Construction is not.

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

## Landing record
