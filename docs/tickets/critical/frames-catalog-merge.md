---
needs: [macro-author-surface, spelling-crate-rename]
---
**Retire `plugins/builtin/frames/constructors.ron`: its seven entries
migrate into their words' `frames:`, and the lexicon becomes
single-origin.** Design: `docs/decisions/authoring-spelling-lowering.md`
(§8). Do this while the catalog is still seven entries — it only grows
otherwise.

## Scope

- Move each entry's frames onto the owning def (identity defs exist by
  `macro-author-surface`): `DealDamage`, `Target`, `This`, `You`,
  `AnyTarget` directly; the bodied `GainLife` entry's frames land on the
  life word's def (currently `GainsLife.ron`).
- **`TargetedDealDamage`'s interim home is a spelling-crate TEST FIXTURE**
  — a non-author-facing differential baseline inside the spelling test
  suite, NOT a macro def (that would make it author-spellable vocabulary)
  and NOT a catalog remnant. Its deletion is owned by
  `target-sugar-elaboration`, when compositional targeting covers it.
- Capability unification: macro-origin lexicon entries gain the
  constructor-only capabilities (body patterns, `announcement:`); the
  `Origin::Constructor` path and the catalog loader retire.
- From this point on, the english effort's frame-minting rounds write
  frames into defs — never a catalog.

## Gates

Standard constraints apply. Lexicon assembly single-origin; existing
unify/render fixtures green with frames relocated; a grep gate proves no
`constructors.ron` remains.
