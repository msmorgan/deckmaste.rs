---
needs: []
design: true
---
Close the P0.W6 multiplayer leave-game seam (`decide.rs` `todo!("P0.W6:
multiplayer leave-game cleanup ([CR#800.4a])")` in `concede`): with more than
two live players, a player's loss must remove them from the game — owned
objects leave, control-change effects they control end, anything else that
would remain is exiled ([CR#800.4a]) — rather than a half-departed player
haunting the table. Today the guard is loud for `live_count() > 2`;
two-player concession terminalizes via `check_game_end` and is unaffected.

Design-gated: multiplayer is not on the 1v1 happy path — the design dialogue
should settle turn-order repair, pending-decision ownership, and
priority-round reshaping for a departed seat before any implementation.
