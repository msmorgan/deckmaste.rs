---
needs: [english-v3-comparative-quantity-determiners]
---
# Read copular scalar location: its power is N, is N or less

## Why

Split from `english-v3-scalar-comparisons` (2026-10-06), which bundled four
constructions. This ticket owns one: *be* with a scalar value as predicative
Complement (*its power is 2*, *its mana value is 2 or less*, *The number of
cards in your hand is three.*). On change `wlvwtnppyovn` (32,828 supported
faces, 13,716 covered, 19,112 unread), the surface bucket overlaps the *N or
more* bucket (2,095 touched / 274 sole) and has no count of its own; probes
(admitted roots):

- "If that creature's power is 0, destroy it." 0
- "If that creature's power is 0 or less, destroy it." 0
- "The number of cards in your hand is three." 0 in either number on
  2026-10-06; the *becomes* version reads. A copula gap, not a concord one.

This ticket takes over from `english-v3-systemic-residuals` its "Copular
scalar location" item.

## Goal

A scalar value (numeral, *X*, or the numeral-*or*-comparative coordination
built by `english-v3-comparative-quantity-determiners`, the `needs:` edge)
reads as the predicative Complement of *be* when the Subject denotes a
quantity or scalar property. Reuse that coordination; do not build a second
one. Do not admit a value Complement after a Subject that denotes neither.

## Analysis

Simple location on a scale is commonly expressed by *be* with an NP as
predicative Complement: *The price is $12*, *The temperature is 10°*, *This
case is over 20 kilos* (CGEL, Ch. 8, §5.4, p. 693, [11]–[12]). *Its power is 0*
and *The number of cards in your hand is three* are this construction. The
numeral-*or*-comparative coordination is cited in the quantity-determiner
ticket (Ch. 5, §7.6, p. 386, [44iiia]).

The split ticket grouped *is equal to* and *is less than* with this host.
They are be + comparative AdjP, a different predicative Complement, and belong
to `english-v3-comparative-complements`.

## Witnesses

- Savage Swipe: "Target creature you control gets +2/+2 until end of turn if
  its power is 2."
- Stature, Size Shifter: "Stature can't be blocked if her power is 1 or less."
- Depressurize: "Then if that creature's power is 0 or less, destroy it."
- Domestication: "At the beginning of your end step, if enchanted creature's
  power is 4 or greater, sacrifice this Aura."
- Guidelight Pathmaker: "Put it onto the battlefield if its mana value is 2 or
  less."
- Technodrome: "This creature can't attack or block unless its power is 6 or
  greater."

Each witness's only failing unit is the one quoted (recon on `wlvwtnppyovn`).
Also pin the synthetic "The number of cards in your hand is three." in both
numbers.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/copular-scalar-location-before.json` on the
   claim parent, stamped with its change id.
2. Write each witness as a test first, with a negative probe whose Subject is
   not scalar (for example "If that creature is 2, destroy it.").
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *or less* read as clausal
   *or*-coordination, or a value Complement admitted after a non-scalar
   Subject, is a defect, not a gain.
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

- *is equal to*, *is less than*: `english-v3-comparative-complements`.
- *with power 2*, *with mana value X*: `english-v3-scalar-property-values`.
- *where X is* (1,119 touched / 114 sole) and *the number of* (1,296 / 9) as
  constructions: note how many become readable, but do not build for them.
  Magma Sliver's scalar Subject *X* (from the done
  `english-v3-number-transparent-concord`) stays with
  `english-v3-systemic-residuals`.
- Life-total comparisons (*your life total is less than …*) are comparatives,
  not this host.

## Landing record

In addition to the standard record: before/after counts stamped with change
ids; the Subject feature that licenses the value Complement; the status of
the synthetic number-of probe in both numbers; timings as integer ns and ns/B
with host load and worker count.
