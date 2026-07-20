---
needs: []
---
**`parse_conditional` renders a wrapper and immediately un-renders it by string
surgery.** Same wrapper-surgery class as [[parse-ascend-fold-structured]]: the
parser builds a `Static(…)` string, then peels the shell back off to re-wrap.

`crates/deckmaste_migrations/src/parsers/static_ability.rs:292-297`: the "As long
as X, Y" production recursively re-enters `parse()`, receives the fully rendered
`Static(<effect>)` ability string, strips the `Static(` shell by
`strip_prefix("Static(").and_then(|s| s.strip_suffix(')'))`, and re-wraps as
`Static(Conditionally(<condition>, <inner>))`. Sound today only because every
internal production happens to emit exactly `Static(…)`; a production emitting
anything else (a macro invocation, an extra field) silently declines, and a
`Static(`-prefixed-but-differently-shaped emission would mis-slice.

## Fix

Split the module's internal channel: internal productions return the bare
`StaticEffect` RON; `resolve_line` applies the `Static(…)` shell at one exit point;
`parse_conditional` composes `Conditionally(cond, inner)` structurally, never
un-rendering. See [Effect atom independence](../../decisions/effect-atom-independence.md).

Parser-side and distinct from [[engine-conditional-statics-wiring]] (which wires
`Conditionally` into the engine layers for execution); they meet at the same
grammar variant.

Verify: the existing `static_ability` tests pin the same `Static(Conditionally(…))`
output; a production emitting a non-`Static(` shape no longer mis-slices; `cargo
test --workspace` green.
