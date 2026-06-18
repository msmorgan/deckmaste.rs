---
needs: []
---
The `eval_count` implementation in `crates/deckmaste_engine/src/layer.rs` currently only supports `Count::Literal`. This breaks all Characteristic-Defining Abilities (CDAs) and dynamic continuous effects (e.g., "This creature gets +1/+1 for each card in your hand").

While `resolve.rs` has a full `eval_count` implementation, it depends on a `Frame` and the live `GameState`. The layer engine needs a way to evaluate these counts against the *working* characteristics being derived in the current pass (especially for CDAs in Layer 7a).

Tasks:
1. Refactor `eval_count` in `layer.rs` to support `CountOf`, `StatOf`, etc.
2. Ensure it uses the `working` characteristics map where appropriate (mirrors `matches_derived`).
3. Add tests for CDAs that depend on dynamic counts (e.g., Tarmogoyf-style power/toughness).
