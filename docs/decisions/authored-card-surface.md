# Authored card surface

## Decision

Card RON uses terse positional forms for low-arity constructs, named macro atoms
for recurring phrases, explicit `Selection` values for plurality, and
structural or positional anaphora rather than string-named binders. Established
authored shapes are not restructured without explicit user review.

## Rationale

Cards are data, and their surface should resemble the rules text's structure
without exposing engine bookkeeping. A single plurality abstraction and
stable, compact forms make the corpus readable and keep grammar changes
deliberate.

## Consequences

Prefer singular operations wrapped by explicit distributors, and add or reuse
typed macro atoms instead of embedding repeated filter trees. Do not introduce
string labels as variables, implicit whole-set scope, or vector-shaped authored
slots where a singular slot plus a plural wrapper expresses the rule.

## Tracked references

- [Rules taxonomy](../rules-taxonomy.md)
- [Conformance matrix](../conformance.md)
- [Keyword policy](../keyword-policy.md)
