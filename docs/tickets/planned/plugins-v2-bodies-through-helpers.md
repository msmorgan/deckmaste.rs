---
needs: [plugins-v2-positional-macro-application, facts-reader-reads-the-dialect]
---
**Family bodies are written through the helper layer.** After
`plugins-v2-dialect`, the keyword-ability, keyword-action and turn-part
bodies are correct but written as raw constructor trees (`flying`:
`Static(spec: DeonticRule(subject: AsType(...), ...))` with
`HasType(type: Creature)` and `Not(predicate: Or(...))` spelled out) while
the helper layer now declares `hasType`, `not`, `or`, `static`, and the
rest. Rewrite every family body to invoke a helper wherever one expands
to exactly the sub-term written — the same substitution the fourth
dialect landing applied to cards — leaving raw constructors only where no
helper exists (list them; each is an embed or helper candidate). Oracles:
`lean/Generated` byte-identical, `lean-check` 118/118 and 2/2,
`facts check` byte-identical (the facts reader must read the result:
`facts-reader-reads-the-dialect` first if it does not). Standard
constraints apply.
