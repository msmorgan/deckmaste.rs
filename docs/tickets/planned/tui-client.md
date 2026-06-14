---
needs: []
---
Epic. An interactive ratatui hotseat client — the project's primary binary — that
plays a full game (Goblins vs Elves — decklists committed, the cards themselves
generated locally from MTGJSON rather than canonized) by answering the engine's OWN
decisions through its public
`step()`/`submit_decision()` API. One human drives both seats; the active
perspective auto-follows whoever the engine is asking to decide. Side-by-side
battlefields (public), a middle pane showing the selected object's rendered text,
each player's hand revealed only on their own perspective; arrow keys move the
object selection, spacebar passes. Every offered choice is built from the engine's
enumerated legal actions — nothing hand-rolled — and no-choice decisions
auto-resolve. This is the README "see it work" piece and the engine's first real
external consumer.

Decomposed into — UI: `tui-crate-scaffold`, `tui-board-view`,
`tui-decision-actions`, `tui-shortcuts`, `tui-polish`; engine/render:
`engine-decision-enumeration`, `card-text-render`; cards: `demo-goblins-elves` and
its generator tasks `gen-token-effect`, `gen-dynamic-count`, `gen-sacrifice-cost`.
Lives as `deckmaste_tui` for now; may move to a dedicated crate family once the
engine has other consumers.
