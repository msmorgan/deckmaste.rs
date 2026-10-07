---
needs: []
---
# Read comparative quantity determiners: N or more, one or more, more than one

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: a comparative quantity expression in
Determiner function of a counted Noun Phrase. On change `wlvwtnppyovn`
(32,828 supported faces, 13,716 covered, 19,112 unread), as faces touched /
faces where the bucket is the only recognised cause (overlapping buckets, not
gain forecasts):

| Item | Touched / sole | Probe (admitted roots) |
|---|---|---|
| *N or more / or fewer* (surface bucket, excluding *one or more*) | 2,095 / 274 | "If you control two or more creatures, draw a card." 0; "… two creatures …" 1 |
| *one or more* | 592 / 123 | "Whenever one or more cards leave your graveyard, draw a card." 0; "Whenever a card leaves …" 1 |
| *more / fewer than N* before a head noun | (in the *than* bucket, 541 / 99) | "This creature can't be blocked by more than one creature." 0 |

The *N or more* surface bucket also counts *power 2 or less* and *is 0 or
less*, whose outer constructions belong to the sibling tickets below; the
touched figure is a surface count, not this ticket's population.

## What is already owned or done

- `english-v3-scalar-cardinals` (done) composes *up to N* as a quantitative
  Preposition Phrase in Determiner function. It names anaphoric and
  scalar-Amount bounds as remaining. This ticket does not reopen *up to*.
- `english-v3-number-transparent-concord` (done) handles *any number of*.

## Goal

*three or more creatures*, *one or more cards*, *more than one creature* and
*fewer than seven cards* read with the comparative quantity expression in
Determiner function, composed through the shared Quantitative Determiner
interface built by `english-v3-scalar-cardinals`. Do not build a second
counted-NP family. The numeral-*or*-comparative coordination built here
(*two or more*, *0 or less*) is the one `english-v3-scalar-property-values`
and `english-v3-copular-scalar-location` reuse; build it as a unit those
hosts can consume, not as a Determiner-only recipe.

## Analysis

*Three or more* coordinates a cardinal numeral with *more*. CGEL uses *one or
more volunteers* to show the numerical status of *one*, citing *three or more*
as the parallel (Ch. 5, §7.6, p. 386, [44iiia]). The head is plural when
determined by the coordination *one or more*, while *one* in a DP headed by
*more* (*more than one*) selects a singular head (Ch. 5, §3.4, p. 353, n. 13;
the split ticket cited p. 354, but the footnote is anchored on p. 353). The
Agreement of the whole NP follows that structure, not the first numeral.

*More than one* and *fewer than twenty* are Determinative Phrases headed by
the comparative determinative, whose *than* + quantifier PP is its Complement
(Ch. 5, §11(d), p. 432, [5]–[6]). *More than one glass* is syntactically
singular (p. 432, [6i]). These are DP-internal comparative Complements in
Determiner function; the predicative and postpositive comparatives (*is less
than*, *other than*) are `english-v3-comparative-complements`.

## Witnesses

- Warcry Phoenix: "Whenever you attack with three or more creatures, you may
  pay {2}{R}."
- Garrison Excavator: "Whenever one or more cards leave your graveyard, create
  a 2/2 red and white Spirit creature token." (plural Agreement with *leave*)
- Ironhoof Ox: "This creature can't be blocked by more than one creature."
  (singular head)
- Eidolon of Rhetoric: "Each player can't cast more than one spell each turn."
- Shadowborn Demon: "At the beginning of your upkeep, if there are fewer than
  six creature cards in your graveyard, sacrifice a creature."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also usable: Norwood Riders and Outland Colossus (as Ironhoof Ox), Cunning
Bandit ("if there are two or more ki counters on this creature").

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/quantity-determiners-before.json` on the claim
   parent, stamped with its change id.
2. Write each witness as a test first, with its bare-cardinal twin where one
   reads.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *or more* read as clausal
   *or*-coordination, *than one* read as a free Adjunct, or a plural head
   admitted under *more than one*, is a defect, not a gain.
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

- Comparative governors with *to*/*than* Complements outside the Determiner
  (*equal to*, *is less than*, *other than*):
  `english-v3-comparative-complements`.
- *with power 2 or less*, *with mana value X*: `english-v3-scalar-property-values`.
- *its power is 0 or less*: `english-v3-copular-scalar-location`.
- Cost modification (*costs {1} less*), *the greatest/least*, *tied for*.
- Clause-level hosts that fail independently (*as long as*, *only if*); count
  a face only when this construction is its last cause.

## Landing record

In addition to the standard record: per item, before/after counts stamped with
change ids; the Agreement observed for *one or more* and *more than one*
heads; the interface the numeral-*or*-comparative coordination exposes for
the two sibling hosts; timings as integer ns and ns/B with host load and
worker count.
