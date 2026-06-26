---
needs: []
---
**Protection's damage-prevention leg is a silent no-op in the engine.** A
creature with "protection from black" still takes combat and noncombat damage
from black sources. Surfaced by the idris↔rust structure audit (2026-06-26).

The DATA is correct: `crates/deckmaste_core/src/replacement.rs` defines
`Prevention` (`PreventAll { from, to }`), `crates/deckmaste_core/src/continuous.rs`
defines `StaticEffect::Prevention(Box<Prevention>)`, and
`plugins/builtin/macros/keyword/Protection.ron` emits the damage ("D") leg as
`Prevention(PreventAll(from: Param(0), to: Ref(This)))`. The other three protection
legs (can't be Enchanted/Blocked/Targeted, as `CantHappen`) work.

The bug: **no engine code ever reads `StaticEffect::Prevention`.**
`grep Prevention crates/deckmaste_engine/src` returns nothing. `gather_applicable`
(`replace_registry.rs`) collects only `StaticEffect::Replacement`; the
`DamageDealt` apply path (`step.rs`, ~`obj.damage += amount` / player life loss)
marks damage unconditionally — only lifelink/deathtouch are consulted, both
*after* the damage lands. There is no `todo!`/panic guard, so the effect is
silently inert.

A prevention effect is a replacement effect that reduces the would-be damage to
zero ([CR#615.1], [CR#702.16e] — "damage that would be dealt by [a source with
the stated quality] … is prevented"). The fix: route `StaticEffect::Prevention`
into the damage-intent path the way `Replacement` already is — at the point the
`DamageDealt` amount is computed, a matching `PreventAll`/`PreventN` zeroes or
reduces it before it is marked.

Note: the **Idris** grammar models this leg correctly as
`ReplaceAmount (DealDamage Nothing) (^0)` (`idris/src/Macros.idr`, `protection`
macro) — i.e. amount-to-zero on the damage event — which is a good shape to mirror.
Acceptance: a creature with protection from a color takes 0 damage from sources of
that color (combat and noncombat); regeneration/indestructible interactions
unaffected.
