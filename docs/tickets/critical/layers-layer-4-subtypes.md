---
needs: [engine-layers-misc]
---
In `crates/deckmaste_engine/src/layer.rs`, Layer 4 subtype modification is currently a no-op stub:

```rust
// [CR#613.1d] subtype-set deferred: Ident→Subtype reconcile (no fixture yet)
// `Modification::SetSubtypes`/`AddSubtypes` carry `Vec<Ident>` but
// `Characteristics::subtypes` holds `Vec<Subtype>` (structs with confers/types).
```

This is because `Modification` carries bare `Ident` names, but the derived `Characteristics` uses `Subtype` structs (which carry inherent abilities/rules). We need a mechanism to reconcile these.

Tasks:
1. Implement `Ident` to `Subtype` lookup using the plugin/registry data.
2. Wire `Modification::SetSubtypes` and `Modification::AddSubtypes` in `layer.rs`.
3. Verify that tribal-granting effects (e.g., "Each creature you control is a Slivers in addition to its other types") work correctly and that the granted subtype's inherent rules (if any) are correctly applied.
