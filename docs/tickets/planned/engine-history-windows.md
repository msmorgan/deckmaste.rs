---
needs: []
design: true
---
Sub-turn history-lookback windows: the remaining `ThisCombat` and `ThisStep` arms.
Rewritten 2026-10-21 (`docs/evidence/mtg-rules-review-2026-10-21/departures-docs.md`
#43): the earlier text quoted echo wrongly and proposed an anchor that would never
fire.

## Landed: `SinceYour(step)` (echo)

Echo [CR#702.30a]: "At the beginning of your upkeep, if this permanent came under
your control since the beginning of your last upkeep, sacrifice it unless you pay
[cost]." `plugins/builtin/macros/keyword/Echo.ron` uses
`within: SinceYour(Beginning(Upkeep))`, and the scan is implemented in
`crates/deckmaste_engine/src/history.rs` (`in_window_for`,
`position_in_window_for`), watcher-relative: the anchor is the watcher's most recent
`StepBegan(step)` onset, except while that step is itself current on the watcher's
turn, where the anchor is the PREVIOUS onset (the current onset is the one firing the
trigger, so anchoring on it would never match; Paradox Haze ruling 2006-09-25 on
"last upkeep"). With no earlier onset the window runs from game start. Entries
strictly after the anchor are in the window.

## Remaining

`eval.rs` `window_contains` (around `:758-768`) still `unreachable!`s for
`ThisCombat | ThisStep | SinceYour(_)`: `SinceYour` never reaches it (history.rs
handles it first), and `ThisCombat`/`ThisStep` have no consumer yet. Implement those
two arms on the same anchor machinery (the current combat phase's / step's onset by
log position) when the first card needs them, and make the `SinceYour` arm's
unreachability a typed impossibility rather than a panic. The design question is only
how `ThisCombat` and `ThisStep` anchor when read outside that combat or step (for
example from a delayed trigger), which needs a ruling per consumer.
