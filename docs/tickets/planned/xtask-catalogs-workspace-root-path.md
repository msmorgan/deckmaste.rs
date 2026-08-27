---
needs: []
---
Stop resolving the project workspace root from a compile-time-baked path.

`crates/deckmaste_catalogs/src/io.rs` canonicalizes
`env!("CARGO_MANIFEST_DIR")/../..` at run time, purely as a safety check that a
catalog output directory is not the workspace root or an ancestor of it. The
path is baked in when the crate is compiled, so the check fails whenever the
binary runs from a tree that is not the one it was built in — the canonicalize
returns `No such file or directory` and every caller unwraps it into a panic.

Observed: five `catalogs::tests::*` cases in `cargo test -p xtask --lib`
failing with "canonicalizing project workspace root" in one jj feature
workspace, while the same tests pass in a freshly provisioned workspace and in
the default checkout. The trigger is the environment, not the code under test —
this repository runs several workspaces under `.workspaces/` and reflinks a
prewarmed `target/` between them, so a binary built against one manifest path
routinely executes from another. Nothing about the failure is specific to
catalogs; it is the baked path.

The safety check itself is worth keeping. Derive the root the way a run-time
check should — from the output path's own ancestry, from `CARGO_MANIFEST_DIR`
resolved lazily and treated as absent when it does not exist, or from an
explicit caller-supplied root — and degrade to skipping the ancestor check
rather than panicking when no root can be resolved.

Acceptance: `cargo test -p xtask --lib` passes from a freshly provisioned
workspace, from the default checkout, and from a workspace whose `target/` was
copied from a different checkout; the ancestor check still refuses a catalog
output directory that is the workspace root or an ancestor of it.
