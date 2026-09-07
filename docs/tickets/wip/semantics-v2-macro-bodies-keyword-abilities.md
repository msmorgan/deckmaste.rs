---
needs: [semantics-v2-crate, lean-card-soundness-gate, lean-keyword-definition-bodies]
---
**Give every `plugins_v2/builtin/macros/stubs/keyword_abilities` declaration its semantic
`body`** (195 declarations, all bodyless today). Each body is a term-for-term
expansion over the v2 basis with no default slots, one macro per printed
phrase shape, invoking other macros where the CR definition does. The Lean
gate checks each body as it lands; Lean `Macros.lean` is the reference for
shapes already modelled there. Mechanical per declaration once the family's
shape is settled: terra tier. Standard constraints apply.

Ruling (user, 2026-09-06): each body is the keyword's definition, ported
from the v1 definitional macro under `plugins/builtin/macros/keyword/` where
one exists, carried in `Keyword(keyword: "<Label>", params: [...], body:
Some(<definition>))` under the widened law of `lean-keyword-definition-bodies`.
Keywords with no v1 body get theirs from the CR with its citation. v1 doc
comments and citations port with the body.
