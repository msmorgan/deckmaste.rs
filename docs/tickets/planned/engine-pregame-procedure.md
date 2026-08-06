---
needs: []
---
**Engine: the pre-game procedure (`PreGame` decision) has no handler.**

`crates/deckmaste_engine/src/decide/pending/choice.rs` — `PreGame::resolve` is
unbuilt; `PreGameKind::Mulligan` is the only kind currently shaped.

[CR#103] is the whole pre-game procedure: determining who goes first, opening
hands, the mulligan loop, and the actions taken before the first turn (leaving
companions outside the game, revealing pre-game-visible cards, and so on).

`strategy-evaluator-core` (done) deferred this explicitly. Today the runner
presumably seeds a fixed opening state; this ticket makes the procedure real.
Decide first whether the sim needs a genuine mulligan loop or a deterministic
seeded opener — that choice sets the size.

Effort: **L**. Design input needed on scope.
