---
needs: []
---
**Engine: a granted continuous prevention row can't be minted.**

`crates/deckmaste_engine/src/resolve/effect.rs` trips on
`Continuously(Prevention(_))` — an effect that *grants* a continuous
prevention effect ([CR#615.1]).

**Stale-comment discrepancy, recorded by `engine-todo-triage`:** the site's
old comment said this was blocked on `engine-prevention`. That ticket closed
2026-07-16, but its closeout covers only the one-shot `PreventNext` /
`PreventAll` / `CantPrevent` primitives that shipped in
`replace_registry.rs`. It never unblocked this arm. The seam was orphaned, not
closed — hence this ticket.

Fix: decide whether a granted continuous prevention row reuses the
`replace_registry` shield representation with a duration attached, or needs a
distinct row kind in the layers/replacement pipeline. Then mint it and test a
card that grants prevention for a duration.

Effort: **M**. Design input needed on the representation question.
