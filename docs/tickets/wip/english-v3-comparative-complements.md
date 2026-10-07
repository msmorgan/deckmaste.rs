---
needs: []
---
# Read comparative governors with their complements: equal to, less than, other than

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: an adjective comparative governor with
its expanded comparative Complement (*equal to X*, *less/greater than X*,
*other than X*), in postpositive or predicative function. On change
`wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread), as
faces touched / faces where the bucket is the only recognised cause
(overlapping buckets, not gain forecasts):

| Item | Touched / sole | Probe (admitted roots) |
|---|---|---|
| *equal to* | 1,606 / 285 | "This creature deals damage equal to its power to any target." 0; "… deals 2 damage to any target." 1; "Its power is equal to that creature's power." 0 |
| *less / greater / more / other than* outside a Determiner | (in the *than* bucket, 541 / 99) | "Whenever you cast a spell other than your first spell each turn, draw a card." 0 |

Lexical declarations already exist: `vocab:ScalarDegree/Equal` takes a
To-marked `MeasurePhrase`, and `vocab:ScalarDegree/Greater`, `…/Lesser` and
`vocab:Adjective/Less` take Than-marked `MeasurePhrase`s (the `frame_additions`
block of `crates/deckmaste_lexical_source/lexicon/core.ron`). Deal, Draw, Gain
and Lose declare `Role("ScalarEquality")` frames in `verbs.ron`. The gap is
composition and slot reconciliation, not vocabulary.

## What is already owned or done

- `english-v3-by-complement-functions` (done) handles *by N* scalar-change
  extent Complements. It leaves Spikeshell Harrier uncovered because it also
  needs "comparative and *below*". This ticket owns the comparative sentence.
  Its other failing unit, "This effect can't reduce their speed below 1.", is
  a *below* PP, not a comparative governor; it stays with
  `english-v3-systemic-residuals`.
- From `english-v3-systemic-residuals`: Deal's two `ScalarEquality` rows in
  the unsupported-inventory table of the done
  `english-v3-generic-frame-consumption`, and Discerning Taste's comparison
  body ("You gain life equal to the greatest power among …"); its superlative
  composition stays audited there.

## Goal

Each item reads through the existing declarations, with the comparative
Complement attached to its governor and not as a free PP. *Equal to* after a
verb's Object (*deals damage equal to its power*, *gain life equal to …*)
either postmodifies the Object or fills the verb's `ScalarEquality` slot:
decide which by the frame and record it.

## Analysis

*Equal to*, *more/less than*, ·*er than* (*greater than*) and *other than* are
comparative governors with the prepositions their expanded Complements take
(CGEL, Ch. 13, §1.3, p. 1104, [15]): *equal* is non-scalar equality [15ii],
*more/less*/·*er than* scalar inequality [15iii], *other than* non-scalar
inequality [15iv]. The complement of *to*/*than* is the comparative
Complement, licensed by the governor (p. 1104, [13]–[15]). An AdjP with its own
post-head dependent occurs postpositively in NP structure, as in *members
[dissatisfied with the board's decision]* (Ch. 5, §14.2(b), p. 445), which is
the shape of *damage equal to its power* and *a spell other than your first
spell*. In *its power is less than this creature's power* the same AdjP is the
predicative Complement of *be*; the be + NP scalar-location host (*its power
is 0*) is `english-v3-copular-scalar-location`, not this ticket.

DP-internal *more than one creature* / *fewer than six cards* (comparative
determinative heading a Determinative Phrase) is
`english-v3-comparative-quantity-determiners`.

## Witnesses

- Soul's Fire: "Target creature you control deals damage equal to its power to
  any target."
- Mirkwood Elk: "You gain life equal to that card's power."
- Freedom Fighter Recruit: "Freedom Fighter Recruit's power is equal to the
  number of creatures you control."
- Sage-Eye Avengers: "Whenever this creature attacks, you may return target
  creature to its owner's hand if its power is less than this creature's
  power."
- Arcbound Tracker: "Whenever you cast a spell other than your first spell
  each turn, put a +1/+1 counter on this creature."
- Worldslayer: "Whenever equipped creature deals combat damage to a player,
  destroy all permanents other than this Equipment."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also usable: Cosmos Elixir (*greater than*), Spikeshell Harrier's comparative
sentence, and Ghastly Demise / Dispersal Shield (*less than or equal to*,
coordinated governors sharing one Complement).

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/comparative-complements-before.json` on the
   claim parent, stamped with its change id.
2. Land one commit per governor class (equality, scalar inequality, non-scalar
   inequality). Write each witness as a test first.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading, covering every governor
   class. A comparative Complement read as a free Adjunct, or *to its power*
   read as Deal's recipient, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.
9. A CGEL citation may back only what the cited passage itself says; a project
   or orchestrator ruling is cited as a ruling, never attributed to CGEL.
10. Retire a superseded route on both the lexicon and the grammar side; do not
    leave unreachable declarations.

## Out of scope

- *the same … as* (non-scalar equality with *same*): stays in
  `english-v3-systemic-residuals` ("Comparative *same … as*").
- *more than one creature*, *three or more creatures*:
  `english-v3-comparative-quantity-determiners`.
- *its power is 0*, *is 1 or less*: `english-v3-copular-scalar-location`.
- *with power 2*, *with mana value X*: `english-v3-scalar-property-values`.
- *the number of* (1,296 touched, 9 sole) inside an *equal to* Complement: not
  a comparison. Note how many become readable, but do not build for it.
- Cost modification (*costs {1} less*), *the greatest/least*, *tied for*,
  *rather than* (also excluded by `english-v3-instead-replacement`).

## Landing record

In addition to the standard record: per governor class, before/after counts
stamped with change ids; the attachment chosen for *equal to* after a verb's
Object, with its frame basis, and the disposition of Deal's two
`ScalarEquality` rows; Spikeshell Harrier's status; timings as integer ns and
ns/B with host load and worker count.
