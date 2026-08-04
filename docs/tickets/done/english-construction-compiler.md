---
needs: [english-feature-vocabulary, english-construction-identity]
---
**Implement the construction declaration compiler and its suite.**

Build the declaration language and compiler of
`docs/decisions/english-grammar-is-derived.md`, scoped to the hole classes
and backend the coordination pilot needs (fan-out-one productions on the
existing chart; further hole classes and any PMCFG extension land only when
a family first meets the decision's criterion).

Implement in four separable layers:

1. A typed declaration model covering stable identity, AST construction and
   destruction, linearization and accepted surface domain, the pilot's hole
   classes and feature constraints, surface witnesses, and dominance edges.
2. Declaration validation that rejects impossible combinations, cyclic
   dominance, and incomparable maxima where the declaration promises one
   answer.
3. Deterministic generation of parser productions with reduction/lowering, a
   total destructurer and linearizer, public smart constructors and validation
   adapters, `inspect` metadata, and structural registry data.
4. Backend adapters for the existing fan-out-one chart and for validated AST
   ingress. Invariant-bearing generated families seal direct field
   construction outside generated code, and deserialization passes through
   the same validator; a bypassable family is not eligible to migrate.

Exact parse results carry `(ast, surface)` alternatives. The suite proves the
set-valued laws, not a single-parse shorthand:

~~~text
(ast, surface) in parse_as(
    construction,
    linearize(construction, ast, surface),
)

for every (ast, surface) in parse_as(construction, bytes):
    linearize(construction, ast, surface) == bytes
~~~

Packed chart nodes may be shared, but generated reductions must retain
alternatives that differ in surface witness. Generated output and semantic
selection must also be invariant under declaration and registry permutation
except where explicit dominance applies.

Gates: golden generated code plus property round-trips establish both
construction laws over every exercised `(ast, surface)` combination, including
ambiguous and negative fixtures. Compile-fail/API-visibility and negative
deserialization fixtures prove sealed families cannot be bypassed. A
structural fixture proves the emitted parser, renderer, builder, `inspect`
metadata, and registry row all originate from one declaration. The suite is
green before any handwritten path is deleted. Coordination declarations may
serve as the test corpus, but the registry keeps production traffic on the
handwritten family until `english-coordination-derived` flips it — no
production behavior change in this ticket.

Standard constraints apply.
