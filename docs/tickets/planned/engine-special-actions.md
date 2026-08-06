---
needs: []
---
**Engine: special actions ([CR#116.2]) are shaped but never offered.**

`Action::Special` exists in the action taxonomy, but `legal_actions` never
enumerates it, so two sites are unreachable shells:

- `crates/deckmaste_engine/src/decide/mod.rs` — `take_priority_action` has no
  `Action::Special` arm.
- `crates/deckmaste_engine/src/render.rs` — no render shape for it.

[CR#116.1]: a special action is one a player may take *when they have
priority*, and it does not use the stack — so it never becomes an object other
effects can respond to. [CR#116.2] lists the twelve. Most have their own
prerequisites (playing a land, turning a face-down creature face up, suspend's
exile, companion, etc.), and several are owned by other tickets
(`engine-face-down`, `engine-exile-command`).

Scope this ticket to the *dispatch spine*: decide which special actions the v1
corpus needs, enumerate them from `legal_actions`, and give each a render
shape. Per-mechanic behavior stays with the mechanic's own ticket.

Effort: **M**.
