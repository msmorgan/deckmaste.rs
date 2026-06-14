---
needs: [card-text-render]
---
Bridge a live engine object to `deckmaste_cards::render::CardView` so the TUI detail
pane can render *derived* characteristics (tokens, pumped, animated, control-changed),
not just printed faces. The renderer is engine-free by design (`cards` must not depend
on `deckmaste_engine`), so this ~5-line adapter lives on the engine/TUI side of the
seam: read `engine::LayeredView::get(id)` (the derived `Characteristics`) plus the
object's name/mana, assemble a `CardView` of `core` types, and call `render`. The
renderer already proves this path with synthesized `CardView`s; this wires it to real
game state. Feeds tui-board-view's detail pane; part of tui-client.
