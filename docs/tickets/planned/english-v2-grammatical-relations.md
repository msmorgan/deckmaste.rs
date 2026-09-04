---
needs: [english-v2-np-postmodifiers]
---
**Separate grammatical relations from constituent categories in English v2.**
Use the [`Subject`, grammatical `Object`, `Complement`, `Nominal`, and `Noun
Phrase` definitions](../../contexts/oracle-english/CONTEXT.md). Subject and Object are relations
within larger constructions, not phrase categories. Their spelling may coexist
with the unrelated CR Object because the layers mean genuinely different
things.

`english-v2-np-postmodifiers` has landed; audit category names, construction slots,
AST variants, and parser diagnostics that use `Subject` or `Object` for a
constituent shape. Name the actual constituent category and record its relation
separately. Replace `SubjectPronoun` and `ObjectPronoun` with nominative and
accusative personal-pronoun vocabulary, or model case directly if that is the
smaller coherent change. Clean up the current broad `Object` phrase category
rather than adding another compatibility layer.

2026-09-04: stale precondition refreshed — was: "After the NP/postmodifier WIP
lands, audit …"; `english-v2-np-postmodifiers` is in `done/`.

Acceptance includes the same Noun Phrase serving as Subject, Object, and
prepositional Complement without changing its category, plus pronoun witnesses
whose case—not a presumed clause relation—selects the form.
