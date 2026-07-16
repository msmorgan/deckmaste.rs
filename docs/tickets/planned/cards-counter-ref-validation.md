---
needs: []
---
`Plugin::counters` (`crates/deckmaste_cards/src/plugin.rs`) long documented a
post-load `validate_counter_refs` pass checking every authored `CounterRef`
against the counter registry — but no such pass ever existed (`validate.rs`
lints subtypes/types/keywords only; the doc comment was corrected 2026-07-16).
Today an unknown counter ref surfaces only at Idris-emit time
(`idris_emit.rs`'s counter gap), which runs locally, not at load. Implement
the load-time validation pass so a bad `CounterRef` is caught at plugin load
with a real diagnostic, then point the `counters` doc comment back at it.
