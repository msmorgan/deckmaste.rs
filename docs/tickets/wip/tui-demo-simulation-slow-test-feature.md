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
