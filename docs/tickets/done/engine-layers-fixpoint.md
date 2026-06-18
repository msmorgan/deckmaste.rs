---
needs: [engine-layers-misc]
---
Currently, `GameState::layers` in `layer.rs` gathers static abilities by reading the *printed* abilities of objects to avoid recursion. 

```rust
// v1: uses printed_abilities (not the derived view) to break the
// layers() → derive::abilities → layers() recursion.
```

This means that if a static ability is *granted* to an object by another continuous effect (e.g., "Creatures you control have 'Creatures you control get +1/+1'"), the granted ability will not be gathered and thus won't function.

Tasks:
1. Implement a fixpoint iteration in `GameState::layers`.
2. Re-gather static abilities from the *derived* characteristics after Layer 6 (Ability Adding/Removing) has applied.
3. If new static abilities are found, re-run the relevant layers until the set of active effects stabilizes.
4. Ensure termination (MTG layers are designed to be finite, but guard against infinite loops).
