---
needs: []
---
# Make a fresh workspace's first build reuse default's cache

## Why

Provisioning reflinks `default`'s `target/` into each new workspace, but every
workspace crate still rebuilds cold there. Cargo unit hashes are identical
across workspaces (the metadata hash is path-independent); the cause is mtimes:
a jj checkout stamps every file with the checkout time, so each workspace
crate's source looks newer than its reflinked fingerprint and all 25 workspace
crates rebuild (7m13 for `cargo build -p xtask`, 9m28 for the
`deckmaste_english_v3` test build). After copying `default`'s mtimes onto the
byte-identical files, only truly changed crates rebuild (2m44 / 3m21).

Two smaller costs ride along: `deckmaste_english_v3`'s lib-test unit has no
tests yet costs a full opt-2 compile, and the build scripts that detect
gitignored fixtures emit no `rerun-if-changed`, so cargo re-runs them (and
rebuilds their crates) whenever any package file's mtime moves.

## Scope

1. **Sync mtimes in `scripts/provision-workspace`.** After the `target/`
   reflink, for every tracked file whose bytes equal `default`'s copy, set its
   mtime to `default`'s (`touch -r`); skip missing and symlinked paths, run the
   comparisons in parallel, print one summary line (synced / differing).
   Best-effort: a failure never fails provisioning. Correct the script comment
   that blames a path-dependent hash; `incremental/` deletion stays (rustc keys
   it on cwd).
2. **`[lib] test = false` in `crates/deckmaste_english_v3/Cargo.toml`**, after
   confirming its `src/` holds no `#[test]` / `#[cfg(test)]`.
3. **`rerun-if-changed` in the build scripts** of `xtask` and
   `deckmaste_plugin` (and every other workspace `build.rs` that reads inputs
   without declaring them): declare exactly the inputs each script reads plus
   `build.rs`. What the scripts generate does not change.

## Method

Standard constraints apply. Verification: time a cold `cargo build -p xtask`
in a freshly provisioned probe workspace before and after item 1, and time
`cargo test -p deckmaste_english_v3 --no-run` and `cargo test -p
deckmaste_english_v3` in the feature workspace after items 2-3 (all tests
pass). A real `jj st` in the probe workspace must report no changes after
provisioning. `cargo clippy -p xtask -p deckmaste_plugin --tests -- -D
warnings`.

## Landing record

To be written by the claimant: the six items below plus the timings.

1. Change ids of the landed commits.
2. The timing table (before / after) with host load.
3. Each `build.rs` changed and what it now declares.
4. The `#[test]` / `#[cfg(test)]` search result for item 2.
5. Test and clippy commands run, with outcomes.
6. Deviations, and anything left out.
