---
needs: []
---
# Disable test targets on the frozen `english-v3` crates until `english-v4-cutover-v3`

`deckmaste_english_v3`, `deckmaste_construction_v3`,
`deckmaste_construction_v3_core`, and `deckmaste_lexical_source` are frozen and
scheduled for deletion at `english-v4-cutover-v3` (`CLAUDE.md`,
`docs/decisions/english-v4.md`), and five tests in `deckmaste_english_v3` fail
under workspace test runs. Disable all test and doctest targets in those four
manifests so workspace test sweeps do not run or trip over the frozen v3 stack.

## Actionable changes

- In `crates/deckmaste_english_v3/Cargo.toml`,
  `crates/deckmaste_construction_v3/Cargo.toml`,
  `crates/deckmaste_construction_v3_core/Cargo.toml`, and
  `crates/deckmaste_lexical_source/Cargo.toml`:
  - Set `autotests = false` under `[package]`; and
  - Set `test = false` and `doctest = false` under `[lib]`.

## Gate

- `cargo test -p deckmaste_english_v3 -p deckmaste_construction_v3 -p
  deckmaste_construction_v3_core -p deckmaste_lexical_source` exits 0 with no
  test targets executed.
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_construction_v3 -p
  deckmaste_construction_v3_core -p deckmaste_lexical_source -- -D warnings`
  exits 0.

Standard constraints apply.
