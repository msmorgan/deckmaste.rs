---
needs: [english-v2-grammar-migration-design, english-v2-grammar-family-breadth]
---
# Consolidate measures, comparisons and distribution

The first shared interface is supplied by `english-v2-grammar-family-breadth`.
Complete the acceptance below against those interfaces; other families need
not finish their inventories before this work begins.

Use shared Measure Phrase, cardinal determinative and comparative Complement
constructions instead of game-specific quantity/cost predicates. Supply
arithmetic operands, fraction/multiplicative expressions, rounding, degree,
superlative and equality/comparison as typed grammatical structure. Keep count
versus scalar notation and much/many agreement separate from arithmetic truth.
Extend the reviewed measure/feature fragment with the admitted forms and
qualification/distributive site exclusions before production migration.

Replace `CostComparisonPredicate` and the comparison-specific frame codecs
with ordinary declared frames taking measures. Own `half their life, rounded
up`, `twice X`, `X plus Y`, `X minus Y`, bare variable cardinal X,
`amount of X equal to Y`, `the greatest X`, `equal to` nominal qualification,
`twice that much/many`, `target beyond the first` and degree-modified AdvP.
Preserve the distribution/counter grammar already landed; an energy-symbol
sequence is notation used as a measure, not a new game-effect category.

Acceptance: positive typed arithmetic/comparison nesting and exact surface,
wrong count/mass and much/many forms excluded, qualification never gets a VP
Adjunct site, distributive measure never gets a nominal Postmodifier site,
and syntactically valid arithmetic remains admitted regardless of its value.
Cover `cost {1} less to cast` through the shared frame, with no special cost
predicate. Source evidence: style-guide §4 “Numbers, quantities, and comparisons”.

Standard constraints apply. Production correspondence and the applicable
[obligations](../../english-grammar-migration-obligations.md) are part of this
ticket; re-spell existing tests by their independently justified outcomes.
