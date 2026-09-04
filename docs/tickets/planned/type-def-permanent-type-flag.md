---
needs: []
---
**Rename `TypeDef.permanent` to `permanent_type` everywhere.** The old name
sounds as though the Card Type is itself a Permanent or that it partitions
Cards into Spell and Permanent kinds. The field actually records whether the
declared Card Type is one of the six Permanent Types in [CR#110.4]. See
[`Permanent Type`](../../contexts/game-model/CONTEXT.md).

Keep the flat `TypeDef` shape and its `confers` collection. Do not introduce a
two-member `Spell | Permanent` enum and do not encode combat or attackability
metadata in this flag: those capabilities remain positive conferrals. In
particular, Land is a Permanent Type and appears on Permanent Cards, but a land
card is not a Permanent Spell ([CR#110.4a..110.4b]). Any cast-resolution helper
must use the renamed fact with that distinction intact.

Migrate the semantic/core mirrors, Idris model, builtin-v2 type RON, generated
fixtures, emitters, tests, and documentation in one ticket. Acceptance includes
a Land witness and confirms the authored RON remains flat.
