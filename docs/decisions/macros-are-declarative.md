# Macros are declarative

## Decision

Plugin macros are kind-checked positional templates with typed parameters.
They may forward parameters into nested macros, but they are not a language for
conditionals, loops, reflection, wildcard kinds, macro generation, or
intentional recursion.

## Rationale

Macros name and reuse card-data structure; control flow would make them a
second behavior language and weaken validation. Parameter forwarding is
ordinary template composition and does not require that expansion in power.

## Consequences

Add behavior to the core data model or engine when it cannot be declared as a
template. Expansion cycles are authoring errors, and proposals for macro
meta-features require a separate design decision rather than incremental
convenience changes.

## Tracked references

- [Keyword policy](../keyword-policy.md)
- [Rules taxonomy](../rules-taxonomy.md)
