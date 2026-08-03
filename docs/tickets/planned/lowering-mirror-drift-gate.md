---
needs: []
---
**Nothing enforces the authoring↔core mirror.** 33 of 35 source files in
`deckmaste_core/src` and `deckmaste_authoring/src` are byte-identical
today (all but `lib.rs` and `ron.rs`, plus the intentional singletons:
authoring's `card.rs`/`macros.rs`, core's `plugin.rs`), and
`deckmaste_lowering` is ~11.5k lines of identity arms with an empty
divergence ledger. The fork's contract
(`docs/decisions/authoring-spelling-lowering.md`) makes divergence a
deliberate, justified act — but no gate distinguishes deliberate from
accidental: a bugfix applied to one copy and not the other drifts
silently, and the per-variant lowering tests only catch drift that
changes a shape, not a doc comment, a derive, or a helper body.

Add the mechanical guard: an xtask check (or a plugin-suite test, next to
the `syn`-based gates) that diffs each mirrored file pair against an
explicit allowlist of intentionally-diverged files — the divergence
ledger made machine-readable. A pair diverging without an allowlist entry
fails; an allowlisted pair that has converged back warns (stale entry).
Flag the reverse direction too: a core-only variant unreachable from
authoring is currently silent.

Timing: `core-demacro` and `macro-author-surface` (critical/) will
diverge these files wholesale and legitimately — this gate is what turns
each of those diffs into a recorded ledger entry instead of ambient
drift, so land it before (or with) the first of them.
