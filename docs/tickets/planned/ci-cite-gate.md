---
needs: []
---
**CI runs no real citation checking.** The workflow's only citation step is
`cargo xtask cite coverage --check` — the self-contained path-heuristic
ratchet. The checker the README leads with, `cargo xtask cite check`, is a
64-line shim (`crates/xtask/src/cite.rs:34`) that shells out to
`~/.claude/skills/mtg-rules/scripts/cite` — a fish script in a different,
uncommitted repository — and needs the CR snapshot under `data/rules/`,
which the CI data mirror does not carry. So the repo's flagship
verification claim is unverifiable from a clean clone: stale or
unregistered citations surface only on a maintainer machine.

Wire it in, either way closes the gap:

1. **Install the skill in CI** — check out `msmorgan/mtg-rules` at a
   pinned revision, fetch just the rules snapshot, run
   `cargo xtask cite check` (0 stale; `--list-noncompliant` empty); or
2. **Vendor the checker** — move the cite script into this repo's
   `scripts/` and make the skill the consumer (decide which copy is
   authoritative), removing the external dependency entirely.

Delete the ci.yml comment block documenting the gap along with the gap.
Coordinate with `ci-derived-data-fixtures` (owns CI data staging) on where
the CR snapshot comes from.
