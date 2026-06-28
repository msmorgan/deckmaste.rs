---
needs: []
---
**Engine/core: ability-level function-zone (`from`) on triggered (and static)
abilities.** From the 2026-06-28 idris↔rust model audit.

Today only `ActivatedAbility` carries `from: Option<Zone>`
(`crates/deckmaste_core/src/ability.rs`); `TriggeredAbility` and `StaticAbility`
have none. So a graveyard/hand trigger (Madness; "while this is in your
graveyard, ...") or a graveyard static (Riftstone Portal's land-mana from the
graveyard) can't declare the zone it functions in.

Idris carries `from : List Zone` uniformly on `Activated` / `Triggered` /
`Static` (`idris/src/Core.idr`).

Adoption: add `from` to `TriggeredAbility` (and `StaticAbility`), defaulting to
the battlefield and omitted on write when default; wire the trigger/static
machinery to honor it.

Verdict: **improvement** (graveyard/hand triggers and statics need it). Effort:
**S–M**. Related: `engine-static-ability-zone-gating` (planned/) covers the
Static slice — the **Triggered `from`** slice is the uncovered part here.
