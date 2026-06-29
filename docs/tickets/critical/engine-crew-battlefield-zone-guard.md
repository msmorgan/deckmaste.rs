---
needs: []
---
**Engine BUG: Crew (`TapTotal`) candidate set has no battlefield zone guard.**
Found in the 2026-06-29 code review.

`tap_total_subset` (`crates/deckmaste_engine/src/activate.rs:392`) filters
candidates only by `!tapped` + stat; `candidates_with` is zone-agnostic and
`ControlledBy(You)` matches the `controller` field set on library/hand objects
too. So the feasibility gate `can_activate` passes with zero battlefield
creatures, and the payment path emits `Tapped` for creatures in your
library/hand/graveyard — but only untapped creatures on the battlefield may be
tapped to crew ([CR#702.122a]). The sibling `PayPips` path guards exactly this
(`cast.rs:1090`, `zone == Some(zone)`); `tap_total_subset` is missing the
analogous guard. Untested — the only `TapTotal` test checks `cost_summary`, never
the subset against a populated game.

Fix: add `&& self.objects.obj(id).zone == Some(Zone::Battlefield)` to the
candidate filter; add an engine test that a library creature cannot crew.

Severity: **high** (unsound cost gate + spurious state mutation). Effort: **S**.
