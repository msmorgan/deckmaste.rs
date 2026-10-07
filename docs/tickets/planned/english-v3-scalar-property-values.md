---
needs: [english-v3-comparative-quantity-determiners]
---
# Read scalar property values: with power N, with mana value X

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: a scalar property noun (*power*,
*toughness*, *mana value*) immediately followed by its value, inside a Noun
Phrase (*with power 2*, *with mana value X*, *with power 4 or greater*). This
is not a comparison; the value may be a bare numeral or a
numeral-*or*-comparative coordination. On change `wlvwtnppyovn` (32,828
supported faces, 13,716 covered, 19,112 unread), as faces touched / faces
where the bucket is the only recognised cause (overlapping buckets, not gain
forecasts):

| Item | Touched / sole | Probe (admitted roots) |
|---|---|---|
| *mana value* (surface bucket) | 1,124 / 39 | "… with mana value 2." 0; "… with mana value X." 0 |
| *with power N or less/greater* | (in the *N or more* bucket) | "Destroy target creature with power 2." 1; "… with power 2 or less." 0 |

The probes show the bare-numeral value already reads after *power* but not
after *mana value*, and no value reads as a coordination.

## Goal

*with power 2 or less*, *with toughness 4 or greater*, *with mana value 1* and
*with mana value X* read with the value as a dependent of the property noun,
the PP headed by *with* modifying the target Nominal. The *2 or less* value is
the numeral-*or*-comparative coordination built by
`english-v3-comparative-quantity-determiners` (the `needs:` edge): reuse it,
do not build a second one. Make *mana value* and *power* take their value
through one route; record whether the existing *power 2* route is that route
or is superseded (Method 10).

## Analysis

CGEL recognises NP post-head modifiers denoting age, size and similar
properties (*a man my age*, *shoes this size*; Ch. 5, §14.2(c), p. 446, [13i]),
but that passage does not discuss a bare numeral following a property noun,
and no CGEL passage located for this ticket does. The structure of *power 2*
is therefore a decision this landing makes and records as a project ruling,
not a CGEL citation. The numeral-*or*-comparative coordination is cited in
the quantity-determiner ticket (Ch. 5, §7.6, p. 386, [44iiia]); here it fills
the value position, not a Determiner.

## Witnesses

- Disembowel: "Destroy target creature with mana value X."
- Mental Misstep: "Counter target spell with mana value 1."
- Easy Prey: "Destroy target creature with mana value 2 or less."
- Kor Line-Slinger: "{T}: Tap target creature with power 3 or less."
- Retribution of the Meek: "Destroy all creatures with power 4 or greater."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also usable: Valorous Stance (*toughness 4 or greater*), Here Comes a New
Hero! (*mana value X or less*), Rigo, Streetwise Mentor (with *one or more*,
after the prerequisite lands).

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/scalar-property-values-before.json` on the claim
   parent, stamped with its change id.
2. Write each witness as a test first, with its bare-numeral twin.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *or less* read as clausal
   *or*-coordination, or the value read as a Determiner of a following noun,
   is a defect, not a gain.
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

- *its power is 2*, *its mana value is 2 or less* (be + value):
  `english-v3-copular-scalar-location`.
- *equal to*, *less than*, *other than*: `english-v3-comparative-complements`.
- *three or more creatures*, *more than one creature*:
  `english-v3-comparative-quantity-determiners`.
- *base power and toughness 0/2*, *the greatest mana value among*, cost
  modification (*costs {1} less*).

## Landing record

In addition to the standard record: before/after counts per property noun
stamped with change ids; the ruling recorded for the *power 2* structure and
which route survived; timings as integer ns and ns/B with host load and
worker count.
