# Macro templates are bidirectional

## Decision

A macro's template is the shared source of truth for rendering and reverse
parsing. Typed slot codecs use the macro's declared parameter types in both
directions; a separate parse pattern is added only after a real case proves
template derivation insufficient.

## Rationale

Independent render and parse descriptions drift. Declared slot types preserve
grammar boundaries that cannot be recovered reliably from an expanded,
text-substitution body.

## Consequences

Template syntax and slot resolution must be interpreted consistently by the
renderer and pattern compiler. Macros without templates do not participate in
this path.

## Tracked references

- [Keyword policy](../keyword-policy.md)
- [Rules taxonomy](../rules-taxonomy.md)
