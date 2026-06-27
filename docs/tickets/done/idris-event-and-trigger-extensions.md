---
needs: []
---
**Idris grammar: event-anaphora and trigger-shape extensions.** `[design]` — net-new
`EventKind`s and event caps in `idris/src/Core.idr`. From the 2026-06-26 grammar census.
The event body currently exposes only `EventObject`/`EventActor`/`ThatMuch`, and the
`EventKind` set is missing several common triggers.

1. **Patient anaphor (highest value).** `Facet.Patient` is a *filter*, not a binder, so an
   event's defending player / damage recipient is unreachable. Add `hasPatient` to
   `EventCaps` and `EventPatient : {hasPatient …} -> Reference b k`, populated for damage
   (recipient) and `Begins Attack` (defender), plus a `DefendingPlayer` reference usable in
   static block-restriction scope. Unblocks landwalk, Annihilator, Afflict, "deals combat
   damage to a player → that player…".
2. **Life-change events.** `GainLife`/`LoseLife` are `Action`s but not `EventKind`s — no
   "whenever you gain/lose life" trigger, no life `EventAgg`, no replacement. Add them to
   `EventKind` with `hasActor` + `hasAmount`.
3. **Control-change event.** A control change isn't a zone change, so `ZoneChanged` misses
   it. Add `ControlChanged : Maybe (Predicate b APlayer) -> EventKind`.
4. **Nth-occurrence facet.** Only `IsFirst` exists. Add `IsNth : Nat -> Window` with
   `IsFirst = IsNth 1` (Erayo "4th spell", "your second card each turn").
5. **Leaving-state / untap events.** `Becomes` only fires *entering* a state. Add `Untap`
   to `EventKind`, `Untapped` to `ObjectState` (with `IsBecomesState`) so "doesn't untap"
   can be a `CantHappen`, and an optional effect body on `TurnFaceUp` (Megamorph's face-up
   counter).
6. **Trigger multiplication.** No primitive for "this triggered ability fires an additional
   time" (Panharmonicon, Teysa Karlov). Add `TriggerMultiplier : EventQuery -> Count ->
   StaticEffect` (`Also` adds an effect, not a re-fire).

*Serializes with the other `idris-*` grammar tickets — they all rewrite
`idris/src/Core.idr`, so only one can be in flight at a time. `needs:` is empty because
the blocking is file-level, not logical precedence.*
