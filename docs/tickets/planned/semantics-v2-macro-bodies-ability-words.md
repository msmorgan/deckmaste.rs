---
needs: [semantics-v2-crate, lean-card-soundness-gate]
---
**Give every `plugins_v2/builtin/macros/stubs/ability_words` declaration its semantic
`body`** (61 declarations, all bodyless today). Each body is a term-for-term
expansion over the v2 basis with no default slots, one macro per printed
phrase shape, invoking other macros where the CR definition does. The Lean
gate checks each body as it lands; Lean `Macros.lean` is the reference for
shapes already modelled there. Mechanical per declaration once the family's
shape is settled: terra tier. Standard constraints apply.

Every ability word has the same body, `ItalicHead(word: AbilityWord(label:
<name>), ability: Param(0))` with `params: [Ability]`; supply it once from
the `AbilityWord` meta-declaration via `Param(name)` rather than in 61
files, unless the meta cannot express it, in which case per-file.
