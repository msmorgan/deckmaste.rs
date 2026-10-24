---
needs: [semantics-v3-parity]
---
**Retire v1 scaffolding and rename `_v3` to canonical names (updated 2026-10-14
per [`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).**
Delete the `v1` set (`deckmaste_semantics`, `deckmaste_lowering`,
`deckmaste_legacy_render`, `deckmaste_plugin` with `deck.rs` and
`provenance.rs` rehomed, `idris/`, `plugins/builtin`, `plugins/canon`, and the
`deckmaste_spelling` splice machinery; `deckmaste_semantics_v2`,
`plugins_v2/canon/`, and `plugins_v2/builtin/` are retired in
`semantics-v3-dsl-and-plugins`, and `deckmaste_lexical_source` was retired in
`english-v4-cutover-v3`), rename `deckmaste_semantics_v3` to
`deckmaste_semantics`, `deckmaste_lowering_v3` to `deckmaste_lowering`, and
`plugins_v3/` to `plugins/`, drop the symlink, and re-point
`deckmaste_migrations`. Coordinate with `idris-retirement` and
`english-v4-final-suffix-cutover` (which deletes legacy v1 `deckmaste_english`,
drops `_v4` suffixes, and moves `catalogs/*.ron` into `plugins/` metadata).
Standard constraints apply.

The Idris card gate retires here with Idris itself (user ruling,
2026-09-06; routed from `lean-card-soundness-gate`): delete
`crates/deckmaste_plugin/src/idris_emit.rs`, `crates/xtask/src/idris_check.rs`,
the `IdrisCheck` command, every `plugins/*/idris-check-baseline.ron`, and
the CI `idris` job. `idris-check --differential` has no successor to build
before this point; `semantics-v3-parity` is the v3 cross-check.
