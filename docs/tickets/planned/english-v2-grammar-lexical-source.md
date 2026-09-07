---
needs: [english-v2-lexical-analysis]
---
# Enforce lexical and source correspondence across the migrated grammar

Complete the inventory and source-recipe accounting from
`english-v2-lexical-analysis` under
[the accepted decision](../../decisions/english-lexical-analysis.md#lexical-analysis).
Work independently of grammar-family completion. Preserve its supported-corpus
word list, catalog/symbol accounting and unknown-form examples; batch missing
lexemes/forms with declarations and explicit overrides rather than guessing POS.

Start from the [lexical interface and inventory command](../../../crates/deckmaste_lexical/README.md).
The report's `unmapped_sources` names unmapped core inventories, construction
literals, modal applicability and keyword-derived adjectives. Its raw occurrences
also retain genitive/contraction, symbol and catalog gaps. Consolidate the
supplemental form overrides under their existing declaration owners as their
authoring sources acquire the shared morphology interface.

Audit the independent declaration/lexical/source interface using the source map
in the grammar design. Enforce declaration provenance for word-bearing forms, declared
morphological allomorphs/genitive endings, Onset, lexical identities and
structural separators. Ordinary word payloads reject whitespace/quote
boundaries under their declared recipe; opaque exact identities and explicitly
structured multiword or notation recipes keep their distinct contracts. Do
not apply the workbench's single-word example predicate to all provider strings.
No raw source span may become an escape-hatch AST leaf.

Populate remaining lexical forms needed by the named migration families;
re-check the historical batched rows without repeating completed additions.
Own linguistic `door`/half-reference expressions, `historic`, `devotion`,
`party`, `The Ring tempts you`, `transforms into <name>`, and missing counter
kinds as lexical/frame/phrase uses. Game implementations of those concepts and
retirement-bound v1 tickets are not English prerequisites. If a named family
still needs a general grammatical change, route it to its grammatical owner without turning vocabulary completion
into a nested grammar migration.

Test source-to-render equality at the actual normalized-input boundary and
state any preexisting upstream normalization separately from raw source
preservation. Cover apostrophes/genitives, opaque names with punctuation,
bound-keyword forms, contraction/allomorph selection and source punctuation
through lexical analysis and lexical realization. Derived positions may replace
redundant claim storage; exact lexical identity and surface evidence still applies.
The chart-integration ticket owns the same cases through the parser and generated
renderer, with later family-specific cases owned by their completion tickets;
those consumers are not hidden prerequisites to completing this inventory. Close the
review's supplied-Lexicon correspondence obligation for the declared recipes;
no game validator, Lean renderer, or whole-English completeness claim.
Style-guide evidence: §§2–3, 5, 7 and 14 in the source map.

Report every remainder as declared morphology/vocabulary, catalog/notation,
or a named unresolved lexical/source issue. Do not equate a short remainder
list with full grammar coverage. Acceptance here checks declared lexical analysis/generation in both directions
and the supported-corpus remainder accounting. The chart and family owners check
both whole-reading roundtrip laws through their consumers; this ticket hands
them named source cases rather than waiting for those implementations.

Standard constraints apply. The accepted lexical-analysis decision and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
