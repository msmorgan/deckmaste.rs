---
needs: []
---
**Core: provenance-explicit event references — split `ThatObject`/`ThatPlayer` into
agent / patient / actor / defending-player references.** Surfaced by the 2026-06-28
idris↔rust model audit; the headline of the Idris model's "make every anaphor
explicit" pass.

Today a triggered/replacement body names the event's participants with just two
flat references — `Reference::ThatObject` and `Reference::ThatPlayer`
(`crates/deckmaste_core/src/reference.rs`): one object + one player per event.
That cannot spell a two-object event ("whenever a creature deals damage to
another creature" — source and recipient are both objects), and the defending
player of an attack is reachable only as a `DeciderSpec`
(`crates/deckmaste_core/src/decision.rs`), not as a `Reference`.

The Idris model (`idris/src/Core.idr`, the `Reference` namespace) decomposes the
event's roles into distinct references:

- `EventObject` — the event's agent/doer (the moving object of a zone change, the
  source of damage).
- `EventPatient` — the acted-upon thing (a damage recipient, a destroyed/
  countered object), kind-polymorphic (may be a player or an object).
- `EventActor` — the responsible player ("that player").
- `DefendingPlayer` — the defending player of an attack/combat (always a player,
  even versus a planeswalker or battle).

In Idris each is gated so it is only legal where the event supplies it; in Rust
that stays a runtime/validation concern — the *gating* implies no data-model
change, only the *role split* does.

Possible Rust adoption:

1. Add `EventAgent`, `EventPatient`, `EventActor`, `DefendingPlayer` reference
   forms to `Reference` (mapping/aliasing the current `ThatObject`/`ThatPlayer`
   during migration).
2. Populate the roles when a trigger/replacement frame is built — the engine
   already resolves the triggering event; this exposes its agent/patient/actor
   distinctly instead of collapsing to one object + one player.
3. Update the trigger/replacement parsers and the affected canon cards.

Verdict: **improvement** — the user's explicit "explicit anaphors/references"
goal, and a real expressivity gain (two-object events, and the defender as a
first-class player reference, are currently unspellable). Effort: **M** — the
`Reference` enum + RON spellings + the engine frame binder + parser + affected
cards.

Related (none subsumes the reference-vocabulary split): `engine-relation-spine`
(maybe/) adds defender-*side events*; `damage-provenance` (planned/) adds a
damage `Source`; `engine-combat-damage-event` (planned/) adds the combat
coordinate. The Idris side already landed as `idris-event-and-trigger-extensions`
(done/).
