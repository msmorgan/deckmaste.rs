---
needs: [english-v4-cutover-v3]
---
# A cheap test selector to replace the retired derived gate

Priority: LOW. The user (2026-10-07): 'at this time it's wayyyy too much extra shit'. Do not schedule before `english-v4-cutover-v3`.

## Why

The retired `cargo xtask gate --changed` derived the reverse-dependency closure
of every crate whose code or consumed data changed and ran `cargo test` over
it. Two failure modes made it a tax:

1. Any change to `xtask` itself pulled xtask's whole test suite into the gate,
   including the `lean_check` tests that run real `lake` builds (15+ minutes).
2. A change to a low-level crate (e.g. `deckmaste_syntax`)
   rebuilt and tested nearly the whole workspace, with no caching between
   landings.

On 2026-10-07 the user retired it: "the gate is a badly designed tax on
productivity. get rid of it." The interim rule is `cargo test -p <crate>` plus
clippy for the crates changed, with Lean checks only when Lean sources or
plugin definitions changed. CI's full test job is unchanged and remains
authoritative.

## Goal

Requirements for a replacement, not a design:

- Select tests by actual dependency on the changed code, not by package-level
  reverse closure alone (for example test targets whose compiled artifacts
  changed, or a content-hash cache of passing test results keyed by the test
  binary's input fingerprint, so an unchanged test binary is never re-run).
- Exclude slow suites (`lean_check`, `slow-tests`) unless their own inputs
  (`lean/`, `plugins_v3`, the Lean checker code) changed.
- Run in well under a minute for a typical grammar or lexicon landing on an
  idle host.
- Print the exact command list it will run before running it, so an agent can
  state it in a landing record.
- Be optional for agents and authoritative only in CI's full job.

## Non-goals

- New CI infrastructure.
- Replacing `cargo test`.
- Any mandatory step in the landing contract (the user wants fewer mandatory
  steps, not more).

## Method

Standard constraints apply. Write a design note before implementation naming
the mechanism chosen and the measured before/after wall times (integer ns) on
three representative landings: a grammar-only change, a lexicon-only change,
and an xtask-only change. Evidence under `target/`.

## Landing record

Must state the three timings, the selection rule, and what the full CI job
still runs.
