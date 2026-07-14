---
needs: [engine-replacements]
design: true
---
## Needs a design pass (or a won't-do call) — investigated 2026-07-08

The functional work here has LANDED (binding model shipped; residual item 1
migrated). The ONLY remaining item (residual item 2 below) is explicitly framed
as "architectural elegance, not a functional blocker," and it is NOT a mechanical
edit — it reshapes a core type and the engine's shield capture/matching key.

`CreateReplacement.subject` is a load-bearing engine matching key, coupled across
six layers: the core field (`crates/deckmaste_core/src/action.rs`), shield
creation freezing it (`resolve/effect.rs::create_shield`), `floating_watches` matching
events BY frozen subject identity (`replace_registry.rs` — whose doc-comment notes
the `would` clause's `Ref(EventObject)` filter can't be evaluated by a frameless
gather, which is exactly why the captured field exists), emit/render
(`idris_emit.rs`, `render/effect.rs`), and subject-identity tests
(`builtin.rs`, `replace_registry.rs`). Routing capture through `With`/`That`
forces unspecified design answers: how a persistent shield FREEZES the
frame-bound `That` (the resolution frame is gone when the shield later fires);
how `floating_watches` derives its matching key without `subject`; whether the
`would` predicate becomes `Ref(That)` and what that does to identity matching;
and the resulting Idris emission shape. No `With`-wrapped-shield precedent exists
to mirror. Recommend either closing this as optional/won't-do, or routing the
shield-capture reshape through a design pass before implementation. Surfaced by
the batch executor.

## Status: binding model LANDED (Scry/Surveil/Fateseal work)

The core binding model has been **built and shipped** as of the look-and-distribute / scry-surveil-fateseal integration.

### What landed

**`Effect::With { selection, body }`** — resolves the whole `selection` as a
PLURAL anaphor **`Those`** (bound in the resolution frame), runs `body` once,
does NOT distribute. When plural effects need to act on each element, use
`Each` or `ForEach` as described below.

**`Selection::Those`** — reads the bound plural group (order-preserved). Set up
by `With`'s binding; stable reference to the whole group.

**`Each` / `ForEach` — RETAINED** — explicit per-element distributor binding
the singular **`That`** (`ThatObject`/`ThatPlayer`). They were **not** retired.
`Each(Filter(Creature), effect)` distributes the effect over matching creatures
one at a time, binding the current element to `That` each iteration.

**`This` never rebinds** — still true, unchanged — always denotes the ability's
source.

**Unified model:** `With` → `Those` (whole ordered group, once); `Each`/`ForEach`
→ `That` (one element at a time, singular). Group-vs-element is explicit at the
effect level.

Example:

```rust
With(
    selection: Choose(2, And([ControlledBy(You), Creature])),
    // `Those` is bound here to the chosen 2 creatures, order-preserved.
    body: Each(Those,
        // per-element distribution binds each creature to singular `That`:
        Targeted(targets: [Player], effect: DealDamage(That, Target(0))),
    ),
)
// NOTE: `Each`/`ForEach` over a bound group is the residual-cleanup form
// (item 1 below). Do NOT confuse it with `PlayerAction::Distribute` — the
// already-built scry/surveil/fateseal partition action, which is unrelated.
```

### Residual scope

1. **(DONE) Migrate existing `Selection::Each` / `Effect::ForEach` call sites to explicit `Each`-over-selection form** — completed.

2. **(Unfinished) Re-express regeneration-shield `That`-capture through `With`**
   — `engine-replacements` added `CreateReplacement.subject` to capture the
   shield's target as a bespoke field. Rework `Regenerate` (and other
   "the next time …" shields) to wrap their body in a `With` binding, so the
   capture rides the standard model (`That` inside the shield's effect body).
   This is architectural elegance, not a functional blocker.
