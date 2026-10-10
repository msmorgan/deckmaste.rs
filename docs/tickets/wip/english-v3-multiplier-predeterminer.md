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
