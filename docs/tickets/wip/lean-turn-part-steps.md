---
needs: []
---
**`TurnPart` lacks two parts the CR names.** Found by
`semantics-v2-macro-bodies-turn-parts` (2026-09-06): the beginning of combat
step [CR#506.1] has no constructor distinct from the `combat` phase, and the
ending phase [CR#500.1] has no constructor symmetric to `beginningPhase`.
Add `beginningOfCombat` and `endingPhase` to `lean/Semantics/Words.lean`
with their citations, thread them through every `TurnPart` read in `Check/`
(window ordering, skip and insert laws), pin one card each (a real
"beginning of combat" trigger and a real ending-phase reference, verified
against the corpus), mirror both in `crates/deckmaste_semantics_v2`
(drift test), then give `BeginningOfCombatStep` and `EndingPhase` under
`plugins_v2/builtin/macros/stubs/turn_parts/` their bodies. Standard
constraints apply.
