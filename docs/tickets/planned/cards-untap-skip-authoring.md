---
needs: [engine-untap-skip]
design: true
---
Wire **card authoring** for the untap-skip primitives landed by `engine-untap-skip`,
then graduate the ~26+ blocked "doesn't untap" cards. The engine already has both
representations — `DeonticAction::Untap { what }` (continuous, under `Cant`) and the
one-shot `GameObject.skip_next_untap` consumed flag — and the untap step honors both.
What's missing is the **path from card RON/oracle text to those primitives**.

## Two authoring surfaces

1. **Continuous flavor → a static `Cant(Untap(what:…))`.** Cards:
   `Enchanted creature doesn't untap during its controller's untap step.` (~9 auras),
   `~ doesn't untap during your untap step.` The `DeonticAction::Untap` variant exists,
   but there is **no Core.idr grammar / emitter path** to spell it: `idris_emit.rs`
   `emit_deed` currently *gaps* `DeonticAction::Untap` because an **agentless deed**
   (untap is a turn-based action `[CR#502.3]`, no `Enact Relation agent patient`
   counterpart) has no existing Idris shape. **Design decision required:** how to
   represent an agent-less deontic deed in Core.idr (a dedicated nullary-agent form vs.
   a `TurnBasedAction` predicate vs. reusing an existing shape). This is the `design:`
   gate.

2. **One-shot "next untap step" rider → set `skip_next_untap` from a resolving effect.**
   Cards: `{T}: Add {C}. ~ doesn't untap during your next untap step.` (~17 temple/
   painland mana riders), `You may choose not to untap ~ …`, exert `[CR#701.43a]`.
   Needs a card-facing verb (e.g. `PlayerAction::SkipNextUntap { what }`) wired
   resolution → `GameEvent` → apply (sets the flag). The agent that built the primitive
   deliberately left this unwired ("a half-added unwired verb would be a silent-no-op
   footgun") — so this ticket threads the new `GameEvent` variant through ALL exhaustive
   event matches (apply / eval / trigger / replace / sba / render) rather than a
   catch-all arm.

## Scope
- Core.idr grammar + `idris_emit.rs` for the continuous `Cant(Untap)` deed (design gate above).
- `PlayerAction::SkipNextUntap` (or equivalent) + its `GameEvent`, threaded through every
  event match, setting `skip_next_untap`.
- Macros (`macros/action/` + any static-ability macro) for both English forms, bidirectional
  (parse ⇄ render), following the landed keyword-action macro pattern.
- Graduate the blocked cards via `cargo xtask generate`; verify the delta and render fidelity.

## Gate
`cd idris && ./scripts/build` PASS; `cargo xtask idris-check` on a representative graduated
card (each new grammar shape needs a render arm or fidelity fails); `cargo xtask fidelity`
PASS; `cargo test -p deckmaste_core -p deckmaste_engine -p deckmaste_plugin` green;
`cargo clippy --all-targets -- -D warnings` clean; cite check 0 stale / 0 non-compliant.
