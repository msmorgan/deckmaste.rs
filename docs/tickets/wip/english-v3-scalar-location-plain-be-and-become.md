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

1. **Census delta — `lpnkpkpu`, against claim-parent tree `vukukmwz`.**
   Bounded manifest: 3,353 faces, comprising the broad Method pattern
   (extended to every declared modal spelling and all *size* uses), all 129
   baseline faces using `ScalarLocationPredicate` or
   `QuantifiedScalarLocation`, and 300 random covered controls, seed
   20261008. The grep union has 2,955 faces; plural *sizes* has 0 supported
   faces. BEFORE joins the 3,352-face claim-parent run and its one additional
   quantified-scalar face, Angry Mob; component reports remain under target/.
   Covered faces **3,049 → 3,076**; total Readings **99,257 → 99,620**.
   `cargo xtask english-v3 diff --lines`: gained 27; lost **none**;
   decreases 0; increases on previously covered faces 0; digest-only 0
   (no classes or lines); unchanged 3,326. All 60 changed line entries
   belong to gains. Wrong-analysis gains observed: 0. Evidence:
   `target/english-v3/english-v3-scalar-location-plain-be-and-become/{manifest,before,after,diff}.json`.
   Post-refresh, the same bounded manifest gives covered **3,049 → 3,080**,
   Readings **99,257 → 101,754**, gained 31, lost/decreases/increases/
   digest-only all 0, unchanged 3,322, and 66 changed lines. The official
   pre/post-refresh diff attributes four additional gains (Azorius Herald,
   Batwing Brume, Boros Fury-Shield, Mythos of Vadrok), 2,134 Readings and
   six changed lines to trunk's mana-passive landing `onymknyk`; all 27
   own gains retain their counts and digests. No digest-only classes/lines.
   Evidence: `after-post-refresh.json`, `diff-post-refresh.json`, and
   `diff-refresh-only.json` in the same directory.
2. **Witnesses — `lpnkpkpu`.** Re-probed before implementation: Aladdin's
   Lamp, Gogo's final sentence, Worldfire's life-total sentence, Vraska's
   −10 line, Null Profusion's hand-size sentence, and Cursed Rack's hand-size
   sentence each **0 → 1**; none dropped. The three additional neighbour
   probes (*X can be 0*, *your life total becomes 1*, *its power becomes 0*)
   also give 0 → 1. All ten listed controls retain their claim-parent
   counts: 1 each except *It can't be blocked* at 2. *It is 0*, *It can't
   be 0*, and *It becomes 0* remain 0. Full-document W9-A checks:
   Aladdin's Lamp witness/control **96/96**, Gogo **8/8**, replacing only
   *X can't be 0* with the existing *X can't be greater than 3* route.
3. **Spot checks — `lpnkpkpu`.** Ten newly covered faces, census Reading
   counts in parentheses: Aladdin's Lamp (96) and Gogo (8), auxiliary with
   an overt plain scalar-location predicate; Worldfire (6), Vraska (8),
   Blessed Wind (1), and Sorin Markov (3), *become* selecting the life-total
   scalar value; Null Profusion (3) and Cursed Rack (2), scalar *hand size*
   with finite *be*; Stunning Reversal (25) and The Golden Throne (6), the
   same *become* value under their existing *instead* coordination. These
   last two gains fall out without adding an *instead* route. Inspected
   Readings retain the selected scalar role and render byte-exactly.
4. **Deviations and STOPs — `lpnkpkpu`.** No new Construction, Lexeme,
   loader code, or superseded route. Two Categories and one plain-head
   policy connect existing scalar, overt-complement, and auxiliary schemas;
   the scalar category gains the two boundary exports required by the
   auxiliary instance. Its Subject check stays in `ScalarLocation`.
   The optional-auxiliary attempt was rejected by the compiler's unique
   result-category rule; the schema-instance route resolves this without a
   clause projection or STOP. *Hand size* inherits Scalar Denotation from
   its declared *size* head. *Become*'s frame is in core.ron's native frame
   additions because verbs.ron's frame enum cannot express ScalarLocation.
   The unused boundary projections are recorded in hygiene-3 section B.
   Initial citation cleanup of two pre-existing decimal timings in the
   planned predicative-gap-relative ticket conflicted with trunk's own
   cleanup during refresh; retained trunk's wording, leaving no feature
   diff in that ticket. The first refresh exited 69 on this one docs
   conflict; it was resolved at `lpnkpkpu`, and retry exited 0. Incoming
   grammar/lexicon changes are the mana-passive landing `onymknyk`.
   No glossary gap; no unresolved STOP.
5. **Tests — `lpnkpkpu`.** `cargo test -p deckmaste_english_v3` initially
   failed only on the new Vraska assertion's ambiguous substring *1*
   (also present in *−10*). Direct diagnostic:
   `target/debug/deps/suite_2-f75055dd7abcea6b --exact scalar_location_plain_be_and_become::witnesses_keep_scalar_location_complement_roles --nocapture`
   confirmed that span-selection error. The assertion now explicitly
   selects the value's second occurrence with the same role and value.
   `cargo test -p deckmaste_english_v3 -- --test-threads 6`: **passed**,
   1,138 tests, including all five new tests and every selected-complement
   budget check; one inherited MS4 ignore remains, with its named blocker.
   `cargo test -p deckmaste_lexical_source`: **passed**, 44 tests.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`: **passed**.
   Restored 0 / re-spelled existing tests 0 / newly ignored 0 / added 5 /
   removed 0; no removal to justify. Existing independently constructed
   expected values are unchanged. `cargo build -p xtask` passed;
   citation checks: 0 noncompliant, 0 stale, diff audit 0 new sites.
   Post-refresh, repeated `cargo build -p xtask`,
   `cargo test -p deckmaste_english_v3 -- --test-threads 6` (1,144 passed,
   one inherited MS4 ignore), `cargo test -p deckmaste_lexical_source`
   (44 passed), `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`:
   **all passed**, including the unchanged selected-complement ceilings.
   Repeated citation checks/audit: 0 noncompliant, 0 stale, 0 new sites.
   No plugins_v2 changes, Lean checks, or full-corpus census.
6. **Timing — `lpnkpkpu`.** Bounded AFTER invocation: **7 min 2 s 420 ms**
   wall under `/usr/bin/time -v`, 6 workers, samples per face 0; includes
   waiting for Cargo's test-build lock. Post-refresh bounded AFTER:
   **6 min 3 s 280 ms** wall with the same workers/sample settings,
   including Cargo's build-lock wait. No `--all` run.
