---
needs: []
---
# Read a scalar value after modal *be* and after *become*: X can't be 0; your life total becomes 1.

## Why

`english-v3-copular-scalar-location` (landed) reads a scalar value as
predicative Complement of finite *be* when the Subject is scalar ("X is 0."
1; "Its power is 0." 1; "Your life total is 1." 1). Three neighbours fail.
(a) The plain form under a modal: "X can't be 0." 0, "X can be 0." 0, while
"X can't be greater than 3." 1. (b) *become*: "Your life total becomes 1." 0,
"Its power becomes 0." 0, while "Your life total becomes half your starting
life total." 1 and "Its power becomes equal to 3." 1. (c) A Subject noun
that lacks the scalar declaration: "Your maximum hand size is two." 0, while
"Your maximum hand size is reduced by four." 1. Blocker report at trunk
`qzprwuxz` (`target/english-v3/blockers-wave9.json`): `at:0 .` (7 of 9),
`at:1 .` (5 of 12), and the one-face keys `at:two .`, `at:eight .`,
`at:ten .`. `cargo xtask english-v3 grep -i "\b(can't|can|must) be \d"`:
**17** faces, all unread, **15** sole. `grep -i '\bbecomes? \d+([., ]|$)'`:
**20** faces, all unread, **15** sole (12 are *life total becomes N*).
`grep -i 'maximum hand size is \w+\.'`: **8** faces, all unread, **8** sole.
Together **38** sole-blocker faces. `english-v3-hygiene-3` listed "life total
becomes N 12" as unrouted.

## Goal

The scalar-location Complement reads after the plain form of *be* under an
auxiliary and after *become*, for the same scalar Subjects, and *maximum hand
size* is declared scalar like the other scalar property nouns. Byte-exact.

## Analysis

CGEL Ch. 8 §5.4, p. 693: "Simple location on a scale is commonly expressed by
*be* with an NP as predicative complement" ([11] *The temperature is 10°*).
That backs the Complement after *be* in any form. Treating *become* + value
as the change-of-state counterpart of the same Complement is a project
ruling, as is the Subject restriction (already recorded in the code comment
on `ScalarLocation`).

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`):
construction `ScalarLocation: FiniteClause` (`[subject: NominativePhrase,
…, predicate: ScalarLocationPredicate]`, `require subject.ScalarDenotation =
Yes`); schema `ScalarLocationPredicate` (`[head: lexical(Verb), complements:
selected_frame(head, ScalarLocation)]`) with instances for
`FiniteHeadAgreement` and, since IS1 of `english-v3-irrealis-scalar-difference`,
`IrrealisScalarHeadAgreement`; `QuantifiedScalarLocation`. The frame
`ScalarLocation` is selected by *be* only; *become*
(`lexicon/verbs.ron`) declares `Predicate([PredicativeComplement])` alone.
`ScalarDenotation` is a declared noun feature.

Orchestrator rulings 2026-10-11:

- SL1. Refine the existing schema: add instances of `ScalarLocationPredicate`
  for the plain-form head under an auxiliary (the way IS1 added the irrealis
  head) and let *become* select the `ScalarLocation` frame in the lexicon. No
  new Construction (R-budget); if the modal route needs a clause-level
  projection that the schema cannot express, STOP with the shape.
- SL2. The Subject requirement stays `ScalarDenotation = Yes`, read wherever
  the clause is assembled; a modal or *become* must not open the Complement to
  non-scalar Subjects (*It can't be 0.* stays 0 unless *it* already satisfies
  the finite route: probe "It is 0." first and keep parity with it).
- SL3 (R-class). Every auxiliary of the modal class licenses the plain-form
  head (*can*, *can't*, *must*, *may*…), not the attested *can't*; every
  scalar value the finite route takes is taken (numeral, *X*, spelled
  cardinal, *N or less*). Among verbs, the frame is declared on *become*
  only: it is the one change-of-state copula in the lexicon; mint no Lexeme.
- SL4. Data: declare `ScalarDenotation` for *maximum hand size* the way
  *life total*, *power*, *mana value* declare it (find the declaration first:
  if scalar denotation is derived from a noun class rather than listed, put
  *hand size* in that class). 8 sole faces; lands with or without SL1.
- SL5 (R-one). One analysis per string. The twins to avoid: (a) the value as
  an ordinary predicative NP through *become*'s `PredicativeComplement` frame
  (if "Its power becomes 0." would read both ways, the scalar frame is the
  one kept and the NP predicative must not admit a bare numeral: check that
  it does not today, it reads 0); (b) *X can't be 0* as passive or
  progressive *be* with an elided participle.
- SL6. Newly covered faces are gains. R-reach: `ScalarLocationPredicate`
  gains instances and *become* gains a frame, so the selector includes every
  face with *become(s)* or a modal + *be*. No Reading-count increase is
  expected on a covered face; report any with its shape (most likely: a
  covered *becomes* + NP line gaining a scalar Reading, which is a defect
  unless the NP is a scalar value).

## Witnesses

Each is a sentence (or the line) of the face's only failing line. Probed
2026-10-11 with the trunk binary (`--readings 8`): all 0.

- Aladdin's Lamp: "X can't be 0."
- Gogo, Master of Mimicry: "This ability can't be copied and X can't be 0."
- Worldfire: "Each player's life total becomes 1."
- Vraska, Relic Seeker (Document): "−10: Target player's life total becomes
  1."
- Null Profusion: "Your maximum hand size is two."

Also 0: "The chosen player's maximum hand size is four." (Cursed Rack).
Controls that must not change: "X is 0." 1; "Its power is 0." 1; "Your life
total is 1." 1; "X can't be greater than 3." 1; "Your life total becomes half
your starting life total." 1; "Its power becomes equal to 3." 1; "You have no
maximum hand size." 1; "It can't be a creature." 1; "It can't be blocked." 2;
"It becomes a 1/1 creature." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(become|becomes|became)\b|\b(can|can't|must|may|could|would)
be\b|maximum hand size` plus 300 random covered controls (`--random 300
--seed 20261008`), `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; SL6 for changes.
Full `cargo test -p deckmaste_english_v3`, `-p deckmaste_lexical_source` and
clippy `--tests -- -D warnings` for both, once, at the end (a grammar rebuild
takes 2–3 minutes, the six test binaries about 75 seconds; lexicon data loads
at runtime). Tests assert the role (the value is the scalar-location
Complement of the plain-form or *become* head; SL2 and SL5 negatives), never
an incidental count or a Debug-printed tree. Attestation by `cargo xtask
english-v3 grep` only.

## Out of scope

- P/T values as predicative (*except it's 1/1*, *base power and toughness
  become 0/2*): `english-v3-slash-pair-predicative`.
- *The escape cost is equal to its mana cost plus exile three other cards*
  (3 faces); *its mana cost reduced by {2}* (7 sole).
- *instead … and your life total becomes 1* (Stunning Reversal, The Golden
  Throne): they also need the preposed *instead* coordination; gains only if
  it falls out.

## Landing record
