---
needs: [idris-sba-not-a-static-ability]
---
**Align executable syntax, Abilities, and Effects with the
[Game Model glossary](../../contexts/game-model/CONTEXT.md) and
[CR#609.1,610.1,611.1,614.1,615.1].** Today
`OneShotEffect` names executable semantic syntax, its `Instr` alias points in
the opposite direction, and `StaticEffect` is the payload of
`Ability::Static` even though a Static Ability is not itself an Effect. The
latter enum also mixes continuous rule changes, replacement and prevention
effects, and non-effect rule data.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Make `Instruction` the canonical name for executable semantic syntax and
remove `OneShotEffect` as a compatibility alias. Rename the authored Static
Ability payload accordingly while preserving the compact
`Ability::Static(...)` RON shape. Model actual `OneShotEffect` and
`ContinuousEffect` values only where the engine represents results described
by [CR#610] and [CR#611]; keep `ReplacementEffect` and `PreventionEffect` as
the applicable continuous-effect subfamilies, with self-replacement effects
explicitly outside that family ([CR#614.15]).

Classify every current `StaticEffect` variant before moving it. Rule-affecting
continuous effects may remain continuous effects under [CR#611.1]; state-based
rules, instructions, and authoring-only structure move to their own homes.
Update `deckmaste_core`'s types, the Idris workbench
(`idris/src/Experimental/`), the core-facing RON stubs, lowering's core-facing
output, engine consumers, and terminology documentation together. Do not
retain misleading type aliases for compatibility.

Overlaps `workbench-fused-variants-3`, which collapses and renames overloaded
workbench effect constructors; whichever runs second reconciles the names.

Done when: `Instruction` is the only name for executable syntax and no
`OneShotEffect`/`Instr` alias survives; every former `StaticEffect` variant
sits in its classified home; the workspace is green and `cd idris &&
./scripts/build` is green at its module count.
