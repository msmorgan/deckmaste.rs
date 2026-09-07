---
needs: []
---
# Connect lexical analyses to admission and preserve all grammatical readings

Refine the independent `english/` model for
[the accepted lexical/ambiguity contract](../../decisions/english-lexical-analysis.md).
This is the formal companion of lexical analysis and first chart integration,
not an executable Lean parser, a Semantics consumer or a proof of Rust code.

Add a relation for declared lexical analyses, keeping Lexeme, lexical Category,
Word Form and applicable features correlated. Connect it to actual grammatical
admission and realization. Preserve the `WordFormInteractions.wrong_*_admitted`
counterexamples as historical evidence of the old judgment, or name their
replacement exclusions explicitly; the corrected judgment must reject the
crossed forms while admitting the valid counterparts on the same syntax path.

Make all admitted readings available independently of optional preference.
`Analysis.Selected` already allows ties; `Analysis.package` currently covers
only one `Related` class. Demonstrate two grammatical readings of the same
surface outside that class both survive. Preserve local scope relationships
and prove the chosen packaging retains exactly the admitted alternatives,
without creating correlations by Cartesian product. Instantiate the law on a
grammatical witness with individually possible but jointly invalid choices;
the Boolean-only example is insufficient. This discharges proof-gaps item 4
and the corresponding feature-bypass/packing-key obligations where actually met.

State both relational roundtrip obligations on a clearly delimited set of
independently constructible valid readings and their realizations. Mere
projection from the definition of `Admissible` is not evidence of reverse
recovery or deterministic rendering. Preserve exact lexical/structural and
spelling-variant distinctions; record remaining assumptions explicitly.
Model forest structure only when needed to expose a concrete loss/merging defect.

Acceptance: nonvacuous inhabitants, crossed-form exclusions, unrelated
same-surface readings and a non-Cartesian grammatical correlation witness;
named replacement for every changed proof claim. Record which residual
`english-lean-proof-gaps` obligations this work actually discharges. Use Lean
LSP MCP, `english/scripts/build`, and the standard axiom audit. Standard
constraints apply; no Rust corpus gate is required for Lean-only changes.
