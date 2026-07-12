---
needs: []
---
**Carve `crates/deckmaste_engine/src/step.rs` (~3.5k lines) into a `step/`
module directory, using the `resolve/` split as the template.** Second-largest
file after `trigger.rs`; same treatment, same gates. `layer.rs` (~3.0k) and
`cast.rs` (~2.7k) are the next candidates after this if they keep growing —
but they are NOT in scope here.

Follow the split recipe and gotcha list in [[split-trigger-rs]] (mv to
`mod.rs` first; carve code along method-cluster seams into sibling
`impl GameState` submodules; `pub(super)` for cross-submodule helpers and
`pub(crate) use` re-exports to keep external `crate::step::…` paths stable;
tests split per owning submodule with a shared `#[cfg(test)] mod fixtures`).

Gates: engine suite green with exact `#[test]`-count parity, workspace clippy
clean, nightly fmt, `cargo xtask cite check`, comment-multiset audit.
