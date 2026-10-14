---
needs: [semantics-v3-parity]
---
**Retire v1 and intermediate v2 scaffolding (updated 2026-10-14 per
[`docs/decisions/semantics-v3.md`](../../decisions/semantics-v3.md)).** Delete
the set `semantics-v2.md` and `semantics-v3.md` enumerate
(`deckmaste_semantics`, `deckmaste_semantics_v2`, `deckmaste_lowering`,
`deckmaste_legacy_render`, `deckmaste_plugin` with `deck.rs` and
`provenance.rs` rehomed, `idris/`, `plugins/builtin`, `plugins/canon`,
`plugins_v2/`, the `deckmaste_spelling` splice machinery), rename
`deckmaste_semantics_v3` to `deckmaste_semantics`, `deckmaste_lowering_v3` to
`deckmaste_lowering`, and `plugins_v3/` to `plugins/`, drop the symlink, and
re-point `deckmaste_migrations`. Coordinate with `idris-retirement` and the
retirement of the legacy English stack. Standard constraints apply.

The Idris card gate retires here with Idris itself (user ruling,
2026-09-06; routed from `lean-card-soundness-gate`): delete
`crates/deckmaste_plugin/src/idris_emit.rs`, `crates/xtask/src/idris_check.rs`,
the `IdrisCheck` command, every `plugins/*/idris-check-baseline.ron`, and
the CI `idris` job. `idris-check --differential` has no successor to build
before this point; `semantics-v3-parity` is the v3 cross-check.
