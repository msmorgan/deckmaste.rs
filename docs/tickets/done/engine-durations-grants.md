---
needs: []
design: true
---
Close the two P0.W1 seams in `resolve.rs`'s `OneShotEffect::Continuously` arm
(the one-shot that mints a continuous-effect instance, [CR#611.2c]).

**Durations** (`resolve.rs` `todo!("P0.W1: duration …")`): only
`FixedUntil(EndOfTurn)` (cleanup sweep, [CR#514.2]) and `EndOfGame` may mint
instances today — any other duration would silently last forever, so the seam
is loud. Unbuilt sweeps/tracking: `FixedUntil(EndOfCombat)` ([CR#511.2]),
`FixedUntil(YourNextTurn)` ("until your next turn"), `UntilEvent(EventFilter)`
(engine pairs the undo one-shot, [CR#610.3]), `ForAsLongAs(Condition)` (tracked
predicate with the never-started / once-stopped-never-resumes `started` latch,
[CR#611.2b]), `ForThisEvent` (instruction-scoped rider, e.g. DestroyNoRegen).

**Grants** (`todo!("P0.W1: Continuously({…}) — non-Modify grants unbuilt")`):
only `Modify(reference, change)` (locked scope, [CR#613.6]) and
`Each(SelectAll(f), Modify(It, change))` (floating anthem/distributor shape)
are wired. A granted `Deontic` row ("target creature can't block this turn"),
`CostModifier` row, or `Each` over a non-`SelectAll` selection would be
silently inert — loud instead. Building grants means the minted instance can
carry non-Modification static rows and the legality/cost readers must see them
(the derived-view side: `legal.rs` deontic evaluation is its own ticket,
core-casting-restrictions / engine-combat-requirements — this ticket covers
instance creation + scope/duration bookkeeping so those rows EXIST in the
view).

Both seams share the `ContinuousEffect` instance record (timestamp, scope,
duration, controller) — build together. Sim strategy / decide arms are
unaffected (no new decision kinds).

---

## Spec (design settled 2026-07-12)

Rulings: rider-in-occurrence for `ForThisEvent` (no Idris variant added);
grant consumers wired this ticket = event-cants + `CostModifier` (Prevention
stays loud — engine-prevention owns shields/windows [CR#615.1]); `ForAsLongAs`
context = controller + source captured at mint; row storage = parallel `rows`
field, NOT derived-view injection (a resolved one-shot's restriction is not an
ability of the object — later ability-removal must not strip it (restriction
effects modify game rules, outside the characteristic layers, [CR#613.11]);
printed rows stay layer-6-sensitive, instance rows immune).

### 1. Durations

`ContinuousEffect` gains `origin: Option<Box<Frame>>` (populated for
`UntilEvent`/`ForAsLongAs`; `None` otherwise — anchors `This`/`You` in
filter/condition eval). `ReplacementInstance` (shields) shares every sweep
below; `create_shield` (resolve.rs:317) currently accepts ANY duration
unguarded — a latent silent-forever bug. Extract ONE canonical
sweepable-duration guard used by both mint sites.

- **`FixedUntil(EndOfCombat)`**: new `expire_end_of_combat` on `GameState`
  (instances + shields), called from `end_of_combat` (step.rs:2332, beside
  `combat.clear()`). Cleanup also sweeps EndOfCombat as a catch-all (a turn
  whose combat is skipped must not leak the effect past the turn — comment
  this rationale).
- **`FixedUntil(YourNextTurn)`**: sweep in `begin_turn` (step.rs:1495) after
  `active_player` advances: remove where `controller == new active player`
  ("your next turn" = next turn the controller actually takes — skipped turns
  handled naturally). Shields key on their source's controller; if that needs
  a new field, add it at mint.
- **`UntilEvent(EventFilter)`**: re-checked per applied fact beside
  `scan_triggers` (step.rs:1284): eval the filter via `GameState::eval` with
  bindings anchored on `origin` (watcher pattern mirrors
  `event_matches_delayed`, trigger.rs:185). Match → remove (post-apply check =
  effect ends once the event has happened, [CR#610.3]). Lane: reuse
  `Lane::Delayed` unless an eval arm objects; comment the choice.
- **`ForAsLongAs(Condition)`**: mint-time `condition_holds` check — false →
  never starts, no instance pushed [CR#611.2b]. Re-check at the same
  post-occurrence point + step transitions; false → REMOVE (removal is the
  once-stopped-never-resumes latch — no stored bool needed). Do NOT eval
  conditions inside `layer::gather` — `condition_holds` can recurse into
  `layers()` (comment this hazard). `condition_holds` needs a `Frame`: rebuild
  from `origin`.
- **`ForThisEvent`**: NEVER mints an instance. `Sequentially` lowering
  (resolve.rs:524) pre-scans children: an `Until(ForThisEvent, parts)` child
  folds `parts` as RIDERS onto the immediately-preceding sibling's `RunEffect`
  work item; riders travel with that child's emitted occurrences and are
  consulted only for those events. `Cant(Regenerate(ref))` rider → the
  replacement loop skips regeneration shields for this occurrence's destroys
  ([CR#701.19c] — shields are not applied, not consumed). Other rider kinds:
  loud per-kind. A rider with no preceding sibling is an authoring mistake →
  fizzle (drop, never panic). Scheduling fact forcing this shape: `Destroy`'s
  front-scheduled `Emit` applies BEFORE the next `RunEffect` child runs, so a
  sibling-minted instance would arrive too late.

### 2. Grants

`ContinuousEffect` gains `rows: Vec<StaticEffect>` (empty default; `changes`
stays the hot layer-pass path — `layer::gather` unchanged). Mint arm routing
at resolve.rs:602-621:

- **`Deontic`**: mint as row. Subject `Reference`s resolve at mint
  ([CR#611.2c] object lock): `scope = Locked(ids)`, subject rewritten to `It`;
  consumers interpret `It` = scope members. Evaluation stays LOUD at
  legal.rs:86 — core-casting-restrictions / engine-combat-requirements own it;
  this ticket only makes the rows exist.
- **`CostModifier { of, change }`**: mint as row (self-filtered — `of` is a
  spell predicate; scope unused). WIRE: `cost_modifier_rows` (cast.rs:1647)
  additionally scans `state.continuous` rows, same phase dispatch.
- **`CantHappen`**: mint as row (self-filtered). WIRE: `cant_event`
  (replace_registry.rs:125) scans instance rows beside battlefield statics.
- **`Prevention`**: LOUD at mint, message pointing at engine-prevention
  (PreventNext/PreventAll macros stay blocked until that ticket).
- Everything else (`TriggerMultiplier`, `AsThough`, `Sba`, `OutcomeGate`, …):
  per-kind `todo!` naming the kind — narrower than today's catch-all.
- `Each(SelectAll, non-Modify inner)`: unchanged (still loud).

### 3. Idris (soundness gate)

Add `UntilEndOfCombat` / `UntilYourNextTurn` to `Duration` (Core.idr:1815).
NO `ForThisEvent` — the rider is engine-level scheduling, and Idris already
models no-regen via cants. Idris check is slow: run it foregrounded once, or
skip and say so.

### 4. Tests

- `tests/layers.rs` (pattern: `one_shot_pump_expires_at_cleanup`, :164):
  EndOfCombat sweeps at end of combat AND at cleanup when combat skipped;
  YourNextTurn survives opponent's turn, expires at controller's turn begin;
  UntilEvent removed on matching event, survives non-matching; ForAsLongAs
  never-starts when false at mint, ends once false and never resumes.
- resolve.rs unit tests (pattern: `continuously_of_this_registers_locked_scope`,
  :6594): rows minted for Deontic (locked subject) / CostModifier / CantHappen;
  Prevention still panics; Sequentially rider fold pairs with preceding child.
- End-to-end rider: destroy with live regen shield + DestroyNoRegen → object
  stays destroyed, shield NOT consumed. Canon card Do or Die via cards suite.
- CostModifier row: cast cost reflects one-shot increase, expires at cleanup.
- Shield duration guard: non-sweepable duration on `create_shield` now loud.

### 5. Gates

nightly fmt, clippy, engine suite, cards suite, `cargo xtask cite check` +
`bless` for any new rule + `audit --diff` read-back. No core RON-surface
change → no wizards regen. No new decision kinds → sim/decide untouched.
