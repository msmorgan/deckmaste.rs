---
needs: [plugins-v2-dialect]
---
**Generate `lean/Semantics/Macros.lean` from the `plugins_v2` declarations**
so the Lean macro layer stops being hand-written (`semantics-v2.md` §6
"eventually"). Not needed by the gate, which checks expanded terms. Parked
until the macro-body families land.
