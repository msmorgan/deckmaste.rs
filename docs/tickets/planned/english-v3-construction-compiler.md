---
needs: [english-v3-packed-chart]
---
# Build the fresh bidirectional construction compiler

Create `deckmaste_construction_v3_core` as a normal, directly testable compiler
crate and `deckmaste_construction_v3` as its thin proc-macro shell. They do not
depend on the v2 compiler or parser. Treat the old compiler as a quarry: port
declaration syntax, diagnostics, normalization and generation patterns only
after each piece fits the v3 intermediate representation.

One construction declaration must generate the Category/AST types, chart
Productions and lexical terminal constraints, grammatical-summary/admission
logic, lazy Reading materializers, Realization, and structural plus lexical
traversal. All generated projections come from one validated compiler IR. The
compiler may depend on `deckmaste_lexical_model`; it must not load lexical
catalogs, generate morphology, scan input, construct full AST products during
recognition, store redundant token ownership, or select one Reading from a tie.

Reject declarations whose parser and renderer surfaces diverge, whose required
feature dependency cannot reach admission, whose materializer loses a lexical
identity or spelling variant, or whose recursive/nullable shape cannot be
packed safely. Diagnostics identify the declaration and generated obligation,
without exposing proc-macro panics.

Acceptance compiles synthetic constructions through the public proc macro and
checks every emitted projection. For every finite synthetic Reading, check
byte-exact parse/realize roundtrip and independently constructed-value
realize/parse roundtrip before the generated slice relies on the compiler.
Include alternative surface forms, optional and repeated constituents,
nullable structure, feature agreement, lexical frames, distinct same-surface
Readings, and duplicate-derivation packing. No v2 compatibility aliases or
generated scanner callback are introduced. Standard constraints apply.
