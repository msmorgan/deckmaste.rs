---
needs: [semantics-v3-dsl-and-plugins, semantics-v2-macro-bodies-keyword-actions, semantics-v2-macro-bodies-keyword-abilities, semantics-v2-macro-bodies-ability-words, semantics-v2-macro-bodies-turn-parts, lean-rules-tables]
---
**Re-author `plugins/canon` and `plugins/builtin`'s cards, the seven
predefined tokens, and the rules tables in `plugins_v3/canon` and
`plugins_v3/builtin` (updated 2026-10-14 per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).**
Hand-authored in the v3 DSL; the English parser is not ready,
so no migrated cards. Every card is gate-checked by the Lean emitter as it
lands. Each card that had a hand-written Lean spelling retires it in favour of
the emitted term (`lean-hand-bench-retirement` tracks the bench's shrink).
Standard constraints apply.
