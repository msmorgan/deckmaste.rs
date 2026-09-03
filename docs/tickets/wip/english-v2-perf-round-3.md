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
