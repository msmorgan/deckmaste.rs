---
needs: [effect-instruction-taxonomy]
---
**Distinguish the CR Ability categories from orthogonal classifications and
the authored keyword surface.** Use the [`Ability` family in the domain
glossary](../../contexts/game-model/CONTEXT.md). [CR#113.3] has four general Ability categories:
spell, activated, triggered, and static. Mana Ability is an orthogonal
classification of some activated and triggered Abilities ([CR#113.4]); it is
not a fifth sibling kind.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Remove `Ability::Mana` and express mana classification on the applicable
activated or triggered Ability. Preserve `Ability::Keyword(...)` as an authored
sibling in the core-facing RON: it is the clean named container that lets
primitive and composite keyword definitions appear beside `Static(...)`, and it
does not claim that Keyword is a fifth CR Ability category.

Rename the engine's directly implemented keyword forms from `intrinsic` to
`primitive`. Reserve Intrinsic Ability for the CR usage, including the mana
Abilities supplied by basic land types ([CR#305.6]). Keep the existing
composite keyword shape—name plus nested Ability definitions—so declarations
such as a future `TripleThreat = [FirstStrike, Lifelink, Exalted]` remain
expressible without another RON wrapper. Apply the vocabulary consistently in
`deckmaste_core`, the engine, lowering's core-facing output, the Idris
workbench (`idris/src/Experimental/`), keyword data, classifiers, comments, and
policy documentation.

Overlaps `workbench-ability-kind-fold`, which folds the workbench's ability
kind into `Object` with an `AbilityP` payload; whichever runs second reconciles
the names.

Done when: `Ability` has no `Mana` variant and mana classification rides on the
activated or triggered Ability that carries it; no `intrinsic` spelling
survives for the directly implemented keyword forms; the workspace is green and
`cd idris && ./scripts/build` is green at its module count.
