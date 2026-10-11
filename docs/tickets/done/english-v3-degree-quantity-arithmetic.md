---
needs: []
---
# Read addition and subtraction on a degree-quantified Noun Phrase: that many plus one +1/+1 counters are put on it instead

## Why

*that many +1/+1 counters* and *twice that many +1/+1 counters* read; *that
many plus one +1/+1 counters* and *that many cards plus one* do not. Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): keys
`at:many plus` 15 and failing token `plus` 38 sole-blocker faces.
`cargo xtask english-v3 grep -i` counts, all unread:

| Shape | Pattern | Faces | Sole failing line |
|---|---|---|---|
| inside the Determiner: *that many plus one +1/+1 counters* | `that many plus (one\|two\|[0-9])` | 19 | 15 |
| delayed: *that many cards plus one*, *that many dice plus one*, *that many cards minus one* | `that many [a-z0-9/+ ]+ (plus\|minus) (one\|two\|[a-z]+)\b` | 21 | 15 |

Second causes inside those: 5 of the 19 continue *… plus one of each of
those kinds of counters* (the partitive is unread without the arithmetic
too); 4 of the 21 start *Discard all the cards in your hand*
(`english-v3-predeterminer-all`); the *roll that many dice plus one and
ignore the lowest roll* lines have other gaps. The clean set is about 14 +
7.

## Goal

A degree Determinative Phrase (*that many*) takes an addition or subtraction
phrase (*plus one*, *minus one*, *plus four*) either inside the Determiner,
before the noun, or delayed after the head noun, and the whole still
quantifies that noun, with byte-exact realisation and plural agreement.

## Analysis

CGEL Ch. 5 §11(f), p. 433, "PPs: condition, exception, proportion, and
addition/subtraction", [8iv]: "*There's room for* [*ninety thousand minus a
few hundred*] *spectators*"; "Here we have PPs in post-head modifier
function. There are alternants in which the PP is delayed, so that it is not
a constituent of the DP: … *There's room for ninety thousand spectators
minus a few hundred*." That backs both attested orders: the *plus* / *minus*
PP as post-head Modifier in the Determinative Phrase, and its delayed
alternant after the noun. CGEL's head there is a numeral; a degree
Determinative head (*that many*) and a variable are project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`DegreeDeterminativePhrase` (`QuantitativeDeterminer`, `[modifier:
lexical(Determinative), " ", head: lexical(Determinative)]`, `DegreeUse`
Modifier / Head) reads *that many*; `PrepositionDeterminer`
(`[value: QuantitativePrepositionPhrase]`) is the precedent for a PP inside
a quantitative Determiner; `ArithmeticMeasure` (`[left: ArithmeticOperand,
" ", operator: lexical(Preposition), " ", right: ArithmeticOperand]`,
`require operator.MeasureOperator = Yes`) reads *2 plus the number of …*;
*plus* and *minus* are `vocab:Preposition/Plus` / `Minus` with
`MeasureOperator: Yes`. Glossary: **Degree Modifier**, **Degree Use**,
**Arithmetic Operand**, **Measure Phrase**.

Orchestrator rulings 2026-10-10:

- DQ1. Inside the Determiner: refine `DegreeDeterminativePhrase` (or the
  `QuantitativeDeterminer` layer above it) with an optional post-head
  Modifier: a `MeasureOperator` Preposition with a cardinal Complement
  (*plus one*). The result is still a `QuantitativeDeterminer` with the
  degree head's features and plural agreement. No new Construction.
- DQ2. Delayed: the same PP after the head noun (*that many cards plus one*)
  is a post-head dependent of the Noun Phrase, admitted only when the NP's
  Determiner is a degree Determinative Phrase that could have hosted it
  (read from the Determiner's exported features, e.g. its `DegreeUse`-
  derived kind), so it is the DQ1 Modifier in its delayed position, not a
  general NP postmodifier. One form on an existing NP Construction; if it
  needs a new Construction, land DQ1 alone and STOP with the count.
- DQ3. The licence reads declared features: `MeasureOperator` on the
  Preposition, the degree head on the Determiner, the cardinal's
  `numeral_kind`. No test on *that*, *many*, *plus* or *one*. The
  Complement is a cardinal (spelled or digit) as attested; do not add a
  check to block longer operands.
- DQ4. One analysis per string. The twins to avoid: (a) *that many plus one
  +1/+1 counters* is not `ArithmeticMeasure` over an NP operand, and not
  *that many* (fused head) + a *plus* PP whose Complement is the NP *one
  +1/+1 counters*; (b) *that many cards plus one* does not attach *plus one*
  to the verb or Clause; (c) *2 plus the number of Swamps your opponents
  control* keeps its `ArithmeticMeasure` Readings (2 today); (d) *twice that
  many +1/+1 counters* keeps its Predeterminer Reading.
