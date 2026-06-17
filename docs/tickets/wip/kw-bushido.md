---
needs: []
---
# Bushido

Author **Bushido** [CR#702.45a] as a Keyword-kind macro: "Bushido N" =
"Whenever this creature blocks or becomes blocked, it gets +N/+N until end of
turn."

Macro `plugins/builtin/macros/keyword/Bushido.ron`: a `Count` param `N`, a
`OneOf` trigger over the two declare-blockers transitions of THIS creature —
`StateBecomes(of: Ref(This), becomes: Blocking)` ("blocks", [CR#509.3a]) and
`StateBecomes(of: Ref(This), becomes: Blocked)` ("becomes blocked",
[CR#509.3c]) — driving the inlined until-end-of-turn pump
(`Continuously(Modify [AddPower, AddToughness] FixedUntil(EndOfTurn))`,
[CR#611.2a]). The pump is inlined rather than calling `PumpThisUntilEot`
because a macro body can't thread its own `Param` into a nested macro.

Core change: added `StateFilterEvent::Blocking` ([CR#509.1g], "becomes a
blocking creature"), symmetric to the existing `Attacking`, to give the
blocker side of the declare-blockers fact a trigger shape. Engine wires it to
the blocker of `GameEvent::Blocked` in `trigger.rs` (seam: multi-block "blocks"
under-counts because `scan_triggers` dedups `Blocked` once per attacker).
