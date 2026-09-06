# Oracle English grammar in Lean

This independent Lake project models an NLP grammar for parsing and bracketing
Oracle English. Its categories, lexical frames, composition, surface relations
and ambiguity proofs concern linguistic structure. It does not import,
communicate with, or validate against the Semantics project.

Run the complete gate from this directory:

```sh
./scripts/build
```

The project has its own `lakefile.toml`, `lean-toolchain`, dependency manifest
and build artifacts. There are no package dependencies. `English.lean` imports
the grammar and all checked witnesses. Surface relations let the model compare
analyses of the same text; an executable parser or renderer is not required.

See the [design](../docs/english-grammar-design.md),
[review](../docs/english-grammar-review.md), and
[decision](../docs/decisions/english-lean-design-workbench.md).
