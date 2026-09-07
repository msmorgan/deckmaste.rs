---
needs: [english-v2-grammar-family-breadth]
---
# Consolidate measures, comparisons and distribution

Complete this family on the shared breadth interfaces under
[the lexical-analysis contract](../../decisions/english-lexical-analysis.md).

Use shared Measure Phrase, cardinal determinative and comparative Complement
constructions instead of game-specific quantity/cost predicates. Supply
arithmetic operands, fraction/multiplicative expressions, rounding, degree,
superlative and equality/comparison as typed grammatical structure. Keep count
versus scalar notation and much/many agreement separate from arithmetic truth.
Preserve the reviewed measure/feature fragment's admitted forms and
qualification/distributive site exclusions; extend it where a changed constraint
needs a focused check.

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

Preserve the applicable [inherited witnesses](../../english-grammar-migration-obligations.md)
and check this family's structures under both roundtrip laws. Standard constraints
apply as amended by the lexical-analysis decision; targeted Lean changes belong
here only when needed to settle a changed grammatical constraint.
