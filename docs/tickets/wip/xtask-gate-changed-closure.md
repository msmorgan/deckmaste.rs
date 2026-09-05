---
needs: []
---
# `cargo xtask gate --changed`: test the reverse-dependency closure of what changed

CLAUDE.md's gate-scope rule (2026-09-05) says a landing tests the
reverse-dependency closure of every crate whose code or consumed data
changed — never the whole workspace, never a hand-picked `-p` list. Today the
closure is computed by hand from `cargo metadata`, so it is wrong as often as
it is right (the with-preposition landing missed `deckmaste_construction_core`'s
builtin_v2 integration test; several landings ran the 310 s workspace suite for
a two-crate change).

Pinned shape: an xtask subcommand `cargo xtask gate --changed [--from <rev>]`
that (1) lists changed paths from `jj diff --from <rev> --summary` (default:
the feature's fork point, `default@`; accept a revset), (2) maps each path to
its owning crate (`Cargo.toml` ancestry) or to a declaration-data root
(`plugins/builtin_v2/` → its reader crates found by grepping `include_str!`/
path literals in `crates/*/src` and `crates/*/tests`; `core_verbs.ron` → its
crate), (3) computes the reverse-dependency closure over `cargo metadata
--no-deps` workspace packages, (4) prints the exact `cargo test -p …` line and,
with `--run`, executes it (and `cargo clippy --all-targets -- -D warnings` for
the same set with `--clippy`). Paths outside any crate (docs, tickets, the
lock, CI) contribute no crate. A change under a shared-layer crate
(`deckmaste_data`, `deckmaste_catalogs`) simply yields its larger closure.
Tests: closure of `construction_core` = {construction_core, construction,
english_v2, xtask}; of `english_v2` = {english_v2, xtask}; of a
`plugins/builtin_v2` path = the reader set; of a docs-only diff = empty with a
clear message; a fixture repo is not needed — test the pure functions over a
`cargo metadata` JSON snapshot and a path list.

Consumption boundary: `crates/xtask` only. After landing, CLAUDE.md's
gate-scope bullet names the command. Standard constraints apply.

## Landing record

Tree: `xoqmtkpz` after `kata refresh` reported a no-op.

PROVE:

- The changed-path closure for this landing is `xtask`; `cargo xtask gate
  --changed` printed `cargo test -p xtask`, and `cargo xtask gate --changed
  --run` executed it successfully.
- `cargo fmt --all`, `cargo clippy -p xtask --all-targets -- -D warnings`, and
  `cargo test -p xtask` passed. Test artifact: `test result: ok` (465 tests
  invoked by the selected package, command exit 0).
- Grammar and corpus gates: unchanged (no grammar change). No coverage,
  selection, roundtrip, licensing-checker, or environment-load result was
  remeasured because this ticket changes only `xtask`'s gate machinery.

DISCLOSE:

- Newly covered identities: none (no grammar change). Selection census and
  construction pair: unchanged (no grammar change).
- Deviations and additions: none beyond the ticket's command-surface and
  snapshot/path-list tests.
- Assurance: restored 0; re-spelled 0; ignored 0; added 6; removed 0.
- STOP: none. Glossary gap: none. Citations: unchanged.

REPORT:

- Closure gate: `cargo test -p xtask`; strict companion:
  `cargo clippy -p xtask --all-targets -- -D warnings`.
- Coverage lock, construction count, homograph/form-literal overlap
  inventories, and performance advisory: unchanged (no grammar change).
- Verification host load after the no-op refresh: 16.41 / 17.02 / 13.68
  (1/5/15 minute); `CARGO_BUILD_JOBS=8`.
