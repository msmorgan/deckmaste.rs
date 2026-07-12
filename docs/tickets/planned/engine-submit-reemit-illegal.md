---
needs: []
---
Direction (user, 2026-07-12): an illegal `submit_decision` should re-emit the
same decision rather than being a caller-fatal event. Known tension (user):
naive re-emit lets a deterministic buggy driver (esp. a test) infinite-loop by
resubmitting the same illegal answer.

Today this is ~90% true mechanically: on `Err(DecisionError::Illegal)` the
pending stays set and the next `step()` surfaces the same `NeedsDecision`.
What's missing is observability + consumer discipline.

Recommended shape (design conversation before building):
- Keep the `Err(Illegal)` return (immediate feedback), pending stays — and the
  surfaced pending carries `attempts: u32` + the last rejection reason as DATA.
  No retry-cap policy in core.
- Loop-breaking is RUNNER policy: the test harness's mechanical driver panics
  on `attempts > 0` (buggy tests fail loud, never hang); `sim::play` answers a
  re-emitted decision with the guaranteed-legal mechanical fallback (empty
  blocks, pass priority, first-N discard) instead of re-asking the strategy —
  self-heals in one extra step (and replaces its current
  `.expect("a strategy submits only legal decisions")`); the TUI re-prompts the
  human showing the reason.
- Requires a guaranteed-legal fallback answer per decision kind (mostly exists
  as the mechanical defaults; audit for gaps).
- Composes with engine-block-legality-query: the probe prevents most illegal
  proposals; re-emit-with-attempts handles the residue.

Open questions: where `attempts`/reason live (fields on each PendingDecision
variant vs a wrapper alongside `pending`); whether Err+re-emit dual channel is
right or Err should be dropped once all consumers migrate.
