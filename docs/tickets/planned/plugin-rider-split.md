---
needs: [plugin-crate-split]
---
**Split the non-loader riders out of `deckmaste_plugin`.** Design:
`docs/decisions/semantics-spelling-lowering.md` (§1, §11). Deferred from
`plugin-crate-split`, which landed the rename only.

The loader core is `plugin.rs` + `macros.rs` (mutually referential). Two
riders depend on `plugin` and nothing else, so they extract cleanly:
`idris_emit.rs` (~4.5k lines) and `validate.rs` (~0.8k lines).

## Scope

- Move `idris_emit` and `validate` into their own crate; `deckmaste_plugin`
  keeps the loader, `deck`, `energy`, and the legacy renderer.
- The legacy renderer (`render/`, `template/`) dies in place and must NOT get
  a new home; `fidelity` rides the renderer and stays in the plugin crate
  (design §11: "the comparison harness rides fidelity in the plugin crate").
- No semantic change.

## Gates

Standard constraints apply. Zero behavior change: full workspace suites,
`cargo xtask idris-check plugins/canon` no regressions, `cargo xtask fidelity`
PASS.
