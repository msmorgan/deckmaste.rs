---
needs: [core-eventfilter-master-forms]
---
**The compile-down matcher bridge: `EventFilter` runs on today's engine
matchers, with load-time expressiveness caps.** Sequencing stance (a deliberate
ruling, not drift): the event grammar does NOT wait for the one-evaluator
engine rebase — it lands on this bridge first, and the rebase
([[engine-one-evaluator]]) is scheduled independently.

## Scope

- **Lowering:** compile each master-form `EventFilter` (via its field-atom
  normal form over the fact record `{kind, object, patient, actor, source,
  from, to, cause, amount, counter, batch, before, after, time}`) down to the
  engine's existing matcher forms — the live trigger matcher, the
  LKI-snapshot matcher, and the history scan — so trigger scanning,
  replacements, delayed triggers, and `Happened`/event-count reads all consume
  the new grammar without an engine rewrite.
- **Expressiveness caps at load:** constructs the current matchers cannot
  faithfully evaluate are REJECTED at load (a capped-construct load error with
  its own code + reject fixture), never silently mis-matched: `Not`, `Nth`,
  and `Where`-under-snapshot are capped until [[engine-one-evaluator]] lifts
  them. The cap list is data (an emitted per-atom lane-support table), not
  scattered `if`s.
- **Entailment matching:** wire the CR-cited entailment table (landed in
  [[core-eventfilter-master-forms]]) into cause matching — a `ZoneChange`
  pattern with `cause: (verb: Sacrifice)` matches the facts the engine
  already records with sacrifice causes (crates/deckmaste_engine/src/trigger.rs
  stores cause-verbs on `ZoneChanged`), and a plain battlefield→graveyard
  `ZoneChange` pattern matches sacrifices by entailment [CR#701.21a] — "dies"
  triggers see sacrifices, and graveyard replacements intercept them, with no
  per-verb engine arm.
- **Deletion:** the engine's `Performed` string-verb matching arms — including
  the LKI-snapshot matcher's hard-false and `todo!()` arms for it — are
  deleted; grammar-side deletion already happened in
  [[core-eventfilter-master-forms]].
- Lane behavior follows the emitted lane table: `Within` evaluated only in
  history lanes; `OneOrMore` matches a batch Occurrence once [CR#603.3b];
  replacement lanes evaluate would-facts [CR#614]; delayed events fire once
  with targets dropped [CR#603.7c].

## Done

- All existing trigger/replacement/history tests pass with events expressed
  as `EventFilter` end to end; no `Performed` reader remains in the engine.
- Capped constructs fail at load with their code; one fixture per cap.
- An entailment test: a "dies" trigger fires on a sacrifice; a
  graveyard-replacement intercepts a sacrifice — both without verb-specific
  engine code.

## Verification

- `cargo test -p deckmaste_engine` and `cargo test --workspace` green.
- `rg 'Performed' crates/deckmaste_engine crates/deckmaste_core` — empty.
- `cargo xtask validate` clean on hand-authored plugins.
- `cargo xtask cite check` — 0 stale, `--list-noncompliant` empty.