- DQ5. *the top X plus one cards*, *X minus one 2/2 … tokens* (3 faces): the
  same DQ1 Modifier on a variable Determiner. In scope if the refinement
  sits on the shared `QuantitativeDeterminer` layer; otherwise list them.
- DQ6. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face matches either pattern. An increase on a
  covered *that many* face (`(?i)\bthat (many|much)\b`: 590 faces) or a
  covered *plus* / *minus* face is a DQ4 defect; report it with its shape
  before landing.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--readings 3`): witness 0. Re-probe on the claim parent first.

- Hardened Scales: "If one or more +1/+1 counters would be put on a creature
  you control, that many plus one +1/+1 counters are put on it instead."
  (controls: without "plus one" 2; with "twice that many" 2)
- Conclave Mentor: "If one or more +1/+1 counters would be put on a creature
  you control, that many plus one +1/+1 counters are put on that creature
  instead."
- Kenessos, Priest of Thassa: "If you would scry a number of cards, scry
  that many cards plus one instead."
- Colossus of the Blood Age: "When this creature dies, discard any number of
  cards, then draw that many cards plus one."
- The Water Crystal: "If an opponent would mill one or more cards, they mill
  that many cards plus four instead."

Also 0 today: "that many plus one +1/+1 counters" (`noun-phrase`); "Draw
that many cards plus one." (control "Draw that many cards." 1). Controls
that must not change: "that many +1/+1 counters" (`noun-phrase`) 1; "Its
power is 2 plus the number of Swamps your opponents control." 2. The
without-arithmetic controls for Kenessos, Colossus and The Water Crystal,
and The Water Crystal's witness itself (sole failing line per the blocker
report), were not probed (the trunk moved under the prebuilt binary); probe
them on the claim parent.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bthat (many|much)\b|\b(plus|minus)\b` (748 faces) plus
300 random covered controls (`--random 300 --seed 20261010`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; DQ6 for increases. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests --
-D warnings`, once, at the end: a rebuild after a grammar edit takes about
2–3 minutes and the six test binaries about 75 seconds, so batch edits.
Tests assert the role (the *plus* PP as Modifier within the Determinative
Phrase; its delayed position as a dependent tied to that Determiner; the
noun still quantified and plural; the DQ4 negatives), never an incidental
count or a Debug-printed tree. Attestation by `cargo xtask english-v3 grep`
only. Glossary: extend **Degree Modifier** / add the addition PP (CGEL Ch. 5
§11(f), p. 433) in the landing.

## Out of scope

- *that many plus one of each of those kinds of counters* (5 faces): the
  partitive after a degree Determinative Phrase is unread without the
  arithmetic.
- *one plus the number of …*, *X plus one*
  (`english-v3-spelled-cardinal-scalar-measure`).
- *that many times plus an additional time* (Twinning Staff); *plus an
  additional {1} for each …*; *those tokens plus a … token*.
- *exactly that many colors plus one* (Evolving Door).

## Landing record

1. **Census delta** (`lvsykyns`, clean claim-parent grammar `yytzzztu` →
   implementation, rechecked after refresh at `zospqmwy`, `xlovkpvk` and `ukysysvw`): bounded selector `(?i)\bthat (many|much)\b|\b(plus|minus)\b`,
   748 matching faces plus 300 covered controls selected by `english-v3 diff
   --manifest --random 300 --seed 20261008`; 1,048 faces. Covered 824 → 841;
   Readings 20,981 → 21,131. Lost: none; gained 17; decreases 0; increases 0;
   digest-only 414 (the optional post-head Modifier slot added to existing
   degree Determinative Phrases); unchanged 617. Zero issues, limited enumerations or internal
   failures. Compared with `cargo xtask english-v3 diff before.json after.json
   --json diff.json --lines`; evidence and the identity manifest are under
   `target/english-v3/english-v3-degree-quantity-arithmetic/`. Refresh preserves
   every count; its two additional digest changes (Galvanic Discharge and
   Harnessed Lightning) belong to quantified energy symbols, `tznkrzpw`,
   as shown by the official pre-refresh → refreshed `trunk-diff.json`.
   The integration refresh brought `ropwqyov` (difference noun) and
   `wxxvsntt` (starting-with adjunct); `trunk-integration-diff.json` reports
   all 1,048 selected faces unchanged from the reviewed census. Integration
   was then refused because default moved; the retry refresh brought the
   devour-quality landing (`ulkrnzyt`, `vuyokrvx`, `pknkolqq`). Its official
   `trunk-devour-diff.json` also reports all 1,048 faces unchanged.
2. **Witnesses** (`lvsykyns`, complete direct probes before → after, with
   no-arithmetic control counts): Hardened Scales 0 → 2 / control 2;
   Conclave Mentor 0 → 2 / 2; Kenessos 0 → 0 / 1; Colossus of the Blood Age
   0 → 0 / 3; The Water Crystal 0 → 0 / 1; the counter NP 0 → 1 / 1;
   `Draw that many cards plus one.` 0 → 0 / 1. All failed before editing;
   none were dropped as already covered. DQ4 controls retain scalar arithmetic
   2, multiplier 2 and plain counter NP 1; Torbran's scalar amount retains 1.
   W9-A: the gained arithmetic lines on all 17 faces retain their respective
   no-arithmetic control counts; independent ambiguity remains.
3. **Spot checks** (`lvsykyns`): all 17 gained faces were probed; ten selected
   analyses are Benevolent Hydra, Caradora, Heart of Alacria, Conclave Mentor,
   Hardened Scales, High Score, Kami of Whispered Hopes, Mauhúr, Uruk-hai
   Captain and Michelangelo, Weirdness to 11 (the PP is internal to the degree
   Determiner quantifying the plural counter Nominal), and Doc Samson, Super
   Psychiatrist and Lae'zel, Vlaakith's Champion (the same internal PP with
   the existing partitive NP). Wrong-analysis-only gains: 0.
4. **Deviations and STOPs** (`lvsykyns`): DQ1 alone, per DQ2's explicit STOP
   ruling. The compiler requires each Construction form to retain the same
   named fields and cardinalities; optional syntactic fields have no single
   feature value for cross-field licensing. An existing QuantifiedNounPhrase
   form cannot acquire the delayed dependent with the required Determiner
   licence; its separate route needs a new Construction. The Method's delayed
   grep still finds 21 unread faces; the proposal and named inventory are in
   [the delayed arithmetic follow-up](../planned/english-v3-delayed-degree-arithmetic-modifier.md).
   DQ5 variable Determiners remain deferred:
   Green Sun's Twilight, Epiphany at the Drownyard and Eiganjo Uprising.
   No named Construction, schema or frame was added or retired: the existing
   QuantitativePrepositionPhrase schema supplies an Arithmetic Modifier
   Category, and DegreeDeterminativePhrase takes it optionally. Cardinal
   Complement Use derives from numeral kind and retains cardinal Coordination;
   no operand-length restriction was added. Both glossary terms are recorded
   with CGEL's PP account distinguished from DQ1–DQ3's project rulings.
   Three partitive faces also gain through the existing route (Doc Samson,
   Lae'zel and Winding Constrictor); no partitive rule changed. The initial
   claim STOP on another workspace's divergence was resolved by the orchestrator.
5. **Tests** (`lvsykyns`): `cargo test -p deckmaste_english_v3` PASS, 1,020
   tests before refresh, 0 failures/ignores. Its first run caught the new test's repeated
   counter surface; the assertion now explicitly selects its second
   occurrence. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   PASS after replacing three diagnostic/style assertions with full value
   comparisons; `cargo test -p deckmaste_english_v3 --test suite_6
   degree_quantity_arithmetic::` PASS, all 5 tests on those final assertions.
   Restored 0 / re-spelled 0 / ignored 0 / added 5 / removed 0; no removals.
   After refresh, `cargo build -p xtask` PASS; full suites via
   `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source -p xtask
   -p deckmaste_semantics_v2 -p deckmaste_construction_core` PASS
   (English 1,030 tests). Clippy for the same five packages with
   `--tests -- -D warnings` PASS. Incoming plugin/Lean changes also required
   `cargo xtask lean check`: PASS, 242/242 cards. Before integration,
   another `cargo build -p xtask`, `cargo test -p deckmaste_english_v3
   -p deckmaste_lexical_source` and `cargo clippy -p deckmaste_english_v3
   -p deckmaste_lexical_source --tests -- -D warnings` all PASS on the
   grammar/lexicon refresh. After the devour-quality retry refresh,
   `cargo build -p xtask` and the full five-package test/clippy commands above
   PASS again, including full xtask, construction core and semantics v2.
   `cargo xtask lean check` PASS again, 243/243 cards; citation checks remain
   clean. The new planned ticket passes `kata kanban check`.
   Two existing fixture builders set the new optional field to None without
   changing assertions. Citation checks: 0 noncompliant, 0 stale;
   `jj --no-pager diff --git | cargo xtask cite audit --diff` selected 0 sites.
6. **Timing** (`lvsykyns`): the bounded AFTER census took 19.30 s wall before
   refresh, 31.90 s after review refresh and 67.90 s after integration refresh
   and 1:15.21 after the devour retry refresh, all under `/usr/bin/time -v`, 6 workers,
   `--samples-per-face 0`. No full census ran.
