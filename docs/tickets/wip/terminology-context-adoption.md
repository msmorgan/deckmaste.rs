---
needs: [engine-combatant-role]
---
**Adopt the candidate root [context map](../../../CONTEXT-MAP.md),
[Game Model glossary](../../contexts/game-model/CONTEXT.md), and
[Oracle English glossary](../../contexts/oracle-english/CONTEXT.md) after the
nomenclature correction family lands.** (2026-09-04: the three renames still
open — `construction-dsl-node`, `english-v2-grammatical-relations`,
`english-v2-person-number-agreement` — are recorded by the audit as open
aliases rather than waited on.) This is the conformance and activation
ticket, not another compatibility pass.

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Audit public names and explanatory prose across `deckmaste_core`, the engine,
the Idris workbench (`idris/src/Experimental/`), construction, English-v2,
data, the core-facing RON, generated fixtures, and contributor docs. Core, the
engine, and the workbench should use the same Game Model term for the same
concept; Oracle English keeps standard linguistic terms even where a spelling
such as Object or Predicate legitimately means something different in the Game
Model. Fix remaining contradictory aliases, stale comments, and generated
vocabulary. Update the owning context's `CONTEXT.md` only where implementation
evidence exposed a real flaw in a definition, not to accommodate an old
identifier.

Only after that audit, add the stable instruction in `CLAUDE.md` to read the
root `CONTEXT-MAP.md` before terminology-sensitive repository work, follow it
to each relevant glossary, and keep those documents current through the
domain-modeling workflow. Until this ticket lands, the map and glossaries
remain unactivated proposals referenced by this ticket family only.

Done when: the audited surfaces carry one Game Model term per concept, the
`CLAUDE.md` instruction is in place, the workspace is green, and `cd idris &&
./scripts/build` is green at its module count.
