---
needs: []
---
Measured with `-Zprint-type-sizes` (2026-07-16): one fat variant inflates the
whole event pipeline. `TriggerBindings` is **280 bytes** (three 80-byte
last-known-information fields: `this`, `that_object`, `that_patient`), and it
sits *unboxed* in `GameEvent::TriggerFired` — making that variant **304 B**
against a 148 B runner-up (`TokenCreated`). Enum size = largest variant, so the
bloat cascades: `GameEvent`, `Occurrence`, `Progress`, `StepOutcome`,
`ReplaceOutcome`, and `ResumeOutcome` are **all 304 bytes**, moved by value
through the hot `step()` loop on every step.

The four engine `#[expect(clippy::large_enum_variant)]` sites (on
`StepOutcome`, `Progress`, `ReplaceOutcome`, `ResumeOutcome`) each argue
"boxing would allocate on every step / on the common path" — true against
clippy's naive suggestion (box the whole payload), but silent on the root fix:

## 1. `Box<TriggerBindings>` in `GameEvent::TriggerFired`

Precedent is one field over: the sibling `created:
Option<Box<deckmaste_core::TriggeredAbility>>` is already boxed. Boxing
`bindings` allocates only when a trigger actually fires (not per step) and
should drop `GameEvent` to ~152 B, halving every by-value event move in the
pipeline.

Self-verifying success criterion: the four `#[expect]`s above go
`unfulfilled_lint_expectations` (variant gap falls under clippy's 200 B
threshold) and clippy itself forces their removal. If any still fires,
`-Zprint-type-sizes` again and chase the remaining spread — do not re-suppress.

## 2. Box `ApplicableEffect` payloads (`replace_registry.rs`)

`ApplicableEffect` is **896 B** (`Replacement` 896 / `Prevention` 632), and its
expect's "not stored in bulk" claim is shaky: `gather_applicable` builds a
`Vec<Applicable>` (~920 B/element) **per event**, deep-cloning replacement ASTs
into it. Box the `Replacement`/`Prevention` payloads: the clone already
allocates internally, so one more box is noise, and `Applicable` drops to
~40 B. Remove or update the three `large_enum_variant` expects in that file.

## 3. Investigate (stretch): core authored-AST enums

- `Ability::Triggered` = **1352 B** vs `Spell` 592 (dominated by
  `TriggeredAbility.effect` 568 + `.condition` 328 + `.event` 320).
- `OneShotEffect::Distribute` = 568 B vs `Delayed`/`Reflexive` 16.

These are parse-once authored data, so boxing costs an allocation only at load
time and shrinks every `Vec<Ability>`/`Vec<OneShotEffect>` element. Constraint:
serde treats `Box<T>` transparently, so the RON card surface must come out
byte-identical — prove it with the cards suite + wizards corpus (no regen
needed if truly transparent; any RON diff = stop and rethink). The bare
`#[allow(clippy::large_enum_variant)]` on `Ability` and the "balanced AST leaf"
comment on `OneShotEffect` get resolved (removed or re-reasoned with real
numbers) here either way.

## Guard rails

- Perf is motivation-adjacent, not the goal (priority ordering puts perf last)
  — the goal is making the suppressions' premises true or the suppressions
  gone. Keep scope to the boxing; no wider event-model changes.
- Sanity-run `examples/full_game_1k` before/after (expect neutral-to-small-win
  from halved moves; any regression = investigate before integrating).
- Full suite once, per house rule.
