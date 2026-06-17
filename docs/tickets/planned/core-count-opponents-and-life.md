---
needs: []
---
Opponent-count and player-life-total `Count` primitives. `deckmaste_core::Count`
has no variant for "number of opponents you have" or "a player's life total", so
board-state conditions over them decline.

Add the two `Count` variants (+ their `eval_count` arms). The
parse-enters-tapped worker's `parse_board_condition` plumbing already handles the
surrounding grammar — these unlock its deferred forms:
- `~ enters tapped unless you have two or more opponents.` (~10)
- `~ enters tapped unless a player has N or less life.` (~10)
and the broader "you have N+ opponents" / "a player has N life" condition family.
Flagged by the parse-enters-tapped worker (`declines_unbuilt_count_conditions`).
