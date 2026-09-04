# Minimal core and data-driven rules

## Decision

The core grammar and engine retain a small irreducible primitive set. Named
keyword actions compose those primitives, while subtype and similar named
behavior is declared as data and consumed generically rather than recognized by
name in engine branches.

## Rationale

Composition avoids parallel implementations of related rules and lets plugins
extend vocabulary without expanding engine enums for every named mechanic.
Players participate through the shared object model where the rules treat them
as objects.

## Consequences

Before adding a primitive keyword action, player-only machinery, or a
type/subtype name match, identify the missing primitive, capability, or registry
datum. A true primitive requires an engine operation that cannot be expressed
as a composition of existing ones.

## Tracked references

- [Keyword policy](../keyword-policy.md)
- [Rules taxonomy](../rules-taxonomy.md)
- [Engine ADRs](../engine-adrs.md)
