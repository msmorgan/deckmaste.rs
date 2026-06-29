---
needs: [core-event-reference-provenance]
---
**Core: `AdditionalCost(pay, body)` effect whose body reads the paid object via
event references.** From the 2026-06-28 idris↔rust model audit.

Today an additional cost lives only as cast-time cost data
(`CostChange::Additional` in `crates/deckmaste_core/src/continuous.rs`); a
resolution effect cannot bind the sacrificed/exiled object to read it
("sacrifice a creature: ~ deals damage equal to its power" — Fling, Momentous
Fall).

Idris `OneShotEffect.AdditionalCost (pay : Cost) body` (`idris/src/Core.idr`)
runs `body` with the payment's object bound via the event references
(`EventObject`/`EventActor`/`EventAmount`). At a spell/ability root the engine
hoists it to cast/activation time (the printed additional cost); nested, it is an
extra resolution-time cost.

Adoption: add the effect node; its body reads the paid object through the
provenance references — hence the dependency on `core-event-reference-provenance`.

Verdict: **improvement** (one mechanism for the hoisted printed cost and a nested
resolution cost, with the paid object readable in the body). Effort: **L**.
Related: depends on `core-event-reference-provenance`; `idris-effects-costs-and-choices`
(done/) covered a narrower paid-object binder (Emerge) on the Idris side.
