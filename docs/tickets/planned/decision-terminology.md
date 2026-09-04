---
needs: []
---
**Use Choice, Decision Point, and Decision for three different layers.** The
[`Choice`, `Decision Point`, and `Decision` definitions](../../contexts/game-model/CONTEXT.md)
define the target vocabulary: a Choice is a Magic rules selection or
procedure, a Decision Point is where engine progress waits for an external
agent, and a Decision is the submitted response. A Decision may encode either
a Choice or a proposed Action.

Audit semantic/core request types, engine suspension and resume state, player
interfaces, and tests. Rename generic `Choice` or `Decision` identifiers where
they currently name another member of the trio, without renaming CR-facing
rules operations merely for uniformity. Prefer domain-qualified names at
cross-layer APIs when the unqualified noun would be ambiguous. Preserve the
existing serialization shape only where it remains honest; this research
project does not need compatibility aliases.
