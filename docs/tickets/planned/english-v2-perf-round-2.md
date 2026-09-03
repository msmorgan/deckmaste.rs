Performance round 2 (perf landing review H1/M1/M2/L6). Starting point:
the constant factor paid ~1.5x at fixed workers; the rest of the 3.8x was
the removed worker cap, so parallelism headroom is now spent.
- Per-byte acceptance cost from thread CPU time (CLOCK_THREAD_CPUTIME_ID)
  per unit, not summed wall; report it beside wall in every corpus
  command (telemetry per the timing ruling in the rewrite ADR).
- Use R0's per-construction metrics as intended: a profile before and
  after each repair, the top thrashing constructions named, so the next
  constant-factor work is targeted (memo/family/scan hot spots).
- R8 token-indexed chart: evaluate on the R0 profile; take it only if the
  profile says columns dominate.
- RSS half of the diagnostics sub-item: 282 MB at matched workers is
  unchanged; deliver the report/RSS reduction or record why not.
- Clean-ups: `parents_by_child` populated but never read; "and N more"
  counts raw entries; `inspect` prints no performance line; roundtrip
  --json `"message": null` rows disclosed or reverted.
Zero semantic change (per-unit identity of status/render/ownership, as
the round-1 review proved); timings state worker count and host load.
Standard constraints apply.
