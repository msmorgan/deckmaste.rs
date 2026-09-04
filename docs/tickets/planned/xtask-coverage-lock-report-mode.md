---
needs: []
---
# Coverage lock report mode for the generalization refactors

Ruling 2026-09-04 (`CLAUDE.md`, "Lock ratchet SUSPENDED"): while the
generalization refactors land, `english-v2-coverage.lock` is not monotonic.
`cargo xtask english_v2 coverage --check` still enforces it: a drop fails the
gate and `--bless --retire` refuses without the retirement manifest and
obligation line.

Add a lock mode read from the environment, `DECKMASTE_COVERAGE_LOCK`:

- unset or `ratchet`: today's behaviour, unchanged;
- `report`: `--check` prints the lock delta — every identity that stopped
  being covered and every identity newly covered, each with its card name
  and, for gains, its selected analysis (the same rows the newly-covered
  listing already renders) — and exits 0 on drops and gains alike; `--bless`
  writes the lock to exactly what the tree covers with no manifest and no
  obligation line; the summary line carries `lock_mode=report` so a review
  can see which mode measured it.

Every other check in the command stays a gate in both modes: unresolved ties,
roundtrip mismatches, ownership and traversal failures, internal failures,
the licensing-checker forbidden count.

Consumption boundary: `crates/xtask` only. Tests: report mode exits 0 on a
synthetic drop and prints the delta; ratchet mode still fails it; bless in
report mode needs no manifest; the summary carries the mode. Standard
constraints apply.
