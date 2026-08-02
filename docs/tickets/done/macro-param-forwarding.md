---
needs: []
---
Support forwarding a macro's own `Param(n)` into a nested macro invocation, so
param-carrying sub-macros can be factored (not only zero-param and concrete-arg
ones).

Ruling (2026-07-12): the no-forwarding state was an implementation shortcut,
not doctrine. The macro language deliberately bans conditionals, recursion, and
meta-features — plain declarative param forwarding is none of those and works.

Resolution: the forwarding machinery already exists. `forward_arg` /
`read_args` in `crates/macro_ron/src/expand.rs` (landed 2026-07-09) pre-
substitute a body's own `Param`s into a nested invocation's arguments against
the caller's frame before the inner macro is pushed, with `HoleMode::
PassThrough` leaving the inner macro's own holes for its frame. The bans stay
intact: recursion is still a load/`MAX_DEPTH` error, no conditionals, no
wildcard kinds. `PowerAndToughnessUp` (`body: PowerAndToughness(Up(Param(0)),
Up(Param(1)))`) is a real plugin macro that already relies on it.

The original "fails with Expected opening `[`" no longer reproduces — that was
the pre-`forward_arg` state. The lore survived it in two stale spots, now
fixed:
- `plugins/builtin/macros/keyword/Flashback.ron`'s comment claimed the macro
  layer "can't forward a Param into a nested macro invocation" — corrected to
  say forwarding IS supported (the cast half is left inline by choice, to avoid
  an extra remembered-`Expanded` wrapper, not by limitation).
- No regression test pinned the ticket's exact shape (whole-value positional
  forward, and a list-typed `Cost`/`Vec` param at a nested sequence position).
  Added: `body_forwards_whole_value_param_into_positional_macro` and
  `body_forwards_list_typed_whole_value_param_into_seq_position` (macro_ron),
  plus `cost_param_forwards_into_nested_macro` (deckmaste_plugin, real
  `CastFromGraveyard(Param(0))` → `May(Cast(cost: Components(…)))`).

Follow-up candidates (deferred, each adds a remembered-`Expanded` wrapper at a
remembering kind, so they are shape changes worth their own pass): factor
Flashback's cast half into a shared `CastFromGraveyard`-style macro across the
graveyard-recast keyword family; factor `PumpThisUntilEot`'s inline pump into
`PowerAndToughnessUp(Param(0), Param(1))`.
