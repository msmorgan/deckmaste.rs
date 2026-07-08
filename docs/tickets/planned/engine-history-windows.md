---
needs: []
design: true
---
Phase/step-anchored history-lookback windows. Several intervening-if / history
conditions need finer windows than turn/game granularity — e.g. Echo [CR#702.30a]
("if this came under your control since the beginning of your most recent
upkeep, …"), whose kw-echo macro currently uses `ThisTurn` as a lossy
approximation (graduation is fine; engine EXECUTION under-counts a permanent that
arrived during a prior turn's later steps).

## Needs a design pass — ticket premise has drifted from the code

The original text below described `deckmaste_core::Window` with a `todo!` scan
seam. That area was since refactored and the mechanical framing no longer holds:

- **No `Window` enum.** The type is now `deckmaste_core::Lookback`
  (`crates/deckmaste_core/src/temporal.rs`).
- **The upkeep variant already exists**, in a more general parameterized form:
  `Lookback::SinceYour(PhaseStep)`. "Since your last upkeep" is
  `Lookback::SinceYour(PhaseStep::Beginning(BeginningStep::Upkeep))`. It already
  round-trips in RON and has a passing test. So "add the variant" is effectively
  a no-op; only the ticket's flat `SinceYourLastUpkeep` spelling is obsolete.
- **The scan side does not `todo!` — it `unreachable!`s** for the whole sub-turn
  family (`ThisCombat | ThisStep | SinceYour(_)`) in
  `crates/deckmaste_engine/src/eval.rs` (`window_contains`).

The scan arm is the real work, and it is a **design decision**, not a fill.
`History::scan`/`in_window`/`window_contains` have a turn-granularity,
reader-agnostic signature (`(within, current_turn)`) at ~15 call sites. A faithful
`SinceYour(Beginning(Upkeep))` needs two things that signature can't express:
1. **The reader's identity** ("*your* upkeep") — available in the `frame` at the
   `Happened` call site but deliberately not threaded into the scan predicates.
2. **A step-onset anchor by log position, not turn** — the log records
   `GameEvent::StepBegan(PhaseStep)` with the active player, so the anchor is
   derivable, but only by scanning history for the most recent `StepBegan(upkeep)`
   on the reader's turn and including entries at `seq >= anchor`. `window_contains`
   is a per-entry pure predicate with no view of the log or the reader.

Implementing means choosing an architecture: thread the reader (or a pre-resolved
anchor seq) through `scan`/`in_window` and refactor `window_contains`, then define
the anchor semantics (seq-based onset lookup, whose-turn matching on the recorded
`StepBegan` view). Cross-cutting; needs a design pass before it's claimable as
mechanical work.

## Scope (post-design)
Implement the `SinceYour(step)` scan arm per the approved architecture, then
repoint `plugins/builtin/macros/keyword/Echo.ron` from `within: ThisTurn` to
`SinceYour(PhaseStep::Beginning(BeginningStep::Upkeep))` (see the seam comment in
that macro). Flagged by the kw-echo worker; drift surfaced by the batch executor.
