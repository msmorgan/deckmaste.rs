---
needs: [semantics-v2-parity]
---
**Retire v1.** Delete the set the promoted `semantics-v2.md` enumerates
(`deckmaste_semantics`, `deckmaste_lowering`, `deckmaste_legacy_render`,
`deckmaste_plugin` with `deck.rs` and `provenance.rs` rehomed, `idris/`,
`plugins/builtin`, `plugins/canon`, the `deckmaste_spelling` splice
machinery), rename `deckmaste_semantics_v2` to `deckmaste_semantics` and
`plugins_v2/` to `plugins/`, drop the symlink, and re-point
`deckmaste_migrations`. Coordinate with `idris-retirement` and
`english-v2-rewrite.md`'s own cutover. Standard constraints apply.
