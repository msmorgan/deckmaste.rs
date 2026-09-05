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

Tree: `xoqmtkpz` as implemented; reviewed, corrected and measured on
`ruskkqtr` after `kata refresh` reported a no-op.

PROVE:

- The changed-path closure for this landing is `xtask`. After the review
  corrections `cargo xtask gate --changed` prints `cargo test -p xtask` (with
  `--clippy`, also `cargo clippy -p xtask --all-targets -- -D warnings`), and
  `cargo xtask gate --changed --run` executed it successfully. As delivered it
  printed `cargo test -p deckmaste -p xtask`, because every `docs/` path was
  attributed to the workspace-root package; see Review corrections.
- `cargo fmt --all`, `cargo clippy -p xtask --all-targets -- -D warnings`, and
  `cargo test -p xtask` pass: `test result: ok. 468 passed; 0 failed; 1 ignored`
  for the library, plus 13 + 1 + 1 in the binary and integration targets. As
  delivered `cargo test -p xtask` was RED: the new CLI test asserted that a
  bare `gate` fails to parse, which it does not.
- `--run` propagates a failed child: the clippy leg of the verification run
  failed once on a doc-markdown lint and the gate exited nonzero
  (`Error: selected cargo gate failed: exit status: 101`) rather than
  reporting success.
- Real-metadata spot checks against `cargo metadata --no-deps`: a diff whose
  crate paths are `deckmaste_construction`, `deckmaste_english_v2`,
  `deckmaste_tui` and `xtask` yields
  `cargo test -p deckmaste_construction -p deckmaste_english_v2 -p deckmaste_tui -p deckmaste -p xtask`
  (`deckmaste` reached as a dependent of `deckmaste_tui`), while the `docs/`,
  `CLAUDE.md` and `.github/` paths in the same range contribute nothing.
- Grammar and corpus gates: unchanged (no grammar change). No coverage,
  selection, roundtrip, licensing-checker, or environment-load result was
  remeasured because this ticket changes only `xtask`'s gate machinery.

DISCLOSE:

- Newly covered identities: none (no grammar change). Selection census and
  construction pair: unchanged (no grammar change).
- Deviations and additions: none beyond the ticket's command surface and its
  snapshot/path-list tests. The review added no command surface.
- Assurance: restored 0; re-spelled 3 (the CLI test now asserts the contract
  that holds — a bare `gate` parses and refuses to act; the summary-parser test
  now uses jj's real rename rendering; the docs-only test now runs against a
  snapshot that carries the workspace-root package); ignored 0; added 4
  (`core_verbs.ron` ownership, root-package ownership, a move between crates,
  the strict clippy command); removed 0. Test count in `gate.rs`: 5 → 9, plus
  the CLI parse test.
- STOP: none. Glossary gap: none. Citations: unchanged.

### Review corrections

- HIGH — `cargo test -p xtask` was red on the delivered tree:
  `gate_changed_accepts_its_exact_flags` asserted
  `Cli::try_parse_from(["cargo xtask", "gate"]).is_err()`, but every `GateArgs`
  field is optional, so a bare `gate` parses. Fixed by asserting the real
  contract: a bare `gate` parses and `gate::run` refuses it.
- HIGH — every path outside a crate was attributed to the workspace-root
  package `deckmaste`, whose directory is the whole repository, so a docs-only
  or ticket-only diff produced a gate instead of the ticket's "no crates
  affected". The delivered `docs_only_paths_produce_no_gate` test passed only
  because the metadata snapshot omitted the root package. Fixed by giving the
  workspace-root package the ownership it actually has — its manifest and the
  top-level directories its targets are built from — and by putting the root
  package into the snapshot.
- MEDIUM — rename parsing did not match jj's output. jj 0.44 always brackets
  what differs (`docs/tickets/{wip => done}/slug.md`), so the delivered parser
  produced two garbage paths whenever the shared prefix was non-empty, and a
  file moved between crates gated neither crate. The delivered test pinned
  `R crates/a.rs => crates/b.rs`, a rendering jj never emits. Fixed by
  expanding jj's bracketed form (renames and copies) and pinning all three real
  renderings.
- LOW — the printed command and the executed command were built by two separate
  code paths and could drift; they are now one argument vector, printed and
  run. The `builtin_v2` reader scan (23 MB of `.rs` sources) ran on every
  invocation and now runs only when a `plugins/builtin_v2` path changed.
- MEDIUM — the CLAUDE.md edit appended a sentence after "Until `cargo xtask
  gate --changed` exists, compute the closure with `cargo metadata` …",
  leaving a now-false clause standing. That clause is replaced by the command.

REPORT:

- Closure gate: `cargo test -p xtask`; strict companion:
  `cargo clippy -p xtask --all-targets -- -D warnings`.
- Coverage lock, construction count, homograph/form-literal overlap
  inventories: unchanged (no grammar change, nothing re-measured).
- Performance advisory: no corpus command runs in this landing, so there is no
  wall-time or per-byte telemetry to compare against the 16.26s ceiling.
  `cargo xtask gate --changed` itself costs one `jj diff --summary` plus one
  `cargo metadata`. Contention during verification: three other executors were
  live in sibling workspaces; `CARGO_BUILD_JOBS=8`. One of them integrated
  mid-verification, and because `--from default@` is a two-tree diff rather
  than a range, the stale tree briefly selected `deckmaste_tui` and `deckmaste`
  as well; both suites passed and the closure returned to `xtask` alone after
  the refresh.
