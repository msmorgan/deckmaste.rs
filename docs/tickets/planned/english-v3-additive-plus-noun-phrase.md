---
needs: []
---
# Read additive *plus* joining two noun phrases: those tokens plus a Clue token are created instead.

## Why

*plus* reads between arithmetic operands (*2 plus the number of …*, *that
many plus one*), but not between two ordinary noun phrases: "If one or more
tokens would be created under your control, those tokens plus a Clue token
are created instead." 0 against "… those tokens are created instead." 3+;
"Counter target spell unless its controller pays {2} plus an additional {1}
for each Faerie you control." 0 against "… pays {2} for each Faerie you
control." 1. Blocker report at trunk `qzprwuxz`
(`target/english-v3/blockers-wave9.txt`): keys `at:plus a` 4 and `at:plus an`
6 sole, `at:plus that` 1. `cargo xtask english-v3 grep '\bplus (a|an|that
many) '` finds **21** faces, all unread; **19** have that line as their only
failing line:

- subject, *those tokens plus* NP *are created instead*: 11 faces, all sole;
- *plus an additional* X *for each …* after a cost, counter or count (*pays
  {2} plus an additional {1}*, *with a +1/+1 counter on it plus an additional
  +1/+1 counter on it*, *once plus an additional time*): 15 touched with the
  subject cases that also say *an additional*; 13 sole.

## Goal

Two noun phrases joined by *plus* read as one noun phrase in subject, object
and cost position, plural for agreement, with byte-exact realisation.

## Analysis

CGEL Ch. 15 §2.9, p. 1318: *plus* "straddles the boundary between
prepositions and coordinators"; [75ii] "[*The cost-billing system*][*plus
other control refinements*] *has reduced the deficit*", [75iii] "[*His
stamina*][*plus his experience*] *make him unbeatable*"; "*Plus* is
predominantly followed by an NP, making it more like a preposition than a
coordinator." Ch. 7 §5, p. 635, lists *plus* among the prepositions taking
an object NP. Those back NP + *plus* NP as a unit and both agreement
patterns. Treating it as a PP inside the first NP (preposition analysis) in
this grammar, and exporting plural agreement, are project rulings.

Existing declarations: `deckmaste_lexical_source/lexicon/core.ron`,
`vocab:Preposition/Plus` (no features, no frames);
`crates/deckmaste_english_v3/src/declarations.rs`: `ArithmeticMeasure`
(`[left: ArithmeticOperand, " ", operator: lexical(Preposition), " ", right:
ArithmeticOperand]`) and `ArithmeticModifier` read arithmetic *plus*; the NP
postmodifier route reads a PP after a nominal when the preposition's
**Preposition Function Licence** contains the NP Modifier function.

Orchestrator rulings 2026-10-11:

- PL1. Data first, per the 2026-10-06 amendment (a PP's function is licensed
  per preposition as a declared property): give `Plus` a Preposition Function
  Licence admitting the NP-internal Modifier function with an NP Complement,
  so *those tokens plus a Clue token* reads through the existing postmodifier
  route. No new Construction (R-budget). Do not license *plus* as a clause
  Adjunct or a verb-selected Complement: neither is attested, and the licence
  is a list of functions, so leaving them out is not a guard.
- PL2. Agreement: the attested subject takes a plural verb (*those tokens
  plus a Clue token are created*); with a plural first NP the existing head
  agreement already gives that. Do not build singular-head + *plus* plural
  agreement (CGEL [75iii]): unattested, and no check against it either.
- PL3. Cost position (*pays {2} plus an additional {1} for each …*): the
  first operand is a mana cost, the second a determined cost NP. In scope if
  the cost Object is an NP that takes the PL1 postmodifier, or if
  `ArithmeticMeasure`'s operand category already admits a determined NP; if
  it needs a new operand form, add it as a form on `ArithmeticOperand`
  licensed by a declared feature of the head. If neither works, return this
  shape with its count; the subject shape lands alone.
- PL4. One analysis per string (R-one). *those tokens plus a Clue token* is
  one NP. It must not also read with *plus a Clue token* attached to the verb
  or the clause, and arithmetic *2 plus the number of cards in your hand*
  keeps exactly its present Readings: the arithmetic route and the NP route
  must not both read the same string (their operands differ by declared
  category: a scalar operand against a full NP; if an overlap appears, keep
  the arithmetic analysis for scalar operands and report the shape).
- PL5. Newly covered faces are gains. 110 covered faces contain *plus*
  (arithmetic); none is expected to gain Readings. Any increase there is the
  PL4 twin: report it with its shape; it is a defect unless both analyses are
  independently correct.
- PL6. *that many plus one {E}* (Izzet Generatorium; "you get that many plus
  one {E} instead" 0) is the delayed degree arithmetic of
  `english-v3-delayed-degree-arithmetic-modifier`: not this ticket.

## Witnesses

Each is the only failing line of its face in `blockers-wave9.json`. Whole
faces probed 2026-10-11 on the `qzprwuxz` binary (`--category document
--as-card`): all 0. Re-probe on the claim parent first.

- Case of the Pilfered Proof: "Solved — If one or more tokens would be
  created under your control, those tokens plus a Clue token are created
  instead." (sentence 0)
- Donatello, the Brains: "If one or more tokens would be created under your
  control, those tokens plus a Mutagen token are created instead."
- Peregrin Took: "… those tokens plus an additional Food token are created
  instead."
- Chatterfang, Squirrel General: "… those tokens plus that many 1/1 green
  Squirrel creature tokens are created instead." (sentence 0)
- PL3: Spell Stutter: "Counter target spell unless its controller pays {2}
  plus an additional {1} for each Faerie you control." (0)

Also 0: "This creature enters with a +1/+1 counter on it plus an additional
+1/+1 counter on it for each other creature you control." (Sheriff of Safe
Passage). Controls that must not change: "If one or more tokens would be
created under your control, those tokens are created instead." 3+ (re-probe
with a higher `--readings`); "Counter target spell unless its controller pays
{2} for each Faerie you control." 1; and the Reading counts of the 110
covered *plus* faces.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bplus\b` (202; the new licence is on the lexeme, so every
occurrence is selected) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; PL5 for increases. Full `cargo test -p
deckmaste_lexical_source` and `-p deckmaste_english_v3` with clippy `--tests
-- -D warnings`, once, at the end. Lexicon data loads at runtime; a grammar
edit (PL3) costs a 2–3 minute rebuild and the six test binaries about 75
seconds. Tests assert the role (the *plus* PP as Modifier inside the subject
NP, plural agreement from the head, the PL4 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only.

## Out of scope

- *that many plus one* / *that much damage plus 2* (delayed degree
  arithmetic, ticketed).
- *roll that many dice plus one and ignore the lowest roll* (needs *ignore*,
  `english-v3-small-frames-batch-3`).
- *once plus an additional time for each opponent* (Tempt with Mayhem): a
  frequency operand; report if PL3 reads it, do not build for it.
- Clause-level *plus* (CGEL [75v]): unattested.

## Landing record

To be written by the claimant: the six items of `docs/tickets/README.md`.
