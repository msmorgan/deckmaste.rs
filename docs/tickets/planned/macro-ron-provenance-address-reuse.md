---
needs: []
---
# macro_ron provenance is keyed by addresses that a nested read frees

`cargo test -p deckmaste_semantics_v2 --test corpus every_nullary_helper_expands` fails about one
run in six (2026-10-24, six runs of the review of `lean-v3-checker-and-drt-fixes`, unrelated to
it), reporting a self-recursion that is not there:

```
Ability/giftATreasure (plugins_v2/builtin/macros/keyword_abilities/giftATreasure.ron): …
in the expansion of `gift`: … macro `createToken` argument characteristics
(CharacteristicBundle): 1:1-1:14: `treasureToken` is invoked inside its own definition
(treasureToken -> treasureToken); macros don't recurse
```

(`crates/deckmaste_semantics_v2/tests/corpus.rs:109`.)

Likely cause, inferred from the code, not yet reproduced deterministically:
`crates/macro_ron/src/expand.rs` keys provenance by memory address. `ReadCtx::chain_of` (~181)
looks a text up by its address in the thread-local `Provenance.bodies` and `Provenance.spliced`,
and `ReadCtx::enter` (~218) identifies a definition by its body's address. `spliced` entries stay
until the outermost read ends ("a splice's memory lives as long as its read"), but a nested read
(a parameter's validator reading an argument through a `ReadCtx` of its own) owns its own
`splices` arena, which drops when that read ends. A later allocation that reuses the freed
range is then given the stale entry's chain, which can hold the definition being entered: a false
cycle, depending on the allocator.

Measured rates (2026-10-25, `lean-v3-rules-and-card-fidelity`): the corpus suite failed with a
false cycle in 2 of 40 runs before that feature's range, and in 5 of 30 runs with its agent-role
`shadow.ron` mirror, which gave the false cycle one more place to strike (*Spirit en-Dal*:
`shadow -> shadow`); that mirror was reverted. One green corpus run is no evidence either way.

Fix, proposed: remove a read's `spliced` entries when the `ReadCtx` that owns those splices drops
(record the keys per read and clear them in its drop), or key provenance by an id the read hands
out instead of by address. Pin it with a test that frees a nested read's splice and allocates a
string over it, or by running the corpus test in a loop under a fixed seed.
