---
needs: [english-v2-grammar-migration-design, english-v2-grammar-family-breadth]
---
# Replace subordinator-specific clauses with declared dependent-clause grammar

The first shared interface is supplied by `english-v2-grammar-family-breadth`.
Complete the acceptance below against those interfaces; other families need
not finish their inventories before this work begins.

Use one lexical subordinator inventory declaring the dependent form it selects,
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

Extend the existing composition model on these clause forms with a positive
finite/nonfinite/gerund interaction and a wrong-selected-form exclusion for
each distribution. Rust acceptance exercises the same body under two distinct
subordinators and in initial/final positions, negative/auxiliary combinations,
and existing restrictive-focus attachments. Source evidence is style-guide
§1 “Write rules instructions, not conversational prose” and §10 “Logic, choice, and coordination”.
All old-family consumers and tests are re-spelled with the replacement.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
