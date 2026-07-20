# Types grant capabilities

## Decision

Types and subtypes grant positive, default-deny capabilities that engine
subsystems consume. Those grants are derived from the object's current layered
characteristics, so gaining or losing a characteristic gains or loses its
conferred behavior.

## Rationale

Positive grants model what a characteristic enables without encoding the
complement as a prohibition. Recomputing from current characteristics preserves
layer semantics and prevents stale cached abilities.

## Consequences

Data declares capabilities; casting, play, combat, attachment, and other engine
systems interpret them. Eligibility may subtract applicable prohibitions from a
grant, while facts such as an object's current participation in combat must use
the capability semantics appropriate to that subsystem.

## Tracked references

- [Engine ADRs](../engine-adrs.md)
- [Rules taxonomy](../rules-taxonomy.md)
- [Conformance matrix](../conformance.md)
