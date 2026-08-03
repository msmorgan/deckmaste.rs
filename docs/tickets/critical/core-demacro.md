---
needs: [plugin-repoint, idris-mirror-authoring, runtime-prose-link]
---
**Strip the macro machinery from `deckmaste_core`: pure engine AST, plain
serde, no author-surface knowledge.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§1, §11-12). The priced big-boring item lives here: the test-fixture
sweep.

**Sequenced behind `runtime-prose-link` (2026-08-02):** deleting core's
`Expanded` variants compile-breaks the legacy renderer + fidelity in
`deckmaste_plugin` (~30+ match sites outside both this ticket's sweep
and the repoint's ~97 `deckmaste_engine` sites); the input-type repoint
to authored terms lands there first.

## Scope

- Remove `SupportsMacros` derives, `#[macro_ron(…)]` attributes, the kind
  registry remnants, and the macro-aware reader plumbing from core; keep
  plain serde for snapshots/fixtures.
- **Fixture sweep** (spec §12): every `#[cfg(test)]` site that spells core
  values through the macro-aware reader (engine resolve/trigger test
  modules, core mana/filter tests, the plugin-crate fixture helpers,
  integration suites) either repoints to authoring + `lower` or re-spells
  in plain serde RON. Script the mechanical bulk; hand-review the
  remainder.
- After this lands, a bare authored spelling cannot reach core except
  through `deckmaste_lowering` — enforced by the dependency graph.

## Gates

Standard constraints apply. Full workspace suites green; idris-check and
fidelity unchanged; `grep` proves no `macro_ron` dependency or attribute
survives in `deckmaste_core`.
