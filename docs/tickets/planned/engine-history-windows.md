---
needs: []
---
Phase/step-anchored history-lookback Windows. `deckmaste_core::Window` is a closed
vocabulary whose only history-lookback variants are `ThisTurn` and `ThisGame`
(`History::scan` supports exactly those two; anything else hits a `todo!`).
Several intervening-if / history conditions need finer windows.

First need: **`Window::SinceYourLastUpkeep`** (upkeep-anchored lookback) + matching
`History::scan` arm. Echo [CR#702.30a] is "if this came under your control since
the beginning of your most recent upkeep, …" — the kw-echo macro currently uses
`ThisTurn` as a lossy approximation (graduation is fine; engine EXECUTION
under-counts a permanent that arrived during a prior turn's later steps). Likely
also wanted by other upkeep-anchored "since" effects.

Scope: add the window variant(s) + scan support, then repoint
`plugins/builtin/macros/keyword/Echo.ron` from `within: ThisTurn` to the precise
window (see the seam comment in that macro). Flagged by the kw-echo worker.
