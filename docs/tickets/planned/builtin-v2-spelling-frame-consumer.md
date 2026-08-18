---
needs: [builtin-v2-plugin-grammar-provider]
---
Compile positional semantic `spelling` frames and match them against parsed
English under the declaration-built environment ratified by the
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
This ticket begins after grammar has produced an AST and ends with a compact
semantic invocation. It does not own `CardSource`, plugin source lanes, or
the Idris validation pipeline.

Compile each reachable graduated declaration's `spelling` exactly once into
a typed partial pattern over the generated English AST. Resolve every
`<Param(n)>` directly against the declaration's positional type vector.
Unknown types, out-of-range holes, duplicate or missing required positions,
and a literal/grammar-head coherence failure are load errors. There is no
named formal parameter, name-to-index table, declaration-order lookup, source
substring matching, or textual splice-and-reparse.

Bind every frame to its enclosing category-safe declaration identity. Never
select a frame by globally looking up its head surface: homonymous declarations
and multiple syntactic readings remain distinct candidates. An ungraduated
nursery record can contribute grammar and parse/render vocabulary but cannot
be selected because it lacks the typed signature and semantic body.

Use the generated total traversal/substitution surface to recursively recover
typed arguments and assemble `M(a0, ..., an)`. Valence constrains syntax only:
`Numerative` admits both `scry 1` and `scry X`, while basic crossed shapes
such as `destroy 1` and `scry each creature target opponent controls` fail
in grammar. A same-valence phrase with the wrong hole type may parse and then
fail frame matching. Binding and availability of `X`, Magic legality, guards,
and semantic refinements remain downstream validation concerns.

Retain every equally valid frame match and report ambiguity; registration,
dependency, catalog, filesystem, and traversal order are not tie-breakers.
Keep the assembled invocation positional so direct RON translation remains
structurally identical to Idris application.

Acceptance covers all four valences, the exact finite `Custom` tail shapes,
literal and `X` amount holes, recursive typed argument recovery, a
hole-type-incompatible same-valence parse with no frame match, homonymous
declarations, an unselectable nursery record, identity-local coherence
failure, and multiple valid frames reported as ambiguity. Parser assertions
inspect AST and selected declaration identity rather than bracket-string
goldens. Standard constraints apply.
