---
needs: [semantics-v2-crate, lean-card-soundness-gate]
---
**Give every `plugins_v2/builtin/macros/stubs/keyword_actions` declaration its semantic
`body`** (65 declarations, all bodyless today). Each body is a term-for-term
expansion over the v2 basis with no default slots, one macro per printed
phrase shape, invoking other macros where the CR definition does. The Lean
gate checks each body as it lands; Lean `Macros.lean` is the reference for
shapes already modelled there. Mechanical per declaration once the family's
shape is settled: terra tier. Standard constraints apply.

Lean is the spec for each body: translate the matching `semantic_macro` in
`lean/Semantics/Macros.lean` positionally, agent explicit (no default slot;
callers write `You`). Port the v1 declaration's doc comment, CR citations,
and rationale from `plugins/builtin/macros/action/<Name>.ron` where one
exists; any v1 action meaning Lean cannot express is a STOP, not a body.
