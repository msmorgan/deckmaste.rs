# Effect atom independence

## Decision

An engine effect atom depends only on its literal arguments and explicitly
bound references. It does not infer meaning from its parent, siblings, target
cardinality, or rendered representation.

## Rationale

Independent atoms compose across ability shapes and target counts. Reading
enclosing state to repair an underspecified atom creates hidden coupling and
turns one semantic shape into an engine special case.

## Consequences

Cross-boundary branches signal a missing structural form or binding channel and
should be fixed there. A parser may analyze English phrase shape when translating
linear prose into structured data, but must not reparse emitted RON or rely on
an engine exception to recover discarded structure.

## Tracked references

- [Conformance matrix](../conformance.md)
- [Rules taxonomy](../rules-taxonomy.md)
