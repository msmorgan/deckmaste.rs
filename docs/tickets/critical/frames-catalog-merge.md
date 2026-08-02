---
needs: [macro-author-surface, spelling-crate-rename]
---
**Retire `plugins/builtin/frames/constructors.ron`: its seven entries
migrate into their words' `frames:`, and the lexicon becomes
single-origin.** Design:
`docs/decisions/authoring-spelling-lowering.md`
(§8). Do this while the catalog is still seven entries — it only grows
otherwise.

## Scope

- Move each entry's frames onto the owning def (identity defs exist by
  `macro-author-surface`): `DealDamage`, `Target`, `This`, `You`,
  `AnyTarget` directly; the bodied compound entries (`GainLife`,
  `TargetedDealDamage`) are transitional workarounds — `GainLife`'s frames
  land on the life word; `TargetedDealDamage` persists TEMPORARILY as a
  differential baseline and dies when compositional targeting lands
  (`target-sugar-elaboration` + the spelling engine's occurrence work).
- Capability unification: macro-origin lexicon entries gain the
  constructor-only capabilities (body patterns, `announcement:`); the
  `Origin::Constructor` path and the catalog loader retire.
- English round 3 mints frames into defs from this point on — never the
  catalog.

## Gates

Standard constraints apply. Lexicon assembly single-origin; existing
unify/render fixtures green with frames relocated; a grep gate proves no
`constructors.ron` remains.
