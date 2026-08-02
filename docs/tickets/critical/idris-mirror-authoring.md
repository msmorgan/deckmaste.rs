---
needs: [plugin-repoint]
---
**Reattach the Idris mirror to the authoring kernel: the gate certifies
what authors wrote, not the compiler's output.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§10). Every existing obligation (riders-battlefield, unbound-anaphor
R1/R2, target-read range/cardinality, `distinctOk`) is an author-mistake
proof — the mirror follows its purpose.

## Scope

- The mirror models the post-expansion, post-desugar authored normal form.
  Day one this is shape-identical to today's `Core.idr`: rename the Idris
  module per the naming principle and repoint the emitter to walk authored
  terms (the emitter drops its dependency on lowering entirely).
- `deckmaste_core` carries no Idris obligations afterward (thin-mirror
  escape hatch recorded in the spec if an engine-side dependent invariant
  ever appears).
- Re-aim notes on the existing idris tickets
  (`idris-mirror-enum-gaps`, `idris-emittables-drift-control`,
  `idris-retire-cards-idr`, `idris-coalesce-payer`): one-line frontmatter/
  body updates pointing at the authoring mirror.

## Gates

Standard constraints apply. `cargo xtask idris-check plugins/canon`
identical pass set before/after (the reattachment itself is zero-diff);
`idris/scripts/build` PASS.
