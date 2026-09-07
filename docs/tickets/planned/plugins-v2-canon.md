---
needs: [semantics-v2-macro-bodies-keyword-actions, semantics-v2-macro-bodies-keyword-abilities, semantics-v2-macro-bodies-ability-words, semantics-v2-macro-bodies-turn-parts, lean-rules-tables]
---
**Re-author `plugins/canon` and `plugins/builtin`'s cards, the seven
predefined tokens, and the rules tables in `plugins_v2/canon` and
`plugins_v2/builtin`.** Hand-authored RON; the English parser is not ready,
so no migrated cards. Every card is gate-checked by the Lean emitter as it
lands. Each card that had a hand-written Lean spelling retires it in favour of
the emitted term (`lean-hand-bench-retirement` tracks the bench's shrink).
Standard constraints apply.

Routed here by `lean-card-soundness-gate` (2026-09-06), whose own scope
condition was "once the Lean gate covers every card the Idris gate did":

- **Retire the Idris card gate.** Delete `crates/deckmaste_plugin/src/idris_emit.rs`,
  `crates/xtask/src/idris_check.rs`, the `IdrisCheck` command, and every
  `plugins/*/idris-check-baseline.ron`, and drop the `check canon Idris
  baseline` step from the CI `idris` job. The condition is met when
  `plugins_v2/canon` covers the cards `plugins/canon` holds; the landing
  record lists every card whose verdict differs between the two gates and why.
- **Re-home the differential.** `idris-check --differential` paired the Idris
  mirror's certification against `deckmaste_lowering`'s computed resolution.
  Its successor pairs `lean-check`'s verdict against `deckmaste_lowering_v2`,
  so it cannot be built before `lowering-v2` exists; if `lowering-v2` has not
  landed by the time this ticket does, mint the differential as its own ticket
  rather than deleting the idea with the Idris code.
