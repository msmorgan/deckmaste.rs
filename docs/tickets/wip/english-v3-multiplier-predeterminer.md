---
needs: []
---
# Read a multiplier before an NP: It deals double that damage instead. / X is twice its power.

## Why

No multiplier reads as a dependent of an NP. *twice* lexes only as
`vocab:FrequencyAdverb/Twice` (*Copy that spell twice.*), *double* only as the
keyword-action verb `lexeme:keyword_action/double` (*Double target player's
life total.*) and inside the keyword *double strike*. The 2026-10-10 triage
(`docs/memory/english-v3-blocker-triage-2026-10-10.md`, blocker report at
trunk `vwlkmvns`) ranks this second: 99 faces whose single failing line
matches `\b(twice|double) (that|the|its|their|your|this|X|-X)\b`.
`cargo xtask english-v3 grep '\b(twice|double|triple) (that|the number|its|this|X)\b'`
finds **174** faces. Shapes among the 99 lines: *double that damage* 27,
*twice the number of …* 24, *twice that much* 10, *twice that many* 10,
*twice that many of those tokens* 8, *twice X* 7, *twice its power* 5.

The 99 is an upper bound, not a forecast: 8 lines (*that many of those
tokens*) fail without the multiplier too, and 3 are the imperative verb
*double* after a preposed clause.

## Goal

A multiplier (*twice*, *double*, *triple*) reads as a predeterminer Modifier
of a definite NP (*double that damage*, *twice its power*, *twice the number
of artifacts you control*, *twice that much life*, *twice that many cards*)
wherever that NP reads today, with byte-exact realisation.

## Analysis

CGEL Ch. 5 §12(c), p. 434, [4]: multipliers are predeterminer modifiers;
"*Twice* and the archaic *thrice* in [ii] are adverbs, occurring elsewhere as
frequency adjuncts"; of *double*, *triple*, "we take them to be nouns";
"Multipliers occur only with definite NPs, and there is no corresponding
partitive construction". That backs the external-Modifier analysis, the
categories and the definiteness restriction only.

Existing routes: no Construction has a predeterminer slot. The NP routes the
multiplier must sit outside are `DeterminedNounPhrase`, `GenitiveNounPhrase` /
`PossessiveNounPhrase` and the *that much* / *that many* route
(`DegreeDeterminativePhrase` under `QuantifiedNounPhrase`), all in
`crates/deckmaste_english_v3/src/declarations.rs`. `CardinalModifier` (the
*at least* landing) is the nearest precedent for an optional leading
dependent added to an existing Construction rather than a parallel one.

Orchestrator rulings 2026-10-10:

- MP1. One external-Modifier position on the NP, not one Construction per
  host. Prefer an optional leading dependent shared by the existing definite
  NP routes (schema or policy); a single new Construction wrapping a
  NounPhrase is acceptable only if the shared dependent cannot be expressed,
  and must be justified in the landing record (the count is over the ADR
  ceiling).
- MP2. The licence is a declared feature on the multiplier's lexeme (a
  predeterminer use), read by the grammar; no word test. *twice* keeps its
  FrequencyAdverb lexeme and gains the declared use (CGEL: the same adverb).
  *double* / *triple* get a lexeme of their own in `lexicon/core.ron`; the
  keyword-action verb is untouched.
- MP3. The host must be definite (CGEL p. 434), read from a declared or
  exported feature of the host NP, never from the determiner's spelling.
- MP4. *twice X stun counters*, *twice X life*, *twice -X/-X* have no
  definite host. Project ruling: they are in scope only if they fall out of
  MP1–MP3 through a host the grammar already calls definite or scalar;
  otherwise list them in the landing record with counts (7 + 1 lines) and
  leave them unread. Do not add a second route for them.
- MP5. One analysis per string. *Copy that spell twice.* and *Activate only
  twice each turn.* keep exactly their Readings (no NP *twice each turn*);
  *Double target player's life total.* stays verb-only (no multiplier +
  NP Reading of an imperative); *double strike* is untouched.
