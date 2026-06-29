---
needs: []
---
**Low-severity follow-ups from the 2026-06-29 code review** (batched).

- `PlayerAttr::Life`/`HandSize` (`crates/deckmaste_core/src/continuous.rs:427`)
  are dead/unreachable and model non-continuous values (a
  `ModifyPlayer(You, SetTo(Life, 20))` is a silent no-op) — drop them or document
  as read-only axes invalid for `PlayerMod`.
- Stale docs: `Modification::flatten` still says `Several([AddPower,
  AddToughness])` (`continuous.rs:198`); `Selection::Those` / `Effect::With` docs
  cite a nonexistent `Reference::That` (`selection.rs:69`, `effect.rs:84`);
  `from_zone_qualified` doc says "While" but emits "As long as".
- `library_position` deep-offset renders "2 from the top"
  (`render/fragment.rs:380`) — ungrammatical for a rare non-zero anchor.
- `macro_ron_derive/input.rs` accepts trailing-default layouts the deserializer
  rejects with `compile_error!` (`generate.rs:337`) — reject earlier with a
  clearer message, or document the supported set (arity-3, 2 required + 1
  default).
- `pt_delta_clause` (`render/ability.rs:263`) drops dynamic (non-literal) P/T
  deltas from the modify predicate (pre-existing; preserved by the rename).
- `eval_count` uses non-saturating `(a - b).max(0)` (`layer.rs:916`) while
  Plus/Times saturate — consistency nit.
- `trigger_multiplier_extra` (`trigger.rs:773`) doesn't `flatten_composites`, so
  a `TriggerMultiplier` delivered via a keyword/composite would be missed (no card
  today).
- `decide.rs:438` `todo!()` for a `TapTotal` used inside `Effect::Unless` — a
  documented-unreachable seam; consider a graceful decline.
- `Domain` macro over-counts (counts all land subtypes, not just the 5 basic land
  types) — documented v1 approximation; needs a basic-land-type subset.
- `CostComponent` boxing asymmetry (`cost.rs:31,70`): `ManaCostOf` /
  `TapTotal.count` left inline while `filter` / `Do` are boxed — verify enum size.
- pip-payment convoke test comment overstates which pip is tap-covered
  (`tests/pip_payment.rs`) — Grizzly Bears matches both the generic and green
  rows.

Severity: **low** (cleanup / docs / edge cases). Effort: **M** (batchable).
