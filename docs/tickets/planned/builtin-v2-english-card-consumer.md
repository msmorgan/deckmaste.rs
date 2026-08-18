---
needs: [builtin-v2-spelling-frame-consumer]
---
Implement the English-authored card source lane and carry frame-assembled
semantic invocations through the ordinary semantic and Idris validation
pipeline, completing the runtime path ratified by the
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).

Put an owned serde-readable `CardSource` / `CardFaceSource` snapshot (names
illustrative) in `deckmaste_data`. It contains structural printed fields and
exact name-bearing English rules text. Wizards-data adapters, hand-authored
plugins, and external importers all produce this same value. The type must not
depend on parser ASTs, plugin loader state, or semantic RON implementation
details.

Add `cards_english/` as the plugin-local prose lane. Each file is a
`CardSource` envelope whose rules text remains English until compilation.
Existing `cards/` remains the direct semantic-RON lane; both may coexist.
A duplicate card or face identity across either lane is a load error, never a
precedence rule. External importers may stream `CardSource` values through
the same library entry point without first writing plugin files. Generated
semantic RON may be an optional cache or review artifact but is never required
source.

After the plugin provider has built the closure-wide parser environment, parse
each English face, match reachable positional spelling frames, assemble the
compact invocation, and enter the same semantic loading, normalization,
lowering, and Idris checks used by direct RON. Do not create a second semantic
validator or a named-parameter elaboration layer. Syntactic acceptance does
not certify a binding such as `X`; an unbound value must reach and fail
semantic validation rather than being rejected as unknown English.

Acceptance is end-to-end and independent of official catalogs: a plugin
declares a novel verb and uses it in the same plugin's English card; a
dependency supplies a novel subtype and counter kind used by the dependent's
English; the same `CardSource` supplied through `cards_english/` and an
importer compiles to identical normalized semantics; a cross-lane duplicate
is diagnosed; an unbound `X` reaches semantic validation; and direct RON and
English for the same positional invocation emit the same Idris term and pass
or fail the same validation path. Standard constraints apply.
