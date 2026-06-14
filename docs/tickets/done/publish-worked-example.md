---
needs: []
---
Superseded → decomposed into the `tui-client` epic. The README "see it work" piece is
no longer a single narrated interaction but an interactive ratatui client that plays
Goblins vs Elves by driving the engine's real decision API — a stronger, honest demo
(one human answers every decision for both seats; nothing is hand-rolled). Outcome of
this ticket: that decomposition. See `tui-client` and its children in `planned/` — UI:
tui-crate-scaffold, tui-board-view, tui-decision-actions, tui-shortcuts, tui-polish;
engine/render: engine-decision-enumeration, card-text-render; cards: canon-goblins-elves
with generator tasks gen-token-effect, gen-dynamic-count, gen-sacrifice-cost.
