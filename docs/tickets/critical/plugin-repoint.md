---
needs: [lowering-crate]
---
**Repoint loading and migration at the authoring grammar: parse
`deckmaste_authoring` types, lower to core for everything downstream.**
Design:
`docs/decisions/authoring-spelling-lowering.md`
(§11). Zero behavior change — the engine receives byte-identical core
values.

## Scope

- `Plugin::card`/`token`, the per-directory loaders (including the
  `rules/{sba,grant,damage}` tables — they are authored containers too),
  and the migrations graduation/parser paths construct authored terms and
  call `lower`.
- The `Expanded`/`remembers_expansion` invocation-provenance machinery
  relocates from core values to authored values (a real sub-project — spec
  §12 — not a rename; storage round-trips must keep preserving
  invocations).
- Frames guard-constant reads retarget to authoring kinds.

## Gates

Standard constraints apply. Full workspace suites, `cargo xtask
idris-check plugins/canon` no regressions, fidelity PASS, canon/wizards
save-load round-trips byte-identical.
