---
needs: []
---
**Decide where a state-checked static ability lives, then finish hoisting
`Sba` out of `Ability::Static`.** Residue of `idris-sba-not-a-static-ability`
(2026-09-04): its conferral half landed (Aura's [CR#704.5m] rule is now
`Property::StateBased`), but `StaticEffect::Sba` stayed, on two grounds:
Ascend is a genuine static ability ([CR#702.131b]) whose only core spelling is
`Static(Sba(…))`, and v1 lowering is total over a source grammar that keeps
`Sba` until cutover.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched.

Ruling needed: a static ability whose effect holds while a game state holds
(Ascend; the testing Sagas) is a conditional static, not a state-based action.
Proposed: rename `StaticEffect::Sba { when, then }` to a conditional-static
spelling (the workbench already has `Conditionally c se` for exactly this),
so `Ability::Static` never says "Sba"; rules-defined SBAs stay on
`Property::StateBased` / `SbaRule`. Lowering's v1-facing input maps v1's `Sba`
onto the renamed variant until cutover. Re-spell `plugins/builtin/macros/keyword/Ascend.ron`'s
core-facing twin and `plugins/testing/cards/Test Saga*.ron`; keep
`deckmaste_plugin::keywords::ascend_macro_expands_to_static_sba` re-spelled
against the new name, never deleted.

Size: S–M. Done when: no `Sba` reachable from `Ability::Static` in
`deckmaste_core`; Ascend and the test Sagas lower and pass; workspace tests
green; workbench build at its module count. Standard constraints apply.

Ruling 2026-09-04: option (a) — rename `StaticEffect::Sba { when, then }`
to the conditional-static spelling mirroring the workbench's
`Conditionally c se`; lowering maps v1's `Sba` onto it until cutover;
rules-defined SBAs stay on `Property::StateBased`. Ready to run.