- MP6. Newly covered faces are gains. Any Reading-count increase on a
  previously covered face is reported with its shape before landing; under
  MP5 most are defects.

## Witnesses

Each is the face's only failing line at `vwlkmvns`. Probed 2026-10-10 with the
`zrrnvqto` binary (`--category sentence --readings 3 --as-card`): witness 0,
control as shown. Trunk has since taken `nvotwtqp`; re-probe on the claim
parent first.

- Boon Reflection: "If you would gain life, you gain twice that much life
  instead." (control "… you gain that much life instead." 1)
- Fire Servant: "If a red instant or sorcery spell you control would deal
  damage, it deals double that damage instead." (control "… it deals that
  damage instead." 3)
- Polliwallop: "Target creature you control deals damage equal to twice its
  power to target creature you don't control." (control "… equal to its power
  …" 3)
- Price of Progress: "Price of Progress deals damage to each player equal to
  twice the number of nonbasic lands that player controls." (control "… equal
  to the number of …" 3)
- Jovial Evil: "Jovial Evil deals X damage to target opponent, where X is
  twice the number of white creatures that player controls." (control "… X is
  the number of …" 3)

Controls that must not change: "Copy that spell twice." 1; "Activate only
twice each turn." 1; "Double target player's life total." 1; "Target creature
gains double strike until end of turn." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(twice|double|triple)\b

