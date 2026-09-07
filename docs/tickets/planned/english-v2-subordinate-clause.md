---
needs: [english-v2-grammar-family-breadth]
---
# Replace subordinator-specific clauses with declared dependent-clause grammar

Complete this family on the shared breadth interfaces under
[the lexical-analysis contract](../../decisions/english-lexical-analysis.md).

Use independent lexical subordinator analyses declaring the dependent form selected,
with shared finite, infinitival and gerund-participial clause categories.
Preserve distinct constructions where the dependency or surface structure
actually differs. Replace the per-subordinator × form families and update
`PreposedClauseTail`, `SimplePostposedClauseTail` and `PostposedClauseTail`
consumers from the landed focus-adverb work; no second tail family.

Initial and final adverbial attachment, conditionals, tense/voice/negation,
resubjected sequencing and focus use the common clause/VP grammar. This owns
`as long as`, `as though`, `until you <verb>`, `Otherwise,`, and the adverbial
structure of `as an additional cost to cast this spell, ...`. Lexical PP uses
of `as` and comparative `than` are assigned to prepositions/measures. Gerund
clauses must also be usable as PP Complements (`instead of putting ...`,
`by replacing ...`, `rather than paying ...`). Ellipsis is explicitly deferred
to the context ticket, not represented by an optional unconstrained VP here.

Preserve positive finite/nonfinite/gerund interactions and wrong-selected-form
exclusions for each distribution. Use the existing composition model to resolve
specific constraint questions as needed. Rust acceptance exercises the same body under two distinct
subordinators and in initial/final positions, negative/auxiliary combinations,
and existing restrictive-focus attachments. Source evidence is style-guide
§1 “Write rules instructions, not conversational prose” and §10 “Logic, choice, and coordination”.
All old-family consumers and tests are re-spelled with the replacement.

Preserve the applicable [inherited witnesses](../../english-grammar-migration-obligations.md)
and check this family's structures under both roundtrip laws. Standard constraints
apply as amended by the lexical-analysis decision; targeted Lean changes belong
here only when needed to settle a changed grammatical constraint.
