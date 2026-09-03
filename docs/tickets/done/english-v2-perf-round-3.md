Performance round 3 (perf-round-2 landing review M1/M2/M3).
- Add R0 counters that measure column/scan work directly (chart columns
  visited, predictions per column, scan attempts), then re-evaluate R8
  (token-indexed chart: 6.06x fewer columns) against THAT evidence —
  predictions are 50.5% of all recorded events and scale with column
  count; decide take/decline with the number, not the top-20 view.
- `CORPUS_WALL_CEILING_SECONDS` carries a comment citing the rewrite
  ADR's "corpus timing ceiling and acceptance-cost telemetry" ruling.
- Disclose the `parse --json` `"message": null` omission in its record
  (or restore it); state the new metric's residual load spread.
- Per-gate ceiling decision (coverage ~14 s quiet vs the parse-calibrated
  16.26 s) recorded as a ruling proposal, not a silent change.
Zero semantic change per unit; timings state worker count and host load;
scratch under ~/Dump only. Standard constraints apply.

## Landing record

The corpus remains exactly 32,641 units: 16,174 accepted/selected/covered and
16,467 ordinary parse failures, with zero round-trip mismatches, unresolved
ties, internal failures, exception uses, ownership failures, gaps, overlaps,
synthetic claims, or provenance mismatches. The coverage lock passed
byte-unchanged at source fingerprint
`e85359d7b8c578df13dff2fdf7c743a520a5b367d5ed25ab0a5f03cb8b3637dd`
and normalization digest
`f3a2fccd079f0bc53c79b4c23e28e0351b3cf69a5324b3893c695638935341c9`.
The generated grammar remains 434 unique construction origins (378 literal
`construction` declarations) before and after. No grammar, candidate,
selection, rendering, ownership, or corpus identity changed.

R0 now reports two feature-gated global work counters beside the existing
generated per-construction rows: nonempty chart columns visited and lexical
scanner calls that missed the per-parse cache. It also derives predictions per
visited column from the existing prediction counter. A generated 4-worker
pre-change profile took 41.168 s at load 22.35/19.26/17.50 and reported
217,818,070 predictions, 17,692,957 completions, 69,470,372
materializations, 46,304,954 memo misses, and 80,280,706 clone-heavy events.
The 4-worker profile with the new counters took 56.102 s at load
34.30/23.21/19.20 and reproduced every old total exactly. It additionally
reported 767,843 visited columns, about 284 predictions per visited column, and
47,923,085 uncached scan attempts (62.41 per visited column). The profile is
feature-gated and its observation overhead is outside the ordinary-command
ceiling; the two walls are not a speed comparison because host load differed
sharply.

R8 token indexing was re-evaluated against those direct counters and declined.
The established 6.06 bytes/token estimate describes the byte-indexed chart's
allocated coordinate slots, but prediction and scan work already occurs only
at the 767,843 live columns. Token coordinates would compact empty slot
storage; they would not remove the measured 218 million predictions or 47.9
million scanner calls at live boundaries. That allocation-only opportunity
does not justify a structural coordinate rewrite carrying byte-span and
scanner-mapping risk.

Fresh ordinary dev-profile samples used 16 explicitly requested workers. The
host was heavily contended, so every ceiling warning below is advisory under
the governing quiet-host ruling:

| gate | wall | accepted CPU cost | host load (1/5/15 min) | result |
|---|---:|---:|---:|---|
| `parse` | 30.603 s | 111,856 ns/byte | 39.36/27.63/21.32 | exact 16,174/16,467 partition |
| `roundtrip --require-clean` | 23.838 s | 113,286 ns/byte | 33.25/27.32/21.45 | 16,174 clean, zero mismatches |
| `ambiguity --require-resolved` | 23.199 s | 110,268 ns/byte | 31.30/27.29/21.59 | zero unresolved/exception uses |
| `coverage --check` | 30.143 s | 112,966 ns/byte | 30.82/27.53/21.88 | selected set totally covered; lock clean |

With the binary already built, `expand` took 1.339 s at load
15.80/20.94/20.43 and `report` took 1.194 s at load 17.18/21.29/20.54; neither
command has corpus workers. The accepted thread-CPU metric is not wholly
load-insensitive: the prior quiet 16-worker parse recorded 67.544 us/byte,
while this contended sample recorded 111,856 ns/byte, a residual 1.66x spread.
The four similarly loaded samples above cluster from 110,268 to 113,286
ns/byte (1.027x). `CLOCK_THREAD_CPUTIME_ID` excludes time descheduled, but
shared-cache and memory-bandwidth contention still inflate actual worker CPU.

`parse --json` retains the round-2 omission that its earlier landing record did
not disclose: a clean accepted row omits the `message` member rather than
serializing `"message": null` (the current corpus has no accepted mismatches,
so this applies to all 16,174 accepted rows). Failure rows retain their real
messages. The reviewed effect was a 2.4% parse-report reduction; round 3 makes
no further JSON-shape change and strengthens the fixture assertion for the
retained behavior.

**Ruling proposal (not enacted):** retain one 16.26 s quiet-host ceiling for
all corpus commands rather than create or relax a coverage-specific ceiling.
Coverage's prior quiet 13.744 s sample passes the existing criterion, and this
round's four accepted-cost samples move together under host contention; a
per-gate relaxation would encode transient command overhead while weakening
the shared regression tripwire. If coverage persistently breaches 16.26 s on
a quiet host, re-calibration remains a coordinator ruling. The constant is
unchanged and now cites the rewrite ADR's "corpus timing ceiling and
acceptance-cost telemetry" ruling in its source comment.

Verification passed:

- `cargo test -p deckmaste_english_v2 -p xtask -p deckmaste_construction_core --features xtask/parser-metrics --quiet`
- `cargo clippy -p deckmaste_construction_core -p deckmaste_english_v2 -p xtask --all-targets --features xtask/parser-metrics -- -D warnings`
- `cargo fmt --all -- --check`
- `cargo xtask cite check --list-noncompliant` (empty)
- `cargo xtask cite check` (zero stale)

Test assurance accounting: zero restored, one existing parse-JSON test
strengthened to authenticate the disclosed omission, zero ignored with
blockers, two metric tests added (work-event instrumentation and zero/populated
ratio behavior), and zero removed.

Deviations and additions:

- R8 was declined on direct live-column and scan evidence as recorded above.
- No per-gate ceiling change was made; the decision is recorded only as the
  ruling proposal above.
- No scratch artifact was created; command output was streamed or discarded
  directly rather than written outside `~/Dump`.
- No STOP was taken. The loaded-host warnings are advisory under the existing
  ADR and all semantic gates passed unchanged.


## Erratum (perf round 3 landing review, 2026-09-03)

Figures were measured on the pre-integrate tree (`c85a9d6ad972`, covered
16,174); the integrate rebased onto the since-reverted locative landing
(15,635) where none of the corpus figures held — today's head is back at
16,174 so the record reads correctly again. Records now stamp the
measured tree (CLAUDE.md). R8 is STRUCK, not deferred: live columns equal
the token set (767,843 vs ~747,358), predictions are invariant under
token indexing, total upside <1%. The reviewer's sweep counter shows
`requeue_completed_items_after_forest_growth` visiting 198.5M columns of
which 171.0M (86.1%) are empty — ~0.6% of parse; R9 (bounded requeue) is
the remaining structural target, low value now that the ceiling clears.
