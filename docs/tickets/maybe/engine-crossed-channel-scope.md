---
needs: []
design: true
---
**Engine: `Condition::Crossed` is correct by convention, not by construction.**

`condition.rs` reads `frame.anaphora.crossed`, a before/after pair set by
counter-placement event roles ([CR#714.2b] — "if the number of lore counters
on it was less than N and became at least N"). `Crossed` has exactly one
grammar producer, the `Chapter` saga macro, which always expands to a
triggered ability's intervening-if condition. `core-saga-chapters` (done) says
so outright: chapters are `Crossed`'s only consumer.

**That path is fully wired and works.** `crossed` threads through
`EventRoles`/`TriggerBindings` at both the printed
(`trigger.rs::scan_event`) and delayed gates, and is rechecked at resolution.
Nothing is broken today.

What is missing is enforcement of the exclusivity. `condition_holds` has many
other call sites that build a frame with no `crossed` channel — an activated
ability's "Activate only if" gate in `activate.rs`, the `Sba` statics and
`ForAsLongAs` duration sweeps in `sba.rs` (all via `Frame::bare`), the
derived-condition pass in `layer.rs` (which short-circuits `Crossed => false`
rather than reaching `condition_holds` at all), and `eval.rs`'s
`EventFilter::When` refinement. If a future macro composed `Crossed` into one
of those positions it would panic at runtime with no authoring-time signal.

Decide one of:

1. **Grammar-gate it.** Restrict `Crossed` to the triggered-ability
   `condition:` position at macro-expansion or Idris-check time, making it a
   build-time error elsewhere. Matches the stated design intent literally.
2. **Make it self-sufficient.** Have `Crossed` query history for the firing
   counter-placement fact the way `Condition::Happened` already does, needing
   no frame plumbing anywhere. Higher leverage if `engine-sagas`' per-crossed
   firing ends up evaluating `Crossed` somewhere the trigger gate doesn't
   reach.

Either closes the panic by construction. Filed under `maybe/` because no
grammar produces the failing shape — this is hardening, not coverage.

Effort: **S**. Design input needed on which option.
