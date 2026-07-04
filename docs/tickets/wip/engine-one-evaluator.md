---
needs: [engine-eventfilter-bridge]
---
**The one-evaluator rebase: replace the three divergent event matchers with a
single lane-parameterized `eval`.** Deferred-but-designed: schedulable any
time after the compile-down bridge ([[engine-eventfilter-bridge]]) lands; the
bridge's load-time expressiveness caps exist precisely so this rebase is never
on the critical path of a grammar ticket.

Today the engine matches events three ways — the live-fact trigger matcher,
the LKI-snapshot matcher (with hard-false and `todo!()` arms), and the history
scan — and they disagree at the edges. Target signature:

```rust
fn eval(pred: &LoweredEventFilter, fact: &FactView, lane: Lane,
        bindings: &Bindings) -> bool
enum Lane { Trigger, Replacement, Delayed, History, Snapshot }
```

- `FactView` = the one fact record `{kind, object, patient, actor, source,
  from, to, cause, amount, counter, batch, before, after, time}`.
- Object-valued atoms evaluate their embedded `Filter` against a
  `CandidateView = Live(board) | Lki(snapshot)`, chosen by the lane row —
  snapshot semantics become a parameter, not a second matcher.
- `Used` resolves object identity through `bindings` — object-scoped
  identity, an object that changes zones is a new object [CR#400.7].
- Per-atom lane support is EMITTED DATA (the lane table from
  `data/grammar-tables/`), not code: `Within` only in history lanes;
  `OneOrMore` batch semantics in trigger/replacement lanes [CR#603.3b];
  replacement lanes evaluate would-facts [CR#614]; delayed lanes fire once
  against the elaborated expected zone [CR#603.7c].

## Consequences

- **Lift the load caps:** `Not`, `Nth`, and `Where`-under-snapshot become
  loadable (delete the cap rows + their load errors; the reject fixtures flip
  to acceptance fixtures — Notion Thief / Erayo-style nth-event and negated
  patterns are the acceptance cards).
- **Delete the bridge:** the compile-down path and the LKI matcher's
  hard-false/`todo!()` arms go away; one evaluator, one semantics.
- Every existing trigger/replacement/delayed/history test must pass unchanged
  — this is a rebase, not a behavior change, except where a `todo!()` arm
  becomes real behavior (each such case gets a pinning test).

## Done

- `eval` + `Lane` land; the three matchers are gone; lane table consumed as
  data.
- Caps lifted with acceptance fixtures for the previously-capped constructs.
- No `todo!()` in the engine's event-matching paths.

## Verification

- `cargo test -p deckmaste_engine` and `cargo test --workspace` green.
- `rg 'todo!' crates/deckmaste_engine/src/` — no event-matching hits.
- `cargo xtask validate` clean (capped-construct fixtures now load).
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
