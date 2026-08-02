---
needs: [cards-elaborator-tables]
---
**Wire the elaborator into `Plugin::load` with a staged deny/warn rollout, and
land the elaboration lockfile.** After [[cards-elaborator-tables]], the walker
exists behind `cargo xtask validate`; this ticket makes it the load gate so the
engine only ever consumes elaborated cards — a malformed shape becomes a load
error, never a resolve-time panic. The audit-flagged resolve-time `.expect()`
panic sites in `crates/deckmaste_engine/src/resolve.rs` become unreachable for
loaded cards: the engine's input contract is `ElabCard`, not raw grammar.

## Load wiring

- `Plugin::load` / `load_with_prelude` (crates/deckmaste_plugin/src/plugin.rs)
  run `deckmaste_plugin::elaborate` on every card after macro expansion.
- Elaboration failures surface as load errors carrying the error code, card
  name, and file path (`E-POS-TARGETED at plugins/…/foo.ron (Foo)`).
- The engine-facing card type is the elaborated IR; downstream consumers
  (engine, TUI, strategy) take `ElabCard`.

## Staged rollout

| Plugin | stage |
|---|---|
| `plugins/builtin`, `plugins/canon`, `plugins/testing`, `plugins/demo` | **deny** immediately (hand-authored; must already be clean) |
| `plugins/wizards` (generated) | **warn** for one milestone — failures print but load proceeds with the offending cards skipped — then flip to **deny** (hard sunset; the flip is part of this ticket's follow-through, triggered by the first wizards regeneration after the warn milestone integrates) |

The stage is a property of the load call, not a global: tests that load
`testing` get deny from day one.

## Lockfile

- `cargo xtask elaborate --lock` writes `cards.elab.lock`: a per-card hash of
  the elaborated IR for the hand-authored plugins. CI recomputes and compares;
  any table, macro, or card change that re-points a reference fails until
  re-blessed — drift is loud, never silent.
- `cargo xtask elaborate --dump <card>` prints the computed resolutions
  (which antecedent each reference bound to) for review.

## Done

- Loading a plugin with a malformed card fails (deny) or warns (wizards) with
  the `E-*` code; a test pins each stage's behavior.
- All hand-authored plugins load clean under deny; wizards loads under warn
  with a counted report.
- `cards.elab.lock` committed and CI-checked; `--dump` output covers at least
  the reject-fixture cards' accept twins.

## Verification

- `cargo test -p deckmaste_plugin` and `cargo test -p deckmaste_engine` green.
- `cargo xtask validate` on each hand-authored plugin — clean.
- `cargo xtask elaborate --lock` idempotent (second run: no diff).
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
