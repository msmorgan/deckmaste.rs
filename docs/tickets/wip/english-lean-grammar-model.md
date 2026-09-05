---
needs: []
---
# Establish the English formal grammar model

Implement the first step of the [Lean design decision](../../decisions/english-lean-design-workbench.md):
`lean/English.lean` and an `English` library beside the integrated `Semantics`
package. Reuse its toolchain and build conventions. Work in the normal feature
workspace after the Semantics port is integrated; do not copy that workspace
or port Semantics again. Keep the two bounded contexts separate.

First write `docs/english-grammar-design.md`: a compact map of the whole
intended grammar, its categories, relations, lexical features, frames,
composition, and surface ownership. Use the Oracle English glossary and
style-guide sections as the source map. Account for all guide sections,
including phrase/clause grammar, notation, keyword surfaces, and document
structure; mark an exclusion or open question explicitly. Include the
[wayfinder's](../../english-grammar-wayfinder.md) inherited obligations and fog
families without deriving the inventory from today's failure frontier.

Pin the formal vocabulary and dependency structure, including how recursively
related phrases and clauses share one model. Choose indices or ordinary syntax
with judgments on their merits; state the tradeoff. Define grammatical
derivation and a relational connection to surfaces, separately from selection.
Make a small nonempty fragment typecheck with an explicit structural witness
and its surface derivation. This establishes the representation, not full
grammar coverage. Record exact remaining decisions for the following tickets.

Acceptance: the scope map and Lean definitions agree; `lake build English`
checks without warnings or proof placeholders; the chosen surface relation
can express two distinct analyses of the same surface without assuming their
equality. No parser, renderer algorithm, Rust changes, or production corpus
measurement is required. Standard constraints apply.
