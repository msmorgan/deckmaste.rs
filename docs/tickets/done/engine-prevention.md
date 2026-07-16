---
needs: []
---
Prevention shields and windows [CR#615.1], including "can't be prevented"
overrides. Covers one-shot prevention ("prevent the next N damage"), ongoing
shields, and the interaction with damage-replacement effects.

Closed by the 2026-07-16 drift review: everything above had already shipped
piecemeal — `Prevention::{PreventNext, PreventNextInstance, PreventAll}`
(`crates/deckmaste_core/src/replacement.rs`), the authored `PreventNext.ron` /
`PreventAll.ron` macros (`plugins/builtin/macros/effect/`), and the
`StaticEffect::CantPrevent` override (`crates/deckmaste_core/src/continuous.rs`),
with coverage in `crates/deckmaste_engine/src/replace_registry.rs`'s prevention
tests. No open scope remained, so the ticket moved straight to done without a
claim.
