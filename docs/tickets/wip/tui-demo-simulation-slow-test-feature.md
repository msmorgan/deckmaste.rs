---
needs: []
---
# Move the TUI demo auto-play simulation behind a `slow-tests` feature

`game::tests::demo_auto_plays_to_completion`
(`crates/deckmaste_tui/src/game.rs:330`) runs 130–236 s on the dev host
(measured 2026-09-05; high variance) — about 45% of the whole
`cargo test --workspace` wall time. No other test in any crate exceeds 20 s.
Every english_v2 landing whose gate scope is the workspace suite pays for
auto-playing a demo game.

Pinned shape: a `slow-tests` cargo feature on `deckmaste_tui` (default off)
gating this test with `#[cfg(feature = "slow-tests")]` — not `#[ignore]`
(assurance: `#[ignore]` needs a named blocker; this is a schedule, not a
blocker) — plus one documented invocation that runs it
(`cargo test -p deckmaste_tui --features slow-tests`), wired into CI's full
job so the test still runs on every trunk change, and a one-line note in
CLAUDE.md's gate-scope section that `cargo test --workspace` excludes
`slow-tests` and CI runs them. If the test's variance comes from a
nondeterministic seed, say so in the landing record (do not fix it here).

Fences: deleting or weakening the test; `#[ignore]`; any change to what the
test asserts. Consumption boundary: `crates/deckmaste_tui`, CI config,
CLAUDE.md. Standard constraints apply.

## Landing record

Measured on refreshed change `zxwtnrqroovp` (refresh was a no-op) with
`CARGO_BUILD_JOBS=8` on the shared host.

- Added default-off `deckmaste_tui/slow-tests`; only
  `game::tests::demo_auto_plays_to_completion` is conditional on it. The test
  and its assertion are retained. CI's full job runs the feature-enabled TUI
  suite, while the normal workspace suite leaves the feature disabled.
- Gates: `cargo fmt --all`; both requested TUI clippy invocations with
  `-D warnings`; YAML parsing with PyYAML; and `cargo test --workspace` all
  passed. The default test listing does not contain the gated test.
- Timing: pre-change `cargo test --workspace` was 486 s; refreshed
  feature-default `cargo test --workspace` was 332 s (154 s less under shared
  host load). Feature-enabled TUI runs took 256.43 s and 234.18 s for the
  76-test unit target (22.25 s variance); both ran and passed the demo
  simulation. Its test seed is fixed, so this is scheduling/load variance, not
  a nondeterministic seed.
- Corpus figures: coverage lock, construction count, selection census, newly
  covered identities, and performance advisory are unchanged (no grammar or
  xtask change); no english_v2 corpus gates were run.
- Assurance counts: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
  The existing test is feature-gated, not ignored, deleted, or weakened.
- Deviations and additions: none. STOP: none. Glossary gap: none.
