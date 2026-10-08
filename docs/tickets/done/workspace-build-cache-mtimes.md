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

1. **Commits** (on claim `tsxvuwtl`): `pzryzulx` provision-workspace mtime
   sync + phase timing + CLAUDE.md note; `uqpkzwsx` `[lib] test = false` for
   `deckmaste_english_v3`; `yloooqqq` build-script `rerun-if-changed`; plus
   this record.
2. **Timings** (24 cores; other sessions' builds running throughout, load
   5-14, so wall times are noisy):

   | measurement | before | after |
   |---|---|---|
   | `cargo build -p xtask` in a fresh probe workspace | 7m56 (25 workspace crates compiled; load 5.8) | 0.07s (nothing compiled; load 6.0) |
   | provisioning, total | n/a | 45.9s |
   | - symlinks | 0s | 0s |
   | - `plugins/wizards` generate | 8.2-9.5s | 9.5s |
   | - `target/` reflink | 29-35s | 35.2s |
   | - mtime sync (new) | prototype 16-23s serial | 1.1s (4960 synced, 1 differing) |
   | - `lean/.lake` reflink | 0.16s | (now timed too) |
   | `cargo test -p deckmaste_english_v3 --no-run`, feature ws | 9m28 (probe, pre-fix) | 10m38 (load 10-13; only `deckmaste_english_v3` lib + `tests/main` compiled, no lib-test unit) |
   | `cargo test -p deckmaste_english_v3` (built) | | 16.5s, 518 passed |

   The after probe's tree matched what default's `target/` was last built
   from, so nothing needed recompiling; a probe on a tree with real source
   differences recompiles only the crates that own them (the probe that found
   the cause measured 2m44 for this build).
3. **Build scripts** (all seven workspace `build.rs` files read inputs
   without declaring any; all now declare). `crates/deckmaste_construction_core/src/emit/build.rs`
   is a source module, not a build script.
   - `xtask`: `build.rs`, `../../data/gen/catalogs-legacy`,
     `../../data/derived/cards.jsonl`.
   - `deckmaste_migrations`: `build.rs`, `../../data/catalogs`,
     `../../data/rules`.
   - `deckmaste_spelling`: `build.rs`, `../../data/gen/catalogs-legacy`.
   - For those three, an absent input is declared as its nearest existing
     ancestor under `data/` (so its appearance re-runs the script; cargo
     re-runs every build on a missing declared path). With no `data/` at all
     nothing is declared, keeping cargo's default (re-run on any package
     change) for bare checkouts.
   - `deckmaste_plugin`, `deckmaste_engine`, `deckmaste_tui`,
     `deckmaste_noncanon` (byte-identical copies, kept in sync): `build.rs`
     always; `../../plugins` only while `plugins/wizards` is absent. While
     present, the 31k-file corpus is deliberately not watched: only presence
     matters and every workspace regenerates it, so watching it would rebuild
     four crates per workspace. Removing the corpus needs `touch build.rs`
     (documented in the file).
   - The cfgs emitted are unchanged (checked in each script's `output`).
4. **Item 2 search**: `rg '#\[test\]|#\[cfg\(test\)\]|#\[cfg\(.*test'
   crates/deckmaste_english_v3/src` matched nothing; all tests are in
   `tests/main.rs`.
5. **Tests**: `cargo test -p deckmaste_english_v3` 518 passed;
   `cargo test -p deckmaste_plugin -p deckmaste_migrations -p
   deckmaste_spelling -p deckmaste_noncanon` all ok (1 pre-existing ignore);
   `cargo clippy -p xtask -p deckmaste_plugin -p deckmaste_engine -p
   deckmaste_tui -p deckmaste_noncanon -p deckmaste_migrations -p
   deckmaste_spelling --tests -- -D warnings` clean. `cargo test` for
   `xtask`, `deckmaste_engine`, `deckmaste_tui` not run (build-script-only
   change, covered by clippy). Real `jj st` in the after probe: clean.
   Assurance counts: 0 restored, 0 re-spelled, 0 ignored, 0 added, 0
   removed.
6. **Deviations**: the five other build scripts beyond `xtask` and
   `deckmaste_plugin` were fixed too (scope item 3 allows it). Provisioning
   now prints per-phase timings. A follow-up proposal to reflink
   `plugins/wizards` from default instead of regenerating was **not done**:
   generate is 9.5s of 45.9s (the `target/` reflink dominates), and default
   keeps an empty `plugins/wizards`, so there is nothing to copy from it.