(444 faces) plus 300 random covered controls (`cargo xtask english-v3 diff
--manifest … --random 300 --seed 20261008`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; MP6 for increases. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_english_v3` and `deckmaste_lexical_source`,
once, at the end: a rebuild after a grammar edit takes about 2–3 minutes and
the test binary about 7, so batch edits. Tests assert the role (the multiplier
as external Modifier of its host NP, its lexeme, and the MP5 controls' full
Reading sets), never an incidental count or a Debug-printed tree. Attestation
by `cargo xtask english-v3 grep` only.

## Out of scope

- *half … rounded up/down* (84 faces match `\bhalf\b`): own ticket.
- *twice as much of that mana* (comparison of equality) and *N times* NPs
  (*three times X cards*, 5 faces): not this mechanism.
- *that many of those tokens* (8 lines): fails without the multiplier.

## Landing record

1. **Census delta:** claim-parent `uqlozwkr` (measured in empty `qoswqlns`)
   → implementation `qoswqlns`: **521 → 598 covered faces**, **17,455 →
   19,306 Readings**; no/one/multiple: 221/211/310 → 144/215/383.
   **77 gains; lost none; decreases 0; increases 0; digest-only 0; issues 0.**
   Selector: the Method's `\b(twice|double|triple)\b` (444 supported faces)
   plus 300 covered controls, seed 20261008: **742 distinct faces** (two
   overlaps), in `target/english-v3/english-v3-multiplier-predeterminer/manifest.json`.
   Official `english-v3 diff --lines` compares `before.json` and `after.json`
   in that directory; `diff.json` retains the complete comparison.
   After refresh, empty `kwpynuzm` over rebased `qoswqlns` gives
   **602 covered faces, 19,434
   Readings** (140/215/387 no/one/multiple), **81 gains** against the original
   claim parent, with **0 lost/decreases/increases/digest-only/issues**.
   `diff-refresh.json` is the official parent comparison;
   `trunk-refresh-diff.json` compares the two implementation censuses:
   **4 gains, 128 Readings**, no other changes. These inherited gains are
   Invasion of Pyrulea, Nick Fury, Agent of S.H.I.E.L.D., Overgrown Pest,
   and The Golden-Gear Colossus, from `nqnztuzo`'s *double-faced* adjective
   declaration (retained by vocabulary correction `pwoslssz`).
   The refresh also brought plugin/core change
   `xwuuqwvw`; both conflicted additive lists (lexemes and test modules)
   were stacked, retaining every addition.

2. **Witnesses:** on `uqlozwkr`, all five re-probed witnesses have **0**
   Readings; none was dropped. On `qoswqlns`: Boon Reflection **1**, Fire
   Servant **10**, Polliwallop **6**, Price of Progress **32**, Jovial Evil
   **3**, matching their unmultiplied controls' complete counts. The four MP5
   controls remain **1 each**; tests compare their complete Reading sets.
   Probe evidence is `before-probe-*.json` / `after-probe-*.json` in the
   evidence directory above (complete enumeration, rather than the ticket's
   historical three-Reading cap).

3. **Spot checks:** ten newly covered faces on `qoswqlns`, with byte-exact
   selected Readings: Alhammarret's Archive — external *twice* over *that
   much life*; Angel of Suffering — *twice* over *that many cards*;
   Angrath's Marauders — noun *double* over *that damage*; Animist's Might —
   *twice* over *its power*; Branching Evolution — *twice* over the plural
   *that many +1/+1 counters* Subject; Bruvac the Grandiloquent — *twice*
   over *that many cards*; City on Fire — noun *triple* over *that damage*;
   Corpseweft — *twice* over the definite number NP in the variable
   definition; Fiery Emancipation — noun *triple* over *that damage*;
   Reflecting Mirror — *twice* over *the mana value of that spell*.
   Selected trees are `spot-0.json` through `spot-9.json`.

4. **Deviations and STOPs:** MP1's permitted single NP wrapper is used:
   optional fields accept Categories, cannot expose a single feature value
   for conditional host licensing, and policies admit only feature equations.
   A shared optional lexical dependent across the existing host routes cannot
   express the required licence with the current schema; the wrapper shares
   one external position across them without compiler changes. One
   Construction added, no frames added or deleted, no routes superseded;
   frequency *twice*, imperative *double*, and *double strike* remain live.
   Other NP routes retain unclassified definiteness. MP4 leaves **17
   scalar-X faces plus Nuclear Fallout's negative-X face unread** (18 faces,
   18 matches on `qoswqlns`, attested with `english-v3 grep --census …
   --unread`). Glossary gaps filled: Predeterminer, Multiplier, Predeterminer
   Use, Definiteness; CGEL supports the linguistic terms, MP1–MP3 the project
   licensing. No STOPs.

5. **Tests:** on `qoswqlns`, `cargo test -p deckmaste_english_v3` passed
   **758 tests**, and `cargo test -p deckmaste_lexical_source` passed **76
   tests** (35 unit + 41 integration). Both
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   The first English run passed all 748 existing tests and eight new tests;
   two new helper assertions incorrectly restricted the host's PP scope.
   Corrected them to compare the complete original sentence's host Reading
   set while also retaining every intended-host value; the full suite then
   passed. Restored **0**, re-spelled **0**, ignored **0**, added **10**,
   removed **0**; no existing test changed. Citation checks: **0 stale, 0
   noncompliant**, no newly cited CR sites.
   After the grammar/data-bearing refresh, on `kwpynuzm` over `qoswqlns`,
   the full English suite passed
   **759 tests**, and the same full lexical-source suite passed **80 tests**
   (39 unit + 41 integration), with **0 failed/ignored**; the extra tests are
   inherited from vocabulary batch 2. Both clippy commands above passed
   again with warnings denied. The refresh-required
   `cargo test -p xtask english_v3` passed **56 tests**, **0 failed/ignored**.
   Post-refresh logs are `test-*-refresh.log` / `clippy-*-refresh.log` in the
   evidence directory. Citation checks again had **0 stale/noncompliant**.

6. **Timing:** `qoswqlns` bounded census **46.17 s**, **6 workers**, under
   `/usr/bin/time -v`, prebuilt xtask, `--samples-per-face 0`. No full corpus
   census was run. The required post-refresh bounded re-run on `kwpynuzm`
   took **17.43 s**
   with the same manifest and options (`after-refresh-census.time`).
