---
needs: []
---
# Saga rules rows: lore only with chapters, the pending-trigger guard, and "an effect" excludes the lore action

Standard constraints apply. Provenance: `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md`
#8, #41, #66. Verified against head 2026-10-21. Latent in Lean; the `saga.ron` rows are read by
`xtask` facts and `deckmaste_macro_def`. The engine half is [engine-sagas](engine-sagas.md).

## 1. The sacrifice SBA waits for chapter abilities that have triggered

`Proofs/Rules.lean:240-250` (`sagaDefinition`):

```lean
.stateBased (.and [ .compareAmt (finalChapterNumber thisSaga) .atLeast (.lit 1),
  .compareAmt (countersOn (.named "loreCounter") thisSaga) .atLeast (finalChapterNumber thisSaga) ])
  (sacrifice thisSaga)
```

So the Saga is sacrificed at the first check after its final chapter triggers, and *Fable of the
Mirror-Breaker*'s chapter III finds no Saga to exile and return. [CR#714.4,704.5s] add "and it isn't
the source of a chapter ability that has triggered but not yet left the stack". "Triggered" covers
abilities waiting to be put on the stack, not only those on it.

[lean-v3-proofs-and-docs-alignment](lean-v3-proofs-and-docs-alignment.md) §2 planned the guard as
`chapterAbilityOf` + `in_ stack`, which misses the waiting case; that bullet now points here. No
Lean predicate for "triggered but not yet put on the stack" exists (`chapterAbilityOf`,
`Templates.lean:934`, is only `.and [.abilityOf saga, .compare [.chapter] .atLeast 1]`).

Change: add a predicate for a source's triggered abilities that are waiting or on the stack, and
conjoin "not the source of such a chapter ability" to the row. Reuse it for the Siege row
[CR#704.5v,310.7] and the dungeon row [CR#704.5t] when those are written. Mirror it in
`plugins_v2/builtin/macros/subtypes/enchantment/saga.ron:63-69`. The engine fixture
`crates/deckmaste_engine/src/sba.rs:1262-1350` (`state_based_conferral_sacrifices_a_finished_saga`)
follows in engine-sagas. Pins: the Saga survives while chapter III is waiting and while it is on
the stack, and is sacrificed after it leaves the stack.

## 2. The lore turn-based action reaches only Sagas with chapter abilities

`Proofs/Rules.lean:244-245` is an ungated `.turnBased .firstMain (.putCounters (.lit 1) (.printed
(.named "loreCounter")) thisSaga .one true)`; `saga.ron:62` is the same, and the v1 comment in
`plugins/builtin/macros/types/enchantment/Saga.ron:9-12` paraphrases the rule without the clause.
[CR#714.3c,703.4f]: "puts a lore counter on each Saga they control with one or more chapter
abilities". Gate the row on `finalChapterNumber thisSaga ≥ 1`, as the SBA row already is, and
correct both comments (v1's comment only; no new v1 work).

## 3. Doubling Season doesn't double the lore turn-based action

`doublingSeason` (`Cards/Counters.lean:348-352`) is `replace (would any (putCounters anyAmount
anyKind (a (and [isPermanent, youControl])) .many)) …`, the passive `any` that also matches the
`.turnBased .firstMain (.putCounters …)` lore increment. Entering with 2 lore is right: the entry
counters are a replacement effect [CR#714.3a], which [CR#614.16] lets *Doubling Season* modify. The
increment is not: it is a turn-based action [CR#714.3c], a game action [CR#703.1], and an effect is
"something that happens in the game as a result of a spell or ability" [CR#609.1]. [CR#614.16]
names exactly what "an effect would put" reaches: the effect of a resolving spell or ability, and
another replacement or prevention effect.

`docs/tickets/done/core-saga-chapters.md:39-42` says a Saga under Doubling Season "enters with 2
lore and increments by 2"; this ticket supersedes the "increments by 2" half.

Change: give "If an effect would put/create …" a subject limited to the effects [CR#614.16] names,
distinct from the passive `any`, and use it for *Doubling Season* and the other "if an effect
would" sites. Pins: a Saga under Doubling Season enters with 2 lore and gains 1 per precombat main
phase.

## Verification

`./lean/scripts/build`; `cargo xtask lean check`; `cargo test -p xtask -p deckmaste_semantics_v2
-p deckmaste_macro_def` and clippy for each; the CR citation gates.
