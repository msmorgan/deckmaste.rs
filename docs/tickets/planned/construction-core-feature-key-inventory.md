---
needs: []
---
**Publish the sealed `Feature` name inventory from `construction_core`.** The
English-v2 licensing-checker census in
`crates/xtask/src/english_v2/licensing_checkers.rs` decides whether a `checked
by` body reads a declared licence feature by matching identifiers against a
hand-copied `FEATURE_NAMES` string list. Those are exactly the variants
of `deckmaste_construction_core`'s sealed `Feature`
(`crates/deckmaste_construction_core/src/model.rs`), whose authoritative key
strings live in `crates/deckmaste_construction_core/src/feature.rs` behind
`pub(crate) fn key`. A newly added feature will not be recognised and its
readers will silently demote from `DeclaredLicenseFeature` to
`StructuralPredicate` — a split every landing record now reports.

2026-09-04: stale counts removed — was: "a hand-copied `FEATURE_NAMES` list of
21 strings" and "A twenty-second feature will not be recognised". The list is
already at 22 entries, which is the drift this ticket exists to end.

Pinned shape: `deckmaste_construction_core` exposes the sealed feature key
inventory as public API (the model type's own keys, not a second copy), and
xtask's classifier consumes it. Delete `FEATURE_NAMES`.

Fence: a second hand-written copy of the feature names anywhere.

Acceptance: `FEATURE_NAMES` is gone, the census's permitted split is unchanged
on the current grammar, and adding a `Feature` variant is picked up without an
xtask edit. Gate on `cargo test --workspace` — this is an emit-contract
surface. Standard constraints apply.
