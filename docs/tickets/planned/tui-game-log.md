---
needs: [tui-polish]
---
A game-log / history pane for the TUI hotseat client: a scrolling record of what
has happened so far (spells/abilities put on the stack and resolved, attacks and
blocks, combat damage, deaths, life changes, draws, turn/phase transitions) so a
player can see the sequence of events — especially the ones that resolve between
their decisions. Lives in the right-hand column, which splits into Detail (top)
and Log (bottom); show the most recent entries, newest at the bottom. Part of
tui-client; follows the `tui-polish` UX pass.

There is a design choice to settle first (pick one before implementing):

- **UI action-log** — append a human line each time a decision is submitted in
  the client (cast/play/activate, declare attackers, declare blocks, chosen
  targets), formatted from the same `describe_action` / decision data the footer
  and ability popup already use. Self-contained, no engine change. Limitation:
  it only sees decisions the human drives; it does NOT capture events the engine
  auto-resolves between decisions (triggers firing, combat damage, state-based
  deaths, draws), so the log is a record of *plays*, not of everything that
  happened.

- **Engine-history-backed** — render `GameState.history`, the engine's
  append-only event log (it already records the full `GameEvent` stream: turn
  began, step began, spell cast, zone changes/deaths, tapped, mana, tokens,
  etc.). Far more complete, but needs two things: (1) a public read accessor on
  `History` (today its `Vec` is private and `scan` is `pub(crate)` — add e.g. an
  `entries()`/iterator), and (2) curation + human formatting of `GameEvent`
  variants, filtering the low-level intent/bookkeeping events (`WillDraw`,
  `WillDestroy`, `Untapped`, `StepBegan`, per-unit `ManaAdded`, …) that would
  otherwise spam the log down to the player-meaningful ones.

Recommendation: the engine-history approach is the "real" log and the better
end state; the read accessor is a small, legitimate addition (the UI genuinely
needs to read history) and the work is mostly the event-curation/formatting
judgment. The UI action-log is a cheaper fallback if a self-contained change is
preferred.

Stretch, not blocking a playable demo — the client is already fully playable
without it.
