# Idris is a soundness gate

## Decision

The Idris model is a design probe and soundness gate for expanded card data, not
a parallel game engine. It should reject invalid semantic states when the model
can express the constraint without duplicating intrinsic or already-modeled
state.

## Rationale

Dependent types can make malformed references and incompatible shapes
unrepresentable before play. Keeping runtime semantics in one engine avoids two
implementations drifting while still letting the stronger model guide the Rust
grammar.

## Consequences

Validation operates on expanded semantic data and reports translation gaps
explicitly. Prefer structural types to redundant runtime flags, but do not add
an Idris representation solely to mirror engine state or rules that are
intrinsic to engine execution.

## Tracked references

- [Idris README](../../idris/README.md)
- [Keyword policy](../keyword-policy.md)
- [Rules taxonomy](../rules-taxonomy.md)
