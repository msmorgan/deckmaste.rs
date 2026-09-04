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
Review: 2026-09-04T11:30-11:50-07:00.

- Measured on refreshed change `omnwyvnkprum` (review commit; the ticket-only
  commit above it reaches no gate); the schema-4 lock contains 16,825 covered
  identities and is byte-unchanged. `DECKMASTE_COVERAGE_LOCK` accepts unset or
  `ratchet` for the existing ratchet path and `report` for a non-blocking lock
  delta. An invalid value names `DECKMASTE_COVERAGE_LOCK` and the accepted
  values, before the corpus is loaded.
- Report checks reuse the single lock-delta renderer, so ratchet output is
  unchanged: every loss carries its identity and its card name (with face when
  the corpus records one), every gain also carries its selected analysis.
  Loss names come from the current corpus, falling back to the name the
  baseline lock recorded only when the corpus no longer holds the identity at
  all. Report blessings write the current tree directly and do not inspect a
  retirement manifest or landing-record obligation; supplying `--retire` in
  report mode is refused. Ratchet checks and blessings retain the existing
  retirement path byte-for-byte.
- Coverage before/after: 16,825 -> 16,825 selected and covered corpus units
  out of 32,641; lock +0/-0 and byte-unchanged; 0 newly covered identities,
  0 drops, 0 selected-uncovered units, unresolved ties, internal failures,
  exception resolutions/uses, round-trip mismatches, ownership failures,
  traversal failures, gaps, overlaps, synthetic claims, provenance-plan
  mismatches, or forbidden licensing checkers. Newly covered identities and
  selected analyses: none. Identities that stopped being covered: none.
- Construction declarations before/after: 397 -> 397 (re-measured at review;
  the implementer's record said 391 -> 391, which was stale — the tree held 397
  both before and after the change, and an xtask-only diff cannot move it).
  Selection census: 13,328 -> 13,328 unique, 3,497 -> 3,497
  specificity-resolved, and 0 exception-resolved selections. The licensing
  census remains 20 permitted / 0 forbidden.
- Positive gates on the reviewed, refreshed tree: `cargo fmt --all -- --check`
  clean; `cargo clippy -p xtask --all-targets -- -D warnings` clean;
  `cargo test -p xtask` 451 passed / 0 failed / 1 ignored (plus 12 + 1 + 1 in
  the binary and integration targets); `cargo xtask english_v2 coverage --check
  --workers 8` exited 0 with a summary ending `lock_mode=ratchet`;
  `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check
  --workers 8` exited 0 with a summary ending `lock_mode=report`;
  `DECKMASTE_COVERAGE_LOCK=abc` exited nonzero, naming the variable, the
  rejected value and both accepted values;
  `cargo xtask english_v2 ambiguity --require-resolved --workers 8` reported
  `unresolved_ties=0`. No citation changed, so citation gates were not required.
- Performance advisory: all corpus commands used 8 workers and one foreground
  process. Ratchet coverage took 82.670 s at 104,606 ns/B with host load
  5.92/9.12/12.98; report coverage took 91.399 s at 138,331 ns/B with load
  7.70/9.29/12.67; ambiguity took 77.506 s at 105,636 ns/B with load
  5.43/8.04/11.85. Contention stamp: two sol executors and one other review
  shared the host (load ~12-27 across the window). These exceed the 16.26 s
  quiet-host ceiling under concurrent load and are advisory, not a STOP.
- Assurance census: restored 0; re-spelled 1 (the report-mode delta test was
  re-spelled against a fixture that names each identity distinctly, because its
  original assertions could not fail); ignored 0; added 5 (2 by the
  implementer, 3 at review); removed 0.
- Deviations and additions: report mode also handles a schema-3 baseline lock
  (`--check` prints the delta instead of demanding migration, `--bless` writes a
  fresh schema-4 lock) — the implementer added this branch and did not disclose
  it; review kept it, because refusing in report mode would reintroduce a
  blocking lock gate, and covered it with a test. STOP: none. Glossary gap:
  none.

### Review corrections

- MEDIUM — loss rows named identities from the baseline lock, so an identity the
  lock did not record per-unit (every schema-3 loss, and any V4 identity absent
  from the lock's normalization units) printed `<unknown>`, and a renamed card
  printed its stale name. Fixed: `CoverageReport::lock_delta_details` indexes
  the current corpus once and supplies the card label (with face) for every
  identity, losses included; the baseline lock is now only the fallback for an
  identity the corpus has dropped entirely.
- MEDIUM — the delivered report-mode test could not fail: the gate fixture gave
  every row the card name `Covered` and an empty rendered analysis, so
  `card "Covered"` / `selected_analysis ""` held for any row or a hardcoded
  string. Fixed: the fixture now derives a distinct card name and analysis per
  identity and can mark an identity present-but-unparsed; the test asserts the
  corpus-sourced name (with face) for one loss, the lock-sourced name for the
  other, and the per-identity gain analysis. Each assertion was falsified once
  against a deliberately broken renderer before being kept.
- MEDIUM — three of the ticket's named cases had no test. Added:
  `coverage_lock_policy_reads_the_environment_value` (unset -> ratchet,
  `ratchet`, `report`), the empty-string and case-variant rejections in
  `coverage_lock_policy_rejects_unknown_environment_values`,
  `report_policy_rejects_a_retirement_manifest`, and
  `report_policy_reports_a_schema_three_lock_instead_of_demanding_migration`.
  `CoverageLockPolicy::from_environment` was split into a testable
  `from_value(Result<String, VarError>)` so the unset case is asserted without
  mutating the process environment.
- MEDIUM — the record's construction count (391 -> 391) did not match the tree
  (397 both before and after). Corrected above.
- Verified unchanged in ratchet mode: the schema-3 migration refusal, the
  `lost N previously covered corpus identit{y,ies}` failure, the retirement
  manifest match check, the authentication call, and the coordinator-ruling
  warning text.
