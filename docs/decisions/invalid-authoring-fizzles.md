# Invalid authoring fizzles

## Decision

An impossible authored reference or missing authored object resolves to no
applicable object and its effect fizzles. It must not crash an in-progress game;
genuine internal engine invariant violations remain distinct and may fail
loudly.

## Rationale

Runtime object availability can change legitimately, and malformed authored
input should follow the same robust no-effect path. Static validation improves
authoring soundness, but runtime safety cannot depend on every consumer having
run that validation.

## Consequences

Reference resolution and authored-data evaluation must return an inert result
for unbound or absent objects and emit no game facts. Do not turn internal
corruption into silent behavior, and do not use panics to enforce constraints
that belong to authored data.

## Tracked references

- [Engine ADRs](../engine-adrs.md)
- [Conformance matrix](../conformance.md)
