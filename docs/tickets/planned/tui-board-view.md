---
needs: [tui-crate-scaffold]
---
The ratatui board view and navigation, no game actions yet. Side-by-side
battlefields (both public), a middle pane showing the selected object's rendered text
(via card-text-render; debug-stub until it lands), per-perspective hand, the stack,
and a header with life / turn / phase. Scrollable zones, a visible object selection,
and the active perspective auto-following whoever the engine is asking to decide.
Arrow keys move the selection between objects/zones; spacebar is reserved for pass.
Read-only over `GameState` + `state.layers()`. Part of tui-client.
