---
needs: []
---
# Read scalar comparisons: N or more, equal to, than, mana value N

## Why

Comparative quantity and degree expressions are the largest family of unread
scalar text. On change `wlvwtnppyovn` (32,828 supported faces, 13,716 covered,
19,112 unread), as faces touched / faces where the bucket is the only
recognised cause (overlapping buckets, not gain forecasts):

| Item | Touched / sole | Probe (admitted roots) |
|---|---|---|
| *N or more / or less / or greater / or fewer* (excluding *one or more*) | 2,095 / 274 | "If you control two or more creatures, draw a card." 0; "… two creatures …" 1 |
| *one or more* | 592 / 123 | "Whenever one or more cards leave your graveyard, draw a card." 0; "Whenever a card leaves …" 1 |
| *equal to* | 1,606 / 285 | "This creature deals damage equal to its power to any target." 0; "… deals 2 damage to any target." 1; "Its power is equal to that creature's power." 0 |
| *less / greater / more / fewer / other than* | 541 / 99 | "This creature can't be blocked by more than one creature." 0; "Whenever you cast a spell other than your first spell each turn, draw a card." 0 |
| *with power N or greater*, *with mana value N/X* | mana value 1,124 / 39 | "Destroy target creature with power 2." 1; "… with power 2 or less." 0; "… with mana value 2." 0; "… with mana value X." 0 |
| copular scalar location | (overlaps the above) | "If that creature's power is 0, destroy it." 0; "If that creature's power is 0 or less, destroy it." 0 |

Lexical declarations already exist: `vocab:ScalarDegree/Equal` takes a
To-marked `MeasurePhrase`, and `vocab:ScalarDegree/Greater`, `…/Lesser` and
`vocab:Adjective/Less` take Than-marked `MeasurePhrase`s (the `frame_additions`
block of `crates/deckmaste_lexical_source/lexicon/core.ron`). Deal, Draw, Gain
and Lose declare `Role("ScalarEquality")` frames in `verbs.ron`. The gap is
composition and slot reconciliation, not vocabulary.

### What is already owned or done

- `english-v3-scalar-cardinals` (done) composes *up to N* as a quantitative
  Preposition Phrase in Determiner function. It names anaphoric and
  scalar-Amount bounds as remaining. This ticket does not reopen *up to*.
- `english-v3-number-transparent-concord` (done) handles *any number of*.
  Magma Sliver's scalar Subject *X* stays with `english-v3-systemic-residuals`.
- `english-v3-by-complement-functions` (done) handles *by N* scalar-change
  extent Complements. It leaves Spikeshell Harrier uncovered because it also
  needs "comparative and *below*". This ticket owns the comparative part.

This ticket takes over from `english-v3-systemic-residuals`:

- Deal's two `ScalarEquality` rows in the unsupported-inventory table of the
  done `english-v3-generic-frame-consumption`;
- the "Copular scalar location" item ("The number of cards in your hand is
  three."), because every *is N or less*, *is equal to* and *is less than*
  witness needs the same copular host;
- "comparison" in its "Label/body, ordered-destination, only-if/only-during,
  comparison, …" line.

## Goal

Each item reads through the existing declarations, with the comparative
Complement attached to its governor. Quantity comparisons (*three or more
creatures*) compose with the shared Quantitative Determiner interface built by
`english-v3-scalar-cardinals`. Do not build a second counted-NP family.

## Analysis

*Three or more* coordinates a cardinal numeral with *more*. CGEL uses it to
show the numerical status of *one* in *one or more volunteers* (CGEL, Ch. 5,
§7.6, p. 386, [iiia]). *One or more* takes a plural head, unlike bare *one*
(CGEL, Ch. 5, §3.4, p. 354, n. 13). The Agreement of the whole NP follows that
coordination, not the first numeral. *Equal to*, *more than*, *less than*,
*other than* are comparative governors with their selected prepositions
(CGEL, Ch. 13, §1.3, p. 1104, [15]). *Equal to* is non-scalar equality,
*more/less than* scalar inequality, *other than* non-scalar inequality. The
complement of *to*/*than* is the comparative Complement, licensed by the
governor and not a free PP. In *deals damage equal to its power* the AdjP
headed by *equal* is a Postmodifier of *damage* or the Deal frame's
ScalarEquality slot. Decide which by the frame and record it. *Is N* and *is
N or less* are *be* + NP scalar location (CGEL, Ch. 8, §5.4, pp. 693–694, as
recorded in the residuals ticket).

## Witnesses

- Warcry Phoenix: "Whenever you attack with three or more creatures, you may
  pay {2}{R}."
- Garrison Excavator: "Whenever one or more cards leave your graveyard, create
  a 2/2 red and white Spirit creature token."
- Soul's Fire: "Target creature you control deals damage equal to its power to
  any target."
- Ironhoof Ox: "This creature can't be blocked by more than one creature."
- Sage-Eye Avengers: "Whenever this creature attacks, you may return target
  creature to its owner's hand if its power is less than this creature's
  power." (copular host plus *less than*)
- Disembowel: "Destroy target creature with mana value X."

Each witness's only failing unit is the one quoted. Also usable: Mirkwood Elk
("You gain life equal to that card's power."), Stature, Size Shifter ("…if her
power is 1 or less."), Arcbound Tracker (*other than*).

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/scalar-comparisons-before.json` on the claim
   parent, stamped with its change id.
2. Land one commit per item, in table order. Write each item's witness as a
   test first.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading, covering every item. A
   comparative Complement read as a free Adjunct, or *or more* read as clausal
   *or*-coordination, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- *the same … as* (non-scalar equality with *same*): stays in
  `english-v3-systemic-residuals` ("Comparative *same … as*").
- *the number of* (1,296 touched, 9 sole) and *where X is* (1,119 / 114): not
  comparisons. Note how many become readable, but do not build for them.
- Cost modification (*costs {1} less*), *the greatest/least*, *tied for*.

## Landing record

In addition to the standard record: per item, before/after counts stamped with
change ids; the attachment chosen for *equal to* after a verb's Object, with
its frame basis; Spikeshell Harrier's status; timings as integer ns and ns/B
with host load and worker count.
