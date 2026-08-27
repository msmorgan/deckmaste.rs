---
needs: []
---
`cargo test -p xtask --lib` is red: five `catalogs::tests::*` cases fail, all
with the same panic at `crates/xtask/src/catalogs.rs:171`, unwrapping an `Err`
on "canonicalizing project workspace root" (`No such file or directory`).

The origin is `crates/deckmaste_catalogs/src/io.rs:182-188`, which canonicalizes
`env!("CARGO_MANIFEST_DIR")/../..` — a path baked in at compile time. That
resolves to a directory that need not exist when the binary runs, which is why
the failure looks environmental rather than behavioural. The cause has not been
isolated further: it may be a stale build cache, a workspace-relocation artifact
(the repository runs several jj workspaces under `.workspaces/`), or a genuine
defect in the root resolution.

Nothing here is an english_v2 or construction-compiler concern. It is flagged
because "the v2 tests are green" is currently only true if the command is scoped
away from `catalogs::`, so an unowned red gate is masking whatever else lands in
that module.

Isolate the cause first, then fix it at the resolution site rather than in the
tests: workspace-root discovery should not depend on a compile-time-baked path.
Acceptance: `cargo test -p xtask --lib` passes from a freshly provisioned
workspace as well as from the default checkout.
