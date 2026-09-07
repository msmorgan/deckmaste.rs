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
