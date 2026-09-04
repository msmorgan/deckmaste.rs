---
needs: [ability-kind-taxonomy, idris-sba-not-a-static-ability]
---
**Delete the generic `Innate` Ability wrapper and represent each job it was
performing structurally.** The [`Intrinsic Ability`, `Conferral`, and keyword
definitions](../../contexts/game-model/CONTEXT.md) reserve distinct names for these concepts.
`Innate` currently hides type-conferred pseudo-abilities from visible ability
lists, makes some rows removal-immune, and hides nested keyword components.
Those are three independent concerns; [CR#113.12] does not create a special
removal-immune Ability class for them.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

An Ability conferred by a Card Type or Subtype occupies the ordinary Ability
hierarchy. A real type rule is conferred through the appropriate state-based,
turn-based, continuous, attachment, or deontic property rather than being
forged into an Ability. Basic land types confer ordinary mana Abilities without
an `Innate` wrapper; those Abilities retain the CR classification Intrinsic
Ability ([CR#305.6]). Composite Keyword Ability components remain executable
children of the named keyword container but do not independently enter the
carrier's visible Ability list.

Remove `Ability::Innate` from `deckmaste_core` and the engine, and from every
core-facing RON author (`plugins/builtin_v2/macros/stubs/subtypes/`: Aura,
Equipment, Fortification, Saga). The workbench has no `Innate` wrapper and must
not grow one as its conferral shapes fill in. Re-home Aura, Ward, basic-land,
Equipment, Fortification, and other existing uses according to the distinctions
above.

Acceptance checks both `has/lacks [ability]` behavior and `loses all
abilities` behavior so structural hiding is not confused with rules immunity,
with the workspace green and `cd idris && ./scripts/build` green at its module
count.
