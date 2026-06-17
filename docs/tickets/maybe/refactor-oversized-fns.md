---
needs: []
design: true
---
Three functions blow well past the `too_many_lines` threshold (150, set in
`clippy.toml`) not because they are irreducible per-variant dispatch — the
engine's genuine flat dispatchers (`resolve_object`, `run_effect`,
`action_items`, `player_action_items`, `event_matches`) keep a permanent
`#[expect(.., reason = "one arm per ...; splitting would scatter the dispatch")]`
— but because they have grown real internal structure that wants decomposing.
Each currently carries a placeholder `#[expect(clippy::too_many_lines, reason =
"... tracked in refactor-oversized-fns")]`. `#[expect]` is self-clearing: once a
function drops under the threshold its expectation goes unfulfilled and clippy
forces the attribute's removal, so these placeholders cannot silently outlive
the refactor.

- `decide::submit_decision` (~635 lines) — split by decision kind:
  priority / cast-procedure / combat. The outer `match (pending, decision)` is
  the dispatch; each arm's body is the candidate to extract into a per-kind
  handler method.
- `step::apply` (~400 lines) — split by subsystem: stack / zone-change /
  player. (The action-driven zone-change collapse — draw / land / discard →
  `ZoneWillChange` — is already done; that part need not move.)
- `tui::interactive_loop` (~243 lines) — extract the key-dispatch `match` (and
  the popup/navigation handling) out of the event loop into a handler that maps
  a key event to an action, leaving the loop itself short.

`[design]`: the split boundaries above are a starting proposal, not a settled
plan — agree the seams (especially for `submit_decision`) before carving, so the
decomposition doesn't just trade one long function for a scatter of helpers that
are harder to follow than the original dispatch.
