---
needs: []
---
**The lethal-damage state-based actions have no spelling.** Found by
`lean-rules-tables` (2026-09-07): [CR#704.5g] reads the total damage marked on
a creature and [CR#704.5h] reads whether a source with deathtouch has dealt it
damage *since the last time state-based actions were checked*. Neither is
spellable — `ProjAxis` has no marked-damage axis (`Stat` is
`power|toughness|manaValue|loyalty`), and `Lookback` (`thisTurn`,
`earlierThisTurn`, `thisCombat`, `lastTurn`, `thisGame`, `thisWay`,
`triggering`) has no sweep-relative window. So `plugins/builtin/rules/sba/`'s
`lethal-damage.ron` is the one v1 row `Proofs/Rules.lean`'s builtin table
cannot carry.

Add the marked-damage projection and the sweep-relative lookback to
`lean/Semantics/`, with their citations, threading both through every read in
`Check/` that matches the enum exhaustively; pin the two rows as `SbaRule`s in
`Proofs/Rules.lean` beside the three that already prove, mirror both in
`crates/deckmaste_semantics_v2` (drift test), and note in `lean/CONTRACTS.md`
that the "Rules tables" gap is closed. Card evidence for the projection exists
independently of the tables ("damage marked on it"), so verify the spelling
against the corpus rather than against the rule alone. Standard constraints
apply.
