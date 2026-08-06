---
needs: []
---
Lean the engine's runtime **event** types onto `Arc` the way the semantics
grammar now leans on `Arc<[T]>` — so the large runtime enums stop tripping
`clippy::large_enum_variant` on their own merits instead of via `#[expect]`.

## Why this exists (2026-07-19)

The grammar `Vec<T> → Arc<[T]>` refactor (change `arc-engine-event-types`'s
parent work) shrank the small variants of several engine runtime enums, which
*widened* the size gap against the one variant that still holds a `GameEvent`
(~288 bytes) inline. Five enums now trip `large_enum_variant` and are suppressed
with `#[expect(clippy::large_enum_variant, reason = "… arc-engine-event-types …")]`:

- `Occurrence::Single(GameEvent)` — `crates/deckmaste_engine/src/event.rs`
- `Progress::Applied(Occurrence)` — `crates/deckmaste_engine/src/step/mod.rs`
- `StepOutcome` (carries `Progress`) — `crates/deckmaste_engine/src/step/mod.rs`
- `ReplaceOutcome::Pass(GameEvent)` — `crates/deckmaste_engine/src/replace_registry.rs`
- `ResumeOutcome::Fact(GameEvent)` — `crates/deckmaste_engine/src/replace_registry.rs`

These join the pre-existing suppressions on `WorkItem` (agenda.rs, same hot-path
rationale) and `Card`/`TodoCard` (a `CardFace`-per-variant leaf; core `card.rs`
and migrations `todo_card.rs`) — all candidates for the same treatment.

The goal is to remove those suppressions by making `GameEvent` (and `CardFace`)
cheap to move around, consistent with the "shared pointers are fine, clones are
cheap" direction.

## The design wall that deferred it

Wrapping `GameEvent` in `Arc` at the variant (`Single(Arc<GameEvent>)`, etc.) was
attempted and reverted. Two shapes were tried and both sprawled far past a
CI-hygiene fix:

1. **`Occurrence::Single(Arc<GameEvent>)`, `Batch(Vec<GameEvent>)`** — the
   `Single`/`Batch` asymmetry breaks every site that flattens an `Occurrence`
   into a uniform `Vec<GameEvent>` (Single yields `Arc<GameEvent>`, Batch yields
   `GameEvent`). ~68 consumer sites, plus `Arc` cannot be **pattern-matched
   through** — `if let …Single(GameEvent::TriggerFired(TriggerFired { .. }))` must
   become bind-then-`match ev.as_ref()`/`let-else`.
2. **`Batch(Vec<Arc<GameEvent>>)` too** (uniform element type) — pushes
   `Arc<GameEvent>` into the whole event-**production** pipeline (combat, mana,
   cast, choice, every handler that builds events for a batch), ~118 sites and
   unbounded (return-type changes cascade to callers).

The engine both **produces** and **consumes** events, so Arc-ing the event type
splits that world; it needs a deliberate, whole-pipeline decision (uniform
`Arc<GameEvent>` representation end-to-end, vs. a `Box`/`Arc` only at the enum
boundary with `as_ref()`-based matching), not an incremental hack. `Card` has the
same "can't pattern-match through `Arc`" problem for `Card::Normal(CardFace)` /
`TwoFaced { front, back }`.

## Acceptance

Remove the five `#[expect(clippy::large_enum_variant)]` above (and ideally
`WorkItem`/`Card`/`TodoCard`) by shrinking the offending variants via `Arc`,
with `cargo clippy --workspace --all-targets -- -D warnings` clean and the full
suite green. Decide and document the uniform event representation first.
