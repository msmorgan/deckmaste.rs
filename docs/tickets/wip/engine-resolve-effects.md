---
needs: []
---
Effect frames: May, If/Unless, ForEach, Modal, Delayed, and Reflexive. Each
frame type needs resolution-time evaluation and, for choice-bearing frames,
surfacing a decision for the player before continuing resolution.

## Scope (decided 2026-06-17)

`If` was already live (engine-citys-blessing). This ticket builds the remaining
four resolution-time frames; **Delayed + Reflexive are deferred** to
`engine-delayed-reflexive-triggers` because they need a brand-new delayed/
reflexive triggered-ability registry on `GameState` plus `scan_triggers`
integration (no such store exists — the scan reads only live battlefield
permanents' abilities), a distinct subsystem from effect-frame resolution.

In scope:
- **ForEach** — evaluate `over` once at resolution; schedule the inner effect
  once per matched object, binding the iterated object as `ThatObject` (per-
  iteration `bindings.that_object`). Reuses the existing `Reference::ThatObject`.
- **May** — surface `YesNo` to the controller; yes → `effect` then `if_did`;
  no → `if_not` ([CR#603,608]).
- **Modal** — surface `ChooseModes`; run each chosen mode's `effect` in written
  order ([CR#700.2]). Per-mode **targeting/costs** are announce-time and stay a
  loud seam.
- **Unless** — add `who: Reference` (default `You`) to `UnlessEffect`; surface
  `YesNo` to the payer; pay → run the `unless` cost components, skip `effect`;
  don't pay → run `effect` ([CR#118.12a,608.2d]). v1 pays `Do(PlayerAction)`
  verb costs + `Tap`/`Untap`; **mana** unless-costs are a loud seam (the
  PayCost/PayMana flow is announce-slot-bound; mid-resolution mana payment is
  unbuilt).

Mechanism: generalize `ChoiceContinuation` (state.rs) from a single
`{effect, frame}` into an enum so a decision answer can branch (May/Unless/
Modal), reusing the established pause (`self.pending` + `self.choice`) / resume
(`submit_decision` → `schedule_front(RunEffect)`) pattern.
