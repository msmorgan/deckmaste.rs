---
needs: [plugin-repoint]
---
**Reattach the Idris mirror to the semantics kernel: the gate certifies
semantic input, not the compiler's output.** Design:
`docs/decisions/semantics-spelling-lowering.md` (§10). The mirror's real
obligations are semantic-input proofs — unbound-anaphor soundness (the
R1/R2 gates), target-read range and cardinality, `Distinct` range
checking — so the mirror follows its purpose.

## Scope

- The mirror models the semantics kernel (spec §10): the post-expansion,
  post-desugar normal-form value universe of the emitted families —
  containers + grammar as emitted today; strategy, MacroDef machinery,
  and frames data excluded. The reattachment is CONTENT-preserving, not
  shape-identical: the emitted kernel and idris-check pass set are
  unchanged, while the pre-existing Rust↔Idris drift (missing variants,
  emitter-bridged arity differences — `idris-mirror-enum-gaps`'s
  inventory) is untouched, neither fixed nor worsened. Rename the Idris
  module per the naming principle and repoint the emitter to walk
  semantic terms (the emitter drops its dependency on lowering —
  normalization is semantics-side, spec §9).
- **Phantom obligation, resolve one way or the other**: the
  riders-battlefield-only rule exists ONLY as prose on `EnterRider` in
  `action.rs` ("rejected by the Idris re-emit gate" — verified false: no
  such proof exists, and the emitter passes rider shapes through or gaps
  on them without destination checks). Either mint the real destination
  proof on the semantic mirror, or correct the prose to say the rule is
  unenforced — do not leave the false claim standing.
- `deckmaste_core` carries no Idris obligations afterward (thin-mirror
  escape hatch recorded in the spec if an engine-side dependent invariant
  ever appears).
- Re-aim the affected ticket set: this is an OPEN-ENDED sweep — grep the
  ticket tree for `Semantics.idr` / emitter / `deckmaste_plugin` references at
  claim time (at minimum the `idris-*` family plus
  `memoize-macro-invocations`, `macro-keyword-templates`,
  `parse-macro-slot-readers`) rather than trusting any closed list.

## Gates

Standard constraints apply. `cargo xtask idris-check plugins/canon`
identical pass set before/after (the reattachment itself is zero-diff);
`idris/scripts/build` PASS; the riders-battlefield resolution (proof
minted or prose corrected) landed.
