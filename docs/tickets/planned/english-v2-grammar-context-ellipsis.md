---
needs: [english-v2-grammar-family-breadth]
---
# Implement grammatical recoverability, gapping and shared dependents

Complete this family on the shared breadth interfaces under
[the lexical-analysis contract](../../decisions/english-lexical-analysis.md).

Thread grammatical recoverability through the migrated paragraph/body
constructors. Earlier overt VP form/voice summaries are available to later
items; no future antecedent. Quotation resets the context; Parentheticals
inherit preceding context without exporting their internal introductions.
Keep this context separate from external card-name `ParseContext` and from
all Game Model state. Do not recursively embed an antecedent tree in an
omission or add an unconstrained optional predicate.

Use the checked `EllipsisInteractions` fragment to refine intra-sentence
recoverability, nominal ellipsis and omitted destinations. Distinguish a
preceding-context omission from explicit shared Gaps discharged by following
material. Implement gapped repeated frame portions, right-node raising/shared
heads, shared determiners/prepositions and their exact retained alternatives
through the shared feature-aware packed forest. Preserve grammatical feature
correlations and alternative structures without requiring the old hoisted AST
or redundant ownership bookkeeping.

Acceptance owns the three retired destination identities: Cavalier of Thorns,
Animal Magnetism and Genesis Ultimatum, preserving their distinct destination
NPs. Include `If you can't, ...`, `If you don't, ...`, `Do this.`, grammatical
resubjected sequencing, former/latter anaphoric forms, nominal omission,
shared heads and mismatched-gap exclusions. Preserve Browbeat and Renegade
Doppelganger context examples from the review, wrong-form/voice/future/quoted
antecedent negatives, and the nested-scope distinction. Extend targeted formal
witnesses where they resolve an omission-distribution question; no proof of game
reference resolution or complete discourse adequacy is requested.
Source evidence: style-guide §5 “Names, self-reference, pronouns, and anaphora”
and §10 “Logic, choice, and coordination”.

Preserve the applicable [inherited witnesses](../../english-grammar-migration-obligations.md)
and check this family's structures under both roundtrip laws. Standard constraints
apply as amended by the lexical-analysis decision; targeted Lean changes belong
here only when needed to settle a changed grammatical constraint.
