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

## Landing record

Implemented in change `wqvspmnr`.

### Prove

- Added the independent `English` Lake library and default build target, with
  `Grammar` and `Witnesses` imported by `English.lean`.
- `docs/english-grammar-design.md` maps all sixteen style-guide sections,
  grammatical categories/functions, lexical features/frames, recursive
  dependencies, realization ownership, and inherited ticket/fog obligations.
- Ordinary syntax plus inductive judgments separates grammatical licensing
  from realization and selection. A finite synthetic lexicon witnesses two
  unequal, grammatical trees realizing the same four-word surface. An
  adjective-to-noun misuse has no grammatical derivation.
- After `kata refresh`, `cd lean && ./scripts/build English` passed with
  warnings treated as failures (5 jobs). Lean LSP MCP diagnostics are empty
  for all three new Lean files. Live LSP goal inspection confirms the tree
  inequality closes; LSP `lean_verify` reports no axioms or source warnings
  for `English.Witnesses.distinct_analyses` and
  `English.Witnesses.adjective_not_noun`. The former includes both structural
  and surface derivations in its proof dependencies.
- Production sources and data are unchanged. The workbench-specific acceptance
  contract in the Lean design decision applies; no Rust coverage, roundtrip,
  selection, or performance result is claimed.

### Disclose

- Added 9 checked theorems: 2 structural derivations, 3 lexical realization
  witnesses, 2 composed realizations, 1 distinct-analysis witness and 1
  exclusion. Restored 0; re-spelled 0; ignored 0; removed 0.
- Deviations and additions: the lexical exclusion witness additionally checks
  that licensing is restrictive. The Lean README now distinguishes the two
  workbenches and links to the design map. No ticket-scope deviations.
- No glossary additions were needed. No STOPs or existing-test regressions.
- Only this workspace's WIP ticket was edited.

### Report and limits

This establishes representation and intended scope, not the complete grammar.
The checked fragment covers nouns, adjectives, modification and binary `and`
coordination of plural nominals. It uses surface atoms, not exact byte text.
The witness is synthetic and makes no claim about a named card or preferred
reading. Composition owns general agreement, frames, recursive phrases/clauses
and gap accounting; document grammar owns exact surface composition. Selection
and packing remain separate future work. The design map explicitly flags the
uncertainty of gap representation and extension from atoms to exact text.
