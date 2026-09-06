---
needs: [english-v2-grammar-migration-design, english-v2-grammar-frame-compiler]
---
# Make every planned grammar family usable in the shared production grammar

Implement the first meaningful shared form of every planned family before
finishing one family's complete inventory. This is the delivery sequence
chosen on 2026-09-06. The reviewed whole-grammar design remains the target;
existing migration tickets retain their full acceptance and deletion work.

Use the existing Earley parser, generated checked constructors, declaration
inventories and derived AST under the design's
[production correspondence](../../english-grammar-design.md#production-correspondence)
and [selection contract](../../english-grammar-design.md#admission-selection-and-retained-alternatives).
English remains independent of Semantics. A family counts only when its
intended shared representation is consumed by the production parser, with an
inhabited structural example and an independently justified exclusion. Existing
conforming implementations count; unused declarations and permissive catch-all
productions do not.

| Family | First shared implementation and interaction |
|---|---|
| Lexical predicates and agreement | Declared complete/gapped frames, finite agreement and copular complements consumed by ordinary clauses and relatives. Continue the active verb-frame migration without requiring its entire residual inventory first. |
| Nominals and adjectives | One declared adjective source with distribution and morphology, consumed by attributive and predicative phrases; nominal countability, modifier order and verb-owned participles remain enforced. |
| Subordination | Declared selected-form features on a shared dependent-clause body, used by finite and gerund clauses in initial/final attachment. |
| Prepositions | Independent declared attachment and complement licenses consumed by nominal and gerund-taking PPs. |
| Relatives and extraction | Shared clause bodies carrying category/relation Gaps, used by subject and object relatives with explicit discharge and wrong-gap exclusions. |
| Coordination and scope | Native/shared coordination plus exact retained alternatives for a correlated nested-mobile example; distinguish changes in scope from differences in construction family. |
| Measures | Shared typed operands and operators consumed by nominal qualification and a verb frame; preserve count/scalar and much/many restrictions. |
| Documents | Flat document/body collections used by ordinary and activated text, with quotation and parenthetical boundaries available to grammatical context. |
| Recoverability and ellipsis | A preceding VP form/voice summary used by one checked omission across body items; exclude future, wrong-form and quoted antecedents. |
| Type lines | Open declared Type ordering consumed by a Type Line parser and renderer; admit an unattested valid combination and reject noncanonical order. |
| Lexical/source boundary | Declared ordinary-word, opaque-name and notation recipes exercised through their real scanners, builders and rendering. The separate Target Verb rivalry remains an explicit integration obligation. |

Build in short passes across this table. Establish shared interfaces first,
then exercise cross-family compositions: adjective plus copula; subordination
plus PP; frame gaps plus relatives; measures plus frames; documents plus
recoverability. Use representative corpus subsets to expose missing shared
structure. A newly discovered interface requirement belongs in this pass;
inventory completion and uncommon forms remain with their existing owners.

Acceptance: every row has a consumed implementation, named positive/exclusion
witnesses and a cross-family interaction; the applicable rendering, ownership,
traversal and selection checks pass for those witnesses. Record remaining
family limitations against the existing migration tickets. This milestone
establishes breadth, not whole-corpus completion or readiness for the long tail.
The migration-close ticket still owns that decision and all full acceptance.
`english-lean-proof-gaps` remains required and the next formal-model task;
bounded formal corrections accompany the concrete interfaces that need them.
Standard constraints apply.
