---
needs: []
---
**An Aura's attach-host is read positionally from targets slot 0 via a card-shape
predicate, not from a declared binding.** The engine-side twin of
[[parse-positional-target-reads]]: an implicit convention where a channel should be.

Spell-resolution dispatch (`crates/deckmaste_engine/src/resolve/mod.rs:106-115`):

```rust
enters_attached_self(source) && entry.targets.first().and_then(|slot| slot.first())
```

The enchant-host linkage is "slot 0's single member", gated by the
`enters_attached_self` card-shape predicate, instead of a link declared from the
`AsEnters` enter-status to a specific target slot. It special-cases the one-slot
Aura arity: any targeted permanent spell whose attach-host is not slot 0 (or whose
slot 0 is plural) silently attaches to the wrong object.

## Fix

The `AsEnters` / enter-status data names its host as a declared slot link
(`Reference::Target(n)`), resolved through the normal reference channel — the same
positional-target channel [[parse-positional-target-reads]] establishes on the
surface. No card-shape predicate, no positional convention. See
[[atom-independence-anaphora-only]].

Verify: engine tests; an Aura attaches to its declared slot regardless of slot
index; the existing single-slot Auras (Enchant Creature canon) attach unchanged.
