---
needs: [english-construction-compiler, spelling-crate-rename]
---
**Replace serde layout with the typed construction boundary used by spelling.**

The exact English tree is a compiler-owned surface construction tree, not a
stable serialization schema and not the canonical Magic meaning. Generate a
transient projection from construction declarations with stable construction
and form identities, named typed roles, scalar identities, and retained surface
witnesses. Rust storage layout, private wrappers, enum indices, and field order
must not be part of that contract.

Compile the `frames:` extracted from imported RON definitions against this
projection. Keep the existing import-time frame compilation and lexicon
assembly, but migrate frame holes, agreement dependencies, unification,
substitution/rendering, diagnostics, shapes, and structural lints away from
`Serialize -> View` paths. RON definitions remain the owner of frame inventory,
parameters, and semantic bodies. Serde may remain an exhaustive diagnostic
walker, with no compatibility promise.

Prove all live frames still compile and recover/render their semantic
invocations. Delete legacy-view schema tests and production matching on serde
type names, variant names, field names, or sequence indices. Temporary adapters
for still-flat family outputs must expose grammatical roles and retire with the
family retrofit that owns them.

Standard constraints apply.
