---
needs: []
---
# Read a spelled cardinal as a scalar operand and comparison standard: where X is one plus the number of Foods you control

## Why

A numeral written in digits is a scalar Measure Phrase; the same numeral
spelled out is not, so arithmetic and comparison fail on it. Blocker report
at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`): keys `at:one
plus` 10 and `at:is less` 21 sole-blocker faces. Probes: "Its power is less
than 5." 1, "Its power is less than five." 0; "… where X is 1 plus the number
of opponents who control an artifact." 1, "… one plus …" 0;
`arithmetic-operand` "1" 1, "one" 0; `scalar-measure-phrase` "5" 1, "five" 0.
The spelled cardinal does read alone as a value ("Its power is five." 1,
"Its power is five or greater." 1).

`cargo xtask english-v3 grep -i` counts, all unread:

| Shape | Pattern | Faces | Sole failing line |
|---|---|---|---|
| *one plus the number of …* | `\b(one\|two\|three\|four\|five) (plus\|minus)\b` | 17 | 16 |
| *is less than five / seven* | `\b(is\|are) (less\|greater) than (one\|…\|ten)\b` | 22 | 21 |
| *to be greater than four* | (Clockwork Steed, Clockwork Avian) | 2 | 2 |
| *<scalar NP> plus / minus one* | (Sea Gate Restoration, Kyren Toy, Proft's Eidetic Memory) | 3 | 3 |

The 22 *is less than five* faces are the devotion Gods and also need
`english-v3-devotion-to-colour`; this ticket alone is expected to cover about
20 faces.

## Goal

A spelled cardinal fills the two scalar positions a digit numeral fills:
Arithmetic Operand (*one plus the number of …*, *… plus one*) and the
standard of a scalar comparison (*less than five*, *greater than four*),
with byte-exact realisation.

## Analysis

No CGEL citation is relied on: that a cardinal numeral denotes a number is
not in dispute, and which positions Oracle English fills with the spelled
form is a project ruling. (CGEL Ch. 7 §5.1, p. 635, [1i] lists *plus* and
*minus* among the prepositions; that is already the grammar's analysis.)

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`UngroupedScalarNumeral` (`MeasurePhrase`, `require head.numeral_kind =
Arabic`) feeds `ScalarMeasurePhrase`, which is what
`ScalarComparativeComplement` (`[value: ScalarMeasurePhrase]`) and
`ScalarMeasureOperand` (`ArithmeticOperand`, `[value: MeasurePhrase]`) take;
`ArithmeticMeasure` joins two operands with a `MeasureOperator` Preposition.
The spelled cardinal (`numeral_kind = Cardinal`) reads through
`ScalarCardinalValue` (`ScalarValue`), `CardinalAmount` and the `Cardinal`
category. The house pattern for "digit or spelled" is a sibling pair:
`ScalarExtentMeasure` / `CardinalExtentMeasure`, `ScalarAmount` /
`CardinalAmount`. Glossary: **Arithmetic Operand**, **Comparative
Complement**, **Scalar Value**.

Orchestrator rulings 2026-10-10:

- SC1. Follow the sibling pattern at exactly two sites: `ArithmeticOperand`
  and `ComparativeComplement` each admit a spelled cardinal
  (`numeral_kind = Cardinal`). Prefer a second form on the existing
  Construction (`ScalarMeasureOperand`, `ScalarComparativeComplement`); a
  sibling Construction per site, as `CardinalExtentMeasure`, is the
  permitted fallback. Do not widen `UngroupedScalarNumeral` or
  `MeasurePhrase` itself: that would put a spelled cardinal wherever a
  measure reads (*three damage*, *five life*) and twin every cardinal
  Determiner.
- SC2. The licence is the Numeral's declared `numeral_kind`; no list of
  number words.
- SC3. One analysis per string. The twins to avoid: (a) *Its power is five.*
  and *five or greater* keep exactly their Readings (`ScalarCardinalValue`
  and `AdjectivalScalarCoordination` stay the only routes; the new operand
  is not a `ScalarValue` on its own); (b) *more than one creature*, *fewer
  than seven cards* keep the Comparative Quantity Determiner Reading only
  (no comparative Adjective Phrase + noun twin); (c) *one plus the number of
  Foods you control* has the cardinal as left operand, not a Determiner of a
  fused head followed by a *plus* PP.
- SC4. Both operand orders are in scope (*one plus N*, *N plus one*, *N
  minus one*) when the other operand already reads as an Arithmetic Operand.
  Arithmetic inside a Determiner (*that many plus one counters*, *the top X
  plus one cards*) is `english-v3-degree-quantity-arithmetic`.
- SC5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected; an increase on a face with a spelled cardinal after
  *than* means SC3(b) leaked. Report any increase with its shape before
  landing.

- SC6. Orchestrator ruling 2026-10-10, applying W9-A (the control is the
  measure): compare Burn at the Stake's otherwise identical digit-3 line
  shape by shape. If the digit control has the split Readings (*equal to N*
  closes the comparison; *times the number of …* is deal's second Object),
  accept the six additions under SC5 as expected by control. Record the
  overgeneration candidate for both spellings in the systemic residuals
  ticket if unowned, otherwise here. If the digit lacks those Readings,
  align the new licence with its site and exported features, without a word
  test. Tests compare witness and digit-control values rather than counts.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category document --readings 3`): witness 0. Re-probe on the claim
