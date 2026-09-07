---
needs: []
---
**`cargo xtask facts generate` must not read v2 declarations through v1
types.** Found by `lean-conferral-tag-removal` (2026-09-07): after the
conferrer types went, `crates/xtask/src/facts/` still imports
`deckmaste_semantics` for `DesignationDecl`, `DesignationDef`,
`DesignationScope`, `DesignationShape`, `CounterScope`, `Bearing`, and
`ron::options`, because those are how it deserialises the `body` of the
`plugins_v2/builtin` designation and counter-kind declarations, and the
counter-kind `dedicated` column needs v1's `Bearing` tree. v1 is
deletion-bound (`semantics-v1-cutover`), so the v2 declaration bodies need a
v2 home: decide between relocating the body types into
`deckmaste_semantics_v2` (they are semantic data the v2 reader already
loads as macro bodies) and `deckmaste_construction_core` (which types the
declaration shell), then re-point the generator and delete the import.
`facts check` must show both generated tables unchanged. Standard
constraints apply.
