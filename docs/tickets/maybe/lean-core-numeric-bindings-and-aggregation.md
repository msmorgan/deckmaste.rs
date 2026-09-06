---
needs: []
---
# Design member binding, aggregation, and numeric anaphora

Parked by the user on 2026-09-06. Investigate these two parts independently;
proximity in the numeric vocabulary does not make either a prerequisite for
the other. Existing constructors remain until a concrete design is accepted.

## Member binding and aggregation

Explore an explicit collection-member binder for numeric folds. The proposed
expansions make `countOf` a sum of one per member and projected totals/extrema
folds over property reads. Keep selection separate from measurement and
preserve numeric regimes and surrounding references. Authoring macros would
hide the binder.

First establish how this fits the [layer contract](../../decisions/semantics-v2.md#1-the-layer-contract):
constructions stay in surface positions, and hoisting/indexing belongs to
lowering. A locally scoped member binder is not permission for prenex binders
or positional core reads in the semantics workbench. Show concrete terms,
their scope threading, and a reduction in shared obligations before choosing
a representation. Do not reopen the parked Idris segment-indexed telescope
without its own trigger.

## Numeric anaphora

Separately assess folding `Amount.thatMuch`, `theOutcome`, `groupSize`, and
`theDifference` through typed binding reads and projections. They are not all
outcomes: a group can belong to a proposed event and a difference comes from
a comparison. Preserve category, cardinality, scope, and ambiguity rules.
Bare "that much" must not mean the most recent number; two eligible numeric
outcomes remain ambiguous, while an explicit category can disambiguate a read.
Comparison gaps do not escape negation or alternatives. Group size must work
for proposed event groups as well as completed results.

Audit which reads are admitted in comparisons without blindly inheriting
constructor-specific exclusions. This proposal does not require member binding
or replacement of every `OutcomeSort` value. It needs its own benefit and
scope evidence.

## Decision evidence

Produce concrete before/after terms, scope traces, and preserved positive and
negative pins for each part. Include aggregate member reads, ambiguous numeric
outcomes, comparison gaps, and event-group sizes (Woodland Champion and Divine
Visitation). Compare outer constructors and total vocabulary separately.
Record the design decision before implementation; either part may remain
parked if the payoff is insufficient. Standard constraints apply.
