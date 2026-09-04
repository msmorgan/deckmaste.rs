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

## Landing record

Start: 2026-09-04T11:05:13-07:00. End: 2026-09-04T11:26:10-07:00.

- Measured on refreshed change `spkusszzmvqu`; the schema-4 lock contains
  16,825 covered identities. `DECKMASTE_COVERAGE_LOCK` accepts unset or
  `ratchet` for the existing ratchet path and `report` for a non-blocking lock
  delta. An invalid value names `DECKMASTE_COVERAGE_LOCK` and the accepted
  values.
- Report checks reuse the lock-delta renderer: every loss includes its
  identity and card name; every gain also includes its selected analysis.
  Report blessings write the current tree directly and do not inspect a
  retirement manifest or landing-record obligation. Ratchet checks and
  blessings retain the existing retirement path.
- Coverage before/after: 16,825 -> 16,825 selected and covered corpus units
  out of 32,641; lock +0/-0 and byte-unchanged; 0 newly covered identities,
  0 drops, 0 selected-uncovered units, unresolved ties, internal failures,
  exception resolutions/uses, round-trip mismatches, ownership failures,
  traversal failures, gaps, overlaps, synthetic claims, provenance-plan
  mismatches, or forbidden licensing checkers. Newly covered identities and
  selected analyses: none.
- Construction declarations before/after: 391 -> 391. Selection census:
  13,328 -> 13,328 unique, 3,497 -> 3,497 specificity-resolved, and 0
  exception-resolved selections. The licensing census remains 20 permitted / 0
  forbidden.
- Positive gates on the refreshed tree: `cargo fmt --all` clean; `cargo
  clippy -p xtask --all-targets -- -D warnings` clean; `cargo test -p xtask`
  green; both `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage
  --check --workers 8` and default-mode `cargo xtask english_v2 coverage
  --check --workers 8` exited 0 and ended in `lock_mode=report` and
  `lock_mode=ratchet`, respectively. No citation changed, so citation gates
  were not required.
- Performance advisory: all three corpus commands used 8 workers and one
  foreground process. Report coverage took 143.735 s at 181,512 ns/B with
  host load 26.16/27.57/22.47; ratchet coverage took 83.740 s at 109,150 ns/B
  with load 17.83/24.70/21.91; ambiguity took 83.195 s at 114,651 ns/B with
  load 12.43/20.53/20.75. These exceed the 16.26 s quiet-host ceiling under
  concurrent host load and are advisory, not a STOP.
- Assurance census: restored 0; re-spelled 0; ignored 0; added 2; removed 0.
  The added tests cover report-mode synthetic loss/gain output and blessing
  without a retirement manifest, plus invalid mode input; the existing
  renderer test now spells the report summary mode.
- Deviations and additions: none beyond the ticket's lock-mode tests. STOP:
  none. Glossary gap: none.
