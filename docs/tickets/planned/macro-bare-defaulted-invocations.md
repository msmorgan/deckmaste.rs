---
needs: []
---
Allow a bare macro name to read as a zero-arg invocation when every param has a
default (`Hexproof` instead of today's required `Hexproof()`). Needs a probe-free
path through serde's one-shot `VariantAccess` or a macro-layer pre-scan.

## Implementation notes (head-start, 2026-06-14)
Empirically: ron 0.12 `VariantAccess` is one-shot with no shape-peek — a bare name
only resolves via `unit_variant()`, `struct_variant()` errors on it, and there's no
clean discriminator (`read_args` at `crates/macro_ron/src/expand.rs:973`). The
pre-scan route is the viable one: capture the raw fragment (the `&RawValue` capture
at expand.rs:1023 is precedent) and look ahead for `(` before dispatching, then
rework the hot `EnumIntercept` path (expand.rs:1079). Guard the ~31k-card corpus
against regression. Serialization stays `M()`.
