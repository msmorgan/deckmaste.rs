---
needs: [engine-replacements]
---
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
    selection: Choose(2, AllOf([ControlledBy(You), Creature])),
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

### Residual scope (optional, unblocked by model)

The following cleanup is no longer model-critical but may still be useful:

1. **(Optional/low-priority) Migrate existing `Selection::Each` / `Effect::ForEach`
   call sites to explicit `Each`-over-selection form** — NOT required (Scry/Surveil/Fateseal
   already shipped using only `With`/`Those`/`Distribute`). The old call sites work
   as-is; this is a code-uniformity improvement only.

2. **(Unfinished) Re-express regeneration-shield `That`-capture through `With`**
   — `engine-replacements` added `CreateReplacement.subject` to capture the
   shield's target as a bespoke field. Ideally, rework `Regenerate` (and other
   "the next time …" shields) to wrap their body in a `With` binding, so the
   capture rides the standard model (`That` inside the shield's effect body).
   This is architectural elegance, not a functional blocker.
