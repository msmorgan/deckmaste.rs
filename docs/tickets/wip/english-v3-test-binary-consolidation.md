---
needs: []
---
# Consolidate integration-test binaries so the grammar loads once per package

## Why (user, 2026-10-07)

Every `tests/*.rs` file compiles to its own test binary, and each binary
spends about 5 s loading the lexicon and compiling the declarations before it
runs tests that take milliseconds. Counted on 2026-10-07:

| Package | `tests/*.rs` files |
|---|---|
| `deckmaste_english_v3` | 63 (plus `tests/common/mod.rs`) |
| `deckmaste_construction_v3` | 13 |
| `deckmaste_lexical_source` | 9 |
| `xtask` | 2 |
| `deckmaste_construction_v3_core` | 0 |

The derived gate runs about 94 suites one after another: about 8 min on an
idle host and 18 min on a loaded one, twice per landing plus once per review.
In the user's words, "these rounds are taking way too long due to
long-running badly-architected commands".

## Goal

- One integration-test target per package, e.g. `tests/main.rs` declaring every
  existing test file as a `mod`. Shared fixtures load once per binary through
  the existing lazily initialised statics in
  `crates/deckmaste_english_v3/tests/common/mod.rs` (`LEXICON`, `lexicon()`,
  `readings()`, `assert_constituents()` and siblings). The other packages
  get the same shape, with a `common` module wherever they repeat setup.
- Every existing `#[test]` is kept with identical assertions. Tests are moved,
  never deleted or weakened (Assurance). The only content changes allowed are
  the ones the module structure forces: path and `use` fixes, and duplicate
  helper definitions merged into `common`. Each merge is disclosed.
- Gate derivation: `crates/xtask/src/gate.rs` maps a path to its owning
  package (`closure_for_paths`, `owner_for_path`) and runs `cargo test -p …` over the
  reverse-dependency closure (`test_command`). No package declares `[[test]]`
  or `autotests`. The derivation should need no change. The landing confirms
  that `cargo xtask gate --changed` still selects the same packages for a test-file
  path and says whether any change was needed.
- Document how to add a new test module: a new landing adds a `mod` line to
  `tests/main.rs`. Put this in `crates/deckmaste_english_v3/tests/README` or
  the `common` module's doc comment.

## Method

Standard constraints apply. Land as a series with one commit per package.
For each package, run `cargo test -p X -- --list` before and after, and check
that the two test-name lists are equal, allowing only the module-path prefix
the move adds. Clippy and fmt must be clean. Measure gate wall time before and
after on the same host state and record it as integer ns, with host load and
worker count.

## Out of scope

Changing any test's assertions; cargo-nextest or any other test-runner
tooling.

## Landing record

Per-package test counts before and after, and the proof that the name lists
are equal. Gate wall times before and after (integer ns, host load). The
derived gate command and its outcome. Every helper merge, by name.
