---
needs: [semantics-v2-adr-promotion]
---
**Create `crates/deckmaste_semantics_v2`: the Lean-mirrored type universe,
the `macro_ron` kinds, and the reader for `plugins_v2`.** Design per the
promoted `semantics-v2.md`; standard constraints apply. Design-bearing:
sol or Opus tier.

- Types mirror `lean/Semantics/{Words,Events,Phrase,Triggers,Abilities,Card}.lean`
  constructor-for-constructor and field-for-field, post
  `lean-constructor-collapse`. Settle §3's joined-kind payload here. A drift
  test compares Rust constructor and field names against the Lean
  declarations and fails on any difference; the emitter in
  `lean-card-soundness-gate` later builds on the same mapping.
- `macro_ron` dialect, not plain serde: `SupportsMacros` on every enum a
  macro may occupy, one kind per such enum, `MacroDef<()>`-style opaque
  metadata so the crate never depends on `deckmaste_construction_core` or
  `deckmaste_spelling`. The crate depends on `macro_ron` and nothing
  deletion-bound.
- Reader for `plugins_v2/<plugin>/{macros,cards,tokens,rules}`: walks the
  tree, expands macros (macros invoke other macros), yields cards, tokens,
  and the three rules tables. Rules tables are semantics-language RON, same
  as today's `plugins/builtin/rules`.
- Move `plugins/builtin_v2` to `plugins_v2/builtin`, leave a symlink at the
  old path, re-point `deckmaste_construction_core::macro_def::read_builtin_v2`
  and `crates/xtask/src/facts.rs`. An xtask drift test loads every
  declaration both ways (english_v2 metadata, semantics_v2 body).
- Read `Check/` for the reads lowering will need (binding resolution, kind
  projection) and expose them as plain functions; no refusals, no
  validation.