parent first.

- Journey On: "Create X Map tokens, where X is one plus the number of
  opponents who control an artifact." (control with "1" 1; without "one
  plus" 1)
- Welcome to Sweettooth: "III — Put X +1/+1 counters on target creature you
  control, where X is one plus the number of Foods you control." (sentence
  without "one plus" 2)
- Boss's Chauffeur: "This creature enters with a number of +1/+1 counters on
  it equal to one plus the number of other creatures you control." (without
  "one plus" 3+)
- Mind Burst: "Target player discards X cards, where X is one plus the
  number of cards named Mind Burst in all graveyards." (without "one plus"
  3+)
- Clockwork Steed: "… This ability can't cause the total number of +1/+0
  counters on this creature to be greater than four. …" (may have a second
  cause; report)

Joint with the devotion ticket, 0 today: "Its power is less than five."
(control "… less than 5." 1); "Your life total is less than seven."
(control "Your life total is less than 5." 1).
Controls that must not change: "Its power is five." 1; "Its power is five or
greater." 1; "Your life total is seven or greater." 1; "If you have fewer
than seven cards in hand, draw a card." 2; "You can't cast more than one
spell each turn." 2; "Its power is 2 plus the number of Swamps your
opponents control." 2.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\b(plus|minus)\b|\b(less|greater) than\b` (485 faces) or
`(?i)\b(than|plus|minus|equal to) (one|two|three|four|five|six|seven|eight|nine|ten)\b`
(part of 189 faces), plus 300 random covered controls (`--random 300 --seed
20261010`), `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; SC5 for increases.
Full `cargo test -p deckmaste_english_v3` and `cargo clippy -p
deckmaste_english_v3 --tests -- -D warnings`, once, at the end: a rebuild
after a grammar edit takes about 2–3 minutes and the six test binaries about
75 seconds, so batch edits. Tests assert the role (the spelled cardinal as
Arithmetic Operand; as the Complement of the comparative governor; the SC3
negatives), never an incidental count or a Debug-printed tree. Attestation
by `cargo xtask english-v3 grep` only.

## Out of scope

- *devotion to <colour>* (`english-v3-devotion-to-colour`).
- *that many plus one counters*, *that many cards plus one*, *X plus one
  cards* (`english-v3-degree-quantity-arithmetic`).
- *less than three mana*, *less than one mana* (Trinisphere, Convergence of
  Dominion): a spelled measured amount after *than*.
- *plus an additional …*, *those tokens plus a … token* (about 12 faces).

## Landing record

1. **Census delta** — Bounded selector:
   `(?i)\b(plus|minus)\b|\b(less|greater) than\b|\b(than|plus|minus|equal to) (one|two|three|four|five|six|seven|eight|nine|ten)\b`
   matches 572 faces; plus 300 covered controls from `english-v3 diff
   --manifest --random 300 --seed 20261008` gives 869 distinct faces.
   BEFORE is empty `tsvuoyup` on claim parent `ymtxrrsu`; pre-refresh AFTER
   `tsvuoyup` has 602 → 621 covered, 90,100 → 91,276 Readings and 19 gains.
   First refreshed `xnnmkups` has 602 → 626 covered, 90,100 → 91,359 Readings,
   24 gains, lost none, decreases none, digest-only none and issues none.
   `refresh-diff.json` isolates five gains and 83 Readings: Clockwork Avian,
   Beast, Steed and Swarm (jointly enabled by imported cause `nzmrrqky` and
   this ticket), and The Moment (imported phase `roomopmr`). No increase,
   decrease, loss or digest-only change came from that refresh. The last
   refresh imported kind-of-counter `wkooyuko`; bounded `kyymvuqx` has
   zero changed faces or lines against `xnnmkups` (`refresh-2-diff.json`).
   Acceptance refresh imported `mpvousxzotns`, `rtzolrtxrrru`,
   `mzkvyunrqtrw`, `ztrnpkyvzntv`, `vlxrulqwmzzk`, `opxvnokryrro`,
   `wxwozrtzrsqp` and `qktrquumtlmk`. `accepted-refresh-diff.json` reports
   five gains, 598 additional Readings, no loss/decrease/increase, and nine
   digest-only changes. Final `accepted-final-diff.json`: **29 gains**,
   602 → 631 covered, 90,100 → 91,957 Readings, zero lost/decreased faces,
   one SC6 increase and nine digest-only changes. The extra gains are Arcane
   Endeavor (292) and Valiant Endeavor (168), enabled by die nouns
   `vlxrulqwmzzk`; Desperate Plea (2), enabled by modal quantities
   `ztrnpkyvzntv`; Ojer Axonil, Deepest Might (117), enabled by noncombat
   `mzkvyunrqtrw`; and Tahngarth, First Mate (19), enabled by progressive
   Object-gap relatives `opxvnokryrro`. The modal count-Complement refinement
   `ztrnpkyvzntv` accounts for unchanged-count digests of Arbalest Engineers,
   Drown in the Loch, Feast or Famine, Light of Hope, Nightmare Unmaking,
   Professor's Warning, Shriveling Rot, Tetsuo, Imperial Champion and
   Vannifar, Evolved Enigma. These refresh changes are attributed to trunk;
   the ticket's initial 19 gains remain.
   The only increase is **SC5 expected by SC6(a)**: Burn at the Stake
   39 → 45 (line 1: 36 → 42). Six new Readings close the comparison at
   *equal to three* and take *times the number of creatures tapped* as
   deal's second Object, with *the number of creatures* the Subject of
   preterite *tapped*. Normalizing only the new cardinal Complement to the
   digit scalar route makes the six complete Reading values equal to the
   digit-3 control's set. Both `xnnmkups` and `kyymvuqx` probes retain
   spelled-line 42 and digit-line 6; the other 36 spelled-line Readings and three other face
   Readings remain. Evidence is under
   `target/english-v3/english-v3-spelled-cardinal-scalar-measure/`;
   `diff.json`, `refresh-diff.json` and `final-diff.json` are the stock tool's
   comparisons, including per-line attribution.
2. **Witnesses** — Claim-parent `ymtxrrsu` (probes on empty `tsvuoyup`) →
   implementation `tsvuoyup`: Journey On 0 → 1; Welcome to Sweettooth 0 → 2;
   Boss's Chauffeur 0 → 26; Mind Burst 0 → 3; Clockwork Steed's sentence
   0 → 0. Clockwork Steed's digit-4 control also stays 0; its *greater than
   four* comparative constituent now reads; the remaining cause was tracked
   by
   `english-v3-cause-catenative-complement`. After refresh, `xnnmkups` gives
   the sentence and its digit-4 control 5 each, from imported `nzmrrqky`;
   all remaining reprobed counts match the pre-refresh AFTER. *Its power is
   less than five*
   and *Your life total is less than seven* each go 0 → 1. The six listed
   controls retain 1 / 1 / 1 / 2 / 2 / 2 Readings. Removing *one plus* retains
   Journey On 1, Welcome to Sweettooth 2, Boss's Chauffeur 26, Mind Burst 5;
   Journey On's digit-1 control retains 1. *one* as Arithmetic Operand goes
   0 → 1; bare *five* as Scalar Measure Phrase stays 0. Every witness/control
   probe enumerated completely (cap 100), with evidence under the same
   target directory. The rebuilt xtask binary re-probed all 30 inputs plus
   both Burn controls after refresh; all enumerations completed.
3. **Spot checks** — On implementation `tsvuoyup`, selected newly covered
   Readings: Boss's Chauffeur, *one plus the number of other creatures*;
   Goblin Gathering, *two plus the number of named cards*; Journey On,
   *one plus the number of opponents*; Klaw, Sonic Subjugator, *one plus the
   number of creature cards*; Liliana, the Last Hope, *two plus the number of
   Zombies*; Magus of the Mind, *one plus the number of spells*; Memorial
   Vault, *one plus the sacrificed artifact's mana value*; Mind Burst,
   *one plus the number of named cards*; Natural Balance, *five minus the
   number of lands*: each has a cardinal left Arithmetic Operand and a
   scalar nominal right operand. Kyren Toy has scalar variable *X* on the
   left and cardinal *one* on the right. No selected gained Reading was
   found to rely only on an unintended analysis. Refreshed `xnnmkups` also
   has a selected intended Reading for each of the five refresh gains:
   four Clockworks use cause's Catenative Complement and the cardinal
   comparative standard; The Moment selects phase's out Complement.
   `accepted-gain-probe00..04.json` retain one selected Reading for each
   acceptance-refresh gain: Tahngarth's progressive Object-gap relative;
   Valiant Endeavor's plural d6 Object; Arcane Endeavor's plural d8 Object;
   Desperate Plea's modal *one or both* header; and Ojer Axonil's negatively
   formed noncombat noun. Each selected Reading has the intended imported
   role, with no reported issues.
4. **Deviations and STOPs** — SC1's permitted sibling fallback adds
   `CardinalScalarOperand` and `CardinalComparativeComplement`: lexical Word
   storage cannot share a form field with phrase Reading storage. The
   comparative transports the existing Adjective Premodifier Use feature,
   denying Premodifier distribution only for the new cardinal standard
   under SC3. No construction or lexical route was retired. The Arithmetic
   Operand glossary entry now includes SC1–SC4's Cardinal Numeral licence;
   no new term was needed. The initial STOP for Burn at the Stake's increase
   was resolved by orchestrator ruling SC6(a), with the digit control
   supplying the measure.
   `kata kanban wip` showed the systemic residuals ticket unowned; one
   overgeneration candidate for both spellings was added there. Normal
   STOP BEFORE INTEGRATE was lifted by orchestrator acceptance on 2026-10-10. No CGEL passage is claimed to
   supply the new licences. Refresh first hit the banked-workspace concurrency
   guard; devotion's new tip was empty, and retry was safe. The subsequent
   registration conflict in `tests/main.rs` was resolved at `tsvuoyup` by
   retaining phase_in_out and this module exactly once. Refresh then exited
   0 and the global conflicts query was empty. The later refresh brought
   `wkooyuko` grammar and lexicon changes, triggering another xtask build,
   bounded census, full suites and clippy under the standing rule. A citation
   inventory snapshot race split this documentation update into divergent
   successors; both disjoint record edits were preserved by squashing the
   snapshot into the committed successor. No code or data changed in repair.
   Acceptance refresh conflicted only at the appended `tests/main.rs`
   registrations; retaining supplementary_temporal_adjunct and this module
   once each at `tsvuoyup` resolved it. The retry exited 0, and the full
   English suite passed. Unrelated conflicts in live subtype workspaces were
   left to their owners.
5. **Tests** — On `tsvuoyup`, `cargo test -p deckmaste_english_v3`
   passed 905 integration tests before SC6 and 906 with its added regression;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed both
   times. On refreshed `xnnmkups`, `cargo build -p xtask` passed;
   `cargo test -p deckmaste_english_v3` passed all 919 integration tests in
   six binaries and doc tests, and `cargo clippy -p deckmaste_english_v3
   --tests -- -D warnings` passed. Imported lexicon data also received full
   `cargo test -p deckmaste_lexical_source` (84 tests and doc tests) and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, both
   passed. After kind-of-counter refresh, `kyymvuqx` reran the same build,
   full tests and clippy commands: English 924 tests, lexical-source 84,
   both doc-test runs and both clippy checks passed. Acceptance refresh again
   ran `cargo build -p xtask`, full `cargo test -p deckmaste_english_v3`
   (961 tests), `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   full `cargo test -p deckmaste_lexical_source` (86 tests), and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`:
   all passed, including doc tests and SC6's Reading-set equality check.
   Evidence: `build-accepted.log`, `tests-accepted.log`,
   `clippy-accepted.log`, `tests-lexical-source-accepted.log`, and
   `clippy-lexical-source-accepted.log`. Own counts: restored 0 /
   re-spelled 0 / ignored 0 / added 10 /
   removed 0. The module is registered once in `tests/main.rs`; assertions
   check operand sides, comparative Complement and existing quantity roles,
   independently constructed roundtrips, and equality of Burn at the
   Stake's normalized new Reading values with the digit control. The
   implementation changes only English code; lexicon changes came from
   refresh. Final citation checks: 0 noncompliant, 0 stale; the earlier bless
   changed no tracked registrations; diff audit selects no new CR sites.
6. **Timing** — Acceptance-refresh bounded AFTER census wall time
   **2:55.45**, `/usr/bin/time -v`, six workers, samples per face 0. No
   full-corpus census or Lean check was run.
