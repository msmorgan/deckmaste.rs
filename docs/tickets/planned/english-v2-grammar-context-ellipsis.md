---
needs: [english-v2-grammar-migration-design, english-v2-grammar-family-breadth]
---
# Implement grammatical recoverability, gapping and shared dependents

The first shared interface is supplied by `english-v2-grammar-family-breadth`.
Complete the acceptance below against those interfaces; other families need
not finish their inventories before this work begins.

Thread grammatical recoverability through the migrated paragraph/body
constructors. Earlier overt VP form/voice summaries are available to later
items; no future antecedent. Quotation resets the context; Parentheticals
inherit preceding context without exporting their internal introductions.
Keep this context separate from external card-name `ParseContext` and from
all Game Model state. Do not recursively embed an antecedent tree in an
omission or add an unconstrained optional predicate.

Extend the checked `EllipsisInteractions` fragment for intra-sentence
recoverability, nominal ellipsis and omitted destinations. Distinguish a
preceding-context omission from explicit shared Gaps discharged by following
material. Implement gapped repeated frame portions, right-node raising/shared
heads, shared determiners/prepositions and their exact retained alternatives
through the existing scope device, not a second packing mechanism.

Acceptance owns the three retired destination identities: Cavalier of Thorns,
Animal Magnetism and Genesis Ultimatum, preserving their distinct destination
NPs. Include `If you can't, ...`, `If you don't, ...`, `Do this.`, grammatical
resubjected sequencing, former/latter anaphoric forms, nominal omission,
shared heads and mismatched-gap exclusions. Preserve Browbeat and Renegade
Doppelganger context examples from the review, wrong-form/voice/future/quoted
antecedent negatives, and the nested-scope distinction. Extend formal
witnesses before relying on each new omission distribution; no proof of game
reference resolution or complete discourse adequacy is requested.
Source evidence: style-guide §5 “Names, self-reference, pronouns, and anaphora”
and §10 “Logic, choice, and coordination”.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
