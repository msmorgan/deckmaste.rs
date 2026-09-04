---
needs: []
---
**Rename `TypeDef.permanent` to `permanent_type` everywhere.** The old name
sounds as though the Card Type is itself a Permanent or that it partitions
Cards into Spell and Permanent kinds. The field actually records whether the
declared Card Type is one of the six Permanent Types in [CR#110.4]. See
[`Permanent Type`](../../contexts/game-model/CONTEXT.md).

Scope (ruling 2026-09-04): the Idris workbench, core, engine, and lowering's
core-facing output only; `deckmaste_semantics` and `idris/src/Semantics.idr`
are deletion-bound and untouched; their side converges at cutover when the RON
re-emit path points at `Experimental`.

Keep the flat `TypeDef` shape and its `confers` collection. Do not introduce a
two-member `Spell | Permanent` enum and do not encode combat or attackability
metadata in this flag: those capabilities remain positive conferrals. In
particular, Land is a Permanent Type and appears on Permanent Cards, but a land
card is not a Permanent Spell ([CR#110.4a..110.4b]). Any cast-resolution helper
must use the renamed fact with that distinction intact.

The workbench already spells this `Experimental.Words.permanentType`; core and
its consumers move to that spelling rather than the other way round. Migrate
`deckmaste_core`, the engine, lowering's core-facing output, the builtin-v2
type RON, generated fixtures, emitters, tests, and documentation in one ticket.

Overlaps `workbench-supplements-out`, which drops six card types from the
workbench `CardType`, and the `workbench-levelers`/`workbench-prototype` card
wrappers; whichever runs second reconciles the names.

Acceptance includes a Land witness, confirms the authored builtin-v2 RON
remains flat, and keeps `cd idris && ./scripts/build` green at its module
count.
