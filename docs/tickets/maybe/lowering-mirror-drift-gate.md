---
needs: []
---
**[design] Reconsider machine-enforced authoring↔core mirror drift.** Today
33 of the 35 shared source files in `deckmaste_core/src` and
`deckmaste_authoring/src` are byte-identical; only `lib.rs` and `ron.rs`
differ, alongside the intentional one-sided `card.rs`, `macros.rs`, and
`plugin.rs`. A correction applied to only one mirrored helper or derive can
therefore drift silently.

This proposal reopens an owner-settled tradeoff. Section 9 and downside 3 of
`docs/decisions/authoring-spelling-lowering.md` deliberately make the
lowering crate's per-variant mapping tests plus in-place justifications the
divergence ledger, accepting prose/review governance instead of a second
machine-maintained mirror. Do not implement a gate until that decision is
explicitly reaffirmed or amended.

Design questions:

1. Is the desired invariant whole-file identity, an expected diff for each
   pair, or only variant/field-shape parity? A bare allowlist stops checking a
   file forever after its first legitimate divergence and is therefore too
   weak to mean "every divergence is deliberate."
2. If expected diffs are recorded, should CI fail both on new drift and on a
   stale entry whose pair converged? A warning is too easy for a ledger to
   rot behind.
3. How does the check coexist with the existing one-mapping-test-per-variant
   obligation without creating a third defining schema?
4. Should core-only variants be classified as engine-internal or
   authoring-unreachable? That reverse-reachability question is semantic and
   cannot be answered by a source-file diff.

If approved, record the amended governance in the decision, choose one
machine-readable representation, add positive and negative fixtures, and
then add this ticket as a graph dependency of any not-yet-landed ticket that
intentionally creates the first governed divergence. Until then it must not
block `portfolio-polish` or the authoring program.
